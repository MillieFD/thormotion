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

use crate::messages::utils::{long, short};
use crate::traits::ThorlabsDevice;

const MOVE: [u8; 2] = [0x53, 0x04];
const MOVED: [u8; 2] = [0x64, 0x04];

/// Moves the specified device channel to an absolute position.
#[doc(hidden)]
pub(crate) async fn __move_absolute<A>(device: &A, channel: u8, position: f32)
where
    A: ThorlabsDevice,
{
    device.check_channel(channel);
    let rx = device.inner().receiver(&MOVED).await;
    if rx.is_new() {
        let mut command = long(MOVE, 12);
        command.push(0);
        command.push(channel);
        command.extend(position.to_le_bytes());
        device.inner().send(command).await;
    }
    let response = rx.receive().await;
    match response[2] == channel && response[8..12] == position.to_le_bytes() {
        true => {} // No-op: Move was completed successfully
        false => Box::pin(__move_absolute(device, channel, position)).await,
    }
}

/// Moves the specified device channel to an absolute position (mm) using pre-set parameters
#[doc(hidden)]
pub(crate) async fn __move_absolute_from_params<A>(device: &A, channel: u8) -> f32
where
    A: ThorlabsDevice,
{
    device.check_channel(channel);
    let rx = device.inner().receiver(&MOVED).await;
    if rx.is_new() {
        let command = short(MOVE, channel, 0);
        device.inner().send(command).await;
    }
    let response = rx.receive().await;
    match response[2] == channel {
        true => f32::from_le_bytes([response[8], response[9], response[10], response[11]]),
        false => Box::pin(__move_absolute_from_params(device, channel)).await,
    }
}
