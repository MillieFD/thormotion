/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: todo
Description: This file todo
---------------------------------------------------------------------------------------------------
Notes:
*/

use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct MessageMetadata {
    pub(crate) id: [u8; 2],
    pub(crate) length: usize,
}

impl MessageMetadata {
    pub(crate) fn new(id: u16, length: usize) -> Arc<Self> {
        Arc::new(Self {
            id: id.to_le_bytes(),
            length,
        })
    }
}
