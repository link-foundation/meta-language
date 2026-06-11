#![cfg(feature = "doublets")]

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use meta_language::{
    AccessMode, DoubletsLinkStore, EngineLinkStore, Link, LinkMetadata, LinkNetwork, LinkStore,
    LinkStoreQuery, LinkType, ParseConfiguration, StorageError, LANGUAGE_FIXTURES,
};

fn temp_store_path(name: &str) -> PathBuf {
    let sanitized = name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .collect::<String>();
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time is after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "meta-language-{sanitized}-{}-{nonce}.doublets",
        std::process::id()
    ))
}

fn remove_store_files(path: PathBuf) {
    let _ = fs::remove_file(DoubletsLinkStore::snapshot_path(&path));
    let _ = fs::remove_file(path);
}

fn assert_isomorphic(original: &LinkNetwork, restored: &LinkNetwork) {
    let original_links: Vec<&Link> = original.links().collect();
    let restored_links: Vec<&Link> = restored.links().collect();
    assert_eq!(original_links, restored_links);

    let mut terms: BTreeSet<&str> = BTreeSet::new();
    for link in original.links() {
        if let Some(term) = link.metadata().term() {
            terms.insert(term);
        }
    }
    for term in terms {
        assert_eq!(original.find_term(term), restored.find_term(term));
    }
}

#[test]
fn doublets_store_round_trips_language_fixtures_through_file_mapped_storage() {
    for fixture in LANGUAGE_FIXTURES {
        let network = LinkNetwork::parse(
            fixture.source(),
            fixture.language(),
            ParseConfiguration::default(),
        );
        let path = temp_store_path(fixture.language());

        {
            let mut store =
                DoubletsLinkStore::create_file(&path).expect("create file-mapped doublets store");
            store
                .replace_with_network(&network)
                .expect("write network to doublets");
        }

        let reopened = DoubletsLinkStore::open_file(&path).expect("reopen doublets store");
        let restored = reopened.to_network().expect("read network from doublets");
        assert_isomorphic(&network, &restored);
        assert_eq!(restored.to_lino(), network.to_lino());

        remove_store_files(path);
    }
}

#[test]
fn lino_text_and_doublets_binary_storage_are_equivalent() {
    let source = "const answer = 42;\n";
    let network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default());
    let lino = network.to_lino();
    let from_lino = LinkNetwork::from_lino(&lino).expect("LiNo restores network");
    let path = temp_store_path("lino-equivalence");

    {
        let mut store =
            DoubletsLinkStore::create_file(&path).expect("create file-mapped doublets store");
        store
            .replace_with_network(&from_lino)
            .expect("write LiNo-restored network to doublets");
    }

    let restored = DoubletsLinkStore::open_file(&path)
        .expect("reopen doublets store")
        .to_network()
        .expect("restore doublets network");
    assert_eq!(restored.to_lino(), lino);
    assert_eq!(restored.reconstruct_text(), source);

    remove_store_files(path);
}

#[test]
fn doublets_link_store_supports_crud_search_and_read_only_access() {
    let path = temp_store_path("crud");
    let mut store = DoubletsLinkStore::create_file(&path).expect("create doublets store");

    let concept = LinkStore::create(
        &mut store,
        &[],
        LinkMetadata::new()
            .with_link_type(LinkType::Concept)
            .with_term("concept"),
    )
    .expect("create concept");
    let relation = LinkStore::create(
        &mut store,
        &[concept],
        LinkMetadata::new().with_link_type(LinkType::Relation),
    )
    .expect("create relation");

    let matches = LinkStore::search(
        &store,
        &LinkStoreQuery::new()
            .with_link_type(LinkType::Relation)
            .with_references([concept]),
    )
    .expect("search relation");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].id(), relation);

    assert!(LinkStore::update(
        &mut store,
        relation,
        &[],
        LinkMetadata::new()
            .with_link_type(LinkType::Concept)
            .with_term("updated"),
    )
    .expect("update relation"));
    assert_eq!(
        LinkStore::read(&store, relation)
            .expect("read relation")
            .expect("relation exists")
            .metadata()
            .term(),
        Some("updated")
    );

    let mut read_only = EngineLinkStore::with_access_mode(store, AccessMode::ReadOnly);
    let error = LinkStore::delete(&mut read_only, relation)
        .expect_err("read-only doublets store rejects delete");
    assert!(matches!(error, StorageError::ReadOnly(_)));

    drop(read_only);
    remove_store_files(path);
}
