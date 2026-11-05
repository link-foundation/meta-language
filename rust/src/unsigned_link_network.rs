//! Exact u64 identity transport around the shared source-defined envelope.
//! Metadata, UTF-8, term registrations and selected syntax use the unchanged
//! complete version-two codec after an explicit reversible coordinate change.
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::unsigned_link_identity_rules::{
    decode_unsigned_link_envelope, encode_unsigned_link_envelope,
};
use crate::{BinaryLinksOptions, Link, LinkId, LinkNetwork};

/// Invalid exact-ID graph, resource limit or noncanonical snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnsignedLinksError(String);
impl fmt::Display for UnsignedLinksError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "binary links snapshot: {}", self.0)
    }
}
impl Error for UnsignedLinksError {}
pub(crate) fn fail(message: impl ToString) -> UnsignedLinksError {
    UnsignedLinksError(message.to_string())
}
fn limits(options: BinaryLinksOptions) -> Result<(), UnsignedLinksError> {
    if options.byte_limit < 33
        || u64::try_from(options.byte_limit).map_err(fail)? > 9_007_199_254_740_991
        || options.record_limit == 0
        || u64::try_from(options.record_limit).map_err(fail)? > 9_007_199_254_740_991
    {
        return Err(fail("invalid unsigned link resource limits"));
    }
    Ok(())
}
fn words(id: LinkId) -> Result<[f64; 2], UnsignedLinksError> {
    if id.as_u64() == 0 {
        return Err(fail("zero is not an unsigned link identity"));
    }
    let low = u32::try_from(id.as_u64() & u64::from(u32::MAX)).map_err(fail)?;
    let high = u32::try_from(id.as_u64() >> 32).map_err(fail)?;
    Ok([f64::from(low), f64::from(high)])
}
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn identity(low: f64, high: f64) -> Result<LinkId, UnsignedLinksError> {
    if !crate::unsigned_link_identity_rules::accept_unsigned_link_identity(&[low, high]) {
        return Err(fail("invalid unsigned link identity limbs"));
    }
    Ok(LinkId::from_u64((low as u64) | ((high as u64) << 32)))
}
fn insert(graph: &mut LinkNetwork, id: LinkId, references: Vec<LinkId>, link: &Link) {
    graph.links.insert(
        id,
        Arc::new(Link {
            id,
            references: Arc::from(references),
            metadata: link.metadata().clone(),
        }),
    );
}

/// Temporary local coordinates and the complete inverse directory. This is
/// an ID transport, never a formal equivalence or global-concept merge proof.
#[derive(Clone, Debug)]
pub struct DenseUnsignedLinkNetwork {
    /// Ordinary graph algorithms use these explicit local coordinates.
    pub network: LinkNetwork,
    /// One original exact identity for each one-based local coordinate.
    pub directory: Vec<LinkId>,
    /// `None` is the distinct 2^64 exhausted cursor.
    pub next_identity: Option<LinkId>,
}

/// Construct a complete reversible dense view without rounding any identity.
/// # Errors
/// Rejects zero IDs, malformed allocators or exceeded resource bounds.
pub fn create_dense_unsigned_link_network(
    source: &LinkNetwork,
    options: BinaryLinksOptions,
) -> Result<DenseUnsignedLinkNetwork, UnsignedLinksError> {
    limits(options)?;
    if source.len() > options.record_limit {
        return Err(fail("record resource limit exceeded"));
    }
    let exhausted = u128::from(u64::MAX) + 1;
    if source.next_id == 0 || source.next_id > exhausted {
        return Err(fail("invalid unsigned link allocator"));
    }
    let mut names = BTreeSet::new();
    let mut remember = |id| -> Result<(), UnsignedLinksError> {
        words(id)?;
        names.insert(id);
        if names.len() > (options.byte_limit - 25) / 8 {
            return Err(fail("unsigned link directory resource limit exceeded"));
        }
        Ok(())
    };
    for link in source.links() {
        if u128::from(link.id().as_u64()) >= source.next_id {
            return Err(fail("unsigned allocator does not follow every stored link"));
        }
        remember(link.id())?;
        for id in link.references() {
            remember(*id)?;
        }
    }
    for id in source.terms.values() {
        remember(*id)?;
    }
    let directory: Vec<_> = names.into_iter().collect();
    let indexes: BTreeMap<_, _> = directory
        .iter()
        .enumerate()
        .map(|(index, id)| {
            u64::try_from(index + 1)
                .map(|local| (*id, LinkId::from_u64(local)))
                .map_err(fail)
        })
        .collect::<Result<_, _>>()?;
    let mut network = LinkNetwork::new();
    for link in source.links() {
        insert(
            &mut network,
            indexes[&link.id()],
            link.references().iter().map(|id| indexes[id]).collect(),
            link,
        );
    }
    network.next_id = u128::try_from(directory.len()).map_err(fail)? + 1;
    network.terms = source
        .terms
        .iter()
        .map(|(term, id)| (Arc::clone(term), indexes[id]))
        .collect();
    network.concept_syntax = source.concept_syntax.clone();
    network.strings = source.strings.clone();
    let next_identity = if source.next_id == exhausted {
        None
    } else {
        Some(LinkId::from_u64(
            u64::try_from(source.next_id).map_err(fail)?,
        ))
    };
    Ok(DenseUnsignedLinkNetwork {
        network,
        directory,
        next_identity,
    })
}

