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

pub(crate) trait MessageMetadataInterface {
    // todo Change Vec<u8> to [u8].
    // This trait is unused. Have an empty `buffer: [u8; N]` member variable in MessageMetadata
    // which is a fixed size (correct size for the message) and empty. Can be copied for use.
    type Buffer;
    fn get_id(&self) -> [u8; 2];
    fn get_empty_buffer(&self) -> Self::Buffer;
}
