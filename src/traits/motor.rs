/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: motor.rs
*/

use crate::traits::ThorlabsDevice;
use std::f64;

/// # Motor
/// The `Motor` trait is implemented by all Thorlabs brushed and brushless DC servo motors,
/// as well as all stepper motor devices.
/// The `Motor` trait functions are not intended to be accessible to end-users.
/// Instead, it provides functions to simplify motion control.
/// End-user movement functions are implemented for each device struct independently.
/// These functions are detailed in the Thorlabs APT protocol **Motor Control Messages**
/// section and are named with the `MOT` prefix.
pub trait Motor: ThorlabsDevice {
    /// # Unit Conversion
    /// Internally, all thorlabs motor devices use an encoder to keep track of their current
    /// position. All distances must therefore be converted from real-word units (mm) into
    /// encoder-counts using the correct scaling factor for the device. This scaling factor
    /// may differ between device types due to different encoder resolutions and gearing
    /// ratios.
    ///
    /// The device's unit of time is determined by the encoder polling frequency. All
    /// time-based units (such as velocity and acceleration) must therefore be converted
    /// from real-word units (seconds) into device units using the correct scaling factor
    /// for the device. This scaling factor may differ between device types due to
    /// different encoder polling frequencies.
    const DISTANCE_ANGLE_SCALING_FACTOR: f64;
    const VELOCITY_SCALING_FACTOR: f64;
    const ACCELERATION_SCALING_FACTOR: f64;

    fn position_to_bytes(position: f64) -> [u8; 4] {
        if !position.is_finite() {
            panic!(
                "f64 value {} cannot be converted to i32 because it is not finite",
                position
            );
        }
        let rounded = (position * Self::DISTANCE_ANGLE_SCALING_FACTOR).round();
        if rounded < i32::MIN.into() || rounded > i32::MAX.into() {
            panic!(
                "f64 value {} rounded to {}, which cannot be converted to i32 because it is out \
                of range. i32 can only represent integers from {} to {} inclusive.",
                position,
                rounded,
                i32::MIN,
                i32::MAX
            );
        }
        i32::to_le_bytes(rounded as i32)
    }

    fn position_from_bytes(bytes: [u8; 4]) -> f64 {
        let encoder_counts: f64 = i32::from_le_bytes(bytes).into();
        encoder_counts / Self::DISTANCE_ANGLE_SCALING_FACTOR
    }

    fn velocity_to_bytes(velocity: f64) -> [u8; 4] {
        if !velocity.is_finite() {
            panic!(
                "f64 value {} cannot be converted to i32 because it is not finite",
                velocity
            );
        }
        let rounded = (velocity * Self::VELOCITY_SCALING_FACTOR).round();
        if rounded < i32::MIN.into() || rounded > i32::MAX.into() {
            panic!(
                "f64 value {} rounded to {}, which cannot be converted to i32 because it is out \
                of range. i32 can only represent integers from {} to {} inclusive.",
                velocity,
                rounded,
                i32::MIN,
                i32::MAX
            );
        }
        i32::to_le_bytes(rounded as i32)
    }

    fn velocity_from_bytes(bytes: [u8; 4]) -> f64 {
        let encoder_counts: f64 = i32::from_le_bytes(bytes).into();
        encoder_counts / Self::VELOCITY_SCALING_FACTOR
    }

    fn acceleration_to_bytes(acceleration: f64) -> [u8; 4] {
        if !acceleration.is_finite() {
            panic!(
                "f64 value {} cannot be converted to i32 because it is not finite",
                acceleration
            );
        }
        let rounded = (acceleration * Self::ACCELERATION_SCALING_FACTOR).round();
        if rounded < i32::MIN.into() || rounded > i32::MAX.into() {
            panic!(
                "f64 value {} rounded to {}, which cannot be converted to i32 because it is out \
                of range. i32 can only represent integers from {} to {} inclusive.",
                acceleration,
                rounded,
                i32::MIN,
                i32::MAX
            );
        }
        i32::to_le_bytes(rounded as i32)
    }

    fn acceleration_from_bytes(bytes: [u8; 4]) -> f64 {
        let encoder_counts: f64 = i32::from_le_bytes(bytes).into();
        encoder_counts / Self::ACCELERATION_SCALING_FACTOR
    }
}
