use crate::spacepacket::primary_header::PrimaryHeader;

#[derive(Debug, Clone, PartialEq, Eq)]

/// https://ccsds.org/Pubs/133x0b2e2.pdf
pub struct SpacePacket {
    pub primary_header: PrimaryHeader,
    pub frame: Vec<u8>,
}

impl TryFrom<Vec<u8>> for SpacePacket {
    type Error = crate::errors::ParserError;

    fn try_from(frame: Vec<u8>) -> Result<Self, Self::Error> {
        let primary_header = PrimaryHeader::parse(frame.as_slice())?;
        Ok(Self {
            primary_header,
            frame,
        })
    }
}

impl std::fmt::Display for SpacePacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CCSDS Space Packet\n")?;
        writeln!(f, "{}", self.primary_header)?;
        writeln!(f, "  Frame: {}", hex::encode(&self.frame))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::spacepacket::primary_header::{PacketType, SecondaryHeaderFlag};

    use super::*;

    #[test]
    fn parse_example_spp_frame() {
        let frame = hex::decode("0c00c001000a48656c6c6f5370616365").unwrap();

        let spp = SpacePacket::try_from(frame).unwrap();
        assert_eq!(spp.primary_header.packet_version_number, 0b000);
        assert_eq!(spp.primary_header.packet_type, PacketType::Telecommand);
        assert_eq!(
            spp.primary_header.secondary_header_flag,
            SecondaryHeaderFlag::NotPresent
        );
        assert_eq!(spp.primary_header.application_process_id, 1024);
        assert_eq!(spp.primary_header.sequence_flags, 0b11);
        assert_eq!(spp.primary_header.packet_sequence_count, 1);
    }

    #[test]
    fn too_short_spp_frame() {
        let frame = hex::decode("0c00c0").unwrap();

        let spp = SpacePacket::try_from(frame);
        assert!(spp.is_err());
    }
}
