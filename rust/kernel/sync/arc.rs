// SPDX-License-Identifier: GPL-2.0

//! A reference-counted pointer.
//!
//! This module implements a way for users to create reference-counted objects and pointers to
//! them. Such a pointer automatically increments and decrements the count, and drops the
//! underlying object when it reaches zero. It is also safe to use concurrently from multiple
//! threads.
//!
//! It is different from the standard library's [`Arc`] in a few ways:
//! 1. It is backed by the kernel's `refcount_t` type.
//! 2. It does not support weak references, which allows it to be half the size.
//! 3. It saturates the reference count instead of aborting when it goes over a threshold.
//! 4. It does not provide a `get_mut` method, so the ref counted object is pinned.
//!
//! [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html

use crate::{bindings, Result};
use alloc::boxed::Box;
use core::{
    alloc::Layout,
    cell::UnsafeCell,
    convert::AsRef,
    marker::{PhantomData, Unsize},
    mem::ManuallyDrop,
    ops::Deref,
    pin::Pin,
    ptr::{self, NonNull},
};

/// A reference-counted pointer to an instance of `T`.
///
/// The reference count is incremented when new instances of [`Ref`] are created, and decremented
/// when they are dropped. When the count reaches zero, the underlying `T` is also dropped.
///
/// # Invariants
///
/// The reference count on an instance of [`Ref`] is always non-zero.
/// The object pointed to by [`Ref`] is always pinned.
pub struct Ref<T: ?Sized> {
    ptr: NonNull<RefInner<T>>,
    _p: PhantomData<RefInner<T>>,
}

#[repr(C)]
struct RefInner<T: ?Sized> {
    refcount: UnsafeCell<bindings::refcount_t>,
    data: T,
}

// This is to allow [`Ref`] (and variants) to be used as the type of `self`.
impl<T: ?Sized> core::ops::Receiver for Ref<T> {}

// This is to allow coercion from `Ref<T>` to `Ref<U>` if `T` can be converted to the
// dynamically-sized type (DST) `U`.
impl<T: ?Sized + Unsize<U>, U: ?Sized> core::ops::CoerceUnsized<Ref<U>> for Ref<T> {}

// This is to allow `Ref<U>` to be dispatched on when `Ref<T>` can be coerced into `Ref<U>`.
impl<T: ?Sized + Unsize<U>, U: ?Sized> core::ops::DispatchFromDyn<Ref<U>> for Ref<T> {}

// SAFETY: It is safe to send `Ref<T>` to another thread when the underlying `T` is `Sync` because
// it effectively means sharing `&T` (which is safe because `T` is `Sync`); additionally, it needs
// `T` to be `Send` because any thread that has a `Ref<T>` may ultimately access `T` directly, for
// example, when the reference count reaches zero and `T` is dropped.
unsafe impl<T: ?Sized + Sync + Send> Send for Ref<T> {}

// SAFETY: It is safe to send `&Ref<T>` to another thread when the underlying `T` is `Sync` for
// the same reason as above. `T` needs to be `Send` as well because a thread can clone a `&Ref<T>`
// into a `Ref<T>`, which may lead to `T` being accessed by the same reasoning as above.
unsafe impl<T: ?Sized + Sync + Send> Sync for Ref<T> {}

impl<T> Ref<T> {
    /// Constructs a new reference counted instance of `T`.
    pub fn try_new(contents: T) -> Result<Self> {
        Self::try_new_and_init(contents, |_| {})
    }

    /// Constructs a new reference counted instance of `T` and calls the initialisation function.
    ///
    /// This is useful because it provides a mutable reference to `T` at its final location.
    pub fn try_new_and_init<U: FnOnce(Pin<&mut T>)>(contents: T, init: U) -> Result<Self> {
        // INVARIANT: The refcount is initialised to a non-zero value.
        let mut inner = Box::try_new(RefInner {
            // SAFETY: Just an FFI call that returns a `refcount_t` initialised to 1.
            refcount: UnsafeCell::new(unsafe { bindings::REFCOUNT_INIT(1) }),
            data: contents,
        })?;

        // SAFETY: By the invariant, `RefInner` is pinned and `T` is also pinned.
        let pinned = unsafe { Pin::new_unchecked(&mut inner.data) };

        // INVARIANT: The only places where `&mut T` is available are here, which is explicitly
        // pinned, and in `drop`. Both are compatible with the pin requirements.
        init(pinned);

        // SAFETY: We just created `inner` with a reference count of 1 and we're leaking it. So the
        // new `Ref` object owns the reference.
        Ok(unsafe { Self::from_inner(NonNull::from(Box::leak(inner))) })
    }

