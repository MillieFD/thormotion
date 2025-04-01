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

use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::devices::UsbPrimitive;

pub trait ThorlabsDevice: Display + Send + Sync {
    /// Returns a borrow which dereferences to the inner [`UsbPrimitive`]
    fn inner(&self) -> &UsbPrimitive;

    /// Returns an [`Arc`] wrapping a dynamic trait object.
    ///
    /// This simplifies dynamic dispatch for code which can act on any
    /// [`Thorlabs Device`][ThorlabsDevice] type.
    fn as_dyn(&self) -> Arc<dyn ThorlabsDevice> {
        Arc::clone(self)
    }

    /// Returns a `&str` representing the device name, serial number, and current status.
    fn as_str(&self) -> &str;

    /// Safely brings the [`USB Device`][UsbPrimitive] to a resting state and releases the claimed
    /// [`Interface`][nusb::Interface].
    ///
    /// No action is taken if the device [`Status`][1] is already [`Closed`][2].
    ///
    /// Does not remove the device from the [`Global Device Manager`][3].
    /// You can use `open` to resume communication.
    ///
    /// To release the claimed [`Interface`][nusb::Interface] without bringing the device to a
    /// resting state, use `close`.
    ///
    /// [1]: crate::devices::usb_primitive::status::Status
    /// [2]: crate::devices::usb_primitive::status::Status::Closed
    /// [3]: crate::devices::device_manager::DeviceManager
    fn abort(&self);
}

impl Hash for dyn ThorlabsDevice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().serial_number().hash(state);
    }
}

impl PartialEq for dyn ThorlabsDevice {
    fn eq(&self, other: &Self) -> bool {
        self.inner().serial_number() == other.serial_number()
    }
}

impl Eq for dyn ThorlabsDevice {}
