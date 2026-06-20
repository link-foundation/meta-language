use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::{
    tree_sitter_adapter, ByteRange, Link, LinkId, LinkMetadata, LinkNetwork, LinkType,
    ParseConfiguration,
};

impl LinkNetwork {
    /// Applies a byte-range source edit and reparses the network.
    ///
    /// Tree-sitter-backed languages use tree-sitter's incremental parse path;
    /// other languages fall back to the built-in lossless parser. Links whose
    /// source spans are outside the replaced byte range keep their identifiers.
    pub fn apply_edit(&mut self, range: ByteRange, replacement: &str) -> bool {
        self.apply_edit_with_configuration(range, replacement, ParseConfiguration::default())
    }

    /// Applies a byte-range source edit using an explicit parse configuration.
    ///
    /// Returns `false` when the range is outside the reconstructed source text,
    /// splits a UTF-8 code point, or the network has no document language.
    pub fn apply_edit_with_configuration(
        &mut self,
        range: ByteRange,
        replacement: &str,
        configuration: ParseConfiguration,
    ) -> bool {
        let old_text = self.reconstruct_text();
        let Some(edited_text) = apply_text_edit(&old_text, range, replacement) else {
            return false;
        };
        let Some(language) = document_language(self).map(ToOwned::to_owned) else {
            return false;
        };

        let reparsed = tree_sitter_adapter::parse_incremental(
            &old_text,
            range,
            replacement,
            &language,
            configuration,
        )
        .unwrap_or_else(|| Self::parse(&edited_text, &language, configuration));

        let edit = AppliedEdit::new(range, replacement.len());
        *self = remap_reparsed_network(self, &reparsed, edit);
        true
    }
}

fn document_language(network: &LinkNetwork) -> Option<&str> {
    network
        .links()
        .find(|link| link.metadata().link_type() == Some(LinkType::Document))
        .and_then(|link| link.metadata().language())
}

fn remap_reparsed_network(
    old: &LinkNetwork,
    reparsed: &LinkNetwork,
    edit: AppliedEdit,
) -> LinkNetwork {
    let mut id_map = stable_id_map(old, reparsed, edit);
    let mut used_targets = id_map.values().copied().collect::<BTreeSet<_>>();
    let mut next_id = old.next_id.max(reparsed.next_id);

    for link in reparsed.links() {
        if id_map.contains_key(&link.id()) {
            continue;
        }

        let target = if used_targets.contains(&link.id()) {
            let fresh = next_unused_id(&mut next_id, &used_targets);
            used_targets.insert(fresh);
            fresh
        } else {
            used_targets.insert(link.id());
            link.id()
        };
        id_map.insert(link.id(), target);
    }

    let mut links = BTreeMap::new();
    for link in reparsed.links() {
        let id = id_map[&link.id()];
        let references = link
            .references()
            .iter()
            .map(|reference| id_map[reference])
            .collect::<Vec<_>>();
        let candidate = Link {
            id,
            references: Arc::from(references),
            metadata: link.metadata().clone(),
        };

        if let Some(shared) = old.links.get(&id) {
            if shared.as_ref() == &candidate {
                links.insert(id, Arc::clone(shared));
                continue;
            }
        }

        links.insert(id, Arc::new(candidate));
    }

    let terms = reparsed
        .terms
        .iter()
        .filter_map(|(term, id)| id_map.get(id).map(|mapped| (Arc::clone(term), *mapped)))
        .collect();
    let next_id = used_targets
        .iter()
        .map(|id| id.as_u64())
        .max()
        .map_or(1, |id| id + 1);

    LinkNetwork {
        next_id,
        links,
        terms,
        concept_syntax: reparsed.concept_syntax.clone(),
        strings: reparsed.strings.clone(),
    }
}

