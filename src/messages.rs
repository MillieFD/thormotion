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
use phf::phf_map;
use std::ops::Deref;
use std::sync::RwLock;
use tokio::sync::broadcast::{channel, Receiver, Sender};

///

#[derive(Debug, Clone)]
pub(crate) struct MessageMetadata {
    pub(crate) id: [u8; 2],
    pub(crate) length: usize,
}

impl MessageMetadata {
    pub(crate) const fn new(id: u16, length: usize) -> Self {
        Self {
            id: id.to_le_bytes(),
            length,
        }
    }
}

///

#[derive(Debug)]
pub(crate) struct MessageGroup {
    pub(crate) set: Option<&'static MessageMetadata>,
    pub(crate) req: &'static MessageMetadata,
    pub(crate) get: &'static MessageMetadata,
    pub(crate) waiting_sender: RwLock<Option<Sender<Box<[u8]>>>>,
}

impl MessageGroup {
    pub(crate) const fn new(
        set: Option<&'static MessageMetadata>,
        req: &'static MessageMetadata,
        get: &'static MessageMetadata,
    ) -> Self {
        MessageGroup {
            set,
            req,
            get,
            waiting_sender: RwLock::new(None),
        }
    }
}

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

static SET_CHANENABLESTATE: MessageMetadata = MessageMetadata::new(0x0210, 6);
static REQ_CHANENABLESTATE: MessageMetadata = MessageMetadata::new(0x0211, 6);
static GET_CHANENABLESTATE: MessageMetadata = MessageMetadata::new(0x0212, 6);
static HW_START_UPDATEMSGS: MessageMetadata = MessageMetadata::new(0x0011, 6);
static HW_STOP_UPDATEMSGS: MessageMetadata = MessageMetadata::new(0x0012, 6);
static HW_REQ_INFO: MessageMetadata = MessageMetadata::new(0x0005, 6);
static HW_GET_INFO: MessageMetadata = MessageMetadata::new(0x0006, 6);

static ALL_MESSAGE_METADATA: phf::Map<[u8; 2], &MessageMetadata> = phf_map! {
    [0x10, 0x02] => &SET_CHANENABLESTATE,
    [0x11, 0x02] => &REQ_CHANENABLESTATE,
    [0x12, 0x02] => &GET_CHANENABLESTATE,
    [0x00, 0x11] => &HW_START_UPDATEMSGS,
    [0x00, 0x12] => &HW_STOP_UPDATEMSGS,
    [0x00, 0x05] => &HW_REQ_INFO,
    [0x00, 0x06] => &HW_GET_INFO,
};

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

static CHANENABLESTATE: MessageGroup = MessageGroup::new(
    Some(&SET_CHANENABLESTATE),
    &REQ_CHANENABLESTATE,
    &GET_CHANENABLESTATE,
);
static HW_INFO: MessageGroup = MessageGroup::new(None, &HW_REQ_INFO, &HW_GET_INFO);

static ALL_MESSAGE_GROUPS: phf::Map<[u8; 2], &MessageGroup> = phf_map! {
    [0x10, 0x02] => &CHANENABLESTATE,
    [0x11, 0x02] => &CHANENABLESTATE,
    [0x12, 0x02] => &CHANENABLESTATE,
    [0x00, 0x11] => &HW_INFO,
    [0x00, 0x12] => &HW_INFO,
};

/// # Lookup Functions
/// This section contains functions for looking up message metadata and groups by their IDs:

pub(crate) fn get_metadata_by_id(id: [u8; 2]) -> Result<&'static MessageMetadata, Error> {
    Ok(ALL_MESSAGE_METADATA
        .get(&id)
        .ok_or_else(|| {
            Error::AptProtocolError(format!(
                "{:?} does not correspond to a known message ID",
                id
            ))
        })?
        .deref())
}

pub(crate) fn get_group_by_id(id: [u8; 2]) -> Result<&'static MessageGroup, Error> {
    Ok(ALL_MESSAGE_GROUPS
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
    let mut waiting_sender = get_group_by_id(id)?.waiting_sender.write()?;
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
    let mut waiting_sender = get_group_by_id(id)?.waiting_sender.write()?;
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
