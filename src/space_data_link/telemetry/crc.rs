use crate::errors::ParserError;

const CRC16_BYTE_LEN: usize = 2;
const CRC16_1ST_BYTE_OFFSET_FROM_END: usize = 2;
const CRC16_2ND_BYTE_OFFSET_FROM_END: usize = 1;

#[derive(Debug, Clone)]
pub enum Crc16Check {
    Passed { checksum: u16 },
    Failed { checksum: u16, calculated: u16 },
}

impl std::fmt::Display for Crc16Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Crc16Check::Passed { checksum } => {
                writeln!(f, "CRC-16 Check:\tPassed")?;
                write!(f, "  Checksum:\t{:#x}", checksum)
            }
            Crc16Check::Failed {
                checksum,
                calculated,
            } => {
                writeln!(f, "CRC-16 Check:\tFailed")?;
                writeln!(f, "  Checksum:\t{:#x}", checksum)?;
                write!(f, "  Calculated:\t{:#x}", calculated)
            }
        }
    }
}

impl TryFrom<&[u8]> for Crc16Check {
    type Error = ParserError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < CRC16_BYTE_LEN {
            return Err(ParserError::CrcCheckFailed {
                reason: "Checksum retrieval failed.".to_string(),
            });
        }
        let checksum = u16::from_be_bytes([
            value[value.len() - CRC16_1ST_BYTE_OFFSET_FROM_END],
            value[value.len() - CRC16_2ND_BYTE_OFFSET_FROM_END],
        ]);
        let calculated = crc16::State::<crc16::CCITT_FALSE>::calculate(
            &value[..value.len() - CRC16_1ST_BYTE_OFFSET_FROM_END],
        );
        let crc_check = if checksum == calculated {
            Crc16Check::Passed { checksum }
        } else {
            Crc16Check::Failed {
                checksum,
                calculated,
            }
        };
        Ok(crc_check)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_crc_err() {
        let too_short_value: [u8; 1] = [0xff];
        assert!(Crc16Check::try_from(too_short_value.as_slice()).is_err());
    }

    #[test]
    fn test_calculate_crc_failed() {
        let too_short_value: [u8; 1] = [0xff];
        assert!(Crc16Check::try_from(too_short_value.as_slice()).is_err());

        let invalid_crc: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
        let crc_check = Crc16Check::try_from(invalid_crc.as_slice()).expect("Checked value");
        let crc_check_display = format!("{}", crc_check);
        assert!(matches!(
            crc_check,
            Crc16Check::Failed {
                calculated: _,
                checksum: _
            }
        ));
        assert_eq!(
            crc_check_display,
            "CRC-16 Check:\tFailed\n  Checksum:\t0xffff\n  Calculated:\t0x0"
        );
    }
    #[test]
    fn test_calculate_crc_passed() {
        let valid_crc: [u8; 4] = [0xff, 0xff, 0x00, 0x00];
        let crc_check = Crc16Check::try_from(valid_crc.as_slice()).expect("Checked value");
        let crc_check_display = format!("{}", crc_check);
        assert!(matches!(crc_check, Crc16Check::Passed { checksum: 0x0 }));
        assert_eq!(crc_check_display, "CRC-16 Check:\tPassed\n  Checksum:\t0x0");
    }
}
