//! CI-only mirrors of the six JavaScript grammar-composition controls.
//! The production decorators, generated policies and mapped-CST checker own
//! all decisions; helpers below transport public grammar/CST data only.
use super::grammar_included_range_transport as transport;
use transport::{
    included_range_bundle as bundle, included_range_physical as physical,
    included_range_stored as stored, included_range_tree_data as tree_data,
};

#[path = "grammar_composition_transport.rs"]
mod composition_transport;
use composition_transport::*;

#[test]
fn composition_decisions_are_closed_and_reject_context_candidate_and_source_name_collisions() {
    use rules::*;
    assert_eq!(
        read_grammar_composition_field_order("selection"),
        "kind|item|items"
    );
    assert_eq!(read_grammar_composition_field_order("unknown"), "");
    for mode in ["any-grammar", "all-grammar"] {
        assert_eq!(
            read_grammar_composition_context_issue("normal", mode, "empty", false),
            ""
        );
    }
    for (kind, mode, slot, trees) in [
        ("silent", "any-grammar", "empty", false),
        ("normal", "choice", "empty", false),
        ("normal", "any-grammar", "literal", false),
        ("normal", "any-grammar", "empty", true),
    ] {
        assert_ne!(
            read_grammar_composition_context_issue(kind, mode, slot, trees),
            ""
        );
    }
    assert!(accept_grammar_composition_candidate_profile(
        "grammar-field-candidate",
        "field-ranges"
    ));
    assert!(accept_grammar_composition_candidate_profile(
        "grammar-candidate",
        "json-string"
    ));
    assert!(!accept_grammar_composition_candidate_profile(
        "grammar-candidate",
        "identity"
    ));
    for columns in [
        [
            vec!["grammar-candidate"],
            vec!["json-string"],
            vec!["Json"],
            vec![""],
        ],
        [
            vec!["grammar-field-candidate", "grammar-field-candidate"],
            vec!["field-ranges", "field-ranges"],
            vec!["A", "B"],
            vec!["same", "same"],
        ],
    ] {
        assert_eq!(
            read_grammar_composition_candidate_issue(
                &strings(&columns[0]),
                &strings(&columns[1]),
                &strings(&columns[2]),
                &strings(&columns[3])
            ),
            ""
        );
    }
    for columns in [
        [vec![], vec![], vec![], vec![]],
        [vec!["grammar-candidate"], vec![], vec!["A"], vec![""]],
        [
            vec!["grammar-candidate"],
            vec!["json-string"],
            vec![""],
            vec![""],
        ],
        [
            vec!["grammar-candidate", "grammar-candidate"],
            vec!["json-string", "json-string"],
            vec!["A", "A"],
            vec!["", ""],
        ],
        [
            vec!["grammar-candidate", "grammar-candidate"],
            vec!["json-string", "field-ranges"],
            vec!["A", "B"],
            vec!["", ""],
        ],
        [
            vec!["grammar-field-candidate", "grammar-field-candidate"],
            vec!["field-ranges", "field-ranges"],
            vec!["A", "B"],
            vec!["first", "second"],
        ],
        [
            vec!["grammar-field-candidate"],
            vec!["json-string"],
            vec!["A"],
            vec![""],
        ],
        [
            vec!["grammar-candidate"],
            vec!["included-ranges"],
            vec!["A"],
            vec![""],
        ],
    ] {
        assert_ne!(
            read_grammar_composition_candidate_issue(
                &strings(&columns[0]),
                &strings(&columns[1]),
                &strings(&columns[2]),
                &strings(&columns[3])
            ),
            ""
        );
    }
    for (actual, key, requested, next, opcode) in [
        ("any-grammar", "full", "any-grammar", "full", 1),
        ("any-grammar", "first", "any-grammar", "second", 2),
        ("all-grammar", "full", "any-grammar", "full", 2),
    ] {
        assert_eq!(
            ml_array::number_index(read_grammar_composition_disposition(
                actual, key, requested, next
            )),
            Some(opcode)
        );
    }
    assert_eq!(read_grammar_composition_publication_issue(false, true), "");
    assert_eq!(read_grammar_composition_publication_issue(true, false), "");
    assert_ne!(read_grammar_composition_publication_issue(true, true), "");
}

