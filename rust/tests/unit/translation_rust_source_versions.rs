//! Deferred explicit-version source, provenance, codec and native replay controls.
#[path = "../../src/translation/source_version_column_rules.rs"]
mod columns;
#[path = "../../src/translation/rust_primitive_try_from_rules.rs"]
mod conversions;
#[path = "../../src/translation/rust_source_version_rules.rs"]
mod versions;
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    emit_typed_program_links, encode_link_network_binary,
};
use serde_json::{Value, json};
use std::{fs, process::Command};
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
fn number(args: &Value, index: usize) -> f64 {
    args[index].as_f64().unwrap()
}
#[test]
fn selected_source_std_survives_both_transports_and_replays_on_the_current_target() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-source-versions.lino"
    ))
    .unwrap();
    for row in fixture["premises"].as_array().unwrap() {
        let args = &row["args"];
        let actual = match row["operation"].as_str().unwrap() {
            "readSourceVersionColumns" => {
                let actual = columns::read_source_version_columns(
                    &columns::ml_string::Text::from_text(text(args, 0)),
                );
                let expected = row["expected"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_f64().unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(actual, expected, "{row}");
                continue;
            }
            "readRustSourceVersionTextLimit" => {
                assert_eq!(
                    versions::read_rust_source_version_text_limit(),
                    row["expected"].as_f64().unwrap(),
                    "{row}"
                );
                continue;
            }
            "readRustSourceVersionDefault" => json!(versions::read_rust_source_version_default()),
            "readRustSourceVersionError" => json!(versions::read_rust_source_version_error(
                number(args, 0),
                number(args, 1),
                number(args, 2),
                args[3].as_bool().unwrap()
            )),
            "readRustIntegerConversionErrorDebugModel" => json!(
                versions::read_rust_integer_conversion_error_debug_model(number(args, 0))
            ),
            "readRustPrimitiveTryFromVersionedFailureDebug" => json!(
                conversions::read_rust_primitive_try_from_versioned_failure_debug(
                    text(args, 0),
                    number(args, 1)
                )
            ),
            unknown => panic!("uncovered source version decision {unknown}"),
        };
        assert_eq!(actual, row["expected"], "{row}");
    }
    let directory = TemporaryDirectory::new("explicit-source-versions");
    for version in fixture["sourceVersions"].as_array().unwrap() {
        let version = version.as_str().unwrap();
        let packet = meta_language::create_rust_source_owner_program_links_with_source_version(
            fixture["source"].as_str().unwrap(),
            version,
        )
        .unwrap();
        let plain = meta_language::create_typed_program_links_with_rust_source_version(
            fixture["source"].as_str().unwrap(),
            version,
            None,
        )
        .unwrap();
        for actual in [&packet, &plain] {
            let before = decode_typed_program_links(&actual.network, actual.root).unwrap();
            for network in [
                LinkNetwork::from_lino(&actual.network.to_lino()).unwrap(),
                decode_link_network_binary(
                    &encode_link_network_binary(&actual.network, BinaryLinksOptions::default())
                        .unwrap(),
                    BinaryLinksOptions::default(),
                )
                .unwrap(),
            ] {
                assert_eq!(
                    decode_typed_program_links(&network, actual.root).unwrap(),
                    before
                );
                assert!(
                    network.to_lino().contains(version),
                    "the selected source context stays in provenance"
                );
                for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
                    assert_eq!(
                        emit_typed_program_links(&network, actual.root, target).unwrap(),
                        emit_typed_program_links(&actual.network, actual.root, target).unwrap()
                    );
                }
            }
        }
        let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
        let emitted = meta_language::translation::emit_rust::emit_rust(&program).unwrap();
        let header = emitted
            .text
            .lines()
            .find(|line| line.starts_with("#![allow("))
            .expect("actual complete library attributes");
        let mut driver = String::from("fn main(){std::panic::set_hook(Box::new(|_|{}));");
        let mut expected = Vec::new();
        for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
            let source = format!("{}.result", row["module"].as_str().unwrap());
            let target = &emitted
                .mappings
                .iter()
                .find(|mapping| mapping.kind == "function" && mapping.source == source)
                .unwrap()
                .target;
            driver.push_str(&format!("{{match std::panic::catch_unwind(||{target}()){{Ok(value)=>println!(\"{index}:value:{{}}\",value),Err(error)=>{{let message=if let Some(value)=error.downcast_ref::<String>(){{value.as_str()}}else if let Some(value)=error.downcast_ref::<&str>(){{*value}}else{{panic!(\"unrepresented panic payload\")}};let bytes=message.as_bytes().iter().map(|byte|format!(\"{{:02x}}\",byte)).collect::<String>();println!(\"{index}:abort:{{}}\",bytes);}}}}}}"));
            let result = &row["expectedByVersion"][version];
            let kind = result["kind"].as_str().unwrap();
            let value = if kind == "abort" {
                result["message"]
                    .as_str()
                    .unwrap()
                    .as_bytes()
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect::<String>()
            } else {
                result["value"].as_str().unwrap().to_owned()
            };
            expected.push(format!("{index}:{kind}:{value}"));
        }
        driver.push('}');
        let source = [
            header.to_owned(),
            emitted.preludes.join("\n"),
            emitted.definitions.join("\n"),
            driver,
        ]
        .join("\n\n");
        let file = directory.0.join(format!("selected_{version}.rs"));
        let binary = directory.0.join(format!("selected_{version}"));
        fs::write(&file, source).unwrap();
        let compiled = run(Command::new("rustc")
            .arg("--edition=2024")
            .args([
                "--crate-name",
                "selected_source_version",
                "-C",
                "overflow-checks=on",
            ])
            .arg(&file)
            .arg("-o")
            .arg(&binary));
        assert!(compiled.ok, "{}", compiled.stderr);
        let observed = run(&mut Command::new(&binary));
        assert!(observed.ok, "{}", observed.stderr);
        assert_eq!(
            observed
                .stdout
                .lines()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            expected
        );
    }
    for version in fixture["invalidVersions"].as_array().unwrap() {
        let error = meta_language::translation::rust::parse_rust_with_source_version(
            "fn main(){}",
            version.as_str().unwrap(),
        )
        .unwrap_err();
        assert_eq!(error.reason, fixture["versionError"].as_str().unwrap());
    }
}
