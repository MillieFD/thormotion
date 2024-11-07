/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: error_types
Description: todo
---------------------------------------------------------------------------------------------------
Notes:
*/

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum Error {
    // UsbDevicePrimitive errors
    DeviceNotFound(String),
    DeviceNotSupported(String),
    DeviceHardwareError(u32, u32),

    // Protocol errors
    MessageIdNotFound(u16),
    MessageGroupNameNotFound(String),
    MessageGroupNameAlreadyExists(String),
    DeviceMessageSendFailure(String, Box<[u8]>),
    DeviceMessageReceiveFailure(u32),

    // Channel/Threading errors
    OneshotSenderExists(String, Box<[u8]>),

    // External errors with implicit conversions
    OneshotError(Box<[u8]>),
    OneshotReceiverError(String),
    RwLockPoisoned(String),
    RusbError(String),

    // Other errors
    FatalError(String),
}

impl Error {
    pub(crate) fn message(&self) -> String {
        match self {
            Error::DeviceNotFound(serial_number) =>
                format!("Device with serial number {serial_number} could not be found"),
            Error::DeviceNotSupported(serial_number) =>
                format!("Device with serial number {serial_number} is not supported"),
            Error::DeviceHardwareError(serial_number_get_hw_info, serial_number) =>
                format!("Serial number returned by GET_HW_INFO ({serial_number_get_hw_info}) does no match expected serial number ({serial_number})"),
            Error::MessageIdNotFound(id) =>
                format!("Invalid message id: {id}"),
            Error::MessageGroupNameNotFound(name) =>
                format!("Message group with name '{name}' could not be found"),
            Error::MessageGroupNameAlreadyExists(name) =>
                format!("Message group with name '{name}' already exists"),
            Error::DeviceMessageSendFailure(serial_number, data) => {
                let id_str = String::from_utf8(data[0..2].to_vec()).unwrap_or_else(|e| format!("Invalid UTF-8: {e}"));
                format!("Failed to send message (id: {id_str}) to device (serial number: {serial_number})")
            }
            Error::DeviceMessageReceiveFailure(serial_number) =>
                format!("Failed to read incoming message from device (serial number: {serial_number})"),
            Error::OneshotSenderExists(serial_number, data) => {
                let id_str = String::from_utf8(data[0..2].to_vec()).unwrap_or_else(|e| format!("Invalid UTF-8: {e}"));
                format!("Tried to send message (id: {id_str}) to device (serial number: {serial_number}) but a message of this type is already waiting for a response")
            }
            Error::OneshotError(data) =>
                format!("Failed to send response message via oneshot::channel. Lost data: {data:?}"),
            Error::OneshotReceiverError(msg) =>
                format!("Error from oneshot channel receiver: {msg}"),
            Error::RwLockPoisoned(msg) =>
                format!("Error from RwLock: {msg}"),
            Error::RusbError(msg) =>
                format!("Error from rusb: {msg}"),
            Error::FatalError(msg) =>
                format!("Encountered a fatal error: {msg}\nExiting program..."),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
