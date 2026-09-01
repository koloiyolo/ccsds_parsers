#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    /// Raised when value is out of bounds for the parsed type.
    ValueOutOfBounds { value: String, parsed_type: String },
    /// Raised when failed to calculate CRC.
    CrcCheckFailed { reason: String },
}
