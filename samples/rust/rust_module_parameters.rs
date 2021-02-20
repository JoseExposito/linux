// SPDX-License-Identifier: GPL-2.0

//! Rust module parameters sample

#![no_std]
#![feature(allocator_api, global_asm)]
#![feature(test)]

use kernel::prelude::*;

module! {
    type: RustModuleParameters,
    name: b"rust_module_parameters",
    author: b"Rust for Linux Contributors",
    description: b"Rust module parameters sample",
    license: b"GPL v2",
    params: {
        my_bool: bool {
            default: true,
            permissions: 0,
            description: b"Example of bool",
        },
        my_i32: i32 {
            default: 42,
            permissions: 0o644,
            description: b"Example of i32",
        },
        my_str: str {
            default: b"default str val",
            permissions: 0o644,
            description: b"Example of a string param",
        },
        my_usize: usize {
            default: 42,
            permissions: 0o644,
            description: b"Example of usize",
        },
        my_array: ArrayParam<i32, 3> {
            default: [0, 1],
            permissions: 0,
            description: b"Example of array",
        },
    },
}

struct RustModuleParameters;

impl KernelModule for RustModuleParameters {
    fn init() -> KernelResult<Self> {
        info!("Rust module parameters sample (init)");

        {
            let lock = THIS_MODULE.kernel_param_lock();
            info!("Parameters:");
            info!("  my_bool:    {}", my_bool.read());
            info!("  my_i32:     {}", my_i32.read(&lock));
            info!(
                "  my_str:     {}",
                core::str::from_utf8(my_str.read(&lock))?
            );
            info!("  my_usize:   {}", my_usize.read(&lock));
            info!("  my_array:   {:?}", my_array.read());
        }

        Ok(RustModuleParameters)
    }
}

impl Drop for RustModuleParameters {
    fn drop(&mut self) {
        info!("Rust module parameters sample (exit)");
    }
}
