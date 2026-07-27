use serde::{Deserialize, Serialize};

use crate::{
    ids::{ChunkId, TransferId},
    path::RelativePath,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferManifest {
    transfer_id: TransferId,
    entries: Vec<ManifestEntry>,
    total_size: u64,
}

impl TransferManifest {
    pub fn new(transfer_id: TransferId, entries: Vec<ManifestEntry>) -> Self {
        let total_size = entries
            .iter()
            .filter_map(|entry| match entry {
                ManifestEntry::File(file) => Some(file.size()),
                ManifestEntry::Directory(_) => None,
            })
            .sum();

        Self { transfer_id, entries, total_size }
    }

    pub const fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }

    pub fn entries(&self) -> &[ManifestEntry] {
        &self.entries
    }

    pub const fn total_size(&self) -> u64 {
        self.total_size
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManifestEntry {
    File(FileEntry),
    Directory(DirectoryEntry),
}

impl ManifestEntry {
    pub const fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    pub const fn is_directory(&self) -> bool {
        matches!(self, Self::Directory(_))
    }

    pub fn path(&self) -> &RelativePath {
        match self {
            Self::File(file) => file.path(),
            Self::Directory(directory) => directory.path(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileEntry {
    path: RelativePath,
    size: u64,
    chunks: Vec<ChunkRef>,
}

impl FileEntry {
    pub fn new(path: RelativePath, size: u64, chunks: Vec<ChunkRef>) -> Self {
        Self { path, size, chunks }
    }

    pub const fn path(&self) -> &RelativePath {
        &self.path
    }

    pub const fn size(&self) -> u64 {
        self.size
    }

    pub fn chunks(&self) -> &[ChunkRef] {
        &self.chunks
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectoryEntry {
    path: RelativePath,
}

impl DirectoryEntry {
    pub const fn new(path: RelativePath) -> Self {
        Self { path }
    }

    pub const fn path(&self) -> &RelativePath {
        &self.path
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkRef {
    id: ChunkId,
    offset: u64,
    size: u32,
}

impl ChunkRef {
    pub fn new(id: ChunkId, offset: u64, size: u32) -> Self {
        assert!(size > 0, "chunk size must be greater than zero");

        Self { id, offset, size }
    }

    pub const fn id(&self) -> ChunkId {
        self.id
    }

    pub const fn offset(&self) -> u64 {
        self.offset
    }

    pub const fn size(&self) -> u32 {
        self.size
    }
}
