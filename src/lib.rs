use crate::devices::*;
use pyo3::prelude::*;
use pyo3::{pymodule, Bound, PyResult};
mod devices;
mod enumerate;
mod env;
mod error;
mod messages;

/// # Exposing the Thormotion Library to Python
///
/// The **Thormotion** Rust library is published as a Python package using `PyO3` and `maturin`.
///
/// * `PyO3` is a Rust library that provides tools and macros for creating Python bindings,
/// allowing Rust code to be called directly from Python.
///
/// * `maturin` is a build system specifically designed for building and publishing
/// Python packages written in Rust.
///
/// ## PyModule
/// The `#[pymodule]` attribute is used within this library to define the module that will be
/// exposed to Python.
/// This attribute is assigned to a function which returns `PyResult<()>`.
/// This function is the entry point for the Python module and is responsible for registering
/// Rust structures and templates as Python classes.
///
/// ```rust
/// #[pymodule]
/// fn thormotion(module: &Bound<'_, PyModule>) -> PyResult<()> {
///     module.add_class::<ClassName1>()?;
///     module.add_class::<ClassName2>()?;
///     Ok(())
/// }
/// ```
///
/// In this case, the function is named `thormotion`, which means that the resulting module is
/// also named `thormotion`.
/// Once compiled and installed as a Python package, this module can be imported in Python using
/// `import thormotion`

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
