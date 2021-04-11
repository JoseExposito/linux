// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample

#![no_std]
#![feature(allocator_api, global_asm)]

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
        info!("Rust minimal sample (init)");
        info!("Am I built-in? {}", !cfg!(MODULE));

        Ok(RustMinimal {
            message: "on the heap!".to_owned(),
        })
    }
}

impl Drop for RustMinimal {
    fn drop(&mut self) {
        info!("My message is {}", self.message);
        info!("Rust minimal sample (exit)");
    }
}
