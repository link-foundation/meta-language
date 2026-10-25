//! Full source definitions for ordinary record callable parameter contexts.
use crate::{LinkNetwork, TypedProgramLinks, TypedProgramLinksError, decode_typed_program_links};
/// Load the complete inspectable actual source and original pattern closure.
/// # Errors
/// Refuses malformed roots, graphs or static types.
pub fn create_java_script_record_callable_parameter_program_links()
-> Result<TypedProgramLinks, TypedProgramLinksError> {
    let network = LinkNetwork::from_lino(include_str!(
        "../data/record-callable-parameter-program.lino"
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
            message: "the record callable parameter program needs one actual schema root"
                .to_owned(),
        });
    }
    let root = roots[0];
    decode_typed_program_links(&network, root)?;
    Ok(TypedProgramLinks { network, root })
}
