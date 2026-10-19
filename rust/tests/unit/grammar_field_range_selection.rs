//! Native CI controls for the authored JavaScript field-selection programs.
//! All decisions use the production parser, generated policies and checker.
use super::*;
use meta_language::grammar::interchange::{deserialize_grammar, serialize_grammar};
use meta_language::grammar::{FeatureExpr, FeatureForm, FieldValue};
use meta_language::{
    GlobalConceptOptions, GlobalConceptSource, create_global_concept_space,
    create_global_concept_space_parser, global_concept_space_to_links,
    read_global_concept_space_links,
};

fn field_range_json() -> Grammar {
    parse_grammar_links(include_str!("../../src/data/native-grammars/json.lino")).unwrap()
}

// Replace only the actual start expression, retaining every source rule,
// declaration, kind, alias and grammar format. No generated listing is edited.
fn field_range_with_expression(grammar: &Grammar, expression: &GrammarExpr) -> Grammar {
    let start = grammar.start_rule().unwrap().name.clone();
    let mut result = Grammar::new()
        .with_start(start.clone())
        .with_declarations(grammar.declarations().clone());
    result.set_kinds(grammar.kinds().to_vec());
    if let Some(format) = grammar.source_format() {
        result.set_source_format(format);
    }
    for source_rule in grammar.rules() {
        let mut rule = source_rule.clone();
        if rule.name == start {
            rule.expr = expression.clone();
        }
        result.add_rule(rule);
    }
    result
}

fn field_range_host(
    grammar: &Grammar,
    names: &[&str],
    first: i64,
    count: i64,
    mode: &str,
    policy: &str,
) -> Grammar {
    let candidates = names.iter().map(|name| parse_links_expression(&format!(
        "(grammar-field-candidate {} field-ranges (source-field-selector value {mode} {first} {count}))",
        percent_encode_links_text(name),
    )).unwrap()).collect();
    field_range_with_expression(
        grammar,
        &GrammarExpr::feature(FeatureExpr::Form(FeatureForm::new(
            policy,
            vec![
                FieldValue::Expression(grammar.start_rule().unwrap().expr.clone()),
                FieldValue::Expressions(candidates),
            ],
        ))),
    )
}

fn field_range_embedding_path(tree: &Value, path: &str) -> Option<String> {
    if tree["type"] == "embed" && tree["selection"]["mapping"] == "field-ranges" {
        return Some(path.to_owned());
    }
    for field in ["children", "sourceChildren"] {
        if let Some(children) = tree[field].as_array() {
            for (index, child) in children.iter().enumerate() {
                if let Some(found) =
                    field_range_embedding_path(child, &format!("{path}/{field}/{index}"))
                {
                    return Some(found);
                }
            }
        }
    }
    tree.get("root")
        .and_then(|root| field_range_embedding_path(root, &format!("{path}/root")))
}

fn field_range_embedding(tree: &Value) -> &Value {
    tree.pointer(&field_range_embedding_path(tree, "").unwrap())
        .unwrap()
}

fn field_range_settings() -> GlobalConceptOptions {
    GlobalConceptOptions {
        records: Some(json!([])),
        ..GlobalConceptOptions::default()
    }
}

fn field_range_sources(
    host: Grammar,
    child: Grammar,
    child_name: &str,
) -> Vec<GlobalConceptSource> {
    [("Host", host), (child_name, child)]
        .into_iter()
        .map(|(id, grammar)| GlobalConceptSource {
            id: id.to_owned(),
            language: id.to_owned(),
            edition: String::new(),
            grammar,
        })
        .collect()
}

