//! Preparing sources and merging the rules of one language edition.

use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

use super::declarations::{declarations_text, merge_declarations, rule_fields};
use super::normalize::{normalize, quote};
use super::rename::renamed_rule;
use super::{
    GRAMMAR_MERGE_METHOD, GrammarMergeAlternative, GrammarMergeAlternativeReason,
    GrammarMergeDecision, GrammarMergeDecisionKind, GrammarMergeError, GrammarMergeFailure,
    GrammarMergeFailureKind, GrammarMergeFailureReason, GrammarMergeNomination,
    GrammarMergeNominationBasis, GrammarMergeNominationOutcome, GrammarMergeSource,
    MergedGrammarGroup,
};
use crate::grammar::{Grammar, GrammarFormat, GrammarRule};

pub(super) struct Prepared<'a> {
    pub(super) id: &'a str,
    pub(super) language: &'a str,
    pub(super) edition: &'a str,
    pub(super) precedence: u32,
    pub(super) grammar: &'a Grammar,
    pub(super) names: BTreeSet<&'a str>,
}

impl Prepared<'_> {
    fn format(&self) -> &'static str {
        format_of(self.grammar)
    }

    fn has_rule(&self, name: &str) -> bool {
        self.names.contains(name)
    }
}

pub(super) fn prepare_sources(
    sources: &[GrammarMergeSource],
) -> Result<Vec<Prepared<'_>>, GrammarMergeError> {
    let mut seen = BTreeSet::new();
    let mut prepared = Vec::with_capacity(sources.len());
    for source in sources {
        let id = source.id.as_str();
        if id.is_empty() || id.contains(':') {
            return Err(GrammarMergeError::new(format!(
                "invalid merge source id {}",
                quote(id)
            )));
        }
        if !seen.insert(id) {
            return Err(GrammarMergeError::new(format!(
                "duplicate merge source id {id}"
            )));
        }
        if source.language.is_empty() {
            return Err(GrammarMergeError::new(format!(
                "merge source {id} has no language"
            )));
        }
        let mut names = BTreeSet::new();
        for rule in source.grammar.rules() {
            if !names.insert(rule.name.as_str()) {
                return Err(GrammarMergeError::new(format!(
                    "merge source {id} defines rule {} twice",
                    rule.name
                )));
            }
        }
        prepared.push(Prepared {
            id,
            language: &source.language,
            edition: &source.edition,
            precedence: source.precedence,
            grammar: &source.grammar,
            names,
        });
    }
    prepared.sort_by(|left, right| {
        left.precedence
            .cmp(&right.precedence)
            .then_with(|| left.id.cmp(right.id))
    });
    Ok(prepared)
}

pub(super) fn normalize_samples(
    samples: &BTreeMap<String, Vec<String>>,
) -> BTreeMap<String, Vec<String>> {
    samples
        .iter()
        .filter(|(_, values)| !values.is_empty())
        .map(|(alias, values)| {
            let unique: BTreeSet<&String> = values.iter().collect();
            (alias.clone(), unique.into_iter().cloned().collect())
        })
        .collect()
}

pub(super) fn group_key(language: &str, edition: &str) -> String {
    format!("{language}@{edition}")
}

pub(super) fn source_of(alias: &str) -> &str {
    alias.split_once(':').map_or(alias, |(source, _)| source)
}

fn format_of(grammar: &Grammar) -> &'static str {
    grammar
        .source_format()
        .map_or("none", GrammarFormat::as_str)
}

