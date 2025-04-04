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

use std::sync::Arc;

use crate::devices::{BUG, global_abort};
use crate::messages::{Receiver, Sender};

/// Indicates whether the wrapped [`Receiver`] is bound to a [`New`][1] or [`Existing`][2]
/// [`Sender`]
///
/// [1]: Provenance::New
/// [2]: Provenance::Existing
pub(crate) enum Provenance {
    /// If a [`Sender`] does not exist for the given command ID, a new [`broadcast`][1] channel is
    /// created. The new [`Sender`] is inserted into the [`Dispatcher`][2] [`HashMap`][3] and the
    /// new [`Receiver`] is returned wrapped in [`Provenance::New`].
    ///
    /// [1]: async_broadcast::broadcast
    /// [2]: crate::messages::Dispatcher
    /// [3]: ahash::HashMap
    New(Receiver),
    /// If the [`Dispatcher`][1] [`HashMap`][2] already contains a [`Sender`] for the given command
    /// ID, a [`new_receiver`][3] is created and returned wrapped in [`Provenance::Existing`].
    ///
    /// [1]: crate::messages::Dispatcher
    /// [2]: ahash::HashMap
    /// [3]: Sender::new_receiver
    Existing(Receiver),
}

impl Provenance {
    /// Consumes the [`Provenance`], returning the wrapped [`Receiver`] regardless of whether it is
    /// [`New`][1] or [`Existing`][2].
    ///
    /// This function does not panic.
    ///
    /// [1]: Provenance::New
    /// [2]: Provenance::Existing
    pub(super) fn unpack(self) -> Receiver {
        match self {
            Provenance::New(rx) => rx,
            Provenance::Existing(rx) => rx,
        }
    }

    /// Returns `True` if the [`Provenance`] is [`New`][1].
    ///
    /// [1]: Provenance::New
    pub(crate) fn is_new(&self) -> bool {
        match self {
            Provenance::New(_) => true,
            Provenance::Existing(_) => false,
        }
    }

    /// Consumes the [`Provenance`], returning the message received by the wrapped [`Receiver`].
    pub(crate) async fn receive(self) -> Arc<[u8]> {
        self.unpack().recv_direct().await.unwrap_or_else(|e| {
            global_abort(format!(
                "Failed to receive command from broadcast channel : {} : {}",
                e, BUG
            ))
        })
    }
}
