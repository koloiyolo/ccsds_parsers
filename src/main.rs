use ccsds_parsers::space_data_link::basic::telemetry::parsed::ParsedTmTransferFrame;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let frame = arguments.get(1);
    match frame {
        Some(frame) => match hex::decode(frame) {
            Ok(frame_bytes) => match ParsedTmTransferFrame::try_from(frame_bytes) {
                Ok(parsed_frame) => {
                    println!("{parsed_frame}")
                }
                Err(e) => eprintln!("Failed to parse provided frame. \nReason: {e:?}"),
            },
            Err(e) => eprintln!("Failed to decode provided frame: {e}"),
        },
        None => eprintln!("Frame not passed."),
    }
}
