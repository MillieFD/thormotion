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

use crate::devices::{add_device, UsbPrimitive};
use crate::error::sn;
use crate::functions;
use crate::messages::Metadata;
use crate::traits::{CheckSerialNumber, ThorlabsDevice, UnitConversion, Units};

/// KDC101 devices have one channel.
const CH: usize = 1;

#[cfg_attr(feature = "py", pyo3::pyclass)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KDC101 {
    inner: Arc<UsbPrimitive<CH>>,
}

impl KDC101 {}

#[cfg_attr(feature = "py", pyo3::pymethods)]
impl KDC101 {
    const IDS: [Metadata<1>; 5] = [
        // MOD
        Metadata::header([0x12, 0x02]), // GET_CHANENABLESTATE
        // MOT
        Metadata::header([0x44, 0x04]), // MOVE_HOMED
        Metadata::payload([0x64, 0x04], 20), // MOVE_COMPLETED
        Metadata::payload([0x66, 0x04], 20), // MOVE_STOPPED
        Metadata::payload([0x91, 0x04], 20), // GET_USTATUSUPDATE
    ];

    #[cfg(not(feature = "py"))]
    #[doc = include_str!("../documentation/new.md")]
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

    #[cfg(feature = "py")]
    #[new]
    #[doc = include_str!("../documentation/new.md")]
    pub fn new(serial_number: String) -> Result<Self, sn::Error> {
        Self::check_serial_number(&sn)?;
        let device = Self {
            inner: Arc::new(UsbPrimitive::new(&sn, &Self::IDS)?),
        };
        let d = device.clone(); // Inexpensive Arc Clone
        let f = move || d.abort();
        add_device(sn, f);
        Ok(device)
    }

    /* ------------------------------------------------------------------------------------- MOD */

    #[thormacros::sync]
    #[doc = include_str!("../documentation/open.md")]
    pub async fn open_async(&mut self) -> Result<(), Error> {
        self.inner.open().await
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/close.md")]
    pub async fn close_async(&mut self) -> Result<(), Error> {
        self.inner.close().await
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/is_open.md")]
    pub async fn is_open_async(&self) -> bool {
        self.inner.is_open().await
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/identify.md")]
    pub async fn identify_async(&self) {
        functions::identify(self, 1).await;
    }

    /* ---------------------------------------------------------------------------------- STATUS */

    #[thormacros::sync]
    #[doc = include_str!("../documentation/get_status.md")]
    pub async fn get_status_async(&self) -> (f64, f64, u32) {
        functions::get_u_status_update(self, 1).await
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/get_position.md")]
    pub async fn get_position_async(&self) -> f64 {
        self.get_status_async().await.0
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/get_velocity.md")]
    pub async fn get_velocity_async(&self) -> f64 {
        self.get_status_async().await.1
    }

    #[thormacros::sync]
    pub async fn get_status_bits_async(&self) -> u32 {
        self.get_status_async().await.2
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/is_homed.md")]
    pub async fn is_homed_async(&self) -> bool {
        let bits = self.get_status_bits_async().await;
        (bits & 0x00000400) != 0
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/hw_start_update_messages.md")]
    pub async fn hw_start_update_messages_async(&self) {
        functions::hw_start_update_messages(self).await;
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/hw_stop_update_messages.md")]
    pub async fn hw_stop_update_messages_async(&self) {
        functions::hw_stop_update_messages(self).await;
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/get_channel_enable_state.md")]
    pub async fn get_channel_enable_state_async(&self) {
        functions::get_channel_enable_state(self, 1).await;
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/set_channel_enable_state.md")]
    pub async fn set_channel_enable_state_async(&self, enable: bool) {
        functions::set_channel_enable_state(self, 1, enable).await;
    }

    /* ------------------------------------------------------------------------------------ MOVE */

    #[thormacros::sync]
    #[doc = include_str!("../documentation/home.md")]
    pub async fn home_async(&self) {
        functions::home(self, 1).await;
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/move_absolute.md")]
    pub async fn move_absolute_async(&self, position: f64) {
        functions::move_absolute(self, 1, position).await;
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/move_absolute_from_params.md")]
    pub async fn move_absolute_from_params_async(&self) {
        functions::move_absolute_from_params(self, 1).await;
    }

    #[thormacros::sync]
    pub async fn move_relative_async(&self, distance: f64) {
        let start = self.get_position_async().await;
        functions::move_relative(self, 1, distance).await;
        let end = self.get_position_async().await;
        if !Units::approx((end - start).abs(), distance) {
            log::error!("{self} MOVE_RELATIVE (failed tolerance) START {start:.3} END {end:.3}");
        }
    }

    #[thormacros::sync]
    pub async fn move_relative_from_params_async(&self) {
        functions::move_relative_from_params(self, 1).await;
    }

    /* ------------------------------------------------------------------------------------ STOP */

    #[thormacros::sync]
    #[doc = include_str!("../documentation/stop.md")]
    pub async fn stop_async(&self) {
        functions::stop(self, 1).await;
    }

    #[thormacros::sync]
    #[doc = include_str!("../documentation/estop.md")]
    pub async fn estop_async(&self) {
        functions::estop(self, 1).await;
    }
}

impl ThorlabsDevice<CH> for KDC101 {
    fn inner(&self) -> &UsbPrimitive<1> {
        &self.inner
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
        write!(f, "KDC101 {}", self.serial_number())
    }
}
