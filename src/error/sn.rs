/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::fmt::{Debug, Display, Formatter};

use nusb::DeviceInfo;

type Sn = String;

#[derive(Debug)]
pub enum Error {
    Invalid(Sn),
    Multiple(Sn),
    NotFound(Sn),
    Unknown(DeviceInfo),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Invalid(sn) => write!(
                f,
                "{:?} is not a valid serial number for the requested Thorlabs device type.",
                sn
            ),
            Error::Multiple(sn) => {
                write!(f, "Multiple devices found with serial number {}", sn)
            }
            Error::NotFound(sn) => {
                write!(f, "No devices found with serial number {}", sn)
            }
            Error::Unknown(dev) => {
                write!(f, "Serial number could not be read from device {:?}", dev)
            }
        }
    }
}

impl std::error::Error for Error {}

#[cfg(feature = "py")]
impl From<Error> for pyo3::PyErr {
    fn from(error: Error) -> Self {
        pyo3::exceptions::PyException::new_err(error.to_string())
    }
}
