// SPDX-License-Identifier: GPL-2.0

use core::convert::{TryFrom, TryInto};
use core::{marker, mem, ptr};

use alloc::boxed::Box;

use crate::bindings;
use crate::c_types;
use crate::error::{Error, KernelResult};
use crate::user_ptr::{UserSlicePtr, UserSlicePtrReader, UserSlicePtrWriter};

bitflags::bitflags! {
    pub struct FileFlags: c_types::c_uint {
        const NONBLOCK = bindings::O_NONBLOCK;
    }
}

pub struct File {
    ptr: *const bindings::file,
}

impl File {
    unsafe fn from_ptr(ptr: *const bindings::file) -> File {
        File { ptr }
    }

    pub fn pos(&self) -> u64 {
        unsafe { (*self.ptr).f_pos as u64 }
    }

    pub fn flags(&self) -> FileFlags {
        FileFlags::from_bits_truncate(unsafe { (*self.ptr).f_flags })
    }
}

// Matches std::io::SeekFrom in the Rust stdlib
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

unsafe extern "C" fn open_callback<T: FileOperations>(
    _inode: *mut bindings::inode,
    file: *mut bindings::file,
) -> c_types::c_int {
    let f = match T::open() {
        Ok(f) => Box::new(f),
        Err(e) => return e.to_kernel_errno(),
    };
    (*file).private_data = Box::into_raw(f) as *mut c_types::c_void;
    0
}

unsafe extern "C" fn read_callback<T: FileOperations>(
    file: *mut bindings::file,
    buf: *mut c_types::c_char,
    len: c_types::c_size_t,
    offset: *mut bindings::loff_t,
) -> c_types::c_ssize_t {
    let mut data = match UserSlicePtr::new(buf as *mut c_types::c_void, len) {
        Ok(ptr) => ptr.writer(),
        Err(e) => return e.to_kernel_errno().try_into().unwrap(),
    };
    let f = &*((*file).private_data as *const T);
    // No FMODE_UNSIGNED_OFFSET support, so offset must be in [0, 2^63).
    // See discussion in #113
    let positive_offset = match (*offset).try_into() {
        Ok(v) => v,
        Err(_) => return Error::EINVAL.to_kernel_errno().try_into().unwrap(),
    };
    let read = T::READ.unwrap();
    match read(f, &File::from_ptr(file), &mut data, positive_offset) {
        Ok(()) => {
            let written = len - data.len();
            (*offset) += bindings::loff_t::try_from(written).unwrap();
            written.try_into().unwrap()
        }
        Err(e) => e.to_kernel_errno().try_into().unwrap(),
    }
}

unsafe extern "C" fn write_callback<T: FileOperations>(
    file: *mut bindings::file,
    buf: *const c_types::c_char,
    len: c_types::c_size_t,
    offset: *mut bindings::loff_t,
) -> c_types::c_ssize_t {
    let mut data = match UserSlicePtr::new(buf as *mut c_types::c_void, len) {
        Ok(ptr) => ptr.reader(),
        Err(e) => return e.to_kernel_errno().try_into().unwrap(),
    };
    let f = &*((*file).private_data as *const T);
    // No FMODE_UNSIGNED_OFFSET support, so offset must be in [0, 2^63).
    // See discussion in #113
    let positive_offset = match (*offset).try_into() {
        Ok(v) => v,
        Err(_) => return Error::EINVAL.to_kernel_errno().try_into().unwrap(),
    };
    let write = T::WRITE.unwrap();
    match write(f, &mut data, positive_offset) {
        Ok(()) => {
            let read = len - data.len();
            (*offset) += bindings::loff_t::try_from(read).unwrap();
            read.try_into().unwrap()
        }
        Err(e) => e.to_kernel_errno().try_into().unwrap(),
    }
}

unsafe extern "C" fn release_callback<T: FileOperations>(
    _inode: *mut bindings::inode,
    file: *mut bindings::file,
) -> c_types::c_int {
    let ptr = mem::replace(&mut (*file).private_data, ptr::null_mut());
    drop(Box::from_raw(ptr as *mut T));
    0
}

