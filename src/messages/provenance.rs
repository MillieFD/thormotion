/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::sync::Arc;

use crate::devices::bug_abort;
use crate::messages::Receiver;

/// Indicates whether the wrapped [`Receiver`] is bound to a [`New`][1] or [`Existing`][2]
/// [`Sender`][3]
///
/// [1]: Provenance::New
/// [2]: Provenance::Existing
/// [3]: crate::messages::Sender
#[derive(Debug, Clone)]
pub(crate) enum Provenance {
    /// If a [`Sender`][1] does not exist for the given command ID, a new [`broadcast`][2] channel
    /// is created. The new [`Sender`][1] is inserted into the [`Dispatcher`][3] [`HashMap`][4]
    /// and the new [`Receiver`] is returned wrapped in [`Provenance::New`].
    ///
    /// [1]: crate::messages::Sender
    /// [2]: async_broadcast::broadcast
    /// [3]: crate::messages::Dispatcher
    /// [4]: ahash::HashMap
    New(Receiver),
    /// If the [`Dispatcher`][1] [`HashMap`][2] already contains a [`Sender`][3] for the given
    /// command ID, a [`new_receiver`][4] is subscribed to the existing channel and returned wrapped
    /// in [`Provenance::Existing`].
    ///
    /// [1]: crate::messages::Dispatcher
    /// [2]: ahash::HashMap
    /// [3]: crate::messages::Sender
    /// [4]: async_broadcast::Sender::new_receiver
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
            bug_abort(format!(
                "Failed to receive command from broadcast channel : {}",
                e
            ))
        })
    }
}
