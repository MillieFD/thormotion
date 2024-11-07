/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: todo
Description: todo
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::devices::thorlabs_device_primitive::ThorlabsDevicePrimitive;
use std::ops::Deref;

pub(crate) trait ThorlabsDevice:
    Deref<Target = ThorlabsDevicePrimitive> + Send + Sync
{
}

// todo From<ThorlabsDevicePrimitive>
