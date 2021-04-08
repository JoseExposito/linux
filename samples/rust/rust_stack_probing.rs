// SPDX-License-Identifier: GPL-2.0

//! Rust stack probing sample

#![no_std]
#![feature(allocator_api, global_asm)]
#![feature(test)]

use kernel::prelude::*;

module! {
    type: RustStackProbing,
    name: b"rust_stack_probing",
    author: b"Rust for Linux Contributors",
    description: b"Rust stack probing sample",
    license: b"GPL v2",
    params: {
    },
}

struct RustStackProbing;

impl KernelModule for RustStackProbing {
    fn init() -> KernelResult<Self> {
        println!("Rust stack probing sample (init)");

        // Including this large variable on the stack will trigger
        // stack probing on the supported archs.
        // This will verify that stack probing does not lead to
        // any errors if we need to link `__rust_probestack`.
        let x: [u64; 514] = core::hint::black_box([5; 514]);
        println!("Large array has length: {}", x.len());

        Ok(RustStackProbing)
    }
}

impl Drop for RustStackProbing {
    fn drop(&mut self) {
        println!("Rust stack probing sample (exit)");
    }
}
