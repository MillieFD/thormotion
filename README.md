# Thormotion

A cross-platform motion control library for Thorlabs systems, written in Rust.

> [!WARNING]
> Thormotion is currently pre-release and offers limited functionality for KDC101 devices only.

### 🚀 Features
- Designed for robotics, automation, and scientific applications.
- Fast and efficient, with minimal overhead.
- Python API simplifies experiment design.
- Supports macOS, Linux, and Windows. Supports ARM64 and x86 architectures.

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

Add Thormotion to your Cargo.toml file:

```toml
[dependencies]
thormotion = "0.2.0" # Check for the latest version on crates.io
```

### 📖 Documentation

Thormotion implements the Thorlabs APT communication protocol. 
For full details, please refer to the APT protocol documentation.

### 🤝 Contributing

Thormotion is an open-source project! 
Contributions are welcome, and we are always looking for ways to improve the library. 
If you would like to help out, please check the list of open issues. 
If you have an idea for a new feature or would like to report a bug, please open a new issue or submit a pull request. 
Please ask questions and discuss features in the issues if anything is unclear. 
Note that all code submissions and pull requests are assumed to agree with the BSD 3-Clause License. 
Make sure to read the contributing guidelines before getting started.

### 📝 License

This project is licensed under the BSD 3-Clause License. 
Opening a pull request indicates agreement with these terms.