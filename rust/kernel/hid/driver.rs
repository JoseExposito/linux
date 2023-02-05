// SPDX-License-Identifier: GPL-2.0

//! HID driver trait and utils to register it.
//!
//! Copyright (c) 2023 José Expósito <jose.exposito89@gmail.com>

use crate::{
    driver, error::from_kernel_result, hid, str::CStr, to_result, PointerWrapper, Result,
    ThisModule,
};
use macros::kunit_tests;

#[cfg(CONFIG_KUNIT)]
use crate::hid::bindings_mock as bindings;

/// HID driver.
pub trait Driver {
    /// Context data associated with the HID driver.
    ///
    /// It determines the type of the context data passed to each of the methods of the trait.
    type Data: PointerWrapper + Sync + Send + driver::DeviceRemoval;

    /// Type holding information about each device id supported by the driver.
    type IdInfo: 'static = ();

    /// Table of device IDs supported by the driver.
    const ID_TABLE: Option<driver::IdTable<'static, hid::DeviceId, Self::IdInfo>> = None;

    /// HID driver probe.
    ///
    /// Called when a new HID device is added or discovered. Implementers should attempt to
    /// initialize the device here.
    ///
    /// On success, the device private data should be returned.
    fn probe(dev: &mut hid::Device, id_info: Option<&Self::IdInfo>) -> Result<Self::Data>;

    /// HID driver remove.
    ///
    /// Called when a HID device is removed. Implementers should prepare the device for complete
    /// removal here.
    fn remove(_dev: &mut hid::Device, _data: &Self::Data) {}
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
        hid.probe = Some(probe_callback::<T>);
        hid.remove = Some(remove_callback::<T>);
        if let Some(t) = T::ID_TABLE {
            hid.id_table = t.as_ref();
        }

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

unsafe extern "C" fn probe_callback<T: Driver>(
    hdev: *mut bindings::hid_device,
    id: *const bindings::hid_device_id,
) -> core::ffi::c_int {
    from_kernel_result! {
        // SAFETY: `hdev` is valid by the contract with the C code. `dev` is alive only for the
        // duration of this call, so it is guaranteed to remain alive for the lifetime of `hdev`.
        let mut dev = unsafe { hid::Device::from_ptr(hdev) };

        // SAFETY: `id` is valid by the requirements the contract with the C code.
        let info = id.cast::<u8>().cast::<Option<T::IdInfo>>();
        // SAFETY: The id table has a static lifetime, so `ptr` is guaranteed to be valid for read.
        let info = unsafe { (*info).as_ref() };

        let data = T::probe(&mut dev, info)?;

        let ptr = T::Data::into_pointer(data);
        // SAFETY: `hdev` is valid for write by the contract with the C code.
        unsafe { bindings::hid_set_drvdata(hdev, ptr as _) };
        Ok(0)
    }
}

unsafe extern "C" fn remove_callback<T: Driver>(hdev: *mut bindings::hid_device) {
    let (mut dev, data) = get_device_and_data::<T>(hdev);

    T::remove(&mut dev, &data);

    <T::Data as driver::DeviceRemoval>::device_remove(&data);
}

fn get_device_and_data<T: Driver>(
    hdev: *mut bindings::hid_device,
) -> (hid::Device, <T as Driver>::Data) {
    // SAFETY: `hdev` is valid by the contract with the C code. `dev` is alive only for the
    // duration of this call, so it is guaranteed to remain alive for the lifetime of `hdev`.
    let dev = unsafe { hid::Device::from_ptr(hdev) };

    // SAFETY: `hdev` is valid by the contract with the C code.
    let ptr = unsafe { bindings::hid_get_drvdata(hdev) };
    // SAFETY: The value returned by `hid_get_drvdata` was stored by a previous call to
    // `hid_set_drvdata` in `probe_callback` above; the value comes from a call to
    // `T::Data::into_pointer`.
    let data = unsafe { T::Data::from_pointer(ptr) };

    (dev, data)
}

#[kunit_tests(rust_kernel_hid_driver)]
mod tests {
    use super::*;
    use crate::{c_str, device, driver, hid, prelude::*, str::CStr, sync::Arc};
    use core::ptr;

    static mut REMOVED: bool = false;

    struct TestDriverData {
        probed: bool,
    }

    impl TestDriverData {
        fn new() -> Self {
            TestDriverData { probed: true }
        }
    }

    type DeviceData = device::Data<(), (), TestDriverData>;

    struct TestDriver;
    impl hid::Driver for TestDriver {
        type Data = Arc<DeviceData>;

        fn probe(
            _dev: &mut hid::Device,
            _id_info: Option<&Self::IdInfo>,
        ) -> Result<Arc<DeviceData>> {
            let data = crate::new_device_data!(
                (),
                (),
                TestDriverData::new(),
                "TestDriver::Registrations"
            )?;
            let data = Arc::<DeviceData>::from(data);
            Ok(data)
        }

        fn remove(_dev: &mut hid::Device, _data: &Self::Data) {
            unsafe { REMOVED = true };
        }
    }

    #[test]
    fn rust_test_hid_driver_adapter() {
        let mut reg = bindings::hid_driver::default();
        let name = c_str!("TestDriver");
        static MODULE: ThisModule = unsafe { ThisModule::from_ptr(ptr::null_mut()) };

        let res = unsafe {
            <hid::Adapter<TestDriver> as driver::DriverOps>::register(&mut reg, name, &MODULE)
        };
        assert_eq!(
            unsafe { CStr::from_char_ptr(reg.name).to_str().unwrap() },
            "TestDriver"
        );
        // The `__hid_register_driver` mock returns -19.
        assert_eq!(res, Err(ENODEV));

        unsafe { <hid::Adapter<TestDriver> as driver::DriverOps>::unregister(&mut reg) };
        // The `hid_unregister_driver` mock set the driver name to "unregistered".
        assert_eq!(
            unsafe { CStr::from_char_ptr(reg.name).to_str().unwrap() },
            "unregistered"
        );
    }

    #[test]
    fn rust_test_hid_driver_probe() {
        let mut hdev = bindings::hid_device::default();
        let id = bindings::hid_device_id::default();

        let res = unsafe { probe_callback::<TestDriver>(&mut hdev, &id) };
        assert_eq!(res, 0);

        let data = unsafe { bindings::hid_get_drvdata(&mut hdev) };
        let data = unsafe { Arc::<DeviceData>::from_pointer(data) };
        assert_eq!(data.probed, true);
    }

    #[test]
    fn rust_test_hid_driver_remove() {
        let mut hdev = bindings::hid_device::default();
        let id = bindings::hid_device_id::default();

        let res = unsafe { probe_callback::<TestDriver>(&mut hdev, &id) };
        assert_eq!(res, 0);
        assert_eq!(unsafe { REMOVED }, false);

        unsafe { remove_callback::<TestDriver>(&mut hdev) };
        assert_eq!(unsafe { REMOVED }, true);
    }
}
