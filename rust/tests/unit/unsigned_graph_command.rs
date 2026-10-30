//! Deferred saved graph policies, actual process products and dynamic metadata.
#[path = "../../src/unsigned_graph_command_rules.rs"]
mod rules;
use meta_language::{
    BinaryLinksOptions, LinkId, LinkMetadata, LinkNetwork, read_structured_data_lino,
    run_unsigned_graph_command,
};
use serde_json::Value;
use std::cell::Cell;
use std::process::Command;
fn data() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/unsigned-graph-command.lino"
    ))
    .unwrap()
}
fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect()
}
fn texts(value: &Value) -> Vec<rules::ml_string::Text> {
    strings(value)
        .iter()
        .map(|value| rules::ml_string::Text::from_text(value))
        .collect()
}
fn text(value: &str) -> rules::ml_string::Text {
    rules::ml_string::Text::from_text(value)
}
#[test]
fn shared_exact_graph_policy_retains_all_declared_root_surface_bound_and_command_rows() {
    super::unsigned_source_documents::require_versions();
    let data = data();
    for row in data["roots"].as_array().unwrap() {
        let output = rules::plan_unsigned_typed_graph_root(
            &texts(&row["ids"]),
            &texts(&row["types"]),
            &texts(&row["languages"]),
            &texts(&row["terms"]),
            &texts(&row["definitions"]),
            &text(row["requested"].as_str().unwrap()),
            row["limit"].as_f64().unwrap(),
        );
        assert_eq!(
            output.iter().map(ToString::to_string).collect::<Vec<_>>(),
            strings(&row["expected"]),
            "{}",
            row["name"]
        );
    }
    for row in data["bounds"].as_array().unwrap() {
        let args = row["args"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            rules::read_unsigned_graph_read_bounds_error(args[0], args[1], args[2], args[3])
                .to_string(),
            row["expected"].as_str().unwrap()
        );
    }
    for row in data["surface"].as_array().unwrap() {
        assert_eq!(
            rules::read_unsigned_graph_text_surface_error(&text(row["value"].as_str().unwrap()))
                .to_string(),
            row["expected"].as_str().unwrap()
        );
        if row["expected"] != "" {
            assert!(
                meta_language::read_unsigned_graph_snapshot_text(
                    row["value"].as_str().unwrap(),
                    BinaryLinksOptions::default()
                )
                .is_err()
            );
        }
    }
    for row in data["commands"].as_array().unwrap() {
        let args = strings(&row["args"]);
        let output = rules::plan_unsigned_graph_command(&texts(&row["args"]));
        let expected = strings(&row["expected"]);
        assert_eq!(
            output.iter().map(ToString::to_string).collect::<Vec<_>>(),
            expected
        );
        if expected.len() == 1 {
            let reads = Cell::new(0);
            let result = run_unsigned_graph_command(
                &args,
                |_| {
                    reads.set(reads.get() + 1);
                    Err("must not read".to_owned())
                },
                |_| {
                    reads.set(reads.get() + 1);
                    Err("must not read".to_owned())
                },
            );
            assert_eq!(reads.get(), 0);
            assert_eq!(result.exit_code, if expected[0] == "help" { 0 } else { 2 });
            let usage = rules::read_unsigned_graph_command_usage().to_string();
            assert_eq!(
                result.stdout,
                if expected[0] == "help" {
                    usage.as_bytes()
                } else {
                    &[]
                }
            );
            assert_eq!(
                result.stderr,
                if expected[0] == "help" {
                    String::new()
                } else {
                    format!("error: invalid exact graph arguments\n{usage}")
                }
            );
        }
    }
}
#[test]
fn native_saved_graph_transport_matches_actual_javascript_snapshots_and_edits() {
    super::unsigned_source_documents::require_versions();
    let fixture = data();
    let product = super::issue_195_translation_pairs::run(
        Command::new("node")
            .arg("--max-old-space-size=256")
            .arg("../experiments/unsigned-graph/produce.mjs"),
    );
    assert!(product.ok, "{}", product.stderr);
    let observed = read_structured_data_lino(&product.stdout).unwrap();
    assert_eq!(
        observed["io"].as_array().unwrap().len(),
        fixture["io"].as_array().unwrap().len()
    );
    for (row, actual) in fixture["io"]
        .as_array()
        .unwrap()
        .iter()
        .zip(observed["io"].as_array().unwrap())
    {
        assert_eq!(row["name"], actual["name"]);
        let args = strings(&actual["args"]);
        let input_text = actual["inputText"].as_str().unwrap();
        let input_bytes = actual["inputBytes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| u8::try_from(value.as_u64().unwrap()).unwrap())
            .collect::<Vec<_>>();
        let reads = Cell::new(0);
        let byte_reads = Cell::new(0);
        let result = run_unsigned_graph_command(
            &args,
            |path| {
                reads.set(reads.get() + 1);
                assert_eq!(path, "saved.graph");
                Ok(input_text.to_owned())
            },
            |path| {
                byte_reads.set(byte_reads.get() + 1);
                assert_eq!(path, "saved.graph");
                Ok(input_bytes.clone())
            },
        );
        assert_eq!(
            u64::try_from(reads.get()).unwrap(),
            actual["reads"].as_u64().unwrap()
        );
        assert_eq!(
            u64::try_from(byte_reads.get()).unwrap(),
            actual["byteReads"].as_u64().unwrap()
        );
        assert_eq!(reads.get() + byte_reads.get(), 1);
        assert_eq!(
            i64::from(result.exit_code),
            row["exitCode"].as_i64().unwrap()
        );
        assert_eq!(
            i64::from(result.exit_code),
            actual["exitCode"].as_i64().unwrap()
        );
        assert_eq!(result.stderr, actual["stderr"].as_str().unwrap());
        if args.iter().any(|arg| arg == "--binary") && result.exit_code == 0 {
            assert_eq!(
                result.stdout,
                actual["bytes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|byte| u8::try_from(byte.as_u64().unwrap()).unwrap())
                    .collect::<Vec<_>>()
            );
        } else {
            assert_eq!(result.stdout, actual["stdout"].as_str().unwrap().as_bytes());
        }
        if row["exitCode"] != 0 {
            assert!(result.stdout.is_empty());
            assert_eq!(
                result.stderr,
                format!("error: {}\n", row["error"].as_str().unwrap())
            );
        }
    }
}
#[test]
fn metadata_edits_preserve_original_graph_state_and_missing_identity_is_atomic() {
    super::unsigned_source_documents::require_versions();
    let options = BinaryLinksOptions::default();
    let packet = meta_language::create_unsigned_typed_program_links(
        "pub fn result()->u8{7}",
        "Rust",
        LinkId::from_u64(9_007_199_254_740_993),
        options,
    )
    .unwrap();
    let before = packet.network.clone();
    let mut edited = packet.network.clone();
    let scalar = meta_language::typed_program_links_schema()["scalars"]["string"]
        .as_str()
        .unwrap();
    let values = edited
        .links()
        .filter(|link| {
            link.metadata().term() == Some(scalar) && link.metadata().definition() == Some("7")
        })
        .map(meta_language::Link::id)
        .collect::<Vec<_>>();
    assert_eq!(values.len(), 1);
    let value = edited.link(values[0]).unwrap();
    let references = value.references().to_vec();
    let metadata = value.metadata().clone().with_definition("9");
    assert!(edited.set_link_metadata(values[0], metadata));
    assert_eq!(edited.link(values[0]).unwrap().references(), references);
    assert_eq!(
        edited.link(values[0]).unwrap().metadata().definition(),
        Some("9")
    );
    assert_eq!(packet.network, before);
    let snapshot = meta_language::encode_unsigned_link_network_binary(&edited, options).unwrap();
    assert!(!edited.set_link_metadata(
        LinkId::from_u64(u64::MAX),
        LinkMetadata::new().with_term("unused")
    ));
    assert_eq!(
        meta_language::encode_unsigned_link_network_binary(&edited, options).unwrap(),
        snapshot
    );
    assert_eq!(
        meta_language::select_unsigned_typed_program_root(
            &edited,
            Some(LinkId::from_u64(0)),
            options.record_limit
        )
        .unwrap_err()
        .message,
        "a typed graph root needs an exact positive unsigned identity"
    );
    let root =
        meta_language::select_unsigned_typed_program_root(&edited, None, options.record_limit)
            .unwrap();
    assert_eq!(root, packet.root);
    let emitted =
        meta_language::emit_unsigned_typed_program_links(&edited, root, "JavaScript", options)
            .unwrap();
    assert_ne!(
        emitted,
        meta_language::emit_unsigned_typed_program_links(
            &packet.network,
            root,
            "JavaScript",
            options
        )
        .unwrap()
    );
    let link = edited.link(root).unwrap();
    let references = link.references().to_vec();
    let metadata = link.metadata().clone();
    let second = edited.insert_dynamic_link(&references, metadata);
    assert!(
        meta_language::select_unsigned_typed_program_root(&edited, None, options.record_limit)
            .is_err()
    );
    assert_eq!(
        meta_language::select_unsigned_typed_program_root(
            &edited,
            Some(second),
            options.record_limit
        )
        .unwrap(),
        second
    );
    assert_eq!(
        meta_language::emit_unsigned_typed_program_links(&edited, second, "JavaScript", options)
            .unwrap(),
        meta_language::emit_unsigned_typed_program_links(&edited, root, "JavaScript", options)
            .unwrap()
    );
}
#[test]
fn saved_graph_source_definition_retains_both_codecs_and_four_targets() {
    super::unsigned_source_documents::require_versions();
    use meta_language::{
        create_source_definition_program_links, decode_link_network_binary,
        emit_typed_program_links, encode_link_network_binary,
    };
    let packet = create_source_definition_program_links(
        "meta-language/source-program/unsigned-graph-command-program",
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
