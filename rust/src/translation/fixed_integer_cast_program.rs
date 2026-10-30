//! Reconstruct the same complete editable modular integer source definition.
use crate::{LinkNetwork, TypedProgramLinks, TypedProgramLinksError, decode_typed_program_links};

/// Read actual typed operation definitions, including their recursive helpers.
///
/// # Errors
/// Returns source graph or ordinary whole-program static typing errors.
pub fn create_fixed_integer_cast_program_links() -> Result<TypedProgramLinks, TypedProgramLinksError>
{
    let network = LinkNetwork::from_lino(include_str!("../data/fixed-integer-cast-program.lino"))
        .map_err(|error| TypedProgramLinksError {
        message: error.to_string(),
    })?;
    let roots: Vec<_> = network
        .links()
        .filter(|link| {
            link.metadata().language() == Some("meta-language typed program")
                && link.metadata().term() == Some("typed program")
                && link.metadata().definition() == Some("1")
        })
        .map(crate::link_network::Link::id)
        .collect();
    if roots.len() != 1 {
        return Err(TypedProgramLinksError {
            message: "the fixed integer cast program needs one actual schema root".to_owned(),
        });
    }
    let root = roots[0];
    decode_typed_program_links(&network, root)?;
    Ok(TypedProgramLinks { network, root })
}
