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
