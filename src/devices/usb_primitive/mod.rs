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
mod serial_port;
mod status;

use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::io;

use communicator::Communicator;
use nusb::DeviceInfo;
use status::Status;

use crate::devices::get_device;
use crate::error::sn;
use crate::messages::Dispatcher;

pub(super) struct UsbPrimitive {
    /// Information about a device that can be obtained without calling [`DeviceInfo::open`].
    device_info: DeviceInfo,
    /// The current device status.
    ///
    /// - [`Open`][`Status::Open`] → Contains an active [`Communicator`]
    /// - [`Closed`][`Status::Closed`] → Contains an idle [`Dispatcher`]
    ///
    /// Open the device by calling [`open`][`UsbPrimitive::open`]
    status: Status,
}

impl UsbPrimitive {
    pub(super) fn new(serial_number: String, ids: &[[u8; 2]]) -> Result<Self, sn::Error> {
        let dispatcher = Dispatcher::new(ids);
        Ok(Self {
            device_info: get_device(serial_number)?,
            status: Status::Closed(dispatcher),
        })
    }

    fn to_str(&self) -> &str {
        format!("{:?} | {}", self.device_info, self.status.as_str()).as_str()
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
            Status::Closed(_) => false,
        }
    }

    pub(super) async fn open(&mut self) -> Result<(), io::Error> {
        match &self.status {
            Status::Open(_) => Ok(()), // No-op: Nothing to do here
            Status::Closed(dsp) => {
                let interface = self.device_info.open()?.detach_and_claim_interface(0)?;
                let dispatcher = dsp.clone(); // Inexpensive Arc Clone
                self.status = Status::Open(Communicator::new(interface, dispatcher).await);
                Ok(())
            }
        }
    }

    fn close(&mut self) -> Result<(), io::Error> {
        match &mut self.status {
            Status::Open(communicator) => {
                let dispatcher = communicator.get_dispatcher();
                self.status = Status::Closed(dispatcher);
                Ok(())
            }
            Status::Closed(_) => Ok(()), // No-op: Nothing to do here
        }
    }
}

impl PartialEq<UsbPrimitive> for UsbPrimitive {
    fn eq(&self, other: &Self) -> bool {
        self.device_info.vendor_id() == other.device_info.vendor_id()
            && self.device_info.product_id() == other.device_info.product_id()
            && self.device_info.serial_number().unwrap_or("")
                == other.device_info.serial_number().unwrap_or("")
    }
}

impl Eq for UsbPrimitive {}

impl Hash for UsbPrimitive {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.device_info.vendor_id().hash(state);
        self.device_info.product_id().hash(state);
        self.device_info.serial_number().unwrap_or("").hash(state);
    }
}

impl Debug for UsbPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

impl Display for UsbPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}
