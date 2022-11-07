// SPDX-License-Identifier: GPL-2.0

//! Build-time error.
//!
//! This crate provides a function `build_error`, which will panic in
//! compile-time if executed in const context, and will cause a build error
//! if not executed at compile time and the optimizer does not optimise away the
//! call.
//!
//! It is used by `build_assert!` in the kernel crate, allowing checking of
//! conditions that could be checked statically, but could not be enforced in
//! Rust yet (e.g. perform some checks in const functions, but those
//! functions could still be called in the runtime).

#![no_std]

/// Panics if executed in const context, or triggers a build error if not.
#[inline(never)]
#[cold]
#[export_name = "rust_build_error"]
#[track_caller]
pub const fn build_error(msg: &'static str) -> ! {
    panic!("{}", msg);
}
