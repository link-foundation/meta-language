//! Exact-ID transport into the existing checked source/Links/emission pipeline.
use crate::{
    BinaryLinksOptions, LinkId, LinkNetwork, TypedProgramLinks, TypedProgramLinksEmission,
    TypedProgramLinksEmissionOptions, TypedProgramLinksError, create_dense_unsigned_link_network,
    rebase_unsigned_link_network,
};
type Result<T> = std::result::Result<T, TypedProgramLinksError>;
fn error(message: impl ToString) -> TypedProgramLinksError {
    TypedProgramLinksError {
        message: message.to_string(),
    }
}
/// A complete exact-ID semantic graph with its root and inverse directory.
#[derive(Clone, Debug)]
pub struct RebasedUnsignedTypedProgramLinks {
    /// The complete current semantic graph, available for direct editing.
    pub network: LinkNetwork,
    /// Actual program root under the newly allocated coordinates.
    pub root: LinkId,
    /// Original identity for each corresponding allocated identity.
    pub source_identities: Vec<LinkId>,
    /// Exact allocated identities in original directory order.
    pub identities: Vec<LinkId>,
}
/// Retain the semantic root when assigning a separate fresh coordinate space.
/// # Errors
/// Refuses missing roots, malformed graphs and exceeded allocation bounds.
pub fn rebase_unsigned_typed_program_links(
    packet: &TypedProgramLinks,
    first_identity: LinkId,
    limits: BinaryLinksOptions,
) -> Result<RebasedUnsignedTypedProgramLinks> {
    let rebased =
        rebase_unsigned_link_network(&packet.network, first_identity, limits).map_err(error)?;
    let index = rebased
        .source_identities
        .binary_search(&packet.root)
        .map_err(|_| error("an unsigned typed program needs its actual stored source root"))?;
    if packet.network.link(packet.root).is_none() {
        return Err(error(
            "an unsigned typed program needs its actual stored source root",
        ));
    }
    Ok(RebasedUnsignedTypedProgramLinks {
        root: rebased.identities[index],
        network: rebased.network,
        source_identities: rebased.source_identities,
        identities: rebased.identities,
    })
}
/// Use the ordinary language parser/checker before exact coordinate allocation.
/// # Errors
/// Retains all ordinary source/checker refusals and allocation resource errors.
pub fn create_unsigned_typed_program_links(
    source: &str,
    language: &str,
    first_identity: LinkId,
    limits: BinaryLinksOptions,
) -> Result<RebasedUnsignedTypedProgramLinks> {
    let packet = crate::create_typed_program_links(source, language)?;
    rebase_unsigned_typed_program_links(&packet, first_identity, limits)
}
/// Preserve complete actual ordinary Rust type owners in the exact-ID graph.
/// # Errors
/// Retains source-owner, type-checking and exact allocation refusals.
pub fn create_unsigned_rust_source_owner_program_links(
    source: &str,
    first_identity: LinkId,
    limits: BinaryLinksOptions,
) -> Result<RebasedUnsignedTypedProgramLinks> {
    let packet = crate::create_rust_source_owner_program_links(source)?;
    rebase_unsigned_typed_program_links(&packet, first_identity, limits)
}
/// Preserve the requested Rust source version along with original type owners.
/// # Errors
/// Retains version, source-owner, type-checking and exact allocation refusals.
pub fn create_unsigned_rust_source_owner_program_links_with_source_version(
    source: &str,
    version: &str,
    first_identity: LinkId,
    limits: BinaryLinksOptions,
) -> Result<RebasedUnsignedTypedProgramLinks> {
    let packet =
        crate::create_rust_source_owner_program_links_with_source_version(source, version)?;
    rebase_unsigned_typed_program_links(&packet, first_identity, limits)
}
/// Emit from current graph semantics after the existing complete static checks.
/// # Errors
/// Refuses missing roots, invalid semantic graphs, limits and target obligations.
pub fn emit_unsigned_typed_program_links(
    network: &LinkNetwork,
    root: LinkId,
    target: &str,
    limits: BinaryLinksOptions,
) -> Result<TypedProgramLinksEmission> {
    emit_unsigned_typed_program_links_with_options(
        network,
        root,
        target,
        &TypedProgramLinksEmissionOptions::default(),
        limits,
    )
}
/// Pass explicit emission choices through the existing semantic emitter.
/// # Errors
/// Retains exact graph, scope, typing, source-model and target refusals.
pub fn emit_unsigned_typed_program_links_with_options(
    network: &LinkNetwork,
    root: LinkId,
    target: &str,
    options: &TypedProgramLinksEmissionOptions,
    limits: BinaryLinksOptions,
) -> Result<TypedProgramLinksEmission> {
    let view = create_dense_unsigned_link_network(network, limits).map_err(error)?;
    let index = view
        .directory
        .binary_search(&root)
        .map_err(|_| error("an unsigned typed program needs its actual stored source root"))?;
    if network.link(root).is_none() {
        return Err(error(
            "an unsigned typed program needs its actual stored source root",
        ));
    }
    let local = u64::try_from(index)
        .ok()
        .and_then(|value| value.checked_add(1))
        .ok_or_else(|| {
            error("an unsigned typed program root exceeds its local coordinate range")
        })?;
    crate::emit_typed_program_links_with_options(
        &view.network,
        LinkId::from_u64(local),
        target,
        options,
    )
}
