/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: thorlabs_device.rs
*/

use crate::devices::UsbDevicePrimitive;
use crate::error::{EnumerationError, Error};
use std::fmt::Display;
use std::ops::Deref;

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
            Err(EnumerationError(format!(
                "Serial number {} is not valid for the selected device type. \
                Expected a serial number starting with {}",
                serial_number,
                Self::SERIAL_NUMBER_PREFIX,
            )))
        }
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
        let data = UsbDevicePrimitive::pack_short_message(ID, 0, 0);
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
        let data = UsbDevicePrimitive::pack_short_message(ID, 0, 0);
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
        let data = UsbDevicePrimitive::pack_short_message(ID, 0, 0);
        self.port_write(data)?;
        Ok(())
    }
}
