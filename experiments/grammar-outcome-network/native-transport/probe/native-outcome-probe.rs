//! PRIVATE, CI-only scratch integration test. This source is not a native observation.
//! The candidate API exists only in the separately authenticated scratch overlay.

use std::collections::HashMap;
use std::fs;

use meta_language::structured_data_links::encode_grammar_parse_outcome_links;
use meta_language::{
    Ambiguity, BinaryLinksOptions, FeatureParseOptions, Grammar, GrammarSelection,
    GrammarSelectionAlternative, LeafText, LinkNetwork, OperationValue, ParseOutcome,
    ParseRejection, StructuredDataLinks, StructuredDataLinksLimits, StructuredDataValue,
    SyntaxAttributes, SyntaxTree, check_grammar_selection_mapped_tree, compile_feature_grammar,
    decode_link_network_binary, decode_structured_data_links,
    decode_structured_data_links_ordered_with_limits, encode_link_network_binary,
    parse_grammar_links, read_structured_data_lino, read_structured_data_lino_ordered,
    structured_data_to_lino,
};
use serde_json::{Value, json};

fn member<'a>(value: &'a StructuredDataValue, name: &str) -> &'a StructuredDataValue {
    let StructuredDataValue::Object(entries) = value else {
        panic!("expected object at {name}")
    };
    entries
        .iter()
        .find(|(key, _)| key.iter().copied().eq(name.encode_utf16()))
        .map(|(_, value)| value)
        .unwrap_or_else(|| panic!("missing ordered field {name}"))
}

fn array(value: &StructuredDataValue) -> &[StructuredDataValue] {
    let StructuredDataValue::Array(values) = value else {
        panic!("expected ordered array")
    };
    values
}

// A value comparator only: actual Number bits, ordered UTF-16 keys and every field.
// It neither constructs a wire graph nor chooses any numerical spelling.
fn exact(actual: &StructuredDataValue, expected: &StructuredDataValue) {
    match (actual, expected) {
        (StructuredDataValue::Number(a), StructuredDataValue::Number(b)) => {
            assert_eq!(a.to_bits(), b.to_bits(), "number bits, including -0");
        }
        (StructuredDataValue::Array(a), StructuredDataValue::Array(b)) => {
            assert_eq!(a.len(), b.len());
            for (a, b) in a.iter().zip(b) {
                exact(a, b);
            }
        }
        (StructuredDataValue::Object(a), StructuredDataValue::Object(b)) => {
            assert_eq!(a.len(), b.len());
            for ((ak, av), (bk, bv)) in a.iter().zip(b) {
                assert_eq!(ak, bk, "actual ordered object keys");
                exact(av, bv);
            }
        }
        _ => assert_eq!(actual, expected),
    }
}

fn limits(row: &Value) -> StructuredDataLinksLimits {
    let size = |key: &str| {
        usize::try_from(row["limits"][key].as_u64().expect("unsigned limit"))
            .expect("limit fits usize")
    };
    StructuredDataLinksLimits {
        node_limit: size("nodes"),
        byte_limit: size("bytes"),
        depth_limit: size("depth"),
    }
}

fn replay(packet: &StructuredDataLinks, expected: &StructuredDataValue) {
    // Independent historical decoder budgets, not the candidate's exact expanded
    // scalar-byte budget. Serialized text/binary size has a different meaning.
    let replay_limits = StructuredDataLinksLimits::default();
    assert_eq!(
        packet.network.links().last().unwrap().id(),
        packet.root,
        "schema root is emitted last"
    );
    let text = packet.network.to_lino();
    let text_graph = LinkNetwork::from_lino(&text).expect("this actual native text graph reloads");
    assert_eq!(text_graph.to_lino(), text, "native graph exact text replay");
    let binary = encode_link_network_binary(&packet.network, BinaryLinksOptions::default())
        .expect("encode this actual native graph");
    let binary_graph = decode_link_network_binary(&binary, BinaryLinksOptions::default())
        .expect("native binary graph reloads");
    assert_eq!(
        binary_graph.to_lino(),
        text,
        "this native graph's IDs/references/metadata"
    );
    assert_eq!(
        encode_link_network_binary(&binary_graph, BinaryLinksOptions::default()).unwrap(),
        binary
    );
    for graph in [&packet.network, &text_graph, &binary_graph] {
        exact(
            &decode_structured_data_links_ordered_with_limits(graph, packet.root, replay_limits)
                .unwrap(),
            expected,
        );
    }
    // Deliberately no comparison of JS versus native physical graph IDs or aliases.
}

