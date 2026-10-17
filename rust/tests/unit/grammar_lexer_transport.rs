//! Source-local lexer declaration transport; lexical decisions are generated
//! from the shared JavaScript module and tested separately.

use meta_language::grammar::import::{GrammarImportError, import_antlr};
use meta_language::grammar::interchange::{
    deserialize_grammar, parse_grammar_links, parse_native_grammar, render_grammar_links,
    render_native_grammar, serialize_grammar,
};
use meta_language::grammar::merge::rename_grammar_rule;
use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, GrammarMergeOptions, GrammarMergeSource, LeafText,
    SyntaxTree, compile_feature_grammar, merge_grammars,
};
use serde_json::Value;

const LISTING: &str = "start start\n\
lexer Source = seq(lexicalOwner(start, ref(start)), lexicalMode(quoted), lexicalChannel(CUSTOM), lexicalType(UNUSED), lexicalRule(ID, ID, default, default, emit, repeat1(ref(LETTER)), action()), lexicalFragment(LETTER, charRange(\"a\", \"z\")), lexicalBinding(ID, ref(ID)), lexicalLiteral(\"if\", KEYWORD), lexicalObligation(ID, predicate, \"unknownGuard()\"))\n\
rule start = normal lexicalScope(Source, start, seq(lexicalToken(Source, ID), not(any)))\n\
rule ID = token repeat1(ref(LETTER))\n\
rule LETTER = silent charRange(\"a\", \"z\")\n";

#[test]
fn lexical_declarations_preserve_all_fields_through_native_links_and_serialization() {
    // The JSON kind is charRange; both native listing codecs spell it range.
    // Retain every original authored byte as an exact syntax-refusal input.
    let original_error =
        parse_native_grammar(LISTING).expect_err("charRange is not native notation");
    let GrammarImportError::Parse { message, .. } = original_error else {
        panic!("the original listing must retain its syntax error");
    };
    assert_eq!(message, "line 2: unknown expression charRange");
    let canonical_listing = LISTING.replace("charRange(", "range(");
    let grammar =
        parse_native_grammar(&canonical_listing).expect("the canonical lexical listing parses");
    assert_eq!(grammar.declarations().present(), vec!["lexers"]);
    assert_eq!(grammar.declarations().lexers[0].name, "Source");
    let normalized = serialize_grammar(&grammar);
    let value: Value = serde_json::from_str(&normalized).expect("the serialization is JSON wire");
    let entries = value["declarations"]["lexers"][0]["expression"]["items"]
        .as_array()
        .expect("the lexical plan remains inspectable");
    assert_eq!(entries.len(), 9);
    assert_eq!(entries[4]["kind"], "lexicalRule");
    assert_eq!(entries[4]["disposition"], "emit");
    assert_eq!(entries[4]["item"]["item"]["name"], "LETTER");
    assert_eq!(entries[5]["item"]["kind"], "charRange");
    assert_eq!(entries[8]["source"], "unknownGuard()");
    assert_eq!(value["rules"][0]["expression"]["owner"], "start");
    for restored in [
        parse_native_grammar(&render_native_grammar(&grammar)).expect("native reload"),
        parse_grammar_links(&render_grammar_links(&grammar)).expect("Links reload"),
        deserialize_grammar(&normalized).expect("serialized reload"),
    ] {
        assert_eq!(restored, grammar);
        assert_eq!(serialize_grammar(&restored), normalized);
    }
}

#[test]
fn lexical_public_binding_renames_preserve_the_original_source_matcher_namespace() {
    let canonical_listing = LISTING.replace("charRange(", "range(");
    let grammar =
        parse_native_grammar(&canonical_listing).expect("the canonical lexical listing parses");
    let renamed = rename_grammar_rule(&grammar, "ID", "word", None, &[])
        .expect("rename public token")
        .grammar;
    let renamed = rename_grammar_rule(&renamed, "LETTER", "character", None, &[])
        .expect("rename public fragment")
        .grammar;
    let renamed = rename_grammar_rule(&renamed, "start", "document", None, &[])
        .expect("rename public parser owner")
        .grammar;
    let value: Value = serde_json::from_str(&serialize_grammar(&renamed)).expect("serialized");
    assert!(renamed.undefined_nonterminals().is_empty());
    let entries = &value["declarations"]["lexers"][0]["expression"]["items"];
    assert_eq!(entries[4]["name"], "ID");
    assert_eq!(entries[4]["item"]["item"]["name"], "LETTER");
    assert_eq!(entries[5]["name"], "LETTER");
    assert_eq!(entries[6]["name"], "ID");
    assert_eq!(entries[6]["item"]["name"], "word");
    assert_eq!(entries[0]["name"], "start");
    assert_eq!(entries[0]["item"]["name"], "document");
    assert_eq!(value["rules"][0]["name"], "document");
    assert_eq!(value["rules"][0]["expression"]["owner"], "start");
    assert_eq!(
        value["rules"][0]["expression"]["item"]["items"][0]["token"],
        "ID"
    );
    assert_eq!(value["rules"][1]["name"], "word");
    assert_eq!(value["rules"][1]["expression"]["item"]["name"], "character");
}

