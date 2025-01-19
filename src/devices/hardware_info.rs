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
pub(crate) struct HardwareInfo {
    pub(crate) serial_number: u32,
    pub(crate) model_number: String,
    pub(crate) hardware_type: u16,
    pub(crate) firmware_version: String,
    pub(crate) hardware_version: u16,
    pub(crate) mod_state: u16,
    pub(crate) number_channels: u16,
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
            self.mod_state,
            self.number_channels,
        )
    }
}
