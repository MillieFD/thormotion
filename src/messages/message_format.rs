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

use std::fmt::{Display, Formatter};
use std::ops::Deref;

/**
Wraps the bytes of a message to indicate whether the message is `Short`
(six-byte header only) or `Long` (six-byte header plus variable length data package).

# Thorlabs APT Protocol

The Thorlabs APT communication protocol uses a fixed length six-byte message header,
which may be followed by a variable-length data packet. For simple commands,
the six-byte message header is sufficient to convey the entire command. For more complex commands
(e.g. commands where a set of parameters needs to be passed to the device)
the six-byte header is followed by a variable-length data packet.

The `MsgFormat` enum encapsulates these two forms of messages:

- `Short`: A six-byte message header only.
- `Long`: A six-byte message header followed by an arbitrarily long data packet.

The `MsgFormat` enum makes it easier to distinguish between `Short` and `Long` messages.
This simplifies situations where different logic must be applied to the different message types.

# Traits

- Implements `Deref` to facilitate easy access to the underlying message bytes.
- Implements `Display` for easier message debugging.

# Examples

```rust
use thormotion::MsgFormat;

// Create a short message
let short_message = MsgFormat::Short([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);

// Create a long message
let long_message = MsgFormat::Long(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
```
*/
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum MsgFormat {
    Short([u8; 6]),
    Long(Vec<u8>),
}

impl MsgFormat {
    fn len(&self) -> usize {
        match self {
            MsgFormat::Short(arr) => arr.len(),
            MsgFormat::Long(vec) => vec.len(),
        }
    }
}

impl Deref for MsgFormat {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match self {
            MsgFormat::Short(arr) => arr,
            MsgFormat::Long(vec) => vec.as_slice(),
        }
    }
}

impl Extend<u8> for MsgFormat {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        match self {
            MsgFormat::Short(arr) => {
                let mut vec = arr.to_vec();
                vec.extend(iter);
                *self = MsgFormat::Long(vec);
            }
            MsgFormat::Long(vec) => vec.extend(iter),
        }
    }
}

impl Display for MsgFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MsgFormat::Short(arr) => {
                write!(
                    f,
                    "Short Message [ {} ]",
                    arr.iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            MsgFormat::Long(vec) => {
                write!(
                    f,
                    "Long Message [ {} ]",
                    vec.iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
        }
    }
}
