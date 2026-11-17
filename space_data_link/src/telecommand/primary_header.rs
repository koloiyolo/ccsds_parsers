use deku::{DekuRead, DekuWrite};

#[allow(unused)]
pub const TCTF_PRIMARY_HEADER_SIZE: usize = 5;

/// The Transfer Frame Primary Header is mandatory and shall consist of eight fields,
/// positioned contiguously, in the following sequence:
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, DekuWrite, DekuRead)]
pub struct TransferFramePrimaryHeader {
    /// Transfer Frame Version Number (2 bits, mandatory)
    #[deku(bits = 2)]
    pub version_number: u8,
    /// Bypass Flag (1 bit, mandatory)
    pub bypass_flag: BypassFlag,
    /// Control Command Flag (1 bit, mandatory)
    pub control_command_flag: ControlCommandFlag,
    /// Reserved Spare (2 bits, mandatory).
    /// These two bits are reserved for future application and shall be set to ‘00
    #[deku(bits = 2)]
    pub rsvd_spare: u8,
    /// Spacecraft Identifier (10 bits, mandatory)
    #[deku(bits = 10)]
    pub spacecraft_id: u16,
    /// Virtual Channel Identifier (6 bits, mandatory)
    #[deku(bits = 6)]
    pub virtual_channel_id: u8,
    /// Frame Length (10 bits, mandatory)
    #[deku(bits = 10)]
    pub frame_length: u16,
    /// Frame Sequence Number (8 bits, mandatory)
    #[deku(bytes = 1)]
    pub frame_sequence_number: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, DekuWrite, DekuRead)]
#[repr(u8)]
#[deku(
    bits = "1",
    id_type = "u8",
    ctx = "_: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// The single-bit Bypass Flag shall be used to control
/// the application of Frame Acceptance Checks by the receiving end:
///
///  * setting the Bypass Flag to value ‘0’ shall specify that this Transfer Frame is a Type-A
///    Transfer Frame, and acceptance of this Transfer Frame by the receiving end shall be
///    subject to the normal Frame Acceptance Checks of the FARM;
///  * setting the Bypass Flag to value ‘1’ shall specify that this Transfer Frame is a Type-B
///    Transfer Frame, and the Frame Acceptance Checks of the FARM by the receiving end shall be bypassed.
pub enum BypassFlag {
    TypeA = Self::TYPE_A_VALUE,
    TypeB = Self::TYPE_B_VALUE,
}

impl BypassFlag {
    const TYPE_A_VALUE: u8 = 0;
    const TYPE_B_VALUE: u8 = 1;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, DekuWrite, DekuRead)]
#[repr(u8)]
#[deku(
    bits = "1",
    id_type = "u8",
    ctx = "_: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// The single-bit Control Command Flag shall be used to specify whether the Data
/// Field of the Transfer Frame is conveying Control Commands (Type-C) or Data (Type-D):
///  * setting the Control Command Flag to value ‘0’ shall indicate that the Transfer Frame
///    Data Field contains a Frame Data Unit (FDU) (Type-D);
///  * setting the Control Command Flag to value ‘1’ shall indicate that the Transfer Frame
///    Data Field contains control information (Type-C).
pub enum ControlCommandFlag {
    TypeD = Self::TYPE_D_VALUE,
    TypeC = Self::TYPE_C_VALUE,
}

impl ControlCommandFlag {
    const TYPE_D_VALUE: u8 = 0;
    const TYPE_C_VALUE: u8 = 1;
}
