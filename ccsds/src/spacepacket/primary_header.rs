use crate::{errors::ParserError, utils::extract_value_from_left};

pub const PRIMARY_HEADER_SIZE: usize = 6;
const DEFAULT_DISPLAY_PREFIX: &str = "  ";

/// https://ccsds.org/publications/bluebooks/entry/3264/ page 32
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimaryHeader {
    pub packet_version_number: u8,
    pub packet_type: PacketType,
    pub secondary_header_flag: SecondaryHeaderFlag,
    pub application_process_id: u16,
    pub sequence_flags: SequenceFlags,
    pub packet_sequence_count: u16,
    pub packet_data_length: u16,
    pub frame: [u8; PRIMARY_HEADER_SIZE],
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
        let sequence_flags =
            SequenceFlags::try_from(extract_value_from_left(second_octets_pair, 0, 2) as u8)?;
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
            frame: frame[..PRIMARY_HEADER_SIZE]
                .try_into()
                .expect("Enforced by const value"),
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
            "{DEFAULT_DISPLAY_PREFIX}  Sequence flags: {}",
            self.sequence_flags
        )?;
        writeln!(
            f,
            "{DEFAULT_DISPLAY_PREFIX}  Packet sequence count: {}",
            self.packet_sequence_count
        )?;
        writeln!(
            f,
            "{DEFAULT_DISPLAY_PREFIX}  Frame: {}",
            hex::encode(self.frame)
        )
    }
}

/// https://ccsds.org/publications/bluebooks/entry/3264/ page 33
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

/// https://ccsds.org/publications/bluebooks/entry/3264/ page 33
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SecondaryHeaderFlag {
    NotPresent,
    Present,
}

impl TryFrom<u8> for SecondaryHeaderFlag {
    type Error = ParserError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotPresent),
            1 => Ok(Self::Present),
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

/// https://ccsds.org/publications/bluebooks/entry/3264/ page 34
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SequenceFlags {
    ContinuationSegment = 0b00,
    StartSegment = 0b01,
    EndSegment = 0b10,
    Unsegmented = 0b11,
}

impl TryFrom<u8> for SequenceFlags {
    type Error = ParserError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Self::ContinuationSegment),
            0b01 => Ok(Self::StartSegment),
            0b10 => Ok(Self::EndSegment),
            0b11 => Ok(Self::Unsegmented),
            _ => Err(ParserError::ValueOutOfBounds {
                value: value.to_string(),
                parsed_type: "SequenceFlags".into(),
            }),
        }
    }
}

impl std::fmt::Display for SequenceFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ContinuationSegment => "Continuation Segment (0b00)",
                Self::StartSegment => "Start Segment (0b01)",
                Self::EndSegment => "End Segment (0b10)",
                Self::Unsegmented => "Unsegmented (0b11)",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_primary_header_with_version_zero() {
        let frame = [0x09, 0x23, 0xA5, 0x67, 0x01, 0x02];

        let header = PrimaryHeader::try_from(&frame[..]).unwrap();

        assert_eq!(header.packet_version_number, 0b000);
        assert_eq!(header.packet_type, PacketType::Telecommand);
        assert_eq!(header.secondary_header_flag, SecondaryHeaderFlag::Present);
        assert_eq!(header.application_process_id, 0x123);
        assert_eq!(header.sequence_flags, SequenceFlags::EndSegment);
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
    fn parse_packet_type_values() {
        assert_eq!(PacketType::try_from(0).unwrap(), PacketType::Telecommand);
        assert_eq!(PacketType::try_from(1).unwrap(), PacketType::Telemetry);
        assert!(PacketType::try_from(2).is_err());
    }

    #[test]
    fn parse_secondary_header_flag_values() {
        assert_eq!(
            SecondaryHeaderFlag::try_from(0).unwrap(),
            SecondaryHeaderFlag::NotPresent,
        );
        assert_eq!(
            SecondaryHeaderFlag::try_from(1).unwrap(),
            SecondaryHeaderFlag::Present,
        );
        assert!(SecondaryHeaderFlag::try_from(2).is_err());
    }

    #[test]
    fn parse_segmentation_flag() {
        assert_eq!(
            SequenceFlags::try_from(0b00).unwrap(),
            SequenceFlags::ContinuationSegment,
        );
        assert_eq!(
            SequenceFlags::try_from(0b01).unwrap(),
            SequenceFlags::StartSegment,
        );
        assert_eq!(
            SequenceFlags::try_from(0b10).unwrap(),
            SequenceFlags::EndSegment,
        );
        assert_eq!(
            SequenceFlags::try_from(0b11).unwrap(),
            SequenceFlags::Unsegmented,
        );
        assert!(SequenceFlags::try_from(42).is_err());
        assert!(SequenceFlags::try_from(255).is_err());
    }
}
