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

use std::fmt::Debug;

pub(crate) trait DistanceAngleConversion {
    const DISTANCE_ANGLE_SCALE_FACTOR: f64;

    /**
    Converts a position or angle from real-world units (millimetres) into device units
    (little-endian byte array).

    This function translates the input position or angle value from millimetres (mm) into the
    corresponding device-specific units, represented as a signed 32-bit integer encoded as a
    four-byte little-endian array. This conversion is performed using the device's
    `DISTANCE_ANGLE_SCALE_FACTOR` constant.

    # Thorlabs "Device Units" Explained

    Internally, thorlabs devices use an encoder to keep track of their current position. All
    distances must therefore be converted from real-word units (mm) into encoder-counts using the
    correct scaling factor for the device. This scaling factor may differ between device types due
    to different encoder resolutions and gearing ratios.

    # Examples

    ```rust
    const DISTANCE_ANGLE_SCALE_FACTOR: f64 = 34304.0; // Example scale factor

    let velocity = 6.345;
    let bytes = velocity_to_le_bytes(velocity);

    assert_eq!(bytes, [0x3B, 0x52, 0x03, 0x00]);
    ```

    # Panics

    - Panics if the input `position` cannot be converted to `f64`
    - Panics if the scaled position is out of range for `i32`.
    */
    fn position_angle_to_le_bytes<T>(position: T) -> [u8; 4]
    where
        T: TryInto<f64> + Clone + Debug,
        <T as TryInto<f64>>::Error: Debug,
    {
        let position_f64: f64 = position
            .clone()
            .try_into()
            .expect(&format!("Cannot convert {:?} to f64", position));
        let scaled = position_f64 * Self::DISTANCE_ANGLE_SCALE_FACTOR;
        let rounded = scaled.round();
        if rounded < i32::MIN.into() || rounded > i32::MAX.into() {
            panic!(
                "f64 value {} cannot be converted to i32 because it is out of range.\n\
                i32 can only represent integers from {} to {} inclusive.",
                rounded,
                i32::MIN,
                i32::MAX,
            );
        }
        i32::to_le_bytes(rounded as i32)
    }

    /**
    Converts a position or angle from device units (little-endian byte array) into real-world
    units (mm).

    This function translates the input position or angle value from device-specific units,
    represented as a signed 32-bit integer encoded as a four-byte little-endian array, into
    millimetres (mm). This conversion is performed using the device's `DISTANCE_ANGLE_SCALE_FACTOR`
    constant.

    # Thorlabs "Device Units" Explained

    Internally, thorlabs devices use an encoder to keep track of their current position. All
    distances must therefore be converted from encoder-counts into real-word units (mm) using the
    correct scaling factor for the device. This scaling factor may differ between device types due
    to different encoder resolutions and gearing ratios.
    */
    fn position_angle_from_le_bytes<T>(bytes: [u8; 4]) -> T
    where
        T: From<f64>,
    {
        let encoder_counts: f64 = i32::from_le_bytes(bytes).into();
        (encoder_counts / Self::DISTANCE_ANGLE_SCALE_FACTOR).into()
    }
}

pub(crate) trait VelocityConversion {
    const VELOCITY_SCALE_FACTOR: f64;

    /**
    Converts a velocity from real-world units (mm/s) into device units (little-endian byte array).

    This function translates the input velocity value from millimetres per second (mm/s) into the
    corresponding device-specific units, represented as a signed 32-bit integer encoded as a
    four-byte little-endian array. This conversion is performed using the device's
    `DISTANCE_ANGLE_SCALE_FACTOR` constant.

    # Thorlabs "Device Units" Explained

    Internally, thorlabs devices use an encoder to keep track of their current position. All
    distances must therefore be converted from real-word units (mm/s) into encoder-counts using the
    correct scaling factor for the device. This scaling factor may differ between device types due
    to different encoder resolutions and gearing ratios.

    The device's unit of time is determined by the encoder polling frequency. All time-based units
    (such as velocity and acceleration) must therefore be converted from real-word units (seconds)
    into device units using the correct scaling factor for the device. This scaling factor may
    differ between device types due to different encoder polling frequencies.

    # Examples

    ```rust
    const VELOCITY_SCALE_FACTOR: f64 = 34304.0; // Example scale factor

    let velocity = 6.345;
    let bytes = velocity_to_le_bytes(velocity);

    assert_eq!(bytes, [0x3B, 0x52, 0x03, 0x00]);
    ```

    # Panics

    - Panics if the input `position` cannot be converted to `f64`
    - Panics if the scaled position is out of range for `i32`.
    */
    fn velocity_to_le_bytes<T>(position: T) -> [u8; 4]
    where
        T: TryInto<f64> + Clone + Debug,
        <T as TryInto<f64>>::Error: Debug,
    {
        let position_f64: f64 = position
            .clone()
            .try_into()
            .expect(&format!("Cannot convert {:?} to f64", position));
        let scaled = position_f64 * Self::VELOCITY_SCALE_FACTOR;
        let rounded = scaled.round();
        if rounded < i32::MIN.into() || rounded > i32::MAX.into() {
            panic!(
                "f64 value {} cannot be converted to i32 because it is out of range.\n\
                i32 can only represent integers from {} to {} inclusive.",
                rounded,
                i32::MIN,
                i32::MAX,
            );
        }
        i32::to_le_bytes(rounded as i32)
    }

