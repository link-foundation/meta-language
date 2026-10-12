//! Aliased extras keep their source identity during lexing and scanner state
//! transitions, then expose the alias in the public tree. JavaScript executes
//! the same miniature fixture in grammar-aliased-extras.test.js.

use super::issue_195_native_grammar_rows::{
    Rows, leaves, oracle_rows, parse, parser, rebuilt, text,
};
use meta_language::{FeatureParseOptions, SyntaxTree};
use serde_json::{Value, json};

const FIXTURE: &str = include_str!("../../../parity/fixtures/grammar-aliased-extras.json");

fn nodes(tree: &SyntaxTree) -> Vec<&SyntaxTree> {
    let mut found = vec![tree];
    if let SyntaxTree::Node { children, .. } = tree {
        for child in children {
            found.extend(nodes(child));
        }
    }
    found
}

fn kind(tree: &SyntaxTree) -> Option<&str> {
    match tree {
        SyntaxTree::Node { kind, .. } => Some(kind),
        SyntaxTree::Token { kind, .. } => kind.as_deref(),
        _ => None,
    }
}

#[test]
fn aliased_extra_fixtures_preserve_state_fields_spans_and_source_retake() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 4);
    for case in cases {
        let name = case["name"].as_str().unwrap();
        let parser = parser(case["listing"].as_str().unwrap());
        for input in case["reject"].as_array().unwrap() {
            let input = input.as_str().unwrap();
            assert!(parse(&parser, input).is_none(), "{name}: {input:?}");
        }
        let input = case["source"].as_str().unwrap();
        let tree = parse(&parser, input).unwrap_or_else(|| panic!("{name}: {input:?}"));
        assert_eq!(tree.render(), case["tree"].as_str().unwrap(), "{name}");
        assert_eq!(rebuilt(&tree), input, "{name}");
        let mut offset = 0;
        for leaf in leaves(&tree) {
            assert_eq!(leaf.start(), offset, "{name}: leaf start");
            let SyntaxTree::Token { text: value, .. } = leaf else {
                panic!("{name}: unexpected accepted leaf {leaf:?}");
            };
            offset += text(value).len();
            assert_eq!(leaf.end(), offset, "{name}: leaf end");
        }
        assert_eq!(offset, input.len(), "{name}: complete byte coverage");

        if let Some(extra) = case.get("extra") {
            let extra_kind = extra["kind"].as_str().unwrap();
            let found = nodes(&tree)
                .into_iter()
                .find(|node| kind(node) == Some(extra_kind))
                .unwrap_or_else(|| panic!("{name}: missing extra alias {extra_kind}"));
            assert!(
                matches!(
                    found,
                    SyntaxTree::Node { trivia: true, .. } | SyntaxTree::Token { trivia: true, .. }
                ),
                "{name}: the alias remains an extra"
            );
            assert_eq!(
                found.start(),
                usize::try_from(extra["start"].as_u64().unwrap()).unwrap()
            );
            assert_eq!(
                found.end(),
                usize::try_from(extra["end"].as_u64().unwrap()).unwrap()
            );
            if let Some(expected) = extra["text"].as_str() {
                let SyntaxTree::Token { text: value, .. } = found else {
                    panic!("{name}: expected an extra leaf");
                };
                assert_eq!(text(value), expected, "{name}: extra text");
            }
        }
        if let Some(field) = case.get("field") {
            let field_name = field["name"].as_str().unwrap();
            let found = nodes(&tree)
                .into_iter()
                .find(|node| match node {
                    SyntaxTree::Node { field, .. } | SyntaxTree::Token { field, .. } => {
                        field.as_deref() == Some(field_name)
                    }
                    _ => false,
                })
                .unwrap_or_else(|| panic!("{name}: missing field {field_name}"));
            assert_eq!(
                found.start(),
                usize::try_from(field["start"].as_u64().unwrap()).unwrap()
            );
            assert_eq!(
                found.end(),
                usize::try_from(field["end"].as_u64().unwrap()).unwrap()
            );
            let SyntaxTree::Token { text: value, .. } = found else {
                panic!("{name}: expected a captured token");
            };
            assert_eq!(text(value), field["text"].as_str().unwrap());
        }
        if let Some(expected) = case.get("rows") {
            let projection = json!({
                "hidden": case["projection"]["hidden"].as_array().cloned().unwrap_or_default(),
                "anonymous": case["projection"]["anonymous"].as_array().cloned().unwrap_or_default(),
                "extras": case["projection"]["extras"].as_array().cloned().unwrap_or_default(),
                "oracleKinds": case["projection"]["oracleKinds"].as_object().cloned().unwrap_or_default(),
            });
            assert_eq!(
                Value::Array(Rows::new(&projection).rows(&tree, input)),
                *expected,
                "{name}: anonymous trivia stays separate from significant syntax"
            );
        }
    }
}

