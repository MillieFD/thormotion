/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

/* ------------------------------------------------------------------------------ Public modules */

pub mod devices;
pub mod error;

/* ----------------------------------------------------------------------------- Private modules */

mod functions;
mod messages;
mod traits;

/* ------------------------------------------------------------------------------- Python Module */

#[cfg(feature = "py")]
mod py {
    use pyo3::prelude::*;

    use crate::devices::*;
    #[pymodule(name = "thormotion")]
    ///A cross-platform motion control library for Thorlabs systems, written in Rust.
    fn initialise_thormotion_pymodule(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add_class::<KDC101>()?;
        Ok(())
    }
}

/* ------------------------------------------------------------------------------ Public Exports */

pub use traits::ThorlabsDevice;

/* --------------------------------------------------------------------------------------- Tests */

#[cfg(test)]
mod tests {
    use crate::devices::*;

    /// Initialises the logging infrastructure for tests.
    /// Logging output is captured and displayed during test execution.
    fn logger(level: log::LevelFilter) {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(level)
            .try_init();
    }

    #[test]
    fn kdc101() {
        logger(log::LevelFilter::Trace);
        let mut device = KDC101::new("27XXX").unwrap();
        device.open().unwrap();
        device.identify();
    }
}
