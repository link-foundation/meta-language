//! Requirement I195-GRAMMAR-NATIVE-C: the native merged C grammar,
//! parity/grammars/native/c.lino, which js/scripts/import-native-grammars.mjs
//! imports from the pinned tree-sitter-c grammar, builds the concrete syntax
//! trees of the tree-sitter-c oracle the native grammar replaced as the
//! default C parse. parity/fixtures/native-grammars/c.json holds the upstream
//! corpus with the oracle rows, which
//! js/scripts/generate-native-grammar-fixtures.mjs generates; this suite
//! projects the Rust executor's trees with `issue_195_native_grammar_rows.rs`
//! and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-c.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::{
    Rows, cases, leaves, oracle_agrees, parse, rebuilt, source, text,
};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/c.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/c.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/c.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-C",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-c",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native C fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

fn rows_of(rows: &Rows, parser: &FeatureGrammarParser, case: &Value) -> Value {
    let tree = parse(parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
    Value::Array(rows.rows(&tree, source(case)))
}

fn case<'a>(cases: &'a [Value], wanted: &str) -> &'a Value {
    cases
        .iter()
        .find(|case| source(case) == wanted)
        .unwrap_or_else(|| panic!("the {wanted:?} case"))
}

#[test]
fn native_c_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native C grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("translation_unit"));
    for rule in [
        "translation_unit",
        "function_definition",
        "declaration",
        "type_definition",
        "preprocessor_include",
        "preprocessor_definition",
        "preprocessor_if",
        "structure_specifier",
        "enumeration_specifier",
        "compound_statement",
        "if_statement",
        "for_statement",
        "while_statement",
        "do_statement",
        "return_statement",
        "expression",
        "binary_expression",
        "call_expression",
        "conditional_expression",
        "string_literal",
        "character_literal",
        "concatenated_string",
        "comment",
        "identifier",
        "primitive_type",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    assert!(GRAMMAR.trim_end().lines().all(|line| {
        ["(grammar ", "(extra ", "(conflict ", "(rule "]
            .iter()
            .any(|prefix| line.starts_with(prefix))
    }));
    // A renamed rule keeps its tree-sitter name only as a source-name alias.
    let foreign = regex::Regex::new(r" \(source-names(?: \([^()]*\))+\)")
        .expect("the source-names pattern compiles")
        .replace_all(GRAMMAR, "");
    let generator = regex::Regex::new(r"\bgrammar\.js\b").expect("the generator pattern compiles");
    assert!(
        !foreign.contains("tree-sitter")
            && !foreign.contains("(regex ")
            && !generator.is_match(&foreign)
    );
    observe(
        &["nativeCGrammarIsCanonicalLinks"],
        "native C grammar is canonical Links Notation",
    );
}

#[test]
fn native_c_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 100);
    for case in matches {
        assert_eq!(
            rows_of(&rows, &parser, case),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    // A keyword is an identifier only where the keyword itself cannot stand;
    // an anonymous alias such as `#include` is an anonymous row.
    assert_eq!(
        case(matches, "x = typedef;")["rows"],
        json!([
            [0, null, "translation_unit", 1, 0, 12, ""],
            [1, null, "expression_statement", 1, 0, 12, ""],
            [2, null, "assignment_expression", 1, 0, 11, ""],
            [3, "left", "identifier", 1, 0, 1, ""],
            [3, "operator", "=", 0, 2, 3, ""],
            [3, "right", "identifier", 1, 4, 11, ""],
            [2, null, ";", 0, 11, 12, ""]
        ])
    );
    assert_eq!(
        case(matches, "#include <stdio.h>\n")["rows"],
        json!([
            [0, null, "translation_unit", 1, 0, 19, ""],
            [1, null, "preproc_include", 1, 0, 19, ""],
            [2, null, "#include", 0, 0, 8, ""],
            [2, "path", "system_lib_string", 1, 9, 18, ""]
        ])
    );
    // The oracle's conflicts are settled as its generated parser settles
    // them: `a;` is a statement, not a declaration (a declared conflict of
    // silent rules); `#if` outranks a directive; a right-associative case
    // statement keeps the statements after it.
    for source in [
        "{ a; }",
        "#if A\n#endif\n",
        "{ case 1: a; b; }",
        "int x = sizeof(char * ());",
    ] {
        let outcome = parser
            .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(outcome.ambiguities.is_empty(), "{source:?}");
    }
    observe(
        &["nativeCTreesMatchOracle"],
        "native C grammar builds the oracle rows",
    );
}

#[test]
fn native_c_grammar_rejects_invalid_input() {
    let fixture = fixture();
    let parser = parser();
    assert!(cases(&fixture, "divergences").is_empty());
    let rejections = cases(&fixture, "rejections");
    assert!(rejections.len() >= 20);
    for case in rejections {
        let outcome = parser
            .parse_tree(source(case).as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(!outcome.ok, "{:?}", source(case));
        assert!(outcome.rejection.is_some(), "{:?}", source(case));
    }
    // A keyword is not an identifier where the keyword may begin a statement.
    for keyword in ["typedef;", "if;", "struct;", "while;"] {
        case(rejections, keyword);
    }
    observe(
        &["nativeCRejectsInvalidInput"],
        "native C grammar rejects invalid input",
    );
}

#[test]
fn native_c_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(
        &parser,
        "#include <stdio.h> /* c */\nint main(void) { return a ? \"s\\n\" : 'c'; } // d\n",
    )
    .expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted C tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (Some("'#include"), "#include"),
            (None, " "),
            (Some("system_library_string"), "<stdio.h>"),
            (None, " "),
            (Some("comment"), "/* c */"),
            (Some("unnamed_token"), "\n"),
            (Some("primitive_type"), "int"),
            (None, " "),
            (Some("identifier"), "main"),
            (None, "("),
            (Some("primitive_type"), "void"),
            (None, ")"),
            (None, " "),
            (None, "{"),
            (None, " "),
            (None, "return"),
            (None, " "),
            (Some("identifier"), "a"),
            (None, " "),
            (None, "?"),
            (None, " "),
            (None, "\""),
            (Some("string_content"), "s"),
            (Some("escape_sequence"), "\\n"),
            (None, "\""),
            (None, " "),
            (None, ":"),
            (None, " "),
            (None, "'"),
            (Some("character"), "c"),
            (None, "'"),
            (None, ";"),
            (None, " "),
            (None, "}"),
            (None, " "),
            (Some("comment"), "// d"),
            (None, "\n"),
        ]
    );
    observe(&["nativeCTreesLossless"], "native C trees keep every byte");
}

#[test]
fn pinned_c_oracle_gives_the_fixture() {
    // The oracle is a development dependency since the native grammar
    // replaced it as the default C parse.
    oracle_agrees(&tree_sitter_c::LANGUAGE.into(), &fixture());
}
