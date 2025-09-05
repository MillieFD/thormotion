/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::sync::Arc;

use ahash::HashMap;
use async_broadcast::broadcast;
use smol::lock::MutexGuard;

use crate::devices::{abort, bug_abort};
use crate::messages::{Command, Provenance, Receiver, Sender};

/// A thread-safe message dispatcher for handling async `Req → Get` callback patterns.
///
/// This type includes an internal [`Arc`] to enable inexpensive cloning.
/// The [`Dispatcher`] is released when all clones are dropped.
#[derive(Debug, Clone, Default)]
pub(crate) struct Dispatcher {
    map: Arc<HashMap<[u8; 2], Command>>,
}

impl Dispatcher {
    /// Constructs a new [`Dispatcher`] from the provided array of command ID bytes.
    pub(crate) fn new(ids: &[Command]) -> Self {
        Self {
            map: Arc::new(HashMap::from_iter(
                ids.iter()
                    .map(|cmd| (cmd.id, Command::payload(cmd.id, cmd.length))),
            )),
        }
    }

    /// Returns a reference to the [`Command`] corresponding to the ID.
    #[doc(hidden)]
    async fn get(&self, id: &[u8]) -> &Command {
        // SAFETY: Using Dispatcher::get outside this impl block may allow a channel to remain in
        // the Dispatcher::map after sending a message. Use Dispatcher::take instead.
        self.map
            .get(id)
            .unwrap_or_else(|| abort(format!("Dispatcher does not contain command ID {:?}", id)))
    }

    /// Creates a new [`broadcast channel`][1].
    /// Inserts the [`Sender`] into the [`HashMap`] and returns the [`Receiver`].
    ///
    /// [1]: broadcast
    #[doc(hidden)]
    fn insert(opt: &mut MutexGuard<Option<Sender>>) -> Receiver {
        // SAFETY: Using Dispatcher::insert outside this impl block may cause an existing sender to
        // drop before it has broadcast. Any existing receivers will await indefinitely.
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
        let mut opt = self.get(id).await.sender.lock().await;
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
        self.receiver(id).await.unpack()
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
        self.get(id).await.sender.lock().await.take()
    }

    /// Returns the expected length (number of bytes) for the given command ID.
    pub(crate) async fn length(&self, id: &[u8]) -> usize {
        self.get(id).await.length
    }

    /// [`Broadcasts`][1] the command response to any waiting receivers.
    ///
    /// [1]: Sender::broadcast_direct
    pub(crate) async fn dispatch(&self, data: Arc<[u8]>) {
        let id: &[u8] = &data[..2];
        if let Some(sender) = self.take(id).await {
            // Sender::broadcast returns an error if either:
            //  1. The channel is closed
            //  2. The channel has no active receivers & Sender::await_active is False
            sender
                .broadcast_direct(data)
                .await
                .unwrap_or_else(|e| bug_abort(format!("Broadcast failed : {}", e)));
        }
    }
}

impl From<&[Command]> for Dispatcher {
    fn from(ids: &[Command]) -> Self {
        Self::new(ids)
    }
}
