//! Shared authored grammar data only; actual catalog language coverage is
//! independent. These tests execute only in CI, never local native builds.
use std::collections::HashMap;

use meta_language::{
    FeatureParseOptions, GlobalConceptOptions, GlobalConceptSource, Grammar, GrammarExpr, LeafText,
    SyntaxTree, check_grammar_selection_mapped_tree, compile_feature_grammar,
    create_global_concept_space, global_concept_space_to_links, parse_grammar_links,
    read_global_concept_space_links, render_grammar_links,
};
use serde_json::{Value, json};

#[path = "grammar_recursive_selection_mapping.rs"]
mod mapping_transport;
use mapping_transport::{
    mapped_bundle, mapped_embeds, mapped_first, mapped_fixture, mapped_json_grammar,
    mapped_json_host, mapped_literal, mapped_parse, mapped_physical, mapped_python_grammar,
    mapped_stored_variants, mapped_tree_data,
};

fn selection_fixture() -> Value {
    super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/grammar-recursive-selection.lino"
    ))
    .unwrap()
}

fn selected_grammars(fixture: &Value) -> HashMap<String, Grammar> {
    fixture["grammars"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| {
            (
                source["name"].as_str().unwrap().to_owned(),
                parse_grammar_links(source["listing"].as_str().unwrap()).unwrap(),
            )
        })
        .collect()
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

fn rebuilt_selection_source(tree: &SyntaxTree) -> String {
    match tree {
        SyntaxTree::Node { children, .. } => {
            children.iter().map(rebuilt_selection_source).collect()
        }
        SyntaxTree::Embed { root, .. } => rebuilt_selection_source(root),
        SyntaxTree::Token {
            text: LeafText::Text(text),
            ..
        }
        | SyntaxTree::Error {
            text: LeafText::Text(text),
            ..
        } => text.clone(),
        SyntaxTree::Missing { start, end, .. } => {
            assert_eq!(start, end);
            String::new()
        }
        other => panic!("this authored UTF-8 control needs actual leaf text: {other:?}"),
    }
}

fn authored_global_space(fixture: &Value, grammars: &HashMap<String, Grammar>) -> Value {
    let settings = GlobalConceptOptions {
        records: Some(json!([])),
        ..GlobalConceptOptions::default()
    };
    let sources = fixture["grammars"]
        .as_array()
        .unwrap()
        .iter()
        .take(4)
        .map(|source| {
            let name = source["name"].as_str().unwrap();
            GlobalConceptSource {
                id: name.to_owned(),
                language: name.to_owned(),
                edition: "authored-1".to_owned(),
                grammar: grammars[name].clone(),
            }
        })
        .collect::<Vec<_>>();
    let space = create_global_concept_space(&sources, &settings).unwrap();
    read_global_concept_space_links(&global_concept_space_to_links(&space).unwrap(), &settings)
        .unwrap()
}

#[test]
fn qualified_root_and_repeated_language_keep_actual_global_origins_and_all_source_bytes() {
    let fixture = selection_fixture();
    let grammars = selected_grammars(&fixture);
    let space = authored_global_space(&fixture, &grammars);
    let resolve = |name: &str| grammars.get(name).cloned();
    let options = FeatureParseOptions {
        grammar_name: Some("Markdown".to_owned()),
        global_concept_space: Some(space.clone()),
        ..FeatureParseOptions::default()
    };
    let parser =
        compile_feature_grammar(&grammars["Markdown"], Some(&resolve), options.clone()).unwrap();
    let source = fixture["chain"]["source"].as_str().unwrap();
    let result = parser
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .unwrap();
    assert!(result.ok, "{:?}", result.rejection);
    let tree = result.tree.unwrap();
    assert_eq!(rebuilt_selection_source(&tree), source);
    let mut embedded = Vec::new();
    embedded_trees(&tree, &mut embedded);
    let mut names = Vec::new();
    let mut root_concept = Value::Null;
    for (index, tree) in embedded.iter().enumerate() {
        let SyntaxTree::Embed {
            language,
            start,
            end,
            root,
            selection: Some(selection),
            ..
        } = tree
        else {
            panic!("each invocation retains selection metadata");
        };
        names.push(language.clone());
        assert_eq!((root.start(), root.end()), (*start, *end));
        assert_eq!(selection.mapping, "identity");
        assert_eq!(selection.provenance["edition"], "authored-1");
        let origin = format!("{language}:document");
        let root_rule = selection.provenance["rules"]
            .as_array()
            .unwrap()
            .iter()
            .find(|rule| rule["rule"] == "document")
            .unwrap();
        assert_eq!(root_rule["concept"], space["mappings"][&origin]);
        assert_eq!(
            root_rule["sourceNames"],
            json!([{"source":"authored","name":"document"}])
        );
        if index == 0 {
            assert_eq!(selection.mode, "grammar-root");
            root_concept = root_rule["concept"].clone();
        }
        if index == 3 {
            assert_eq!(
                root_rule["concept"], root_concept,
                "the return to Markdown reuses the root origin"
            );
        }
    }
    assert_eq!(json!(names), fixture["chain"]["languages"]);
    let restored: HashMap<_, _> = grammars
        .iter()
        .map(|(name, grammar)| {
            (
                name.clone(),
                parse_grammar_links(&render_grammar_links(grammar)).unwrap(),
            )
        })
        .collect();
    let restore = |name: &str| restored.get(name).cloned();
    let replay = compile_feature_grammar(&restored["Markdown"], Some(&restore), options)
        .unwrap()
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .unwrap();
    assert_eq!(replay.tree.as_ref(), Some(&tree));
    let legacy = compile_feature_grammar(
        &grammars["Markdown"],
        Some(&resolve),
        FeatureParseOptions::default(),
    )
    .unwrap()
    .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
    .unwrap();
    assert!(
        matches!(legacy.tree, Some(SyntaxTree::Node { .. })),
        "legacy root shape remains unchanged"
    );
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
        let limited = parser.parse_tree(source.as_bytes(), &options).unwrap();
        let rejection = limited.rejection.unwrap();
        assert_eq!(rejection.reason, reason);
        assert_eq!(rejection.limit, Some(0));
    }
}

