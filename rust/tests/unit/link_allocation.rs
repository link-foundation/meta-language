//! Future real endpoint controls; no native compilation during drafting.
use super::issue_195_translation_pairs::run;
use meta_language::{
    BinaryLinksOptions, LinkId, LinkMetadata, LinkNetwork, LinkStore, StorageError,
    accept_link_allocation_capacity, create_link_allocation_program_links,
    decode_link_network_binary, emit_typed_program_links, encode_link_network_binary,
    read_structured_data_lino,
};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::process::Command;
use std::sync::OnceLock;

fn versions() {
    static GUARDED: OnceLock<()> = OnceLock::new();
    GUARDED.get_or_init(|| {
        let version = run(Command::new("rustc").arg("--version"));
        assert!(version.ok, "{}", version.stderr);
        let text = version.stdout.split_whitespace().nth(1).unwrap();
        let fields = text.split('.').collect::<Vec<_>>();
        let minor = fields[1].parse::<u32>().unwrap();
        assert!(
            fields[0] == "1" && (90..100).contains(&minor),
            "uncovered Rust {text}; >=1.90 <1.100"
        );
    });
}

fn fixture() -> serde_json::Value {
    versions();
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/link-allocation.lino"
    ))
    .unwrap()
}

fn endpoint(index: usize) -> LinkNetwork {
    LinkNetwork::from_lino(fixture()["endpoints"][index]["source"].as_str().unwrap()).unwrap()
}

fn assert_refused(network: &mut LinkNetwork, operation: impl FnOnce(&mut LinkNetwork)) {
    let before = network.clone();
    let error = catch_unwind(AssertUnwindSafe(|| operation(network)))
        .expect_err("the exhausted operation must refuse");
    let message = error
        .downcast_ref::<&str>()
        .map(|value| (*value).to_owned())
        .or_else(|| error.downcast_ref::<String>().cloned())
        .expect("the refusal retains text");
    assert_eq!(message, "meta-language link identifier space exhausted");
    assert_eq!(
        *network, before,
        "no graph, cache or allocator mutation may precede refusal"
    );
}

#[test]
fn actual_maximum_text_identity_round_trips_and_exhausts_atomically() {
    let maximum = LinkId::from_u64(u64::MAX);
    let mut network = endpoint(0);
    assert_eq!(network.link(maximum).unwrap().references(), &[maximum]);
    assert_eq!(network.insert_point("maximum"), maximum);
    let text = network.to_lino();
    assert_eq!(LinkNetwork::from_lino(&text).unwrap().to_lino(), text);
    assert_refused(&mut network, |network| {
        network.insert_point("new point");
    });
    assert_refused(&mut network, |network| {
        network.insert_dynamic_link(&[maximum], LinkMetadata::new().with_term("new relation"));
    });
    assert_refused(&mut network, |network| {
        network.insert_object_instance(maximum);
    });
    assert_refused(&mut network, |network| {
        network.insert_field(maximum, "new field", maximum);
    });
    assert_refused(&mut network, |network| {
        network.insert_field(maximum, "maximum", maximum);
    });
    let before = network.clone();
    let error = LinkStore::create(&mut network, &[maximum], LinkMetadata::new()).unwrap_err();
    assert!(matches!(error, StorageError::IdentifierSpaceExhaustion));
    assert_eq!(network, before);
}

#[test]
fn final_identity_is_available_after_atomic_two_link_field_refusal() {
    let mut network = endpoint(1);
    let parent = LinkId::from_u64(u64::MAX - 1);
    assert_refused(&mut network, |network| {
        network.insert_field(parent, "new field", parent);
    });
    assert_eq!(
        network.insert_object_instance(parent),
        LinkId::from_u64(u64::MAX)
    );
    assert_eq!(
        network
            .link(LinkId::from_u64(u64::MAX))
            .unwrap()
            .references(),
        &[LinkId::from_u64(u64::MAX), parent]
    );
    assert_refused(&mut network, |network| {
        network.insert_point("after maximum");
    });
}

#[test]
fn two_link_field_can_consume_the_exact_final_two_identities() {
    let mut network = endpoint(2);
    let parent = LinkId::from_u64(u64::MAX - 2);
    let field = network.insert_field(parent, "last field", parent);
    assert_eq!(field, LinkId::from_u64(u64::MAX));
    assert_eq!(
        network.link(field).unwrap().references(),
        &[parent, LinkId::from_u64(u64::MAX - 1), parent]
    );
    assert_refused(&mut network, |network| {
        network.insert_point("after field");
    });
}

#[test]
fn actual_native_capacity_uses_the_shared_source_rule() {
    for row in fixture()["cases"].as_array().unwrap() {
        let words = row["words"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            accept_link_allocation_capacity(
                words[0],
                words[1],
                words[2],
                words[3],
                row["exhausted"].as_bool().unwrap(),
                row["count"].as_f64().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
}

#[test]
fn complete_allocation_definition_retains_paired_graphs_codecs_and_four_emissions() {
    versions();
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/link-allocation/oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let expected = read_structured_data_lino(&producer.stdout).unwrap();
    let packet = create_link_allocation_program_links().unwrap();
    assert_eq!(packet.root.as_u64(), expected["root"].as_u64().unwrap());
    assert_eq!(
        packet.network.to_lino(),
        expected["links"].as_str().unwrap()
    );
    let binary =
        encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
    ] {
        for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
            let value = emit_typed_program_links(&network, packet.root, target).unwrap();
            let observed = serde_json::json!({"code":value.code,"language":value.language,"mappings":value.mappings,"assumptions":value.assumptions,"encodings":value.encodings,"theorems":value.theorems});
            assert_eq!(observed, expected["emissions"][target]);
        }
    }
}
