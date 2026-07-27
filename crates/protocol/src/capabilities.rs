use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{CapabilityError, limits::MAX_CAPABILITIES_PER_CATEGORY};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportCapability {
    Tcp,
    Quic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompressionCapability {
    None,
    Zstd,
    Lz4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChunkingCapability {
    Fixed,
    FastCdc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeatureCapability {
    Resume,
    Deduplication,
    DeltaTransfer,
    Multipath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capabilities {
    transports: Vec<TransportCapability>,
    compression: Vec<CompressionCapability>,
    chunking: Vec<ChunkingCapability>,
    features: Vec<FeatureCapability>,
}

impl Capabilities {
    pub fn new(
        transports: Vec<TransportCapability>,
        compression: Vec<CompressionCapability>,
        chunking: Vec<ChunkingCapability>,
        features: Vec<FeatureCapability>,
    ) -> Result<Self, CapabilityError> {
        validate_count(transports.len(), CapabilityCategory::Transport)?;

        validate_count(compression.len(), CapabilityCategory::Compression)?;

        validate_count(chunking.len(), CapabilityCategory::Chunking)?;

        validate_count(features.len(), CapabilityCategory::Feature)?;

        if has_duplicates(&transports)
            || has_duplicates(&compression)
            || has_duplicates(&chunking)
            || has_duplicates(&features)
        {
            return Err(CapabilityError::DuplicateCapability);
        }

        Ok(Self { transports, compression, chunking, features })
    }

    pub fn transports(&self) -> &[TransportCapability] {
        &self.transports
    }

    pub fn compression(&self) -> &[CompressionCapability] {
        &self.compression
    }

    pub fn chunking(&self) -> &[ChunkingCapability] {
        &self.chunking
    }

    pub fn features(&self) -> &[FeatureCapability] {
        &self.features
    }

    pub fn common_transports(&self, other: &Self) -> Vec<TransportCapability> {
        intersection(&self.transports, &other.transports)
    }

    pub fn common_compression(&self, other: &Self) -> Vec<CompressionCapability> {
        intersection(&self.compression, &other.compression)
    }

    pub fn common_chunking(&self, other: &Self) -> Vec<ChunkingCapability> {
        intersection(&self.chunking, &other.chunking)
    }

    pub fn common_features(&self, other: &Self) -> Vec<FeatureCapability> {
        intersection(&self.features, &other.features)
    }
}

fn has_duplicates<T>(values: &[T]) -> bool
where
    T: Eq + std::hash::Hash,
{
    let mut seen = HashSet::with_capacity(values.len());

    values.iter().any(|value| !seen.insert(value))
}

fn intersection<T>(left: &[T], right: &[T]) -> Vec<T>
where
    T: Copy + Eq + std::hash::Hash,
{
    let right: HashSet<_> = right.iter().copied().collect();

    left.iter().copied().filter(|item| right.contains(item)).collect()
}

#[derive(Clone, Copy)]
enum CapabilityCategory {
    Transport,
    Compression,
    Chunking,
    Feature,
}

fn validate_count(actual: usize, category: CapabilityCategory) -> Result<(), CapabilityError> {
    if actual <= MAX_CAPABILITIES_PER_CATEGORY {
        return Ok(());
    }

    let maximum = MAX_CAPABILITIES_PER_CATEGORY;

    Err(match category {
        CapabilityCategory::Transport => CapabilityError::TooManyTransports { actual, maximum },

        CapabilityCategory::Compression => {
            CapabilityError::TooManyCompressionAlgorithms { actual, maximum }
        }

        CapabilityCategory::Chunking => {
            CapabilityError::TooManyChunkingAlgorithms { actual, maximum }
        }

        CapabilityCategory::Feature => CapabilityError::TooManyFeatures { actual, maximum },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capabilities() -> Capabilities {
        Capabilities::new(
            vec![TransportCapability::Quic, TransportCapability::Tcp],
            vec![CompressionCapability::Zstd, CompressionCapability::None],
            vec![ChunkingCapability::FastCdc, ChunkingCapability::Fixed],
            vec![FeatureCapability::Resume, FeatureCapability::Multipath],
        )
        .unwrap()
    }

    #[test]
    fn constructs_valid_capabilities() {
        let capabilities = capabilities();

        assert_eq!(capabilities.transports().len(), 2);
        assert_eq!(capabilities.features().len(), 2);
    }

    #[test]
    fn rejects_duplicate_capabilities() {
        let result = Capabilities::new(
            vec![TransportCapability::Tcp, TransportCapability::Tcp],
            vec![],
            vec![],
            vec![],
        );

        assert_eq!(result, Err(CapabilityError::DuplicateCapability));
    }

    #[test]
    fn finds_common_transports_in_local_preference_order() {
        let local = capabilities();

        let remote = Capabilities::new(
            vec![TransportCapability::Tcp, TransportCapability::Quic],
            vec![CompressionCapability::None],
            vec![ChunkingCapability::Fixed],
            vec![FeatureCapability::Resume],
        )
        .unwrap();

        assert_eq!(
            local.common_transports(&remote),
            vec![TransportCapability::Quic, TransportCapability::Tcp,]
        );
    }

    #[test]
    fn finds_common_features() {
        let local = capabilities();

        let remote = Capabilities::new(
            vec![TransportCapability::Tcp],
            vec![],
            vec![],
            vec![FeatureCapability::Resume, FeatureCapability::Deduplication],
        )
        .unwrap();

        assert_eq!(local.common_features(&remote), vec![FeatureCapability::Resume]);
    }
}