#[test]
fn any_retains_alternative_concrete_trees_and_all_requires_every_explicit_candidate() {
    let fixture = selection_fixture();
    let grammars = selected_grammars(&fixture);
    let resolve = |name: &str| grammars.get(name).cloned();
    for name in ["any", "all", "allRejected"] {
        let grammar = parse_grammar_links(fixture[name]["listing"].as_str().unwrap()).unwrap();
        let parser =
            compile_feature_grammar(&grammar, Some(&resolve), FeatureParseOptions::default())
                .unwrap();
        let source = fixture[name]["source"].as_str().unwrap();
        let result = parser
            .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
            .unwrap();
        assert_eq!(result.ok, name != "allRejected");
        if name == "allRejected" {
            assert_eq!(result.rejection.unwrap().reason, "syntax");
            continue;
        }
        let tree = result.tree.unwrap();
        assert_eq!(rebuilt_selection_source(&tree), source);
        let mut embedded = Vec::new();
        embedded_trees(&tree, &mut embedded);
        let SyntaxTree::Embed {
            root,
            selection: Some(selection),
            ..
        } = embedded[0]
        else {
            panic!("selected boundary");
        };
        assert_eq!(selection.alternatives.len(), 1);
        assert!(matches!(**root, SyntaxTree::Node { ref kind, .. } if kind == "first_root"));
        assert!(
            matches!(*selection.alternatives[0].root, SyntaxTree::Node { ref kind, .. } if kind == "second_root")
        );
        if name == "any" {
            assert_eq!(
                result.ambiguities.len(),
                2,
                "the original root ambiguity and language ambiguity both remain"
            );
            assert_eq!(result.ambiguities[0].rule, "document");
            assert_eq!(result.ambiguities[1].rule, "any-grammar");
            assert_eq!(
                result.ambiguities[1].languages,
                Some(vec!["first".to_owned(), "second".to_owned()])
            );
            let rejected = parser
                .parse_tree(
                    source.as_bytes(),
                    &FeatureParseOptions {
                        reject_ambiguity: Some(true),
                        ..FeatureParseOptions::default()
                    },
                )
                .unwrap();
            assert_eq!(rejected.rejection.unwrap().reason, "ambiguity");
        } else {
            assert!(result.ambiguities.is_empty());
        }
    }
}

