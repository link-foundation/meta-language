//! Requirement I195-GRAMMAR-NATIVE-RECOVERY: automatic error recovery in the
//! native executor. With `error_recovery` a parse the grammar rejects is
//! repaired into a tree of ERROR and MISSING leaves, with no recovery rules in
//! the grammar. Every rejection of parity/fixtures/native-grammars/*.json
//! records its `recovered` tree, which js/scripts/generate-native-grammar-fixtures.mjs
//! generates with the JavaScript executor; this suite checks that the Rust
//! executor builds the same trees, as
//! js/tests/issue-195-grammar-native-recovery.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, LinkNetwork, ParseConfiguration, ParseOutcome,
    SyntaxTree,
};
use serde_json::Value;

use super::cst_sexpression::{normalize, render_network};
use super::issue_195_native_grammar_rows::{cases, parser, source};
use super::issue_195_observations::{Observation, record};

const GRAMMARS: [(&str, &str, &str); 22] = [
    (
        "lua",
        include_str!("../../../parity/grammars/native/lua.lino"),
        include_str!("../../../parity/fixtures/native-grammars/lua.json"),
    ),
    (
        "json",
        include_str!("../../../parity/grammars/native/json.lino"),
        include_str!("../../../parity/fixtures/native-grammars/json.json"),
    ),
    (
        "ini",
        include_str!("../../../parity/grammars/native/ini.lino"),
        include_str!("../../../parity/fixtures/native-grammars/ini.json"),
    ),
    (
        "diff",
        include_str!("../../../parity/grammars/native/diff.lino"),
        include_str!("../../../parity/fixtures/native-grammars/diff.json"),
    ),
    (
        "csv",
        include_str!("../../../parity/grammars/native/csv.lino"),
        include_str!("../../../parity/fixtures/native-grammars/csv.json"),
    ),
    (
        "json5",
        include_str!("../../../parity/grammars/native/json5.lino"),
        include_str!("../../../parity/fixtures/native-grammars/json5.json"),
    ),
    (
        "scheme",
        include_str!("../../../parity/grammars/native/scheme.lino"),
        include_str!("../../../parity/fixtures/native-grammars/scheme.json"),
    ),
    (
        "racket",
        include_str!("../../../parity/grammars/native/racket.lino"),
        include_str!("../../../parity/fixtures/native-grammars/racket.json"),
    ),
    (
        "c",
        include_str!("../../../parity/grammars/native/c.lino"),
        include_str!("../../../parity/fixtures/native-grammars/c.json"),
    ),
    (
        "rust",
        include_str!("../../../parity/grammars/native/rust.lino"),
        include_str!("../../../parity/fixtures/native-grammars/rust.json"),
    ),
    (
        "javascript",
        include_str!("../../../parity/grammars/native/javascript.lino"),
        include_str!("../../../parity/fixtures/native-grammars/javascript.json"),
    ),
    (
        "typescript",
        include_str!("../../../parity/grammars/native/typescript.lino"),
        include_str!("../../../parity/fixtures/native-grammars/typescript.json"),
    ),
    (
        "tsx",
        include_str!("../../../parity/grammars/native/tsx.lino"),
        include_str!("../../../parity/fixtures/native-grammars/tsx.json"),
    ),
    (
        "lean",
        include_str!("../../../parity/grammars/native/lean.lino"),
        include_str!("../../../parity/fixtures/native-grammars/lean.json"),
    ),
    (
        "rocq",
        include_str!("../../../parity/grammars/native/rocq.lino"),
        include_str!("../../../parity/fixtures/native-grammars/rocq.json"),
    ),
    (
        "java",
        include_str!("../../../parity/grammars/native/java.lino"),
        include_str!("../../../parity/fixtures/native-grammars/java.json"),
    ),
    (
        "go",
        include_str!("../../../parity/grammars/native/go.lino"),
        include_str!("../../../parity/fixtures/native-grammars/go.json"),
    ),
    (
        "regex",
        include_str!("../../../parity/grammars/native/regex.lino"),
        include_str!("../../../parity/fixtures/native-grammars/regex.json"),
    ),
    (
        "graphql",
        include_str!("../../../parity/grammars/native/graphql.lino"),
        include_str!("../../../parity/fixtures/native-grammars/graphql.json"),
    ),
    (
        "proto",
        include_str!("../../../parity/grammars/native/proto.lino"),
        include_str!("../../../parity/fixtures/native-grammars/proto.json"),
    ),
    (
        "make",
        include_str!("../../../parity/grammars/native/make.lino"),
        include_str!("../../../parity/fixtures/native-grammars/make.json"),
    ),
    (
        "solidity",
        include_str!("../../../parity/grammars/native/solidity.lino"),
        include_str!("../../../parity/fixtures/native-grammars/solidity.json"),
    ),
];

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-RECOVERY",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-recovery",
        fixture_file: "parity/fixtures/native-grammars/json.json",
        assertions,
        test_name,
    });
}