fn parse(row: &Value) -> ParseOutcome {
    let mut grammars: HashMap<String, Grammar> = HashMap::new();
    if let Some(bindings) = row["grammars"].as_object() {
        for (name, listing) in bindings {
            grammars.insert(
                name.clone(),
                parse_grammar_links(listing.as_str().unwrap()).unwrap(),
            );
        }
    }
    let resolve = |name: &str| grammars.get(name).cloned();
    let options = FeatureParseOptions {
        grammar_name: row["options"]["grammarName"].as_str().map(str::to_owned),
        error_recovery: row["options"]["errorRecovery"].as_bool(),
        accept_recovery: (row["options"]["recovery"].as_str() == Some("accept")).then_some(true),
        reject_ambiguity: (row["options"]["ambiguity"].as_str() == Some("reject")).then_some(true),
        ..FeatureParseOptions::default()
    };
    let grammar = parse_grammar_links(row["listing"].as_str().unwrap()).unwrap();
    let parser = compile_feature_grammar(&grammar, Some(&resolve), options).unwrap();
    let bytes = row["sourceBytes"].as_array().map_or_else(
        || row["source"].as_str().unwrap().as_bytes().to_vec(),
        |values| {
            values
                .iter()
                .map(|value| u8::try_from(value.as_u64().unwrap()).unwrap())
                .collect()
        },
    );
    parser
        .parse_tree(&bytes, &FeatureParseOptions::default())
        .unwrap()
}

fn minimal() -> ParseOutcome {
    ParseOutcome {
        ok: true,
        tree: None,
        ambiguities: Vec::new(),
        rejection: None,
        source_planes: None,
    }
}

fn opaque() -> Value {
    json!({"negativeZero": -0.0, "empty": [], "nothing": null, "flag": false, "zero": 0, "text": "é"})
}

fn token() -> SyntaxTree {
    SyntaxTree::Token {
        kind: Some("word".into()),
        field: Some("value".into()),
        trivia: true,
        start: 0,
        end: 1,
        attributes: None,
        text: LeafText::Text("x".into()),
        lexical: None,
    }
}

fn full_boundary() -> ParseOutcome {
    // Owned enum boundary values, not a claim that a grammar/action language can
    // produce these arbitrary metadata records or out-of-safe-range operation integers.
    let attributes: SyntaxAttributes = [
        ("2".into(), OperationValue::Integer(2)),
        ("10".into(), OperationValue::Integer(10)),
        ("z".into(), OperationValue::Text("last".into())),
        ("a".into(), OperationValue::Text("first".into())),
        (
            "wide".into(),
            OperationValue::Integer(9_007_199_254_740_992),
        ),
        ("minimum".into(), OperationValue::Integer(i64::MIN)),
    ]
    .into_iter()
    .collect();
    let selection = GrammarSelection {
        mode: "boundary".into(),
        mapping: "identity".into(),
        start: 0,
        end: 4,
        candidates: vec![opaque()],
        provenance: Value::Null,
        alternatives: vec![GrammarSelectionAlternative {
            language: "alternative".into(),
            root: Box::new(SyntaxTree::Missing {
                kind: None,
                start: 1,
                end: 1,
                literal: false,
            }),
            provenance: json!([]),
        }],
        ambiguous: true,
        source_mapping: Some(Value::Null),
    };
    ParseOutcome {
        ok: false,
        tree: Some(SyntaxTree::Node {
            kind: "boundary".into(),
            field: Some(String::new()),
            trivia: true,
            start: 0,
            end: 4,
            children: vec![
                SyntaxTree::Token {
                    kind: None,
                    field: None,
                    trivia: false,
                    start: 0,
                    end: 1,
                    attributes: Some(SyntaxAttributes::new()),
                    text: LeafText::Hex("ff".into()),
                    lexical: Some(opaque()),
                },
                SyntaxTree::Error {
                    start: 1,
                    end: 3,
                    text: LeafText::Text("π".into()),
                    reason: Some(String::new()),
                },
                SyntaxTree::Missing {
                    kind: Some("word".into()),
                    start: 3,
                    end: 3,
                    literal: true,
                },
                SyntaxTree::Embed {
                    language: "child".into(),
                    start: 0,
                    end: 4,
                    root: Box::new(token()),
                    selection: Some(Box::new(selection)),
                    source_children: Some(Vec::new()),
                },
            ],
            attributes: Some(attributes),
        }),
        ambiguities: vec![Ambiguity {
            rule: "boundary".into(),
            start: 0,
            end: 4,
            languages: Some(Vec::new()),
            source: Some(0),
            source_start: Some(0),
            source_end: Some(4),
            source_route: Some(vec![opaque()]),
            offset_semantics: Some("boundary".into()),
        }],
        rejection: Some(ParseRejection {
            reason: "boundary",
            offset: Some(0),
            line: Some(1),
            column: Some(2),
            expected: Some(vec!["x".into(), "π".into()]),
            limit: Some(9),
            diagnostics: Some(vec![opaque()]),
            offset_semantics: Some("boundary".into()),
        }),
        source_planes: Some(vec![opaque()]),
    }
}

