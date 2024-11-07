/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: todo
Description: todo
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::env::{BUFFER_SIZE, IN_ENDPOINT, OUT_ENDPOINT, READ_INTERVAL, TIMEOUT};
use crate::errors::error_types::Error;
use crate::messages::all_messages::ALL_MESSAGES;
use rusb::{DeviceDescriptor, DeviceHandle, GlobalContext, Language};
use std::process;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::oneshot::error::TryRecvError;
use tokio::sync::oneshot::Receiver;
use tokio::sync::oneshot::{channel, Sender};

#[derive(Debug)]
pub(crate) struct UsbDevicePrimitive {
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
        dev.init_port()?;
        dev.start_listening(shutdown_rx)?;
        Ok(dev)
    }

    fn init_port(&self) -> Result<(), Error> {
        // Claim the interface
        self.handle.claim_interface(0)?;

        // Reset the device
        self.handle
            .write_control(0x40, 0x00, 0x0000, 0, &[], TIMEOUT)?;

        // Set baud rate (115200)
        self.handle
            .write_control(0x40, 0x03, 0x001A, 0, &[], TIMEOUT)?;

        // Set data format (8 data bits, 1 stop bit, no parity)
        self.handle
            .write_control(0x40, 0x04, 0x0008, 0, &[], TIMEOUT)?;

        // Pre-purge dwell
        sleep(Duration::from_millis(50));

        // Purge receive buffer
        self.handle
            .write_control(0x40, 0x00, 0x0001, 0, &[], TIMEOUT)?;

        // Purge transmit buffer
        self.handle
            .write_control(0x40, 0x00, 0x0002, 0, &[], TIMEOUT)?;

        // Post-purge dwell
        sleep(Duration::from_millis(500));

        // Set flow control (RTS/CTS)
        self.handle
            .write_control(0x40, 0x02, 0x0200, 0, &[], TIMEOUT)?;

        // Set RTS high
        self.handle
            .write_control(0x40, 0x01, 0x0202, 0, &[], TIMEOUT)?;

        Ok(())
    }

    pub(crate) fn write_port(&self, data: Box<[u8]>) -> Result<Receiver<Box<[u8]>>, Error> {
        let id: u16 = u16::from_le_bytes(data[..2].try_into().unwrap());
        let group = ALL_MESSAGES.get_group_by_id(id)?;
        if group.waiting_sender.read()?.is_some() {
            return Err(Error::OneshotSenderExists(self.serial_number.clone(), data));
        }
        let (tx, rx) = channel();
        group.waiting_sender.write()?.replace(tx);
        if data.len() == self.handle.write_bulk(OUT_ENDPOINT, &data, TIMEOUT)? {
            Ok(())
        } else {
            Err(Error::DeviceMessageSendFailure(
                self.serial_number.clone(),
                data,
            ))
        }?;
        Ok(rx)
    }

    fn start_listening(&self, mut shutdown_rx: Receiver<()>) -> Result<(), Error> {
        let handle = self.handle.clone();
        tokio::spawn(async move {
            loop {
                if shutdown_rx.try_recv() != Err(TryRecvError::Empty) {
                    break;
                }
                if let Err(e) = handle.read_port().await {
                    // todo use this error "e"
                    process::exit(1)
                }
            }
        });
        Ok(())
    }
}

trait ReadPort {
    async fn read_port(&self) -> Result<(), Error>;
}

impl ReadPort for DeviceHandle<GlobalContext> {
    async fn read_port(&self) -> Result<(), Error> {
        let mut buffer = [0u8; BUFFER_SIZE];
        let num_bytes_read = self.read_bulk(IN_ENDPOINT, &mut buffer, TIMEOUT)?;
        if num_bytes_read == 2 {
            tokio::time::sleep(READ_INTERVAL).await;
            return Ok(());
        }
        let message: Box<[u8]> = Box::from(&buffer[2..num_bytes_read - 2]);
        let id: u16 = u16::from_le_bytes([message[0], message[1]]);
        if let Some(sender) = ALL_MESSAGES
            .get_group_by_id(id)?
            .waiting_sender
            .write()?
            .take()
        {
            sender.send(message)?;
        };
        tokio::time::sleep(READ_INTERVAL).await;
        Ok(())
    }
}
