//! Future CI controls; not executed during the complete source draft phase.
use super::issue_195_observations::{Observation, record};
use meta_language::{
    BinaryLinksOptions, FeatureParseOptions, GrammarScannerModeStackObservation,
    GrammarScannerObservation, LinkNetwork, RegularGrammarInferenceOptions,
    accept_inferred_grammar_symbols, compile_feature_grammar, decode_link_network_binary,
    encode_link_network_binary, infer_grammar_scanner, infer_grammar_scanner_classes,
    infer_grammar_scanner_mode_stack, infer_regular_grammar, parse_grammar_links,
    read_refined_regular_grammar_inference_links, read_regular_grammar_inference_links,
    read_structured_data_lino, refine_regular_grammar_inference, render_grammar_links,
    serialize_grammar,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::process::Command;

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/regular-grammar-inference.lino"
    ))
    .unwrap()
}

#[test]
fn scanner_character_classes_preserve_gaps_literal_precedence_and_archive_refinement() {
    versions();
    let data = fixture();
    let fixture = &data["scannerCharacterClasses"];
    let observations: Vec<GrammarScannerObservation> =
        serde_json::from_value(fixture["observations"].clone()).unwrap();
    let inferred = infer_grammar_scanner_classes(&observations, None).unwrap();
    assert!(
        inferred
            .listing
            .contains(fixture["matcher"].as_str().unwrap())
    );
    assert!(
        inferred.listing.rfind("(valid word)").unwrap()
            < inferred.listing.find("(valid member)").unwrap()
    );
    let listing = format!(
        "(grammar (start source))\n{}{}",
        inferred.listing,
        fixture["syntax"].as_str().unwrap()
    );
    let loaded = parse_grammar_links(&render_grammar_links(
        &parse_grammar_links(&listing).unwrap(),
    ))
    .unwrap();
    let options = FeatureParseOptions::default();
    let parser = compile_feature_grammar(&loaded, None, options.clone()).unwrap();
    for row in fixture["accepted"].as_array().unwrap() {
        let input = row["input"].as_str().unwrap();
        let result = parser.parse_tree(input.as_bytes(), &options).unwrap();
        assert!(result.ok, "{input}: {:?}", result.rejection);
        assert_eq!(result.tree.unwrap().render(), row["tree"].as_str().unwrap());
    }
    for input in fixture["rejected"].as_array().unwrap() {
        let input = input.as_str().unwrap();
        assert!(
            !parser.parse_tree(input.as_bytes(), &options).unwrap().ok,
            "{input}"
        );
    }
    let mut inference = RegularGrammarInferenceOptions {
        scanner_character_classes: true,
        ..Default::default()
    };
    assert!(
        infer_regular_grammar(&[], &[], &inference)
            .unwrap_err()
            .contains("require scanner observations")
    );
    inference.scanner_observations = Some(observations);
    let original = infer_regular_grammar(
        &[vec!["member".to_owned()], vec!["word".to_owned()]],
        &[vec![]],
        &inference,
    )
    .unwrap();
    let text = LinkNetwork::from_lino(&original.network.to_lino()).unwrap();
    let restored = read_regular_grammar_inference_links(&text, original.root).unwrap();
    assert_eq!(
        serialize_grammar(&restored.grammar),
        serialize_grammar(&original.grammar)
    );
    let counterexample = [vec!["word".to_owned(), "member".to_owned()]];
    let refined = refine_regular_grammar_inference(
        &restored.network,
        restored.root,
        &counterexample,
        &[],
        None,
    )
    .unwrap();
    let options = BinaryLinksOptions::default();
    let bytes = encode_link_network_binary(&refined.network, options).unwrap();
    let binary = decode_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap();
    let reloaded = read_refined_regular_grammar_inference_links(&binary, refined.root).unwrap();
    assert_eq!(
        serialize_grammar(&reloaded.grammar),
        serialize_grammar(&refined.grammar)
    );
    assert!(accept_inferred_grammar_symbols(&reloaded, &counterexample[0]).unwrap());
}

