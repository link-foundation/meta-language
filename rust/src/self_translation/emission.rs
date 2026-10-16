//! Existing item emission with an internal representation context.
use std::collections::HashMap;

use super::binding::declared_signatures;
use super::representation::Context;
use super::{RUST_MAIN, family, project_type_script_typing};
use crate::translation::check::check_program;
use crate::translation::diagnostics::TranslationError;
use crate::translation::emit_common::Emitted;
use crate::translation::emit_javascript::emit_javascript;
use crate::translation::emit_rust::{emit_rust_constants, emit_rust_with_representation};
use crate::translation::javascript::parse_javascript_bound_in;
use crate::translation::rust::parse_rust;
use crate::translation::surface::{External, SExpr};

/// The item emitted by the portable core with the signatures it gives the
/// module's other items, or `None` for a top-level statement.
pub(super) fn emit_item(
    text: &str,
    from: &str,
    to: &str,
    context: &Context,
    literal_bindings: &HashMap<String, SExpr>,
) -> Result<Option<(Emitted, Vec<External>)>, TranslationError> {
    let surface = if family(from) == "Rust" {
        // The Rust frontend reads a program, so an item alone gets an empty main.
        if RUST_MAIN.is_match(text) {
            parse_rust(text)?
        } else {
            parse_rust(&format!("{text}\nfn main() {{}}\n"))?
        }
    } else {
        if from == "TypeScript" {
            parse_javascript_bound_in(
                &project_type_script_typing(text)?,
                &context.module,
                literal_bindings,
                family(to) == "Rust",
            )?
        } else {
            parse_javascript_bound_in(
                text,
                &context.module,
                literal_bindings,
                family(to) == "Rust",
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
    let emitted = match constants {
        Some(emitted) => emitted,
        None if family(to) == "Rust" && context.utf16_strings => {
            match super::representation::validate(&program)? {
                Some(emitted) => emitted,
                None => emit_rust_with_representation(&program, true)?,
            }
        }
        None if family(to) == "Rust" => {
            emit_rust_with_representation(&program, context.utf16_strings)?
        }
        None => emit_javascript(&program)?,
    };
    let signatures = if program.imports.is_empty() {
        declared_signatures(&program, is_constant)
    } else {
        context.module.externals.clone()
    };
    Ok(Some((emitted, signatures)))
}
