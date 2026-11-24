//! Full actual formal syntax/context/inference and original iterable definitions.
use crate::{TypedProgramLinks, TypedProgramLinksError, create_source_definition_program_links};
/// Load the complete source definition through the authenticated common catalog.
/// # Errors
/// Returns catalog, source graph and static type diagnostics.
pub fn create_java_script_array_formal_parameter_program_links()
-> Result<TypedProgramLinks, TypedProgramLinksError> {
    create_source_definition_program_links(
        "meta-language/source-program/array-formal-parameter-program",
    )
    .map_err(|message| TypedProgramLinksError { message })
}
