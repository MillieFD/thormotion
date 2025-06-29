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

use std::fmt::{Debug, Display, Formatter};

use nusb::DeviceInfo;
use pyo3::PyErr;

type Sn = String;

#[derive(Debug)]
pub enum Error {
    Invalid(Sn),
    Multiple(Sn),
    NotFound(Sn),
    Unknown(DeviceInfo),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Invalid(sn) => write!(
                f,
                "{:?} is not a valid serial number for the requested Thorlabs device type.",
                sn
            ),
            Error::Multiple(sn) => {
                write!(f, "Multiple devices found with serial number {}", sn)
            }
            Error::NotFound(sn) => {
                write!(f, "No devices found with serial number {}", sn)
            }
            Error::Unknown(dev) => {
                write!(f, "Serial number could not be read from device {:?}", dev)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<Error> for PyErr {
    fn from(error: Error) -> Self {
        pyo3::exceptions::PyException::new_err(error.to_string())
    }
}
