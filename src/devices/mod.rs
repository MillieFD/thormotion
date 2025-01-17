/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: devices/mod.rs
Description: This file defines the devices module, which contains submodules for each Thorlabs
device type. Each submodule contains a struct which represents the device type. Each struct
implements the ThorlabsDevice trait, which provides a basic functionality that is shared across
all Thorlabs devices. Each struct also implements traits which provide additional functions
specific to its device type. These are defined in the "traits" module.
---------------------------------------------------------------------------------------------------
Notes:
*/
mod hardware_info;
mod kdc101;
mod usb_device_primitive;

pub use hardware_info::HardwareInfo;
pub use kdc101::KDC101;
pub use usb_device_primitive::UsbDevicePrimitive;
