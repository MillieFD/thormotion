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

use ahash::HashMap;
use async_broadcast::broadcast;
use smol::lock::{Mutex, MutexGuard};

use crate::devices::{BUG, global_abort};
use crate::messages::{Receiver, Sender};

/// Indicates whether the wrapped [`Receiver`] is bound to a [`New`][1] or [`Existing`][2]
/// [`Sender`]
///
/// [1]: Provenance::New
/// [2]: Provenance::Existing
pub(crate) enum Provenance {
    /// If a [`Sender`] does not exist for the given command ID, a new [`broadcast`] channel is
    /// created. The new [`Sender`] is inserted into the [`Dispatcher`] [`HashMap`] and the new
    /// [`Receiver`] is returned wrapped in [`Provenance::New`].
    New(Receiver),
    /// If the [`Dispatcher`] [`HashMap`] already contains a [`Sender`] for the given command ID,
    /// a [`new_receiver`][1] is created and returned wrapped in [`Provenance::Existing`].
    ///
    /// [1]: Sender::new_receiver
    Existing(Receiver),
}

impl Provenance {
    fn unwrap(self) -> Receiver {
        match self {
            Provenance::New(rx) => rx,
            Provenance::Existing(rx) => rx,
        }
    }
}

/// A thread-safe message dispatcher for handling async `Req → Get` callback patterns.
///
/// This type includes an internal [`Arc`] to enable inexpensive cloning.
/// The [`Dispatcher`] is released when all clones are dropped.
#[derive(Debug, Clone, Default)]
pub(crate) struct Dispatcher {
    map: Arc<HashMap<[u8; 2], Mutex<Option<Sender>>>>,
}

impl Dispatcher {
    /// Constructs a new [`Dispatcher`] from the provided array of command ID bytes.
    pub(crate) fn new(ids: &[[u8; 2]]) -> Self {
        Self {
            map: Arc::new(HashMap::from_iter(
                ids.iter().map(|id| (*id, Mutex::new(None))),
            )),
        }
    }

    // SAFETY: Using Dispatcher::get outside this impl block may allow a channel to remain in the
    // Dispatcher::map after sending a message. Use Dispatcher::take instead.
    #[doc(hidden)]
    async fn get(&self, id: &[u8]) -> MutexGuard<Option<Sender>> {
        self.map
            .get(id)
            .unwrap_or_else(|| {
                global_abort(format!("Dispatcher does not contain command ID {:?}", id))
            })
            .lock()
            .await
    }

    // SAFETY: Using Dispatcher::insert outside this impl block may cause an existing sender to
    // drop before it has broadcast. Any existing receivers will await indefinitely.
    /// Creates a new [`broadcast channel`][1].
    /// Inserts the [`Sender`] into the [`HashMap`] and returns the [`Receiver`].
    ///
    /// [1]: broadcast
    #[doc(hidden)]
    fn insert(opt: &mut MutexGuard<Option<Sender>>) -> Receiver {
        let (tx, rx) = broadcast(1);
        opt.replace(tx);
        rx
    }

    /// Returns a receiver for the given command ID, wrapped in the [`Provenance`] enum. This is
    /// useful for pattern matching.
    ///
    /// - [`New`][1] → A [`Sender`] does not exist for the given command ID. A new broadcast channel
    ///   is created.
    ///
    /// - [`Existing`][2] → The system is already waiting for a response from the Thorlabs device
    ///   for this command
    ///
    /// If pattern matching is not required, see [`any_receiver`][3] and [`new_receiver`][4] for
    /// simpler alternatives.
    ///
    /// [1]: Provenance::New
    /// [2]: Provenance::Existing
    /// [3]: Dispatcher::any_receiver
    /// [4]: Dispatcher::new_receiver
    pub(crate) async fn receiver(&self, id: &[u8]) -> Provenance {
        let mut opt = self.get(id).await;
        match &*opt {
            None => Provenance::New(Self::insert(&mut opt)),
            Some(existing) => Provenance::Existing(existing.new_receiver()),
        }
    }

    /// Returns a receiver for the given command ID.
    ///
    /// If the [`HashMap`] already contains a [`Sender`] for the given command ID, a
    /// [`new_receiver`][1] is created.
    ///
    /// If a [`Sender`] does not exist for the given command ID, a new [`broadcast channel`][2] is
    /// created. The new [`Sender`] is inserted into the [`HashMap`] and the new [`Receiver`] is
    /// returned.
    ///
    /// If you need to guarantee that the device is not currently executing the command for the
    /// given ID, use [`new_receiver`][3]. If you need pattern matching, see [`receiver`][4].
    ///
    /// [1]: Sender::new_receiver
    /// [2]: broadcast
    /// [3]: Dispatcher::new_receiver
    /// [4]: Dispatcher::receiver
    pub(crate) async fn any_receiver(&self, id: &[u8]) -> Receiver {
        self.receiver(id).await.unwrap()
    }

    /// Returns a [`Receiver`] for the given command ID. Guarantees that the device is not currently
    /// executing the command for the given ID.
    ///
    /// See also [`any_receiver`][1].
    ///
    /// [1]: Dispatcher::any_receiver
    pub(crate) async fn new_receiver(&self, id: &[u8]) -> Receiver {
        match self.receiver(id).await {
            Provenance::New(rx) => rx,
            Provenance::Existing(rx) => {
                // Wait for the pending command to complete. No need to read the response
                let _ = rx.new_receiver().recv().await;
                // Then call new_receiver recursively to check again.
                Box::pin(async { self.new_receiver(id).await }).await
            }
        }
    }

    /// Removes the [`HashMap`] entry for the given command ID.
    ///
    /// - Returns a [`Sender`] if functions are awaiting the command response.
    /// - Returns [`None`] if no functions are awaiting the command response.
    #[doc(hidden)]
    pub(crate) async fn take(&self, id: &[u8]) -> Option<Sender> {
        self.get(id).await.take()
    }

    /// [`Broadcasts`][1] the command response to any waiting receivers.
    ///
    /// [1]: Sender::broadcast_direct
    pub(crate) async fn dispatch(&self, command: Vec<u8>) {
        let data: Arc<[u8]> = Arc::from(command);
        let id: &[u8] = &data[..2];
        if let Some(sender) = self.take(id).await {
            // Sender::broadcast returns an error if either:
            //  1. The channel is closed
            //  2. The channel has no active receivers & Sender::await_active is False
            sender
                .broadcast_direct(data)
                .await
                .unwrap_or_else(|e| global_abort(format!("Broadcast failed : {} : {}", e, BUG)));
        }
    }
}

impl From<&[[u8; 2]]> for Dispatcher {
    fn from(ids: &[[u8; 2]]) -> Self {
        Self::new(ids)
    }
}
