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

```bash
ccsds_parsers -p <PROTOCOL> <YOUR_FRAME>
```

like:

```bash
ccsds_parsers -p tm_datalink a5b31234abcdff
```

Run with flag `--help` for all available options:

```bash
ccsds_parsers --help
```

## Supported -- current status

### Space Data Link TM

- [x] Primary header parsing
- [ ] Secondary header parsing
- [ ] OCF Parsing
- [x] CRC16 checksum check

### Space Data Link TC

N/A

### Space Packet

N/A

## Bibliography

- [CCSDS Space Packet protocol]
- [CCSDS TM Space Data link protocol]
- [CCSDS TC Space Data link protocol]
- [CCSDS Space Data Link Security Protocol]
- [CCSDS Space Data Link Security Protocol—Extended Procedures]

[CCSDS Space Packet protocol]: https://ccsds.org/publications/bluebooks/entry/3264/
[CCSDS TM Space Data link protocol]: https://ccsds.org/publications/bluebooks/entry/3274/
[CCSDS TC Space Data link protocol]: https://ccsds.org/publications/bluebooks/entry/3261/
[CCSDS Space Data Link Security Protocol]: https://ccsds.org/publications/bluebooks/entry/3258/
[CCSDS Space Data Link Security Protocol—Extended Procedures]: https://ccsds.org/publications/bluebooks/entry/3259/
