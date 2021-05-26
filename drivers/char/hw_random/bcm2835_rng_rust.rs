// SPDX-License-Identifier: GPL-2.0

//! Broadcom BCM2835 Random Number Generator support.

#![no_std]
#![feature(allocator_api, global_asm)]

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::of::OfMatchTable;
use kernel::platdev::PlatformDriver;
use kernel::prelude::*;
use kernel::{c_str, platdev};

module! {
    type: RngModule,
    name: b"bcm2835_rng_rust",
    author: b"Rust for Linux Contributors",
    description: b"BCM2835 Random Number Generator (RNG) driver",
    license: b"GPL v2",
}

struct RngDriver;

impl PlatformDriver for RngDriver {
    fn probe(device_id: i32) -> Result {
        pr_info!("probing discovered hwrng with id {}\n", device_id);
        Ok(())
    }

    fn remove(device_id: i32) -> Result {
        pr_info!("removing hwrng with id {}\n", device_id);
        Ok(())
    }
}

struct RngModule {
    _pdev: Pin<Box<platdev::Registration>>,
}

impl KernelModule for RngModule {
    fn init() -> Result<Self> {
        let of_match_tbl = OfMatchTable::new(&c_str!("brcm,bcm2835-rng"))?;

        let pdev = platdev::Registration::new_pinned::<RngDriver>(
            c_str!("bcm2835-rng-rust"),
            Some(of_match_tbl),
            &THIS_MODULE,
        )?;

        Ok(RngModule { _pdev: pdev })
    }
}
