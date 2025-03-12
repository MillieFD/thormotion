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

macro_rules! apt_doc {
    ($sync_async:ident, $SET_COMMAND:literal, $REQ_COMMAND:literal, $GET_COMMAND:literal $(, $error:ident)*) => {
        concat!(
            set_req_get_doc!($SET_COMMAND, $REQ_COMMAND, $GET_COMMAND),
            sync_async_doc!($sync_async, $SET_COMMAND),
            errors_doc!($($error),*)
        )
    };
    ($sync_async:ident, $REQ_COMMAND:literal, $GET_COMMAND:literal $(, $error:ident)*) => {
        concat!(
            set_req_get_doc!($REQ_COMMAND, $GET_COMMAND),
            sync_async_doc!($sync_async, $REQ_COMMAND),
            errors_doc!($($error),*)
        )
    };
    ($sync_async:ident, $COMMAND:literal $(, $error:ident)*) => {
        concat!(
            set_req_get_doc!($COMMAND),
            sync_async_doc!($sync_async, $COMMAND),
            errors_doc!($($error),*)
        )
    };
}

macro_rules! set_req_get_doc {
    ($SET_COMMAND:literal, $REQ_COMMAND:literal, $GET_COMMAND:literal) => {
        concat!(
            "\n\n# Thorlabs APT Protocol\n\n",
            "This function follows a three-stage `Set → Req → Get` pattern\n\n",
            "1. `Set`: Thormotion sends the `",
            $SET_COMMAND,
            "` message to the device.\n\n",
            "2. `Req`: Thormotion then sends the `",
            $REQ_COMMAND,
            "` message to request information from the device.\n\n",
            "3. `Get`: The device sends the `",
            $GET_COMMAND,
            "` message in response. Thormotion checks this response to ensure that `",
            $SET_COMMAND,
            "` was correctly executed.\n\n",
        )
    };
    ($REQ_COMMAND:literal, $GET_COMMAND:literal) => {
        concat!(
            "\n\n# Thorlabs APT Protocol\n\n",
            "This function follows a two-stage `Req → Get` pattern\n\n",
            "1. `Req`: Thormotion sends the `",
            $REQ_COMMAND,
            "` message to the device.\n\n",
            "2. `Get`: The device sends the `",
            $GET_COMMAND,
            "` message in response. If necessary, Thormotion checks this response to ensure that `",
            $REQ_COMMAND,
            "` was correctly executed.\n\n",
        )
    };
    ($COMMAND:literal) => {
        concat!(
            "\n\n# Thorlabs APT Protocol\n\n",
            "Thormotion sends the `",
            $COMMAND,
            "` message to the device. The device does not send any response message.\n\n",
        )
    };
}

macro_rules! sync_async_doc {
    (sync, $COMMAND:literal) => {
        concat!(
            "\n\n# Sync\n\n",
            "This is the synchronous (blocking) version of `",
            $COMMAND,
            "`. An asynchronous (non-blocking) version is also available.\n\n",
        )
    };
    (async, $COMMAND:literal) => {
        concat!(
            "\n\n# Async\n\n",
            "This is the asynchronous (non-blocking) version of `",
            $COMMAND,
            "`. A synchronous (blocking) version is also available.\n\n",
        )
    };
}

macro_rules! errors_doc {
    ($($error:ident),*) => {
        concat!(
            "\n\n# Errors\n\n",
            errors_doc!(@inner $($error),*),
            "\n\n"
        )
    };
    (@inner RUSB $(, $rest:ident)*) => {
        concat!(
            "- `Error::RUSB` if the underlying USB write operation encounters any form of error \
            while fulfilling the transfer request. See [rusb::DeviceHandle::write_bulk]\
            (https://docs.rs/rusb/latest/rusb/struct.DeviceHandle.html#method.write_bulk) \
            for a more detailed explanation of the possible returned error variants.\n\n",
            errors_doc!(@inner $($rest),*)
        )
    };
    (@inner InvalidSerialNumber $(, $rest:ident)*) => {
        concat!(
            "- `Error::InvalidSerialNumber` if the provided serial number is not valid \
            for the specified Thorlabs device type.\n\n",
            errors_doc!(@inner $($rest),*)
        )
    };
    (@inner DeviceNotFound $(, $rest:ident)*) => {
        concat!(
            "- `Error::NoDevice` if no Thorlabs devices with the specified serial number \
            could be found.\n\n",
            errors_doc!(@inner $($rest),*)
        )
    };
    (@inner MultipleDevicesFound $(, $rest:ident)*) => {
        concat!(
            "- `Error::ManyDevices` if multiple conflicting Thorlabs devices with the \
            specified serial number were found.\n\n",
            errors_doc!(@inner $($rest),*)
        )
    };
    (@inner Timeout $(, $rest:ident)*) => {
        concat!(
            "- `Error::Timeout` if the timeout elapses while waiting for a response \
            from the device.\n\n",
            errors_doc!(@inner $($rest),*)
        )
    };
    (@inner FatalError $(, $rest:ident)*) => {
        concat!(
            "- `Error::FatalError` if an unrecoverable error occurs. Thormotion \
            includes internal mechanisms to prevent this from occurring. If you \
            encounter this error, Please open a new [GitHub issue]\
            (https://github.com/MillieFD/thormotion/issues) and provide the relevant details.\n\n",
            errors_doc!(@inner $($rest),*)
        )
    };
    (@inner AptUnsuccessful $(, $rest:ident)*) => {
        concat!(
            "- `Error::AptUnsuccessful` if the device failed to execute the APT protocol \
            command and returned an unexpected result.\n\n",
            errors_doc!(@inner $($rest),*)
        )
    };
    (@inner) => { "" };
}

pub(crate) use apt_doc;
pub(crate) use errors_doc;
pub(crate) use set_req_get_doc;
pub(crate) use sync_async_doc;
