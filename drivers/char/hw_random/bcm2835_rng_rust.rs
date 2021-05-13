// SPDX-License-Identifier: GPL-2.0

//! Broadcom BCM2835 Random Number Generator support.

#![no_std]
#![feature(allocator_api, global_asm)]

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{cstr, platdev};

module! {
    type: RngModule,
    name: b"bcm2835_rng_rust",
    author: b"Rust for Linux Contributors",
    description: b"BCM2835 Random Number Generator (RNG) driver",
    license: b"GPL v2",
}

struct RngModule {
    _pdev: Pin<Box<platdev::Registration>>,
}

impl KernelModule for RngModule {
    fn init() -> Result<Self> {
        let pdev = platdev::Registration::new_pinned(cstr!("bcm2835-rng-rust"), &THIS_MODULE)?;

        Ok(RngModule { _pdev: pdev })
    }
}
