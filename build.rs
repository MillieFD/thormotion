/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License

Copyright (c) 2025, Amelia Fraser-Dale

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
use std::collections::{HashMap, HashSet};
use std::env::var_os;
use std::error::Error;
use std::fs::{read_to_string, File};
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// # Message
/// The `Message` struct represents a single record (row) in the `messages.csv` file.
/// It contains the ID, length, and group names for a given message.
/// The struct implements the `serde::Deserialize` trait,
/// which automatically parses columns in the CSV file to struct fields with the same name.
/// Specific deserialization templates are provided where needed.
#[derive(Deserialize, Hash, Eq, PartialEq)]
struct Message {
    name: String,
    #[serde(deserialize_with = "deserialize_message_id")]
    id: String,
    #[serde(deserialize_with = "deserialize_message_length")]
    length: Vec<usize>,
    group: Option<String>,
}

/// # Deserialize Message ID Function
/// This custom deserialization function parses hexadecimal values from the messages.csv `id`
/// column into two-byte hexadecimal arrays in little-endian order.
/// For example, the ID `0x1234` will be parsed into the array `[0x34, 0x12]`.
fn deserialize_message_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let id_hex: &str = Deserialize::deserialize(deserializer)?;
    let array: [u8; 2] = u16::from_str_radix(id_hex.trim_start_matches("0x"), 16)
        .map_err(|e| serde::de::Error::custom(format!("Invalid hex: {}", e)))?
        .to_le_bytes();
    let id_bytes = format!("[0x{:02x}, 0x{:02x}]", array[0], array[1]);
    Ok(id_bytes)
}

/// # Deserialize Message Length Function
/// This custom deserialization function parses a semicolon-separated string of lengths from
/// the messages.csv `length` column into a set of unique `usize` values.
/// A `HashSet` is used initially to ensure uniqueness, and is then converted into a `Vec`
/// to simplify later usage in the `Record` struct.
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

/// # Device
/// The `Device` struct represents a single record (row) in the `devices.csv` file.
/// It contains the name and available templates for a given device.
/// The struct implements the `serde::Deserialize` trait,
/// which automatically parses columns in the CSV file to struct fields with the same name.
/// Specific deserialization templates are provided where needed.
#[derive(Deserialize)]
struct Device {
    name: String,
    serial_number_prefix: String,
    identify: bool,
    channel_enable_state: bool,
    distance_angle_scale_factor: Option<f64>,
    velocity_scale_factor: Option<f64>,
    acceleration_scale_factor: Option<f64>,
    home: bool,
    move_absolute: bool,
}

/// # Equality Testing for Device
/// The `PartialEq` and `Eq` traits are implemented for the `Device` struct.
/// These are required for the `Hash` trait implementation below.
impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Device {}

