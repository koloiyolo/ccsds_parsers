use deku::DekuError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    /// Raised when value is out of bounds for the parsed type.
    ValueOutOfBounds { value: String, parsed_type: String },
    /// Raised when failed to calculate CRC.
    CrcCheckFailed { reason: String },
    /// Raised when provided value is not binary representation.
    InvalidBinaryValue(u32),
    /// Invalid frame length.
    InvalidFrameLength {
        expected_size_bytes: usize,
        received_size_bytes: usize,
    },
    /// Error raised by [deku] library.
    RaisedByDeku(DekuError),
}

impl From<DekuError> for ParserError {
    fn from(err: DekuError) -> Self {
        ParserError::RaisedByDeku(err)
    }
}
