/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: error
Description: This file defines the Error enum, which is used to represent custom error types which
may occur during execution. From<T> trait implementations enable implicit conversion with other
error types.
---------------------------------------------------------------------------------------------------
Notes:
*/

use pyo3::exceptions::PyRuntimeError;
use pyo3::PyErr;
use std::fmt::{Display, Formatter};
use std::sync::PoisonError;
use tokio::sync::broadcast;

#[derive(Debug)]
pub enum Error {
    DeviceNotFound(String),
    MultipleDevicesFound(String),
    InvalidSerialNumber(String),
    InvalidMessageId([u8; 2]),
    UsbWriteError(String),
    DeviceError(&'static str),
    WaitingSenderExists([u8; 2]),

    // External errors with implicit conversions
    ChannelReceiveError(String),
    ChannelSendError(String),
    RwLockPoisoned(String),
    RusbError(String),
}

impl Error {
    pub(crate) fn message(&self) -> String {
        match self {
            Error::DeviceNotFound(serial_number) => {
                format!("Device with serial number {serial_number} could not be found")
            }
            Error::MultipleDevicesFound(serial_number) => {
                format!("Multiple devices with serial number {serial_number} were found")
            }
            Error::InvalidSerialNumber(serial_number) => {
                format!("Serial number {serial_number} is not valid for the selected device type")
            }
            Error::InvalidMessageId(id) => {
                format!("{id:?} does not correspond to a known message ID")
            }
            Error::UsbWriteError(serial_number) => {
                format!("Failed to write message to device (serial number: {serial_number})")
            }
            Error::DeviceError(err) => format!("Error whilst communicating with device: {err}"),
            Error::WaitingSenderExists(id) => {
                format!("A waiting sender already exists for message ID {id:?})")
            }
            Error::ChannelReceiveError(err) => format!("Error from channel receiver: {err}"),
            Error::ChannelSendError(err) => format!("Error from channel sender: {err}"),
            Error::RwLockPoisoned(err) => format!("Error from RwLock: {err}"),
            Error::RusbError(err) => format!("Error from rusb: {err}"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl From<rusb::Error> for Error {
    fn from(err: rusb::Error) -> Self {
        Error::RusbError(err.to_string())
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        Error::RwLockPoisoned(err.to_string())
    }
}

impl From<broadcast::error::RecvError> for Error {
    fn from(err: broadcast::error::RecvError) -> Self {
        Error::ChannelReceiveError(err.to_string())
    }
}

impl From<broadcast::error::SendError<Box<[u8]>>> for Error {
    fn from(err: broadcast::error::SendError<Box<[u8]>>) -> Self {
        Error::ChannelSendError(err.to_string())
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            _ => PyRuntimeError::new_err(err.message()),
        }
    }
}
