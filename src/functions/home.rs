/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::short;
use crate::traits::ThorlabsDevice;

const HOME: [u8; 2] = [0x43, 0x04];
const HOMED: [u8; 2] = [0x44, 0x04];

#[doc = include_str!("../documentation/home.md")]
pub(crate) async fn home<A, const CH: usize>(device: &A, channel: usize)
where
    A: ThorlabsDevice<CH>,
{
    log::info!("{device} CHANNEL {channel} HOME (requested)");
    // Subscribe to HOMED broadcast channel
    let rx = device.inner().receiver(&HOMED, channel).await;
    if rx.is_new() {
        // No HOMED response pending from the device. Send new HOME command.
        log::info!("{device} CHANNEL {channel} HOME (is new)");
        let command = short(HOME, channel as u8, 0);
        device.inner().send(command).await;
    }
    // Wait for HOMED response
    let _ = rx.receive().await; // No need to parse response
    log::info!("{device} CHANNEL {channel} HOME (success)");
}