fn imported(source: &str) -> FeatureGrammarParser {
    let grammar = import_antlr(source).expect("the original authored ANTLR source imports");
    compile_feature_grammar(&grammar, None, FeatureParseOptions::default())
        .expect("the retained lexical plan compiles")
}

fn lexical_leaves(tree: &SyntaxTree) -> Vec<(String, String, bool, Value)> {
    match tree {
        SyntaxTree::Node { children, .. } => children.iter().flat_map(lexical_leaves).collect(),
        SyntaxTree::Token {
            kind,
            text: LeafText::Text(text),
            trivia,
            lexical,
            ..
        } => vec![(
            kind.clone().unwrap_or_default(),
            text.clone(),
            *trivia,
            lexical
                .clone()
                .expect("ANTLR token metadata remains explicit"),
        )],
        _ => Vec::new(),
    }
}

fn source_text(tree: &SyntaxTree) -> String {
    match tree {
        SyntaxTree::Node { children, .. } => children.iter().map(source_text).collect(),
        SyntaxTree::Token {
            text: LeafText::Text(text),
            ..
        }
        | SyntaxTree::Error {
            text: LeafText::Text(text),
            ..
        } => text.clone(),
        SyntaxTree::Missing { .. } => String::new(),
        other => panic!("the authored UTF-8 fixture has no other leaf form: {other:?}"),
    }
}

#[test]
fn lexical_election_retains_custom_channel_tokens_after_full_length_and_source_order() {
    let source = "grammar Election; channels { ERROR } start: WORD '=' WORD EOF; WORD: [a-z]+; EQUAL: '='; WS: [ ]+ -> channel(HIDDEN); Unexpected: . -> channel(ERROR);";
    let grammar = import_antlr(source).expect("the original source imports");
    for candidate in [
        grammar.clone(),
        parse_grammar_links(&render_grammar_links(&grammar)).expect("Links reload"),
        parse_native_grammar(&render_native_grammar(&grammar)).expect("native reload"),
        deserialize_grammar(&serialize_grammar(&grammar)).expect("serialized reload"),
    ] {
        let compiled = compile_feature_grammar(&candidate, None, FeatureParseOptions::default())
            .expect("each original plan loads");
        let result = compiled
            .parse_tree(b"left = right@", &FeatureParseOptions::default())
            .expect("the source has no unresolved operations");
        assert!(result.ok);
        let tree = result.tree.expect("the accepted result has a tree");
        assert_eq!(source_text(&tree), "left = right@");
        let leaves = lexical_leaves(&tree);
        assert_eq!(
            leaves
                .iter()
                .map(|(kind, text, trivia, _)| (kind.as_str(), text.as_str(), *trivia))
                .collect::<Vec<_>>(),
            vec![
                ("WORD", "left", false),
                ("WS", " ", true),
                ("=", "=", false),
                ("WS", " ", true),
                ("WORD", "right", false),
                ("Unexpected", "@", true)
            ]
        );
        assert_eq!(leaves[5].3["channel"], "ERROR");
        assert_eq!(leaves[5].3["disposition"], "emit");
        assert!(
            !compiled
                .parse_tree(b"left right", &FeatureParseOptions::default())
                .expect("syntax rejection")
                .ok
        );
    }
    for (source, accepted) in [
        (
            "grammar Election; start: A EOF; A: 'x'; H: 'x' -> channel(HIDDEN);",
            true,
        ),
        (
            "grammar Election; start: A EOF; H: 'x' -> channel(HIDDEN); A: 'x';",
            false,
        ),
        (
            "grammar Election; start: A A EOF; A: 'x'; H: 'xx' -> channel(HIDDEN);",
            false,
        ),
    ] {
        let input = if source.contains("'xx'") {
            b"xx".as_slice()
        } else {
            b"x".as_slice()
        };
        assert_eq!(
            imported(source)
                .parse_tree(input, &FeatureParseOptions::default())
                .expect("complete election")
                .ok,
            accepted
        );
    }
}

