// SPDX-License-Identifier: GPL-2.0

#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;

module! {
    type: RustExample4,
    name: b"rust_example_4",
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

struct RustExample4 {
    message: String,
}

impl KernelModule for RustExample4 {
    fn init() -> KernelResult<Self> {
        println!("[4] Rust Example (init)");
        println!("[4] Am I built-in? {}", !cfg!(MODULE));
        println!("[4] Parameters:");
        println!("[4]   my_bool:  {}", my_bool.read());
        println!("[4]   my_i32:   {}", my_i32.read());
        Ok(RustExample4 {
            message: "on the heap!".to_owned(),
        })
    }
}

impl Drop for RustExample4 {
    fn drop(&mut self) {
        println!("[4] My message is {}", self.message);
        println!("[4] Rust Example (exit)");
    }
}

// XXX: Only for GitHub -- do not commit into mainline
