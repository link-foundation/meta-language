use super::model::{
    array, copy, fail, inspect, metadata, node, number, object, state, string, validate_entry,
    validate_limits,
};
use super::{
    MetaLanguageArchive, MetaLanguageArchiveEntry, MetaLanguageArchiveLimits,
    read_meta_language_archive,
};
use crate::structured_data_links::{
    StructuredDataLinksLimits, encode_structured_data_links_ordered_with_limits,
};
use crate::{BinaryLinksOptions, LinkNetwork, encode_link_network_binary};
use std::collections::BTreeSet;
use std::sync::Arc;

/// Copy complete source groups and retain every explicit entry root.
///
/// # Errors
/// Refuses malformed roots, duplicate identities and resource exhaustion.
pub fn create_meta_language_archive(
    entries: &[MetaLanguageArchiveEntry],
    limits: MetaLanguageArchiveLimits,
) -> Result<MetaLanguageArchive, String> {
    validate_limits(limits)?;
    if entries.is_empty() || entries.len() > limits.node_limit {
        return Err(fail("expected bounded nonempty entries"));
    }
    let mut network = LinkNetwork::new();
    let mut groups: Vec<Arc<LinkNetwork>> = Vec::new();
    let mut states = Vec::new();
    let mut roots = Vec::new();
    let mut records = Vec::new();
    let mut names = BTreeSet::new();
    for entry in entries {
        validate_entry(entry)?;
        if !names.insert(entry.identifier.as_str()) {
            return Err(fail("duplicate entry identifier"));
        }
        let group = if let Some(index) = groups
            .iter()
            .position(|value| Arc::ptr_eq(value, &entry.network))
        {
            index
        } else {
            let mapping = copy(&mut network, &entry.network, limits)?;
            roots.push(node(
                &mut network,
                &mapping.values().copied().collect::<Vec<_>>(),
                "group",
            ));
            states.push(state(&entry.network));
            groups.push(Arc::clone(&entry.network));
            groups.len() - 1
        };
        records.push(object(vec![
            ("identifier", string(&entry.identifier)),
            ("kind", string(&entry.kind)),
            (
                "group",
                number(
                    u64::try_from(group)
                        .map_err(|_| fail("entry source group exceeds the identity profile"))?,
                ),
            ),
            ("root", number(entry.root.as_u64())),
        ]));
    }
    let descriptor = object(vec![
        ("schemaVersion", number(1)),
        ("entries", array(records)),
        ("groups", array(states)),
    ]);
    let encoded = encode_structured_data_links_ordered_with_limits(
        &descriptor,
        StructuredDataLinksLimits {
            node_limit: limits.node_limit,
            depth_limit: 256,
            byte_limit: limits.byte_limit,
        },
    )?;
    let copied = copy(&mut network, &encoded.network, limits)?;
    let mut references = vec![copied[&encoded.root]];
    references.extend(roots);
    let root = network.insert_link([], metadata("root"));
    network.set_references(root, &references);
    inspect(&network, limits)?;
    Ok(MetaLanguageArchive { network, root })
}

/// Write the complete selected archive as canonical Links text.
///
/// # Errors
/// Refuses an invalid archive or an exceeded output budget.
pub fn meta_language_archive_to_text(
    archive: &MetaLanguageArchive,
    limits: MetaLanguageArchiveLimits,
) -> Result<String, String> {
    read_meta_language_archive(&archive.network, archive.root, limits)?;
    let text = archive.network.to_lino();
    if text.len() > limits.byte_limit {
        return Err(fail("byte resource limit exceeded"));
    }
    Ok(text)
}

/// Write the same complete graph through the shared binary codec.
///
/// # Errors
/// Refuses invalid archive state or binary profile/resource violations.
pub fn meta_language_archive_to_binary(
    archive: &MetaLanguageArchive,
    limits: MetaLanguageArchiveLimits,
) -> Result<Vec<u8>, String> {
    read_meta_language_archive(&archive.network, archive.root, limits)?;
    encode_link_network_binary(
        &archive.network,
        BinaryLinksOptions {
            record_limit: limits.node_limit,
            byte_limit: limits.byte_limit,
        },
    )
    .map_err(|error| error.to_string())
}

/// Extend a complete graph with explicit replacement and removal selections.
///
/// # Errors
/// Refuses unknown removals, duplicate additions and implicit replacements.
pub fn extend_meta_language_archive(
    archive: &MetaLanguageArchive,
    additions: &[MetaLanguageArchiveEntry],
    replace: bool,
    remove: &[String],
    limits: MetaLanguageArchiveLimits,
) -> Result<MetaLanguageArchive, String> {
    let mut entries = read_meta_language_archive(&archive.network, archive.root, limits)?.entries;
    let mut removed = BTreeSet::new();
    for identifier in remove {
        if !removed.insert(identifier) {
            return Err(fail("unknown or duplicate removed entry"));
        }
        let position = entries
            .iter()
            .position(|entry| &entry.identifier == identifier)
            .ok_or_else(|| fail("unknown or duplicate removed entry"))?;
        entries.remove(position);
    }
    let mut added = BTreeSet::new();
    for entry in additions {
        if !added.insert(&entry.identifier) {
            return Err(fail("duplicate added entry"));
        }
        if let Some(position) = entries
            .iter()
            .position(|old| old.identifier == entry.identifier)
        {
            if !replace {
                return Err(fail("replacement requires an explicit selection"));
            }
            entries[position] = entry.clone();
        } else {
            entries.push(entry.clone());
        }
    }
    create_meta_language_archive(&entries, limits)
}
