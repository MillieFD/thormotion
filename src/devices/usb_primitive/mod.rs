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

pub mod communicator;

use crate::error::{sn, usb};
use crate::messages::{Communicator, Dispatcher};
use nusb::DeviceInfo;
use std::fmt::Debug;

enum Status {
    Open(Communicator),
    Closed,
}

impl Default for Status {
    fn default() -> Self {
        Self::Closed
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open(_) => write!(f, "Status::Open"),
            Self::Closed => write!(f, "Status::Closed"),
        }
    }
}

#[derive(Debug)]
pub struct UsbPrimitive {
    /**
    Information about a device that can be obtained without calling [`DeviceInfo::open`].
    */
    pub(super) device_info: DeviceInfo,
    /**
    A thread-safe message dispatcher for handling async `Req → Get` callback patterns.
    */
    dispatcher: Dispatcher,
    /**
    Contains [`Communicator`] if the USB device is open, or [`None`] if the USB device is closed.
    Open the device by calling [`UsbPrimitive::open`]
    */
    status: Status,
}

impl UsbPrimitive {
    fn into_device_info(self) -> DeviceInfo {
        self.device_info
    }

    pub(super) fn serial_number(&self) -> Result<&str, sn::Error> {
        self.device_info
            .serial_number()
            .ok_or(sn::Error::Unknown(self.device_info.clone()))
    }

    fn is_open(&self) -> bool {
        match self.status {
            Status::Open(_) => true,
            Status::Closed => false,
        }
    }

    async fn open(&mut self) -> Result<(), usb::Error> {
        if self.is_open() {
            return Ok(());
        }
        let interface = self.device_info.open()?.detach_and_claim_interface(0)?;
        let dispatcher = self.dispatcher.clone();
        let communicator = Communicator::new(interface, dispatcher);
        self.status = Status::Open(communicator);
        Ok(())
    }

    fn close(&self) {}
}
