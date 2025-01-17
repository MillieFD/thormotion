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
use tokio::sync::oneshot::{channel, error::TryRecvError, Receiver, Sender};

/// # UsbDevicePrimitive
/// This struct provides a wrapper around the rusb `DeviceHandle` struct,
/// which implements functions for communicating with USB devices.
/// `UsbDevicePrimitive` handles device initialisation,
/// message formatting, and asynchronous I/O operations.
///
/// # Example
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
    /// # Initialising UsbDevicePrimitive
    /// This struct provides a wrapper around the rusb `DeviceHandle` struct,
    /// which implements functions for communicating with USB devices.
    /// Instances of the `UsbDevicePrimitive` struct are created using the `new()`
    /// function which is called during **enumeration**.
    /// The `new()` function is passed information about the USB device from the
    /// rusb `DeviceList<GlobalContext>`
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

    /// # Initialise Serial-USB Settings
    /// The required serial port settings are described in the Thorlabs APT protocol documentation.
    ///
    /// 1. **Claim the Interface**: Ensures exclusive access to the device's USB interface.
    /// 2. **Reset the Device**: Sends a control request to clear any previous communication settings.
    /// 3. **Set Baud Rate**: Configures the communication speed to 115,200 baud.
    /// 4. **Set Data Format**: Specifies properties such as 8 data bits, 1 stop bit, and no parity.
    /// 5. **Purge Buffers**: Pauses momentarily, then clears both receive and transmit buffers.
    /// 6. **Flow Control Configuration**: Enables RTS/CTS (Request to Send / Clear to Send).
    /// 7. **Set RTS High**: Activates the RTS (Ready to Send) signal,
    /// indicating readiness for communication.
    ///
    /// If an error occurs at any step, it propagates back to the caller,
    /// halting the initialisation process.
    fn port_init(&self) -> Result<(), Error> {
        self.handle.claim_interface(0)?;
        self.handle
            .write_control(0x40, 0x00, 0x0000, 0, &[], SHORT_TIMEOUT)?;
        self.handle
            .write_control(0x40, 0x03, 0x001A, 0, &[], SHORT_TIMEOUT)?;
        self.handle
            .write_control(0x40, 0x04, 0x0008, 0, &[], SHORT_TIMEOUT)?;
        std::thread::sleep(Duration::from_millis(50));
        self.handle
            .write_control(0x40, 0x00, 0x0001, 0, &[], SHORT_TIMEOUT)?;
        self.handle
            .write_control(0x40, 0x00, 0x0002, 0, &[], SHORT_TIMEOUT)?;
        std::thread::sleep(Duration::from_millis(500));
        self.handle
            .write_control(0x40, 0x02, 0x0200, 0, &[], SHORT_TIMEOUT)?;
        self.handle
            .write_control(0x40, 0x01, 0x0202, 0, &[], SHORT_TIMEOUT)?;
        Ok(())
    }

    /// # Sending Messages to the USB Device
    ///
    /// The `port_write()` function sends a formatted message to the USB device.
    ///
    /// This function writes data to the USB device in bulk.
    /// It accepts an instance of the `MsgFormat` enum which contains the data to be sent.
    /// After sending the message, the function ensures that the correct number of bytes
    /// were written by verifying against the returned value.
    /// If the number of bytes written does not match the data length,
    /// an `Error::DeviceError` containing the device's serial number is returned.
    pub(crate) fn port_write(&self, data: MsgFormat) -> Result<(), Error> {
        if data.len() != self.handle.write_bulk(OUT_ENDPOINT, &data, SHORT_TIMEOUT)? {
            return Err(Error::DeviceError(format!(
                // todo ?make `serial_number` struct which implements `From<T>` so it can be made from numbers and strings, also implements `Display` so it automatically prints "(serial number: #######)"?
                "Failed to write correct number of bytes to device (serial number: {})",
                self.serial_number,
            )));
        }
        Ok(())
    }

    /// # Receiving Incoming Messages from the USB Device
    ///
    /// The `poll_read` function spawns an asynchronous background task that
    /// continuously polls the USB device for incoming data.
    ///
    /// ## Key Steps:
    ///
    /// 1. **Setup**
    ///     - A clone of the USB handle is created for concurrent access.
    ///     - A `VecDeque` is initialised to store incoming data in a queue
    ///     format for processing.
    ///
    /// 2. **Polling Loop**
    ///     - The function enters a loop which continuously reads data from the device
    ///     - The function waits for a specified interval (`POLL_READ_INTERVAL`) between
    ///     polls to reduce unnecessary CPU usage
    ///     - Data is read from the USB device into a buffer using `read_bulk`.
    ///
    /// 3. **When a message is received**
    ///     - Received data is appended to the queue
    ///     - Each message in the Thorlabs APT protocol can be uniquely identified
    ///     using a 2-byte message ID. The function reads this ID to determine the
    ///     length of the message.
    ///     - The queue is checked to see if a sufficient number of bytes are
    ///     present to form a full message.
    ///     - When sufficient data is available:
    ///         - The complete message is extracted from the queue.
    ///         - The message is broadcast to any `awaiting` functions using `tx.send()`
    ///
    /// ## Error Handling:
    ///
    /// Errors in reading or processing data propagate via the `Result` type.
    /// If the `debug_assertions` build flag is enabled, debug information
    /// will be printed to the console for troubleshooting.
    fn poll_read(&self, mut shutdown_rx: Receiver<()>) -> Result<(), Error> {
        let handle = self.handle.clone();
        tokio::spawn(async move {
            let mut queue: VecDeque<u8> = VecDeque::with_capacity(2 * BUFFER_SIZE);
            loop {
                tokio::time::sleep(POLL_READ_INTERVAL).await;
                if shutdown_rx.try_recv() != Err(TryRecvError::Empty) {
                    break;
                    // todo shutdown should trigger a shutdown or disconnect message to be sent to the Thorlabs device, then elegantly release the rusb handle
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
