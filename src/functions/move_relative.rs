/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::{long, short};
use crate::traits::{ThorlabsDevice, UnitConversion, Units};

const MOVE_RELATIVE: [u8; 2] = [0x48, 0x04];
const MOVE_COMPLETED: [u8; 2] = [0x64, 0x04];

#[doc = include_str!("../documentation/move_relative.md")]
pub(crate) async fn move_relative<A, const CH: usize>(device: &A, channel: usize, distance: f64)
where
    A: ThorlabsDevice<CH> + UnitConversion,
{
    log::info!("{device} CHANNEL {channel} MOVE_RELATIVE (requested)");
    // Subscribe to MOVE_COMPLETED broadcast channel
    let rx = device.inner().new_receiver(&MOVE_COMPLETED, channel).await;
    {
        // No MOVE_COMPLETED response pending from the device. Send MOVE_RELATIVE command.
        log::info!("{device} CHANNEL {channel} MOVE_RELATIVE (is new)");
        let command = {
            let mut data: Vec<u8> = Vec::with_capacity(6);
            data.extend((channel as u16).to_le_bytes());
            data.extend(A::distance_from_f64(distance));
            long(MOVE_RELATIVE, &data)
        };
        device.inner().send(command).await;
    }
    // Wait for MOVE_COMPLETED response
    let response = rx.receive().await;
    log::info!("{device} CHANNEL {channel} MOVE_RELATIVE (responded)");
    // Parse the MOVE_COMPLETED response
    log::info!("{device} CHANNEL {channel} MOVE_RELATIVE (success)");
}

#[doc = include_str!("../documentation/move_relative_from_params.md")]
pub(crate) async fn move_relative_from_params<A, const CH: usize>(device: &A, channel: usize) -> f64
where
    A: ThorlabsDevice<CH> + UnitConversion,
{
    log::info!("{device} CHANNEL {channel} MOVE_RELATIVE_FROM_PARAMS (requested)");
    // Subscribe to MOVE_COMPLETED broadcast channel
    let rx = device.inner().new_receiver(&MOVE_COMPLETED, channel).await;
    {
        // No MOVE_COMPLETED response pending from the device. Send MOVE_RELATIVE command.
        log::info!("{device} CHANNEL {channel} MOVE_RELATIVE_FROM_PARAMS (is new)");
        let command = short(MOVE_RELATIVE, channel as u8, 0);
        device.inner().send(command).await;
    }
    // Wait for MOVE_COMPLETED response
    let response = rx.receive().await;
    log::info!("{device} CHANNEL {channel} MOVE_RELATIVE_FROM_PARAMS (success)");
    // Return the new position
    device.decode(Units::distance_from_slice(&response[8..12]))
}
