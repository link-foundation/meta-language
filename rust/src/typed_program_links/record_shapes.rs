//! Ordered Links layout adapters; the shared generated rules own admission.
use super::{Result, TypedProgramLinks, error, typed_program_links_schema};
use crate::{Link, LinkId, LinkMetadata, LinkNetwork, LinkType};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;
pub(super) type Fields = HashMap<LinkId, Vec<(String, LinkId)>>;
const MODEL: &str = "ordered typed program record fields 1";
const ROOT_FIELD: &str = "record layout";
const PROFILE_TERM: &str = "typed program record layout";
const SHAPE_TERM: &str = "typed program record field shape";
fn failure(message: &str) -> super::TypedProgramLinksError {
    error(format!("typed program record layout: {message}"))
}
fn profile() -> Result<()> {
    static VALUE: OnceLock<Result<()>> = OnceLock::new();
    VALUE
        .get_or_init(|| {
            let value = crate::read_structured_data_lino(include_str!(
                "../data/typed-program-record-shapes.lino"
            ))
            .map_err(|value| error(value.to_string()))?;
            if value["schemaVersion"] != 1
                || value["model"] != MODEL
                || value["rootField"] != ROOT_FIELD
                || value["profileTerm"] != PROFILE_TERM
                || value["shapeTerm"] != SHAPE_TERM
                || value["recordOccurrences"] != false
            {
                return Err(failure("unknown qualified layout profile"));
            }
            Ok(())
        })
        .clone()
}
fn language() -> &'static str {
    typed_program_links_schema()["language"]
        .as_str()
        .expect("registered typed schema language")
}
fn metadata(term: &str) -> LinkMetadata {
    LinkMetadata::new()
        .with_link_type(LinkType::Semantic)
        .with_language(language())
        .with_term(term)
}
fn qualified_field(point: &Link) -> bool {
    let value = point.metadata();
    value.link_type() == Some(LinkType::Field)
        && value.is_named()
        && value.language().is_none()
        && value.definition() == Some("A field label names a relation between links.")
        && point.references().len() == 1
        && point.references()[0] == point.id()
}
pub(super) fn selected(network: &LinkNetwork, root: LinkId, fields: &Fields) -> Result<bool> {
    let values = fields
        .get(&root)
        .into_iter()
        .flatten()
        .filter(|(label, _)| label == ROOT_FIELD)
        .collect::<Vec<_>>();
    if values.is_empty() {
        return Ok(false);
    }
    profile()?;
    let id = values[0].1;
    let marker = network.link(id);
    let relation=network.links().find(|link|link.metadata().link_type()==Some(LinkType::Field)&&matches!(link.references(),[parent,label,child]if *parent==root&&*child==id&&network.link(*label).and_then(|point|point.metadata().term())==Some(ROOT_FIELD)));
    let qualified_relation = relation.is_some_and(|edge| {
        let value = edge.metadata();
        let label = network.link(edge.references()[1]);
        value.language().is_none()
            && value.term().is_none()
            && value.definition().is_none()
            && !value.is_named()
            && label
                .is_some_and(|point| qualified_field(point) && !fields.contains_key(&point.id()))
    });
    let qualified_marker = marker.is_some_and(|point| {
        let value = point.metadata();
        value.link_type() == Some(LinkType::Semantic)
            && !value.is_named()
            && value.language() == Some(language())
            && value.term() == Some(PROFILE_TERM)
            && value.definition() == Some(MODEL)
    });
    let count = serde_json::Number::from(values.len())
        .as_f64()
        .ok_or_else(|| failure("record coordinates require exact Number values"))?;
    let references = marker.map_or(-1.0, |point| {
        serde_json::Number::from(point.references().len())
            .as_f64()
            .unwrap_or(f64::INFINITY)
    });
    let message=crate::translation::typed_record_shape_rules::read_typed_program_record_shape_selection_error(count,qualified_relation&&qualified_marker,references,fields.contains_key(&id));
    if !message.is_empty() {
        return Err(failure(&message));
    }
    Ok(true)
}
pub(super) fn read_fields(
    network: &LinkNetwork,
    record: LinkId,
    is_selected: bool,
    fields: &Fields,
) -> Result<Vec<(String, LinkId)>> {
    let record = network
        .link(record)
        .ok_or_else(|| failure("dangling record identity"))?;
    let references = record.references();
    let shape = references.first().and_then(|id| network.link(*id));
    let roles = shape.map_or(&[][..], |shape| shape.references());
    if references.len() > 100001 || roles.len() > 100000 {
        return Err(failure(
            "ordered program record roles must match every actual value coordinate",
        ));
    }
    let points = roles.iter().map(|id| network.link(*id)).collect::<Vec<_>>();
    let mut units = 0usize;
    for point in &points {
        let size = point
            .and_then(|point| point.metadata().term())
            .unwrap_or("")
            .encode_utf16()
            .take(1000001)
            .count();
        units = units.saturating_add(size);
        let observed = serde_json::Number::from(units)
            .as_f64()
            .unwrap_or(f64::INFINITY);
        let message =
            crate::translation::typed_record_shape_rules::read_typed_program_record_role_text_error(
                observed,
                size > 1000000,
            );
        if !message.is_empty() {
            return Err(failure(&message));
        }
    }
    let names = points
        .iter()
        .map(|point| {
            point
                .and_then(|point| point.metadata().term())
                .unwrap_or("")
                .to_owned()
        })
        .collect::<Vec<_>>();
    let kinds = points
        .iter()
        .map(|point| point.is_some_and(qualified_field))
        .collect::<Vec<_>>();
    let self_points = points
        .iter()
        .zip(roles)
        .map(|(point, id)| {
            point.is_some_and(|point| point.references().len() == 1 && point.references()[0] == *id)
        })
        .collect::<Vec<_>>();
    let named = points
        .iter()
        .map(|point| point.is_some_and(|point| fields.contains_key(&point.id())))
        .collect::<Vec<_>>();
    let observed = serde_json::Number::from(references.len())
        .as_f64()
        .ok_or_else(|| failure("record coordinates require exact Number values"))?;
    let message =
        crate::translation::typed_record_shape_rules::read_typed_program_record_shape_error(
            is_selected,
            observed,
            fields.contains_key(&record.id()),
            shape.is_some_and(|shape| fields.contains_key(&shape.id())),
            shape.is_some_and(|shape| {
                shape.metadata().link_type() == Some(LinkType::Semantic)
                    && !shape.metadata().is_named()
            }),
            shape
                .and_then(|shape| shape.metadata().language())
                .unwrap_or(""),
            shape
                .and_then(|shape| shape.metadata().term())
                .unwrap_or(""),
            shape
                .and_then(|shape| shape.metadata().definition())
                .unwrap_or(""),
            language(),
            SHAPE_TERM,
            &names,
            &kinds,
            &self_points,
            &named,
        );
    if !message.is_empty() {
        return Err(failure(&message));
    }
    Ok(names
        .into_iter()
        .zip(references.iter().skip(1).copied())
        .collect())
}
pub(super) fn validate_selection(selection: &str) -> Result<()> {
    profile()?;
    if selection != MODEL {
        return Err(error("unregistered typed program record shape profile"));
    }
    Ok(())
}
pub(super) fn compact(mut value: TypedProgramLinks, selection: &str) -> Result<TypedProgramLinks> {
    validate_selection(selection)?;
    let network = &mut value.network;
    let record_term = typed_program_links_schema()["containers"]["record"]
        .as_str()
        .expect("registered record term");
    let records = network
        .links()
        .filter(|link| {
            link.metadata().language() == Some(language())
                && link.metadata().term() == Some(record_term)
        })
        .map(|link| link.id())
        .collect::<HashSet<_>>();
    let mut groups: HashMap<LinkId, Vec<(LinkId, LinkId, LinkId)>> = HashMap::new();
    let mut removed = HashSet::new();
    for link in network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
    {
        if let [parent, label, child] = link.references() {
            if records.contains(parent) {
                groups
                    .entry(*parent)
                    .or_default()
                    .push((link.id(), *label, *child));
                removed.insert(link.id());
            }
        }
    }
    for link in network.links() {
        if removed.contains(&link.id()) {
            continue;
        }
        if link.references().iter().any(|id| removed.contains(id)) {
            return Err(failure("a field relation has external identity references"));
        }
    }
    for id in &records {
        if !network
            .link(*id)
            .expect("owned record")
            .references()
            .is_empty()
        {
            return Err(failure(
                "legacy records must have complete named fields before compaction",
            ));
        }
    }
    let mut records = records.into_iter().collect::<Vec<_>>();
    records.sort();
    let mut shapes: HashMap<Vec<LinkId>, LinkId> = HashMap::new();
    for record in records {
        let values = groups.get(&record).cloned().unwrap_or_default();
        let roles = values.iter().map(|(_, role, _)| *role).collect::<Vec<_>>();
        let shape = if let Some(id) = shapes.get(&roles) {
            *id
        } else {
            let id = network.insert_link([], metadata(SHAPE_TERM));
            network.set_references(id, &roles);
            shapes.insert(roles, id);
            id
        };
        let mut references = vec![shape];
        references.extend(values.iter().map(|(_, _, child)| *child));
        network.set_references(record, &references);
    }
    for id in removed {
        network.links.remove(&id);
    }
    let marker = network.insert_link([], metadata(PROFILE_TERM).with_definition(MODEL));
    network.insert_field(value.root, ROOT_FIELD, marker);
    Ok(value)
}

