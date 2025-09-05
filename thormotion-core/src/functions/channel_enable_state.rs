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

/// Returns `True` if the specified device channel is enabled.
pub(crate) async fn __req_channel_enable_state<A>(device: &A, channel: u8) -> bool
where
    A: ThorlabsDevice,
{
    const REQ: [u8; 2] = [0x11, 0x02];
    const GET: [u8; 2] = [0x12, 0x02];

    device.check_channel(channel);
    let rx = device.inner().receiver(&GET).await;
    if rx.is_new() {
        let command = short(REQ, channel, 0);
        device.inner().send(command).await;
    }
    let response = rx.receive().await;
    if channel == response[2] {
        match response[3] {
            0x01 => true,
            0x02 => false,
            _ => abort(format!(
                "{} GET_CHANENABLESTATE contained invalid channel enable state : {}",
                device, response[3]
            )),
        }
    } else {
        Box::pin(async { __req_channel_enable_state(device, channel).await }).await
    }
}

/// Enables or disables the specified device channel.
pub(crate) async fn __set_channel_enable_state<A>(device: &A, channel: u8, enable: bool)
where
    A: ThorlabsDevice,
{
    const SET: [u8; 2] = [0x10, 0x02];
    const REQ: [u8; 2] = [0x11, 0x02];
    const GET: [u8; 2] = [0x12, 0x02];

    device.check_channel(channel);
    let enable_byte: u8 = if enable { 0x01 } else { 0x02 };
    let rx = device.inner().receiver(&GET).await;
    if rx.is_new() {
        let set = short(SET, channel, enable_byte);
        device.inner().send(set).await;
        let req = short(REQ, channel, 0);
        device.inner().send(req).await;
    }
    let response = rx.receive().await;
    match response[2] == channel && response[3] == enable_byte {
        true => {} // No-op: Enable state was set successfully
        false => {
            Box::pin(__set_channel_enable_state(device, channel, enable)).await;
        }
    }
}
