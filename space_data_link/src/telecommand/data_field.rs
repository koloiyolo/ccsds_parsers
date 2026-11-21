use deku::{DekuRead, DekuWrite};

/// The Transfer Frame Data Field shall follow, without gap, the Transfer Frame Primary Header.
/// The Transfer Frame Data Field, which shall contain an integral number of octets,
/// may vary in length up to a maximum of 1019 octets (1017 octets if the Frame Error Control
/// Field is present).
/// The Transfer Frame Data Field shall contain either an integral number of octets of
/// data corresponding to one Frame Data Unit (for a Type-D Transfer Frame) or an integral
/// number of octets of Control Command information (for a Type-C Transfer Frame)
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, DekuWrite, DekuRead)]
#[deku(ctx = "data_len: usize")]
pub struct TransferFrameDataField {
    #[deku(count = "data_len")]
    pub raw: Vec<u8>,
}
