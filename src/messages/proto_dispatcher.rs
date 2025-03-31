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

use rustc_hash::FxBuildHasher;
use smol::lock::Mutex;

use crate::messages::{Dispatcher, Sender};

/**
A mutable precursor used to build a message dispatch [`HashMap`].
*/
#[derive(Debug)]
pub(crate) struct ProtoDispatcher {
    /// The supported APT protocol commands are predetermined by the Thorlabs device type,
    /// meaning we do not need to add nor remove [`HashMap`] entries during operation.
    /// Wrapping the entire HashMap with synchronization primitives is therefore unnecessary.
    /// Instead, we can wrap each [`Sender`] value in its own [`Mutex`] to minimize contention.
    ///
    /// Since cryptographic hashing is not required,
    /// we can use the faster [`fxhash`][`rustc_hash::FxHasher`] algorithm.
    pub(super) inner: HashMap<[u8; 2], Mutex<Option<Sender>>, FxBuildHasher>,
}

impl ProtoDispatcher {
    /**
    Allocates a new empty [`ProtoDispatcher`]
    */
    fn new() -> Self {
        Self {
            inner: HashMap::with_hasher(FxBuildHasher),
        }
    }

    /**
    Inserts a new [`HashMap`] entry for the given `id`.
    Allocates a new [`Mutex`] containing an empty [`Option<Sender>`].
    */
    fn insert(&mut self, id: [u8; 2]) {
        self.inner.insert(id, Mutex::new(None));
    }

    /**
    Consumes the [`ProtoDispatcher`] to return a [`Dispatcher`].
    */
    fn into_dispatcher(mut self) -> Dispatcher {
        self.inner.shrink_to_fit();
        Dispatcher::new(self)
    }
}
