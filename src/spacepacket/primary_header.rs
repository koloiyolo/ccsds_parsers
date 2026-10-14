use crate::errors::ParserError;
use deku::prelude::{DekuContainerRead, DekuRead, DekuWrite};

const PRIMARY_HEADER_SIZE: usize = 6;
const DEFAULT_DISPLAY_PREFIX: &str = "  ";

/// https://ccsds.org/Pubs/133x0b2e2.pdf
#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct PrimaryHeader {
    #[deku(bits = 3)]
    pub packet_version_number: u8,
    pub packet_type: PacketType,
    pub secondary_header_flag: SecondaryHeaderFlag,
    #[deku(bits = 11)]
    pub application_process_id: u16,
    #[deku(bits = 2)]
    pub sequence_flags: u8,
    #[deku(bits = 14)]
    pub packet_sequence_count: u16,
    pub packet_data_length: u16,
    #[deku(skip, default = "Vec::new()")]
    pub frame: Vec<u8>,
}

impl PrimaryHeader {
    pub fn parse(frame: &[u8]) -> Result<Self, ParserError> {
        if frame.len() < PRIMARY_HEADER_SIZE {
            return Err(ParserError::InvalidFrameLength {
                expected_size_bytes: PRIMARY_HEADER_SIZE,
                received_size_bytes: frame.len(),
            });
        }

        let (_, mut header) = Self::from_bytes((frame, 0)).map_err(map_deku_error)?;
        header.frame = frame[..PRIMARY_HEADER_SIZE].to_vec();
        Ok(header)
    }
}

fn map_deku_error(error: deku::DekuError) -> ParserError {
    match error {
        deku::DekuError::IdVariantNotFound => ParserError::ValueOutOfBounds {
            value: "unknown enum discriminator".into(),
            parsed_type: "PacketType or SecondaryHeaderFlag".into(),
        },
        error => ParserError::ValueOutOfBounds {
            value: format!("{error:?}"),
            parsed_type: "PrimaryHeader".into(),
        },
    }
}

impl std::fmt::Display for PrimaryHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{DEFAULT_DISPLAY_PREFIX}Primary header")?;
        if self.packet_version_number == 0 {
            writeln!(
                f,
                "{DEFAULT_DISPLAY_PREFIX}  Packet version number: {} ({:#b})",
                self.packet_version_number, self.packet_version_number
            )?;
        } else {
            writeln!(
                f,
                "{DEFAULT_DISPLAY_PREFIX}  Packet version number: {} ({:#b}) Invalid, expected `0b000`",
                self.packet_version_number, self.packet_version_number
            )?;
        }
        writeln!(
            f,
            "{DEFAULT_DISPLAY_PREFIX}  Packet type: {}",
            self.packet_type
        )?;
        writeln!(
            f,
            "{DEFAULT_DISPLAY_PREFIX}  Secondary header flag: {}",
            self.secondary_header_flag
        )?;
        writeln!(
            f,
            "{DEFAULT_DISPLAY_PREFIX}  Application process ID: {} ({:#x})",
            self.application_process_id, self.application_process_id
        )?;
        writeln!(
            f,
            "{DEFAULT_DISPLAY_PREFIX}  Sequence flags: {} ({:#b})",
            self.sequence_flags, self.sequence_flags
        )?;
        writeln!(
            f,
            "{DEFAULT_DISPLAY_PREFIX}  Packet sequence count: {}",
            self.packet_sequence_count
        )?;
        writeln!(
            f,
            "{DEFAULT_DISPLAY_PREFIX}  Frame: {}",
            hex::encode(&self.frame)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[repr(u8)]
#[deku(
    id_type = "u8",
    bits = "1",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian"
)]
pub enum PacketType {
    #[deku(id = "0")]
    Telecommand,
    #[deku(id = "1")]
    Telemetry,
}

impl std::fmt::Display for PacketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Telecommand => "Telecommand",
                Self::Telemetry => "Telemetry",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[repr(u8)]
#[deku(
    id_type = "u8",
    bits = "1",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian"
)]
pub enum SecondaryHeaderFlag {
    #[deku(id = 0b0)]
    Present,
    #[deku(id = 0b1)]
    NotPresent,
}

impl std::fmt::Display for SecondaryHeaderFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Present => "Present",
                Self::NotPresent => "Not Present",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use deku::prelude::DekuContainerWrite;

    use super::*;

    #[test]
    fn parses_primary_header_with_version_zero() {
        let frame = [0x09, 0x23, 0xA5, 0x67, 0x01, 0x02];

        let header = PrimaryHeader::parse(&frame[..]).unwrap();

        assert_eq!(header.packet_version_number, 0b000);
        assert_eq!(header.packet_type, PacketType::Telecommand);
        assert_eq!(
            header.secondary_header_flag,
            SecondaryHeaderFlag::NotPresent
        );
        assert_eq!(header.application_process_id, 0x123);
        assert_eq!(header.sequence_flags, 0b10);
        assert_eq!(header.packet_sequence_count, 0x2567);
        assert_eq!(header.packet_data_length, 0x0102);
        assert_eq!(header.to_bytes().unwrap(), frame);
    }

    #[test]
    fn too_short_primary_header_frame_is_rejected() {
        let frame = [0x09, 0x23, 0xA5, 0x67];

        let header = PrimaryHeader::parse(&frame[..]);

        assert!(header.is_err());
    }

    #[test]
    fn parses_telemetry_packet_with_secondary_header() {
        let frame = [0x10, 0x00, 0, 0, 0, 0];

        let header = PrimaryHeader::parse(&frame[..]).unwrap();

        assert_eq!(header.packet_type, PacketType::Telemetry);
        assert_eq!(header.secondary_header_flag, SecondaryHeaderFlag::Present);
        assert_eq!(header.to_bytes().unwrap(), frame);
    }
}
