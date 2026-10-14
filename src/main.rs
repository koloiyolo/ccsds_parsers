mod cli;

use ccsds_parsers::{
    space_data_link::telemetry::packet::ParsedTmTransferFrame, spacepacket::SpacePacket,
};

use crate::cli::{CcsdsProtocol, Cli};
use clap::Parser;

fn main() {
    let arguments = Cli::parse();
    let frame = arguments.frame;
    let offset = arguments.offset;
    let frame = match hex::decode(frame) {
        Ok(frame) => frame,
        Err(e) => {
            eprintln!("Failed to parse provided frame. \nReason: {e:?}");
            return;
        }
    };
    match arguments.protocol {
        CcsdsProtocol::DataLinkTm => {
            match ParsedTmTransferFrame::try_from(frame[offset..].to_vec()) {
                Ok(parsed_frame) => {
                    println!("{parsed_frame}")
                }
                Err(e) => eprintln!("Failed to parse provided frame. \nReason: {e:?}"),
            }
        }
        CcsdsProtocol::DataLinkTc => todo!("CLTU parser not yet implemented"),
        CcsdsProtocol::SpacePacket => match SpacePacket::try_from(frame[offset..].to_vec()) {
            Ok(spp) => println!("{spp}"),
            Err(e) => eprintln!("Failed to parse provided frame. \nReason: {e:?}"),
        },
    }
}
