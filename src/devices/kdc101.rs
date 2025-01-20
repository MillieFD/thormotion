/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: test.rs
*/

use crate::devices::usb_device_primitive::UsbDevicePrimitive;
use crate::enumerate::get_device_primitive;
use crate::error::Error;
use crate::traits::{ChannelEnableState, Motor, ThorlabsDevice};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Deref;

#[derive(Debug)]
pub struct KDC101 {
    device: UsbDevicePrimitive,
    serial_number: u32,
    model_number: String,
    hardware_type: u16,
    firmware_version: String,
    hardware_version: u16,
    module_state: u16,
    number_of_channels: u16,
}

impl ThorlabsDevice for KDC101 {
    fn new(serial_number: &str) -> Result<Self, Error> {
        Self::check_serial_number(serial_number)?;
        let device = get_device_primitive(serial_number)?;
        Ok(Self::from(device))
    }

    const SERIAL_NUMBER_PREFIX: &'static str = "27";
}

impl From<UsbDevicePrimitive> for KDC101 {
    fn from(device: UsbDevicePrimitive) -> Self {
        Self::check_serial_number(device.serial_number.as_str()).unwrap_or_else(|err| {
            panic!("KDC101 From<UsbDevicePrimitive> failed: {}", err);
        });
        let (
            serial_number,
            model_number,
            hardware_type,
            firmware_version,
            hardware_version,
            module_state,
            number_of_channels,
        ) = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { device.hw_req_info().await })
            .unwrap();
        Self {
            device,
            serial_number,
            model_number,
            hardware_type,
            firmware_version,
            hardware_version,
            module_state,
            number_of_channels,
        }
    }
}

impl From<String> for KDC101 {
    fn from(serial_number: String) -> Self {
        Self::new(serial_number.as_str()).unwrap_or_else(|err| {
            panic!("KDC101 From<String> failed: {}", err);
        })
    }
}

impl From<&'static str> for KDC101 {
    fn from(serial_number: &'static str) -> Self {
        Self::new(serial_number).unwrap_or_else(|err| {
            panic!("KDC101 From<&'static str> failed: {}", err);
        })
    }
}

impl Deref for KDC101 {
    type Target = UsbDevicePrimitive;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl Display for KDC101 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "KDC101 (serial number : {})", self.serial_number)
    }
}

impl ChannelEnableState for KDC101 {}

impl Motor for KDC101 {
    const DISTANCE_ANGLE_SCALING_FACTOR: f64 = 34554.96;
    const VELOCITY_SCALING_FACTOR: f64 = 772981.3692;
    const ACCELERATION_SCALING_FACTOR: f64 = 263.8443072;
}
