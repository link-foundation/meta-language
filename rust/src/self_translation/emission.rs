//! Existing item emission with an internal representation context.
use std::collections::HashMap;

use super::binding::declared_signatures;
use super::representation::Context;
use super::{
    Item, RUST_MAIN, SelfTranslation, SelfTranslationItem, SelfTranslationSignature, family,
    project_type_script_typing, sha256,
};
use crate::LinkNetwork;
use crate::query::QueryIndex;
use crate::translation::check::check_program;
use crate::translation::diagnostics::TranslationError;
use crate::translation::emit_common::Emitted;
use crate::translation::emit_javascript::emit_javascript;
use crate::translation::emit_rust::{emit_rust_constants, emit_rust_with_representation};
use crate::translation::frontend_rules::accept_source_envelope_restoration;
use crate::translation::ir::Program;
use crate::translation::javascript::{
    parse_javascript, parse_javascript_bound_in_with_module_scope,
};
use crate::translation::primitive_method_rules::read_primitive_method_lowering_form;
use crate::translation::rust::parse_rust;
use crate::translation::surface::{External, SExpr};
use regex::Regex;
use std::sync::LazyLock;

pub(super) fn uses_relative_element_call(network: &LinkNetwork, source: &str) -> bool {
    let index = QueryIndex::new(network);
    network
        .links()
        .filter(|link| link.metadata().term() == Some("call_expression"))
        .flat_map(|call| index.field_targets(call.id(), "function"))
        .filter(|member| {
            index.link(*member).and_then(|link| link.metadata().term()) == Some("member_expression")
        })
        .flat_map(|member| index.field_targets(member, "property"))
        .filter_map(|name| index.link(name).and_then(|link| link.metadata().span()))
        .filter_map(|span| {
            let range = span.byte_range();
            source.get(range.start()..range.end())
        })
        .any(|name| {
            read_primitive_method_lowering_form("array", name) == "relative-element-template"
        })
}

pub(super) fn emit_complete_relative_element_program(
    source: &str,
    utf16_strings: bool,
) -> Option<String> {
    let program = check_program(&parse_javascript(source).ok()?).ok()?;
    if program
        .main
        .as_ref()
        .is_none_or(|main| main.effects.is_empty())
        || !program.imports.is_empty()
        || !program.externals.is_empty()
    {
        return None;
    }
    emit_rust_with_representation(&program, utf16_strings)
        .ok()
        .map(|emitted| emitted.text)
}

pub(super) fn standalone_translation(
    from: &'static str,
    to: &'static str,
    code: String,
    items: &[Item<'_>],
    status: &'static str,
) -> (SelfTranslation, Vec<SelfTranslationSignature>) {
    let items = items
        .iter()
        .map(|item| SelfTranslationItem {
            term: item.term.clone(),
            start: item.start,
            end: item.end,
            status: if item.comment { "provenance" } else { status },
            reason: None,
        })
        .collect();
    (
        SelfTranslation {
            source_language: from,
            target_language: to,
            code,
            items,
        },
        Vec::new(),
    )
}

pub(super) fn source_envelope(source: &str, tail: &str) -> String {
    let original = serde_json::to_string(source)
        .expect("source text serializes")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029");
    format!(" envelope-sha256={} original={original}", sha256(tail))
}

pub(super) fn restore_source_envelope(
    source: &str,
    header: &Item<'_>,
    target: &str,
) -> Option<String> {
    static SOURCE_ENVELOPE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r" envelope-sha256=([a-f0-9]{64}) original=(.*)$").unwrap());
    let envelope = SOURCE_ENVELOPE.captures(header.text)?;
    let original = serde_json::from_str::<String>(&envelope[2]).ok()?;
    accept_source_envelope_restoration(
        header.text.contains(&format!(" source={target} ")),
        sha256(&source[header.end..]) == envelope[1],
        header.text.split_ascii_whitespace().find_map(|field| {
            field
                .strip_prefix("bytes=")
                .and_then(|value| value.parse::<usize>().ok())
        }) == Some(original.len())
            && header
                .text
                .contains(&format!(" sha256={} ", sha256(&original))),
    )
    .then_some(original)
}