#[test]
fn lexical_election_preserves_modes_ordered_commands_and_superseded_aliases() {
    let parser = imported(
        "parser grammar Parser; options { tokenVocab = Election; } start: OPEN TEXT CLOSE EOF; lexer grammar Election; tokens { CLOSE } OPEN: '<' -> pushMode(STRING); OUTSIDE: [a-z]+; mode STRING; TEXT: [a-z]+; END: '>' -> type(CLOSE), popMode;",
    );
    let result = parser
        .parse_tree(b"<abc>", &FeatureParseOptions::default())
        .expect("mode operations execute");
    assert!(result.ok);
    assert_eq!(
        lexical_leaves(&result.tree.expect("tree"))
            .iter()
            .map(|(kind, text, _, metadata)| (
                kind.as_str(),
                text.as_str(),
                metadata["rule"].as_str().expect("source rule"),
                metadata["type"].as_str().expect("emitted type")
            ))
            .collect::<Vec<_>>(),
        vec![
            ("OPEN", "<", "OPEN", "OPEN"),
            ("TEXT", "abc", "TEXT", "TEXT"),
            ("CLOSE", ">", "END", "CLOSE")
        ]
    );
    assert!(
        !parser
            .parse_tree(b"abc", &FeatureParseOptions::default())
            .expect("wrong mode")
            .ok
    );
    assert!(
        !parser
            .parse_tree(b"<abc", &FeatureParseOptions::default())
            .expect("missing closing token")
            .ok
    );
    let source = "grammar Election; tokens { TEXT, WORD, UNUSED } start: WORD EOF; RAW: 'a' -> type(TEXT), type(WORD);";
    let grammar = import_antlr(source).expect("warning-bearing command sequence imports");
    assert!(grammar.rule("TEXT").is_some());
    assert!(grammar.rule("WORD").is_some());
    assert!(grammar.rule("UNUSED").is_none());
    assert!(
        imported(source)
            .parse_tree(b"a", &FeatureParseOptions::default())
            .expect("final emitted type")
            .ok
    );
    assert!(
        !imported(&source.replace("start: WORD", "start: TEXT"))
            .parse_tree(b"a", &FeatureParseOptions::default())
            .expect("superseded type rejection")
            .ok
    );
    assert!(
        imported(
            "grammar Election; tokens { WORD } start: WORD EOF; RAW: 'a' -> skip, type(WORD);"
        )
        .parse_tree(b"a", &FeatureParseOptions::default())
        .expect("later type restores emission")
        .ok
    );
    assert!(
        !imported(
            "grammar Election; tokens { WORD } start: WORD EOF; RAW: 'a' -> type(WORD), skip;"
        )
        .parse_tree(b"a", &FeatureParseOptions::default())
        .expect("later skip suppresses emission")
        .ok
    );
}

#[test]
fn lexical_election_keeps_unresolved_winners_distinct_from_syntax_recovery() {
    for source in [
        "grammar Election; start: A EOF; A: {guard()}? 'a';",
        "grammar Election; start: A EOF; A: 'a' {effect();};",
    ] {
        let error = imported(source)
            .parse_tree(b"a", &FeatureParseOptions::default())
            .expect_err("an unresolved winning operation refuses execution");
        assert_eq!(error.reason, "operation");
        assert!(error.message.contains("unresolved"));
    }
    assert!(
        imported("grammar Election; start: B EOF; B: 'b'; A: {guard()}? 'a';")
            .parse_tree(b"b", &FeatureParseOptions::default())
            .expect("a losing predicate is not executed")
            .ok
    );
    let result = imported("grammar Election; start: A EOF; A: 'a'; B: [a-z]+;")
        .parse_tree(
            b"ba",
            &FeatureParseOptions {
                error_recovery: Some(true),
                max_repairs: Some(2),
                ..FeatureParseOptions::default()
            },
        )
        .expect("recovery remains a parse outcome");
    assert!(!result.ok);
    assert_eq!(
        result.rejection.expect("recovery rejection").reason,
        "recovered"
    );
    let tree = result.tree.expect("a recovered tree remains lossless");
    assert_eq!(source_text(&tree), "ba");
    assert!(
        !lexical_leaves(&tree)
            .iter()
            .any(|(kind, _, _, _)| kind == "A")
    );
    let unbound =
        parse_native_grammar("start start\nrule start = normal lexicalToken(\"\", WORD)\n")
            .expect("the structural form is inspectable");
    assert_eq!(
        compile_feature_grammar(&unbound, None, FeatureParseOptions::default())
            .expect_err("an empty token scope never reaches an invalid map access")
            .reason,
        "declaration"
    );
}

