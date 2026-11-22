//! Native transport for exact, source-qualified concrete Rust item selection.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::rust_concrete_source_rules::{
    read_rust_concrete_item_kind, test_rust_concrete_selectable_item_kind,
    test_rust_concrete_source_containment, test_rust_concrete_source_enclosure,
};
use crate::rust_source_project::limit;
use crate::translation::{
    Language,
    lexer::{TokenKind, tokenize},
};
use crate::{
    LinkId, LinkNetwork, LinkType, ProgramConcreteSyntaxFact, read_program_concrete_syntax_facts,
};

const MODEL: &str = "Rust actual concrete source items 1";
type Fields = BTreeMap<LinkId, BTreeMap<String, Vec<LinkId>>>;

fn failure(reason: &str) -> String {
    format!("Rust concrete source item: {reason}")
}

fn term(fact: &ProgramConcreteSyntaxFact) -> &str {
    fact.term.as_deref().unwrap_or_default()
}

fn units(fact: &ProgramConcreteSyntaxFact) -> Value {
    json!({"start":fact.start,"end":fact.end})
}

pub(crate) fn source_unit(offset: usize) -> f64 {
    serde_json::Number::from(offset)
        .as_f64()
        .expect("source offsets fit in a Number")
}

fn inside(parent: &ProgramConcreteSyntaxFact, child: &ProgramConcreteSyntaxFact) -> bool {
    test_rust_concrete_source_containment(
        source_unit(parent.start),
        source_unit(parent.end),
        source_unit(child.start),
        source_unit(child.end),
    )
}

fn text<'a>(source: &'a str, fact: &ProgramConcreteSyntaxFact) -> &'a str {
    &source[fact.byte_range.start()..fact.byte_range.end()]
}

pub(crate) fn concrete_fields(
    network: &LinkNetwork,
) -> impl Iterator<Item = (LinkId, &str, LinkId)> {
    network.links().filter_map(|link| {
        let [parent, label, child] = link.references() else {
            return None;
        };
        if link.metadata().link_type() != Some(LinkType::Field) {
            return None;
        }
        Some((*parent, network.link(*label)?.metadata().term()?, *child))
    })
}

struct ConcreteItemSelection<'a> {
    source: &'a str,
    facts: Vec<ProgramConcreteSyntaxFact>,
    by_id: BTreeMap<LinkId, usize>,
    parents: BTreeMap<LinkId, Vec<LinkId>>,
    fields: Fields,
    remaining: usize,
}

impl<'a> ConcreteItemSelection<'a> {
    fn new(network: &LinkNetwork, source: &'a str, options: &Value) -> Result<Self, String> {
        let node_limit = limit(options, "nodeLimit", 1_000_000)?;
        let field_limit = limit(options, "fieldLimit", 4_000_000)?;
        if network.links().take(node_limit.saturating_add(1)).count() > node_limit {
            return Err(failure(
                "the supplied current graph exceeds its node budget",
            ));
        }
        let facts = read_program_concrete_syntax_facts(network, source)
            .map_err(|error| failure(&error.to_string()))?;
        let by_id = facts
            .iter()
            .enumerate()
            .map(|(index, fact)| (fact.link_id, index))
            .collect();
        let mut selection = Self {
            source,
            facts,
            by_id,
            parents: BTreeMap::new(),
            fields: BTreeMap::new(),
            remaining: limit(options, "workLimit", 8_000_000)?,
        };
        for link in network.links() {
            let Some(&parent) = selection.by_id.get(&link.id()) else {
                continue;
            };
            for reference in link.references() {
                selection.charge(1)?;
                if let Some(&child) = selection.by_id.get(reference)
                    && inside(&selection.facts[parent], &selection.facts[child])
                {
                    selection
                        .parents
                        .entry(*reference)
                        .or_default()
                        .push(link.id());
                }
            }
        }
        for (index, (parent, label, child)) in concrete_fields(network).enumerate() {
            if index >= field_limit {
                return Err(failure(
                    "the supplied graph exceeds its field relation budget",
                ));
            }
            if selection.by_id.contains_key(&parent) {
                selection
                    .fields
                    .entry(parent)
                    .or_default()
                    .entry(label.to_owned())
                    .or_default()
                    .push(child);
            }
        }
        Ok(selection)
    }

