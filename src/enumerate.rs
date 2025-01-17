/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: enumerate.rs
Description: todo
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::devices::UsbDevicePrimitive;
use crate::env::{SHORT_TIMEOUT, VENDOR_ID};
use crate::error::Error;
use rusb::{DeviceDescriptor, DeviceHandle, DeviceList, GlobalContext, Language};

/// # Connecting to a Specific Thorlabs Device
///
/// The `get_device` function attempts to find a specific USB device from the rusb
/// `DeviceList<GlobalContext>` using its serial number.
///
/// This internal function is not intended to be used directly.
/// Instead, the `get_device` function is intended to be called by the `new()`
/// functions of specific Thorlabs devices.
///
/// # Arguments
/// - `serial_number`: The serial number of the target USB device as a string.  
///
/// # Returns
/// - `Ok(UsbDevicePrimitive)`: If a single matching device is found, the function will
/// initialise a new instance of the `UsbDevicePrimitive` struct.
/// - `Err(Error::EnumerationError)`: If no device with the specified serial number is found,
/// or if multiple devices with the same serial number are found, then the function will return
/// an `EnumerationError` with a helpful error message.
///
/// # Steps
/// The function performs the following steps:
/// 1. Enumerates all connected USB devices.
/// 2. Filters by the Thorlabs vendor ID.
/// 3. Matches the device's serial number with the input string.
/// 4. Constructs and returns a `UsbDevicePrimitive` for the matching device.
pub fn get_device(serial_number: &str) -> Result<UsbDevicePrimitive, Error> {
    let devices: Vec<UsbDevicePrimitive> = DeviceList::new()?
        .iter()
        .filter_map(|dev| {
            let descriptor = dev.device_descriptor().ok()?;
            if descriptor.vendor_id() != VENDOR_ID {
                return None;
            }
            let handle = dev.open().ok()?;
            let language = get_language(&handle)?;
            let device_serial = get_serial_number(&descriptor, &handle, language)?;
            if device_serial != serial_number {
                return None;
            }
            UsbDevicePrimitive::new(handle, descriptor, language, device_serial).ok()
        })
        .collect();

    match devices.len() {
        0 => Err(Error::EnumerationError(format!(
            "Device with serial number {} could not be found",
            serial_number
        ))),
        1 => Ok(devices.into_iter().next().unwrap()),
        _ => Err(Error::EnumerationError(format!(
            "Multiple devices with serial number {} were found",
            serial_number
        ))),
    }
}
fn get_language(handle: &DeviceHandle<GlobalContext>) -> Option<Language> {
    handle.read_languages(SHORT_TIMEOUT).ok()?.get(0).copied()
}

fn get_serial_number(
    descriptor: &DeviceDescriptor,
    handle: &DeviceHandle<GlobalContext>,
    language: Language,
) -> Option<String> {
    Some(
        handle
            .read_serial_number_string(language, &descriptor, SHORT_TIMEOUT)
            .ok()?,
    )
}
