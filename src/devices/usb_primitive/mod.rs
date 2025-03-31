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
mod status;

use std::io;

use communicator::Communicator;
use nusb::DeviceInfo;
use status::Status;

use crate::devices::get_device;
use crate::error::sn;
use crate::messages::Dispatcher;

pub(super) struct UsbPrimitive {
    /**
    Information about a device that can be obtained without calling [`open`][`DeviceInfo::open`].
    */
    device_info: DeviceInfo,
    /**
    A thread-safe message [`Dispatcher`] for handling async `Req → Get` callback patterns.
    */
    dispatcher: Dispatcher,
    /// The current device status.
    /// Can be [`Open`][`Status::Open`] or [`Closed`][`Status::Closed`].
    /// Open the device by calling [`open`][`UsbPrimitive::open`]
    status: Status,
}

impl UsbPrimitive {
    pub(super) fn new(serial_number: String, ids: &[[u8; 2]]) -> Result<Self, sn::Error> {
        Ok(Self {
            device_info: get_device(serial_number)?,
            status: Status::Closed,
            dispatcher: Dispatcher::from(ids),
        })
    }

    pub(super) fn serial_number(&self) -> &str {
        self.device_info.serial_number().unwrap_or_else(|| {
            // SAFETY: The USB device must report its serial number during enumeration with
            // devices::utils::get_device. Thus, DeviceInfo::serial_number should never fail.
            panic!(
                "Serial number could not be read from device {:?}",
                self.device_info
            )
        })
    }

    fn is_open(&self) -> bool {
        match self.status {
            Status::Open(_) => true,
            Status::Closed => false,
        }
    }

    async fn open(&mut self) -> Result<(), io::Error> {
        match &self.status {
            Status::Open(_) => Ok(()),
            Status::Closed => {
                let interface = self.device_info.open()?.detach_and_claim_interface(0)?;
                let dispatcher = self.dispatcher.clone();
                self.status = Status::Open(Communicator::new(interface, dispatcher).await);
                Ok(())
            }
        }
    }

    fn close(&mut self) -> Result<(), io::Error> {
        match &mut self.status {
            Status::Open(_) => {
                self.status = Status::Closed;
                Ok(())
            }
            Status::Closed => Ok(()),
        }
    }
}
