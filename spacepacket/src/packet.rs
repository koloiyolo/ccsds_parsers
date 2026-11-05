use common::ParserError;
use deku::{DekuContainerWrite, DekuRead, DekuWrite};

use crate::{
    packet_data_field::PacketDataField,
    primary_header::{PrimaryHeader, SecondaryHeaderFlag},
};

/// https://ccsds.org/publications/bluebooks/entry/3264/
///
/// A Space Packet shall include the defined fields, positioned contiguously, in the
/// following sequence:
///
/// * Packet Primary Header (6 octets, mandatory);
/// * Packet Data Field (from 1 to 65536 octets, mandatory).
///
/// Space Packet shall consist of at least 7 and at most 65542 octets
#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct SpacePacket {
    pub primary_header: PrimaryHeader,
    #[deku(
        ctx = "primary_header.packet_data_length as usize, primary_header.secondary_header_flag"
    )]
    pub packet_data: PacketDataField,
}

impl TryFrom<Vec<u8>> for SpacePacket {
    type Error = ParserError;

    fn try_from(frame: Vec<u8>) -> Result<Self, Self::Error> {
        let primary_header = PrimaryHeader::try_from(&frame)?;
        let packet_data = match primary_header.secondary_header_flag {
            SecondaryHeaderFlag::NotPresent => frame[PrimaryHeader::SIZE..].to_vec(),
            SecondaryHeaderFlag::Present => {
                let secondary_header_size = 0; // TODO: Implement Secondary Header.
                let offset = PrimaryHeader::SIZE + secondary_header_size;
                frame[offset..].to_vec()
            }
        };
        let packet_data = PacketDataField { raw: packet_data };
        Ok(Self {
            primary_header,
            packet_data,
        })
    }
}

impl std::fmt::Display for SpacePacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CCSDS Space Packet\n")?;
        writeln!(f, "{}", self.primary_header)?;
        writeln!(f, "  Packet Data")?;
        writeln!(f, "    Frame: {}", hex::encode(&self.packet_data.raw))?;
        writeln!(
            f,
            "    UTF-8: {}\n",
            String::from_utf8(self.packet_data.raw.to_vec())
                .expect("Failed to parse SPP Packet Data to UTF-8")
        )?;
        writeln!(
            f,
            "  Frame: {}",
            hex::encode(self.to_bytes().expect("Invalid data"))
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use deku::DekuContainerRead;

    use crate::primary_header::{PacketType, SequenceFlags};

    use super::*;

    const EXAMPLE_FRAME: [u8; 16] = [
        0x0c, 0x00, 0xc0, 0x01, 0x00, 0x0a, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x53, 0x70, 0x61, 0x63,
        0x65,
    ];

    #[test]
    fn parse_example_spp_frame() {
        let spp = SpacePacket::try_from(EXAMPLE_FRAME.to_vec()).unwrap();
        assert_eq!(spp.primary_header.packet_version_number, 0b000);
        assert_eq!(spp.primary_header.packet_type, PacketType::Telemetry);
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

        let _ = format!("{}", spp);
    }

    #[test]
    fn ensure_declarative_output_matches_imperative() {
        let imperative_spp = SpacePacket::try_from(EXAMPLE_FRAME.to_vec()).unwrap();
        let (_, declarative_spp) = SpacePacket::from_bytes((&EXAMPLE_FRAME, 0)).unwrap();

        assert_eq!(
            imperative_spp.primary_header,
            declarative_spp.primary_header
        )
    }

    #[test]
    fn too_short_spp_frame() {
        let frame = vec![0x0c, 0x00, 0xc0];

        let spp = SpacePacket::try_from(frame);
        assert!(spp.is_err());
    }
}
