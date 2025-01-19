/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: thorlabs_device.rs
*/

use crate::devices::{HardwareInfo, UsbDevicePrimitive};
use crate::env::{DEST, LONG_TIMEOUT, SOURCE};
use crate::error::AptError::EnumerationError;
use crate::error::Error;
use crate::error::Error::AptError;
use crate::messages::ChannelStatus::{New, Sub};
use crate::messages::{get_rx_new_or_sub, MsgFormat};
use std::fmt::Display;
use std::ops::Deref;
use tokio::time::timeout;

/// # Thorlabs Device
/// The `ThorlabsDevice` trait is a base trait implemented by all Thorlabs devices.
/// It defines functions which are common to all Thorlabs devices,
/// including functions to simplify communication using the APT protocol.
pub trait ThorlabsDevice:
    From<UsbDevicePrimitive>
    + From<String>
    + From<&'static str>
    + Deref<Target = UsbDevicePrimitive>
    + Display
    + Send
    + Sync
{
    fn new(serial_number: &str) -> Result<Self, Error>;

    /// # Serial Number Prefix
    /// Each Thorlabs device type has a unique serial number prefix. For example, KDC101
    /// "K-cubes" always have serial numbers which begin with "27". The `new()` function
    /// checks that the target serial number begins with the correct prefix for the
    /// calling struct. This prevents users from accidentally connecting to devices
    /// from the incorrect struct.
    const SERIAL_NUMBER_PREFIX: &'static str;
    fn check_serial_number(serial_number: &str) -> Result<(), Error> {
        if serial_number.starts_with(Self::SERIAL_NUMBER_PREFIX) {
            Ok(())
        } else {
            Err(AptError(EnumerationError(format!(
                "Serial number {} is not valid for the selected device type. \
                Expected a serial number starting with {}",
                serial_number,
                Self::SERIAL_NUMBER_PREFIX,
            ))))
        }
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
    /// **Function implemented from Thorlabs APT protocol**
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
    ///
    /// Message ID: 0x0223
    ///
    /// Message Length: 6 bytes (header only)
    fn identify(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x23, 0x02];
        let data = Self::pack_short_message(ID, 0, 0);
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
    fn start_update_messages(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x11, 0x00];
        let data = Self::pack_short_message(ID, 0, 0);
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
    fn stop_update_messages(&self) -> Result<(), Error> {
        const ID: [u8; 2] = [0x12, 0x00];
        let data = Self::pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }

    /// # HW_REQ_INFO (0x0005)
    ///
    /// **Function implemented from Thorlabs APT protocol**
    ///
    /// This function is used to request hardware information from the controller.
    ///
    /// Message ID: 0x0005
    ///
    /// Message length: 6 bytes (header only)
    ///
    /// # Response
    ///
    /// The controller will send a `HW_GET_INFO (0x0006)` message in response, which
    /// is then parsed into a new instance of the `HardwareInfo` struct.
    ///
    /// Response ID: 0x0006
    ///
    /// Response length: 90 bytes (6-byte header followed by 84-byte data packet)
    async fn hw_req_info(&self) -> Result<HardwareInfo, Error> {
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
        let serial_number = u32::from_le_bytes(response[6..10].try_into()?);
        let model_number = String::from_utf8_lossy(&response[10..18]).to_string();
        let hardware_type = u16::from_le_bytes(response[18..20].try_into()?);
        let firmware_minor = u8::from_le_bytes(response[20..21].try_into()?);
        let firmware_interim = u8::from_le_bytes(response[21..22].try_into()?);
        let firmware_major = u8::from_le_bytes(response[22..23].try_into()?);
        let hardware_version = u16::from_le_bytes(response[84..86].try_into()?);
        let mod_state = u16::from_le_bytes(response[86..88].try_into()?);
        let number_channels = u16::from_le_bytes(response[88..90].try_into()?);
        Ok(HardwareInfo {
            serial_number,
            model_number,
            hardware_type,
            firmware_version: format!("{}.{}.{}", firmware_major, firmware_interim, firmware_minor),
            hardware_version,
            mod_state,
            number_channels,
        })
    }
}
