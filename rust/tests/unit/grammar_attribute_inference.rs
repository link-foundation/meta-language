use super::issue_195_observations::{Observation, record};
use super::issue_195_translation_pairs::{TemporaryDirectory, execute};
use meta_language::translation::Language;
use meta_language::{
    DynamicAttributeSelection, DynamicMetaLanguage, DynamicMetaLanguageOptions,
    DynamicParseOptions, FeatureParseOptions, FormalConceptDefinitionInput,
    FormalConceptDefinitionOptions, GlobalConceptSource, GrammarAttributeInferenceRequest,
    GrammarAttributeObservation, MetaLanguageArchiveLimits, OperationValue, SyntaxTree,
    compile_feature_grammar, create_dynamic_meta_language, encode_formal_concept_definition,
    infer_grammar_attribute_action, meta_language_archive_to_binary, parse_grammar_links,
    prove_formal_concept_equivalence, read_meta_language_archive_binary, read_structured_data_lino,
    render_grammar_links, run_dynamic_language_command, structured_data_to_lino,
    verify_formal_concept_equivalence,
};
use serde_json::{Value, json};

const FIXTURE: &str = "parity/fixtures/grammar-attribute-inference.lino";

fn text(value: &Value) -> &str {
    value.as_str().unwrap()
}

fn observed_value(value: &Value) -> OperationValue {
    match value {
        Value::String(text) => OperationValue::Text(text.clone()),
        value => OperationValue::Integer(value.as_i64().unwrap()),
    }
}

fn observations(case: &Value) -> Vec<GrammarAttributeObservation> {
    case["observations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| GrammarAttributeObservation {
            source: text(&row["source"]).into(),
            path: serde_json::from_value(row["path"].clone()).unwrap(),
            start: serde_json::from_value(row["start"].clone()).unwrap(),
            end: serde_json::from_value(row["end"].clone()).unwrap(),
            value: observed_value(&row["value"]),
        })
        .collect()
}

fn assert_command_results(fixture: &Value, options: &FeatureParseOptions) {
    let arguments = [
        "infer-attribute",
        "observations.lino",
        "--grammar",
        "grammar.lino",
    ]
    .map(str::to_owned);
    for name in ["unique", "ambiguous", "forged", "unsupported", "numeric"] {
        let current = &fixture[name];
        let observations = structured_data_to_lino(&json!({
            "request": fixture["request"],
            "observations": current["observations"],
        }))
        .unwrap();
        let outcome = run_dynamic_language_command(
            &arguments,
            |path| match path {
                "grammar.lino" => {
                    Ok(text(current.get("grammar").unwrap_or(&fixture["grammar"])).to_owned())
                }
                "observations.lino" => Ok(observations.clone()),
                path => panic!("unexpected command input {path}"),
            },
            |_| panic!("attribute inference reads only native text"),
        );
        if name == "forged" {
            assert_eq!(outcome.exit_code, 2);
            assert!(outcome.stderr.contains(text(&current["diagnostic"])));
            continue;
        }
        assert_eq!(outcome.exit_code, 0, "{}", outcome.stderr);
        let report =
            read_structured_data_lino(&String::from_utf8(outcome.stdout).unwrap()).unwrap();
        if current.get("candidate").is_some() {
            assert_eq!(report["candidates"], json!([current["candidate"]]));
            assert!(report["diagnostic"].is_null());
            let grammar = parse_grammar_links(text(&report["grammar"])).unwrap();
            let parser = compile_feature_grammar(&grammar, None, options.clone()).unwrap();
            let holdout = parser
                .parse_tree(text(&current["holdout"]).as_bytes(), options)
                .unwrap();
            assert!(holdout.ok);
            let Some(SyntaxTree::Node {
                attributes: Some(attributes),
                ..
            }) = holdout.tree
            else {
                panic!("expected attributes on the holdout node");
            };
            assert_eq!(
                attributes.get(text(&fixture["request"]["attribute"])),
                Some(&observed_value(&current["value"]))
            );
        } else {
            let expected = current
                .get("candidates")
                .cloned()
                .unwrap_or_else(|| json!([]));
            assert_eq!(report["candidates"], expected);
            assert!(report["grammar"].is_null());
            assert!(text(&report["diagnostic"]).contains(text(&current["diagnostic"])));
        }
    }
}

