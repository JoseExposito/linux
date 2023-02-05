// SPDX-License-Identifier: GPL-2.0

//! HID subsystem Rust abstractions.
//!
//! C header: [`include/linux/hid.h`](../../../include/linux/hid.h).
//!
//! Copyright (c) 2023 José Expósito <jose.exposito89@gmail.com>

pub mod driver;

pub use self::driver::*;
