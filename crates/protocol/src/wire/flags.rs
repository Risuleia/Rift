use crate::ProtocolError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameFlags(u8);

impl FrameFlags {
    pub const NONE: Self = Self(0);

    pub const END_OF_STREAM: Self = Self(1 << 0);

    const KNOWN_MASK: u8 = Self::END_OF_STREAM.0;

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub fn from_bits(bits: u8) -> Result<Self, ProtocolError> {
        if bits & !Self::KNOWN_MASK != 0 {
            return Err(ProtocolError::UnknownFrameFlags(bits));
        }

        Ok(Self(bits))
    }

    pub const fn contains(self, flag: Self) -> bool {
        self.0 & flag.0 == flag.0
    }
}
