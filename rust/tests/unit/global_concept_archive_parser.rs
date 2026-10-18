//! Archive parser parity controls; native execution belongs to CI.

use std::collections::BTreeMap;

use meta_language::{
    FeatureParseOptions, FormalConceptDefinitionInput, GlobalConceptOptions, GlobalConceptSource,
    GlobalConceptSpaceParser, Grammar, GrammarFormat, GrammarSelection, LeafText, OperationValue,
    ParseOutcome, ParseWithGrammarError, SyntaxAttributes, SyntaxTree, compile_feature_grammar,
    create_global_concept_space, create_global_concept_space_parser,
    encode_formal_concept_definition, global_concept_space_to_links, parse_grammar_links,
    read_global_concept_space_links, read_structured_data_lino, structured_data_to_lino,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::Rows;

fn selection_fixture() -> Value {
    super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/grammar-recursive-selection.lino"
    ))
    .unwrap()
}

fn archive_settings() -> GlobalConceptOptions {
    GlobalConceptOptions {
        records: Some(json!([])),
        ..GlobalConceptOptions::default()
    }
}

fn archived_source(id: &str, listing: &str) -> GlobalConceptSource {
    GlobalConceptSource {
        id: id.to_owned(),
        language: id.to_owned(),
        edition: "authored-1".to_owned(),
        grammar: parse_grammar_links(listing).unwrap(),
    }
}

fn archive_sources(sources: &[GlobalConceptSource]) -> String {
    global_concept_space_to_links(
        &create_global_concept_space(sources, &archive_settings()).unwrap(),
    )
    .unwrap()
}

fn archive_parser(archive: &str, id: &str) -> GlobalConceptSpaceParser {
    create_global_concept_space_parser(
        archive,
        id,
        &archive_settings(),
        FeatureParseOptions::default(),
    )
    .unwrap()
}

