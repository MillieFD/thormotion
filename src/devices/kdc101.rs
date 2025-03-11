/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use crate::devices::UsbDevicePrimitive;
use crate::error::Error;
use crate::functions::*;
use crate::macros::*;
use crate::traits::*;
use async_std::task::block_on;
use pyo3::prelude::*;
use std::fmt::Debug;
use std::ops::Deref;

#[pyclass]
#[derive(Debug)]
pub struct KDC101 {
    inner: UsbDevicePrimitive,
}

impl Deref for KDC101 {
    type Target = UsbDevicePrimitive;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFrom<UsbDevicePrimitive> for KDC101 {
    type Error = Error;

    fn try_from(inner: UsbDevicePrimitive) -> Result<Self, Self::Error> {
        Self::check_serial_number(inner.serial_number.as_str())?;
        let device = Self { inner };
        Ok(device)
    }
}

impl ThorlabsDevice for KDC101 {
    const SERIAL_NUMBER_PREFIX: &'static str = "27";
}

impl DistanceAngleConversion for KDC101 {
    const DISTANCE_ANGLE_SCALE_FACTOR: f64 = 34554.96;
}

impl VelocityConversion for KDC101 {
    const VELOCITY_SCALE_FACTOR: f64 = 772981.3692;
}

impl AccelerationConversion for KDC101 {
    const ACCELERATION_SCALE_FACTOR: f64 = 263.8443072;
}

#[pymethods]
impl KDC101 {
    #[new]
    pub fn new(serial_number: &str) -> Result<Self, Error> {
        ThorlabsDevice::new(serial_number)
    }

    #[doc = "Identifies the device by flashing the front panel LED"]
    #[doc = apt_doc!(async, "MOD_IDENTIFY", RUSB)]
    pub fn identify(&self) -> Result<(), Error> {
        __identify(self)
    }

    #[doc = "Starts periodic update messages from the device every 100 milliseconds (10 Hz)."]
    #[doc = "Automatic updates will continue until the `stop_update_messages` function is called."]
    #[doc = "A 'one-off' status update can be requested using `get_status_update_async`."]
    #[doc = apt_doc!(async, "HW_START_UPDATEMSGS", "GET_STATUSTUPDATE", RUSB)]
    pub fn start_update_messages(&self) -> Result<(), Error> {
        __start_update_messages(self)
    }

    #[doc = "Stops periodic update messages from the device every 100 milliseconds (10 Hz)."]
    #[doc = "Automatic updates will cease until the `start_update_messages` function is called."]
    #[doc = apt_doc!(async, "HW_STOP_UPDATEMSGS", RUSB)]
    pub fn stop_update_message(&self) -> Result<(), Error> {
        __stop_update_messages(self)
    }

    #[doc = "Returns the current position (mm), velocity (mm/s), and status of the device"]
    #[doc = apt_doc!(async, "MOT_REQ_STATUSUPDATE", "MOT_GET_STATUSUPDATE", RUSB, Timeout, FatalError)]
    pub async fn get_u_status_update_async(&self) -> Result<(f64, f64, i32), Error> {
        __get_u_status_update_async(self, 1).await
    }

    #[doc = "Returns the current position (mm), velocity (mm/s), and status of the device"]
    #[doc = apt_doc!(sync, "MOT_REQ_STATUSUPDATE", "MOT_GET_STATUSUPDATE", RUSB, Timeout, FatalError)]
    pub async fn get_u_status_update(&self) -> Result<(f64, f64, i32), Error> {
        block_on(__get_u_status_update_async(self, 1))
    }

    #[doc = "Homes the device."]
    #[doc = apt_doc!(async, "MOT_MOVE_HOME", "MOT_MOVE_HOMED", RUSB, Timeout, FatalError)]
    pub async fn home_async(&self) -> Result<(), Error> {
        __home_async(self, 1).await
    }

    #[doc = "Homes the device."]
    #[doc = apt_doc!(sync, "MOT_MOVE_HOME", "MOT_MOVE_HOMED", RUSB, Timeout, FatalError)]
    pub fn home(&self) -> Result<(), Error> {
        block_on(__home_async(self, 1))
    }

    #[doc = "Returns `True` if the device is enabled."]
    #[doc = apt_doc!(async, "MOD_REQ_CHANENABLESTATE", "MOD_GET_CHANENABLESTATE", RUSB, Timeout, FatalError)]
    pub async fn get_channel_enable_state_async(&self) -> Result<bool, Error> {
        __get_channel_enable_state_async(self, 1).await
    }

    #[doc = "Returns `True` if the device is enabled."]
    #[doc = apt_doc!(sync, "MOD_REQ_CHANENABLESTATE", "MOD_GET_CHANENABLESTATE", RUSB, Timeout, FatalError)]
    pub fn get_channel_enable_state(&self) -> Result<bool, Error> {
        block_on(__get_channel_enable_state_async(self, 1))
    }

    #[doc = "Enables or disables the device."]
    #[doc = apt_doc!(async, "MOD_REQ_CHANENABLESTATE", "MOD_GET_CHANENABLESTATE", RUSB, Timeout, FatalError)]
    pub async fn set_channel_enable_state_async(&self, enable: bool) -> Result<(), Error> {
        __set_channel_enable_state_async(self, 1, enable).await
    }

    #[doc = "Enables or disables the device."]
    #[doc = apt_doc!(sync, "MOD_REQ_CHANENABLESTATE", "MOD_GET_CHANENABLESTATE", RUSB, Timeout, FatalError)]
    pub async fn set_channel_enable_state(&self, enable: bool) -> Result<(), Error> {
        block_on(__set_channel_enable_state_async(self, 1, enable))
    }

    #[doc = "Moves the device to an absolute position (mm)"]
    #[doc = apt_doc!(async, "MOT_MOVE_ABSOLUTE", "MOT_MOVE_COMPLETED ", RUSB, Timeout, FatalError)]
    pub async fn move_absolute_async(&self, absolute_position: f64) -> Result<(), Error> {
        __move_absolute_async(self, 1, absolute_position).await
    }

    #[doc = "Moves the device to an absolute position (mm)"]
    #[doc = apt_doc!(async, "MOT_MOVE_ABSOLUTE", "MOT_MOVE_COMPLETED ", RUSB, Timeout, FatalError)]
    pub fn move_absolute(&self, absolute_position: f64) -> Result<(), Error> {
        block_on(__move_absolute_async(self, 1, absolute_position))
    }
}
