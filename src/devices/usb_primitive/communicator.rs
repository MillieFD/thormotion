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

use nusb::Interface;
use nusb::transfer::{Queue, RequestBuffer};
use smol::Task;

use crate::devices::usb_primitive::serial_port::init;
use crate::error::Error;
use crate::messages::Dispatcher;

const BUFFER_SIZE: usize = 255 + 6;

/**
Handles all incoming and outgoing messages between the host and a specific USB [`Interface`].
*/
pub(super) struct Communicator {
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
    pub(super) async fn new(interface: Interface, dispatcher: Dispatcher) -> Self {
        const OUT_ENDPOINT: u8 = 0x02;
        init(&interface).await;
        let incoming = Self::spawn(&interface, dispatcher);
        let outgoing = interface.interrupt_out_queue(OUT_ENDPOINT);
        Self {
            interface,
            incoming,
            outgoing,
        }
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
                dispatcher.dispatch(completion.data).await;
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