#[test]
fn lexical_default_owners_and_empty_families_preserve_obligations_and_eof() {
    let source = "grammar Election; start: first EOF; first: A {effect();}; second: A; A: 'a';";
    let grammar = import_antlr(source).expect("original parser owner facts import");
    assert_eq!(
        grammar.rule("first").expect("original rule").expr,
        meta_language::GrammarExpr::NonTerminal("A".to_owned())
    );
    for candidate in [
        grammar.clone(),
        parse_grammar_links(&render_grammar_links(&grammar)).expect("Links reload"),
        parse_native_grammar(&render_native_grammar(&grammar)).expect("native reload"),
    ] {
        let compiled = compile_feature_grammar(&candidate, None, FeatureParseOptions::default())
            .expect("unresolved owners remain declarations until invoked");
        let error = compiled
            .parse_tree(b"a", &FeatureParseOptions::default())
            .expect_err("the invoked opaque owner refuses execution");
        assert_eq!(error.reason, "operation");
        assert!(
            error
                .message
                .contains("unresolved action in Election:first")
        );
        assert!(
            compiled
                .parse_tree(
                    b"a",
                    &FeatureParseOptions {
                        start_rule: Some("second".to_owned()),
                        ..FeatureParseOptions::default()
                    }
                )
                .expect("an uninvoked action does not affect the other owner")
                .ok
        );
    }
    let empty = import_antlr("grammar Empty; start: EOF;").expect("EOF-only source imports");
    for candidate in [
        empty.clone(),
        parse_grammar_links(&render_grammar_links(&empty)).expect("Links reload"),
        parse_native_grammar(&render_native_grammar(&empty)).expect("native reload"),
    ] {
        let compiled = compile_feature_grammar(&candidate, None, FeatureParseOptions::default())
            .expect("an empty election has no matcher program");
        let accepted = compiled
            .parse_tree(b"", &FeatureParseOptions::default())
            .expect("EOF election");
        assert!(accepted.ok);
        assert_eq!(
            source_text(&accepted.tree.expect("zero-width concrete root")),
            ""
        );
        assert!(
            !compiled
                .parse_tree(b"a", &FeatureParseOptions::default())
                .expect("unrecognized source remains rejected")
                .ok
        );
    }
}

#[test]
fn lexical_unsupported_options_retain_original_bytes_and_refuse_unknown_context() {
    let options = " superClass = FirstBase; /* retained π */ language = Java;\n";
    let grammar = import_antlr(&format!(
        "grammar Election; options {{{options}}} start: A EOF; A: 'a';"
    ))
    .expect("unsupported options remain inspectable");
    let other = import_antlr(&format!(
        "grammar Election; options {{{}}} start: A EOF; A: 'a';",
        options.replace("FirstBase", "SecondBase")
    ))
    .expect("other original context imports");
    assert_eq!(grammar.rule("A"), other.rule("A"));
    assert_ne!(grammar.declarations().lexers, other.declarations().lexers);
    for candidate in [
        grammar.clone(),
        parse_grammar_links(&render_grammar_links(&grammar)).expect("Links reload"),
        parse_native_grammar(&render_native_grammar(&grammar)).expect("native reload"),
    ] {
        let value: Value = serde_json::from_str(&serialize_grammar(&candidate)).expect("JSON wire");
        let obligations = value["declarations"]["lexers"][0]["expression"]["items"]
            .as_array()
            .expect("entries")
            .iter()
            .filter(|entry| entry["kind"] == "lexicalObligation")
            .collect::<Vec<_>>();
        assert_eq!(obligations.len(), 1);
        assert_eq!(obligations[0]["name"], "");
        assert_eq!(obligations[0]["category"], "option");
        assert_eq!(obligations[0]["source"], options);
        let error = compile_feature_grammar(&candidate, None, FeatureParseOptions::default())
            .expect_err("unknown grammar bases are never assumed equivalent at execution");
        assert_eq!(error.reason, "operation");
        assert!(error.message.contains("unresolved option"));
    }
    let parser_options = " tokenVocab = Election; superClass = ParserBase; ";
    let source = format!(
        "parser grammar Parser; options {{{parser_options}}} start: A EOF; lexer grammar Election; A: 'a';"
    );
    let grammar = import_antlr(&source).expect("parser context imports");
    let value: Value = serde_json::from_str(&serialize_grammar(&grammar)).expect("JSON wire");
    let obligation = value["declarations"]["lexers"][0]["expression"]["items"]
        .as_array()
        .expect("entries")
        .iter()
        .find(|entry| entry["kind"] == "lexicalObligation")
        .expect("the owner retains its original options");
    assert_eq!(obligation["name"], "start");
    assert_eq!(obligation["source"], parser_options);
    let error = imported(&source)
        .parse_tree(b"a", &FeatureParseOptions::default())
        .expect_err("invoking an unknown parser base refuses execution");
    assert_eq!(error.reason, "operation");
    assert!(
        error
            .message
            .contains("unresolved option in Election:start")
    );
}