    /// Deconstructs a [`Ref`] object into a `usize`.
    ///
    /// It can be reconstructed once via [`Ref::from_usize`].
    pub fn into_usize(obj: Self) -> usize {
        ManuallyDrop::new(obj).ptr.as_ptr() as _
    }

    /// Borrows a [`Ref`] instance previously deconstructed via [`Ref::into_usize`].
    ///
    /// # Safety
    ///
    /// `encoded` must have been returned by a previous call to [`Ref::into_usize`]. Additionally,
    /// [`Ref::from_usize`] can only be called after *all* instances of [`RefBorrow`] have been
    /// dropped.
    pub unsafe fn borrow_usize(encoded: usize) -> RefBorrow<T> {
        // SAFETY: By the safety requirement of this function, we know that `encoded` came from
        // a previous call to `Ref::into_usize`.
        let obj = ManuallyDrop::new(unsafe { Ref::from_usize(encoded) });

        // SAFEY: The safety requirements ensure that the object remains alive for the lifetime of
        // the returned value. There is no way to create mutable references to the object.
        unsafe { RefBorrow::new(obj) }
    }

    /// Recreates a [`Ref`] instance previously deconstructed via [`Ref::into_usize`].
    ///
    /// # Safety
    ///
    /// `encoded` must have been returned by a previous call to [`Ref::into_usize`]. Additionally,
    /// it can only be called once for each previous call to [``Ref::into_usize`].
    pub unsafe fn from_usize(encoded: usize) -> Self {
        // SAFETY: By the safety invariants we know that `encoded` came from `Ref::into_usize`, so
        // the reference count held then will be owned by the new `Ref` object.
        unsafe { Self::from_inner(NonNull::new(encoded as _).unwrap()) }
    }
}

impl<T: ?Sized> Ref<T> {
    /// Constructs a new [`Ref`] from an existing [`RefInner`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that `inner` points to a valid location and has a non-zero reference
    /// count, one of which will be owned by the new [`Ref`] instance.
    unsafe fn from_inner(inner: NonNull<RefInner<T>>) -> Self {
        // INVARIANT: By the safety requirements, the invariants hold.
        Ref {
            ptr: inner,
            _p: PhantomData,
        }
    }

    /// Determines if two reference-counted pointers point to the same underlying instance of `T`.
    pub fn ptr_eq(a: &Self, b: &Self) -> bool {
        ptr::eq(a.ptr.as_ptr(), b.ptr.as_ptr())
    }

    /// Returns a pinned version of a given `Ref` instance.
    pub fn pinned(obj: Self) -> Pin<Self> {
        // SAFETY: The type invariants guarantee that the value is pinned.
        unsafe { Pin::new_unchecked(obj) }
    }

    /// Deconstructs a [`Ref`] object into a raw pointer.
    ///
    /// It can be reconstructed once via [`Ref::from_raw`].
    pub fn into_raw(obj: Self) -> *const T {
        let ret = &*obj as *const T;
        core::mem::forget(obj);
        ret
    }

