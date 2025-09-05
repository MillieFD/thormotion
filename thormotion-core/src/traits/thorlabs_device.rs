/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};

use crate::devices::{UsbPrimitive, abort};

pub trait ThorlabsDevice: Display + Debug + Send + Sync {
    /// Returns a borrow which dereferences to the inner [`UsbPrimitive`]
    fn inner(&self) -> &UsbPrimitive;

    /// Returns the serial number of the device as a `&str`.
    fn serial_number(&self) -> &str {
        self.inner().serial_number()
    }

    /// Returns the number of device channels.
    fn channels(&self) -> u8;

    /// No action if the specified channel is valid.
    ///
    /// Aborts if the specified channel is outside the valid range.
    fn check_channel(&self, channel: u8) {
        if channel <= 0 || channel > self.channels() {
            abort(format!(
                "Channel number {} is not valid for {}",
                channel, self
            ));
        }
    }

    /// Safely brings the [`USB Device`][1] to a resting state and releases the claimed
    /// [`Interface`][2].
    ///
    /// If the device [`Status`][3] is [`Closed`][4], a temporary [`Interface`][2] is [`Opened`][5]
    /// to send the abort command.
    ///
    /// Does not remove the device from the global [`DEVICES`][6] [`HashMap`][7]. You can use
    /// [`Open`][5] to resume communication.
    ///
    /// To release the claimed [`Interface`][2] without bringing the device to a resting state,
    /// use `close`.
    ///
    /// [1]: UsbPrimitive
    /// [2]: nusb::Interface
    /// [3]: crate::devices::usb_primitive::status::Status
    /// [4]: crate::devices::usb_primitive::status::Status::Closed
    /// [5]: UsbPrimitive::open
    /// [6]: crate::devices::utils::DEVICES
    /// [7]: ahash::HashMap
    fn abort(&self);
}

impl Hash for dyn ThorlabsDevice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().serial_number().hash(state);
    }
}

impl PartialEq for dyn ThorlabsDevice {
    fn eq(&self, other: &Self) -> bool {
        self.inner().serial_number() == other.inner().serial_number()
    }
}

impl Eq for dyn ThorlabsDevice {}
