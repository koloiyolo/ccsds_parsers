use crate::{errors::ParserError, utils::extract_value_from_left};

const OCF_START_IDX_FROM_RIGHT: usize = 6;
const OCF_END_DIX_FROM_RIGHT: usize = 2;

/// https://ccsds.org/publications/bluebooks/entry/3259/
#[derive(Debug, Clone, Copy)]
pub struct SdlsFrameSecurityReport {
    pub fsr_version_number: u8,
    pub alarm_field: u8,
    pub security_event_flags: u8,
    pub last_spi_field: u16,
    pub arsn: u8,
}

impl std::fmt::Display for SdlsFrameSecurityReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  SDLS Frame Security Report")?;
        writeln!(f, "    FSR Version Number: {:#b}", self.fsr_version_number)?;
        writeln!(f, "    Alarm Field: {:#b}", self.alarm_field)?;
        writeln!(
            f,
            "    Security Event Flags: {:#b}",
            self.security_event_flags
        )?;
        writeln!(
            f,
            "    Last SPI Field: {} ({:#x})",
            self.last_spi_field, self.last_spi_field
        )?;
        write!(f, "    ARSN: {} ({:#x})", self.arsn, self.arsn)
    }
}

impl TryFrom<&[u8]> for SdlsFrameSecurityReport {
    type Error = ParserError;

    fn try_from(frame: &[u8]) -> Result<Self, Self::Error> {
        let frame_len = frame.len();
        let ocf_frame =
            &frame[frame_len - OCF_START_IDX_FROM_RIGHT..frame_len - OCF_END_DIX_FROM_RIGHT];
        let fsr_head = u16::from_be_bytes([ocf_frame[0], ocf_frame[1]]);
        let fsr_version_number = extract_value_from_left(fsr_head, 1, 3) as u8;
        let alarm_field = extract_value_from_left(fsr_head, 4, 1) as u8;
        let security_event_flags = extract_value_from_left(fsr_head, 5, 3) as u8;
        let last_spi_field = u16::from_be_bytes([ocf_frame[1], ocf_frame[2]]);
        let arsn = ocf_frame[3];

        Ok(Self {
            fsr_version_number,
            alarm_field,
            security_event_flags,
            last_spi_field,
            arsn,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OperationalControlField {
    Clcw(u32),
    SdlsFsr(SdlsFrameSecurityReport),
}

impl std::fmt::Display for OperationalControlField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Operational Control Field:")?;
        match self {
            Self::Clcw(clcw) => write!(f, "  CCSDS Clcw:\t{:#x}", clcw),
            Self::SdlsFsr(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<&[u8]> for OperationalControlField {
    type Error = ParserError;

    fn try_from(frame: &[u8]) -> Result<Self, Self::Error> {
        let frame_len = frame.len();
        let ocf_frame =
            &frame[frame_len - OCF_START_IDX_FROM_RIGHT..frame_len - OCF_END_DIX_FROM_RIGHT];
        let ocf_flag =
            extract_value_from_left(u16::from_be_bytes([ocf_frame[0], ocf_frame[1]]), 0, 1);

        match ocf_flag {
            0b0 => Ok(Self::Clcw(u32::from_be_bytes(
                ocf_frame.try_into().expect(""),
            ))),
            0b1 => Ok(Self::SdlsFsr(SdlsFrameSecurityReport::try_from(frame)?)),
            _ => Err(ParserError::InvalidBinaryValue(ocf_flag as u32)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operational_control_field_clcw() {
        let frame = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x34];
        let ocf = OperationalControlField::try_from(frame.as_slice()).unwrap();

        match ocf {
            OperationalControlField::Clcw(value) => assert_eq!(value, 0x0000),
            _ => panic!("Expected Clcw variant"),
        }
    }
}
