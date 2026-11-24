//! Versioned fresh scalar vector result admission and source-free replay.

use std::fs;
use std::process::Command;

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::rust::{parse_rust, parse_rust_with_array_result_model};
use meta_language::{
    LinkId, LinkNetwork, LinkType, create_typed_program_links,
    create_typed_program_links_with_rust_array_result_model, decode_typed_program_links,
    emit_typed_program_links,
};
use serde_json::Value;

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-array-results.lino"
    ))
    .expect("the shared fresh scalar vector fixture is valid Links data")
}

fn field(network: &LinkNetwork, parent: LinkId, label: &str) -> LinkId {
    network
        .links()
        .find_map(|link| match link.references() {
            [owner, name, child]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && *owner == parent
                    && network.link(*name).and_then(|name| name.metadata().term())
                        == Some(label) =>
            {
                Some(*child)
            }
            _ => None,
        })
        .expect("the selected source profile retains its structured field")
}

#[test]
fn fresh_results_retain_checked_surface_source_profile_and_actual_definition_replay() {
    let fixture = fixture();
    let model = fixture["arrayResultModel"].as_str().unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let source = case["source"].as_str().unwrap();
        let surface = parse_rust_with_array_result_model(source, model).unwrap();
        assert_eq!(
            serde_json::to_value(&surface).unwrap(),
            case["surface"],
            "{id}"
        );
        let program = check_program(&surface).unwrap();
        assert_eq!(
            emit_javascript(&program).unwrap().text,
            case["javascript"].as_str().unwrap(),
            "{id}"
        );
        let definitions = emit_rust(&program).unwrap().definitions.join("\n\n");
        assert_eq!(
            definitions,
            case["rustDefinitions"].as_str().unwrap(),
            "{id}"
        );
        let replay = check_program(
            &parse_rust_with_array_result_model(
                &format!("{definitions}\n{}", case["main"].as_str().unwrap()),
                model,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            emit_rust(&replay).unwrap().definitions.join("\n\n"),
            definitions,
            "{id}"
        );
        assert_eq!(
            emit_javascript(&replay).unwrap().text,
            case["replayJavaScript"].as_str().unwrap(),
            "{id}"
        );

        let user_provenance = serde_json::json!({ "project": id, "premises": fixture["premises"] });
        let archive = create_typed_program_links_with_rust_array_result_model(
            source,
            "Rust",
            model,
            Some(&user_provenance),
        )
        .unwrap();
        let network = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
        let provenance = field(&network, archive.root, "provenance");
        let profile = field(&network, provenance, "source profile");
        let selected_model = field(&network, profile, "rust array result model");
        assert_eq!(
            network
                .link(selected_model)
                .unwrap()
                .metadata()
                .definition(),
            Some(model),
            "{id}"
        );
        let user = field(&network, provenance, "user provenance");
        assert_eq!(
            network
                .link(field(&network, user, "project"))
                .unwrap()
                .metadata()
                .definition(),
            Some(id),
            "{id}"
        );
        assert!(
            network.links().all(|link| !link
                .metadata()
                .definition()
                .is_some_and(|definition| definition.contains(source))),
            "{id}"
        );
        assert_eq!(
            emit_typed_program_links(&network, archive.root, "JavaScript")
                .unwrap()
                .code,
            case["javascript"].as_str().unwrap(),
            "{id}"
        );
        assert!(
            emit_typed_program_links(&network, archive.root, "Rust")
                .unwrap()
                .code
                .contains(&definitions),
            "{id}"
        );
    }
}

