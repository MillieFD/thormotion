[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.17064172.svg)](https://doi.org/10.5281/zenodo.17064172)
[![Crates.io](https://img.shields.io/crates/v/thormotion.svg)](https://crates.io/crates/thormotion)
[![PyPI version](https://img.shields.io/pypi/v/thormotion.svg)](https://pypi.org/project/thormotion/)
[![Docs.rs](https://docs.rs/thormotion/badge.svg)](https://docs.rs/thormotion)
![Platforms](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)

# Thormotion

A cross-platform motion control library for Thorlabs systems, written in Rust.

### 🚀 Features

- Designed for robotics, automation, and scientific applications.
- Python and Rust API
- Fast and efficient, with minimal overhead.
- Runs on macOS, Linux, and Windows.

This project is still growing. We are happy to add support for any new devices and functions as needed. Please open 
a new GitHub issue to make a request.

### 🛠️ Installation

**Python users**

Install from PyPI using Pip:

```bash
pip install thormotion
```

Then import the package at the top of your python file:

```python
import thormotion
```

**Rust users**

Run the following Cargo command in your project directory:

```bash
cargo add thormotion
```

Or add Thormotion to your Cargo.toml file:

```toml
[dependencies]
thormotion = "*" # Check for the latest version on crates.io
```

### 📝 Citing Thormotion

Please cite Thormotion in your research. To find the correct DOI for the version of Thormotion you are using, visit
[Zenodo](https://zenodo.org) and search for `thormotion`. Alternatively, You can cite all versions by using the
generic DOI [10.5281/zenodo.15006067](https://doi.org/10.5281/zenodo.15006067) which always resolves to the latest
release.

```markdown
Amelia Fraser-Dale. (2025). MillieFD/thormotion: Thormotion v0.4.0
Zenodo. https://doi.org/10.5281/zenodo.17064172
```

### 📖 Documentation

A complete list of the supported Thorlabs devices and functions can be found on [docs.rs](https://docs.rs/thormotion/).

Thormotion implements the Thorlabs APT communication protocol. For full details, please refer to the APT protocol
documentation.

### 🪵 Logging

Thormotion uses the [log](https://crates.io/crates/log) crate to record information at runtime. This lightweight
logging API is compatible with a variety of backends, allowing users to select the most appropriate logging
implementation for their use case.

| Level   | Description                                   |
|---------|-----------------------------------------------|
| `trace` | The raw bytes sent and received               |
| `debug` | Low-level activity common to all device types |
| `info`  | High-level activity specific to each device   |
| `warn`  | Issues that do not preclude the request       |
| `error` | Issues that terminate the application         |

### 🤝 Contributing

Thormotion is an open-source project! Contributions are welcome, and we are always looking for ways to improve the
library. If you would like to help out, please check the list of open issues. If you have an idea for a new feature
or would like to report a bug, please open a new issue or submit a pull request. Note that all code submissions and
pull requests are assumed to agree with the BSD 3-Clause Licence.

### 🧑‍⚖️ License

This project is licensed under the BSD 3-Clause Licence. Opening a pull request indicates agreement with these terms.