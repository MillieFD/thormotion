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

mod messages;
mod functions;
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
    // #[test]
    // fn identify_kdc101() {
    //     use crate::devices::KDC101;
    //     smol::block_on(async {
    //         let serial_number = String::from("27xxxxxx");
    //         let mut device = KDC101::new(serial_number).await.unwrap();
    //         device.open().await.unwrap();
    //         device.identify().await;
    //     })
    // }
}