unsafe extern "C" fn llseek_callback<T: FileOperations>(
    file: *mut bindings::file,
    offset: bindings::loff_t,
    whence: c_types::c_int,
) -> bindings::loff_t {
    let off = match whence as u32 {
        bindings::SEEK_SET => match offset.try_into() {
            Ok(v) => SeekFrom::Start(v),
            Err(_) => return Error::EINVAL.to_kernel_errno().into(),
        },
        bindings::SEEK_CUR => SeekFrom::Current(offset),
        bindings::SEEK_END => SeekFrom::End(offset),
        _ => return Error::EINVAL.to_kernel_errno().into(),
    };
    let f = &*((*file).private_data as *const T);
    let seek = T::SEEK.unwrap();
    match seek(f, &File::from_ptr(file), off) {
        Ok(off) => off as bindings::loff_t,
        Err(e) => e.to_kernel_errno().into(),
    }
}

unsafe extern "C" fn fsync_callback<T: FileOperations>(
    file: *mut bindings::file,
    start: bindings::loff_t,
    end: bindings::loff_t,
    datasync: c_types::c_int,
) -> c_types::c_int {
    let start = match start.try_into() {
        Ok(v) => v,
        Err(_) => return Error::EINVAL.to_kernel_errno(),
    };
    let end = match end.try_into() {
        Ok(v) => v,
        Err(_) => return Error::EINVAL.to_kernel_errno(),
    };
    let datasync = datasync != 0;
    let fsync = T::FSYNC.unwrap();
    let f = &*((*file).private_data as *const T);
    match fsync(f, &File::from_ptr(file), start, end, datasync) {
        Ok(result) => result as c_types::c_int,
        Err(e) => e.to_kernel_errno(),
    }
}

pub(crate) struct FileOperationsVtable<T>(marker::PhantomData<T>);

impl<T: FileOperations> FileOperationsVtable<T> {
    pub(crate) const VTABLE: bindings::file_operations = bindings::file_operations {
        open: Some(open_callback::<T>),
        release: Some(release_callback::<T>),
        read: if let Some(_) = T::READ {
            Some(read_callback::<T>)
        } else {
            None
        },
        write: if let Some(_) = T::WRITE {
            Some(write_callback::<T>)
        } else {
            None
        },
        llseek: if let Some(_) = T::SEEK {
            Some(llseek_callback::<T>)
        } else {
            None
        },

        check_flags: None,
        compat_ioctl: None,
        copy_file_range: None,
        fallocate: None,
        fadvise: None,
        fasync: None,
        flock: None,
        flush: None,
        fsync: if let Some(_) = T::FSYNC {
            Some(fsync_callback::<T>)
        } else {
            None
        },
        get_unmapped_area: None,
        iterate: None,
        iterate_shared: None,
        iopoll: None,
        lock: None,
        mmap: None,
        mmap_supported_flags: 0,
        owner: ptr::null_mut(),
        poll: None,
        read_iter: None,
        remap_file_range: None,
        sendpage: None,
        setlease: None,
        show_fdinfo: None,
        splice_read: None,
        splice_write: None,
        unlocked_ioctl: None,
        write_iter: None,
    };
}

pub type ReadFn<T> = Option<fn(&T, &File, &mut UserSlicePtrWriter, u64) -> KernelResult<()>>;
pub type WriteFn<T> = Option<fn(&T, &mut UserSlicePtrReader, u64) -> KernelResult<()>>;
pub type SeekFn<T> = Option<fn(&T, &File, SeekFrom) -> KernelResult<u64>>;
pub type FSync<T> = Option<fn(&T, &File, u64, u64, bool) -> KernelResult<u32>>;

/// `FileOperations` corresponds to the kernel's `struct file_operations`. You
/// implement this trait whenever you'd create a `struct file_operations`.
/// File descriptors may be used from multiple threads (or processes)
/// concurrently, so your type must be `Sync`.
pub trait FileOperations: Sync + Sized {
    /// Creates a new instance of this file. Corresponds to the `open` function
    /// pointer in `struct file_operations`.
    fn open() -> KernelResult<Self>;

    /// Reads data from this file to userspace. Corresponds to the `read`
    /// function pointer in `struct file_operations`.
    const READ: ReadFn<Self> = None;

    /// Writes data from userspace o this file. Corresponds to the `write`
    /// function pointer in `struct file_operations`.
    const WRITE: WriteFn<Self> = None;

    /// Changes the position of the file. Corresponds to the `llseek` function
    /// pointer in `struct file_operations`.
    const SEEK: SeekFn<Self> = None;

    /// Syncs pending changes to this file. Corresponds to the `fsync` function 
    /// pointer in the `struct file_operations`.
    const FSYNC: FSync<Self> = None;
}
