//! Shared authored grammar data only; actual catalog language coverage is
//! independent. These tests execute only in CI, never local native builds.
use std::collections::HashMap;

use meta_language::{
    FeatureParseOptions, GlobalConceptOptions, GlobalConceptSource, Grammar, LeafText, SyntaxTree,
    compile_feature_grammar, create_global_concept_space, global_concept_space_to_links,
    parse_grammar_links, read_global_concept_space_links, render_grammar_links,
};
use serde_json::{Value, json};

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
