/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

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

// Private modules

mod devices;
mod durations;
mod functions;
mod macros;
mod messages;
mod traits;

// Public modules

pub mod error;

// Public Exports

pub use crate::devices::KDC101;

// Initialize PyO3 Python module

use pyo3::prelude::*;

#[pymodule(name = "thormotion")]
#[doc = "A cross-platform motion control library for Thorlabs systems, written in Rust."]
fn initialise_thormotion_pymodule(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<KDC101>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::devices::KDC101;
    use crate::error::Error;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_kdc101() -> Result<(), Error> {
        let device = KDC101::new("27264344")?;
        device.home()?;
        let mut i = 0.0;
        while i <= 5.0 {
            device.move_absolute(i)?;
            i += 0.05;
            sleep(Duration::from_secs(2));
        }
        Ok(())
    }
}
