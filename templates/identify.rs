/// # MOD_IDENTIFY (0x0223)
///
/// **Implemented from Thorlabs APT protocol**
///
/// This function instructs the hardware unit to identify itself by flashing its
/// front panel LED.
///
/// In card-slot (bay) type of systems (which are usually multichannel
/// controllers such as BSC102, BSC103, BPC302, BPC303, PPC102) the front panel
/// LED that flashes in response to this command is controlled by the
/// motherboard, not the individual channel cards.
/// For these controllers, the destination byte of the `MOD_IDENTIFY` message
/// must be the motherboard `(0x11)` and the `Channel Ident` byte is used to
/// select the channel to be identified.
///
/// In single-channel controllers, the `Channel Ident` byte is ignored as the
/// destination of the command is uniquely identified by the USB serial number
/// of the controller.
///
/// Message ID: 0x0223
///
/// Message Length: 6 bytes (header only)
///
/// # Response
///
/// The hardware unit will respond by flashing its front panel LED to identify
/// itself.
pub fn identify(&self) -> Result<(), Error> {
    const ID: [u8; 2] = [0x23, 0x02];
    let data = pack_short_message(ID, 0, 0);
    self.device.port_write(data)?;
    Ok(())
}