#[test]
fn lexical_canonical_token_sharing_retains_original_parser_expectations() {
    for expected in ["A", "B"] {
        let source = format!("grammar Election; start: {expected} EOF; A: 'x'; B: 'x';");
        let grammar = import_antlr(&source).expect("original source imports");
        assert_eq!(
            imported(&source)
                .parse_tree(b"x", &FeatureParseOptions::default())
                .expect("original election")
                .ok,
            expected == "A"
        );
        let merged = merge_grammars(
            &[GrammarMergeSource::new("source", "Toy", grammar)],
            &GrammarMergeOptions::default(),
        )
        .expect("raw structural concepts can be shared");
        let group = &merged.groups[0];
        assert_eq!(group.identities["source:A"], group.identities["source:B"]);
        for candidate in [
            group.grammar.clone(),
            parse_grammar_links(&render_grammar_links(&group.grammar)).expect("Links reload"),
            parse_native_grammar(&render_native_grammar(&group.grammar)).expect("native reload"),
        ] {
            let compiled =
                compile_feature_grammar(&candidate, None, FeatureParseOptions::default())
                    .expect("source-qualified token expectations load");
            assert_eq!(
                compiled
                    .parse_tree(b"x", &FeatureParseOptions::default())
                    .expect("contextual election")
                    .ok,
                expected == "A"
            );
        }
    }
}

// This assertion binding is the existing JavaScript tooling selector, rather
// than another native implementation of reference-version applicability.
fn token_negation_reference() {
    static REFERENCE: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    let reference = REFERENCE.get_or_init(|| {
        let output = std::process::Command::new("node")
            .current_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".."))
            .args([
                "--input-type=module",
                "--eval",
                "import {selectAntlrReferenceAssertion} from './js/scripts/antlr-version-assertions.mjs'; process.stdout.write(JSON.stringify(selectAntlrReferenceAssertion()));",
            ])
            .output()
            .expect("the declared reference selector runs in native CI");
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        serde_json::from_slice(&output.stdout).expect("the actual selector returns its selected policy")
    });
    assert_eq!(reference["activeVersion"], "4.13.2");
    assert_eq!(reference["sourceVersion"], "ANTLR 4.13.2");
}

fn token_negation_baseline(source: &str) -> Value {
    use sha2::{Digest, Sha256};
    let fixture: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/grammar-parser-token-negation-controls.lino"
    ))
    .expect("the original importer snapshot is registered linked data");
    assert_eq!(
        fixture["baseline"]["revision"],
        "63e37589e9d7f37c5f1099651c1cbc702b7403d3"
    );
    let importer = &fixture["baseline"]["importer"];
    let original = importer["source"]
        .as_str()
        .expect("the exact original source is retained");
    assert_eq!(
        importer["bytes"].as_u64(),
        Some(u64::try_from(original.len()).unwrap())
    );
    assert_eq!(
        Sha256::digest(original.as_bytes()).iter().fold(
            String::with_capacity(64),
            |mut hexadecimal, byte| {
                use std::fmt::Write as _;
                write!(hexadecimal, "{byte:02x}").expect("writing into a String succeeds");
                hexadecimal
            }
        ),
        importer["sha256"].as_str().unwrap()
    );
    assert_eq!(
        importer["sha256"],
        "52b58caf89799535192131987e018aed71654cff19942290c965d2482e19eb29"
    );
    let cases = fixture["cases"]
        .as_array()
        .expect("all original input rows remain");
    assert_eq!(cases.len(), 4);
    let row = cases
        .iter()
        .find(|row| row["source"].as_str() == Some(source))
        .expect("the unchanged authored input has an exact original snapshot");
    assert_eq!(
        row["bytes"].as_u64(),
        Some(u64::try_from(source.len()).unwrap())
    );
    assert_eq!(
        Sha256::digest(source.as_bytes()).iter().fold(
            String::with_capacity(64),
            |mut hexadecimal, byte| {
                use std::fmt::Write as _;
                write!(hexadecimal, "{byte:02x}").expect("writing into a String succeeds");
                hexadecimal
            }
        ),
        row["sha256"].as_str().unwrap()
    );
    row["normalized"].clone()
}

fn token_negation_replays(grammar: &meta_language::Grammar) -> Vec<meta_language::Grammar> {
    use meta_language::{
        BinaryLinksOptions, FromLinks, Grammar, LinkNetwork, LinksDecoder, LinksEncoder, ToLinks,
        decode_link_network_binary, encode_link_network_binary,
    };
    // The normalized wire intentionally has no legacy rule documentation.
    // Its actual lexical declarations must suffice after source-free reload.
    let normalized = serialize_grammar(grammar);
    let without_docs = deserialize_grammar(&normalized).expect("all normalized fields reload");
    let mut encoder = LinksEncoder::new();
    let root = without_docs.to_links(&mut encoder);
    let network = encoder.into_network();
    let text = LinkNetwork::from_lino(&network.to_lino()).expect("full graph text reload");
    let bytes = encode_link_network_binary(&text, BinaryLinksOptions::default())
        .expect("the complete grammar graph encodes");
    let binary = decode_link_network_binary(&bytes, BinaryLinksOptions::default())
        .expect("the complete grammar graph reloads");
    let restored = Grammar::from_links(&mut LinksDecoder::new(&binary), root)
        .expect("the source-free graph decodes with the same root");
    let candidates = vec![
        grammar.clone(),
        parse_grammar_links(&render_grammar_links(grammar)).expect("grammar Links reload"),
        restored,
    ];
    for candidate in &candidates {
        assert_eq!(serialize_grammar(candidate), normalized);
    }
    candidates
}