#[test]
fn scanner_mode_stacks_restore_each_actual_parent_and_replay_the_selected_archive() {
    versions();
    let data = fixture();
    let fixture = &data["scannerModeStack"];
    let observations: Vec<GrammarScannerModeStackObservation> =
        serde_json::from_value(fixture["observations"].clone()).unwrap();
    for classes in [false, true] {
        let inferred = infer_grammar_scanner_mode_stack(&observations, None, classes).unwrap();
        for operation in fixture["operations"].as_array().unwrap() {
            assert!(inferred.listing.contains(operation.as_str().unwrap()));
        }
        assert_eq!(
            inferred
                .listing
                .contains(fixture["matcher"].as_str().unwrap()),
            classes
        );
        assert_eq!(inferred.listing.matches("(popMode)").count(), 1);
        let listing = format!(
            "(grammar (start source))\n{}{}",
            inferred.listing,
            fixture["syntax"].as_str().unwrap()
        );
        let loaded = parse_grammar_links(&render_grammar_links(
            &parse_grammar_links(&listing).unwrap(),
        ))
        .unwrap();
        let parse_options = FeatureParseOptions::default();
        let parser = compile_feature_grammar(&loaded, None, parse_options.clone()).unwrap();
        for row in fixture["accepted"].as_array().unwrap() {
            let input = row["input"].as_str().unwrap();
            let result = parser.parse_tree(input.as_bytes(), &parse_options).unwrap();
            assert!(result.ok, "{input}: {:?}", result.rejection);
            assert_eq!(result.tree.unwrap().render(), row["tree"].as_str().unwrap());
        }
        for input in fixture["rejected"].as_array().unwrap() {
            let input = input.as_str().unwrap();
            assert!(
                !parser
                    .parse_tree(input.as_bytes(), &parse_options)
                    .unwrap()
                    .ok,
                "{input}"
            );
        }
        let original = infer_regular_grammar(
            &[["begin", "word", "end", "suffix"]
                .map(str::to_owned)
                .to_vec()],
            &[vec![]],
            &RegularGrammarInferenceOptions {
                scanner_mode_stack_observations: Some(observations.clone()),
                scanner_character_classes: classes,
                ..Default::default()
            },
        )
        .unwrap();
        let text = LinkNetwork::from_lino(&original.network.to_lino()).unwrap();
        let restored = read_regular_grammar_inference_links(&text, original.root).unwrap();
        assert_eq!(
            serialize_grammar(&restored.grammar),
            serialize_grammar(&original.grammar)
        );
        let addition = ["alternate", "begin", "member", "end", "suffix"]
            .map(str::to_owned)
            .to_vec();
        let refined = refine_regular_grammar_inference(
            &restored.network,
            restored.root,
            std::slice::from_ref(&addition),
            &[],
            None,
        )
        .unwrap();
        let bytes =
            encode_link_network_binary(&refined.network, BinaryLinksOptions::default()).unwrap();
        let binary = decode_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap();
        let reloaded = read_refined_regular_grammar_inference_links(&binary, refined.root).unwrap();
        assert_eq!(
            serialize_grammar(&reloaded.grammar),
            serialize_grammar(&refined.grammar)
        );
        assert!(accept_inferred_grammar_symbols(&reloaded, &addition).unwrap());
    }
    let mut conflicting = observations;
    conflicting.push(serde_json::from_value(fixture["conflicting"].clone()).unwrap());
    assert!(
        infer_grammar_scanner_mode_stack(&conflicting, None, false)
            .unwrap_err()
            .contains("unsupported context sensitivity")
    );
    let mut unsupported: GrammarScannerModeStackObservation =
        serde_json::from_value(fixture["unsupported"].clone()).unwrap();
    assert!(
        infer_grammar_scanner_mode_stack(std::slice::from_ref(&unsupported), None, false)
            .unwrap_err()
            .contains("at most one native scanner mode operation")
    );
    unsupported.after.clear();
    assert!(
        infer_grammar_scanner_mode_stack(&[unsupported], None, false)
            .unwrap_err()
            .contains("bounded nonempty")
    );
    let mixed = RegularGrammarInferenceOptions {
        scanner_observations: Some(Vec::new()),
        scanner_mode_stack_observations: Some(Vec::new()),
        ..Default::default()
    };
    assert!(
        infer_regular_grammar(&[], &[], &mixed)
            .unwrap_err()
            .contains("one scanner observation format")
    );
    record(&Observation {
        requirement_id: "I195-GRAMMAR-INFERENCE-SCANNER-MODE-STACK",
        suffix: "behavior",
        fixture_id: "regular-grammar-inference-scanner-mode-stack",
        fixture_file: "parity/fixtures/regular-grammar-inference.lino",
        assertions: &[
            "scannerModeStackTreesAndRejectionsMatch",
            "scannerModeStackArchiveReplaysAndRefines",
            "unsupportedScannerModeStackEffectsAreRefused",
        ],
        test_name: "scanner_mode_stacks_restore_each_actual_parent_and_replay_the_selected_archive",
    });
}

