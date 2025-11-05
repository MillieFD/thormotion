/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::ThorlabsDevice;
use crate::messages::utils::short;

const REQ_STATUS_BITS: [u8; 2] = [0x29, 0x04];
const GET_STATUS_BITS: [u8; 2] = [0x2A, 0x04];

#[doc = include_str!("../documentation/get_status_bits.md")]
pub(crate) async fn get_status_bits<A, const CH: usize>(device: &A, channel: usize) -> u32
where
    A: ThorlabsDevice<CH>,
{
    log::info!("{device} CHANNEL {channel} GET_STATUS_BITS (requested)");
    // Subscribe to GET_STATUS_BITS broadcast channel
    let rx = device.inner().receiver(&GET_STATUS_BITS, channel).await;
    if rx.is_new() {
        // No GET_STATUS_BITS response pending from the device. Send REQ_STATUS_BITS command.
        log::info!("{device} CHANNEL {channel} GET_STATUS_BITS (is new)");
        let command = short(REQ_STATUS_BITS, channel as u8, 0);
        device.inner().send(command).await;
    }
    // Wait for GET_STATUS_BITS response
    let response = rx.receive().await;
    log::info!("{device} CHANNEL {channel} GET_STATUS_BITS (success)");
    // Return little-endian status bits as u32
    u32::from_le_bytes([response[8], response[9], response[10], response[11]])
}
