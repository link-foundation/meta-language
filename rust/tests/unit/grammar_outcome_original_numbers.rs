//! Original native Number observations execute only in CI.
#[path = "native_json_number_observation_model.rs"]
mod original;
#[path = "native_json_number_publication.rs"]
mod publication;

use meta_language::{
    Ambiguity, GrammarSelection, GrammarSelectionAlternative, LeafText, ParseOutcome,
    ParseRejection, StructuredDataLinksLimits, SyntaxTree,
};
use serde_json::{Number, Value};

fn original_number_limits() -> StructuredDataLinksLimits {
    StructuredDataLinksLimits {
        node_limit: 100_000,
        byte_limit: 16 * 1024 * 1024,
        depth_limit: 256,
    }
}

fn original_number_token(lexical: Value) -> SyntaxTree {
    SyntaxTree::Token {
        kind: Some("word".to_owned()),
        field: None,
        trivia: false,
        start: 0,
        end: 1,
        attributes: None,
        text: LeafText::Text("a".to_owned()),
        lexical: Some(lexical),
    }
}

fn original_number_outcome(lexical: Value) -> ParseOutcome {
    ParseOutcome {
        ok: true,
        tree: Some(original_number_token(lexical)),
        ambiguities: Vec::new(),
        rejection: None,
        source_planes: None,
    }
}

#[test]
fn original_borrowed_number_predicates_payloads_and_wire_positions_match_every_independent_control()
{
    let numbers = vec![
        Number::from(0_u64),
        Number::from(-1_i64),
        Number::from(1_u64 << 60),
        Number::from_f64(-0.0).unwrap(),
        Number::from_f64(1.5).unwrap(),
        Number::from_f64(f64::from_bits(1)).unwrap(),
    ];
    let outcome = original_number_outcome(Value::Array(
        numbers.into_iter().map(Value::Number).collect(),
    ));
    let actual =
        original::capture_original_json_number_observations(&outcome, original_number_limits())
            .unwrap();
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/original-native-number-observations.lino"
    ))
    .unwrap();
    assert_eq!(fixture["numberVersions"]["minimum"], "1.0.151");
    assert_eq!(fixture["numberVersions"]["maximumExclusive"], "1.0.152");
    let expected = fixture["observations"].as_array().unwrap();
    assert_eq!(
        actual.len(),
        6,
        "host usize conversions are not original JSON Numbers"
    );
    assert_eq!(actual.len(), expected.len());
    for (observed, row) in actual.iter().zip(expected) {
        assert_eq!(
            observed.wire_occurrence,
            usize::try_from(row["wireOccurrence"].as_u64().unwrap()).unwrap()
        );
        assert_eq!(observed.signed, row["signed"].as_bool().unwrap());
        assert_eq!(observed.unsigned, row["unsigned"].as_bool().unwrap());
        assert_eq!(observed.floating, row["floating"].as_bool().unwrap());
        assert_eq!(
            observed.signed_decimal.as_deref(),
            row["signedDecimal"].as_str()
        );
        assert_eq!(
            observed.unsigned_decimal.as_deref(),
            row["unsignedDecimal"].as_str()
        );
        assert_eq!(
            observed.floating_bits.as_deref(),
            row["floatingBits"].as_str()
        );
    }
    assert!(
        actual
            .windows(2)
            .all(|pair| pair[0].wire_occurrence < pair[1].wire_occurrence)
    );
    publication::publish_original_number_controls(&actual).unwrap();
}

#[test]
fn recursive_original_number_fields_retain_distinct_occurrences_across_every_json_owner() {
    let mut outcome = original_number_outcome(Value::from(1));
    let root = outcome.tree.take().unwrap();
    outcome.tree = Some(SyntaxTree::Embed {
        language: "child".to_owned(),
        start: 0,
        end: 1,
        root: Box::new(root),
        source_children: Some(vec![original_number_token(Value::from(2))]),
        selection: Some(Box::new(GrammarSelection {
            mode: "boundary".to_owned(),
            mapping: "identity".to_owned(),
            start: 0,
            end: 1,
            candidates: vec![Value::from(3)],
            provenance: Value::from(4),
            alternatives: vec![GrammarSelectionAlternative {
                language: "other".to_owned(),
                root: Box::new(original_number_token(Value::from(5))),
                provenance: Value::from(6),
            }],
            ambiguous: true,
            source_mapping: Some(Value::from(7)),
        })),
    });
    outcome.source_planes = Some(vec![Value::from(8)]);
    outcome.ambiguities = vec![Ambiguity {
        rule: "boundary".to_owned(),
        start: 0,
        end: 1,
        languages: None,
        source: None,
        source_start: None,
        source_end: None,
        source_route: Some(vec![Value::from(9)]),
        offset_semantics: None,
    }];
    outcome.rejection = Some(ParseRejection {
        reason: "boundary",
        offset: None,
        line: None,
        column: None,
        expected: None,
        limit: None,
        diagnostics: Some(vec![Value::from(10)]),
        offset_semantics: None,
    });
    let facts =
        original::capture_original_json_number_observations(&outcome, original_number_limits())
            .unwrap();
    assert_eq!(facts.len(), 10);
    let mut values = facts
        .iter()
        .map(|row| {
            row.unsigned_decimal
                .as_deref()
                .unwrap()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    values.sort();
    assert_eq!(values, (1_u64..=10).collect::<Vec<_>>());
    assert!(
        facts
            .windows(2)
            .all(|pair| pair[0].wire_occurrence < pair[1].wire_occurrence)
    );
    assert!(
        facts
            .iter()
            .all(|row| row.signed && row.unsigned && !row.floating)
    );
}

#[test]
fn original_counting_limits_and_lossy_number_refusals_remain_before_observation_publication() {
    let ordinary = original_number_limits();
    let empty = ParseOutcome {
        ok: true,
        tree: None,
        ambiguities: Vec::new(),
        rejection: None,
        source_planes: None,
    };
    assert!(
        original::capture_original_json_number_observations(&empty, ordinary)
            .unwrap()
            .is_empty()
    );
    for limits in [
        StructuredDataLinksLimits {
            node_limit: 0,
            ..ordinary
        },
        StructuredDataLinksLimits {
            byte_limit: 0,
            ..ordinary
        },
        StructuredDataLinksLimits {
            depth_limit: 0,
            ..ordinary
        },
        StructuredDataLinksLimits {
            node_limit: 100_001,
            ..ordinary
        },
        StructuredDataLinksLimits {
            byte_limit: 16 * 1024 * 1024 + 1,
            ..ordinary
        },
        StructuredDataLinksLimits {
            depth_limit: 257,
            ..ordinary
        },
    ] {
        assert!(original::capture_original_json_number_observations(&empty, limits).is_err());
    }
    let rounded = original_number_outcome(Value::Number(Number::from(9_007_199_254_740_993_u64)));
    assert!(
        original::capture_original_json_number_observations(&rounded, ordinary)
            .unwrap_err()
            .contains("not exactly representable as binary64")
    );
    let original = original_number_outcome(Value::Array(vec![Value::from(1), Value::from(1)]));
    let facts = original::capture_original_json_number_observations(&original, ordinary).unwrap();
    assert_eq!(
        facts.len(),
        2,
        "equal independent source occurrences cannot collapse"
    );
    assert_ne!(facts[0].wire_occurrence, facts[1].wire_occurrence);
}
