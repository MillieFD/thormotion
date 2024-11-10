/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: unit_conversion.rs
Description: This file defines the UnitConversion trait, which provides functions for converting
between real and device units. This trait is required by all Thorlabs devices which move.
---------------------------------------------------------------------------------------------------
Notes:
*/

pub trait UnitConversion {
    const DISTANCE_ANGLE_SCALING_FACTOR: f32; // todo more efficient to use f64?
    const VELOCITY_SCALING_FACTOR: f32;
    const ACCELERATION_SCALING_FACTOR: f32;

    fn position_real_to_dev(&self, position_real: f32) -> f32 {
        Self::DISTANCE_ANGLE_SCALING_FACTOR * position_real
    }

    fn position_dev_to_real(&self, position_dev: f32) -> f32 {
        position_dev / Self::DISTANCE_ANGLE_SCALING_FACTOR
    }

    fn velocity_real_to_dev(&self, velocity_real: f32) -> f32 {
        Self::VELOCITY_SCALING_FACTOR * velocity_real
    }

    fn velocity_dev_to_real(&self, velocity_dev: f32) -> f32 {
        velocity_dev / Self::VELOCITY_SCALING_FACTOR
    }

    fn acceleration_real_to_dev(&self, acceleration_real: f32) -> f32 {
        Self::ACCELERATION_SCALING_FACTOR * acceleration_real
    }

    fn acceleration_dev_to_real(&self, acceleration_dev: f32) -> f32 {
        acceleration_dev / Self::ACCELERATION_SCALING_FACTOR
    }
}
