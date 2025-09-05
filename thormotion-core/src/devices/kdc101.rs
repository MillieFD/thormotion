/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::fmt::{Display, Formatter};
use std::io::Error;
use std::sync::Arc;

use smol::block_on;

use crate::devices::{UsbPrimitive, add_device};
use crate::error::sn;
use crate::functions::*;
use crate::messages::Command;
use crate::traits::{CheckSerialNumber, ThorlabsDevice, UnitConversion};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KDC101 {
    inner: Arc<UsbPrimitive>,
}

impl KDC101 {
    const IDS: [Command; 4] = [
        // MOD
        Command::header([0x12, 0x02]), // GET_CHANENABLESTATE
        // MOT
        Command::header([0x44, 0x04]),      // MOVE_HOMED
        Command::payload([0x64, 0x04], 20), // MOVE_COMPLETED
        Command::payload([0x91, 0x04], 20), // GET_USTATUSUPDATE
    ];

    pub fn new<A>(serial_number: A) -> Result<Self, sn::Error>
    where
        A: Into<String>,
    {
        let sn = serial_number.into();
        Self::check_serial_number(&sn)?;
        let device = Self {
            inner: Arc::new(UsbPrimitive::new(&sn, &Self::IDS)?),
        };
        let d = device.clone(); // Inexpensive Arc Clone
        let f = move || d.abort();
        add_device(sn, f);
        Ok(device)
    }

    /// Returns `True` if the [`USB Interface`][1] is open.
    ///
    /// [1]: nusb::Interface
    pub async fn is_open(&self) -> bool {
        self.inner.is_open().await
    }

    /// Opens an [`Interface`][1] to the [`USB Device`][2].
    ///
    /// No action is taken if the device [`Status`][3] is already [`Open`][4].
    ///
    /// For a synchronous alternative, see [`open`][5].
    ///
    /// [1]: nusb::Interface
    /// [2]: UsbPrimitive
    /// [3]: crate::devices::usb_primitive::status::Status
    /// [4]: crate::devices::usb_primitive::status::Status::Open
    /// [5]: KDC101::open
    pub async fn open_async(&mut self) -> Result<(), Error> {
        self.inner.open().await
    }

    /// Opens an [`Interface`][1] to the [`USB Device`][2].
    ///
    /// No action is taken if the device [`Status`][3] is already [`Open`][4].
    ///
    /// For an asynchronous alternative, see [`async_open`][5].
    ///
    /// [1]: nusb::Interface
    /// [2]: UsbPrimitive
    /// [3]: crate::devices::usb_primitive::status::Status
    /// [4]: crate::devices::usb_primitive::status::Status::Open
    /// [5]: KDC101::open_async
    pub fn open(&mut self) -> Result<(), Error> {
        block_on(async { self.open_async().await })
    }

    /// Releases the claimed [`Interface`][1] to the [`USB Device`][2].
    ///
    /// No action is taken if the device [`Status`][3] is already [`Closed`][4].
    ///
    /// This does not stop the device's current action. If you need to safely bring the
    /// [`USB Device`][2] to a resting state, see [`abort`][5].
    ///
    /// For a synchronous alternative, see [`close`][6].
    ///
    /// [1]: nusb::Interface
    /// [2]: UsbPrimitive
    /// [3]: crate::devices::usb_primitive::status::Status
    /// [4]: crate::devices::usb_primitive::status::Status::Closed
    /// [5]: ThorlabsDevice::abort
    /// [6]: KDC101::close
    pub async fn close_async(&mut self) -> Result<(), Error> {
        self.inner.close().await
    }

    /// Releases the claimed [`Interface`][1] to the [`USB Device`][2].
    ///
    /// No action is taken if the device [`Status`][3] is already [`Closed`][4].
    ///
    /// This does not stop the device's current action. If you need to safely bring the
    /// [`USB Device`][2] to a resting state, see [`abort`][5].
    ///
    /// For an asynchronous alternative, see [`async_close`][6].
    ///
    /// [1]: nusb::Interface
    /// [2]: UsbPrimitive
    /// [3]: crate::devices::usb_primitive::status::Status
    /// [4]: crate::devices::usb_primitive::status::Status::Closed
    /// [5]: ThorlabsDevice::abort
    /// [6]: KDC101::close_async
    pub fn close(&mut self) -> Result<(), Error> {
        block_on(async { self.close_async().await })
    }

    /// Identifies the device by flashing the front panel LED.
    ///
    /// For a synchronous alternative, see [`identify`][1].
    ///
    /// [1]: KDC101::identify
    pub async fn identify_async(&self) {
        __identify(self, 1).await;
    }

    /// Identifies the device by flashing the front panel LED.
    ///
    /// For an asynchronous alternative, see [`async_identify`][1].
    ///
    /// [1]: KDC101::identify_async
    pub fn identify(&self) {
        block_on(async { self.identify_async().await })
    }

    /// Returns the current position (mm) and velocity (mm/s) for the specified device channel.
    ///
    /// For a synchronous alternative, see [`get_status`][1].
    ///
    /// [1]: KDC101::get_status
    pub async fn get_status_async(&self) -> (f64, f64) {
        __get_u_status_update(self, 1).await
    }

    /// Returns the current position (mm) and velocity (mm/s) for the specified device channel.
    ///
    /// For an asynchronous alternative, see [`async_get_status`][1].
    ///
    /// [1]: KDC101::get_status_async
    pub fn get_status(&self) -> (f64, f64) {
        block_on(async { self.get_status_async().await })
    }

