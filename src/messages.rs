/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: messages
Description: This file contains the master-list of all MessageMetadata and MessageGroup instances.
These static instances are used to populate the ALL_MESSAGE_METADATA and ALL_MESSAGE_GROUPS
hashmaps at compile-time. Functions are provided to look up the metadata and group for a given ID.
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::error::Error;
use std::ops::Deref;
use tokio::sync::broadcast::{channel, Receiver, Sender};

include!(concat!(env!("OUT_DIR"), "/init_groups.rs"));
include!(concat!(env!("OUT_DIR"), "/length_map.rs"));
include!(concat!(env!("OUT_DIR"), "/sender_map.rs"));

/// # Define messages
///
/// Each message is defined as a static MessageMetadata instance with:
/// - A unique 16-bit ID (split into two u8 values in little-endian order)
/// - A fixed message length in bytes
///
/// To add a new message:
/// 1. Define a static MessageMetadata instance with the message ID and length
/// 2. Add an entry to the `ALL_MESSAGE_METADATA` hashmap
///
/// The ALL_MESSAGE_METADATA hashmap allows looking up message metadata by ID. Keys are `[u8; 2]`
/// arrays containing the low and high bytes of the message ID in little-endian order. Values are
/// references to the corresponding MessageMetadata instances.

/// # Define groups
///
/// Static MessageGroup instances are defined here to group related messages together.
/// Each group consists of:
/// - An optional SET message for setting values
/// - A REQ message for requesting values
/// - A GET message for receiving values
///
/// To add a new message group:
/// 1. Define the individual messages as MessageMetadata instances above
/// 2. Create a static MessageGroup instance combining the related messages
/// 3. Add entries to the ALL_MESSAGE_GROUPS hashmap for each message ID in the group
///
/// The ALL_MESSAGE_GROUPS hashmap allows looking up a group using any message ID in the group.
/// Keys are [u8; 2] arrays containing the message ID bytes in little-endian order.
/// Values are references to the corresponding MessageGroup instances.
/// This enables the system to handle related messages together and manage their shared state.

/// # Lookup Functions
/// This section contains functions for looking up message metadata and groups by their IDs:

pub(crate) fn get_length(id: [u8; 2]) -> Result<usize, Error> {
    Ok(LENGTH_MAP
        .get(&id)
        .ok_or_else(|| {
            Error::AptProtocolError(format!(
                "{:?} does not correspond to a known message ID",
                id
            ))
        })?
        .clone())
}

pub(crate) fn get_waiting_sender<'a>(
    id: [u8; 2],
) -> Result<&'a RwLock<Option<Sender<Box<[u8]>>>>, Error> {
    Ok(SENDER_MAP
        .get(&id)
        .ok_or_else(|| {
            Error::AptProtocolError(format!(
                "{:?} does not correspond to a known message ID",
                id
            ))
        })?
        .deref())
}

pub(crate) enum ChannelStatus {
    Sub(Receiver<Box<[u8]>>),
    New(Receiver<Box<[u8]>>),
}

pub(crate) fn get_rx_new_or_sub(id: [u8; 2]) -> Result<ChannelStatus, Error> {
    let mut waiting_sender = get_waiting_sender(id)?.write()?;
    if let Some(tx) = waiting_sender.as_ref() {
        let rx = tx.subscribe();
        Ok(ChannelStatus::Sub(rx))
    } else {
        let (tx, rx) = channel(1);
        waiting_sender.replace(tx);
        Ok(ChannelStatus::New(rx))
    }
}

pub(crate) fn get_rx_new_or_err(id: [u8; 2]) -> Result<Receiver<Box<[u8]>>, Error> {
    let mut waiting_sender = get_waiting_sender(id)?.write()?;
    if let Some(_) = waiting_sender.as_ref() {
        Err(Error::AptProtocolError(format!(
            "A waiting sender already exists for message ID {:?}",
            id
        )))
    } else {
        let (tx, rx) = channel(1);
        waiting_sender.replace(tx);
        Ok(rx)
    }
}
