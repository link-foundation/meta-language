//! Under `(matching longest)` a token the parse asks for is lexed over a
//! separator or the token of a silent rule's extra whose text it also
//! matches, as tree-sitter shifts a valid token before it reduces the same
//! text to an extra (INI's and CSV's line ends).
//! js/tests/grammar-token-over-extra.test.js checks the JavaScript executor
//! against the same fixture.

use std::fs;
use std::path::Path;

use meta_language::{FeatureParseOptions, compile_feature_grammar, parse_native_grammar};
use serde_json::Value;

#[test]
fn a_token_is_lexed_over_the_separator_or_silent_extra_its_text_also_matches() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/grammar-token-over-extra.json");
    let fixture: Value =
        serde_json::from_str(&fs::read_to_string(path).expect("fixture")).expect("fixture JSON");
    let options = FeatureParseOptions::default();
    for case in fixture["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let grammar = parse_native_grammar(case["grammar"].as_str().expect("grammar"))
            .expect("the grammar parses");
        let parser =
            compile_feature_grammar(&grammar, None, options.clone()).expect("the grammar compiles");
        for parse in case["parses"].as_array().expect("parses") {
            let input = parse["input"].as_str().expect("input");
            let outcome = parser
                .parse_tree(input.as_bytes(), &options)
                .expect("the parse runs");
            assert_eq!(
                outcome.tree.map(|tree| tree.render()).as_deref(),
                parse["tree"].as_str(),
                "{name}: {input:?}"
            );
        }
    }
}
