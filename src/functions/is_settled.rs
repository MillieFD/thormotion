/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use crate::functions;
use crate::traits::{ThorlabsDevice, UnitConversion};

#[doc = include_str!("../documentation/is_settled.md")]
pub(crate) async fn is_settled<A, const CH: usize>(device: &A, channel: usize) -> bool
where
    A: ThorlabsDevice<CH> + UnitConversion,
{
    let bits = functions::get_status_bits(device, channel).await;
    (bits & 0x00002000) != 0 // Thorlabs APT Protocol, Issue 39, Page 126
}
