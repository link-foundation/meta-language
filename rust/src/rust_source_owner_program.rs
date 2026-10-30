//! Source/checker/Links transport; owner decisions come from shared JS rules.
use crate::{TypedProgramLinks, TypedProgramLinksError, encode_typed_program_links};
/// Retain actual ordinary Rust source owners in the complete typed program.
///
/// # Errors
/// Refuses invalid source, unsupported type owners and malformed typed graphs.
pub fn create_rust_source_owner_program_links(
    source: &str,
) -> Result<TypedProgramLinks, TypedProgramLinksError> {
    let surface =
        crate::translation::rust::parse_rust_with_source_type_owners(source).map_err(|error| {
            TypedProgramLinksError {
                message: error.to_string(),
            }
        })?;
    let program = crate::translation::check::check_program(&surface).map_err(|error| {
        TypedProgramLinksError {
            message: error.to_string(),
        }
    })?;
    encode_typed_program_links(&program)
}

/// Retain source owners and requested std version in structured Links provenance.
///
/// # Errors
/// Refuses unsupported source versions, owners or malformed typed graphs.
pub fn create_rust_source_owner_program_links_with_source_version(
    source: &str,
    version: &str,
) -> Result<TypedProgramLinks, TypedProgramLinksError> {
    let surface =
        crate::translation::rust::parse_rust_with_source_type_owners_and_version(source, version)
            .map_err(|error| TypedProgramLinksError {
            message: error.to_string(),
        })?;
    let program = crate::translation::check::check_program(&surface).map_err(|error| {
        TypedProgramLinksError {
            message: error.to_string(),
        }
    })?;
    let provenance = serde_json::json!({"sourceProfile":{"language":"Rust","rustSourceVersion":crate::translation::rust::source_versions::canonical(version).map_err(|error|TypedProgramLinksError{message:error.to_string()})?},"userProvenance":null});
    crate::typed_program_links::encode_typed_program_links_with_provenance(
        &program,
        Some(&provenance),
    )
}
