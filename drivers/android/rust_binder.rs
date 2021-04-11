// SPDX-License-Identifier: GPL-2.0

//! Binder -- the Android IPC mechanism.
//!
//! TODO: This module is a work in progress.

#![no_std]
#![feature(global_asm, try_reserve, allocator_api, concat_idents)]

use alloc::{boxed::Box, sync::Arc};
use core::{marker::PhantomData, pin::Pin, ptr};
use kernel::{
    bindings, c_types, cstr,
    miscdev::Registration,
    prelude::*,
    user_ptr::{UserSlicePtrReader, UserSlicePtrWriter},
    Error,
};

mod allocation;
mod context;
mod defs;
mod linked_list;
mod node;
mod process;
mod range_alloc;
mod raw_list;
mod thread;
mod transaction;

use {context::Context, thread::Thread};

module! {
    type: BinderModule,
    name: b"rust_binder",
    author: b"Wedson Almeida Filho",
    description: b"Android Binder",
    license: b"GPL v2",
    params: {},
}

enum Either<L, R> {
    Left(L),
    Right(R),
}

trait DeliverToRead {
    /// Performs work. Returns true if remaining work items in the queue should be processed
    /// immediately, or false if it should return to caller before processing additional work
    /// items.
    fn do_work(
        self: Arc<Self>,
        thread: &Thread,
        writer: &mut UserSlicePtrWriter,
    ) -> KernelResult<bool>;

    /// Cancels the given work item. This is called instead of [`DeliverToRead::do_work`] when work
    /// won't be delivered.
    fn cancel(self: Arc<Self>) {}

    /// Returns the linked list links for the work item.
    fn get_links(&self) -> &linked_list::Links<dyn DeliverToRead>;
}

impl linked_list::GetLinks for Arc<dyn DeliverToRead> {
    type EntryType = dyn DeliverToRead;
    fn get_links(obj: &dyn DeliverToRead) -> &linked_list::Links<dyn DeliverToRead> {
        obj.get_links()
    }
}

struct DeliverCode {
    code: u32,
    links: linked_list::Links<dyn DeliverToRead>,
}

impl DeliverCode {
    fn new(code: u32) -> Self {
        Self {
            code,
            links: linked_list::Links::new(),
        }
    }
}

impl DeliverToRead for DeliverCode {
    fn do_work(
        self: Arc<Self>,
        _thread: &Thread,
        writer: &mut UserSlicePtrWriter,
    ) -> KernelResult<bool> {
        writer.write(&self.code)?;
        Ok(true)
    }

    fn get_links(&self) -> &linked_list::Links<dyn DeliverToRead> {
        &self.links
    }
}

extern "C" {
    #[allow(improper_ctypes)]
    fn rust_helper_alloc_pages(
        gfp_mask: bindings::gfp_t,
        order: c_types::c_uint,
    ) -> *mut bindings::page;

    #[allow(improper_ctypes)]
    fn rust_helper_kmap(page: *mut bindings::page) -> *mut c_types::c_void;

    #[allow(improper_ctypes)]
    fn rust_helper_kunmap(page: *mut bindings::page);
}

/// Pages holds a reference to a set of pages of order `ORDER`. Having the order as a generic const
/// allows the struct to have the same size as pointer.
struct Pages<const ORDER: u32> {
    pages: *mut bindings::page,
}

impl<const ORDER: u32> Pages<ORDER> {
    fn new() -> KernelResult<Self> {
        // TODO: Consider whether we want to allow callers to specify flags.
        let pages = unsafe {
            rust_helper_alloc_pages(
                bindings::GFP_KERNEL | bindings::__GFP_ZERO | bindings::__GFP_HIGHMEM,
                ORDER,
            )
        };
        if pages.is_null() {
            return Err(Error::ENOMEM);
        }
        Ok(Self { pages })
    }

    fn insert_page(&self, vma: &mut bindings::vm_area_struct, address: usize) -> KernelResult {
        let ret = unsafe { bindings::vm_insert_page(vma, address as _, self.pages) };
        if ret != 0 {
            Err(Error::from_kernel_errno(ret))
        } else {
            Ok(())
        }
    }

    fn copy_into_page(
        &self,
        reader: &mut UserSlicePtrReader,
        offset: usize,
        len: usize,
    ) -> KernelResult {
        let mapping = self.kmap(0).unwrap();
        unsafe { reader.read_raw((mapping.ptr as usize + offset) as _, len) }?;
        Ok(())
    }

    unsafe fn read(&self, dest: *mut u8, offset: usize, len: usize) {
        let mapping = self.kmap(0).unwrap();
        ptr::copy((mapping.ptr as *mut u8).add(offset), dest, len);
    }

    unsafe fn write(&self, src: *const u8, offset: usize, len: usize) {
        let mapping = self.kmap(0).unwrap();
        ptr::copy(src, (mapping.ptr as *mut u8).add(offset), len);
    }

    fn kmap(&self, index: usize) -> Option<PageMapping> {
        if index >= 1usize << ORDER {
            return None;
        }
        let page = unsafe { self.pages.add(index) };
        let ptr = unsafe { rust_helper_kmap(page) };
        Some(PageMapping {
            page,
            ptr,
            _phantom: PhantomData,
        })
    }
}

impl<const ORDER: u32> Drop for Pages<ORDER> {
    fn drop(&mut self) {
        unsafe { bindings::__free_pages(self.pages, ORDER) };
    }
}

struct PageMapping<'a> {
    page: *mut bindings::page,
    ptr: *mut c_types::c_void,
    _phantom: PhantomData<&'a i32>,
}

impl Drop for PageMapping<'_> {
    fn drop(&mut self) {
        unsafe { rust_helper_kunmap(self.page) };
    }
}

const fn ptr_align(value: usize) -> usize {
    let size = core::mem::size_of::<usize>() - 1;
    (value + size) & !size
}

unsafe impl Sync for BinderModule {}

struct BinderModule {
    _reg: Pin<Box<Registration<Arc<Context>>>>,
}

impl KernelModule for BinderModule {
    fn init() -> KernelResult<Self> {
        let pinned_ctx = Context::new()?;
        let ctx = unsafe { Pin::into_inner_unchecked(pinned_ctx) };
        let reg = Registration::<Arc<Context>>::new_pinned::<process::Process>(
            cstr!("rust_binder"),
            None,
            ctx,
        )?;
        Ok(Self { _reg: reg })
    }
}
