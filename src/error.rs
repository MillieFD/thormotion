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

/// # Error Enum
/// todo documentation
#[derive(Debug)]
pub enum Error {
    AptError(AptError),
    ExternalError(ExternalError),
}

/// todo documentation
#[derive(Debug)]
pub enum AptError {
    AptProtocolError(String),
    DeviceError(String),
    EnumerationError(String),
}

/// todo documentation External errors with implicit conversions
#[derive(Debug)]
pub enum ExternalError {
    ChannelReceiveError(broadcast::error::RecvError),
    ChannelSendError(broadcast::error::SendError<Box<[u8]>>),
    RusbError(rusb::Error),
    TryFromSliceError(std::array::TryFromSliceError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::AptError(apt_error) => write!(f, "Error : AptError : {}", apt_error),
            Error::ExternalError(ext_error) => write!(f, "Error : ExternalError : {}", ext_error),
        }
    }
}

impl Display for AptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            AptError::AptProtocolError(msg) => write!(f, "AptProtocolError : {}", msg),
            AptError::DeviceError(msg) => write!(f, "DeviceError : {}", msg),
            AptError::EnumerationError(msg) => write!(f, "EnumerationError : {}", msg),
        }
    }
}

impl Display for ExternalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ExternalError::ChannelReceiveError(err) => write!(f, "ChannelReceiveError : {}", err),
            ExternalError::ChannelSendError(err) => write!(f, "ChannelSendError : {}", err),
            ExternalError::RusbError(err) => write!(f, "RusbError : {}", err),
            ExternalError::TryFromSliceError(err) => write!(f, "TryFromSliceError : {}", err),
        }
    }
}

impl From<rusb::Error> for Error {
    fn from(err: rusb::Error) -> Self {
        Error::ExternalError(ExternalError::RusbError(err))
    }
}

// impl<T> From<PoisonError<T>> for Error {
//     fn from(err: PoisonError<T>) -> Self {
//         Error::RwLockPoisoned(err.to_string())
//     }
// }

impl From<broadcast::error::RecvError> for Error {
    fn from(err: broadcast::error::RecvError) -> Self {
        Error::ExternalError(ExternalError::ChannelReceiveError(err))
    }
}

impl From<broadcast::error::SendError<Box<[u8]>>> for Error {
    fn from(err: broadcast::error::SendError<Box<[u8]>>) -> Self {
        Error::ExternalError(ExternalError::ChannelSendError(err))
    }
}

impl From<Elapsed> for Error {
    fn from(err: Elapsed) -> Self {
        Error::AptError(AptError::AptProtocolError(err.to_string()))
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Error::ExternalError(ExternalError::TryFromSliceError(err))
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            _ => PyRuntimeError::new_err(err.to_string()),
        }
    }
}