#[test]
fn composition_materializes_the_original_expression_once_and_preserves_fallback_metadata_and_renaming()
 {
    let initial = parse_grammar_links("(grammar (start root))\n(rule root normal (choice ordered (ref item) (literal %222%22)) (concept grammar.root) (source-names (own original_root)))\n(rule item normal (literal %221%22))\n").unwrap();
    let mut rule = initial.start_rule().unwrap().clone();
    rule.doc = Some("Original complete rule documentation.".to_owned());
    let mut grammar = replaced(&initial, "root", &rule);
    grammar.set_kinds(vec![GrammarKind {
        name: "kept".to_owned(),
        source_names: vec![GrammarSourceName {
            source: "own".to_owned(),
            name: "old_kept".to_owned(),
        }],
    }]);
    let original = grammar.clone();
    assert_eq!(
        decorate_grammar(
            &grammar,
            &DecoratorSet::default(),
            DecoratorLevel::GrammarRule
        )
        .unwrap(),
        grammar
    );
    let absent = DecoratorSet::new(vec![author(
        "none",
        DecoratorLevel::GrammarRule,
        "absent",
        "not parsed because it did not match",
    )])
    .unwrap();
    assert_eq!(
        decorate_grammar(&grammar, &absent, DecoratorLevel::GrammarRule).unwrap(),
        grammar
    );
    let set = policy(&grammar, vec![json_candidate("Json")], "any-grammar");
    let wrapped = decorate_grammar(&grammar, &set, DecoratorLevel::GrammarRule).unwrap();
    assert_eq!(
        item(&wrapped.start_rule().unwrap().expr),
        &grammar.start_rule().unwrap().expr
    );
    let mut conserved = wrapped.start_rule().unwrap().clone();
    conserved.expr = grammar.start_rule().unwrap().expr.clone();
    assert_eq!(conserved, *grammar.start_rule().unwrap());
    assert_eq!(wrapped.kinds(), grammar.kinds());
    assert_eq!(wrapped.declarations(), grammar.declarations());
    assert_eq!(
        decorate_grammar(&wrapped, &set, DecoratorLevel::GrammarRule).unwrap(),
        wrapped
    );
    assert_eq!(
        decorate_grammar(
            &grammar,
            &set.remove("compose").unwrap(),
            DecoratorLevel::GrammarRule
        )
        .unwrap(),
        grammar
    );
    let template = render_links_expression(
        &parse_links_expression("(any-grammar empty (grammar-candidate Json json-string))")
            .unwrap(),
    );
    let importer = DecoratorSet::new(vec![author(
        "compose",
        DecoratorLevel::Importer,
        "root",
        &template,
    )])
    .unwrap();
    assert_eq!(
        item(
            &decorate_grammar(&grammar, &importer, DecoratorLevel::Importer)
                .unwrap()
                .start_rule()
                .unwrap()
                .expr
        ),
        &grammar.start_rule().unwrap().expr
    );
    let later = Decorator::new(
        "later",
        DecoratorLevel::GrammarRule,
        2,
        vec![("name".to_owned(), "root".to_owned())],
        vec![DecoratorAction::Replace {
            field: "composition".to_owned(),
            from: "Json".to_owned(),
            to: "RenamedSource".to_owned(),
        }],
    )
    .unwrap();
    let ordered = set.add([later]).unwrap();
    assert_eq!(
        form(
            &candidates(
                &decorate_grammar(&grammar, &ordered, DecoratorLevel::GrammarRule)
                    .unwrap()
                    .start_rule()
                    .unwrap()
                    .expr
            )[0]
        )
        .fields[0],
        FieldValue::Name("RenamedSource".to_owned())
    );
    let sources = HashMap::from([("Json".to_owned(), json_grammar())]);
    for source in ["\"1\"", "\"2\""] {
        let baseline = parse(&grammar, &HashMap::new(), source);
        let outcome = parse(&wrapped, &sources, source);
        let result = mapped(&outcome, source);
        assert_eq!(
            original_trees(&result["tree"]),
            vec![tree_data(baseline.tree.as_ref().unwrap())]
        );
        let resolve = |name: &str| sources.get(name).cloned();
        let options = FeatureParseOptions {
            decorators: set.clone(),
            ..FeatureParseOptions::default()
        };
        for input in [&grammar, &wrapped] {
            let actual = compile_feature_grammar(input, Some(&resolve), options.clone())
                .unwrap()
                .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
                .unwrap();
            assert_eq!(
                actual, outcome,
                "compilation with the same policy is idempotent"
            );
        }
    }
    let rename = Decorator::new(
        "rename",
        DecoratorLevel::GrammarRule,
        0,
        vec![("name".to_owned(), "item".to_owned())],
        vec![DecoratorAction::Set {
            field: "name".to_owned(),
            value: "leaf".to_owned(),
        }],
    )
    .unwrap();
    let renamed_set = policy(&grammar, vec![json_candidate("item")], "any-grammar")
        .add([rename])
        .unwrap();
    let renamed = decorate_grammar(&grammar, &renamed_set, DecoratorLevel::GrammarRule).unwrap();
    assert!(
        render_links_expression(item(&renamed.start_rule().unwrap().expr)).contains("(ref leaf)")
    );
    assert_eq!(
        form(&candidates(&renamed.start_rule().unwrap().expr)[0]).fields[0],
        FieldValue::Name("item".to_owned())
    );
    let old_only = decorate_grammar(
        &grammar,
        &renamed_set.remove("compose").unwrap(),
        DecoratorLevel::GrammarRule,
    )
    .unwrap();
    assert_eq!(
        item(&renamed.start_rule().unwrap().expr),
        &old_only.start_rule().unwrap().expr
    );
    for source_rule in old_only.rules() {
        let mut actual = renamed.rule(&source_rule.name).unwrap().clone();
        actual.expr = source_rule.expr.clone();
        assert_eq!(&actual, source_rule);
    }
    let collision = Decorator::new(
        "collide",
        DecoratorLevel::GrammarRule,
        0,
        vec![("name".to_owned(), "item".to_owned())],
        vec![DecoratorAction::Set {
            field: "name".to_owned(),
            value: "root".to_owned(),
        }],
    )
    .unwrap();
    assert!(
        decorate_grammar(
            &grammar,
            &set.add([collision]).unwrap(),
            DecoratorLevel::GrammarRule
        )
        .unwrap_err()
        .message
        .contains("capture the existing name")
    );
    let distinct = composed(&wrapped, vec![json_candidate("Other")], "any-grammar");
    assert_eq!(
        item(&distinct.start_rule().unwrap().expr),
        &wrapped.start_rule().unwrap().expr
    );
    let identity = parse_grammar_links("(grammar (start root))\n(rule root normal (any-grammar (literal %221%22) (grammar-candidate Old identity)))\n").unwrap();
    assert_eq!(
        item(
            &composed(&identity, vec![json_candidate("Json")], "any-grammar")
                .start_rule()
                .unwrap()
                .expr
        ),
        &identity.start_rule().unwrap().expr
    );
    let ordered = composed(
        &grammar,
        vec![json_candidate("First"), json_candidate("Second")],
        "any-grammar",
    );
    assert_eq!(
        composed(
            &ordered,
            vec![json_candidate("First"), json_candidate("Second")],
            "any-grammar"
        ),
        ordered
    );
    assert_eq!(
        item(
            &composed(
                &ordered,
                vec![json_candidate("Second"), json_candidate("First")],
                "any-grammar"
            )
            .start_rule()
            .unwrap()
            .expr
        ),
        &ordered.start_rule().unwrap().expr
    );
    let selected = composed(&grammar, vec![field_candidate("Json", 0)], "any-grammar");
    assert_eq!(
        item(
            &composed(&selected, vec![field_candidate("Json", 1)], "any-grammar")
                .start_rule()
                .unwrap()
                .expr
        ),
        &selected.start_rule().unwrap().expr
    );
    assert_eq!(
        grammar, original,
        "all original grammar data remains unchanged"
    );
}

