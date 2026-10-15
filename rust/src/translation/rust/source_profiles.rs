//! Existing independently selected Rust source factories.
//! Public reexports preserve the frontend API and shared parser implementation.
use super::{Result, SProgram, SourceProfiles, parse_rust_with_selected_profiles};

/// Reads a Rust program.
///
/// # Errors
/// On syntax errors and on constructs outside the portable core.
pub fn parse_rust(source: &str) -> Result<SProgram> {
    parse_rust_with_array_result_model(source, "")
}

/// Read Rust under an explicit versioned scalar vector result admission model.
///
/// The empty model retains the original array result refusals. The fresh model
/// admits constructor tails of owned vectors of direct scalar Copy values.
///
/// # Errors
/// On unknown models, syntax errors and unsupported ownership or result proofs.
pub fn parse_rust_with_array_result_model(source: &str, model: &str) -> Result<SProgram> {
    parse_rust_with_source_models(source, model, "")
}

/// Read closed guarded returns under an explicitly selected source profile.
///
/// # Errors
/// Refuses unknown models and unsupported or escaping completions.
pub fn parse_rust_with_completion_model(source: &str, model: &str) -> Result<SProgram> {
    parse_rust_with_source_models(source, "", model)
}

/// Read Rust with independently selected array-result and completion profiles.
///
/// Empty strings retain each corresponding legacy source-admission contract.
///
/// # Errors
/// Refuses unknown models and propagates precise source-admission diagnostics.
pub fn parse_rust_with_source_models(
    source: &str,
    array_result_model: &str,
    completion_model: &str,
) -> Result<SProgram> {
    parse_rust_with_source_profiles(source, array_result_model, completion_model, "")
}

/// Capture actual structured source header facts under an explicit model.
///
/// # Errors
/// Refuses unknown source models and unsupported source declarations.
pub fn parse_rust_with_source_signature_model(source: &str, model: &str) -> Result<SProgram> {
    parse_rust_with_source_profiles(source, "", "", model)
}

/// Read independently selected array, completion and source signature models.
///
/// # Errors
/// Propagates model, syntax and source admission diagnostics.
pub fn parse_rust_with_source_profiles(
    source: &str,
    array_result_model: &str,
    completion_model: &str,
    source_signature_model: &str,
) -> Result<SProgram> {
    parse_rust_with_loop_profiles(
        source,
        array_result_model,
        completion_model,
        source_signature_model,
        "",
    )
}

/// Read an explicitly selected closed primitive scalar loop interpretation.
///
/// # Errors
/// Refuses unknown models, unproved owners and unsupported loop state or effects.
pub fn parse_rust_with_scalar_loop_model(source: &str, model: &str) -> Result<SProgram> {
    parse_rust_with_loop_profiles(source, "", "", "", model)
}

pub(crate) fn parse_rust_with_loop_profiles(
    source: &str,
    array_result_model: &str,
    completion_model: &str,
    source_signature_model: &str,
    scalar_loop_model: &str,
) -> Result<SProgram> {
    parse_rust_with_primitive_profiles(
        source,
        array_result_model,
        completion_model,
        source_signature_model,
        scalar_loop_model,
        "",
    )
}

/// Read the explicitly selected standard binary64 source owner interpretation.
///
/// # Errors
/// Refuses unproved qualified paths, aliases, ownership and primitive contexts.
pub fn parse_rust_with_primitive_owner_model(
    source: &str,
    model: &str,
    source_signature_model: &str,
) -> Result<SProgram> {
    parse_rust_with_primitive_profiles(source, "", "", source_signature_model, "", model)
}

fn parse_rust_with_primitive_profiles(
    source: &str,
    array_result_model: &str,
    completion_model: &str,
    source_signature_model: &str,
    scalar_loop_model: &str,
    primitive_owner_model: &str,
) -> Result<SProgram> {
    parse_rust_with_admitted_profiles(
        source,
        array_result_model,
        completion_model,
        source_signature_model,
        scalar_loop_model,
        primitive_owner_model,
        "",
    )
}

/// Read the explicit closed empty primitive option source interpretation.
///
/// # Errors
/// Refuses unknown models, nonstandard owners and bodies outside the empty profile.
pub fn parse_rust_with_empty_option_source_model(source: &str, model: &str) -> Result<SProgram> {
    parse_rust_with_admitted_profiles(source, "", "", "", "", "", model)
}

fn parse_rust_with_admitted_profiles(
    source: &str,
    array_result_model: &str,
    completion_model: &str,
    source_signature_model: &str,
    scalar_loop_model: &str,
    primitive_owner_model: &str,
    empty_option_source_model: &str,
) -> Result<SProgram> {
    parse_rust_with_selected_profiles(
        source,
        SourceProfiles {
            array_result: array_result_model,
            completion: completion_model,
            source_signature: source_signature_model,
            scalar_loop: scalar_loop_model,
            primitive_owner: primitive_owner_model,
            empty_option_source: empty_option_source_model,
            ..SourceProfiles::default()
        },
    )
}

/// Read Rust with actual source type-namespace ownership and ordered spellings.
///
/// # Errors
/// Refuses unknown selectors, unowned source bindings and unsupported positions.
pub fn parse_rust_with_type_namespace_model(
    source: &str,
    model: &str,
    source_signature_model: &str,
) -> Result<SProgram> {
    parse_rust_with_selected_profiles(
        source,
        SourceProfiles {
            type_namespace: model,
            source_signature: source_signature_model,
            ..SourceProfiles::default()
        },
    )
}

/// Read complete closed pure Unit programs under the independently selected model.
///
/// # Errors
/// Refuses unknown selectors, unsupported source context and unproved Unit calls.
pub fn parse_rust_with_pure_unit_call_model(source: &str, model: &str) -> Result<SProgram> {
    parse_rust_with_selected_profiles(
        source,
        SourceProfiles {
            pure_unit_call: model,
            ..SourceProfiles::default()
        },
    )
}
