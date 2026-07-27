use crate::{ChunkRef, DirectoryEntry, FileEntry, ManifestEntry, ProtocolError, RelativePath};

use super::{super::proto, ids::*};

pub(super) fn encode_manifest_entry(entry: &ManifestEntry) -> proto::ManifestEntry {
    use proto::manifest_entry::Entry;

    proto::ManifestEntry {
        entry: Some(match entry {
            ManifestEntry::File(file) => Entry::File(encode_file_entry(file)),
            ManifestEntry::Directory(directory) => {
                Entry::Directory(encode_directory_entry(directory))
            }
        }),
    }
}

pub(super) fn decode_manifest_entry(
    entry: proto::ManifestEntry,
) -> Result<ManifestEntry, ProtocolError> {
    use proto::manifest_entry::Entry;

    match entry.entry.ok_or(ProtocolError::InvalidManifestEntry)? {
        Entry::File(file) => Ok(ManifestEntry::File(decode_file_entry(file)?)),

        Entry::Directory(directory) => {
            Ok(ManifestEntry::Directory(decode_directory_entry(directory)?))
        }
    }
}

pub(super) fn encode_file_entry(file: &FileEntry) -> proto::FileEntry {
    proto::FileEntry {
        path: file.path().to_string(),
        size: file.size(),
        chunks: file.chunks().iter().map(encode_chunk_ref).collect(),
    }
}

pub(super) fn decode_file_entry(file: proto::FileEntry) -> Result<FileEntry, ProtocolError> {
    let path = RelativePath::parse(file.path)?;

    let chunks = file.chunks.into_iter().map(decode_chunk_ref).collect::<Result<Vec<_>, _>>()?;

    Ok(FileEntry::new(path, file.size, chunks))
}

pub(super) fn encode_directory_entry(directory: &DirectoryEntry) -> proto::DirectoryEntry {
    proto::DirectoryEntry { path: directory.path().to_string() }
}

pub(super) fn decode_directory_entry(
    directory: proto::DirectoryEntry,
) -> Result<DirectoryEntry, ProtocolError> {
    Ok(DirectoryEntry::new(RelativePath::parse(directory.path)?))
}

pub(super) fn encode_chunk_ref(chunk: &ChunkRef) -> proto::ChunkRef {
    proto::ChunkRef {
        id: Some(encode_chunk_id(chunk.id())),
        offset: chunk.offset(),
        size: chunk.size(),
    }
}

pub(super) fn decode_chunk_ref(chunk: proto::ChunkRef) -> Result<ChunkRef, ProtocolError> {
    let id = decode_chunk_id(chunk.id.ok_or(ProtocolError::MissingField("id"))?)?;

    Ok(ChunkRef::new(id, chunk.offset, chunk.size))
}
