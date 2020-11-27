// SPDX-License-Identifier: GPL-2.0

use core::convert::{TryFrom, TryInto};
use core::{marker, mem, ptr};

use alloc::boxed::Box;
use alloc::sync::Arc;

use crate::bindings;
use crate::c_types;
use crate::error::{Error, KernelResult};
use crate::user_ptr::{UserSlicePtr, UserSlicePtrReader, UserSlicePtrWriter};

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
}

// Matches std::io::SeekFrom in the Rust stdlib
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

fn from_kernel_result<T>(r: KernelResult<T>) -> T
where
    T: TryFrom<c_types::c_int>,
    T::Error: core::fmt::Debug,
{
    match r {
        Ok(v) => v,
        Err(e) => T::try_from(e.to_kernel_errno()).unwrap(),
    }
}

macro_rules! from_kernel_result {
    ($($tt:tt)*) => {{
        from_kernel_result((|| {
            $($tt)*
        })())
    }};
}

unsafe extern "C" fn open_callback<T: FileOperations>(
    _inode: *mut bindings::inode,
    file: *mut bindings::file,
) -> c_types::c_int {
    from_kernel_result! {
        let ptr = T::open()?.into_pointer();
        (*file).private_data = ptr as *mut c_types::c_void;
        Ok(0)
    }
}

unsafe extern "C" fn read_callback<T: FileOperations>(
    file: *mut bindings::file,
    buf: *mut c_types::c_char,
    len: c_types::c_size_t,
    offset: *mut bindings::loff_t,
) -> c_types::c_ssize_t {
    from_kernel_result! {
        let mut data = UserSlicePtr::new(buf as *mut c_types::c_void, len)?.writer();
        let f = &*((*file).private_data as *const T);
        // No FMODE_UNSIGNED_OFFSET support, so offset must be in [0, 2^63).
        // See discussion in #113
        T::READ.unwrap()(f, &File::from_ptr(file), &mut data, (*offset).try_into()?)?;
        let written = len - data.len();
        (*offset) += bindings::loff_t::try_from(written).unwrap();
        Ok(written.try_into().unwrap())
    }
}

unsafe extern "C" fn write_callback<T: FileOperations>(
    file: *mut bindings::file,
    buf: *const c_types::c_char,
    len: c_types::c_size_t,
    offset: *mut bindings::loff_t,
) -> c_types::c_ssize_t {
    from_kernel_result! {
        let mut data = UserSlicePtr::new(buf as *mut c_types::c_void, len)?.reader();
        let f = &*((*file).private_data as *const T);
        // No FMODE_UNSIGNED_OFFSET support, so offset must be in [0, 2^63).
        // See discussion in #113
        T::WRITE.unwrap()(f, &mut data, (*offset).try_into()?)?;
        let read = len - data.len();
        (*offset) += bindings::loff_t::try_from(read).unwrap();
        Ok(read.try_into().unwrap())
    }
}

unsafe extern "C" fn release_callback<T: FileOperations>(
    _inode: *mut bindings::inode,
    file: *mut bindings::file,
) -> c_types::c_int {
    let ptr = mem::replace(&mut (*file).private_data, ptr::null_mut());
    T::release(T::Wrapper::from_pointer(ptr as _), &File::from_ptr(file));
    0
}

unsafe extern "C" fn llseek_callback<T: FileOperations>(
    file: *mut bindings::file,
    offset: bindings::loff_t,
    whence: c_types::c_int,
) -> bindings::loff_t {
    from_kernel_result! {
        let off = match whence as u32 {
            bindings::SEEK_SET => SeekFrom::Start(offset.try_into()?),
            bindings::SEEK_CUR => SeekFrom::Current(offset),
            bindings::SEEK_END => SeekFrom::End(offset),
            _ => return Err(Error::EINVAL),
        };
        let f = &*((*file).private_data as *const T);
        let off = T::SEEK.unwrap()(f, &File::from_ptr(file), off)?;
        Ok(off as bindings::loff_t)
    }
}

unsafe extern "C" fn fsync_callback<T: FileOperations>(
    file: *mut bindings::file,
    start: bindings::loff_t,
    end: bindings::loff_t,
    datasync: c_types::c_int,
) -> c_types::c_int {
    from_kernel_result! {
        let start = start.try_into()?;
        let end = end.try_into()?;
        let datasync = datasync != 0;
        let f = &*((*file).private_data as *const T);
        let res = T::FSYNC.unwrap()(f, &File::from_ptr(file), start, end, datasync)?;
        Ok(res.try_into().unwrap())
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
    type Wrapper: PointerWrapper<Self>;

    /// Creates a new instance of this file. Corresponds to the `open` function
    /// pointer in `struct file_operations`.
    fn open() -> KernelResult<Self::Wrapper>;

    /// Cleans up after the last reference to the file goes away. Note that the object is moved, so
    /// it will be freed automatically unless the implemention moves it elsewhere. Corresponds to
    /// the `release` function pointer in `struct file_operations`.
    fn release(_obj: Self::Wrapper, _file: &File) {}

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

/// `PointerWrapper` is used to convert an object into a raw pointer that represents it. It can
/// eventually be converted back into the object. This is used to store objects as pointers in
/// kernel data structures, for example, an implementation of `FileOperations` in `struct
/// file::private_data`.
pub trait PointerWrapper<T> {
    fn into_pointer(self) -> *const T;
    unsafe fn from_pointer(ptr: *const T) -> Self;
}

impl<T> PointerWrapper<T> for Box<T> {
    fn into_pointer(self) -> *const T {
        Box::into_raw(self)
    }

    unsafe fn from_pointer(ptr: *const T) -> Self {
        Box::<T>::from_raw(ptr as _)
    }
}

impl<T> PointerWrapper<T> for Arc<T> {
    fn into_pointer(self) -> *const T {
        Arc::into_raw(self)
    }

    unsafe fn from_pointer(ptr: *const T) -> Self {
        Arc::<T>::from_raw(ptr)
    }
}
