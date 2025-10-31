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
    const IDS: [Command; 5] = [
        // MOD
        Command::header([0x12, 0x02]), // GET_CHANENABLESTATE
        // MOT
        Command::header([0x44, 0x04]),      // MOVE_HOMED
        Command::payload([0x64, 0x04], 20), // MOVE_COMPLETED
        Command::payload([0x66, 0x04], 20), // MOVE_STOPPED
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

    #[doc = include_str!("../documentation/is_open.md")]
    pub async fn is_open_async(&self) -> bool {
        self.inner.is_open().await
    }

    #[doc = include_str!("../documentation/is_open.md")]
    pub fn is_open(&self) -> bool {
        block_on(async { self.is_open_async().await })
    }

    #[doc = include_str!("../documentation/open.md")]
    pub async fn open_async(&mut self) -> Result<(), Error> {
        self.inner.open().await
    }

    #[doc = include_str!("../documentation/open.md")]
    pub fn open(&mut self) -> Result<(), Error> {
        block_on(async { self.open_async().await })
    }

    #[doc = include_str!("../documentation/close.md")]
    pub async fn close_async(&mut self) -> Result<(), Error> {
        self.inner.close().await
    }

    #[doc = include_str!("../documentation/close.md")]
    pub fn close(&mut self) -> Result<(), Error> {
        block_on(async { self.close_async().await })
    }

    #[doc = include_str!("../documentation/identify.md")]
    pub async fn identify_async(&self) {
        __identify(self, 1).await;
    }

    #[doc = include_str!("../documentation/identify.md")]
    pub fn identify(&self) {
        block_on(async { self.identify_async().await })
    }

    #[doc = include_str!("../documentation/get_status.md")]
    pub async fn get_status_async(&self) -> (f64, f64, u32) {
        __get_u_status_update(self, 1).await
    }

    #[doc = include_str!("../documentation/get_status.md")]
    pub fn get_status(&self) -> (f64, f64, u32) {
        block_on(async { self.get_status_async().await })
    }

    #[doc = include_str!("../documentation/is_homed.md")]
    pub async fn is_homed_async(&self) -> bool {
        let (_, _, bits) = self.get_status_async().await;
        (bits & 0x00000400) != 0
    }

    #[doc = include_str!("../documentation/is_homed.md")]
    pub async fn is_homed(&self) -> bool {
        block_on(async { self.is_homed_async().await })
    }

    #[doc = include_str!("../documentation/hw_start_update_messages.md")]
    pub async fn hw_start_update_messages_async(&self) {
        __hw_start_update_messages(self).await;
    }

    #[doc = include_str!("../documentation/hw_start_update_messages.md")]
    pub fn hw_start_update_messages(&self) {
        block_on(async { self.hw_start_update_messages_async().await })
    }

    #[doc = include_str!("../documentation/hw_stop_update_messages.md")]
    pub async fn hw_stop_update_messages_async(&self) {
        __hw_stop_update_messages(self).await;
    }

    #[doc = include_str!("../documentation/hw_stop_update_messages.md")]
    pub fn hw_stop_update_messages(&self) {
        block_on(async { self.hw_stop_update_messages_async().await })
    }

    #[doc = include_str!("../documentation/get_channel_enable_state.md")]
    pub async fn get_channel_enable_state_async(&self) {
        __req_channel_enable_state(self, 1).await;
    }

    #[doc = include_str!("../documentation/get_channel_enable_state.md")]
    pub async fn get_channel_enable_state(&self) {
        block_on(async { self.get_channel_enable_state_async().await })
    }

    #[doc = include_str!("../documentation/set_channel_enable_state.md")]
    pub async fn set_channel_enable_state_async(&self, enable: bool) {
        __set_channel_enable_state(self, 1, enable).await;
    }

    #[doc = include_str!("../documentation/set_channel_enable_state.md")]
    pub async fn set_channel_enable_state(&self, enable: bool) {
        block_on(async { self.set_channel_enable_state_async(enable).await })
    }

    #[doc = include_str!("../documentation/home.md")]
    pub async fn home_async(&self) {
        __home(self, 1).await;
    }

    #[doc = include_str!("../documentation/home.md")]
    pub fn home(&self) {
        block_on(async { self.home_async().await })
    }

    #[doc = include_str!("../documentation/move_absolute.md")]
    pub async fn move_absolute_async(&self, position: f64) {
        __move_absolute(self, 1, position).await;
    }

    #[doc = include_str!("../documentation/move_absolute.md")]
    pub fn move_absolute(&self, position: f64) {
        block_on(async { self.move_absolute_async(position).await })
    }

    #[doc = include_str!("../documentation/move_absolute_from_params.md")]
    pub async fn move_absolute_from_params_async(&self) {
        __move_absolute_from_params(self, 1).await;
    }

    #[doc = include_str!("../documentation/move_absolute_from_params.md")]
    pub fn move_absolute_from_params(&self) {
        block_on(async { self.move_absolute_from_params_async().await })
    }

    #[doc = include_str!("../documentation/stop.md")]
    pub async fn stop_async(&self) {
        __stop(self, 1).await;
    }

    #[doc = include_str!("../documentation/stop.md")]
    pub fn stop(&self) {
        block_on(async { self.stop_async().await })
    }

    #[doc = include_str!("../documentation/estop.md")]
    pub async fn estop_async(&self) {
        __estop(self, 1).await;
    }

    #[doc = include_str!("../documentation/estop.md")]
    pub fn estop(&self) {
        block_on(async { self.estop_async().await })
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
        self.estop()
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
