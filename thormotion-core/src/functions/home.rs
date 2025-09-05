/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::short;
use crate::traits::ThorlabsDevice;

/// Homes the specified device channel.
pub(crate) async fn __home<A>(device: &A, channel: u8)
where
    A: ThorlabsDevice,
{
    const HOME: [u8; 2] = [0x43, 0x04];
    const HOMED: [u8; 2] = [0x44, 0x04];

    device.check_channel(channel);
    let rx = device.inner().receiver(&HOMED).await;
    if rx.is_new() {
        let command = short(HOME, channel, 0);
        device.inner().send(command).await;
    }
    let _ = rx.receive().await; // No need to parse the response
}
