use super::{data_limits, fail};
use crate::dynamic_meta_language_rules::{
    read_dynamic_concrete_syntax_version, read_dynamic_meta_language_identifier,
};
use crate::{
    MetaLanguageArchiveEntry, MetaLanguageArchiveLimits, decode_structured_data_links_with_limits,
    read_discovered_concept_links,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Arc;

pub(super) fn pin(
    entries: &mut Vec<MetaLanguageArchiveEntry>,
    entry: &MetaLanguageArchiveEntry,
    role: &str,
) -> String {
    let prefix = read_dynamic_meta_language_identifier(role);
    if let Some(existing) = entries.iter().find(|value| {
        value.identifier.starts_with(&prefix)
            && Arc::ptr_eq(&value.network, &entry.network)
            && value.root == entry.root
    }) {
        return existing.identifier.clone();
    }
    let identifier = crate::dynamic_meta_language_rules::read_dynamic_snapshot_identifier(
        &entries
            .iter()
            .map(|entry| entry.identifier.clone())
            .collect::<Vec<_>>(),
        &prefix,
    );
    entries.push(MetaLanguageArchiveEntry {
        identifier: identifier.clone(),
        ..entry.clone()
    });
    identifier
}

pub(super) fn fingerprint(
    entry: &MetaLanguageArchiveEntry,
    limits: MetaLanguageArchiveLimits,
) -> Result<String, String> {
    let selected = crate::meta_language_archive::select_network(
        &entry.network,
        entry.root,
        limits.node_limit,
    )?;
    let mut hash = Sha256::new();
    hash.update(format!("{}\0", entry.root.as_u64()));
    hash.update(selected.to_lino());
    Ok(format!("{:x}", hash.finalize()))
}
pub(super) fn data(
    entry: &MetaLanguageArchiveEntry,
    limits: MetaLanguageArchiveLimits,
    depth: usize,
) -> Result<Value, String> {
    let selected = crate::meta_language_archive::select_network(
        &entry.network,
        entry.root,
        limits.node_limit,
    )?;
    decode_structured_data_links_with_limits(&selected, entry.root, data_limits(limits, depth))
}
pub(super) fn check(
    entries: &[MetaLanguageArchiveEntry],
    limits: MetaLanguageArchiveLimits,
    depth: usize,
) -> Result<(), String> {
    let directory: BTreeMap<_, _> = entries
        .iter()
        .map(|entry| (entry.identifier.as_str(), entry))
        .collect();
    let mut checked = BTreeMap::<String, String>::new();
    let mut sources = BTreeMap::<String, Vec<String>>::new();
    for entry in entries {
        if entry.kind != "concrete syntax"
            || entry
                .network
                .link(entry.root)
                .and_then(|link| link.metadata().language())
                != Some("meta-language structured data")
        {
            continue;
        }
        let snapshot = data(entry, limits, depth)?;
        if snapshot["schemaVersion"].as_f64() != Some(read_dynamic_concrete_syntax_version())
            || snapshot["projection"].as_str() != Some("concrete syntax")
        {
            continue;
        }
        for (field, role, kind) in [
            (
                "definitionContext",
                "definition snapshot",
                "discovered concepts",
            ),
            (
                "configurationContext",
                "configuration snapshot",
                "language configuration",
            ),
        ] {
            let identifier = snapshot[field]
                .as_str()
                .ok_or_else(|| fail("missing concrete syntax context"))?;
            let selected = directory
                .get(identifier)
                .ok_or_else(|| fail("missing concrete syntax context"))?;
            if !identifier.starts_with(&read_dynamic_meta_language_identifier(role))
                || selected.kind != kind
                    && !(kind == "language configuration" && selected.kind == "decorators")
            {
                return Err(fail("missing concrete syntax context"));
            }
            if !checked.contains_key(identifier) {
                if kind == "discovered concepts" {
                    let selected_network = crate::meta_language_archive::select_network(
                        &selected.network,
                        selected.root,
                        limits.node_limit,
                    )?;
                    let discovered =
                        read_discovered_concept_links(&selected_network, selected.root)?;
                    sources.insert(
                        identifier.to_owned(),
                        discovered
                            .sources
                            .iter()
                            .map(|source| source.id.clone())
                            .collect(),
                    );
                } else {
                    data(selected, limits, depth)?;
                }
                checked.insert(identifier.to_owned(), fingerprint(selected, limits)?);
            }
            let fingerprint = snapshot[format!("{field}Fingerprint")]
                .as_str()
                .ok_or_else(|| fail("concrete syntax context changed"))?;
            if checked.get(identifier).map(String::as_str) != Some(fingerprint) {
                return Err(fail("concrete syntax context changed"));
            }
            if let Some(identifiers) = sources.get(identifier) {
                let source = snapshot["sourceIdentifier"]
                    .as_str()
                    .ok_or_else(|| fail("unknown concrete syntax source"))?;
                if !identifiers.iter().any(|identifier| identifier == source) {
                    return Err(fail("unknown concrete syntax source"));
                }
            }
        }
    }
    Ok(())
}