pub(super) fn group_fingerprint(
    language: &str,
    edition: &str,
    entry: &[&Prepared<'_>],
    required: &[&(String, String)],
    samples: &BTreeMap<String, Vec<String>>,
) -> Result<String, GrammarMergeError> {
    let mut lines = vec![
        "grammar-merge v1".to_owned(),
        format!("group {} {}", quote(language), quote(edition)),
    ];
    let mut ids = BTreeSet::new();
    for source in entry {
        ids.insert(source.id);
        let start = source
            .grammar
            .start_rule()
            .map_or_else(|| "-".to_owned(), |rule| quote(&rule.name));
        lines.push(format!(
            "source {} {} {} {start}",
            quote(source.id),
            source.precedence,
            quote(source.format())
        ));
        let label = source_label(source, &name_label);
        for rule in source.grammar.rules() {
            lines.push(format!(
                "rule {} {}:{}{}",
                quote(&rule.name),
                rule.kind.as_str(),
                normalize(&rule.expr, &label)?.text,
                rule_fields(rule, &label)
            ));
            // Concept and documentation annotations exist only in the Rust IR;
            // they are fingerprinted so that changing them re-merges the group.
            if let Some(concept) = &rule.concept {
                lines.push(format!("concept {} {}", quote(&rule.name), quote(concept)));
            }
            if let Some(doc) = &rule.doc {
                lines.push(format!("doc {} {}", quote(&rule.name), quote(doc)));
            }
        }
        let declarations = declarations_text(source.grammar.declarations());
        if !declarations.is_empty() {
            lines.push(format!(
                "declarations {} {}",
                quote(source.id),
                quote(&declarations)
            ));
        }
    }
    for (first, second) in required {
        lines.push(format!("required {} {}", quote(first), quote(second)));
    }
    for (alias, values) in samples {
        if ids.contains(source_of(alias)) {
            let values: Vec<String> = values.iter().map(|value| quote(value)).collect();
            lines.push(format!("samples {} {}", quote(alias), values.join(",")));
        }
    }
    let mut text = lines.join("\n");
    text.push('\n');
    Ok(Sha256::digest(text.as_bytes())
        .iter()
        .fold(String::with_capacity(64), |mut hex, byte| {
            use std::fmt::Write as _;
            let _ = write!(hex, "{byte:02x}");
            hex
        }))
}

fn source_label<'a>(
    source: &'a Prepared<'_>,
    internal: &'a dyn Fn(&str) -> String,
) -> impl Fn(&str) -> String + 'a {
    move |name: &str| {
        if source.has_rule(name) {
            internal(name)
        } else {
            format!("ext({},{})", quote(source.format()), quote(name))
        }
    }
}

pub(super) fn name_label(name: &str) -> String {
    format!("ref({})", quote(name))
}

fn alias_of(source: &str, name: &str) -> String {
    format!("{source}:{name}")
}

struct Node<'a> {
    alias: String,
    source: &'a Prepared<'a>,
    rule: &'a GrammarRule,
}

