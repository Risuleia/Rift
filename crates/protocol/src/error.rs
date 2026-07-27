use thiserror::Error;

use crate::wire::FrameType;

#[derive(Debug, Error, PartialEq)]
pub enum ProtocolError {
    #[error("invalid frame magic")]
    InvalidMagic,

    #[error("unsupported protocol version {major}")]
    UnsupportedVersion { major: u16 },

    #[error("unknown frame type {0}")]
    UnknownFrameType(u16),

    #[error("frame payload exceeds limit: {actual} > {maximum}")]
    FrameTooLarge { actual: usize, maximum: usize },

    #[error("malformed payload")]
    MalformedPayload,

    #[error("invalid path: {0}")]
    InvalidPath(#[from] PathError),

    #[error("invalid manifest: {0}")]
    InvalidManifest(#[from] ManifestError),

    #[error(transparent)]
    InvalidMessage(#[from] MessageError),

    #[error(transparent)]
    InvalidCapability(#[from] CapabilityError),

    #[error(transparent)]
    ProtobufDecode(#[from] prost::DecodeError),

    #[error("protocol limit exceeded: {0}")]
    LimitExceeded(&'static str),

    #[error("unknown frame flags: {0:#010b}")]
    UnknownFrameFlags(u8),

    #[error("unsupported frame header version {version}")]
    UnsupportedHeaderVersion { version: u8 },

    #[error("reserved frame header field must be zero, got {value}")]
    NonZeroReservedField { value: u32 },

    #[error("invalid identifier length: expected {expected}, got {actual}")]
    InvalidIdentifierLength { expected: usize, actual: usize },

    #[error("malformed identifier")]
    MalformedIdentifier,

    #[error("invalid chunk identifier length: expected {expected}, got {actual}")]
    InvalidChunkIdLength { expected: usize, actual: usize },

    #[error("payload length mismatch: expected {expected} bytes, got {actual} bytes")]
    PayloadLengthMismatch { expected: usize, actual: usize },

    #[error("unexpected frame type: expected {expected:?}, got {actual:?}")]
    UnexpectedFrameType { expected: FrameType, actual: FrameType },

    #[error("missing required protobuf field `{0}`")]
    MissingField(&'static str),

    #[error("invalid protocol version")]
    InvalidProtocolVersion,

    #[error("invalid session close reason: {0}")]
    InvalidSessionCloseReason(i32),

    #[error("invalid transport capability: {0}")]
    InvalidTransportCapability(i32),

    #[error("invalid compression capability: {0}")]
    InvalidCompressionCapability(i32),

    #[error("invalid chunking capability: {0}")]
    InvalidChunkingCapability(i32),

    #[error("invalid feature capability: {0}")]
    InvalidFeatureCapability(i32),

    #[error("invalid transfer reject reason: {0}")]
    InvalidTransferRejectReason(i32),

    #[error("invalid transfer cancel reason: {0}")]
    InvalidCancelReason(i32),

    #[error("invalid transfer failure reason: {0}")]
    InvalidTransferFailureReason(i32),

    #[error("invalid manifest entry")]
    InvalidManifestEntry,

    #[error("invalid control message")]
    InvalidControlMessage,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PathError {
    #[error("path is empty")]
    Empty,

    #[error("path exceeds maximum length: {actual} > {maximum} bytes")]
    TooLong { actual: usize, maximum: usize },

    #[error("absolute paths are not allowed")]
    Absolute,

    #[error("parent traversal is not allowed")]
    ParentTraversal,

    #[error("current-directory components are not allowed")]
    CurrentDirectory,

    #[error("empty path components are not allowed")]
    EmptyComponent,

    #[error("backslashes are not allowed in wire paths")]
    InvalidSeparator,

    #[error("NUL bytes are not allowed")]
    NullByte,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ManifestError {
    #[error("manifest contains too many entries: {actual} > {maximum}")]
    TooManyEntries { actual: usize, maximum: usize },

    #[error("duplicate manifest path: {0}")]
    DuplicatePath(String),

    #[error("file contains too many chunks: {actual} > {maximum}")]
    TooManyChunks { actual: usize, maximum: usize },

    #[error("non-empty file contains no chunks: {0}")]
    MissingChunks(String),

    #[error("empty file contains chunks: {0}")]
    UnexpectedChunks(String),

    #[error("chunk has zero size in file: {0}")]
    ZeroSizedChunk(String),

    #[error("chunk range overflows in file: {0}")]
    ChunkRangeOverflow(String),

    #[error("chunk exceeds file bounds: {0}")]
    ChunkOutOfBounds(String),

    #[error("chunks overlap in file: {0}")]
    OverlappingChunks(String),

    #[error("chunks do not completely cover file: {0}")]
    IncompleteChunkCoverage(String),

    #[error("manifest total size overflow")]
    TotalSizeOverflow,

    #[error("manifest total size mismatch: declared {declared}, calculated {calculated}")]
    TotalSizeMismatch { declared: u64, calculated: u64 },
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CapabilityError {
    #[error("too many transport capabilities: {actual} > {maximum}")]
    TooManyTransports { actual: usize, maximum: usize },

    #[error("too many compression capabilities: {actual} > {maximum}")]
    TooManyCompressionAlgorithms { actual: usize, maximum: usize },

    #[error("too many chunking capabilities: {actual} > {maximum}")]
    TooManyChunkingAlgorithms { actual: usize, maximum: usize },

    #[error("too many feature capabilities: {actual} > {maximum}")]
    TooManyFeatures { actual: usize, maximum: usize },

    #[error("duplicate capability")]
    DuplicateCapability,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum MessageError {
    #[error("device name is empty")]
    EmptyDeviceName,

    #[error("device name exceeds maximum length: {actual} > {maximum} bytes")]
    DeviceNameTooLong { actual: usize, maximum: usize },

    #[error("device name contains control characters")]
    InvalidDeviceName,

    #[error("transfer display name is empty")]
    EmptyTransferDisplayName,

    #[error("transfer display name exceeds maximum length: {actual} > {maximum} bytes")]
    TransferDisplayNameTooLong { actual: usize, maximum: usize },

    #[error("transfer display name contains control characters")]
    InvalidTransferDisplayName,

    #[error("empty manifest batch")]
    EmptyManifestBatch,

    #[error("manifest batch size exceeds maximum length: {actual} > {maximum} bytes")]
    ManifestBatchTooLarge { actual: usize, maximum: usize },

    #[error("chunk request must contain at least one chunk")]
    EmptyChunkRequest,

    #[error("chunk request contains {actual} chunk IDs, maximum allowed is {maximum}")]
    ChunkRequestTooLarge { actual: usize, maximum: usize },
}
