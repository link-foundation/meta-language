//! Deferred shared command interpretation and actual process transports.
#[path = "../../src/unsigned_source_command_rules.rs"]
mod rules;
use meta_language::{read_structured_data_lino, run_unsigned_source_command};
use std::cell::Cell;
use std::process::Command;
fn data() -> serde_json::Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/unsigned-source-command.lino"
    ))
    .unwrap()
}
fn arguments(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect()
}
#[test]
fn every_declared_command_plan_and_identity_boundary_has_a_native_control() {
    super::unsigned_source_documents::require_versions();
    let data = data();
    for row in data["identities"].as_array().unwrap() {
        assert_eq!(
            rules::accept_unsigned_source_command_identity(&rules::ml_string::Text::from_text(
                row["value"].as_str().unwrap()
            )),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["commands"].as_array().unwrap() {
        let args = arguments(&row["args"]);
        let expected = arguments(&row["expected"]);
        let encoded = args
            .iter()
            .map(|arg| rules::ml_string::Text::from_text(arg))
            .collect::<Vec<_>>();
        assert_eq!(
            rules::plan_unsigned_source_command(&encoded)
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            expected
        );
        if expected.len() == 1 {
            let reads = Cell::new(0);
            let output = run_unsigned_source_command(&args, |_| {
                reads.set(reads.get() + 1);
                Err("must not read".to_owned())
            });
            assert_eq!(reads.get(), 0);
            assert_eq!(output.exit_code, if expected[0] == "help" { 0 } else { 2 });
            let usage = rules::read_unsigned_source_command_usage().to_string();
            assert_eq!(
                output.stdout,
                if expected[0] == "help" {
                    usage.as_bytes()
                } else {
                    &[]
                }
            );
            assert_eq!(
                output.stderr,
                if expected[0] == "help" {
                    String::new()
                } else {
                    format!("error: invalid exact source arguments\n{usage}")
                }
            );
        }
    }
}
#[test]
fn command_outputs_match_actual_javascript_graph_snapshots_and_emissions() {
    super::unsigned_source_documents::require_versions();
    let data = data();
    let product = super::issue_195_translation_pairs::run(
        Command::new("node")
            .arg("--max-old-space-size=256")
            .arg("../experiments/unsigned-source-command/produce.mjs"),
    );
    assert!(product.ok, "{}", product.stderr);
    let actual = read_structured_data_lino(&product.stdout).unwrap();
    assert_eq!(
        actual["io"].as_array().unwrap().len(),
        data["io"].as_array().unwrap().len()
    );
    for (row, observed) in data["io"]
        .as_array()
        .unwrap()
        .iter()
        .zip(actual["io"].as_array().unwrap())
    {
        assert_eq!(row["name"], observed["name"]);
        let args = arguments(&row["args"]);
        let reads = Cell::new(0);
        let output = run_unsigned_source_command(&args, |path| {
            reads.set(reads.get() + 1);
            assert_eq!(path, "input.source");
            Ok(row["source"].as_str().unwrap().to_owned())
        });
        assert_eq!(reads.get(), 1);
        assert_eq!(observed["reads"].as_u64().unwrap(), 1);
        assert_eq!(output.exit_code, 0, "{}", output.stderr);
        assert_eq!(observed["exitCode"].as_i64().unwrap(), 0);
        assert!(output.stderr.is_empty());
        assert_eq!(observed["stderr"], "");
        if row["binary"].as_bool().unwrap() {
            assert_eq!(
                output.stdout,
                observed["bytes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|byte| u8::try_from(byte.as_u64().unwrap()).unwrap())
                    .collect::<Vec<_>>()
            );
        } else {
            assert_eq!(
                output.stdout,
                observed["stdout"].as_str().unwrap().as_bytes()
            );
        }
    }
    let reads = Cell::new(0);
    let output = run_unsigned_source_command(
        &[
            "input.source".to_owned(),
            "--language".to_owned(),
            "Rust".to_owned(),
        ],
        |_| {
            reads.set(reads.get() + 1);
            Err("fixture read failure".to_owned())
        },
    );
    assert_eq!(reads.get(), 1);
    assert_eq!(output.exit_code, 1);
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, "error: fixture read failure\n");
}
#[test]
fn exact_command_source_definition_keeps_both_codecs_and_four_target_emissions() {
    super::unsigned_source_documents::require_versions();
    use meta_language::{
        BinaryLinksOptions, LinkNetwork, create_source_definition_program_links,
        decode_link_network_binary, emit_typed_program_links, encode_link_network_binary,
    };
    let packet = create_source_definition_program_links(
        "meta-language/source-program/unsigned-source-command-program",
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