#[test]
fn number_and_boolean_algorithms_return_from_actual_javascript_definitions_to_rust() {
    let fixture = fixture();
    let model = fixture["arrayResultModel"].as_str().unwrap();
    for id in fixture["bidirectionalCases"].as_array().unwrap() {
        let case = fixture["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["id"] == *id)
            .unwrap();
        let archive = create_typed_program_links(
            case["javascriptDefinitions"].as_str().unwrap(),
            "JavaScript",
        )
        .unwrap();
        let network = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
        let program = decode_typed_program_links(&network, archive.root).unwrap();
        let definitions = emit_rust(&program).unwrap().definitions.join("\n\n");
        assert_eq!(
            definitions,
            case["rustDefinitions"].as_str().unwrap(),
            "{id}"
        );
        let replay = check_program(
            &parse_rust_with_array_result_model(
                &format!("{definitions}\n{}", case["main"].as_str().unwrap()),
                model,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            emit_javascript(&replay).unwrap().definitions.join("\n\n"),
            case["javascriptDefinitions"].as_str().unwrap(),
            "{id}"
        );
    }
}

#[test]
fn legacy_refusals_and_unproved_fresh_result_premises_remain_explicit() {
    let fixture = fixture();
    let model = fixture["arrayResultModel"].as_str().unwrap();
    let legacy = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-array-source.lino"
    ))
    .unwrap();
    let original = legacy["invalid"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == fixture["legacyRefusal"])
        .unwrap();
    assert_eq!(
        original["source"],
        "fn values()->Vec<f64>{vec![1f64]} fn main(){}"
    );
    super::translation_rust_fresh_vector_results::assert_original_array_result_case(original);
    check_program(
        &parse_rust_with_array_result_model(original["source"].as_str().unwrap(), model).unwrap(),
    )
    .unwrap();
    for case in fixture["invalid"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let error = parse_rust_with_array_result_model(case["source"].as_str().unwrap(), model)
            .and_then(|surface| check_program(&surface))
            .expect_err("unproved ownership remains explicit");
        assert_eq!(error.kind.as_str(), case["kind"].as_str().unwrap(), "{id}");
        assert_eq!(error.reason, case["reason"].as_str().unwrap(), "{id}");
        assert_eq!(
            serde_json::to_value(error.span).unwrap(),
            case["span"],
            "{id}"
        );
    }
    assert!(
        create_typed_program_links_with_rust_array_result_model(
            "fn main(){}",
            "Rust",
            "unknown",
            None
        )
        .unwrap_err()
        .message
        .contains("Rust array result model")
    );
    assert!(
        create_typed_program_links_with_rust_array_result_model(
            "function value(){return 1;}",
            "JavaScript",
            model,
            None
        )
        .unwrap_err()
        .message
        .contains("requires Rust source")
    );
}

// Original, emitted and reparsed definitions have an independent native caller.
// This oracle is executed by CI; local work runs only the paired JavaScript.
#[test]
fn native_constructor_results_match_all_bounded_shared_inputs() {
    let directory = TemporaryDirectory::new("rust-array-results");
    for (index, case) in fixture()["cases"].as_array().unwrap().iter().enumerate() {
        let main = case["main"].as_str().unwrap();
        let suffix = format!(" {main}");
        let original = case["source"]
            .as_str()
            .unwrap()
            .strip_suffix(suffix.as_str())
            .unwrap();
        let definitions = case["rustDefinitions"].as_str().unwrap();
        let model = "immutable fresh scalar vector results 1";
        let replay = check_program(
            &parse_rust_with_array_result_model(&format!("{definitions}\nfn main(){{}}"), model)
                .unwrap(),
        )
        .unwrap();
        let replayed = emit_rust(&replay).unwrap().definitions.join("\n\n");
        for (variant, definition) in [
            ("original", original),
            ("generated", definitions),
            ("replayed", replayed.as_str()),
        ] {
            let source = directory
                .0
                .join(format!("array_result_{index}_{variant}.rs"));
            let binary = directory.0.join(format!(
                "array_result_{index}_{variant}{}",
                std::env::consts::EXE_SUFFIX
            ));
            fs::write(
                &source,
                format!(
                    "{definition}\nfn main(){{{}}}",
                    case["nativeCalls"].as_str().unwrap()
                ),
            )
            .unwrap();
            let mut build_command = Command::new("rustc");
            build_command.args(["--edition", "2024"]);
            #[cfg(windows)]
            if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
                build_command.arg("-C").arg(format!("linker={linker}"));
            }
            let compiled = run(build_command.arg(&source).arg("-o").arg(&binary));
            assert!(compiled.ok, "{index}: {variant}: {}", compiled.stderr);
            let observed = run(&mut Command::new(binary));
            assert!(observed.ok, "{index}: {variant}: {}", observed.stderr);
        }
    }
}
