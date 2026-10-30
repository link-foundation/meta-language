//! Complete editable source identifier namespace definition.
use crate::{LinkNetwork, TypedProgramLinks, TypedProgramLinksError, decode_typed_program_links};

/// Read and statically recheck the complete source-derived definition.
///
/// # Errors
/// Returns malformed source graph or static typing errors.
pub fn create_source_identifier_program_links() -> Result<TypedProgramLinks, TypedProgramLinksError>
{
    let network = LinkNetwork::from_lino(include_str!("../data/source-identifier-program.lino"))
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
            message: "the source identifier program needs one actual schema root".to_owned(),
        });
    }
    let root = roots[0];
    decode_typed_program_links(&network, root)?;
    Ok(TypedProgramLinks { network, root })
}
