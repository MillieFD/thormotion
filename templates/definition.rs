#[pyclass]
#[derive(Debug)]
pub struct TemplateStructName {
    device: UsbDevicePrimitive,
}

impl TemplateStructName {
    // Internal functions inserted here
}

/// # Exposing `TemplateStructName` to Python
/// The **Thormotion** Rust library is published as a Python package using `PyO3`.
/// PyO3 is a Rust library that provides tools and macros for creating Python
/// bindings, allowing Rust code to be called directly from Python.
/// Functions exposed to Python are defined below.
#[pymethods]
impl TemplateStructName {
    // External functions inserted here
}
