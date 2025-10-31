/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::fmt;
use std::fmt::Display;

use super::communicator::Communicator;
use crate::messages::Dispatcher;

/// The current device status.
///
/// - [`Open`][`Status::Open`] → Contains an active [`Communicator`]
/// - [`Closed`][`Status::Closed`] → Contains an idle [`Dispatcher`]
///
/// Open the device by calling [`open`][`UsbPrimitive::open`]
pub(super) enum Status {
    /// The [`Interface`][nusb::Interface] is [`open`][`nusb::DeviceInfo::open`] and communicating.
    ///
    /// This enum variant contains an active [`Communicator`].
    Open(Communicator),
    /// The [`Interface`][nusb::Interface] is [`closed`][`nusb::DeviceInfo::close`].
    ///
    /// This enum variant contains an idle [`Dispatcher`].
    Closed(Dispatcher),
}

impl Status {
    /// Returns a string representation of the current status.
    ///
    /// Returns "Open" if the device is open, or "Closed" if the device is closed.
    pub(super) fn as_str(&self) -> &str {
        match self {
            Self::Open(_) => "Open",
            Self::Closed(_) => "Closed",
        }
    }

    /// Returns the [`Dispatcher`] wrapped in an [`Arc`][std::sync::Arc].
    pub(super) fn dispatcher(&self) -> Dispatcher {
        match self {
            Status::Open(communicator) => communicator.get_dispatcher(),
            Status::Closed(dispatcher) => dispatcher.clone(), // Inexpensive Arc Clone
        }
    }
}

impl Display for Status {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}
