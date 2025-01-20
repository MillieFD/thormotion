/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: kdc101.rs
*/

use crate::devices::{pack_long_message, pack_short_message, UsbDevicePrimitive};
use crate::enumerate::get_device_primitive;
use crate::env::{LONG_TIMEOUT, SHORT_TIMEOUT};
use crate::error::{DeviceError, Error};
use crate::impl_thorlabs_device;
use crate::messages::ChannelStatus::{New, Sub};
use crate::messages::{get_rx_new_or_err, get_rx_new_or_sub};
use crate::traits::{Motor, ThorlabsDevice};
use pyo3::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Deref;
use tokio::time::timeout;

#[pyclass]
#[derive(Debug)]
pub struct KDC101 {
    device: UsbDevicePrimitive,
    #[pyo3(get)]
    serial_number: u32,
    #[pyo3(get)]
    model_number: String,
    #[pyo3(get)]
    hardware_type: u16,
    #[pyo3(get)]
    firmware_version: String,
    #[pyo3(get)]
    hardware_version: u16,
    #[pyo3(get)]
    module_state: u16,
    #[pyo3(get)]
    number_of_channels: u16,
}

#[pymethods]
impl KDC101 {
    #[new]
    pub fn new(serial_number: &str) -> Result<Self, Error> {
        Self::check_serial_number(serial_number)?;
        let device = get_device_primitive(serial_number)?;
        Ok(Self::from(device))
    }

