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

use crate::durations::DEFAULT_LONG_TIMEOUT;
use crate::error::Error;
use crate::macros::*;
use crate::messages::utils::{get_new_receiver, pack_long_message, pack_short_message};
use crate::traits::{DistanceAngleConversion, ThorlabsDevice};
use async_std::future::timeout;
use std::fmt::Debug;

#[doc(hidden)]
#[doc = "Moves the specified device channel to an absolute position"]
#[doc = apt_doc!(async, "MOT_MOVE_ABSOLUTE", "MOT_MOVE_COMPLETED ", RUSB, Timeout, FatalError)]
pub(crate) async fn __move_absolute_async<A, B>(
    device: &A,
    channel: i32,
    absolute_position: B,
) -> Result<(), Error>
where
    A: ThorlabsDevice + DistanceAngleConversion,
    B: Into<f64> + Clone + Debug,
{
    const ID: [u8; 2] = [0x53, 0x04];
    const LENGTH: usize = 12;
    let receiver = get_new_receiver(ID).await?;
    let mut data = pack_long_message(ID, LENGTH);
    let channel_bytes: [u8; 2] = u16::try_from(channel)?.to_le_bytes();
    data.extend(channel_bytes);
    data.extend(A::position_angle_to_le_bytes(absolute_position));
    device.write(&data)?;
    let _response = timeout(DEFAULT_LONG_TIMEOUT, receiver.recv()).await??;
    Ok(())
}

#[doc(hidden)]
#[doc = "Moves the specified device channel to an absolute position (mm) using pre-set parameters"]
#[doc = apt_doc!(sync, "MOT_MOVE_ABSOLUTE", "MOT_MOVE_COMPLETED ", RUSB, Timeout, FatalError)]
pub(crate) async fn __move_absolute_from_params_async<A>(
    device: &A,
    channel: i32,
) -> Result<(), Error>
where
    A: ThorlabsDevice,
{
    const ID: [u8; 2] = [0x53, 0x04];
    let receiver = get_new_receiver(ID).await?;
    let data = pack_short_message(ID, channel, 0);
    device.write(&data)?;
    let _response = timeout(DEFAULT_LONG_TIMEOUT, receiver.recv()).await??;
    Ok(())
}
