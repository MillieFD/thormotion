/// # MOT_MOVE_HOME (0x0443)
///
/// **Implemented from Thorlabs APT protocol**
///
/// This function starts the homing move sequence on the specified motor channel.
/// The homing parameters can be set using `MOT_SET_HOMEPARAMS (0x0440)`.
/// This is the asynchronous (non-blocking) version of `MOT_MOVE_HOME`.
/// A synchronous (blocking) version of this command is also available.
///
/// Message ID: 0x0443
///
/// Message Length: 6 bytes (header only)
///
/// # MOT_MOVE_HOMED (0x0444)
///
/// The controller will respond with a `MOT_MOVE_HOMED (0x0444)` once the homing
/// sequence has successfully completed.
pub async fn home_async(&self, channel: u8) -> Result<(), Error> {
    const ID: [u8; 2] = [0x43, 0x04];
    let mut rx = match get_rx_new_or_sub(ID)? {
        Sub(rx) => rx,
        New(rx) => {
            let data = pack_short_message(ID, channel, 0);
            self.device.port_write(data)?;
            rx
        }
    };
    let response = rx.recv().await;
    Ok(())
}

/// # MOT_MOVE_HOME (0x0443)
///
/// **Implemented from Thorlabs APT protocol**
///
/// This function starts the homing move sequence on the specified motor channel.
/// The homing parameters can be set using `MOT_SET_HOMEPARAMS (0x0440)`.
/// This is the synchronous (blocking) version of `MOT_MOVE_HOME`.
/// An asynchronous (non-blocking) version of this command is also available.
///
/// Message ID: 0x0443
///
/// Message Length: 6 bytes (header only)
///
/// # MOT_MOVE_HOMED (0x0444)
///
/// The controller will respond with a `MOT_MOVE_HOMED (0x0444)` once the homing
/// sequence has successfully completed.
pub fn home(&self, channel: u8) -> Result<(), Error> {
    self.runtime
        .block_on(async { self.home_async(channel).await })
}