/// The item emitted by the portable core with the signatures it gives the
/// module's other items, or `None` for a top-level statement.
pub(super) fn emit_item(
    text: &str,
    from: &str,
    to: &str,
    context: &Context,
    literal_bindings: &HashMap<String, SExpr>,
) -> Result<Option<(Emitted, Vec<External>)>, TranslationError> {
    check_and_emit_item(text, from, to, context, literal_bindings)
        .map(|item| item.map(|item| (item.emitted, item.signatures)))
}

/// The exact program and emission already produced by the normal item path.
pub(super) struct CheckedItem {
    pub(super) program: Program,
    pub(super) emitted: Emitted,
    pub(super) signatures: Vec<External>,
    pub(super) definition_ownership: Option<crate::translation::emit_rust::DefinitionOwnership>,
    pub(super) source: String,
    pub(super) source_language: String,
}

pub(super) fn check_and_emit_item(
    text: &str,
    from: &str,
    to: &str,
    context: &Context,
    literal_bindings: &HashMap<String, SExpr>,
) -> Result<Option<CheckedItem>, TranslationError> {
    check_and_emit_item_profile(text, from, to, context, literal_bindings, false)
}

pub(super) fn check_and_emit_item_with_ownership(
    text: &str,
    from: &str,
    to: &str,
    context: &Context,
    literal_bindings: &HashMap<String, SExpr>,
) -> Result<Option<CheckedItem>, TranslationError> {
    check_and_emit_item_profile(text, from, to, context, literal_bindings, true)
}

fn check_and_emit_item_profile(
    text: &str,
    from: &str,
    to: &str,
    context: &Context,
    literal_bindings: &HashMap<String, SExpr>,
    record_ownership: bool,
) -> Result<Option<CheckedItem>, TranslationError> {
    let surface = if family(from) == "Rust" {
        // The Rust frontend reads a program, so an item alone gets an empty main.
        if RUST_MAIN.is_match(text) {
            parse_rust(text)?
        } else {
            parse_rust(&format!("{text}\nfn main() {{}}\n"))?
        }
    } else {
        if from == "TypeScript" {
            parse_javascript_bound_in_with_module_scope(
                &project_type_script_typing(text)?,
                &context.module,
                literal_bindings,
                family(to) == "Rust",
                context.module_binding_scope.as_ref(),
            )?
        } else {
            parse_javascript_bound_in_with_module_scope(
                text,
                &context.module,
                literal_bindings,
                family(to) == "Rust",
                context.module_binding_scope.as_ref(),
            )?
        }
    };
    let program = check_program(&surface)?;
    // A top-level constant is a Rust constant; other top-level statements run
    // once, as a program.
    let constants = if family(to) == "Rust" {
        emit_rust_constants(&program)?
    } else {
        None
    };
    let is_constant = constants.is_some();
    if !is_constant
        && program
            .main
            .as_ref()
            .is_some_and(|main| !main.effects.is_empty())
    {
        return Ok(None);
    }
    let (emitted, definition_ownership) = match constants {
        Some(emitted) => (emitted, None),
        None if family(to) == "Rust" && context.utf16_strings => {
            match super::representation::validate(&program)? {
                Some(emitted) => (emitted, None),
                None if record_ownership => {
                    crate::translation::emit_rust::emit_rust_with_definition_ownership(
                        &program, true,
                    )?
                }
                None => (emit_rust_with_representation(&program, true)?, None),
            }
        }
        None if family(to) == "Rust" && record_ownership => {
            crate::translation::emit_rust::emit_rust_with_definition_ownership(
                &program,
                context.utf16_strings,
            )?
        }
        None if family(to) == "Rust" => (
            emit_rust_with_representation(&program, context.utf16_strings)?,
            None,
        ),
        None => (emit_javascript(&program)?, None),
    };
    let signatures = if program.imports.is_empty() {
        declared_signatures(&program, is_constant)
    } else {
        context.module.externals.clone()
    };
    Ok(Some(CheckedItem {
        program,
        emitted,
        signatures,
        definition_ownership,
        source: text.to_owned(),
        source_language: from.to_owned(),
    }))
}
