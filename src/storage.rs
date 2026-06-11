//! Pluggable storage backends for meta-language links.
//!
//! [`LinkStore`] is the storage boundary for the links network: reads use
//! `&self`, writes use `&mut self`, and the default implementation is the
//! existing in-memory [`LinkNetwork`](crate::LinkNetwork). The optional
//! `doublets` Cargo feature adds a file-mapped binary backend over the
//! `doublets` crate.

use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::access::{EngineNetwork, ReadOnlyNetwork, ReadOnlyViolation};
use crate::configuration::AccessMode;
use crate::link_network::{Link, LinkId, LinkMetadata, LinkNetwork, LinkType};

/// Storage-level errors returned by [`LinkStore`] implementations.
#[derive(Debug)]
pub enum StorageError {
    /// A write was attempted through a read-only storage handle.
    ReadOnly(ReadOnlyViolation),
    /// File I/O failed while opening or maintaining a file-backed store.
    Io(std::io::Error),
    /// The optional `doublets` backend returned an error.
    Doublets(String),
    /// Stored bytes do not match the meta-language storage schema.
    Corrupt(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadOnly(error) => error.fmt(formatter),
            Self::Io(error) => write!(formatter, "storage I/O error: {error}"),
            Self::Doublets(error) => write!(formatter, "doublets storage error: {error}"),
            Self::Corrupt(error) => write!(formatter, "corrupt storage: {error}"),
        }
    }
}

impl Error for StorageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ReadOnly(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::Doublets(_) | Self::Corrupt(_) => None,
        }
    }
}

impl From<ReadOnlyViolation> for StorageError {
    fn from(error: ReadOnlyViolation) -> Self {
        Self::ReadOnly(error)
    }
}

impl From<std::io::Error> for StorageError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// Storage backend names used by downstream engine configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkStoreBackend {
    /// Store and exchange networks through canonical LiNo text.
    LinoProjection,
    /// Store networks in `doublets-rs` file-mapped binary doublets.
    DoubletsRs,
    /// Exchange the same binary layout with a browser-side `doublets-web` host.
    DoubletsWeb,
}

impl LinkStoreBackend {
    /// Human-readable backend label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::LinoProjection => "LiNo projection",
            Self::DoubletsRs => "doublets-rs",
            Self::DoubletsWeb => "doublets-web",
        }
    }
}

/// Query used by [`LinkStore::search`] and [`LinkStore::count`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkStoreQuery {
    id: Option<LinkId>,
    references: Option<Vec<LinkId>>,
    link_type: Option<LinkType>,
    term: Option<String>,
    language: Option<String>,
    named: Option<bool>,
}

impl LinkStoreQuery {
    /// Creates a query that matches every link.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: None,
            references: None,
            link_type: None,
            term: None,
            language: None,
            named: None,
        }
    }

    /// Restricts the query to one link id.
    #[must_use]
    pub const fn with_id(mut self, id: LinkId) -> Self {
        self.id = Some(id);
        self
    }

    /// Restricts the query to links with exactly these ordered references.
    #[must_use]
    pub fn with_references<I>(mut self, references: I) -> Self
    where
        I: IntoIterator<Item = LinkId>,
    {
        self.references = Some(references.into_iter().collect());
        self
    }

    /// Restricts the query to a link type.
    #[must_use]
    pub const fn with_link_type(mut self, link_type: LinkType) -> Self {
        self.link_type = Some(link_type);
        self
    }

    /// Restricts the query to links with this term.
    #[must_use]
    pub fn with_term(mut self, term: impl Into<String>) -> Self {
        self.term = Some(term.into());
        self
    }

    /// Restricts the query to links with this language label.
    #[must_use]
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Restricts the query to named or anonymous links.
    #[must_use]
    pub const fn with_named(mut self, named: bool) -> Self {
        self.named = Some(named);
        self
    }

    fn matches(&self, link: &Link) -> bool {
        if self.id.is_some_and(|id| id != link.id()) {
            return false;
        }
        if self
            .references
            .as_deref()
            .is_some_and(|references| references != link.references())
        {
            return false;
        }
        if self
            .link_type
            .is_some_and(|link_type| Some(link_type) != link.metadata().link_type())
        {
            return false;
        }
        if self
            .term
            .as_deref()
            .is_some_and(|term| Some(term) != link.metadata().term())
        {
            return false;
        }
        if self
            .language
            .as_deref()
            .is_some_and(|language| Some(language) != link.metadata().language())
        {
            return false;
        }
        if self
            .named
            .is_some_and(|named| named != link.metadata().is_named())
        {
            return false;
        }
        true
    }
}

