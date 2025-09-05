/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::short;
use crate::traits::ThorlabsDevice;

/// Identifies the device by flashing the front panel LED.
pub(crate) async fn __identify<A>(device: &A, channel: u8)
where
    A: ThorlabsDevice,
{
    const ID: [u8; 2] = [0x23, 0x02];
    let command = short(ID, channel, 0);
    device.inner().send(command).await
}
