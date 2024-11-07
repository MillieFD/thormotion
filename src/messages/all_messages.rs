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
use crate::messages::message_map::MessageMap;
use crate::messages::message_metadata::MessageMetadata;
use std::sync::LazyLock;

pub(crate) static ALL_MESSAGES: LazyLock<MessageMap> = LazyLock::new(|| {
    get_all_messages().unwrap_or_else(|err| {
        Error::FatalError(format!("Failed to populate static ALL_MESSAGES: {}", err));
        std::process::exit(1);
    })
});

fn get_all_messages() -> Result<MessageMap, Error> {
    let mut map = MessageMap::new();
    map.insert(
        "ChanEnableState",
        Some(MessageMetadata::new(0x0210, 6)),
        MessageMetadata::new(0x0211, 6),
        MessageMetadata::new(0x0212, 6),
    )?;
    map.insert(
        "HwInfo",
        None,
        MessageMetadata::new(0x0005, 6),
        MessageMetadata::new(0x0006, 90),
    )?;
    // Add more message groups here...
    Ok(map)
}