    fn charge(&mut self, amount: usize) -> Result<(), String> {
        self.remaining = self
            .remaining
            .checked_sub(amount)
            .ok_or_else(|| failure("source selection exceeds its bounded inspection work"))?;
        Ok(())
    }

    fn field(&self, parent: usize, label: &str, required: bool) -> Result<Option<usize>, String> {
        let parent = &self.facts[parent];
        let children = self
            .fields
            .get(&parent.link_id)
            .and_then(|fields| fields.get(label))
            .map_or(&[][..], Vec::as_slice);
        let child = match children {
            [] if !required => return Ok(None),
            [child] => *child,
            _ => {
                return Err(failure(&format!(
                    "an actual {label} field must have one unambiguous CST owner"
                )));
            }
        };
        let index = self
            .by_id
            .get(&child)
            .copied()
            .filter(|index| {
                inside(parent, &self.facts[*index])
                    && self
                        .parents
                        .get(&child)
                        .is_some_and(|owners| owners.contains(&parent.link_id))
            })
            .ok_or_else(|| {
                failure(&format!(
                    "an actual {label} field must remain inside its owner"
                ))
            })?;
        Ok(Some(index))
    }

    fn required_field(&self, parent: usize, label: &str) -> Result<usize, String> {
        self.field(parent, label, true)?
            .ok_or_else(|| failure("missing required CST field"))
    }

    fn name(&self, parent: usize) -> Result<(usize, &'a str), String> {
        let index = self.required_field(parent, "name")?;
        let name = &self.facts[index];
        if !matches!(term(name), "identifier" | "type_identifier") {
            return Err(failure(
                "an item name must retain its actual identifier field",
            ));
        }
        let source_name = text(self.source, name);
        Ok((index, source_name.strip_prefix("r#").unwrap_or(source_name)))
    }

    fn ancestor(&mut self, parent: LinkId, child: LinkId) -> Result<bool, String> {
        let mut pending = vec![child];
        let mut seen = BTreeSet::new();
        while let Some(node) = pending.pop() {
            self.charge(1)?;
            if !seen.insert(node) {
                continue;
            }
            let owners = self.parents.get(&node).map_or(&[][..], Vec::as_slice);
            for &owner in owners {
                if self.remaining == 0 {
                    return Err(failure(
                        "source selection exceeds its bounded inspection work",
                    ));
                }
                self.remaining -= 1;
                if owner == parent {
                    return Ok(true);
                }
                pending.push(owner);
            }
        }
        Ok(false)
    }

    fn parents(&mut self, index: usize, owner_term: &str) -> Result<Vec<usize>, String> {
        let mut parents = Vec::new();
        let owner_kind = read_rust_concrete_item_kind(owner_term);
        for candidate in 0..self.facts.len() {
            self.charge(1)?;
            let parent = &self.facts[candidate];
            let child = &self.facts[index];
            if read_rust_concrete_item_kind(term(parent))
                == read_rust_concrete_item_kind(owner_term)
                && test_rust_concrete_source_enclosure(
                    source_unit(parent.start),
                    source_unit(parent.end),
                    source_unit(child.start),
                    source_unit(child.end),
                )
                && self.ancestor(parent.link_id, child.link_id)?
            {
                parents.push(candidate);
            }
        }
        parents.sort_by_key(|index| {
            let fact = &self.facts[*index];
            (fact.start, std::cmp::Reverse(fact.end))
        });
        Ok(parents)
    }

    fn owners(&mut self, index: usize) -> Result<(Vec<Value>, Value), String> {
        let mut modules = Vec::new();
        for owner in self.parents(index, "module_item")? {
            let body = self.required_field(owner, "body")?;
            if !inside(&self.facts[body], &self.facts[index]) {
                return Err(failure(
                    "an inline module must actually contain the selected source item",
                ));
            }
            let (name, value) = self.name(owner)?;
            let parent = &self.facts[owner];
            modules.push(
                json!({"name":value,"sourceLink":parent.link_id.as_u64(),"range":units(parent),
                "nameLink":self.facts[name].link_id.as_u64()}),
            );
        }
        let implementation = match self.parents(index, "implementation_item")?.as_slice() {
            [] => Value::Null,
            [owner] => {
                let body = self.required_field(*owner, "body")?;
                if !inside(&self.facts[body], &self.facts[index]) {
                    return Err(failure(
                        "the actual implementation body must contain the selected source item",
                    ));
                }
                let owner_type = &self.facts[self.required_field(*owner, "type")?];
                let parent = &self.facts[*owner];
                let mut value = json!({"sourceLink":parent.link_id.as_u64(),"range":units(parent),
                    "type":text(self.source,owner_type),"typeLink":owner_type.link_id.as_u64(),"typeRange":units(owner_type)});
                if let Some(trait_index) = self.field(*owner, "trait", false)? {
                    let fact = &self.facts[trait_index];
                    value["trait"] = json!(text(self.source, fact));
                    value["traitLink"] = json!(fact.link_id.as_u64());
                }
                value
            }
            _ => {
                return Err(failure(
                    "nested implementation ownership requires its separate source interpretation",
                ));
            }
        };
        Ok((modules, implementation))
    }

