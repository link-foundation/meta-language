//! Existing independently selected Rust source factories.
//! Public reexports preserve the frontend API and shared parser implementation.
use super::{Result, SProgram, SourceProfiles, parse_rust_with_selected_profiles};
use std::rc::Rc;
use crate::translation::rust_standard_collection_carriers::CollectionSourceDeclarations;
use crate::translation::rust_standard_collection_library::RustStandardCollectionSourceLibrary;
use crate::translation::surface::SExpr;

/// Read concrete standard collections against the checked shared source library.
///
/// # Errors
/// Refuses unsupported model, version or target and unproved source operations.
pub fn parse_rust_with_standard_collection_source_model<Invalid, Resource>(
    source: &str,
    model: &str,
    source_version: &str,
    pointer_width: u32,
    library: Rc<RustStandardCollectionSourceLibrary<Invalid, Resource>>,
) -> Result<SProgram>
where
    Invalid: Fn(SExpr) -> SExpr + 'static,
    Resource: Fn(SExpr, SExpr) -> SExpr + 'static,
{
    use crate::translation::rust_standard_collection_value_packet::{
        ml_string::Text, read_rust_standard_collection_source_version_error,
    };
    let fail = |reason: &str| super::unsupported("Rust standard collection source", reason, None);
    if model != "Rust closed standard collections 1" {
        return Err(fail("select the exact closed standard collection source profile"));
    }
    if pointer_width != 64 {
        return Err(fail("the Rust source primitive usize/isize interface requires an explicit 64-bit target"));
    }
    let minor = super::source_versions::minor(source_version)?;
    let reason = read_rust_standard_collection_source_version_error(minor);
    if reason != Text::from_text("") {
        return Err(fail(&reason.to_string()));
    }
    let checked: Rc<dyn CollectionSourceDeclarations> = library;
    parse_rust_with_selected_profiles(
        source,
        SourceProfiles {
            source_version,
            standard_collections: Some(&checked),
            ..SourceProfiles::default()
        },
    )
}

/// Read actual standard shared-array handles under their independent source model.
///
/// # Errors
/// Refuses unknown selectors, unproved ownership and unsupported compositions.
pub fn parse_rust_with_shared_array_handle_source_model(
    source: &str,
    model: &str,
) -> Result<SProgram> {
    parse_rust_with_selected_profiles(
        source,
        SourceProfiles {
            shared_array_handle_source: model,
            ..SourceProfiles::default()
        },
    )
}

/// Read a complete emitted Rust runtime under its independent owned interpretation.
///
/// # Errors
/// Refuses unknown selectors and unproved runtime operations or source context.
pub fn parse_rust_with_runtime_shell_model(source: &str, model: &str) -> Result<SProgram> {
    crate::translation::runtime_shell::parse_source(source, model)
}

/// Reads a Rust program.
///
/// # Errors
/// On syntax errors and on constructs outside the portable core.
pub fn parse_rust(source: &str) -> Result<SProgram> {
    parse_rust_with_array_result_model(source, "")
}
/// Read ordinary Rust while retaining its actual type owners as linked facets.
///
/// # Errors
/// Refuses invalid source, unsupported constructs and unrepresentable owners.
pub fn parse_rust_with_source_type_owners(source: &str) -> Result<SProgram> {
    parse_rust_with_selected_profiles(
        source,
        SourceProfiles {
            retain_source_type_owners: true,
            ..SourceProfiles::default()
        },
    )
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

pub fn parse_rust_with_loop_profiles(
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

/// Read ordinary Rust under an explicitly selected stable source std version.
///
/// # Errors
/// Refuses noncanonical, prerelease and uncovered source versions or constructs.
pub fn parse_rust_with_source_version(source: &str, version: &str) -> Result<SProgram> {
    parse_rust_with_selected_profiles(
        source,
        SourceProfiles {
            source_version: version,
            ..SourceProfiles::default()
        },
    )
}
/// Select a source signature witness without discarding the active std version.
///
/// # Errors
/// Retains every version, model, ownership and source admission check.
pub fn parse_rust_with_source_version_and_signature_model(
    source: &str,
    version: &str,
    model: &str,
) -> Result<SProgram> {
    parse_rust_with_selected_profiles(source,SourceProfiles {
        source_version: version,
        source_signature: model,
        ..SourceProfiles::default()
    })
}

/// Retain source type owners under an explicit stable source std version.
///
/// # Errors
/// Refuses uncovered source versions, invalid owners and unsupported constructs.
pub fn parse_rust_with_source_type_owners_and_version(
    source: &str,
    version: &str,
) -> Result<SProgram> {
    parse_rust_with_selected_profiles(
        source,
        SourceProfiles {
            source_version: version,
            retain_source_type_owners: true,
            ..SourceProfiles::default()
        },
    )
}

/// Read shared handles using the selected source std snapshot.
///
/// # Errors
/// Refuses invalid selectors, ownership or unsupported source constructs.
pub fn parse_rust_with_shared_array_handle_source_model_and_version(
    source: &str,
    model: &str,
    version: &str,
) -> Result<SProgram> {
    parse_rust_with_selected_profiles(
        source,
        SourceProfiles {
            shared_array_handle_source: model,
            source_version: version,
            ..SourceProfiles::default()
        },
    )
}
