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

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use async_broadcast::broadcast;
use rustc_hash::{FxBuildHasher, FxHashMap};
use smol::lock::{Mutex, MutexGuard};

use crate::devices::{BUG_MESSAGE, abort};
use crate::messages::{Receiver, Sender};

/// A thread-safe message dispatcher for handling async `Req → Get` callback patterns.
///
/// This type includes an internal [`Arc`] to enable inexpensive cloning.
/// The [`Dispatcher`] is released when all clones are dropped.
#[derive(Debug, Clone, Default)]
pub(crate) struct Dispatcher {
    map: Arc<FxHashMap<[u8; 2], Mutex<Option<Sender>>>>,
}

impl Dispatcher {
    /// Constructs a new [`Dispatcher`] from the provided array of command ID bytes.
    pub(crate) fn new(ids: &[[u8; 2]]) -> Self {
        let mut fxmap = HashMap::with_hasher(FxBuildHasher);
        for id in ids {
            fxmap.insert(*id, Mutex::new(None));
        }
        Self {
            map: Arc::new(fxmap),
        }
    }

    // SAFETY: Using Dispatcher::get outside this impl block may allow a channel to remain in the
    // Dispatcher::map after sending a message. Use Dispatcher::take instead.
    #[doc(hidden)]
    async fn get(&self, id: &[u8]) -> MutexGuard<Option<Sender>> {
        self.map
            .get(id)
            .unwrap_or_else(|| abort(format!("Dispatcher does not contain command ID {:?}", id)))
            .lock()
            .await
    }

    // SAFETY: Using Dispatcher::insert outside this impl block may cause an existing sender to
    // drop before it has broadcast. Any existing receivers will await indefinitely.
    /// Creates a new [`broadcast`] channel.
    /// Inserts the [`Sender`] into the [`HashMap`][1] and returns the [`Receiver`].
    ///
    /// [1]: FxHashMap
    #[doc(hidden)]
    fn insert(opt: &mut MutexGuard<Option<Sender>>) -> Receiver {
        let (tx, rx) = broadcast(1);
        opt.replace(tx);
        rx
    }

    /// Returns a receiver for the given command ID.
    ///
    /// If the [`HashMap`] already contains a [`Sender`] for the given command ID, a new
    /// [`Receiver`] is created using [`Sender::new_receiver`] and returned.
    ///
    /// If a [`Sender`] does not exist for the given command ID, a new broadcast channel is created
    /// using [`broadcast`]. The new [`Sender`] is inserted into the [`HashMap`] and the new
    /// [`Receiver`] is returned.
    ///
    /// If you need to guarantee that the device is not currently executing the command for the
    /// given ID, use [`new_receiver`][1].
    ///
    /// [1]: Dispatcher::new_receiver
    pub(crate) async fn any_receiver(&self, id: &[u8]) -> Receiver {
        let mut opt = self.get(id).await;
        match opt.deref() {
            None => Self::insert(&mut opt),
            Some(existing) => existing.new_receiver(),
        }
    }

    /// Returns a [`Receiver`] for the given command ID. Guarantees that the device is not
    /// currently executing the command for the given ID.
    ///
    /// See also [`any_receiver`][1].
    ///
    /// [1]: Dispatcher::any_receiver
    pub(crate) async fn new_receiver(&self, id: &[u8]) -> Receiver {
        let mut opt = self.get(id).await;
        match opt.deref() {
            None => Self::insert(&mut opt),
            Some(existing) => {
                // Wait for the pending command to complete. No need to read the response
                let _ = existing.new_receiver().recv().await;
                // Then call new_receiver recursively to check again.
                Box::pin(async { self.new_receiver(id).await }).await
            }
        }
    }

    /// Removes the [`HashMap`][1] entry for the given command ID.
    ///
    /// - Returns a [`Sender`] if functions are awaiting the command response.
    /// - Returns [`None`] if no functions are awaiting the command response.
    ///
    /// [1]: FxHashMap
    #[doc(hidden)]
    pub(crate) async fn take(&self, id: &[u8]) -> Option<Sender> {
        self.get(id).await.take()
    }

    /// [`Broadcasts`][1] the command response to any waiting receivers.
    ///
    /// [1]: Sender::broadcast
    pub(crate) async fn dispatch(&self, command: Vec<u8>) {
        let data: Arc<[u8]> = Arc::from(command);
        let id: &[u8] = &data[..2];
        if let Some(sender) = self.take(id).await {
            // Sender::broadcast returns an error if either:
            //  1. The channel is closed
            //  2. The channel has no active receivers & Sender::await_active is False
            sender.broadcast_direct(data).await.unwrap_or_else(|err| {
                abort(format!("Broadcast failed\n\n{}\n\n{}", err, BUG_MESSAGE))
            });
        }
    }

    async fn is_some(&self, id: &[u8]) -> bool {
        self.get(id).await.is_some()
    }

    async fn is_none(&self, id: &[u8]) -> bool {
        self.get(id).await.is_none()
    }
}

impl From<&[[u8; 2]]> for Dispatcher {
    fn from(ids: &[[u8; 2]]) -> Self {
        Self::new(ids)
    }
}
