/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    DeviceClosed,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DeviceClosed => write!(f, "Cannot send command to closed device"),
        }
    }
}

impl std::error::Error for Error {}
