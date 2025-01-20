/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: impl_from_deref_display
*/

use crate::devices::UsbDevicePrimitive;
use crate::error::{EnumerationError, Error};
use std::fmt::Display;
use std::ops::Deref;

/// # Thorlabs Device
/// The `ThorlabsDevice` trait is a base trait implemented by all Thorlabs devices.
/// It defines functions which are common to all Thorlabs devices,
/// including functions to simplify communication using the APT protocol.
pub trait ThorlabsDevice:
    From<UsbDevicePrimitive>
    + From<String>
    + From<&'static str>
    + Deref<Target = UsbDevicePrimitive>
    + Display
    + Send
    + Sync
{
    /// # Serial Number Prefix
    /// Each Thorlabs device type has a unique serial number prefix. For example, KDC101
    /// "K-cubes" always have serial numbers which begin with "27". The `new()` function
    /// checks that the target serial number begins with the correct prefix for the
    /// calling struct. This prevents users from accidentally connecting to devices
    /// from the incorrect struct.
    const SERIAL_NUMBER_PREFIX: &'static str;
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
}

/// # `impl_thorlabs_device!` macro
///
/// This macro simplifies the implementation of several traits for a struct.
///
/// # Purpose
/// The macro takes a struct name as input and implements the following for the given struct:
///
/// - **`From<UsbDevicePrimitive>`**
///
/// - **`From<String>` and `From<&'static str>`**:
///   Provides the ability to construct the struct from a `String` or a string slice
///   (`&'static str`) representing the device's serial number.
///
/// - **`Deref`**:
///   Allows the struct to act as a reference to the inner `UsbDevicePrimitive` object.
///   This enables treating the struct as if it were directly interacting with the `UsbDevicePrimitive`,
///   simplifying usage in code that expects a `UsbDevicePrimitive`.
///
/// - **`Display` for the given struct**:
///   Provides a string representation of the struct in the format
///   `"{StructName} (serial number: {serial_number})"`.
///
/// # Notes
/// - This macro assumes the `new` and `check_serial_number` methods are implemented for the target struct.
/// - The asynchronous work required to fetch device details (`hw_req_info`) uses a `tokio` runtime,
///   which creates a new instance for each invocation.

#[macro_export]
macro_rules! impl_thorlabs_device {
    ($name: ident, $serial_number_prefix: expr) => {
        impl ThorlabsDevice for $name {
            const SERIAL_NUMBER_PREFIX: &'static str = $serial_number_prefix;
        }

        impl From<UsbDevicePrimitive> for $name {
            fn from(device: UsbDevicePrimitive) -> Self {
                Self::check_serial_number(device.serial_number.as_str()).unwrap_or_else(|err| {
                    panic!(
                        "{} From<UsbDevicePrimitive> failed: {}",
                        stringify!($name),
                        err
                    );
                });
                let (
                    serial_number,
                    model_number,
                    hardware_type,
                    firmware_version,
                    hardware_version,
                    module_state,
                    number_of_channels,
                ) = tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async { device.hw_req_info().await })
                    .unwrap();
                Self {
                    device,
                    serial_number,
                    model_number,
                    hardware_type,
                    firmware_version,
                    hardware_version,
                    module_state,
                    number_of_channels,
                }
            }
        }

        impl From<String> for $name {
            fn from(serial_number: String) -> Self {
                Self::new(serial_number.as_str()).unwrap_or_else(|err| {
                    panic!("{} From<String> failed: {}", stringify!($name), err);
                })
            }
        }

        impl From<&'static str> for $name {
            fn from(serial_number: &'static str) -> Self {
                Self::new(serial_number).unwrap_or_else(|err| {
                    panic!("{} From<&'static str> failed: {}", stringify!($name), err);
                })
            }
        }

        impl Deref for $name {
            type Target = UsbDevicePrimitive;

            fn deref(&self) -> &Self::Target {
                &self.device
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(
                    f,
                    "{} (serial number: {})",
                    stringify!($name),
                    self.serial_number
                )
            }
        }
    };
}
