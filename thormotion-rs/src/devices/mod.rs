/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

/* ----------------------------------------------------------------------------- Private Modules */

mod kdc101;
#[doc(hidden)]
mod usb_primitive;
mod utils;

/* ------------------------------------------------------------------------------ Public Exports */

pub use kdc101::KDC101;
pub use utils::{get_devices, show_devices};

/* ----------------------------------------------------------------------------- Private Exports */

pub(crate) use usb_primitive::UsbPrimitive;
pub(crate) use utils::{abort, bug_abort};
use utils::{abort_device, add_device, get_device, remove_device};
