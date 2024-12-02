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
use crate::messages::ChannelStatus::{New, Sub};
use std::sync::RwLock;
use tokio::sync::broadcast::{channel, Receiver, Sender};

include!(concat!(env!("OUT_DIR"), "/init_groups.rs"));
include!(concat!(env!("OUT_DIR"), "/length_map.rs"));
include!(concat!(env!("OUT_DIR"), "/sender_map.rs"));

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
) -> Result<&'a &'a RwLock<Option<Sender<Box<[u8]>>>>, Error> {
    Ok(SENDER_MAP.get(&id).ok_or_else(|| {
        Error::AptProtocolError(format!(
            "{:?} does not correspond to a known message ID",
            id
        ))
    })?)
}

pub(crate) enum ChannelStatus {
    Sub(Receiver<Box<[u8]>>),
    New(Receiver<Box<[u8]>>),
}

pub(crate) fn get_rx_new_or_sub(id: [u8; 2]) -> Result<ChannelStatus, Error> {
    let mut waiting_sender = get_waiting_sender(id)?.write()?;
    if let Some(tx) = waiting_sender.as_ref() {
        let rx = tx.subscribe();
        Ok(Sub(rx))
    } else {
        let (tx, rx) = channel(1);
        waiting_sender.replace(tx);
        Ok(New(rx))
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
