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
    DeviceError(String),
    EnumerationError(String),
    InvalidMessageId([u8; 2]),
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
            Error::DeviceError(msg) => {
                format!("Error occurred whilst communicating with device: {msg}")
            }
            Error::EnumerationError(msg) => {
                format!("Error occurred during device enumeration: {}", msg)
            }
            Error::InvalidMessageId(id) => {
                format!("{id:?} does not correspond to a known message ID")
            }
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
