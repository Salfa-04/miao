use crate::private::*;

#[test]
fn test_dji_crc8() {
    let data = b"123456789";
    assert_eq!(calc_dji8(data), 0x0B);
}

#[test]
fn test_dji_crc16() {
    let data = b"123456789";
    assert_eq!(calc_dji16(data), 0x6F91);
}

struct TestCase<const N: usize> {
    payload: [u8; N],
}

impl<const N: usize> Marshaler for TestCase<N> {
    const CMD_ID: u16 = 0x1234;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < N {
            return Err(Error::InsufficientBuffer { need: N });
        }
        dst[..N].copy_from_slice(&self.payload);
        Ok(N)
    }

    fn unmarshal(src: &[u8]) -> Result<Self> {
        if src.len() < N || src.len() != N {
            return Err(Error::InvalidDataLength { expected: N });
        }
        let mut payload = [0u8; N];
        payload.copy_from_slice(&src[..N]);
        Ok(Self { payload })
    }
}

impl<const N: usize> TestCase<N> {
    fn new(payload: [u8; N]) -> Self {
        Self { payload }
    }
}

#[test]
fn test_encode_decode() {
    let mut msger: Messager<DjiValidator> = Messager::new(0);

    let test = TestCase::new([1, 2, 3, 4, 5]);
    let mut buffer = [0u8; 64];

    let size_a = msger.pack(&test, &mut buffer).unwrap();

    let (raw, size_b) = msger.unpack(&buffer[..size_a]).unwrap();

    let this = TestCase::unmarshal(raw.payload()).unwrap();

    println!("Encoded size: {}", size_a);
    println!("Encoded data: {:X?}", &buffer[..size_a]);
    println!("Decoded size: {}", size_b);
    println!("Decoded payload: {:X?}", this.payload);

    assert_eq!(size_a, size_b);
    assert_eq!(raw.cmd_id(), TestCase::<5>::CMD_ID);
    assert_eq!(test.payload, this.payload);
}

#[test]
fn test_invalid_decode() {
    let invalid_data = [0u8; 10];
    let msger: Messager<DjiValidator> = Messager::new(0);

    assert!(matches!(
        msger.unpack(&invalid_data),
        Err(Error::MissingHeader { skip: 10 })
    ));
}

#[test]
fn test_validate_decode() {
    let valid_data = [
        0xA5, 0x5, 0x0, 0x56, 0xF0, // Header
        0x34, 0x12, // CMD ID
        0x1, 0x2, 0x3, 0x4, 0x5, // Data
        0x84, 0x71, // Tail CRC
    ];
    let msger: Messager<DjiValidator> = Messager::new(0);

    assert!(msger.unpack(&valid_data).is_ok());
}

#[test]
fn test_encode() {
    let test = TestCase::new([1, 2, 3, 4, 5]);
    let mut buffer = [0u8; 64];

    let mut msger: Messager<DjiValidator> = Messager::new(0x56);
    let size = msger.pack(&test, &mut buffer).unwrap();

    let expected: [u8; 14] = [
        0xA5, 0x5, 0x0, 0x56, 0xF0, // Header
        0x34, 0x12, // CMD ID
        0x1, 0x2, 0x3, 0x4, 0x5, // Data
        0x84, 0x71, // Tail CRC
    ];

    assert_eq!(&buffer[..size], &expected);
}

#[test]
fn test_insufficient_buffer() {
    let test = TestCase::new([1, 2, 3, 4, 5]);
    let mut buffer = [0u8; 10]; // Intentionally small buffer
    let mut msger: Messager<DjiValidator> = Messager::new(0x56);
    let result = msger.pack(&test, &mut buffer);
    assert!(matches!(result, Err(Error::InsufficientBuffer { need: 5 })));
}

#[test]
fn test_sof_not_found() {
    let invalid_data = [
        0x5, 0x0, 0x56, 0xF0, // Header
        0x34, 0x12, // CMD ID
        0x1, 0x2, 0x3, 0x4, 0x5, // Data
        0x84, 0x71, // Tail CRC
    ];
    let msger: Messager<DjiValidator> = Messager::new(0x56);
    let result = msger.unpack(&invalid_data);
    assert!(matches!(result, Err(Error::MissingHeader { skip: 13 })));
}

#[test]
fn test_invalid_header_checksum() {
    let invalid_data = [
        0xA5, 0x5, 0xFF, 0x56, 0xF0, // Invalid Header
        0x34, 0x12, // CMD ID
        0x1, 0x2, 0x3, 0x4, 0x5, // Data
        0x84, 0x71, // Tail CRC
    ];
    let msger: Messager<DjiValidator> = Messager::new(0x56);
    let result = msger.unpack(&invalid_data);
    assert!(matches!(result, Err(Error::InvalidChecksum { at: 5 })));
}

#[test]
fn test_invalid_tail_checksum() {
    let invalid_data = [
        0xA5, 0x5, 0x0, 0x56, 0xF0, // Header
        0x34, 0x12, // CMD ID
        0x1, 0x2, 0x3, 0x4, 0x5, // Data
        0x00, 0x00, // Invalid Tail CRC
    ];
    let msger: Messager<DjiValidator> = Messager::new(0x56);
    let result = msger.unpack(&invalid_data);
    assert!(matches!(result, Err(Error::InvalidChecksum { at: 14 })));
}
