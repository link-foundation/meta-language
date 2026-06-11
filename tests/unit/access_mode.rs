use meta_language::{
    AccessMode, EngineNetwork, LinkMetadata, LinkNetwork, LinkType, NetworkProjection,
    ParseConfiguration, ReadOnlyNetwork, ReadOnlyViolation,
};

#[test]
fn parse_configuration_defaults_to_mutable_access() {
    assert_eq!(
        ParseConfiguration::default().access_mode(),
        AccessMode::Mutable
    );
    assert!(AccessMode::default().is_mutable());
    assert!(!AccessMode::default().is_read_only());

    let read_only = ParseConfiguration::default().with_access_mode(AccessMode::ReadOnly);
    assert_eq!(read_only.access_mode(), AccessMode::ReadOnly);
    assert!(read_only.access_mode().is_read_only());
    assert_eq!(read_only.access_mode().label(), "read-only");
}

#[test]
fn frozen_view_supports_non_mutating_operations() {
    let network = LinkNetwork::parse("(a b)", "plain-text", ParseConfiguration::default());
    let expected_text = network.reconstruct_text();
    let expected_len = network.len();
    let view = network.freeze();

    // Read-only operations: query, project, reconstruct, verify, serialize.
    assert_eq!(view.reconstruct_text(), expected_text);
    assert_eq!(view.len(), expected_len);
    assert!(!view.is_empty());
    assert_eq!(
        view.projected_links(NetworkProjection::Lossless).count(),
        expected_len
    );
    assert!(view.verify_full_match(None).issues().is_empty());
    assert!(view.find_term("plain-text").is_some());
}

#[test]
fn frozen_view_shares_one_allocation_across_clones() {
    let network = LinkNetwork::parse("alpha", "plain-text", ParseConfiguration::default());
    let view = network.freeze();
    assert_eq!(view.shared_count(), 1);

    let cloned = view.clone();
    assert_eq!(view.shared_count(), 2);
    assert_eq!(cloned.shared_count(), 2);
    assert_eq!(view, cloned);

    drop(cloned);
    assert_eq!(view.shared_count(), 1);
}

#[test]
fn frozen_view_can_fork_back_to_a_mutable_network() {
    let network = LinkNetwork::parse("alpha", "plain-text", ParseConfiguration::default());
    let view = network.freeze();

    let mut editable = view.to_mutable();
    let added = editable.insert_link([], LinkMetadata::new().with_link_type(LinkType::Concept));
    assert!(editable.link(added).is_some());
    // The original frozen view is unaffected by edits to the fork.
    assert!(view.link(added).is_none());

    // into_mutable reuses the allocation when the view is the only handle.
    let recovered = view.into_mutable();
    assert_eq!(recovered.reconstruct_text(), "alpha");
}

#[test]
fn parse_engine_returns_read_only_form_under_read_only_mode() {
    let configuration = ParseConfiguration::default().with_access_mode(AccessMode::ReadOnly);
    let mut engine = LinkNetwork::parse_engine("alpha beta", "plain-text", configuration);

    assert!(engine.is_read_only());
    assert_eq!(engine.access_mode(), AccessMode::ReadOnly);
    assert_eq!(engine.network().reconstruct_text(), "alpha beta");
    // Deref exposes the read-only operations directly.
    assert_eq!(engine.reconstruct_text(), "alpha beta");

    // Mutation at the engine boundary fails with a clear diagnostic.
    let error = engine
        .as_mutable()
        .expect_err("read-only engine rejects mutation");
    assert_eq!(error, ReadOnlyViolation);
    assert!(error.to_string().contains("read-only"));
}

#[test]
fn parse_engine_returns_mutable_form_by_default() {
    let mut engine =
        LinkNetwork::parse_engine("alpha", "plain-text", ParseConfiguration::default());

    assert!(engine.is_mutable());
    assert_eq!(engine.access_mode(), AccessMode::Mutable);

    let editable = engine.as_mutable().expect("mutable engine allows mutation");
    let added = editable.insert_link([], LinkMetadata::new().with_link_type(LinkType::Concept));
    assert!(engine.network().link(added).is_some());
}

#[test]
fn read_only_view_interoperates_with_snapshots() {
    let network = LinkNetwork::parse("alpha", "plain-text", ParseConfiguration::default());

    // Snapshot -> read-only view reuses the snapshot's network allocation.
    let snapshot = network.snapshot(1, "initial parse");
    let view = snapshot.as_read_only();
    assert_eq!(view.reconstruct_text(), "alpha");
    // The snapshot and the view share the same Arc, not a clone.
    assert_eq!(snapshot.shared_snapshot_count(), 2);
    assert_eq!(view.shared_count(), 2);

    // Read-only view -> snapshot reuses the view's network allocation.
    let frozen = network.freeze();
    let rebuilt = meta_language::NetworkSnapshot::from_read_only(5, &frozen, "frozen import");
    assert_eq!(rebuilt.version(), 5);
    assert_eq!(rebuilt.provenance(), "frozen import");
    assert_eq!(rebuilt.network().reconstruct_text(), "alpha");
    assert_eq!(frozen.shared_count(), 2);
}

#[test]
fn engine_network_round_trips_between_modes() {
    let mutable = EngineNetwork::with_access_mode(
        LinkNetwork::parse("alpha", "plain-text", ParseConfiguration::default()),
        AccessMode::Mutable,
    );
    let frozen: ReadOnlyNetwork = mutable.into_read_only();
    assert_eq!(frozen.reconstruct_text(), "alpha");

    let editable = EngineNetwork::ReadOnly(frozen).into_mutable();
    assert_eq!(editable.reconstruct_text(), "alpha");
}
