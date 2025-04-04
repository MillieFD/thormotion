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

pub(crate) struct UnitConverter {
    scale_factor: f64,
}

impl UnitConverter {
    /// Converts an acceleration from real-world units (millimeters and seconds) to device units
    /// using the appropriate [`scale factor`][1].
    ///
    /// [1]: UnitConversion
    pub(crate) fn to_le_bytes(&self, position: f64) -> [u8; 4] {
        let scaled = position * self.scale_factor;
        let rounded = scaled.round();
        i32::to_le_bytes(rounded as i32)
    }

    /// Converts an acceleration from device units to real-world units (millimeters and seconds)
    /// using the appropriate [`scale factor`][1].
    ///
    /// [1]: UnitConversion
    pub(crate) fn to_f64(&self, bytes: [u8; 4]) -> f64 {
        let encoder_counts = i32::from_le_bytes(bytes) as f64;
        encoder_counts / self.scale_factor
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

    const ACCELERATION: UnitConverter = UnitConverter {
        scale_factor: Self::ACCELERATION_SCALE_FACTOR,
    };

    const DISTANCE_ANGLE_SCALE_FACTOR: f64;

    const DISTANCE_ANGLE: UnitConverter = UnitConverter {
        scale_factor: Self::DISTANCE_ANGLE_SCALE_FACTOR,
    };

    const VELOCITY_SCALE_FACTOR: f64;

    const VELOCITY: UnitConverter = UnitConverter {
        scale_factor: Self::VELOCITY_SCALE_FACTOR,
    };
}
