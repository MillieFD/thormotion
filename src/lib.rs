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

/* ------------------------------------------------------------------------------ Public modules */

pub mod devices;
pub mod error;

/* ----------------------------------------------------------------------------- Private modules */

mod functions;
mod messages;
mod traits;

/* -------------------------------------------------------------------- Initialize Python module */

mod py_module {
    use crate::devices::*;
    use pyo3::prelude::*;

    #[pymodule(name = "thormotion")]
    ///A cross-platform motion control library for Thorlabs systems, written in Rust.
    fn initialise_thormotion_pymodule(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add_class::<KDC101>()?;
        Ok(())
    }
}

/* --------------------------------------------------------------------------------------- Tests */

#[cfg(test)]
mod tests {
    #[test]
    fn identify_kdc101() {
        use crate::devices::KDC101;
        smol::block_on(async {
            let serial_number = String::from("27xxxxxx");
            let mut device = KDC101::new(serial_number).await.unwrap();
            device.open().await.unwrap();
            device.identify().await;
        })
    }
}
