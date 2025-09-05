/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

/// Identifier for “generic USB units” (Thorlabs APT Protocol, Issue 39, Page 35).
///
/// Messages sent to Thorlabs devices use [`DEVICE`] as the destination byte.
/// Messages sent from Thorlabs devices use [`DEVICE`] as the source byte.
const DEVICE: u8 = 0x50;

/// Identifier for "host" (Thorlabs APT Protocol, Issue 39, Page 35).
///
/// Messages sent to Thorlabs devices use [`HOST`] as the source byte.
/// Messages sent from Thorlabs devices use [`HOST`] as the destination byte.
const HOST: u8 = 0x01;

/// Returns a six-byte header-only command, packaged according to the Thorlabs APT Protocol.
///
/// All Thorlabs commands use a fixed length six-byte message header. For simple commands, this
/// header is enough to convey the entire instruction. For more complex commands that require
/// additional data to be passed to the device, the six-byte header is followed by a
/// variable-length data packet.
pub(crate) fn short(id: [u8; 2], param_one: u8, param_two: u8) -> Vec<u8> {
    vec![id[0], id[1], param_one, param_two, DEVICE, HOST]
}

/// Returns a header-plus-payload command, packaged according to the Thorlabs APT Protocol.
///
/// All Thorlabs commands use a fixed length six-byte message header. For simple commands, this
/// header is enough to convey the entire instruction. For more complex commands that require
/// additional data to be passed to the device, the six-byte header is followed by a
/// variable-length data packet.
pub(crate) fn long(id: [u8; 2], data: &[u8]) -> Vec<u8> {
    [
        &id,
        &(data.len() as u16).to_le_bytes(),
        &[DEVICE | 0x80, HOST],
        data,
    ]
    .concat()
}
