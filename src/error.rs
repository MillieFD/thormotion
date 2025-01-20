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

use std::fmt::{Display, Formatter, Result as FmtResult};

/// todo documentation to explain re-exporting and visibility
pub(crate) use Error::*;

/// # Error Enum
/// todo documentation
#[derive(Debug)]
pub enum Error {
    AptProtocolError(String),
    DeviceError(String),
    EnumerationError(String),
    ChannelReceiveError(tokio::sync::broadcast::error::RecvError),
    ChannelSendError(tokio::sync::broadcast::error::SendError<Box<[u8]>>),
    RusbError(rusb::Error),
    TryFromSliceError(std::array::TryFromSliceError),
    RwLockPoisoned(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            AptProtocolError(msg) => write!(f, "APT Protocol Error : {}", msg),
            DeviceError(msg) => write!(f, "Device Error : {}", msg),
            EnumerationError(msg) => write!(f, "Enumeration Error : {}", msg),
            ChannelReceiveError(err) => write!(f, "Channel Receive Error : {}", err),
            ChannelSendError(err) => write!(f, "Channel Send Error : {}", err),
            RusbError(err) => write!(f, "Rusb Error : {}", err),
            TryFromSliceError(err) => write!(f, "Try From Slice Error : {}", err),
            RwLockPoisoned(err) => write!(f, "RwLock Poisoned : {}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        DeviceError(err.to_string())
    }
}

impl From<rusb::Error> for Error {
    fn from(err: rusb::Error) -> Self {
        RusbError(err)
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        RwLockPoisoned(err.to_string())
    }
}

impl From<tokio::sync::broadcast::error::RecvError> for Error {
    fn from(err: tokio::sync::broadcast::error::RecvError) -> Self {
        ChannelReceiveError(err)
    }
}

impl From<tokio::sync::broadcast::error::SendError<Box<[u8]>>> for Error {
    fn from(err: tokio::sync::broadcast::error::SendError<Box<[u8]>>) -> Self {
        ChannelSendError(err)
    }
}

impl From<tokio::time::error::Elapsed> for Error {
    fn from(err: tokio::time::error::Elapsed) -> Self {
        AptProtocolError(err.to_string())
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        TryFromSliceError(err)
    }
}

impl From<Error> for pyo3::PyErr {
    fn from(err: Error) -> pyo3::PyErr {
        match err {
            _ => pyo3::exceptions::PyRuntimeError::new_err(err.to_string()),
        }
    }
}
