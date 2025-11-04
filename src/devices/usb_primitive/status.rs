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
/// - [`Open`][1] → Contains an active [`Communicator`]
/// - [`Closed`][2] → Contains an idle [`Dispatcher`]
///
/// Open the device by calling [`open`][3]
///
/// [1]: Status::Open
/// [2]: Status::Closed
/// [3]: crate::devices::UsbPrimitive::open
#[derive(Debug)]
pub(super) enum Status<const CH: usize> {
    /// The [`Interface`][1] is `open` and communicating.
    ///
    /// This enum variant contains an active [`Communicator`].
    ///
    /// [1]: nusb::Interface
    Open(Communicator<CH>),
    /// The [`Interface`][1] is `closed`.
    ///
    /// This enum variant contains an idle [`Dispatcher`].
    ///
    /// [1]: nusb::Interface
    Closed(Dispatcher<CH>),
}

impl<const CH: usize> Status<CH> {
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
    pub(super) fn dispatcher(&self) -> Dispatcher<CH> {
        match self {
            Status::Open(communicator) => communicator.get_dispatcher(),
            Status::Closed(dispatcher) => dispatcher.clone(), // Inexpensive Arc Clone
        }
    }
}

impl<const CH: usize> Display for Status<CH> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "STATUS ({})", self.as_str())
    }
}
