/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::short;
use crate::traits::{ThorlabsDevice, UnitConversion, Units};

/// Returns the current position (mm) and velocity (mm/s) for the specified device channel.
// TODO: Fn should also return status bits and motor current
pub(crate) async fn __get_u_status_update<A>(device: &A, channel: u8) -> (f64, f64)
where
    A: ThorlabsDevice + UnitConversion,
{
    const REQ_USTATUSUPDATE: [u8; 2] = [0x90, 0x04];
    const GET_USTATUSUPDATE: [u8; 2] = [0x91, 0x04];
    device.check_channel(channel);
    let response = loop {
        let rx = device.inner().receiver(&GET_USTATUSUPDATE).await;
        if rx.is_new() {
            let command = short(REQ_USTATUSUPDATE, channel, 0);
            device.inner().send(command).await;
        }
        let response = rx.receive().await;
        if response[6] == channel {
            break response;
        }
    };
    let position = Units::distance_from_slice(&response[8..12]).decode::<A>();
    let velocity = Units::velocity_from_slice(&response[12..14]).decode::<A>();
    (position, velocity)
}
