// SPDX-License-Identifier: GPL-2.0

//! Types for module parameters.
//!
//! C header: [`include/linux/moduleparam.h`](../../../include/linux/moduleparam.h)

use core::fmt::Write;

/// Types that can be used for module parameters.
///
/// Note that displaying the type in `sysfs` will fail if
/// [`alloc::string::ToString::to_string`] (as implemented through the
/// [`core::fmt::Display`] trait) writes more than `kernel::PAGE_SIZE`
/// bytes (including an additional null terminator).
pub trait ModuleParam: core::fmt::Display + core::marker::Sized {
    /// Whether the parameter is allowed to be set without an argument.
    ///
    /// Setting this to `true` allows the parameter to be passed without an
    /// argument (e.g. just `module.param` instead of `module.param=foo`).
    const NOARG_ALLOWED: bool;

    /// Convert a parameter argument into the parameter value.
    ///
    /// `None` should be returned when parsing of the argument fails.
    /// `arg == None` indicates that the parameter was passed without an
    /// argument. If `NOARG_ALLOWED` is set to `false` then `arg` is guaranteed
    /// to always be `Some(_)`.
    fn try_from_param_arg(arg: Option<&[u8]>) -> Option<Self>;

    /// Set the module parameter from a string.
    ///
    /// Used to set the parameter value when loading the module or when set
    /// through `sysfs`.
    ///
    /// # Safety
    ///
    /// If `val` is non-null then it must point to a valid null-terminated
    /// string. The `arg` field of `param` must be an instance of `Self`.
    unsafe extern "C" fn set_param(
        val: *const crate::c_types::c_char,
        param: *const crate::bindings::kernel_param,
    ) -> crate::c_types::c_int {
        let arg = if val.is_null() {
            None
        } else {
            Some(crate::c_types::c_string_bytes(val))
        };
        match Self::try_from_param_arg(arg) {
            Some(new_value) => {
                let old_value = (*param).__bindgen_anon_1.arg as *mut Self;
                let _ = core::ptr::replace(old_value, new_value);
                0
            }
            None => crate::error::Error::EINVAL.to_kernel_errno(),
        }
    }

    /// Write a string representation of the current parameter value to `buf`.
    ///
    /// Used for displaying the current parameter value in `sysfs`.
    ///
    /// # Safety
    ///
    /// `buf` must be a buffer of length at least `kernel::PAGE_SIZE` that is
    /// writeable. The `arg` field of `param` must be an instance of `Self`.
    unsafe extern "C" fn get_param(
        buf: *mut crate::c_types::c_char,
        param: *const crate::bindings::kernel_param,
    ) -> crate::c_types::c_int {
        let slice = core::slice::from_raw_parts_mut(buf as *mut u8, crate::PAGE_SIZE);
        let mut buf = crate::buffer::Buffer::new(slice);
        match write!(buf, "{}\0", *((*param).__bindgen_anon_1.arg as *mut Self)) {
            Err(_) => crate::error::Error::EINVAL.to_kernel_errno(),
            Ok(()) => buf.bytes_written() as crate::c_types::c_int,
        }
    }

    /// Drop the parameter.
    ///
    /// Called when unloading a module.
    ///
    /// # Safety
    ///
    /// The `arg` field of `param` must be an instance of `Self`.
    unsafe extern "C" fn free(arg: *mut crate::c_types::c_void) {
        core::ptr::drop_in_place(arg as *mut Self);
    }
}

/// Trait for parsing integers.
///
/// Strings begining with `0x`, `0o`, or `0b` are parsed as hex, octal, or
/// binary respectively. Strings beginning with `0` otherwise are parsed as
/// octal. Anything else is parsed as decimal. A leading `+` or `-` is also
/// permitted. Any string parsed by [`kstrtol()`] or [`kstrtoul()`] will be
/// successfully parsed.
///
/// [`kstrtol()`]: https://www.kernel.org/doc/html/latest/core-api/kernel-api.html#c.kstrtol
/// [`kstrtoul()`]: https://www.kernel.org/doc/html/latest/core-api/kernel-api.html#c.kstrtoul
trait ParseInt: Sized {
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, core::num::ParseIntError>;
    fn checked_neg(self) -> Option<Self>;

    fn from_str_unsigned(src: &str) -> Result<Self, core::num::ParseIntError> {
        let (radix, digits) = if let Some(n) = src.strip_prefix("0x") {
            (16, n)
        } else if let Some(n) = src.strip_prefix("0X") {
            (16, n)
        } else if let Some(n) = src.strip_prefix("0o") {
            (8, n)
        } else if let Some(n) = src.strip_prefix("0O") {
            (8, n)
        } else if let Some(n) = src.strip_prefix("0b") {
            (2, n)
        } else if let Some(n) = src.strip_prefix("0B") {
            (2, n)
        } else if src.starts_with('0') {
            (8, src)
        } else {
            (10, src)
        };
        Self::from_str_radix(digits, radix)
    }

    fn from_str(src: &str) -> Option<Self> {
        match src.bytes().next() {
            None => None,
            Some(b'-') => Self::from_str_unsigned(&src[1..]).ok()?.checked_neg(),
            Some(b'+') => Some(Self::from_str_unsigned(&src[1..]).ok()?),
            Some(_) => Some(Self::from_str_unsigned(src).ok()?),
        }
    }
}

