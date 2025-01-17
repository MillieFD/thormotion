/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: traits.rs
Description: This file defines traits for Thorlabs devices. Each trait contains functions which can
 be called by Thorlabs devices that implement the trait.
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::devices::{HardwareInfo, UsbDevicePrimitive};
use crate::enumerate::get_device;
use crate::env::{DEST, LONG_TIMEOUT, SHORT_TIMEOUT, SOURCE};
use crate::messages::ChannelStatus::{New, Sub};
use crate::messages::{get_rx_new_or_err, get_rx_new_or_sub, MsgFormat};
use crate::Error;
use std::ops::Deref;
use tokio::time::timeout;

/// # Thorlabs Device
/// The `ThorlabsDevice` trait is a base trait implemented by all Thorlabs devices.
/// It defines functions which are common to all Thorlabs devices,
/// including functions to simplify communication using the APT protocol.
pub trait ThorlabsDevice:
    From<UsbDevicePrimitive> + Deref<Target = UsbDevicePrimitive> + Send + Sync
{
    const SERIAL_NUMBER_PREFIX: &'static str;

    fn new(serial_number: &str) -> Result<Self, Error> {
        if !serial_number.starts_with(Self::SERIAL_NUMBER_PREFIX) {
            return Err(Error::EnumerationError(format!(
                "Serial number {} is not valid for the selected device type. Expected a serial number starting with {}",
                serial_number,
                Self::SERIAL_NUMBER_PREFIX,
            )));
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

    fn start_update_messages(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x11, 0x00];
        let data = Self::pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }

    fn stop_update_messages(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x12, 0x00];
        let data = Self::pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }

    async fn req_hw_info(&self) -> Result<HardwareInfo, Error> {
        const ID: [u8; 2] = [0x00, 0x05];
        let mut rx = match get_rx_new_or_sub(ID)? {
            Sub(rx) => rx,
            New(rx) => {
                let data = Self::pack_short_message(ID, 0, 0);
                self.port_write(data)?;
                rx
            }
        };
        let response = timeout(LONG_TIMEOUT, rx.recv()).await??;

        // Parse response todo why all of these .try_into(). Why .unwrap() instead of `?`
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

        Ok(HardwareInfo {
            hardware_serial_number,
            model_number,
            hardware_type,
            firmware_version,
            hardware_version,
            mod_state,
            number_channels,
        })
    }

    async fn req_serial_number(&self) -> Result<u32, Error> {
        let hw_info = self.req_hw_info().await?;
        Ok(hw_info.hardware_serial_number)
    }

    async fn req_model_number(&self) -> Result<String, Error> {
        let hw_info = self.req_hw_info().await?;
        Ok(hw_info.model_number)
    }

    async fn req_hardware_type(&self) -> Result<u16, Error> {
        let hw_info = self.req_hw_info().await?;
        Ok(hw_info.hardware_type)
    }

    async fn req_firmware_version(&self) -> Result<String, Error> {
        let hw_info = self.req_hw_info().await?;
        Ok(hw_info.firmware_version)
    }

    async fn req_hardware_version(&self) -> Result<u16, Error> {
        let hw_info = self.req_hw_info().await?;
        Ok(hw_info.hardware_version)
    }

    async fn req_mod_state(&self) -> Result<u16, Error> {
        let hw_info = self.req_hw_info().await?;
        Ok(hw_info.mod_state)
    }

    async fn req_num_channels(&self) -> Result<u16, Error> {
        let hw_info = self.req_hw_info().await?;
        Ok(hw_info.number_channels)
    }
}

/// # Unit Conversion
/// The UnitConversion trait provides functions for converting between real units (e.g. mm) and
/// device units (encoder counts). This trait is required by all Thorlabs devices that move.

pub trait UnitConversion {
    const DISTANCE_ANGLE_SCALING_FACTOR: f64;
    const VELOCITY_SCALING_FACTOR: f64;
    const ACCELERATION_SCALING_FACTOR: f64;

    fn position_to_bytes(position: f64) -> [u8; 4] {
        let counts = (position * Self::DISTANCE_ANGLE_SCALING_FACTOR) as i32;
        counts.to_le_bytes()
    }

    fn position_from_bytes(bytes: [u8; 4]) -> f64 {
        let counts = i32::from_le_bytes(bytes);
        (counts as f64) / Self::DISTANCE_ANGLE_SCALING_FACTOR
    }

