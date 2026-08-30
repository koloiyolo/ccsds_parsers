# CCSDS Parsers

Collection of simple parsers used to extract metadata from CCSDS compliant frames.

## Example usage:

via `cargo`:

```bash
cargo run -- <YOUR_FRAME>
```

like:

```bash
cargo run -- abbaabbaff2137abbaabbaff2137
```

## Supported -- current status

### Space Data Link TM

- [x] Primary header parsing
- [ ] Secondary header parsing
- [ ] OCF Parsing
- [ ] CRC16 checksum check

### Space Data Link TC

N/A

### Space Packet

N/A

## Bibliography

[CCSDS Space Packet protocol]: https://ccsds.org/publications/bluebooks/entry/3264/
[CCSDS TM Space Data link protocol]: https://ccsds.org/publications/bluebooks/entry/3274/
[CCSDS TC Space Data link protocol]: https://ccsds.org/publications/bluebooks/entry/3261/
