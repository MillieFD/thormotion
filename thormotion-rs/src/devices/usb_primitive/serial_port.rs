/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::time::Duration;

use nusb::Interface;
use nusb::transfer::{ControlOut, ControlType, Recipient};
use smol::Timer;

use crate::devices::abort;

/// Control transfer to reset the USB device controller
const RESET_CONTROLLER: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x00,
    value: 0x0000,
    index: 0,
    data: &[],
};
/// Control transfer to set the baud rate to 115,200
const BAUD_RATE: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x03,
    value: 0x001A, // 115,200 baud
    index: 0,
    data: &[],
};

/// Control transfer to set the data format to 8 data bits, 1 stop bit, and no parity.
const EIGHT_DATA_ONE_STOP_NO_PARITY: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x04,
    value: 0x0008, // 8 data bits, 1 stop bit, no parity
    index: 0,
    data: &[],
};

/// Control transfer to purge (clear) the receiving buffer.
///
/// This ensures no stale data remains in the device's receiving buffer.
const PURGE_RX: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x00,
    value: 0x0001, // Purge RX buffer
    index: 0,
    data: &[],
};

/// Control transfer to purge (clear) the transmitting buffer.
///
/// This ensures no stale data remains in the device's transmitting buffer.
const PURGE_TX: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x00,
    value: 0x0002, // Purge TX buffer
    index: 0,
    data: &[],
};

/// Control transfer to enable RTS/CTS hardware flow control.
///
/// This prevents buffer overruns by allowing the device and host to signal when they are
/// ready to receive data.
const FLOW_CONTROL: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x02,
    value: 0x0200, // Enable RTS/CTS flow control
    index: 0,
    data: &[],
};

/// Control transfer to set the Request To Send (RTS) signal.
///
/// This indicates to the device that the host is ready to receive data.
const RTS: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x01,
    value: 0x0202, // Set RTS
    index: 0,
    data: &[],
};

/// Initializes serial port settings according to Thorlabs APT protocol requirements:
/// - Baud rate 115200
/// - Eight data bits
/// - One stop bit
/// - No parity
/// - RTS/CTS flow control
pub(super) async fn init(interface: &Interface) {
    let control_out = async |control_out: ControlOut| {
        interface
            .control_out(control_out)
            .await
            .status
            .unwrap_or_else(|e| abort(format!("Control transfer failed : {}", e)))
    };
    control_out(RESET_CONTROLLER).await;
    control_out(BAUD_RATE).await;
    control_out(EIGHT_DATA_ONE_STOP_NO_PARITY).await;
    Timer::after(Duration::from_millis(50)).await; // Pre-purge dwell 50 ms
    control_out(PURGE_RX).await;
    control_out(PURGE_TX).await;
    Timer::after(Duration::from_millis(50)).await; // Post-purge dwell 50 ms
    control_out(FLOW_CONTROL).await;
    control_out(RTS).await;
}
