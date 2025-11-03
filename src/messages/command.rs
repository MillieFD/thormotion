/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use smol::lock::Mutex;

use crate::messages::Sender;

/// The maximum possible size for a Thorlabs APT command
///
/// Currently, no data packet exceeds 255 bytes (Thorlabs APT Protocol, Issue 39, Page 35).
/// The max possible command length is therefore six-bytes (header) plus 255 bytes (data payload).
pub(crate) const CMD_LEN_MAX: usize = 255 + 6;

/// Defines command metadata from the Thorlabs APT Protocol.
/// Used to construct [`Command`] instances.
///
/// 1. Unique two-byte ID
/// 2. Total command length
/// 3. Number of device channels
#[derive(Debug)]
pub(crate) struct Metadata<const CHANNELS: usize> {
    /// Unique two-byte identifier for the command
    pub(super) id: [u8; 2],
    /// Total number of bytes in the command
    pub(crate) length: usize,
}

impl<const CH: usize> Metadata<CH> {
    /// Creates a new [`Metadata`] with the specified ID and length.
    ///
    /// The total `length` consists of:
    /// - Six-byte message header
    /// - Data payload if present
    ///
    /// Currently, no data packet exceeds 255 bytes (Thorlabs APT Protocol, Issue 39, Page 35).
    /// The maximum possible command length is given by [`CMD_LEN_MAX`].
    pub(crate) const fn payload(id: [u8; 2], length: usize) -> Self {
        if length < 6 || length > CMD_LEN_MAX {
            panic!("Invalid command length"); // Compile-time error
        }
        Self { id, length }
    }

    /// Creates a new header-only [`Metadata`] with the specified ID.
    ///
    /// Header-only commands are always six bytes long (Thorlabs APT Protocol, Issue 39, Page 34).
    pub(crate) const fn header(id: [u8; 2]) -> Self {
        Self::payload(id, 6)
    }
}

#[derive(Debug)]
pub(crate) struct Command<const CH: usize> {
    /// Unique two-byte identifier for the command
    pub(super) id: [u8; 2],
    /// Total number of bytes in the command
    pub(crate) length: usize,
    /// A sender for broadcasting command responses to multiple receivers
    pub(super) senders: [Mutex<Option<Sender>>; CH],
}

impl<const CH: usize> Command<CH> {
    /// Construct a new [`Command`] instance from the provided metadata.
    ///
    /// Returns a tuple used to construct the [`Dispatcher`][1] sensers [`HashMap`][2].
    ///
    /// [1]: crate::messages::dispatcher::Dispatcher
    /// [2]: ahash::HashMap
    pub(super) const fn new(m: &Metadata<CH>) -> ([u8; 2], Command<CH>) {
        let cmd = Command {
            id: m.id,
            length: m.length,
            senders: [const { Mutex::new(None) }; CH],
        };
        (m.id, cmd)
    }

    pub(super) const fn sender(&self, channel: usize) -> &Mutex<Option<Sender>> {
        let i = if channel <= 1 { 0 } else { channel - 1 };
        &self.senders[i]
    }
}
