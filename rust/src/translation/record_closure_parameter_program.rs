//! Complete closure contexts and original required-pattern source definitions.
use crate::{LinkNetwork, TypedProgramLinks, TypedProgramLinksError, decode_typed_program_links};
/// Read the complete editable source definition and its actual dependencies.
///
/// # Errors
/// Refuses malformed roots, graphs or static types.
pub fn create_java_script_record_closure_parameter_program_links()
-> Result<TypedProgramLinks, TypedProgramLinksError> {
    let network = LinkNetwork::from_lino(include_str!(
        "../data/record-closure-parameter-program.lino"
    ))
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
            message: "the record closure parameter program needs one actual schema root".to_owned(),
        });
    }
    let root = roots[0];
    decode_typed_program_links(&network, root)?;
    Ok(TypedProgramLinks { network, root })
}
