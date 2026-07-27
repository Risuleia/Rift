use uuid::Uuid;

use crate::{CHUNK_ID_SIZE, ChunkId, PeerId, ProtocolError, SessionId, TransferId, wire::proto};

const UUID_SIZE: usize = std::mem::size_of::<Uuid>();

pub(super) trait UuidId: Sized + Copy + From<Uuid> + Into<Uuid> {}

impl UuidId for PeerId {}
impl UuidId for SessionId {}
impl UuidId for TransferId {}

fn encode_uuid<T>(id: T) -> Vec<u8>
where
    T: UuidId,
{
    let uuid: Uuid = id.into();
    uuid.as_bytes().to_vec()
}

fn decode_uuid<T>(bytes: &[u8]) -> Result<T, ProtocolError>
where
    T: UuidId,
{
    if bytes.len() != UUID_SIZE {
        return Err(ProtocolError::InvalidIdentifierLength {
            expected: UUID_SIZE,
            actual: bytes.len(),
        });
    }

    let uuid = Uuid::from_slice(bytes).map_err(|_| ProtocolError::MalformedIdentifier)?;

    Ok(uuid.into())
}

pub(super) fn encode_peer_id(id: PeerId) -> proto::PeerId {
    proto::PeerId { value: encode_uuid(id) }
}

pub(super) fn decode_peer_id(id: proto::PeerId) -> Result<PeerId, ProtocolError> {
    decode_uuid(&id.value)
}

pub(super) fn encode_session_id(id: SessionId) -> proto::SessionId {
    proto::SessionId { value: encode_uuid(id) }
}

pub(super) fn decode_session_id(id: proto::SessionId) -> Result<SessionId, ProtocolError> {
    decode_uuid(&id.value)
}

pub(super) fn encode_transfer_id(id: TransferId) -> proto::TransferId {
    proto::TransferId { value: encode_uuid(id) }
}

pub(super) fn decode_transfer_id(id: proto::TransferId) -> Result<TransferId, ProtocolError> {
    decode_uuid(&id.value)
}

pub(super) fn encode_chunk_id(id: ChunkId) -> proto::ChunkId {
    proto::ChunkId { value: id.as_bytes().to_vec() }
}

pub(super) fn decode_chunk_id(id: proto::ChunkId) -> Result<ChunkId, ProtocolError> {
    if id.value.len() != CHUNK_ID_SIZE {
        return Err(ProtocolError::InvalidChunkIdLength {
            expected: CHUNK_ID_SIZE,
            actual: id.value.len(),
        });
    }

    let mut bytes = [0u8; CHUNK_ID_SIZE];
    bytes.copy_from_slice(&id.value);

    Ok(ChunkId::new(bytes))
}

#[cfg(test)]
mod tests {
    use crate::{
        CHUNK_ID_SIZE, ChunkId, PeerId, ProtocolError, SessionId, TransferId, wire::{
            convert::{
                UuidId, ids::{decode_uuid, encode_uuid},
            }, proto,
        },
    };

    fn uuid_round_trip<T>()
    where
        T: UuidId + Default + Eq + std::fmt::Debug,
    {
        let original = T::default();

        let encoded = encode_uuid(original);
        let decoded = decode_uuid::<T>(&encoded).unwrap();

        assert_eq!(decoded, original);
    }

    #[test]
    fn peer_round_trip() {
        uuid_round_trip::<PeerId>();
    }

    #[test]
    fn session_round_trip() {
        uuid_round_trip::<SessionId>();
    }

    #[test]
    fn transfer_round_trip() {
        uuid_round_trip::<TransferId>();
    }

    #[test]
    fn chunk_round_trip() {
        let original = ChunkId::new([42; CHUNK_ID_SIZE]);

        let encoded = super::encode_chunk_id(original);
        let decoded = super::decode_chunk_id(encoded).unwrap();

        assert_eq!(decoded, original);
    }

    #[test]
    fn rejects_invalid_chunk_length() {
        let result = super::decode_chunk_id(proto::ChunkId { value: vec![0; 8] });

        assert!(matches!(
            result,
            Err(ProtocolError::InvalidChunkIdLength { expected: CHUNK_ID_SIZE, actual: 8 })
        ));
    }
}
