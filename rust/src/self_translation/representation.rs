//! Mechanical CST and checked-value boundaries for the shared representation.
use std::sync::OnceLock;

use crate::translation::diagnostics::{Result, type_error, unsupported};
use crate::translation::ir::Program;
use crate::translation::javascript::ModuleContext;
use crate::{LinkNetwork, LinkType};
use serde_json::Value;

use super::family;
use super::representation_rules::{
    accept_self_translation_utf16_member, require_self_translation_utf16_representation,
    require_self_translation_utf16_text,
};

#[derive(Default)]
pub(super) struct Context {
    pub(super) module: ModuleContext,
    pub(super) utf16_strings: bool,
}

fn policy() -> &'static Value {
    static POLICY: OnceLock<Value> = OnceLock::new();
    POLICY.get_or_init(|| {
        crate::read_structured_data_lino(include_str!(
            "../data/self-translation-representation.lino"
        ))
        .expect("the generated representation asset is valid")
    })
}

pub fn string_prelude() -> String {
    policy()["prelude"]
        .as_str()
        .expect("the generated representation asset has exact prelude text")
        .to_owned()
}

pub(super) fn select(network: &LinkNetwork, source: &str, language: &str, explicit: bool) -> bool {
    let member = !explicit
        && family(language) == "JavaScript"
        && network.links().any(|relation| {
            if relation.metadata().link_type() != Some(LinkType::Field) {
                return false;
            }
            let [parent, label, child] = relation.references() else {
                return false;
            };
            if network
                .link(*parent)
                .and_then(|link| link.metadata().term())
                != Some("member_expression")
                || network.link(*label).and_then(|link| link.metadata().term()) != Some("property")
            {
                return false;
            }
            network
                .link(*child)
                .and_then(|link| link.metadata().span())
                .and_then(|span| {
                    let range = span.byte_range();
                    source.get(range.start()..range.end())
                })
                .is_some_and(accept_self_translation_utf16_member)
        });
    require_self_translation_utf16_representation(explicit, family(language), member)
}

fn contains_text(value: &Value) -> bool {
    match value {
        Value::Object(fields) => {
            fields
                .get("kind")
                .and_then(Value::as_str)
                .is_some_and(require_self_translation_utf16_text)
                || fields.values().any(contains_text)
        }
        Value::Array(values) => values.iter().any(contains_text),
        _ => false,
    }
}

pub(super) fn validate(program: &Program) -> Result<()> {
    // Every nested checked type and bound external signature participates.
    // Primitive source strings and comments are not type records.
    let value =
        serde_json::to_value(program).map_err(|failure| type_error(failure.to_string(), None))?;
    if contains_text(&value) {
        return Err(unsupported(
            "UTF16 Text representation",
            policy()["nativeBoundary"]
                .as_str()
                .expect("the generated representation asset has its native boundary"),
            None,
        ));
    }
    Ok(())
}