#[test]
fn installed_inferred_actions_translate_after_reload_and_invalidate_old_semantic_certificates() {
    super::regular_grammar_inference::versions();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/grammar-attribute-inference.lino"
    ))
    .unwrap();
    let request = GrammarAttributeInferenceRequest {
        rule: text(&fixture["request"]["rule"]),
        attribute: text(&fixture["request"]["attribute"]),
        field: None,
        child_attribute: None,
    };
    let selection = DynamicAttributeSelection {
        path: Vec::new(),
        kind: request.rule.to_owned(),
        attribute: request.attribute.to_owned(),
    };
    let selections = [selection.clone(), selection];
    let settings = DynamicMetaLanguageOptions::default();
    let session = create_dynamic_meta_language(&[], &settings).unwrap();
    let source_id = "observed words";
    let source = |grammar| GlobalConceptSource {
        id: source_id.to_owned(),
        language: "Observed words".to_owned(),
        edition: "1".to_owned(),
        grammar,
    };
    let restore = |language: &DynamicMetaLanguage| {
        let limits = MetaLanguageArchiveLimits::default();
        let bytes = meta_language_archive_to_binary(language.archive(), limits).unwrap();
        let loaded = read_meta_language_archive_binary(&bytes, limits).unwrap();
        DynamicMetaLanguage::new(&loaded.archive, &settings).unwrap()
    };
    let inferred = |grammar: &Value, case: &Value| {
        infer_grammar_attribute_action(
            &parse_grammar_links(text(grammar)).unwrap(),
            request,
            &observations(case),
        )
        .unwrap()
        .grammar
        .unwrap()
    };
    let directory = TemporaryDirectory::new("installed-attribute-translation");
    let translated = |language: &DynamicMetaLanguage, input: &str, expected: &Value, name: &str| {
        let parsed = language
            .parse(source_id, input.as_bytes(), &DynamicParseOptions::default())
            .unwrap();
        assert!(parsed.outcome.ok);
        let Some(SyntaxTree::Node {
            attributes: Some(attributes),
            ..
        }) = parsed.outcome.tree
        else {
            panic!("expected attributes on the installed language's concrete tree");
        };
        assert_eq!(
            attributes.get(request.attribute),
            Some(&observed_value(expected))
        );
        let rendered = expected
            .as_str()
            .map_or_else(|| expected.to_string(), String::from);
        let javascript = language
            .translate_attributes(
                source_id,
                input.as_bytes(),
                Language::JavaScript,
                &selections,
                FeatureParseOptions::default(),
            )
            .unwrap();
        let rust = language
            .translate_attributes(
                source_id,
                input.as_bytes(),
                Language::Rust,
                &selections,
                FeatureParseOptions::default(),
            )
            .unwrap();
        assert_eq!(javascript.output.language, Language::JavaScript);
        assert_eq!(rust.output.language, Language::Rust);
        let archive = meta_language_archive_to_binary(language.archive(), limits).unwrap();
        let selection_file = structured_data_to_lino(&json!([
            {"path": [], "kind": request.rule, "attribute": request.attribute},
            {"path": [], "kind": request.rule, "attribute": request.attribute}
        ]))
        .unwrap();
        for output in [javascript.output, rust.output] {
            let arguments = [
                "translate-attributes",
                "installed.lino.bin",
                "--input-binary",
                "--source",
                source_id,
                "--input",
                "input",
                "--target",
                output.language.as_str(),
                "--selections",
                "selections.lino",
            ]
            .map(str::to_owned);
            let command = run_dynamic_language_command(
                &arguments,
                |name| match name {
                    "selections.lino" => Ok(selection_file.clone()),
                    _ => Err(format!("unexpected text file {name}")),
                },
                |name| match name {
                    "installed.lino.bin" => Ok(archive.clone()),
                    "input" => Ok(input.as_bytes().to_vec()),
                    _ => Err(format!("unexpected byte file {name}")),
                },
            );
            assert_eq!(command.exit_code, 0, "{}", command.stderr);
            assert_eq!(command.stdout, output.code.as_bytes());
            let result = execute(output.language.as_str(), &output.code, &directory.0, name);
            assert!(result.accepted && result.ran, "{}", result.diagnostics);
            assert_eq!(result.lines, [rendered.clone(), rendered.clone()]);
        }
        encode_formal_concept_definition(
            FormalConceptDefinitionInput::LinkedProgram {
                network: &javascript.links.network,
                root: javascript.links.root,
            },
            FormalConceptDefinitionOptions::default(),
        )
        .unwrap()
    };
    let unique = &fixture["unique"];
    let holdout = text(&unique["holdout"]);
    let installed = restore(
        &session
            .extend(&[source(inferred(&fixture["grammar"], unique))], &settings)
            .unwrap(),
    );
    let first = translated(&installed, holdout, &unique["value"], "original");
    let certificate = prove_formal_concept_equivalence(&first, &first).unwrap();
    assert!(verify_formal_concept_equivalence(
        &first,
        &first,
        &certificate
    ));

    let listing = render_grammar_links(&installed.sources()[0].grammar);
    let changed_listing = listing.replace(text(&unique["candidate"]), text(&unique["mutation"]));
    assert_ne!(changed_listing, listing);
    let replace = DynamicMetaLanguageOptions {
        replace: true,
        ..DynamicMetaLanguageOptions::default()
    };
    let changed = restore(
        &installed
            .extend(
                &[source(parse_grammar_links(&changed_listing).unwrap())],
                &replace,
            )
            .unwrap(),
    );
    let second = translated(&changed, holdout, &unique["mutatedValue"], "changed");
    assert!(!verify_formal_concept_equivalence(
        &first,
        &second,
        &certificate
    ));
    assert!(prove_formal_concept_equivalence(&first, &second).is_none());
    translated(&installed, holdout, &unique["value"], "original_again");

    let numeric = &fixture["numeric"];
    let numbers = restore(
        &changed
            .extend(&[source(inferred(&numeric["grammar"], numeric))], &replace)
            .unwrap(),
    );
    translated(
        &numbers,
        text(&numeric["holdout"]),
        &numeric["value"],
        "numeric",
    );
    let wrong_path = DynamicAttributeSelection {
        path: vec![99],
        ..selections[0].clone()
    };
    for (input, selection, reason) in [
        (
            text(&unique["negative"]),
            &selections[0],
            "accepted unambiguous",
        ),
        (holdout, &wrong_path, "concrete attributed"),
    ] {
        let error = installed
            .translate_attributes(
                source_id,
                input.as_bytes(),
                Language::JavaScript,
                std::slice::from_ref(selection),
                FeatureParseOptions::default(),
            )
            .unwrap_err();
        assert!(error.contains(reason), "{error}");
    }
}

