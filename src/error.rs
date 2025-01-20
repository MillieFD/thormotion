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
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::PoisonError;
use tokio::sync::broadcast;
use tokio::time::error::Elapsed;

/// todo documentation to explain re-exporting and visibility
pub(crate) use Error::*;

/// # Error Enum
/// todo documentation
#[derive(Debug)]
pub enum Error {
    AptProtocolError(String),
    DeviceError(String),
    EnumerationError(String),
    ChannelReceiveError(broadcast::error::RecvError),
    ChannelSendError(broadcast::error::SendError<Box<[u8]>>),
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

impl From<rusb::Error> for Error {
    fn from(err: rusb::Error) -> Self {
        RusbError(err)
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        RwLockPoisoned(err.to_string())
    }
}

impl From<broadcast::error::RecvError> for Error {
    fn from(err: broadcast::error::RecvError) -> Self {
        ChannelReceiveError(err)
    }
}

impl From<broadcast::error::SendError<Box<[u8]>>> for Error {
    fn from(err: broadcast::error::SendError<Box<[u8]>>) -> Self {
        ChannelSendError(err)
    }
}

impl From<Elapsed> for Error {
    fn from(err: Elapsed) -> Self {
        AptProtocolError(err.to_string())
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        TryFromSliceError(err)
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            _ => PyRuntimeError::new_err(err.to_string()),
        }
    }
}
