/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

/* ----------------------------------------------------------------------------- Private Modules */

mod check_serial_number;
mod thorlabs_device;
mod unit_conversion;

/* ----------------------------------------------------------------------------- Private Exports */

pub(crate) use check_serial_number::CheckSerialNumber;
pub(crate) use unit_conversion::{UnitConversion, Units};

/* ------------------------------------------------------------------------------ Public Exports */

pub use thorlabs_device::ThorlabsDevice;
