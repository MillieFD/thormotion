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

use serde::{Deserialize, Deserializer};
use std::collections::HashSet;

/**
Represents a single record (row) from the `messages.csv` file.

`messages.csv` contains the following columns:
- `name`: String containing an arbitrary message name.
- `id`: Unique message identifier, stored as a hexadecimal value.
- `length`: Semicolon-separated list of positive integers.
- `group`: Optional group name for linking `REQ` messages with their corresponding `GET` responses.

# Traits

The `Message` struct derives the `serde::Deserialize` trait to enable automatic parsing
of columns in the CSV file into matching struct fields. Custom deserialization internal
are used for the `id` and `length` fields:
- The `id` field is deserialized as a hexadecimal value and converted into a two-byte
  little-endian array.
- The `length` field is deserialized from a semicolon-separated string into a vector of
  unique integers. Duplicate lengths are removed during deserialization.

# Examples

Below is an example of how a message record might look in the `messages.csv` file, and
how it would deserialize into a `Message` struct:

```csv
name,id,length,group
TestMessage,0x1234,4;4;8;16,TestGroup
```

The above record would deserialize into the following `Message`:
```rust
Message {
    name: "TestMessage".to_string(),
    id: "[0x34, 0x12]".to_string(),
    length: vec![4, 8, 16],
    group: Some("TestGroup".to_string())
};
```
*/
#[derive(Deserialize, Hash, Eq, PartialEq)]
pub(super) struct Message {
    name: String,
    #[serde(deserialize_with = "deserialize_message_id")]
    pub(super) id: [u8; 2],
    #[serde(deserialize_with = "deserialize_message_length")]
    pub(super) length: Vec<usize>,
    pub(super) group: Option<String>,
}

/**
Parses the hexadecimal value from the `id` column of `messages.csv` into a two-byte
hexadecimal array in little-endian order and returns it as a formatted string.

For example, an ID like `0x1234` will be converted to the string
`"[0x34, 0x12]"`.

# Examples

```rust
use serde::de::{self, Deserializer};
use your_module::deserialize_message_id;

let deserialized: String = deserialize_message_id("0x1234")
    .unwrap(); // Simulate Deserializer integration
assert_eq!(deserialized, "[0x34, 0x12]");
```

# Errors

This function will return a deserialization error if the input value
is not a valid hexadecimal number or cannot be parsed into a
two-byte hex array (e.g. malformed `id` values).

# Panics

This function will not panic under normal operation. Invalid `id`
values will result in structured deserialization errors instead.
*/
fn deserialize_message_id<'de, D>(deserializer: D) -> Result<[u8; 2], D::Error>
where
    D: Deserializer<'de>,
{
    let id_hex: &str = Deserialize::deserialize(deserializer)?;
    let id_bytes: [u8; 2] = u16::from_str_radix(id_hex.trim_start_matches("0x"), 16)
        .map_err(|e| serde::de::Error::custom(format!("Invalid hex: {}", e)))?
        .to_le_bytes();
    Ok(id_bytes)
}

/**
Parses a semicolon-separated string of lengths from the `id` column of `messages.csv`
into a `Vec<usize>`.

This function is intended for deserializing data from the `length`
column of the `messages.csv` file.
The input string is split into substrings using a semicolon (`;`) as the delimiter,
and then each substring is parsed into a `usize` value.
To ensure uniqueness, the parsed values are initially stored in a `HashSet`,
which is subsequently converted into a `Vec` for compatibility with the `Record` struct.

# Errors

Returns a deserialization error if the input string cannot be split, trimmed, or
if any of the substrings fail to parse into a `usize`.

# Examples

```rust
use serde::de::{Deserializer, Error as SerdeError};
use std::collections::HashSet;

fn deserialize_message_length<'de, D>(deserializer: D) -> Result<Vec<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    let lengths_str: &str = Deserialize::deserialize(deserializer)?;
    let lengths_set: HashSet<usize> = lengths_str
        .split(';')
        .map(|s| s.trim().parse::<usize>().map_err(SerdeError::custom))
        .collect::<Result<HashSet<usize>, _>>()?;
    let lengths_vec: Vec<usize> = lengths_set.into_iter().collect();
    Ok(lengths_vec)
}
```
*/
fn deserialize_message_length<'de, D>(deserializer: D) -> Result<Vec<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    let lengths_str: &str = Deserialize::deserialize(deserializer)?;
    let lengths_set: HashSet<usize> = lengths_str
        .split(';')
        .map(|s| s.trim().parse::<usize>().map_err(serde::de::Error::custom))
        .collect::<Result<HashSet<usize>, _>>()?;
    let lengths_vec: Vec<usize> = lengths_set.into_iter().collect();
    Ok(lengths_vec)
}
