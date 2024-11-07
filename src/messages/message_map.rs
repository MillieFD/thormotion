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

use crate::errors::error_types::Error;
use crate::messages::message_group::MessageGroup;
use crate::messages::message_metadata::MessageMetadata;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) struct MessageMap {
    id_to_metadata: HashMap<[u8; 2], Arc<MessageMetadata>>,
    message_groups: Vec<Arc<MessageGroup>>,
    name_to_group: HashMap<&'static str, Arc<MessageGroup>>,
    id_to_group: HashMap<[u8; 2], Arc<MessageGroup>>,
}
impl MessageMap {
    pub(crate) fn new() -> Self {
        MessageMap {
            id_to_metadata: HashMap::new(),
            message_groups: Vec::new(),
            name_to_group: HashMap::new(),
            id_to_group: HashMap::new(),
        }
    }

    pub(crate) fn insert(
        &mut self,
        name: &'static str,
        set: Option<Arc<MessageMetadata>>,
        req: Arc<MessageMetadata>,
        get: Arc<MessageMetadata>,
    ) -> Result<(), Error> {
        if self.name_to_group.contains_key(name) {
            return Err(Error::MessageGroupNameAlreadyExists(name.to_string()));
        }
        let group = Arc::new(MessageGroup::new(
            name,
            set,
            Arc::clone(&req),
            Arc::clone(&get),
        ));
        self.name_to_group.insert(name, Arc::clone(&group));
        self.message_groups.push(Arc::clone(&group));

        if let Some(set) = &group.set {
            if !self.id_to_metadata.contains_key(&set.id) {
                self.id_to_metadata.insert(set.id, Arc::clone(set));
                self.id_to_group.insert(set.id, Arc::clone(&group));
            }
        }

        self.id_to_metadata.insert(req.id, Arc::clone(&req));
        self.id_to_group.insert(req.id, Arc::clone(&group));

        self.id_to_metadata.insert(get.id, Arc::clone(&get));
        self.id_to_group.insert(get.id, Arc::clone(&group));

        Ok(())
    }

    pub(crate) fn get_group_by_name(&self, name: &str) -> Result<Arc<MessageGroup>, Error> {
        if let Some(group) = self.name_to_group.get(name) {
            return Ok(Arc::clone(group));
        }
        Err(Error::MessageGroupNameNotFound(name.to_string()))
    }

    pub(crate) fn get_group_by_id(&self, id: u16) -> Result<Arc<MessageGroup>, Error> {
        let id_le_bytes: [u8; 2] = id.to_le_bytes();
        if let Some(group) = self.id_to_group.get(&id_le_bytes) {
            return Ok(Arc::clone(group));
        }
        Err(Error::MessageIdNotFound(id))
    }

    pub(crate) fn get_metadata_by_id(&self, id: [u8; 2]) -> Result<Arc<MessageMetadata>, Error> {
        if let Some(metadata) = self.id_to_metadata.get(&id) {
            return Ok(Arc::clone(metadata));
        }
        Err(Error::MessageIdNotFound(u16::from_le_bytes(id)))
    }
}
