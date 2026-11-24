use super::{MetaLanguageArchiveEntry, MetaLanguageArchiveLimits};
use crate::meta_language_archive_rules::{
    accept_meta_language_archive_kind, accept_meta_language_archive_next_identity,
    read_meta_language_archive_language, read_meta_language_archive_record_fields,
    read_meta_language_archive_term,
};
use crate::structured_data_links::StructuredDataValue;
use crate::{Link, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub(super) fn fail(message: &str) -> String {
    format!("meta-language archive: {message}")
}
pub(super) fn string(value: &str) -> StructuredDataValue {
    StructuredDataValue::String(value.encode_utf16().collect())
}
#[allow(clippy::cast_precision_loss)]
pub(super) fn number(value: u64) -> StructuredDataValue {
    StructuredDataValue::Number(value as f64)
}
pub(super) fn array(values: Vec<StructuredDataValue>) -> StructuredDataValue {
    StructuredDataValue::Array(values)
}
pub(super) fn object(values: Vec<(&str, StructuredDataValue)>) -> StructuredDataValue {
    StructuredDataValue::Object(
        values
            .into_iter()
            .map(|(name, value)| (name.encode_utf16().collect(), value))
            .collect(),
    )
}
pub(super) fn text(value: &Value) -> Result<&str, String> {
    value.as_str().ok_or_else(|| fail("expected linked text"))
}

pub(super) fn record(value: &Value, role: &str) -> Result<(), String> {
    let expected = read_meta_language_archive_record_fields(role);
    let fields: Vec<_> = expected.split('|').collect();
    let value = value
        .as_object()
        .ok_or_else(|| fail("invalid archive record shape"))?;
    if expected.is_empty()
        || value.len() != fields.len()
        || fields.iter().any(|field| !value.contains_key(*field))
    {
        return Err(fail("invalid archive record shape"));
    }
    Ok(())
}
pub(super) fn list(value: &Value) -> Result<&[Value], String> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| fail("expected linked sequence"))
}
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub(super) fn identity(value: &Value) -> Result<LinkId, String> {
    let value = value
        .as_f64()
        .ok_or_else(|| fail("invalid graph identity"))?;
    if !accept_meta_language_archive_next_identity(value, 0.0) {
        return Err(fail("invalid graph identity"));
    }
    Ok(LinkId::from_u64(value as u64))
}
pub(super) fn validate_limits(limits: MetaLanguageArchiveLimits) -> Result<(), String> {
    #[allow(clippy::cast_precision_loss)]
    let valid = [limits.node_limit, limits.reference_limit, limits.byte_limit]
        .into_iter()
        .all(|value| accept_meta_language_archive_next_identity(value as f64, 0.0));
    if valid {
        Ok(())
    } else {
        Err(fail("invalid resource limit"))
    }
}

pub(super) fn metadata(role: &str) -> LinkMetadata {
    LinkMetadata::new()
        .with_link_type(LinkType::Semantic)
        .with_named(true)
        .with_language(read_meta_language_archive_language())
        .with_term(read_meta_language_archive_term(role))
}
pub(super) fn node(network: &mut LinkNetwork, references: &[LinkId], role: &str) -> LinkId {
    let id = network.insert_link([], metadata(role));
    network.set_references(id, references);
    id
}
pub(super) fn references<'a>(
    network: &'a LinkNetwork,
    id: LinkId,
    role: &str,
) -> Result<&'a [LinkId], String> {
    let link = network
        .link(id)
        .ok_or_else(|| fail("missing archive role"))?;
    let data = link.metadata();
    if data.link_type() != Some(LinkType::Semantic)
        || !data.is_named()
        || data.language() != Some(read_meta_language_archive_language().as_str())
        || data.term() != Some(read_meta_language_archive_term(role).as_str())
        || data.definition().is_some()
        || data.span().is_some()
        || data.flags() != LinkFlags::clean()
    {
        return Err(fail("invalid archive role"));
    }
    Ok(link.references())
}

