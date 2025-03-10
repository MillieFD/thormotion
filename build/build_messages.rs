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

use super::{Message, utils};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::process::Command;

/**
Builds static `phf::HashMap` instances from `messages.csv`.

This function reads data from the `messages.csv` file, processes it, and generates static
`phf::HashMap` instances representing message length mappings and message group mappings.
The generated code is written to the `built_messages.rs` file, which is placed in the
build output directory specified by the environment variable `OUT_DIR`. This generated
file is then included in the `src/messages.rs` file to provide runtime access to the
precomputed mappings.

# Examples

Build the static `phf::HashMap` instances by calling `build_messages()` during the build process.

```rust
build_messages()
```

Then include the generated code in the `src/messages.rs` file

```rust
include!(concat!(env!("OUT_DIR"), "/built_messages.rs"));
```

# Panics

This function panics if:

- The `messages.csv` file cannot be found or fails to parse.
- Writing to the `built_messages.rs` file fails.
- The `rustfmt` command encounters an error when formatting the output file.

# Errors

If the `messages.csv` file has incorrect formatting (e.g. missing columns or corrupt data),
the generated file may contain incorrect mappings or may fail to compile.

When the `format-built-files` feature is enabled, `rustfmt` will format the generated code
to make reading and debugging the file easier.
This behaviour can cause a compiler error if `rustfmt` is not installed.
*/
pub(super) fn build_messages() {
    let messages: Vec<Message> = utils::from_csv("messages.csv");
    let mut groups_map = HashMap::new();
    let mut length_map_entries = phf_codegen::Map::new();
    let mut sender_map_entries = phf_codegen::Map::new();
    let mut channels = String::new();
    for message in messages {
        if let [length] = message.length.as_slice() {
            length_map_entries.entry(message.id, &length.to_string());
        }
        if let Some(group) = message.group {
            groups_map
                .entry(group)
                .or_insert(HashSet::new())
                .insert(message.id);
        }
    }
    for (group, ids) in groups_map {
        let template = utils::get_template("static_message_channel.txt")
            .replace("TEMPLATE_CHANNEL_NAME", &group);
        channels.push_str(&template);
        for id in ids {
            sender_map_entries.entry(id, &format!("&{}", group));
        }
    }
    let template = utils::get_template("messages_template.txt")
        .replace(
            "LENGTH_MAP_ENTRIES",
            &length_map_entries.build().to_string(),
        )
        .replace(
            "CHANNEL_MAP_ENTRIES",
            &sender_map_entries.build().to_string(),
        )
        .replace("// CHANNELS", &channels);
    let (mut file, path) = utils::get_output_file("built_messages.rs");
    writeln!(file, "{}", template).expect(&format!("Failed to write to {}", path.display()));
    #[cfg(feature = "format-built-files")]
    Command::new("rustfmt").arg(path).status().unwrap(); // Format the generated file
}
