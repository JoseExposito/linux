// SPDX-License-Identifier: GPL-2.0

#![no_std]
#![feature(global_asm)]

use kernel::prelude::*;

struct RustExample {
    message: String,
}

impl KernelModule for RustExample {
    fn init() -> KernelResult<Self> {
        println!("Rust Example (init)");
        println!("Am I built-in? {}", !cfg!(MODULE));
        Ok(RustExample {
            message: "on the heap!".to_owned(),
        })
    }
}

impl Drop for RustExample {
    fn drop(&mut self) {
        println!("My message is {}", self.message);
        println!("Rust Example (exit)");
    }
}

kernel_module!(
    RustExample,
    author: b"Rust for Linux Contributors",
    description: b"An example kernel module written in Rust",
    license: b"GPL v2"
);
