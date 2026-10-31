use crate::spacepacket::primary_header::{PRIMARY_HEADER_SIZE, PrimaryHeader, SecondaryHeaderFlag};

#[derive(Debug, Clone, PartialEq, Eq)]

/// https://ccsds.org/publications/bluebooks/entry/3264/
pub struct SpacePacket {
    pub primary_header: PrimaryHeader,
    pub packet_data: Vec<u8>,
    pub frame: Vec<u8>,
}

impl TryFrom<Vec<u8>> for SpacePacket {
    type Error = crate::errors::ParserError;

    fn try_from(frame: Vec<u8>) -> Result<Self, Self::Error> {
        let primary_header = PrimaryHeader::try_from(frame.as_slice())?;
        let packet_data = match primary_header.secondary_header_flag {
            SecondaryHeaderFlag::NotPresent => frame[PRIMARY_HEADER_SIZE..].to_vec(),
            SecondaryHeaderFlag::Present => {
                let secondary_header_size = 0; // TODO: Implement Secondary Header.
                let offset = PRIMARY_HEADER_SIZE + secondary_header_size;
                frame[offset..].to_vec()
            }
        };
        Ok(Self {
            primary_header,
            packet_data,
            frame,
        })
    }
}

impl std::fmt::Display for SpacePacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CCSDS Space Packet\n")?;
        writeln!(f, "{}", self.primary_header)?;
        writeln!(f, "  Packet Data")?;
        writeln!(f, "    Frame: {}", hex::encode(&self.packet_data))?;
        writeln!(
            f,
            "    UTF-8: {}\n",
            String::from_utf8(self.packet_data.to_vec())
                .expect("Failed to parse SPP Packet Data to UTF-8")
        )?;
        writeln!(f, "  Frame: {}", hex::encode(&self.frame))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::spacepacket::primary_header::{PacketType, SecondaryHeaderFlag, SequenceFlags};

    use super::*;

    #[test]
    fn parse_example_spp_frame() {
        let frame = vec![
            0x0c, 0x00, 0xc0, 0x01, 0x00, 0x0a, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x53, 0x70, 0x61,
            0x63, 0x65,
        ];

        let spp = SpacePacket::try_from(frame).unwrap();
        assert_eq!(spp.primary_header.packet_version_number, 0b000);
        assert_eq!(spp.primary_header.packet_type, PacketType::Telecommand);
        assert_eq!(
            spp.primary_header.secondary_header_flag,
            SecondaryHeaderFlag::Present
        );
        assert_eq!(spp.primary_header.application_process_id, 1024);
        assert_eq!(
            spp.primary_header.sequence_flags,
            SequenceFlags::Unsegmented
        );
        assert_eq!(spp.primary_header.packet_sequence_count, 1);
    }

    #[test]
    fn too_short_spp_frame() {
        let frame = vec![0x0c, 0x00, 0xc0];

        let spp = SpacePacket::try_from(frame);
        assert!(spp.is_err());
    }
}
