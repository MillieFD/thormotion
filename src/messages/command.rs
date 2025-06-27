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

#[derive(Debug)]
pub(crate) struct Command {
    /// Unique two-byte identifier for the command
    pub(super) id: [u8; 2],
    /// Total number of bytes in the command
    pub(crate) length: usize,
    /// A sender for broadcasting command responses to multiple receivers
    pub(super) sender: Mutex<Option<Sender>>,
}

impl Command {
    /// Creates a new [`Command`] with the specified ID and length.
    ///
    /// The total `length` consists of:
    /// - Six-byte message header
    /// - Data payload if present
    ///
    /// Currently, no data packet exceeds 255 bytes (Thorlabs APT Protocol, Issue 39, Page 35).
    /// The maximum possible command length is given by [`CMD_LEN_MAX`].
    pub(crate) const fn payload(id: [u8; 2], length: usize) -> Self {
        if length < 6 || length > CMD_LEN_MAX {
            panic!("Invalid command length");
        }
        Self {
            id,
            length,
            sender: Mutex::new(None),
        }
    }

    /// Creates a new header-only [`Command`] with the specified ID.
    ///
    /// Header-only commands are always six bytes long (Thorlabs APT Protocol, Issue 39, Page 34).
    pub(crate) const fn header(id: [u8; 2]) -> Self {
        Self::payload(id, 6)
    }
}
