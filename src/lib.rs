mod devices;
mod enumeration;
mod env;
mod errors;
mod messages;
mod traits;

use crate::enumeration::enumerate::get_device;
use crate::errors::error_types::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_device_connection_and_message() -> Result<(), Error> {
        println!("Starting test_device_connection_and_message");
        let device = get_device("27266788").await?;
        let message = Box::new([0x23, 0x02, 0x01, 0x00, 0x50, 0x01]);
        device.write_port(message)?;
        Ok(())
    }
}
