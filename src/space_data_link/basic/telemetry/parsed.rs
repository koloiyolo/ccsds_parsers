use crate::space_data_link::basic::{
    errors::ParserError,
    telemetry::{crc::Crc16Check, primary_header::TransferFramePrimaryHeader},
};

pub struct ParsedTmTransferFrame {
    pub primary_header: TransferFramePrimaryHeader,
    pub crc_check: Crc16Check,
    pub frame: Vec<u8>,
}

impl TryFrom<Vec<u8>> for ParsedTmTransferFrame {
    type Error = ParserError;

    fn try_from(frame: Vec<u8>) -> Result<Self, Self::Error> {
        let primary_header = TransferFramePrimaryHeader::try_from(frame.as_slice())?;
        let crc_check = Crc16Check::try_from(frame.as_slice())?;
        Ok(Self {
            frame,
            primary_header,
            crc_check,
        })
    }
}

impl std::fmt::Display for ParsedTmTransferFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Parsed Telemetry Transfer Frame")?;
        writeln!(f)?;
        writeln!(f, "{}", self.primary_header)?;
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
                "CRC-16 Check:\tFailed\n",
                "  Checksum:\t0xcdff\n",
                "  Calculated:\t0x92e1\n",
            )
        );
    }
}
