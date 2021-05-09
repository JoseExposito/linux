// SPDX-License-Identifier: GPL-2.0

//! Kernel types.
//!
//! C header: [`include/linux/types.h`](../../../../include/linux/types.h)

use core::{ops::Deref, pin::Pin};

use alloc::{boxed::Box, sync::Arc};

use crate::bindings;
use crate::c_types;
use crate::sync::{Ref, RefCounted};

/// Permissions.
///
/// C header: [`include/uapi/linux/stat.h`](../../../../include/uapi/linux/stat.h)
///
/// C header: [`include/linux/stat.h`](../../../../include/linux/stat.h)
pub struct Mode(bindings::umode_t);

impl Mode {
    /// Creates a [`Mode`] from an integer.
    pub fn from_int(m: u16) -> Mode {
        Mode(m)
    }

    /// Returns the mode as an integer.
    pub fn as_int(&self) -> u16 {
        self.0
    }
}

/// Used to convert an object into a raw pointer that represents it.
///
/// It can eventually be converted back into the object. This is used to store objects as pointers
/// in kernel data structures, for example, an implementation of [`FileOperations`] in `struct
/// file::private_data`.
pub trait PointerWrapper {
    /// Returns the raw pointer.
    fn into_pointer(self) -> *const c_types::c_void;

    /// Returns the instance back from the raw pointer.
    ///
    /// # Safety
    ///
    /// The passed pointer must come from a previous call to [`PointerWrapper::into_pointer()`].
    unsafe fn from_pointer(ptr: *const c_types::c_void) -> Self;
}

impl<T> PointerWrapper for Box<T> {
    fn into_pointer(self) -> *const c_types::c_void {
        Box::into_raw(self) as _
    }

    unsafe fn from_pointer(ptr: *const c_types::c_void) -> Self {
        Box::from_raw(ptr as _)
    }
}

impl<T: RefCounted> PointerWrapper for Ref<T> {
    fn into_pointer(self) -> *const c_types::c_void {
        Ref::into_raw(self) as _
    }

    unsafe fn from_pointer(ptr: *const c_types::c_void) -> Self {
        Ref::from_raw(ptr as _)
    }
}

impl<T> PointerWrapper for Arc<T> {
    fn into_pointer(self) -> *const c_types::c_void {
        Arc::into_raw(self) as _
    }

    unsafe fn from_pointer(ptr: *const c_types::c_void) -> Self {
        Arc::from_raw(ptr as _)
    }
}

impl<T: PointerWrapper + Deref> PointerWrapper for Pin<T> {
    fn into_pointer(self) -> *const c_types::c_void {
        // SAFETY: We continue to treat the pointer as pinned by returning just a pointer to it to
        // the caller.
        let inner = unsafe { Pin::into_inner_unchecked(self) };
        inner.into_pointer()
    }

    unsafe fn from_pointer(p: *const c_types::c_void) -> Self {
        // SAFETY: The object was originally pinned.
        Pin::new_unchecked(T::from_pointer(p))
    }
}
