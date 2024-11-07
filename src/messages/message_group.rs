/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: todo
Description: todo
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::messages::message_metadata::MessageMetadata;
use std::sync::{Arc, RwLock};
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub(crate) struct MessageGroup {
    pub(crate) name: &'static str,
    pub(crate) set: Option<Arc<MessageMetadata>>,
    pub(crate) req: Arc<MessageMetadata>,
    pub(crate) get: Arc<MessageMetadata>,
    pub(crate) waiting_sender: RwLock<Option<Sender<Box<[u8]>>>>,
}

impl Clone for MessageGroup {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            set: self.set.clone(),
            req: Arc::clone(&self.req),
            get: Arc::clone(&self.get),
            waiting_sender: RwLock::new(None),
        }
    }
}

impl MessageGroup {
    pub(crate) fn new(
        name: &'static str,
        set: Option<Arc<MessageMetadata>>,
        req: Arc<MessageMetadata>,
        get: Arc<MessageMetadata>,
    ) -> Self {
        MessageGroup {
            name,
            set,
            req,
            get,
            waiting_sender: Default::default(),
        }
    }
}
