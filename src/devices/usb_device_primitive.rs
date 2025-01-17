/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: usb_device_primitive.rs
Description: This file defines the UsbDevicePrimitive struct, which provides a wrapper around the
rusb DeviceHandle struct. The associated functions simplify the processes of initialising and
communicating with the underlying USB device.
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::env::{BUFFER_SIZE, IN_ENDPOINT, OUT_ENDPOINT, POLL_READ_INTERVAL, SHORT_TIMEOUT};
use crate::messages::{get_length, get_waiting_sender};
use crate::traits::MsgFormat;
use crate::Error;
use rusb::{DeviceDescriptor, DeviceHandle, GlobalContext, Language};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot::error::TryRecvError;
use tokio::sync::oneshot::{channel, Receiver, Sender};

/// UsbDevicePrimitive provides a wrapper around rusb's DeviceHandle for communicating with USB devices.
/// It handles device initialization, message formatting, and asynchronous I/O operations.
///
/// # Example
///
/// ```rust
/// use thormotion::devices::UsbDevicePrimitive;
/// use thormotion::enumerate::get_device;
/// use thormotion::Error;
///
/// fn main() -> Result<(), Error> {
///     // Initialize USB device
///     let serial_number: &str = "USB123456";
///     let device: UsbDevicePrimitive = get_device(serial_number)?;
///     
///     // The device is now initialised and ready for communication
///     Ok(())
/// }
/// ```

#[derive(Debug)]
pub struct UsbDevicePrimitive {
    handle: Arc<DeviceHandle<GlobalContext>>,
    descriptor: DeviceDescriptor,
    language: Language,
    pub(crate) serial_number: String,
    shutdown: Arc<Sender<()>>,
}

impl UsbDevicePrimitive {
    pub(crate) fn new(
        handle: DeviceHandle<GlobalContext>,
        descriptor: DeviceDescriptor,
        language: Language,
        serial_number: String,
    ) -> Result<Self, Error> {
        let (shutdown_tx, shutdown_rx) = channel();
        let dev = Self {
            handle: Arc::new(handle),
            descriptor,
            language,
            serial_number,
            shutdown: Arc::new(shutdown_tx),
        };
        dev.port_init()?;
        dev.poll_read(shutdown_rx)?;
        Ok(dev)
    }

    fn port_init(&self) -> Result<(), Error> {
        // Claim the interface
        self.handle.claim_interface(0)?;

        // Reset the device
        self.handle
            .write_control(0x40, 0x00, 0x0000, 0, &[], SHORT_TIMEOUT)?;

        // Set baud rate (115200)
        self.handle
            .write_control(0x40, 0x03, 0x001A, 0, &[], SHORT_TIMEOUT)?;

        // Set data format (8 data bits, 1 stop bit, no parity)
        self.handle
            .write_control(0x40, 0x04, 0x0008, 0, &[], SHORT_TIMEOUT)?;

        // Pre-purge dwell
        std::thread::sleep(Duration::from_millis(50));

        // Purge receive buffer
        self.handle
            .write_control(0x40, 0x00, 0x0001, 0, &[], SHORT_TIMEOUT)?;

        // Purge transmit buffer
        self.handle
            .write_control(0x40, 0x00, 0x0002, 0, &[], SHORT_TIMEOUT)?;

        // Post-purge dwell
        std::thread::sleep(Duration::from_millis(500));

        // Set flow control (RTS/CTS)
        self.handle
            .write_control(0x40, 0x02, 0x0200, 0, &[], SHORT_TIMEOUT)?;

        // Set RTS high
        self.handle
            .write_control(0x40, 0x01, 0x0202, 0, &[], SHORT_TIMEOUT)?;

        Ok(())
    }

    pub(crate) fn port_write(&self, data: MsgFormat) -> Result<(), Error> {
        if data.len() != self.handle.write_bulk(OUT_ENDPOINT, &data, SHORT_TIMEOUT)? {
            return Err(Error::DeviceError(format!(
                "Failed to write correct number of bytes to device (serial number: {})",
                self.serial_number,
            )));
        }
        Ok(())
    }

    fn poll_read(&self, mut shutdown_rx: Receiver<()>) -> Result<(), Error> {
        let handle = self.handle.clone();
        tokio::spawn(async move {
            let mut queue: VecDeque<u8> = VecDeque::with_capacity(2 * BUFFER_SIZE);
            loop {
                tokio::time::sleep(POLL_READ_INTERVAL).await;
                if shutdown_rx.try_recv() != Err(TryRecvError::Empty) {
                    break;
                }
                let mut buffer = [0u8; BUFFER_SIZE];
                let num_bytes_read = handle.read_bulk(IN_ENDPOINT, &mut buffer, SHORT_TIMEOUT)?;
                #[cfg(debug_assertions)]
                {
                    println!("num_bytes_read: {}", num_bytes_read);
                }
                if num_bytes_read == 2 {
                    continue;
                }
                queue.extend(&buffer[2..num_bytes_read]);
                #[cfg(debug_assertions)]
                {
                    println!(
                        "\nAdding {} bytes to queue\nQueue: {:?}\nQueue length: {} bytes",
                        num_bytes_read,
                        queue,
                        queue.len()
                    );
                }
                loop {
                    if queue.is_empty() {
                        #[cfg(debug_assertions)]
                        {
                            println!("Queue is empty. Breaking from inner loop.\n");
                        }
                        break;
                    }
                    let id: [u8; 2] = [queue[0], queue[1]];
                    let message_length = get_length(id)?;
                    #[cfg(debug_assertions)]
                    {
                        println!(
                            "\nMessage ID: {:?}\nExpected message length: {}",
                            id, message_length
                        );
                    }
                    if queue.len() < message_length {
                        #[cfg(debug_assertions)]
                        {
                            println!("Not enough bytes in queue\n");
                        }
                        break;
                    }
                    let message: Box<[u8]> = queue.drain(..message_length).collect();
                    #[cfg(debug_assertions)]
                    {
                        println!("Drained {} bytes from queue", message.len());
                    }
                    if let Some(tx) = get_waiting_sender(id)?.write()?.take() {
                        #[cfg(debug_assertions)]
                        {
                            println!("Sender found for id: {:?}", id);
                        }
                        tx.send(message)?;
                    }
                }
            }
            Ok::<(), Error>(())
        });
        Ok(())
    }
}