    fn annotations(&mut self, index: usize) -> Result<(usize, usize, Vec<Value>), String> {
        let mut preceding = Vec::new();
        for candidate in 0..self.facts.len() {
            self.charge(1)?;
            if term(&self.facts[candidate]) == "attribute_item"
                && self.facts[candidate].end <= self.facts[index].start
            {
                preceding.push(candidate);
            }
        }
        preceding.sort_by_key(|index| {
            let fact = &self.facts[*index];
            (std::cmp::Reverse(fact.end), std::cmp::Reverse(fact.start))
        });
        let mut start = self.facts[index].start;
        let mut byte_start = self.facts[index].byte_range.start();
        let mut annotations = Vec::new();
        for candidate in preceding {
            let end = self.facts[candidate].end;
            self.charge(start.saturating_sub(end).saturating_add(1))?;
            let fact = &self.facts[candidate];
            let Some(gap) = self.source.get(fact.byte_range.end()..byte_start) else {
                break;
            };
            let tokens =
                tokenize(gap, Language::Rust).map_err(|error| failure(&error.to_string()))?;
            if tokens
                .tokens
                .iter()
                .any(|token| token.kind != TokenKind::Eof)
            {
                break;
            }
            annotations.push(self.annotation(fact, "leading"));
            start = fact.start;
            byte_start = fact.byte_range.start();
        }
        annotations.reverse();
        for candidate in 0..self.facts.len() {
            self.charge(1)?;
            let fact = &self.facts[candidate];
            if matches!(term(fact), "attribute_item" | "inner_attribute_item")
                && inside(&self.facts[index], fact)
            {
                annotations.push(self.annotation(fact, "enclosed"));
            }
        }
        Ok((start, byte_start, annotations))
    }

    fn annotation(&self, fact: &ProgramConcreteSyntaxFact, ownership: &str) -> Value {
        json!({"sourceLink":fact.link_id.as_u64(),"kind":term(fact),"ownership":ownership,
            "range":units(fact),"text":text(self.source,fact)})
    }
}

