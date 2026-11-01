use std::hint::black_box;
use std::time::Instant;

use deku::DekuContainerRead;
use spacepacket::primary_header::PrimaryHeader;

const NANOS_IN_SECOND: f64 = 1_000_000_000.0;

fn main() {
    let packet = vec![
        0x0c, 0x00, 0xc0, 0x01, 0x00, 0x0a, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x53, 0x70, 0x61, 0x63,
        0x65,
    ];
    let iterations = 10_000_000;

    let start = Instant::now();
    for _ in 0..iterations {
        let result = PrimaryHeader::try_from(black_box(&packet));
        let _ = black_box(result);
    }
    let imperative_elapsed = start.elapsed();

    let declarative_approach_input: (&[u8], usize) = (black_box(&packet), 0);
    let start = Instant::now();
    for _ in 0..iterations {
        let result = PrimaryHeader::from_bytes(declarative_approach_input);
        let _ = black_box(result);
    }
    let declarative_elapsed = start.elapsed();

    println!(
        "\nSpacepacket Primary Header extraction (Imperative) vs (Declarative) throughput p/s"
    );
    let header = "|    Type     | Throughput (p/s) \t| Total \t| Iteration \t| ";
    println!("{header}");
    let width = header.chars().fold(0, |column, ch| {
        if ch == '\t' {
            (column / 8 + 1) * 8
        } else {
            column + 1
        }
    });
    println!("{:-<width$}", "", width = width);
    println!(
        "| Imperative  | {:.3} p/s\t| {imperative_elapsed:?}\t| {:.3}ns\t|",
        iterations as f64 / imperative_elapsed.as_nanos() as f64 * NANOS_IN_SECOND,
        imperative_elapsed.as_nanos() as f64 / iterations as f64
    );
    println!(
        "| Declarative | {:.3} p/s\t\t| {declarative_elapsed:?}\t| {:.3}ns\t|",
        iterations as f64 / declarative_elapsed.as_nanos() as f64 * NANOS_IN_SECOND,
        declarative_elapsed.as_nanos() as f64 / iterations as f64
    );
    println!();
}
