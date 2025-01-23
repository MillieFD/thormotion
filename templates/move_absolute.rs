/// # MOT_MOVE_ABSOLUTE (0x0453)
///
/// **Implemented from Thorlabs APT protocol**
///
/// This function causes the specified motor channel to move to an absolute position.
/// Internally, the motor uses an encoder to keep track of its current position.
/// The absolute distance must therefore be converted from real-word units (mm)
/// into encoder-counts using the correct scale factor for the device.
/// The KDC101 struct implements templates to simplify these conversions.
///
/// The Thorlabs APT protocol describes two versions of this command:
///
/// * **Short 6-byte version** (header only) uses the pre-existing absolute move
/// parameters for the specified motor channel,
/// which can be set using the `MOT_SET_MOVEABSPARAMS (0x0450)` command.
///
/// * **Long 12-byte version** (6-byte header followed by 6-byte data packet)
/// which transmits the target position within the message's data packet.
///
/// # MOT_MOVE_COMPLETED (0x0464)
///
/// The controller responds by sending a `MOT_MOVE_COMPLETED (0x0464)` message
/// once the specified channel has come to rest at the target position.
///
/// Message ID: 0x0453
///
/// Message Length: 20 bytes (header followed by 14-byte status update message)
pub fn move_absolute(&self, channel: u16, absolute_distance: f64) -> Result<(), Error> {
    const ID: [u8; 2] = [0x53, 0x04];
    const LENGTH: usize = 12;
    let mut rx = get_rx_new_or_err(ID)?;
    let mut data = pack_long_message(ID, LENGTH);
    data.extend(channel.to_le_bytes());
    data.extend(Self::position_to_bytes(absolute_distance));
    self.device.port_write(data)?;
    let response = self
        .runtime
        .block_on(async { tokio::time::timeout(LONG_TIMEOUT, rx.recv()).await })??;
    Ok(())
}

pub fn move_absolute_from_params(&self, channel: u8) -> Result<(), Error> {
    const ID: [u8; 2] = [0x53, 0x04];
    let mut rx = get_rx_new_or_err(ID)?;
    let data = pack_short_message(ID, channel, 0);
    self.device.port_write(data)?;
    let response = self
        .runtime
        .block_on(async { tokio::time::timeout(LONG_TIMEOUT, rx.recv()).await })??;
    Ok(())
}