#[test]
fn r_aliased_scanner_extras_preserve_the_original_comment_before_else_oracle() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/native-grammars/r.json"
    ))
    .unwrap();
    let input = "{if(x) y\n# note\nelse z}\n";
    assert!(
        fixture["matches"]
            .as_array()
            .unwrap()
            .iter()
            .any(|case| { case["source"].as_str() == Some(input) })
    );
    let (expected, recovers) = oracle_rows(&tree_sitter_r::LANGUAGE.into(), input);
    assert!(!recovers);
    let parser = parser(include_str!("../../../parity/grammars/native/r.lino"));
    let tree = parse(&parser, input).expect("the unchanged R focused source is accepted");
    assert_eq!(Rows::new(&fixture).rows(&tree, input), expected);
    assert_eq!(rebuilt(&tree), input);
    assert!(leaves(&tree).into_iter().any(|leaf| {
        matches!(leaf, SyntaxTree::Token { kind: Some(kind), trivia: true, .. } if kind == "unnamed_token")
    }));
}

#[test]
fn odin_aliased_scanner_extras_preserve_the_original_invalid_comment_recovery() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/native-grammars/odin.json"
    ))
    .unwrap();
    let input = "package garden\n/*\"not a comment*/\n";
    let case = fixture["rejections"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["source"].as_str() == Some(input))
        .expect("the original Odin recovery fixture remains present");
    assert!(oracle_rows(&tree_sitter_odin::LANGUAGE.into(), input).1);
    let parser = parser(include_str!("../../../parity/grammars/native/odin.lino"));
    assert!(parse(&parser, input).is_none());
    let outcome = parser
        .parse_tree(
            input.as_bytes(),
            &FeatureParseOptions {
                error_recovery: Some(true),
                ..FeatureParseOptions::default()
            },
        )
        .unwrap();
    assert!(!outcome.ok);
    let tree = outcome.tree.expect("recovered tree");
    assert_eq!(tree.render(), case["recovered"].as_str().unwrap());
    let recovered: String = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token { text: value, .. } | SyntaxTree::Error { text: value, .. } => {
                text(value)
            }
            SyntaxTree::Missing { start, end, .. } => {
                assert_eq!(start, end);
                ""
            }
            other => panic!("unexpected recovery leaf {other:?}"),
        })
        .collect();
    assert_eq!(recovered, input);
}

#[test]
fn anonymous_extra_fixtures_preserve_names_state_prefixes_and_exact_spans() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/grammar-extra-projection.json"
    ))
    .unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let parser = parser(case["listing"].as_str().unwrap());
        for input in case["reject"].as_array().unwrap() {
            assert!(parse(&parser, input.as_str().unwrap()).is_none(), "{name}");
        }
        let input = case["source"].as_str().unwrap();
        let tree = parse(&parser, input).unwrap_or_else(|| panic!("{name}: {input:?}"));
        assert_eq!(tree.render(), case["tree"].as_str().unwrap(), "{name}");
        assert_eq!(rebuilt(&tree), input, "{name}");
        let actual: Vec<Value> = leaves(&tree)
            .into_iter()
            .map(|leaf| match leaf {
                SyntaxTree::Token {
                    kind,
                    start,
                    end,
                    trivia,
                    text: value,
                    ..
                } => json!([kind, start, end, trivia, text(value)]),
                other => panic!("{name}: unexpected accepted leaf {other:?}"),
            })
            .collect();
        assert_eq!(Value::Array(actual), case["tokens"], "{name}");
        if let Some(recoveries) = case["recover"].as_array() {
            for recovery in recoveries {
                let input = recovery["source"].as_str().unwrap();
                let outcome = parser
                    .parse_tree(
                        input.as_bytes(),
                        &FeatureParseOptions {
                            error_recovery: Some(true),
                            ..FeatureParseOptions::default()
                        },
                    )
                    .unwrap();
                assert!(!outcome.ok);
                let tree = outcome.tree.expect("recovered tree");
                assert_eq!(tree.render(), recovery["tree"].as_str().unwrap(), "{name}");
                let reconstructed: String = leaves(&tree)
                    .into_iter()
                    .map(|leaf| match leaf {
                        SyntaxTree::Token { text: value, .. }
                        | SyntaxTree::Error { text: value, .. } => text(value),
                        SyntaxTree::Missing { .. } => "",
                        other => panic!("unexpected recovered leaf {other:?}"),
                    })
                    .collect();
                assert_eq!(reconstructed, input);
            }
        }
    }
}