#[test]
fn scanner_trace_inference_emits_native_mode_transitions_and_preserves_bytes() {
    versions();
    let data = fixture();
    let fixture = &data["scanner"];
    let mut observations: Vec<GrammarScannerObservation> =
        serde_json::from_value(fixture["observations"].clone()).unwrap();
    let inferred = infer_grammar_scanner(&observations, None).unwrap();
    assert_eq!(inferred.modes, ["inside"]);
    assert!(
        inferred.listing.find("(valid long)").unwrap()
            < inferred.listing.find("(valid short)").unwrap()
    );
    assert_eq!(inferred.listing.matches("(valid content)").count(), 1);
    let listing = format!(
        "(grammar (start source))\n{}{}",
        inferred.listing,
        fixture["syntax"].as_str().unwrap()
    );
    let loaded = parse_grammar_links(&render_grammar_links(
        &parse_grammar_links(&listing).unwrap(),
    ))
    .unwrap();
    let options = FeatureParseOptions::default();
    let parser = compile_feature_grammar(&loaded, None, options.clone()).unwrap();
    for row in fixture["accepted"].as_array().unwrap() {
        let input = row["input"].as_str().unwrap();
        let result = parser.parse_tree(input.as_bytes(), &options).unwrap();
        assert!(result.ok, "{input}: {:?}", result.rejection);
        let tree = result.tree.unwrap();
        assert_eq!(tree.render(), row["tree"].as_str().unwrap());
        assert_eq!(super::issue_195_native_grammar_rows::rebuilt(&tree), input);
    }
    for input in fixture["rejected"].as_array().unwrap() {
        let input = input.as_str().unwrap();
        assert!(
            !parser.parse_tree(input.as_bytes(), &options).unwrap().ok,
            "{input}"
        );
    }
    observations.push(serde_json::from_value(fixture["conflicting"].clone()).unwrap());
    assert!(
        infer_grammar_scanner(&observations, None)
            .unwrap_err()
            .contains("unsupported context sensitivity")
    );
}
pub(super) fn examples(value: &Value) -> Vec<Vec<String>> {
    serde_json::from_value(value.clone()).unwrap()
}
pub(super) fn versions() {
    let output = Command::new("rustc").arg("--version").output().unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    let version = text.split_whitespace().nth(1).unwrap();
    let parts = version
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {version}; >=1.90 <1.100"
    );
}

#[test]
fn observations_controls_codecs_and_complete_archive_have_independent_runtime_parity() {
    versions();
    let fixtures = fixture();
    for (index, input) in fixtures["cases"].as_array().unwrap().iter().enumerate() {
        let options = RegularGrammarInferenceOptions {
            parameters: serde_json::from_value(input["options"].clone()).unwrap(),
            ..Default::default()
        };
        let original = infer_regular_grammar(
            &examples(&input["positives"]),
            &examples(&input["negatives"]),
            &options,
        )
        .unwrap();
        let producer = Command::new("node")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .args([
                "--max-old-space-size=352",
                "--max-semi-space-size=1",
                "../experiments/grammar-inference/oracle.mjs",
                &index.to_string(),
            ])
            .output()
            .unwrap();
        assert!(
            producer.status.success(),
            "{}",
            String::from_utf8_lossy(&producer.stderr)
        );
        let expected =
            read_structured_data_lino(&String::from_utf8(producer.stdout).unwrap()).unwrap();
        let grammar: Value = serde_json::from_str(&serialize_grammar(&original.grammar)).unwrap();
        assert_eq!(
            json!({"name":input["name"], "alphabet":original.alphabet, "table":original.table,
            "grammar":grammar, "root":original.root.as_u64(),
            "archiveSha256":format!("{:x}", Sha256::digest(original.network.to_lino().as_bytes()))}),
            expected
        );
        let binary =
            encode_link_network_binary(&original.network, BinaryLinksOptions::default()).unwrap();
        for network in [
            LinkNetwork::from_lino(&original.network.to_lino()).unwrap(),
            decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
        ] {
            let model = read_regular_grammar_inference_links(&network, original.root).unwrap();
            assert_eq!(model.table, original.table);
            for symbols in examples(&input["positives"]) {
                assert!(accept_inferred_grammar_symbols(&model, &symbols).unwrap());
            }
            for symbols in examples(&input["negatives"]) {
                assert!(!accept_inferred_grammar_symbols(&model, &symbols).unwrap());
            }
            for control in input["controls"].as_array().unwrap() {
                let symbols: Vec<String> =
                    serde_json::from_value(control["symbols"].clone()).unwrap();
                assert_eq!(
                    accept_inferred_grammar_symbols(&model, &symbols).unwrap(),
                    control["accepted"].as_bool().unwrap()
                );
            }
        }
    }
}

#[test]
fn contradictions_and_every_explicit_resource_budget_remain_refusals() {
    versions();
    for input in fixture()["refusals"].as_array().unwrap() {
        let options = RegularGrammarInferenceOptions {
            parameters: serde_json::from_value(input["options"].clone()).unwrap(),
            ..Default::default()
        };
        let error = infer_regular_grammar(
            &examples(&input["positives"]),
            &examples(&input["negatives"]),
            &options,
        )
        .unwrap_err();
        assert!(error.contains(input["reason"].as_str().unwrap()), "{error}");
    }
}
