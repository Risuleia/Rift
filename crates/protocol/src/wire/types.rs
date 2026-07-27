use crate::ProtocolError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FrameType {
    Control = 0x0001,
    ChunkHeader = 0x0002,
}

impl FrameType {
    pub const fn as_u16(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for FrameType {
    type Error = ProtocolError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0001 => Ok(Self::Control),
            0x0002 => Ok(Self::ChunkHeader),

            unknown => Err(ProtocolError::UnknownFrameType(unknown)),
        }
    }
}
