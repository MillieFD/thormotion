/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: thorlabs_device.rs
Description: This file defines the ThorlabsDevice trait, which provides basic functionality that
is shared across all Thorlabs devices.
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::devices::UsbDevicePrimitive;
use crate::enumerate::get_device;
use crate::env::{DEST, SOURCE};
use crate::error::Error;
use crate::messages::get_rx_new_or_sub;
use crate::messages::ChannelStatus::{New, Sub};
use std::ops::Deref;

///

pub enum MsgFormat {
    Short([u8; 6]),
    Long(Vec<u8>),
}

impl Deref for MsgFormat {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match self {
            MsgFormat::Short(arr) => arr,
            MsgFormat::Long(vec) => vec.as_slice(),
        }
    }
}

impl Extend<u8> for MsgFormat {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        match self {
            MsgFormat::Short(arr) => {
                let mut vec = arr.to_vec();
                vec.extend(iter);
                *self = MsgFormat::Long(vec);
            }
            MsgFormat::Long(vec) => vec.extend(iter),
        }
    }
}

impl MsgFormat {
    pub(crate) fn len(&self) -> usize {
        match self {
            MsgFormat::Short(arr) => arr.len(),
            MsgFormat::Long(vec) => vec.len(),
        }
    }
}

///

#[derive(Debug, Clone)]
pub(crate) struct HwInfo {
    hardware_serial_number: u32,
    model_number: String,
    hardware_type: u16,
    firmware_version: String,
    hardware_version: u16,
    mod_state: u16,
    number_channels: u16,
}

///

pub trait ThorlabsDevice:
    From<UsbDevicePrimitive> + Deref<Target = UsbDevicePrimitive> + Send + Sync
{
    const SERIAL_NUMBER_PREFIX: &'static str;

    fn new(serial_number: &str) -> Result<Self, Error> {
        if !serial_number.starts_with(Self::SERIAL_NUMBER_PREFIX) {
            return Err(Error::InvalidSerialNumber(serial_number.to_string()));
        };
        let device = get_device(serial_number)?;
        Ok(Self::from(device))
    }

    fn pack_short_message(id: [u8; 2], param1: u8, param2: u8) -> MsgFormat {
        MsgFormat::Short([id[0], id[1], param1, param2, DEST, SOURCE])
    }

    fn pack_long_message(id: [u8; 2], length: usize) -> MsgFormat {
        let mut data: Vec<u8> = Vec::with_capacity(length);
        data.extend(id);
        data.extend(((length - 6) as u16).to_le_bytes());
        data.push(DEST | 0x80);
        data.push(SOURCE);
        MsgFormat::Long(data)
    }

    fn identify(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x23, 0x02];
        let data = Self::pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }

    fn hw_start_update_messages(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x11, 0x00];
        let data = Self::pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }

    fn hw_stop_update_messages(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x12, 0x00];
        let data = Self::pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }

    async fn hw_get_info(&self) -> Result<HwInfo, Error> {
        const ID: [u8; 2] = [0x00, 0x05];
        let mut rx = match get_rx_new_or_sub(ID)? {
            Sub(rx) => rx,
            New(rx) => {
                let data = Self::pack_short_message(ID, 0, 0);
                self.port_write(data)?;
                rx
            }
        };
        let response = rx.recv().await?;

        // Parse response
        let hardware_serial_number = u32::from_le_bytes(response[6..10].try_into().unwrap());
        let model_number = String::from_utf8_lossy(&response[10..18]).to_string();
        let hardware_type = u16::from_le_bytes(response[18..20].try_into().unwrap());
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

        Ok(HwInfo {
            hardware_serial_number,
            model_number,
            hardware_type,
            firmware_version,
            hardware_version,
            mod_state,
            number_channels,
        })
    }
}
