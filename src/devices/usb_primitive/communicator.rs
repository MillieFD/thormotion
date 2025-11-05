/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::time::Duration;

use nusb::transfer::{Buffer, Bulk, In, Out, TransferError};
use nusb::{Endpoint, Interface};
use smol::Task;
use smol::lock::Mutex;

use super::serial_port;
use crate::devices::abort;
use crate::messages::{CMD_LEN_MAX, Dispatcher};

/// The USB endpoint used for incoming commands from the device
const IN_ENDPOINT: u8 = 0x81;
/// The number of concurrent transfers to maintain in the queue
const N_TRANSFERS: usize = 3;
/// The USB endpoint used for outgoing commands to the device
const OUT_ENDPOINT: u8 = 0x02;
/// Background task polling interval
const TIME: Duration = Duration::from_millis(10);

/// Handles all incoming and outgoing commands between the host and a specific USB [`Interface`].
pub(super) struct Communicator<const CH: usize> {
    /// A thread-safe message [`Dispatcher`] for handling async `Req → Get` callback patterns.
    dispatcher: Dispatcher<CH>,
    /// An async background task that handles a stream of incoming commands from the [`Interface`].
    #[allow(unused)]
    incoming: Task<()>,
    /// An [`outgoing`][1] [`Bulk`][2] [`Endpoint`][3] for sending commands to the USB
    /// [`Interface`]. Protected by a [`Mutex`] for async access.
    ///
    /// [1]: Out
    /// [2]: Bulk
    /// [3]: Endpoint
    pub(super) outgoing: Mutex<Endpoint<Bulk, Out>>,
}

impl<const CH: usize> Communicator<CH> {
    /// Creates a new [`Communicator`] instance for the specified USB [`Interface`].
    pub(super) async fn new(interface: Interface, dispatcher: Dispatcher<CH>) -> Self {
        log::debug!("{dispatcher} COMMUNICATOR::NEW (requested)");
        serial_port::init(&interface).await;
        let dsp = dispatcher.clone(); // Inexpensive Arc Clone
        let endpoint = interface
            .endpoint(OUT_ENDPOINT)
            .unwrap_or_else(|e| abort(format!("Failed to open OUT endpoint : {e}")));
        let outgoing = Mutex::new(endpoint);
        let incoming = Self::spawn(interface, dsp);
        log::debug!("{dispatcher} COMMUNICATOR::NEW (success)");
        Self {
            dispatcher,
            incoming,
            outgoing,
        }
    }

    /// Handles any [`TransferError`] returned from the [`incoming task`][Self::spawn].
    // NOTE: If the incoming task terminates, it cannot be restarted without the [`Interface`].
    // Automatic recovery may be implemented in a future version of Thormotion.
    fn handle_error(error: TransferError) {
        // NOTE: Currently, all errors cause the program to abort. A `match` statement allows
        // TransferError variants to be handled differently if required.
        match error {
            _ => abort(format!("Background task error : {}", error)),
        }
    }

    /// Spawns an async background task that handles a stream of incoming commands from the
    /// [`Interface`].
    ///
    /// The task loops indefinitely until either:
    /// 1. It is explicitly [`cancelled`][Task::cancel]
    /// 2. The [`Communicator`] is dropped
    /// 3. A [`TransferError`] occurs. See [`Self::handle_error`].
    fn spawn(interface: Interface, dispatcher: Dispatcher<CH>) -> Task<()> {
        log::debug!("{dispatcher} SPAWN (requested)");
        let mut endpoint: Endpoint<Bulk, In> = interface
            .endpoint(IN_ENDPOINT)
            .unwrap_or_else(|e| abort(format!("Failed to open IN endpoint : {e}")));
        // Buffer size must be a nonzero multiple of the endpoint's maximum packet size
        let pkt_size = endpoint.max_packet_size();
        let buf_size = CMD_LEN_MAX.div_ceil(pkt_size) * pkt_size;
        while endpoint.pending() < N_TRANSFERS {
            endpoint.submit(Buffer::new(buf_size));
        }
        let mut queue: VecDeque<u8> = VecDeque::with_capacity(N_TRANSFERS * CMD_LEN_MAX);
        let mut id = [0u8; 2]; // Reusable ID buffer
        let mut listen = async move || -> Result<(), TransferError> {
            log::debug!("{dispatcher} SPAWN (starting background task)");
            loop {
                smol::Timer::after(TIME).await;
                let mut completion = endpoint.next_complete().await;
                if completion.actual_len > 2 {
                    completion.status?;
                    log::trace!(
                        "BACKGROUND {} RECEIVED {:02X?}",
                        dispatcher.serial_number(),
                        &completion.buffer[2..],
                    );
                    let iter = completion.buffer.iter().skip(2); // Skip prefix bytes
                    queue.extend(iter); // Copy u8 bytes into queue ring buffer
                    while queue.get(5).is_some() {
                        id[0] = queue[0]; // Copying is more efficient than borrowing for u8
                        id[1] = queue[1]; // Copied bytes remain in queue
                        log::trace!(
                            "BACKGROUND {} MESSAGE ID {:02X?}",
                            dispatcher.serial_number(),
                            id
                        );
                        let len = dispatcher.length(&id).await;
                        if queue.len() < len {
                            log::trace!(
                                "BACKGROUND {} INCOMPLETE (waiting) QUEUE {} REQUIRE {}",
                                dispatcher.serial_number(),
                                queue.len(),
                                len,
                            );
                            break;
                        }
                        let msg = queue.drain(..len).collect();
                        log::trace!(
                            "BACKGROUND {} DISPATCH {:02X?}",
                            dispatcher.serial_number(),
                            msg
                        );
                        dispatcher.dispatch(msg, CH).await;
                    }
                }
                completion.buffer.clear(); // Clear the buffer for reuse
                endpoint.submit(completion.buffer); // Resubmit buffer to endpoint
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
        log::trace!("{self} SEND (requested) {command:02X?}");
        let buf = Buffer::from(command);
        self.outgoing.lock().await.submit(buf);
        log::trace!("{self} SEND (success)");
    }

    /// Returns the [`Dispatcher`] wrapped in an [`Arc`][std::sync::Arc].
    pub(super) fn get_dispatcher(&self) -> Dispatcher<CH> {
        self.dispatcher.clone() // Inexpensive Arc Clone
    }
}

impl<const CH: usize> Debug for Communicator<CH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "COMMUNICATOR {{ {} }}", self.dispatcher)
    }
}

impl<const CH: usize> Display for Communicator<CH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "COMMUNICATOR {}", self.dispatcher.serial_number())
    }
}
