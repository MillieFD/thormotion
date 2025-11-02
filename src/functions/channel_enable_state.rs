/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::devices::abort;
use crate::messages::utils::short;
use crate::traits::ThorlabsDevice;

const SET: [u8; 2] = [0x10, 0x02];
const REQ: [u8; 2] = [0x11, 0x02];
const GET: [u8; 2] = [0x12, 0x02];

#[doc = include_str!("../documentation/get_channel_enable_state.md")]
pub(crate) async fn get_channel_enable_state<A, const CH: usize>(device: &A, channel: usize) -> bool
where
    A: ThorlabsDevice<CH>,
{
    // Subscribe to the GET broadcast channel
    let rx = device.inner().receiver(&GET, channel).await;
    if rx.is_new() {
        // No GET response pending from the device. Send new REQ command.
        let command = short(REQ, channel as u8, 0);
        device.inner().send(command).await;
    }
    // Parse the GET response
    let response = rx.receive().await;
    match response[3] {
        0x01 => true,
        0x02 => false,
        _ => abort(format!(
            "{} GET_CHANENABLESTATE contained invalid channel enable state : {:02X?}",
            device, response[3]
        )),
    }
}

#[doc = include_str!("../documentation/set_channel_enable_state.md")]
pub(crate) async fn set_channel_enable_state<A, const CH: usize>(
    device: &A,
    channel: usize,
    enable: bool,
) where
    A: ThorlabsDevice<CH>,
{
    // Convert the boolean "enable" into a byte (Thorlabs APT Protocol)
    let enable_byte: u8 = if enable { 0x01 } else { 0x02 };
    loop {
        // Subscribe to the GET broadcast channel
        let rx = device.inner().receiver(&GET, channel).await;
        if rx.is_new() {
            // No GET response pending from the device. Send new SET & REQ commands.
            let set = short(SET, channel as u8, enable_byte);
            device.inner().send(set).await;
            let req = short(REQ, channel as u8, 0);
            device.inner().send(req).await;
        };
        // Parse the GET response
        let response = rx.receive().await;
        if response[3] == enable_byte {
            break;
        }
    }
}
