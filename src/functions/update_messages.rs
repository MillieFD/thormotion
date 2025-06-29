/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::messages::utils::short;
use crate::traits::ThorlabsDevice;

/// Starts periodic update messages from the device every 100 milliseconds (10 Hz).
///
/// Automatic updates will continue until the `stop_update_messages` function is called.
/// A 'one-off' status update can be requested using `get_status_update_async`.
pub(crate) async fn __hw_start_update_messages<A>(device: &A)
where
    A: ThorlabsDevice,
{
    const ID: [u8; 2] = [0x11, 0x00];
    let command = short(ID, 0, 0);
    device.inner().send(command).await;
}

/// Stops periodic update messages from the device every 100 milliseconds (10 Hz).
///
/// Automatic updates will cease until the `start_update_messages` function is called.
pub(crate) async fn __hw_stop_update_messages<A>(device: &A)
where
    A: ThorlabsDevice,
{
    const ID: [u8; 2] = [0x12, 0x00];
    let command = short(ID, 0, 0);
    device.inner().send(command).await;
}