fn token_negation_rows(tree: &SyntaxTree) -> Vec<(String, String, usize, usize, bool)> {
    match tree {
        SyntaxTree::Node { children, .. } => {
            children.iter().flat_map(token_negation_rows).collect()
        }
        SyntaxTree::Token {
            kind,
            text: LeafText::Text(text),
            start,
            end,
            trivia,
            lexical,
            ..
        } => {
            assert!(
                lexical.is_some(),
                "every elected token retains its source-local metadata"
            );
            vec![(
                kind.clone().unwrap_or_default(),
                text.clone(),
                *start,
                *end,
                *trivia,
            )]
        }
        _ => Vec::new(),
    }
}

fn token_negation_declaration(body: &str) -> String {
    format!(
        "grammar Exclusion; start: ({body})+ EOF;\n  STOP: 'stop'; ALT: 'a'; LONG: 'alpha'; RETYPE: 'x' -> type(STOP);\n  WORD: [a-z]+; TEXT: [é]+; WS: [ ]+ -> channel(HIDDEN);"
    )
}

// An independent oracle for these exact authored fixtures, not an executed
// ANTLR oracle: complete lexical words are elected before token exclusion.
fn token_negation_oracle(input: &str) -> Option<Vec<(String, String, usize, usize, bool)>> {
    let mut rows = Vec::new();
    let mut start = 0;
    while start < input.len() {
        let first = input[start..]
            .chars()
            .next()
            .expect("a complete UTF-8 character");
        if first != ' ' && first != 'é' && !first.is_ascii_lowercase() {
            return None;
        }
        let mut end = start;
        for character in input[start..].chars() {
            if !(if first == ' ' {
                character == ' '
            } else if first == 'é' {
                character == 'é'
            } else {
                character.is_ascii_lowercase()
            }) {
                break;
            }
            end += character.len_utf8();
        }
        let text = &input[start..end];
        let kind = match text {
            "stop" | "x" | "a" => return None,
            "alpha" => "LONG",
            _ if first == ' ' => "WS",
            _ if first == 'é' => "TEXT",
            _ => "WORD",
        };
        rows.push((kind.to_owned(), text.to_owned(), start, end, kind == "WS"));
        start = end;
    }
    rows.iter().any(|row| !row.4).then_some(rows)
}

fn token_negation_assert_inputs(grammar: &meta_language::Grammar, inputs: &[&str]) {
    let parser = compile_feature_grammar(grammar, None, FeatureParseOptions::default())
        .expect("all retained lexical declarations compile");
    for input in inputs {
        let expected = token_negation_oracle(input);
        let result = parser
            .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
            .expect("all authored operations are resolved");
        assert_eq!(result.ok, expected.is_some(), "{input:?}");
        if let Some(expected) = expected {
            let tree = result
                .tree
                .expect("the accepted parse has its concrete tree");
            assert_eq!(token_negation_rows(&tree), expected, "{input:?}");
            assert_eq!(source_text(&tree), *input);
            assert_eq!(tree.end(), input.len());
        }
    }
}

#[test]
fn parser_token_negation_keeps_the_original_lexer_complement_fields_exact() {
    token_negation_reference();
    let source = include_str!("../fixtures/grammar/antlr/set-complement.g4");
    let grammar = import_antlr(source).expect("the original lexer complement still imports");
    let normalized: Value = serde_json::from_str(&serialize_grammar(&grammar)).unwrap();
    assert_eq!(normalized, token_negation_baseline(source));
}

#[test]
fn parser_token_negation_consumes_complete_words_utf8_spans_hidden_trivia_and_retypes() {
    token_negation_reference();
    let expected = vec![
        ("WS".to_owned(), " ".to_owned(), 0, 1, true),
        ("LONG".to_owned(), "alpha".to_owned(), 1, 6, false),
        ("WS".to_owned(), " ".to_owned(), 6, 7, true),
        ("WORD".to_owned(), "beta".to_owned(), 7, 11, false),
        ("WS".to_owned(), " ".to_owned(), 11, 12, true),
        ("TEXT".to_owned(), "é".to_owned(), 12, 14, false),
        ("WS".to_owned(), " ".to_owned(), 14, 15, true),
    ];
    assert_eq!(token_negation_oracle(" alpha beta é "), Some(expected));
    for excluded in ["~(STOP | ALT)", "~('stop' | 'a')"] {
        let grammar =
            import_antlr(&token_negation_declaration(excluded)).expect("authored source imports");
        for candidate in token_negation_replays(&grammar) {
            token_negation_assert_inputs(
                &candidate,
                &[
                    " alpha beta é ",
                    "alpha",
                    "stopper",
                    "beta",
                    "éé",
                    "",
                    " ",
                    "a",
                    "stop",
                    "x",
                    "alpha stop",
                    "@",
                ],
            );
        }
    }
    let parser = imported("grammar One; start: ~'a' EOF; A:'a'; LONG:'alpha';");
    for (input, accepted) in [("alpha", true), ("a", false), ("", false)] {
        assert_eq!(
            parser
                .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
                .expect("one complete token")
                .ok,
            accepted
        );
    }
}

