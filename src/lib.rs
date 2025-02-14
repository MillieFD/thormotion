mod devices;
mod enumerate;
mod env;
mod error;
mod messages;
mod traits;

#[pymodule]
fn thormotion(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<KDC101>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::KDC101;
    use crate::traits::{ChanEnableState, Motor, ThorlabsDevice};
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn identify_device_test() -> Result<(), Error> {
        let device = KDC101::new("27266788")?;
        device.set_channel_enable_state(1, true).await?;
        device.home(1).await?;
        device.move_absolute(1, 1.0).await?;
        sleep(Duration::from_secs(1)).await;
        device.move_absolute(1, 0.0).await?;

        // let device = KDC101::new("27264344")?;
        // device.identify()?;
        //
        // let device = KDC101::new("27266825")?;
        // device.identify()?;
        Ok(())
    }
}
