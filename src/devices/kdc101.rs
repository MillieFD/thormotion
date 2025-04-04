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

use std::fmt::{Display, Formatter};
use std::io::Error;
use std::sync::Arc;

use smol::block_on;

use crate::devices::{UsbPrimitive, add_device};
use crate::error::sn;
use crate::functions::*;
use crate::traits::{CheckSerialNumber, ThorlabsDevice, UnitConversion};

#[pyo3::pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KDC101 {
    inner: Arc<UsbPrimitive>,
}

impl KDC101 {
    const IDS: [[u8; 2]; 3] = [
        // MOD
        [0x23, 0x02], // IDENTIFY
        [0x12, 0x02], // GET_CHANENABLESTATE
        // MOT
        [0x44, 0x04], // MOVE_HOMED
    ];

    pub async fn new(serial_number: String) -> Result<Self, sn::Error> {
        Self::check_serial_number(&serial_number)?;
        let device = Self {
            inner: Arc::new(UsbPrimitive::new(serial_number.clone(), &Self::IDS)?),
        };
        let d = device.clone(); // Inexpensive Arc Clone
        let f = move || d.abort();
        add_device(serial_number.clone(), f).await;
        Ok(device)
    }

    /// Returns `True` if the [`USB Interface`][1] is open.
    ///
    /// [1]: nusb::Interface
    async fn is_open(&self) -> bool {
        self.inner.is_open().await
    }

    /// Opens an [`Interface`][1] to the [`USB Device`][2].
    ///
    /// No action is taken if the device [`Status`][3] is already [`Open`][4].
    ///
    /// [1]: nusb::Interface
    /// [2]: UsbPrimitive
    /// [3]: crate::devices::usb_primitive::status::Status
    /// [4]: crate::devices::usb_primitive::status::Status::Open
    pub async fn open(&mut self) -> Result<(), Error> {
        self.inner.open().await
    }

    /// Releases the claimed [`Interface`][1] to the [`USB Device`][2].
    ///
    /// No action is taken if the device [`Status`][3] is already [`Closed`][4].
    ///
    /// This does not stop the device's current action. If you need to safely bring the
    /// [`USB Device`][2] to a resting state, see [`abort`][5].
    ///
    /// [1]: nusb::Interface
    /// [2]: UsbPrimitive
    /// [3]: crate::devices::usb_primitive::status::Status
    /// [4]: crate::devices::usb_primitive::status::Status::Closed
    /// [5]: ThorlabsDevice::abort
    async fn close(&mut self) -> Result<(), Error> {
        self.inner.close().await
    }

    /// Identifies the device by flashing the front panel LED.
    pub async fn identify(&self) {
        __identify(self, 1).await;
    }

    /// Returns `True` if the specified device channel is enabled.
    pub async fn get_channel_enable_state(&self) {
        __req_channel_enable_state(self, 1).await;
    }

    /// Enables or disables the specified device channel.
    pub async fn set_channel_enable_state(&self, enable: bool) {
        __set_channel_enable_state(self, 1, enable).await;
    }

    /// Homes the specified device channel.
    pub async fn home(&self) {
        __home(self, 0).await;
    }
}

impl ThorlabsDevice for KDC101 {
    fn inner(&self) -> &UsbPrimitive {
        &self.inner
    }

    fn channels(&self) -> u8 {
        1
    }

    fn abort(&self) {
        // todo()!
    }
}

impl CheckSerialNumber for KDC101 {
    const SERIAL_NUMBER_PREFIX: &'static str = "27";
}

impl UnitConversion for KDC101 {
    const DISTANCE_ANGLE_SCALE_FACTOR: f64 = 34554.96;
    const VELOCITY_SCALE_FACTOR: f64 = 772981.3692;
    const ACCELERATION_SCALE_FACTOR: f64 = 263.8443072;
}

impl Display for KDC101 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&block_on(async {
            format!("KDC101 ({})", self.inner.to_string()) // See Display trait impl for UsbPrimitive
        }))
    }
}