#[test]
fn actual_json_fields_select_varying_topmost_values_and_utf8_bytes_through_every_codec() {
    let json = field_range_json();
    for (source, first, expected, ranges, pair_index) in [
        (r#"{"k":1}"#, 0, "1", json!([[5, 6]]), 1),
        (r#"{"k":{"x":23}}"#, 0, r#"{"x":23}"#, json!([[5, 13]]), 1),
        (r#"{"a":1,"b":2}"#, 1, "2", json!([[11, 12]]), 3),
        (r#"{"é":"β"}"#, 0, r#""β""#, json!([[6, 10]]), 1),
    ] {
        let host = field_range_host(&json, &["Json"], first, 1, "include", "any-grammar");
        let links = render_grammar_links(&host);
        let native = render_native_grammar(&host);
        assert_eq!(
            render_grammar_links(&parse_grammar_links(&links).unwrap()),
            links
        );
        assert_eq!(
            render_native_grammar(&parse_native_grammar(&native).unwrap()),
            native
        );
        let direct = included_range_parse(&json, &HashMap::new(), expected);
        assert!(direct.ok, "{:?}", direct.rejection);
        for restored in [
            host.clone(),
            parse_grammar_links(&links).unwrap(),
            parse_native_grammar(&native).unwrap(),
            deserialize_grammar(&serialize_grammar(&host)).unwrap(),
        ] {
            assert_eq!(
                restored.start_rule().unwrap().expr,
                host.start_rule().unwrap().expr
            );
            let outcome = included_range_parse(
                &restored,
                &HashMap::from([("Json".to_owned(), json.clone())]),
                source,
            );
            assert!(outcome.ok, "{:?}", outcome.rejection);
            let bundle = included_range_bundle(&outcome);
            let embedded = field_range_embedding(&bundle["tree"]);
            let mapping = &embedded["selection"]["sourceMapping"];
            assert_eq!(mapping["ranges"], ranges);
            assert_eq!(
                mapping["occurrences"],
                json!([{
                    "path":["children",0,"children",pair_index,"children",2],
                    "start":ranges[0][0],"end":ranges[0][1],
                }])
            );
            assert_eq!(
                embedded["root"],
                included_range_tree_data(direct.tree.as_ref().unwrap())
            );
            assert_eq!(included_range_physical(&bundle["tree"]), source.as_bytes());
            assert_eq!(
                included_range_physical(&embedded["root"]),
                expected.as_bytes()
            );
            assert_eq!(bundle["sourcePlanes"].as_array().unwrap().len(), 2);
            let origin = &bundle["sourcePlanes"][1]["derivation"];
            assert_eq!(origin["operation"], "field-ranges");
            assert_eq!(
                origin["selector"],
                json!({"kind":"source-field-selector","label":"value",
                "mode":"include","first":first,"count":1})
            );
            assert_eq!(origin["ranges"], ranges);
            let physical = origin["witness"]["children"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(included_range_physical)
                .collect::<Vec<_>>();
            assert_eq!(physical, source.as_bytes());
            for stored in included_range_stored(&bundle) {
                check_grammar_selection_mapped_tree(&stored, &FeatureParseOptions::default())
                    .unwrap();
            }
        }
    }
    // Preserve the complete original grammar and the CST that owns leading
    // UTF-8 trivia and the padding before each actual value capture.
    let quoted = serde_json::to_string(r#"{"é":"β"}"#).unwrap();
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
        let listing = render_grammar_links(&grammar);
        let baseline = included_range_parse(&grammar, &HashMap::new(), &source);
        assert!(baseline.ok, "{:?}", baseline.rejection);
        let baseline = included_range_tree_data(baseline.tree.as_ref().unwrap());
        let mut pending = vec![String::new()];
        let capture = loop {
            let path = pending
                .pop()
                .expect("the original grammar retains a value capture");
            let node = baseline.pointer(&path).unwrap();
            if node["field"] == "value" && node["trivia"] != true {
                break path;
            }
            if let Some(children) = node["children"].as_array() {
                for index in (0..children.len()).rev() {
                    pending.push(format!("{path}/children/{index}"));
                }
            }
        };
        let node = baseline.pointer(&capture).unwrap();
        let start = usize::try_from(node["start"].as_u64().unwrap()).unwrap();
        let end = usize::try_from(node["end"].as_u64().unwrap()).unwrap();
        let quote_start = source
            .as_bytes()
            .windows(quoted.len())
            .position(|bytes| bytes == quoted.as_bytes())
            .unwrap();
        assert_eq!(&source.as_bytes()[start..quote_start], b" ");
        assert_eq!(end, quote_start + quoted.len());
        let selected = &source[start..end];
        let expected = included_range_parse(&json, &HashMap::new(), selected);
        assert!(expected.ok, "{:?}", expected.rejection);
        let capture_path = capture
            .trim_start_matches('/')
            .split('/')
            .map(|part| {
                if part == "children" {
                    json!(part)
                } else {
                    json!(part.parse::<usize>().unwrap())
                }
            })
            .collect::<Vec<_>>();
        let host = field_range_host(&grammar, &["Json"], 0, 1, "include", "any-grammar");
        let links = render_grammar_links(&host);
        let native = render_native_grammar(&host);
        assert_eq!(
            render_grammar_links(&parse_grammar_links(&links).unwrap()),
            links
        );
        assert_eq!(
            render_native_grammar(&parse_native_grammar(&native).unwrap()),
            native
        );
        for restored in [
            host.clone(),
            parse_grammar_links(&links).unwrap(),
            parse_native_grammar(&native).unwrap(),
            deserialize_grammar(&serialize_grammar(&host)).unwrap(),
        ] {
            assert_eq!(
                restored.start_rule().unwrap().expr,
                host.start_rule().unwrap().expr
            );
            let outcome = included_range_parse(
                &restored,
                &HashMap::from([("Json".to_owned(), json.clone())]),
                &source,
            );
            assert!(outcome.ok, "{:?}", outcome.rejection);
            let bundle = included_range_bundle(&outcome);
            let embedded = field_range_embedding(&bundle["tree"]);
            let children = bundle["tree"]["children"].as_array().unwrap();
            assert!(
                children.iter().any(|child| std::ptr::eq(child, embedded)),
                "the embedding is an original direct child"
            );
            let restored_children = children
                .iter()
                .flat_map(|child| {
                    if std::ptr::eq(child, embedded) {
                        child["sourceChildren"].as_array().unwrap().clone()
                    } else {
                        vec![child.clone()]
                    }
                })
                .collect::<Vec<_>>();
            let mut hierarchy = bundle["tree"].clone();
            hierarchy["children"] = json!(restored_children);
            assert_eq!(hierarchy, baseline);
            let mapping = &embedded["selection"]["sourceMapping"];
            assert_eq!(mapping["ranges"], json!([[start, end]]));
            assert_eq!(
                mapping["occurrences"],
                json!([{"path":capture_path,"start":start,"end":end}])
            );
            assert_eq!(bundle["sourcePlanes"].as_array().unwrap().len(), 2);
            let origin = &bundle["sourcePlanes"][1]["derivation"];
            assert_eq!(origin["start"], 0);
            assert_eq!(origin["end"], source.len());
            assert_eq!(origin["occurrences"], mapping["occurrences"]);
            assert_eq!(origin["witness"]["start"], 0);
            assert_eq!(origin["witness"]["end"], source.len());
            assert_eq!(
                origin["witness"]["children"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .flat_map(included_range_physical)
                    .collect::<Vec<_>>(),
                source.as_bytes()
            );
            assert_eq!(included_range_physical(&bundle["tree"]), source.as_bytes());
            assert_eq!(
                included_range_physical(&embedded["root"]),
                selected.as_bytes()
            );
            assert_eq!(
                embedded["root"],
                included_range_tree_data(expected.tree.as_ref().unwrap())
            );
            for stored in included_range_stored(&bundle) {
                assert_eq!(stored, bundle);
                check_grammar_selection_mapped_tree(&stored, &FeatureParseOptions::default())
                    .unwrap();
                let embedded = field_range_embedding(&stored["tree"]);
                let restored_children = stored["tree"]["children"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .flat_map(|child| {
                        if std::ptr::eq(child, embedded) {
                            child["sourceChildren"].as_array().unwrap().clone()
                        } else {
                            vec![child.clone()]
                        }
                    })
                    .collect::<Vec<_>>();
                let mut hierarchy = stored["tree"].clone();
                hierarchy["children"] = json!(restored_children);
                assert_eq!(hierarchy, baseline);
            }
        }
        assert_eq!(render_grammar_links(&grammar), listing);
    }
}

#[test]
fn any_and_all_field_candidates_keep_parallel_csts_and_one_completed_source_derivation() {
    let json = field_range_json();
    let grammars = HashMap::from([
        ("Json".to_owned(), json.clone()),
        ("JsonTwin".to_owned(), json.clone()),
    ]);
    for policy in ["any-grammar", "all-grammar"] {
        let host = field_range_host(&json, &["Json", "JsonTwin"], 0, 1, "include", policy);
        for (source, expected) in [(r#"{"k":1}"#, "1"), (r#"{"k":23}"#, "23")] {
            let outcome = included_range_parse(&host, &grammars, source);
            assert!(outcome.ok, "{:?}", outcome.rejection);
            let bundle = included_range_bundle(&outcome);
            let embedded = field_range_embedding(&bundle["tree"]);
            assert_eq!(bundle["sourcePlanes"].as_array().unwrap().len(), 2);
            assert_eq!(
                embedded["selection"]["alternatives"]
                    .as_array()
                    .unwrap()
                    .len(),
                1
            );
            assert_eq!(
                embedded["root"],
                embedded["selection"]["alternatives"][0]["root"]
            );
            assert_eq!(
                included_range_physical(&embedded["root"]),
                expected.as_bytes()
            );
            check_grammar_selection_mapped_tree(&bundle, &FeatureParseOptions::default()).unwrap();
        }
    }
}

#[test]
fn global_archives_execute_json_and_include_exclude_field_views_after_original_sources_are_dropped()
{
    let json = field_range_json();
    let mut controls = vec![(
        field_range_host(&json, &["Child"], 0, 1, "include", "any-grammar"),
        json,
        r#"{"k":{"x":23}}"#,
    )];
    for policy in ["any-grammar", "all-grammar"] {
        for (mode, selected) in [("include", "π"), ("exclude", "<>")] {
            let host = parse_grammar_links(&format!(
                "(grammar (start document))\n(rule document normal ({policy} (seq (literal %3C) (capture labeled body (literal %CF%80)) (literal %3E)) (grammar-field-candidate Child field-ranges (source-field-selector body {mode} 0 1))))\n",
            )).unwrap();
            controls.push((host, included_range_literal(selected), "<π>"));
        }
    }
    for (host, child, source) in controls {
        let mut sources = field_range_sources(host, child, "Child");
        let settings = field_range_settings();
        let space = create_global_concept_space(&sources, &settings).unwrap();
        let options = FeatureParseOptions {
            grammar_name: Some("Host".to_owned()),
            global_concept_space: Some(space.clone()),
            ..FeatureParseOptions::default()
        };
        let expected = {
            let resolve = |name: &str| {
                sources
                    .iter()
                    .find(|source| source.id == name)
                    .map(|source| source.grammar.clone())
            };
            compile_feature_grammar(&sources[0].grammar, Some(&resolve), options.clone())
                .unwrap()
                .parse_tree(source.as_bytes(), &options)
                .unwrap()
        };
        assert!(expected.ok, "{:?}", expected.rejection);
        let expected = included_range_bundle(&expected);
        let archive = global_concept_space_to_links(&space).unwrap();
        sources.clear();
        for stored in included_range_stored(&json!({"archive":archive})) {
            let parser = create_global_concept_space_parser(
                stored["archive"].as_str().unwrap(),
                "Host",
                &settings,
                FeatureParseOptions::default(),
            )
            .unwrap();
            let actual = parser
                .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
                .unwrap();
            assert!(actual.ok, "{:?}", actual.rejection);
            let actual = included_range_bundle(&actual);
            assert_eq!(actual, expected);
            assert_eq!(included_range_physical(&actual["tree"]), source.as_bytes());
            check_grammar_selection_mapped_tree(&actual, &FeatureParseOptions::default()).unwrap();
        }
    }
}

#[test]
fn coordinated_valid_child_and_range_forgeries_cannot_retarget_the_selected_json_field() {
    let source = r#"{"a":1,"b":2}"#;
    let json = field_range_json();
    let grammars = HashMap::from([("Json".to_owned(), json.clone())]);
    let original = included_range_parse(
        &field_range_host(&json, &["Json"], 0, 1, "include", "any-grammar"),
        &grammars,
        source,
    );
    assert!(original.ok, "{:?}", original.rejection);
    let original = included_range_bundle(&original);
    let original_path = field_range_embedding_path(&original["tree"], "").unwrap();
    let original_children = &original["tree"].pointer(&original_path).unwrap()["sourceChildren"];
    assert!(
        original_children[0].get("field").is_none(),
        "an absent native field stays absent, rather than becoming an explicit null"
    );
    assert_eq!(
        original_children[0]["children"][1]["children"][2]["field"], "value",
        "an actual captured field remains present"
    );
    assert!(original_children[0].get("trivia").is_none());
    assert!(original_children[0].get("attributes").is_none());
    for stored in included_range_stored(&original) {
        check_grammar_selection_mapped_tree(&stored, &FeatureParseOptions::default())
            .expect("the untouched public CST and full witness correspond before any mutation");
    }
    let donor = included_range_parse(
        &included_range_grammar(source, &[[11, 12]], &["Json"], "any-grammar", "", ""),
        &grammars,
        source,
    );
    assert!(donor.ok, "{:?}", donor.rejection);
    let donor = included_range_bundle(&donor);
    let donor = included_range_embedding(&donor["tree"]);
    assert_eq!(included_range_physical(&donor["root"]), b"2".as_slice());
    let mut forged = original.clone();
    let path = field_range_embedding_path(&forged["tree"], "").unwrap();
    let embedded = forged["tree"].pointer_mut(&path).unwrap();
    embedded["selection"]["sourceMapping"]["ranges"] =
        donor["selection"]["sourceMapping"]["ranges"].clone();
    embedded["selection"]["sourceMapping"]["intervals"] =
        donor["selection"]["sourceMapping"]["intervals"].clone();
    let occurrence = json!([{"path":["children",0,"children",3,"children",2],"start":11,"end":12}]);
    embedded["selection"]["sourceMapping"]["occurrences"] = occurrence.clone();
    embedded["root"] = donor["root"].clone();
    forged["sourcePlanes"][1]["derivation"]["ranges"] = json!([[11, 12]]);
    forged["sourcePlanes"][1]["derivation"]["occurrences"] = occurrence;
    for stored in included_range_stored(&forged) {
        assert!(
            check_grammar_selection_mapped_tree(&stored, &FeatureParseOptions::default()).is_err()
        );
    }
    for mutation in 0..4 {
        let mut forged = original.clone();
        let path = field_range_embedding_path(&forged["tree"], "").unwrap();
        match mutation {
            0 => {
                forged["sourcePlanes"][1]["derivation"]["witness"]["children"][0]["attributes"] =
                    json!({"note":"forged"});
            }
            1 => {
                forged["tree"].pointer_mut(&path).unwrap()["sourceChildren"][0]["attributes"] =
                    json!({"note":"forged"});
            }
            2 => {
                forged["sourcePlanes"][1]["derivation"]["witness"]["children"][0]["unrepresented"] =
                    json!(true);
            }
            _ => {
                // SyntaxTree.field is Option<String>; actual absence cannot
                // express null. A forged present null must still be compared.
                forged["tree"].pointer_mut(&path).unwrap()["sourceChildren"][0]["field"] =
                    Value::Null;
            }
        }
        for stored in included_range_stored(&forged) {
            assert!(
                check_grammar_selection_mapped_tree(&stored, &FeatureParseOptions::default())
                    .is_err(),
                "mutation {mutation}"
            );
        }
    }
    let mut reason_forgery = original.clone();
    let error = json!({"type":"error","start":1,"end":4,"text":"\"a\""});
    reason_forgery["sourcePlanes"][1]["derivation"]["witness"]["children"][0]["children"][1]["children"]
        [0] = error.clone();
    let path = field_range_embedding_path(&reason_forgery["tree"], "").unwrap();
    let key = &mut reason_forgery["tree"].pointer_mut(&path).unwrap()["sourceChildren"][0]["children"]
        [1]["children"][0];
    *key = error;
    key["reason"] = json!(1);
    // Both key replacements retain the original bytes and selected value.
    // This checks descriptor correspondence, not language-CST validity.
    assert_eq!(
        included_range_physical(&reason_forgery["tree"]),
        source.as_bytes()
    );
    for stored in included_range_stored(&reason_forgery) {
        assert!(
            check_grammar_selection_mapped_tree(&stored, &FeatureParseOptions::default()).is_err()
        );
    }
    let mut signed = original;
    let path = field_range_embedding_path(&signed["tree"], "").unwrap();
    signed["sourcePlanes"][1]["derivation"]["witness"]["children"][0]["attributes"] =
        json!({"marker":-0.0});
    signed["tree"].pointer_mut(&path).unwrap()["sourceChildren"][0]["attributes"] =
        json!({"marker":-0.0});
    for mut stored in included_range_stored(&signed) {
        check_grammar_selection_mapped_tree(&stored, &FeatureParseOptions::default()).unwrap();
        stored["tree"].pointer_mut(&path).unwrap()["sourceChildren"][0]["attributes"]["marker"] =
            json!(0.0);
        assert!(
            check_grammar_selection_mapped_tree(&stored, &FeatureParseOptions::default()).is_err()
        );
    }
}

#[test]
fn failed_field_attempts_retain_independently_replayable_witnesses_after_a_literal_fallback() {
    let source = r#"{"k":1}"#;
    let json = field_range_json();
    let host = field_range_host(&json, &["Refusing"], 0, 1, "include", "any-grammar");
    let expression = GrammarExpr::choice(
        true,
        vec![
            host.start_rule().unwrap().expr.clone(),
            GrammarExpr::Terminal(source.to_owned()),
        ],
    );
    let host = field_range_with_expression(&host, &expression);
    let outcome = included_range_parse(
        &host,
        &HashMap::from([("Refusing".to_owned(), included_range_literal("absent"))]),
        source,
    );
    assert!(outcome.ok, "{:?}", outcome.rejection);
    let bundle = included_range_bundle(&outcome);
    assert!(field_range_embedding_path(&bundle["tree"], "").is_none());
    assert_eq!(bundle["sourcePlanes"].as_array().unwrap().len(), 2);
    assert_eq!(
        bundle["sourcePlanes"][1]["derivation"]["witness"]["children"][0]["kind"],
        "object"
    );
    for mut stored in included_range_stored(&bundle) {
        check_grammar_selection_mapped_tree(&stored, &FeatureParseOptions::default()).unwrap();
        stored["sourcePlanes"][1]["derivation"]["witness"]["children"][0]["children"][1]["children"]
            [2]["field"] = json!("unselected");
        assert!(
            check_grammar_selection_mapped_tree(&stored, &FeatureParseOptions::default()).is_err()
        );
    }
}

#[test]
fn invalid_field_windows_and_full_views_refuse_before_child_execution_or_derived_plane_creation() {
    let json = field_range_json();
    let mut cases = [("{}", 0, 1), (r#"{"k":1}"#, 1, 1), (r#"{"k":1}"#, 0, 2)]
        .into_iter()
        .map(|(source, first, count)| {
            (
                source,
                field_range_host(&json, &["Json"], first, count, "include", "any-grammar"),
            )
        })
        .collect::<Vec<_>>();
    cases.push(("x", parse_grammar_links("(grammar (start document))\n(rule document normal (any-grammar (capture labeled value (literal x)) (grammar-field-candidate Json field-ranges (source-field-selector value include 0 1))))\n").unwrap()));
    for (source, host) in cases {
        let calls = Cell::new(0);
        let resolve = |_: &str| {
            calls.set(calls.get() + 1);
            Some(json.clone())
        };
        let parser =
            compile_feature_grammar(&host, Some(&resolve), FeatureParseOptions::default()).unwrap();
        // Native compilation prepares the selected grammar. The production
        // diagnostic and absence of derived planes prove parse-phase ordering.
        assert_eq!(calls.get(), 1);
        calls.set(0);
        let outcome = parser
            .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
            .unwrap();
        assert!(!outcome.ok);
        assert_eq!(calls.get(), 0);
        if let Some(planes) = &outcome.source_planes {
            assert!(planes.iter().all(|plane| plane["derivation"].is_null()));
        }
        let rejection = outcome.rejection.unwrap();
        assert_eq!(rejection.reason, "invalidEmbeddingMapping");
        let diagnostics = rejection.diagnostics.unwrap();
        assert_ne!(diagnostics, [] as [serde_json::Value; 0]);
        assert!(
            diagnostics
                .iter()
                .all(|diagnostic| diagnostic.get("language").is_none())
        );
    }
}

#[test]
fn nested_field_json_string_and_field_views_replay_four_source_planes_from_links_archives() {
    let json = field_range_json();
    let outer = field_range_host(&json, &["Quoted"], 0, 1, "include", "any-grammar");
    let inner = field_range_host(&json, &["Leaf"], 0, 1, "include", "any-grammar");
    let quoted = field_range_with_expression(
        &json,
        &GrammarExpr::feature(FeatureExpr::Form(FeatureForm::new(
            "any-grammar",
            vec![
                FieldValue::Expression(json.start_rule().unwrap().expr.clone()),
                FieldValue::Expressions(vec![
                    parse_links_expression("(grammar-candidate Inner json-string)").unwrap(),
                ]),
            ],
        ))),
    );
    let grammars = HashMap::from([
        ("Outer".to_owned(), outer.clone()),
        ("Quoted".to_owned(), quoted),
        ("Inner".to_owned(), inner),
        ("Leaf".to_owned(), json.clone()),
    ]);
    let source = r#"{"k":"{\"k\":2}"}"#;
    let outcome = included_range_parse(&outer, &grammars, source);
    assert!(outcome.ok, "{:?}", outcome.rejection);
    let bundle = included_range_bundle(&outcome);
    let planes = bundle["sourcePlanes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|plane| {
            json!([
                plane["identity"],
                plane["byteLength"],
                plane["derivation"]["operation"],
                plane["derivation"]["input"]
            ])
        })
        .collect::<Vec<_>>();
    assert_eq!(
        planes,
        vec![
            json!([0, 17, null, null]),
            json!([1, 11, "field-ranges", 0]),
            json!([2, 7, "json-string", 1]),
            json!([3, 1, "field-ranges", 2])
        ]
    );
    let first = field_range_embedding(&bundle["tree"]);
    let last = field_range_embedding(&first["root"]);
    assert_eq!(first["selection"]["sourceMapping"]["output"], 1);
    assert_eq!(last["selection"]["sourceMapping"]["input"], 2);
    assert_eq!(last["selection"]["sourceMapping"]["output"], 3);
    assert_eq!(included_range_physical(&bundle["tree"]), source.as_bytes());
    assert_eq!(included_range_physical(&first["root"]), br#""{\"k\":2}""#);
    let direct = included_range_parse(&json, &HashMap::new(), "2");
    assert!(direct.ok);
    assert_eq!(
        last["root"],
        included_range_tree_data(direct.tree.as_ref().unwrap())
    );
    let mut documents = grammars.iter().map(|(name, grammar)| {
        let syntax = parse_lino_to_links_with_config(&render_grammar_links(grammar), &ParserConfig::without_comments()).unwrap();
        json!({"name":name,"syntax":syntax.iter().map(included_range_link_data).collect::<Vec<_>>()})
    }).collect::<Vec<_>>();
    documents.sort_by(|left, right| left["name"].as_str().cmp(&right["name"].as_str()));
    for decoded in included_range_stored(&json!({"grammars":documents,"bundle":bundle})) {
        check_grammar_selection_mapped_tree(&decoded["bundle"], &FeatureParseOptions::default())
            .unwrap();
        let restored = decoded["grammars"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| {
                let listing = row["syntax"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(included_range_link_text)
                    .collect::<Vec<_>>()
                    .join("\n");
                (
                    row["name"].as_str().unwrap().to_owned(),
                    parse_grammar_links(&listing).unwrap(),
                )
            })
            .collect::<HashMap<_, _>>();
        let replay = included_range_parse(&restored["Outer"], &restored, source);
        assert!(replay.ok, "{:?}", replay.rejection);
        assert_eq!(included_range_bundle(&replay), decoded["bundle"]);
        let mut forged = decoded["bundle"].clone();
        forged["sourcePlanes"][3]["derivation"]["input"] = json!(0);
        assert!(
            check_grammar_selection_mapped_tree(&forged, &FeatureParseOptions::default()).is_err()
        );
    }
}

#[test]
fn raw_field_forms_refuse_wrong_types_unknown_fields_and_invalid_windows_before_projection() {
    let host = parse_grammar_links("(grammar (start document))\n(rule document normal (any-grammar (seq (literal %3C) (capture labeled body (literal %CF%80)) (literal %3E)) (grammar-field-candidate Child field-ranges (source-field-selector body include 0 1))))\n").unwrap();
    let child = included_range_literal("π");
    let settings = field_range_settings();
    let sources = field_range_sources(host.clone(), child, "Child");
    let space = create_global_concept_space(&sources, &settings).unwrap();
    let document: Value = serde_json::from_str(&serialize_grammar(&host)).unwrap();
    let candidate_path = "/rules/0/expression/items/0";
    assert_eq!(
        document.pointer(candidate_path).unwrap()["selector"]["count"],
        1
    );
    for (suffix, value) in [
        ("/extra", Some(json!(true))),
        ("/selector/extra", Some(json!(true))),
        ("/selector", None),
        ("/selector/label", None),
        ("/selector/first", None),
        ("/selector/count", None),
        ("/kind", Some(json!("grammar-range-candidate"))),
        ("/mapping", Some(json!("included-ranges"))),
        ("/name", Some(json!(7))),
        ("/name", Some(json!(""))),
        ("/selector/kind", Some(json!("source-range"))),
        ("/selector/label", Some(json!(""))),
        ("/selector/label", Some(json!(7))),
        ("/selector/mode", Some(json!("all"))),
        ("/selector/first", Some(json!("0"))),
        ("/selector/count", Some(json!("1"))),
        ("/selector/first", Some(json!(-1))),
        ("/selector/first", Some(json!(0.5))),
        ("/selector/first", Some(json!(4_294_967_296_u64))),
        ("/selector/first", Some(Value::Null)),
        ("/selector/count", Some(json!(0))),
        ("/selector/count", Some(json!(-1))),
        ("/selector/count", Some(json!(4_294_967_296_u64))),
        ("/selector/count", Some(Value::Null)),
    ] {
        let mut changed = document.clone();
        let mut keys = suffix.trim_start_matches('/').rsplitn(2, '/');
        let key = keys.next().unwrap();
        let parent = keys.next().map_or_else(
            || candidate_path.to_owned(),
            |path| format!("{candidate_path}/{path}"),
        );
        let owner = changed
            .pointer_mut(&parent)
            .unwrap()
            .as_object_mut()
            .unwrap();
        if let Some(value) = value {
            owner.insert(key.to_owned(), value);
        } else {
            owner.remove(key);
        }
        assert!(
            deserialize_grammar(&changed.to_string()).is_err(),
            "{suffix}"
        );
        assert!(
            GlobalConceptSource::from_json(
                &json!({"id":"Host","language":"Host","grammar":changed})
            )
            .is_err(),
            "{suffix}"
        );
        let mut forged = space.clone();
        let definition = forged["concepts"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .flat_map(|concept| concept["definitions"].as_array_mut().unwrap())
            .find(|definition| definition["origin"] == "Host:document")
            .unwrap();
        definition["definition"]["expression"]["items"][0] =
            changed.pointer(candidate_path).unwrap().clone();
        assert!(global_concept_space_to_links(&forged).is_err(), "{suffix}");
    }
    let grammars = HashMap::from([("Child".to_owned(), included_range_literal("π"))]);
    let expected = included_range_parse(&host, &grammars, "<π>");
    assert!(expected.ok, "{:?}", expected.rejection);
    let mut floating = document;
    let selector = floating
        .pointer_mut("/rules/0/expression/items/0/selector")
        .unwrap();
    selector["first"] = json!(-0.0);
    selector["count"] = json!(1.0);
    let restored = deserialize_grammar(&floating.to_string()).unwrap();
    let links = render_grammar_links(&restored);
    assert_eq!(links, render_grammar_links(&host));
    let linked = parse_grammar_links(&links).unwrap();
    for grammar in [&restored, &linked] {
        let actual = included_range_parse(grammar, &grammars, "<π>");
        assert!(actual.ok, "{:?}", actual.rejection);
        let bundle = included_range_bundle(&actual);
        assert_eq!(bundle, included_range_bundle(&expected));
        check_grammar_selection_mapped_tree(&bundle, &FeatureParseOptions::default()).unwrap();
    }
    let archive = global_concept_space_to_links(&space).unwrap();
    let selector = "(source-field-selector body include 0 1)";
    assert!(archive.contains(selector));
    for replacement in [
        "(source-field-selector body include -1 1)",
        "(source-field-selector body include 0 0)",
        "(source-field-selector body include 0.5 1)",
        "(source-field-selector body include 0 4294967296)",
        "(source-field-selector body other 0 1)",
        "(source-field-selector % include 0 1)",
        "(source-field-selector body include 0 1 extra)",
    ] {
        let malformed = archive.replace(selector, replacement);
        assert!(
            read_global_concept_space_links(&malformed, &settings).is_err(),
            "{replacement}"
        );
        assert!(
            create_global_concept_space_parser(
                &malformed,
                "Host",
                &settings,
                FeatureParseOptions::default()
            )
            .is_err(),
            "{replacement}"
        );
    }
}
