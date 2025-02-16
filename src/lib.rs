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

/// # Tokio Runtime
/// The `tokio` runtime is a core component used to manage and execute asynchronous tasks.
/// Tokio is an asynchronous runtime for the Rust programming language that allows you to run and
/// manage non-blocking I/O tasks.
/// It facilitates running async/await code in Rust by providing an event-driven execution model.
///
/// # Why is the Tokio runtime necessary in this project?
///
/// In this project, the `tokio::runtime` is used to enable and manage asynchronous operations
/// for interacting with external devices.
/// For example, after sending a `MOT_MOVE_HOME (0x0443)` command to a motor controller,
/// the host computer doesn't know when, or indeed if, the controller will send the
/// `MOT_MOVE_HOMED (0x0444)` response.
/// Motor homing is a physical process which may take varying lengths of time depending on
/// physical factors beyond the host computer's knowledge or control.
/// For this reason, we must `.await` the response using `tokio` non-blocking async/await code.
///
/// By using `Tokio`, multiple async operations can run concurrently, ensuring better performance
/// while handling I/O tasks or delays without blocking the executor.
///
/// # Member Functions of Tokio Runtime
///
/// The `tokio::runtime` provides several templates, but in this context, `Runtime::new()`
/// and `Handle::try_current()` are primarily used:
///
/// - `Runtime::new()`: Creates a new async runtime. It is used when there is no existing runtime
///   for the current thread (e.g., if running outside of an async-aware context).
/// -
///
/// # Initialisation Logic
///
/// The `RUNTIME_HANDLE` constant uses a `LazyLock` initialisation pattern to ensure that the
/// runtime is only created when needed.
///
/// `Handle::try_current()`: Attempts to grab a runtime handle for the current thread.
/// This prevents conflicting runtimes when the project is already running in an async context,
/// such as when using the `#[tokio::main]` or `#[tokio::test]` macros.
///
/// If a runtime doesn't yet exist, `Handle::try_current()` fails,
/// and a new runtime is created using `Runtime::new()`.

// const RUNTIME_HANDLE: LazyLock<Handle> = LazyLock::new(|| {
//     Handle::try_current().unwrap_or_else(|_| {
//         Runtime::new()
//             .map_err(|err| format!("Failed to initialise Tokio runtime: {}", err))
//             .unwrap()
//             .handle()
//             .clone()
//     })
// });

/*
todo write tests which cycle through all templates
create test device struct with implements every trait
without checking serial number
separate test for each function
failed test skips to next test without stopping the whole process
output to csv
 */

#[cfg(test)]
mod tests {
    use crate::devices::KDC101;
    use crate::error::Error;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn identify_device_test() -> Result<(), Error> {
        let device = KDC101::new("27266788");
        device.set_channel_enable_state(1, true)?;
        device.home(1)?;
        device.move_absolute(1, 1.0)?;
        sleep(Duration::from_secs(1)).await;
        device.move_absolute(1, 0.0)?;

        // let device = KDC101::new("27264344")?;
        // device.identify()?;

        // let device = KDC101::new("27266825")?;
        // device.identify()?;

        Ok(())
    }
}
