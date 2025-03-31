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

use std::fmt;
use std::fmt::Display;

use super::communicator::Communicator;
use crate::messages::Dispatcher;

/// The current device status.
///
/// - [`Open`][`Status::Open`] → Contains an active [`Communicator`]
/// - [`Closed`][`Status::Closed`] → Contains an idle [`Dispatcher`]
///
/// Open the device by calling [`open`][`UsbPrimitive::open`]
pub(super) enum Status {
    /// The [`Interface`][nusb::Interface] is [`open`][`crate::devices::UsbPrimitive::open`] and
    /// communicating. This enum variant contains an active [`Communicator`].
    Open(Communicator),
    /// The [`Interface`][nusb::Interface] is [`closed`][`crate::devices::UsbPrimitive::close`].
    /// This enum variant contains an idle [`Dispatcher`].
    Closed(Dispatcher),
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Open(_) => "Open",
            Self::Closed(_) => "Closed",
        }
    }
}

impl Display for Status {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}
