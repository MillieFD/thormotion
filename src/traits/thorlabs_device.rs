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

use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};

use crate::devices::UsbPrimitive;

pub trait ThorlabsDevice: Display + Debug + Send + Sync {
    /// Returns a borrow which dereferences to the inner [`UsbPrimitive`]
    fn inner(&self) -> &UsbPrimitive;

    /// Returns the serial number of the device as a `&str`.
    fn serial_number(&self) -> &str {
        self.inner().serial_number()
    }

    /// Returns the number of device channels.
    // Use a hardcoded const u8
    fn channels(&self) -> u8;

    /// Safely brings the [`USB Device`][1] to a resting state and releases the claimed
    /// [`Interface`][2].
    ///
    /// If the device [`Status`][3] is [`Closed`][4], a temporary [`Interface`][2] is [`Opened`][5]
    /// to send the abort command.
    ///
    /// Does not remove the device from the global [`DEVICES`][6] [`HashMap`][7]. You can use
    /// [`Open`][5] to resume communication.
    ///
    /// To release the claimed [`Interface`][2] without bringing the device to a resting state,
    /// use `close`.
    ///
    /// [1]: UsbPrimitive
    /// [2]: nusb::Interface
    /// [3]: crate::devices::usb_primitive::status::Status
    /// [4]: crate::devices::usb_primitive::status::Status::Closed
    /// [5]: UsbPrimitive::open
    /// [6]: crate::devices::utils::DEVICES
    /// [7]: ahash::HashMap
    fn abort(&self);
}

impl Hash for dyn ThorlabsDevice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().serial_number().hash(state);
    }
}

impl PartialEq for dyn ThorlabsDevice {
    fn eq(&self, other: &Self) -> bool {
        self.inner().serial_number() == other.inner().serial_number()
    }
}

impl Eq for dyn ThorlabsDevice {}
