/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::{long, short};
use crate::traits::{ThorlabsDevice, UnitConversion, Units};

const MOVE_ABSOLUTE: [u8; 2] = [0x53, 0x04];
const MOVE_COMPLETED: [u8; 2] = [0x64, 0x04];

#[doc = include_str!("../documentation/move_absolute.md")]
pub(crate) async fn move_absolute<A, const CH: usize>(device: &A, channel: usize, position: f64)
where
    A: ThorlabsDevice<CH> + UnitConversion,
{
    log::info!("{device} CHANNEL {channel} MOVE_ABSOLUTE {position} (requested)");
    loop {
        // Subscribe to MOVE_COMPLETED broadcast channel
        let rx = device.inner().receiver(&MOVE_COMPLETED, channel).await;
        if rx.is_new() {
            // No MOVE_COMPLETED response pending from the device. Send MOVE_ABSOLUTE command.
            log::info!("{device} CHANNEL {channel} MOVE_ABSOLUTE {position} (is new)");
            let command = {
                let mut data: Vec<u8> = Vec::with_capacity(6);
                data.extend((channel as u16).to_le_bytes());
                data.extend(A::distance_from_f64(position));
                long(MOVE_ABSOLUTE, &data)
            };
            device.inner().send(command).await;
        }
        // Wait for MOVE_COMPLETED response
        let response = rx.receive().await;
        log::info!("{device} CHANNEL {channel} MOVE_ABSOLUTE {position} (responded)");
        // Parse the MOVE_COMPLETED response
        let p = device.decode(Units::distance_from_slice(&response[8..12]));
        if Units::approx(p, position) {
            log::info!("{device} CHANNEL {channel} MOVE_ABSOLUTE {position} (success)");
            return;
        }
    }
}

#[doc = include_str!("../documentation/move_absolute_from_params.md")]
pub(crate) async fn move_absolute_from_params<A, const CH: usize>(device: &A, channel: usize) -> f64
where
    A: ThorlabsDevice<CH> + UnitConversion,
{
    log::info!("{device} CHANNEL {channel} MOVE_ABSOLUTE_FROM_PARAMS (requested)");
    // Subscribe to MOVE_COMPLETED broadcast channel
    let rx = device.inner().new_receiver(&MOVE_COMPLETED, channel).await;
    {
        // No MOVE_COMPLETED response pending from the device. Send MOVE_ABSOLUTE command.
        log::info!("{device} CHANNEL {channel} MOVE_ABSOLUTE_FROM_PARAMS (is new)");
        let command = short(MOVE_ABSOLUTE, channel as u8, 0);
        device.inner().send(command).await;
    }
    // Wait for MOVE_COMPLETED response
    let response = rx.receive().await;
    log::info!("{device} CHANNEL {channel} MOVE_ABSOLUTE_FROM_PARAMS (responded)");
    // Return the new position
    device.decode(Units::distance_from_slice(&response[8..12]))
}
