// SPDX-License-Identifier: GPL-2.0

//! Rust example module

#![no_std]
#![feature(allocator_api, global_asm)]
#![feature(test)]

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{chrdev, cstr, file_operations::FileOperations, miscdev};

module! {
    type: RustExample,
    name: b"rust_example",
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
    },
}

struct RustFile;

impl FileOperations for RustFile {
    type Wrapper = Box<Self>;

    fn open() -> KernelResult<Self::Wrapper> {
        println!("rust file was opened!");
        Ok(Box::try_new(Self)?)
    }
}

struct RustExample {
    message: String,
    _chrdev: Pin<Box<chrdev::Registration<2>>>,
    _dev: Pin<Box<miscdev::Registration>>,
}

impl KernelModule for RustExample {
    fn init() -> KernelResult<Self> {
        println!("Rust Example (init)");
        println!("Am I built-in? {}", !cfg!(MODULE));
        {
            let lock = THIS_MODULE.kernel_param_lock();
            println!("Parameters:");
            println!("  my_bool:    {}", my_bool.read());
            println!("  my_i32:     {}", my_i32.read(&lock));
            println!(
                "  my_str:     {}",
                core::str::from_utf8(my_str.read(&lock))?
            );
        }

        // Including this large variable on the stack will trigger
        // stack probing on the supported archs.
        // This will verify that stack probing does not lead to
        // any errors if we need to link `__rust_probestack`.
        let x: [u64; 1028] = core::hint::black_box([5; 1028]);
        println!("Large array has length: {}", x.len());

        let mut chrdev_reg =
            chrdev::Registration::new_pinned(cstr!("rust_chrdev"), 0, &THIS_MODULE)?;
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustExample {
            message: "on the heap!".to_owned(),
            _dev: miscdev::Registration::new_pinned::<RustFile>(cstr!("rust_miscdev"), None)?,
            _chrdev: chrdev_reg,
        })
    }
}

impl Drop for RustExample {
    fn drop(&mut self) {
        println!("My message is {}", self.message);
        println!("Rust Example (exit)");
    }
}
