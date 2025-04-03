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

use crate::devices::{add_device, UsbPrimitive};
use crate::error::sn;
use crate::functions::*;
use crate::traits::{CheckSerialNumber, ThorlabsDevice, UnitConversion};

#[pyo3::pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KDC101 {
    inner: Arc<UsbPrimitive>,
}

impl KDC101 {
    const IDS: [[u8; 2]; 2] = [
        // MOD
        [0x23, 0x02], // IDENTIFY
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

    async fn is_open(&self) -> bool {
        self.inner.is_open().await
    }

    /// todo Opens a connection to the device
    pub async fn open(&mut self) -> Result<(), Error> {
        self.inner.open().await
    }

    /// todo Closes the connection to the device
    /// Releases the claimed interface
    /// Does not stop the device's current action (use abort instead)
    ///
    /// Does not stop the device's current action.
    ///
    /// For example, you can tell the
    /// device to `HOME` and then `close` the interface without waiting for the task to complete.
    /// The device will continue to `HOME` after losing the connection. Once homing is complete,
    /// the device will send the `HOMED` command to the closed buffer.
    ///
    /// To stop the device's current action, use [`Self::abort`]
    async fn close(&mut self) -> Result<(), Error> {
        self.inner.close().await
    }

    pub async fn identify(&self) {
        __identify(self, 0).await;
    }

    pub async fn home(&self) {
        __home(self, 0).await;
    }
}

impl ThorlabsDevice for KDC101 {
    fn inner(&self) -> &UsbPrimitive {
        &self.inner
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
