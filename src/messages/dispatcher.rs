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
use std::sync::Arc;

use smol::lock::{Mutex, MutexGuard};

use crate::messages::{ProtoDispatcher, Sender};

/**
A thread-safe message dispatcher for handling async `Req → Get` callback patterns.
This type includes an internal [`Arc`] to enable inexpensive cloning.
The [`Dispatcher`] is released when all clones are dropped.
 */
#[derive(Debug, Clone)]
pub(crate) struct Dispatcher {
    inner: Arc<HashMap<[u8; 2], Mutex<Option<Sender>>>>,
}

impl Dispatcher {
    pub(super) fn new(dispatch_map: ProtoDispatcher) -> Self {
        Self {
            inner: Arc::new(dispatch_map.inner),
        }
    }

    async fn get(&self, id: &[u8]) -> MutexGuard<Option<Sender>> {
        self.inner
            .get(id)
            .unwrap_or_else(|| panic!("Dispatcher does not contain command ID {:?}", id))
            .lock()
            .await
    }

    async fn take(&self, id: &[u8]) -> Option<Sender> {
        self.get(id).await.take()
    }

    pub(crate) async fn dispatch(&self, message: Vec<u8>) {
        let data: Arc<[u8]> = Arc::from(message);
        let id: &[u8] = &data[..2];
        if let Some(sender) = self.take(id).await {
            sender.broadcast(data).await.unwrap_or_else(|err| {
                panic!(
                    "Failed to broadcast message:\n\n\tThis error is returned from \
                     `Sender::broadcast` if either:\n\t\t- The channel is closed\n\t\t- The \
                     channel has no active receivers and `Sender::await_active` is \
                     `false`\n\n\tUnsent message: {:?}",
                    err.0
                )
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

impl From<ProtoDispatcher> for Dispatcher {
    fn from(dispatch_map: ProtoDispatcher) -> Self {
        Self::new(dispatch_map)
    }
}