/// Restore the actual global identities of a dense coordinate view.
/// # Errors
/// Rejects unused, duplicate, unordered or foreign coordinates and bad cursors.
pub fn restore_dense_unsigned_link_network(
    view: &DenseUnsignedLinkNetwork,
    options: BinaryLinksOptions,
) -> Result<LinkNetwork, UnsignedLinksError> {
    limits(options)?;
    if view.network.len() > options.record_limit
        || view.directory.len() > (options.byte_limit - 25) / 8
    {
        return Err(fail("invalid dense unsigned graph"));
    }
    for id in &view.directory {
        words(*id)?;
    }
    if view.directory.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(fail("unsigned link directory must be strictly ordered"));
    }
    if let Some(id) = view.next_identity {
        words(id)?;
    }
    let mut used = BTreeSet::new();
    let mut global = |id: LinkId| -> Result<LinkId, UnsignedLinksError> {
        let index = usize::try_from(id.as_u64()).map_err(fail)?;
        let value = index
            .checked_sub(1)
            .and_then(|index| view.directory.get(index))
            .copied()
            .ok_or_else(|| fail("foreign dense unsigned coordinate"))?;
        used.insert(index);
        Ok(value)
    };
    let mut graph = LinkNetwork::new();
    for link in view.network.links() {
        let id = global(link.id())?;
        if view.next_identity.is_some_and(|cursor| id >= cursor) {
            return Err(fail("unsigned allocator does not follow every stored link"));
        }
        let references = link
            .references()
            .iter()
            .map(|id| global(*id))
            .collect::<Result<Vec<_>, _>>()?;
        insert(&mut graph, id, references, link);
    }
    graph.terms = view
        .network
        .terms
        .iter()
        .map(|(term, id)| global(*id).map(|global| (Arc::clone(term), global)))
        .collect::<Result<_, _>>()?;
    if used.len() != view.directory.len() {
        return Err(fail("unused unsigned link directory entry"));
    }
    graph.concept_syntax = view.network.concept_syntax.clone();
    graph.strings = view.network.strings.clone();
    graph.next_id = view
        .next_identity
        .map_or(u128::from(u64::MAX) + 1, |id| u128::from(id.as_u64()));
    Ok(graph)
}

/// An explicit fresh coordinate assignment and its complete inverse mapping.
/// The source network is unchanged; this supplies no concept equivalence proof.
#[derive(Clone, Debug)]
pub struct RebasedUnsignedLinkNetwork {
    /// Source, syntax, concepts, metadata and registrations under exact new IDs.
    pub network: LinkNetwork,
    /// Original ID at each local directory position, including dangling refs.
    pub source_identities: Vec<LinkId>,
    /// New ID at the same local directory position.
    pub identities: Vec<LinkId>,
}

/// Allocate fresh exact coordinates while retaining complete graph contents.
/// # Errors
/// Refuses zero IDs, exceeded graph/directory bounds and unsigned exhaustion.
pub fn rebase_unsigned_link_network(
    source: &LinkNetwork,
    first_identity: LinkId,
    options: BinaryLinksOptions,
) -> Result<RebasedUnsignedLinkNetwork, UnsignedLinksError> {
    let view = create_dense_unsigned_link_network(source, options)?;
    let first = words(first_identity)?;
    let count = u32::try_from(view.directory.len()).map_or(f64::INFINITY, f64::from);
    let limit = u32::try_from(options.record_limit).map_or(9_007_199_254_740_991.0, f64::from);
    let reason = crate::unsigned_source_document_rules::read_unsigned_source_allocation_error(
        &first, count, limit,
    );
    if !reason.is_empty() {
        return Err(fail(reason));
    }
    let plan = crate::unsigned_source_document_rules::allocate_unsigned_source_identities(
        &first, count, limit,
    );
    if plan.len() != 3 + 2 * view.directory.len() {
        return Err(fail("source graph allocation lost its complete directory"));
    }
    let identities = plan[3..]
        .chunks_exact(2)
        .map(|pair| identity(pair[0], pair[1]))
        .collect::<Result<Vec<_>, _>>()?;
    let next_identity = if plan[1] < 0.0 {
        None
    } else {
        Some(identity(plan[1], plan[2])?)
    };
    let network = restore_dense_unsigned_link_network(
        &DenseUnsignedLinkNetwork {
            network: view.network,
            directory: identities.clone(),
            next_identity,
        },
        options,
    )?;
    Ok(RebasedUnsignedLinkNetwork {
        network,
        source_identities: view.directory,
        identities,
    })
}