#[test]
fn unknown_grammars_and_unimplemented_escape_mappings_remain_explicit_rejections() {
    let fixture = selection_fixture();
    let grammars = selected_grammars(&fixture);
    let resolve = |name: &str| grammars.get(name).cloned();
    for name in ["unknown", "escaped"] {
        let grammar = parse_grammar_links(fixture[name]["listing"].as_str().unwrap()).unwrap();
        let result =
            compile_feature_grammar(&grammar, Some(&resolve), FeatureParseOptions::default())
                .unwrap()
                .parse_tree(
                    fixture[name]["source"].as_str().unwrap().as_bytes(),
                    &FeatureParseOptions::default(),
                )
                .unwrap();
        assert!(!result.ok);
        let rejection = result.rejection.unwrap();
        assert_eq!(json!(rejection.reason), fixture[name]["reason"]);
        assert!(!rejection.diagnostics.unwrap().is_empty());
        if name == "unknown" {
            assert_eq!(rebuilt_selection_source(&result.tree.unwrap()), "π");
        }
    }
}

#[test]
fn nonconsuming_cycles_diagnose_the_exact_bounds_and_shrinking_recursion_remains_lossless() {
    let fixture = selection_fixture();
    for name in ["cyclic", "consuming"] {
        let grammar = parse_grammar_links(fixture[name]["listing"].as_str().unwrap()).unwrap();
        let language = fixture[name]["name"].as_str().unwrap();
        let resolve = |name: &str| (name == language).then(|| grammar.clone());
        let parser = compile_feature_grammar(
            &grammar,
            Some(&resolve),
            FeatureParseOptions {
                grammar_name: Some(language.to_owned()),
                ..FeatureParseOptions::default()
            },
        )
        .unwrap();
        if name == "cyclic" {
            let result = parser
                .parse_tree("π".as_bytes(), &FeatureParseOptions::default())
                .unwrap();
            let rejection = result.rejection.unwrap();
            assert_eq!(rejection.reason, "nonConsumingEmbeddingCycle");
            assert_eq!(
                rejection.diagnostics.unwrap()[0],
                json!({"reason":"nonConsumingEmbeddingCycle","language":"cycle","start":0,"end":2})
            );
        } else {
            for source in fixture[name]["sources"].as_array().unwrap() {
                let source = source.as_str().unwrap();
                let result = parser
                    .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
                    .unwrap();
                assert!(result.ok, "{:?}", result.rejection);
                assert_eq!(rebuilt_selection_source(&result.tree.unwrap()), source);
            }
        }
    }
}

