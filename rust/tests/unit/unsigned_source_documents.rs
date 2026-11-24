//! Deferred exact source coordinates and actual JavaScript/native products.
#[path = "../../src/unsigned_source_document_rules.rs"]
mod rules;
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, NetworkProjection, ParseConfiguration,
    create_dense_unsigned_link_network, create_unsigned_source_document_links,
    create_unsigned_source_document_links_with_registry, decode_unsigned_link_network_binary,
    encode_unsigned_link_network_binary, read_structured_data_lino, rebase_unsigned_link_network,
};
use serde_json::Value;
use std::process::Command;
fn data() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/unsigned-source-documents.lino"
    ))
    .unwrap()
}
pub(super) fn require_versions() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let version = super::issue_195_translation_pairs::run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let fields = version
        .stdout
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split('.')
        .collect::<Vec<_>>();
    assert!(
        fields[0] == "1" && (90..100).contains(&fields[1].parse::<u32>().unwrap()),
        "uncovered Rust {}; require >=1.90.0 <1.100.0",
        version.stdout
    );
}
fn numbers(value: &Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_f64().unwrap())
        .collect()
}
fn ids(value: &Value) -> Vec<LinkId> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| LinkId::from_u64(value.as_str().unwrap().parse().unwrap()))
        .collect()
}
#[test]
fn shared_exact_source_allocation_retains_each_declared_boundary() {
    require_versions();
    let data = data();
    for row in data["allocations"].as_array().unwrap() {
        let first = numbers(&row["first"]);
        let count = row["count"].as_f64().unwrap();
        let limit = row["limit"].as_f64().unwrap();
        assert_eq!(
            rules::read_unsigned_source_allocation_error(&first, count, limit),
            ""
        );
        assert_eq!(
            rules::allocate_unsigned_source_identities(&first, count, limit),
            numbers(&row["expected"])
        );
    }
    for row in data["refusals"].as_array().unwrap() {
        let first = numbers(&row["first"]);
        let count = row["count"].as_f64().unwrap();
        let limit = row["limit"].as_f64().unwrap();
        assert_eq!(
            rules::read_unsigned_source_allocation_error(&first, count, limit),
            row["error"].as_str().unwrap()
        );
        assert!(rules::allocate_unsigned_source_identities(&first, count, limit).is_empty());
    }
}
#[test]
fn exact_source_rebase_retains_metadata_references_projections_and_source() {
    require_versions();
    let data = data();
    for row in data["graphs"].as_array().unwrap() {
        let source = LinkNetwork::from_lino(row["source"].as_str().unwrap()).unwrap();
        let before =
            encode_unsigned_link_network_binary(&source, BinaryLinksOptions::default()).unwrap();
        let first = LinkId::from_u64(row["first"].as_str().unwrap().parse().unwrap());
        let packet =
            rebase_unsigned_link_network(&source, first, BinaryLinksOptions::default()).unwrap();
        assert_eq!(packet.source_identities, ids(&row["sourceIdentities"]));
        assert_eq!(packet.identities, ids(&row["identities"]));
        let cursor =
            create_dense_unsigned_link_network(&packet.network, BinaryLinksOptions::default())
                .unwrap()
                .next_identity;
        assert_eq!(
            cursor.map(|id| id.as_u64().to_string()),
            row["next"].as_str().map(str::to_owned)
        );
        let map = packet
            .source_identities
            .iter()
            .zip(&packet.identities)
            .collect::<std::collections::BTreeMap<_, _>>();
        for link in source.links() {
            let next = packet.network.link(*map[&link.id()]).unwrap();
            assert_eq!(next.metadata(), link.metadata());
            assert_eq!(
                next.references(),
                link.references()
                    .iter()
                    .map(|id| *map[id])
                    .collect::<Vec<_>>()
            );
        }
        for projection in [
            NetworkProjection::ConcreteSyntax,
            NetworkProjection::AbstractSyntax,
            NetworkProjection::Semantic,
            NetworkProjection::Lossless,
        ] {
            assert_eq!(
                packet
                    .network
                    .projected_links(projection)
                    .map(meta_language::Link::id)
                    .collect::<Vec<_>>(),
                source
                    .projected_links(projection)
                    .map(|link| *map[&link.id()])
                    .collect::<Vec<_>>()
            );
        }
        assert_eq!(
            encode_unsigned_link_network_binary(&source, BinaryLinksOptions::default()).unwrap(),
            before
        );
        let bytes =
            encode_unsigned_link_network_binary(&packet.network, BinaryLinksOptions::default())
                .unwrap();
        assert_eq!(
            decode_unsigned_link_network_binary(&bytes, BinaryLinksOptions::default())
                .unwrap()
                .to_lino(),
            packet.network.to_lino()
        );
    }
}
#[test]
fn exact_source_documents_match_actual_javascript_graphs_and_binary_snapshots() {
    require_versions();
    let data = data();
    let product = super::issue_195_translation_pairs::run(
        Command::new("node")
            .arg("--max-old-space-size=256")
            .arg("../experiments/unsigned-source-documents/produce.mjs"),
    );
    assert!(product.ok, "{}", product.stderr);
    let observed = read_structured_data_lino(&product.stdout).unwrap();
    assert_eq!(
        observed["documents"].as_array().unwrap().len(),
        data["documents"].as_array().unwrap().len()
    );
    for (row, actual) in data["documents"]
        .as_array()
        .unwrap()
        .iter()
        .zip(observed["documents"].as_array().unwrap())
    {
        assert_eq!(row["name"], actual["name"]);
        let source = row["source"].as_str().unwrap();
        let language = row["language"].as_str().unwrap();
        let original = LinkNetwork::parse(source, language, ParseConfiguration::default());
        let before = original.to_lino();
        let packet = create_unsigned_source_document_links(
            source,
            language,
            ParseConfiguration::default(),
            LinkId::from_u64(9_007_199_254_740_993),
            BinaryLinksOptions::default(),
        )
        .unwrap();
        assert_eq!(packet.source_identities, ids(&actual["sourceIdentities"]));
        assert_eq!(packet.identities, ids(&actual["identities"]));
        assert!(
            packet
                .identities
                .iter()
                .all(|id| id.as_u64() > 9_007_199_254_740_991)
        );
        assert_eq!(packet.network.to_lino(), actual["text"].as_str().unwrap());
        let bytes =
            encode_unsigned_link_network_binary(&packet.network, BinaryLinksOptions::default())
                .unwrap();
        assert_eq!(
            bytes,
            actual["bytes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|byte| u8::try_from(byte.as_u64().unwrap()).unwrap())
                .collect::<Vec<_>>()
        );
        for loaded in [
            packet.network.clone(),
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            decode_unsigned_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap(),
        ] {
            assert_eq!(loaded.reconstruct_text(), source);
        }
        assert_eq!(original.to_lino(), before);
    }
}
#[test]
fn exact_source_rebase_refuses_without_mutating_and_retains_byte_documents() {
    require_versions();
    let graph = LinkNetwork::from_lino("(1: 2)\n(2: 1)").unwrap();
    let before =
        encode_unsigned_link_network_binary(&graph, BinaryLinksOptions::default()).unwrap();
    assert!(
        rebase_unsigned_link_network(
            &graph,
            LinkId::from_u64(u64::MAX),
            BinaryLinksOptions::default()
        )
        .is_err()
    );
    assert!(
        rebase_unsigned_link_network(
            &graph,
            LinkId::from_u64(1),
            BinaryLinksOptions {
                record_limit: 1,
                ..BinaryLinksOptions::default()
            }
        )
        .is_err()
    );
    assert_eq!(
        encode_unsigned_link_network_binary(&graph, BinaryLinksOptions::default()).unwrap(),
        before
    );
    let bytes = [0, 255, 128, 13, 10];
    let graph = LinkNetwork::parse_bytes(&bytes, "opaque bytes");
    let packet = rebase_unsigned_link_network(
        &graph,
        LinkId::from_u64(9_007_199_254_740_993),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    assert_eq!(packet.network.reconstruct_bytes(), bytes);
    let loaded = decode_unsigned_link_network_binary(
        &encode_unsigned_link_network_binary(&packet.network, BinaryLinksOptions::default())
            .unwrap(),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    assert_eq!(loaded.reconstruct_bytes(), bytes);
}
#[test]
fn registered_source_parsers_keep_dispatch_before_unsigned_allocation() {
    require_versions();
    #[derive(Debug)]
    struct Parser;
    impl meta_language::LanguageParser for Parser {
        fn parse_source(
            &self,
            text: &str,
            language: &str,
            configuration: ParseConfiguration,
        ) -> LinkNetwork {
            LinkNetwork::parse_lossless_text(&text.to_uppercase(), language, configuration)
        }
    }
    let registry = meta_language::ParserRegistry::new()
        .with_parser("private source", std::sync::Arc::new(Parser));
    let packet = create_unsigned_source_document_links_with_registry(
        &registry,
        "custom source",
        "private source",
        ParseConfiguration::default(),
        LinkId::from_u64(9_007_199_254_740_993),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    assert_eq!(packet.network.reconstruct_text(), "CUSTOM SOURCE");
}

#[test]
fn exact_source_allocation_definition_retains_both_codecs_and_four_targets() {
    require_versions();
    use meta_language::{
        create_source_definition_program_links, decode_link_network_binary,
        emit_typed_program_links, encode_link_network_binary,
    };
    let packet = create_source_definition_program_links(
        "meta-language/source-program/unsigned-source-document-program",
    )
    .unwrap();
    let text = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    let binary = decode_link_network_binary(
        &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
        let expected = emit_typed_program_links(&packet.network, packet.root, target).unwrap();
        for graph in [&text, &binary] {
            assert_eq!(
                emit_typed_program_links(graph, packet.root, target).unwrap(),
                expected
            );
        }
    }
}

#[test]
fn exact_typed_programs_retain_actual_roots_and_all_target_emissions() {
    require_versions();
    let data = data();
    let product = super::issue_195_translation_pairs::run(
        Command::new("node")
            .arg("--max-old-space-size=256")
            .arg("../experiments/unsigned-source-documents/produce.mjs"),
    );
    assert!(product.ok, "{}", product.stderr);
    let observed = read_structured_data_lino(&product.stdout).unwrap();
    assert_eq!(
        observed["programs"].as_array().unwrap().len(),
        data["programs"].as_array().unwrap().len()
    );
    for (row, actual) in data["programs"]
        .as_array()
        .unwrap()
        .iter()
        .zip(observed["programs"].as_array().unwrap())
    {
        assert_eq!(row["name"], actual["name"]);
        let source = row["source"].as_str().unwrap();
        let language = row["language"].as_str().unwrap();
        let first = LinkId::from_u64(9_007_199_254_740_993);
        let limits = BinaryLinksOptions::default();
        let original = if row["factory"] == "Rust source owners" {
            meta_language::create_rust_source_owner_program_links(source)
        } else {
            meta_language::create_typed_program_links(source, language)
        }
        .unwrap();
        let before = original.network.to_lino();
        let packet = if row["factory"] == "Rust source owners" {
            meta_language::create_unsigned_rust_source_owner_program_links(source, first, limits)
        } else {
            meta_language::create_unsigned_typed_program_links(source, language, first, limits)
        }
        .unwrap();
        assert_eq!(
            packet.root.as_u64().to_string(),
            actual["root"].as_str().unwrap()
        );
        assert_eq!(packet.source_identities, ids(&actual["sourceIdentities"]));
        assert_eq!(packet.identities, ids(&actual["identities"]));
        assert_eq!(packet.network.to_lino(), actual["text"].as_str().unwrap());
        let bytes = encode_unsigned_link_network_binary(&packet.network, limits).unwrap();
        assert_eq!(
            bytes,
            actual["bytes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|byte| u8::try_from(byte.as_u64().unwrap()).unwrap())
                .collect::<Vec<_>>()
        );
        for loaded in [
            packet.network.clone(),
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            decode_unsigned_link_network_binary(&bytes, limits).unwrap(),
        ] {
            for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
                let emitted = meta_language::emit_unsigned_typed_program_links(
                    &loaded,
                    packet.root,
                    target,
                    limits,
                )
                .unwrap();
                assert_eq!(
                    emitted,
                    meta_language::emit_typed_program_links(
                        &original.network,
                        original.root,
                        target
                    )
                    .unwrap()
                );
                let produced = actual["emissions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|value| value["target"] == target)
                    .unwrap();
                assert_eq!(emitted.code, produced["code"].as_str().unwrap());
            }
        }
        let mut damaged = packet.network.clone();
        damaged.delete_link(packet.root);
        assert!(
            meta_language::emit_unsigned_typed_program_links(
                &damaged,
                packet.root,
                "JavaScript",
                limits
            )
            .is_err()
        );
        assert!(
            meta_language::rebase_unsigned_typed_program_links(
                &meta_language::TypedProgramLinks {
                    network: damaged,
                    root: packet.root
                },
                LinkId::from_u64(1),
                limits
            )
            .is_err()
        );
        assert_eq!(original.network.to_lino(), before);
    }
}
#[test]
fn exact_typed_source_version_and_emission_choices_remain_explicit() {
    require_versions();
    let limits = BinaryLinksOptions::default();
    let source = "pub fn result()->u8{7}";
    let original =
        meta_language::create_rust_source_owner_program_links_with_source_version(source, "1.90.0")
            .unwrap();
    let packet =
        meta_language::create_unsigned_rust_source_owner_program_links_with_source_version(
            source,
            "1.90.0",
            LinkId::from_u64(9_007_199_254_740_993),
            limits,
        )
        .unwrap();
    for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
        assert_eq!(
            meta_language::emit_unsigned_typed_program_links(
                &packet.network,
                packet.root,
                target,
                limits
            )
            .unwrap(),
            meta_language::emit_typed_program_links(&original.network, original.root, target)
                .unwrap()
        );
    }
    let choices = meta_language::TypedProgramLinksEmissionOptions {
        rust_source_header_emission_model: Some("not a model".to_owned()),
        ..meta_language::TypedProgramLinksEmissionOptions::default()
    };
    assert!(
        meta_language::emit_unsigned_typed_program_links_with_options(
            &packet.network,
            packet.root,
            "JavaScript",
            &choices,
            limits
        )
        .is_err()
    );
}
