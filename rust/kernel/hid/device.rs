// SPDX-License-Identifier: GPL-2.0

//! HID device and utils to register it.
//!
//! Copyright (c) 2023 José Expósito <jose.exposito89@gmail.com>

use crate::driver;
use core::ffi::c_ulong;
use macros::kunit_tests;

/// HID device ID, similar to the C structure `hid_device_id`.
#[derive(Clone, Copy, Default)]
pub struct DeviceId {
    /// Bus used to connect the device.
    pub bus: u16,

    /// Group of the device.
    pub group: u16,

    /// Device vendor ID.
    pub vendor: u32,

    /// Device product ID.
    pub product: u32,

    /// Device driver data, usually for quirks.
    pub driver_data: c_ulong,
}

// SAFETY: `ZERO` is all zeroed-out and `to_rawid` does not use `_offset`.
unsafe impl const driver::RawDeviceId for DeviceId {
    type RawType = bindings::hid_device_id;

    const ZERO: Self::RawType = bindings::hid_device_id {
        bus: 0,
        group: 0,
        vendor: 0,
        product: 0,
        driver_data: 0,
    };

    fn to_rawid(&self, _offset: isize) -> Self::RawType {
        bindings::hid_device_id {
            bus: self.bus,
            group: self.group,
            vendor: self.vendor,
            product: self.product,
            driver_data: self.driver_data,
        }
    }
}

/// Defines the ID table for a HID driver.
///
/// # Examples
///
/// ```ignore
/// # use kernel::{define_hid_id_table, hid};
/// #
/// struct MyDriver;
///
/// impl hid::Driver for MyDriver {
///     // [...]
///     define_hid_id_table! {(), [
///         ({
///             bus: 0x03,
///             group: 0,
///             vendor: 0x1234,
///             product: 0xABCD,
///             driver_data: 0,
///         }, None),
///     ]}
/// }
/// ```
#[macro_export]
macro_rules! define_hid_id_table {
    ($data_type:ty, $($t:tt)*) => {
        type IdInfo = $data_type;
        $crate::define_id_table!(ID_TABLE, $crate::hid::DeviceId, $data_type, $($t)*);
    };
}

#[kunit_tests(rust_kernel_hid_device)]
mod tests {
    use super::*;
    use crate::{driver::RawDeviceId, hid};

    fn assert_device_id(
        device_id: DeviceId,
        bus: u16,
        group: u16,
        vendor: u32,
        product: u32,
        driver_data: c_ulong,
    ) {
        assert_eq!(device_id.bus, bus);
        assert_eq!(device_id.group, group);
        assert_eq!(device_id.vendor, vendor);
        assert_eq!(device_id.product, product);
        assert_eq!(device_id.driver_data, driver_data);
    }

    #[test]
    fn rust_test_hid_device_device_id_default() {
        let device_id = hid::DeviceId::default();
        assert_device_id(device_id, 0, 0, 0, 0, 0);
    }

    #[test]
    fn rust_test_hid_device_device_id_to_rawid() {
        let device_id = hid::DeviceId {
            bus: 0x03,
            group: 1,
            vendor: 0x1234,
            product: 0xABCD,
            driver_data: 2,
        };

        let raw_id = device_id.to_rawid(0);
        assert_eq!(raw_id.bus, 0x03);
        assert_eq!(raw_id.group, 1);
        assert_eq!(raw_id.vendor, 0x1234);
        assert_eq!(raw_id.product, 0xABCD);
        assert_eq!(raw_id.driver_data, 2);
    }
}
