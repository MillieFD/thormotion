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

use crate::devices::UsbDevicePrimitive;
use crate::error::Error;
use rusb::DeviceList;
use std::time::Duration;

const VENDOR_ID: u16 = 0x0403;

/**
Finds a specific `UsbDevicePrimitive` using its serial number.

This function is only intended for internal use, and should not be called directly by end users.
It finds any `rusb::Device` instances with the Thorlabs `VENDOR_ID` and the specified
serial number. It does not check that the specified serial number is valid for any particular
Thorlabs device type. It returns `Ok(UsbDevicePrimitive)` if exactly one matching device is found.
The `UsbDevicePrimitive` is not wrapped in a Thorlabs device struct,
so does not have access to any Thorlabs APT Protocol internal.

# Errors

This function can return three different `Error` variants:
- `DeviceNotFound`: A USB device with the specified serial number cannot be found.
- `MultipleDevicesFound`: More than one device matches the specified serial number,
leading to ambiguity.
- `RusbError`: If `rusb` is unable to read the device descriptor, open the device,
or fetch its serial number.

# Steps

1. Enumerates all connected USB devices.
2. Filters devices by the Thorlabs vendor ID (`VENDOR_ID`).
3. Compares the serial number of each device against the provided string.
4. If a single match is found, constructs and returns a `UsbDevicePrimitive`.

# Examples

```rust
let serial = "123456";
match get_device_primitive(serial) {
    Ok(usb_device_primitive) => println!("Device found: {}", usb_device_primitive),
    Err(e) => eprintln!("Error: {}", e),
};
```
*/
pub(crate) fn get_usb_device_primitive<A>(serial_number: A) -> Result<UsbDevicePrimitive, Error>
where
    A: Into<String> + Clone,
{
    let devices: Vec<UsbDevicePrimitive> = DeviceList::new()?
        .iter()
        .filter_map(|rusb_device| {
            let descriptor = rusb_device.device_descriptor().ok()?;
            if descriptor.vendor_id() != VENDOR_ID {
                return None;
            }
            let handle = rusb_device.open().ok()?;
            let language = handle
                .read_languages(Duration::from_millis(500))
                .ok()?
                .get(0)
                .copied()?;
            let device_serial_number = handle
                .read_serial_number_string(language, &descriptor, Duration::from_millis(500))
                .ok()?;
            if device_serial_number != serial_number.clone().into() {
                return None;
            }
            let usb_device_primitive = UsbDevicePrimitive::new(handle, descriptor, language);
            Some(usb_device_primitive)
        })
        .collect();
    match devices.len() {
        0 => Err(Error::DeviceNotFound(serial_number.into())),
        1 => Ok(devices
            .into_iter()
            .next()
            .ok_or(Error::DeviceNotFound(serial_number.into()))?),
        _ => Err(Error::MultipleDevicesFound(serial_number.into())),
    }
}
