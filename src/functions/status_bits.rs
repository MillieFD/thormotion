/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::ThorlabsDevice;
use crate::messages::utils::short;

#[doc = include_str!("../documentation/get_status_bits.md")]
pub(crate) async fn get_status_bits<A, const CH: usize>(device: &A, channel: usize) -> u32
where
    A: ThorlabsDevice<CH>,
{
    const REQ: [u8; 2] = [0x29, 0x04];
    const GET: [u8; 2] = [0x2A, 0x04];
    // Subscribe to the GET broadcast channel
    let rx = device.inner().receiver(&GET, channel).await;
    if rx.is_new() {
        // No GET response pending from the device. Send new REQ command.
        let command = short(REQ, channel as u8, 0);
        device.inner().send(command).await;
    }
    // Parse the GET response
    let response = rx.receive().await;
    u32::from_le_bytes([response[8], response[9], response[10], response[11]])
}
