/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: traits/mod.rs
*/

/// # Traits Module
/// This module defines traits for controlling different Thorlabs devices.
///
/// Each trait contains functions which can be called by Thorlabs devices that implement
/// the trait. This keeps the code modular and easier to maintain.
mod channel_enable_state;
mod hub;
mod motor;
mod thorlabs_device;

pub use channel_enable_state::ChannelEnableState;
pub use hub::Hub;
pub use motor::Motor;
pub use thorlabs_device::ThorlabsDevice;