/// # Hash for Device
/// The `Hash` trait is implemented for the `Device`
/// struct to enable storing `Device` instances in a `HashSet`.
/// The `hash()` function compares `Device` instances using the device.csv `name` column
/// A `HashSet` is used to ensure uniqueness,
/// panicking if the `devices.csv` file contains duplicate rows.
impl Hash for Device {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

/// # Build Devices
/// The `build_devices()` function reads the `devices.csv` file and generates structs which
/// each implements their respective required functions.
/// The generated code is written to the `OUT_DIR/devices.rs` file in the `OUT_DIR` directory.
/// The contents of `OUT_DIR/devices.rs` are then included in the `src/devices.rs` file.
fn build_devices() -> Result<(), Box<dyn Error>> {
    let out_dir = var_os("OUT_DIR").ok_or("OUT_DIR not set")?;
    let mut file = File::create(Path::new(&out_dir).join("devices_built.rs"))?;

    let mut reader = csv::Reader::from_path("devices.csv")?;
    let devices = reader
        .deserialize()
        .collect::<Result<HashSet<Device>, csv::Error>>()?;
    if devices.len() < reader.into_records().count() {
        panic!("WARNING: devices.csv contains duplicate rows");
    }

    writeln!(file, "{}", read_to_string("templates/dependencies.txt")?)?;

    for device in &devices {
        let mut internal_functions = String::new();
        let mut external_functions = String::new();

        // Define `check_serial_number()` function
        let template = read_to_string("templates/check_serial_number.rs")?;
        internal_functions.push_str(&template);

        // Define `distance_angle_scale_factor` and conversion functions if needed
        if let Some(f) = device.distance_angle_scale_factor {
            let template = read_to_string("templates/distance_conversion.rs")?
                .replace("template_scale_factor", f.to_string().as_str());
            internal_functions.push_str(&template);
        }

        // Define `velocity_scale_factor` and conversion functions if needed
        if let Some(f) = device.velocity_scale_factor {
            let template = read_to_string("templates/velocity_conversion.rs")?
                .replace("template_scale_factor", f.to_string().as_str());
            internal_functions.push_str(&template);
        }

        // Define `acceleration_scale_factor` and conversion functions if needed
        if let Some(f) = device.acceleration_scale_factor {
            let template = read_to_string("templates/acceleration_conversion.rs")?
                .replace("template_scale_factor", f.to_string().as_str());
            internal_functions.push_str(&template);
        }

        // Define `new()` function
        let template = read_to_string("templates/new.rs")?;
        external_functions.push_str(&template);

        // Define `start_update_messages()` and `stop_update_messages()` functions
        let template = read_to_string("templates/update_messages.rs")?;
        external_functions.push_str(&template);

        // Define `identify()` function if needed
        if device.identify {
            let template = read_to_string("templates/identify.rs")?;
            external_functions.push_str(&template);
        }

        // Define `set_channel_enable_state()` function if needed
        if device.channel_enable_state {
            let template = read_to_string("templates/channel_enable_state.rs")?;
            external_functions.push_str(&template);
        }

        // Define `home()` and `home_async()` functions if needed
        if device.home {
            let template = read_to_string("templates/home.rs")?;
            external_functions.push_str(&template);
        }

        // Define `move_absolute()` and `move_absolute_from_params()` functions if needed
        if device.move_absolute {
            let template = read_to_string("templates/move_absolute.rs")?;
            external_functions.push_str(&template);
        }

        let template = read_to_string("templates/definition.rs")?
            .replace("TemplateStructName", &device.name)
            .replace("// Internal functions inserted here", &internal_functions)
            .replace("// External functions inserted here", &external_functions);
        writeln!(file, "{}", template)?;

        // Implement `From<T>` traits
        let template = read_to_string("templates/from.rs")?;
        let modified = template.replace("TemplateStructName", device.name.as_str());
        writeln!(file, "{}", modified)?;

        // Implement the `Deref` trait
        let template = read_to_string("templates/deref.rs")?;
        let modified = template.replace("TemplateStructName", device.name.as_str());
        writeln!(file, "{}", modified)?;

        // Implement the `Display` trait
        let template = read_to_string("templates/display.rs")?;
        let modified = template.replace("TemplateStructName", device.name.as_str());
        writeln!(file, "{}", modified)?;
    }

    // Format the generated file using rustfmt
    if !Command::new("rustfmt")
        .arg(Path::new(&out_dir).join("devices_built.rs"))
        .status()?
        .success()
    {
        panic!("Failed to format using rustfmt");
    }

    Ok(())
}

/// # Main
/// This build script reads the `messages.csv` file and generates several static hash maps to
/// lookup length and waiting sender for a given message ID. The generated code is saved to
/// separate `.rs` files in the `OUT_DIR`, which are then included in the `messages.rs` file.
fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = var_os("OUT_DIR").ok_or("OUT_DIR not set")?;
    let mut length_map = File::create(Path::new(&out_dir).join("length_map.rs"))?;
    let mut init_groups = File::create(Path::new(&out_dir).join("init_groups.rs"))?;
    let mut sender_map = File::create(Path::new(&out_dir).join("sender_map.rs"))?;
    let mut groups_map = HashMap::new();

    let mut reader = csv::Reader::from_path("messages.csv")?;
    let messages = reader
        .deserialize()
        .collect::<Result<HashSet<Message>, csv::Error>>()?;
    if messages.len() < reader.into_records().count() {
        panic!("WARNING: messages.csv contains duplicate rows");
    }

    writeln!(length_map, "use phf::phf_map;")?;
    writeln!(
        length_map,
        "pub(crate) static LENGTH_MAP: phf::Map<[u8; 2], usize> = phf_map! {{"
    )?;
    for message in &messages {
        if let [length] = message.length.as_slice() {
            writeln!(length_map, "{} => {},", message.id, length)?;
        }
        if let Some(group) = &message.group {
            groups_map
                .entry(group.clone())
                .or_insert(HashSet::new())
                .insert(message.id.clone());
        }
    }
    writeln!(length_map, "}};")?;

    writeln!(
        sender_map,
        "pub(crate) static SENDER_MAP: phf::Map<[u8; 2], &RwLock<Option<Sender<Box<[u8]>>>>> = phf_map! {{"
    )?;
    for (group, ids) in groups_map {
        writeln!(
            init_groups,
            "static {}: RwLock<Option<Sender<Box<[u8]>>>> = RwLock::new(None);",
            group
        )?;
        for id in ids {
            writeln!(sender_map, "{} => &{},", id, group)?;
        }
    }
    writeln!(sender_map, "}};")?;

    build_devices();

    Ok(())
}
