/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: hardware_info.rs
Description: todo
---------------------------------------------------------------------------------------------------
Notes:
*/

/// # Hardware Information
/// The `HardwareInfo` struct is used to package information about the hardware of the
/// Thorlabs device.
/// An instance of the `HardwareInfo` struct is returned by the `req_hw_info()` function
/// as part of the `ThorlabsDevice` trait.

#[derive(Debug, Clone)]
pub struct HardwareInfo {
    // todo impl Display for HardwareInfo to show all the sections with names
    pub(crate) hardware_serial_number: u32,
    pub(crate) model_number: String,
    pub(crate) hardware_type: u16,
    pub(crate) firmware_version: String,
    pub(crate) hardware_version: u16,
    pub(crate) mod_state: u16,
    pub(crate) number_channels: u16,
}
