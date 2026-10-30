//! Full editable typed definitions of exact JavaScript numeric conversion decisions.
use crate::{LinkNetwork, TypedProgramLinks, TypedProgramLinksError, decode_typed_program_links};

/// Read the actual complete typed source graph.
///
/// # Errors
/// Returns malformed source graph or static typing errors.
pub fn create_string_number_program_links() -> Result<TypedProgramLinks, TypedProgramLinksError> {
    let network = LinkNetwork::from_lino(include_str!("../data/string-number-program.lino"))
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
            message: "the JavaScript numeric conversion program needs one actual schema root"
                .to_owned(),
        });
    }
    let root = roots[0];
    decode_typed_program_links(&network, root)?;
    Ok(TypedProgramLinks { network, root })
}
