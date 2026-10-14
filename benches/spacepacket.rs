use std::hint::black_box;
use std::time::Instant;

use ccsds_parsers::spacepacket::SpacePacket;

fn main() {
    let packet = vec![
        0x0c, 0x00, 0xc0, 0x01, 0x00, 0x0a, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x53, 0x70, 0x61, 0x63,
        0x65,
    ];
    let iterations = 10_000_000;

    let start = Instant::now();

    for _ in 0..iterations {
        let result = SpacePacket::try_from(black_box(packet.clone()));
        let _ = black_box(result);
    }

    let elapsed = start.elapsed();

    println!("Total: {elapsed:?}");
    println!(
        "Per iteration: {:.3} ns",
        elapsed.as_nanos() as f64 / iterations as f64
    );
}
