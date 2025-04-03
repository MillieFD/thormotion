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

use nusb::transfer::{Queue, RequestBuffer, TransferError};
use nusb::Interface;
use smol::future::yield_now;
use smol::lock::Mutex;
use smol::Task;

use crate::devices::usb_primitive::serial_port::init;
use crate::devices::{global_abort, BUG};
use crate::messages::Dispatcher;

// SAFETY: Currently, no data packet exceeds 255 bytes (Thorlabs APT Protocol, Issue 38, Page 35).
// The max possible command length is therefore six-bytes (header) plus 255 bytes (data packet).
/// The maximum possible size for a Thorlabs APT command
const BUFFER_SIZE: usize = 255 + 6;

/// Handles all incoming and outgoing commands between the host and a specific USB [`Interface`].
pub(super) struct Communicator {
    /// A thread-safe message [`Dispatcher`] for handling async `Req → Get` callback patterns.
    dispatcher: Dispatcher,
    /// An async background task that handles a stream of incoming commands from the [`Interface`].
    #[allow(unused)]
    incoming: Task<()>,
    ///A [`Queue`] that handles a stream of outgoing commands to the USB [`Interface`].
    pub(super) outgoing: Mutex<Queue<Vec<u8>>>,
}

impl Communicator {
    pub(super) async fn new(interface: Interface, dispatcher: Dispatcher) -> Self {
        const OUT_ENDPOINT: u8 = 0x02;
        init(&interface).await;
        let dsp = dispatcher.clone(); // Inexpensive Arc Clone
        let outgoing = Mutex::new(interface.interrupt_out_queue(OUT_ENDPOINT));
        let incoming = Self::spawn(interface, dsp);
        Self {
            dispatcher,
            incoming,
            outgoing,
        }
    }

    /// Handles any [`TransferError`] returned from the [`incoming task`][Self::spawn].
    ///
    /// A `match` statement allows [`TransferError`] variants to be handled differently if required.
    ///
    /// If the incoming task terminates, it cannot be restarted without the [`Interface`].
    /// Automatic recovery may be implemented in a future version of Thormotion.
    fn handle_error(error: TransferError) {
        match error {
            _ => global_abort(format!("Background task error : {}", error)),
        }
    }

    /// Spawns an async background task that handles a stream of incoming commands from the
    /// [`Interface`].
    ///
    /// The task loops indefinitely until either:
    /// 1. It is explicitly [`cancelled`][Task::cancel]
    /// 2. The [`Communicator`] is dropped
    /// 3. A [`TransferError`] occurs. See [`Self::handle_error`].
    fn spawn(interface: Interface, dispatcher: Dispatcher) -> Task<()> {
        const IN_ENDPOINT: u8 = 0x81;
        let mut queue = interface.interrupt_in_queue(IN_ENDPOINT);

        let mut listen = async move || -> Result<(), TransferError> {
            loop {
                queue.submit(RequestBuffer::new(BUFFER_SIZE));
                let mut completion = queue.next_complete().await;
                match completion.data.len() {
                    ..2 => global_abort(format!(
                        "Received {}-byte command from USB device\n{}",
                        completion.data.len(),
                        BUG
                    )),
                    2 => {} // Command contains framing bytes only. Proceed to the yield point.
                    3.. => {
                        completion.status?;
                        completion.data.drain(..2); // Drop the framing bytes
                        dispatcher.dispatch(completion.data).await;
                    }
                }
                yield_now().await;
            }
        };

        smol::spawn(async move {
            if let Err(error) = listen().await {
                Self::handle_error(error);
            }
        })
    }

    /// Send a command to the device [`Interface`].
    pub(super) async fn send(&self, command: Vec<u8>) {
        self.outgoing.lock().await.submit(command);
    }

    /// Returns the [`Dispatcher`] wrapped in an [`Arc`][std::sync::Arc].
    pub(super) fn get_dispatcher(&self) -> Dispatcher {
        self.dispatcher.clone() // Inexpensive Arc Clone
    }
}
