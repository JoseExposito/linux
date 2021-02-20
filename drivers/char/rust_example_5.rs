// SPDX-License-Identifier: GPL-2.0

#![no_std]
#![feature(allocator_api, global_asm)]
#![feature(test)]

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{cstr, file_operations::FileOperations, chrdev};

module! {
    type: RustExample,
    name: b"rust_exampl_5",
    author: b"Rust for Linux Contributors",
    description: b"An example kernel module written in Rust",
    license: b"GPL v2",
}

struct RustFile;

impl FileOperations for RustFile {
    type Wrapper = Box<Self>;

    fn open() -> KernelResult<Self::Wrapper> {
        println!("rust file was opened!");
        Ok(Box::try_new(Self)?)
    }
}

struct RustExample5 {
    _dev: Pin<Box<chrdev::Registration<{2}>>>,
}

impl KernelModule for RustExample5 {
    fn init() -> KernelResult<Self> {
        let reg = chrdev::Registration::new_pinned(cstr!("rust_chrdev"), 0, THIS_MODULE)?;
        reg.register::<RustFile>()?;
        reg.register::<RustFile>()?;
        Ok(RustExample5 {
            _dev: reg,
        })
    }
}
