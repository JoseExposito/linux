// SPDX-License-Identifier: GPL-2.0

//! Rust synchronisation primitives sample

#![no_std]
#![feature(allocator_api, global_asm)]
#![feature(test)]

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{
    condvar_init, mutex_init, spinlock_init,
    sync::{CondVar, Mutex, SpinLock},
};

module! {
    type: RustSync,
    name: b"rust_sync",
    author: b"Rust for Linux Contributors",
    description: b"Rust synchronisation primitives sample",
    license: b"GPL v2",
    params: {
    },
}

struct RustSync;

impl KernelModule for RustSync {
    fn init() -> KernelResult<Self> {
        println!("Rust synchronisation primitives sample (init)");

        // Test mutexes.
        {
            // SAFETY: `init` is called below.
            let data = Pin::from(Box::try_new(unsafe { Mutex::new(0) })?);
            mutex_init!(data.as_ref(), "RustSync::init::data1");
            *data.lock() = 10;
            println!("Value: {}", *data.lock());

            // SAFETY: `init` is called below.
            let cv = Pin::from(Box::try_new(unsafe { CondVar::new() })?);
            condvar_init!(cv.as_ref(), "RustSync::init::cv1");
            {
                let mut guard = data.lock();
                while *guard != 10 {
                    let _ = cv.wait(&mut guard);
                }
            }
            cv.notify_one();
            cv.notify_all();
            cv.free_waiters();
        }

        // Test spinlocks.
        {
            // SAFETY: `init` is called below.
            let data = Pin::from(Box::try_new(unsafe { SpinLock::new(0) })?);
            spinlock_init!(data.as_ref(), "RustSync::init::data2");
            *data.lock() = 10;
            println!("Value: {}", *data.lock());

            // SAFETY: `init` is called below.
            let cv = Pin::from(Box::try_new(unsafe { CondVar::new() })?);
            condvar_init!(cv.as_ref(), "RustSync::init::cv2");
            {
                let mut guard = data.lock();
                while *guard != 10 {
                    let _ = cv.wait(&mut guard);
                }
            }
            cv.notify_one();
            cv.notify_all();
            cv.free_waiters();
        }

        Ok(RustSync)
    }
}

impl Drop for RustSync {
    fn drop(&mut self) {
        println!("Rust synchronisation primitives sample (exit)");
    }
}
