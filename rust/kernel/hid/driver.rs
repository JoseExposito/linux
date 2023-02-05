// SPDX-License-Identifier: GPL-2.0

//! HID driver trait and utils to register it.
//!
//! Copyright (c) 2023 José Expósito <jose.exposito89@gmail.com>

use crate::{driver, str::CStr, to_result, PointerWrapper, Result, ThisModule};

/// HID driver.
pub trait Driver {
    /// Context data associated with the HID driver.
    ///
    /// It determines the type of the context data passed to each of the methods of the trait.
    type Data: PointerWrapper + Sync + Send;
}

/// Declares a kernel module that exposes a HID driver.
///
/// # Examples
///
/// ```ignore
/// # use kernel::{hid, module_hid_driver};
/// #
/// struct MyDriver;
///
/// impl hid::Driver for MyDriver {
///     // [...]
/// }
///
/// module_hid_driver! {
///     type: MyDriver,
///     name: "hid_driver_name",
///     author: "Your name",
///     license: "GPL",
/// }
/// ```
#[macro_export]
macro_rules! module_hid_driver {
    ($($f:tt)*) => {
        $crate::module_driver!(<T>, $crate::hid::Adapter<T>, { $($f)* });
    };
}

/// An adapter for the registration of HID drivers.
///
/// The HID subsystem uses the C macro `module_hid_driver` that implements custom handlers for
/// `register` and `unregister`.
///
/// This adapter calls the same helpers when a Rust drivers calls `module_hid_driver!`.
pub struct Adapter<T: Driver>(T);

impl<T: Driver> driver::DriverOps for Adapter<T> {
    /// The C struct `hid_driver`.
    type RegType = bindings::hid_driver;

    /// Registers a HID driver.
    ///
    /// # Safety
    ///
    /// `reg` must point to valid, initialized, and writable memory. It may be modified by this
    /// function to hold registration state.
    ///
    /// On success, `reg` must remain pinned and valid until the matching call to
    /// [`DriverOps::unregister`].
    unsafe fn register(
        reg: *mut bindings::hid_driver,
        name: &'static CStr,
        module: &'static ThisModule,
    ) -> Result {
        let owner = module.0;
        let drv_name = name.as_char_ptr();

        // SAFETY: By the safety requirements of this function (defined in the trait definition),
        // `reg` is non-null and valid.
        let hid = unsafe { &mut *reg };
        hid.name = drv_name as _;

        // The C macro `module_hid_driver` calls `__hid_register_driver` on register. Do the same
        // for Rust drivers.
        //
        // SAFETY: By the safety requirements of this function (defined in the trait definition),
        // `reg` is non-null and valid.
        to_result(unsafe { bindings::__hid_register_driver(reg, owner, drv_name) })
    }

    /// Unregisters a driver previously registered with [`DriverOps::register`].
    ///
    /// # Safety
    ///
    /// `reg` must point to valid writable memory, initialized by a previous successful call to
    /// [`DriverOps::register`].
    unsafe fn unregister(reg: *mut bindings::hid_driver) {
        // The C macro `module_hid_driver` calls `hid_unregister_driver` on unregister. Do the same
        // for Rust drivers.
        //
        // SAFETY: By the safety requirements of this function (defined in the trait definition),
        // `reg` was passed (and updated) by a previous successful call to `__hid_register_driver`.
        unsafe { bindings::hid_unregister_driver(reg) };
    }
}
