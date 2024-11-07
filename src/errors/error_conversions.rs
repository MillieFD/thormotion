/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: todo
Description: todo
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::errors::error_types::Error;
use pyo3::exceptions::{PyIOError, PyRuntimeError, PyValueError};
use pyo3::PyErr;
use std::sync::PoisonError;
use tokio::sync::oneshot::error::RecvError;

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            Error::DeviceNotFound(_) => PyIOError::new_err(err.message()),
            Error::DeviceNotSupported(_) => PyValueError::new_err(err.message()),
            Error::MessageIdNotFound(_) => PyValueError::new_err(err.message()),
            Error::MessageGroupNameNotFound(_) => PyValueError::new_err(err.message()),
            Error::DeviceMessageSendFailure(_, _) => PyRuntimeError::new_err(err.message()),
            Error::OneshotSenderExists(_, _) => PyRuntimeError::new_err(err.message()),
            Error::OneshotError(_) => PyRuntimeError::new_err(err.message()),
            Error::DeviceHardwareError(_, _) => PyIOError::new_err(err.message()),
            Error::MessageGroupNameAlreadyExists(_) => PyValueError::new_err(err.message()),
            Error::DeviceMessageReceiveFailure(_) => PyIOError::new_err(err.message()),
            Error::OneshotReceiverError(_) => PyRuntimeError::new_err(err.message()),
            Error::RwLockPoisoned(_) => PyRuntimeError::new_err(err.message()),
            Error::RusbError(_) => PyIOError::new_err(err.message()),
            Error::FatalError(_) => PyRuntimeError::new_err(err.message()),
        }
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

impl From<RecvError> for Error {
    fn from(err: RecvError) -> Self {
        Error::OneshotReceiverError(err.to_string())
    }
}

impl From<Box<[u8]>> for Error {
    fn from(unsent_message: Box<[u8]>) -> Self {
        Error::OneshotError(unsent_message)
    }
}