/// Storage trait for create/read/update/delete/search over meta-language links.
pub trait LinkStore {
    /// Creates a link and returns its stable id.
    ///
    /// # Errors
    ///
    /// Returns [`StorageError`] when the backend cannot write the link.
    fn create(
        &mut self,
        references: &[LinkId],
        metadata: LinkMetadata,
    ) -> Result<LinkId, StorageError>;

    /// Reads a link by id.
    ///
    /// # Errors
    ///
    /// Returns [`StorageError`] when the backend cannot read its storage.
    fn read(&self, id: LinkId) -> Result<Option<Link>, StorageError>;

    /// Replaces an existing link's references and metadata.
    ///
    /// # Errors
    ///
    /// Returns [`StorageError`] when the backend cannot write the update.
    fn update(
        &mut self,
        id: LinkId,
        references: &[LinkId],
        metadata: LinkMetadata,
    ) -> Result<bool, StorageError>;

    /// Deletes a link by id.
    ///
    /// # Errors
    ///
    /// Returns [`StorageError`] when the backend cannot delete the link.
    fn delete(&mut self, id: LinkId) -> Result<bool, StorageError>;

    /// Returns links matching `query`.
    ///
    /// # Errors
    ///
    /// Returns [`StorageError`] when the backend cannot search its storage.
    fn search(&self, query: &LinkStoreQuery) -> Result<Vec<Link>, StorageError>;

    /// Counts links matching `query`.
    ///
    /// # Errors
    ///
    /// Returns [`StorageError`] when the backend cannot search its storage.
    fn count(&self, query: &LinkStoreQuery) -> Result<usize, StorageError> {
        self.search(query).map(|links| links.len())
    }
}

impl LinkStore for LinkNetwork {
    fn create(
        &mut self,
        references: &[LinkId],
        metadata: LinkMetadata,
    ) -> Result<LinkId, StorageError> {
        Ok(self.insert_dynamic_link(references, metadata))
    }

    fn read(&self, id: LinkId) -> Result<Option<Link>, StorageError> {
        Ok(self.link(id).cloned())
    }

    fn update(
        &mut self,
        id: LinkId,
        references: &[LinkId],
        metadata: LinkMetadata,
    ) -> Result<bool, StorageError> {
        Ok(replace_network_link(self, id, references, metadata, true))
    }

    fn delete(&mut self, id: LinkId) -> Result<bool, StorageError> {
        Ok(delete_network_link(self, id))
    }

    fn search(&self, query: &LinkStoreQuery) -> Result<Vec<Link>, StorageError> {
        Ok(self
            .links()
            .filter(|link| query.matches(link))
            .cloned()
            .collect())
    }
}

fn insert_network_link_with_id(
    network: &mut LinkNetwork,
    id: LinkId,
    references: &[LinkId],
    metadata: LinkMetadata,
    registered_term: bool,
) {
    let term = registered_term
        .then(|| metadata.term().map(Arc::<str>::from))
        .flatten();
    network.links.insert(
        id,
        Arc::new(Link {
            id,
            references: Arc::from(references.to_vec()),
            metadata,
        }),
    );
    if let Some(term) = term {
        network.terms.insert(term, id);
    }
    network.next_id = network.next_id.max(id.as_u64() + 1);
}

fn replace_network_link(
    network: &mut LinkNetwork,
    id: LinkId,
    references: &[LinkId],
    metadata: LinkMetadata,
    registered_term: bool,
) -> bool {
    if !network.links.contains_key(&id) {
        return false;
    }
    network.terms.retain(|_, stored_id| *stored_id != id);
    insert_network_link_with_id(network, id, references, metadata, registered_term);
    true
}

fn delete_network_link(network: &mut LinkNetwork, id: LinkId) -> bool {
    let removed = network.links.remove(&id).is_some();
    if removed {
        network.terms.retain(|_, stored_id| *stored_id != id);
    }
    removed
}

#[cfg(feature = "doublets")]
fn network_from_stored_links(links: Vec<(Link, bool)>) -> LinkNetwork {
    let mut network = LinkNetwork::new();
    for (link, registered_term) in links {
        insert_network_link_with_id(
            &mut network,
            link.id,
            &link.references,
            link.metadata,
            registered_term,
        );
    }
    network
}

