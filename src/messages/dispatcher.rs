/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

use ahash::HashMap;
use async_broadcast::broadcast;
use smol::lock::MutexGuard;

use crate::devices::{abort, bug_abort};
use crate::messages::{Command, Metadata, Provenance, Receiver, Sender};

/// A thread-safe message dispatcher for handling async `Req → Get` callback patterns.
///
/// This type includes an internal [`Arc`] to enable inexpensive cloning.
/// The [`Dispatcher`] is released when all clones are dropped.
#[derive(Debug, Clone, Default)]
pub(crate) struct Dispatcher<const CH: usize> {
    /// A unique eight-digit serial number that is printed on the Thorlabs device.
    serial_number: String,
    /// A [`HashMap`] of `Message ID` keys and [`Command`] values.
    map: Arc<HashMap<[u8; 2], Command<CH>>>,
}

impl<const CH: usize> Dispatcher<CH> {
    /// Constructs a new [`Dispatcher`] from the provided array of command ID bytes.
    pub(crate) fn new(ids: &[Metadata<CH>], serial_number: &String) -> Self {
        Self {
            serial_number: serial_number.clone(),
            map: Arc::new(HashMap::from_iter(ids.iter().map(Command::new))),
        }
    }

    pub(crate) fn serial_number(&self) -> &String {
        &self.serial_number
    }

    /// Returns a reference to the [`Command`] corresponding to the ID.
    #[doc(hidden)]
    async fn get(&self, id: &[u8]) -> &Command<CH> {
        // SAFETY: Using Dispatcher::get outside this impl block may allow a channel to remain in
        // the Dispatcher::map after sending a message. Use Dispatcher::take instead.
        self.map
            .get(id)
            .unwrap_or_else(|| abort(format!("{self} does not contain command ID {id:02X?}")))
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
    /// If pattern matching is not required, see [`new_receiver`][4] for a simpler alternative.
    /// This guarantees that the device is not currently executing the command for the given ID.
    ///
    /// [1]: Provenance::New
    /// [2]: Provenance::Existing
    /// [4]: Dispatcher::new_receiver
    pub(crate) async fn receiver(&self, id: &[u8], channel: usize) -> Provenance {
        let mut opt = self.get(id).await.sender(channel).lock().await;
        match &*opt {
            None => Provenance::New(Self::insert(&mut opt)),
            Some(existing) => Provenance::Existing(existing.new_receiver()),
        }
    }

    /// Returns a [`Receiver`] for the given command ID. Guarantees that the device is not currently
    /// executing the command for the given ID.
    pub(crate) async fn new_receiver(&self, id: &[u8], channel: usize) -> Provenance {
        log::debug!("NEW RECEIVER (requested) ID {id:02X?} CHANNEL {channel}");
        loop {
            let rx = self.receiver(id, channel).await;
            if rx.is_new() {
                // Break out of the loop and return the new Receiver
                log::debug!("NEW RECEIVER (success) ID {id:02X?} CHANNEL {channel}");
                return rx;
            } else {
                // Wait for the pending command to complete. No need to read the response
                log::debug!("NEW RECEIVER (waiting) ID {id:02X?} CHANNEL {channel}");
                let _ = rx.receive().await;
            }
        }
    }

    /// Takes the [`Sender`] out of the [`Dispatcher`] if functions are awaiting the command
    /// response, leaving [`None`] in its place.
    ///
    /// Returns [`None`] if no functions are awaiting the command response.
    #[doc(hidden)]
    pub(crate) async fn take(&self, id: &[u8], channel: usize) -> Option<Sender> {
        self.get(id).await.sender(channel).lock().await.take()
    }

    /// Returns the expected length (number of bytes) for the given command ID.
    pub(crate) async fn length(&self, id: &[u8]) -> usize {
        self.get(id).await.length
    }

    /// [`Broadcasts`][1] the command response to any waiting receivers.
    ///
    /// [1]: Sender::broadcast_direct
    pub(crate) async fn dispatch(&self, data: Arc<[u8]>, channel: usize) {
        let id: &[u8] = &data[..2];
        if let Some(sender) = self.take(id, channel).await {
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

impl<const CH: usize> Display for Dispatcher<CH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DISPATCHER {}", self.serial_number)
    }
}
