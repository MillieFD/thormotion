/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: enumeration
Description: This file contains static functions to open serial connections to target devices.
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::error::Error;
use rusb::{Context, UsbContext};
use std::fs::{read_dir, DirEntry};
use std::io;
use std::path::Path;

static BAUD_RATE: u32 = 115200;
static VENDOR_ID: u16 = 1027;
static PRODUCT_ID: u16 = 64240;

fn is_dev_connected(serial_number: &String) -> Result<(), Error> {
    let context = Context::new()?;
    for device in context.devices()?.iter() {
        let device_desc = device.device_descriptor()?;
        if device_desc.vendor_id() == VENDOR_ID && device_desc.product_id() == PRODUCT_ID {
            let handle = device.open()?;
            if handle.read_serial_number_string_ascii(&device_desc)? == *serial_number {
                return Ok(());
            }
        }
    }
    Err(Error::NotFound("Device not connected or switched on"))
}

fn filter_paths(path: &DirEntry, serial_number: &String) -> Option<String> {
    let file_name = path.file_name().into_string().ok()?;
    if file_name.contains(serial_number) {
        Some(path.path().to_string_lossy().to_string())
    } else {
        None
    }
}

fn get_dev_paths<P: AsRef<Path>>(serial_number: &String, dir_path: P) -> io::Result<Vec<String>> {
    let dev_paths = read_dir(dir_path)?
        .filter_map(Result::ok)
        .filter_map(|path| filter_paths(&path, &serial_number))
        .collect::<Vec<String>>();
    Ok(dev_paths)
}

fn get_dev_path(serial_number: String) -> Result<String, Error> {
    is_dev_connected(&serial_number)?;
    let dev_paths = get_dev_paths(&serial_number, "/dev/serial/by-id/")?;
    match dev_paths.len() {
        0 => Err(Error::Path("No device paths found")),
        1 => Ok(dev_paths.first().unwrap().to_string()),
        _ => Err(Error::Path("Multiple device paths found")),
    }
}

pub(crate) fn open_serial_port(
    serial_number: String,
) -> Result<Box<dyn serialport::SerialPort>, Error> {
    let device_path = get_dev_path(serial_number)?;
    let port = serialport::new(&device_path, BAUD_RATE).open()?;
    Ok(port)
}
