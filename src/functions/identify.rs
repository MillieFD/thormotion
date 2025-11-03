/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::short;
use crate::traits::ThorlabsDevice;

const IDENTIFY: [u8; 2] = [0x23, 0x02];

#[doc = include_str!("../documentation/identify.md")]
pub(crate) async fn identify<A, const CH: usize>(device: &A, channel: u8)
where
    A: ThorlabsDevice<CH>,
{
    log::info!("{device} CHANNEL {channel} IDENTIFY (requested)");
    let command = short(IDENTIFY, channel, 0);
    device.inner().send(command).await;
    log::info!("{device} CHANNEL {channel} IDENTIFY (success)");
}
