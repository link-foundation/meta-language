//! Network traversal and snapshotting only; proof acceptance uses generated rules.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};

use super::metadata;
use super::relation_rules::{
    accept_formal_relation_edge_identity, accept_formal_relation_identity,
    accept_formal_relation_label_identity,
};
use crate::global_concept_rules::verify_global_concept_correspondence;
use crate::link_network::{LinkId, LinkMetadata, LinkNetwork, LinkType};

#[derive(Clone)]
pub(super) struct Field {
    pub label: String,
    pub child: LinkId,
    relation: LinkId,
    label_id: Option<LinkId>,
}

struct Entry {
    metadata: LinkMetadata,
    references: Vec<LinkId>,
    fields: Vec<Field>,
}

pub(super) struct Graph {
    pub shapes: Vec<String>,
    pub offsets: Vec<usize>,
    pub edges: Vec<usize>,
    entries: Vec<Entry>,
    pub indices: BTreeMap<LinkId, usize>,
}

impl Graph {
    pub fn same_shape(&self, other: &Self) -> bool {
        self.shapes == other.shapes && self.offsets == other.offsets && self.edges == other.edges
    }
}

pub(super) fn exclude_binding_names(
    network: &LinkNetwork,
    body: LinkId,
    exclusions: &mut BTreeSet<(LinkId, String)>,
    bindings: &[LinkId],
) -> Result<(), String> {
    let semantic = graph(network, body, exclusions)?;
    let mut protected = BTreeSet::new();
    let mut pending = Vec::new();
    for entry in &semantic.entries {
        let record = entry
            .fields
            .iter()
            .find(|field| field.label == "construct kind")
            .and_then(|field| network.link(field.child))
            .is_some_and(|link| link.metadata().definition() == Some("record"));
        for field in &entry.fields {
            if matches!(
                field.label.as_str(),
                "type" | "domain" | "return type" | "from" | "to" | "constructors"
            ) {
                pending.push(field.child);
            }
            if record && matches!(field.label.as_str(), "fields" | "entries") {
                let list = network
                    .link(field.child)
                    .ok_or("a formal record has no field list")?;
                protected.extend(list.references().iter().copied());
            }
        }
    }
    let mut visited_types = BTreeSet::new();
    let mut at = 0;
    while at < pending.len() {
        let id = pending[at];
        at += 1;
        if !visited_types.insert(id) {
            continue;
        }
        protected.insert(id);
        if let Some(index) = semantic.indices.get(&id) {
            let entry = &semantic.entries[*index];
            pending.extend(entry.references.iter().copied());
            pending.extend(entry.fields.iter().map(|field| field.child));
        }
    }
    for binding in bindings {
        if semantic.indices.contains_key(binding) && !protected.contains(binding) {
            exclusions.insert((*binding, "name".to_owned()));
        }
    }
    Ok(())
}

pub(super) fn data(network: &mut LinkNetwork, value: &Value) -> Result<LinkId, String> {
    match value {
        Value::Array(values) => {
            let references = values
                .iter()
                .map(|value| data(network, value))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(network.insert_dynamic_link(&references, metadata("formal declaration list", None)))
        }
        Value::Object(values) => {
            let id = network.insert_link([], metadata("formal declaration", None));
            let sorted: BTreeMap<_, _> = values.iter().collect();
            for (name, value) in sorted {
                let child = data(network, value)?;
                network.insert_field(id, name, child);
            }
            Ok(id)
        }
        Value::String(value) => {
            Ok(network.insert_link([], metadata("formal declaration string", Some(value))))
        }
        Value::Number(value) => Ok(network.insert_link(
            [],
            metadata("formal declaration number", Some(&value.to_string())),
        )),
        Value::Bool(value) => Ok(network.insert_link(
            [],
            metadata(
                "formal declaration boolean",
                Some(if *value { "true" } else { "false" }),
            ),
        )),
        Value::Null => Err("formal primitive declarations contain only linked data".to_owned()),
    }
}

