//! Complete editable source definition of command admission.
use crate::{LinkNetwork, TypedProgramLinks, TypedProgramLinksError, decode_typed_program_links};

/// Read and statically recheck the complete source-derived command vocabulary.
///
/// # Errors
/// Refuses malformed roots, graphs or static types.
pub fn create_dynamic_language_command_program_links()
-> Result<TypedProgramLinks, TypedProgramLinksError> {
    let network =
        LinkNetwork::from_lino(include_str!("data/dynamic-language-command-program.lino"))
            .map_err(|error| TypedProgramLinksError {
                message: error.to_string(),
            })?;
    let roots = network
        .links()
        .filter(|link| {
            link.metadata().language() == Some("meta-language typed program")
                && link.metadata().term() == Some("typed program")
                && link.metadata().definition() == Some("1")
        })
        .map(crate::Link::id)
        .collect::<Vec<_>>();
    if roots.len() != 1 {
        return Err(TypedProgramLinksError {
            message: "the dynamic language command program needs one actual schema root".to_owned(),
        });
    }
    let root = roots[0];
    decode_typed_program_links(&network, root)?;
    Ok(TypedProgramLinks { network, root })
}
