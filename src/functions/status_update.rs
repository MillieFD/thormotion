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

use crate::messages::utils::short;
use crate::traits::{ThorlabsDevice, UnitConversion, Units};

/// Returns the current position (mm), velocity (mm/s), and status bits for the specified device
/// channel.
pub(crate) async fn __get_u_status_update<A>(device: &A, channel: u8) -> (f64, f64)
where
    A: ThorlabsDevice + UnitConversion,
{
    const REQ_USTATUSUPDATE: [u8; 2] = [0x90, 0x04];
    const GET_USTATUSUPDATE: [u8; 2] = [0x91, 0x04];
    device.check_channel(channel);
    let response = loop {
        let rx = device.inner().receiver(&GET_USTATUSUPDATE).await;
        if rx.is_new() {
            let command = short(REQ_USTATUSUPDATE, channel, 0);
            device.inner().send(command).await;
        }
        let response = rx.receive().await;
        if response[6] == channel {
            break response;
        }
    };
    let position = Units::distance_from_slice(&response[8..12]).decode::<A>();
    let velocity = Units::velocity_from_slice(&response[12..14]).decode::<A>();
    (position, velocity)
    // TODO: Fn should also return status bits and motor current
}
