//! Structural identity codec for the schema; the existing checker owns semantics.
use super::{
    Result, TypedProgramLinks, TypedProgramLinksLimits, error, field_of, typed_program_links_schema,
};
use crate::translation::ir::Program;
use crate::translation::primitive_value_rules::{accept_tuple_index, read_tuple_position};
use crate::translation::typed_program_link_rules::{
    accept_program_container_fields, accept_program_expression_field,
    accept_program_reference_field, accept_program_schema_root, is_unsafe_program_field,
    read_program_container_kind, read_program_neutral_field_default,
    read_program_reference_source_field, read_program_scalar_kind,
};
use crate::{LinkId, LinkNetwork, LinkType};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[path = "codec_encoding.rs"]
mod encoding;
#[path = "codec_scopes.rs"]
mod scopes;
pub(super) fn encode(program: &Program, provenance: Option<&Value>) -> Result<TypedProgramLinks> {
    encoding::encode(program, provenance)
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
    references: HashMap<LinkId, BTreeMap<String, LinkId>>,
    limits: TypedProgramLinksLimits,
    visited: usize,
}
impl Decoder<'_> {
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
        if !kind.is_empty()
            && !accept_program_container_fields(&kind, self.fields.contains_key(&id))
        {
            return Err(error("an ordered program container contains named fields"));
        }
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
                        if self
                            .references
                            .entry(id)
                            .or_default()
                            .insert(label, child)
                            .is_some()
                        {
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
                        if fields.insert(field, child).is_some() {
                            return Err(error("a program record contains a duplicate field"));
                        }
                    }
                }
                if fields.contains_key("type")
                    && !fields.contains_key("fullName")
                    && let Some(Data::Scalar(Value::String(kind))) =
                        fields.get("k").and_then(|id| self.values.get(id))
                    && typed_program_links_schema()["expressionFields"]
                        .get(kind)
                        .is_some()
                    && fields
                        .keys()
                        .any(|field| !accept_program_expression_field(kind, field))
                {
                    return Err(error(
                        "a linked expression contains an unrecognized semantic field",
                    ));
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
    fn ordered_items(
        &self,
        list: LinkId,
        declarations: &BTreeSet<LinkId>,
        seen_items: &mut BTreeSet<LinkId>,
        seen_declarations: &mut BTreeSet<LinkId>,
        remaining: &mut usize,
        bytes: &mut usize,
    ) -> Result<Vec<Value>> {
        let mut items = Vec::new();
        for id in self.array(list)? {
            *remaining = remaining
                .checked_sub(1)
                .ok_or_else(|| error("the linked program exceeds its ordered item limit"))?;
            if !seen_items.insert(*id) {
                return Err(error("the linked ordered items repeat an owned item"));
            }
            if declarations.contains(id) {
                seen_declarations.insert(*id);
                items.push(json!({"k":"decl", "fullName":self.full_name(*id)?}));
            } else if self.tag(*id, "k") == "module" {
                // Preserve the whole owned record: serde and the checked scope
                // adapter must see every supplied module field and flag.
                let mut module = self.render(*id, remaining, bytes)?;
                let children = self
                    .field(*id, "items")
                    .ok_or_else(|| error("an owned module has no ordered items"))?;
                module["items"] = Value::Array(self.ordered_items(
                    children,
                    declarations,
                    seen_items,
                    seen_declarations,
                    remaining,
                    bytes,
                )?);
                items.push(module);
            } else {
                return Err(error(
                    "a program item is not an owned declaration or module",
                ));
            }
        }
        Ok(items)
    }
    fn tag(&self, id: LinkId, field: &str) -> &str {
        match self
            .field(id, field)
            .and_then(|value| self.values.get(&value))
        {
            Some(Data::Scalar(Value::String(value))) => value,
            _ => "",
        }
    }
    fn kind(&self, id: LinkId) -> &str {
        self.tag(id, "k")
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
                if let Some(references) = self.references.get(&id) {
                    for (label, target) in references {
                        let field = read_program_reference_source_field(
                            label,
                            self.kind(id),
                            self.type_kind(id),
                        );
                        if field.is_empty() {
                            return Err(error(
                                "a linked semantic reference has an invalid owner role",
                            ));
                        }
                        let name = self.reference_name(label, *target)?;
                        *bytes = bytes.checked_sub(name.len()).ok_or_else(|| {
                            error("the linked program exceeds its Rust materialization byte limit")
                        })?;
                        if record.insert(field, Value::String(name)).is_some() {
                            return Err(error(
                                "a program reference conflicts with a raw name field",
                            ));
                        }
                    }
                }
                // A tuple position is a natural integer. Its scalar record
                // can use a floating spelling such as -0 or 0e0; normalize
                // only this owned role before serde reads the usize field.
                if self.kind(id) == "tupleIndex"
                    && let Some(object) = record.get("object")
                    && object["type"]["kind"] == "tuple"
                    && let Some(elements) = object["type"]["elements"].as_array()
                    && let Some(index) = record.get("index").and_then(Value::as_f64)
                    && serde_json::Number::from(elements.len())
                        .as_f64()
                        .is_some_and(|length| accept_tuple_index(index, length))
                {
                    let position = read_tuple_position(index)
                        .to_string()
                        .parse::<u64>()
                        .map_err(|_| error("a linked tuple position is not a natural integer"))?;
                    record.insert("index".to_owned(), Value::from(position));
                }
                Ok(Value::Object(record))
            }
        }
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
            let context = if known.contains_key("schemaVersion") {
                "program".to_owned()
            } else if known.contains_key("effects") {
                "main".to_owned()
            } else if let Some(kind) = known.get("k").and_then(Value::as_str) {
                if known.contains_key("fullName") {
                    format!("declaration:{kind}")
                } else if known.contains_key("type") {
                    format!("expression:{kind}")
                } else {
                    String::new()
                }
            } else {
                String::new()
            };
            for (field, value) in fields {
                if let Some(normalized) = known.get(field) {
                    reject_unknown_fields(value, normalized)?;
                } else {
                    let neutral = match read_program_neutral_field_default(&context, field).as_str()
                    {
                        "false" => value == &Value::Bool(false),
                        "null" => value.is_null(),
                        "empty-array" => value.as_array().is_some_and(Vec::is_empty),
                        _ => false,
                    };
                    if neutral {
                        continue;
                    }
                    return Err(error(format!(
                        "the Rust typed links adapter does not yet represent field {field}"
                    )));
                }
            }
        }
        (Value::Array(values), Value::Array(known)) => {
            if values.len() != known.len() {
                return Err(error(
                    "the Rust typed links adapter changed an ordered list",
                ));
            }
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
        limits,
        visited: 0,
    };
    let body = link.references()[0];
    decoder.read(body, 0)?;
    for references in decoder.references.values() {
        for target in references.values() {
            if !matches!(decoder.values.get(target), Some(Data::Record(_))) {
                return Err(error(
                    "a program reference does not target an owned declaration",
                ));
            }
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
    let mut traversal = scopes::Traversal::new(limits.node_limit);
    let ownership = decoder.ownership(&declarations, &mut traversal.remaining)?;
    decoder.validate_references(&ownership)?;
    for entry in declarations.values() {
        let mut environment = BTreeMap::new();
        if let Some(parameters) = decoder.field(*entry, "params") {
            for (slot, parameter) in decoder.array(parameters)?.iter().enumerate() {
                scopes::own_binding(&mut traversal.binding_owners, *parameter, *entry, slot)?;
                environment.insert(decoder.name(*parameter)?, *parameter);
                traversal.identities.insert(*parameter);
            }
        }
        if let Some(body) = decoder
            .field(*entry, "body")
            .or_else(|| decoder.field(*entry, "value"))
        {
            decoder.bindings(body, &environment, &ownership, &mut traversal, false)?;
        }
    }
    if let Some(main) = decoder.field(body, "main") {
        decoder.bindings(main, &BTreeMap::new(), &ownership, &mut traversal, true)?;
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
    let owned_declarations = declarations.values().copied().collect::<BTreeSet<_>>();
    let mut seen_items = BTreeSet::new();
    let mut seen_declarations = BTreeSet::new();
    let items = decoder.ordered_items(
        items_id,
        &owned_declarations,
        &mut seen_items,
        &mut seen_declarations,
        &mut remaining,
        &mut bytes,
    )?;
    if seen_declarations != owned_declarations {
        return Err(error(
            "the linked declaration mapping and ordered items disagree",
        ));
    }
    value["items"] = Value::Array(items);
    value["declarations"] = Value::Array(ordered_declarations);
    validate_spans(&value)?;
    scopes::restore_patterns(&mut value)?;
    let program: Program = serde_json::from_value(value.clone()).map_err(|failure| {
        error(format!(
            "the Rust typed links adapter cannot decode this construct: {failure}"
        ))
    })?;
    reject_unknown_fields(
        &value,
        &serde_json::to_value(&program).map_err(|failure| error(failure.to_string()))?,
    )?;
    Ok((program, traversal.identities.into_iter().collect()))
}
