/// # USTATUSUPDATE (0x0491)
///
/// **Implemented from Thorlabs APT protocol**
///
/// This function parses the status update message returned from the Thorlabs device.
///
/// Message ID: 0x0491
///
/// Message Length: 20 bytes (6-byte header followed by 14-byte status update message)
fn u_status_update(response: &[u8; 20]) {
    let channel = u16::from_le_bytes(response[6..8].try_into()?);
    let position = self::position_from_le_bytes(response[8..12].try_into()?);
    let velocity = self::position_from_le_bytes(response[12..14].try_into()?);
    let motor_current = u16::from_le_bytes(response[14..16].try_into()?);
}
