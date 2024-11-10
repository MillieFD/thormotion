pub mod devices;
pub mod enumerate;
mod env;
pub mod error;
mod messages;
mod traits;

pub use error::Error;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::KDC101;
    use crate::traits::ThorlabsDevice;

    #[tokio::test]
    async fn identify_device_test() -> Result<(), Error> {
        let device = KDC101::new("27266788")?;
        device.identify()?;
        Ok(())
    }
}
