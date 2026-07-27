use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! define_uuid_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[repr(transparent)]
        pub struct $name(Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub const fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }

            pub const fn into_uuid(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<Uuid> for $name {
            fn from(value: Uuid) -> Self {
                Self::from_uuid(value)
            }
        }

        impl From<$name> for Uuid {
            fn from(value: $name) -> Self {
                value.into_uuid()
            }
        }
    };
}

define_uuid_id!(PeerId);
define_uuid_id!(SessionId);
define_uuid_id!(TransferId);

pub const CHUNK_ID_SIZE: usize = 32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkId([u8; CHUNK_ID_SIZE]);

impl ChunkId {
    pub const fn new(bytes: [u8; CHUNK_ID_SIZE]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; CHUNK_ID_SIZE] {
        &self.0
    }

    pub const fn into_bytes(self) -> [u8; CHUNK_ID_SIZE] {
        self.0
    }
}

impl std::fmt::Debug for ChunkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChunkId(")?;

        for byte in &self.0[..4] {
            write!(f, "{byte:02x}")?;
        }

        write!(f, "…)")
    }
}

impl AsRef<[u8]> for ChunkId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; CHUNK_ID_SIZE]> for ChunkId {
    fn from(value: [u8; CHUNK_ID_SIZE]) -> Self {
        Self(value)
    }
}

impl From<ChunkId> for [u8; CHUNK_ID_SIZE] {
    fn from(value: ChunkId) -> Self {
        value.0
    }
}

impl From<&ChunkId> for [u8; CHUNK_ID_SIZE] {
    fn from(value: &ChunkId) -> Self {
        value.0
    }
}

impl std::borrow::Borrow<[u8]> for ChunkId {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}
