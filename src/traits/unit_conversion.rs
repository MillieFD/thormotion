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

use std::ops::Deref;

use crate::devices::abort;

pub(crate) enum Units {
    Distance([u8; 4]),
    Velocity([u8; 4]),
    Acceleration([u8; 4]),
}

impl Units {
    /// Coerces a slice `&[u8]` into an array `[u8; 4]`.
    #[doc(hidden)]
    #[inline]
    fn array_from_slice(slice: &[u8]) -> [u8; 4] {
        slice.try_into().unwrap_or_else(|e| {
            abort(format!(
                "Cannot coerce slice {:?} to array [u8; 4] : {}",
                slice, e
            ))
        })
    }

    /// Constructs a new [`Units::Distance`] from device units.
    ///
    /// ### Aborts
    ///
    /// This function aborts if the slice cannot be coerced into a four-byte array `[u8; 4]`
    pub(crate) fn distance_from_slice(slice: &[u8]) -> Units {
        Units::Distance(Units::array_from_slice(slice))
    }

    /// Constructs a new [`Units::Velocity`] from device units.
    ///
    /// ### Aborts
    ///
    /// This function aborts if the slice cannot be coerced into a four-byte array `[u8; 4]`
    pub(crate) fn velocity_from_slice(slice: &[u8]) -> Units {
        Units::Velocity(Units::array_from_slice(slice))
    }

    /// Constructs a new [`Units::Acceleration`] from device units.
    ///
    /// ### Aborts
    ///
    /// This function aborts if the slice cannot be coerced into a four-byte array `[u8; 4]`
    pub(crate) fn acceleration_from_slice(slice: &[u8]) -> Units {
        Units::Acceleration(Units::array_from_slice(slice))
    }

    /// Converts an `f64` to an unwrapped little-endian byte array `[u8; 4]`.
    ///
    /// You can manually wrap the result in the appropriate [`Units`] variant. To automatically wrap
    /// the result, see the [`new_distance`][1], [`new_velocity`][2], and [`new_acceleration`][3]
    /// functions.
    ///
    /// [1]: Units::distance_from_f64
    /// [2]: Units::velocity_from_f64
    /// [3]: Units::acceleration_from_f64
    fn encode(value: f64, scale_factor: f64) -> [u8; 4] {
        let scaled = value * scale_factor;
        let rounded = scaled.round();
        i32::to_le_bytes(rounded as i32)
    }

    /// Converts a distance (millimeters) or angle (degrees) from real-world units to device units
    /// using the appropriate [`scale factor`][1].
    ///
    /// [1]: UnitConversion::DISTANCE_ANGLE_SCALE_FACTOR
    pub(crate) fn distance_from_f64<A>(distance: f64) -> Units
    where
        A: UnitConversion,
    {
        let bytes = Units::encode(distance, A::DISTANCE_ANGLE_SCALE_FACTOR);
        Units::Distance(bytes)
    }

    /// Converts a velocity from real-world units (mm/s) to device units using the appropriate
    /// [`scale factor`][1].
    ///
    /// [1]: UnitConversion::VELOCITY_SCALE_FACTOR
    pub(crate) fn velocity_from_f64<A>(velocity: f64) -> Units
    where
        A: UnitConversion,
    {
        let bytes = Units::encode(velocity, A::VELOCITY_SCALE_FACTOR);
        Units::Distance(bytes)
    }

    /// Converts an acceleration from real-world units (mm/s²) to device units using the appropriate
    /// [`scale factor`][1].
    ///
    /// [1]: UnitConversion::ACCELERATION_SCALE_FACTOR
    pub(crate) fn acceleration_from_f64<A>(acceleration: f64) -> Units
    where
        A: UnitConversion,
    {
        let bytes = Units::encode(acceleration, A::ACCELERATION_SCALE_FACTOR);
        Units::Distance(bytes)
    }

    /// Consumes the [`Units`] enum, returning real-world units (millimeters and seconds) using the
    /// appropriate [`scale factor`][1].
    ///
    /// [1]: UnitConversion
    pub(crate) const fn decode<A>(&self) -> f64
    where
        A: UnitConversion,
    {
        match self {
            Units::Distance(d) => i32::from_le_bytes(*d) as f64 / A::DISTANCE_ANGLE_SCALE_FACTOR,
            Units::Velocity(v) => i32::from_le_bytes(*v) as f64 / A::VELOCITY_SCALE_FACTOR,
            Units::Acceleration(a) => i32::from_le_bytes(*a) as f64 / A::ACCELERATION_SCALE_FACTOR,
        }
    }

    /// Returns `True` if [`self`][1] and [`other`][1] are equivalent within three decimal places.
    ///
    /// [1]: Units
    pub(crate) const fn approx<A>(&self, other: f64) -> bool
    where
        A: UnitConversion,
    {
        (self.decode::<A>() - other).abs() < 0.001
    }
}

impl Deref for Units {
    type Target = [u8; 4];

    fn deref(&self) -> &Self::Target {
        match self {
            Units::Distance(distance) => distance,
            Units::Velocity(velocity) => velocity,
            Units::Acceleration(acceleration) => acceleration,
        }
    }
}

impl IntoIterator for Units {
    type IntoIter = std::array::IntoIter<u8, 4>;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Units::Distance(bytes) => IntoIterator::into_iter(bytes),
            Units::Velocity(bytes) => IntoIterator::into_iter(bytes),
            Units::Acceleration(bytes) => IntoIterator::into_iter(bytes),
        }
    }
}

/// # Thorlabs "Device Units" Explained
///
/// Internally, thorlabs devices use an encoder to track of their current position. All distances
/// must therefore be converted from real-word units (millimeters) to encoder-counts using the
/// correct scaling factor. This scaling factor may differ between device types due to different
/// encoder resolutions and gearing ratios.
///
/// The device's unit of time is determined by the encoder polling frequency. All time-dependent
/// units (e.g. velocity and acceleration) must therefore be converted from real-word units
/// (seconds) to device units using the correct scaling factor. This scaling factor may differ
/// between device types due to different encoder polling frequencies.
pub(crate) trait UnitConversion {
    const ACCELERATION_SCALE_FACTOR: f64;
    const DISTANCE_ANGLE_SCALE_FACTOR: f64;
    const VELOCITY_SCALE_FACTOR: f64;
}
