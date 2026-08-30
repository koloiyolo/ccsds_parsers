use crate::{space_data_link::basic::errors::ParserError, utils::extract_value_from_left};

#[derive(Debug, Clone, Copy)]
pub enum OperationalControlFieldFlag {
    NotPresent = 0,
    Present = 1,
}

impl std::fmt::Display for OperationalControlFieldFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NotPresent => "Not present",
            Self::Present => "Present",
        })
    }
}

impl TryFrom<u8> for OperationalControlFieldFlag {
    type Error = ParserError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let result = match value {
            0 => OperationalControlFieldFlag::NotPresent,
            1 => OperationalControlFieldFlag::Present,
            _ => Err(ParserError::ValueOutOfBounds {
                value: value.to_string(),
                parsed_type: "OperationalControlFieldFlag".into(),
            })?,
        };
        Ok(result)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SecondaryHeaderFlag {
    NotPresent = 0,
    Present = 1,
}

impl std::fmt::Display for SecondaryHeaderFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NotPresent => "Not present",
            Self::Present => "Present",
        })
    }
}

impl TryFrom<u8> for SecondaryHeaderFlag {
    type Error = ParserError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let result = match value {
            0 => SecondaryHeaderFlag::NotPresent,
            1 => SecondaryHeaderFlag::Present,
            _ => Err(ParserError::ValueOutOfBounds {
                value: value.to_string(),
                parsed_type: "SecondaryHeaderFlag".into(),
            })?,
        };
        Ok(result)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MasterChannelIdentifier {
    pub transfer_frame_version_number: u8,
    pub spacecraft_id: u16,
}

impl std::fmt::Display for MasterChannelIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "    Transfer Frame Version Number: {} ({:#x})",
            self.transfer_frame_version_number, self.transfer_frame_version_number,
        )?;
        write!(
            f,
            "    Spacecraft ID: {} ({:#x})",
            self.spacecraft_id, self.spacecraft_id
        )
    }
}

impl From<u16> for MasterChannelIdentifier {
    fn from(value: u16) -> Self {
        let transfer_frame_version_number = extract_value_from_left(value, 0, 2) as u8;
        let spacecraft_id = extract_value_from_left(value, 2, 10);
        MasterChannelIdentifier {
            transfer_frame_version_number,
            spacecraft_id,
        }
    }
}

const TF_DATA_FIELD_STATUS_BIT_OFFSET: u16 = 32;

#[derive(Debug, Clone, Copy)]
pub struct TransferFrameDataFieldStatus {
    pub secondary_header_flag: SecondaryHeaderFlag,
    pub synchronization_flag: u8,
    pub packet_order_flag: u8,
    pub segment_length_id: u8,
    pub first_header_pointer: u16,
}

impl std::fmt::Display for TransferFrameDataFieldStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "    Secondary Header Flag: {}",
            self.secondary_header_flag
        )?;
        writeln!(f, "    Synchronization Flag: {}", self.synchronization_flag)?;
        writeln!(f, "    Packet Order Flag: {}", self.packet_order_flag)?;
        writeln!(f, "    Segment Length ID: {:#b}", self.segment_length_id)?;
        write!(f, "    First Header pointer: {}", self.first_header_pointer)
    }
}

