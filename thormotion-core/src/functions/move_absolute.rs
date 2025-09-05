/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::{long, short};
use crate::traits::{ThorlabsDevice, UnitConversion, Units};

const MOVE: [u8; 2] = [0x53, 0x04];
const MOVED: [u8; 2] = [0x64, 0x04];

/// Moves the specified device channel to an absolute position.
pub(crate) async fn __move_absolute<A>(device: &A, channel: u8, position: f64)
where
    A: ThorlabsDevice + UnitConversion,
{
    device.check_channel(channel);
    let rx = device.inner().receiver(&MOVED).await;
    if rx.is_new() {
        let mut data: Vec<u8> = Vec::with_capacity(6);
        data.extend((channel as u16).to_le_bytes());
        data.extend(Units::distance_from_f64::<A>(position));
        let command = long(MOVE, &data);
        device.inner().send(command).await;
    }
    let response = rx.receive().await;
    match response[6] == channel
        && Units::distance_from_slice(&response[8..12]).approx::<A>(position)
    {
        true => {} // No-op: Move was completed successfully
        false => Box::pin(__move_absolute(device, channel, position)).await,
    }
}

/// Moves the specified device channel to an absolute position (mm) using pre-set parameters.
pub(crate) async fn __move_absolute_from_params<A>(device: &A, channel: u8) -> f32
where
    A: ThorlabsDevice,
{
    device.check_channel(channel);
    let rx = device.inner().receiver(&MOVED).await;
    if rx.is_new() {
        let command = short(MOVE, channel, 0);
        device.inner().send(command).await;
    }
    let response = rx.receive().await;
    match response[6] == channel {
        true => f32::from_le_bytes([response[8], response[9], response[10], response[11]]),
        false => Box::pin(__move_absolute_from_params(device, channel)).await,
    }
}
