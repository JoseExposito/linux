// SPDX-License-Identifier: GPL-2.0

#![no_std]
#![feature(allocator_api, global_asm)]
#![feature(test)]

use kernel::prelude::*;

module! {
    type: RustExample3,
    name: b"rust_example_3",
    author: b"Rust for Linux Contributors",
    description: b"An example kernel module written in Rust",
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
    },
}

struct RustExample3 {
    message: String,
}

impl KernelModule for RustExample3 {
    fn init() -> KernelResult<Self> {
        println!("[3] Rust Example (init)");
        println!("[3] Am I built-in? {}", !cfg!(MODULE));
        {
            let lock = THIS_MODULE.kernel_param_lock();
            println!("[3] Parameters:");
            println!("[3]   my_bool:    {}", my_bool.read());
            println!("[3]   my_i32:     {}", my_i32.read(&lock));
            println!(
                "[3]   my_str:     {}",
                core::str::from_utf8(my_str.read(&lock))?
            );
            println!("[3]   my_usize:   {}", my_usize.read(&lock));
        }

        // Including this large variable on the stack will trigger
        // stack probing on the supported archs.
        // This will verify that stack probing does not lead to
        // any errors if we need to link `__rust_probestack`.
        let x: [u64; 1028] = core::hint::black_box([5; 1028]);
        println!("Large array has length: {}", x.len());

        Ok(RustExample3 {
            message: "on the heap!".to_owned(),
        })
    }
}

impl Drop for RustExample3 {
    fn drop(&mut self) {
        println!("[3] My message is {}", self.message);
        println!("[3] Rust Example (exit)");
    }
}

// XXX: Only for GitHub -- do not commit into mainline
