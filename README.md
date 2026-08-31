# CCSDS Parsers

Collection of simple parsers used to extract metadata from CCSDS compliant frames.

## Installation

via `cargo`:

```bash
cargo install --git https://github.com/koloiyolo/ccsds_parsers
```

### Nix

via `home-manager`:

```nix
{ pkgs, ... }:

{
  home.packages = [

    (pkgs.callPackage (pkgs.fetchFromGitHub {
      owner = "koloiyolo";
      repo = "ccsds_parsers";
      rev = "c98856fad74f3f077a698a96bbda63cf691446e8"; # commit hash or tag.
      hash = "sha256-r823eqXhVNlXuJOxlb8STTpt0O3Qrv61QQCJRV1p+oA=";
      # sha256 checksum, please correct if nix fails the check.
    }) { })
  ];
}
```

## Example usage

via `cargo`:

```bash
cargo run -- <YOUR_FRAME>
```

like:

```bash
cargo run -- abbaabbaff2137abbaabbaff2137
```

Run with flag `--help` for all available options:

```bash
cargo run -- --help
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

- [CCSDS Space Packet protocol]
- [CCSDS TM Space Data link protocol]
- [CCSDS TC Space Data link protocol]

[CCSDS Space Packet protocol]: https://ccsds.org/publications/bluebooks/entry/3264/
[CCSDS TM Space Data link protocol]: https://ccsds.org/publications/bluebooks/entry/3274/
[CCSDS TC Space Data link protocol]: https://ccsds.org/publications/bluebooks/entry/3261/
