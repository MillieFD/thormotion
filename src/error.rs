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
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Device not found: {0}")]
    NotFound(&'static str),
    #[error("rusb::Error: {0:?}")]
    RusbError(#[from] rusb::Error),
    #[error("std::io::Error: {0:?}")]
    IoError(#[from] std::io::Error),
    #[error("serialport::Error: {0:?}")]
    SerialPortError(#[from] serialport::Error),
}

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            Error::NotFound(msg) => PyIOError::new_err(msg),
            Error::RusbError(e) => PyIOError::new_err(e.to_string()),
            Error::IoError(e) => PyIOError::new_err(e.to_string()),
            Error::SerialPortError(e) => PyIOError::new_err(e.to_string()),
        }
    }
}
