/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: kdc101
Description: This file defines the KDC101 struct and associated functions.
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::devices::usb_device_primitive::UsbDevicePrimitive;
use crate::traits::ThorlabsDevice;
use std::ops::Deref;

#[derive(Debug)]
pub struct KDC101 {
    device: UsbDevicePrimitive,
}

impl ThorlabsDevice for KDC101 {
    const SERIAL_NUMBER_PREFIX: &'static str = "27";
}

impl From<UsbDevicePrimitive> for KDC101 {
    fn from(device: UsbDevicePrimitive) -> Self {
        KDC101 { device }
    }
}

impl Deref for KDC101 {
    type Target = UsbDevicePrimitive;
    fn deref(&self) -> &Self::Target {
        &self.device
    }
}
