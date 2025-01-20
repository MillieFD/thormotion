/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: channel_enable_state.rs
*/

use crate::devices::pack_short_message;
use crate::env::{LONG_TIMEOUT, SHORT_TIMEOUT};
use crate::error::{DeviceError, Error};
use crate::messages::get_rx_new_or_err;
use crate::traits::thorlabs_device::ThorlabsDevice;
use tokio::time::timeout;

/// # MOD_SET_CHANENABLESTATE (0x0210)
///
/// **Function implemented from Thorlabs APT protocol**
///
/// This function enables or disables the specified drive channel.
/// The channel must be enabled before the device can move.
///
/// Message ID: 0x0210
///
/// Message Length: 6 bytes (header only)
///
/// # MOD_REQ_CHANENABLESTATE (0x0211)
///
/// This function is sent to request the current state (enabled or disabled) for the specified
/// channel.
///
/// Message ID: 0x0211
///
/// Message Length: 6 bytes (header only)
///
/// # MOD_GET_CHANENABLESTATE (0x0212)
///
/// The controller will respond by sending a `MOD_GET_CHANENABLESTATE` message.
/// Byte 2 indicates the channel identity, and byte 3 indicates whether it is enabled
/// (0x01) or disabled (0x02).
///
/// Message ID: 0x0212
///
/// Message Length: 6 bytes (header only)
pub trait ChannelEnableState: ThorlabsDevice {
    async fn set_channel_enable_state(&self, channel: u8, enable: bool) -> Result<(), Error> {
        const SET_ID: [u8; 2] = [0x10, 0x02];
        const REQ_ID: [u8; 2] = [0x11, 0x02];
        let enable_byte: u8 = if enable { 0x01 } else { 0x02 };
        let mut rx = get_rx_new_or_err(SET_ID)?;
        let set_data = pack_short_message(SET_ID, channel, enable_byte);
        self.port_write(set_data)?;
        tokio::time::sleep(SHORT_TIMEOUT).await;
        let req_data = pack_short_message(REQ_ID, channel, 0);
        self.port_write(req_data)?;
        let response = timeout(LONG_TIMEOUT, rx.recv()).await??;
        if response[3] == enable_byte {
            return Ok(());
        }
        Err(DeviceError(format!(
            "Failed to set channel {} enable state to {} for device with serial number {}",
            channel, enable, self.serial_number,
        )))
    }
}
