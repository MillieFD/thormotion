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

use nusb::{DeviceInfo, list_devices};

use crate::error::sn::Error;

fn get_devices() -> impl Iterator<Item = DeviceInfo> {
    list_devices()
        .expect("Failed to list devices due to OS error")
        .filter(|dev| dev.vendor_id() == 0x0403)
}

pub(crate) fn get_device(serial_number: String) -> Result<DeviceInfo, Error> {
    let mut devices =
        get_devices().filter(|dev| dev.serial_number().map_or(false, |sn| sn == serial_number));
    match (devices.next(), devices.next()) {
        (None, _) => Err(Error::NotFound(serial_number)),
        (Some(device), None) => Ok(device),
        (Some(_), Some(_)) => Err(Error::Multiple(serial_number)),
    }
}

fn show_devices() {
    let devices = get_devices();
    for device in devices {
        println!("{:?}\n", device);
    }
}
