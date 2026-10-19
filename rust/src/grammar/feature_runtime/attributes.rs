//! Native ordered storage; key classification and comparison are translated
//! from the complete shared JavaScript rule module.
use std::collections::{BTreeMap, btree_map};
use std::slice;

use super::operations::OperationValue;
use super::results::Tree;
use super::tree::SyntaxAttributes;
use crate::translation::grammar_attribute_order_rules::{
    compare_grammar_attribute_integer_keys, read_grammar_attribute_integer_key,
};

fn attribute_integer_key(name: &str) -> f64 {
    // More than ten units always refuse the integer-key profile. Retain one
    // extra unit to distinguish that case without copying an arbitrary name.
    let units: Vec<_> = name.encode_utf16().take(11).map(f64::from).collect();
    read_grammar_attribute_integer_key(&units)
}

pub(super) fn set_attribute(node: &mut Tree, name: &str, value: OperationValue) {
    let attributes = node.attributes.get_or_insert_with(Default::default);
    let order = node
        .attribute_order
        .get_or_insert_with(|| attributes.keys().cloned().collect());
    if let Some(stored) = attributes.get_mut(name) {
        *stored = value;
        return;
    }
    let index = attribute_integer_key(name);
    let position = if index.is_sign_negative() {
        order.len()
    } else {
        order.partition_point(|key| {
            compare_grammar_attribute_integer_keys(attribute_integer_key(key), index)
                .is_sign_negative()
        })
    };
    order.insert(position, name.to_owned());
    attributes.insert(name.to_owned(), value);
}

pub(super) struct AttributeEntries<'a> {
    values: &'a BTreeMap<String, OperationValue>,
    order: Option<slice::Iter<'a, String>>,
    historical: btree_map::Iter<'a, String, OperationValue>,
}

impl<'a> Iterator for AttributeEntries<'a> {
    type Item = (&'a String, &'a OperationValue);

    fn next(&mut self) -> Option<Self::Item> {
        self.order.as_mut().map_or_else(
            || self.historical.next(),
            |order| {
                order.next().map(|name| {
                    (
                        name,
                        self.values
                            .get(name)
                            .expect("stored attribute name owns its value"),
                    )
                })
            },
        )
    }
}

pub(super) fn attribute_entries(node: &Tree) -> Option<AttributeEntries<'_>> {
    node.attributes.as_ref().map(|values| AttributeEntries {
        values,
        order: node.attribute_order.as_deref().map(<[String]>::iter),
        historical: values.iter(),
    })
}

pub(super) fn public_attributes(node: &Tree) -> Option<SyntaxAttributes> {
    attribute_entries(node).map(|entries| {
        entries
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect()
    })
}
