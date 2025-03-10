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

use std::collections::HashSet;
use std::env::var_os;
use std::fs::{File, read_to_string};
use std::path::{Path, PathBuf};

/**
Creates an output file in the directory specified by the `OUT_DIR`
environment variable and returns the file handle alongside its path.

# Examples
```rust
use std::fs::File;
use std::path::PathBuf;

let (file, path): (File, PathBuf) = get_output_file("example.txt");
assert_eq!(path.file_name().unwrap(), "example.txt");
```

# Panics
- Panics if the `OUT_DIR` environment variable is not set.
- Panics if the file cannot be created (e.g. due to insufficient permissions).
*/
pub(super) fn get_output_file(name: &str) -> (File, PathBuf) {
    let out_dir = var_os("OUT_DIR").expect("OUT_DIR not set");
    let path = Path::new(&out_dir).join(name);
    let file = File::create(&path)
        .unwrap_or_else(|err| panic!("Failed to create file '{}': {}", name, err));
    (file, path)
}

/**
Returns a `Vec<T>` containing all unique records from the CSV file.

### Examples
```rust
let path_to_csv = PathBuf::from("records.csv")
let records: Vec<Record> = from_csv();
for record in records {
    println!("Record Name: {}", record.name);
}
```

### Panics
- If a record fails to deserialize, the function panics with a warning message.
- If any duplicate rows are detected, the function panics with a warning message.
*/
pub(super) fn from_csv<T>(name: &str) -> Vec<T>
where
    T: std::hash::Hash + Eq + serde::de::DeserializeOwned,
{
    let path = Path::new("build").join(name);
    let mut reader = csv::Reader::from_path(&path).expect("Failed to open CSV file");
    reader.has_headers();
    let mut hashset: HashSet<T> = HashSet::new();
    for result in reader.deserialize() {
        let record: T = result.expect("Failed to deserialize CSV record");
        assert!(
            hashset.insert(record),
            "{} contains duplicate rows",
            path.display()
        );
    }
    let vector: Vec<T> = hashset.into_iter().collect();
    vector
}

/**
Reads a template file from the `templates` directory and returns the file content as a `String`.

### Examples
```rust
let template = get_template("example_template.txt");
println!("Template Content: {}", template);
```

### Panics
If the file cannot be found or read, the function panics with a warning message.
The file path is included in the panic message for easier debugging.
*/
pub(super) fn get_template(name: &str) -> String {
    let path = Path::new("build/templates/").join(name);
    read_to_string(&path).expect(&format!("Failed to read {}", path.display()))
}
