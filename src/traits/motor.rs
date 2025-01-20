/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: motor.rs
*/

use crate::devices::{pack_long_message, pack_short_message};
use crate::env::LONG_TIMEOUT;
use crate::error::Error;
use crate::messages::ChannelStatus::{New, Sub};
use crate::messages::{get_rx_new_or_err, get_rx_new_or_sub};
use crate::traits::{ChannelEnableState, ThorlabsDevice};
use tokio::time::timeout;

/// # Motor
/// The `Motor` trait is implemented by all Thorlabs brushed and brushless DC servo motors,
/// as well as all stepper motor devices. It provides functions for motion control, which
/// correspond to the Thorlabs APT protocol **Motor Control Messages** section. All included
/// functions are named with the `MOT` prefix.
pub trait Motor: ThorlabsDevice + ChannelEnableState {
    /// # MOT_MOVE_HOME (0x0443)
    ///
    /// **Function implemented from Thorlabs APT protocol**
    ///
    /// This function initiates the homing move sequence on the specified motor channel.
    /// The homing parameters can be set using `MOT_SET_HOMEPARAMS (0x0440)`
    /// The controller will respond with a `MOT_MOVE_HOMED (0x0444)` once the homing sequence
    /// has successfully completed.
    async fn home(&self, channel: u8) -> Result<(), Error> {
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
    async fn move_absolute(&self, channel: u16, absolute_distance: f64) -> Result<(), Error> {
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

    async fn move_absolute_from_params(&self, channel: u8) -> Result<(), Error> {
        const ID: [u8; 2] = [0x53, 0x04];
        let mut rx = get_rx_new_or_err(ID)?;
        let data = pack_short_message(ID, channel, 0);
        self.port_write(data)?;
        timeout(LONG_TIMEOUT, rx.recv()).await??;
        Ok(())
    }

    /// # Unit Conversion
    /// Internally, all thorlabs motor devices use an encoder to keep track of their current
    /// position. All distances must therefore be converted from real-word units (mm) into
    /// encoder-counts using the correct scaling factor for the device. This scaling factor
    /// may differ between device types due to different encoder resolutions and gearing
    /// ratios.
    ///
    /// The device's unit of time is determined by the encoder polling frequency. All
    /// time-based units (such as velocity and acceleration) must therefore be converted
    /// from real-word units (seconds) into device units using the correct scaling factor
    /// for the device. This scaling factor may differ between device types due to
    /// different encoder polling frequencies.
    const DISTANCE_ANGLE_SCALING_FACTOR: f64;
    const VELOCITY_SCALING_FACTOR: f64;
    const ACCELERATION_SCALING_FACTOR: f64;

    fn position_to_bytes(position: f64) -> [u8; 4] {
        i32::to_le_bytes(
            (position * Self::DISTANCE_ANGLE_SCALING_FACTOR)
                .round()
                .into(),
        )
    }

    fn position_from_bytes(bytes: [u8; 4]) -> f64 {
        i32::from_le_bytes(bytes).into() / Self::DISTANCE_ANGLE_SCALING_FACTOR
    }

    fn velocity_to_bytes(velocity: f64) -> [u8; 4] {
        i32::to_le_bytes((velocity * Self::VELOCITY_SCALING_FACTOR).into())
    }

    fn velocity_from_bytes(bytes: [u8; 4]) -> f64 {
        i32::from_le_bytes(bytes).into() / Self::VELOCITY_SCALING_FACTOR
    }

    fn acceleration_to_bytes(acceleration: f64) -> [u8; 4] {
        i32::to_le_bytes((acceleration * Self::ACCELERATION_SCALING_FACTOR).into())
    }

    fn acceleration_from_bytes(bytes: [u8; 4]) -> f64 {
        i32::from_le_bytes(bytes).into() / Self::ACCELERATION_SCALING_FACTOR
    }
}
