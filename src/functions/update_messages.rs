/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::short;
use crate::traits::ThorlabsDevice;

const START_UPDATE_MESSAGES: [u8; 2] = [0x11, 0x00];
const STOP_UPDATE_MESSAGES: [u8; 2] = [0x12, 0x00];

#[doc = include_str!("../documentation/start_update_messages.md")]
pub(crate) async fn start_update_messages<A, const CH: usize>(device: &A)
where
    A: ThorlabsDevice<CH>,
{
    log::info!("{device} START_UPDATE_MESSAGES (requested)");
    let command = short(START_UPDATE_MESSAGES, 0, 0);
    device.inner().send(command).await;
    log::info!("{device} START_UPDATE_MESSAGES (success)");
}

#[doc = include_str!("../documentation/stop_update_messages.md")]
pub(crate) async fn stop_update_messages<A, const CH: usize>(device: &A)
where
    A: ThorlabsDevice<CH>,
{
    log::info!("{device} STOP_UPDATE_MESSAGES (requested)");
    let command = short(STOP_UPDATE_MESSAGES, 0, 0);
    device.inner().send(command).await;
    log::info!("{device} STOP_UPDATE_MESSAGES (success)");
}
