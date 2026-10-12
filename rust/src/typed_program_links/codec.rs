//! Structural identity codec for the schema; the existing checker owns semantics.
use super::{
    Result, TypedProgramLinks, TypedProgramLinksLimits, error, field_of, label_of, metadata,
    typed_program_links_schema,
};
use crate::translation::ir::Program;
use crate::translation::typed_program_link_rules::{
    accept_program_reference_field, accept_program_schema_root, is_unsafe_program_field,
    read_program_container_kind, read_program_scalar_kind,
};
use crate::{LinkId, LinkNetwork, LinkType};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

type Reference = (String, String, String);
fn child_path(path: &str, field: impl std::fmt::Display) -> String {
    format!("{path}/{field}")
}

fn binding_targets(
    value: &Value,
    path: &str,
    environment: &BTreeMap<String, String>,
    declarations: &BTreeMap<String, String>,
    targets: &mut HashMap<String, Reference>,
) {
    if let Some(object) = value.as_object() {
        if value["k"] == "var" {
            if let Some(target) = value["name"]
                .as_str()
                .and_then(|name| environment.get(name))
            {
                targets.insert(
                    path.to_owned(),
                    (
                        "name".to_owned(),
                        "binding reference".to_owned(),
                        target.clone(),
                    ),
                );
            }
        }
        if let Some(target) = value["fn"].as_str().and_then(|name| declarations.get(name)) {
            targets.insert(
                path.to_owned(),
                (
                    "fn".to_owned(),
                    "declaration reference".to_owned(),
                    target.clone(),
                ),
            );
        }
        if let Some(effects) = value["effects"].as_array() {
            let mut inner = environment.clone();
            for (index, effect) in effects.iter().enumerate() {
                let effect_path = child_path(&child_path(path, "effects"), index);
                if effect["k"] == "let" {
                    binding_targets(
                        &effect["value"],
                        &child_path(&effect_path, "value"),
                        &inner,
                        declarations,
                        targets,
                    );
                    if let Some(name) = effect["name"].as_str() {
                        inner.insert(name.to_owned(), effect_path);
                    }
                } else {
                    binding_targets(effect, &effect_path, &inner, declarations, targets);
                }
            }
            return;
        }
        if value["k"] == "let" {
            binding_targets(
                &value["value"],
                &child_path(path, "value"),
                environment,
                declarations,
                targets,
            );
            let mut inner = environment.clone();
            if let Some(name) = value["name"].as_str() {
                inner.insert(name.to_owned(), path.to_owned());
            }
            binding_targets(
                &value["body"],
                &child_path(path, "body"),
                &inner,
                declarations,
                targets,
            );
            return;
        }
        for (field, child) in object {
            if !matches!(field.as_str(), "type" | "domain" | "span" | "from" | "to") {
                binding_targets(
                    child,
                    &child_path(path, field),
                    environment,
                    declarations,
                    targets,
                );
            }
        }
    } else if let Some(values) = value.as_array() {
        for (index, child) in values.iter().enumerate() {
            binding_targets(
                child,
                &child_path(path, index),
                environment,
                declarations,
                targets,
            );
        }
    }
}

