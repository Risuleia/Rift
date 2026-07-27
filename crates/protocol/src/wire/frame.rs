use crate::{
    ProtocolError,
    wire::{FrameFlags, FrameHeader, FrameType},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    header: FrameHeader,
    payload: Vec<u8>,
}

impl Frame {
    pub fn new(
        r#type: FrameType,
        flags: FrameFlags,
        payload: Vec<u8>,
    ) -> Result<Self, ProtocolError> {
        let header = FrameHeader::new(r#type, flags, payload.len() as u32)?;

        Ok(Self { header, payload })
    }

    pub const fn header(&self) -> &FrameHeader {
        &self.header
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn into_payload(self) -> Vec<u8> {
        self.payload
    }

    pub fn into_parts(self) -> (FrameHeader, Vec<u8>) {
        (self.header, self.payload)
    }

    pub fn from_parts(header: FrameHeader, payload: Vec<u8>) -> Result<Self, ProtocolError> {
        if payload.len() != header.payload_len() as usize {
            return Err(ProtocolError::PayloadLengthMismatch {
                expected: header.payload_len() as usize,
                actual: payload.len(),
            });
        }

        Ok(Self { header, payload })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_round_trip() {
        let payload = vec![1, 2, 3, 4];

        let frame = Frame::new(FrameType::Control, FrameFlags::NONE, payload.clone()).unwrap();

        let (header, payload) = frame.into_parts();

        let rebuilt = Frame::from_parts(header, payload).unwrap();

        assert_eq!(rebuilt.header().payload_len(), 4);
        assert_eq!(rebuilt.payload(), &[1, 2, 3, 4]);
    }

    #[test]
    fn rejects_payload_length_mismatch() {
        let header = FrameHeader::new(FrameType::Control, FrameFlags::NONE, 10).unwrap();

        let payload = vec![0u8; 5];

        assert!(matches!(
            Frame::from_parts(header, payload),
            Err(ProtocolError::PayloadLengthMismatch { .. })
        ));
    }
}
