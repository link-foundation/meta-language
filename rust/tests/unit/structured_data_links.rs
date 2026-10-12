//! Independent shared Links data and malformed graph regression cases.

use meta_language::{
    LinkId, LinkMetadata, LinkNetwork, LinkType, StructuredDataLinksLimits, StructuredDataValue,
    decode_structured_data_links, decode_structured_data_links_ordered,
    decode_structured_data_links_ordered_with_limits, encode_structured_data_links,
    encode_structured_data_links_ordered, percent_encode_links_text, read_structured_data_lino,
    read_structured_data_lino_ordered, read_structured_data_lino_ordered_with_limits,
    structured_data_to_lino, structured_data_to_lino_ordered,
};
use serde_json::json;

fn string(text: &str) -> StructuredDataValue {
    StructuredDataValue::String(text.encode_utf16().collect())
}

fn object(entries: Vec<(&str, StructuredDataValue)>) -> StructuredDataValue {
    StructuredDataValue::Object(
        entries
            .into_iter()
            .map(|(key, value)| (key.encode_utf16().collect(), value))
            .collect(),
    )
}

fn expected() -> StructuredDataValue {
    use StructuredDataValue::{Array, Boolean, Null, Number, String as Units};
    Array(vec![
        object(vec![
            ("a", string("λ 😀\n\0")),
            ("empty", string("")),
            (
                "keys",
                object(vec![
                    ("__proto__", Boolean(true)),
                    ("constructor", Boolean(false)),
                ]),
            ),
            (
                "numbers",
                Array(vec![
                    Number(-0.0),
                    Number(0.0),
                    Number(1.25),
                    Number(f64::from_bits(1)),
                    Number(f64::MAX),
                    Number(1e-7),
                    Number(1e21),
                ]),
            ),
        ]),
        Array(vec![Units(vec![0xd800]), Units(vec![0xdfff])]),
        Null,
        Boolean(true),
        Boolean(false),
    ])
}

#[test]
fn reused_unpaired_code_unit_strings_obey_depth_at_every_position() {
    let text = StructuredDataValue::String(vec![0xd800]);
    let mut boundary = text.clone();
    for _ in 0..253 {
        boundary = StructuredDataValue::Array(vec![boundary]);
    }
    let accepted = StructuredDataValue::Array(vec![text.clone(), boundary]);
    let graph = encode_structured_data_links_ordered(&accepted).unwrap();
    assert_eq!(
        decode_structured_data_links_ordered(&graph.network, graph.root).unwrap(),
        accepted
    );
    let mut deep = text.clone();
    for _ in 0..254 {
        deep = StructuredDataValue::Array(vec![deep]);
    }
    // The shallow first occurrence interns its code-unit graph. At the second
    // occurrence the string is at depth 256, so its unit would be at 257.
    let value = StructuredDataValue::Array(vec![text, deep]);
    assert!(
        encode_structured_data_links_ordered(&value)
            .unwrap_err()
            .contains("resource limit")
    );
}

fn metadata(term: &str, definition: Option<&str>) -> LinkMetadata {
    let metadata = LinkMetadata::new()
        .with_link_type(LinkType::Semantic)
        .with_named(true)
        .with_language("meta-language structured data")
        .with_term(term);
    if let Some(definition) = definition {
        metadata.with_definition(definition)
    } else {
        metadata
    }
}

fn statement(id: u64, term: &str, definition: Option<&str>, references: &[u64]) -> String {
    let references = references
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");
    let definition = definition.map_or_else(String::new, |value| {
        format!(" (def: {})", percent_encode_links_text(value))
    });
    format!(
        "({id}: {references} (meta: (t: semantic) (n: 1) (term: {}){definition} (lang: meta-language%20structured%20data)))\n",
        percent_encode_links_text(term),
    )
}

fn scalar(term: &str, definition: Option<&str>) -> String {
    statement(1, term, definition, &[]) + &statement(2, "structured data", Some("1"), &[1])
}