#[test]
fn changed_definitions_meanings_and_operational_contexts_cannot_borrow_stored_global_origins() {
    let fixture = selection_fixture();
    let original = selected_grammars(&fixture);
    let space = authored_global_space(&fixture, &original);
    for edit in ["definition", "meaning", "context"] {
        let mut grammars = original.clone();
        let base = &original["Python"];
        let mut changed = Grammar::new();
        changed.set_start("document");
        let mut declarations = base.declarations().clone();
        if edit == "context" {
            declarations.matching = Some("peg".to_owned());
        }
        changed.set_declarations(declarations);
        for mut rule in base.rules().iter().cloned() {
            if rule.name == "document" {
                if edit == "definition" {
                    rule.expr = meta_language::GrammarExpr::Terminal("x".to_owned());
                }
                if edit == "meaning" {
                    rule.concept = Some("fixture.changed-meaning".to_owned());
                }
            }
            changed.add_rule(rule);
        }
        grammars.insert("Python".to_owned(), changed);
        let resolve = |name: &str| grammars.get(name).cloned();
        let parser = compile_feature_grammar(
            &grammars["Markdown"],
            Some(&resolve),
            FeatureParseOptions {
                global_concept_space: Some(space.clone()),
                ..FeatureParseOptions::default()
            },
        )
        .unwrap();
        let source = if edit == "definition" {
            "m{x}"
        } else {
            "m{π}"
        };
        let result = parser
            .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
            .unwrap();
        assert!(!result.ok);
        assert_eq!(
            result.rejection.unwrap().reason,
            "unprovedGrammarOrigin",
            "{edit}"
        );
        let tree = result.tree.unwrap();
        assert_eq!(rebuilt_selection_source(&tree), source);
        let mut embedded = Vec::new();
        embedded_trees(&tree, &mut embedded);
        let SyntaxTree::Embed {
            selection: Some(selection),
            ..
        } = embedded[0]
        else {
            panic!("selected Python tree");
        };
        assert!(
            selection.provenance["rules"]
                .as_array()
                .unwrap()
                .iter()
                .find(|rule| rule["rule"] == "document")
                .unwrap()["concept"]
                .is_null(),
            "{edit}"
        );
    }
}

