# Thormotion

A cross-platform motion control library for Thorlabs systems, written in Rust.

> [!WARNING]
> Thormotion is currently pre-release and offers limited functionality for KDC101 devices only.
> To request additional functions and device types, please open a new GitHub discussion.

### 🚀 Features

- Designed for robotics, automation, and scientific applications.
- Fast and efficient, with minimal overhead.
- API supports Python and Rust to simplify your experiments.
- Runs on macOS, Linux, and Windows.

### 🛠️ Installation

**Python users**

Install from PyPI using Pip:

```python
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
thormotion = "0.3.0" # Check for the latest version on crates.io
```

### ⚙️ Libusb

Thormotion dynamically binds to `libusb` in order to communicate with USB devices. You may already have `libusb`
installed. If not, follow the instructions below.

**MacOS**

Install libusb using [homebrew](https://brew.sh):

```bash
brew install libusb
```

**Linux**

Install libusb using your package manager:

```bash
sudo apt update
sudo apt install libusb-1.0-0-dev
```

**Windows**

Install libusb using [vcpkg](https://vcpkg.io/en/):

```bash
vcpkg install libusb
```

### 📝 Citing Thormotion

Please cite Thormotion in your research. To find the correct DOI for the version of Thormotion you are using, visit 
[Zenodo](https://zenodo.org) and search for `thormotion`. Alternatively, You can cite all versions by using the 
generic DOI [10.5281/zenodo.15006067](https://doi.org/10.5281/zenodo.15006067) which always resolves to the latest 
release.

```markdown
MillieFD. (2025). MillieFD/thormotion: v0.3.0 Stable Pre-Release (v0.3.0). Zenodo. https://doi.org/10.5281/zenodo.15006067
```

### 📖 Documentation

A complete list of the supported Thorlabs devices and functions can be found on
[docs.rs](https://docs.rs/thormotion/).

Thormotion implements the Thorlabs APT communication protocol. For full details, please refer to the APT protocol
documentation.

### 🤝 Contributing

Thormotion is an open-source project! Contributions are welcome, and we are always looking for ways to improve the
library. If you would like to help out, please check the list of open issues. If you have an idea for a new feature
or would like to report a bug, please open a new issue or submit a pull request. Please ask questions and discuss
features in the issues if anything is unclear. Note that all code submissions and pull requests are assumed to agree
with the BSD 3-Clause License. Make sure to read the contributing guidelines before getting started.

### 🧑‍⚖️ License

This project is licensed under the BSD 3-Clause License. Opening a pull request indicates agreement with these terms.