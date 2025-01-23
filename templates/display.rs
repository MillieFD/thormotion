/// # `Display`
/// Provides a string representation of the struct in the format
/// `"{StructName} (serial number: {serial_number})"`.
impl Display for TemplateStructName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "KDC101 (serial number: {})", self.serial_number)
    }
}
