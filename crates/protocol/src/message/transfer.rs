use crate::{ChunkId, ManifestEntry, MessageError, TransferId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferMessage {
    Offer(TransferOffer),
    Accept(TransferAccept),
    Reject(TransferReject),
    ManifestStart(ManifestStart),
    ManifestBatch(ManifestBatch),
    ManifestEnd(ManifestEnd),
    NeedChunks(NeedChunks),
    Cancel(TransferCancel),
    Complete(TransferComplete),
    Verified(TransferVerified),
    Failed(TransferFailed),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferOffer {
    transfer_id: TransferId,
    display_name: String,
    total_size: u64,
    entry_count: u64,
}

impl TransferOffer {
    pub fn new(
        transfer_id: TransferId,
        display_name: impl Into<String>,
        total_size: u64,
        entry_count: u64,
    ) -> Result<Self, MessageError> {
        let display_name = display_name.into();

        super::validation::validate_text_field(
            &display_name,
            crate::limits::MAX_TRANSFER_DISPLAY_NAME_BYTES,
            MessageError::EmptyTransferDisplayName,
            |actual, maximum| MessageError::TransferDisplayNameTooLong { actual, maximum },
            MessageError::InvalidTransferDisplayName,
        )?;

        Ok(Self { transfer_id, display_name, total_size, entry_count })
    }

    pub const fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub const fn total_size(&self) -> u64 {
        self.total_size
    }

    pub const fn entry_count(&self) -> u64 {
        self.entry_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferAccept {
    transfer_id: TransferId,
}

impl TransferAccept {
    pub const fn new(transfer_id: TransferId) -> Self {
        Self { transfer_id }
    }

    pub const fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferReject {
    transfer_id: TransferId,
    reason: TransferRejectReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferRejectReason {
    Declined,
    InsufficientSpace,
    UnsupportedContent,
    PolicyDenied,
    Busy,
}

impl TransferReject {
    pub const fn new(transfer_id: TransferId, reason: TransferRejectReason) -> Self {
        Self { transfer_id, reason }
    }

    pub const fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }

    pub const fn reason(&self) -> TransferRejectReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestStart {
    transfer_id: TransferId,
    entry_count: u64,
    total_size: u64,
}

impl ManifestStart {
    pub const fn new(transfer_id: TransferId, entry_count: u64, total_size: u64) -> Self {
        Self { transfer_id, entry_count, total_size }
    }

    pub const fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }

    pub const fn entry_count(&self) -> u64 {
        self.entry_count
    }

    pub const fn total_size(&self) -> u64 {
        self.total_size
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestBatch {
    transfer_id: TransferId,
    sequence: u32,
    entries: Vec<ManifestEntry>,
}

impl ManifestBatch {
    pub fn new(
        transfer_id: TransferId,
        sequence: u32,
        entries: Vec<ManifestEntry>,
    ) -> Result<Self, MessageError> {
        if entries.is_empty() {
            return Err(MessageError::EmptyManifestBatch);
        }

        if entries.len() > crate::limits::MAX_MANIFEST_ENTRIES_PER_BATCH {
            return Err(MessageError::ManifestBatchTooLarge {
                actual: entries.len(),
                maximum: crate::limits::MAX_MANIFEST_ENTRIES_PER_BATCH,
            });
        }

        Ok(Self { transfer_id, sequence, entries })
    }

    pub const fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }

    pub const fn sequence(&self) -> u32 {
        self.sequence
    }

    pub fn entries(&self) -> &[ManifestEntry] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestEnd {
    transfer_id: TransferId,
    batch_count: u32,
}

impl ManifestEnd {
    pub const fn new(transfer_id: TransferId, batch_count: u32) -> Self {
        Self { transfer_id, batch_count }
    }

    pub const fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }

    pub const fn batch_count(&self) -> u32 {
        self.batch_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeedChunks {
    transfer_id: TransferId,
    request_id: u64,
    chunks: Vec<ChunkId>,
}

impl NeedChunks {
    pub fn new(
        transfer_id: TransferId,
        request_id: u64,
        chunks: Vec<ChunkId>,
    ) -> Result<Self, MessageError> {
        if chunks.is_empty() {
            return Err(MessageError::EmptyChunkRequest);
        }

        if chunks.len() > crate::limits::MAX_CHUNK_IDS_PER_REQUEST {
            return Err(MessageError::ChunkRequestTooLarge {
                actual: chunks.len(),
                maximum: crate::limits::MAX_CHUNK_IDS_PER_REQUEST,
            });
        }

        Ok(Self { transfer_id, request_id, chunks })
    }

    pub const fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }

    pub const fn request_id(&self) -> u64 {
        self.request_id
    }

    pub fn chunks(&self) -> &[ChunkId] {
        &self.chunks
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferCancel {
    transfer_id: TransferId,
    reason: CancelReason,
}

impl TransferCancel {
    pub const fn new(transfer_id: TransferId, reason: CancelReason) -> Self {
        Self { transfer_id, reason }
    }

    pub const fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }

    pub const fn reason(&self) -> CancelReason {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelReason {
    UserRequested,
    InsufficientSpace,
    PolicyDenied,
    ShuttingDown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferComplete {
    transfer_id: TransferId,
}

impl TransferComplete {
    pub const fn new(transfer_id: TransferId) -> Self {
        Self { transfer_id }
    }

    pub const fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferVerified {
    transfer_id: TransferId,
}

impl TransferVerified {
    pub const fn new(transfer_id: TransferId) -> Self {
        Self { transfer_id }
    }

    pub const fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferFailed {
    transfer_id: TransferId,
    reason: TransferFailureReason,
}

impl TransferFailed {
    pub const fn new(transfer_id: TransferId, reason: TransferFailureReason) -> Self {
        Self { transfer_id, reason }
    }

    pub const fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }

    pub const fn reason(&self) -> TransferFailureReason {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferFailureReason {
    ProtocolViolation,
    IntegrityFailure,
    StorageFailure,
    UnsupportedCapability,
    InternalError,
}