impl LinkStore for ReadOnlyNetwork {
    fn create(
        &mut self,
        _references: &[LinkId],
        _metadata: LinkMetadata,
    ) -> Result<LinkId, StorageError> {
        Err(ReadOnlyViolation.into())
    }

    fn read(&self, id: LinkId) -> Result<Option<Link>, StorageError> {
        LinkStore::read(self.network(), id)
    }

    fn update(
        &mut self,
        _id: LinkId,
        _references: &[LinkId],
        _metadata: LinkMetadata,
    ) -> Result<bool, StorageError> {
        Err(ReadOnlyViolation.into())
    }

    fn delete(&mut self, _id: LinkId) -> Result<bool, StorageError> {
        Err(ReadOnlyViolation.into())
    }

    fn search(&self, query: &LinkStoreQuery) -> Result<Vec<Link>, StorageError> {
        LinkStore::search(self.network(), query)
    }
}

impl LinkStore for EngineNetwork {
    fn create(
        &mut self,
        references: &[LinkId],
        metadata: LinkMetadata,
    ) -> Result<LinkId, StorageError> {
        LinkStore::create(self.as_mutable()?, references, metadata)
    }

    fn read(&self, id: LinkId) -> Result<Option<Link>, StorageError> {
        LinkStore::read(self.network(), id)
    }

    fn update(
        &mut self,
        id: LinkId,
        references: &[LinkId],
        metadata: LinkMetadata,
    ) -> Result<bool, StorageError> {
        LinkStore::update(self.as_mutable()?, id, references, metadata)
    }

    fn delete(&mut self, id: LinkId) -> Result<bool, StorageError> {
        LinkStore::delete(self.as_mutable()?, id)
    }

    fn search(&self, query: &LinkStoreQuery) -> Result<Vec<Link>, StorageError> {
        LinkStore::search(self.network(), query)
    }
}

/// Access-mode wrapper for any [`LinkStore`] implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineLinkStore<S> {
    /// Mutable storage.
    Mutable(S),
    /// Read-only storage.
    ReadOnly(S),
}

impl<S> EngineLinkStore<S> {
    /// Wraps a store according to the configured access mode.
    #[must_use]
    pub fn with_access_mode(store: S, access_mode: AccessMode) -> Self {
        match access_mode {
            AccessMode::Mutable => Self::Mutable(store),
            AccessMode::ReadOnly => Self::ReadOnly(store),
        }
    }

    /// Returns the configured access mode.
    #[must_use]
    pub const fn access_mode(&self) -> AccessMode {
        match self {
            Self::Mutable(_) => AccessMode::Mutable,
            Self::ReadOnly(_) => AccessMode::ReadOnly,
        }
    }

    /// Borrows the wrapped store.
    #[must_use]
    pub const fn store(&self) -> &S {
        match self {
            Self::Mutable(store) | Self::ReadOnly(store) => store,
        }
    }

    /// Consumes the wrapper and returns the store.
    pub fn into_inner(self) -> S {
        match self {
            Self::Mutable(store) | Self::ReadOnly(store) => store,
        }
    }
}

impl<S: LinkStore> LinkStore for EngineLinkStore<S> {
    fn create(
        &mut self,
        references: &[LinkId],
        metadata: LinkMetadata,
    ) -> Result<LinkId, StorageError> {
        match self {
            Self::Mutable(store) => store.create(references, metadata),
            Self::ReadOnly(_) => Err(ReadOnlyViolation.into()),
        }
    }

    fn read(&self, id: LinkId) -> Result<Option<Link>, StorageError> {
        self.store().read(id)
    }

    fn update(
        &mut self,
        id: LinkId,
        references: &[LinkId],
        metadata: LinkMetadata,
    ) -> Result<bool, StorageError> {
        match self {
            Self::Mutable(store) => store.update(id, references, metadata),
            Self::ReadOnly(_) => Err(ReadOnlyViolation.into()),
        }
    }

    fn delete(&mut self, id: LinkId) -> Result<bool, StorageError> {
        match self {
            Self::Mutable(store) => store.delete(id),
            Self::ReadOnly(_) => Err(ReadOnlyViolation.into()),
        }
    }

    fn search(&self, query: &LinkStoreQuery) -> Result<Vec<Link>, StorageError> {
        self.store().search(query)
    }
}

#[cfg(feature = "doublets")]
mod doublets;
#[cfg(feature = "doublets")]
pub use doublets::DoubletsLinkStore;
