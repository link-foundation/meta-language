use super::issue_195_observations::{Observation, record};
use meta_language::{
    FeatureParseOptions, GrammarAttributeInferenceRequest, GrammarAttributeObservation,
    OperationValue, SyntaxTree, compile_feature_grammar, infer_grammar_attribute_action,
    parse_grammar_links, read_structured_data_lino, render_grammar_links,
    run_dynamic_language_command, structured_data_to_lino,
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