macro_rules! impl_parse_int {
    ($ty:ident) => {
        impl ParseInt for $ty {
            fn from_str_radix(src: &str, radix: u32) -> Result<Self, core::num::ParseIntError> {
                $ty::from_str_radix(src, radix)
            }
            fn checked_neg(self) -> Option<Self> {
                self.checked_neg()
            }
        }
    };
}

impl_parse_int!(i8);
impl_parse_int!(u8);
impl_parse_int!(i16);
impl_parse_int!(u16);
impl_parse_int!(i32);
impl_parse_int!(u32);
impl_parse_int!(i64);
impl_parse_int!(u64);
impl_parse_int!(isize);
impl_parse_int!(usize);

macro_rules! impl_module_param {
    ($ty:ident) => {
        impl ModuleParam for $ty {
            const NOARG_ALLOWED: bool = false;

            fn try_from_param_arg(arg: Option<&[u8]>) -> Option<Self> {
                let bytes = arg?;
                let utf8 = core::str::from_utf8(bytes).ok()?;
                <$ty as crate::module_param::ParseInt>::from_str(utf8)
            }
        }
    };
}

#[macro_export]
/// Generate a static [`kernel_param_ops`](../../../include/linux/moduleparam.h) struct.
///
/// # Example
/// ```rust
/// make_param_ops!(
///     /// Documentation for new param ops.
///     PARAM_OPS_MYTYPE, // Name for the static.
///     MyType            // A type which implements [`ModuleParam`].
/// );
/// ```
macro_rules! make_param_ops {
    ($ops:ident, $ty:ident) => {
        make_param_ops!(
            #[doc=""]
            $ops,
            $ty
        );
    };
    ($(#[$meta:meta])* $ops:ident, $ty:ident) => {
        $(#[$meta])*
        ///
        /// Static [`kernel_param_ops`](../../../include/linux/moduleparam.h)
        /// struct generated by [`make_param_ops`].
        pub static $ops: crate::bindings::kernel_param_ops = crate::bindings::kernel_param_ops {
            flags: if <$ty as crate::module_param::ModuleParam>::NOARG_ALLOWED {
                crate::bindings::KERNEL_PARAM_OPS_FL_NOARG
            } else {
                0
            },
            set: Some(<$ty as crate::module_param::ModuleParam>::set_param),
            get: Some(<$ty as crate::module_param::ModuleParam>::get_param),
            free: Some(<$ty as crate::module_param::ModuleParam>::free),
        };
    };
}

impl_module_param!(i8);
impl_module_param!(u8);
impl_module_param!(i16);
impl_module_param!(u16);
impl_module_param!(i32);
impl_module_param!(u32);
impl_module_param!(i64);
impl_module_param!(u64);
impl_module_param!(isize);
impl_module_param!(usize);

make_param_ops!(
    /// Rust implementation of [`kernel_param_ops`](../../../include/linux/moduleparam.h)
    /// for [`i8`].
    PARAM_OPS_I8,
    i8
);
make_param_ops!(
    /// Rust implementation of [`kernel_param_ops`](../../../include/linux/moduleparam.h)
    /// for [`u8`].
    PARAM_OPS_U8,
    u8
);
make_param_ops!(
    /// Rust implementation of [`kernel_param_ops`](../../../include/linux/moduleparam.h)
    /// for [`i16`].
    PARAM_OPS_I16,
    i16
);
make_param_ops!(
    /// Rust implementation of [`kernel_param_ops`](../../../include/linux/moduleparam.h)
    /// for [`u16`].
    PARAM_OPS_U16,
    u16
);
make_param_ops!(
    /// Rust implementation of [`kernel_param_ops`](../../../include/linux/moduleparam.h)
    /// for [`i32`].
    PARAM_OPS_I32,
    i32
);
make_param_ops!(
    /// Rust implementation of [`kernel_param_ops`](../../../include/linux/moduleparam.h)
    /// for [`u32`].
    PARAM_OPS_U32,
    u32
);
make_param_ops!(
    /// Rust implementation of [`kernel_param_ops`](../../../include/linux/moduleparam.h)
    /// for [`i64`].
    PARAM_OPS_I64,
    i64
);
make_param_ops!(
    /// Rust implementation of [`kernel_param_ops`](../../../include/linux/moduleparam.h)
    /// for [`u64`].
    PARAM_OPS_U64,
    u64
);
make_param_ops!(
    /// Rust implementation of [`kernel_param_ops`](../../../include/linux/moduleparam.h)
    /// for [`isize`].
    PARAM_OPS_ISIZE,
    isize
);
make_param_ops!(
    /// Rust implementation of [`kernel_param_ops`](../../../include/linux/moduleparam.h)
    /// for [`usize`].
    PARAM_OPS_USIZE,
    usize
);

impl ModuleParam for bool {
    const NOARG_ALLOWED: bool = true;

    fn try_from_param_arg(arg: Option<&[u8]>) -> Option<Self> {
        match arg {
            None => Some(true),
            Some(b"y") | Some(b"Y") | Some(b"1") | Some(b"true") => Some(true),
            Some(b"n") | Some(b"N") | Some(b"0") | Some(b"false") => Some(false),
            _ => None,
        }
    }
}

make_param_ops!(
    /// Rust implementation of [`kernel_param_ops`](../../../include/linux/moduleparam.h)
    /// for [`bool`].
    PARAM_OPS_BOOL,
    bool
);
