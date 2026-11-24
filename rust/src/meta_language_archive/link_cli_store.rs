//! link-cli's two-packet store envelope for the shared archive incidence layout.

use super::model::{fail, identity, list, state, validate_limits};
use super::read::restore;
use super::{
    LoadedMetaLanguageArchive, MetaLanguageArchive, MetaLanguageArchiveLimits,
    read_meta_language_archive, read_meta_language_archive_network_text,
};
use crate::link_cli_archive_topology_rules::{
    LinkCliArchiveTopology, plan_link_cli_archive_doublets, read_link_cli_archive_doublets,
    read_link_cli_archive_state_name, read_link_cli_archive_store_name,
};
use crate::lino_serialization::encode_link;
use crate::structured_data_links::{
    StructuredDataLinksLimits, decode_structured_data_links_with_limits,
    encode_structured_data_links_ordered_with_limits,
};
use crate::{Link, LinkId, LinkNetwork};
use link_cli_links_notation::binary::{DecodeLimits, LinksPacket, Reference};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::sync::Arc;

fn packet_limits(limits: MetaLanguageArchiveLimits) -> (u32, DecodeLimits, DecodeLimits) {
    let records = (limits.node_limit as u64 * 2).min(u64::from(u32::MAX) - 2);
    let links = (records + 2 + limits.reference_limit as u64 * 2).min(u64::from(u32::MAX));
    (
        u32::try_from(links).expect("store addresses fit in a u32"),
        DecodeLimits {
            max_links: links,
            max_references: links * 2,
            ..DecodeLimits::default()
        },
        DecodeLimits {
            max_links: records + 2,
            max_references: limits.byte_limit as u64 + (records + 2).min(limits.byte_limit as u64),
            ..DecodeLimits::default()
        },
    )
}

fn data_limits(limits: MetaLanguageArchiveLimits) -> StructuredDataLinksLimits {
    StructuredDataLinksLimits {
        node_limit: limits.node_limit,
        depth_limit: 256,
        byte_limit: limits.byte_limit,
    }
}

fn store_number(number: usize) -> Result<f64, String> {
    u32::try_from(number)
        .map(f64::from)
        .map_err(|_| fail("archive store link limit exceeded"))
}

// Only the checked topology transit uses binary64. Its validated output is
// always integral and bounded by the unsigned store address width.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn store_index(number: f64) -> usize {
    number as u32 as usize
}

struct StoreDirectory {
    ends: Vec<f64>,
    targets: Vec<f64>,
    names: Vec<String>,
}

impl StoreDirectory {
    fn append(
        &mut self,
        links: &[&Link],
        prefix: &str,
        reference_limit: usize,
    ) -> Result<BTreeMap<LinkId, f64>, String> {
        let addresses = links
            .iter()
            .enumerate()
            .map(|(index, link)| Ok((link.id(), store_number(self.names.len() + index + 1)?)))
            .collect::<Result<BTreeMap<_, _>, String>>()?;
        let start = self.targets.len();
        for link in links {
            if self.targets.len() - start + link.references().len() > reference_limit {
                return Err(fail("reference resource limit exceeded"));
            }
            self.targets
                .extend(link.references().iter().map(|target| addresses[target]));
            self.ends.push(store_number(self.targets.len())?);
            let mut name = prefix.to_owned();
            encode_link(link.id(), &[], link.metadata(), false, &mut name);
            self.names.push(name);
        }
        Ok(addresses)
    }
}

