/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

mod communicator;
mod serial_port;
mod status;

use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::io;
use std::ops::Deref;

use communicator::Communicator;
use log;
use nusb::DeviceInfo;
use smol::lock::RwLock;
use status::Status;

use crate::devices::{abort, abort_device, get_device, remove_device};
use crate::error::{cmd, sn};
use crate::messages::{Dispatcher, Metadata, Provenance};

#[derive(Debug)]
pub(crate) struct UsbPrimitive<const CHANNELS: usize> {
    /// A unique eight-digit serial number that is printed on the Thorlabs device.
    serial_number: String,
    /// Information about a device that can be obtained without calling [`DeviceInfo::open`].
    device_info: DeviceInfo,
    /// The current device status.
    ///
    /// - [`Open`][1] → Contains an active [`Communicator`]
    /// - [`Closed`][2] → Contains an idle [`Dispatcher`]
    ///
    /// Open the device by calling [`open`][3].
    ///
    /// [1]: Status::Open
    /// [2]: Status::Closed
    /// [3]: UsbPrimitive::open
    status: RwLock<Status<CHANNELS>>,
}

impl<const CHANNELS: usize> UsbPrimitive<CHANNELS> {
    /// Constructs a new [`UsbPrimitive`] for a Thorlabs device with the specified serial number.
    ///
    /// Returns [`Error::NotFound`] if the specified device is not connected.
    ///
    /// Returns [`Error::Multiple`] if more than one device with the specified serial number is
    /// found.
    pub(super) fn new(
        serial_number: &String,
        ids: &[Metadata<CHANNELS>],
    ) -> Result<Self, sn::Error> {
        log::debug!("USB Primitive {serial_number} NEW (requested)");
        let device_info = get_device(serial_number)?;
        log::debug!("USB Primitive {serial_number} NEW (found)");
        let device = Self {
            serial_number: serial_number.clone(),
            device_info,
            status: RwLock::new(Status::Closed(Dispatcher::new(ids, serial_number))),
        };
        log::debug!("USB Primitive {serial_number} NEW (success)");
        Ok(device)
    }

    /// Returns the serial number of the device as a `&str`.
    pub(crate) fn serial_number(&self) -> &str {
        &self.serial_number
    }

    /// Returns `True` if the device is open.
    pub(super) async fn is_open(&self) -> bool {
        match *self.status.read().await {
            Status::Open(_) => true,
            Status::Closed(_) => false,
        }
    }

    /// Opens an [`Interface`][1] to the [`USB Device`][2].
    ///
    /// No action is taken if the device [`Status`] is already [`Open`][3].
    ///
    /// [1]: nusb::Interface
    /// [2]: UsbPrimitive
    /// [3]: Status::Open
    pub(super) async fn open(&self) -> Result<(), io::Error> {
        log::debug!("{self} OPEN (requested)");
        let mut guard = self.status.write().await;
        if let Status::Closed(dsp) = guard.deref() {
            log::debug!("{self} OPEN (is closed)");
            let interface = self.device_info.open()?.detach_and_claim_interface(0)?;
            let dispatcher = dsp.clone(); // Inexpensive Arc Clone
            let communicator = Communicator::new(interface, dispatcher).await;
            *guard = Status::Open(communicator);
        }
        log::debug!("{self} OPEN (success)");
        Ok(())
    }

    /// Releases the claimed [`Interface`][1] to the [`USB Device`][2].
    ///
    /// No action is taken if the device [`Status`] is already [`Closed`][3].
    ///
    /// [1]: nusb::Interface
    /// [2]: UsbPrimitive
    /// [3]: Status::Closed
    pub(super) async fn close(&self) -> Result<(), io::Error> {
        log::debug!("{self} CLOSE (requested)");
        let mut guard = self.status.write().await;
        if let Status::Open(communicator) = guard.deref() {
            log::debug!("{self} CLOSE (is open)");
            let dispatcher = communicator.get_dispatcher();
            *guard = Status::Closed(dispatcher);
        }
        log::debug!("{self} CLOSE (success)");
        Ok(())
    }

    /// Safely brings the [`USB Device`][1] to a resting state and releases the claimed
    /// [`Interface`][2].
    ///
    /// If the device [`Status`] is [`Closed`][3], a temporary [`Interface`][2] is [`Opened`][4]
    /// to send the abort command.
    ///
    /// Does not remove the device from the global [`DEVICES`][5] [`HashMap`][6]. You can use
    /// [`Open`][4] to resume communication.
    ///
    /// To release the claimed [`Interface`][2] without bringing the device to a resting state,
    /// use [`close`][7].
    ///
    /// [1]: UsbPrimitive
    /// [2]: nusb::Interface
    /// [3]: Status::Closed
    /// [4]: UsbPrimitive::open
    /// [5]: crate::devices::utils::DEVICES
    /// [6]: ahash::HashMap
    /// [7]: UsbPrimitive::close
    async fn abort(&self) {
        log::warn!("{self} ABORT (requested)");
        abort_device(self.serial_number());
        log::warn!("{self} ABORT (success)");
    }

