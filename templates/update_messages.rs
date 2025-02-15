/// # HW_START_UPDATEMSGS (0x0011)
///
/// **Implemented from Thorlabs APT protocol**
///
/// This function starts automatic status updates from the embedded controller.
/// Status update messages contain information about the current position and status
/// of the controller.
///
/// Message ID: 0x0011
///
/// Message Length: 6 bytes (header only)
///
/// # Response
///
/// The controller will send a status update message every 100 milliseconds (10 Hz)
/// until receiving a `HW_STOP_UPDATEMSGS` command.
/// The same status information can also be requested at a single time point
/// (as a one-off rather than every 100 milliseconds) using the controller's
/// relevant `GET_STATUTSUPDATE` function.
pub fn start_update_messages(&self) -> Result<(), Error> {
    const ID: [u8; 2] = [0x11, 0x00];
    let data = pack_short_message(ID, 0, 0);
    self.device.port_write(data)?;
    Ok(())
}

/// # HW_STOP_UPDATEMSGS (0x0012)
///
/// **Implemented from Thorlabs APT protocol**
///
/// This function stops automatic status updates from the embedded controller.
/// The `HW_STOP_UPDATEMSGS` message is normally sent by client applications when
/// shutting down, to instruct the controller to turn off status updates to prevent
/// USB buffer overflows on the PC.
///
/// Message ID: 0x0012
///
/// Message Length: 6 bytes (header only)
///
/// # Response
///
/// The controller will stop sending automatic status messages every 100 milliseconds
pub fn stop_update_messages(&self) -> Result<(), Error> {
    const ID: [u8; 2] = [0x12, 0x00];
    let data = pack_short_message(ID, 0, 0);
    self.device.port_write(data)?;
    Ok(())
}
