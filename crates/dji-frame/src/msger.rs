//!
//! Frame packing and unpacking logic.
//!
//! This module implements the framing layer responsible for:
//! - Constructing binary frames from typed payloads
//! - Parsing and validating raw frames from byte streams
//!
//! It combines:
//! - A `Validator` for CRC verification
//! - A `Marshaler` for payload serialization
//!
//! The framing format includes:
//! - Start-of-frame marker
//! - Payload length
//! - Sequence number
//! - Command ID
//! - CRC-protected payload
//!
//! This module performs no I/O and allocates no memory.
//! All operations are buffer-oriented and suitable for
//! embedded and `no_std` environments.
//!

use crate::private::*;

/// Start of Frame Byte
const SOF: u8 = 0xA5;

/// Size of the frame header (SOF + length + sequence + CRC8).
const HEAD_SIZE: usize = 5;
/// Size of the command ID field.
const CMDID_SIZE: usize = 2;
/// Size of the tail CRC field.
const TAIL_SIZE: usize = 2;

///
/// Frame encoder and decoder.
///
/// `Messager` is responsible for packing typed messages into
/// binary frames and unpacking validated frames from raw bytes.
///
/// It is generic over a `Validator`, allowing different CRC
/// implementations to be used with the same framing logic.
///
/// The internal sequence counter is automatically incremented
/// on each successful call to `pack`.
///
/// # Frame Layout
///
/// ```text
/// +--------+--------+--------+--------+--------+---------+--------+
/// |  SOF   |  LEN   |  SEQ   |  CRC8  | CMD_ID |  DATA   | CRC16  |
/// +--------+--------+--------+--------+--------+---------+--------+
/// | 1 byte | 2 byte | 1 byte | 1 byte | 2 byte | N bytes | 2 byte |
/// +--------+--------+--------+--------+--------+---------+--------+
/// ```
///
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Messager<V: Validator> {
    /// Current frame sequence number.
    sequence: u8,
    /// Marker for the validator type.
    _marker: PhantomData<V>,
}

impl<V: Validator> Messager<V> {
    /// Create a new `Messager` with the given initial sequence number.
    pub const fn new(seq: u8) -> Self {
        Self {
            sequence: seq,
            _marker: PhantomData,
        }
    }

    ///
    /// Pack a message into a binary frame.
    ///
    /// This method serializes a payload using its `Marshaler`
    /// implementation, then wraps it with framing metadata:
    ///
    /// - Start-of-frame marker
    /// - Payload length
    /// - Sequence number
    /// - Command ID
    /// - CRC8 (header) and CRC16 (frame)
    ///
    /// The resulting frame is written into `dst`.
    ///
    /// On success, returns the total number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The destination buffer is too small
    /// - The payload is too large
    /// - Payload marshaling fails
    ///
    pub fn pack<M: Marshaler>(&mut self, msg: &M, dst: &mut [u8]) -> Result<usize> {
        let mut cursor: usize = 0;

        // Ensure space for header and command ID.
        let payload_offset = HEAD_SIZE + CMDID_SIZE;
        if dst.len() < payload_offset {
            return Err(Error::InsufficientBuffer {
                need: payload_offset,
            });
        }

        // Serialize payload directly into the destination buffer.
        let size = msg.marshal(&mut dst[payload_offset..])?;

        // Validate payload length.
        if size > u16::MAX as usize {
            return Err(Error::InputTooLarge {
                max: u16::MAX as usize,
            });
        }

        // Ensure space for the entire frame.
        let total = HEAD_SIZE + CMDID_SIZE + size + TAIL_SIZE;
        if dst.len() < total {
            return Err(Error::InsufficientBuffer {
                need: total - dst.len(),
            });
        }

        // Prepare Header
        let cmd_id = M::CMD_ID;
        let sequence = self.sequence;

        // Build frame header.
        let header = {
            let mut temp = [0; 5];
            let size_bytes = (size as u16).to_le_bytes();
            temp[0] = SOF;
            temp[1] = size_bytes[0];
            temp[2] = size_bytes[1];
            temp[3] = sequence;
            temp[4] = V::calculate_crc8(&temp[..4]);
            temp
        };

        // Write header.
        dst[cursor..cursor + HEAD_SIZE].copy_from_slice(&header);
        cursor += HEAD_SIZE;

        // Write command ID.
        dst[cursor..cursor + CMDID_SIZE].copy_from_slice(&cmd_id.to_le_bytes());
        cursor += CMDID_SIZE;

        // Skip over payload (already written).
        cursor += size;

        // Write frame CRC.
        let crc = V::calculate_crc16(&dst[..cursor]);
        dst[cursor..cursor + TAIL_SIZE].copy_from_slice(&crc.to_le_bytes());
        cursor += TAIL_SIZE;

        // Advance sequence number.
        self.sequence = self.sequence.wrapping_add(1);

        #[cfg(any(feature = "log", feature = "defmt"))]
        fmt::trace!(
            "Packed Frame: {{ CMD: {}, SEQ: {}, LEN: {} }}",
            cmd_id,
            sequence,
            cursor
        );

        Ok(cursor)
    }

