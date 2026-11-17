//! Contains implementations for Space Data link protocol for telemetry.
//!
//! https://ccsds.org/publications/bluebooks/entry/3274/

pub mod packet;

mod crc;
mod operational_control_field;
mod primary_header;