#[test]
fn shared_javascript_data_reloads_and_encodes_exact_ordered_links() {
    let source = include_str!("../../../parity/fixtures/structured-data-links.lino");
    let value = read_structured_data_lino_ordered(source).unwrap();
    assert_eq!(value, expected());
    assert_eq!(structured_data_to_lino_ordered(&value).unwrap(), source);
    let StructuredDataValue::Array(items) = value else {
        panic!("the shared fixture is an array");
    };
    let StructuredDataValue::Object(entries) = &items[0] else {
        panic!("the first shared value is an object");
    };
    let StructuredDataValue::Array(numbers) = &entries[3].1 else {
        panic!("the fourth entry contains numbers");
    };
    let StructuredDataValue::Number(negative_zero) = numbers[0] else {
        panic!("the first number is negative zero");
    };
    assert_eq!(negative_zero.to_bits(), (-0.0_f64).to_bits());
    let StructuredDataValue::Number(positive_zero) = numbers[1] else {
        panic!("the second number is positive zero");
    };
    assert_eq!(positive_zero.to_bits(), 0.0_f64.to_bits());
    assert!(
        read_structured_data_lino(source)
            .unwrap_err()
            .contains("unpaired UTF-16")
    );
}

#[test]
fn json_convenience_preserves_integer_consumers_and_finite_float_bits() {
    let value = json!({"schemaVersion": 1, "root": 9007199254740992_u64, "text": "漢字", "array": [true, false, null, 1.25]});
    let graph = encode_structured_data_links(&value).unwrap();
    let decoded = decode_structured_data_links(&graph.network, graph.root).unwrap();
    assert_eq!(decoded, value);
    assert_eq!(decoded["schemaVersion"].as_u64(), Some(1));
    assert_eq!(decoded["root"].as_u64(), Some(9_007_199_254_740_992));
    assert_eq!(
        read_structured_data_lino(&structured_data_to_lino(&value).unwrap()).unwrap(),
        value
    );
    assert!(encode_structured_data_links(&json!(9007199254740993_u64)).is_err());
    assert!(encode_structured_data_links(&json!(i64::MAX)).is_err());
    for number in [f64::from_bits(1), f64::MAX, -0.0, 1e20, 1e21, 1e-6, 1e-7] {
        let source = structured_data_to_lino_ordered(&StructuredDataValue::Number(number)).unwrap();
        let value = read_structured_data_lino(&source)
            .unwrap()
            .as_f64()
            .unwrap();
        assert_eq!(value.to_bits(), number.to_bits());
    }
}

#[test]
fn primary_reader_preserves_recorded_object_entry_order() {
    let ordered = object(vec![
        ("z", StructuredDataValue::Number(1.0)),
        ("a", StructuredDataValue::Number(2.0)),
    ]);
    let source = structured_data_to_lino_ordered(&ordered).unwrap();
    let value = read_structured_data_lino(&source).unwrap();
    assert_eq!(
        value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec!["z", "a"]
    );
    assert_eq!(value["z"].as_u64(), Some(1));
    assert_eq!(value["a"].as_u64(), Some(2));
    assert_eq!(structured_data_to_lino(&value).unwrap(), source);
}

#[test]
fn ordered_objects_preserve_integer_key_order_and_exact_surrogate_keys() {
    let value = StructuredDataValue::Object(vec![
        ("10".encode_utf16().collect(), string("first")),
        ("2".encode_utf16().collect(), string("second")),
        (
            vec![0xd800],
            StructuredDataValue::String(vec![0xdfff, 0, 0xd800]),
        ),
    ]);
    let source = structured_data_to_lino_ordered(&value).unwrap();
    assert_eq!(read_structured_data_lino_ordered(&source).unwrap(), value);
    assert!(read_structured_data_lino(&source).is_err());
    let duplicates = object(vec![("same", string("one")), ("same", string("two"))]);
    assert!(encode_structured_data_links_ordered(&duplicates).is_err());
}

