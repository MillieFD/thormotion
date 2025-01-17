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
use std::fmt::{Display, Formatter};
use std::ops::Deref;
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

/// # Message Format
/// The Thorlabs APT communication protocol uses a fixed length 6-byte message header, which may
/// be followed by a variable-length data packet. For simple commands, the 6-byte message header
/// is sufficient to convey the entire command. For more complex commands (e.g. commands where a
/// set of parameters needs to be passed to the device) the 6-byte header is insufficient and
/// must be followed by a data packet. The `MsgFormat` enum is used to wrap the bytes of a message
/// and indicate whether the message is `Short` (six byte header only) or `Long` (six byte header
/// plus variable length data package).

pub enum MsgFormat {
    Short([u8; 6]),
    Long(Vec<u8>),
}

impl Deref for MsgFormat {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match self {
            MsgFormat::Short(arr) => arr,
            MsgFormat::Long(vec) => vec.as_slice(),
        }
    }
}

impl Extend<u8> for MsgFormat {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        match self {
            MsgFormat::Short(arr) => {
                let mut vec = arr.to_vec();
                vec.extend(iter);
                *self = MsgFormat::Long(vec);
            }
            MsgFormat::Long(vec) => vec.extend(iter),
        }
    }
}

impl Display for MsgFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MsgFormat::Short(arr) => {
                write!(
                    f,
                    "Short Message [ {} ]",
                    arr.iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            MsgFormat::Long(vec) => {
                write!(
                    f,
                    "Long Message [ {} ]",
                    vec.iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
        }
    }
}

impl MsgFormat {
    pub(crate) fn len(&self) -> usize {
        match self {
            MsgFormat::Short(arr) => arr.len(),
            MsgFormat::Long(vec) => vec.len(),
        }
    }
}
