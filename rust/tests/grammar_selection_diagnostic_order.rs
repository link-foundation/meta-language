//! PRIVATE UNEXECUTED CI-only integration test proposal. No native replacement
//! semantics: invoke the production parser and compare every original JS record.

use std::collections::HashMap;
use std::fs;

use meta_language::{
    FeatureParseOptions, Grammar, compile_feature_grammar, parse_grammar_links,
    read_structured_data_lino,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .fold(String::with_capacity(64), |mut hexadecimal, byte| {
            use std::fmt::Write as _;
            write!(hexadecimal, "{byte:02x}").expect("writing into a String succeeds");
            hexadecimal
        })
}

#[test]
fn grammar_selection_diagnostics_retain_javascript_member_order() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../parity/fixtures/grammar-selection-diagnostic-order.lino"
    );
    let source = fs::read_to_string(path).expect("generated original JavaScript fixture");
    let fixture = read_structured_data_lino(&source).expect("normal Links fixture");
    let profile = &fixture["dependencyProfile"];
    assert_eq!(profile["name"].as_str(), Some("serde_json"));
    let version = profile["version"].as_str().expect("exact locked version");
    assert_eq!(
        profile["coveredRange"]["minimumInclusive"].as_str(),
        Some(version),
        "the assertion range starts at its actual checked lock version"
    );
    let parts = version
        .split('.')
        .map(|part| {
            part.parse::<u64>()
                .expect("uncovered non-numeric dependency version")
        })
        .collect::<Vec<_>>();
    assert_eq!(parts.len(), 3, "complete exact patch version");
    let maximum = format!(
        "{}.{}.{}",
        parts[0],
        parts[1],
        parts[2].checked_add(1).expect("bounded patch")
    );
    assert_eq!(
        profile["coveredRange"]["maximumExclusive"].as_str(),
        Some(maximum.as_str()),
        "only the actual locked patch range is covered"
    );
    for (field, filename) in [("lock", "Cargo.lock"), ("manifest", "Cargo.toml")] {
        let path = format!("{}/{filename}", env!("CARGO_MANIFEST_DIR"));
        let actual = fs::read(path).expect("actual current dependency source");
        assert_eq!(
            digest(&actual),
            profile[field]["sha256"]
                .as_str()
                .expect("recorded source hash"),
            "uncovered dependency version, lock or manifest profile; revalidate the explicit range before applying order assertions"
        );
        let selected = profile[field]["selected"]["source"]
            .as_str()
            .expect("complete source-owner block");
        let actual = String::from_utf8(actual).expect("UTF-8 dependency owner");
        assert_eq!(
            actual.matches(selected).count(),
            1,
            "complete selected dependency source remains unique"
        );
        if field == "lock" {
            assert!(
                selected
                    .lines()
                    .any(|line| line == format!("version = \"{version}\"")),
                "actual locked version selects the explicit assertion range"
            );
        }
    }
    assert_eq!(
        profile["requiredFeatures"][0].as_str(),
        Some("preserve_order")
    );
    let capability = serde_json::json!({"second":0,"first":1});
    assert_eq!(
        capability
            .as_object()
            .expect("active object profile")
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["second", "first"],
        "uncovered active serde_json order feature profile"
    );
    eprintln!(
        "diagnostic order profile: serde_json {version}, range [{version}, {maximum}), required preserve_order capability observed; source lock/manifest hashes matched"
    );
    let cases = fixture["cases"].as_array().expect("all fixture cases");
    assert_eq!(cases.len(), 2, "both original diagnostic paths remain");
    for case in cases {
        let mut grammars: HashMap<String, Grammar> = HashMap::new();
        for grammar in case["grammars"].as_array().expect("actual linked grammars") {
            grammars.insert(
                grammar["name"].as_str().expect("grammar name").to_owned(),
                parse_grammar_links(grammar["listing"].as_str().expect("actual listing"))
                    .expect("parse linked grammar"),
            );
        }
        let resolve = |name: &str| grammars.get(name).cloned();
        let grammar = parse_grammar_links(case["listing"].as_str().expect("host listing"))
            .expect("parse original host grammar");
        let parser =
            compile_feature_grammar(&grammar, Some(&resolve), FeatureParseOptions::default())
                .expect("compile original grammar");
        let outcome = parser
            .parse_tree(
                case["source"].as_str().expect("original input").as_bytes(),
                &FeatureParseOptions::default(),
            )
            .expect("retain original native parse outcome");
        assert!(!outcome.ok);
        let rejection = outcome.rejection.expect("actual rejection");
        assert_eq!(
            rejection.reason,
            case["expectedReason"]
                .as_str()
                .expect("expected original reason")
        );
        let actual = rejection.diagnostics.expect("all actual diagnostics");
        let expected = case["expectedDiagnostics"]
            .as_array()
            .expect("all actual original JavaScript diagnostics");
        assert_eq!(
            actual.len(),
            expected.len(),
            "every original region remains"
        );
        let keys = case["expectedDiagnosticKeys"]
            .as_array()
            .expect("actual source-derived field order")
            .iter()
            .map(|key| key.as_str().expect("field name"))
            .collect::<Vec<_>>();
        for (actual, expected) in actual.iter().zip(expected) {
            assert_eq!(actual, expected, "complete original diagnostic values");
            let Value::Object(object) = actual else {
                panic!("diagnostic remains an object");
            };
            assert_eq!(
                object.keys().map(String::as_str).collect::<Vec<_>>(),
                keys,
                "every actual ordered member, not sorted or normalized"
            );
        }
    }
}
