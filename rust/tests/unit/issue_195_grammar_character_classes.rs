//! Execute the JavaScript importer's compiled Links data in Rust. The pinned
//! regex crate is an independent test oracle, never the production executor.

use meta_language::{FeatureParseOptions, compile_feature_grammar, parse_grammar_links};
use regex::Regex;
use serde_json::Value;

use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-character-class-grammars.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-character-class-grammars.json");

#[test]
fn character_class_sets_match_fixtures_and_independent_regex_oracle() {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("shared fixture is JSON");
    for entry in fixture["cases"].as_array().expect("cases") {
        let id = entry["id"].as_str().expect("case identity");
        let listing = entry["grammar"].as_str().expect("compiled Links grammar");
        assert!(!listing.contains("(regex "), "{id} uses native Links");
        let grammar = parse_grammar_links(listing).expect("compiled Links parses");
        let options = FeatureParseOptions::default();
        let parser = compile_feature_grammar(&grammar, None, options.clone())
            .expect("native character-class grammar compiles");
        let pattern = entry["pattern"].as_str().expect("source pattern");
        let oracle = Regex::new(&format!(r"\A(?:{pattern})\z"))
            .unwrap_or_else(|error| panic!("{id}: oracle pattern compiles: {error}"));
        for scalar in 0..128_u8 {
            let input = char::from(scalar).to_string();
            let expected = entry["asciiExcludes"].as_str().map_or_else(
                || {
                    entry["asciiAccepts"]
                        .as_str()
                        .expect("ASCII set")
                        .contains(&input)
                },
                |excluded| !excluded.contains(&input),
            );
            assert_eq!(
                oracle.is_match(&input),
                expected,
                "{id}: oracle ASCII {scalar}"
            );
            let result = parser
                .parse_tree(input.as_bytes(), &options)
                .expect("start rule exists");
            assert_eq!(result.ok, expected, "{id}: native ASCII {scalar}");
        }
        for (field, expected) in [("accepts", true), ("rejects", false)] {
            for input in entry[field].as_array().expect("input corpus") {
                let input = input.as_str().expect("input text");
                assert_eq!(oracle.is_match(input), expected, "{id}: oracle {input:?}");
                let result = parser
                    .parse_tree(input.as_bytes(), &options)
                    .expect("start rule exists");
                assert_eq!(result.ok, expected, "{id}: native {input:?}");
                if expected {
                    let tree = result.tree.expect("accepted input has a tree");
                    assert_eq!(
                        (tree.start(), tree.end()),
                        (0, input.len()),
                        "{id}: complete input"
                    );
                }
            }
        }
    }
    record(&Observation {
        requirement_id: "I195-GRAMMAR-CHARACTER-CLASS-SETS",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-character-class-sets",
        fixture_file: FIXTURE_FILE,
        assertions: &[
            "compiledClassSetsUseLinks",
            "nativeClassSetsMatchFixtures",
            "nativeClassSetsConsumeCompleteInputs",
        ],
        test_name: "character_class_sets_match_fixtures_and_independent_regex_oracle",
    });
    record(&Observation {
        requirement_id: "I195-GRAMMAR-HEXADECIMAL-SCALARS",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-hexadecimal-scalars",
        fixture_file: FIXTURE_FILE,
        assertions: &[
            "nativeHexadecimalScalarsMatchFixtures",
            "nativeScalarEscapesConsumeCompleteInputs",
        ],
        test_name: "character_class_sets_match_fixtures_and_independent_regex_oracle",
    });
}
