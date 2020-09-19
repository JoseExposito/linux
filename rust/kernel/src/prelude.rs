// SPDX-License-Identifier: GPL-2.0

//! The `kernel` prelude

pub use alloc::{
    string::String,
    borrow::ToOwned,
};

pub use module::module;

pub use super::{
    println,
    KernelResult,
    KernelModule,
};

