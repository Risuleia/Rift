use crate::ProtocolError;

use super::{FrameFlags, FrameType};

pub const FRAME_MAGIC: [u8; 4] = *b"RIFT";
pub const FRAME_HEADER_SIZE: usize = 16;
pub const FRAME_HEADER_VERSION: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameHeader {
    frame_type: FrameType,
    flags: FrameFlags,
    payload_len: u32,
}

impl FrameHeader {
    pub fn new(
        frame_type: FrameType,
        flags: FrameFlags,
        payload_len: u32,
    ) -> Result<Self, ProtocolError> {
        validate_payload_len(frame_type, payload_len)?;

        Ok(Self { frame_type, flags, payload_len })
    }

    pub const fn frame_type(&self) -> FrameType {
        self.frame_type
    }

    pub const fn flags(&self) -> FrameFlags {
        self.flags
    }

    pub const fn payload_len(&self) -> u32 {
        self.payload_len
    }

    pub fn encode(self) -> [u8; FRAME_HEADER_SIZE] {
        let mut bytes = [0u8; FRAME_HEADER_SIZE];

        bytes[0..4].copy_from_slice(&FRAME_MAGIC);

        bytes[4] = FRAME_HEADER_VERSION;
        bytes[5] = self.flags.bits();

        bytes[6..8].copy_from_slice(&self.frame_type.as_u16().to_be_bytes());

        bytes[8..12].copy_from_slice(&self.payload_len.to_be_bytes());

        // bytes 12..16 are reserved and must currently be zero.

        bytes
    }

    pub fn decode(bytes: &[u8; FRAME_HEADER_SIZE]) -> Result<Self, ProtocolError> {
        if bytes[0..4] != FRAME_MAGIC {
            return Err(ProtocolError::InvalidMagic);
        }

        let header_version = bytes[4];

        if header_version != FRAME_HEADER_VERSION {
            return Err(ProtocolError::UnsupportedHeaderVersion { version: header_version });
        }

        let flags = FrameFlags::from_bits(bytes[5])?;

        let frame_type = FrameType::try_from(u16::from_be_bytes([bytes[6], bytes[7]]))?;

        let payload_len = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

        let reserved = u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);

        if reserved != 0 {
            return Err(ProtocolError::NonZeroReservedField { value: reserved });
        }

        Self::new(frame_type, flags, payload_len)
    }
}

fn validate_payload_len(frame_type: FrameType, payload_len: u32) -> Result<(), ProtocolError> {
    let maximum = match frame_type {
        FrameType::Control => crate::limits::MAX_CONTROL_FRAME_SIZE,

        FrameType::ChunkHeader => crate::limits::MAX_CHUNK_HEADER_SIZE,
    };

    if payload_len > maximum {
        return Err(ProtocolError::FrameTooLarge {
            actual: payload_len as usize,
            maximum: maximum as usize,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_header_has_stable_size() {
        assert_eq!(FRAME_HEADER_SIZE, 16);
    }

    #[test]
    fn encodes_control_header_exactly() {
        let header = FrameHeader::new(FrameType::Control, FrameFlags::NONE, 1024).unwrap();

        assert_eq!(
            header.encode(),
            [
                0x52, 0x49, 0x46, 0x54, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ]
        );
    }

    #[test]
    fn header_round_trip() {
        let original = FrameHeader::new(FrameType::Control, FrameFlags::NONE, 4096).unwrap();

        let encoded = original.encode();
        let decoded = FrameHeader::decode(&encoded).unwrap();

        assert_eq!(decoded, original);
    }

    #[test]
    fn rejects_invalid_magic() {
        let mut bytes = [0u8; FRAME_HEADER_SIZE];

        bytes[0..4].copy_from_slice(b"NOPE");
        bytes[4] = FRAME_HEADER_VERSION;

        assert_eq!(FrameHeader::decode(&bytes), Err(ProtocolError::InvalidMagic));
    }

    #[test]
    fn rejects_unknown_header_version() {
        let bytes = [
            0x52, 0x49, 0x46, 0x54, 0xff, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];

        assert!(matches!(
            FrameHeader::decode(&bytes),
            Err(ProtocolError::UnsupportedHeaderVersion { .. })
        ));
    }

    #[test]
    fn rejects_oversized_control_frame() {
        let result = FrameHeader::new(
            FrameType::Control,
            FrameFlags::NONE,
            crate::limits::MAX_CONTROL_FRAME_SIZE + 1,
        );

        assert!(matches!(result, Err(ProtocolError::FrameTooLarge { .. })));
    }
}
