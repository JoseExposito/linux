// SPDX-License-Identifier: GPL-2.0

//! Delay functions for operations like sleeping.
//!
//! C header: [`include/linux/delay.h`](../../../../include/linux/delay.h)

use crate::bindings;
use core::{cmp::min, time::Duration};

const MILLIS_PER_SEC: u64 = 1_000;

/// Sleeps safely even with waitqueue interruptions.
///
/// This function forwards the call to the C side `msleep` function. As a result,
/// `duration` will be rounded up to the nearest millisecond if granularity less
/// than a millisecond is provided. Any [`Duration`] that exceeds
/// [`c_uint::MAX`][core::ffi::c_uint::MAX] in milliseconds is saturated.
pub fn coarse_sleep(duration: Duration) {
    let milli_as_nanos = Duration::MILLISECOND.subsec_nanos();

    // Rounds the nanosecond component of `duration` up to the nearest millisecond.
    let nanos_as_millis = duration.subsec_nanos().wrapping_add(milli_as_nanos - 1) / milli_as_nanos;

    // Saturates the second component of `duration` to `c_uint::MAX`.
    let seconds_as_millis = min(
        duration.as_secs().saturating_mul(MILLIS_PER_SEC),
        u64::from(core::ffi::c_uint::MAX),
    ) as core::ffi::c_uint;

    // SAFETY: msleep is safe for all values of an `unsigned int`.
    unsafe { bindings::msleep(seconds_as_millis.saturating_add(nanos_as_millis)) }
}
