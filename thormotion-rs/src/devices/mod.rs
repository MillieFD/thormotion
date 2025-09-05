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

/* --------------------------------------------------------------------------- Public Re-Exports */

pub use kdc101::KDC101;
/* -------------------------------------------------------------------------- Private
 * Re-Exports */
pub(crate) use usb_primitive::UsbPrimitive;
pub use utils::show_devices;
pub(crate) use utils::{abort, bug_abort};
use utils::{abort_device, add_device, get_device, remove_device};