#[test]
fn mapped_json_retains_shipped_python_cst_original_oracle_rows_and_exact_escape_intervals() {
    let inventory = mapped_fixture("language-grammar-inventory.lino");
    let python = inventory["languages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["name"] == "Python")
        .unwrap();
    let source = python["source"].as_str().unwrap();
    let grammars = HashMap::from([("Python".to_owned(), mapped_python_grammar())]);
    let host = mapped_json_host(&[("Python", "json-string")], "any-grammar", "");
    let physical = serde_json::to_string(source).unwrap();
    let outcome = mapped_parse(&host, &grammars, &physical);
    assert!(outcome.ok, "{:?}", outcome.rejection);
    let direct = mapped_parse(&grammars["Python"], &HashMap::new(), source);
    let direct_json = mapped_parse(&mapped_json_grammar(), &HashMap::new(), &physical);
    assert!(direct.ok && direct_json.ok);
    let bundle = mapped_bundle(&outcome);
    let embedded = mapped_first(&bundle["tree"]);
    assert_eq!(
        embedded["sourceChildren"],
        json!([mapped_tree_data(direct_json.tree.as_ref().unwrap())])
    );
    assert_eq!(
        embedded["root"],
        mapped_tree_data(direct.tree.as_ref().unwrap())
    );
    assert_eq!(mapped_physical(&bundle["tree"]), physical.as_bytes());
    assert_eq!(mapped_physical(&embedded["root"]), source.as_bytes());
    let entry = meta_language::native_grammar("native-python").unwrap();
    let configuration = json!({"hidden":entry.hidden,"anonymous":entry.anonymous,"extras":entry.extras,
        "rootIncludesLeadingTrivia":entry.root_includes_leading_trivia,"oracleKinds":entry.oracle_kinds});
    let current = mapped_fixture("fixtures/native-default-cst-expected.lino");
    let original = mapped_fixture("fixtures/default-cst-expected.lino");
    let expected = current["languages"]
        .get("Python")
        .unwrap_or_else(|| &original["languages"]["Python"]);
    assert_eq!(
        json!(
            super::issue_195_native_grammar_rows::Rows::new(&configuration)
                .rows(direct.tree.as_ref().unwrap(), source)
        ),
        expected["positive"]
    );
    assert!(std::ptr::eq(
        check_grammar_selection_mapped_tree(&bundle, &FeatureParseOptions::default()).unwrap(),
        &bundle
    ));
    let resolve = |name: &str| grammars.get(name).cloned();
    let parser =
        compile_feature_grammar(&host, Some(&resolve), FeatureParseOptions::default()).unwrap();
    let error = parser
        .parse(physical.as_bytes(), &FeatureParseOptions::default())
        .unwrap_err();
    assert_eq!(error.rejection.reason, "selection");
    assert!(
        error.rejection.diagnostics.unwrap()[0]["detail"]
            .as_str()
            .unwrap()
            .contains("sourcePlanes")
    );
    let raw = r#""a\n\u03C0\uD83D\uDE00""#;
    let decoded = "a\nπ😀";
    assert_eq!((raw.len(), decoded.len()), (23, 8));
    let expected = [
        [0, 1, 0, 0],
        [1, 2, 0, 1],
        [2, 4, 1, 2],
        [4, 10, 2, 4],
        [10, 22, 4, 8],
        [22, 23, 8, 8],
    ];
    for prefix in ["", "λ<"] {
        let grammars = HashMap::from([("decoded".to_owned(), mapped_literal(decoded))]);
        let host = mapped_json_host(&[("decoded", "json-string")], "any-grammar", prefix);
        let result = mapped_parse(&host, &grammars, &format!("{prefix}{raw}"));
        assert!(result.ok, "{:?}", result.rejection);
        let bundle = mapped_bundle(&result);
        let embedded = mapped_first(&bundle["tree"]);
        let intervals = expected.map(|[a, b, c, d]| [a + prefix.len(), b + prefix.len(), c, d]);
        assert_eq!(
            embedded["selection"]["sourceMapping"]["intervals"],
            json!(intervals)
        );
        assert_eq!(
            bundle["sourcePlanes"],
            json!([
                {"kind":"grammar-source","identity":0,"encoding":"UTF-8","byteLength":prefix.len()+23,"derivation":null},
                {"kind":"grammar-source","identity":1,"encoding":"UTF-8","byteLength":8,
                    "derivation":{"operation":"json-string","input":0,"start":prefix.len(),"end":prefix.len()+23}},
            ])
        );
        assert_eq!(embedded["selection"]["sourceMapping"]["input"], 0);
        assert_eq!(embedded["selection"]["sourceMapping"]["output"], 1);
        assert_eq!(
            mapped_physical(&bundle["tree"]),
            format!("{prefix}{raw}").as_bytes()
        );
        assert_eq!(mapped_physical(&embedded["root"]), decoded.as_bytes());
        assert_eq!(
            (
                embedded["root"]["start"].as_u64(),
                embedded["root"]["end"].as_u64()
            ),
            (Some(0), Some(8))
        );
        check_grammar_selection_mapped_tree(&bundle, &FeatureParseOptions::default()).unwrap();
    }
    mapping_transport::record_mapped_primary(source, raw, decoded, &expected);
}

#[test]
fn nested_mapped_registry_and_complete_grammars_replay_from_text_and_binary_links() {
    mapping_transport::check_nested_mapped_registry_and_complete_grammar_archives();
}

#[test]
fn mapped_source_forgeries_survive_generic_archives_but_fail_the_correspondence_checker() {
    let raw = r#""a\n\u03C0\uD83D\uDE00""#;
    let decoded = "a\nπ😀";
    let grammars = HashMap::from([("decoded".to_owned(), mapped_literal(decoded))]);
    let host = mapped_json_host(&[("decoded", "json-string")], "any-grammar", "");
    let outcome = mapped_parse(&host, &grammars, raw);
    assert!(outcome.ok, "{:?}", outcome.rejection);
    let genuine = mapped_bundle(&outcome);
    let boundary = "/tree/children/0";
    assert_eq!(
        genuine
            .pointer(&format!("{boundary}/selection/mapping"))
            .unwrap(),
        "json-string"
    );
    assert_eq!(
        genuine
            .pointer(&format!("{boundary}/root/children/0/text"))
            .unwrap(),
        decoded
    );
    check_grammar_selection_mapped_tree(&genuine, &FeatureParseOptions::default()).unwrap();
    let mut replacements = Vec::new();
    for (path, replacement) in [
        ("/tree/start", json!(1)),
        ("/tree/end", json!(22)),
        ("/tree/children/0/start", json!(1)),
        ("/tree/children/0/end", json!(22)),
        ("/tree/children/0/root/start", json!(1)),
        ("/tree/children/0/root/end", json!(7)),
        ("/sourcePlanes", Value::Null),
        ("/sourcePlanes/0/kind", json!("unowned-source")),
        ("/sourcePlanes/0/encoding", json!("UTF-16")),
        ("/sourcePlanes/0/identity", json!(1)),
        ("/sourcePlanes/0/byteLength", json!(24)),
        ("/sourcePlanes/1/identity", json!(0)),
        ("/sourcePlanes/1/identity", json!(0.5)),
        ("/sourcePlanes/1/byteLength", json!(9)),
        ("/sourcePlanes/1/derivation", Value::Null),
        ("/sourcePlanes/1/derivation/input", json!(1)),
        ("/sourcePlanes/1/derivation/operation", json!("identity")),
        ("/sourcePlanes/1/derivation/start", json!(1)),
        (
            "/tree/children/0/selection/mapping",
            json!("guessed-string"),
        ),
        ("/tree/children/0/sourceChildren", Value::Null),
        ("/tree/children/0/selection/sourceMapping/input", json!(1)),
        ("/tree/children/0/selection/sourceMapping/output", json!(2)),
        (
            "/tree/children/0/selection/sourceMapping/operation",
            json!("identity"),
        ),
        (
            "/tree/children/0/selection/sourceMapping/intervals/1/0",
            json!(2),
        ),
        (
            "/tree/children/0/selection/sourceMapping/intervals/1/1",
            json!(3),
        ),
        (
            "/tree/children/0/selection/sourceMapping/intervals/1/2",
            json!(0.5),
        ),
        (
            "/tree/children/0/selection/sourceMapping/intervals/0/0",
            json!(-1),
        ),
        (
            "/tree/children/0/selection/sourceMapping/intervals/4/3",
            json!(7),
        ),
        ("/tree/children/0/root/children/0/text", json!("b\nπ😀")),
    ] {
        replacements.push(json!({"path":path,"value":replacement.clone()}));
        let mut forged = genuine.clone();
        *forged
            .pointer_mut(path)
            .unwrap_or_else(|| panic!("the genuine source owns {path}")) = replacement;
        for restored in mapped_stored_variants(&forged) {
            assert_eq!(
                restored, forged,
                "generic decoding does not silently repair {path}"
            );
            let error =
                check_grammar_selection_mapped_tree(&restored, &FeatureParseOptions::default())
                    .unwrap_err();
            assert_eq!(error.reason, "selection", "{path}: {error}");
            assert!(
                error.message.starts_with("mapped grammar CST:"),
                "{path}: {error}"
            );
        }
    }
    for path in [
        "/sourcePlanes/0",
        "/sourcePlanes/1/derivation",
        "/tree/children/0/selection/sourceMapping",
    ] {
        let mut forged = genuine.clone();
        forged.pointer_mut(path).unwrap()["opaqueBinding"] = json!(true);
        for restored in mapped_stored_variants(&forged) {
            assert_eq!(restored, forged);
            assert_eq!(
                check_grammar_selection_mapped_tree(&restored, &FeatureParseOptions::default())
                    .unwrap_err()
                    .reason,
                "selection"
            );
        }
    }
    let mut absent = genuine;
    absent.as_object_mut().unwrap().remove("sourcePlanes");
    assert_eq!(
        check_grammar_selection_mapped_tree(&absent, &FeatureParseOptions::default())
            .unwrap_err()
            .reason,
        "selection"
    );
    mapping_transport::record_mapped_forgeries(&replacements, &absent);
}

#[test]
fn mapped_profiles_keep_witnesses_and_existing_limits_while_failed_derivations_are_not_csts() {
    let child = mapped_literal("x");
    let grammars = HashMap::from([
        ("decoded".to_owned(), child.clone()),
        ("twin".to_owned(), child),
    ]);
    let source = "\"x\"";
    for mode in ["any-grammar", "all-grammar"] {
        let host = mapped_json_host(
            &[("decoded", "json-string"), ("twin", "json-string")],
            mode,
            "",
        );
        let outcome = mapped_parse(&host, &grammars, source);
        assert!(outcome.ok, "{:?}", outcome.rejection);
        let bundle = mapped_bundle(&outcome);
        let selected = &mapped_first(&bundle["tree"])["selection"];
        assert_eq!(selected["ambiguous"], mode == "any-grammar");
        assert_eq!(
            selected["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item["relation"].clone())
                .collect::<Vec<_>>(),
            vec![json!("accepted"), json!("accepted")]
        );
        assert_eq!(selected["alternatives"].as_array().unwrap().len(), 1);
        assert_eq!(selected["alternatives"][0]["language"], "twin");
        assert_eq!(
            selected["alternatives"][0]["root"],
            mapped_first(&bundle["tree"])["root"]
        );
        assert_eq!(bundle["sourcePlanes"].as_array().unwrap().len(), 2);
        check_grammar_selection_mapped_tree(&bundle, &FeatureParseOptions::default()).unwrap();
    }
    let host = mapped_json_host(&[("decoded", "json-string")], "any-grammar", "");
    let outcome = mapped_parse(&host, &grammars, source);
    let bundle = mapped_bundle(&outcome);
    let resolve = |name: &str| grammars.get(name).cloned();
    let parser =
        compile_feature_grammar(&host, Some(&resolve), FeatureParseOptions::default()).unwrap();
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
        let limited = parser.parse_tree(source.as_bytes(), &options).unwrap();
        let rejection = limited.rejection.unwrap();
        assert_eq!(rejection.reason, reason);
        assert_eq!(rejection.limit, Some(0));
        assert_eq!(
            check_grammar_selection_mapped_tree(&bundle, &options)
                .unwrap_err()
                .reason,
            reason
        );
    }
    for (names, reason) in [
        (vec![("missing", "json-string")], "unknownGrammar"),
        (
            vec![("decoded", "quoted-escape")],
            "unsupportedEmbeddingMapping",
        ),
        (
            vec![("decoded", "json-string"), ("twin", "identity")],
            "unsupportedEmbeddingMapping",
        ),
    ] {
        let rejected = mapped_parse(
            &mapped_json_host(&names, "any-grammar", ""),
            &grammars,
            source,
        );
        assert!(!rejected.ok);
        assert_eq!(rejected.rejection.unwrap().reason, reason);
    }
    for raw in [r#""\uD800""#, r#""\uDC00""#] {
        assert!(mapped_parse(&mapped_json_grammar(), &HashMap::new(), raw).ok);
        let rejected = mapped_parse(&host, &grammars, raw);
        assert!(!rejected.ok);
        assert_eq!(
            rejected.rejection.unwrap().reason,
            "unsupportedEmbeddingMapping"
        );
    }
    let cycle = parse_grammar_links("(grammar (start document))\n(rule document normal (any-grammar (literal x) (grammar-candidate cycle identity)))\n").unwrap();
    let rejected = mapped_parse(
        &mapped_json_host(&[("cycle", "json-string")], "any-grammar", ""),
        &HashMap::from([("cycle".to_owned(), cycle)]),
        source,
    );
    let rejection = rejected.rejection.unwrap();
    assert_eq!(rejection.reason, "nonConsumingEmbeddingCycle");
    let diagnostics = rejection.diagnostics.unwrap();
    let cycle = diagnostics
        .iter()
        .find(|item| item["reason"] == "nonConsumingEmbeddingCycle")
        .unwrap();
    assert_eq!(
        (
            cycle["source"].as_u64(),
            cycle["sourceStart"].as_u64(),
            cycle["sourceEnd"].as_u64()
        ),
        (Some(1), Some(0), Some(1))
    );
    assert_eq!(cycle["offsetSemantics"], "covering-source-spelling");
    assert_eq!(
        cycle["sourceRoute"].as_array().unwrap().last().unwrap(),
        &json!({"output":1,"outputStart":0,"outputEnd":1,"input":0,"inputStart":1,"inputEnd":2,"displayAnchor":1})
    );
    let inventory = mapped_fixture("language-grammar-inventory.lino");
    let invalid = inventory["languages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["name"] == "Python")
        .unwrap()["recoverySource"]
        .as_str()
        .unwrap();
    let python = HashMap::from([("Python".to_owned(), mapped_python_grammar())]);
    assert!(!mapped_parse(&python["Python"], &HashMap::new(), invalid).ok);
    let rejected = mapped_parse(
        &mapped_json_host(&[("Python", "json-string")], "any-grammar", ""),
        &python,
        &serde_json::to_string(invalid).unwrap(),
    );
    assert!(!rejected.ok);
    assert!(rejected.tree.is_none());
    assert_eq!(rejected.source_planes.as_ref().unwrap().len(), 2);
    let rejection = rejected.rejection.as_ref().unwrap();
    assert_eq!(rejection.reason, "syntax");
    let diagnostic = rejection
        .diagnostics
        .as_ref()
        .unwrap()
        .iter()
        .find(|item| item["reason"] == "mappedSyntax" && item["language"] == "Python")
        .unwrap();
    assert_eq!(diagnostic["source"], 1);
    assert_eq!(diagnostic["offsetSemantics"], "covering-source-spelling");
    assert_eq!(
        diagnostic["sourceRoute"]
            .as_array()
            .unwrap()
            .last()
            .unwrap()["input"],
        0
    );
    assert!(
        check_grammar_selection_mapped_tree(
            &mapped_bundle(&rejected),
            &FeatureParseOptions::default()
        )
        .unwrap_err()
        .message
        .contains("complete physical root CST")
    );
    mapping_transport::record_mapped_profiles(invalid);
}

#[test]
fn mapped_selection_memo_keys_keep_delimiter_bearing_names_separate_from_profiles() {
    let source = "\"x\"";
    let identity_name = "json-string|decoded";
    let grammars = HashMap::from([
        (identity_name.to_owned(), mapped_literal(source)),
        ("decoded".to_owned(), mapped_literal("x")),
    ]);
    let mapped = mapped_json_host(&[("decoded", "json-string")], "any-grammar", "");
    let identity = mapped_json_host(&[(identity_name, "identity")], "any-grammar", "");
    for warm_mapping in ["identity", "json-string"] {
        let (warm, selected, mapping, name, child_source) = if warm_mapping == "identity" {
            (&identity, &mapped, "json-string", "decoded", "x")
        } else {
            (&mapped, &identity, "identity", identity_name, source)
        };
        let expression = GrammarExpr::Sequence(vec![
            GrammarExpr::And(Box::new(warm.rule("mapped_document").unwrap().expr.clone())),
            selected.rule("mapped_document").unwrap().expr.clone(),
        ]);
        let host = mapping_transport::mapped_root_expression(&mapped, expression);
        let outcome = mapped_parse(&host, &grammars, source);
        assert!(outcome.ok, "{:?}", outcome.rejection);
        let bundle = mapped_bundle(&outcome);
        let children = bundle["tree"]["children"].as_array().unwrap();
        assert_eq!(
            children.len(),
            1,
            "warming lookahead retains no selected CST"
        );
        let selected = &children[0];
        assert_eq!(selected["language"], name);
        assert_eq!(selected["selection"]["mapping"], mapping);
        assert_eq!(
            selected["root"],
            mapped_tree_data(
                mapped_parse(&grammars[name], &HashMap::new(), child_source)
                    .tree
                    .as_ref()
                    .unwrap()
            )
        );
        assert_eq!(bundle["sourcePlanes"].as_array().unwrap().len(), 2);
        assert_eq!(mapped_physical(&bundle["tree"]), source.as_bytes());
        check_grammar_selection_mapped_tree(&bundle, &FeatureParseOptions::default()).unwrap();
    }
    mapping_transport::record_mapped_memo();
}
