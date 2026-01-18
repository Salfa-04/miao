//! A lightweight binary framing protocol library.
//!
//! This crate provides a minimal, allocation-free framing layer
//! designed for embedded and resource-constrained environments.
//! It focuses on deterministic binary encoding, checksum validation,
//! and zero-copy decoding.
//!
//! # Architecture Overview
//!
//! - **`Validator`**
//!   Defines CRC algorithms used to validate frames.
//!
//! - **`DjiValidator`**
//!   A concrete validator using DJI-compatible CRC8 and CRC16.
//!
//! - **`Marshaler`**
//!   Describes how a typed payload is serialized into bytes and
//!   deserialized from raw payload data.
//!
//! - **`Messager`**
//!   Implements frame packing and unpacking, combining framing,
//!   validation, and payload marshaling.
//!
//! - **`RawFrame`**
//!   A validated, zero-copy view of a decoded frame.
//!
//! # Typical Usage
//!
//! 1. Implement `Marshaler` for your message types
//! 2. Create a `Messager` with a chosen `Validator`
//! 3. Use `pack` to encode frames
//! 4. Use `unpack` to decode frames into `RawFrame`
//! 5. Use `Marshaler` to convert `RawFrame` payloads to typed messages
//!
//! ---
//!
//! # Frame Layout
//!
//! ```text
//! +--------+--------+--------+--------+--------+---------+--------+
//! |  SOF   |  LEN   |  SEQ   |  CRC8  | CMD_ID |  DATA   | CRC16  |
//! +--------+--------+--------+--------+--------+---------+--------+
//! | 1 byte | 2 byte | 1 byte | 1 byte | 2 byte | N bytes | 2 byte |
//! +--------+--------+--------+--------+--------+---------+--------+
//! ```
//!
#![cfg_attr(not(test), no_std)]

#[cfg(all(feature = "defmt", feature = "log"))]
compile_error!("Features `defmt` and `log` are mutually exclusive.");

/// Logging abstraction layer.
#[allow(unused_imports)]
mod fmt {
    #[cfg(feature = "log")]
    pub use ::log::{debug, error, info, trace, warn};

    #[cfg(feature = "defmt")]
    pub use ::defmt::{debug, error, info, trace, warn};
}

pub use crc8_dji::calculate as calc_dji8;
pub use crc16_dji::calculate as calc_dji16;
pub use error::{Error, Result};
pub use frame::{DjiValidator, Marshaler, RawFrame, Validator};
pub use msger::Messager;

mod crc16_dji;
mod crc8_dji;
mod error;
mod frame;
mod msger;

mod private {
    pub use super::*;

    pub use core::error::Error as StdError;
    pub use core::fmt::{Display, Formatter, Result as FmtResult};
    pub use core::marker::PhantomData;
    pub use core::result::Result as StdResult;
}

#[cfg(test)]
mod tests;