impl TryFrom<&[u8]> for TransferFrameDataFieldStatus {
    type Error = ParserError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let tf_data_field = u16::from_be_bytes([value[4], value[5]]);
        let secondary_header_flag = SecondaryHeaderFlag::try_from(extract_value_from_left(
            tf_data_field,
            32 - TF_DATA_FIELD_STATUS_BIT_OFFSET,
            1,
        ) as u8)?;
        let synchronization_flag =
            extract_value_from_left(tf_data_field, 33 - TF_DATA_FIELD_STATUS_BIT_OFFSET, 1) as u8;
        let packet_order_flag =
            extract_value_from_left(tf_data_field, 34 - TF_DATA_FIELD_STATUS_BIT_OFFSET, 1) as u8;
        let segment_length_id =
            extract_value_from_left(tf_data_field, 35 - TF_DATA_FIELD_STATUS_BIT_OFFSET, 2) as u8;
        let first_header_pointer =
            extract_value_from_left(tf_data_field, 37 - TF_DATA_FIELD_STATUS_BIT_OFFSET, 11);
        Ok(Self {
            secondary_header_flag,
            synchronization_flag,
            packet_order_flag,
            segment_length_id,
            first_header_pointer,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TransferFramePrimaryHeader {
    pub master_channel_id: MasterChannelIdentifier,
    pub virtual_channel_id: u8,
    pub operational_control_field_flag: OperationalControlFieldFlag,
    pub master_channel_frame_count: u8,
    pub virtual_channel_frame_count: u8,
    pub transfer_frame_data_field_status: TransferFrameDataFieldStatus,
    pub frame: Vec<u8>,
}

impl std::fmt::Display for TransferFramePrimaryHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Primary Header:")?;
        writeln!(f, "  Master Channel ID:")?;
        write!(f, "{}", self.master_channel_id)?;
        writeln!(f)?;
        writeln!(
            f,
            "  Virtual Channel ID: {} ({:#x})",
            self.virtual_channel_id, self.virtual_channel_id
        )?;
        writeln!(
            f,
            "  Operational Control Field Flag: {}",
            self.operational_control_field_flag
        )?;
        writeln!(
            f,
            "  Master Channel Frame Count: {}",
            self.master_channel_frame_count
        )?;
        writeln!(
            f,
            "  Virtual Channel Frame Count: {}",
            self.virtual_channel_frame_count
        )?;
        writeln!(
            f,
            "  Data Field Status:\n{}",
            self.transfer_frame_data_field_status
        )?;
        write!(f, "  Frame: {}", hex::encode(&self.frame))
    }
}

impl TryFrom<&[u8]> for TransferFramePrimaryHeader {
    type Error = ParserError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let master_channel_frame_count = value[2];
        let virtual_channel_frame_count = value[3];
        let primary_header_head = u16::from_be_bytes([value[0], value[1]]);
        let master_channel_id = MasterChannelIdentifier::from(primary_header_head);
        let virtual_channel_id = extract_value_from_left(primary_header_head, 12, 3) as u8;
        let operational_control_field_flag = OperationalControlFieldFlag::try_from(
            extract_value_from_left(primary_header_head, 15, 1) as u8,
        )?;
        let transfer_frame_data_field_status = TransferFrameDataFieldStatus::try_from(value)?;
        let frame = value[..6].to_vec();
        Ok(Self {
            master_channel_id,
            virtual_channel_id,
            operational_control_field_flag,
            master_channel_frame_count,
            virtual_channel_frame_count,
            transfer_frame_data_field_status,
            frame,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operational_control_field_flag_is_converted() {
        assert!(matches!(
            OperationalControlFieldFlag::try_from(0).expect("Checked value"),
            OperationalControlFieldFlag::NotPresent
        ));
        assert!(matches!(
            OperationalControlFieldFlag::try_from(1).expect("Checked value"),
            OperationalControlFieldFlag::Present
        ));
        assert!(OperationalControlFieldFlag::try_from(3).is_err())
    }

    #[test]
    fn test_secondary_header_flag_is_converted() {
        assert!(matches!(
            SecondaryHeaderFlag::try_from(0).expect("Checked value"),
            SecondaryHeaderFlag::NotPresent
        ));
        assert!(matches!(
            SecondaryHeaderFlag::try_from(1).expect("Checked value"),
            SecondaryHeaderFlag::Present
        ));
        assert!(SecondaryHeaderFlag::try_from(3).is_err())
    }

    #[test]
    fn master_channel_identifier_is_parsed() {
        let value = 0xa5b3;
        let identifier = MasterChannelIdentifier::from(value);

        assert_eq!(identifier.transfer_frame_version_number, 2);
        assert_eq!(identifier.spacecraft_id, 0b1001011011);
    }

    #[test]
    fn transfer_frame_primary_header_is_parsed() {
        let frame = [0xa5_u8, 0xb3, 0x12, 0x34, 0xab, 0xcd, 0x00, 0x00, 0xFF];
        let header = TransferFramePrimaryHeader::try_from(&frame[..6]).expect("Checked value");

        assert_eq!(header.master_channel_id.transfer_frame_version_number, 2);
        assert_eq!(header.master_channel_id.spacecraft_id, 0b1001011011);
        assert_eq!(header.virtual_channel_id, 1);
        assert!(matches!(
            header.operational_control_field_flag,
            OperationalControlFieldFlag::Present
        ));
        assert_eq!(header.master_channel_frame_count, 0x12);
        assert_eq!(header.virtual_channel_frame_count, 0x34);
        // assert_eq!(header.transfer_frame_data_field_status, 0xabcd);
    }
}