fn stable_id_map(
    old: &LinkNetwork,
    reparsed: &LinkNetwork,
    edit: AppliedEdit,
) -> BTreeMap<LinkId, LinkId> {
    let old_links = old.links().collect::<Vec<_>>();
    let mut mapped = BTreeMap::new();
    let mut used_old = BTreeSet::new();

    for new_link in reparsed.links() {
        let Some(old_link) = old_links.iter().find(|old_link| {
            !used_old.contains(&old_link.id())
                && stable_span_or_point_match(old_link, new_link, edit)
        }) else {
            continue;
        };
        mapped.insert(new_link.id(), old_link.id());
        used_old.insert(old_link.id());
    }

    loop {
        let mut added = false;
        for new_link in reparsed.links() {
            if mapped.contains_key(&new_link.id()) || new_link.metadata().span().is_some() {
                continue;
            }
            let Some(old_link) = old_links.iter().find(|old_link| {
                !used_old.contains(&old_link.id())
                    && stable_reference_match(old_link, new_link, &mapped)
            }) else {
                continue;
            };
            mapped.insert(new_link.id(), old_link.id());
            used_old.insert(old_link.id());
            added = true;
        }
        if !added {
            break;
        }
    }

    mapped
}

fn apply_text_edit(old_text: &str, range: ByteRange, replacement: &str) -> Option<String> {
    if range.end() > old_text.len()
        || !old_text.is_char_boundary(range.start())
        || !old_text.is_char_boundary(range.end())
    {
        return None;
    }

    let mut edited =
        String::with_capacity(old_text.len() - (range.end() - range.start()) + replacement.len());
    edited.push_str(&old_text[..range.start()]);
    edited.push_str(replacement);
    edited.push_str(&old_text[range.end()..]);
    Some(edited)
}

fn next_unused_id(next_id: &mut u64, used_targets: &BTreeSet<LinkId>) -> LinkId {
    loop {
        let candidate = LinkId(*next_id);
        *next_id += 1;
        if !used_targets.contains(&candidate) {
            return candidate;
        }
    }
}

fn stable_span_or_point_match(old: &Link, new: &Link, edit: AppliedEdit) -> bool {
    if !metadata_without_span_eq(old.metadata(), new.metadata()) {
        return false;
    }

    match (old.metadata().span(), new.metadata().span()) {
        (Some(old_span), Some(new_span)) => edit
            .adjusted_range(old_span.byte_range())
            .is_some_and(|range| range == new_span.byte_range()),
        (None, None) => {
            old.references() == new.references()
                || (is_self_reference(old) && is_self_reference(new))
        }
        _ => false,
    }
}

fn stable_reference_match(old: &Link, new: &Link, stable_map: &BTreeMap<LinkId, LinkId>) -> bool {
    if old.metadata().span().is_some()
        || new.metadata().span().is_some()
        || !metadata_without_span_eq(old.metadata(), new.metadata())
        || old.references().len() != new.references().len()
    {
        return false;
    }

    new.references()
        .iter()
        .zip(old.references())
        .all(|(new_reference, old_reference)| stable_map.get(new_reference) == Some(old_reference))
}

fn metadata_without_span_eq(old: &LinkMetadata, new: &LinkMetadata) -> bool {
    old.link_type() == new.link_type()
        && old.is_named() == new.is_named()
        && old.term() == new.term()
        && old.definition() == new.definition()
        && old.language() == new.language()
        && old.flags() == new.flags()
}

fn is_self_reference(link: &Link) -> bool {
    link.references() == [link.id()]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AppliedEdit {
    old_start: usize,
    old_end: usize,
    new_end: usize,
}

impl AppliedEdit {
    const fn new(range: ByteRange, replacement_len: usize) -> Self {
        Self {
            old_start: range.start(),
            old_end: range.end(),
            new_end: range.start() + replacement_len,
        }
    }

    const fn adjusted_range(self, range: ByteRange) -> Option<ByteRange> {
        if range.end() <= self.old_start {
            return Some(range);
        }
        if range.start() < self.old_end {
            return None;
        }

        let start = shift_byte(range.start(), self.old_end, self.new_end);
        let end = shift_byte(range.end(), self.old_end, self.new_end);
        Some(ByteRange::new(start, end))
    }
}

const fn shift_byte(byte: usize, old_end: usize, new_end: usize) -> usize {
    if new_end >= old_end {
        byte + (new_end - old_end)
    } else {
        byte - (old_end - new_end)
    }
}
