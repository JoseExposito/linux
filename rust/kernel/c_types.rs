// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]

#[cfg(any(target_arch = "arm", target_arch = "x86"))]
mod c {
    pub type c_void = core::ffi::c_void;

    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;

    pub type c_short = i16;
    pub type c_ushort = u16;

    pub type c_int = i32;
    pub type c_uint = u32;

    pub type c_long = i32;
    pub type c_ulong = u32;

    pub type c_longlong = i64;
    pub type c_ulonglong = u64;

    pub type c_ssize_t = isize;
    pub type c_size_t = usize;
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
mod c {
    pub type c_void = core::ffi::c_void;

    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;

    pub type c_short = i16;
    pub type c_ushort = u16;

    pub type c_int = i32;
    pub type c_uint = u32;

    pub type c_long = i64;
    pub type c_ulong = u64;

    pub type c_longlong = i64;
    pub type c_ulonglong = u64;

    pub type c_ssize_t = isize;
    pub type c_size_t = usize;
}

pub use c::*;

/// Reads string until null byte is reached and returns slice excluding the terminating null.
///
/// SAFETY: The data from the pointer until the null terminator must be valid for reads and
/// not mutated for all of `'a`. The length of the string must also be less than `isize::MAX`.
/// See the documentation on [`from_raw_parts`](https://doc.rust-lang.org/core/slice/fn.from_raw_parts.html)
/// for further details on safety of converting a pointer to a slice.
pub unsafe fn c_string_bytes<'a>(ptr: *const crate::c_types::c_char) -> &'a [u8] {
    let length = crate::bindings::strlen(ptr) as usize;
    &core::slice::from_raw_parts(ptr as *const u8, length)
}