struct Encoder {
    network: LinkNetwork,
    identities: HashMap<String, LinkId>,
    types: HashMap<String, LinkId>,
    targets: HashMap<String, Reference>,
    pending: Vec<(LinkId, String, String)>,
}
impl Encoder {
    fn container(&mut self, kind: &str) -> LinkId {
        self.network.insert_link(
            [],
            metadata(
                typed_program_links_schema()["containers"][kind]
                    .as_str()
                    .unwrap(),
                None,
            ),
        )
    }
    fn value(&mut self, value: &Value, path: &str) -> Result<LinkId> {
        if let Some(existing) = self.identities.get(path) {
            return Ok(*existing);
        }
        if let Some(object) = value.as_object() {
            let type_key = value["kind"]
                .as_str()
                .filter(|_| path.starts_with("/declarations/") || path.starts_with("/main/"))
                .map(|_| serde_json::to_string(value).unwrap());
            if let Some(existing) = type_key.as_ref().and_then(|key| self.types.get(key)) {
                self.identities.insert(path.to_owned(), *existing);
                return Ok(*existing);
            }
            let id = self.container("record");
            self.identities.insert(path.to_owned(), id);
            if let Some(key) = type_key {
                self.types.insert(key, id);
            }
            for (name, child) in object {
                if let Some((field, label, target)) =
                    self.targets.get(path).filter(|(field, _, _)| field == name)
                {
                    let _ = field;
                    self.pending.push((id, label.clone(), target.clone()));
                } else {
                    if is_unsafe_program_field(name) {
                        return Err(error("a linked program contains an unsafe property name"));
                    }
                    let child = self.value(child, &child_path(path, name))?;
                    self.network.insert_field(id, &label_of(name), child);
                }
            }
            Ok(id)
        } else if let Some(values) = value.as_array() {
            let id = self.container("array");
            self.identities.insert(path.to_owned(), id);
            let children = values
                .iter()
                .enumerate()
                .map(|(index, value)| self.value(value, &child_path(path, index)))
                .collect::<Result<Vec<_>>>()?;
            self.network.set_references(id, &children);
            Ok(id)
        } else {
            let (kind, definition) = match value {
                Value::Null => ("null", None),
                Value::Bool(value) => ("boolean", Some(value.to_string())),
                Value::Number(value) => ("number", Some(value.to_string())),
                Value::String(value) => ("string", Some(value.clone())),
                _ => unreachable!("containers were handled above"),
            };
            Ok(self.network.insert_link(
                [],
                metadata(
                    typed_program_links_schema()["scalars"][kind]
                        .as_str()
                        .unwrap(),
                    definition.as_deref(),
                ),
            ))
        }
    }
}

pub(super) fn encode(program: &Program, provenance: Option<&Value>) -> Result<TypedProgramLinks> {
    let value = serde_json::to_value(program).map_err(|failure| error(failure.to_string()))?;
    let declarations = value["declarations"].as_array().unwrap();
    let names: BTreeMap<_, _> = declarations
        .iter()
        .enumerate()
        .map(|(index, declaration)| {
            (
                declaration["fullName"].as_str().unwrap().to_owned(),
                child_path("/declarations", index),
            )
        })
        .collect();
    let mut targets = HashMap::new();
    for (index, declaration) in declarations.iter().enumerate() {
        let path = child_path("/declarations", index);
        let environment = declaration["params"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                (
                    parameter["name"].as_str().unwrap().to_owned(),
                    child_path(&child_path(&path, "params"), index),
                )
            })
            .collect();
        binding_targets(
            &declaration["body"],
            &child_path(&path, "body"),
            &environment,
            &names,
            &mut targets,
        );
    }
    binding_targets(
        &value["main"],
        "/main",
        &BTreeMap::new(),
        &names,
        &mut targets,
    );
    let mut encoder = Encoder {
        network: LinkNetwork::new(),
        identities: HashMap::new(),
        types: HashMap::new(),
        targets,
        pending: Vec::new(),
    };
    let body = encoder.container("record");
    for (field, child) in value.as_object().unwrap() {
        if !matches!(field.as_str(), "items" | "declarations") {
            let child = encoder.value(child, &child_path("", field))?;
            encoder.network.insert_field(body, &label_of(field), child);
        }
    }
    let mapping = encoder.container("map");
    let mut pairs = Vec::new();
    let mut declaration_ids = BTreeMap::new();
    for (index, declaration) in declarations.iter().enumerate() {
        let id = encoder.value(declaration, &child_path("/declarations", index))?;
        let name = declaration["fullName"].as_str().unwrap();
        declaration_ids.insert(name.to_owned(), id);
        let key = encoder.value(&Value::String(name.to_owned()), &child_path("/keys", index))?;
        let pair = encoder.container("array");
        encoder.network.set_references(pair, &[key, id]);
        pairs.push(pair);
    }
    encoder.network.set_references(mapping, &pairs);
    encoder
        .network
        .insert_field(body, &label_of("declarations"), mapping);
    let items = encoder.container("array");
    let ordered = value["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| {
            item["fullName"]
                .as_str()
                .and_then(|name| declaration_ids.get(name))
                .copied()
                .ok_or_else(|| error("a program item has no owned declaration"))
        })
        .collect::<Result<Vec<_>>>()?;
    encoder.network.set_references(items, &ordered);
    encoder.network.insert_field(body, "items", items);
    for (parent, label, target) in &encoder.pending {
        let target = encoder
            .identities
            .get(target)
            .ok_or_else(|| error("a program reference has no owned target"))?;
        encoder.network.insert_field(*parent, label, *target);
    }
    let root = encoder.network.insert_link(
        [body],
        metadata(
            typed_program_links_schema()["root"].as_str().unwrap(),
            Some("1"),
        ),
    );
    if let Some(provenance) = provenance {
        let data = encoder.value(provenance, "/provenance")?;
        encoder.network.insert_field(root, "provenance", data);
    }
    Ok(TypedProgramLinks {
        network: encoder.network,
        root,
    })
}

