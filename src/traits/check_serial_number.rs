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

use crate::error::{sn, Error};

pub(crate) trait CheckSerialNumber {
    /**
    Each Thorlabs device has a unique serial number prefix.
    For instance, all KDC101 devices have serial numbers beginning with "27".
    The `check_serial_number` function uses the `Self::SERIAL_NUMBER_PREFIX` constant to prevent
    users from accidentally opening a device using the incorrect struct.
    */
    const SERIAL_NUMBER_PREFIX: &'static str;

    /**
    Returns `Error::InvalidSerialNumber` if the serial number:
    1. Does not match the serial number prefix for the target device type
    2. Is not exactly eight-digits long
    3. Contains non-numeric characters
    */
    fn check_serial_number(serial_number: String) -> Result<(), Error> {
        if serial_number.starts_with(Self::SERIAL_NUMBER_PREFIX)
            && serial_number.len() == 8
            && serial_number.chars().all(|c| c.is_numeric())
        {
            Ok(())
        } else {
            Err(Error::SerialNumber(sn::Error::Invalid(serial_number)))
        }
    }
}