fn archived_recursive_fixture() -> (String, Value, ParseOutcome, Value) {
    let fixture = selection_fixture();
    let chain = fixture["chain"].clone();
    let mut sources = fixture["grammars"]
        .as_array()
        .unwrap()
        .iter()
        .take(4)
        .map(|entry| {
            archived_source(
                entry["name"].as_str().unwrap(),
                entry["listing"].as_str().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut grammars = sources
        .iter()
        .map(|source| (source.id.clone(), source.grammar.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut space = create_global_concept_space(&sources, &archive_settings()).unwrap();
    let root = chain["root"].as_str().unwrap();
    let resolve = |name: &str| grammars.get(name).cloned();
    let expected = compile_feature_grammar(
        &grammars[root],
        Some(&resolve),
        FeatureParseOptions {
            grammar_name: Some(root.to_owned()),
            global_concept_space: Some(space.clone()),
            ..FeatureParseOptions::default()
        },
    )
    .unwrap()
    .parse_tree(
        chain["source"].as_str().unwrap().as_bytes(),
        &FeatureParseOptions::default(),
    )
    .unwrap();
    assert!(expected.ok, "{:?}", expected.rejection);
    let archive = global_concept_space_to_links(&space).unwrap();
    let mappings = space["mappings"].clone();
    grammars.clear();
    sources.clear();
    space["sources"] = json!([]);
    space["concepts"] = json!([]);
    (archive, mappings, expected, chain)
}

fn embedded_trees<'a>(tree: &'a SyntaxTree, found: &mut Vec<&'a SyntaxTree>) {
    match tree {
        SyntaxTree::Node { children, .. } => {
            for child in children {
                embedded_trees(child, found);
            }
        }
        SyntaxTree::Embed { root, .. } => {
            found.push(tree);
            embedded_trees(root, found);
        }
        _ => {}
    }
}

fn selection(tree: &SyntaxTree) -> &GrammarSelection {
    let SyntaxTree::Embed {
        selection: Some(selection),
        ..
    } = tree
    else {
        panic!("the qualified invocation retains its selection")
    };
    selection
}

fn attributes_data(attributes: Option<&SyntaxAttributes>) -> Value {
    attributes.map_or(Value::Null, |attributes| {
        Value::Object(
            attributes
                .iter()
                .map(|(key, value)| {
                    let value = match value {
                        OperationValue::Integer(number) => json!(number),
                        OperationValue::Text(text) => json!(text),
                    };
                    (key.clone(), value)
                })
                .collect(),
        )
    })
}

fn leaf_data(text: &LeafText) -> Value {
    match text {
        LeafText::Text(text) => json!({"text":text}),
        LeafText::Hex(hex) => json!({"text":null,"hex":hex}),
    }
}

// A test-only structural transport records every public tree field. The
// production parser continues to return the existing SyntaxTree directly.
fn tree_data(tree: &SyntaxTree) -> Value {
    match tree {
        SyntaxTree::Node {
            kind,
            field,
            trivia,
            start,
            end,
            children,
            attributes,
        } => json!({"type":"node","kind":kind,"field":field,"trivia":trivia,
                "start":start,"end":end,"children":children.iter().map(tree_data).collect::<Vec<_>>(),
                "attributes":attributes_data(attributes.as_ref())}),
        SyntaxTree::Token {
            kind,
            field,
            trivia,
            start,
            end,
            text,
            attributes,
            lexical,
        } => json!({"type":"token","kind":kind,"field":field,"trivia":trivia,
                "start":start,"end":end,"leaf":leaf_data(text),
                "attributes":attributes_data(attributes.as_ref()),"lexical":lexical}),
        SyntaxTree::Error {
            start,
            end,
            text,
            reason,
        } => json!({"type":"error","start":start,"end":end,"leaf":leaf_data(text),"reason":reason}),
        SyntaxTree::Missing {
            kind,
            start,
            end,
            literal,
        } => json!({"type":"missing","kind":kind,"start":start,"end":end,"literal":literal}),
        SyntaxTree::Embed {
            language,
            start,
            end,
            root,
            selection,
            ..
        } => {
            let selection = selection.as_ref().map(|selected| {
                json!({"mode":selected.mode,"mapping":selected.mapping,"start":selected.start,
                    "end":selected.end,"candidates":selected.candidates,"provenance":selected.provenance,
                    "ambiguous":selected.ambiguous,"alternatives":selected.alternatives.iter().map(|item|
                        json!({"language":item.language,"root":tree_data(&item.root),"provenance":item.provenance})
                    ).collect::<Vec<_>>()})
            });
            json!({"type":"embed","language":language,"start":start,"end":end,
                "root":tree_data(root),"selection":selection})
        }
    }
}

fn archived_tree_bytes(tree: &Value) -> Vec<u8> {
    match tree["type"].as_str().unwrap() {
        "missing" => Vec::new(),
        "token" | "error" => tree["leaf"]["text"].as_str().map_or_else(
            || {
                tree["leaf"]["hex"]
                    .as_str()
                    .unwrap()
                    .as_bytes()
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
                    .collect()
            },
            |text| text.as_bytes().to_vec(),
        ),
        "node" => tree["children"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(archived_tree_bytes)
            .collect(),
        "embed" => archived_tree_bytes(&tree["root"]),
        other => panic!("unexpected stored tree type {other}"),
    }
}

#[test]
fn validated_archive_replays_recursive_cst_after_original_grammars_are_discarded() {
    let (archive, mappings, expected, chain) = archived_recursive_fixture();
    let parser = archive_parser(&archive, chain["root"].as_str().unwrap());
    let source = chain["source"].as_str().unwrap().as_bytes();
    let actual = parser
        .parse_tree(source, &FeatureParseOptions::default())
        .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(
        parser
            .parse(source, &FeatureParseOptions::default())
            .unwrap(),
        *expected.tree.as_ref().unwrap()
    );
    let tree = actual.tree.as_ref().unwrap();
    let mut embedded = Vec::new();
    embedded_trees(tree, &mut embedded);
    let languages = embedded
        .iter()
        .map(|tree| {
            let SyntaxTree::Embed { language, .. } = tree else {
                unreachable!()
            };
            let provenance = &selection(tree).provenance;
            assert_eq!(provenance["edition"], chain["edition"]);
            let origin = format!("{language}:document");
            let definition = provenance["rules"]
                .as_array()
                .unwrap()
                .iter()
                .find(|rule| rule["origin"] == origin)
                .unwrap();
            assert_eq!(definition["concept"], mappings[&origin]);
            language.clone()
        })
        .collect::<Vec<_>>();
    assert_eq!(json!(languages), chain["languages"]);
    assert_eq!(
        selection(embedded[0]).provenance["rules"][0]["concept"],
        selection(embedded[3]).provenance["rules"][0]["concept"]
    );
    let original_data = tree_data(tree);
    let stored =
        read_structured_data_lino(&structured_data_to_lino(&original_data).unwrap()).unwrap();
    assert_eq!(stored, original_data);
    assert_eq!(archived_tree_bytes(&stored), source);
    let decoded = read_global_concept_space_links(&archive, &archive_settings()).unwrap();
    assert!(decoded.get("grammars").is_none());
    assert!(
        decoded["sources"]
            .as_array()
            .unwrap()
            .iter()
            .all(|source| source.get("grammar").is_none())
    );
    assert_eq!(global_concept_space_to_links(&decoded).unwrap(), archive);
}

#[test]
fn exact_archived_source_ids_and_authority_options_remain_protected() {
    let sources = ["one", "two"]
        .iter()
        .enumerate()
        .map(|(index, id)| GlobalConceptSource {
            id: (*id).to_owned(),
            language: "same-language".to_owned(),
            edition: (index + 1).to_string(),
            grammar: parse_grammar_links(&format!(
                "(grammar (start document))\n(rule document normal (literal {id}))\n"
            ))
            .unwrap(),
        })
        .collect::<Vec<_>>();
    let archive = archive_sources(&sources);
    for (id, edition) in [("one", "1"), ("two", "2")] {
        let actual = archive_parser(&archive, id)
            .parse_tree(id.as_bytes(), &FeatureParseOptions::default())
            .unwrap();
        assert!(actual.ok, "{:?}", actual.rejection);
        let tree = actual.tree.unwrap();
        assert!(matches!(&tree, SyntaxTree::Embed { language, .. } if language == id));
        assert_eq!(selection(&tree).provenance["edition"], edition);
    }
    for id in ["same-language", "ONE", ""] {
        let error = create_global_concept_space_parser(
            &archive,
            id,
            &archive_settings(),
            FeatureParseOptions::default(),
        )
        .unwrap_err();
        assert_eq!(error.reason, "selection");
        assert_eq!(
            error.message,
            "an archived grammar needs an exact existing source identifier"
        );
    }
    // Typed options have no resolver argument and represent own presence with
    // Some; Some(null) and Some("") remain protected rather than absent.
    for (options, name) in [
        (
            FeatureParseOptions {
                global_concept_space: Some(Value::Null),
                ..FeatureParseOptions::default()
            },
            "globalConceptSpace",
        ),
        (
            FeatureParseOptions {
                grammar_name: Some(String::new()),
                ..FeatureParseOptions::default()
            },
            "grammarName",
        ),
    ] {
        let message = format!("the archived global concept space owns parser option {name}");
        let error = create_global_concept_space_parser(
            &archive,
            "one",
            &archive_settings(),
            options.clone(),
        )
        .unwrap_err();
        assert_eq!(error.reason, "globalConceptSpace");
        assert_eq!(error.message, message);
        // Guard ordering is unchanged: authority refusal precedes archive read.
        assert_eq!(
            create_global_concept_space_parser(
                "invalid",
                "one",
                &archive_settings(),
                options.clone()
            )
            .unwrap_err()
            .message,
            message
        );
        let parser = archive_parser(&archive, "one");
        assert_eq!(
            parser.parse_tree(b"one", &options).unwrap_err().message,
            message
        );
        let ParseWithGrammarError::Runtime(error) = parser.parse(b"one", &options).unwrap_err()
        else {
            panic!("protected options are runtime errors")
        };
        assert_eq!(error.message, message);
    }
    assert!(matches!(
        archive_parser(&archive, "one").parse(b"two", &FeatureParseOptions::default()),
        Err(ParseWithGrammarError::Parse(_))
    ));
}

#[test]
fn archived_any_all_selections_keep_witnesses_and_unknown_diagnostics() {
    let fixture = selection_fixture();
    let mut sources = fixture["grammars"]
        .as_array()
        .unwrap()
        .iter()
        .skip(4)
        .map(|source| {
            archived_source(
                source["name"].as_str().unwrap(),
                source["listing"].as_str().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    for name in ["any", "all", "allRejected", "unknown", "escaped"] {
        sources.push(archived_source(
            name,
            fixture[name]["listing"].as_str().unwrap(),
        ));
    }
    let archive = archive_sources(&sources);
    for name in ["any", "all"] {
        let actual = archive_parser(&archive, name)
            .parse_tree("π".as_bytes(), &FeatureParseOptions::default())
            .unwrap();
        assert!(actual.ok, "{:?}", actual.rejection);
        let mut embedded = Vec::new();
        embedded_trees(actual.tree.as_ref().unwrap(), &mut embedded);
        let selected = selection(embedded[1]);
        assert_eq!(
            selected
                .candidates
                .iter()
                .map(|item| item["language"].as_str().unwrap())
                .collect::<Vec<_>>(),
            ["first", "second"]
        );
        assert_eq!(
            selected
                .alternatives
                .iter()
                .map(|item| item.language.as_str())
                .collect::<Vec<_>>(),
            ["second"]
        );
        if name == "any" {
            assert_eq!(
                actual
                    .ambiguities
                    .iter()
                    .find(|item| item.rule == "any-grammar")
                    .unwrap()
                    .languages
                    .as_deref(),
                Some(["first".to_owned(), "second".to_owned()].as_slice())
            );
        } else {
            assert!(
                actual
                    .ambiguities
                    .iter()
                    .all(|item| item.rule != "any-grammar")
            );
        }
    }
    assert!(
        !archive_parser(&archive, "allRejected")
            .parse_tree("π".as_bytes(), &FeatureParseOptions::default())
            .unwrap()
            .ok
    );
    for (name, reason) in [
        ("unknown", "unknownGrammar"),
        ("escaped", "unsupportedEmbeddingMapping"),
    ] {
        let actual = archive_parser(&archive, name)
            .parse_tree("π".as_bytes(), &FeatureParseOptions::default())
            .unwrap();
        assert!(!actual.ok);
        assert_eq!(actual.rejection.unwrap().reason, reason);
    }
}

#[test]
fn archive_parser_runs_existing_proof_and_redirect_validation_before_selection() {
    let fixture: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/global-concept-space.lino"
    ))
    .unwrap();
    let archive = fixture["expectedLinks"].as_str().unwrap();
    let settings = GlobalConceptOptions {
        records: Some(fixture["records"].clone()),
        ..GlobalConceptOptions::default()
    };
    create_global_concept_space_parser(archive, "first", &settings, FeatureParseOptions::default())
        .unwrap();
    let changed = archive.replacen("(literal %CE%B1)", "(literal %CE%B2)", 1);
    let removed = archive
        .lines()
        .filter(|line| !line.starts_with("(concept-proof "))
        .collect::<Vec<_>>()
        .join("\n");
    let forged = archive.replacen(
        "(first%3Aroot second%3Apage)",
        "(first%3Aroot first%3Aroot)",
        1,
    );
    let redirected = format!("{archive}(concept-redirect missing nowhere)\n");
    for tampered in [&changed, &removed, &forged, &redirected] {
        assert_ne!(tampered, archive);
        let existing = read_global_concept_space_links(tampered, &settings).unwrap_err();
        for id in ["first", "missing"] {
            let error = create_global_concept_space_parser(
                tampered,
                id,
                &settings,
                FeatureParseOptions::default(),
            )
            .unwrap_err();
            assert_eq!(error.reason, "globalConceptSpace");
            assert_eq!(error.message, existing);
        }
    }
}

include!("global_concept_archive_parser_roots.rs");

#[test]
fn archived_recursion_preserves_shared_budgets_and_nonconsuming_cycle_diagnostics() {
    let (archive, _, _, chain) = archived_recursive_fixture();
    let parser = archive_parser(&archive, chain["root"].as_str().unwrap());
    for (options, reason) in [
        (
            FeatureParseOptions {
                step_limit: Some(0),
                ..FeatureParseOptions::default()
            },
            "stepLimit",
        ),
        (
            FeatureParseOptions {
                memory_limit: Some(0),
                ..FeatureParseOptions::default()
            },
            "memoryBudget",
        ),
        (
            FeatureParseOptions {
                max_depth: Some(0),
                ..FeatureParseOptions::default()
            },
            "nestingDepth",
        ),
    ] {
        let actual = parser
            .parse_tree(chain["source"].as_str().unwrap().as_bytes(), &options)
            .unwrap();
        assert!(!actual.ok);
        let rejection = actual.rejection.unwrap();
        assert_eq!(rejection.reason, reason);
        assert_eq!(rejection.limit, Some(0));
    }
    let fixture = selection_fixture();
    let cycle = &fixture["cyclic"];
    let name = cycle["name"].as_str().unwrap();
    let archive = archive_sources(&[archived_source(name, cycle["listing"].as_str().unwrap())]);
    let actual = archive_parser(&archive, name)
        .parse_tree(
            cycle["source"].as_str().unwrap().as_bytes(),
            &FeatureParseOptions::default(),
        )
        .unwrap();
    assert!(!actual.ok);
    assert_eq!(
        actual.rejection.unwrap().reason,
        "nonConsumingEmbeddingCycle"
    );
}

#[test]
fn archived_shipped_javascript_grammar_matches_existing_independent_cst_rows() {
    let fixture: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/native-grammars/javascript.lino"
    ))
    .unwrap();
    let sample = fixture["matches"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| {
            let source = case["source"].as_str().unwrap();
            !source.is_empty() && source.encode_utf16().count() <= 48
        })
        .expect("the existing independent fixture supplies a bounded source");
    let source = sample["source"].as_str().unwrap();
    let grammar = parse_grammar_links(include_str!(
        "../../../parity/grammars/native/javascript.lino"
    ))
    .unwrap();
    let ordinary = compile_feature_grammar(&grammar, None, FeatureParseOptions::default())
        .unwrap()
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .unwrap();
    assert!(ordinary.ok, "{:?}", ordinary.rejection);
    let rows = Rows::new(&fixture);
    assert_eq!(
        json!(rows.rows(ordinary.tree.as_ref().unwrap(), source)),
        sample["rows"]
    );
    let imported = GlobalConceptSource {
        id: "javascript-pinned".to_owned(),
        language: fixture["language"].as_str().unwrap().to_owned(),
        edition: String::new(),
        grammar,
    };
    let space =
        create_global_concept_space(std::slice::from_ref(&imported), &archive_settings()).unwrap();
    // The qualified root additionally charges all 143 productions and context
    // provenance. Both qualified runs use the same explicit bounded budget;
    // the independent ordinary-default parse above remains unchanged.
    let limits = FeatureParseOptions {
        step_limit: Some(2_000_000),
        ..FeatureParseOptions::default()
    };
    let expected = compile_feature_grammar(
        &imported.grammar,
        None,
        FeatureParseOptions {
            grammar_name: Some(imported.id.clone()),
            global_concept_space: Some(space.clone()),
            ..limits.clone()
        },
    )
    .unwrap()
    .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
    .unwrap();
    assert!(expected.ok, "{:?}", expected.rejection);
    let archive = global_concept_space_to_links(&space).unwrap();
    drop(imported);
    drop(space);
    let parser = create_global_concept_space_parser(
        &archive,
        "javascript-pinned",
        &archive_settings(),
        limits,
    )
    .unwrap();
    let actual = parser
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .unwrap();
    assert_eq!(actual, expected);
    let SyntaxTree::Embed { root, .. } = actual.tree.as_ref().unwrap() else {
        panic!("the archive parser retains a qualified root")
    };
    assert_eq!(json!(rows.rows(root, source)), sample["rows"]);
    assert_eq!(
        archived_tree_bytes(&tree_data(actual.tree.as_ref().unwrap())),
        source.as_bytes()
    );
}

#[test]
fn one_archive_retains_four_priority_grammars_and_independent_concrete_rows() {
    let definitions = [
        (
            "native-c",
            include_str!("../../../parity/fixtures/native-grammars/c.lino"),
            include_str!("../../../parity/grammars/native/c.lino"),
        ),
        (
            "native-javascript",
            include_str!("../../../parity/fixtures/native-grammars/javascript.lino"),
            include_str!("../../../parity/grammars/native/javascript.lino"),
        ),
        (
            "native-rust",
            include_str!("../../../parity/fixtures/native-grammars/rust.lino"),
            include_str!("../../../parity/grammars/native/rust.lino"),
        ),
        (
            "native-typescript",
            include_str!("../../../parity/fixtures/native-grammars/typescript.lino"),
            include_str!("../../../parity/grammars/native/typescript.lino"),
        ),
    ];
    let fixtures = definitions
        .iter()
        .map(|(_, fixture, _)| super::project_data::read_project_data(fixture).unwrap())
        .collect::<Vec<Value>>();
    let samples = fixtures
        .iter()
        .map(|fixture| {
            fixture["matches"]
                .as_array()
                .unwrap()
                .iter()
                .find(|case| {
                    let source = case["source"].as_str().unwrap();
                    !source.is_empty()
                        && source.len() <= 64
                        && case["rows"].as_array().unwrap().len() >= 4
                })
                .expect("an existing independent fixture supplies a bounded nontrivial CST")
                .clone()
        })
        .collect::<Vec<_>>();
    let sources = definitions
        .iter()
        .zip(&fixtures)
        .map(|((id, _, listing), fixture)| GlobalConceptSource {
            id: (*id).to_owned(),
            language: fixture["language"].as_str().unwrap().to_owned(),
            edition: String::new(),
            grammar: parse_grammar_links(listing).unwrap(),
        })
        .collect::<Vec<_>>();
    let space = create_global_concept_space(&sources, &archive_settings()).unwrap();
    let limits = FeatureParseOptions {
        step_limit: Some(4_000_000),
        ..FeatureParseOptions::default()
    };
    let expected = sources
        .iter()
        .zip(&fixtures)
        .zip(&samples)
        .map(|((imported, fixture), sample)| {
            let source = sample["source"].as_str().unwrap();
            let ordinary =
                compile_feature_grammar(&imported.grammar, None, FeatureParseOptions::default())
                    .unwrap()
                    .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
                    .unwrap();
            assert!(ordinary.ok, "{}: {:?}", imported.id, ordinary.rejection);
            assert_eq!(
                json!(Rows::new(fixture).rows(ordinary.tree.as_ref().unwrap(), source)),
                sample["rows"],
                "{}",
                imported.id
            );
            let qualified = compile_feature_grammar(
                &imported.grammar,
                None,
                FeatureParseOptions {
                    grammar_name: Some(imported.id.clone()),
                    global_concept_space: Some(space.clone()),
                    ..limits.clone()
                },
            )
            .unwrap()
            .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
            .unwrap();
            assert!(qualified.ok, "{}: {:?}", imported.id, qualified.rejection);
            qualified
        })
        .collect::<Vec<_>>();
    let archive = global_concept_space_to_links(&space).unwrap();
    let mappings = space["mappings"].clone();
    drop(sources);
    drop(space);
    let restored = read_global_concept_space_links(&archive, &archive_settings()).unwrap();
    assert_eq!(restored["mappings"], mappings);
    assert_eq!(global_concept_space_to_links(&restored).unwrap(), archive);
    for (((id, _, _), fixture), (sample, expected)) in definitions
        .iter()
        .zip(&fixtures)
        .zip(samples.iter().zip(expected))
    {
        let source = sample["source"].as_str().unwrap();
        let actual =
            create_global_concept_space_parser(&archive, id, &archive_settings(), limits.clone())
                .unwrap()
                .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
                .unwrap();
        assert_eq!(actual, expected, "{id}: the complete archived outcome");
        let SyntaxTree::Embed { root, .. } = actual.tree.as_ref().unwrap() else {
            panic!("the archive parser retains a qualified root for {id}")
        };
        assert_eq!(json!(Rows::new(fixture).rows(root, source)), sample["rows"]);
        assert_eq!(
            archived_tree_bytes(&tree_data(actual.tree.as_ref().unwrap())),
            source.as_bytes(),
            "{id}: every physical source byte"
        );
    }
}

fn source_text_meaning() -> Value {
    let declaration = FormalConceptDefinitionInput::Declaration(None);
    encode_formal_concept_definition(declaration, "source text value")
        .unwrap()
        .to_json()
}

#[test]
fn archived_source_formats_retain_peg_repetition_and_declared_matching_overrides() {
    let listing = "(grammar (start document))\n(rule document normal (seq (repeat0 (literal a)) (literal a)))\n";
    let grammar = parse_grammar_links(listing).unwrap();
    for format in [
        None,
        Some(GrammarFormat::Bnf),
        Some(GrammarFormat::Ebnf),
        Some(GrammarFormat::Abnf),
        Some(GrammarFormat::Antlr),
        Some(GrammarFormat::Lark),
        Some(GrammarFormat::Gbnf),
        Some(GrammarFormat::TreeSitter),
        Some(GrammarFormat::MetaLanguage),
        Some(GrammarFormat::Inferred),
    ] {
        let mut source = archived_source("generalized-default", listing);
        if let Some(format) = format {
            source.grammar.set_source_format(format);
        }
        let archive = archive_sources(&[source]);
        assert!(
            archive_parser(&archive, "generalized-default")
                .parse_tree(b"a", &FeatureParseOptions::default())
                .unwrap()
                .ok
        );
    }
    let specifications = [
        ("peg-default", Some(GrammarFormat::Peg), None, false),
        ("peg-explicit", Some(GrammarFormat::Peg), Some("peg"), false),
        ("generalized", None, None, true),
        (
            "override",
            Some(GrammarFormat::Peg),
            Some("generalized"),
            true,
        ),
    ];
    let mut sources = specifications
        .iter()
        .map(|(id, format, matching, _)| {
            let mut source = GlobalConceptSource {
                id: (*id).to_owned(),
                language: (*id).to_owned(),
                edition: String::new(),
                grammar: grammar.clone(),
            };
            if let Some(format) = format {
                source.grammar.set_source_format(*format);
            }
            let mut declarations = source.grammar.declarations().clone();
            declarations.matching = matching.map(str::to_owned);
            source.grammar.set_declarations(declarations);
            source
        })
        .collect::<Vec<_>>();
    let meaning = source_text_meaning();
    let settings = GlobalConceptOptions {
        records: Some(json!([])),
        formal_definitions: Some(Value::Object(
            sources
                .iter()
                .map(|source| (format!("{}:document", source.id), meaning.clone()))
                .collect(),
        )),
        equivalences: vec![[
            "peg-default:document".to_owned(),
            "peg-explicit:document".to_owned(),
        ]],
        ..GlobalConceptOptions::default()
    };
    let space = create_global_concept_space(&sources, &settings).unwrap();
    assert_eq!(space["failures"], json!([]));
    assert_eq!(
        space["mappings"]["peg-default:document"],
        space["mappings"]["peg-explicit:document"]
    );
    assert_ne!(
        space["mappings"]["peg-default:document"],
        space["mappings"]["generalized:document"]
    );
    assert_ne!(
        space["mappings"]["peg-default:document"],
        space["mappings"]["override:document"]
    );
    let original = space["sources"]
        .as_array()
        .unwrap()
        .iter()
        .find(|source| source["id"] == "peg-default")
        .unwrap();
    assert!(original["context"]["matching"].is_null());
    let expected = sources
        .iter()
        .zip(&specifications)
        .map(|(source, (_, _, _, accepted))| {
            let ordinary =
                compile_feature_grammar(&source.grammar, None, FeatureParseOptions::default())
                    .unwrap()
                    .parse_tree(b"a", &FeatureParseOptions::default())
                    .unwrap();
            assert_eq!(ordinary.ok, *accepted, "{}", source.id);
            if !ordinary.ok {
                assert_eq!(ordinary.rejection.as_ref().unwrap().reason, "syntax");
            }
            let qualified = compile_feature_grammar(
                &source.grammar,
                None,
                FeatureParseOptions {
                    grammar_name: Some(source.id.clone()),
                    global_concept_space: Some(space.clone()),
                    ..FeatureParseOptions::default()
                },
            )
            .unwrap()
            .parse_tree(b"a", &FeatureParseOptions::default())
            .unwrap();
            assert_eq!(qualified.ok, ordinary.ok, "{}", source.id);
            qualified
        })
        .collect::<Vec<_>>();
    let archive = global_concept_space_to_links(&space).unwrap();
    assert!(archive.contains("(grammar-source-format peg-default peg)"));
    assert!(!archive.contains("(grammar-source-format generalized "));
    sources.clear();
    drop(grammar);
    let reloaded = read_global_concept_space_links(&archive, &archive_settings()).unwrap();
    assert_eq!(reloaded, space);
    assert_eq!(global_concept_space_to_links(&reloaded).unwrap(), archive);
    for ((id, _, _, _), expected) in specifications.iter().zip(expected) {
        assert_eq!(
            archive_parser(&archive, id)
                .parse_tree(b"a", &FeatureParseOptions::default())
                .unwrap(),
            expected
        );
    }
}

#[test]
fn archived_format_and_matching_tampering_cannot_bypass_current_proof_witnesses() {
    let listing = "(grammar (format peg) (start document))\n(rule document normal (seq (repeat0 (literal a)) (literal a)))\n";
    let mut sources = [
        archived_source("first", listing),
        archived_source("second", listing),
    ];
    for source in &mut sources {
        source.edition.clear();
    }
    let mut declarations = sources[1].grammar.declarations().clone();
    declarations.matching = Some("peg".to_owned());
    sources[1].grammar.set_declarations(declarations);
    let meaning = source_text_meaning();
    let settings = GlobalConceptOptions {
        records: Some(json!([])),
        formal_definitions: Some(json!({"first:document":meaning,"second:document":meaning})),
        equivalences: vec![["first:document".to_owned(), "second:document".to_owned()]],
        ..GlobalConceptOptions::default()
    };
    let archive =
        global_concept_space_to_links(&create_global_concept_space(&sources, &settings).unwrap())
            .unwrap();
    let original = read_global_concept_space_links(&archive, &archive_settings()).unwrap();
    let first_format = "(grammar-source-format first peg)";
    let explicit_format_edit = archive.replacen(
        "(grammar-source-format second peg)",
        "(grammar-source-format second bnf)",
        1,
    );
    assert_ne!(explicit_format_edit, archive);
    let unchanged =
        read_global_concept_space_links(&explicit_format_edit, &archive_settings()).unwrap();
    assert_eq!(
        unchanged["sources"]
            .as_array()
            .unwrap()
            .iter()
            .find(|source| source["id"] == "second")
            .unwrap()["sourceFormat"],
        "bnf"
    );
    assert_eq!(unchanged["mappings"], original["mappings"]);
    assert_eq!(unchanged["proofs"], original["proofs"]);
    assert!(
        !archive_parser(&explicit_format_edit, "second")
            .parse_tree(b"a", &FeatureParseOptions::default())
            .unwrap()
            .ok
    );
    for changed in [
        archive.replacen(first_format, "(grammar-source-format first bnf)", 1),
        archive.replacen("(grammar-source-format first peg)\n", "", 1),
        archive.replacen(
            "(grammar-context second (matching peg))",
            "(grammar-context second (matching generalized))",
            1,
        ),
    ] {
        assert_ne!(changed, archive);
        for id in ["first", "missing"] {
            let error = create_global_concept_space_parser(
                &changed,
                id,
                &archive_settings(),
                FeatureParseOptions::default(),
            )
            .unwrap_err();
            assert!(
                error
                    .message
                    .contains("unproved global concept correspondence")
                    || error.message.contains("one-to-one proof"),
                "{}",
                error.message
            );
        }
    }
    for changed in [
        archive.replacen(first_format, "(grammar-source-format first unknown)", 1),
        archive.replacen(first_format, "(grammar-source-format first (peg))", 1),
        archive.replacen(
            first_format,
            "(grammar-source-format first peg generalized)",
            1,
        ),
        format!("{archive}(grammar-source-format first peg)\n"),
        format!("{archive}(grammar-source-format absent peg)\n"),
        archive.replacen(
            "(grammar-context second (matching peg))",
            "(grammar-context second (matching unknown))",
            1,
        ),
    ] {
        assert_ne!(changed, archive);
        let existing = read_global_concept_space_links(&changed, &archive_settings()).unwrap_err();
        assert!(
            ["format", "matching", "word"]
                .iter()
                .any(|word| existing.contains(word)),
            "{existing}"
        );
        let error = create_global_concept_space_parser(
            &changed,
            "missing",
            &archive_settings(),
            FeatureParseOptions::default(),
        )
        .unwrap_err();
        assert_eq!(error.message, existing);
    }
    let grammar: Value = serde_json::from_str(
        &meta_language::grammar::interchange::serialize_grammar(&sources[0].grammar),
    )
    .unwrap();
    for format in [json!("unknown"), json!(""), json!(0), json!(false)] {
        let mut source = json!({"id":"unsupported","language":"unsupported","grammar":grammar});
        source["grammar"]["sourceFormat"] = format;
        assert!(
            GlobalConceptSource::from_json(&source)
                .unwrap_err()
                .contains("format")
        );
    }
}
