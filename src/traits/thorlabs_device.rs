/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: test.rs
*/

use crate::devices::{HardwareInfo, UsbDevicePrimitive};
use crate::enumerate::get_device;
use crate::env::{DEST, LONG_TIMEOUT, SOURCE};
use crate::error::Error;
use crate::messages::ChannelStatus::{New, Sub};
use crate::messages::{get_rx_new_or_sub, MsgFormat};
use std::ops::Deref;
use tokio::time::timeout;

/// # Thorlabs Device
/// The `ThorlabsDevice` trait is a base trait implemented by all Thorlabs devices.
/// It defines functions which are common to all Thorlabs devices,
/// including functions to simplify communication using the APT protocol.
pub trait ThorlabsDevice:
    From<UsbDevicePrimitive> + Deref<Target = UsbDevicePrimitive> + Send + Sync
{
    /// # Serial Number Prefix
    /// Each Thorlabs device type has a unique serial number prefix. For example, KDC101
    /// "K-cubes" always have serial numbers which begin with "27". The `new()` function
    /// checks that the target serial number begins with the correct prefix for the
    /// calling struct. This prevents users from accidentally connecting to devices
    /// from the incorrect struct.
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

    /// # Pack Functions
    ///
    /// The Thorlabs APT communication protocol uses a fixed length 6-byte message header, which
    /// may be followed by a variable-length data packet. For simple commands, the 6-byte message
    /// header is sufficient to convey the entire command. For more complex commands (e.g.
    /// commands where a set of parameters needs to be passed to the device) the 6-byte header
    /// is insufficient and must be followed by a data packet.
    ///
    /// The `MsgFormat` enum is used to wrap the bytes of a message and indicate whether the
    /// message is `Short` (six byte header only) or `Long` (six byte header plus variable length
    /// data package).
    ///
    /// The `pack_short_message()` and `pack_long_message()` helper functions are implemented to
    /// simplify message formatting and enforce consistency with the APT protocol.  
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

    /// # MOD_IDENTIFY (0x0223)
    ///
    /// **Function implemented from Thorlabs ATP protocol**
    ///
    /// This function instructs the hardware unit to identify itself (by flashing its front
    /// panel LEDs). In card-slot (bay) type of systems (which are usually multichannel
    /// controllers such as BSC102, BSC103, BPC302, BPC303, PPC102) the front panel LED that
    /// flashes in response to this command is controlled by the motherboard, not the individual
    /// channel cards. For these controllers, the destination byte of the `MOD_IDENTIFY` message
    /// must be the motherboard `(0x11)` and the `Channel Ident` byte is used to select the
    /// channel to be identified. In single-channel controllers, the Channel Ident byte is
    /// ignored as the destination of the command is uniquely identified by the USB serial
    /// number of the controller.
    fn identify(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x23, 0x02];
        let data = Self::pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }

    /// # HW_START_UPDATEMSGS (0x0011)
    ///
    /// **Function implemented from Thorlabs ATP protocol**
    ///
    /// This function starts automatic status updates from the embedded controller. Status
    /// update messages contain information about the position and status of the controller
    /// (such as limit switch status or current position). The messages will be sent by the
    /// controller every 100 milliseconds (10 Hz) until the controller receives a
    /// `HW_STOP_UPDATEMSGS` command. The same status information can also be requested at
    /// a single time point (as a one-off rather than every 100 milliseconds) using the
    /// controller's relevant `GET_STATUTSUPDATES` function.
    fn start_update_messages(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x11, 0x00];
        let data = Self::pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }

    /// # HW_STOP_UPDATEMSGS (0x0012)
    ///
    /// **Function implemented from Thorlabs ATP protocol**
    ///
    /// This function stops automatic status updates from the embedded controller. Status
    /// update messages contain information about the position and status of the controller
    /// (such as limit switch status or current position). After receiving a
    /// `HW_START_UPDATEMSGS` command, the controller will send a status message every
    /// 100 milliseconds (10 Hz) until the controller receives a `HW_STOP_UPDATEMSGS`
    /// command.
    ///
    /// This function is normally called by client applications when shutting down, to
    /// instruct the controller to turn off status updates to prevent USB buffer overflows
    /// on the PC.
    fn stop_update_messages(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x12, 0x00];
        let data = Self::pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }

    /// # HW_REQ_INFO (0x0005)
    ///
    /// **Function implemented from Thorlabs ATP protocol**
    ///
    /// This function is used to request hardware information from the controller.
    /// The controller will send a `HW_GET_INFO (0x0006)` message in response, which
    /// is then parsed into a new instance of the `HardwareInfo` struct.

    // todo This internal function is not intended to be used directly.
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
