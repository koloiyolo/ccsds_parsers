mod cli;

use ccsds_parsers::space_data_link::basic::telemetry::parsed::ParsedTmTransferFrame;

use crate::cli::{CcsdsProtocol, Cli};
use clap::Parser;

fn main() {
    let arguments = Cli::parse();
    let frame = arguments.frame;
    match arguments.protocol {
        CcsdsProtocol::DataLinkTm => match hex::decode(frame) {
            Ok(frame_bytes) => match ParsedTmTransferFrame::try_from(frame_bytes) {
                Ok(parsed_frame) => {
                    println!("{parsed_frame}")
                }
                Err(e) => eprintln!("Failed to parse provided frame. \nReason: {e:?}"),
            },
            Err(e) => eprintln!("Failed to decode provided frame: {e}"),
        },
        CcsdsProtocol::DataLinkTc => todo!("CLTU parser not yet implemented"),
        CcsdsProtocol::SpacePacket => todo!("SpacePacket parser not yet implemented"),
    }
}
