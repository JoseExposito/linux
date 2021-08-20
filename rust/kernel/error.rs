// SPDX-License-Identifier: GPL-2.0

//! Kernel errors.
//!
//! C header: [`include/uapi/asm-generic/errno-base.h`](../../../include/uapi/asm-generic/errno-base.h)

use crate::str::CStr;
use crate::{bindings, c_types};
use alloc::{alloc::AllocError, collections::TryReserveError};
use core::convert::From;
use core::fmt;
use core::num::TryFromIntError;
use core::str::{self, Utf8Error};

macro_rules! declare_err {
    ($err:tt) => {
        pub const $err: Self = Error(-(bindings::$err as i32));
    };
    ($err:tt, $($doc:expr),+) => {
        $(
        #[doc = $doc]
        )*
        pub const $err: Self = Error(-(bindings::$err as i32));
    };
}

/// Generic integer kernel error.
///
/// The kernel defines a set of integer generic error codes based on C and
/// POSIX ones. These codes may have a more specific meaning in some contexts.
///
/// # Invariants
///
/// The value is a valid `errno` (i.e. `>= -MAX_ERRNO && < 0`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Error(c_types::c_int);

impl Error {
    declare_err!(EPERM, "Operation not permitted.");

    declare_err!(ENOENT, "No such file or directory.");

    declare_err!(ESRCH, "No such process.");

    declare_err!(EINTR, "Interrupted system call.");

    declare_err!(EIO, "I/O error.");

    declare_err!(ENXIO, "No such device or address.");

    declare_err!(E2BIG, "Argument list too long.");

    declare_err!(ENOEXEC, "Exec format error.");

    declare_err!(EBADF, "Bad file number.");

    declare_err!(ECHILD, "Exec format error.");

    declare_err!(EAGAIN, "Try again.");

    declare_err!(ENOMEM, "Out of memory.");

    declare_err!(EACCES, "Permission denied.");

    declare_err!(EFAULT, "Bad address.");

    declare_err!(ENOTBLK, "Block device required.");

    declare_err!(EBUSY, "Device or resource busy.");

    declare_err!(EEXIST, "File exists.");

    declare_err!(EXDEV, "Cross-device link.");

    declare_err!(ENODEV, "No such device.");

    declare_err!(ENOTDIR, "Not a directory.");

    declare_err!(EISDIR, "Is a directory.");

    declare_err!(EINVAL, "Invalid argument.");

    declare_err!(ENFILE, "File table overflow.");

    declare_err!(EMFILE, "Too many open files.");

    declare_err!(ENOTTY, "Not a typewriter.");

    declare_err!(ETXTBSY, "Text file busy.");

    declare_err!(EFBIG, "File too large.");

    declare_err!(ENOSPC, "No space left on device.");

    declare_err!(ESPIPE, "Illegal seek.");

    declare_err!(EROFS, "Read-only file system.");

    declare_err!(EMLINK, "Too many links.");

    declare_err!(EPIPE, "Broken pipe.");

    declare_err!(EDOM, "Math argument out of domain of func.");

    declare_err!(ERANGE, "Math result not representable.");

    declare_err!(EDEADLK, "Resource deadlock would occur");

    declare_err!(ENAMETOOLONG, "File name too long");

    declare_err!(EADDRNOTAVAIL, "Cannot assign requested address.");

    declare_err!(ERESTARTSYS, "Restart the system call.");

    /// Creates an [`Error`] from a kernel error code.
    ///
    /// It is a bug to pass an out-of-range `errno`. `EINVAL` would
    /// be returned in such a case.
    pub(crate) fn from_kernel_errno(errno: c_types::c_int) -> Error {
        if errno < -(bindings::MAX_ERRNO as i32) || errno >= 0 {
            // TODO: make it a `WARN_ONCE` once available.
            crate::pr_warn!(
                "attempted to create `Error` with out of range `errno`: {}",
                errno
            );
            return Error::EINVAL;
        }

        // INVARIANT: the check above ensures the type invariant
        // will hold.
        Error(errno)
    }

    /// Creates an [`Error`] from a kernel error code.
    ///
    /// # Safety
    ///
    /// `errno` must be within error code range (i.e. `>= -MAX_ERRNO && < 0`).
    pub(crate) unsafe fn from_kernel_errno_unchecked(errno: c_types::c_int) -> Error {
        // INVARIANT: the contract ensures the type invariant
        // will hold.
        Error(errno)
    }

    /// Returns the kernel error code.
    pub fn to_kernel_errno(self) -> c_types::c_int {
        self.0
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        extern "C" {
            fn rust_helper_errname(err: c_types::c_int) -> *const c_types::c_char;
        }
        // SAFETY: FFI call.
        let name = unsafe { rust_helper_errname(-self.0) };

        if name.is_null() {
            // Print out number if no name can be found.
            return f.debug_tuple("Error").field(&-self.0).finish();
        }

        // SAFETY: `'static` string from C, and is not NULL.
        let cstr = unsafe { CStr::from_char_ptr(name) };
        // SAFETY: These strings are ASCII-only.
        let str = unsafe { str::from_utf8_unchecked(cstr) };
        f.debug_tuple(str).finish()
    }
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Error {
        Error::EINVAL
    }
}

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Error {
        Error::EINVAL
    }
}

