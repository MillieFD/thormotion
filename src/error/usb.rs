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

use nusb::transfer::TransferError;

#[derive(Debug)]
pub enum Error {
    /**
    An OS error returned from USB operations other than transfers.
    Wrapper around [`nusb::Error`].
    */
    OperatingSystem(nusb::Error),
    /**
    An error returned from USB transfers.
    Wrapper around [`TransferError`].
    */
    Transfer(TransferError),
    /**
    An error returned if the [`nusb::Device`] field of [`UsbPrimitive`] is `None`.
    */
    NoDevice,
    /**
    An error returned if the [`nusb::Interface`] field of [`UsbPrimitive`] is `None`.
    */
    NoInterface,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::OperatingSystem(err) => write!(f, "{}", err),
            Error::Transfer(err) => write!(f, "{}", err),
            Error::NoDevice => write!(
                f,
                "The `nusb::Device` field of `UsbPrimitive` is `None`.\nHave you called \
                 `UsbPrimitive::open()`?"
            ),
            Error::NoInterface => write!(
                f,
                "The `nusb::Interface` field of `UsbPrimitive` is `None`.\nHave you called \
                 `UsbPrimitive::open()`?"
            ),
        }
    }
}

impl std::error::Error for Error {}

/* ----------------------------------------------------------------------------- From USB errors */

impl From<TransferError> for Error {
    fn from(err: TransferError) -> Self {
        Self::Transfer(err)
    }
}

impl From<nusb::Error> for Error {
    fn from(err: nusb::Error) -> Self {
        Self::OperatingSystem(err)
    }
}
