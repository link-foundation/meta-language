use std::collections::{BTreeMap, BTreeSet, VecDeque};
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
    let mut old_links = OldLinkIndex::new(old, edit);
    let mut mapped = BTreeMap::new();

    for new_link in reparsed.links() {
        let key = MatchKey::of(new_link.metadata());
        let matched = match new_link.metadata().span() {
            Some(span) => old_links.take_spanned(key, span.byte_range()),
            None => old_links.take_unspanned(key, new_link),
        };
        if let Some(old_id) = matched {
            mapped.insert(new_link.id(), old_id);
        }
    }

    // A link without a span is recognized by the identifiers it references, so
    // it can only be matched once everything it references has been. Repeat
    // until a round maps nothing new.
    let mut pending = reparsed
        .links()
        .filter(|link| link.metadata().span().is_none() && !mapped.contains_key(&link.id()))
        .collect::<Vec<_>>();
    while !pending.is_empty() {
        let remaining = pending.len();
        let mut unmatched = Vec::with_capacity(remaining);
        for new_link in pending {
            let matched = new_link
                .references()
                .iter()
                .map(|reference| mapped.get(reference).copied())
                .collect::<Option<Vec<_>>>()
                .and_then(|references| {
                    old_links.take_referencing(MatchKey::of(new_link.metadata()), references)
                });
            match matched {
                Some(old_id) => {
                    mapped.insert(new_link.id(), old_id);
                }
                None => unmatched.push(new_link),
            }
        }
        if unmatched.len() == remaining {
            break;
        }
        pending = unmatched;
    }

    mapped
}

/// Metadata two links must share to keep one identifier, shaped as a map key so
/// that candidates can be looked up instead of searched for.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MatchKey<'a> {
    link_type: Option<LinkType>,
    named: bool,
    term: Option<&'a str>,
    definition: Option<&'a str>,
    language: Option<&'a str>,
    /// Parse flags spelled out as booleans, because `LinkFlags` carries no
    /// ordering of its own.
    flags: (bool, bool, bool, bool),
}

impl<'a> MatchKey<'a> {
    fn of(metadata: &'a LinkMetadata) -> Self {
        let flags = metadata.flags();
        Self {
            link_type: metadata.link_type(),
            named: metadata.is_named(),
            term: metadata.term(),
            definition: metadata.definition(),
            language: metadata.language(),
            flags: (
                flags.is_error(),
                flags.has_error(),
                flags.is_missing(),
                flags.is_extra(),
            ),
        }
    }
}

/// Identifiers of the previous network's links, grouped by everything a match
/// requires.
///
/// Reparsing produces about as many links as the previous parse held, so
/// searching every old link for every new one costs `O(old * new)` and made a
/// 15 KB edit take twenty seconds (issue #193). Every group keeps its links in
/// ascending identifier order and hands them out from the front, which is the
/// link a search over `old.links()` would have stopped at first.
struct OldLinkIndex<'a> {
    /// Links carrying a span, keyed by the span the edit moves them to.
    spanned: BTreeMap<(MatchKey<'a>, usize, usize), VecDeque<LinkId>>,
    /// Links carrying no span, keyed by the identifiers they reference.
    referencing: BTreeMap<(MatchKey<'a>, Vec<LinkId>), VecDeque<LinkId>>,
    /// Links carrying no span that reference themselves and nothing else.
    self_referencing: BTreeMap<MatchKey<'a>, VecDeque<LinkId>>,
    /// Links already handed out, which every group skips.
    used: BTreeSet<LinkId>,
}

impl<'a> OldLinkIndex<'a> {
    fn new(old: &'a LinkNetwork, edit: AppliedEdit) -> Self {
        let mut index = Self {
            spanned: BTreeMap::new(),
            referencing: BTreeMap::new(),
            self_referencing: BTreeMap::new(),
            used: BTreeSet::new(),
        };

        for link in old.links() {
            let key = MatchKey::of(link.metadata());
            if let Some(span) = link.metadata().span() {
                // Links the edit overlaps cannot keep their identifier.
                if let Some(range) = edit.adjusted_range(span.byte_range()) {
                    index
                        .spanned
                        .entry((key, range.start(), range.end()))
                        .or_default()
                        .push_back(link.id());
                }
                continue;
            }

            index
                .referencing
                .entry((key, link.references().to_vec()))
                .or_default()
                .push_back(link.id());
            if is_self_reference(link) {
                index
                    .self_referencing
                    .entry(key)
                    .or_default()
                    .push_back(link.id());
            }
        }

        index
    }

    /// Identifier of an unused old link whose shifted span is `range`.
    fn take_spanned(&mut self, key: MatchKey<'a>, range: ByteRange) -> Option<LinkId> {
        let matched = Self::front(
            self.spanned.get_mut(&(key, range.start(), range.end())),
            &self.used,
        )?;
        self.used.insert(matched);
        Some(matched)
    }

    /// Identifier of an unused old link that carries no span and either
    /// references exactly what `new_link` references or, when `new_link`
    /// references only itself, references only itself as well.
    fn take_unspanned(&mut self, key: MatchKey<'a>, new_link: &Link) -> Option<LinkId> {
        let by_references = Self::front(
            self.referencing
                .get_mut(&(key, new_link.references().to_vec())),
            &self.used,
        );
        let by_self_reference = if is_self_reference(new_link) {
            Self::front(self.self_referencing.get_mut(&key), &self.used)
        } else {
            None
        };

        // A search over `old.links()` would have stopped at whichever of the
        // two candidates was inserted first, which is the smaller identifier.
        let matched = by_references.into_iter().chain(by_self_reference).min()?;
        self.used.insert(matched);
        Some(matched)
    }

    /// Identifier of an unused old link that carries no span and references
    /// exactly `references`.
    fn take_referencing(&mut self, key: MatchKey<'a>, references: Vec<LinkId>) -> Option<LinkId> {
        let matched = Self::front(self.referencing.get_mut(&(key, references)), &self.used)?;
        self.used.insert(matched);
        Some(matched)
    }

    /// First unused identifier in `group`, dropping the used ones it skips.
    fn front(group: Option<&mut VecDeque<LinkId>>, used: &BTreeSet<LinkId>) -> Option<LinkId> {
        let group = group?;
        while group.front().is_some_and(|id| used.contains(id)) {
            group.pop_front();
        }
        group.front().copied()
    }
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