#[test]
fn make_continuation_extras_preserve_the_original_oracle_rows() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/native-grammars/make.json"
    ))
    .unwrap();
    let input = "target: foo \\\n\tbar | baz \\\nfoobar\n";
    let case = fixture["matches"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["source"].as_str() == Some(input))
        .expect("the original Make CI source remains present");
    let (expected, recovers) = oracle_rows(&tree_sitter_make::LANGUAGE.into(), input);
    assert!(!recovers);
    assert_eq!(Value::Array(expected), case["rows"]);
    let tree = parse(
        &parser(include_str!("../../../parity/grammars/native/make.lino")),
        input,
    )
    .expect("the continued prerequisite list is accepted");
    assert_eq!(
        Value::Array(Rows::new(&fixture).rows(&tree, input)),
        case["rows"]
    );
    assert_eq!(rebuilt(&tree), input);
    let continuations: Vec<_> = leaves(&tree)
        .into_iter()
        .filter_map(|leaf| match leaf {
            SyntaxTree::Token {
                kind,
                start,
                end,
                trivia,
                text: value,
                ..
            } if text(value) == "\\\n" => Some((kind.clone(), *start, *end, *trivia)),
            _ => None,
        })
        .collect();
    assert_eq!(
        continuations,
        vec![(None, 12, 14, true), (None, 25, 27, true)]
    );
}

#[test]
fn pascal_roots_preserve_leading_separators_and_independent_syntax_spans() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/native-grammars/pascal.json"
    ))
    .unwrap();
    assert_eq!(fixture["rootIncludesLeadingTrivia"], true);
    let parser = parser(include_str!("../../../parity/grammars/native/pascal.lino"));
    for input in [
        "\nvar x: Integer;",
        "\r\nvar x: Integer;",
        " \t\nvar x: Integer;",
        "var x: Integer;",
    ] {
        let (expected, recovers) = oracle_rows(&tree_sitter_pascal::LANGUAGE.into(), input);
        assert!(!recovers);
        let tree = parse(&parser, input).expect("the leading separator source is accepted");
        assert_eq!(
            Rows::new(&fixture).rows(&tree, input),
            expected,
            "{input:?}"
        );
        assert_eq!(tree.start(), 0);
        assert_eq!(expected[0][4], 0);
        assert_eq!(rebuilt(&tree), input);
    }
}

#[test]
fn odin_anonymous_scanner_extras_preserve_the_original_default_recovery_rows() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/native-grammars/odin.json"
    ))
    .unwrap();
    let inventory: Value = serde_json::from_str(include_str!(
        "../../../parity/language-grammar-inventory.json"
    ))
    .unwrap();
    let input = inventory["languages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["name"] == "Odin")
        .unwrap()["recoverySource"]
        .as_str()
        .unwrap();
    let expected: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/native-default-cst-expected.json"
    ))
    .unwrap();
    assert!(oracle_rows(&tree_sitter_odin::LANGUAGE.into(), input).1);
    let parser = parser(include_str!("../../../parity/grammars/native/odin.lino"));
    let outcome = parser
        .parse_tree(
            input.as_bytes(),
            &FeatureParseOptions {
                error_recovery: Some(true),
                accept_recovery: Some(true),
                ..FeatureParseOptions::default()
            },
        )
        .unwrap();
    assert!(outcome.ok);
    let tree = outcome.tree.expect("recovered tree");
    assert_eq!(
        Value::Array(Rows::new(&fixture).rows(&tree, input)),
        expected["languages"]["Odin"]["recovery"]
    );
}
