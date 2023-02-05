// SPDX-License-Identifier: GPL-2.0

//! HID C API bindings mocks.
//!
//! Copyright (c) 2023 José Expósito <jose.exposito89@gmail.com>

#![cfg(CONFIG_KUNIT)]
#![allow(missing_docs)]
#![allow(clippy::missing_safety_doc)]

use crate::{c_str, kunit::in_kunit_test};
pub(crate) use bindings::*;

pub(crate) unsafe fn __hid_register_driver(
    arg1: *mut hid_driver,
    arg2: *mut bindings::module,
    arg3: *const core::ffi::c_char,
) -> core::ffi::c_int {
    if in_kunit_test() {
        -19
    } else {
        unsafe { bindings::__hid_register_driver(arg1, arg2, arg3) }
    }
}

pub(crate) unsafe fn hid_unregister_driver(arg1: *mut hid_driver) {
    if in_kunit_test() {
        let hid = unsafe { &mut *arg1 };
        hid.name = c_str!("unregistered").as_char_ptr() as _;
    } else {
        unsafe { bindings::hid_unregister_driver(arg1) }
    }
}