pub(super) fn fields(network: &LinkNetwork) -> BTreeMap<LinkId, Vec<Field>> {
    let mut result: BTreeMap<LinkId, Vec<Field>> = BTreeMap::new();
    for link in network.links() {
        if link.metadata().link_type() != Some(LinkType::Field) {
            continue;
        }
        let references = link.references();
        if references.is_empty() || references.len() == 1 && references[0] == link.id() {
            continue;
        }
        let label_id = references.get(1).copied();
        let label = label_id
            .and_then(|id| network.link(id))
            .and_then(|label| label.metadata().term())
            .unwrap_or_default();
        let relations = result.entry(references[0]).or_default();
        relations.push(Field {
            label: label.to_owned(),
            child: references.get(2).copied().unwrap_or(references[0]),
            relation: link.id(),
            label_id,
        });
    }
    for relations in result.values_mut() {
        relations.sort_by(|a, b| a.label.encode_utf16().cmp(b.label.encode_utf16()));
    }
    result
}

fn admitted_relation(network: &LinkNetwork, field: &Field, same_label_identity: bool) -> bool {
    let label = field.label_id.and_then(|id| network.link(id));
    let value = label.map(|link| link.metadata());
    let edge = network
        .link(field.relation)
        .expect("indexed relation exists");
    let label_admitted = accept_formal_relation_label_identity(
        if value.is_some_and(|meta| meta.link_type() == Some(LinkType::Field)) {
            "Field".to_owned()
        } else {
            String::new()
        },
        value
            .and_then(|meta| meta.definition())
            .unwrap_or_default()
            .to_owned(),
        value.is_some_and(|meta| meta.term().is_some()),
        value.is_some_and(|meta| meta.language().is_some()),
        value.is_some_and(LinkMetadata::is_named),
        label.is_some_and(|link| {
            link.references().len() == 1 && Some(link.references()[0]) == field.label_id
        }),
    );
    let relation_admitted = accept_formal_relation_edge_identity(
        if edge.metadata().link_type() == Some(LinkType::Field) {
            "Field".to_owned()
        } else {
            String::new()
        },
        edge.metadata().language().is_some(),
        edge.metadata().term().is_some(),
        edge.metadata().definition().is_some(),
        edge.metadata().is_named(),
        if edge.references().len() == 3 {
            3.0
        } else {
            0.0
        },
    );
    accept_formal_relation_identity(label_admitted, relation_admitted, same_label_identity)
}

pub(super) fn graph(
    network: &LinkNetwork,
    root: LinkId,
    exclusions: &BTreeSet<(LinkId, String)>,
) -> Result<Graph, String> {
    let relations = fields(network);
    let mut pending = vec![root];
    let mut indices = BTreeMap::from([(root, 0)]);
    let mut shapes = Vec::new();
    let mut offsets = vec![0];
    let mut edges = Vec::new();
    let mut entries = Vec::new();
    let mut label_identities = BTreeMap::new();
    let mut at = 0;
    while at < pending.len() {
        if pending.len() > 100_000 {
            return Err("the formal definition exceeds its graph resource limit".to_owned());
        }
        let id = pending[at];
        let link = network
            .link(id)
            .ok_or("a formal definition contains a dangling reference")?;
        let value = link.metadata();
        let positional = link.references();
        let outgoing = relations.get(&id).map_or(&[][..], Vec::as_slice);
        let mut labels = BTreeSet::new();
        for field in outgoing {
            let same_identity = label_identities
                .get(&field.label)
                .is_none_or(|previous| Some(*previous) == field.label_id);
            if !admitted_relation(network, field, same_identity) {
                return Err(
                    "a formal definition contains an unsupported relation identity".to_owned(),
                );
            }
            if !labels.insert(&field.label) {
                return Err("a formal definition contains a duplicate relation label".to_owned());
            }
            label_identities.insert(
                field.label.clone(),
                field.label_id.expect("admitted label exists"),
            );
        }
        let selected = outgoing
            .iter()
            .filter(|relation| !exclusions.contains(&(id, relation.label.clone())))
            .cloned()
            .collect::<Vec<_>>();
        shapes.push(
            json!([
                value.link_type().map(|kind| match kind {
                    LinkType::Link => "Dynamic".to_owned(),
                    LinkType::Token => "SourceToken".to_owned(),
                    _ => format!("{kind:?}"),
                }),
                value.language(),
                value.term(),
                value.definition(),
                value.is_named(),
                positional.len(),
                selected
                    .iter()
                    .map(|field| &field.label)
                    .collect::<Vec<_>>()
            ])
            .to_string(),
        );
        for child in positional
            .iter()
            .copied()
            .chain(selected.iter().map(|field| field.child))
        {
            let index = *indices.entry(child).or_insert_with(|| {
                let index = pending.len();
                pending.push(child);
                index
            });
            edges.push(index);
        }
        entries.push(Entry {
            metadata: value.clone(),
            references: positional.to_vec(),
            fields: selected,
        });
        offsets.push(edges.len());
        at += 1;
    }
    Ok(Graph {
        shapes,
        offsets,
        edges,
        entries,
        indices,
    })
}