    ///
    /// Unpack a binary frame from raw bytes.
    ///
    /// This method validates and parses a single frame from `src`,
    /// performing the following checks:
    ///
    /// - Start-of-frame marker
    /// - Header CRC8
    /// - Frame CRC16
    /// - Payload length consistency
    ///
    /// On success, returns:
    /// - A `RawFrame` containing command ID, sequence number,
    ///   and a borrowed payload slice
    /// - The number of bytes consumed from the input buffer
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No valid start-of-frame is found
    /// - The frame is incomplete
    /// - CRC validation fails
    ///
    pub fn unpack<'t>(&self, src: &'t [u8]) -> Result<(RawFrame<'t>, usize)> {
        let mut cursor = 0;

        // Locate start-of-frame.
        if !src.starts_with(&[SOF]) {
            if let Some(start) = src.iter().position(|&x| SOF == x) {
                return Err(Error::ReSync { skip: start });
            } else {
                return Err(Error::MissingHeader { skip: src.len() });
            }
        }

        // Read header.
        let Some(header) = src.get(cursor..cursor + HEAD_SIZE) else {
            return Err(Error::UnexpectedEnd { read: src.len() });
        };
        cursor += HEAD_SIZE;

        // Validate header and extract metadata.
        let (length, sequence) = {
            let (raw, crc) = (&header[..4], header[4]);
            if V::calculate_crc8(raw) != crc {
                return Err(Error::InvalidChecksum { at: cursor });
            }

            let length = u16::from_le_bytes([raw[1], raw[2]]);
            let sequence = raw[3];
            (length as usize, sequence)
        };

        // Read command ID.
        let Some(cmd) = src.get(cursor..cursor + CMDID_SIZE) else {
            return Err(Error::UnexpectedEnd { read: src.len() });
        };
        cursor += CMDID_SIZE;

        // Read payload.
        let Some(payload) = src.get(cursor..cursor + length) else {
            return Err(Error::UnexpectedEnd { read: src.len() });
        };
        cursor += length;

        // Get the raw data for CRC calculation
        // Safety: `cursor` is within bounds due to previous checks
        let raw = src.get(..cursor).unwrap();

        // Read and validate tail CRC.
        let Some(tail) = src.get(cursor..cursor + TAIL_SIZE) else {
            return Err(Error::UnexpectedEnd { read: src.len() });
        };
        cursor += TAIL_SIZE;

        {
            // Safety: `tail` has a Fixed Length of 2
            let crc = u16::from_le_bytes([tail[0], tail[1]]);

            // Validate CRC
            if V::calculate_crc16(raw) != crc {
                return Err(Error::InvalidChecksum { at: cursor });
            }
        }

        // Parse Cmd ID
        let cmd_id = u16::from_le_bytes([cmd[0], cmd[1]]);

        #[cfg(any(feature = "log", feature = "defmt"))]
        fmt::trace!(
            "Unpacked Frame: {{ CMD: {}, SEQ: {}, LEN: {} }}",
            cmd_id,
            sequence,
            cursor
        );

        // Construct Payload
        Ok((
            RawFrame {
                cmd_id,
                sequence,
                payload,
            },
            cursor,
        ))
    }
}
