//! Future CI controls; not executed during the complete source draft phase.
use meta_language::{
    BinaryLinksOptions, FeatureParseOptions, GrammarScannerObservation, LinkNetwork,
    RegularGrammarInferenceOptions, accept_inferred_grammar_symbols, compile_feature_grammar,
    decode_link_network_binary, encode_link_network_binary, infer_grammar_scanner,
    infer_regular_grammar, parse_grammar_links, read_regular_grammar_inference_links,
    read_structured_data_lino, render_grammar_links, serialize_grammar,
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
