// SPDX-License-Identifier: GPL-2.0

//! The `kernel` crate

#![no_std]
#![feature(allocator_api, alloc_error_handler)]

// Ensure conditional compilation based on the kernel configuration works;
// otherwise we may silently break things like initcall handling.
#[cfg(not(CONFIG_RUST))]
compile_error!("Missing kernel configuration for conditional compilation");

use core::panic::PanicInfo;

mod allocator;

#[doc(hidden)]
pub mod bindings;

pub mod c_types;
pub mod chrdev;
mod error;
pub mod file_operations;
pub mod miscdev;
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

/// An instance equivalent to `THIS_MODULE` in C code.
pub struct ThisModule(*mut bindings::module);

// SAFETY: `THIS_MODULE` may be used from all threads within a module.
unsafe impl Sync for ThisModule {}

impl ThisModule {
    pub const unsafe fn from_ptr(ptr: *mut bindings::module) -> ThisModule {
        ThisModule(ptr)
    }

    pub fn kernel_param_lock(&self) -> KParamGuard<'_> {
        // SAFETY: `kernel_param_lock` will check if the pointer is null and use the built-in mutex
        // in that case.
        #[cfg(CONFIG_SYSFS)]
        unsafe { bindings::kernel_param_lock(self.0) }

        KParamGuard { this_module: self }
    }
}

/// Scoped lock on the kernel parameters of `ThisModule`. Lock will be released
/// when this struct is dropped.
pub struct KParamGuard<'a> {
    this_module: &'a ThisModule
}

#[cfg(CONFIG_SYSFS)]
impl<'a> Drop for KParamGuard<'a> {
    fn drop(&mut self) {
        // SAFETY: `kernel_param_lock` will check if the pointer is null and use the built-in mutex
        // in that case. The existance of `self` guarantees that the lock is held.
        unsafe { bindings::kernel_param_unlock(self.this_module.0) }
    }
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