fn field_map(network: &LinkNetwork) -> Result<Fields> {
    let mut fields: Fields = HashMap::new();
    for link in network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
    {
        if let [parent, label, child] = link.references() {
            let label = network
                .link(*label)
                .and_then(|point| point.metadata().term())
                .ok_or_else(|| failure("a linked program contains a dangling field label"))?;
            fields
                .entry(*parent)
                .or_default()
                .push((label.to_owned(), *child));
        }
    }
    Ok(fields)
}
pub(super) fn expand(original: &LinkNetwork, root: LinkId) -> Result<LinkNetwork> {
    let fields = field_map(original)?;
    if !selected(original, root, &fields)? {
        return Ok(original.clone());
    }
    let mut network = original.clone();
    let mut pending = vec![root];
    let mut seen = HashSet::new();
    let mut records = Vec::new();
    let mut shapes = HashSet::new();
    let mut index = 0;
    let record_term = typed_program_links_schema()["containers"]["record"]
        .as_str()
        .expect("registered record term");
    while index < pending.len() {
        let id = pending[index];
        index += 1;
        if seen.contains(&id) {
            continue;
        }
        if seen.len() >= 100000 || pending.len() > 1000000 {
            return Err(failure(
                "record expansion exceeds its structural resource budget",
            ));
        }
        seen.insert(id);
        let link = network
            .link(id)
            .ok_or_else(|| failure("a record layout contains a dangling reference"))?;
        pending.extend_from_slice(link.references());
        pending.extend(
            fields
                .get(&id)
                .into_iter()
                .flatten()
                .map(|(_, child)| *child),
        );
        if link.metadata().language() == Some(language())
            && link.metadata().term() == Some(record_term)
            && !link.references().is_empty()
        {
            let values = read_fields(&network, id, true, &fields)?;
            shapes.insert(link.references()[0]);
            records.push((id, values));
        }
    }
    for (record, values) in records {
        network.set_references(record, &[]);
        for (label, child) in values {
            network.insert_field(record, &label, child);
        }
    }
    for (label, child) in fields.get(&root).into_iter().flatten() {
        if label == ROOT_FIELD {
            shapes.insert(*child);
        }
    }
    let removed = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
        .filter_map(|link| match link.references() {
            [parent, label, _]
                if *parent == root
                    && network
                        .link(*label)
                        .and_then(|point| point.metadata().term())
                        == Some(ROOT_FIELD) =>
            {
                Some(link.id())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    for id in removed {
        network.links.remove(&id);
    }
    let referenced = network
        .links()
        .flat_map(|link| link.references().iter().copied())
        .collect::<HashSet<_>>();
    for id in shapes {
        if !referenced.contains(&id) {
            network.links.remove(&id);
        }
    }
    Ok(network)
}
