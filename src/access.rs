//! Read-only and mutable engine access controls.
//!
//! [`LinkNetwork`] is mutable by construction. This module adds the
//! [`AccessMode`](crate::configuration::AccessMode)-driven counterpart: a
//! frozen [`ReadOnlyNetwork`] view that exposes only `&self` operations
//! (query, project, reconstruct, verify, serialize) and makes mutation a
//! compile-time error, plus an [`EngineNetwork`] boundary that honours the
//! configured access mode and rejects mutation at runtime with a clear
//! diagnostic.
//!
//! The frozen view reuses the same `Arc<LinkNetwork>` sharing as
//! [`NetworkSnapshot`](crate::snapshots::NetworkSnapshot), so read-only access
//! composes with snapshot versioning instead of duplicating it.

use std::error::Error;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use crate::configuration::{AccessMode, ParseConfiguration};
use crate::link_network::LinkNetwork;

/// Error raised when a mutation is attempted through a read-only engine handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReadOnlyViolation;

impl fmt::Display for ReadOnlyViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            "engine is configured read-only; mutation is rejected. \
             Re-parse with AccessMode::Mutable or fork an editable copy via \
             ReadOnlyNetwork::to_mutable before mutating.",
        )
    }
}

impl Error for ReadOnlyViolation {}

/// Compile-time read-only view over a shared links network.
///
/// `ReadOnlyNetwork` derefs to `&LinkNetwork`, so every non-mutating public
/// operation is reachable while the `&mut self` mutators (`insert_link`,
/// `set_references`, `set_span`, `set_flags`, `apply_substitution`, ...) are
/// unreachable: there is no `DerefMut`, so the borrow checker rejects any
/// attempt to call them. The wrapped network is held behind an `Arc`, so
/// cloning a view shares one allocation rather than copying the network.
///
/// Read-only operations compile and run:
///
/// ```
/// use meta_language::{LinkNetwork, ParseConfiguration};
///
/// let view = LinkNetwork::parse("alpha", "plain-text", ParseConfiguration::default()).freeze();
/// assert_eq!(view.reconstruct_text(), "alpha");
/// ```
///
/// Mutation does not compile, because the mutators require `&mut LinkNetwork`
/// and the view only ever yields `&LinkNetwork`:
///
/// ```compile_fail
/// use meta_language::{LinkMetadata, LinkNetwork, ParseConfiguration};
///
/// let view = LinkNetwork::parse("alpha", "plain-text", ParseConfiguration::default()).freeze();
/// view.insert_link([], LinkMetadata::new()); // error: cannot borrow as mutable
/// ```
#[derive(Clone, Debug)]
pub struct ReadOnlyNetwork {
    network: Arc<LinkNetwork>,
}

impl ReadOnlyNetwork {
    /// Freezes an owned network into a read-only view.
    #[must_use]
    pub fn new(network: LinkNetwork) -> Self {
        Self {
            network: Arc::new(network),
        }
    }

    /// Wraps an already shared network as a read-only view.
    ///
    /// This reuses the existing allocation, allowing read-only access to
    /// compose with snapshot versioning without re-cloning the network.
    #[must_use]
    pub const fn from_shared(network: Arc<LinkNetwork>) -> Self {
        Self { network }
    }

    /// Borrows the underlying immutable network.
    #[must_use]
    pub fn network(&self) -> &LinkNetwork {
        &self.network
    }

    /// Borrows the shared network handle.
    #[must_use]
    pub const fn shared(&self) -> &Arc<LinkNetwork> {
        &self.network
    }

    /// Consumes the view and returns the shared network handle.
    #[must_use]
    pub fn into_shared(self) -> Arc<LinkNetwork> {
        self.network
    }

    /// Number of handles sharing the frozen network allocation.
    #[must_use]
    pub fn shared_count(&self) -> usize {
        Arc::strong_count(&self.network)
    }

    /// Forks an editable copy so callers can return to a mutable engine.
    #[must_use]
    pub fn to_mutable(&self) -> LinkNetwork {
        self.network.as_ref().clone()
    }

    /// Consumes the view and returns an editable network, reusing the
    /// allocation when this is the only handle and cloning otherwise.
    #[must_use]
    pub fn into_mutable(self) -> LinkNetwork {
        Arc::try_unwrap(self.network).unwrap_or_else(|shared| shared.as_ref().clone())
    }
}

impl Deref for ReadOnlyNetwork {
    type Target = LinkNetwork;

