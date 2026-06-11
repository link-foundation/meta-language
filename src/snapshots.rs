use std::collections::BTreeSet;
use std::sync::Arc;

use crate::access::ReadOnlyNetwork;
use crate::link_network::{LinkId, LinkNetwork};

/// Changed, added, and removed link identifiers between two network versions.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StructuralDiff {
    changed: BTreeSet<LinkId>,
    added: BTreeSet<LinkId>,
    removed: BTreeSet<LinkId>,
}

impl StructuralDiff {
    /// Creates a structural diff from its changed, added, and removed sets.
    #[must_use]
    pub const fn new(
        changed: BTreeSet<LinkId>,
        added: BTreeSet<LinkId>,
        removed: BTreeSet<LinkId>,
    ) -> Self {
        Self {
            changed,
            added,
            removed,
        }
    }

    /// Link ids present in both versions whose references or metadata changed.
    #[must_use]
    pub const fn changed(&self) -> &BTreeSet<LinkId> {
        &self.changed
    }

    /// Link ids present only in the newer version.
    #[must_use]
    pub const fn added(&self) -> &BTreeSet<LinkId> {
        &self.added
    }

    /// Link ids present only in the older version.
    #[must_use]
    pub const fn removed(&self) -> &BTreeSet<LinkId> {
        &self.removed
    }

    /// Whether the two versions contain the same structural links.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.changed.is_empty() && self.added.is_empty() && self.removed.is_empty()
    }
}

/// Immutable versioned view of a links network.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkSnapshot {
    version: u64,
    parent_version: Option<u64>,
    provenance: String,
    network: Arc<LinkNetwork>,
}

impl NetworkSnapshot {
    /// Creates an immutable snapshot from a network value.
    #[must_use]
    pub fn new(version: u64, network: LinkNetwork, provenance: impl Into<String>) -> Self {
        Self {
            version,
            parent_version: None,
            provenance: provenance.into(),
            network: Arc::new(network),
        }
    }

    /// Snapshot version.
    #[must_use]
    pub const fn version(&self) -> u64 {
        self.version
    }

    /// Parent version when this snapshot was committed from a mutable snapshot.
    #[must_use]
    pub const fn parent_version(&self) -> Option<u64> {
        self.parent_version
    }

    /// Human-readable change provenance for this snapshot.
    #[must_use]
    pub fn provenance(&self) -> &str {
        &self.provenance
    }

    /// Immutable network data held by this snapshot.
    #[must_use]
    pub fn network(&self) -> &LinkNetwork {
        self.network.as_ref()
    }

    /// Number of immutable snapshot handles sharing the same network value.
    #[must_use]
    pub fn shared_snapshot_count(&self) -> usize {
        Arc::strong_count(&self.network)
    }

    /// Builds an immutable snapshot from a frozen read-only view.
    ///
    /// The read-only view's `Arc<LinkNetwork>` is reused directly, so freezing
    /// and snapshot versioning share one network allocation.
    #[must_use]
    pub fn from_read_only(
        version: u64,
        view: &ReadOnlyNetwork,
        provenance: impl Into<String>,
    ) -> Self {
        Self {
            version,
            parent_version: None,
            provenance: provenance.into(),
            network: view.shared().clone(),
        }
    }

    /// Returns a read-only view sharing this snapshot's network allocation.
    #[must_use]
    pub fn as_read_only(&self) -> ReadOnlyNetwork {
        ReadOnlyNetwork::from_shared(self.network.clone())
    }

    /// Computes changed, added, and removed link ids against another snapshot.
    #[must_use]
    pub fn structural_diff(&self, other: &Self) -> StructuralDiff {
        self.network().structural_diff(other.network())
    }

    /// Creates an editable snapshot fork from this immutable snapshot.
    #[must_use]
    pub fn to_mutable(&self, provenance: impl Into<String>) -> MutableNetworkSnapshot {
        MutableNetworkSnapshot {
            base_version: self.version,
            network: self.network().clone(),
            provenance: provenance.into(),
        }
    }

    fn committed(
        version: u64,
        parent_version: u64,
        network: LinkNetwork,
        provenance: String,
    ) -> Self {
        Self {
            version,
            parent_version: Some(parent_version),
            provenance,
            network: Arc::new(network),
        }
    }
}

/// Editable fork of an immutable network snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MutableNetworkSnapshot {
    base_version: u64,
    network: LinkNetwork,
    provenance: String,
}

impl MutableNetworkSnapshot {
    /// Version this mutable snapshot was forked from.
    #[must_use]
    pub const fn base_version(&self) -> u64 {
        self.base_version
    }

    /// Human-readable provenance that will be attached when committed.
    #[must_use]
    pub fn provenance(&self) -> &str {
        &self.provenance
    }

    /// Immutable view of the editable network.
    #[must_use]
    pub const fn network(&self) -> &LinkNetwork {
        &self.network
    }

    /// Mutable view of the editable network.
    pub fn network_mut(&mut self) -> &mut LinkNetwork {
        &mut self.network
    }

    /// Commits this mutable snapshot as the next sequential version.
    #[must_use]
    pub fn commit(self) -> NetworkSnapshot {
        let next_version = self
            .base_version
            .checked_add(1)
            .expect("snapshot version overflow");
        self.commit_as(next_version)
    }

    /// Commits this mutable snapshot with an explicit forward version.
    #[must_use]
    pub fn commit_as(self, version: u64) -> NetworkSnapshot {
        assert!(
            version > self.base_version,
            "snapshot version must move forward"
        );
        NetworkSnapshot::committed(version, self.base_version, self.network, self.provenance)
    }
}

impl LinkNetwork {
    /// Captures the current network as an immutable versioned snapshot.
    #[must_use]
    pub fn snapshot(&self, version: u64, provenance: impl Into<String>) -> NetworkSnapshot {
        NetworkSnapshot::new(version, self.clone(), provenance)
    }

    /// Computes changed, added, and removed link ids against another network.
    #[must_use]
    pub fn structural_diff(&self, other: &Self) -> StructuralDiff {
        let old_ids = self
            .links()
            .map(crate::link_network::Link::id)
            .collect::<BTreeSet<_>>();
        let new_ids = other
            .links()
            .map(crate::link_network::Link::id)
            .collect::<BTreeSet<_>>();

        let removed = old_ids.difference(&new_ids).copied().collect();
        let added = new_ids.difference(&old_ids).copied().collect();
        let changed = old_ids
            .intersection(&new_ids)
            .copied()
            .filter(|id| self.link(*id) != other.link(*id))
            .collect();

        StructuralDiff::new(changed, added, removed)
    }
}
