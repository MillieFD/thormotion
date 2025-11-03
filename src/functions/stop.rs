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
pub(crate) async fn stop<A, const CH: usize>(device: &A, channel: usize)
where
    A: ThorlabsDevice<CH>,
{
    log::info!("{device} CHANNEL {channel} STOP (requested)");
    // Subscribe to STOPPED broadcast channel
    let rx = device.inner().receiver(&STOPPED, channel).await;
    if rx.is_new() {
        // No STOPPED response pending from the device. Send STOP command.
        log::info!("{device} CHANNEL {channel} STOP (is new)");
        let command = short(STOP, channel as u8, 0x02);
        device.inner().send(command).await;
    }
    // Wait for STOPPED response
    let _ = rx.receive().await; // No need to parse response
    log::info!("{device} CHANNEL {channel} STOP (success)");
}

#[doc = include_str!("../documentation/estop.md")]
pub(crate) async fn estop<A, const CH: usize>(device: &A, channel: usize)
where
    A: ThorlabsDevice<CH>,
{
    log::info!("{device} CHANNEL {channel} ESTOP (requested)");
    // Subscribe to STOPPED broadcast channel
    let rx = device.inner().receiver(&STOPPED, channel).await;
    if rx.is_new() {
        // No STOPPED response pending from the device. Send ESTOP command.
        log::info!("{device} CHANNEL {channel} ESTOP (is new)");
        let command = short(STOP, channel as u8, 0x01);
        device.inner().send(command).await;
    }
    // Wait for STOPPED response
    let _ = rx.receive().await; // No need to parse response
    log::info!("{device} CHANNEL {channel} ESTOP (success)");
}
