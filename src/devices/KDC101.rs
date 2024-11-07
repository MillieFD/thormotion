/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: todo
Description: todo
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::devices::thorlabs_device_primitive::ThorlabsDevicePrimitive;
use crate::enumeration::usb_device_primitive::UsbDevicePrimitive;
use crate::errors::error_types::Error;
use crate::traits::thorlabs_device::ThorlabsDevice;
use std::ops::Deref;

pub(crate) struct KDC101 {
    device: ThorlabsDevicePrimitive,
}

impl Deref for KDC101 {
    type Target = ThorlabsDevicePrimitive;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl From<ThorlabsDevicePrimitive> for KDC101 {
    fn from(device: ThorlabsDevicePrimitive) -> Self {
        Self { device }
    }
}

impl ThorlabsDevice for KDC101 {}

impl KDC101 {
    pub(crate) async fn new(device: UsbDevicePrimitive) -> Result<Self, Error> {
        Ok(Self {
            device: ThorlabsDevicePrimitive::new(device).await?,
        })
    }
}
