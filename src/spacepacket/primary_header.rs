use crate::{errors::ParserError, utils::extract_value_from_left};

const PRIMARY_HEADER_SIZE: usize = 6;
const DEFAULT_DISPLAY_PREFIX: &str = "  ";

/// https://ccsds.org/Pubs/133x0b2e2.pdf
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimaryHeader {
    pub packet_version_number: u8,
    pub packet_type: PacketType,
    pub secondary_header_flag: SecondaryHeaderFlag,
    pub application_process_id: u16,
    pub sequence_flags: u8,
    pub packet_sequence_count: u16,
    pub packet_data_length: u16,
    pub frame: Vec<u8>,
}

impl TryFrom<&[u8]> for PrimaryHeader {
    type Error = ParserError;

    fn try_from(frame: &[u8]) -> Result<Self, Self::Error> {
        if frame.len() < PRIMARY_HEADER_SIZE {
            return Err(ParserError::InvalidFrameLength {
                expected_size_bytes: PRIMARY_HEADER_SIZE,
                received_size_bytes: frame.len(),
            });
        }

        let first_octets_pair = u16::from_be_bytes([frame[0], frame[1]]);
        let packet_version_number = extract_value_from_left(first_octets_pair, 0, 3) as u8;

        let packet_type =
            PacketType::try_from(extract_value_from_left(first_octets_pair, 3, 1) as u8)?;
        let secondary_header_flag =
            SecondaryHeaderFlag::try_from(extract_value_from_left(first_octets_pair, 4, 1) as u8)?;

        let application_process_id = extract_value_from_left(first_octets_pair, 5, 11);

        let second_octets_pair = u16::from_be_bytes([frame[2], frame[3]]);
        let sequence_flags = extract_value_from_left(second_octets_pair, 0, 2) as u8;
        let packet_sequence_count = extract_value_from_left(second_octets_pair, 2, 14);

        let packet_data_length = u16::from_be_bytes([frame[4], frame[5]]);
        Ok(Self {
            packet_version_number,
            packet_type,
            secondary_header_flag,
            application_process_id,
            sequence_flags,
            packet_sequence_count,
            packet_data_length,
            frame: frame[..PRIMARY_HEADER_SIZE].to_vec(),
        })
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketType {
    Telecommand,
    Telemetry,
}

impl TryFrom<u8> for PacketType {
    type Error = ParserError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Telecommand),
            1 => Ok(Self::Telemetry),
            _ => Err(ParserError::ValueOutOfBounds {
                value: value.to_string(),
                parsed_type: "PacketType".into(),
            }),
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SecondaryHeaderFlag {
    Present,
    NotPresent,
}

impl TryFrom<u8> for SecondaryHeaderFlag {
    type Error = ParserError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Present),
            1 => Ok(Self::NotPresent),
            _ => Err(ParserError::ValueOutOfBounds {
                value: value.to_string(),
                parsed_type: "SecondaryHeaderFlag".into(),
            }),
        }
    }
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
    use super::*;

    #[test]
    fn parses_primary_header_with_version_zero() {
        let frame = [0x09, 0x23, 0xA5, 0x67, 0x01, 0x02];

        let header = PrimaryHeader::try_from(&frame[..]).unwrap();

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
    }

    #[test]
    fn too_short_primary_header_frame_is_rejected() {
        let frame = [0x09, 0x23, 0xA5, 0x67];

        let header = PrimaryHeader::try_from(&frame[..]);

        assert!(header.is_err());
    }

    #[test]
    fn parses_packet_type_values() {
        assert_eq!(PacketType::try_from(0).unwrap(), PacketType::Telecommand);
        assert_eq!(PacketType::try_from(1).unwrap(), PacketType::Telemetry);
        assert!(PacketType::try_from(2).is_err());
    }

    #[test]
    fn parses_secondary_header_flag_values() {
        assert_eq!(
            SecondaryHeaderFlag::try_from(0).unwrap(),
            SecondaryHeaderFlag::Present
        );
        assert_eq!(
            SecondaryHeaderFlag::try_from(1).unwrap(),
            SecondaryHeaderFlag::NotPresent
        );
        assert!(SecondaryHeaderFlag::try_from(2).is_err());
    }
}
