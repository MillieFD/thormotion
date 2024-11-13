use crate::messages::{get_rx_new_or_sub, ChannelStatus};
use crate::traits::ThorlabsDevice;
use crate::Error;

enum BayIdent {
    Standalone,
    BayUnknown,
    Bay(i8),
}

pub trait Hub: ThorlabsDevice {
    async fn req_bay_used(&self) -> Result<BayIdent, Error> {
        const ID: [u8; 2] = [0x65, 0x00];
        let mut rx = match get_rx_new_or_sub(ID)? {
            ChannelStatus::New(rx) => {
                let data = Self::pack_short_message(ID, 0, 0);
                self.port_write(data)?;
                rx
            }
            ChannelStatus::Sub(rx) => rx,
        };
        let response = rx.recv().await?;
        Ok(match response[2] as i8 {
            -0x01 => BayIdent::Standalone,
            0x00 => BayIdent::BayUnknown,
            n if n > 0 && n <= 6 => BayIdent::Bay(n),
            _ => {
                return Err(Error::DeviceError(
                    self.serial_number.clone(),
                    format!("Invalid bay number: {}", response[2]),
                ))
            }
        })
    }
}
