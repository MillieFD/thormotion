/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::error::sn::Error;

pub(crate) trait CheckSerialNumber {
    /// The unique serial number prefix for the implementing Thorlabs device type.
    /// See the Thorlabs APT Protocol, Issue 39, Page 32.
    const SERIAL_NUMBER_PREFIX: &'static str;

    /// Returns [`Error::Invalid`] if the serial number:
    /// 1. Does not match the serial number prefix for the target device type
    /// 2. Is not exactly eight-digits long
    /// 3. Contains non-numeric characters
    fn check_serial_number(serial_number: &String) -> Result<(), Error> {
        if serial_number.starts_with(Self::SERIAL_NUMBER_PREFIX)
            && serial_number.len() == 8
            && serial_number.chars().all(|c| c.is_numeric())
        {
            Ok(())
        } else {
            Err(Error::Invalid(serial_number.clone()))
        }
    }
}
