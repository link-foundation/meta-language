//! Actual output fields preserve the source exception components.
use super::source_exception_rules as rules;
use super::{
    Language,
    ir::{DataDecl, Effect, Program},
};
pub(super) fn has_payload(entry: &DataDecl) -> bool {
    let Some(aborted) = entry.ctors.get(1) else {
        return false;
    };
    let names = aborted
        .fields
        .iter()
        .map(|field| field.name.clone())
        .collect::<Vec<_>>();
    let kinds = aborted
        .fields
        .iter()
        .map(|field| field.ty.kind().to_owned())
        .collect::<Vec<_>>();
    rules::accept_source_exception_payload_fields(entry.output, &names, &kinds)
}
pub(super) fn has_output(program: &Program) -> bool {
    program.source_language == Language::JavaScript
        && program
            .declarations
            .iter()
            .any(|entry| matches!(entry,super::ir::Decl::Data(data) if has_payload(data)))
        && program.main.as_ref().is_none_or(|main| {
            main.effects.iter().all(|effect| match effect {
                Effect::Unwrap { data, .. } => has_payload(program.data(data)),
                _ => true,
            })
        })
}
pub(super) fn pattern_suffix(program: &Program, name: &str) -> String {
    let count = program
        .data(name)
        .ctors
        .get(1)
        .map_or(0, |entry| entry.fields.len());
    let count = serde_json::Number::from(count)
        .as_f64()
        .expect("actual constructor field count");
    rules::read_source_exception_ignored_pattern_fields(count)
}
