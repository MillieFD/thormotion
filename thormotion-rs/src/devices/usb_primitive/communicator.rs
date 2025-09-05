/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::collections::VecDeque;

use log::{debug, info, trace};
use nusb::Interface;
use nusb::transfer::{Queue, RequestBuffer, TransferError};
use smol::Task;
use smol::lock::Mutex;

use super::serial_port;
use crate::devices::abort;
use crate::messages::{CMD_LEN_MAX, Dispatcher};

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
    /// Creates a new [`Communicator`] instance for the specified USB [`Interface`].
    pub(super) async fn new(interface: Interface, dispatcher: Dispatcher) -> Self {
        /// The USB endpoint used for outgoing commands to the device
        const OUT_ENDPOINT: u8 = 0x02;
        trace!("Initialising communicator");
        serial_port::init(&interface).await;
        let dsp = dispatcher.clone(); // Inexpensive Arc Clone
        let outgoing = Mutex::new(interface.bulk_out_queue(OUT_ENDPOINT));
        let incoming = Self::spawn(interface, dsp);
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
    fn spawn(interface: Interface, dispatcher: Dispatcher) -> Task<()> {
        /// The USB endpoint used for incoming commands from the device
        const IN_ENDPOINT: u8 = 0x81;
        /// The number of concurrent transfers to maintain in the queue
        const N_TRANSFERS: usize = 3;
        trace!("Initialising incoming USB endpoint");
        let mut endpoint = interface.bulk_in_queue(IN_ENDPOINT);
        while endpoint.pending() < N_TRANSFERS {
            endpoint.submit(RequestBuffer::new(CMD_LEN_MAX));
        }
        let mut queue: VecDeque<u8> = VecDeque::with_capacity(N_TRANSFERS * CMD_LEN_MAX);
        let mut id = [0u8; 2]; // Reusable ID buffer
        let mut listen = async move || -> Result<(), TransferError> {
            trace!("Starting background 'listen' task");
            loop {
                let completion = endpoint.next_complete().await;
                if completion.data.len() > 2 {
                    completion.status?;
                    debug!("Received command {:02X?}", &completion.data[2..],);
                    queue.extend(&completion.data[2..]); // Drop prefix bytes

                    while queue.get(5).is_some() {
                        id[0] = queue[0]; // Copying is more efficient than borrowing for u8
                        id[1] = queue[1]; // Copied bytes remain in queue
                        info!("Message ID : {:02X?}", id);
                        let len = dispatcher.length(&id).await;
                        if queue.len() < len {
                            info!("Incomplete command. Waiting for more data.");
                            trace!(
                                "Queue is {} bytes | Message needs {} bytes",
                                queue.len(),
                                len
                            );
                            break;
                        }
                        let msg = queue.drain(..len).collect();
                        debug!("Dispatching command {:02X?}", msg);
                        dispatcher.dispatch(msg).await;
                    }
                }
                endpoint.submit(RequestBuffer::reuse(completion.data, CMD_LEN_MAX));
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
        debug!("Sending command {:02X?}", command);
        self.outgoing.lock().await.submit(command);
    }

    /// Returns the [`Dispatcher`] wrapped in an [`Arc`][std::sync::Arc].
    pub(super) fn get_dispatcher(&self) -> Dispatcher {
        self.dispatcher.clone() // Inexpensive Arc Clone
    }
}