/// Project a complete archive into the native link-cli store envelope.
///
/// # Errors
/// Refuses incomplete archives and projections outside the resource profile.
pub fn meta_language_archive_to_link_cli_store(
    archive: &MetaLanguageArchive,
    limits: MetaLanguageArchiveLimits,
) -> Result<Vec<u8>, String> {
    read_meta_language_archive(&archive.network, archive.root, limits)?;
    let (store_limit, _, _) = packet_limits(limits);
    let descriptor = encode_structured_data_links_ordered_with_limits(
        &state(&archive.network),
        data_limits(limits),
    )?;
    let links = archive.network.links().collect::<Vec<_>>();
    let state_links = std::iter::once(
        descriptor
            .network
            .link(descriptor.root)
            .expect("encoded state root"),
    )
    .chain(
        descriptor
            .network
            .links()
            .filter(|link| link.id() != descriptor.root),
    )
    .collect::<Vec<_>>();
    let state_address = store_number(links.len() + 3)?;
    let mut directory = StoreDirectory {
        ends: Vec::new(),
        targets: Vec::new(),
        names: vec![
            read_link_cli_archive_store_name(),
            read_link_cli_archive_state_name(),
        ],
    };
    let addresses = directory.append(&links, "link ", limits.reference_limit)?;
    directory.append(&state_links, "state ", limits.reference_limit)?;
    let doublets = match plan_link_cli_archive_doublets(
        &directory.ends,
        &directory.targets,
        addresses[&archive.root],
        state_address,
        f64::from(store_limit),
    ) {
        LinkCliArchiveTopology::ArchiveDoublets(doublets) => doublets,
        LinkCliArchiveTopology::ArchiveTopologyError(reason) => return Err(fail(&reason)),
        LinkCliArchiveTopology::ArchiveDirectory(..) => {
            return Err(fail("invalid archive doublets"));
        }
    };
    let rows = doublets
        .chunks_exact(2)
        .enumerate()
        .map(|(index, pair)| {
            (
                index as u64 + 1,
                pair.iter()
                    .map(|endpoint| Reference::Internal(store_index(*endpoint) as u64))
                    .collect(),
            )
        })
        .collect::<Vec<_>>();
    let mut name_bytes = 0_usize;
    let names = directory
        .names
        .iter()
        .enumerate()
        .map(|(index, name)| {
            name_bytes = name_bytes.saturating_add(name.len());
            if name_bytes > limits.byte_limit {
                return Err(fail("byte resource limit exceeded"));
            }
            let address = index as u64 + 1;
            let references = std::iter::once(Reference::External(address))
                .chain(
                    name.bytes()
                        .map(|character| Reference::External(u64::from(character))),
                )
                .collect();
            Ok((address, references))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut output = Vec::new();
    LinksPacket::pack(false, &rows, true)
        .and_then(|packet| packet.write_to(&mut output))
        .map_err(|error| fail(&error.to_string()))?;
    LinksPacket::pack(true, &names, true)
        .and_then(|packet| packet.write_to(&mut output))
        .map_err(|error| fail(&error.to_string()))?;
    if output.len() > limits.byte_limit {
        return Err(fail("byte resource limit exceeded"));
    }
    Ok(output)
}

fn packet(source: &mut &[u8], limits: &DecodeLimits) -> Result<LinksPacket, String> {
    LinksPacket::read_from(source, limits)
        .map_err(|error| fail(&error.to_string()))?
        .ok_or_else(|| fail("invalid link-cli store packet sequence"))
}

struct Record<'a> {
    id: LinkId,
    metadata: &'a str,
    name: &'a str,
}

fn record<'a>(name: &'a str, prefix: &str) -> Result<Record<'a>, String> {
    let name = name
        .strip_prefix(prefix)
        .ok_or_else(|| fail("invalid archive record name"))?;
    let (id, metadata) = name
        .strip_prefix('(')
        .and_then(|tail| tail.split_once(':'))
        .ok_or_else(|| fail("invalid archive record name"))?;
    let id = id
        .parse::<u64>()
        .ok()
        .filter(|value| *value > 0 && *value <= 9_007_199_254_740_991)
        .ok_or_else(|| fail("invalid archive record name"))?;
    if !metadata.starts_with(" (meta: ")
        || !metadata.ends_with("))")
        || metadata.contains(['\r', '\n'])
    {
        return Err(fail("invalid archive record name"));
    }
    Ok(Record {
        id: LinkId::from_u64(id),
        metadata,
        name,
    })
}

