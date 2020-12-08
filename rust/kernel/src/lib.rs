// SPDX-License-Identifier: GPL-2.0

//! The `kernel` crate

#![no_std]
#![feature(allocator_api, alloc_error_handler)]

// Ensure conditional compilation based on the kernel configuration works;
// otherwise we may silently break things like initcall handling.
#[cfg(not(CONFIG_HAS_RUST))]
compile_error!("Missing kernel configuration for conditional compilation");

extern crate alloc;

use alloc::boxed::Box;
use core::alloc::Layout;
use core::mem::MaybeUninit;
use core::panic::PanicInfo;
use core::pin::Pin;
use core::ptr::NonNull;

mod allocator;
pub mod bindings;
pub mod c_types;
pub mod chrdev;
mod error;
pub mod file_operations;
pub mod prelude;
pub mod printk;
pub mod random;

#[cfg(CONFIG_SYSCTL)]
pub mod sysctl;

mod types;
pub mod user_ptr;

pub use crate::error::{Error, KernelResult};
pub use crate::types::{CStr, Mode};

/// KernelModule is the top level entrypoint to implementing a kernel module. Your kernel module
/// should implement the `init` method on it, which maps to the `module_init` macro in Linux C API.
/// You can use this method to do whatever setup or registration your module should do. For any
/// teardown or cleanup operations, your type may implement [`Drop`].
///
/// [`Drop`]: https://doc.rust-lang.org/stable/core/ops/trait.Drop.html
pub trait KernelModule: Sized + Sync {
    fn init() -> KernelResult<Self>;
}

extern "C" {
    fn rust_helper_BUG() -> !;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        rust_helper_BUG();
    }
}

#[global_allocator]
static ALLOCATOR: allocator::KernelAllocator = allocator::KernelAllocator;

/// Attempts to allocate memory for `value` using the global allocator. On success, `value` is
/// moved into it and returned to the caller wrapped in a `Box`.
pub fn try_alloc<T>(value: T) -> KernelResult<Box<T>> {
    let layout = Layout::new::<MaybeUninit<T>>();
    let ptr: NonNull<MaybeUninit<T>> = if layout.size() == 0 {
        NonNull::dangling()
    // SAFETY: We checked that the layout size is nonzero.
    } else if let Some(nn) = NonNull::new(unsafe { alloc::alloc::alloc(layout) }) {
        nn.cast()
    } else {
        return Err(Error::ENOMEM);
    };

    unsafe {
        // SAFETY: `ptr` was just allocated and isn't used afterwards.
        let mut b = Box::from_raw(ptr.as_ptr());
        // SAFETY: The pointer is valid for write and is properly aligned. The dangling pointer
        // case is only when the size of the value is zero; writing zero bytes to it is allowed.
        b.as_mut_ptr().write(value);
        // SAFETY: The value was initialised in the call above.
        Ok(Box::from_raw(Box::into_raw(b) as *mut T))
    }
}

/// Attempts to allocate memory for `value` using the global allocator. On success, `value` is
/// moved into it and returned to the caller wrapped in a pinned `Box`.
pub fn try_alloc_pinned<T>(value: T) -> KernelResult<Pin<Box<T>>> {
    Ok(Pin::from(try_alloc(value)?))
}