    /// Starts periodic update messages from the device every 100 milliseconds (10 Hz).
    ///
    /// Automatic updates will continue until the [`hw_stop_update_messages`][1] function is called.
    /// A 'one-off' status update can be requested using [`get_status`].
    ///
    /// For a synchronous alternative, see [`hw_start_update_messages`][3].
    ///
    /// [1]: KDC101::hw_stop_update_messages
    /// [2]: KDC101::get_status
    /// [3]: KDC101::hw_start_update_messages
    pub async fn hw_start_update_messages_async(&self) {
        __hw_start_update_messages(self).await;
    }

    /// Starts periodic update messages from the device every 100 milliseconds (10 Hz).
    ///
    /// Automatic updates will continue until the [`hw_stop_update_messages`][1] function is called.
    /// A 'one-off' status update can be requested using [`get_status`].
    ///
    /// For an asynchronous alternative, see [`hw_start_update_messages_async`][3].
    ///
    /// [1]: KDC101::hw_stop_update_messages
    /// [2]: KDC101::get_status
    /// [3]: KDC101::hw_start_update_messages_async
    pub fn hw_start_update_messages(&self) {
        block_on(async { self.hw_start_update_messages_async().await })
    }

    /// Stops periodic update messages from the device every 100 milliseconds (10 Hz).
    ///
    /// Automatic updates will cease until the [`hw_start_update_messages`][1] function is called.
    ///
    /// For a synchronous alternative, see [`hw_stop_update_messages`][2].
    ///
    /// [1]: KDC101::hw_start_update_messages
    /// [2]: KDC101::hw_stop_update_messages
    pub async fn hw_stop_update_messages_async(&self) {
        __hw_stop_update_messages(self).await;
    }

    /// Stops periodic update messages from the device every 100 milliseconds (10 Hz).
    ///
    /// Automatic updates will cease until the [`hw_start_update_messages`][1] function is called.
    ///
    /// For an asynchronous alternative, see [`hw_stop_update_messages_async`][2].
    ///
    /// [1]: KDC101::hw_start_update_messages
    /// [2]: KDC101::hw_stop_update_messages_async
    pub fn hw_stop_update_messages(&self) {
        block_on(async { self.hw_stop_update_messages_async().await })
    }

    /// Returns `True` if the specified device channel is enabled.
    pub async fn get_channel_enable_state_async(&self) {
        __req_channel_enable_state(self, 1).await;
    }

    /// Returns `True` if the specified device channel is enabled.
    pub async fn get_channel_enable_state(&self) {
        block_on(async { self.get_channel_enable_state_async().await })
    }

    /// Enables or disables the specified device channel.
    pub async fn set_channel_enable_state_async(&self, enable: bool) {
        __set_channel_enable_state(self, 1, enable).await;
    }

    /// Enables or disables the specified device channel.
    pub async fn set_channel_enable_state(&self, enable: bool) {
        block_on(async { self.set_channel_enable_state_async(enable).await })
    }

    /// Homes the specified device channel.
    ///
    /// For a synchronous alternative, see [`home`][1]
    ///
    /// [1]: KDC101::home
    pub async fn home_async(&self) {
        __home(self, 1).await;
    }

    /// Homes the specified device channel.
    ///
    /// For an asynchronous alternative, see [`async_home`][1]
    ///
    /// [1]: KDC101::home_async
    pub fn home(&self) {
        block_on(async { self.home_async().await })
    }

    /// Moves the specified device channel to an absolute position.
    ///
    /// For a synchronous alternative, see [`move_absolute`][1]
    ///
    /// [1]: KDC101::move_absolute
    pub async fn move_absolute_async(&self, position: f64) {
        __move_absolute(self, 1, position).await;
    }

    /// Moves the specified device channel to an absolute position.
    ///
    /// For an asynchronous alternative, see [`async_move_absolute`][1]
    ///
    /// [1]: KDC101::move_absolute_async
    pub fn move_absolute(&self, position: f64) {
        block_on(async { self.move_absolute_async(position).await })
    }

    /// Moves the specified device channel to an absolute position (mm) using pre-set parameters.
    ///
    /// For a synchronous alternative, see [`move_absolute_from_params`][1]
    ///
    /// [1]: KDC101::move_absolute_from_params
    pub async fn move_absolute_from_params_async(&self) {
        __move_absolute_from_params(self, 1).await;
    }

    /// Moves the specified device channel to an absolute position (mm) using pre-set parameters.
    ///
    /// For an asynchronous alternative, see [`move_absolute_from_params_async`][1]
    ///
    /// [1]: KDC101::move_absolute_from_params_async
    pub fn move_absolute_from_params(&self) {
        block_on(async { self.move_absolute_from_params_async().await })
    }
}

impl ThorlabsDevice for KDC101 {
    fn inner(&self) -> &UsbPrimitive {
        &self.inner
    }

    fn channels(&self) -> u8 {
        1
    }

    fn abort(&self) {
        // todo()!
    }
}

impl CheckSerialNumber for KDC101 {
    const SERIAL_NUMBER_PREFIX: &'static str = "27";
}

impl UnitConversion for KDC101 {
    const ACCELERATION_SCALE_FACTOR: f64 = 263.8443072;
    const DISTANCE_ANGLE_SCALE_FACTOR: f64 = 34554.96;
    const VELOCITY_SCALE_FACTOR: f64 = 772981.3692;
}

impl Display for KDC101 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&block_on(async {
            format!("KDC101 ({:?})", self.inner) // See Debug trait for UsbPrimitive
        }))
    }
}
