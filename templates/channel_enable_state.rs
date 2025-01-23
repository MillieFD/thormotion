/// # MOD_SET_CHANENABLESTATE (0x0210)
///
/// **Implemented from Thorlabs APT protocol**
///
/// This function enables or disables the specified drive channel.
/// The channel must be enabled before the device can move.
/// Byte 2 indicates the channel identity, and byte 3 indicates whether it is
/// enabled (0x01) or disabled (0x02).
///
/// Message ID: 0x0210
///
/// Message Length: 6 bytes (header only)
///
/// # MOD_REQ_CHANENABLESTATE (0x0211)
///
/// This function is sent to request the current state (enabled or disabled) for
/// the specified channel.///
/// Message ID: 0x0211
///
/// Message Length: 6 bytes (header only)
///
/// # MOD_GET_CHANENABLESTATE (0x0212)
///
/// The controller will respond by sending a `MOD_GET_CHANENABLESTATE` message.
/// Byte 2 indicates the channel identity, and byte 3 indicates whether it is
/// enabled (0x01) or disabled (0x02).
///
/// Message ID: 0x0212
///
/// Message Length: 6 bytes (header only)
pub fn set_channel_enable_state(&self, channel: u8, enable: bool) -> Result<(), Error> {
    const SET_ID: [u8; 2] = [0x10, 0x02];
    const REQ_ID: [u8; 2] = [0x11, 0x02];
    let enable_byte: u8 = if enable { 0x01 } else { 0x02 };
    let mut rx = get_rx_new_or_err(SET_ID)?;
    let set_data = pack_short_message(SET_ID, channel, enable_byte);
    self.device.port_write(set_data)?;
    self.runtime
        .block_on(async { tokio::time::sleep(SHORT_TIMEOUT).await });
    let req_data = pack_short_message(REQ_ID, channel, 0);
    self.device.port_write(req_data)?;
    let response = self
        .runtime
        .block_on(async { tokio::time::timeout(LONG_TIMEOUT, rx.recv()).await })??;
    if response[3] == enable_byte {
        Ok(())
    } else {
        Err(DeviceError(format!(
            "Failed to set channel number {} enable state to {} for {}",
            channel, enable, self,
        )))
    }
}
