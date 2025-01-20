/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
License: BSD 3-Clause "New" or "Revised" License, Copyright (c) 2025, Amelia Fraser-Dale
Filename: usb_device_primitive.rs
*/

use crate::env::{
    BUFFER_SIZE, DEST, IN_ENDPOINT, LONG_TIMEOUT, OUT_ENDPOINT, POLL_READ_INTERVAL, SHORT_TIMEOUT,
    SOURCE,
};
use crate::error::{DeviceError, Error};
use crate::messages::ChannelStatus::{New, Sub};
use crate::messages::{get_length, get_rx_new_or_sub, get_waiting_sender, MsgFormat};
use rusb::{DeviceDescriptor, DeviceHandle, GlobalContext, Language};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot::{channel, error::TryRecvError, Receiver, Sender};
use tokio::time::timeout;

/// # UsbDevicePrimitive
/// The `UsbDevicePrimitive` struct provides a wrapper around the rusb `DeviceHandle` struct,
/// which implements functions for communicating with USB devices.
/// `UsbDevicePrimitive` handles device initialisation,
/// message formatting, and asynchronous I/O operations.
///
/// # Example
/// ```rust
/// use thormotion::devices::UsbDevicePrimitive;
/// use thormotion::enumerate::get_device_primitive;
/// use thormotion::Error;
///
/// fn main() -> Result<(), Error> {
///     // Initialize USB device
///     let serial_number: &str = "USB123456";
///     let device: UsbDevicePrimitive = get_device_primitive(serial_number)?;
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
            return Err(DeviceError(format!(
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

    /// # HW_REQ_INFO (0x0005)
    ///
    /// **Function implemented from Thorlabs APT protocol**
    ///
    /// This function is used to request hardware information from the controller.
    /// This function is not intended to be accessed by end-users.
    /// Instead, `hw_req_info()` is used to populate the hardware information fields for
    /// device structs during their `new()` function.
    ///
    /// Message ID: 0x0005
    ///
    /// Message length: 6 bytes (header only)
    ///
    /// # Response
    ///
    /// The controller will send a `HW_GET_INFO (0x0006)` message in response, which
    /// is then parsed into the component values and packaged into a tuple.
    ///
    /// Response ID: 0x0006
    ///
    /// Response length: 90 bytes (6-byte header followed by 84-byte data packet)
    pub(crate) async fn hw_req_info(
        &self,
    ) -> Result<(u32, String, u16, String, u16, u16, u16), Error> {
        const ID: [u8; 2] = [0x00, 0x05];
        let mut rx = match get_rx_new_or_sub(ID)? {
            Sub(rx) => rx,
            New(rx) => {
                let data = pack_short_message(ID, 0, 0);
                self.port_write(data)?;
                rx
            }
        };
        let response = timeout(LONG_TIMEOUT, rx.recv()).await??;
        let serial_number = u32::from_le_bytes(response[6..10].try_into()?);
        let model_number = String::from_utf8_lossy(&response[10..18]).to_string();
        let hardware_type = u16::from_le_bytes(response[18..20].try_into()?);
        let firmware_minor = u8::from_le_bytes(response[20..21].try_into()?);
        let firmware_interim = u8::from_le_bytes(response[21..22].try_into()?);
        let firmware_major = u8::from_le_bytes(response[22..23].try_into()?);
        let hardware_version = u16::from_le_bytes(response[84..86].try_into()?);
        let module_state = u16::from_le_bytes(response[86..88].try_into()?);
        let number_of_channels = u16::from_le_bytes(response[88..90].try_into()?);
        Ok((
            serial_number,
            model_number,
            hardware_type,
            format!("{}.{}.{}", firmware_major, firmware_interim, firmware_minor),
            hardware_version,
            module_state,
            number_of_channels,
        ))
    }
}

/// # Pack Functions
///
/// The Thorlabs APT communication protocol uses a fixed length 6-byte message header, which
/// may be followed by a variable-length data packet.
/// For simple commands, the 6-byte message header is sufficient to convey the entire command.
/// For more complex commands (e.g. commands where a set of parameters needs to be passed
/// to the device) the 6-byte header is insufficient and must be followed by a data packet.
///
/// The `MsgFormat` enum is used to wrap the bytes of a message and indicate whether the
/// message is `Short` (six byte header only) or `Long` (six byte header plus variable length
/// data package).
///
/// The `pack_short_message()` and `pack_long_message()` helper functions are implemented to
/// simplify message formatting and enforce consistency with the APT protocol.
pub(crate) fn pack_short_message(id: [u8; 2], param1: u8, param2: u8) -> MsgFormat {
    MsgFormat::Short([id[0], id[1], param1, param2, DEST, SOURCE])
}

pub(crate) fn pack_long_message(id: [u8; 2], length: usize) -> MsgFormat {
    let mut data: Vec<u8> = Vec::with_capacity(length);
    data.extend(id);
    data.extend(((length - 6) as u16).to_le_bytes());
    data.push(DEST | 0x80);
    data.push(SOURCE);
    MsgFormat::Long(data)
}
