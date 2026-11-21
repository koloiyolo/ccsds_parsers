use deku::{DekuRead, DekuWrite};

use crate::telecommand::{
    data_field::TransferFrameDataField, primary_header::TransferFramePrimaryHeader,
};

/// TC Transfer Frame shall encompass the major fields, positioned contiguously,
/// in the following sequence:
///
///   * Transfer Frame Header (5 octets, mandatory);
///   * Transfer Frame Data Field (up to 1019 or 1017 octets, mandatory);
///   * Frame Error Control Field (2 octets, optional).
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, DekuWrite, DekuRead)]
pub struct TelecommandTransferFrame {
    #[deku(assert = "primary_header.frame_length.tctf_without_header_length().is_some()")]
    pub primary_header: TransferFramePrimaryHeader,
    #[deku(
        ctx = "primary_header.frame_length.tctf_without_header_length().expect(\"invalid TCTF frame length\")"
    )]
    pub data_field: TransferFrameDataField,
}

#[cfg(test)]
mod tests {
    use deku::DekuContainerRead;

    use crate::telecommand::primary_header::{BypassFlag, ControlCommandFlag};

    use super::*;

    #[test]
    fn test_valid_tctf_frame_parsing() {
        let frame = vec![
            0x00u8, 0x12, 0x04, 0x11, 0x00, 0xC0, 0x18, 0x2A, 0xC0, 0x00, 0x00, 0x03, 0xDE, 0xAD,
            0xBE, 0xEF, 0xBD, 0x39,
        ];

        let (_, tctf) =
            TelecommandTransferFrame::from_bytes((&frame, 0)).expect("Const checked frame");

        assert_eq!(tctf.primary_header.version_number, 0);
        assert_eq!(tctf.primary_header.bypass_flag, BypassFlag::TypeA);
        assert_eq!(
            tctf.primary_header.control_command_flag,
            ControlCommandFlag::TypeD
        );
        assert_eq!(tctf.primary_header.rsvd_spare, 0);
        assert_eq!(tctf.primary_header.spacecraft_id, 18);
        assert_eq!(tctf.primary_header.virtual_channel_id, 0x01);
        assert_eq!(tctf.primary_header.frame_length, 17.into());
        assert_eq!(tctf.primary_header.frame_sequence_number, 0);
    }
}
