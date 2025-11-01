/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::short;
use crate::traits::ThorlabsDevice;

#[doc = include_str!("../documentation/hw_start_update_messages.md")]
pub(crate) async fn hw_start_update_messages<A>(device: &A)
where
    A: ThorlabsDevice,
{
    const ID: [u8; 2] = [0x11, 0x00];
    let command = short(ID, 0, 0);
    device.inner().send(command).await;
}

#[doc = include_str!("../documentation/hw_stop_update_messages.md")]
pub(crate) async fn hw_stop_update_messages<A>(device: &A)
where
    A: ThorlabsDevice,
{
    const ID: [u8; 2] = [0x12, 0x00];
    let command = short(ID, 0, 0);
    device.inner().send(command).await;
}
