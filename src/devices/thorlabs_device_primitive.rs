/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: todo
Description: todo
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::enumeration::usb_device_primitive::UsbDevicePrimitive;
use crate::errors::error_types::Error;
use std::ops::Deref;

pub(crate) struct ThorlabsDevicePrimitive {
    device: UsbDevicePrimitive,
    hardware_serial_number: u32,
    model_number: String,
    hardware_type: u16,
    firmware_version: String,
    hardware_version: u16,
    mod_state: u16,
    number_channels: u16,
}

impl Deref for ThorlabsDevicePrimitive {
    type Target = UsbDevicePrimitive;
    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl ThorlabsDevicePrimitive {
    pub(crate) async fn new(device: UsbDevicePrimitive) -> Result<Self, Error> {
        let (
            hardware_serial_number,
            model_number,
            hardware_type,
            firmware_version,
            hardware_version,
            mod_state,
            number_channels,
        ) = get_hw_info_static(&device).await?;

        Ok(Self {
            device,
            hardware_serial_number,
            model_number,
            hardware_type,
            firmware_version,
            hardware_version,
            mod_state,
            number_channels,
        })
    }

    async fn get_hw_info(&self) -> Result<(u32, String, u16, String, u16, u16, u16), Error> {
        Ok(get_hw_info_static(&self.device).await?)
    }
}

fn pack_short_message(id: u16, param1: u8, param2: u8) -> [u8; 6] {
    let id_le_bytes = id.to_le_bytes();
    [
        id_le_bytes[0],
        id_le_bytes[1],
        param1,
        param2,
        crate::env::DEST,
        crate::env::SOURCE,
    ]
}

fn pack_long_message(id: u16) {} // todo implement

async fn get_hw_info_static(
    device: &UsbDevicePrimitive,
) -> Result<(u32, String, u16, String, u16, u16, u16), Error> {
    let message = pack_short_message(0x0005, 0, 0);
    let response = device.write_port(Box::new(message))?.await?;

    // Parse response
    let serial_number = u32::from_le_bytes(response[6..10].try_into().unwrap());
    let model_number = String::from_utf8_lossy(&response[10..18]).to_string();
    let device_type = u16::from_le_bytes(response[18..20].try_into().unwrap());
    let firmware_minor_revision = u8::from_le_bytes(response[20..21].try_into().unwrap());
    let firmware_interim_revision = u8::from_le_bytes(response[21..22].try_into().unwrap());
    let firmware_major_revision = u8::from_le_bytes(response[22..23].try_into().unwrap());
    let firmware_version: String = format!(
        "{}.{}.{}",
        firmware_major_revision, firmware_interim_revision, firmware_minor_revision
    );
    let hardware_version = u16::from_le_bytes(response[84..86].try_into().unwrap());
    let mod_state = u16::from_le_bytes(response[86..88].try_into().unwrap());
    let number_channels = u16::from_le_bytes(response[88..90].try_into().unwrap());

    Ok((
        serial_number,
        model_number,
        device_type,
        firmware_version,
        hardware_version,
        mod_state,
        number_channels,
    ))
}
