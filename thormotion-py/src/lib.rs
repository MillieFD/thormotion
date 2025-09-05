/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use pyo3::prelude::*;

/* ----------------------------------------------------------------------------- Private modules */

mod kdc101;

/* -------------------------------------------------------------------- Initialize Python module */

#[pymodule(name = "thormotion")]
///A cross-platform motion control library for Thorlabs systems, written in Rust.
fn initialise_thormotion_pymodule(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<kdc101::KDC101>()?;
    Ok(())
}
