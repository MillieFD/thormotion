/// # `From<UsbDevicePrimitive>`
/// This implementation of the `From` trait allows users to construct the struct
/// from an existing `UsbDevicePrimitive` instance using the `.from()` function.
/// A corresponding `.into()` function is also automatically generated.
impl From<UsbDevicePrimitive> for TemplateStructName {
    fn from(device: UsbDevicePrimitive) -> Self {
        Self::check_serial_number(device.serial_number.as_str()).unwrap_or_else(|err| {
            panic!(
                "KDC101 (serial number: {}) From<UsbDevicePrimitive> failed: {}",
                device.serial_number, err
            );
        });
        Self { device }
    }
}

/// # `From<String>`
/// This implementation of the `From` trait allows users to construct the struct
/// from a `String` using the `.from()` function.
/// The `String` should represent the device's serial number.
/// A corresponding `.into()` function is also automatically generated.
impl From<String> for TemplateStructName {
    fn from(serial_number: String) -> Self {
        let device = UsbDevicePrimitive::from(serial_number);
        Self::from(device)
    }
}

/// # `From<&str>`
/// This implementation of the `From` trait allows users to construct the struct
/// from a `&str` using the `.from()` function.
/// The `&str` should represent the device's serial number.
/// A corresponding `.into()` function is also automatically generated.
impl From<&'static str> for TemplateStructName {
    fn from(serial_number: &'static str) -> Self {
        let device = UsbDevicePrimitive::from(serial_number);
        Self::from(device)
    }
}

/// # `From<i32>`
/// This implementation of the `From` trait allows users to construct the struct
/// from an `i32` using the `.from()` function.
/// The `i32` should represent the device's serial number.
/// A corresponding `.into()` function is also automatically generated.
impl From<i32> for TemplateStructName {
    fn from(serial_number: i32) -> Self {
        let device = UsbDevicePrimitive::from(serial_number);
        Self::from(device)
    }
}

/// # `From<u32>`
/// This implementation of the `From` trait allows users to construct the struct
/// from a `u32` using the `.from()` function.
/// The `u32` should represent the device's serial number.
/// A corresponding `.into()` function is also automatically generated.
impl From<u32> for TemplateStructName {
    fn from(serial_number: u32) -> Self {
        let device = UsbDevicePrimitive::from(serial_number);
        Self::from(device)
    }
}
