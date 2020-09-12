// SPDX-License-Identifier: GPL-2.0

//! The `kernel` crate

#![no_std]
#![feature(allocator_api, alloc_error_handler, const_raw_ptr_deref)]

extern crate alloc;

use core::panic::PanicInfo;

mod allocator;
pub mod bindings;
pub mod c_types;
pub mod chrdev;
mod error;
pub mod file_operations;
pub mod filesystem;
pub mod prelude;
pub mod printk;
pub mod random;
pub mod sysctl;
mod types;
pub mod user_ptr;

pub use crate::error::{Error, KernelResult};
pub use crate::types::{CStr, Mode};

/// Declares the entrypoint for a kernel module. The first argument should be a type which
/// implements the [`KernelModule`] trait. Also accepts various forms of kernel metadata.
///
/// Example:
/// ```rust,no_run
/// use kernel::prelude::*;
///
/// struct MyKernelModule;
/// impl KernelModule for MyKernelModule {
///     fn init() -> KernelResult<Self> {
///         Ok(MyKernelModule)
///     }
/// }
///
/// kernel_module!(
///     MyKernelModule,
///     author: b"Rust for Linux Contributors",
///     description: b"My very own kernel module!",
///     license: b"GPL"
/// );
#[macro_export]
macro_rules! kernel_module {
    ($module:ty, $($name:ident : $value:expr),*) => {
        static mut __MOD: Option<$module> = None;

        // Built-in modules are initialized through an initcall pointer
        //
        // TODO: should we compile a C file on the fly to avoid duplication?
        #[cfg(not(MODULE))]
        #[cfg(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS))]
        #[link_section = ".initcall6.init"]
        #[used]
        pub static __initcall: extern "C" fn() -> $crate::c_types::c_int = init_module;

        #[cfg(not(MODULE))]
        #[cfg(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)]
        global_asm!(
            r#".section ".initcall6.init", "a"
            __initcall:
                .long   init_module - .
                .previous
            "#
        );

        // TODO: pass the kernel module name here to generate a unique,
        // helpful symbol name (the name would also useful for the `modinfo`
        // issue below).
        #[no_mangle]
        pub extern "C" fn init_module() -> $crate::c_types::c_int {
            match <$module as $crate::KernelModule>::init() {
                Ok(m) => {
                    unsafe {
                        __MOD = Some(m);
                    }
                    return 0;
                }
                Err(e) => {
                    return e.to_kernel_errno();
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn cleanup_module() {
            unsafe {
                // Invokes drop() on __MOD, which should be used for cleanup.
                __MOD = None;
            }
        }

        $(
            $crate::kernel_module!(@attribute $name, $value);
        )*
    };

    // TODO: The modinfo attributes below depend on the compiler placing
    // the variables in order in the .modinfo section, so that you end up
    // with b"key=value\0" in order in the section. This is a reasonably
    // standard trick in C, but I'm not sure that rustc guarantees it.
    //
    // Ideally we'd be able to use concat_bytes! + stringify_bytes! +
    // some way of turning a string literal (or at least a string
    // literal token) into a bytes literal, and get a single static
    // [u8; * N] with the whole thing, but those don't really exist yet.
    // Most of the alternatives (e.g. .as_bytes() as a const fn) give
    // you a pointer, not an array, which isn't right.

    // TODO: `modules.builtin.modinfo` etc. is missing the prefix (module name)
    (@attribute author, $value:expr) => {
        #[link_section = ".modinfo"]
        #[used]
        pub static AUTHOR_KEY: [u8; 7] = *b"author=";
        #[link_section = ".modinfo"]
        #[used]
        pub static AUTHOR_VALUE: [u8; $value.len()] = *$value;
        #[link_section = ".modinfo"]
        #[used]
        pub static AUTHOR_NUL: [u8; 1] = *b"\0";
    };

    (@attribute description, $value:expr) => {
        #[link_section = ".modinfo"]
        #[used]
        pub static DESCRIPTION_KEY: [u8; 12] = *b"description=";
        #[link_section = ".modinfo"]
        #[used]
        pub static DESCRIPTION_VALUE: [u8; $value.len()] = *$value;
        #[link_section = ".modinfo"]
        #[used]
        pub static DESCRIPTION_NUL: [u8; 1] = *b"\0";
    };

    (@attribute license, $value:expr) => {
        #[link_section = ".modinfo"]
        #[used]
        pub static LICENSE_KEY: [u8; 8] = *b"license=";
        #[link_section = ".modinfo"]
        #[used]
        pub static LICENSE_VALUE: [u8; $value.len()] = *$value;
        #[link_section = ".modinfo"]
        #[used]
        pub static LICENSE_NUL: [u8; 1] = *b"\0";
    };
}

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
