// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample

#![no_std]
#![feature(allocator_api, global_asm)]
#![feature(test)]

use kernel::prelude::*;

module! {
    type: RustMinimal,
    name: b"rust_minimal",
    author: b"Rust for Linux Contributors",
    description: b"Rust minimal sample",
    license: b"GPL v2",
    params: {
    },
}

struct RustMinimal {
    message: String,
}

impl KernelModule for RustMinimal {
    fn init() -> KernelResult<Self> {
        println!("Rust minimal sample (init)");
        println!("Am I built-in? {}", !cfg!(MODULE));

        Ok(RustMinimal {
            message: "on the heap!".to_owned(),
        })
    }
}

impl Drop for RustMinimal {
    fn drop(&mut self) {
        println!("My message is {}", self.message);
        println!("Rust minimal sample (exit)");
    }
}
