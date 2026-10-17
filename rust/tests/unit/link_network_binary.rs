use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, encode_link_network_binary,
    read_structured_data_lino,
};
use serde_json::Value;

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/link-network-binary.lino"
    ))
    .expect("shared binary fixture is valid linked data")
}

fn bytes(fixture: &Value, field: &str) -> Vec<u8> {
    fixture[field]
        .as_array()
        .expect("byte array")
        .iter()
        .map(|value| u8::try_from(value.as_u64().expect("unsigned byte")).expect("byte fits u8"))
        .collect()
}

#[test]
fn binary_network_native_bytes_preserve_full_graph_and_observable_state() {
    let fixture = fixture();
    let source = bytes(&fixture, "version2Bytes");
    let restored = decode_link_network_binary(&source, BinaryLinksOptions::default())
        .expect("restore complete network");
    let graph = LinkNetwork::from_lino(fixture["graph"].as_str().expect("graph listing"))
        .expect("independent graph projection");
    assert_eq!(
        restored.links().collect::<Vec<_>>(),
        graph.links().collect::<Vec<_>>()
    );
    assert_eq!(
        restored
            .find_term("original")
            .expect("original exact key")
            .as_u64(),
        1
    );
    assert_eq!(restored.find_term("renamed"), None);
    assert_eq!(
        restored.reconstruct_concept("meaning", "JavaScript"),
        Some("selected syntax")
    );
    assert_eq!(
        restored.reconstruct_concept("😀 concept", "en"),
        Some("astral syntax")
    );
    assert_eq!(
        encode_link_network_binary(&restored, BinaryLinksOptions::default())
            .expect("encode native network"),
        source
    );
    let mut edited = restored;
    assert_eq!(
        edited
            .insert_link([], meta_language::LinkMetadata::new())
            .as_u64(),
        101
    );
    assert_ne!(
        encode_link_network_binary(&edited, BinaryLinksOptions::default())
            .expect("encode edited graph"),
        source
    );
}

#[test]
fn legacy_binary_network_preserves_its_declared_graph_and_registrations() {
    let fixture = fixture();
    let source = bytes(&fixture, "legacyVersion1Bytes");
    let restored = decode_link_network_binary(&source, BinaryLinksOptions::default())
        .expect("restore legacy graph");
    let graph = LinkNetwork::from_lino(fixture["graph"].as_str().expect("graph listing"))
        .expect("graph projection");
    assert_eq!(
        restored.links().collect::<Vec<_>>(),
        graph.links().collect::<Vec<_>>()
    );
    assert_eq!(restored.find_term("original"), None);
    assert_eq!(restored.find_term("renamed"), None);
    assert_eq!(restored.reconstruct_concept("meaning", "JavaScript"), None);
    let output = encode_link_network_binary(&restored, BinaryLinksOptions::default())
        .expect("upgrade archive");
    assert_eq!(&output[..8], b"MLDSNP02");
}

#[test]
fn native_binary_network_refuses_corruption_unsafe_ids_and_resource_excess() {
    let fixture = fixture();
    let source = bytes(&fixture, "version2Bytes");
    assert!(
        decode_link_network_binary(&source[..source.len() - 1], BinaryLinksOptions::default())
            .is_err()
    );
    let mut corrupt = source.clone();
    corrupt[0] = 0;
    assert!(decode_link_network_binary(&corrupt, BinaryLinksOptions::default()).is_err());
    let mut unsafe_identity = source.clone();
    unsafe_identity[8..16].copy_from_slice(&9_007_199_254_740_992_u64.to_le_bytes());
    assert!(
        decode_link_network_binary(&unsafe_identity, BinaryLinksOptions::default())
            .expect_err("unsafe ID")
            .to_string()
            .contains("unsafe")
    );
    assert!(
        decode_link_network_binary(
            &source,
            BinaryLinksOptions {
                record_limit: 1,
                byte_limit: 128 * 1024 * 1024
            }
        )
        .is_err()
    );
    assert!(
        decode_link_network_binary(
            &source,
            BinaryLinksOptions {
                record_limit: 100,
                byte_limit: 32
            }
        )
        .is_err()
    );
}

#[cfg(feature = "doublets")]
#[test]
fn legacy_archive_matches_the_actual_doublets_companion_codec() {
    use meta_language::DoubletsLinkStore;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let fixture = fixture();
    let graph = decode_link_network_binary(
        &bytes(&fixture, "version2Bytes"),
        BinaryLinksOptions::default(),
    )
    .expect("complete graph");
    let path = std::env::temp_dir().join(format!(
        "meta-language-binary-native-{}-{}.doublets",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let snapshot = DoubletsLinkStore::snapshot_path(&path);
    {
        let mut store =
            DoubletsLinkStore::create_file(&path).expect("create native oracle boundary");
        store
            .replace_with_network(&graph)
            .expect("write native legacy snapshot");
    }
    assert_eq!(
        fs::read(&snapshot).expect("native bytes"),
        bytes(&fixture, "legacyVersion1Bytes")
    );
    fs::remove_file(snapshot).expect("remove native snapshot");
    fs::remove_file(path).expect("remove native store");
}
