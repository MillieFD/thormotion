/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: device_enumeration.rs
Description: This file contains functions to open serial connections to target devices.
---------------------------------------------------------------------------------------------------
Notes:
*/

use rusb::{Context, UsbContext};
use std::fs::DirEntry;
use std::{fs, io};

static BAUD_RATE: u32 = 115200;
static VENDOR_ID: u16 = 1027;
static PRODUCT_ID: u16 = 64240;

fn is_device_connected(serial_number: &str) -> Result<bool, rusb::Error> {
    let context = Context::new()?;
    for device in context.devices()?.iter() {
        let device_desc = device.device_descriptor()?;
        if device_desc.vendor_id() == VENDOR_ID && device_desc.product_id() == PRODUCT_ID {
            let handle = device.open()?;
            return Ok(handle.read_serial_number_string_ascii(&device_desc)? == serial_number);
        }
    }
    Err(rusb::Error::NotFound)
}

fn is_device_path(path: &DirEntry, serial_number: &str) -> Result<bool, io::Error> {
    let file_name = path.file_name().into_string().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Error converting file name to string: {:?}", e),
        )
    })?;
    Ok(file_name.contains(serial_number))
}

fn find_device_path_by_serial_number(serial_number: &str) -> Result<String, io::Error> {
    if !is_device_connected(serial_number).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!(
                "RUSB error whilst searching for device with serial number {}: {}",
                serial_number, e
            ),
        )
    })? {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Device with serial number {} is not connected",
                serial_number
            ),
        ));
    }

    let paths = fs::read_dir("/dev/serial/by-id/")?;
    let target_device_paths: Vec<DirEntry> = paths
        .filter_map(Result::ok)
        .filter(|path| is_device_path(path, serial_number).unwrap_or(false))
        .collect();

    match target_device_paths.len() {
        1 => Ok(target_device_paths
            .first()
            .unwrap()
            .path()
            .to_string_lossy()
            .to_string()),
        0 => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No device paths with serial number: {}", serial_number),
        )),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Multiple devices found with serial number: {}",
                serial_number
            ),
        )),
    }
}

pub fn open_serial_port(serial_number: &str) -> Result<Box<dyn serialport::SerialPort>, io::Error> {
    let device_path = find_device_path_by_serial_number(serial_number)?;
    serialport::new(&device_path, BAUD_RATE)
        .open()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Error opening serial port at path: {:?} with serial number: {}",
                    device_path, e
                ),
            )
        })
}
