/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use crate::traits::ThorlabsDevice;
use async_channel::RecvError;
use async_std::future::TimeoutError;
use pyo3::PyErr;
use pyo3::exceptions::PyRuntimeError;
use std::array::TryFromSliceError;
use std::fmt::Debug;
use std::num::TryFromIntError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    RUSB(#[from] rusb::Error),

    #[error("{0} is not a valid serial number for the requested Thorlabs device type.")]
    InvalidSerialNumber(String),

    #[error("No devices with serial number {0} were found")]
    DeviceNotFound(String),

    #[error("Multiple devices with serial number {0} were found")]
    MultipleDevicesFound(String),

    #[error("Function timed out")]
    Timeout(#[from] TimeoutError),

    #[error(
        "A fatal error occurred: {0}\n\
        This is a bug. Please open a [GitHub issue](https://github.com/MillieFD/thormotion/issues)."
    )]
    FatalError(String),

    #[error("Conversion was unsuccessful: {0}")]
    ConversionError(String),

    #[error("APT Protocol command to {} was unsuccessful: {}", device, error)]
    UnsuccessfulCommand { device: String, error: String },
}

impl Error {
    pub(crate) fn wrong_channel<A, B, C>(device: &A, requested: B, received: C) -> Self
    where
        A: ThorlabsDevice,
        B: Debug,
        C: Debug,
    {
        Error::UnsuccessfulCommand {
            device: device.to_string(),
            error: format!(
                "Requested channel {:?} but device responded with channel {:?}",
                requested, received
            ),
        }
    }

    pub(crate) fn unsuccessful_command<A, B>(device: &A, message: B) -> Self
    where
        A: ThorlabsDevice,
        B: Into<String>,
    {
        Error::UnsuccessfulCommand {
            device: device.to_string(),
            error: message.into(),
        }
    }
}

impl From<RecvError> for Error {
    fn from(err: RecvError) -> Error {
        Error::FatalError(err.to_string())
    }
}

impl From<TryFromSliceError> for Error {
    fn from(err: TryFromSliceError) -> Error {
        Error::ConversionError(err.to_string())
    }
}

impl From<TryFromIntError> for Error {
    fn from(err: TryFromIntError) -> Self {
        Error::ConversionError(err.to_string())
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}
