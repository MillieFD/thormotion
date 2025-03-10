/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use crate::devices::utils::get_usb_device_primitive;
use crate::durations::{DEFAULT_LONG_TIMEOUT, DEFAULT_POLL_INTERVAL, DEFAULT_SHORT_TIMEOUT};
use crate::error::Error;
use crate::messages::{MsgFormat, utils::get_channel, utils::get_length};
use async_std::future::timeout;
use async_std::sync::Arc;
use async_std::task::{self, JoinHandle};
use rusb::{DeviceDescriptor, DeviceHandle, GlobalContext, Language};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::sync::OnceLock;
use std::time::Duration;

const OUT_ENDPOINT: u8 = 0x02;
const IN_ENDPOINT: u8 = 0x81;
const BUFFER_SIZE: usize = 256;

/**
A wrapper for `rusb::DeviceHandle` that simplifies communication with Thorlabs USB devices.
```
*/
#[derive(Debug)]
pub struct UsbDevicePrimitive {
    /**
    An `rusb::DeviceHandle` that is used to read/write data to/from the USB device.
    This is itself a wrapper around `libusb_device_handle`
    */
    rusb_device_handle: Arc<DeviceHandle<GlobalContext>>,
    /**
    An `rusb::DeviceDescriptor` that contains metadata about the USB device.
    This is itself a wrapper around `libusb_device_descriptor`
    */
    rusb_device_descriptor: DeviceDescriptor,
    /**
    An `rusb::Language` that indicates the language (e.g. English)
    used to interpret USB string descriptors.
    */
    rusb_device_language: Language,
    /**
    A serial number which can be used to uniquely identify the USB device.
    This serial number is written on your Thorlabs device.
    */
    pub(super) serial_number: String,
    /**
    Timout for fast operations such as reading from the USB buffer.
    Initialised with a default value of 500 milliseconds.
    */
    short_timeout: Duration,
    /**
    Timout for slow operations such as waiting for the device homing sequence to complete.
    Initialised with a default value of 100 seconds.
    */
    long_timeout: Duration,
    /**
    Duration to wait between polling the USB buffer for incoming messages.
    Initialised with a default value of 200 milliseconds.
    */
    poll_interval: Duration,
    /**
    A `JoinHandle` for the asynchronous background task that periodically polls the USB device
    buffer for incoming messages.
    */
    poll_task_handle: OnceLock<JoinHandle<()>>,
}

impl UsbDevicePrimitive {
    pub(super) fn new(
        rusb_device_handle: DeviceHandle<GlobalContext>,
        rusb_device_descriptor: DeviceDescriptor,
        rusb_device_language: Language,
    ) -> Self {
        let serial_number = rusb_device_handle
            .read_serial_number_string(
                rusb_device_language,
                &rusb_device_descriptor,
                Duration::from_secs(1),
            )
            .expect("Failed to read serial number from rusb device handle");
        let device = Self {
            rusb_device_handle: Arc::new(rusb_device_handle),
            rusb_device_descriptor,
            rusb_device_language,
            serial_number,
            short_timeout: DEFAULT_SHORT_TIMEOUT,
            long_timeout: DEFAULT_LONG_TIMEOUT,
            poll_interval: DEFAULT_POLL_INTERVAL,
            poll_task_handle: OnceLock::new(),
        };
        device
            .initialise_serial_port()
            .expect("Failed to initialise serial port settings");
        device
            .poll_task_handle
            .set(device.spawn_poll_task())
            .expect("Failed to set poll task handle");
        device
    }

    /**
    Spawns an asynchronous background task that periodically reads incoming data from the USB
    device.

    The stream of incoming bytes is segmented into discrete messages using the message ID.
    Messages are then dispatched to any internal waiting for a response from the device. The
    frequency of USB read operations is controlled by `UsbDevicePrimitive.poll_interval`,
    which is initialised with a default value of 200 milliseconds.

    # Panics

    The function will panic if:
    - The `rusb::DeviceHandle` fails to read data from the USB device.
    - An unknown or invalid message ID is encountered.
    - The `async_std::channel::Sender` fails to dispatch the message.

    # Errors

    The spawned task is to panic with `.expect()` instead of raising an error.
    By default, the `async-std` runtime will unwind all threads if a panic occurs on any thread.
    If the USB device becomes unreachable, the spawned background task will panic,
    causing the entire program to terminate.

    This behaviour is intentional to ensure that disconnected devices do not cause undefined
    behaviour or unintended consequences. By enforcing a complete program termination,
    this mechanism guarantees that critical device failures are dealt with safely and explicitly.

    # Examples

    ```rust
    let device = UsbDevicePrimitive::new(device_handle, device_descriptor, device_language);
    device.spawn_poll_task();
    ```
    */
    fn spawn_poll_task(&self) -> JoinHandle<()> {
        let poll_interval = self.poll_interval.clone();
        let rusb_device_handle = self.rusb_device_handle.clone();
        let short_timeout = self.short_timeout;

        let handle = task::spawn(async move {
            let mut queue: VecDeque<u8> = VecDeque::with_capacity(2 * BUFFER_SIZE);
            loop {
                task::sleep(poll_interval).await;
                let mut buffer = [0u8; BUFFER_SIZE];
                let num_bytes_read = rusb_device_handle
                    .read_bulk(IN_ENDPOINT, &mut buffer, short_timeout)
                    .expect(&format!("Failed to read from {:?}", rusb_device_handle));
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
                    let message_length = get_length(id);
                    #[cfg(debug_assertions)]
                    {
                        println!(
                            "\nMessage ID: {:?}\nExpected message length: {}",
                            id, message_length
                        );
                    }
                    if queue.len() < *message_length {
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
                    if let Some(channel) = timeout(DEFAULT_LONG_TIMEOUT, get_channel(id).write())
                        .await
                        .expect(&format!("Function timed out while trying to get WriteGuard for channel for message id {:?}", id))
                        .take()
                    {
                        #[cfg(debug_assertions)]
                        {
                            println!("Sender found for id: {:?}", id);
                        }
                        timeout(DEFAULT_LONG_TIMEOUT, channel.sender.send(message))
                            .await
                            .expect("Function timed out while trying to send message")
                            .expect("Cannot send a message into a closed channel");
                    }
                }
            }
        });
        handle
    }