    fn velocity_to_bytes(velocity: f64) -> [u8; 4] {
        let counts = (velocity * Self::VELOCITY_SCALING_FACTOR) as i32;
        counts.to_le_bytes()
    }

    fn velocity_from_bytes(bytes: [u8; 4]) -> f64 {
        let counts = i32::from_le_bytes(bytes);
        (counts as f64) / Self::VELOCITY_SCALING_FACTOR
    }

    fn acceleration_to_bytes(acceleration: f64) -> [u8; 4] {
        let counts = (acceleration * Self::ACCELERATION_SCALING_FACTOR) as i32;
        counts.to_le_bytes()
    }

    fn acceleration_from_bytes(bytes: [u8; 4]) -> f64 {
        let counts = i32::from_le_bytes(bytes);
        (counts as f64) / Self::ACCELERATION_SCALING_FACTOR
    }
}

/// # Motor
/// The Motor trait provides functions that are specific to Thorlabs motor controllers.

pub trait Motor: ThorlabsDevice + UnitConversion + ChanEnableState {
    async fn home(&self, channel: u8) -> Result<(), Error> {
        const ID: [u8; 2] = [0x43, 0x04];
        let mut rx = match get_rx_new_or_sub(ID)? {
            Sub(rx) => rx,
            New(rx) => {
                let data = Self::pack_short_message(ID, channel, 0);
                self.port_write(data)?;
                rx
            }
        };
        timeout(LONG_TIMEOUT, rx.recv()).await??;
        Ok(())
    }

    async fn move_absolute(&self, channel: u16, absolute_distance: f64) -> Result<(), Error> {
        const ID: [u8; 2] = [0x53, 0x04];
        const LENGTH: usize = 12;
        let mut rx = get_rx_new_or_err(ID)?;
        let mut data = Self::pack_long_message(ID, LENGTH);
        data.extend(channel.to_le_bytes());
        data.extend(Self::position_to_bytes(absolute_distance));
        self.port_write(data)?;
        let response = timeout(LONG_TIMEOUT, rx.recv()).await??;

        Ok(())
    }

    async fn move_absolute_from_params(&self, channel: u8) -> Result<(), Error> {
        const ID: [u8; 2] = [0x53, 0x04];
        let mut rx = get_rx_new_or_err(ID)?;
        let data = Self::pack_short_message(ID, channel, 0);
        self.port_write(data)?;
        timeout(LONG_TIMEOUT, rx.recv()).await??;
        Ok(())
    }
}

///

enum BayIdent {
    Standalone,
    BayUnknown,
    Bay(i8),
}

pub trait Hub: ThorlabsDevice {
    async fn req_bay_used(&self) -> Result<BayIdent, Error> {
        const ID: [u8; 2] = [0x65, 0x00];
        let mut rx = match get_rx_new_or_sub(ID)? {
            New(rx) => {
                let data = Self::pack_short_message(ID, 0, 0);
                self.port_write(data)?;
                rx
            }
            Sub(rx) => rx,
        };
        let response = rx.recv().await?;
        Ok(match response[2] as i8 {
            -0x01 => BayIdent::Standalone,
            0x00 => BayIdent::BayUnknown,
            n if n > 0 && n <= 6 => BayIdent::Bay(n),
            _ => {
                return Err(Error::DeviceError(format!(
                    "Device (serial number {}) returned an invalid bay number {}",
                    self.serial_number, response[2]
                )))
            }
        })
    }
}

///

pub trait ChanEnableState: ThorlabsDevice {
    async fn set_channel_enable_state(&self, channel: u8, enable: bool) -> Result<(), Error> {
        const SET_ID: [u8; 2] = [0x10, 0x02];
        const REQ_ID: [u8; 2] = [0x11, 0x02];

        let enable_byte: u8 = if enable { 0x01 } else { 0x02 };

        let mut rx = get_rx_new_or_err(SET_ID)?;

        let set_data = Self::pack_short_message(SET_ID, channel, enable_byte);
        self.port_write(set_data)?;

        tokio::time::sleep(SHORT_TIMEOUT).await;

        let req_data = Self::pack_short_message(REQ_ID, channel, 0);
        self.port_write(req_data)?;

        let response = timeout(LONG_TIMEOUT, rx.recv()).await??;
        if response[3] == enable_byte {
            return Ok(());
        }
        Err(Error::DeviceError(format!(
            "Failed to set channel {} enable state to {} for device with serial number {}",
            channel, enable, self.serial_number,
        )))
    }
}
