/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: enumerate.rs
Description: This file defines the enumeration process which is used to probe all connected usb
devices, identify Thorlabs devices, and create an instance of the appropriate struct for each
device. The enumeration process is lazily initialised, and the results are stored in the
ALL_DEVICES hash map.
---------------------------------------------------------------------------------------------------
Notes:
todo when a device is disconnected, it is not currently removed from the hash map.
*/

use crate::devices::UsbDevicePrimitive;
use crate::env::{TIMEOUT, VENDOR_ID};
use crate::error::Error;
use rusb::{DeviceDescriptor, DeviceHandle, DeviceList, GlobalContext, Language};

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
        0 => Err(Error::DeviceNotFound(serial_number.to_string())),
        1 => Ok(devices.into_iter().next().unwrap()),
        _ => Err(Error::MultipleDevicesFound(serial_number.to_string())),
    }
}
fn get_language(handle: &DeviceHandle<GlobalContext>) -> Option<Language> {
    handle.read_languages(TIMEOUT).ok()?.get(0).copied()
}

fn get_serial_number(
    descriptor: &DeviceDescriptor,
    handle: &DeviceHandle<GlobalContext>,
    language: Language,
) -> Option<String> {
    Some(
        handle
            .read_serial_number_string(language, &descriptor, TIMEOUT)
            .ok()?,
    )
}