#[derive(Clone)]
enum Data {
    Record(BTreeMap<String, LinkId>),
    Array(Vec<LinkId>),
    Map(Vec<(LinkId, LinkId)>),
    Scalar(Value),
}
struct Decoder<'a> {
    network: &'a LinkNetwork,
    fields: HashMap<LinkId, Vec<(String, LinkId)>>,
    values: HashMap<LinkId, Data>,
    active: HashSet<LinkId>,
    references: HashMap<LinkId, (String, LinkId)>,
    semantic_values: BTreeSet<LinkId>,
    limits: TypedProgramLinksLimits,
    visited: usize,
}
impl Decoder<'_> {
    fn protect_semantic_values(&mut self, id: LinkId) {
        if !self.semantic_values.insert(id) {
            return;
        }
        let children = match self.values.get(&id) {
            Some(Data::Record(fields)) => fields.values().copied().collect::<Vec<_>>(),
            Some(Data::Array(values)) => values.clone(),
            Some(Data::Map(values)) => values
                .iter()
                .flat_map(|(key, value)| [*key, *value])
                .collect(),
            _ => Vec::new(),
        };
        for child in children {
            self.protect_semantic_values(child);
        }
    }
    fn read(&mut self, id: LinkId, depth: usize) -> Result<()> {
        if self.active.contains(&id) {
            return Err(error("a linked program contains a structural cycle"));
        }
        if self.values.contains_key(&id) {
            return Ok(());
        }
        self.visited = self.visited.saturating_add(1);
        if self.visited > self.limits.node_limit || depth > self.limits.depth_limit {
            return Err(error(
                "the linked program exceeds its structural resource limit",
            ));
        }
        let network = self.network;
        let link = network
            .link(id)
            .ok_or_else(|| error("a linked program contains a dangling or foreign value"))?;
        if link.metadata().language() != typed_program_links_schema()["language"].as_str() {
            return Err(error(
                "a linked program contains a dangling or foreign value",
            ));
        }
        let kind = read_program_container_kind(link.metadata().term().unwrap_or(""));
        self.active.insert(id);
        let data = match kind.as_str() {
            "record" => {
                if !link.references().is_empty() {
                    return Err(error("a program record has positional references"));
                }
                let mut fields = BTreeMap::new();
                let mut labels = BTreeSet::new();
                for (label, child) in self.fields.get(&id).cloned().unwrap_or_default() {
                    if !labels.insert(label.clone()) {
                        return Err(error("a program record contains a duplicate field"));
                    }
                    if accept_program_reference_field(&label) {
                        if self.references.insert(id, (label, child)).is_some() {
                            return Err(error(
                                "a program record contains conflicting semantic references",
                            ));
                        }
                    } else {
                        let field = field_of(&label);
                        if is_unsafe_program_field(&field) {
                            return Err(error("a linked program contains an unsafe property name"));
                        }
                        self.read(child, depth + 1)?;
                        if matches!(
                            field.as_str(),
                            "type" | "domain" | "ret" | "from" | "to" | "ctors"
                        ) {
                            self.protect_semantic_values(child);
                        }
                        if fields.insert(field, child).is_some() {
                            return Err(error("a program record contains a duplicate field"));
                        }
                    }
                }
                Data::Record(fields)
            }
            "array" => {
                let children = link.references().to_vec();
                for child in &children {
                    self.read(*child, depth + 1)?;
                }
                Data::Array(children)
            }
            "map" => {
                let children = link.references().to_vec();
                let mut pairs = Vec::new();
                let mut keys = BTreeSet::new();
                for child in children {
                    self.read(child, depth + 1)?;
                    let Some(Data::Array(pair)) = self.values.get(&child) else {
                        return Err(error("a program mapping contains an invalid entry"));
                    };
                    if pair.len() != 2 {
                        return Err(error("a program mapping contains an invalid entry"));
                    }
                    let key = self.string(pair[0])?;
                    if !keys.insert(key) {
                        return Err(error("a program mapping contains a duplicate entry"));
                    }
                    pairs.push((pair[0], pair[1]));
                }
                Data::Map(pairs)
            }
            _ => {
                if !link.references().is_empty() || self.fields.contains_key(&id) {
                    return Err(error("a program scalar contains structural children"));
                }
                let definition = link.metadata().definition().unwrap_or("");
                if definition.encode_utf16().count() > 1_000_000 {
                    return Err(error("a linked program contains an invalid scalar"));
                }
                let scalar = read_program_scalar_kind(link.metadata().term().unwrap_or(""));
                let value = match scalar.as_str() {
                    "string" => Value::String(definition.to_owned()),
                    "null" => Value::Null,
                    "boolean" if matches!(definition, "true" | "false") => {
                        Value::Bool(definition == "true")
                    }
                    "number" => {
                        let number: Value = serde_json::from_str(definition)
                            .map_err(|_| error("a linked program contains an invalid number"))?;
                        if !number.is_number() {
                            return Err(error("a linked program contains an invalid number"));
                        }
                        number
                    }
                    "bigint" | "undefined" => {
                        return Err(error(
                            "the Rust typed program adapter does not admit bigint or undefined host scalar payloads",
                        ));
                    }
                    _ => return Err(error("a linked program contains an invalid scalar")),
                };
                Data::Scalar(value)
            }
        };
        self.active.remove(&id);
        self.values.insert(id, data);
        Ok(())
    }
    fn field(&self, id: LinkId, field: &str) -> Option<LinkId> {
        match self.values.get(&id) {
            Some(Data::Record(fields)) => fields.get(field).copied(),
            _ => None,
        }
    }
    fn string(&self, id: LinkId) -> Result<String> {
        match self.values.get(&id) {
            Some(Data::Scalar(Value::String(value))) => Ok(value.clone()),
            _ => Err(error("a program name or mapping key is not a string")),
        }
    }
    fn name(&self, id: LinkId) -> Result<String> {
        self.string(
            self.field(id, "name")
                .ok_or_else(|| error("a program binding has no name"))?,
        )
    }
    fn full_name(&self, id: LinkId) -> Result<String> {
        self.string(
            self.field(id, "fullName")
                .or_else(|| self.field(id, "name"))
                .ok_or_else(|| error("a declaration has no name"))?,
        )
    }
    fn kind(&self, id: LinkId) -> String {
        self.field(id, "k")
            .and_then(|kind| self.string(kind).ok())
            .unwrap_or_default()
    }
    fn array(&self, id: LinkId) -> Result<&[LinkId]> {
        match self.values.get(&id) {
            Some(Data::Array(values)) => Ok(values),
            _ => Err(error("a program sequence is not an ordered list")),
        }
    }
    fn render(&self, id: LinkId, remaining: &mut usize, bytes: &mut usize) -> Result<Value> {
        *remaining = remaining.checked_sub(1).ok_or_else(|| {
            error("the linked program exceeds its Rust materialization resource limit")
        })?;
        match self
            .values
            .get(&id)
            .ok_or_else(|| error("a program reference does not target an owned declaration"))?
        {
            Data::Scalar(value) => {
                *bytes = bytes
                    .checked_sub(value.as_str().map_or(16, str::len))
                    .ok_or_else(|| {
                        error("the linked program exceeds its Rust materialization byte limit")
                    })?;
                Ok(value.clone())
            }
            Data::Array(values) => Ok(Value::Array(
                values
                    .iter()
                    .map(|child| self.render(*child, remaining, bytes))
                    .collect::<Result<_>>()?,
            )),
            Data::Map(values) => Ok(Value::Array(
                values
                    .iter()
                    .map(|(key, value)| {
                        Ok(Value::Array(vec![
                            self.render(*key, remaining, bytes)?,
                            self.render(*value, remaining, bytes)?,
                        ]))
                    })
                    .collect::<Result<_>>()?,
            )),
            Data::Record(fields) => {
                let mut record = Map::new();
                for (field, child) in fields {
                    record.insert(field.clone(), self.render(*child, remaining, bytes)?);
                }
                if let Some((label, target)) = self.references.get(&id) {
                    let (field, name) = if label == "binding reference" {
                        ("name", self.name(*target)?)
                    } else {
                        ("fn", self.full_name(*target)?)
                    };
                    if record
                        .insert(field.to_owned(), Value::String(name))
                        .is_some()
                    {
                        return Err(error("a program reference conflicts with a raw name field"));
                    }
                }
                Ok(Value::Object(record))
            }
        }
    }
    fn bindings(
        &self,
        id: LinkId,
        environment: &BTreeMap<String, LinkId>,
        declarations: &BTreeMap<String, LinkId>,
        remaining: &mut usize,
        identities: &mut BTreeSet<LinkId>,
    ) -> Result<()> {
        *remaining = remaining.checked_sub(1).ok_or_else(|| {
            error("the linked program exceeds its semantic traversal resource limit")
        })?;
        if let Some(effects) = self.field(id, "effects") {
            let mut inner = environment.clone();
            for effect in self.array(effects)? {
                if self.kind(*effect) == "let" {
                    self.bindings(
                        self.field(*effect, "value")
                            .ok_or_else(|| error("a linked binding has no value"))?,
                        &inner,
                        declarations,
                        remaining,
                        identities,
                    )?;
                    identities.insert(*effect);
                    inner.insert(self.name(*effect)?, *effect);
                } else {
                    self.bindings(*effect, &inner, declarations, remaining, identities)?;
                }
            }
            return Ok(());
        }
        let kind = self.kind(id);
        if kind == "var" {
            let (name, target) = if let Some((_, target)) = self.references.get(&id) {
                (self.name(*target)?, Some(*target))
            } else {
                (self.name(id)?, None)
            };
            let owner = environment
                .get(&name)
                .ok_or_else(|| error(format!("the linked variable {name} is unbound")))?;
            if target.is_some_and(|target| target != *owner) {
                return Err(error(
                    "the linked variable targets a declaration outside its scope",
                ));
            }
        } else if kind == "call" {
            let (name, target) = if let Some((_, target)) = self.references.get(&id) {
                (self.full_name(*target)?, Some(*target))
            } else {
                (
                    self.string(
                        self.field(id, "fn")
                            .ok_or_else(|| error("a linked call has no declaration"))?,
                    )?,
                    None,
                )
            };
            let owner = declarations
                .get(&name)
                .ok_or_else(|| error("a linked declaration reference is unresolved"))?;
            if target.is_some_and(|target| target != *owner) {
                return Err(error(
                    "a linked declaration reference targets a different declaration",
                ));
            }
        } else if kind == "let" {
            self.bindings(
                self.field(id, "value")
                    .ok_or_else(|| error("a linked binding has no value"))?,
                environment,
                declarations,
                remaining,
                identities,
            )?;
            identities.insert(id);
            let mut inner = environment.clone();
            inner.insert(self.name(id)?, id);
            return self.bindings(
                self.field(id, "body")
                    .ok_or_else(|| error("a linked binding has no body"))?,
                &inner,
                declarations,
                remaining,
                identities,
            );
        }
        match self.values.get(&id) {
            Some(Data::Record(fields)) => {
                for (field, child) in fields {
                    if !matches!(field.as_str(), "type" | "domain" | "span" | "from" | "to") {
                        self.bindings(*child, environment, declarations, remaining, identities)?;
                    }
                }
            }
            Some(Data::Array(values)) => {
                for child in values {
                    self.bindings(*child, environment, declarations, remaining, identities)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

fn validate_spans(value: &Value) -> Result<()> {
    if let Some(object) = value.as_object() {
        if let Some(span) = object.get("span").filter(|span| !span.is_null()) {
            let start = span["start"].as_u64();
            let end = span["end"].as_u64();
            if !matches!((start, end), (Some(start), Some(end)) if start <= end && end <= 9_007_199_254_740_991)
            {
                return Err(error("a linked source span is invalid"));
            }
        }
        for child in object.values() {
            validate_spans(child)?;
        }
    } else if let Some(values) = value.as_array() {
        for child in values {
            validate_spans(child)?;
        }
    }
    Ok(())
}

fn reject_unknown_fields(original: &Value, normalized: &Value) -> Result<()> {
    match (original, normalized) {
        (Value::Object(fields), Value::Object(known)) => {
            for (field, value) in fields {
                if let Some(normalized) = known.get(field) {
                    reject_unknown_fields(value, normalized)?;
                } else if !matches!(
                    field.as_str(),
                    "span"
                        | "imports"
                        | "externals"
                        | "outputThreaded"
                        | "abortsThreaded"
                        | "sequentialAsync"
                        | "mutual"
                        | "generated"
                        | "console"
                ) || !(value.is_null()
                    || value == &Value::Bool(false)
                    || value.as_array().is_some_and(Vec::is_empty))
                {
                    return Err(error(format!(
                        "the Rust typed links adapter does not yet represent field {field}"
                    )));
                }
            }
        }
        (Value::Array(values), Value::Array(known)) => {
            for (value, normalized) in values.iter().zip(known) {
                reject_unknown_fields(value, normalized)?;
            }
        }
        _ => {}
    }
    Ok(())
}
pub(super) fn decode(
    network: &LinkNetwork,
    root: LinkId,
    limits: TypedProgramLinksLimits,
) -> Result<Program> {
    Ok(decode_with_binding_identities(network, root, limits)?.0)
}

pub(super) fn decode_with_binding_identities(
    network: &LinkNetwork,
    root: LinkId,
    limits: TypedProgramLinksLimits,
) -> Result<(Program, Vec<LinkId>)> {
    let link = network
        .link(root)
        .ok_or_else(|| error("the linked program root or schema is invalid"))?;
    if !accept_program_schema_root(
        link.metadata().term().unwrap_or(""),
        link.metadata().definition().unwrap_or(""),
        &link.references().len().to_string(),
    ) {
        return Err(error("the linked program root or schema is invalid"));
    }
    let mut fields: HashMap<LinkId, Vec<(String, LinkId)>> = HashMap::new();
    for relation in network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
    {
        if let [parent, label, child] = relation.references() {
            let label = network
                .link(*label)
                .and_then(|label| label.metadata().term())
                .ok_or_else(|| error("a linked program contains a dangling field label"))?;
            fields
                .entry(*parent)
                .or_default()
                .push((label.to_owned(), *child));
        }
    }
    let mut decoder = Decoder {
        network,
        fields,
        values: HashMap::new(),
        active: HashSet::new(),
        references: HashMap::new(),
        semantic_values: BTreeSet::new(),
        limits,
        visited: 0,
    };
    let body = link.references()[0];
    decoder.read(body, 0)?;
    for (_, target) in decoder.references.values() {
        if !matches!(decoder.values.get(target), Some(Data::Record(_))) {
            return Err(error(
                "a program reference does not target an owned declaration",
            ));
        }
    }
    let mapping = decoder
        .field(body, "declarations")
        .ok_or_else(|| error("the linked value has no declaration mapping"))?;
    let Some(Data::Map(entries)) = decoder.values.get(&mapping) else {
        return Err(error("the linked declaration mapping is invalid"));
    };
    let mut declarations = BTreeMap::new();
    for (name, entry) in entries {
        let name = decoder.string(*name)?;
        if decoder.full_name(*entry)? != name {
            return Err(error("the linked declaration mapping is inconsistent"));
        }
        declarations.insert(name, *entry);
    }
    let mut traversal_remaining = limits.node_limit;
    let mut identities = BTreeSet::new();
    for entry in declarations.values() {
        let mut environment = BTreeMap::new();
        if let Some(parameters) = decoder.field(*entry, "params") {
            for parameter in decoder.array(parameters)? {
                environment.insert(decoder.name(*parameter)?, *parameter);
                identities.insert(*parameter);
            }
        }
        if let Some(body) = decoder
            .field(*entry, "body")
            .or_else(|| decoder.field(*entry, "value"))
        {
            decoder.bindings(
                body,
                &environment,
                &declarations,
                &mut traversal_remaining,
                &mut identities,
            )?;
        }
    }
    if let Some(main) = decoder.field(body, "main") {
        decoder.bindings(
            main,
            &BTreeMap::new(),
            &declarations,
            &mut traversal_remaining,
            &mut identities,
        )?;
    }
    let mut remaining = limits.node_limit;
    let mut bytes = 64 * 1024 * 1024;
    let mut value = decoder.render(body, &mut remaining, &mut bytes)?;
    let pairs = value["declarations"]
        .as_array()
        .ok_or_else(|| error("the linked declaration mapping is invalid"))?;
    let ordered_declarations = pairs.iter().map(|pair| pair[1].clone()).collect::<Vec<_>>();
    let items_id = decoder
        .field(body, "items")
        .ok_or_else(|| error("the linked program has no ordered items"))?;
    let mut items = Vec::new();
    for id in decoder.array(items_id)? {
        if !declarations.values().any(|entry| entry == id) {
            return Err(error("a program item is not an owned declaration"));
        }
        items.push(json!({"k":"decl", "fullName":decoder.full_name(*id)?}));
    }
    value["items"] = Value::Array(items);
    value["declarations"] = Value::Array(ordered_declarations);
    validate_spans(&value)?;
    let program: Program = serde_json::from_value(value.clone()).map_err(|failure| {
        error(format!(
            "the Rust typed links adapter cannot decode this construct: {failure}"
        ))
    })?;
    reject_unknown_fields(
        &value,
        &serde_json::to_value(&program).map_err(|failure| error(failure.to_string()))?,
    )?;
    Ok((
        program,
        identities
            .into_iter()
            .filter(|id| !decoder.semantic_values.contains(id))
            .collect(),
    ))
}
