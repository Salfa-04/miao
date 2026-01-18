//! Error types for the binframe crate.

use crate::private::*;

/// A specialized `Result` type with `Error``.
pub type Result<T> = StdResult<T, Error>;

///
/// # Error types for Packet Processing.
///
/// ## Position Indicators
///
/// Many `Error` variants that indicate a
/// position in the input buffer carry a `skip: usize` field. In this crate
/// `skip` is used consistently to indicate either the number of bytes skipped
/// while scanning or the byte offset where an error was detected.
///
/// Typical Meanings:
/// - `Error::ReSync` — `skip` is the index where a valid header was found (bytes skipped).
/// - `Error::MissingHeader` — `skip` is the number of bytes scanned (often `src.len()` when none found).
/// - `Error::UnexpectedEnd` — `read` is the buffer length at the point the data was incomplete.
/// - `Error::InvalidChecksum` — `at` is the offset immediately after the payload where CRC failed.
/// - `Error::ParseError` — `at` is the offset where payload parsing failed.
///
///
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Provided buffer is too small to complete the operation.
    InsufficientBuffer { need: usize },
    /// The Payload size exceeds the maximum allowed limit.
    InputTooLarge { max: usize },
    /// Encountered an unexpected end of input during parsing.
    UnexpectedEnd { read: usize },
    /// The input stream requires resynchronization.
    ReSync { skip: usize },
    /// Expected message header not found at the current position.
    MissingHeader { skip: usize },
    /// Checksum validation failed for the data.
    InvalidChecksum { at: usize },
    /// Failed to parse the payload or a field within the message.
    ParseError { at: usize },
    /// Failed to encode the message.
    EncodeError { inner: usize },
    /// The data length is invalid.
    InvalidDataLength { expected: usize },
}

impl Error {
    /// Get the skip position associated with the error.
    pub fn skip(&self) -> usize {
        #[cfg(any(feature = "log", feature = "defmt"))]
        fmt::trace!("Getting Skip Position for Error: {:?}", self);

        match self {
            Self::InsufficientBuffer { .. } => 0,
            Self::InputTooLarge { .. } => 0,
            Self::UnexpectedEnd { .. } => 0,
            Self::ReSync { skip } => *skip,
            Self::MissingHeader { skip } => *skip,
            Self::InvalidChecksum { at } => *at,
            Self::ParseError { at } => *at,
            Self::EncodeError { .. } => 0,
            Self::InvalidDataLength { .. } => 0,
        }
    }
}

impl StdError for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::InsufficientBuffer { need } => {
                write!(f, "Insufficient buffer, need {need} bytes at least")
            }
            Self::InputTooLarge { max } => {
                write!(f, "Input size exceeds maximum allowed size of {max} bytes")
            }
            Self::UnexpectedEnd { read } => {
                write!(f, "Unexpected end of data at offset {read}")
            }
            Self::ReSync { skip } => {
                write!(f, "Stream requires resynchronization, skipped {skip} bytes")
            }
            Self::MissingHeader { skip } => write!(f, "Missing header at offset {skip}"),
            Self::InvalidChecksum { at } => {
                write!(f, "Invalid checksum at offset {at}")
            }
            Self::ParseError { at } => {
                write!(f, "Failed to parse payload at offset {at}")
            }
            Self::EncodeError { inner } => {
                write!(f, "Failed to encode message: {inner:?}")
            }
            Self::InvalidDataLength { expected } => {
                write!(f, "Invalid data length, expected {expected} bytes")
            }
        }
    }
}
