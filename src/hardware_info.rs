/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: hardware_info.rs
*/

use std::fmt::{Display, Formatter, Result as FmtResult};

/// # Hardware Information
/// The `HardwareInfo` struct is used to package information about the hardware of the
/// Thorlabs device.
/// An instance of the `HardwareInfo` struct is returned by the `req_hw_info()` function
/// as part of the `ThorlabsDevice` trait.

#[derive(Debug, Clone)]
pub struct HardwareInfo {
    pub(crate) serial_number: String,
    pub(crate) model_number: String,
    pub(crate) hardware_type: u16,
    pub(crate) firmware_version: String,
    pub(crate) hardware_version: u16,
    pub(crate) module_state: u16,
    pub(crate) number_of_channels: u16,
}

impl HardwareInfo {
    pub(crate) fn new(
        serial_number: String,
        model_number: String,
        hardware_type: u16,
        firmware_version: String,
        hardware_version: u16,
        module_state: u16,
        number_of_channels: u16,
    ) -> Self {
        HardwareInfo {
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

impl Display for HardwareInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Serial Number: {}\n\
            Model Number: {}\n\
            Hardware Type: {}\n\
            Firmware Version: {}\n\
            Hardware Version: {}\n\
            Module State: {}\n\
            Number of Channels: {}",
            self.serial_number,
            self.model_number,
            self.hardware_type,
            self.firmware_version,
            self.hardware_version,
            self.module_state,
            self.number_of_channels,
        )
    }
}
