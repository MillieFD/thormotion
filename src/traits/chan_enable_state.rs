use crate::messages::get_rx_new_or_err;
use crate::traits::ThorlabsDevice;
use crate::Error;

pub trait ChanEnableState: ThorlabsDevice {
    async fn set_channel_enable_state(&self, channel: u8, enable: bool) -> Result<(), Error> {
        const SET_ID: [u8; 2] = [0x10, 0x02];
        const REQ_ID: [u8; 2] = [0x11, 0x01];

        let enable_byte: u8 = if enable { 0x01 } else { 0x02 };

        let mut rx = get_rx_new_or_err(SET_ID)?;
        let set_data = Self::pack_short_message(SET_ID, channel, enable_byte);
        self.port_write(set_data)?;
        let req_data = Self::pack_short_message(REQ_ID, channel, 0);
        self.port_write(req_data)?;
        let response = rx.recv().await?;
        if response[3] == enable_byte {
            return Ok(());
        }
        Err(Error::DeviceError(
            self.serial_number.clone(),
            "Failed to set channel enable state".to_string(),
        ))
    }
}