impl From<TryReserveError> for Error {
    fn from(_: TryReserveError) -> Error {
        Error::ENOMEM
    }
}

/// A [`Result`] with an [`Error`] error type.
///
/// To be used as the return type for functions that may fail.
///
/// # Error codes in C and Rust
///
/// In C, it is common that functions indicate success or failure through
/// their return value; modifying or returning extra data through non-`const`
/// pointer parameters. In particular, in the kernel, functions that may fail
/// typically return an `int` that represents a generic error code. We model
/// those as [`Error`].
///
/// In Rust, it is idiomatic to model functions that may fail as returning
/// a [`Result`]. Since in the kernel many functions return an error code,
/// [`Result`] is a type alias for a [`core::result::Result`] that uses
/// [`Error`] as its error type.
///
/// Note that even if a function does not return anything when it succeeds,
/// it should still be modeled as returning a `Result` rather than
/// just an [`Error`].
pub type Result<T = ()> = core::result::Result<T, Error>;

impl From<AllocError> for Error {
    fn from(_: AllocError) -> Error {
        Error::ENOMEM
    }
}

// # Invariant: `-bindings::MAX_ERRNO` fits in an `i16`.
crate::static_assert!(bindings::MAX_ERRNO <= -(i16::MIN as i32) as u32);

#[doc(hidden)]
pub fn from_kernel_result_helper<T>(r: Result<T>) -> T
where
    T: From<i16>,
{
    match r {
        Ok(v) => v,
        // NO-OVERFLOW: negative `errno`s are no smaller than `-bindings::MAX_ERRNO`,
        // `-bindings::MAX_ERRNO` fits in an `i16` as per invariant above,
        // therefore a negative `errno` always fits in an `i16` and will not overflow.
        Err(e) => T::from(e.to_kernel_errno() as i16),
    }
}

/// Transforms a [`crate::error::Result<T>`] to a kernel C integer result.
///
/// This is useful when calling Rust functions that return [`crate::error::Result<T>`]
/// from inside `extern "C"` functions that need to return an integer
/// error result.
///
/// `T` should be convertible to an `i16` via `From<i16>`.
///
/// # Examples
///
/// ```ignore
/// # use kernel::from_kernel_result;
/// # use kernel::c_types;
/// # use kernel::bindings;
/// unsafe extern "C" fn probe_callback(
///     pdev: *mut bindings::platform_device,
/// ) -> c_types::c_int {
///     from_kernel_result! {
///         let ptr = devm_alloc(pdev)?;
///         rust_helper_platform_set_drvdata(pdev, ptr);
///         Ok(0)
///     }
/// }
/// ```
#[macro_export]
macro_rules! from_kernel_result {
    ($($tt:tt)*) => {{
        $crate::error::from_kernel_result_helper((|| {
            $($tt)*
        })())
    }};
}

/// Transform a kernel "error pointer" to a normal pointer.
///
/// Some kernel C API functions return an "error pointer" which optionally
/// embeds an `errno`. Callers are supposed to check the returned pointer
/// for errors. This function performs the check and converts the "error pointer"
/// to a normal pointer in an idiomatic fashion.
///
/// # Examples
///
/// ```ignore
/// # use kernel::prelude::*;
/// # use kernel::from_kernel_err_ptr;
/// # use kernel::c_types;
/// # use kernel::bindings;
/// fn devm_platform_ioremap_resource(
///     pdev: &mut PlatformDevice,
///     index: u32,
/// ) -> Result<*mut c_types::c_void> {
///     // SAFETY: FFI call.
///     unsafe {
///         from_kernel_err_ptr(bindings::devm_platform_ioremap_resource(
///             pdev.to_ptr(),
///             index,
///         ))
///     }
/// }
/// ```
// TODO: remove `dead_code` marker once an in-kernel client is available.
#[allow(dead_code)]
pub(crate) fn from_kernel_err_ptr<T>(ptr: *mut T) -> Result<*mut T> {
    extern "C" {
        #[allow(improper_ctypes)]
        fn rust_helper_is_err(ptr: *const c_types::c_void) -> bool;

        #[allow(improper_ctypes)]
        fn rust_helper_ptr_err(ptr: *const c_types::c_void) -> c_types::c_long;
    }

    // CAST: casting a pointer to `*const c_types::c_void` is always valid.
    let const_ptr: *const c_types::c_void = ptr.cast();
    // SAFETY: the FFI function does not deref the pointer.
    if unsafe { rust_helper_is_err(const_ptr) } {
        // SAFETY: the FFI function does not deref the pointer.
        let err = unsafe { rust_helper_ptr_err(const_ptr) };
        // CAST: if `rust_helper_is_err()` returns `true`,
        // then `rust_helper_ptr_err()` is guaranteed to return a
        // negative value greater-or-equal to `-bindings::MAX_ERRNO`,
        // which always fits in an `i16`, as per the invariant above.
        // And an `i16` always fits in an `i32`. So casting `err` to
        // an `i32` can never overflow, and is always valid.
        //
        // SAFETY: `rust_helper_is_err()` ensures `err` is a
        // negative value greater-or-equal to `-bindings::MAX_ERRNO`
        return Err(unsafe { Error::from_kernel_errno_unchecked(err as i32) });
    }
    Ok(ptr)
}