// Graph indices are bounded to 100,000 nodes before conversion to the
// generated verifier's JavaScript Number representation.
#[allow(clippy::cast_precision_loss)]
pub(super) fn verify_pairs(first: &Graph, second: &Graph, pairs: &[(usize, usize)]) -> bool {
    let mut shapes = first.shapes.clone();
    shapes.extend(second.shapes.iter().cloned());
    let meanings = vec!["declared shared formal primitives".to_owned(); shapes.len()];
    let offsets = first
        .offsets
        .iter()
        .copied()
        .chain(
            second
                .offsets
                .iter()
                .skip(1)
                .map(|offset| offset + first.edges.len()),
        )
        .map(|offset| offset as f64)
        .collect();
    let edges = first
        .edges
        .iter()
        .copied()
        .chain(second.edges.iter().map(|edge| edge + first.shapes.len()))
        .map(|edge| edge as f64)
        .collect();
    verify_global_concept_correspondence(
        shapes,
        meanings,
        offsets,
        edges,
        pairs.iter().map(|(a, _)| *a as f64).collect(),
        pairs
            .iter()
            .map(|(_, b)| (b + first.shapes.len()) as f64)
            .collect(),
    )
}

pub(super) fn correspondence(first: &Graph, second: &Graph) -> Option<Vec<(usize, usize)>> {
    let mut pairs = Vec::new();
    let mut forward = BTreeMap::new();
    let mut reverse = BTreeMap::new();
    let mut pending = vec![(0, 0)];
    let mut at = 0;
    while at < pending.len() {
        let (one, other) = pending[at];
        at += 1;
        if forward.contains_key(&one) || reverse.contains_key(&other) {
            if forward.get(&one) != Some(&other) || reverse.get(&other) != Some(&one) {
                return None;
            }
            continue;
        }
        if first.shapes[one] != second.shapes[other] {
            return None;
        }
        forward.insert(one, other);
        reverse.insert(other, one);
        pairs.push((one, other));
        let count = first.offsets[one + 1] - first.offsets[one];
        if count != second.offsets[other + 1] - second.offsets[other] {
            return None;
        }
        for edge in 0..count {
            pending.push((
                first.edges[first.offsets[one] + edge],
                second.edges[second.offsets[other] + edge],
            ));
        }
    }
    if pairs.len() != first.shapes.len() || pairs.len() != second.shapes.len() {
        return None;
    }
    verify_pairs(first, second, &pairs).then_some(pairs)
}

pub(super) fn snapshot(network: &mut LinkNetwork, source: &Graph) -> Option<Vec<LinkId>> {
    let ids = source
        .entries
        .iter()
        .map(|entry| network.insert_link([], entry.metadata.clone()))
        .collect::<Vec<_>>();
    for (index, entry) in source.entries.iter().enumerate() {
        let references = entry
            .references
            .iter()
            .map(|id| source.indices.get(id).map(|index| ids[*index]))
            .collect::<Option<Vec<_>>>()?;
        if !network.set_references(ids[index], &references) {
            return None;
        }
        for field in &entry.fields {
            network.insert_field(
                ids[index],
                &field.label,
                ids[*source.indices.get(&field.child)?],
            );
        }
    }
    Some(ids)
}