fn grammars() -> Vec<(&'static str, Value, FeatureGrammarParser)> {
    GRAMMARS
        .iter()
        .map(|(id, grammar, fixture)| {
            let fixture = serde_json::from_str(fixture).expect("the fixture is JSON");
            (*id, fixture, parser(grammar))
        })
        .collect()
}

fn recover() -> FeatureParseOptions {
    FeatureParseOptions {
        error_recovery: Some(true),
        ..FeatureParseOptions::default()
    }
}

fn parse(
    parser: &FeatureGrammarParser,
    source: &str,
    options: &FeatureParseOptions,
) -> ParseOutcome {
    parser
        .parse_tree(source.as_bytes(), options)
        .expect("the parse runs")
}

fn rendered(outcome: &ParseOutcome) -> String {
    outcome.tree.as_ref().expect("a tree").render()
}

// The bytes of the leaves in source order; a MISSING leaf covers no bytes.
fn leaf_bytes(tree: &SyntaxTree, source: &[u8], out: &mut Vec<u8>) {
    match tree {
        SyntaxTree::Node { children, .. } => {
            for child in children {
                leaf_bytes(child, source, out);
            }
        }
        SyntaxTree::Embed { root, .. } => leaf_bytes(root, source, out),
        SyntaxTree::Missing { start, end, .. } => assert_eq!(start, end),
        SyntaxTree::Token { start, end, .. } | SyntaxTree::Error { start, end, .. } => {
            out.extend_from_slice(&source[*start..*end]);
        }
    }
}

#[test]
fn native_grammars_repair_each_fixture_rejection_into_its_recorded_tree() {
    let mut repaired = 0;
    for (id, fixture, parser) in grammars() {
        let rejections = cases(&fixture, "rejections");
        assert!(!rejections.is_empty(), "{id}");
        for case in rejections {
            let source = source(case);
            let label = format!("{id} {source:?}");
            let recorded = case["recovered"].as_str().expect("recovered tree");
            let plain = parse(&parser, source, &FeatureParseOptions::default());
            assert!(!plain.ok, "{label}");
            assert!(plain.tree.is_none(), "{label}");
            let outcome = parse(&parser, source, &recover());
            assert!(!outcome.ok, "{label}");
            assert_eq!(
                outcome.rejection.as_ref().map(|rejection| rejection.reason),
                Some("recovered"),
                "{label}"
            );
            assert_eq!(rendered(&outcome), recorded, "{label}");
            let mut bytes = Vec::new();
            leaf_bytes(
                outcome.tree.as_ref().expect("a tree"),
                source.as_bytes(),
                &mut bytes,
            );
            assert_eq!(bytes, source.as_bytes(), "{label}");
            let accepted = parse(
                &parser,
                source,
                &FeatureParseOptions {
                    accept_recovery: Some(true),
                    ..recover()
                },
            );
            assert!(accepted.ok, "{label}");
            assert_eq!(rendered(&accepted), recorded, "{label}");
            repaired += 1;
        }
    }
    assert!(repaired >= 200, "{repaired} repaired rejections");
    observe(
        &[
            "nativeRecoveryTreesMatchFixtures",
            "nativeRecoveryTreesLossless",
            "nativeRecoveryReportedAsRecovered",
        ],
        "native_grammars_repair_each_fixture_rejection_into_its_recorded_tree",
    );
}

