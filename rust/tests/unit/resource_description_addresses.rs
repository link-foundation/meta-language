//! Future normative and paired address controls; native execution belongs to CI.
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_resource_description_graph,
    emit_resource_description_ntriples, encode_link_network_binary,
    import_resource_description_ntriples, read_resource_description_ntriples_source,
    read_structured_data_lino,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::process::Command;

#[test]
fn normative_address_grammar_preserves_semantics_source_and_paired_graphs() {
    let version = Command::new("rustc").arg("--version").output().unwrap();
    assert!(version.status.success());
    let version = String::from_utf8(version.stdout).unwrap();
    let version = version.split_whitespace().nth(1).unwrap();
    let parts = version
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {version}; >=1.90 <1.100"
    );
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/resource-description-addresses.lino"
    ))
    .unwrap();
    let producer = Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/resource-addresses/oracle.mjs",
        ])
        .output()
        .unwrap();
    assert!(
        producer.status.success(),
        "{}",
        String::from_utf8_lossy(&producer.stderr)
    );
    let expected = read_structured_data_lino(&String::from_utf8(producer.stdout).unwrap()).unwrap();
    for (index, value) in fixture["valid"].as_array().unwrap().iter().enumerate() {
        let value = value.as_str().unwrap();
        let source = format!("<{value}> <urn:predicate> \"value\" .\n");
        let original =
            import_resource_description_ntriples(&source, "address-fixture", None).unwrap();
        assert_eq!(
            json!({"value":value,"root":original.root.as_u64(),"syntaxRoot":original.syntax_root.as_u64(),
            "sha256":format!("{:x}",Sha256::digest(original.network.to_lino().as_bytes()))}),
            expected[index]
        );
        let binary =
            encode_link_network_binary(&original.network, BinaryLinksOptions::default()).unwrap();
        for network in [
            original.network.clone(),
            LinkNetwork::from_lino(&original.network.to_lino()).unwrap(),
            decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
        ] {
            assert_eq!(
                read_resource_description_ntriples_source(&network, original.syntax_root).unwrap(),
                source
            );
            assert_eq!(
                decode_resource_description_graph(&network, original.root).unwrap()["triples"][0]
                    [0]["value"],
                value
            );
            assert_eq!(
                emit_resource_description_ntriples(&network, original.root)
                    .unwrap()
                    .source,
                source
            );
        }
    }
    for value in fixture["invalid"].as_array().unwrap() {
        let source = format!(
            "<{}> <urn:predicate> \"value\" .\n",
            value.as_str().unwrap()
        );
        assert!(
            import_resource_description_ntriples(&source, "address-fixture", None).is_err(),
            "{source}"
        );
    }
}
