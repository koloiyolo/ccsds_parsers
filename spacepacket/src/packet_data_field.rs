use crate::primary_header::SecondaryHeaderFlag;
use deku::{DekuRead, DekuWrite};

/// The Packet Data Field is mandatory and shall consist of at least one of the
/// following two fields, positioned contiguously, in the following sequence:
///
/// * Packet Secondary Header (variable length);
/// * User Data Field (variable length).
///
/// The Packet Data Field shall contain at least one octet
#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(ctx = "data_len: usize, secondary_header_flag: SecondaryHeaderFlag")]
pub struct PacketDataField {
    #[deku(count = "data_len")]
    pub raw: Vec<u8>,
}
