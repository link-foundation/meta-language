use super::model::record as check_record;
use super::model::{
    fail, identity, inspect, island, list, references, text, validate_entry, validate_limits,
};
use super::{
    LoadedMetaLanguageArchive, MetaLanguageArchive, MetaLanguageArchiveEntry,
    MetaLanguageArchiveLimits, ReconstructedMetaLanguageArchive,
};
use crate::meta_language_archive_rules::{
    accept_meta_language_archive_entry_group, accept_meta_language_archive_flags,
    accept_meta_language_archive_version, read_meta_language_archive_language,
    read_meta_language_archive_term,
};
use crate::structured_data_links::{
    StructuredDataLinksLimits, decode_structured_data_links_with_limits,
};
use crate::{BinaryLinksOptions, Link, LinkId, LinkNetwork, decode_link_network_binary};
use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

fn restore(
    network: &LinkNetwork,
    inventory: &[LinkId],
    state: &Value,
    claimed: &mut BTreeSet<LinkId>,
    limits: MetaLanguageArchiveLimits,
) -> Result<Arc<LinkNetwork>, String> {
    check_record(state, "group")?;
    let originals = list(&state["identities"])?;
    if originals.len() != inventory.len() {
        return Err(fail("invalid source network descriptor"));
    }
    let mut mapping = BTreeMap::new();
    let mut seen = BTreeSet::new();
    for (id, value) in inventory.iter().zip(originals) {
        let original = identity(value)?;
        if !claimed.insert(*id) || mapping.insert(*id, original).is_some() || !seen.insert(original)
        {
            return Err(fail("overlapping or duplicate graph inventory"));
        }
    }
    let mut result = LinkNetwork::new();
    for id in inventory {
        let link = network
            .link(*id)
            .ok_or_else(|| fail("missing source network member"))?;
        let targets = link
            .references()
            .iter()
            .map(|target| {
                mapping
                    .get(target)
                    .copied()
                    .ok_or_else(|| fail("foreign or dangling source network reference"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let original = mapping[id];
        result.links.insert(
            original,
            Arc::new(Link {
                id: original,
                references: Arc::from(targets),
                metadata: link.metadata().clone(),
            }),
        );
    }
    for record in list(&state["terms"])? {
        check_record(record, "term")?;
        let key = text(&record["term"])?;
        let id = identity(&record["identity"])?;
        if !seen.contains(&id) || result.terms.insert(Arc::from(key), id).is_some() {
            return Err(fail("invalid or duplicate term registration"));
        }
    }
    for record in list(&state["syntax"])? {
        check_record(record, "syntax")?;
        let concept = text(&record["concept"])?;
        let language = text(&record["language"])?;
        let value = text(&record["value"])?;
        if concept.contains('\0')
            || language.contains('\0')
            || result
                .concept_syntax
                .insert((Arc::from(concept), Arc::from(language)), Arc::from(value))
                .is_some()
        {
            return Err(fail("invalid or duplicate selected syntax"));
        }
    }
    result.next_id = identity(&state["nextIdentity"])?.as_u64();
    inspect(&result, limits)?;
    Ok(Arc::new(result))
}

/// Eagerly reconstruct all source groups and stable entry selections.
///
/// # Errors
/// Refuses foreign inventories, stale identities and malformed descriptors.
pub fn read_meta_language_archive(
    network: &LinkNetwork,
    root: LinkId,
    limits: MetaLanguageArchiveLimits,
) -> Result<ReconstructedMetaLanguageArchive, String> {
    inspect(network, limits)?;
    let roots = references(network, root, "root")?;
    if roots.len() < 2 {
        return Err(fail("missing descriptor or source groups"));
    }
    let mut claimed: BTreeSet<_> = std::iter::once(root).chain(roots.iter().copied()).collect();
    if claimed.len() != roots.len() + 1 {
        return Err(fail("overlapping archive roots"));
    }
    let descriptor_network = island(network, roots[0], limits.node_limit)?;
    let descriptor_ids: BTreeSet<_> = descriptor_network.links().map(Link::id).collect();
    if roots[1..].iter().any(|id| descriptor_ids.contains(id)) {
        return Err(fail("descriptor aliases a source group"));
    }
    claimed.extend(descriptor_ids);
    let descriptor = decode_structured_data_links_with_limits(
        &descriptor_network,
        roots[0],
        StructuredDataLinksLimits {
            node_limit: limits.node_limit,
            depth_limit: 256,
            byte_limit: limits.byte_limit,
        },
    )?;
    check_record(&descriptor, "descriptor")?;
    if !accept_meta_language_archive_version(descriptor["schemaVersion"].as_f64().unwrap_or(-1.0)) {
        return Err(fail("invalid archive descriptor"));
    }
    let states = list(&descriptor["groups"])?;
    let records = list(&descriptor["entries"])?;
    if states.len() != roots.len() - 1 || records.is_empty() {
        return Err(fail("invalid archive descriptor"));
    }
    let networks = states
        .iter()
        .enumerate()
        .map(|(index, state)| {
            restore(
                network,
                references(network, roots[index + 1], "group")?,
                state,
                &mut claimed,
                limits,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut names = BTreeSet::new();
    let mut used = BTreeSet::new();
    let mut entries = Vec::new();
    for record in records {
        check_record(record, "entry")?;
        let group = record["group"]
            .as_f64()
            .ok_or_else(|| fail("invalid entry source group"))?;
        #[allow(clippy::cast_precision_loss)]
        if !accept_meta_language_archive_entry_group(group, networks.len() as f64) {
            return Err(fail("invalid entry source group"));
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let group = group as usize;
        let entry = MetaLanguageArchiveEntry {
            identifier: text(&record["identifier"])?.to_owned(),
            kind: text(&record["kind"])?.to_owned(),
            network: Arc::clone(&networks[group]),
            root: identity(&record["root"])?,
        };
        validate_entry(&entry)?;
        if !names.insert(entry.identifier.clone()) {
            return Err(fail("duplicate entry identifier"));
        }
        used.insert(group);
        entries.push(entry);
    }
    if used.len() != networks.len() {
        return Err(fail("unreferenced source group"));
    }
    Ok(ReconstructedMetaLanguageArchive { entries, networks })
}

fn find_root(network: &LinkNetwork) -> Result<LinkId, String> {
    let referenced: BTreeSet<_> = network
        .links()
        .flat_map(|link| link.references().iter().copied())
        .collect();
    let language = read_meta_language_archive_language();
    let term = read_meta_language_archive_term("root");
    let roots: Vec<_> = network
        .links()
        .filter(|link| {
            link.metadata().language() == Some(language.as_str())
                && link.metadata().term() == Some(term.as_str())
                && !referenced.contains(&link.id())
        })
        .map(Link::id)
        .collect();
    if roots.len() != 1 {
        return Err(fail("exactly one archive root is required"));
    }
    Ok(roots[0])
}

fn preflight_text(source: &str, limits: MetaLanguageArchiveLimits) -> Result<(), String> {
    let statements = parse_lino_to_links_with_config(
        source,
        &ParserConfig::without_comments().with_max_depth(16),
    )
    .map_err(|error| fail(&error.to_string()))?;
    if statements.len() > limits.node_limit {
        return Err(fail("node resource limit exceeded"));
    }
    let mut seen = BTreeSet::new();
    let mut count = 0_usize;
    for statement in statements {
        let LiNo::Link {
            id: Some(id),
            values,
        } = statement
        else {
            return Err(fail("invalid canonical network statement"));
        };
        if id.is_empty()
            || !id.bytes().all(|byte| byte.is_ascii_digit())
            || !seen.insert(id.clone())
            || id
                .parse::<u64>()
                .ok()
                .filter(|id| *id > 0 && *id <= 9_007_199_254_740_991)
                .is_none()
            || id.starts_with('0')
        {
            return Err(fail("invalid or duplicate Links identity"));
        }
        let mut envelope = None;
        for value in &values {
            match value {
                LiNo::Ref(reference)
                    if !reference.is_empty()
                        && reference.bytes().all(|byte| byte.is_ascii_digit()) =>
                {
                    count = count.saturating_add(1)
                }
                LiNo::Link {
                    id: Some(name),
                    values,
                } if name == "meta" && envelope.is_none() => envelope = Some(values),
                _ => return Err(fail("invalid or duplicate metadata envelope")),
            }
        }
        if count > limits.reference_limit {
            return Err(fail("reference resource limit exceeded"));
        }
        let fields = envelope.ok_or_else(|| fail("missing metadata envelope"))?;
        let mut field_names = BTreeSet::new();
        let mut named = false;
        for field in fields {
            let LiNo::Link {
                id: Some(name),
                values,
            } = field
            else {
                return Err(fail("invalid metadata field"));
            };
            if values.is_empty() {
                return Err(fail("invalid metadata field"));
            }
            if !field_names.insert(name.as_str()) {
                return Err(fail("duplicate graph metadata field"));
            }
            if name == "n" {
                named =
                    matches!(values.as_slice(), [LiNo::Ref(value)] if value == "0" || value == "1");
            }
            if name == "flags" {
                let [LiNo::Ref(value)] = values.as_slice() else {
                    return Err(fail("invalid graph flags"));
                };
                if !value.bytes().all(|byte| byte.is_ascii_digit()) {
                    return Err(fail("invalid graph flags"));
                }
                let bits = value
                    .parse::<u8>()
                    .map_err(|_| fail("invalid graph flags"))?;
                if bits > 15
                    || !accept_meta_language_archive_flags(
                        bits & 1 != 0,
                        bits & 2 != 0,
                        bits & 4 != 0,
                    )
                {
                    return Err(fail("invalid parse status relationship"));
                }
            }
            if name == "span" {
                let positions = values
                    .iter()
                    .map(|value| match value {
                        LiNo::Ref(value)
                            if !value.is_empty()
                                && value.bytes().all(|byte| byte.is_ascii_digit()) =>
                        {
                            value
                                .parse::<u64>()
                                .ok()
                                .filter(|value| *value <= 9_007_199_254_740_991)
                        }
                        _ => None,
                    })
                    .collect::<Option<Vec<_>>>()
                    .ok_or_else(|| fail("invalid graph source span"))?;
                if positions.len() != 6 || positions[1] < positions[0] {
                    return Err(fail("invalid graph source span"));
                }
            }
        }
        if !named {
            return Err(fail("missing or invalid graph metadata"));
        }
    }
    Ok(())
}

/// Reload complete canonical text without the original source buffers.
///
/// # Errors
/// Refuses malformed metadata, duplicate identities and resource violations.
pub fn read_meta_language_archive_text(
    source: &str,
    limits: MetaLanguageArchiveLimits,
) -> Result<LoadedMetaLanguageArchive, String> {
    let network = read_meta_language_archive_network_text(source, limits)?;
    let root = find_root(&network)?;
    let restored = read_meta_language_archive(&network, root, limits)?;
    Ok(LoadedMetaLanguageArchive {
        archive: MetaLanguageArchive { network, root },
        entries: restored.entries,
        networks: restored.networks,
    })
}

/// Read a complete source network under the exact archive metadata profile.
///
/// # Errors
/// Refuses malformed metadata, duplicate identities and resource violations.
pub fn read_meta_language_archive_network_text(
    source: &str,
    limits: MetaLanguageArchiveLimits,
) -> Result<LinkNetwork, String> {
    validate_limits(limits)?;
    if source.len() > limits.byte_limit {
        return Err(fail("byte resource limit exceeded"));
    }
    preflight_text(source, limits)?;
    let network = LinkNetwork::from_lino(source).map_err(|error| fail(&error.to_string()))?;
    inspect(&network, limits)?;
    Ok(network)
}

/// Reload the same complete graph through the shared binary codec.
///
/// # Errors
/// Refuses malformed framing, invalid roots and exceeded budgets.
pub fn read_meta_language_archive_binary(
    bytes: &[u8],
    limits: MetaLanguageArchiveLimits,
) -> Result<LoadedMetaLanguageArchive, String> {
    validate_limits(limits)?;
    let network = decode_link_network_binary(
        bytes,
        BinaryLinksOptions {
            record_limit: limits.node_limit,
            byte_limit: limits.byte_limit,
        },
    )
    .map_err(|error| error.to_string())?;
    let root = find_root(&network)?;
    let restored = read_meta_language_archive(&network, root, limits)?;
    Ok(LoadedMetaLanguageArchive {
        archive: MetaLanguageArchive { network, root },
        entries: restored.entries,
        networks: restored.networks,
    })
}