    /**
    Converts a velocity from device units (little-endian byte array) into real-world units (mm/s).

    This function translates the input velocity value from device-specific units, represented as a
    signed 32-bit integer encoded as a four-byte little-endian array, into millimetres per second
    (mm/s). This conversion is performed using the device's `VELOCITY_SCALE_FACTOR` constant.

    # Thorlabs "Device Units" Explained

    Internally, thorlabs devices use an encoder to keep track of their current position. All
    distances must therefore be converted from encoder-counts into real-word units (mm/s) using the
    correct scaling factor for the device. This scaling factor may differ between device types due
    to different encoder resolutions and gearing ratios.

    The device's unit of time is determined by the encoder polling frequency. All time-based units
    (such as velocity and acceleration) must therefore be converted from real-word units (seconds)
    into device units using the correct scaling factor for the device. This scaling factor may
    differ between device types due to different encoder polling frequencies.
    */
    fn velocity_from_le_bytes<T>(bytes: [u8; 4]) -> T
    where
        T: From<f64>,
    {
        let encoder_counts: f64 = i32::from_le_bytes(bytes).into();
        (encoder_counts / Self::VELOCITY_SCALE_FACTOR).into()
    }
}

pub(crate) trait AccelerationConversion {
    const ACCELERATION_SCALE_FACTOR: f64;

    /**
    Converts an acceleration from real-world units (mm/s²) into device units (little-endian byte
    array).

    This function translates the input acceleration value from millimetres per second per second
    (mm/s²) into the corresponding device-specific units, represented as a signed 32-bit integer
    encoded as a four-byte little-endian array. This conversion is performed using the device's
    `ACCELERATION_SCALE_FACTOR` constant.

    # Thorlabs "Device Units" Explained

    Internally, thorlabs devices use an encoder to keep track of their current position. All
    distances must therefore be converted from real-word units (mm/s²) into encoder-counts using the
    correct scaling factor for the device. This scaling factor may differ between device types due
    to different encoder resolutions and gearing ratios.

    The device's unit of time is determined by the encoder polling frequency. All time-based units
    (such as velocity and acceleration) must therefore be converted from real-word units (seconds)
    into device units using the correct scaling factor for the device. This scaling factor may
    differ between device types due to different encoder polling frequencies.

    # Examples

    ```rust
    const ACCELERATION_SCALE_FACTOR: f64 = 34304.0; // Example scale factor

    let acceleration = 6.345;
    let bytes = acceleration_to_le_bytes(acceleration);

    assert_eq!(bytes, [0x3B, 0x52, 0x03, 0x00]);
    ```

    # Panics

    - Panics if the input `position` cannot be converted to `f64`
    - Panics if the scaled position is out of range for `i32`.
    */
    fn acceleration_to_le_bytes<T>(position: T) -> [u8; 4]
    where
        T: TryInto<f64> + Clone + Debug,
        <T as TryInto<f64>>::Error: Debug,
    {
        let position_f64: f64 = position
            .clone()
            .try_into()
            .expect(&format!("Cannot convert {:?} to i32", position));
        let scaled = position_f64 * Self::ACCELERATION_SCALE_FACTOR;
        let rounded = scaled.round();
        if rounded < i32::MIN.into() || rounded > i32::MAX.into() {
            panic!(
                "f64 value {} cannot be converted to i32 because it is out of range.\n\
                i32 can only represent integers from {} to {} inclusive.",
                rounded,
                i32::MIN,
                i32::MAX,
            );
        }
        i32::to_le_bytes(rounded as i32)
    }

    /**
    Converts an acceleration from device units (little-endian byte array) into real-world units
    (mm/s²).

    This function translates the input acceleration value from device-specific units,
    represented as a signed 32-bit integer encoded as a four-byte little-endian array,
    into millimetres per second per second (mm/s²).
    This conversion is performed using the device's `ACCELERATION_SCALE_FACTOR` constant.

    # Thorlabs "Device Units" Explained

    Internally, thorlabs devices use an encoder to keep track of their current position. All
    distances must therefore be converted from encoder-counts into real-word units (mm/s²) using
    the correct scaling factor for the device. This scaling factor may differ between device types
    due to different encoder resolutions and gearing ratios.

    The device's unit of time is determined by the encoder polling frequency. All time-based units
    (such as velocity and acceleration) must therefore be converted from real-word units (seconds)
    into device units using the correct scaling factor for the device. This scaling factor may
    differ between device types due to different encoder polling frequencies.
    */
    fn acceleration_from_le_bytes<T>(bytes: [u8; 4]) -> T
    where
        T: From<f64>,
    {
        let encoder_counts: f64 = i32::from_le_bytes(bytes).into();
        (encoder_counts / Self::ACCELERATION_SCALE_FACTOR).into()
    }
}