#[test]
fn parser_eof_exclusions_consume_once_and_keep_insensitive_literals_unsupported() {
    token_negation_reference();
    let grammar =
        import_antlr("grammar End; start: ~EOF A EOF; A:'a';").expect("authored EOF exclusion");
    for candidate in token_negation_replays(&grammar) {
        let parser =
            compile_feature_grammar(&candidate, None, FeatureParseOptions::default()).unwrap();
        for input in ["", "a", "aa", "aaa"] {
            let result = parser
                .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
                .unwrap();
            assert_eq!(result.ok, input == "aa", "{input}");
            if result.ok {
                assert_eq!(
                    token_negation_rows(&result.tree.unwrap()),
                    vec![
                        ("A".to_owned(), "a".to_owned(), 0, 1, false),
                        ("A".to_owned(), "a".to_owned(), 1, 2, false),
                    ]
                );
            }
        }
    }
    let grammar = import_antlr(&token_negation_declaration("~(EOF | STOP | ALT)")).unwrap();
    for candidate in token_negation_replays(&grammar) {
        token_negation_assert_inputs(
            &candidate,
            &["alpha", " alpha beta é ", "", " ", "stop", "x", "a"],
        );
    }
    let error = imported(
        "grammar Case; options {caseInsensitive=true;} start: ~'a' EOF; A:'a'; WORD:[a-z]+;",
    )
    .parse_tree(b"b", &FeatureParseOptions::default())
    .expect_err("insensitive parser literals still require a lexical binding proof");
    assert!(
        error
            .message
            .contains("unsupported parser terminal in lexer")
    );
}