#[test]
fn invalid_scalar_definitions_and_structural_arities_fail() {
    for definition in [
        "NaN",
        "Infinity",
        "-Infinity",
        "01",
        "+1",
        "1.0",
        "1e0",
        "-0.0",
        "1e309",
        "0x10",
        "1e-999",
    ] {
        assert!(
            read_structured_data_lino_ordered(&scalar("number value", Some(definition))).is_err(),
            "{definition}"
        );
    }
    for (term, definition) in [
        ("number value", None),
        ("boolean value", Some("TRUE")),
        ("boolean value", None),
        ("null value", Some("null")),
        ("not a structured value", None),
    ] {
        assert!(read_structured_data_lino_ordered(&scalar(term, definition)).is_err());
    }
    let bad_root = scalar("null value", None).replacen("(def: 1)", "(def: 2)", 1);
    assert!(read_structured_data_lino_ordered(&bad_root).is_err());
    let bad_arity =
        statement(1, "null value", None, &[1]) + &statement(2, "structured data", Some("1"), &[1]);
    assert!(read_structured_data_lino_ordered(&bad_arity).is_err());
    for unit in ["65536", "-1", "1.5", "01"] {
        let source = statement(1, "Unicode code unit", Some(unit), &[])
            + &statement(2, "Unicode code unit string", None, &[1])
            + &statement(3, "structured data", Some("1"), &[2]);
        assert!(read_structured_data_lino_ordered(&source).is_err());
    }
}

#[test]
fn duplicate_fields_roots_identities_and_foreign_metadata_fail() {
    let source = scalar("string value", Some("x"));
    for malformed in [
        source.replacen("(n: 1)", "(n: 1) (n: 1)", 1),
        source.replacen("(n: 1)", "(n: 01)", 1),
        source.replacen("(n: 1)", "(n: 0)", 1),
        source.replacen("(t: semantic)", "(t: token)", 1),
        source.replacen("(n: 1)", "(n: 1) (flags: 1)", 1),
        source.replacen("(n: 1)", "(n: 1) (span: 0 1 0 0 0 1)", 1),
        source.replacen("(n: 1)", "(n: 1) (reg: 1)", 1),
        source.replacen("meta-language%20structured%20data", "foreign", 1),
        source.clone() + &statement(1, "string value", Some("other"), &[]),
        source.clone() + &statement(3, "structured data", Some("1"), &[1]),
        source.clone() + &statement(3, "null value", None, &[]),
        source.replacen("(1:", "(01:", 1),
        source.replacen("(1:", "(9007199254740992:", 1),
    ] {
        assert!(read_structured_data_lino_ordered(&malformed).is_err());
    }
}

#[test]
fn cycles_dangling_references_and_misplaced_roles_fail() {
    for body in [
        statement(1, "array value", None, &[1]),
        statement(1, "array value", None, &[99]),
        statement(1, "Unicode code unit", Some("1"), &[]),
        statement(1, "key value pair", None, &[2, 2]),
        statement(1, "array value", None, &[3])
            + &statement(3, "Unicode code unit", Some("1"), &[]),
        statement(1, "Unicode code unit string", None, &[3])
            + &statement(3, "number value", Some("1"), &[]),
    ] {
        let source = body + &statement(2, "structured data", Some("1"), &[1]);
        assert!(read_structured_data_lino_ordered(&source).is_err());
    }
    let duplicates = statement(1, "object value", None, &[4, 5])
        + &statement(2, "string value", Some("same"), &[])
        + &statement(3, "null value", None, &[])
        + &statement(4, "key value pair", None, &[2, 3])
        + &statement(5, "key value pair", None, &[2, 3])
        + &statement(6, "structured data", Some("1"), &[1]);
    assert!(read_structured_data_lino_ordered(&duplicates).is_err());
}

