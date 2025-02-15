/// # Velocity Unit Conversion
///
/// Internally, all thorlabs motor devices use an encoder to keep track of their
/// current position.
/// All distances must therefore be converted from real-word units (mm) into
/// encoder-counts using the correct scale factor for the device.
/// This scale factor may differ between device types due to different encoder
/// resolutions and gearing ratios.
///
/// The device's unit of time is determined by the encoder polling frequency.
/// All time-based units (such as velocity and acceleration) must therefore be
/// converted from real-word units (seconds) into device units using the correct
/// scaling factor for the device.
/// This scaling factor may differ between device types due to different encoder
/// polling frequencies.
const VELOCITY_SCALE_FACTOR: f64 = template_scale_factor;
fn velocity_to_le_bytes(position: f64) -> [u8; 4] {
    let rounded = (position * Self::VELOCITY_SCALE_FACTOR).round();
    if !(rounded > i32::MIN.into() && rounded < i32::MAX.into()) {
        panic!(
            "f64 value {} cannot be converted to i32 because it is out of range. \
            i32 can only represent integers from {} to {} inclusive.",
            rounded,
            i32::MIN,
            i32::MAX,
        );
    }
    i32::to_le_bytes(rounded as i32)
}
fn velocity_from_le_bytes(bytes: [u8; 4]) -> f64 {
    let encoder_counts: f64 = i32::from_le_bytes(bytes).into();
    encoder_counts / Self::VELOCITY_SCALE_FACTOR
}