pub(super) fn merge_group(
    language: &str,
    edition: &str,
    entry: &[&Prepared<'_>],
    fingerprint: String,
    samples: &BTreeMap<String, Vec<String>>,
    previous_identities: &BTreeMap<String, String>,
) -> Result<MergedGrammarGroup, GrammarMergeError> {
    let mut nodes = Vec::new();
    let mut index = BTreeMap::new();
    let mut externals = BTreeSet::new();
    for source in entry {
        for rule in source.grammar.rules() {
            let alias = alias_of(source.id, &rule.name);
            index.insert(alias.clone(), nodes.len());
            nodes.push(Node {
                alias,
                source,
                rule,
            });
        }
        for name in source.grammar.referenced_nonterminals() {
            if !source.has_rule(&name) {
                externals.insert(name);
            }
        }
    }

    let classes = refine(&nodes, &index)?;
    let mut members: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut class_order = Vec::new();
    for (position, class) in classes.iter().enumerate() {
        let list = members.entry(*class).or_default();
        if list.is_empty() {
            class_order.push(*class);
        }
        list.push(position);
    }

    let mut names: BTreeMap<usize, String> = BTreeMap::new();
    let mut taken = externals;
    let mut renamed = BTreeSet::new();
    for class in &class_order {
        for &position in &members[class] {
            if let Some(previous) = previous_identities.get(&nodes[position].alias)
                && !taken.contains(previous)
            {
                names.insert(*class, previous.clone());
                taken.insert(previous.clone());
                break;
            }
        }
    }
    for class in &class_order {
        if names.contains_key(class) {
            continue;
        }
        let best = &nodes[members[class][0]];
        let mut name = best.rule.name.clone();
        if taken.contains(&name) {
            let sanitized: String = best
                .source
                .id
                .chars()
                .map(|character| {
                    if character.is_ascii_alphanumeric() || character == '_' {
                        character
                    } else {
                        '_'
                    }
                })
                .collect();
            let base = format!("{}_from_{sanitized}", best.rule.name);
            name.clone_from(&base);
            let mut suffix = 2_usize;
            while taken.contains(&name) {
                name = format!("{base}_{suffix}");
                suffix += 1;
            }
            renamed.insert(*class);
        }
        taken.insert(name.clone());
        names.insert(*class, name);
    }

    let canonical =
        |source: &str, name: &str| -> &String { &names[&classes[index[&alias_of(source, name)]]] };
    let mut grammar = Grammar::new();
    let mut definitions = BTreeMap::new();
    for class in &class_order {
        let representative = &nodes[members[class][0]];
        let source = representative.source;
        let rename = |name: &str| {
            if source.has_rule(name) {
                canonical(source.id, name).clone()
            } else {
                name.to_owned()
            }
        };
        let renamed = renamed_rule(representative.rule, &rename);
        let form = normalize(&renamed.expr, &name_label)?;
        definitions.insert(
            *class,
            format!(
                "{}:{}{}",
                renamed.kind.as_str(),
                form.text,
                rule_fields(&renamed, &name_label)
            ),
        );
        grammar.add_rule(GrammarRule {
            name: names[class].clone(),
            expr: form.expr(),
            ..renamed
        });
    }
    let (declarations, declaration_conflicts) = merge_declarations(entry, &|source, name| {
        if source.has_rule(name) {
            canonical(source.id, name).clone()
        } else {
            name.to_owned()
        }
    });
    grammar.set_declarations(declarations);

    let mut start_classes = Vec::new();
    for source in entry {
        if let Some(start) = source.grammar.start_rule() {
            let class = classes[index[&alias_of(source.id, &start.name)]];
            if !start_classes.contains(&class) {
                start_classes.push(class);
            }
        }
    }
    if let Some(first) = start_classes.first() {
        grammar.set_start(names[first].clone());
    }
    let formats: BTreeSet<&str> = entry.iter().map(|source| source.format()).collect();
    grammar.source_format = if formats.len() == 1 {
        entry[0].grammar.source_format()
    } else {
        Some(GrammarFormat::MetaLanguage)
    };

    let identities: BTreeMap<String, String> = nodes
        .iter()
        .map(|node| {
            (
                node.alias.clone(),
                canonical(node.source.id, &node.rule.name).clone(),
            )
        })
        .collect();

    let mut decisions = Vec::new();
    for class in &class_order {
        let group = &members[class];
        let merged = group.len() > 1;
        decisions.push(GrammarMergeDecision {
            kind: if merged {
                GrammarMergeDecisionKind::Merged
            } else {
                GrammarMergeDecisionKind::KeptUnique
            },
            name: names[class].clone(),
            members: group
                .iter()
                .map(|&position| nodes[position].alias.clone())
                .collect(),
            basis: if merged {
                GRAMMAR_MERGE_METHOD
            } else {
                "no-equivalent-rule"
            }
            .to_owned(),
            definition: definitions.remove(class),
        });
        if renamed.contains(class) {
            decisions.push(GrammarMergeDecision {
                kind: GrammarMergeDecisionKind::RenamedForCollision,
                name: names[class].clone(),
                members: vec![nodes[group[0]].alias.clone()],
                basis: "precedence".to_owned(),
                definition: None,
            });
        }
    }

    let mut alternatives = Vec::new();
    if let Some((first, rest)) = start_classes.split_first()
        && !rest.is_empty()
    {
        alternatives.push(GrammarMergeAlternative {
            reason: GrammarMergeAlternativeReason::StartRule,
            name: names[first].clone(),
            options: rest.iter().map(|class| names[class].clone()).collect(),
        });
    }
    let mut by_name: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for class in &class_order {
        for &position in &members[class] {
            let list = by_name
                .entry(nodes[position].rule.name.as_str())
                .or_default();
            if !list.contains(class) {
                list.push(*class);
            }
        }
    }
    for (name, list) in by_name {
        if list.len() < 2 {
            continue;
        }
        let options: Vec<String> = list.iter().map(|class| names[class].clone()).collect();
        decisions.push(GrammarMergeDecision {
            kind: GrammarMergeDecisionKind::HomonymKeptDistinct,
            name: name.to_owned(),
            members: options.clone(),
            basis: "different-definitions".to_owned(),
            definition: None,
        });
        alternatives.push(GrammarMergeAlternative {
            reason: GrammarMergeAlternativeReason::DistinctMeaning,
            name: name.to_owned(),
            options,
        });
    }

    let nominations = nominate(&nodes, &classes, samples);
    for nomination in &nominations {
        if nomination.outcome != GrammarMergeNominationOutcome::Unproven {
            continue;
        }
        let first = identities[&nomination.members[0]].clone();
        let second = identities[&nomination.members[1]].clone();
        decisions.push(GrammarMergeDecision {
            kind: GrammarMergeDecisionKind::Uncertain,
            name: first.clone(),
            members: nomination.members.to_vec(),
            basis: nomination.basis.as_str().to_owned(),
            definition: None,
        });
        alternatives.push(GrammarMergeAlternative {
            reason: GrammarMergeAlternativeReason::UncertainMatch,
            name: nomination.basis.as_str().to_owned(),
            options: vec![first, second],
        });
    }
    for conflict in declaration_conflicts {
        decisions.push(GrammarMergeDecision {
            kind: GrammarMergeDecisionKind::DeclarationConflict,
            name: conflict.name.clone(),
            members: conflict.members.clone(),
            basis: conflict.basis.to_owned(),
            definition: None,
        });
        alternatives.push(GrammarMergeAlternative {
            reason: GrammarMergeAlternativeReason::DeclarationConflict,
            name: conflict.name,
            options: conflict.members,
        });
    }

    Ok(MergedGrammarGroup {
        key: group_key(language, edition),
        language: language.to_owned(),
        edition: edition.to_owned(),
        fingerprint,
        sources: entry.iter().map(|source| source.id.to_owned()).collect(),
        grammar,
        identities,
        decisions,
        nominations,
        alternatives,
    })
}

// Greatest structural bisimulation by partition refinement: rules start in one
// class per normalized shape and are split until every member of a class
// references members of the same classes in the same way. Class numbers are
// ranks of sorted signatures, so they do not depend on the input order.
fn refine(
    nodes: &[Node<'_>],
    index: &BTreeMap<String, usize>,
) -> Result<Vec<usize>, GrammarMergeError> {
    let mut classes = vec![0_usize; nodes.len()];
    let mut count = 0_usize;
    loop {
        let mut keys = Vec::with_capacity(nodes.len());
        for (position, node) in nodes.iter().enumerate() {
            let current = &classes;
            let internal =
                |name: &str| format!("#{}", current[index[&alias_of(node.source.id, name)]]);
            let label = source_label(node.source, &internal);
            let prefix = if count == 0 {
                String::new()
            } else {
                classes[position].to_string()
            };
            keys.push(format!(
                "{prefix}|{}:{}{}",
                node.rule.kind.as_str(),
                normalize(&node.rule.expr, &label)?.text,
                rule_fields(node.rule, &label)
            ));
        }
        let ranks: BTreeMap<&String, usize> = keys
            .iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .enumerate()
            .map(|(rank, key)| (key, rank))
            .collect();
        let size = ranks.len();
        classes = keys.iter().map(|key| ranks[key]).collect();
        if size == count {
            return Ok(classes);
        }
        count = size;
    }
}

fn nominate(
    nodes: &[Node<'_>],
    classes: &[usize],
    samples: &BTreeMap<String, Vec<String>>,
) -> Vec<GrammarMergeNomination> {
    let mut nominations = Vec::new();
    let mut seen = BTreeSet::new();
    let mut add = |basis: GrammarMergeNominationBasis, first: usize, second: usize| {
        if !seen.insert((basis.as_str(), first, second)) {
            return;
        }
        nominations.push(GrammarMergeNomination {
            basis,
            members: [nodes[first].alias.clone(), nodes[second].alias.clone()],
            outcome: if classes[first] == classes[second] {
                GrammarMergeNominationOutcome::Proven
            } else {
                GrammarMergeNominationOutcome::Unproven
            },
        });
    };
    let by_key = group_in_order(nodes.iter().enumerate().map(|(position, node)| {
        let key: String = node
            .rule
            .name
            .to_lowercase()
            .chars()
            .filter(|character| *character != '-' && *character != '_')
            .collect();
        (key, position)
    }));
    for group in by_key {
        for (offset, &left) in group.iter().enumerate() {
            for &right in &group[offset + 1..] {
                if nodes[left].rule.name != nodes[right].rule.name {
                    add(GrammarMergeNominationBasis::NameSimilarity, left, right);
                }
            }
        }
    }
    let by_samples = group_in_order(nodes.iter().enumerate().filter_map(|(position, node)| {
        samples.get(&node.alias).map(|values| {
            let key: Vec<String> = values.iter().map(|value| quote(value)).collect();
            (key.join(","), position)
        })
    }));
    for group in by_samples {
        for (offset, &left) in group.iter().enumerate() {
            for &right in &group[offset + 1..] {
                add(GrammarMergeNominationBasis::IdenticalSamples, left, right);
            }
        }
    }
    nominations
}

// Groups values by key, keeping the order in which keys first appear.
fn group_in_order(entries: impl Iterator<Item = (String, usize)>) -> Vec<Vec<usize>> {
    let mut slots: BTreeMap<String, usize> = BTreeMap::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for (key, value) in entries {
        let slot = *slots.entry(key).or_insert_with(|| {
            groups.push(Vec::new());
            groups.len() - 1
        });
        groups[slot].push(value);
    }
    groups
}

pub(super) fn edition_alternatives(groups: &[MergedGrammarGroup]) -> Vec<GrammarMergeAlternative> {
    let mut editions: Vec<(String, Vec<String>)> = Vec::new();
    for group in groups {
        match editions.last_mut() {
            Some((language, list)) if *language == group.language => {
                list.push(group.edition.clone());
            }
            _ => editions.push((group.language.clone(), vec![group.edition.clone()])),
        }
    }
    editions
        .into_iter()
        .filter(|(_, list)| list.len() > 1)
        .map(|(language, options)| GrammarMergeAlternative {
            reason: GrammarMergeAlternativeReason::Edition,
            name: language,
            options,
        })
        .collect()
}

pub(super) fn required_failures(
    required: &BTreeSet<(String, String)>,
    groups: &[MergedGrammarGroup],
) -> Vec<GrammarMergeFailure> {
    let mut group_of = BTreeMap::new();
    for (position, group) in groups.iter().enumerate() {
        for id in &group.sources {
            group_of.insert(id.as_str(), position);
        }
    }
    let lookup = |alias: &String| {
        group_of.get(source_of(alias)).and_then(|&position| {
            groups[position]
                .identities
                .get(alias)
                .map(|name| (position, name))
        })
    };
    let mut failures = Vec::new();
    for (first, second) in required {
        let reason = match (lookup(first), lookup(second)) {
            (Some((left_group, left)), Some((right_group, right))) => {
                if left_group != right_group {
                    Some(GrammarMergeFailureReason::DifferentLanguageOrEdition)
                } else if left != right {
                    Some(GrammarMergeFailureReason::NotProven)
                } else {
                    None
                }
            }
            _ => Some(GrammarMergeFailureReason::UnknownRule),
        };
        if let Some(reason) = reason {
            failures.push(GrammarMergeFailure {
                kind: GrammarMergeFailureKind::UnresolvedRequiredEquivalence,
                members: [first.clone(), second.clone()],
                reason,
            });
        }
    }
    failures
}
