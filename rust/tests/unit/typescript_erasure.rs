//! Operational TypeScript erasure and provenance share the JavaScript fixture.
use meta_language::{erase_type_script, self_translate};
use serde_json::{Value, json};

const FIXTURE: &str = include_str!("../../../parity/fixtures/typescript-erasure.lino");

#[test]
fn linked_typescript_erasure_matches_javascript_byte_spans_and_refusals() {
    assert_eq!(
        include_str!("../../src/data/typescript-erasure.lino"),
        include_str!("../../../js/src/data/typescript-erasure.lino")
    );
    let fixture: Value = super::project_data::read_project_data(FIXTURE).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        match erase_type_script(source) {
            Ok(erased) => {
                assert_eq!(
                    erased.code,
                    case["erased"]["code"].as_str().unwrap(),
                    "{}",
                    case["id"]
                );
                let ranges: Vec<_> = erased.erasures.into_iter().map(|erasure| json!({"start":erasure.start,"end":erasure.end,"term":erasure.term})).collect();
                assert_eq!(
                    Value::Array(ranges),
                    case["erased"]["erasures"],
                    "{}",
                    case["id"]
                );
            }
            Err(error) => {
                assert_eq!(
                    error.reason,
                    case["error"]["reason"].as_str().unwrap(),
                    "{}",
                    case["id"]
                );
                let span = error
                    .span
                    .map(|span| json!({"start":span.start,"end":span.end}));
                assert_eq!(span.unwrap_or(Value::Null), case["error"]["span"]);
            }
        }
    }
}

