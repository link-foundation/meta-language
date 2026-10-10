use meta_language::{FeatureParseOptions, compile_feature_grammar, parse_grammar_links};

#[test]
fn indentation_scanners_keep_block_boundaries_across_virtual_newlines() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/scanner-indentation.json"
    ))
    .unwrap();
    let parser = compile_feature_grammar(
        &parse_grammar_links(fixture["listing"].as_str().unwrap()).unwrap(),
        None,
        FeatureParseOptions::default(),
    )
    .unwrap();
    for case in fixture["accept"].as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let outcome = parser
            .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
            .unwrap();
        assert!(outcome.ok, "{input:?}: {:?}", outcome.rejection);
        assert_eq!(
            outcome.tree.unwrap().render(),
            case["tree"].as_str().unwrap(),
            "{input:?}"
        );
    }
    for input in fixture["reject"].as_array().unwrap() {
        let input = input.as_str().unwrap();
        assert!(
            !parser
                .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
                .unwrap()
                .ok,
            "{input:?}"
        );
    }
    let bounded = fixture["listing"]
        .as_str()
        .unwrap()
        .replace("(integer 65536)", "(integer 16)");
    let parser = compile_feature_grammar(
        &parse_grammar_links(&bounded).unwrap(),
        None,
        FeatureParseOptions::default(),
    )
    .unwrap();
    for (spaces, expected) in [(15, true), (16, false)] {
        let input = format!("a:\n{}b\n", " ".repeat(spaces));
        assert_eq!(
            parser
                .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
                .unwrap()
                .ok,
            expected
        );
    }
}
