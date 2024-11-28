/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: build.rs
Description: This file defines the build script which is run at compile time. This build script
reads the messages.csv file and generates several static hash maps to lookup length and waiting
sender for a given message ID. The generated code is saved to separate .rs files in the OUT_DIR,
which are then included in the src/messages.rs file.
---------------------------------------------------------------------------------------------------
Notes:
*/

use serde::{Deserialize, Deserializer};
use std::collections::{HashMap, HashSet};
use std::env::var_os;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// # Define The Record Struct
/// This struct represents a single record (row) in the CSV file. It contains the ID, length, and
/// group names for a given message. The struct implements the Deserialize trait from the serde
/// crate, which automatically parses columns in the CSV file to struct fields with the same name.

#[derive(Deserialize, Hash, Eq, PartialEq)]
struct Record {
    #[serde(deserialize_with = "deserialize_id")]
    id: String,
    length: usize,
    group: Option<String>,
}

/// # Deserialize ID Function
/// A custom deserialization function is defined to parse hexadecimal values from the messages.csv
/// ID column into two-byte hexadecimal arrays in little-endian order. For example, the ID
/// "0x1234" will be parsed into the array [0x34, 0x12].

fn deserialize_id<'a, 'de, D>(deserializer: D) -> Result<String, D::Error>
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

/// # Main
/// This build script reads the messages.csv file and generates several static hash maps to lookup
/// length and waiting sender for a given message ID. The generated code is saved to separate .rs
/// files in the OUT_DIR, which are then included in the src/messages.rs file.

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = var_os("OUT_DIR").ok_or("OUT_DIR not set")?;
    let mut length_map = File::create(Path::new(&out_dir).join("length_map.rs"))?;
    let mut init_groups = File::create(Path::new(&out_dir).join("init_groups.rs"))?;
    let mut sender_map = File::create(Path::new(&out_dir).join("sender_map.rs"))?;
    let mut groups_map = HashMap::new();

    let mut reader = csv::Reader::from_path("messages.csv")?;
    let records: HashSet<Record> = reader.deserialize().collect::<Result<_, csv::Error>>()?;
    if records.len() < reader.into_records().count() {
        panic!("WARNING: messages.csv contains duplicate rows");
    }

    writeln!(length_map, "use phf::phf_map;")?;
    writeln!(init_groups, "use std::sync::RwLock;")?;

    writeln!(
        length_map,
        "pub(crate) static LENGTH_MAP: phf::Map<[u8; 2], usize> = phf_map! {{"
    )?;
    for record in &records {
        writeln!(length_map, "{} => {},", record.id, record.length)?;
        if let Some(group) = &record.group {
            groups_map
                .entry(group.clone())
                .or_insert(Vec::new())
                .push(record.id.clone());
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

    Ok(())
}