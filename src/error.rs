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
use tokio::sync::broadcast;
use tokio::time::error::Elapsed;

/// todo documentation to explain re-exporting and visibility
pub(crate) use Error::*;
pub(crate) use ExternalErr::*;
pub(crate) use InternalErr::*;

/// # Error Enum
/// todo documentation
#[derive(Debug)]
pub(crate) enum Error {
    ThormotionError(InternalErr),
    ExternalError(ExternalErr),
}

/// todo documentation
#[derive(Debug)]
pub(crate) enum InternalErr {
    AptProtocolError(String),
    DeviceError(String),
    EnumerationError(String),
}

/// todo documentation External errors with implicit conversions
#[derive(Debug)]
pub(crate) enum ExternalErr {
    ChannelReceiveError(broadcast::error::RecvError),
    ChannelSendError(broadcast::error::SendError<Box<[u8]>>),
    RusbError(rusb::Error),
    TryFromSliceError(std::array::TryFromSliceError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::ThormotionError(apt_error) => write!(f, "Error : AptError : {}", apt_error),
            Error::ExternalError(ext_error) => write!(f, "Error : ExternalError : {}", ext_error),
        }
    }
}

impl Display for InternalErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            InternalErr::AptProtocolError(msg) => write!(f, "AptProtocolError : {}", msg),
            InternalErr::DeviceError(msg) => write!(f, "DeviceError : {}", msg),
            InternalErr::EnumerationError(msg) => write!(f, "EnumerationError : {}", msg),
        }
    }
}

impl Display for ExternalErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ExternalErr::ChannelReceiveError(err) => write!(f, "ChannelReceiveError : {}", err),
            ExternalErr::ChannelSendError(err) => write!(f, "ChannelSendError : {}", err),
            ExternalErr::RusbError(err) => write!(f, "RusbError : {}", err),
            ExternalErr::TryFromSliceError(err) => write!(f, "TryFromSliceError : {}", err),
        }
    }
}

impl From<rusb::Error> for Error {
    fn from(err: rusb::Error) -> Self {
        Error::ExternalError(ExternalErr::RusbError(err))
    }
}

// impl<T> From<PoisonError<T>> for Error {
//     fn from(err: PoisonError<T>) -> Self {
//         Error::RwLockPoisoned(err.to_string())
//     }
// }

impl From<broadcast::error::RecvError> for Error {
    fn from(err: broadcast::error::RecvError) -> Self {
        Error::ExternalError(ExternalErr::ChannelReceiveError(err))
    }
}

impl From<broadcast::error::SendError<Box<[u8]>>> for Error {
    fn from(err: broadcast::error::SendError<Box<[u8]>>) -> Self {
        Error::ExternalError(ExternalErr::ChannelSendError(err))
    }
}

impl From<Elapsed> for Error {
    fn from(err: Elapsed) -> Self {
        Error::ThormotionError(InternalErr::AptProtocolError(err.to_string()))
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Error::ExternalError(ExternalErr::TryFromSliceError(err))
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            _ => PyRuntimeError::new_err(err.to_string()),
        }
    }
}
