use crate::{
    errors::ParserError,
    space_data_link::basic::telemetry::{
        crc::Crc16Check,
        operational_control_field::OperationalControlField,
        primary_header::{OperationalControlFieldFlag, TransferFramePrimaryHeader},
    },
};

pub struct ParsedTmTransferFrame {
    pub primary_header: TransferFramePrimaryHeader,
    pub operational_control_field: Option<OperationalControlField>,
    pub crc_check: Crc16Check,
    pub frame: Vec<u8>,
}

impl TryFrom<Vec<u8>> for ParsedTmTransferFrame {
    type Error = ParserError;

    fn try_from(frame: Vec<u8>) -> Result<Self, Self::Error> {
        let primary_header = TransferFramePrimaryHeader::try_from(frame.as_slice())?;
        let operational_control_field = match primary_header.operational_control_field_flag {
            OperationalControlFieldFlag::Present => {
                Some(OperationalControlField::try_from(frame.as_slice())?)
            }
            OperationalControlFieldFlag::NotPresent => None,
        };
        let crc_check = Crc16Check::try_from(frame.as_slice())?;
        Ok(Self {
            primary_header,
            operational_control_field,
            crc_check,
            frame,
        })
    }
}

impl std::fmt::Display for ParsedTmTransferFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Parsed Telemetry Transfer Frame")?;
        writeln!(f)?;
        writeln!(f, "{}", self.primary_header)?;
        if let Some(ocf) = self.operational_control_field {
            writeln!(f)?;
            writeln!(f, "{}", ocf)?;
        }
        writeln!(f)?;
        writeln!(f, "{}", self.crc_check)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telemetry_transfer_frame_has_readable_display() {
        let frame = ParsedTmTransferFrame::try_from(vec![0xa5, 0xb3, 0x12, 0x34, 0xab, 0xcd, 0xff])
            .expect("Checked frame");

        assert_eq!(
            frame.to_string(),
            concat!(
                "Parsed Telemetry Transfer Frame\n\n",
                "Primary Header:\n",
                "  Master Channel ID:\n",
                "    Transfer Frame Version Number: 2 (0x2)\n",
                "    Spacecraft ID: 603 (0x25b)\n",
                "  Virtual Channel ID: 1 (0x1)\n",
                "  Operational Control Field Flag: Present\n",
                "  Master Channel Frame Count: 18\n",
                "  Virtual Channel Frame Count: 52\n",
                "  Data Field Status:\n",
                "    Secondary Header Flag: Present\n",
                "    Synchronization Flag: 0\n",
                "    Packet Order Flag: 1\n",
                "    Segment Length ID: 0b1\n",
                "    First Header pointer: 973\n",
                "  Frame: a5b31234abcd\n\n",
                "Operational Control Field:\n",
                "  SDLS Frame Security Report\n",
                "    FSR Version Number: 0b11\n",
                "    Alarm Field: 0b0\n",
                "    Security Event Flags: 0b11\n",
                "    Last SPI Field: 4660 (0x1234)\n",
                "    ARSN: 171 (0xab)\n\n",
                "CRC-16 Check:\tFailed\n",
                "  Checksum:\t0xcdff\n",
                "  Calculated:\t0x92e1\n",
            )
        );
    }
}