fn constructed(id: &str) -> ParseOutcome {
    match id {
        "minimal" => minimal(),
        "all-native-variants" => full_boundary(),
        "repeated-scalars" => ParseOutcome {
            source_planes: Some(vec![json!(0); 32]),
            ..minimal()
        },
        _ => panic!("unknown independent boundary {id}"),
    }
}

fn refusal(outcome: &ParseOutcome, limit: StructuredDataLinksLimits, phrase: &str) {
    let error = encode_grammar_parse_outcome_links(outcome, limit)
        .expect_err("must refuse before publication");
    assert!(error.contains(phrase), "expected {phrase}: {error}");
}

fn independent_boundaries() {
    let ordinary = StructuredDataLinksLimits::default();
    let base = minimal();
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
    ] {
        refusal(&base, limits, "positive");
    }
    refusal(
        &base,
        StructuredDataLinksLimits {
            depth_limit: 257,
            ..ordinary
        },
        "at most 256",
    );

    let long = ParseOutcome {
        source_planes: Some(vec![Value::String("x".repeat(1024))]),
        ..minimal()
    };
    refusal(
        &long,
        StructuredDataLinksLimits {
            byte_limit: 64,
            ..ordinary
        },
        "scalar byte resource limit",
    );
    let mut deep = token();
    for _ in 0..300 {
        deep = SyntaxTree::Node {
            kind: "deep".into(),
            field: None,
            trivia: false,
            start: 0,
            end: 1,
            children: vec![deep],
            attributes: None,
        };
    }
    refusal(
        &ParseOutcome {
            tree: Some(deep),
            ..minimal()
        },
        ordinary,
        "structural resource limit",
    );

    // Actual serde_json Number domain and existing exact codec, not a new cast.
    for integer in [9_007_199_254_740_993_i64, i64::MAX] {
        let opaque_number = ParseOutcome {
            source_planes: Some(vec![json!(integer)]),
            ..minimal()
        };
        refusal(
            &opaque_number,
            ordinary,
            "exactly representable as binary64",
        );
        let attributes = [("integer".into(), OperationValue::Integer(integer))]
            .into_iter()
            .collect();
        let operation_number = ParseOutcome {
            tree: Some(SyntaxTree::Node {
                kind: "integer-boundary".into(),
                field: None,
                trivia: false,
                start: 0,
                end: 0,
                children: Vec::new(),
                attributes: Some(attributes),
            }),
            ..minimal()
        };
        refusal(
            &operation_number,
            ordinary,
            "exactly representable as binary64",
        );
    }

    // Native IndexMap order is authoritative, even where JS object enumeration
    // would reorder numeric keys. This case is NOT advertised as parser parity.
    let native_keys = ["z", "2", "a"];
    let attrs = native_keys
        .iter()
        .map(|key| ((*key).into(), OperationValue::Integer(1)))
        .collect();
    let owned = ParseOutcome {
        tree: Some(SyntaxTree::Node {
            kind: "owned-order".into(),
            field: None,
            trivia: false,
            start: 0,
            end: 0,
            children: Vec::new(),
            attributes: Some(attrs),
        }),
        ..minimal()
    };
    let graph = encode_grammar_parse_outcome_links(&owned, ordinary).unwrap();
    let expected = StructuredDataValue::Object(vec![
        (
            "ok".encode_utf16().collect(),
            StructuredDataValue::Boolean(true),
        ),
        (
            "tree".encode_utf16().collect(),
            StructuredDataValue::Object(vec![
                (
                    "type".encode_utf16().collect(),
                    StructuredDataValue::String("node".encode_utf16().collect()),
                ),
                (
                    "kind".encode_utf16().collect(),
                    StructuredDataValue::String("owned-order".encode_utf16().collect()),
                ),
                (
                    "start".encode_utf16().collect(),
                    StructuredDataValue::Number(0.0),
                ),
                (
                    "end".encode_utf16().collect(),
                    StructuredDataValue::Number(0.0),
                ),
                (
                    "children".encode_utf16().collect(),
                    StructuredDataValue::Array(Vec::new()),
                ),
                (
                    "attributes".encode_utf16().collect(),
                    StructuredDataValue::Object(
                        native_keys
                            .iter()
                            .map(|key| {
                                (
                                    key.encode_utf16().collect(),
                                    StructuredDataValue::Number(1.0),
                                )
                            })
                            .collect(),
                    ),
                ),
            ]),
        ),
        (
            "ambiguities".encode_utf16().collect(),
            StructuredDataValue::Array(Vec::new()),
        ),
        (
            "rejection".encode_utf16().collect(),
            StructuredDataValue::Null,
        ),
    ]);
    replay(&graph, &expected);
}

