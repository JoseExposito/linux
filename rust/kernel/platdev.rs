// SPDX-License-Identifier: GPL-2.0

//! Platform devices.
//!
//! Also called `platdev`, `pdev`.
//!
//! C header: [`include/linux/platform_device.h`](../../../../include/linux/platform_device.h)

use crate::{
    bindings, c_types,
    error::{Error, Result},
    pr_info, CStr,
};
use alloc::boxed::Box;
use core::{marker::PhantomPinned, pin::Pin};

/// A registration of a platform device.
#[derive(Default)]
pub struct Registration {
    registered: bool,
    pdrv: bindings::platform_driver,
    _pin: PhantomPinned,
}

// SAFETY: `Registration` does not expose any of its state across threads
// (it is fine for multiple threads to have a shared reference to it).
unsafe impl Sync for Registration {}

extern "C" fn probe_callback(_pdev: *mut bindings::platform_device) -> c_types::c_int {
    pr_info!("Rust platform_device probed\n");
    0
}

extern "C" fn remove_callback(_pdev: *mut bindings::platform_device) -> c_types::c_int {
    pr_info!("Rust platform_device removed\n");
    0
}

impl Registration {
    fn register(
        self: Pin<&mut Self>,
        name: CStr<'static>,
        module: &'static crate::ThisModule,
    ) -> Result {
        // SAFETY: We must ensure that we never move out of `this`.
        let this = unsafe { self.get_unchecked_mut() };
        if this.registered {
            // Already registered.
            return Err(Error::EINVAL);
        }
        this.pdrv.driver.name = name.as_ptr() as *const c_types::c_char;
        this.pdrv.probe = Some(probe_callback);
        this.pdrv.remove = Some(remove_callback);
        // SAFETY:
        //   - `this.pdrv` lives at least until the call to `platform_driver_unregister()` returns.
        //   - `name` pointer has static lifetime.
        //   - `module.0` lives at least as long as the module.
        //   - `probe()` and `remove()` are static functions.
        let ret = unsafe { bindings::__platform_driver_register(&mut this.pdrv, module.0) };
        if ret < 0 {
            return Err(Error::from_kernel_errno(ret));
        }
        this.registered = true;
        Ok(())
    }

    /// Registers a platform device.
    ///
    /// Returns a pinned heap-allocated representation of the registration.
    pub fn new_pinned(
        name: CStr<'static>,
        module: &'static crate::ThisModule,
    ) -> Result<Pin<Box<Self>>> {
        let mut r = Pin::from(Box::try_new(Self::default())?);
        r.as_mut().register(name, module)?;
        Ok(r)
    }
}

impl Drop for Registration {
    fn drop(&mut self) {
        if self.registered {
            // SAFETY: if `registered` is true, then `self.pdev` was registered
            // previously, which means `platform_driver_unregister` is always
            // safe to call.
            unsafe { bindings::platform_driver_unregister(&mut self.pdrv) }
        }
    }
}
