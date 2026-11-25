# CCSDS Parsers

[![ci](https://github.com/koloiyolo/ccsds_parsers/actions/workflows/ci.yml/badge.svg)](https://github.com/koloiyolo/ccsds_parsers/actions/workflows/ci.yml)
[![coverage](coverage.svg)](https://github.com/GoogleCloudPlatform/lcov-coverage-badge)

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
ccsds_parsers -p tmtf a5b31234abcdff
```

```bash
ccsds_parsers -p spp 0C00C001000A48656C6C6F5370616365
```

Run with flag `--help` for all available options:

```bash
ccsds_parsers --help
```

## Test coverage

Coverage is measured with [cargo-llvm-cov] in CI.
To generate an HTML report locally (the Nix dev shell includes `cargo-llvm-cov`):

```bash
cargo llvm-cov --workspace --html --output-dir coverage
```

Open `coverage/html/index.html` to view the report.

[cargo-llvm-cov]: https://github.com/taiki-e/cargo-llvm-cov

## Supported -- current status

### Space Data Link TM

- [x] Primary header parsing
- [ ] Secondary header parsing
- [x] OCF Parsing
- [x] CRC16 checksum check
- [ ] Telemetry Channel Coding

### Space Data Link TC

- [x] Primary header parsing

### Space Packet

- [x] Primary header parsing
- [ ] Secondary header parsing

## Bibliography

- [CCSDS Space Packet protocol]
- [CCSDS TM Space Data link protocol]
- [CCSDS TC Space Data link protocol]
- [CCSDS Space Data Link Security Protocol]
- [CCSDS Space Data Link Security Protocol—Extended Procedures]

[CCSDS Space Packet protocol]: https://ccsds.org/publications/bluebooks/entry/3264/
[CCSDS TM Space Data link protocol]: https://ccsds.org/publications/bluebooks/entry/3274/
[CCSDS TC Space Data link protocol]: https://ccsds.org/publications/bluebooks/entry/3279/
[CCSDS Space Data Link Security Protocol]: https://ccsds.org/publications/bluebooks/entry/3258/
[CCSDS Space Data Link Security Protocol—Extended Procedures]: https://ccsds.org/publications/bluebooks/entry/3259/
