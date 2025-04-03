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

use std::collections::HashMap;
use std::sync::OnceLock;

use rustc_hash::{FxBuildHasher, FxHashMap};
use smol::lock::{Mutex, MutexGuard};

/// For convenience, a global [`DeviceManager`] is lazily initialized on first use.
/// It is protected by an asynchronous [`Mutex`] for thread-safe concurrent access.
pub(super) static DEVICE_MANAGER: OnceLock<Mutex<DeviceManager>> = OnceLock::new();

/// Manages a [`HashMap`] containing the `serial number` (key) and `abort function` (value) for
/// each connected [`Thorlabs Device`][1].
///
/// For convenience, a [`Global Device Manager`][2] is lazily initialized on first use.
///
/// If an error occurs anywhere in the program, this is sent to the Global Device Manager, which
/// can then safely [`abort`][3] all devices, bringing the system to a controlled stop.
///
/// [1]: crate::devices::UsbPrimitive
/// [2]: DEVICE_MANAGER
/// [3]: crate::traits::ThorlabsDevice::abort
#[derive(Default)]
pub(super) struct DeviceManager<'a> {
    /// A [`HashMap`] containing the `serial number` (key) and `abort function` (value) for each
    /// connected [`Thorlabs Device`][1].
    ///
    /// [1]: crate::devices::UsbPrimitive
    pub(super) devices: FxHashMap<String, Box<dyn FnOnce() + Send + 'a>>,
}

impl<'a> DeviceManager<'a> {
    /// Adds a new [Thorlabs Device][1] `serial number` (key) and corresponding  `abort function`
    /// (value) to the [Global Device Manager][2].
    ///
    /// [1]: crate::devices::UsbPrimitive
    /// [2]: DEVICE_MANAGER
    pub(super) fn add<Fn>(&mut self, serial_number: String, f: Fn)
    where
        Fn: FnOnce() + Send + 'a,
    {
        self.devices.insert(serial_number, Box::new(f));
    }

    /// Removes a [`ThorlabsDevice`] from the global [`DeviceManager`].
    pub(super) fn remove(&mut self, serial_number: &str) {
        let _ = self.devices.remove(serial_number);
    }
}

/// Returns a locked [`MutexGuard`] containing the [Global Device Manager][1].
///
/// [1]: DEVICE_MANAGER
pub(super) async fn device_manager<'a>() -> MutexGuard<'a, DeviceManager<'static>> {
    DEVICE_MANAGER
        .get_or_init(|| {
            Mutex::new(DeviceManager {
                devices: HashMap::with_hasher(FxBuildHasher::default()),
            })
        })
        .lock()
        .await
}
