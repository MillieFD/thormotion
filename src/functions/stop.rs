/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::short;
use crate::traits::ThorlabsDevice;

const STOP: [u8; 2] = [0x65, 0x04];
const STOPPED: [u8; 2] = [0x66, 0x04];

#[doc = include_str!("../documentation/stop.md")]
pub(crate) async fn stop<A>(device: &A, channel: u8)
where
    A: ThorlabsDevice,
{
    device.check_channel(channel);
    let rx = device.inner().receiver(&STOPPED).await;
    if rx.is_new() {
        let command = short(STOP, channel, 0x02);
        device.inner().send(command).await;
    }
    let _ = rx.receive().await;
}

#[doc = include_str!("../documentation/estop.md")]
pub(crate) async fn estop<A>(device: &A, channel: u8)
where
    A: ThorlabsDevice,
{
    device.check_channel(channel);
    let rx = device.inner().receiver(&STOPPED).await;
    if rx.is_new() {
        let command = short(STOP, channel, 0x01);
        device.inner().send(command).await;
    }
    let _ = rx.receive().await;
}
