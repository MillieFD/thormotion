#[new]
pub fn new(serial_number: &str) -> Self {
    Self::check_serial_number(serial_number).unwrap();
    let device = get_device_primitive(serial_number).unwrap();
    Self { device }
}