    /// Returns a receiver for the given command ID, wrapped in the [`Provenance`] enum. This is
    /// useful for pattern matching.
    ///
    /// - [`New`][1] → A [`Sender`] does not exist for the given command ID. A new broadcast channel
    ///   is created.
    ///
    /// - [`Existing`][2] → The system is already waiting for a response from the Thorlabs device
    ///   for this command
    ///
    /// If pattern matching is not required, see [`any_receiver`][3] and [`new_receiver`][4] for
    /// simpler alternatives.
    ///
    /// [1]: Provenance::New
    /// [2]: Provenance::Existing
    /// [3]: Dispatcher::any_receiver
    /// [4]: Dispatcher::new_receiver
    pub(crate) async fn receiver(&self, id: &[u8], channel: usize) -> Provenance {
        log::debug!("{self} CHANNEL {channel} RECEIVER {id:02X?} (requested)");
        self.status
            .read()
            .await
            .dispatcher()
            .receiver(id, channel)
            .await
    }

    /// Returns a [`Receiver`] for the given command ID. Guarantees that the device is not
    /// currently executing the command for the given ID.
    pub(crate) async fn new_receiver(&self, id: &[u8], channel: usize) -> Provenance {
        log::debug!("{self} CHANNEL {channel} NEW_RECEIVER (requested)");
        self.status
            .read()
            .await
            .dispatcher()
            .new_receiver(id, channel)
            .await
    }

    /// Sends a command to the device.
    pub(crate) async fn send(&self, command: Vec<u8>) {
        log::debug!("{self} SEND (requested)");
        self.try_send(command)
            .await
            .unwrap_or_else(|e| abort(format!("{self} SEND (failed) {e}")));
    }

    /// Sends a command to the device.
    ///
    /// Returns an [`Error`][1] if the device is closed.
    ///
    /// [1]: cmd::Error
    pub(crate) async fn try_send(&self, command: Vec<u8>) -> Result<(), cmd::Error> {
        let guard = self.status.read().await;
        match &*guard {
            Status::Open(communicator) => {
                communicator.send(command).await;
                Ok(())
            }
            Status::Closed(_) => Err(cmd::Error::DeviceClosed),
        }
    }
}

impl<const CHANNELS: usize> PartialEq<UsbPrimitive<CHANNELS>> for UsbPrimitive<CHANNELS> {
    /// Compares two `UsbPrimitive` devices for equality.
    ///
    /// Returns `true` if both devices have the same vendor ID, product ID, and serial number.
    fn eq(&self, other: &Self) -> bool {
        self.device_info.vendor_id() == other.device_info.vendor_id()
            && self.device_info.product_id() == other.device_info.product_id()
            && self.device_info.serial_number().unwrap_or("")
                == other.device_info.serial_number().unwrap_or("")
    }
}

/// Implements the `Eq` trait for `UsbPrimitive`.
///
/// This trait is required in addition to `PartialEq` to use `UsbPrimitive` in collections
/// that require equality comparison, such as `HashSet` or as keys in `HashMap`.
impl<const CH: usize> Eq for UsbPrimitive<CH> {}

/// Implements the `Hash` trait for `UsbPrimitive`.
///
/// This allows `UsbPrimitive` to be used as a key in hash-based collections like `HashMap`.
/// The hash is computed based on the device's vendor ID, product ID, and serial number.
impl<const CH: usize> Hash for UsbPrimitive<CH> {
    /// Computes a hash value for the `UsbPrimitive` device.
    ///
    /// The hash is based on the device's vendor ID, product ID, and serial number.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.device_info.vendor_id().hash(state);
        self.device_info.product_id().hash(state);
        self.device_info.serial_number().unwrap_or("").hash(state);
    }
}

impl<const CH: usize> Display for UsbPrimitive<CH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "USB Primitive {}", self.serial_number)
    }
}

impl<const CH: usize> Drop for UsbPrimitive<CH> {
    /// Removes the `UsbPrimitive` instance from the global registry to prevent resource leaks.
    fn drop(&mut self) {
        smol::block_on(async {
            remove_device(self.serial_number());
        });
    }
}
