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

use std::time::Duration;

use nusb::Interface;
use nusb::transfer::{ControlOut, ControlType, Recipient};
use smol::Timer;

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

/// Initializes serial port settings according to Thorlabs APT protocol requirements:
/// - Baud rate 115200
/// - Eight data bits
/// - One stop bit
/// - No parity
/// - RTS/CTS flow control
pub(super) async fn init(interface: &Interface) {
    let control_out = async |control_out: ControlOut| {
        interface
            .control_out(control_out)
            .await
            .status
            .expect("Control transfer failed");
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
