use crate::component::Component;
use get_size::GetSize;
use serde::Deserialize;
use std::collections::BTreeMap;
use time::OffsetDateTime;

/// A parsed Zarr manifest
#[derive(Clone, Debug, Deserialize, Eq, GetSize, PartialEq)]
pub(crate) struct Manifest {
    /// A tree of the Zarr's entries
    pub(crate) entries: ManifestFolder,
}

/// A representation of a folder within a Zarr manifest: a mapping from entry &
/// subdirectory names to the entries & subdirectories
pub(crate) type ManifestFolder = BTreeMap<Component, FolderEntry>;

#[derive(Clone, Debug, Deserialize, Eq, GetSize, PartialEq)]
#[serde(untagged)]
pub(crate) enum FolderEntry {
    Folder(ManifestFolder),
    Entry(ManifestEntry),
}

/// Information on a Zarr entry in a manifest as of the point in time
/// represented by the manifest
#[derive(Clone, Debug, Deserialize, Eq, GetSize, PartialEq)]
pub(crate) struct ManifestEntry {
    // IMPORTANT: Keep these fields in this order so that deserialization will
    // work properly!
    /// The S3 version ID of the entry's S3 object
    pub(crate) version_id: String,

    /// The entry's S3 object's modification time
    #[get_size(size = 0)] // Nothing on the heap
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) modified: OffsetDateTime,

    /// The size of the entry in bytes
    pub(crate) size: i64,

    /// The ETag of the entry's S3 object
    pub(crate) etag: String,
}