/// Parse an ordinary concrete source document into a fresh exact-ID graph.
/// Optional AST/semantic selection uses the complete resulting source network.
/// # Errors
/// Returns source graph allocation and graph/directory resource errors.
pub fn create_unsigned_source_document_links(
    text: &str,
    language: &str,
    configuration: crate::ParseConfiguration,
    first_identity: LinkId,
    options: BinaryLinksOptions,
) -> Result<RebasedUnsignedLinkNetwork, UnsignedLinksError> {
    let source = LinkNetwork::parse(text, language, configuration);
    rebase_unsigned_link_network(&source, first_identity, options)
}

/// Preserve registered parser dispatch before assigning exact source IDs.
/// # Errors
/// Returns source graph allocation and graph/directory resource errors.
pub fn create_unsigned_source_document_links_with_registry(
    registry: &crate::ParserRegistry,
    text: &str,
    language: &str,
    configuration: crate::ParseConfiguration,
    first_identity: LinkId,
    options: BinaryLinksOptions,
) -> Result<RebasedUnsignedLinkNetwork, UnsignedLinksError> {
    let source = registry.parse(text, language, configuration);
    rebase_unsigned_link_network(&source, first_identity, options)
}

/// Encode all u64 identities, original allocator and complete network state.
/// # Errors
/// Returns malformed graph, metadata, cache or resource errors.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn encode_unsigned_link_network_binary(
    graph: &LinkNetwork,
    options: BinaryLinksOptions,
) -> Result<Vec<u8>, UnsignedLinksError> {
    let view = create_dense_unsigned_link_network(graph, options)?;
    let body = crate::encode_link_network_binary(&view.network, options).map_err(fail)?;
    let directory = view
        .directory
        .iter()
        .map(|id| words(*id))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    let next = view
        .next_identity
        .map(words)
        .transpose()?
        .map_or_else(Vec::new, |words| words.to_vec());
    let body: Vec<_> = body.into_iter().map(f64::from).collect();
    let encoded =
        encode_unsigned_link_envelope(&directory, &next, &body, options.byte_limit as f64);
    if encoded.first().is_some_and(|code| *code < 0.0) {
        return Err(fail(
            crate::unsigned_link_identity_rules::read_unsigned_link_envelope_error(encoded[0]),
        ));
    }
    Ok(encoded.into_iter().map(|byte| byte as u8).collect())
}

/// Decode an exact-ID envelope; never infer missing coordinates or drop bytes.
/// # Errors
/// Returns noncanonical framing, graph, allocator, metadata or cache errors.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn decode_unsigned_link_network_binary(
    bytes: &[u8],
    options: BinaryLinksOptions,
) -> Result<LinkNetwork, UnsignedLinksError> {
    limits(options)?;
    if bytes.len() > options.byte_limit {
        return Err(fail("unsigned link envelope resource limit"));
    }
    let input: Vec<_> = bytes.iter().copied().map(f64::from).collect();
    let decoded = decode_unsigned_link_envelope(&input, options.byte_limit as f64);
    if decoded.first().is_some_and(|code| *code < 0.0) {
        return Err(fail(
            crate::unsigned_link_identity_rules::read_unsigned_link_envelope_error(decoded[0]),
        ));
    }
    if decoded.len() < 4 {
        return Err(fail("missing unsigned link envelope fields"));
    }
    let count = decoded[3] as usize;
    let boundary = count
        .checked_mul(2)
        .and_then(|size| size.checked_add(4))
        .ok_or_else(|| fail("unsigned link directory resource limit exceeded"))?;
    if boundary > decoded.len() {
        return Err(fail("truncated unsigned link directory"));
    }
    let directory = decoded[4..boundary]
        .chunks_exact(2)
        .map(|pair| identity(pair[0], pair[1]))
        .collect::<Result<Vec<_>, _>>()?;
    let next_identity = if decoded[0] == 1.0 {
        None
    } else {
        Some(identity(decoded[1], decoded[2])?)
    };
    let body: Vec<_> = decoded[boundary..].iter().map(|byte| *byte as u8).collect();
    if body.get(7) != Some(&50) {
        return Err(fail(
            "the unsigned link envelope requires a complete version-two inner network",
        ));
    }
    let network = crate::decode_link_network_binary(&body, options).map_err(fail)?;
    if network.next_id != u128::try_from(count).map_err(fail)? + 1 {
        return Err(fail("noncanonical dense unsigned allocator"));
    }
    restore_dense_unsigned_link_network(
        &DenseUnsignedLinkNetwork {
            network,
            directory,
            next_identity,
        },
        options,
    )
}
