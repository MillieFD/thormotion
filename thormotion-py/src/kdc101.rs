/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::io::Error;

use pyo3::{pyclass, pymethods};
use thormotion::{devices, error};

#[pyclass]
pub(crate) struct KDC101 {
    inner: devices::KDC101,
}

#[pymethods]
impl KDC101 {
    #[new]
    fn new(serial_number: String) -> Result<Self, error::sn::Error> {
        Ok(Self {
            inner: devices::KDC101::new(serial_number)?,
        })
    }

    /// Returns `True` if the `USB Interface` is open.
    async fn is_open(&self) -> bool {
        self.inner.is_open().await
    }

    /// Opens an `Interface` to the `USB Device`.
    ///
    /// No action is taken if the device `Status` is already `Open`.
    ///
    /// For a synchronous alternative, see `open`.
    pub async fn open_async(&mut self) -> Result<(), Error> {
        self.inner.open_async().await
    }

    /// Opens an `Interface` to the `USB Device`.
    ///
    /// No action is taken if the device `Status` is already `Open`.
    ///
    /// For an asynchronous alternative, see `async_open`.
    pub fn open(&mut self) -> Result<(), Error> {
        self.inner.open()
    }

    /// Releases the claimed `Interface` to the `USB Device`.
    ///
    /// No action is taken if the device `Status` is already `Closed`.
    ///
    /// This does not stop the device's current action. If you need to safely bring the
    /// `USB Device` to a resting state, see `abort`.
    ///
    /// For a synchronous alternative, see `close`.
    pub async fn close_async(&mut self) -> Result<(), Error> {
        self.inner.close_async().await
    }

    /// Releases the claimed `Interface` to the `USB Device`.
    ///
    /// No action is taken if the device `Status` is already `Closed`.
    ///
    /// This does not stop the device's current action. If you need to safely bring the
    /// `USB Device` to a resting state, see `abort`.
    ///
    /// For an asynchronous alternative, see `async_close`.
    pub fn close(&mut self) -> Result<(), Error> {
        self.inner.close()
    }

    /// Identifies the device by flashing the front panel LED.
    ///
    /// For a synchronous alternative, see `identify`.
    pub async fn identify_async(&self) {
        self.inner.identify_async().await;
    }

    /// Identifies the device by flashing the front panel LED.
    ///
    /// For an asynchronous alternative, see `async_identify`.
    pub fn identify(&self) {
        self.inner.identify();
    }

    /// Returns the current position (mm) and velocity (mm/s) for the specified device channel.
    ///
    /// For a synchronous alternative, see `get_status`.
    pub async fn get_status_async(&self) -> (f64, f64) {
        self.inner.get_status_async().await
    }

    /// Returns the current position (mm) and velocity (mm/s) for the specified device channel.
    ///
    /// For an asynchronous alternative, see `async_get_status`.
    pub fn get_status(&self) -> (f64, f64) {
        self.inner.get_status()
    }

    /// Starts periodic update messages from the device every 100 milliseconds (10 Hz).
    ///
    /// Automatic updates will continue until the `hw_stop_update_messages` function is called.
    /// A 'one-off' status update can be requested using `get_status`.
    ///
    /// For a synchronous alternative, see `hw_start_update_messages`.
    pub async fn hw_start_update_messages_async(&self) {
        self.inner.hw_start_update_messages_async().await;
    }

    /// Starts periodic update messages from the device every 100 milliseconds (10 Hz).
    ///
    /// Automatic updates will continue until the `hw_stop_update_messages` function is called.
    /// A 'one-off' status update can be requested using `get_status`.
    ///
    /// For an asynchronous alternative, see `hw_start_update_messages_async`.
    pub fn hw_start_update_messages(&self) {
        self.inner.hw_start_update_messages();
    }

    /// Stops periodic update messages from the device every 100 milliseconds (10 Hz).
    ///
    /// Automatic updates will cease until the `hw_start_update_messages` function is called.
    ///
    /// For a synchronous alternative, see `hw_stop_update_messages`.
    pub async fn hw_stop_update_messages_async(&self) {
        self.inner.hw_stop_update_messages_async().await;
    }

    /// Stops periodic update messages from the device every 100 milliseconds (10 Hz).
    ///
    /// Automatic updates will cease until the `hw_start_update_messages` function is called.
    ///
    /// For an asynchronous alternative, see `hw_stop_update_messages_async`.
    pub fn hw_stop_update_messages(&self) {
        self.inner.hw_stop_update_messages();
    }

    /// Returns `True` if the specified device channel is enabled.
    pub async fn get_channel_enable_state_async(&self) {
        self.inner.get_channel_enable_state_async().await;
    }

    /// Returns `True` if the specified device channel is enabled.
    pub async fn get_channel_enable_state(&self) {
        self.inner.get_channel_enable_state().await;
    }

    /// Enables or disables the specified device channel.
    pub async fn set_channel_enable_state_async(&self, enable: bool) {
        self.inner.set_channel_enable_state_async(enable).await;
    }

    /// Enables or disables the specified device channel.
    pub async fn set_channel_enable_state(&self, enable: bool) {
        self.inner.set_channel_enable_state(enable).await;
    }

    /// Homes the specified device channel.
    ///
    /// For a synchronous alternative, see `home`
    ///
    /// [1]: thormotion::devices::KDC101::home
    pub async fn home_async(&self) {
        self.inner.home_async().await;
    }

    /// Homes the specified device channel.
    ///
    /// For an asynchronous alternative, see `async_home`
    pub fn home(&self) {
        self.inner.home();
    }

    /// Moves the specified device channel to an absolute position.
    ///
    /// For a synchronous alternative, see `move_absolute`
    pub async fn move_absolute_async(&self, position: f64) {
        self.inner.move_absolute_async(position).await;
    }

    /// Moves the specified device channel to an absolute position.
    ///
    /// For an asynchronous alternative, see `async_move_absolute`
    pub fn move_absolute(&self, position: f64) {
        self.inner.move_absolute(position);
    }

    /// Moves the specified device channel to an absolute position (mm) using pre-set parameters.
    ///
    /// For a synchronous alternative, see `move_absolute_from_params`
    pub async fn move_absolute_from_params_async(&self) {
        self.inner.move_absolute_from_params_async().await;
    }

    /// Moves the specified device channel to an absolute position (mm) using pre-set parameters.
    ///
    /// For an asynchronous alternative, see `move_absolute_from_params_async`
    pub fn move_absolute_from_params(&self) {
        self.inner.move_absolute_from_params();
    }
}