#[test]
fn concrete_attribute_observations_constrain_executable_native_actions() {
    super::regular_grammar_inference::versions();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/grammar-attribute-inference.lino"
    ))
    .unwrap();
    let original = parse_grammar_links(text(&fixture["grammar"])).unwrap();
    let request = GrammarAttributeInferenceRequest {
        rule: text(&fixture["request"]["rule"]),
        attribute: text(&fixture["request"]["attribute"]),
        field: None,
        child_attribute: None,
    };
    let unique = &fixture["unique"];
    let candidate = text(&unique["candidate"]);
    let result = infer_grammar_attribute_action(&original, request, &observations(unique)).unwrap();
    assert_eq!(result.candidates, [candidate]);
    assert!(result.diagnostic.is_none());
    let listing = render_grammar_links(&result.grammar.unwrap());
    assert!(listing.contains(text(&unique["doc"])));
    assert!(
        original
            .rule(request.rule)
            .unwrap()
            .attributes
            .action
            .is_none()
    );
    let options = FeatureParseOptions::default();
    let mutated = listing.replace(candidate, text(&unique["mutation"]));
    for (listing, expected) in [
        (&listing, &unique["value"]),
        (&mutated, &unique["mutatedValue"]),
    ] {
        let grammar = parse_grammar_links(listing).unwrap();
        let parser = compile_feature_grammar(&grammar, None, options.clone()).unwrap();
        let outcome = parser
            .parse_tree(text(&unique["holdout"]).as_bytes(), &options)
            .unwrap();
        assert!(outcome.ok);
        let attributes = match outcome.tree.unwrap() {
            SyntaxTree::Node { attributes, .. } | SyntaxTree::Token { attributes, .. } => {
                attributes.unwrap()
            }
            tree => panic!("expected holdout rule, received {tree:?}"),
        };
        assert_eq!(
            attributes.get(request.attribute),
            Some(&observed_value(expected))
        );
        assert!(
            !parser
                .parse_tree(text(&unique["negative"]).as_bytes(), &options)
                .unwrap()
                .ok
        );
    }

    let ambiguous = &fixture["ambiguous"];
    let result =
        infer_grammar_attribute_action(&original, request, &observations(ambiguous)).unwrap();
    let candidates: Vec<String> = serde_json::from_value(ambiguous["candidates"].clone()).unwrap();
    assert_eq!(result.candidates, candidates);
    assert!(result.grammar.is_none());
    assert!(
        result
            .diagnostic
            .unwrap()
            .contains(text(&ambiguous["diagnostic"]))
    );
    let forged = &fixture["forged"];
    let error =
        infer_grammar_attribute_action(&original, request, &observations(forged)).unwrap_err();
    assert!(error.contains(text(&forged["diagnostic"])));
    let unsupported = &fixture["unsupported"];
    let result =
        infer_grammar_attribute_action(&original, request, &observations(unsupported)).unwrap();
    assert!(result.candidates.is_empty());
    assert!(result.grammar.is_none());
    assert!(
        result
            .diagnostic
            .unwrap()
            .contains(text(&unsupported["diagnostic"]))
    );
    let numeric = &fixture["numeric"];
    let grammar = parse_grammar_links(text(&numeric["grammar"])).unwrap();
    let result = infer_grammar_attribute_action(&grammar, request, &observations(numeric)).unwrap();
    assert_eq!(result.candidates, [text(&numeric["candidate"])]);
    let parser = compile_feature_grammar(&result.grammar.unwrap(), None, options.clone()).unwrap();
    for (source, expected) in [
        ("-0", OperationValue::Integer(0)),
        (text(&numeric["holdout"]), observed_value(&numeric["value"])),
    ] {
        let outcome = parser.parse_tree(source.as_bytes(), &options).unwrap();
        assert!(outcome.ok);
        let Some(SyntaxTree::Node {
            attributes: Some(attributes),
            ..
        }) = outcome.tree
        else {
            panic!("expected attributes on the numeric node");
        };
        assert_eq!(attributes.get(request.attribute), Some(&expected));
    }
    assert_command_results(&fixture, &options);
    record(&Observation {
        requirement_id: "I195-GRAMMAR-INFERENCE-CONCRETE-ATTRIBUTES",
        suffix: "behavior",
        fixture_id: "grammar-attribute-inference",
        fixture_file: FIXTURE,
        assertions: &[
            "concreteAttributeObservationsInferNativeAction",
            "reloadedActionAndLinkedEditExecuteOnHoldout",
            "ambiguousForgedAndUnsupportedTracesRemainExplicit",
            "attributeInferenceCommandReturnsExecutableActionsAndAlternatives",
        ],
        test_name: "concrete_attribute_observations_constrain_executable_native_actions",
    });
}
