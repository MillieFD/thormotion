/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: build.rs
Description: todo
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

///

#[derive(Deserialize, Hash, Eq, PartialEq)]
struct Record {
    #[serde(deserialize_with = "deserialize_id")]
    id: String,
    length: usize,
    group: Option<String>,
}

///

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

///

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
