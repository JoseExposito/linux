use core::fmt::Write;

/// Types that can be used for module parameters.
/// Note that displaying the type in `sysfs` will fail if `to_string` returns
/// more than `kernel::PAGE_SIZE` bytes (including an additional null terminator).
pub trait ModuleParam : core::fmt::Display + core::marker::Sized {
    fn try_from_param_arg(arg: &[u8]) -> Option<Self>;

    /// # Safety
    ///
    /// `val` must point to a valid null-terminated string. The `arg` field of
    /// `param` must be an instance of `Self`.
    unsafe extern "C" fn set_param(val: *const crate::c_types::c_char, param: *const crate::bindings::kernel_param) -> crate::c_types::c_int {
        let arg = crate::c_types::c_string_bytes(val);
        match Self::try_from_param_arg(arg) {
            Some(new_value) => {
                let old_value = (*param).__bindgen_anon_1.arg as *mut Self;
                let _ = core::ptr::replace(old_value, new_value);
                0
            }
            None => crate::error::Error::EINVAL.to_kernel_errno()
        }
    }

    /// # Safety
    ///
    /// `buf` must be a buffer of length at least `kernel::PAGE_SIZE` that is
    /// writeable. The `arg` field of `param` must be an instance of `Self`.
    unsafe extern "C" fn get_param(buf: *mut crate::c_types::c_char, param: *const crate::bindings::kernel_param) -> crate::c_types::c_int {
        let slice = core::slice::from_raw_parts_mut(buf as *mut u8, crate::PAGE_SIZE);
        let mut buf = crate::buffer::Buffer::new(slice);
        match write!(buf, "{}\0", *((*param).__bindgen_anon_1.arg as *mut Self)) {
            Err(_) => crate::error::Error::EINVAL.to_kernel_errno(),
            Ok(()) => buf.bytes_written() as crate::c_types::c_int,
        }
    }

    /// # Safety
    ///
    /// The `arg` field of `param` must be an instance of `Self`.
    unsafe extern "C" fn free(arg: *mut crate::c_types::c_void) {
        core::ptr::drop_in_place(arg as *mut Self);
    }
}

/// Trait for parsing integers. Strings begining with `0x`, `0o`, or `0b` are
/// parsed as hex, octal, or binary respectively. Strings beginning with `0`
/// otherwise are parsed as octal. Anything else is parsed as decimal. A
/// leading `+` or `-` is also permitted. Any string parsed by `kstrtol` or
/// `kstrtoul` will be successfully parsed.
trait ParseInt : Sized {
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
    }
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

macro_rules! make_param_ops {
    ($ops:ident, $ty:ident) => {
        impl ModuleParam for $ty {
            fn try_from_param_arg(arg: &[u8]) -> Option<Self> {
                let utf8 = core::str::from_utf8(arg).ok()?;
                <$ty as crate::module_param::ParseInt>::from_str(utf8)
            }
        }

        pub static $ops: crate::bindings::kernel_param_ops = crate::bindings::kernel_param_ops {
            flags: 0,
            set: Some(<$ty as crate::module_param::ModuleParam>::set_param),    
            get: Some(<$ty as crate::module_param::ModuleParam>::get_param),
            free: Some(<$ty as crate::module_param::ModuleParam>::free),
        };
    }
}

make_param_ops!(PARAM_OPS_I8, i8);
make_param_ops!(PARAM_OPS_U8, u8);
make_param_ops!(PARAM_OPS_I16, i16);
make_param_ops!(PARAM_OPS_U16, u16);
make_param_ops!(PARAM_OPS_I32, i32);
make_param_ops!(PARAM_OPS_U32, u32);
make_param_ops!(PARAM_OPS_I64, i64);
make_param_ops!(PARAM_OPS_U64, u64);
make_param_ops!(PARAM_OPS_ISIZE, isize);
make_param_ops!(PARAM_OPS_USIZE, usize);
