use meta_language::{
    AccessMode, EngineLinkStore, EngineNetwork, LinkMetadata, LinkNetwork, LinkStore,
    LinkStoreQuery, LinkType, ParseConfiguration, StorageError,
};

#[test]
fn in_memory_link_network_implements_link_store_crud_and_search() {
    let mut store = LinkNetwork::new();

    let parent = LinkStore::create(
        &mut store,
        &[],
        LinkMetadata::new()
            .with_link_type(LinkType::Concept)
            .with_term("parent"),
    )
    .expect("create parent");
    let child = LinkStore::create(
        &mut store,
        &[parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Relation)
            .with_term("child"),
    )
    .expect("create child");

    assert_eq!(store.find_term("parent"), Some(parent));
    assert_eq!(
        LinkStore::read(&store, child)
            .expect("read child")
            .expect("child exists")
            .references(),
        &[parent]
    );

    let matches = LinkStore::search(
        &store,
        &LinkStoreQuery::new()
            .with_link_type(LinkType::Relation)
            .with_references([parent]),
    )
    .expect("search relation links");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].id(), child);

    let updated = LinkStore::update(
        &mut store,
        child,
        &[],
        LinkMetadata::new()
            .with_link_type(LinkType::Concept)
            .with_term("updated"),
    )
    .expect("update child");
    assert!(updated);
    assert_eq!(store.find_term("child"), None);
    assert_eq!(store.find_term("updated"), Some(child));

    assert!(LinkStore::delete(&mut store, child).expect("delete child"));
    assert!(LinkStore::read(&store, child)
        .expect("read deleted child")
        .is_none());
}

#[test]
fn access_controlled_link_store_rejects_writes_in_read_only_mode() {
    let network = LinkNetwork::parse("alpha", "plain-text", ParseConfiguration::default());
    let mut engine = EngineNetwork::with_access_mode(network, AccessMode::ReadOnly);

    assert_eq!(
        LinkStore::count(&engine, &LinkStoreQuery::new()).expect("count read-only engine"),
        engine.len()
    );

    let error = LinkStore::create(&mut engine, &[], LinkMetadata::new())
        .expect_err("read-only engine rejects storage writes");
    assert!(matches!(error, StorageError::ReadOnly(_)));

    let mut generic = EngineLinkStore::with_access_mode(LinkNetwork::new(), AccessMode::ReadOnly);
    let error = LinkStore::delete(&mut generic, meta_language::LinkId::from_u64(1))
        .expect_err("generic read-only store rejects deletes");
    assert!(matches!(error, StorageError::ReadOnly(_)));
}
