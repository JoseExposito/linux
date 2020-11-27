// SPDX-License-Identifier: GPL-2.0

#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;

module! {
    type: RustExample2,
    name: b"rust_example_2",
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
    },
}

struct RustExample2 {
    message: String,
}

impl KernelModule for RustExample2 {
    fn init() -> KernelResult<Self> {
        println!("[2] Rust Example (init)");
        println!("[2] Am I built-in? {}", !cfg!(MODULE));
        println!("[2] Parameters:");
        println!("[2]   my_bool:  {}", my_bool.read());
        println!("[2]   my_i32:   {}", my_i32.read());
        Ok(RustExample2 {
            message: "on the heap!".to_owned(),
        })
    }
}

impl Drop for RustExample2 {
    fn drop(&mut self) {
        println!("[2] My message is {}", self.message);
        println!("[2] Rust Example (exit)");
    }
}

// XXX: Only for GitHub -- do not commit into mainline