#[test]
fn borrowed_outcomes_match_fresh_javascript_values_and_independent_boundaries() {
    let source = fs::read_to_string(
        std::env::var("META_LANGUAGE_B95_OUTCOME_FIXTURE")
            .expect("runner supplies the fresh private JS oracle fixture"),
    )
    .unwrap();
    let fixture = read_structured_data_lino(&source).unwrap();
    let ordered = read_structured_data_lino_ordered(&source).unwrap();
    assert_eq!(fixture["schemaVersion"], 1);
    assert_eq!(
        fixture["profile"],
        "private borrowed native outcome probe 1"
    );
    let rows = fixture["cases"].as_array().unwrap();
    let ordered_rows = array(member(&ordered, "cases"));
    assert_eq!(rows.len(), 14);
    assert_eq!(ordered_rows.len(), rows.len());
    let mut parsed = 0;
    let mut constructed_rows = 0;
    let mut failures = Vec::new();
    for (row, ordered_row) in rows.iter().zip(ordered_rows) {
        let case_label = row["id"]
            .as_str()
            .unwrap_or("invalid case identity")
            .to_owned();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let id = row["id"].as_str().unwrap();
            let outcome = if row["listing"].is_string() {
                parsed += 1;
                assert_eq!(row["origin"], "fresh JavaScript public grammar outcome");
                let actual = parse(row);
                assert_eq!(actual.ok, row["ok"].as_bool().unwrap(), "{id}");
                if let Some(reason) = row["reason"].as_str() {
                    assert_eq!(actual.rejection.as_ref().unwrap().reason, reason, "{id}");
                }
                actual
            } else {
                constructed_rows += 1;
                assert_eq!(
                    row["origin"],
                    "independent constructed value; not a parser/mapping validity claim"
                );
                constructed(id)
            };
            let expected = member(ordered_row, "expected");
            let exact_limits = limits(row);
            assert!(
                exact_limits.node_limit > 1
                    && exact_limits.byte_limit > 1
                    && exact_limits.depth_limit > 1
            );
            let packet = encode_grammar_parse_outcome_links(&outcome, exact_limits)
                .unwrap_or_else(|error| panic!("{id}: {error}"));
            if id == "all-native-variants" {
                assert!(
                    packet
                        .network
                        .links()
                        .any(|link| link.metadata().term() == Some("number value")
                            && link.metadata().definition() == Some("-0")),
                    "actual negative-zero scalar spelling"
                );
            }
            replay(&packet, expected);
            replay(
                &encode_grammar_parse_outcome_links(&outcome, StructuredDataLinksLimits::default())
                    .unwrap(),
                expected,
            );
            if id == "repeated-scalars" {
                assert!(
                    packet.network.len() < exact_limits.node_limit,
                    "expanded costs include reused scalar references"
                );
            }
            if row["mapped"] == true {
                let value = decode_structured_data_links(&packet.network, packet.root).unwrap();
                check_grammar_selection_mapped_tree(&value, &FeatureParseOptions::default())
                    .unwrap();
            }
            for smaller in [
                StructuredDataLinksLimits {
                    node_limit: exact_limits.node_limit - 1,
                    ..exact_limits
                },
                StructuredDataLinksLimits {
                    byte_limit: exact_limits.byte_limit - 1,
                    ..exact_limits
                },
                StructuredDataLinksLimits {
                    depth_limit: exact_limits.depth_limit - 1,
                    ..exact_limits
                },
            ] {
                refusal(&outcome, smaller, "resource limit");
            }
        }));
        if result.is_err() {
            failures.push(case_label);
        }
    }
    let counts = std::panic::catch_unwind(|| {
        assert_eq!((parsed, constructed_rows), (11, 3));
    });
    if counts.is_err() {
        failures.push("case counts".to_owned());
    }
    let boundaries = std::panic::catch_unwind(|| {
        independent_boundaries();
    });
    if boundaries.is_err() {
        failures.push("independent boundaries".to_owned());
    }
    assert!(
        failures.is_empty(),
        "native outcome checks failed: {failures:?}"
    );
    // Report only AFTER every native assertion above, through the old generic
    // codec, never through the candidate whose behavior is under examination.
    if let Some(path) = std::env::var_os("META_LANGUAGE_B95_OUTCOME_REPORT") {
        let report = json!({"schemaVersion": 1, "profile": "private borrowed native outcome probe 1",
            "javascriptOutcomeRows": parsed, "independentConstructedRows": constructed_rows,
            "archiveReplay": ["each actual native graph text", "each actual native graph binary"],
            "valueComparison": "complete ordered fields and binary64 bits",
            "limits": "expanded occurrence/scalar-byte/depth exact and one-below boundaries",
            "scope": "scratch candidate only; no public/default CST, alias identity or general-language support claim"});
        fs::write(path, structured_data_to_lino(&report).unwrap()).unwrap();
    }
}