fn restore_records(
    records: &[Record<'_>],
    ends: &[f64],
    targets: &[f64],
    range: std::ops::Range<usize>,
    limits: MetaLanguageArchiveLimits,
) -> Result<LinkNetwork, String> {
    let mut start = range
        .start
        .checked_sub(1)
        .map_or(0, |previous| store_index(ends[previous]));
    let end = store_index(ends[range.end - 1]);
    if end - start > limits.reference_limit {
        return Err(fail("reference resource limit exceeded"));
    }
    let mut text = String::new();
    for index in range.clone() {
        let record = &records[index];
        write!(&mut text, "({}:", record.id).expect("writing to a String never fails");
        for address in &targets[start..store_index(ends[index])] {
            write!(&mut text, " {}", records[store_index(*address) - 3].id)
                .expect("writing to a String never fails");
            if text.len() > limits.byte_limit {
                return Err(fail("byte resource limit exceeded"));
            }
        }
        text.push_str(record.metadata);
        text.push('\n');
        if text.len() > limits.byte_limit {
            return Err(fail("byte resource limit exceeded"));
        }
        start = store_index(ends[index]);
    }
    let network = read_meta_language_archive_network_text(&text, limits)?;
    for record in &records[range] {
        let link = network
            .link(record.id)
            .ok_or_else(|| fail("missing archive record"))?;
        let mut name = String::new();
        encode_link(record.id, &[], link.metadata(), false, &mut name);
        if name != record.name {
            return Err(fail("noncanonical archive record name"));
        }
    }
    Ok(network)
}

/// Reconstruct a complete archive from an exported link-cli store.
///
/// # Errors
/// Refuses malformed packets, incidence chains, metadata, or archive state.
pub fn read_meta_language_archive_link_cli_store(
    mut source: &[u8],
    limits: MetaLanguageArchiveLimits,
) -> Result<LoadedMetaLanguageArchive, String> {
    validate_limits(limits)?;
    if source.len() > limits.byte_limit {
        return Err(fail("invalid or oversized link-cli store"));
    }
    let (store_limit, topology_limits, name_limits) = packet_limits(limits);
    let topology = packet(&mut source, &topology_limits)?;
    let directory = packet(&mut source, &name_limits)?;
    if topology.external_references || !directory.external_references || !source.is_empty() {
        return Err(fail("invalid link-cli store packet sequence"));
    }
    let mut doublets = Vec::new();
    for (address, references) in topology.links() {
        let [Reference::Internal(source), Reference::Internal(target)] = references else {
            return Err(fail("invalid link-cli store doublet"));
        };
        if address != doublets.len() as u64 / 2 + 1 {
            return Err(fail("invalid link-cli store doublet"));
        }
        doublets.push(f64::from(
            u32::try_from(*source).map_err(|_| fail("invalid store address"))?,
        ));
        doublets.push(f64::from(
            u32::try_from(*target).map_err(|_| fail("invalid store address"))?,
        ));
    }
    let names = directory
        .links()
        .enumerate()
        .map(|(index, (address, references))| {
            let [Reference::External(target), characters @ ..] = references else {
                return Err(fail("invalid link-cli store name"));
            };
            if address != index as u64 + 1 || *target != address || characters.is_empty() {
                return Err(fail("invalid link-cli store name"));
            }
            characters
                .iter()
                .map(|character| match character {
                    Reference::External(value) if *value <= 127 => {
                        Ok(char::from(u8::try_from(*value).expect("bounded ASCII")))
                    }
                    _ => Err(fail("invalid archive name character")),
                })
                .collect::<Result<String, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let [store_name, state_name, names @ ..] = names.as_slice() else {
        return Err(fail("unknown link-cli archive version"));
    };
    if store_name != &read_link_cli_archive_store_name()
        || state_name != &read_link_cli_archive_state_name()
    {
        return Err(fail("unknown link-cli archive version"));
    }
    let (root, first_state, ends, targets) = match read_link_cli_archive_doublets(
        &doublets,
        store_number(names.len())?,
        f64::from(store_limit),
    ) {
        LinkCliArchiveTopology::ArchiveDirectory(root, state, ends, targets) => {
            (store_index(root) - 3, store_index(state) - 3, ends, targets)
        }
        LinkCliArchiveTopology::ArchiveTopologyError(reason) => return Err(fail(&reason)),
        LinkCliArchiveTopology::ArchiveDoublets(_) => {
            return Err(fail("invalid archive record directory"));
        }
    };
    if first_state > limits.node_limit || names.len() - first_state > limits.node_limit {
        return Err(fail("node resource limit exceeded"));
    }
    let records = names
        .iter()
        .enumerate()
        .map(|(index, name)| {
            record(
                name,
                if index < first_state {
                    "link "
                } else {
                    "state "
                },
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let physical = restore_records(&records, &ends, &targets, 0..first_state, limits)?;
    let descriptor = restore_records(
        &records,
        &ends,
        &targets,
        first_state..records.len(),
        limits,
    )?;
    let state = decode_structured_data_links_with_limits(
        &descriptor,
        records[first_state].id,
        data_limits(limits),
    )?;
    let inventory = physical.links().map(Link::id).collect::<Vec<_>>();
    let original = list(&state["identities"])?;
    if original.len() != inventory.len()
        || inventory
            .iter()
            .zip(original)
            .any(|(id, value)| identity(value).as_ref() != Ok(id))
    {
        return Err(fail("archive identity directory disagrees with its state"));
    }
    let network = Arc::unwrap_or_clone(restore(
        &physical,
        &inventory,
        &state,
        &mut BTreeSet::new(),
        limits,
    )?);
    let root = records[root].id;
    let restored = read_meta_language_archive(&network, root, limits)?;
    Ok(LoadedMetaLanguageArchive {
        archive: MetaLanguageArchive { network, root },
        entries: restored.entries,
        networks: restored.networks,
    })
}
