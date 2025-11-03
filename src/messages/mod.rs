/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

/* ------------------------------------------------------------------------------ Public Modules */

pub(crate) mod utils;

/* ----------------------------------------------------------------------------- Private Modules */

mod command;
mod dispatcher;
mod provenance;

/* --------------------------------------------------------------------------- Public Re-Exports */

/// A sender for broadcasting command responses to multiple receivers.
pub type Sender = async_broadcast::Sender<std::sync::Arc<[u8]>>;

/// A receiver for listening to command responses from a sender.
pub type Receiver = async_broadcast::Receiver<std::sync::Arc<[u8]>>;

/* -------------------------------------------------------------------------- Private Re-Exports */

pub(crate) use command::*;
pub(crate) use dispatcher::*;
pub(crate) use provenance::*;
