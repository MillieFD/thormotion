/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: motor.rs
Description: This file defines the Motor trait, which provides functions that are specific to
Thorlabs motor controllers.
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::messages::{get_rx_new_or_err, get_rx_new_or_sub, ChannelStatus};
use crate::traits::{ThorlabsDevice, UnitConversion};
use crate::Error;

pub trait Motor: ThorlabsDevice + UnitConversion {
    async fn home(&self, channel: u8) -> Result<(), Error> {
        const ID: [u8; 2] = [0x43, 0x04];
        let mut rx = match get_rx_new_or_sub(ID)? {
            ChannelStatus::Sub(rx) => rx,
            ChannelStatus::New(rx) => {
                let data = Self::pack_short_message(ID, channel, 0);
                self.port_write(data)?;
                rx
            }
        };
        rx.recv().await?;
        Ok(())
    }

    async fn move_absolute(&self, channel: u16, absolute_distance: f32) -> Result<(), Error> {
        const ID: [u8; 2] = [0x53, 0x04];
        const LENGTH: usize = 12;
        let mut rx = get_rx_new_or_err(ID)?;
        let mut data = Self::pack_long_message(ID, LENGTH);
        data.extend(channel.to_le_bytes());
        data.extend(self.position_real_to_dev(absolute_distance).to_le_bytes());
        self.port_write(data)?;
        rx.recv().await?;
        Ok(())
    }

    async fn move_absolute_preset(&self, channel: u8) -> Result<(), Error> {
        const ID: [u8; 2] = [0x53, 0x04];
        let mut rx = get_rx_new_or_err(ID)?;
        let data = Self::pack_short_message(ID, channel, 0);
        self.port_write(data)?;
        rx.recv().await?;
        Ok(())
    }
}