#[allow(clippy::cast_precision_loss)]
pub(super) fn inspect(
    network: &LinkNetwork,
    limits: MetaLanguageArchiveLimits,
) -> Result<(), String> {
    validate_limits(limits)?;
    if network.len() > limits.node_limit {
        return Err(fail("node resource limit exceeded"));
    }
    let mut count = 0_usize;
    let mut bytes = 0_usize;
    let mut maximum = 0_u64;
    for link in network.links() {
        let id = link.id().as_u64();
        if id == 0 || id > 9_007_199_254_740_991 {
            return Err(fail("invalid graph identity"));
        }
        maximum = maximum.max(id);
        count = count.saturating_add(link.references().len());
        for reference in link.references() {
            if network.link(*reference).is_none() {
                return Err(fail("dangling graph reference"));
            }
        }
        let data = link.metadata();
        for value in [data.term(), data.language(), data.definition()]
            .into_iter()
            .flatten()
        {
            bytes = bytes.saturating_add(value.len());
        }
        if let Some(span) = data.span() {
            let range = span.byte_range();
            let first = span.start_point();
            let last = span.end_point();
            if [
                range.start(),
                range.end(),
                first.row(),
                first.column(),
                last.row(),
                last.column(),
            ]
            .into_iter()
            .any(|value| value as u128 > 9_007_199_254_740_991)
            {
                return Err(fail("invalid graph source span"));
            }
        }
    }
    if !accept_meta_language_archive_next_identity(network.next_id as f64, maximum as f64) {
        return Err(fail("invalid allocator state"));
    }
    for (term, id) in &network.terms {
        if network.link(*id).is_none() {
            return Err(fail("invalid term registration"));
        }
        bytes = bytes.saturating_add(term.len());
    }
    for ((concept, language), value) in &network.concept_syntax {
        if concept.contains('\0') || language.contains('\0') {
            return Err(fail("ambiguous selected syntax key"));
        }
        bytes = bytes
            .saturating_add(concept.len())
            .saturating_add(language.len())
            .saturating_add(1)
            .saturating_add(value.len());
    }
    if count > limits.reference_limit || bytes > limits.byte_limit {
        return Err(fail("graph resource limit exceeded"));
    }
    Ok(())
}

pub(super) fn state(network: &LinkNetwork) -> StructuredDataValue {
    let identities = network
        .links()
        .map(|link| number(link.id().as_u64()))
        .collect();
    let terms = network
        .terms
        .iter()
        .map(|(term, id)| {
            object(vec![
                ("term", string(term)),
                ("identity", number(id.as_u64())),
            ])
        })
        .collect();
    let syntax = network
        .concept_syntax
        .iter()
        .map(|((concept, language), value)| {
            object(vec![
                ("concept", string(concept)),
                ("language", string(language)),
                ("value", string(value)),
            ])
        })
        .collect();
    object(vec![
        ("identities", array(identities)),
        (
            "nextIdentity",
            number(
                u64::try_from(network.next_id)
                    .expect("an inspected archive allocator fits the shared Number profile"),
            ),
        ),
        ("terms", array(terms)),
        ("syntax", array(syntax)),
    ])
}
pub(super) fn copy(
    target: &mut LinkNetwork,
    source: &LinkNetwork,
    limits: MetaLanguageArchiveLimits,
) -> Result<BTreeMap<LinkId, LinkId>, String> {
    inspect(source, limits)?;
    if target.len().saturating_add(source.len()) > limits.node_limit {
        return Err(fail("node resource limit exceeded"));
    }
    let mapping: BTreeMap<_, _> = source
        .links()
        .map(|link| (link.id(), target.insert_link([], link.metadata().clone())))
        .collect();
    for link in source.links() {
        let references: Vec<_> = link.references().iter().map(|id| mapping[id]).collect();
        target.set_references(mapping[&link.id()], &references);
    }
    Ok(mapping)
}
pub(super) fn validate_entry(entry: &MetaLanguageArchiveEntry) -> Result<(), String> {
    if entry.identifier.is_empty()
        || !accept_meta_language_archive_kind(&entry.kind)
        || entry.network.link(entry.root).is_none()
    {
        return Err(fail("invalid entry identifier, kind or root"));
    }
    Ok(())
}
pub(super) fn island(
    network: &LinkNetwork,
    root: LinkId,
    limit: usize,
) -> Result<LinkNetwork, String> {
    let mut members = BTreeSet::new();
    let mut pending = vec![root];
    while let Some(id) = pending.pop() {
        if !members.insert(id) {
            continue;
        }
        if members.len() > limit {
            return Err(fail("descriptor resource limit exceeded"));
        }
        let link = network
            .link(id)
            .ok_or_else(|| fail("dangling descriptor reference"))?;
        pending.extend_from_slice(link.references());
    }
    let mut result = LinkNetwork::new();
    for id in members {
        let link = network
            .link(id)
            .ok_or_else(|| fail("missing descriptor member"))?;
        result.observe_identifier(id);
        result.links.insert(
            id,
            Arc::new(Link {
                id,
                references: Arc::from(link.references()),
                metadata: link.metadata().clone(),
            }),
        );
    }
    Ok(result)
}
