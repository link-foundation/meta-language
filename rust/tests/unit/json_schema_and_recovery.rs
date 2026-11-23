use meta_language::grammar::json_document::{
    JsonRecoveryAction, emit_json_schema, import_json_schema, recover_json, validate_json,
};
use meta_language::{
    FeatureParseOptions, Grammar, compile_feature_grammar, emit_gbnf, emit_lark, import_gbnf,
    import_lark,
};
use serde_json::{Value, json};

use super::issue_195_observations::{Observation, record};
use super::project_data::read_project_data;

const FIXTURE: &str = "parity/fixtures/json-schema-and-recovery.lino";

fn fixture() -> Value {
    read_project_data(include_str!(
        "../../../parity/fixtures/json-schema-and-recovery.lino"
    ))
    .expect("shared JSON Schema and recovery fixtures")
}

#[test]
fn shared_json_schema_fixtures_validate_decoders_and_locate_errors() {
    let fixture = fixture();
    for row in fixture["grammars"].as_array().unwrap() {
        let imported = import_json_schema(&row[0]).unwrap();
        if let Some(expected) = row.get(2).and_then(Value::as_array) {
            let actual: Vec<_> = imported
                .diagnostics
                .iter()
                .map(|issue| issue.pointer.as_str())
                .collect();
            assert_eq!(
                actual,
                expected
                    .iter()
                    .map(|pointer| pointer.as_str().expect("diagnostic pointer"))
                    .collect::<Vec<_>>()
            );
            if expected.is_empty() {
                for case in row[1].as_array().unwrap() {
                    let source = case[0].as_str().unwrap();
                    assert_eq!(
                        Some(validate_json(source, &row[0]).valid),
                        case[1].as_bool(),
                        "{source}"
                    );
                }
            }
        }
        let (gbnf, _) = emit_gbnf(&imported.grammar).unwrap();
        let (lark, _) = emit_lark(&imported.grammar).unwrap();
        for grammar in [
            &imported.grammar,
            &import_gbnf(&gbnf).unwrap(),
            &import_lark(&lark).unwrap(),
        ] {
            let parser =
                compile_feature_grammar(grammar, None, FeatureParseOptions::default()).unwrap();
            for case in row[1].as_array().unwrap() {
                let source = case[0].as_str().unwrap();
                let outcome = parser
                    .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
                    .unwrap();
                assert_eq!(Some(outcome.ok), case[1].as_bool(), "{source}");
            }
        }
        if let Some(cases) = row.get(3).and_then(Value::as_array) {
            for case in cases {
                let source = case[0].as_str().unwrap();
                assert_eq!(
                    Some(validate_json(source, &row[0]).valid),
                    case[1].as_bool(),
                    "{source}"
                );
            }
        }
    }
    let schema = &fixture["schema"];
    let imported = import_json_schema(schema).unwrap();
    let (lark, _) = emit_lark(&imported.grammar).unwrap();
    assert!(lark.lines().any(|line| line.starts_with("start: ")));
    assert!(
        !lark
            .lines()
            .any(|line| line.starts_with("json_format_email: "))
    );
    assert_eq!(imported.emit_json_schema().schema.as_ref(), Some(schema));
    let expression = Grammar::expr();
    let grammar = Grammar::builder()
        .start("document")
        .rule(
            "document",
            expression.choice_unordered([
                expression.term(r#"{"status":"met"}"#),
                expression.term(r#"{"status":"unknown"}"#),
            ]),
        )
        .build();
    assert_eq!(
        emit_json_schema(&grammar).schema,
        Some(json!({"enum":[{"status":"met"},{"status":"unknown"}]}))
    );
    let truncated = fixture["truncated"].as_str().unwrap();
    for (source, expected) in [
        (
            fixture["wrong"].as_str().unwrap(),
            vec![
                ("enum", "/status", 10, 21),
                ("additionalProperties", "/extra", 41, 48),
            ],
        ),
        (
            truncated,
            vec![
                ("syntax", "/evidence", truncated.len(), truncated.len()),
                ("enum", "/status", 10, 21),
            ],
        ),
    ] {
        let report = validate_json(source, schema);
        assert!(!report.valid);
        assert_eq!(report.network.reconstruct_text(), source);
        let actual: Vec<_> = report
            .errors
            .iter()
            .map(|issue| {
                let span = issue.span.byte_range();
                assert!(report.network.link(issue.link_id).is_some());
                assert!(!issue.expected.is_empty());
                (
                    issue.keyword.as_str(),
                    issue.pointer.as_str(),
                    span.start(),
                    span.end(),
                )
            })
            .collect();
        assert_eq!(actual, expected);
    }
    let unsupported = import_json_schema(&json!({"type":"string","unregistered":true})).unwrap();
    assert_eq!(unsupported.diagnostics[0].pointer, "/unregistered");
    for schema in fixture["ungeneratableSchemas"].as_array().unwrap() {
        let Err(error) = import_json_schema(schema) else {
            panic!("schema unexpectedly generated a grammar: {schema}");
        };
        assert!(error.to_string().contains("no generatable document"));
    }
    for row in fixture["numericValidation"].as_array().unwrap() {
        let source = row[0].as_str().unwrap();
        let expected = row[2].as_array().unwrap();
        let report = validate_json(source, &row[1]);
        assert!(report.schema_issues.is_empty());
        assert_eq!(report.valid, expected.is_empty(), "{source}");
        let actual: Vec<_> = report
            .errors
            .iter()
            .map(|issue| {
                assert!(report.network.link(issue.link_id).is_some());
                let bytes = issue.span.byte_range();
                json!([issue.keyword, issue.pointer, bytes.start(), bytes.end()])
            })
            .collect();
        assert_eq!(&actual, expected, "{source}");
    }
    record(&Observation {
        requirement_id: "I195-I240-JSON-SCHEMA",
        suffix: "behavior",
        fixture_id: "i195-i240-json-schema",
        fixture_file: FIXTURE,
        assertions: &[
            "decoderGrammarsConstrainSchema",
            "schemaDiagnosticsRetainSyntaxLinks",
            "grammarAndSchemaRoundTrip",
            "decoderDisjointExclusiveBranchesHaveNoResidual",
            "decoderPatternsRespectDecodedStrings",
            "decoderFormatsRespectDecodedStrings",
            "decoderPatternsCombineEscapedSurrogatePairs",
            "decoderObjectConstantsAcceptMemberPermutations",
            "decoderExclusiveIntegerAlternativesRejectOverlap",
            "decoderRequiredStringTagsDistinguishAlternatives",
            "decoderFormatsCombineWithRegularPatterns",
            "decoderChecksFiniteReferencedAlternatives",
            "validationUsesExactNumericSpellings",
            "decoderAppliesSiblingTypeAssertions",
            "decoderSelectsExclusiveRegularStrings",
            "decoderIntersectsSiblingRegularStrings",
            "decoderResolvesLinkedAlternatives",
            "decoderCombinesLinkedNumericBounds",
            "decoderCombinesLinkedCollectionAssertions",
            "decoderAppliesSiblingCollectionAssertionsToAlternatives",
            "decoderBoundsClosedArrayTailsByTheirOwnPrefix",
            "decoderAppliesLinkedAssertionsWithinInstanceTypes",
            "decoderCombinesCompatibleObjectPropertyLayouts",
            "decoderCombinesCompatibleArrayItemLayouts",
            "decoderIntersectsScalarPropertyAndItemAssertions",
            "decoderFiltersFinitePropertyAndItemCandidates",
            "decoderIntersectsNestedCollectionLayouts",
            "decoderDistinguishesExclusiveArrayLengthIntervals",
            "decoderDistinguishesRequiredArrayPrefixValues",
            "decoderExclusiveDecimalAlternativesRespectEndpoints",
            "decoderDistinguishesIntegerValuesWithinNumberAlternatives",
            "decoderChecksExclusiveAlternativesWithinInstanceTypes",
            "decoderCountsFiniteNumericBranchesInExclusiveAlternatives",
        ],
        test_name: "shared_json_schema_fixtures_validate_decoders_and_locate_errors",
    });
}

fn action(name: &Value) -> JsonRecoveryAction {
    match name.as_str().unwrap() {
        "closeOpenContainers" => JsonRecoveryAction::CloseOpenContainers,
        "dropTrailingCommas" => JsonRecoveryAction::DropTrailingCommas,
        "stripCodeFence" => JsonRecoveryAction::StripCodeFence,
        "extractFirstValue" => JsonRecoveryAction::ExtractFirstValue,
        "stripTrailingGarbage" => JsonRecoveryAction::StripTrailingGarbage,
        "repairEscapes" => JsonRecoveryAction::RepairEscapes,
        _ => unreachable!("registered recovery action"),
    }
}

#[test]
fn shared_json_recovery_fixtures_preserve_opt_in_edits_and_provenance() {
    let fixture = fixture();
    for row in fixture["recovery"].as_array().unwrap() {
        let source = row[0].as_str().unwrap();
        let actions: Vec<_> = row[1].as_array().unwrap().iter().map(action).collect();
        let untouched = recover_json(source, None, &[]);
        assert_eq!(untouched.text, source);
        assert!(untouched.edits.is_empty());
        let report = recover_json(source, None, &actions);
        assert_eq!(report.value.as_ref(), Some(&row[2]), "{source}");
        assert_eq!(Some(report.complete), row[3].as_bool(), "{source}");
        assert_eq!(
            report
                .edits
                .iter()
                .map(|edit| edit.inserted.as_str())
                .collect::<Vec<_>>(),
            row[4]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap())
                .collect::<Vec<_>>(),
            "{source}"
        );
        assert_eq!(report.source_network.reconstruct_text(), source);
        assert_eq!(report.network.reconstruct_text(), report.text);
        for edit in report.edits {
            let flags = report
                .source_network
                .link(edit.link_id)
                .unwrap()
                .metadata()
                .flags();
            assert!(flags.is_error() || flags.is_missing());
            assert!(actions.contains(&edit.kind));
            assert_eq!(edit.removed, source[edit.span.start()..edit.span.end()]);
        }
    }
    let schema = &fixture["schema"];
    let missing = recover_json(
        fixture["missing"].as_str().unwrap(),
        Some(schema),
        &[JsonRecoveryAction::CloseOpenContainers],
    );
    assert_eq!(missing.value, Some(json!({"status":"met"})));
    assert!(!missing.complete);
    assert_eq!(missing.issues.len(), 1);
    assert_eq!(missing.issues[0].keyword, "required");
    assert_eq!(missing.issues[0].pointer, "/evidence");
    let source = fixture["repeated"].as_str().unwrap();
    let repeated = recover_json(source, None, &[]);
    assert_eq!(repeated.text, source);
    assert!(repeated.complete);
    assert!(repeated.edits.is_empty());
    let [issue] = repeated.issues.as_slice() else {
        panic!("one repetition")
    };
    let span = issue.span.byte_range();
    assert_eq!(issue.keyword, "repetition");
    assert_eq!(issue.pointer, "/content");
    assert_eq!(issue.pattern.as_deref(), Some("abcde"));
    assert_eq!(&source[span.start()..span.end()], "abcde".repeat(20));
    assert!(repeated.source_network.link(issue.link_id).is_some());
    record(&Observation {
        requirement_id: "I195-I241-JSON-RECOVERY",
        suffix: "behavior",
        fixture_id: "i195-i241-json-recovery",
        fixture_file: FIXTURE,
        assertions: &[
            "permittedRepairsRetainEditAnchors",
            "closingRetainsTruncatedStatusAndMissingFields",
            "repetitionsRetainSourceSpans",
        ],
        test_name: "shared_json_recovery_fixtures_preserve_opt_in_edits_and_provenance",
    });
}
