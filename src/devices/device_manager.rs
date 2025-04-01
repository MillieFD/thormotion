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

use std::collections::HashSet;
use std::sync::{Arc, OnceLock};

use rustc_hash::{FxBuildHasher, FxHashSet};
use smol::lock::{Mutex, MutexGuard};

use crate::traits::ThorlabsDevice;

/// For convenience, a global [`DeviceManager`] is lazily initialized on first use.
/// It is protected by an asynchronous [`Mutex`] for thread-safe concurrent access.
static DEVICE_MANAGER: OnceLock<Mutex<DeviceManager>> = OnceLock::new();

/// Returns a locked [`MutexGuard`] containing the [Global Device Manager][`DEVICE_MANAGER`]
pub(super) async fn device_manager<'a>() -> MutexGuard<'a, DeviceManager> {
    DEVICE_MANAGER
        .get_or_init(|| {
            Mutex::new(DeviceManager {
                devices: HashSet::with_hasher(FxBuildHasher::default()),
            })
        })
        .lock()
        .await
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct DeviceManager {
    /// A [`HashSet`][`FxHashSet`] containing an [`Arc`] to each [Thorlabs Device][ThorlabsDevice].
    ///
    /// [open]: crate::devices::UsbPrimitive::open
    devices: FxHashSet<Arc<dyn ThorlabsDevice>>,
}

impl DeviceManager {
    /// Adds a new [Thorlabs Device][`ThorlabsDevice`] to the
    /// [Global Device Manager][`DEVICE_MANAGER`].
    pub(super) fn add(&mut self, device: Arc<dyn ThorlabsDevice>) {
        self.devices.insert(device);
    }

    /// Remove a [Thorlabs Device][`ThorlabsDevice`] from the
    /// [Global Device Manager][`DEVICE_MANAGER`].
    pub(super) fn remove(&mut self, device: Arc<dyn ThorlabsDevice>) {
        self.devices.remove(&device);
    }
}
