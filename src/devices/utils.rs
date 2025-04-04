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

use std::sync::OnceLock;

use ahash::{HashMap, HashMapExt};
use nusb::{list_devices, DeviceInfo};
use smol::lock::Mutex;

use crate::error::sn::Error;

/// A lazily initialized [`HashMap`] containing the `serial number` (key) and [`abort function`][1]
/// (value) for each connected [`Thorlabs Device`][2]. It is protected by an async [`Mutex`] for
/// thread-safe concurrent access.
///
/// The [`HashMap`] is only accessed when connecting or disconnecting [`Thorlabs Devices`][2]. The
/// [`HashMap`] is not required when [`opening`][3], [`closing`][4], or [`sending`][5] commands to
/// the device. As such, lock contention does not affect device latency.
///
/// If an irrecoverable error occurs anywhere in the program, this triggers the [`global_abort`]
/// function which safely [`aborts`][1] each device, bringing the system to a controlled stop.
///
/// [1]: crate::traits::ThorlabsDevice::abort
/// [2]: crate::traits::ThorlabsDevice
/// [3]: crate::devices::UsbPrimitive::open
/// [4]: crate::devices::UsbPrimitive::close
/// [5]: crate::devices::UsbPrimitive::send
#[doc(hidden)]
static DEVICES: OnceLock<Mutex<HashMap<String, Box<dyn Fn() + Send + 'static>>>> = OnceLock::new();

/// Adds a new [`Thorlabs Device`][1] `serial number` (key) and corresponding [`abort`][2] function
/// (value) to the global [`DEVICES`][3] [`HashMap`].
///
/// [1]: crate::traits::ThorlabsDevice
/// [2]: crate::traits::ThorlabsDevice::abort
/// [3]: DEVICES
#[doc(hidden)]
pub(super) async fn add_device<F>(serial_number: String, f: F)
where
    F: Fn() + Send + 'static,
{
    DEVICES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .await
        .insert(serial_number, Box::new(f));
}

// SAFETY: This is a placeholder function. DO NOT USE. Devices are only removed from the global 
// DEVICES map when they are dropped, which is handled by the `drop` function.
/// Removes a [`Thorlabs Device`][1] from the global [`DEVICES`][2] [`HashMap`].
///
/// [1]: crate::traits::ThorlabsDevice
/// [2]: DEVICES
#[doc(hidden)]
async fn remove_device(serial_number: &str) {
    DEVICES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .await
        .remove(serial_number);
}

/// Calls the [`abort`][1] function for the specified [`Thorlabs Device`][1].
///
/// The device is not removed from the global [`DEVICES`][2] [`HashMap`]. You can use
/// [`Open`][3] to resume communication.
///
/// [1]: crate::traits::ThorlabsDevice::abort
/// [2]: DEVICES
/// [3]: crate::devices::UsbPrimitive::open
#[doc(hidden)]
pub(super) async fn abort_device(serial_number: &str) {
    if let Some(f) = DEVICES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .await
        .get(serial_number)
    {
        f()
    }
}

pub(super) async fn drop_device(serial_number: &str) {
    if let Some(f) = DEVICES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .await
        .remove(serial_number)
    {
        f()
    }
}

/// Safely stops all [`Thorlabs devices`][1], cleans up resources, and terminates the program with
/// an error message.
///
/// Internally, this function iterates over the global [`DEVICES`][2] [`HashMap`] and calls the
/// respective [`abort`][3] function for each device.
///
/// ### Panics
///
/// This function always panics.
///
/// This is intended behaviour to safely unwind and free resources.
///
/// [1]: crate::traits::ThorlabsDevice
/// [2]: DEVICES
/// [3]: crate::traits::ThorlabsDevice::abort
#[doc(hidden)]
pub(crate) fn global_abort(message: String) -> ! {
    smol::block_on(async {
        DEVICES
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .await
            .drain()
            .for_each(|(_, f)| {
                f();
            });
    });
    panic!("\nAbort due to error : {}\n", message);
}

#[doc(hidden)]
pub(crate) const BUG: &str = "This is a bug. If you are able to reproduce this issue, please open \
                              a new GitHub issue and report the relevant details";

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
pub(super) fn get_device(serial_number: &String) -> Result<DeviceInfo, Error> {
    let mut devices =
        get_devices().filter(|dev| dev.serial_number().map_or(false, |sn| sn == serial_number));
    match (devices.next(), devices.next()) {
        (None, _) => Err(Error::NotFound(serial_number.clone())),
        (Some(device), None) => Ok(device),
        (Some(_), Some(_)) => Err(Error::Multiple(serial_number.clone())),
    }
}

/// For convenience, this function prints a list of connected devices to stdout.
pub fn show_devices() {
    let devices = get_devices();
    for device in devices {
        println!("{:?}\n", device);
    }
}
