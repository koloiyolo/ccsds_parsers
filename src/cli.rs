#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Raw CCSDS compliant frame to be parsed.
    #[arg()]
    pub frame: String,
    /// Highest level of CCSDS protocol used to parse the provided packet.
    #[arg(short, long, value_enum, ignore_case = true)]
    pub protocol: CcsdsProtocol,
    /// Byte offset from which to parse the provided frame.
    /// Useful when scripting against vendor specific setups.
    #[arg(long, default_value_t = 0)]
    pub offset: usize,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum CcsdsProtocol {
    #[value(
        name = "space_datalink_tm",
        alias = "spacedatalinktm",
        alias = "cadu",
        alias = "datalinktm",
        alias = "data_link_tm",
        alias = "tm_datalink"
    )]
    DataLinkTm,

    #[value(
        name = "space_datalink_tc",
        alias = "cltu",
        alias = "spacedatalinktc",
        alias = "datalinktc",
        alias = "data_link_tc",
        alias = "tc_datalink"
    )]
    DataLinkTc,
    #[value(name = "spacepacket", alias = "spp", alias = "space_packet")]
    SpacePacket,
}
