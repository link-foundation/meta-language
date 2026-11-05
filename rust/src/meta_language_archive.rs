//! Complete source-network storage over the shared archive vocabulary.
//! Native allocation, ordered data and graph codecs are host adapters.
use crate::{LinkId, LinkNetwork};
use std::sync::Arc;

mod model;
mod read;
mod write;
pub use read::{
    read_meta_language_archive, read_meta_language_archive_binary,
    read_meta_language_archive_network_text, read_meta_language_archive_text,
};
pub use write::{
    create_meta_language_archive, extend_meta_language_archive, meta_language_archive_to_binary,
    meta_language_archive_to_text,
};

/// A complete network and an explicitly selected library entry point.
#[derive(Clone, Debug)]
pub struct MetaLanguageArchiveEntry {
    /// Stable caller-selected entry identity.
    pub identifier: String,
    /// Shared archive kind, such as concrete syntax or ontology.
    pub kind: String,
    /// Shared allocation identifies entries belonging to the same source graph.
    pub network: Arc<LinkNetwork>,
    /// Original source-network identity of this root.
    pub root: LinkId,
}

/// An archive whose descriptors and actual source graphs are all links.
#[derive(Clone, Debug)]
pub struct MetaLanguageArchive {
    /// The full storage graph.
    pub network: LinkNetwork,
    /// Its explicit versioned root.
    pub root: LinkId,
}

/// Eagerly reconstructed networks and their selected roots.
#[derive(Clone, Debug)]
pub struct ReconstructedMetaLanguageArchive {
    /// Every selected entry; shared source groups retain one Arc allocation.
    pub entries: Vec<MetaLanguageArchiveEntry>,
    /// Every complete source group, including disconnected members.
    pub networks: Vec<Arc<LinkNetwork>>,
}

/// A full decoded archive with its eagerly reconstructed entry views.
#[derive(Clone, Debug)]
pub struct LoadedMetaLanguageArchive {
    /// Actual authoritative descriptor and source links.
    pub archive: MetaLanguageArchive,
    /// Complete selected entry views.
    pub entries: Vec<MetaLanguageArchiveEntry>,
    /// Source groups shared by the entry views.
    pub networks: Vec<Arc<LinkNetwork>>,
}
impl std::ops::Deref for LoadedMetaLanguageArchive {
    type Target = MetaLanguageArchive;
    fn deref(&self) -> &Self::Target {
        &self.archive
    }
}

/// Explicit limits for complete reconstruction.
#[derive(Clone, Copy, Debug)]
pub struct MetaLanguageArchiveLimits {
    /// Maximum actual stored links.
    pub node_limit: usize,
    /// Maximum stored references.
    pub reference_limit: usize,
    /// Maximum metadata or serialized bytes.
    pub byte_limit: usize,
}
impl Default for MetaLanguageArchiveLimits {
    fn default() -> Self {
        Self {
            node_limit: 100_000,
            reference_limit: 1_000_000,
            byte_limit: 128 * 1024 * 1024,
        }
    }
}

pub(crate) fn select_network(
    network: &LinkNetwork,
    root: LinkId,
    limit: usize,
) -> Result<LinkNetwork, String> {
    model::island(network, root, limit)
}
