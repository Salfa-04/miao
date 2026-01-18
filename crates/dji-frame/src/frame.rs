//!
//! Frame-related core abstractions.
//!
//! This module defines the fundamental building blocks used by the
//! framing layer of the protocol. It does not perform I/O or buffering
//! itself, but provides the types and traits required to encode, decode,
//! and validate framed messages.
//!
//! Provided abstractions:
//!
//! - **`Validator`**
//!   Defines checksum algorithms used by the protocol (CRC8 / CRC16).
//!   This allows different protocol variants to reuse the same framing
//!   logic with different validation rules.
//!
//! - **`DjiValidator`**
//!   A concrete `Validator` implementation using DJI-compatible CRC
//!   algorithms.
//!
//! - **`Marshaler`**
//!   Describes how a payload type is serialized into bytes and restored
//!   from raw payload data. Each implementation is bound to a fixed
//!   command ID and is independent from framing details.
//!
//! - **`RawFrame`**
//!   A lightweight view of a decoded frame, exposing the command ID,
//!   sequence number, and a borrowed payload slice without allocation.
//!

use crate::private::*;

///
/// CRC validator abstraction for the frame protocol.
///
/// Implementations define how frame integrity is verified:
/// - CRC8 for the frame header
/// - CRC16 for the frame body
///
pub trait Validator {
    ///
    /// Calculate CRC8 over the given raw bytes.
    ///
    /// Typically used for validating the frame header.
    ///
    fn calculate_crc8(raw: &[u8]) -> u8;
    ///
    /// Calculate CRC16 over the given raw bytes.
    ///
    /// Typically used for validating the full frame
    /// (header + command + payload).
    ///
    fn calculate_crc16(raw: &[u8]) -> u16;
}

///
/// DJI protocol CRC validator.
///
/// This implementation uses DJI-compatible CRC8 and CRC16
/// algorithms for frame validation.
///
pub struct DjiValidator;

impl Validator for DjiValidator {
    fn calculate_crc8(raw: &[u8]) -> u8 {
        calc_dji8(raw)
    }

    fn calculate_crc16(raw: &[u8]) -> u16 {
        calc_dji16(raw)
    }
}

///
/// Payload marshaling interface.
///
/// `Marshaler` defines how a message payload is:
/// - Serialized into raw bytes (`marshal`)
/// - Deserialized from raw bytes (`unmarshal`)
///
/// Each payload type corresponds to exactly one command ID.
///
pub trait Marshaler: Sized {
    /// Command ID associated with this payload type.
    const CMD_ID: u16;
    ///
    /// Serialize the payload into the destination buffer.
    ///
    /// Returns the number of bytes written on success.
    ///
    /// # Errors
    ///
    /// Returns an error if the destination buffer
    /// is too small or the payload cannot be encoded.
    ///
    fn marshal(&self, dst: &mut [u8]) -> Result<usize>;
    ///
    /// Deserialize a payload from raw bytes.
    ///
    /// The input slice contains only the payload portion
    /// (no header, command ID, or CRC).
    ///
    /// Implementations must not depend on framing details.
    ///
    /// # Errors
    ///
    /// Returns an error if the data is invalid
    /// or does not match the expected payload format.
    ///
    fn unmarshal(raw: &[u8]) -> Result<Self>;
}

///
/// A validated but undecoded frame.
///
/// `RawFrame` represents a frame that has:
/// - Passed structural checks
/// - Passed CRC validation
///
/// The payload is kept as a raw byte slice and can later
/// be decoded using the appropriate `Marshaler`.
///
/// # Lifetime
///
/// The payload slice borrows from the original input buffer.
///
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawFrame<'t> {
    /// Command ID of the frame.
    pub(crate) cmd_id: u16,
    /// Sequence number of the frame.
    pub(crate) sequence: u8,
    /// Raw payload bytes.
    pub(crate) payload: &'t [u8],
}

impl RawFrame<'_> {
    /// Get the command ID of this frame.
    #[inline]
    pub fn cmd_id(&self) -> u16 {
        self.cmd_id
    }

    /// Get the sequence number of this frame.
    #[inline]
    pub fn sequence(&self) -> u8 {
        self.sequence
    }
}

impl<'t> RawFrame<'t> {
    ///
    /// Get the raw payload bytes.
    ///
    /// The returned slice does not include framing,
    /// command ID, or CRC fields.
    ///
    #[inline]
    pub fn payload(&self) -> &'t [u8] {
        self.payload
    }
}