#[test]
fn expansion_depth_and_scalar_byte_limits_are_enforced_on_graphs() {
    let mut network = LinkNetwork::new();
    let mut value = network.insert_link([], metadata("null value", None));
    for _ in 0..12 {
        value = network.insert_link([value, value], metadata("array value", None));
    }
    let root = network.insert_link([value], metadata("structured data", Some("1")));
    let limits = StructuredDataLinksLimits {
        node_limit: 100,
        ..StructuredDataLinksLimits::default()
    };
    assert!(
        decode_structured_data_links_ordered_with_limits(&network, root, limits)
            .unwrap_err()
            .contains("resource limit")
    );
    let limits = StructuredDataLinksLimits {
        depth_limit: 2,
        ..StructuredDataLinksLimits::default()
    };
    assert!(decode_structured_data_links_ordered_with_limits(&network, root, limits).is_err());

    let mut network = LinkNetwork::new();
    let value = network.insert_link([], metadata("string value", Some("four")));
    let array = network.insert_link([value, value, value], metadata("array value", None));
    let root = network.insert_link([array], metadata("structured data", Some("1")));
    let limits = StructuredDataLinksLimits {
        byte_limit: 10,
        ..StructuredDataLinksLimits::default()
    };
    assert!(decode_structured_data_links_ordered_with_limits(&network, root, limits).is_err());
    let limits = StructuredDataLinksLimits {
        byte_limit: 13,
        ..StructuredDataLinksLimits::default()
    };
    assert!(decode_structured_data_links_ordered_with_limits(&network, root, limits).is_ok());
    for limits in [
        StructuredDataLinksLimits {
            node_limit: 0,
            ..StructuredDataLinksLimits::default()
        },
        StructuredDataLinksLimits {
            depth_limit: 0,
            ..StructuredDataLinksLimits::default()
        },
        StructuredDataLinksLimits {
            byte_limit: 0,
            ..StructuredDataLinksLimits::default()
        },
    ] {
        assert!(decode_structured_data_links_ordered_with_limits(&network, root, limits).is_err());
    }
    assert!(
        read_structured_data_lino_ordered_with_limits(
            &scalar("null value", None),
            StructuredDataLinksLimits {
                byte_limit: 10,
                ..StructuredDataLinksLimits::default()
            }
        )
        .is_err()
    );
}

#[test]
fn graph_decoder_checks_metadata_and_selected_root() {
    let mut network = LinkNetwork::new();
    let value = network.insert_link([], metadata("null value", None));
    let root = network.insert_link([value], metadata("structured data", Some("1")));
    assert_eq!(
        decode_structured_data_links_ordered(&network, root).unwrap(),
        StructuredDataValue::Null
    );
    assert!(decode_structured_data_links_ordered(&network, value).is_err());
    assert!(decode_structured_data_links_ordered(&network, LinkId::from_u64(99)).is_err());
    let source = statement(1, "null value", None, &[])
        + "(2: (meta: (t: semantic) (n: 1) (term: structured%20data) (def: 1) (lang: meta-language%20structured%20data)) 1)\n";
    assert_eq!(
        read_structured_data_lino(&source).unwrap(),
        serde_json::Value::Null
    );
}

#[test]
fn a_shared_subtree_is_rechecked_at_its_deeper_occurrence() {
    let mut network = LinkNetwork::new();
    let scalar = network.insert_link([], metadata("number value", Some("0")));
    let shared = network.insert_link([scalar], metadata("array value", None));
    let nested = network.insert_link([shared], metadata("array value", None));
    let body = network.insert_link([shared, nested], metadata("array value", None));
    let root = network.insert_link([body], metadata("structured data", Some("1")));
    let limits = StructuredDataLinksLimits {
        depth_limit: 3,
        ..StructuredDataLinksLimits::default()
    };
    assert!(decode_structured_data_links_ordered_with_limits(&network, root, limits).is_err());
    let limits = StructuredDataLinksLimits {
        depth_limit: 4,
        ..StructuredDataLinksLimits::default()
    };
    assert_eq!(
        decode_structured_data_links_ordered_with_limits(&network, root, limits).unwrap(),
        StructuredDataValue::Array(vec![
            StructuredDataValue::Array(vec![StructuredDataValue::Number(0.0)]),
            StructuredDataValue::Array(vec![StructuredDataValue::Array(vec![
                StructuredDataValue::Number(0.0)
            ])])
        ])
    );
}
