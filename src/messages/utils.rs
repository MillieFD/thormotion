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
use crate::messages::MsgFormat;
use std::fmt::Debug;

const DEST: u8 = 0x50;
const SOURCE: u8 = 0x01;

/**
Packs a short six-byte (header only) message into the `MsgFormat::Short` enum.

# Thorlabs APT Protocol

The Thorlabs APT communication protocol uses a fixed length six-byte message header,
which may be followed by a variable-length data packet.
For simple commands, the six-byte message header is enough to convey the entire command.
For more complex commands (e.g. commands where a set of parameters needs to be passed
to the device), the six-byte header is followed by a variable-length data packet.

# MsgFormat Enum

The `MsgFormat` enum wraps the bytes of a message to indicate whether the message is `Short`
(six-byte header only) or `Long` (six-byte header plus variable length data package).

# Pack Functions

The `pack_short_message()` and `pack_long_message()` helper functions simplify message
formatting and enforce consistency with the APT protocol.
 */
pub(crate) fn pack_short_message<A, B>(id: [u8; 2], param_one: A, param_two: B) -> MsgFormat
where
    A: TryInto<u8> + Clone + Debug,
    <A as TryInto<u8>>::Error: Debug,
    B: TryInto<u8> + Clone + Debug,
    <B as TryInto<u8>>::Error: Debug,
{
    let param_one_u8: u8 = param_one
        .clone()
        .try_into()
        .expect(&format!("Failed to convert {:?} to u8", param_one));
    let param_two_u8: u8 = param_two
        .clone()
        .try_into()
        .expect(&format!("Failed to convert {:?} to u8", param_two));
    MsgFormat::Short([id[0], id[1], param_one_u8, param_two_u8, DEST, SOURCE])
}

/**
Packs a long message (six-byte header plus variable length data package)
message into the `MsgFormat::Long` enum.

# Thorlabs APT Protocol

The Thorlabs APT communication protocol uses a fixed length six-byte message header,
which may be followed by a variable-length data packet.
For simple commands, the six-byte message header is enough to convey the entire command.
For more complex commands (e.g. commands where a set of parameters needs to be passed
to the device) the six-byte header is followed by a variable-length data packet.

# MsgFormat Enum

The `MsgFormat` enum wraps the bytes of a message to indicate whether the message is `Short`
(six-byte header only) or `Long` (six-byte header plus variable length data package).

# Pack Functions

The `pack_short_message()` and `pack_long_message()` helper functions simplify message
formatting and enforce consistency with the APT protocol.
*/
pub(crate) fn pack_long_message(id: [u8; 2], length: usize) -> MsgFormat {
    let mut data: Vec<u8> = Vec::with_capacity(length);
    data.extend(id);
    data.extend(((length - 6) as u16).to_le_bytes());
    data.push(DEST | 0x80);
    data.push(SOURCE);
    MsgFormat::Long(data)
}

pub(crate) async fn wait_until_clear_to_send(message_id: [u8; 2]) -> Result<(), Error> {
    if let Some(existing_channel) = timeout(DEFAULT_LONG_TIMEOUT, get_channel(message_id).read())
        .await?
        .as_ref()
    {
        existing_channel.receiver.recv().await?;
    }
    Ok(())
}