#[test]
fn composition_rejects_malformed_templates_idempotence_data_and_mapped_witness_rewrites() {
    let grammar = json_grammar();
    for value in [
        "(any-grammar empty (grammar-candidate Json json-string)) empty",
        "(any-grammar (literal x) (grammar-candidate Json json-string))",
        "(seq empty)",
        "(any-grammar empty)",
        "(any-grammar empty (grammar-candidate Json identity))",
        "(any-grammar empty (grammar-candidate Json json-string) (grammar-candidate Json json-string))",
        "(any-grammar empty (grammar-candidate Json json-string) (grammar-field-candidate Child field-ranges (source-field-selector value include 0 1)))",
        "(any-grammar empty (grammar-field-candidate Json field-ranges (source-field-selector value include 0 0)))",
        "(any-grammar empty (grammar-field-candidate Json field-ranges (source-field-selector value include 0 1)) (grammar-field-candidate Other field-ranges (source-field-selector value include 1 1)))",
        "(any-grammar empty (grammar-candidate Json json-string extra))",
    ] {
        let set = DecoratorSet::new(vec![author(
            "bad",
            DecoratorLevel::GrammarRule,
            grammar.start().unwrap(),
            value,
        )])
        .unwrap();
        assert!(
            decorate_grammar(&grammar, &set, DecoratorLevel::GrammarRule).is_err(),
            "{value}"
        );
    }
    let empty = DecoratorSet::new(vec![author(
        "empty",
        DecoratorLevel::GrammarRule,
        grammar.start().unwrap(),
        "",
    )])
    .unwrap();
    assert_eq!(
        decorate_grammar(&grammar, &empty, DecoratorLevel::GrammarRule).unwrap(),
        grammar
    );
    let set = policy(&grammar, vec![field_candidate("Json", 0)], "any-grammar");
    for level in [DecoratorLevel::Executor, DecoratorLevel::Recovery] {
        let tree = Decorator::new(
            "tree",
            level,
            0,
            vec![],
            vec![DecoratorAction::Set {
                field: if level == DecoratorLevel::Executor {
                    "kind"
                } else {
                    "reason"
                }
                .to_owned(),
                value: "changed".to_owned(),
            }],
        )
        .unwrap();
        assert!(
            decorate_grammar(
                &grammar,
                &set.add([tree.clone()]).unwrap(),
                DecoratorLevel::GrammarRule
            )
            .unwrap_err()
            .message
            .contains("source-witness proof")
        );
        let tree_policy = DecoratorSet::new(vec![tree]).unwrap();
        let materialized = composed(&grammar, vec![field_candidate("Json", 0)], "any-grammar");
        let restored = deserialize_grammar(&serialize_grammar(&materialized)).unwrap();
        let resolve = |_: &str| Some(json_grammar());
        for source in [&materialized, &restored] {
            for compiled_policy in [true, false] {
                let decorated = FeatureParseOptions {
                    decorators: tree_policy.clone(),
                    ..FeatureParseOptions::default()
                };
                let options = if compiled_policy {
                    decorated.clone()
                } else {
                    FeatureParseOptions::default()
                };
                let parser = compile_feature_grammar(source, Some(&resolve), options).unwrap();
                let error = parser.parse_tree(b"{\"k\":1}", &decorated).unwrap_err();
                assert_eq!(error.reason, "decorator");
                assert!(error.message.contains("source-witness proof"));
            }
        }
    }
    let kind = Decorator::new(
        "kind",
        DecoratorLevel::GrammarRule,
        0,
        vec![("name".to_owned(), grammar.start().unwrap().to_owned())],
        vec![DecoratorAction::Set {
            field: "kind".to_owned(),
            value: "silent".to_owned(),
        }],
    )
    .unwrap();
    assert!(
        decorate_grammar(
            &grammar,
            &set.add([kind]).unwrap(),
            DecoratorLevel::GrammarRule
        )
        .unwrap_err()
        .message
        .contains("normal syntactic rule")
    );
    // Rust has owned typed fields, not JS accessors/prototypes/sparse arrays.
    // Preserve corresponding malformed arity/type/count inputs at the actual IR.
    for mutation in 0..5 {
        let wrapped = decorate_grammar(&grammar, &set, DecoratorLevel::GrammarRule).unwrap();
        let mut root = wrapped.start_rule().unwrap().clone();
        let selection = form_mut(&mut root.expr);
        if mutation == 4 {
            selection.fields[0] = FieldValue::Name("erased-original".to_owned());
        } else {
            let FieldValue::Expressions(items) = &mut selection.fields[1] else {
                panic!("actual candidates")
            };
            if mutation == 3 {
                items.clear();
            } else {
                let candidate = form_mut(&mut items[0]);
                match mutation {
                    0 => {
                        candidate.fields.push(FieldValue::Name("extra".to_owned()));
                    }
                    1 => {
                        candidate.fields[2] = FieldValue::Name("missing-selector".to_owned());
                    }
                    _ => {
                        let FieldValue::Expression(selector) = &mut candidate.fields[2] else {
                            panic!("actual selector")
                        };
                        form_mut(selector).fields[3] = FieldValue::Integer(0);
                    }
                }
            }
        }
        let forged = replaced(&wrapped, wrapped.start().unwrap(), &root);
        assert!(
            decorate_grammar(&forged, &set, DecoratorLevel::GrammarRule).is_err(),
            "mutation {mutation}"
        );
    }
}