    fn deref(&self) -> &Self::Target {
        &self.network
    }
}

impl PartialEq for ReadOnlyNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.network == other.network
    }
}

impl Eq for ReadOnlyNetwork {}

impl From<LinkNetwork> for ReadOnlyNetwork {
    fn from(network: LinkNetwork) -> Self {
        Self::new(network)
    }
}

impl LinkNetwork {
    /// Freezes this network into a read-only view, consuming it.
    ///
    /// Mutators are unreachable on the returned [`ReadOnlyNetwork`] at compile
    /// time; only `&self` operations remain available.
    #[must_use]
    pub fn freeze(self) -> ReadOnlyNetwork {
        ReadOnlyNetwork::new(self)
    }

    /// Returns a read-only view sharing a clone of this network.
    #[must_use]
    pub fn as_read_only(&self) -> ReadOnlyNetwork {
        ReadOnlyNetwork::new(self.clone())
    }

    /// Parses source text honouring the configured engine access mode.
    ///
    /// Under [`AccessMode::Mutable`] (the default) this returns an editable
    /// network; under [`AccessMode::ReadOnly`] it returns the frozen form,
    /// where mutation attempts at the engine boundary fail with a clear
    /// diagnostic.
    #[must_use]
    pub fn parse_engine(
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> EngineNetwork {
        let network = Self::parse(text, language, configuration);
        EngineNetwork::with_access_mode(network, configuration.access_mode())
    }
}

/// Access-mode-aware engine handle returned by configured parsing.
///
/// This is the runtime boundary where the configured
/// [`AccessMode`](crate::configuration::AccessMode) is enforced: a read-only
/// engine yields a frozen view and rejects [`EngineNetwork::as_mutable`] with a
/// [`ReadOnlyViolation`], while a mutable engine hands back the editable
/// network.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineNetwork {
    /// An editable network produced under [`AccessMode::Mutable`].
    Mutable(LinkNetwork),
    /// A frozen view produced under [`AccessMode::ReadOnly`].
    ReadOnly(ReadOnlyNetwork),
}

impl EngineNetwork {
    /// Wraps a network according to the supplied access mode.
    #[must_use]
    pub fn with_access_mode(network: LinkNetwork, access_mode: AccessMode) -> Self {
        match access_mode {
            AccessMode::Mutable => Self::Mutable(network),
            AccessMode::ReadOnly => Self::ReadOnly(network.freeze()),
        }
    }

    /// The access mode this handle was created with.
    #[must_use]
    pub const fn access_mode(&self) -> AccessMode {
        match self {
            Self::Mutable(_) => AccessMode::Mutable,
            Self::ReadOnly(_) => AccessMode::ReadOnly,
        }
    }

    /// Whether this handle permits mutation.
    #[must_use]
    pub const fn is_mutable(&self) -> bool {
        matches!(self, Self::Mutable(_))
    }

    /// Whether this handle is read-only.
    #[must_use]
    pub const fn is_read_only(&self) -> bool {
        matches!(self, Self::ReadOnly(_))
    }

    /// Borrows the underlying network for read-only operations regardless of
    /// the access mode.
    #[must_use]
    pub fn network(&self) -> &LinkNetwork {
        match self {
            Self::Mutable(network) => network,
            Self::ReadOnly(view) => view.network(),
        }
    }

    /// Borrows the network mutably, or fails with a clear diagnostic when the
    /// engine is read-only.
    ///
    /// # Errors
    ///
    /// Returns [`ReadOnlyViolation`] when this handle was created under
    /// [`AccessMode::ReadOnly`].
    pub fn as_mutable(&mut self) -> Result<&mut LinkNetwork, ReadOnlyViolation> {
        match self {
            Self::Mutable(network) => Ok(network),
            Self::ReadOnly(_) => Err(ReadOnlyViolation),
        }
    }

    /// Converts this handle into a read-only view, freezing a mutable network.
    #[must_use]
    pub fn into_read_only(self) -> ReadOnlyNetwork {
        match self {
            Self::Mutable(network) => network.freeze(),
            Self::ReadOnly(view) => view,
        }
    }

    /// Converts this handle into an editable network, forking a read-only view.
    #[must_use]
    pub fn into_mutable(self) -> LinkNetwork {
        match self {
            Self::Mutable(network) => network,
            Self::ReadOnly(view) => view.into_mutable(),
        }
    }
}

impl Deref for EngineNetwork {
    type Target = LinkNetwork;

    fn deref(&self) -> &Self::Target {
        self.network()
    }
}
