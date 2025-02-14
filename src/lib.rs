/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License

Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
this list of conditions and the following disclaimer in the documentation
and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
contributors may be used to endorse or promote products derived from
this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use crate::devices::*;
use pyo3::prelude::*;
use pyo3::{pymodule, Bound, PyResult};
mod devices;
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

/*
todo write tests which cycle through all templates
create test device struct with implements every trait
without checking serial number
separate test for each function
failed test skips to next test without stopping the whole process
output to csv
 */
