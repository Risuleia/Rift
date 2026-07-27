use crate::{
    Capabilities, ChunkingCapability, CompressionCapability, FeatureCapability, ProtocolError,
    TransportCapability,
};

use super::{super::proto, proto_enum_converter};

proto_enum_converter! {
    encode = encode_transport_capability,
    decode = decode_transport_capability,
    domain = TransportCapability,
    proto = proto::TransportCapability,
    error = InvalidTransportCapability,

    {
        Tcp => TransportTcp,
        Quic => TransportQuic,
    }
}

proto_enum_converter! {
    encode = encode_compression_capability,
    decode = decode_compression_capability,
    domain = CompressionCapability,
    proto = proto::CompressionCapability,
    error = InvalidCompressionCapability,

    {
        None => CompressionNone,
        Zstd => CompressionZstd,
        Lz4 => CompressionLz4,
    }
}

proto_enum_converter! {
    encode = encode_chunking_capability,
    decode = decode_chunking_capability,
    domain = ChunkingCapability,
    proto = proto::ChunkingCapability,
    error = InvalidChunkingCapability,

    {
        Fixed => ChunkingFixed,
        FastCdc => ChunkingFastCdc,
    }
}

proto_enum_converter! {
    encode = encode_feature_capability,
    decode = decode_feature_capability,
    domain = FeatureCapability,
    proto = proto::FeatureCapability,
    error = InvalidFeatureCapability,

    {
        Resume => FeatureResume,
        Deduplication => FeatureDeduplication,
        DeltaTransfer => FeatureDeltaTransfer,
        Multipath => FeatureMultipath,
    }
}

pub(super) fn encode_capabilities(capabilities: &Capabilities) -> proto::Capabilities {
    proto::Capabilities {
        transports: capabilities
            .transports()
            .iter()
            .copied()
            .map(encode_transport_capability)
            .map(|v| v as i32)
            .collect(),

        compression: capabilities
            .compression()
            .iter()
            .copied()
            .map(encode_compression_capability)
            .map(|v| v as i32)
            .collect(),

        chunking: capabilities
            .chunking()
            .iter()
            .copied()
            .map(encode_chunking_capability)
            .map(|v| v as i32)
            .collect(),

        features: capabilities
            .features()
            .iter()
            .copied()
            .map(encode_feature_capability)
            .map(|v| v as i32)
            .collect(),
    }
}

pub(super) fn decode_capabilities(
    capabilities: proto::Capabilities,
) -> Result<Capabilities, ProtocolError> {
    let transports = capabilities
        .transports
        .into_iter()
        .map(decode_transport_capability)
        .collect::<Result<Vec<_>, _>>()?;

    let compression = capabilities
        .compression
        .into_iter()
        .map(decode_compression_capability)
        .collect::<Result<Vec<_>, _>>()?;

    let chunking = capabilities
        .chunking
        .into_iter()
        .map(decode_chunking_capability)
        .collect::<Result<Vec<_>, _>>()?;

    let features = capabilities
        .features
        .into_iter()
        .map(decode_feature_capability)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Capabilities::new(transports, compression, chunking, features)?)
}