    /// # MOD_IDENTIFY (0x0223)
    ///
    /// **Function implemented from Thorlabs APT protocol**
    ///
    /// This function instructs the hardware unit to identify itself (by flashing its front
    /// panel LEDs).
    ///
    /// In card-slot (bay) type of systems (which are usually multichannel controllers such as
    /// BSC102, BSC103, BPC302, BPC303, PPC102) the front panel LED that flashes in response to
    /// this command is controlled by the motherboard, not the individual channel cards.
    /// For these controllers, the destination byte of the `MOD_IDENTIFY` message must be the
    /// motherboard `(0x11)` and the `Channel Ident` byte is used to select the channel to be
    /// identified.
    ///
    /// In single-channel controllers, the `Channel Ident` byte is ignored as the destination of
    /// the command is uniquely identified by the USB serial number of the controller.
    ///
    /// Message ID: 0x0223
    ///
    /// Message Length: 6 bytes (header only)
    pub fn identify(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x23, 0x02];
        let data = pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }

    /// # HW_START_UPDATEMSGS (0x0011)
    ///
    /// **Function implemented from Thorlabs APT protocol**
    ///
    /// This function starts automatic status updates from the embedded controller. Status
    /// update messages contain information about the position and status of the controller
    /// (such as limit switch status or current position).
    ///
    /// Message ID: 0x0011
    ///
    /// Message Length: 6 bytes (header only)
    ///
    /// # Response
    ///
    /// The controller will send a status update message every 100 milliseconds (10 Hz) until
    /// the receiving a `HW_STOP_UPDATEMSGS` command. The same status information can also be
    /// requested at a single time point (as a one-off rather than every 100 milliseconds)
    /// using the controller's relevant `GET_STATUTSUPDATE` function.
    pub fn start_update_messages(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x11, 0x00];
        let data = pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }

    /// # HW_STOP_UPDATEMSGS (0x0012)
    ///
    /// **Function implemented from Thorlabs APT protocol**
    ///
    /// This function stops automatic status updates from the embedded controller.
    /// This function is normally called by client applications when shutting down, to
    /// instruct the controller to turn off status updates to prevent USB buffer overflows
    /// on the PC.
    ///
    /// Message ID: 0x0012
    ///
    /// Message Length: 6 bytes (header only)
    ///
    /// # Response
    ///
    /// The controller will stop sending automatic status messages every 100 milliseconds (10 Hz).
    pub fn stop_update_messages(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x12, 0x00];
        let data = pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }

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

    pub async fn set_channel_enable_state(&self, enable: bool) -> Result<(), Error> {
        const CHANNEL: u8 = 0x01;
        const SET_ID: [u8; 2] = [0x10, 0x02];
        const REQ_ID: [u8; 2] = [0x11, 0x02];
        let enable_byte: u8 = if enable { 0x01 } else { 0x02 };
        let mut rx = get_rx_new_or_err(SET_ID)?;
        let set_data = pack_short_message(SET_ID, CHANNEL, enable_byte);
        self.port_write(set_data)?;
        tokio::time::sleep(SHORT_TIMEOUT).await;
        let req_data = pack_short_message(REQ_ID, CHANNEL, 0);
        self.port_write(req_data)?;
        let response = timeout(LONG_TIMEOUT, rx.recv()).await??;
        if response[3] == enable_byte {
            return Ok(());
        }
        Err(DeviceError(format!(
            "Failed to set channel {} enable state to {} for device with serial number {}",
            CHANNEL, enable, self.serial_number,
        )))
    }

    /// # MOT_MOVE_HOME (0x0443)
    ///
    /// **Function implemented from Thorlabs APT protocol**
    ///
    /// This function initiates the homing move sequence on the specified motor channel.
    /// The homing parameters can be set using `MOT_SET_HOMEPARAMS (0x0440)`
    /// The controller will respond with a `MOT_MOVE_HOMED (0x0444)` once the homing sequence
    /// has successfully completed.
    pub async fn home(&self, channel: u8) -> Result<(), Error> {
        const ID: [u8; 2] = [0x43, 0x04];
        let mut rx = match get_rx_new_or_sub(ID)? {
            Sub(rx) => rx,
            New(rx) => {
                let data = pack_short_message(ID, channel, 0);
                self.port_write(data)?;
                rx
            }
        };
        timeout(LONG_TIMEOUT, rx.recv()).await??;
        Ok(())
    }

    /// # MOT_MOVE_ABSOLUTE (0x0453)
    ///
    /// **Function implemented from Thorlabs APT protocol**
    ///
    /// This function causes the specified motor channel to move to an absolute position.
    /// Internally, the motor uses an encoder to keep track of its current position. The
    /// absolute distance must therefore be converted from real-word units (mm) into
    /// encoder-counts using the correct scaling factor for the device. The `Motor`
    /// trait implements functions to simplify these conversions.
    ///
    /// The Thorlabs APT protocol describes two versions of this command:
    /// * **Short 6-byte version** (header only) uses the absolute move parameters for
    /// the specified motor channel, which can be set using the `MOT_SET_MOVEABSPARAMS (0x0450)`
    /// command.
    /// * **Long 12-byte version** (6-byte header followed by 6-byte data packet) which
    /// transmits the target position within the message's data packet.
    pub async fn move_absolute(&self, channel: u16, absolute_distance: f64) -> Result<(), Error> {
        const ID: [u8; 2] = [0x53, 0x04];
        const LENGTH: usize = 12;
        let mut rx = get_rx_new_or_err(ID)?;
        let mut data = pack_long_message(ID, LENGTH);
        data.extend(channel.to_le_bytes());
        data.extend(Self::position_to_bytes(absolute_distance));
        self.port_write(data)?;
        let response = timeout(LONG_TIMEOUT, rx.recv()).await??;

        Ok(())
    }

    pub async fn move_absolute_from_params(&self, channel: u8) -> Result<(), Error> {
        const ID: [u8; 2] = [0x53, 0x04];
        let mut rx = get_rx_new_or_err(ID)?;
        let data = pack_short_message(ID, channel, 0);
        self.port_write(data)?;
        timeout(LONG_TIMEOUT, rx.recv()).await??;
        Ok(())
    }
}

impl_thorlabs_device!(KDC101, "27");

impl Motor for KDC101 {
    const DISTANCE_ANGLE_SCALING_FACTOR: f64 = 34554.96;
    const VELOCITY_SCALING_FACTOR: f64 = 772981.3692;
    const ACCELERATION_SCALING_FACTOR: f64 = 263.8443072;
}
