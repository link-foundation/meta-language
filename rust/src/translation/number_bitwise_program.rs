//! The same complete checked Number definition, transported as Links data.
use crate::{
    LinkId, LinkNetwork, TypedProgramLinks, TypedProgramLinksError, decode_typed_program_links,
};

/// Reconstruct a fresh editable Number definition graph and validate it.
///
/// # Errors
/// Returns any malformed-data or checked-program diagnostic.
pub fn create_number_bitwise_program_links() -> Result<TypedProgramLinks, TypedProgramLinksError> {
    let network =
        LinkNetwork::from_lino(super::number_bitwise_program_data::LINKS).map_err(|error| {
            TypedProgramLinksError {
                message: error.to_string(),
            }
        })?;
    let root = LinkId::from_u64(super::number_bitwise_program_data::ROOT);
    decode_typed_program_links(&network, root)?;
    Ok(TypedProgramLinks { network, root })
}
