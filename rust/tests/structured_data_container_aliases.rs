//! Original container-alias controls compile the actual private production codec.
//! Only this integration crate exposes its source modules and runtime type aliases.

pub use meta_language::{LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType};

pub mod link_network {
    pub use meta_language::Link;
}

#[path = "../src/structured_data_link_rules.rs"]
pub mod structured_data_link_rules;

#[path = "../src/structured_data_links.rs"]
pub mod structured_data_links;

use structured_data_links::{
    StructuredDataValue, encode_structured_data_links_ordered,
    encode_structured_data_links_ordered_with_container_aliases,
};

#[test]
fn explicit_container_aliases_preserve_owned_sharing_without_merging_equal_values() {
    let container = StructuredDataValue::Object(vec![
        (
            "value".encode_utf16().collect(),
            StructuredDataValue::Number(-0.0),
        ),
        (
            "units".encode_utf16().collect(),
            StructuredDataValue::String(vec![0xd800]),
        ),
    ]);
    let value = StructuredDataValue::Array(vec![container.clone(), container.clone(), container]);
    let StructuredDataValue::Array(children) = &value else {
        unreachable!();
    };
    let independent = encode_structured_data_links_ordered(&value).unwrap();
    let unchanged =
        encode_structured_data_links_ordered_with_container_aliases(&value, &[]).unwrap();
    assert_eq!(independent.network.to_lino(), unchanged.network.to_lino());
    let body = independent
        .network
        .link(independent.root)
        .unwrap()
        .references()[0];
    let references = independent.network.link(body).unwrap().references();
    assert_ne!(references[0], references[1]);
    assert_ne!(references[0], references[2]);
    let shared = encode_structured_data_links_ordered_with_container_aliases(
        &value,
        &[(&children[1], &children[0])],
    )
    .unwrap();
    let body = shared.network.link(shared.root).unwrap().references()[0];
    let references = shared.network.link(body).unwrap().references();
    assert_eq!(references[0], references[1]);
    assert_ne!(references[0], references[2]);
    assert_eq!(shared.network.len() + 3, independent.network.len());

    let arrays = StructuredDataValue::Array(vec![
        StructuredDataValue::Array(vec![]),
        StructuredDataValue::Array(vec![]),
        StructuredDataValue::Array(vec![]),
    ]);
    let StructuredDataValue::Array(children) = &arrays else {
        unreachable!();
    };
    let shared = encode_structured_data_links_ordered_with_container_aliases(
        &arrays,
        &[(&children[1], &children[0])],
    )
    .unwrap();
    let body = shared.network.link(shared.root).unwrap().references()[0];
    let references = shared.network.link(body).unwrap().references();
    assert_eq!(references[0], references[1]);
    assert_ne!(references[0], references[2]);
}

#[test]
fn container_aliases_refuse_unowned_changed_or_incomplete_targets() {
    let value = StructuredDataValue::Array(vec![
        StructuredDataValue::Array(vec![]),
        StructuredDataValue::Array(vec![]),
        StructuredDataValue::Array(vec![]),
    ]);
    let StructuredDataValue::Array(children) = &value else {
        unreachable!();
    };
    let foreign = StructuredDataValue::Array(vec![]);
    let scalar = StructuredDataValue::Null;
    for aliases in [
        vec![(&foreign, &children[0])],
        vec![(&children[1], &foreign)],
        vec![(&children[0], &children[1])],
        vec![(&children[1], &children[0]), (&children[1], &children[2])],
        vec![(&children[0], &children[0])],
        vec![(&children[1], &scalar)],
        vec![(&scalar, &children[0])],
    ] {
        assert!(
            encode_structured_data_links_ordered_with_container_aliases(&value, &aliases).is_err()
        );
    }

    let negative_zero = StructuredDataValue::Array(vec![StructuredDataValue::Number(-0.0)]);
    let positive_zero = StructuredDataValue::Array(vec![StructuredDataValue::Number(0.0)]);
    let first_order = StructuredDataValue::Object(vec![
        (vec![97], StructuredDataValue::Null),
        (vec![98], StructuredDataValue::Null),
    ]);
    let second_order = StructuredDataValue::Object(vec![
        (vec![98], StructuredDataValue::Null),
        (vec![97], StructuredDataValue::Null),
    ]);
    let first_units = StructuredDataValue::Array(vec![StructuredDataValue::String(vec![0xd800])]);
    let second_units = StructuredDataValue::Array(vec![StructuredDataValue::String(vec![0xd801])]);
    let infinite = StructuredDataValue::Array(vec![StructuredDataValue::Number(f64::INFINITY)]);
    for (target, alias) in [
        (negative_zero, positive_zero),
        (first_order, second_order),
        (first_units, second_units),
        (infinite.clone(), infinite),
    ] {
        let value = StructuredDataValue::Array(vec![target, alias]);
        let StructuredDataValue::Array(children) = &value else {
            unreachable!();
        };
        assert!(
            encode_structured_data_links_ordered_with_container_aliases(
                &value,
                &[(&children[1], &children[0])],
            )
            .is_err()
        );
    }

    let target = StructuredDataValue::Array(vec![StructuredDataValue::Null]);
    let mut nested = target.clone();
    for _ in 0..254 {
        nested = StructuredDataValue::Array(vec![nested]);
    }
    let value = StructuredDataValue::Array(vec![target, nested]);
    let StructuredDataValue::Array(children) = &value else {
        unreachable!();
    };
    let mut alias = &children[1];
    for _ in 0..254 {
        let StructuredDataValue::Array(children) = alias else {
            unreachable!();
        };
        alias = &children[0];
    }
    assert!(encode_structured_data_links_ordered(&value).is_err());
    assert!(
        encode_structured_data_links_ordered_with_container_aliases(
            &value,
            &[(alias, &children[0])],
        )
        .is_err()
    );
}
