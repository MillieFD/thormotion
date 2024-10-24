/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: error
Description: This file contains custom error types and functions to handle errors.
---------------------------------------------------------------------------------------------------
Notes:
*/

use pyo3::exceptions::PyIOError;
use pyo3::PyErr;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum Error {
    NotFound(&'static str),
    External(String),
    Path(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound(msg) => write!(f, "NotFound error: {}", msg),
            Error::External(msg) => write!(f, "External error: {}", msg),
            Error::Path(msg) => {
                write!(f, "Path error: {}", msg)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<rusb::Error> for Error {
    fn from(err: rusb::Error) -> Self {
        Error::External(format!("Error from rusb: {:?}", err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::External(format!("Error from std::io: {:?}", err))
    }
}

impl From<serialport::Error> for Error {
    fn from(err: serialport::Error) -> Self {
        Error::External(format!("Error from serialport: {:?}", err))
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            Error::NotFound(msg) => PyIOError::new_err(msg),
            Error::External(msg) => PyIOError::new_err(msg),
            Error::Path(msg) => PyIOError::new_err(msg),
        }
    }
}