#[test]
fn automatic_recovery_leaves_the_trees_of_accepted_input_unchanged() {
    for (id, fixture, parser) in grammars() {
        for case in cases(&fixture, "matches")
            .iter()
            .chain(cases(&fixture, "divergences"))
        {
            let source = source(case);
            let plain = parse(&parser, source, &FeatureParseOptions::default());
            let outcome = parse(&parser, source, &recover());
            assert!(outcome.ok, "{id} {source:?}");
            assert_eq!(rendered(&outcome), rendered(&plain), "{id} {source:?}");
        }
    }
    observe(
        &["nativeRecoveryKeepsAcceptedTrees"],
        "automatic_recovery_leaves_the_trees_of_accepted_input_unchanged",
    );
}

#[test]
fn a_repair_inserts_a_missing_leaf_or_skips_input_as_an_error_leaf() {
    let json = parser(GRAMMARS[0].1);
    let tree =
        |source: &str, options: &FeatureParseOptions| rendered(&parse(&json, source, options));
    // A missing closing bracket is inserted where the rule it ends reduces
    // before the next token; as in tree-sitter, a MISSING leaf comes before
    // the white space after the token it follows.
    assert_eq!(
        tree(r#"{"a" : [1, 2 }"#, &recover()),
        r#"(document (object "{" (pair key:(string "\"" (string_content "a") "\"") ~" " ":" value:(array ~" " "[" (number "1") "," ~" " (number "2") (MISSING@12 "]") ~" ")) "}"))"#
    );
    assert_eq!(
        tree("[1, 2", &recover()),
        r#"(document (array "[" (number "1") "," ~" " (number "2") (MISSING@5 "]")))"#
    );
    // A stray value is skipped: deleting one byte costs less than inserting a comma.
    assert_eq!(
        tree("[1, 2 3]", &recover()),
        r#"(document (array "[" (number "1") "," ~" " (number "2") ~" " (ERROR@6..7 "3") "]"))"#
    );
    // With no repair point left, the rest of the input is one ERROR leaf.
    let none = FeatureParseOptions {
        max_repairs: Some(0),
        ..recover()
    };
    assert_eq!(
        tree("[1, 2 3]", &none),
        r#"(document (ERROR@0..8 "[1, 2 3]"))"#
    );
    // Without the option the parse is rejected with no tree.
    assert!(
        parse(&json, "[1, 2 3]", &FeatureParseOptions::default())
            .tree
            .is_none()
    );
}

#[test]
fn a_skip_never_ends_at_an_external_scanner_token() {
    // A string's content is a scanner token that reads to the next quote and
    // fails at the end of the input; a scan for a skip that ended at it would
    // run the scanner from every later offset, quadratic in the rest of the
    // input. As in tree-sitter, whose scanners refuse to run in error
    // recovery, no skip ends at such a token, so the repair stays within the
    // step budget and the stray no-break space is one ERROR.
    let rust = parser(GRAMMARS[8].1);
    let source = format!("let x;\n\u{a0}\n{}", "pub fn a() {}\n".repeat(64));
    let outcome = parse(&rust, &source, &recover());
    assert_eq!(
        outcome.rejection.as_ref().map(|rejection| rejection.reason),
        Some("recovered")
    );
    let tree = rendered(&outcome);
    let repairs: Vec<&str> = tree
        .match_indices("ERROR@")
        .chain(tree.match_indices("MISSING@"))
        .map(|(at, _)| tree[at..].split(' ').next().unwrap_or_default())
        .collect();
    assert_eq!(repairs, ["ERROR@7..9"]);
    assert_eq!(tree.matches("(function_item ").count(), 64);
    let mut bytes = Vec::new();
    leaf_bytes(
        outcome.tree.as_ref().expect("a tree"),
        source.as_bytes(),
        &mut bytes,
    );
    assert_eq!(bytes, source.as_bytes());
}

#[test]
fn a_later_error_is_repaired_where_it_is_not_by_skipping_the_input_after_an_earlier_one() {
    // With the stray comma as the only repair point, the cheapest complete
    // result takes the rest of the input as ERROR there; the result that
    // skips the comma reaches the stray bracket, so its end becomes the next
    // repair point and each stray token is one ERROR, as in tree-sitter.
    let rust = parser(GRAMMARS[8].1);
    let source = "struct S { a: u8,, }\nimpl S { fn f(&self) -> u8 { self.a } } ]\n";
    let repairs = |options: &FeatureParseOptions| -> Vec<String> {
        let tree = rendered(&parse(&rust, source, options));
        tree.split(['(', ' '])
            .filter(|part| part.starts_with("ERROR@") || part.starts_with("MISSING@"))
            .map(ToString::to_string)
            .collect()
    };
    assert_eq!(repairs(&recover()), ["ERROR@17..18", "ERROR@61..62"]);
    // When the rounds end first, the complete result stands.
    let one = FeatureParseOptions {
        max_repairs: Some(1),
        ..recover()
    };
    assert_eq!(repairs(&one), ["MISSING@17", "ERROR@17..62"]);
}

#[test]
fn a_round_that_completes_nothing_asks_for_the_offset_its_farthest_result_stopped_at() {
    // The first point, after `A.B`, completes nothing, as no MISSING leaf
    // there is followed by the rest of its rule; the module stopped at the
    // `.`, so that is the next point, and the `.` is one ERROR, as in
    // tree-sitter. A round that runs out of steps ends the rounds, the last
    // tree standing.
    let lean = parser(GRAMMARS[12].1);
    for (source, repair) in [
        (
            "\ndef foo.bar.baz := 12\n\ninductive A.B | C\n",
            "ERROR@35..36",
        ),
        (
            "\ndef foo.bar.baz := 12\n\nstructure Foo.Bar.Baz where\n  x : Nat\n\nclass Foo.Bar.Baz.Quux where\n  x : Nat\n\ninstance Spam.Eggs : Foo.Bar.Baz.Quux where\n  x := 3\n\ntheorem Foo.Bar.Baz.Quux.Cheese : 2 = 2 := rfl\n\ninductive A.B | C\n",
            "ERROR@37..222",
        ),
    ] {
        let outcome = parse(&lean, source, &recover());
        assert_eq!(
            outcome.rejection.as_ref().map(|rejection| rejection.reason),
            Some("recovered"),
            "{source:?}"
        );
        let tree = rendered(&outcome);
        let repairs: Vec<&str> = tree
            .split(['(', ' '])
            .filter(|part| part.starts_with("ERROR@") || part.starts_with("MISSING@"))
            .collect();
        assert_eq!(repairs, [repair], "{tree}");
    }
}

#[test]
fn a_repaired_result_preempts_no_cheaper_one_where_the_scanner_scans_a_token_of_no_width() {
    // An object that skips the stray `@9` (cost 2) goes on past `e` to its
    // closing brace; a statement block repaired after a MISSING `}` (more
    // cost) scans an automatic semicolon there, which no longer prunes the
    // cheaper object: as in tree-sitter, the stray token is one ERROR.
    let typescript = parser(GRAMMARS[10].1);
    for source in ["x = { c : 0 @9 , e } ;", "f ( { c : 0 @9 , e } ) ;"] {
        let tree = rendered(&parse(&typescript, source, &recover()));
        let repairs: Vec<&str> = tree
            .split(['(', ' '])
            .filter(|part| part.starts_with("ERROR@") || part.starts_with("MISSING@"))
            .collect();
        assert_eq!(repairs, ["ERROR@12..14"], "{tree}");
        assert!(
            tree.contains(r#"(shorthand_property_identifier "e") ~" " "}")"#),
            "{tree}"
        );
    }
}

#[test]
fn a_lexer_of_merged_lex_states_lexes_a_longer_token_no_item_of_its_state_takes() {
    // Tree-sitter merges the lex states of parse states whose tokens do not
    // conflict: after `0` only a member access `.` is valid, yet the lexer
    // lexes the number `.9`, as no property name begins with `9`, and the
    // error is the number, with no MISSING property name after a `.`. A `.` a
    // property name follows, or the `.` after the number `1.`, stays a member
    // access.
    let typescript = parser(GRAMMARS[10].1);
    for (source, offset, repair) in [
        ("x = 0 .9 ;", 6, "ERROR@6..8"),
        ("x = { c : 0 .9 , s : \"x\" } ;", 12, "ERROR@12..14"),
    ] {
        let outcome = parse(&typescript, source, &FeatureParseOptions::default());
        assert_eq!(
            outcome.rejection.and_then(|rejection| rejection.offset),
            Some(offset),
            "{source:?}"
        );
        let tree = rendered(&parse(&typescript, source, &recover()));
        let repairs: Vec<&str> = tree
            .split(['(', ' '])
            .filter(|part| part.starts_with("ERROR@") || part.starts_with("MISSING@"))
            .collect();
        assert_eq!(repairs, [repair], "{tree}");
    }
    for source in ["x = a . b ;", "x = 1..toString ( ) ;", "x = a ?. b ;"] {
        assert!(
            parse(&typescript, source, &FeatureParseOptions::default()).ok,
            "{source:?}"
        );
    }
}

#[test]
fn a_missing_token_is_inserted_only_where_a_rule_reduces_before_the_next_token() {
    // Tree-sitter inserts a missing token only if the lookahead reduces in the
    // state after it: no part of the rule the MISSING leaf ends takes input
    // after it. A `,` missing before the number `.92` would let the array go
    // on, so the number is skipped as an ERROR, with no MISSING `,` and no
    // `as_expression` that a MISSING `as` would build.
    let typescript = parser(GRAMMARS[10].1);
    for (source, repair) in [
        ("a = [ 0 .92 ] ;", "ERROR@8..11"),
        (r#"x = new Map ( [ [ "A" , 0 .92 ] , ] ) ;"#, "ERROR@26..29"),
    ] {
        let tree = rendered(&parse(&typescript, source, &recover()));
        let repairs: Vec<&str> = tree
            .split(['(', ' '])
            .filter(|part| part.starts_with("ERROR@") || part.starts_with("MISSING@"))
            .collect();
        assert_eq!(repairs, [repair], "{tree}");
        assert!(!tree.contains("as_expression"), "{tree}");
    }
}

#[test]
fn the_public_tree_has_no_node_for_a_missing_token_of_a_hidden_kind() {
    // A MISSING leaf of a hidden or anonymous kind is no node, as in
    // tree-sitter, though the node that has it has an error.
    let network = LinkNetwork::parse(
        "def f : Nat \u{2192} Nat\n  | 0 =>",
        "Lean",
        ParseConfiguration::default(),
    );
    assert_eq!(
        normalize(&render_network(&network, "Lean")),
        "(module (definition name: (identifier) type: (arrow domain: (identifier) codomain: (identifier)) (match_arm patterns: (number) (MISSING identifier))))"
    );
}

#[test]
fn a_missing_keyword_is_named_by_its_literal() {
    // A keyword token is its literal and a lookahead that no word character
    // follows; as tree-sitter names its keyword token, its MISSING leaf is
    // the literal.
    let rust = parser(GRAMMARS[8].1);
    let tree = rendered(&parse(&rust, "oc =c =>=", &recover()));
    assert!(
        tree.contains(r#"(return_expression (MISSING@7 "return"))"#),
        "{tree}"
    );
}

#[test]
fn a_long_repetition_repaired_near_its_end_keeps_every_item_in_order() {
    // A join links its parts instead of copying the children before it, so a
    // repetition of n items costs O(n) and not O(n²); the tree is unchanged.
    let json = parser(GRAMMARS[0].1);
    let count = 10_000;
    let items: Vec<String> = (0..count).map(|index| (index % 10).to_string()).collect();
    let source = format!("[{} 7]", items.join(","));
    let outcome = parse(&json, &source, &recover());
    assert_eq!(
        outcome.rejection.as_ref().map(|rejection| rejection.reason),
        Some("recovered")
    );
    let tree = outcome.tree.as_ref().expect("a tree");
    let SyntaxTree::Node { children, .. } = tree else {
        panic!("a document node");
    };
    let Some(SyntaxTree::Node {
        children: array, ..
    }) = children.first()
    else {
        panic!("an array node");
    };
    let numbers: Vec<(usize, usize)> = array
        .iter()
        .filter_map(|child| match child {
            SyntaxTree::Node {
                kind, start, end, ..
            }
            | SyntaxTree::Token {
                kind: Some(kind),
                start,
                end,
                ..
            } if kind == "number" => Some((*start, *end)),
            _ => None,
        })
        .collect();
    assert_eq!(numbers.len(), count);
    for (index, (start, end)) in numbers.iter().enumerate() {
        assert_eq!(&source[*start..*end], (index % 10).to_string());
    }
    assert!(matches!(array[array.len() - 2], SyntaxTree::Error { .. }));
    let mut bytes = Vec::new();
    leaf_bytes(tree, source.as_bytes(), &mut bytes);
    assert_eq!(bytes, source.as_bytes());
}
