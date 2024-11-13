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
use std::time::Duration;

pub(crate) const MAX_ATTEMPTS: u32 = 5; // todo add a way for users to change this from the default if necessary
pub(crate) const BUFFER_SIZE: usize = 256; // todo add a way for users to change this from the default if necessary
pub(crate) const TIMEOUT: Duration = Duration::from_millis(500); // todo add a way for users to change this from the default if necessary
pub(crate) const OUT_ENDPOINT: u8 = 0x02;
pub(crate) const IN_ENDPOINT: u8 = 0x81;
pub(crate) const READ_INTERVAL: Duration = Duration::from_millis(2000); // todo add a way for users to change this from the default if necessary
pub(crate) const VENDOR_ID: u16 = 0x0403;
pub(crate) const DEST: u8 = 0x50;
pub(crate) const SOURCE: u8 = 0x01;