    /**
    Initialises serial port settings to communicate with the connected Thorlabs device.

    This function configures a serial port according to the requirements described
    in the Thorlabs APT protocol. Refer to the Thorlabs APT protocol for further
    detail about the required serial port settings.

    # Steps

    1. **Claim the Interface**: Establishes exclusive access to the device's USB
       interface to prevent conflicts with other processes.
    2. **Reset the Device**: Sends a control request to clear any previous communication settings.
    3. **Set Baud Rate**: Configures the communication speed to 115,200 baud.
    4. **Set Data Format**: Eight data bits, one stop bit, no parity.
    5. **Purge Buffers**: Pauses momentarily, then clears the `receive` and `transmit` buffers.
    6. **Flow Control Configuration**: Enables RTS/CTS (Request to Send / Clear to Send)
       flow control.
    7. **Set RTS High**: Activates the RTS (Ready to Send) signal to indicate host readiness
       for communication.

    # Errors

    Returns `Error::RUSB` if the underlying `libusb` operation encounters any form of error
    while initialising the serial port. See [rusb::DeviceHandle::write_bulk][1] for a more
    detailed explanation of the possible returned error variants.

    [1]: https://docs.rs/rusb/latest/rusb/struct.DeviceHandle.html#method.write_bulk
    */
    fn initialise_serial_port(&self) -> Result<(), Error> {
        self.rusb_device_handle.claim_interface(0)?;
        self.rusb_device_handle
            .write_control(0x40, 0x00, 0x0000, 0, &[], self.short_timeout)?;
        self.rusb_device_handle
            .write_control(0x40, 0x03, 0x001A, 0, &[], self.short_timeout)?;
        self.rusb_device_handle
            .write_control(0x40, 0x04, 0x0008, 0, &[], self.short_timeout)?;
        std::thread::sleep(Duration::from_millis(50));
        self.rusb_device_handle
            .write_control(0x40, 0x00, 0x0001, 0, &[], self.short_timeout)?;
        self.rusb_device_handle
            .write_control(0x40, 0x00, 0x0002, 0, &[], self.short_timeout)?;
        std::thread::sleep(Duration::from_millis(500));
        self.rusb_device_handle
            .write_control(0x40, 0x02, 0x0200, 0, &[], self.short_timeout)?;
        self.rusb_device_handle
            .write_control(0x40, 0x01, 0x0202, 0, &[], self.short_timeout)?;
        Ok(())
    }

    /**
    Writes a series of bytes to the USB device.

    This function takes an instance of the `MsgFormat` enum containing the data to be sent.
    This data is transferred to the USB device using a bulk write operation.

    # Errors

    - `Error::RUSB` if the underlying USB write operation encounters any form of error while
      fulfilling the transfer request.
      See [rusb::DeviceHandle::write_bulk][1] for a more detailed explanation of the possible
      returned error variants.

      [1]: https://docs.rs/rusb/latest/rusb/struct.DeviceHandle.html#method.write_bulk

    - `Error::FatalError` if the number of successfully written bytes does not match the
      expected data length, as this indicates a critical error in the `write` process that was not
      caught by `libusb` or `rusb`.

    # Examples

    ```rust
    let device = UsbDevice::new();
    let message = MsgFormat::Short(data);

    if let Err(e) = device.write(message) {
        println!("Failed to send message: {:?}", e);
    }
    ```
    */
    pub(crate) fn write(&self, data: &MsgFormat) -> Result<(), Error> {
        if self
            .rusb_device_handle
            .write_bulk(OUT_ENDPOINT, data, self.short_timeout)?
            != data.len()
        {
            return Err(Error::FatalError(format!(
                "Failed to write correct number of bytes to {}",
                self
            )));
        };
        Ok(())
    }
}

impl Display for UsbDevicePrimitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UsbDevicePrimitive (serial_number: {})",
            self.serial_number,
        )
    }
}

impl TryFrom<String> for UsbDevicePrimitive {
    type Error = Error;

    fn try_from(serial_number: String) -> Result<Self, Error> {
        if serial_number.len() != 8 || serial_number.parse::<i32>().unwrap_or(-1) <= 0 {
            return Err(Error::InvalidSerialNumber(serial_number));
        }
        get_usb_device_primitive(serial_number)
    }
}

impl TryFrom<&str> for UsbDevicePrimitive {
    type Error = Error;

    fn try_from(serial_number: &str) -> Result<Self, Error> {
        let sn = String::from(serial_number);
        if sn.len() != 8 || sn.parse::<i32>().unwrap_or(-1) <= 0 {
            return Err(Error::InvalidSerialNumber(sn));
        }
        get_usb_device_primitive(sn)
    }
}

impl TryFrom<i32> for UsbDevicePrimitive {
    type Error = Error;

    fn try_from(serial_number: i32) -> Result<Self, Error> {
        let sn = serial_number.to_string();
        if sn.len() != 8 || serial_number <= 0 {
            return Err(Error::InvalidSerialNumber(sn));
        }
        get_usb_device_primitive(sn)
    }
}