#[test]
fn typescript_translations_restore_from_artifacts_without_original_source() {
    let fixture: Value = super::project_data::read_project_data(FIXTURE).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        let translated = self_translate(source, "TypeScript", "JavaScript").unwrap();
        assert_eq!(
            translated.code,
            case["javascript"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        let replay = self_translate(&translated.code, "JavaScript", "TypeScript").unwrap();
        assert_eq!(replay.code, source, "{}", case["id"]);
        if !case["error"].is_null() {
            assert!(translated.items.iter().any(|item| item.status == "carried"));
        }
    }
}

#[test]
fn edited_erasure_artifacts_do_not_restore_the_original_runtime_body() {
    let source = "function add(a: number, b: number): number { return a + b; }\n";
    let translated = self_translate(source, "TypeScript", "JavaScript").unwrap();
    let at = translated.code.rfind("return a + b;").unwrap();
    let edited = format!(
        "{}{}",
        &translated.code[..at],
        translated.code[at..].replacen("return a + b;", "return a - b;", 1)
    );
    let reversed = self_translate(&edited, "JavaScript", "TypeScript").unwrap();
    assert_ne!(reversed.code, source);
    assert!(reversed.code.contains("return a - b;"));
}

#[test]
fn typed_function_translates_to_the_rust_portable_core() {
    let translated = self_translate(
        "function add(a: number, b: number): number { return a + b; }\n",
        "TypeScript",
        "Rust",
    )
    .unwrap();
    assert!(
        translated
            .items
            .iter()
            .all(|item| item.status == "translated")
    );
    assert!(
        translated
            .code
            .contains("pub fn add(a: f64, b: f64) -> f64")
    );
}

#[test]
fn native_typescript_annotations_preserve_string_and_boolean_types() {
    let fixture: Value = super::project_data::read_project_data(FIXTURE).unwrap();
    for case in fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["rust"].is_string())
    {
        let translated =
            self_translate(case["source"].as_str().unwrap(), "TypeScript", "Rust").unwrap();
        assert_eq!(
            translated.code,
            case["rust"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        assert!(
            translated
                .items
                .iter()
                .all(|item| item.status == "translated")
        );
    }
}

#[test]
fn native_type_script_receivers_match_all_shared_erasure_typing_and_translation_inputs() {
    let fixture: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/typescript-receiver-parameters.lino"
    ))
    .unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        let erased = erase_type_script(source).unwrap();
        assert_eq!(
            erased.code,
            case["erased"]["code"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        let spans: Vec<_> = erased
            .erasures
            .iter()
            .map(|part| json!({"start": part.start, "end": part.end, "term": part.term}))
            .collect();
        assert_eq!(
            Value::Array(spans),
            case["erased"]["erasures"],
            "{}",
            case["id"]
        );
        let typing = meta_language::project_type_script_typing(source);
        if let Some(expected) = case["typing"].as_str() {
            assert_eq!(typing.unwrap(), expected, "{}", case["id"]);
        } else {
            assert_eq!(
                typing.unwrap_err().reason,
                case["typingRefusal"]["reason"].as_str().unwrap()
            );
        }
        let javascript = self_translate(source, "TypeScript", "JavaScript").unwrap();
        assert_eq!(
            javascript.code,
            case["javascript"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        assert_eq!(
            self_translate(&javascript.code, "JavaScript", "TypeScript")
                .unwrap()
                .code,
            source
        );
        if let Some(expected) = case["rust"].as_str() {
            let native = self_translate(source, "TypeScript", "Rust").unwrap();
            assert_eq!(native.code, expected, "{}", case["id"]);
            if case["portable"].as_bool() == Some(true) {
                assert!(
                    native
                        .items
                        .iter()
                        .all(|item| matches!(item.status.as_str(), "translated" | "comment")),
                    "{}",
                    case["id"]
                );
            } else {
                assert!(
                    native.items.iter().any(|item| item.status == "carried"
                        && item.reason.as_deref() == case["portableRefusal"].as_str()),
                    "{}",
                    case["id"]
                );
            }
            assert_eq!(
                self_translate(&native.code, "Rust", "TypeScript")
                    .unwrap()
                    .code,
                source
            );
        }
    }
    for case in fixture["refusals"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        let error = erase_type_script(source).unwrap_err();
        assert_eq!(
            error.reason,
            case["error"]["reason"].as_str().unwrap(),
            "{source}"
        );
        let span = error
            .span
            .map(|span| json!({"start": span.start, "end": span.end}));
        assert_eq!(
            span.unwrap_or(Value::Null),
            case["error"]["span"],
            "{source}"
        );
    }
}

#[test]
fn native_type_script_receiver_outputs_run_against_the_active_bounded_node_oracle() {
    let fixture: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/typescript-receiver-parameters.lino"
    ))
    .unwrap();
    let cases: Vec<_> = fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| {
            let source = case["source"].as_str().unwrap();
            json!({"id": case["id"], "source": source,
            "code": erase_type_script(source).unwrap().code,
            "invocation": case["invocation"], "expected": case["expectedValue"]})
        })
        .collect();
    let script = format!(
        r#"
        import assert from 'node:assert/strict';
        import {{stripTypeScriptTypes}} from 'node:module';
        const [major,minor]=process.versions.node.split('.').map(Number);
        assert.ok(major===22&&minor>=15||major===24,
          'Uncovered Node oracle '+process.versions.node+'; expected >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0');
        const cases={};
        for(const row of cases){{
          const oracle=stripTypeScriptTypes(row.source,{{mode:'strip'}});
          for(const code of [row.code,oracle]){{
            const value=new Function(code+'\nreturn ('+row.invocation+');')();
            assert.deepEqual(value,row.expected,row.id);
          }}
        }}
    "#,
        serde_json::to_string(&cases).unwrap()
    );
    let output = std::process::Command::new("node")
        .args([
            "--max-old-space-size=96",
            "--input-type=module",
            "--eval",
            &script,
        ])
        .output()
        .expect("Node is available in native CI");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn native_type_script_receiver_concrete_trees_retain_all_source_through_both_archives() {
    let fixture: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/typescript-receiver-parameters.lino"
    ))
    .unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        let network = meta_language::LinkNetwork::parse(
            source,
            "TypeScript",
            meta_language::ParseConfiguration::default(),
        );
        assert!(network.verify_full_match(None).is_clean(), "{}", case["id"]);
        let text = network.to_lino();
        let binary = meta_language::encode_link_network_binary(
            &network,
            meta_language::BinaryLinksOptions::default(),
        )
        .unwrap();
        for restored in [
            meta_language::LinkNetwork::from_lino(&text).unwrap(),
            meta_language::decode_link_network_binary(
                &binary,
                meta_language::BinaryLinksOptions::default(),
            )
            .unwrap(),
        ] {
            assert_eq!(restored.to_lino(), text, "{}", case["id"]);
            assert_eq!(restored.reconstruct_text(), source, "{}", case["id"]);
        }
    }
}
