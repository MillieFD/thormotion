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
use async_std::future::TimeoutError;
use pyo3::exceptions::PyRuntimeError;
use pyo3::PyErr;
use std::fmt::{Display, Formatter};
use std::time::Duration;

pub enum Error<'a, Sn>
where
    Sn: AsRef<str> + Display,
{
    /**
    OS error returned from USB operations other than transfers.
    Wrapper around `nusb::Error`.
    */
    UsbOperationError(nusb::Error),
    /**
    Error returned from USB transfers.
    Wrapper around `nusb::transfer::TransferError`.
    */
    UsbTransferError(nusb::transfer::TransferError),
    PoisonError(std::sync::PoisonError<std::sync::MutexGuard<'a, Duration>>),
    InvalidSerialNumber(Sn),
    DeviceNotFound(Sn),
    MultipleDevicesFound(Sn),
    Timeout(TimeoutError),
    ConversionError(Sn),
    UnsuccessfulCommand {
        device: &'a dyn ThorlabsDevice,
        message: Sn,
    },
}

impl<Sn> Display for Error<'_, Sn>
where
    Sn: AsRef<str> + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UsbOperationError(err) => write!(f, "nusb::Error: {}", err),
            Error::UsbTransferError(err) => write!(f, "nusb::transfer::TransferError: {}", err),
            Error::PoisonError(err) => write!(f, "std::sync::PoisonError: {}", err),
            Error::InvalidSerialNumber(sn) => write!(
                f,
                "{} is not a valid serial number for the requested Thorlabs device type.",
                sn
            ),
            Error::DeviceNotFound(sn) => {
                write!(f, "No devices with serial number {} were found", sn)
            }
            Error::MultipleDevicesFound(sn) => {
                write!(f, "Multiple devices with serial number {} were found", sn)
            }
            Error::Timeout(err) => write!(f, "Function timed out: {}", err),
            Error::ConversionError(err) => write!(f, "Conversion error: {}", err),
            Error::UnsuccessfulCommand { device, message } => write!(
                f,
                "APT Protocol command was unsuccessful\n    Device:  {}\n    Command: {}\n",
                device, message
            ),
        }
    }
}

impl<Sn> From<nusb::Error> for Error<'_, Sn>
where
    Sn: AsRef<str> + Display,
{
    fn from(err: nusb::Error) -> Self {
        Error::UsbOperationError(err)
    }
}

impl<Sn> From<nusb::transfer::TransferError> for Error<'_, Sn>
where
    Sn: AsRef<str> + Display,
{
    fn from(err: nusb::transfer::TransferError) -> Self {
        Error::UsbTransferError(err)
    }
}

impl<Sn> From<TimeoutError> for Error<'_, Sn>
where
    Sn: AsRef<str> + Display,
{
    fn from(err: TimeoutError) -> Self {
        Error::Timeout(err)
    }
}

impl<'a, Sn> From<std::sync::PoisonError<std::sync::MutexGuard<'a, Duration>>> for Error<'a, Sn>
where
    Sn: AsRef<str> + Display,
{
    fn from(err: std::sync::PoisonError<std::sync::MutexGuard<'a, Duration>>) -> Self {
        Error::PoisonError(err)
    }
}

impl<Sn> From<Error<'_, Sn>> for PyErr
where
    Sn: AsRef<str> + Display,
{
    fn from(err: Error<Sn>) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}
