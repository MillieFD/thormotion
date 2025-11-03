/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::short;
use crate::traits::{ThorlabsDevice, UnitConversion, Units};

const REQ_U_STATUS_UPDATE: [u8; 2] = [0x90, 0x04];
const GET_U_STATUS_UPDATE: [u8; 2] = [0x91, 0x04];

#[doc = include_str!("../documentation/get_status.md")]
pub(crate) async fn get_u_status_update<A, const CH: usize>(
    device: &A,
    channel: usize,
) -> (f64, f64, u32)
where
    A: ThorlabsDevice<CH> + UnitConversion,
{
    log::info!("{device} CHANNEL {channel} U_STATUS_UPDATE (requested)");
    // Subscribe to GET_U_STATUS_UPDATE broadcast channel
    let rx = device.inner().receiver(&GET_U_STATUS_UPDATE, channel).await;
    if rx.is_new() {
        // No GET_U_STATUS_UPDATE response pending from the device. Send REQ_U_STATUS_UPDATE.
        log::info!("{device} CHANNEL {channel} U_STATUS_UPDATE (is new)");
        let command = short(REQ_U_STATUS_UPDATE, channel as u8, 0);
        device.inner().send(command).await;
    }
    // Wait for GET_U_STATUS_UPDATE response
    let response = rx.receive().await;
    log::info!("{device} CHANNEL {channel} U_STATUS_UPDATE (responded)");
    // Parse the GET_U_STATUS_UPDATE response
    let position = device.decode(Units::distance_from_slice(&response[8..12]));
    let velocity = device.decode(Units::velocity_from_slice(&response[12..14]));
    let bits = u32::from_le_bytes([response[16], response[17], response[18], response[19]]);
    log::info!("{device} CHANNEL {channel} U_STATUS_UPDATE (success)");
    (position, velocity, bits)
}