    /// Recreates a [`Ref`] instance previously deconstructed via [`Ref::into_raw`].
    ///
    /// This code relies on the `repr(C)` layout of structs as described in
    /// <https://doc.rust-lang.org/reference/type-layout.html#reprc-structs>.
    ///
    /// # Safety
    ///
    /// `ptr` must have been returned by a previous call to [`Ref::into_raw`]. Additionally, it
    /// can only be called once for each previous call to [``Ref::into_raw`].
    pub unsafe fn from_raw(ptr: *const T) -> Self {
        // SAFETY: The safety requirement ensures that the pointer is valid.
        let align = core::mem::align_of_val(unsafe { &*ptr });
        let offset = Layout::new::<RefInner<()>>()
            .align_to(align)
            .unwrap()
            .pad_to_align()
            .size();
        // SAFETY: The pointer is in bounds because by the safety requirements `ptr` came from
        // `Ref::into_raw`, so it is a pointer `offset` bytes from the beginning of the allocation.
        let data = unsafe { (ptr as *const u8).sub(offset) };
        let metadata = ptr::metadata(ptr as *const RefInner<T>);
        let ptr = ptr::from_raw_parts_mut(data as _, metadata);
        // SAFETY: By the safety requirements we know that `ptr` came from `Ref::into_raw`, so the
        // reference count held then will be owned by the new `Ref` object.
        unsafe { Self::from_inner(NonNull::new(ptr).unwrap()) }
    }
}

impl<T: ?Sized> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: By the type invariant, there is necessarily a reference to the object, so it is
        // safe to dereference it.
        unsafe { &self.ptr.as_ref().data }
    }
}

impl<T: ?Sized> Clone for Ref<T> {
    fn clone(&self) -> Self {
        // INVARIANT: C `refcount_inc` saturates the refcount, so it cannot overflow to zero.
        // SAFETY: By the type invariant, there is necessarily a reference to the object, so it is
        // safe to increment the refcount.
        unsafe { bindings::refcount_inc(self.ptr.as_ref().refcount.get()) };
        Self {
            ptr: self.ptr,
            _p: PhantomData,
        }
    }
}

impl<T: ?Sized> AsRef<T> for Ref<T> {
    fn as_ref(&self) -> &T {
        // SAFETY: By the type invariant, there is necessarily a reference to the object, so it is
        // safe to dereference it.
        unsafe { &self.ptr.as_ref().data }
    }
}

impl<T: ?Sized> Drop for Ref<T> {
    fn drop(&mut self) {
        // SAFETY: By the type invariant, there is necessarily a reference to the object. We cannot
        // touch `refcount` after it's decremented to a non-zero value because another thread/CPU
        // may concurrently decrement it to zero and free it. It is ok to have a raw pointer to
        // freed/invalid memory as long as it is never dereferenced.
        let refcount = unsafe { self.ptr.as_ref() }.refcount.get();

        // INVARIANT: If the refcount reaches zero, there are no other instances of `Ref`, and
        // this instance is being dropped, so the broken invariant is not observable.
        // SAFETY: Also by the type invariant, we are allowed to decrement the refcount.
        let is_zero = unsafe { bindings::refcount_dec_and_test(refcount) };
        if is_zero {
            // The count reached zero, we must free the memory.
            //
            // SAFETY: The pointer was initialised from the result of `Box::leak`.
            unsafe { Box::from_raw(self.ptr.as_ptr()) };
        }
    }
}

/// A borrowed [`Ref`] with manually-managed lifetime.
///
/// # Invariants
///
/// There are no mutable references to the underlying [`Ref`], and it remains valid for the lifetime
/// of the [`RefBorrow`] instance.
pub struct RefBorrow<T: ?Sized> {
    inner_ref: ManuallyDrop<Ref<T>>,
}

impl<T: ?Sized> RefBorrow<T> {
    /// Creates a new [`RefBorrow`] instance.
    ///
    /// # Safety
    ///
    /// Callers must ensure the following for the lifetime of the returned [`RefBorrow`] instance:
    /// 1. That `obj` remains valid;
    /// 2. That no mutable references to `obj` are created.
    unsafe fn new(obj: ManuallyDrop<Ref<T>>) -> Self {
        // INVARIANT: The safety requirements guarantee the invariants.
        Self { inner_ref: obj }
    }
}

impl<T: ?Sized> Deref for RefBorrow<T> {
    type Target = Ref<T>;

    fn deref(&self) -> &Self::Target {
        self.inner_ref.deref()
    }
}
