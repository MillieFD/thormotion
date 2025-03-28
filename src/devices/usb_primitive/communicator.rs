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

use crate::error::Error;
use crate::messages::Dispatcher;
use nusb::transfer::{ControlOut, ControlType, Queue, Recipient, RequestBuffer};
use nusb::Interface;
use smol::future::FutureExt;
use smol::{Task, Timer};
use std::time::Duration;

const BUFFER_SIZE: usize = 255 + 6;
const RESET_CONTROLLER: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x00,
    value: 0x0000,
    index: 0,
    data: &[],
};
const BAUD_RATE: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x03,
    value: 0x001A,
    index: 0,
    data: &[],
};

const EIGHT_DATA_ONE_STOP_NO_PARITY: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x04,
    value: 0x0008,
    index: 0,
    data: &[],
};

const PURGE_RX: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x00,
    value: 0x0001,
    index: 0,
    data: &[],
};

const PURGE_TX: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x00,
    value: 0x0002,
    index: 0,
    data: &[],
};

const FLOW_CONTROL: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x02,
    value: 0x0200,
    index: 0,
    data: &[],
};

const RTS: ControlOut = ControlOut {
    control_type: ControlType::Vendor,
    recipient: Recipient::Device,
    request: 0x01,
    value: 0x0202,
    index: 0,
    data: &[],
};

/**
Handles all incoming and outgoing messages between the host and a specific USB [`Interface`].
*/
pub(crate) struct Communicator {
    /**
    A claimed open [`Interface`] for communicating with the USB device.
    */
    interface: Interface,
    /**
    An async background task that handles a stream of incoming messages from the USB [`Interface`].
    */
    incoming: Task<()>,
    /**
    A [`Queue`] that handles a stream of outgoing messages to the USB [`Interface`].
    */
    outgoing: Queue<Vec<u8>>,
}

impl Communicator {
    pub(super) async fn new(interface: Interface, dispatcher: Dispatcher) -> Result<Self, Error> {
        const OUT_ENDPOINT: u8 = 0x02;
        Self::init(&interface).await;
        let incoming = Self::spawn(&interface, dispatcher);
        let outgoing = interface.interrupt_out_queue(OUT_ENDPOINT);
        let communicator = Self {
            interface,
            incoming,
            outgoing,
        };
        Ok(communicator)
    }

    /**
    Initializes serial port settings according to Thorlabs APT protocol requirements:
    - Baud rate 115200
    - Eight data bits
    - One stop bit
    - No parity
    - RTS/CTS flow control
    */
    async fn init(interface: &Interface) {
        let mut i = 0;
        let mut control_out = async |control_out: ControlOut| {
            interface
                .control_out(control_out)
                .or(async {
                    Timer::after(Duration::from_millis(50));
                    panic!("Control transfer {i} timed out after 50ms [communicator.rs:172]")
                })
                .await
                .status
                .expect("Control transfer failed [communicator.rs:176]");
            i += 1;
        };
        control_out(RESET_CONTROLLER).await;
        control_out(BAUD_RATE).await;
        control_out(EIGHT_DATA_ONE_STOP_NO_PARITY).await;
        Timer::after(Duration::from_millis(50)).await; // Pre-purge dwell 50ms
        control_out(PURGE_RX).await;
        control_out(PURGE_TX).await;
        Timer::after(Duration::from_millis(50)).await; // Post-purge dwell 50ms
        control_out(FLOW_CONTROL).await;
        control_out(RTS).await;
    }

    fn handle_error(error: Error) {
        match error {
            _ => panic!("{}", error),
        }
    }

    fn spawn(interface: &Interface, dispatcher: Dispatcher) -> Task<()> {
        const IN_ENDPOINT: u8 = 0x81;
        let mut queue = interface.interrupt_in_queue(IN_ENDPOINT);

        let mut listen = async move || -> Result<(), Error> {
            loop {
                queue.submit(RequestBuffer::new(BUFFER_SIZE));
                let completion = queue.next_complete().await;
                completion.status?;
                dispatcher.dispatch(completion.data).await?;
            }
        };

        smol::spawn(async move {
            if let Err(error) = listen().await {
                Self::handle_error(error);
            }
        })
    }

    fn send(&mut self, message: Vec<u8>) {
        self.outgoing.submit(message);
    }
}