/// Capture named source items using their actual Rust concrete syntax links.
///
/// # Errors
/// Refuses foreign source, ambiguous selectors, invalid field ownership and
/// exhaustion of the caller's source, graph, selector or inspection budget.
pub fn capture_rust_concrete_source_items(
    network: &LinkNetwork,
    source: &str,
    options: &Value,
) -> Result<Value, String> {
    if [
        "byteLimit",
        "nodeLimit",
        "fieldLimit",
        "selectorLimit",
        "workLimit",
    ]
    .iter()
    .any(|name| options.get(*name).is_some_and(Value::is_null))
    {
        return Err(failure("resource bounds must be positive safe integers"));
    }
    let byte_limit = limit(options, "byteLimit", 16_777_216)?;
    let selector_limit = limit(options, "selectorLimit", 256)?;
    let required =
        || failure("bounded source text, logical path and explicit selectors are required");
    let path = options["path"]
        .as_str()
        .filter(|path| !path.is_empty())
        .ok_or_else(required)?;
    let selectors = options["selectors"]
        .as_array()
        .filter(|rows| {
            !rows.is_empty() && rows.len() <= selector_limit && source.len() <= byte_limit
        })
        .ok_or_else(required)?;
    let mut selection = ConcreteItemSelection::new(network, source, options)?;
    let mut items = Vec::with_capacity(selectors.len());
    let mut identities = BTreeSet::new();
    for selector in selectors {
        let invalid = || {
            failure("selectors retain explicit item kind, source name and real enclosing ownership")
        };
        let selector = selector
            .as_object()
            .filter(|fields| {
                fields.keys().all(|key| {
                    matches!(
                        key.as_str(),
                        "kind"
                            | "name"
                            | "implementationType"
                            | "implementationTrait"
                            | "modulePath"
                    )
                })
            })
            .ok_or_else(invalid)?;
        let kind = selector
            .get("kind")
            .and_then(Value::as_str)
            .ok_or_else(invalid)?;
        if !test_rust_concrete_selectable_item_kind(kind) {
            return Err(invalid());
        }
        let wanted = selector
            .get("name")
            .and_then(Value::as_str)
            .filter(|name| !name.is_empty())
            .ok_or_else(invalid)?;
        let implementation_type = selector
            .get("implementationType")
            .map(|value| value.as_str().ok_or_else(invalid))
            .transpose()?;
        let implementation_trait = selector
            .get("implementationTrait")
            .map(|value| value.as_str().ok_or_else(invalid))
            .transpose()?;
        let module_path = selector
            .get("modulePath")
            .map(|value| {
                value
                    .as_array()
                    .filter(|rows| {
                        rows.iter()
                            .all(|row| row.as_str().is_some_and(|name| !name.is_empty()))
                    })
                    .ok_or_else(invalid)
            })
            .transpose()?
            .map_or(&[][..], Vec::as_slice);
        let mut candidate = None;
        let nonunique = || {
            failure(&format!(
                "one actual source declaration must match {kind} {wanted}"
            ))
        };
        let canonical_kind = read_rust_concrete_item_kind(kind);
        for index in 0..selection.facts.len() {
            selection.charge(1)?;
            if read_rust_concrete_item_kind(term(&selection.facts[index])) != canonical_kind {
                continue;
            }
            let (name, value) = selection.name(index)?;
            if value != wanted || !selection.parents(index, "function_item")?.is_empty() {
                continue;
            }
            let (modules, implementation) = selection.owners(index)?;
            if modules.len() != module_path.len()
                || modules
                    .iter()
                    .zip(module_path)
                    .any(|(module, name)| &module["name"] != name)
                || implementation["type"].as_str() != implementation_type
                || implementation["trait"].as_str() != implementation_trait
            {
                continue;
            }
            if candidate
                .replace((index, name, modules, implementation))
                .is_some()
            {
                return Err(nonunique());
            }
        }
        let (index, name, modules, implementation) = candidate.ok_or_else(nonunique)?;
        let id = selection.facts[index].link_id;
        if !identities.insert(id) {
            return Err(failure(
                "source item selectors cannot duplicate an actual declaration",
            ));
        }
        if network.link(id).and_then(|link| link.metadata().language()) != Some("Rust") {
            return Err(failure(
                "the selected CST item must retain its actual Rust language metadata",
            ));
        }
        let body = selection.field(index, "body", false)?;
        let (start, byte_start, attributes) = selection.annotations(index)?;
        let fact = &selection.facts[index];
        let name = &selection.facts[name];
        let item_text = text(source, fact);
        let declaration = &source[byte_start..fact.byte_range.end()];
        let mut item = json!({"kind":term(fact),"name":wanted,"sourceLink":id.as_u64(),
            "range":{"start":fact.start,"end":fact.end,"byteStart":fact.byte_range.start(),"byteEnd":fact.byte_range.end()},
            "nameLink":name.link_id.as_u64(),"nameRange":units(name),"text":item_text,"sha256":format!("{:x}",Sha256::digest(item_text)),
            "declarationRange":{"start":start,"end":fact.end},"declarationText":declaration,"declarationSha256":format!("{:x}",Sha256::digest(declaration)),
            "modules":modules,"implementation":implementation,"attributes":attributes});
        if let Some(body) = body {
            let body = &selection.facts[body];
            item["bodyLink"] = json!(body.link_id.as_u64());
            item["bodyRange"] = units(body);
            item["bodyText"] = json!(text(source, body));
        }
        items.push(item);
    }
    Ok(
        json!({"model":MODEL,"language":"Rust","path":path,"sourceSha256":format!("{:x}",Sha256::digest(source)),
        "sourceBytes":source.len(),"sourceUnits":source.encode_utf16().count(),"items":items}),
    )
}