#[test]
fn opt_in_composition_preserves_full_shipped_priority_csts_utf8_and_all_alternatives() {
    let payload = r#"{"é":"β"}"#;
    let quoted = serde_json::to_string(payload).unwrap();
    let json = json_grammar();
    let quoted_grammar = composed(&json_grammar(), vec![json_candidate("Json")], "any-grammar");
    for (document, source) in [
        (
            include_str!("../../src/data/native-grammars/javascript.lino"),
            format!("// 前\nconst data = {quoted};"),
        ),
        (
            include_str!("../../src/data/native-grammars/typescript.lino"),
            format!("// 前\nconst data: string = {quoted};"),
        ),
        (
            include_str!("../../src/data/native-grammars/rust.lino"),
            format!("// 前\nfn main(){{let data = {quoted};}}"),
        ),
    ] {
        let grammar = parse_grammar_links(document).unwrap();
        let original = grammar.clone();
        let listing = render_grammar_links(&grammar);
        let baseline = parse(&grammar, &HashMap::new(), &source);
        assert!(baseline.ok, "{:?}", baseline.rejection);
        let baseline = tree_data(baseline.tree.as_ref().unwrap());
        let capture = first_field(&baseline).unwrap();
        let start = usize::try_from(capture["start"].as_u64().unwrap()).unwrap();
        let end = usize::try_from(capture["end"].as_u64().unwrap()).unwrap();
        let selected = &source[start..end];
        for mode in ["any-grammar", "all-grammar"] {
            let host = composed(
                &grammar,
                vec![field_candidate("QuotedA", 0), field_candidate("QuotedB", 0)],
                mode,
            );
            let sources = HashMap::from([
                ("QuotedA".to_owned(), quoted_grammar.clone()),
                ("QuotedB".to_owned(), quoted_grammar.clone()),
                ("Json".to_owned(), json.clone()),
            ]);
            let result = mapped(&parse(&host, &sources, &source), &source);
            assert_eq!(original_trees(&result["tree"]), vec![baseline.clone()]);
            let first = embedding(&result["tree"], "field-ranges");
            assert_eq!(
                first["selection"]["alternatives"].as_array().unwrap().len(),
                1
            );
            assert_eq!(
                first["selection"]["candidates"],
                json!([{"language":"QuotedA","relation":"accepted"},{"language":"QuotedB","relation":"accepted"}])
            );
            assert_eq!(physical(&first["root"]), selected.as_bytes());
            assert_eq!(
                first["selection"]["sourceMapping"]["ranges"],
                json!([[start, end]])
            );
            let decoded = embedding(&first["root"], "json-string");
            let standalone = parse(&json, &HashMap::new(), payload);
            assert_eq!(
                decoded["root"],
                tree_data(standalone.tree.as_ref().unwrap())
            );
            assert_eq!(physical(&decoded["root"]), payload.as_bytes());
            for copy in stored(&result) {
                check_grammar_selection_mapped_tree(&copy, &FeatureParseOptions::default())
                    .unwrap();
                assert_eq!(copy, result);
            }
            assert_eq!(grammar, original);
            assert_eq!(render_grammar_links(&grammar), listing);
        }
    }
    let grammar = parse_grammar_links(include_str!(
        "../../src/data/native-grammars/javascript.lino"
    ))
    .unwrap();
    let other = serde_json::to_string(r#"{"n":2}"#).unwrap();
    let source = format!("const first = {quoted}; const second = {other};");
    let baseline = parse(&grammar, &HashMap::new(), &source);
    let baseline = tree_data(baseline.tree.as_ref().unwrap());
    let second = baseline["children"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|node| node["type"] == "node")
        .nth(1)
        .unwrap();
    let capture = first_field(second).unwrap();
    let start = usize::try_from(capture["start"].as_u64().unwrap()).unwrap();
    let end = usize::try_from(capture["end"].as_u64().unwrap()).unwrap();
    let sources = HashMap::from([
        ("Quoted".to_owned(), quoted_grammar),
        ("Json".to_owned(), json),
    ]);
    let result = mapped(
        &parse(
            &composed(&grammar, vec![field_candidate("Quoted", 1)], "any-grammar"),
            &sources,
            &source,
        ),
        &source,
    );
    assert_eq!(original_trees(&result["tree"]), vec![baseline]);
    assert_eq!(
        physical(&embedding(&result["tree"], "field-ranges")["root"]),
        source.as_bytes()[start..end]
    );
}

#[test]
fn recursive_composition_retains_four_source_planes_and_source_free_text_binary_archives() {
    let mut sources = HashMap::from([
        (
            "Host".to_owned(),
            composed(
                &json_grammar(),
                vec![field_candidate("Quoted", 0)],
                "any-grammar",
            ),
        ),
        (
            "Quoted".to_owned(),
            composed(
                &json_grammar(),
                vec![json_candidate("Inner")],
                "any-grammar",
            ),
        ),
        (
            "Inner".to_owned(),
            composed(
                &json_grammar(),
                vec![field_candidate("Leaf", 0)],
                "any-grammar",
            ),
        ),
        ("Leaf".to_owned(), json_grammar()),
    ]);
    let source = r#"{"k":"{\"k\":2}"}"#;
    let host = sources["Host"].clone();
    let expected = parse(&host, &sources, source);
    let result = mapped(&expected, source);
    let planes = result["sourcePlanes"].as_array().unwrap();
    assert_eq!(
        planes
            .iter()
            .map(|plane| json!([
                plane["identity"],
                plane["byteLength"],
                plane["derivation"].get("operation"),
                plane["derivation"].get("input")
            ]))
            .collect::<Vec<_>>(),
        vec![
            json!([0, 17, null, null]),
            json!([1, 11, "field-ranges", 0]),
            json!([2, 7, "json-string", 1]),
            json!([3, 1, "field-ranges", 2])
        ]
    );
    let first = embedding(&result["tree"], "field-ranges");
    let last = embedding(&first["root"], "field-ranges");
    assert_eq!(physical(&last["root"]), b"2");
    let set = policy(
        &json_grammar(),
        vec![field_candidate("Quoted", 0)],
        "any-grammar",
    );
    assert_eq!(DecoratorSet::from_lino(&set.to_lino()).unwrap(), set);
    // Native grammar Links are already a complete inspectable definition
    // format. Archive their official parsed forms, not opaque listing strings
    // or the JSON serializer's normalized metadata projection.
    let definitions = Value::Object(
        sources
            .iter()
            .map(|(name, grammar)| {
                let parsed = parse_lino_to_links_with_config(
                    &render_grammar_links(grammar),
                    &ParserConfig::without_comments(),
                )
                .unwrap();
                (
                    name.clone(),
                    json!(
                        parsed
                            .iter()
                            .map(transport::included_range_link_data)
                            .collect::<Vec<_>>()
                    ),
                )
            })
            .collect(),
    );
    for copy in stored(&json!({"definitions":definitions,"policy":set.to_lino(),"result":result})) {
        assert_eq!(
            DecoratorSet::from_lino(copy["policy"].as_str().unwrap()).unwrap(),
            set
        );
        let restored = copy["definitions"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(name, data)| {
                let listing = data
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(transport::included_range_link_text)
                    .collect::<Vec<_>>()
                    .join("\n");
                (name.clone(), parse_grammar_links(&listing).unwrap())
            })
            .collect::<HashMap<_, _>>();
        assert_eq!(
            restored, sources,
            "complete original grammar definitions and metadata survive"
        );
        assert_eq!(parse(&restored["Host"], &restored, source), expected);
        assert_eq!(copy["result"], result);
        check_grammar_selection_mapped_tree(&copy["result"], &FeatureParseOptions::default())
            .unwrap();
        let mut forged = copy["result"].clone();
        forged["sourcePlanes"][3]["derivation"]["input"] = json!(0);
        assert!(
            check_grammar_selection_mapped_tree(&forged, &FeatureParseOptions::default()).is_err()
        );
        let mut coordinated = copy["result"].clone();
        coordinated["tree"]["children"][0]["selection"]["sourceMapping"]["selector"]["first"] =
            json!(1);
        coordinated["sourcePlanes"][1]["derivation"]["selector"]["first"] = json!(1);
        assert!(
            check_grammar_selection_mapped_tree(&coordinated, &FeatureParseOptions::default())
                .is_err()
        );
    }
    let resolve = |name: &str| sources.get(name).cloned();
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
        let outcome = parser.parse_tree(source.as_bytes(), &options).unwrap();
        assert!(!outcome.ok);
        assert_eq!(outcome.rejection.unwrap().reason, reason);
    }
    let settings = GlobalConceptOptions {
        records: Some(json!([])),
        ..GlobalConceptOptions::default()
    };
    let mut records = sources
        .iter()
        .map(|(id, grammar)| GlobalConceptSource {
            id: id.clone(),
            language: id.clone(),
            edition: String::new(),
            grammar: grammar.clone(),
        })
        .collect::<Vec<_>>();
    let space = create_global_concept_space(&records, &settings).unwrap();
    let global_options = FeatureParseOptions {
        grammar_name: Some("Host".to_owned()),
        global_concept_space: Some(space.clone()),
        ..FeatureParseOptions::default()
    };
    let expected = compile_feature_grammar(&host, Some(&resolve), global_options)
        .unwrap()
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .unwrap();
    mapped(&expected, source);
    let archive = global_concept_space_to_links(&space).unwrap();
    let syntax =
        parse_lino_to_links_with_config(&archive, &ParserConfig::without_comments()).unwrap();
    let syntax = json!(
        syntax
            .iter()
            .map(transport::included_range_link_data)
            .collect::<Vec<_>>()
    );
    records.clear();
    sources.clear();
    for copy in stored(&json!({"archive":archive,"syntax":syntax})) {
        assert_eq!(copy["archive"], archive);
        assert_eq!(copy["syntax"], syntax);
        let reconstructed = copy["syntax"]
            .as_array()
            .unwrap()
            .iter()
            .map(transport::included_range_link_text)
            .collect::<Vec<_>>()
            .join("\n");
        for listing in [copy["archive"].as_str().unwrap(), reconstructed.as_str()] {
            let parser = create_global_concept_space_parser(
                listing,
                "Host",
                &settings,
                FeatureParseOptions::default(),
            )
            .unwrap();
            let actual = parser
                .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
                .unwrap();
            assert_eq!(actual, expected);
            check_grammar_selection_mapped_tree(&bundle(&actual), &FeatureParseOptions::default())
                .unwrap();
        }
    }
}