#[test]
fn parser_token_negation_rolls_back_mode_commands_and_preserves_original_obligations() {
    token_negation_reference();
    for (source, input, expected) in [
        (
            "parser grammar Parser; options {tokenVocab=Mode;} start: ~BLOCK TEXT CLOSE EOF;\n    lexer grammar Mode; OPEN:'<' -> pushMode(INNER); BLOCK:'x';\n    mode INNER; TEXT:[a-z]+; CLOSE:'>' -> popMode;",
            "<word>",
            vec![("OPEN", "<"), ("TEXT", "word"), ("CLOSE", ">")],
        ),
        (
            "parser grammar Parser; options {tokenVocab=Mode;} start: ~BLOCK TEXT CLOSE TAIL EOF;\n    lexer grammar Mode; OPEN:'<' -> pushMode(INNER); BLOCK:'x'; TAIL:'!';\n    mode INNER; TEXT:[a-z]+; CLOSE:'>' -> popMode;",
            "<word>!",
            vec![
                ("OPEN", "<"),
                ("TEXT", "word"),
                ("CLOSE", ">"),
                ("TAIL", "!"),
            ],
        ),
    ] {
        let grammar = import_antlr(source).unwrap();
        for candidate in token_negation_replays(&grammar) {
            let parser =
                compile_feature_grammar(&candidate, None, FeatureParseOptions::default()).unwrap();
            let result = parser
                .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
                .unwrap();
            assert!(result.ok);
            let tree = result.tree.unwrap();
            let leaves = lexical_leaves(&tree);
            assert_eq!(
                leaves
                    .iter()
                    .map(|(kind, text, _, _)| (kind.as_str(), text.as_str()))
                    .collect::<Vec<_>>(),
                expected
            );
            assert_eq!(source_text(&tree), input);
        }
    }
    for source in [
        "grammar Foreign; options {superClass=Missing;} start: ~BLOCK EOF; BLOCK:'x'; WORD:[a-z]+;",
        "grammar Foreign; start: {allow()}? ~BLOCK EOF; BLOCK:'x'; WORD:[a-z]+;",
    ] {
        let before = token_negation_baseline(source);
        let grammar = import_antlr(source).unwrap();
        let after: Value = serde_json::from_str(&serialize_grammar(&grammar)).unwrap();
        let obligations = |normalized: &Value| {
            normalized["declarations"]["lexers"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|lexer| lexer["expression"]["items"].as_array().unwrap())
                .filter(|entry| entry["kind"] == "lexicalObligation")
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(obligations(&after), obligations(&before));
        for candidate in token_negation_replays(&grammar) {
            let error = compile_feature_grammar(&candidate, None, FeatureParseOptions::default())
                .and_then(|parser| parser.parse_tree(b"word", &FeatureParseOptions::default()))
                .expect_err("an unresolved foreign operation never gains support");
            assert!(
                error.message.contains("unresolved option")
                    || error.message.contains("unresolved predicate")
            );
        }
    }
    let source = include_str!("../fixtures/grammar/antlr/covering.g4");
    let before = token_negation_baseline(source);
    let after: Value =
        serde_json::from_str(&serialize_grammar(&import_antlr(source).unwrap())).unwrap();
    for old in before["rules"].as_array().unwrap() {
        let name = old["name"].as_str().unwrap();
        let new = after["rules"]
            .as_array()
            .unwrap()
            .iter()
            .find(|rule| rule["name"] == name)
            .unwrap();
        if name != "item" {
            assert_eq!(new, old, "{name}");
        } else {
            assert_eq!(
                &new["expression"]["items"].as_array().unwrap()[..2],
                &old["expression"]["items"].as_array().unwrap()[..2]
            );
            assert_eq!(
                new["expression"]["items"][2],
                serde_json::json!({"kind":"seq","items":[{"kind":"not","item":{"kind":"literal","value":"x"}},{"kind":"any"}]})
            );
        }
    }
}

#[test]
fn parser_token_exclusion_emission_reimports_the_same_tokens_eof_and_following_wildcard() {
    use meta_language::{FeatureExpr, FieldValue, GrammarEmitError, GrammarExpr, GrammarFormat};
    token_negation_reference();
    for excluded in ["~(STOP | ALT)", "~('stop' | 'a')", "~(EOF | STOP | ALT)"] {
        let grammar = import_antlr(&token_negation_declaration(excluded)).unwrap();
        for candidate in token_negation_replays(&grammar) {
            let (source, _) =
                meta_language::emit_antlr(&candidate).expect("the exact supported exclusion emits");
            let reimported = import_antlr(&source).expect("emitted ANTLR reimports");
            token_negation_assert_inputs(
                &reimported,
                &[
                    "alpha",
                    " alpha beta é ",
                    "",
                    " ",
                    "stop",
                    "x",
                    "a",
                    "stopper",
                    "@",
                ],
            );
        }
    }
    for (source, accepted, refused) in [
        (
            "grammar End; start: ~EOF A EOF; A:'a';",
            vec!["aa"],
            vec!["", "a", "aaa"],
        ),
        (
            "grammar Pair; start: ~BLOCK . EOF; BLOCK:'x'; A:'a';",
            vec!["aa", "ax"],
            vec!["", "a", "xa", "aaa"],
        ),
    ] {
        let grammar = import_antlr(source).unwrap();
        for candidate in token_negation_replays(&grammar) {
            let (source, _) = meta_language::emit_antlr(&candidate).unwrap();
            let parser = imported(&source);
            for input in &accepted {
                let result = parser
                    .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
                    .unwrap();
                assert!(result.ok, "{input}");
                let tree = result.tree.unwrap();
                assert_eq!(source_text(&tree), *input);
                assert_eq!(
                    lexical_leaves(&tree).len(),
                    2,
                    "the extra wildcard remains a separate terminal"
                );
            }
            for input in &refused {
                assert!(
                    !parser
                        .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
                        .unwrap()
                        .ok,
                    "{input}"
                );
            }
        }
    }
    let raw = import_antlr(&token_negation_declaration("~(STOP | ALT)")).unwrap();
    let normalized = deserialize_grammar(&serialize_grammar(&raw)).unwrap();
    for mut grammar in [raw, normalized] {
        let mut declarations = grammar.declarations().clone();
        let GrammarExpr::Sequence(entries) = &mut declarations.lexers[0].expression else {
            panic!("the authored lexer has its actual ordered declaration list")
        };
        let first = entries
            .iter_mut()
            .find_map(|entry| match entry {
                GrammarExpr::Feature(feature) => match feature.as_mut() {
                    FeatureExpr::Form(form) if form.head == "lexicalRule" => Some(form),
                    _ => None,
                },
                _ => None,
            })
            .expect("the authored first token has a real feature form");
        let item = first
            .spec()
            .iter()
            .position(|(name, _)| *name == "item")
            .unwrap();
        first.fields[item] = FieldValue::Text("not an expression".to_owned());
        assert!(first.expression("item").is_none());
        grammar.set_declarations(declarations);
        assert_eq!(
            meta_language::emit_antlr(&grammar).unwrap_err(),
            GrammarEmitError::Unsupported {
                format: GrammarFormat::Antlr,
                construct: "source-free lexical commands: invalid lexical declaration descriptors"
                    .to_owned(),
            }
        );
    }
}
