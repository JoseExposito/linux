// SPDX-License-Identifier: GPL-2.0

use core::convert::TryInto;
use core::marker::PhantomPinned;
use core::mem::MaybeUninit;
use core::pin::Pin;

use crate::bindings;
use crate::c_types;
use crate::error::{Error, KernelResult};
use crate::file_operations;
use crate::types::CStr;

struct RegistrationInner<const N: usize> {
    dev: bindings::dev_t,
    used: usize,
    cdevs: [MaybeUninit<bindings::cdev>; N],
    _pin: PhantomPinned,
}

/// chrdev registration. May contain up to a fixed number (`N`) of devices.
/// Must be pinned.
pub struct Registration<const N: usize> {
    name: CStr<'static>,
    minors_start: u16,
    this_module: &'static crate::ThisModule,
    inner: Option<RegistrationInner<N>>,
}

impl<const N: usize> Registration<{ N }> {
    pub fn new(
        name: CStr<'static>,
        minors_start: u16,
        this_module: &'static crate::ThisModule,
    ) -> Self {
        Registration {
            name,
            minors_start,
            this_module,
            inner: None,
        }
    }

    /// Register a character device with this range. Call this once per device
    /// type (up to `N` times).
    pub fn register<T: file_operations::FileOperations>(self: Pin<&mut Self>) -> KernelResult<()> {
        // SAFETY: we must ensure that we never move out of `this`.
        let this = unsafe { self.get_unchecked_mut() };
        if this.inner.is_none() {
            let mut dev: bindings::dev_t = 0;
            // SAFETY: Calling unsafe function. `this.name` has 'static
            // lifetime 
            let res = unsafe {
                bindings::alloc_chrdev_region(
                    &mut dev,
                    this.minors_start.into(),
                    N.try_into()?,
                    this.name.as_ptr() as *const c_types::c_char,
                )
            };
            if res != 0 {
                return Err(Error::from_kernel_errno(res));
            }
            this.inner = Some(RegistrationInner {
                dev,
                used: 0,
                cdevs: [MaybeUninit::<bindings::cdev>::uninit(); N],
                _pin: PhantomPinned,
            });
        }

        let mut inner = this.inner.as_mut().unwrap();
        if inner.used == N {
            return Err(Error::EINVAL);
        }
        let cdev = inner.cdevs[inner.used].as_mut_ptr();
        // SAFETY: calling unsafe functions and manipulating MaybeUninit ptr.
        unsafe {
            bindings::cdev_init(cdev, &file_operations::FileOperationsVtable::<T>::VTABLE);
            (*cdev).owner = this.this_module.0;
            let rc = bindings::cdev_add(cdev, inner.dev + inner.used as bindings::dev_t, 1);
            if rc != 0 {
                return Err(Error::from_kernel_errno(rc));
            }
        }
        inner.used += 1;
        Ok(())
    }
}

// SAFETY: `Registration` doesn't expose any of its state across threads (it's
// fine for multiple threads to have a shared reference to it).
unsafe impl<const N: usize> Sync for Registration<{ N }> {}

impl<const N: usize> Drop for Registration<{ N }> {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.as_mut() {
            // SAFETY: calling unsafe functions, `0..inner.used` of
            // `inner.cdevs` are initialized in `Registration::register`.
            unsafe {
                for i in 0..inner.used {
                    bindings::cdev_del(inner.cdevs[i].as_mut_ptr());
                }
                bindings::unregister_chrdev_region(inner.dev, N.try_into().unwrap());
            }
        }
    }
}