#[test]
fn composition_retains_no_match_child_refusal_profile_boundaries_and_existing_fallback() {
    let json = json_grammar();
    let quoted = composed(&json_grammar(), vec![json_candidate("Json")], "any-grammar");
    let host = composed(
        &json_grammar(),
        vec![field_candidate("Quoted", 0)],
        "any-grammar",
    );
    let sources = HashMap::from([
        ("Quoted".to_owned(), quoted),
        ("Json".to_owned(), json.clone()),
    ]);
    assert!(parse(&json, &HashMap::new(), "{}").ok);
    let absent = parse(&host, &sources, "{}");
    assert!(!absent.ok);
    assert_eq!(absent.rejection.unwrap().reason, "invalidEmbeddingMapping");
    assert!(
        absent
            .source_planes
            .as_ref()
            .is_none_or(|planes| planes.len() == 1)
    );
    // Native resolution is eager at compile time, unlike JS runtime lookup.
    // No derived child plane or execution is claimed for an invalid window.
    let refused = parse(&host, &sources, r#"{"k":"not json"}"#);
    assert!(!refused.ok);
    assert_eq!(refused.source_planes.as_ref().unwrap().len(), 3);
    assert_eq!(
        refused
            .source_planes
            .as_ref()
            .unwrap()
            .iter()
            .map(|plane| plane["derivation"]
                .get("operation")
                .cloned()
                .unwrap_or(Value::Null))
            .collect::<Vec<_>>(),
        vec![Value::Null, json!("field-ranges"), json!("json-string")]
    );
    if refused.tree.is_none() {
        assert!(
            check_grammar_selection_mapped_tree(&bundle(&refused), &FeatureParseOptions::default())
                .is_err()
        );
    }
    for source in [r#"const data = '{"k":1}';"#, r#"const data = "\x31";"#] {
        let grammar = parse_grammar_links(include_str!(
            "../../src/data/native-grammars/javascript.lino"
        ))
        .unwrap();
        assert!(parse(&grammar, &HashMap::new(), source).ok);
        assert!(
            !parse(
                &composed(&grammar, vec![field_candidate("Quoted", 0)], "any-grammar"),
                &sources,
                source
            )
            .ok
        );
    }
    let rust =
        parse_grammar_links(include_str!("../../src/data/native-grammars/rust.lino")).unwrap();
    let raw = r##"fn main(){let data = r#"{"k":1}"#;}"##;
    assert!(parse(&rust, &HashMap::new(), raw).ok);
    assert!(
        !parse(
            &composed(&rust, vec![field_candidate("Quoted", 0)], "any-grammar"),
            &sources,
            raw
        )
        .ok
    );
    let root = json.start_rule().unwrap();
    let mut fallback_rule = root.clone();
    fallback_rule.expr = GrammarExpr::Choice {
        ordered: true,
        alternatives: vec![
            GrammarExpr::NonTerminal("attempt".to_owned()),
            root.expr.clone(),
        ],
    };
    let mut fallback = replaced(&json, &root.name, &fallback_rule);
    fallback.add_rule(GrammarRule::new("attempt", root.expr.clone()));
    let template = render_links_expression(&parse_links_expression("(any-grammar empty (grammar-field-candidate Refusing field-ranges (source-field-selector value include 0 1)))").unwrap());
    let set = DecoratorSet::new(vec![author(
        "attempt-only",
        DecoratorLevel::GrammarRule,
        "attempt",
        &template,
    )])
    .unwrap();
    let retained = decorate_grammar(&fallback, &set, DecoratorLevel::GrammarRule).unwrap();
    assert_eq!(
        retained.start_rule().unwrap().expr,
        fallback.start_rule().unwrap().expr
    );
    assert_eq!(item(&retained.rule("attempt").unwrap().expr), &root.expr);
    let refusing =
        parse_grammar_links("(grammar (start absent))\n(rule absent normal (literal absent))\n")
            .unwrap();
    let outcome = parse(
        &retained,
        &HashMap::from([("Refusing".to_owned(), refusing)]),
        r#"{"k":1}"#,
    );
    let result = mapped(&outcome, r#"{"k":1}"#);
    let mut nodes = Vec::new();
    embeddings(&result["tree"], &mut nodes);
    assert_eq!(nodes.as_slice(), []);
    assert_eq!(outcome.source_planes.as_ref().unwrap().len(), 2);
    assert_eq!(
        outcome.source_planes.as_ref().unwrap()[1]["derivation"]["operation"],
        "field-ranges"
    );
    let cycle = parse_grammar_links("(grammar (start cycle))\n(rule cycle normal (any-grammar (literal x) (grammar-candidate Cycle identity)))\n").unwrap();
    let cyclic = parse(
        &composed(
            &json_grammar(),
            vec![json_candidate("Cycle")],
            "any-grammar",
        ),
        &HashMap::from([("Cycle".to_owned(), cycle)]),
        "\"x\"",
    );
    assert!(!cyclic.ok);
    assert_eq!(
        cyclic.rejection.unwrap().reason,
        "nonConsumingEmbeddingCycle"
    );
}
