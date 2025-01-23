/// # Serial Number Prefix
///
/// Each Thorlabs device type has a unique serial number prefix. For example, KDC101
/// "K-cubes" always have serial numbers which begin with "27". The `new()`
/// function checks that the target serial number begins with the correct prefix
/// for the calling struct. This prevents users from accidentally connecting to
/// devices from the incorrect struct.
const SERIAL_NUMBER_PREFIX: &'static str = "template_prefix";
fn check_serial_number(serial_number: &str) -> Result<(), Error> {
    if serial_number.starts_with(Self::SERIAL_NUMBER_PREFIX) {
        Ok(())
    } else {
        Err(EnumerationError(format!(
            "Serial number {} is not valid for the selected device type. \
            Expected a serial number starting with {}",
            serial_number,
            Self::SERIAL_NUMBER_PREFIX,
        )))
    }
}
