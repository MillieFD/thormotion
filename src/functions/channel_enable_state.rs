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

use crate::durations::{DEFAULT_LONG_TIMEOUT, DEFAULT_SHORT_TIMEOUT};
use crate::error::Error;
use crate::macros::*;
use crate::messages::utils::{get_new_receiver, pack_short_message, wait_until_clear_to_send};
use crate::traits::ThorlabsDevice;
use async_std::future::timeout;
use async_std::task::sleep;

#[doc(hidden)]
#[doc = "Returns `True` if the specified device channel is enabled."]
#[doc = apt_doc!(async, "MOD_REQ_CHANENABLESTATE", "MOD_GET_CHANENABLESTATE", RUSB, Timeout, FatalError)]
pub(crate) async fn __get_channel_enable_state_async<A>(
    device: &A,
    channel: i32,
) -> Result<bool, Error>
where
    A: ThorlabsDevice,
{
    const ID: [u8; 2] = [0x11, 0x02];
    let receiver = get_new_receiver(ID).await?;
    let data = pack_short_message(ID, channel.clone(), 0);
    device.write(&data)?;
    let response = timeout(DEFAULT_LONG_TIMEOUT, receiver.recv()).await??;
    if response[2] != data[2] {
        return Err(Error::wrong_channel(device, channel, data[2]));
    };
    let enable_state = match response[3] {
        0x01 => true,
        0x02 => false,
        _ => {
            return Err(Error::unsuccessful_command(
                device,
                format!(
                    "Response contained invalid channel enable state: {}",
                    response[3]
                ),
            ));
        }
    };
    Ok(enable_state)
}

#[doc(hidden)]
#[doc = "Enables or disables the specified device channel."]
#[doc = apt_doc!(async, "MOD_SET_CHANENABLESTATE", "MOD_REQ_CHANENABLESTATE", "MOD_GET_CHANENABLESTATE", RUSB, Timeout, FatalError)]
pub(crate) async fn __set_channel_enable_state_async<A>(
    device: &A,
    channel: i32,
    enable: bool,
) -> Result<(), Error>
where
    A: ThorlabsDevice,
{
    const ID: [u8; 2] = [0x10, 0x02];
    let enable_byte: u8 = if enable { 0x01 } else { 0x02 };
    wait_until_clear_to_send(ID).await?;
    let data = pack_short_message(ID, channel.clone(), enable_byte);
    device.write(&data)?;
    sleep(DEFAULT_SHORT_TIMEOUT).await;
    if __get_channel_enable_state_async(device, channel.clone()).await? != enable {
        return Err(Error::unsuccessful_command(
            device,
            format!(
                "Failed to set channel number {:?} enable state to {}",
                channel, enable
            ),
        ));
    };
    Ok(())
}
