/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::fmt::Display;
use std::sync::{Mutex, MutexGuard, OnceLock};

use ahash::{HashMap, HashMapExt};
use nusb::{DeviceInfo, list_devices};

use crate::error::sn::Error;

/* ---------------------------------------------------------------------------- Public Functions */

/// Returns an iterator over all connected Thorlabs USB devices.
pub fn get_devices() -> impl Iterator<Item = DeviceInfo> {
    list_devices()
        .expect("Failed to list devices due to OS error")
        .filter(|dev| dev.vendor_id() == 0x0403)
}

/// For convenience, this function prints a list of connected Thorlabs USB devices to stdout.
pub fn show_devices() {
    let devices = get_devices();
    for device in devices {
        println!("{:?}\n", device);
    }
}

/* --------------------------------------------------------------------------- Private Functions */

/// A lazily initialised [`HashMap`] containing the `serial number` (key) and [`abort function`][1]
/// (value) for each connected [`Thorlabs Device`][2]. It is protected by an async [`Mutex`] for
/// thread-safe concurrent access.
///
/// The [`HashMap`] is only accessed when connecting or disconnecting [`Thorlabs Devices`][2]. The
/// [`HashMap`] is not required when [`opening`][3], [`closing`][4], or [`sending`][5] commands to
/// the device. As such, lock contention does not affect device latency.
///
/// If an irrecoverable error occurs anywhere in the program, this triggers the [`abort`]
/// function that safely [`aborts`][1] each device, bringing the system to a controlled stop.
///
/// [1]: crate::traits::ThorlabsDevice::abort
/// [2]: crate::traits::ThorlabsDevice
/// [3]: crate::devices::UsbPrimitive::open
/// [4]: crate::devices::UsbPrimitive::close
/// [5]: crate::devices::UsbPrimitive::send
#[doc(hidden)]
static DEVICES: OnceLock<Mutex<HashMap<String, Box<dyn Fn() + Send + 'static>>>> = OnceLock::new();

/// Returns a [`MutexGuard`] protecting access to the global [`DEVICES`][1] [`HashMap`]. The map is
/// lazily initialised when first accessed.
///
/// ### Panics
///
/// Calls [`abort`] if the mutex is poisoned, safely stopping all devices before terminating
/// the program.
///
/// [1]: DEVICES
#[doc(hidden)]
fn devices<'a>() -> MutexGuard<'a, HashMap<String, Box<dyn Fn() + Send + 'static>>> {
    DEVICES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|e| abort(format!("DEVICES mutex is poisoned: {}", e)))
}

/// Adds a new [`Thorlabs Device`][1] `serial number` (key) and corresponding [`abort`][2] function
/// (value) to the global [`DEVICES`][3] [`HashMap`].
///
/// [1]: crate::traits::ThorlabsDevice
/// [2]: crate::traits::ThorlabsDevice::abort
/// [3]: DEVICES
#[doc(hidden)]
pub(super) fn add_device<F>(serial_number: String, f: F)
where
    F: Fn() + Send + 'static,
{
    devices().insert(serial_number, Box::new(f));
}

/// Removes the specified [`Thorlabs Device`][1] from the global [`DEVICES`][2] [`HashMap`]. Then
/// calls the corresponding [`abort`][2] function.
///
/// [1]: crate::traits::ThorlabsDevice
/// [2]: DEVICES
#[doc(hidden)]
pub(super) fn remove_device(serial_number: &str) {
    if let Some(f) = devices().remove(serial_number) {
        f()
    }
}

/// Calls the [`abort`][1] function for the specified [`Thorlabs Device`][1].
///
/// The device is not removed from the global [`DEVICES`][2] [`HashMap`]. You can use
/// [`Open`][3] to resume communication.
///
/// [1]: crate::traits::ThorlabsDevice::abort
/// [2]: DEVICES
/// [3]: crate::devices::UsbPrimitive::open
#[doc(hidden)]
pub(super) fn abort_device(serial_number: &str) {
    if let Some(f) = devices().get(serial_number) {
        f()
    }
}

// SAFETY: This is a placeholder function. DO NOT USE.
/// Removes the specified [`Thorlabs Device`][1] from the global [`DEVICES`][2] [`HashMap`] without
/// calling the corresponding [`abort`][2] or [`close`][3] functions.
///
/// [1]: crate::traits::ThorlabsDevice
/// [2]: crate::traits::ThorlabsDevice::abort
/// [3]: crate::devices::UsbPrimitive::close
#[doc(hidden)]
fn leak_device(serial_number: &str) {
    devices().remove(serial_number);
}

/// Safely stops all [`Thorlabs devices`][1], cleans up resources, and terminates the program with
/// an error message.
///
/// Internally, this function iterates over the global [`DEVICES`][2] [`HashMap`] and calls the
/// respective [`abort`][3] function for each device. To handle situations that should never occur,
/// see [`bug_abort`].
///
/// ### Panics
///
/// This function always panics.
///
/// This is intended behaviour to safely unwind and free resources.
///
/// [1]: crate::traits::ThorlabsDevice
/// [2]: DEVICES
/// [3]: crate::traits::ThorlabsDevice::abort
#[doc(hidden)]
pub(crate) fn abort<A>(message: A) -> !
where
    A: Display,
{
    log::error!("ABORT → {}", message);
    devices().drain().for_each(|(serial_number, f)| {
        log::info!("ABORT DEVICE {serial_number} (requested)");
        f();
        log::info!("ABORT DEVICE {serial_number} (success)");
    });
    panic!("\nProcess aborted due to error → {}\n", message);
}

/// Safely stops all [`Thorlabs devices`][1], cleans up resources, and terminates the program with
/// an error message. Explains that the error is a bug and encourages the user to open a new GitHub
/// issue.
///
/// Internally, this function calls [`abort`] with an additional message.
///
/// ### Panics
///
/// This function always panics.
///
/// This is intended behaviour to safely unwind and free resources.
///
/// [1]: crate::traits::ThorlabsDevice
/// [2]: DEVICES
/// [3]: crate::traits::ThorlabsDevice::abort
#[doc(hidden)]
pub(crate) fn bug_abort(message: String) -> ! {
    abort(format!(
        "{} : This is a bug. If you are able to reproduce the error, please open a new GitHub \
         issue and report the relevant details",
        message
    ));
}

/// Returns [`DeviceInfo`] for the Thorlabs device with the specified serial number.
///
/// Returns [`Error::NotFound`] if the specified device is not connected.
///
/// Returns [`Error::Multiple`] if more than one device with the specified serial number is found.
pub(super) fn get_device(serial_number: &String) -> Result<DeviceInfo, Error> {
    let mut devices =
        get_devices().filter(|dev| dev.serial_number().map_or(false, |sn| sn == serial_number));
    match (devices.next(), devices.next()) {
        (None, _) => Err(Error::NotFound(serial_number.clone())),
        (Some(device), None) => Ok(device),
        (Some(_), Some(_)) => Err(Error::Multiple(serial_number.clone())),
    }
}
