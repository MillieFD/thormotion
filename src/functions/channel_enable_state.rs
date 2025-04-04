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

use crate::devices::global_abort;
use crate::messages::utils::short;
use crate::traits::ThorlabsDevice;

/// Returns `True` if the specified device channel is enabled.
#[doc(hidden)]
pub(crate) async fn __req_channel_enable_state<A>(device: &A, channel: u8) -> bool
where
    A: ThorlabsDevice,
{
    const REQ: [u8; 2] = [0x11, 0x02];
    const GET: [u8; 2] = [0x12, 0x02];
    
    device.check_channel(channel);
    let rx = device.inner().receiver(&GET).await;
    if rx.is_new() {
        let command = short(REQ, channel, 0);
        device.inner().send(command).await;
    }
    let response = rx.receive().await;
    if channel == response[2] {
        match response[3] {
            0x01 => true,
            0x02 => false,
            _ => global_abort(format!(
                "{} GET_CHANENABLESTATE contained invalid channel enable state : {}",
                device, response[3]
            )),
        }
    } else {
        Box::pin(async { __req_channel_enable_state(device, channel).await }).await
    }
}

/// Enables or disables the specified device channel.
#[doc(hidden)]
pub(crate) async fn __set_channel_enable_state<A>(device: &A, channel: u8, enable: bool)
where
    A: ThorlabsDevice,
{
    const SET: [u8; 2] = [0x10, 0x02];
    const REQ: [u8; 2] = [0x11, 0x02];
    const GET: [u8; 2] = [0x12, 0x02];

    device.check_channel(channel);
    let enable_byte: u8 = if enable { 0x01 } else { 0x02 };
    let rx = device.inner().receiver(&GET).await;
    if rx.is_new() {
        let set = short(SET, channel, enable_byte);
        device.inner().send(set).await;
        let req = short(REQ, channel, 0);
        device.inner().send(req).await;
    }
    let response = rx.receive().await;
    match response[2] == channel && response[3] == enable_byte {
        true => {} // No-op: Enable state was set successfully
        false => {
            Box::pin(__set_channel_enable_state(device, channel, enable)).await;
        }
    }
}
