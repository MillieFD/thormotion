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

use crate::error::Error;
use crate::macros::*;
use crate::messages::utils::pack_short_message;
use crate::traits::ThorlabsDevice;

#[doc = "Starts periodic update messages from the device every 100 milliseconds (10 Hz)."]
#[doc = "Automatic updates will continue until the `stop_update_messages` function is called."]
#[doc = "A 'one-off' status update can be requested using `get_status_update_async`."]
#[doc = apt_doc!(async, "HW_START_UPDATEMSGS", "GET_STATUSTUPDATE", RUSB)]
pub(crate) fn __start_update_messages<A>(device: &A) -> Result<(), Error>
where
    A: ThorlabsDevice,
{
    const ID: [u8; 2] = [0x11, 0x00];
    let data = pack_short_message(ID, 0, 0);
    device.write(&data)?;
    Ok(())
}

#[doc = "Stops periodic update messages from the device every 100 milliseconds (10 Hz)."]
#[doc = "Automatic updates will cease until the `start_update_messages` function is called."]
#[doc = apt_doc!(async, "HW_STOP_UPDATEMSGS", RUSB)]
pub(crate) fn __stop_update_messages<A>(device: &A) -> Result<(), Error>
where
    A: ThorlabsDevice,
{
    const ID: [u8; 2] = [0x12, 0x00];
    let data = pack_short_message(ID, 0, 0);
    device.write(&data)?;
    Ok(())
}
