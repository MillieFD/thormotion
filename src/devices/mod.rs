/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: devices/mod.rs
*/

/// # Devices Module
/// The devices module contains submodules for each Thorlabs device type. Each submodule contains
/// a struct to represent the device type. Each struct implements the ThorlabsDevice trait,
/// which provides a basic functionality that is shared across all Thorlabs devices. Each
/// struct also implements traits which provide additional functions specific to its device type.
/// These are defined in the "traits" module.
mod hardware_info;
mod kdc101;
mod usb_device_primitive;

pub use kdc101::KDC101;
pub use usb_device_primitive::UsbDevicePrimitive;
