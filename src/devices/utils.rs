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

use nusb::{DeviceInfo, list_devices};
use rustc_hash::FxBuildHasher;
use smol::lock::{Mutex, MutexGuard};

use crate::devices::device_manager::DeviceManager;
use crate::error::sn::Error;

/// Returns an iterator over all connected Thorlabs devices.
fn get_devices() -> impl Iterator<Item = DeviceInfo> {
    list_devices()
        .expect("Failed to list devices due to OS error")
        .filter(|dev| dev.vendor_id() == 0x0403)
}

/// Returns [`DeviceInfo`] for the Thorlabs device with the specified serial number.
///
/// Returns [`Error::NotFound`] if the specified device is not connected.
///
/// Returns [`Error::Multiple`] if more than one device with the specified serial number is found.
pub(super) fn get_device(serial_number: String) -> Result<DeviceInfo, Error> {
    let mut devices =
        get_devices().filter(|dev| dev.serial_number().map_or(false, |sn| sn == serial_number));
    match (devices.next(), devices.next()) {
        (None, _) => Err(Error::NotFound(serial_number)),
        (Some(device), None) => Ok(device),
        (Some(_), Some(_)) => Err(Error::Multiple(serial_number)),
    }
}

/// For convenience, this function prints a list of connected devices to stdout.
fn show_devices() {
    let devices = get_devices();
    for device in devices {
        println!("{:?}\n", device);
    }
}

/// Returns a locked [`MutexGuard`] containing the [Global Device Manager][`DEVICE_MANAGER`]
pub(super) async fn device_manager<'a>() -> MutexGuard<'a, DeviceManager> {
    crate::devices::device_manager::DEVICE_MANAGER
        .get_or_init(|| {
            Mutex::new(DeviceManager {
                devices: HashSet::with_hasher(FxBuildHasher::default()),
            })
        })
        .lock()
        .await
}

/// Safely stops all [Thorlabs devices][1], cleans up resources, and terminates
/// the program with an error message.
///
/// Internally, this function iterates over the global [DeviceManager] and calls
/// the respective `abort` function for each device.
///
/// ### Panics
///
/// This function always panics.
///
/// This is intended behaviour to safely unwind and free resources.
///
/// [1]: crate::traits::ThorlabsDevice
pub(crate) fn abort(message: String) -> ! {
    smol::block_on(async {
        for device in device_manager().await.devices.iter() {
            let _ = device.abort();
        }
    });
    panic!("Abort due to error : {}", message);
}

pub(crate) const BUG_MESSAGE: &str =
    "This is a bug. Please open a GitHub issue and report the relevant details";
