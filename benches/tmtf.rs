use std::hint::black_box;
use std::time::Instant;

use ccsds_parsers::space_data_link::telemetry::packet::ParsedTmTransferFrame;

fn main() {
    let packet = vec![0xa5, 0xb3, 0x12, 0x34, 0xab, 0xcd, 0xff];
    let iterations = 10_000_000;

    let start = Instant::now();

    for _ in 0..iterations {
        let result = ParsedTmTransferFrame::try_from(black_box(packet.clone()));
        let _ = black_box(result);
    }

    let elapsed = start.elapsed();

    println!("Total: {elapsed:?}");
    println!(
        "Per iteration: {:.3} ns",
        elapsed.as_nanos() as f64 / iterations as f64
    );
}
