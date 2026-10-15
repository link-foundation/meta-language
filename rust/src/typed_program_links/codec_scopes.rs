//! Owned references and lexical scopes over the shared graph schema.
use super::super::{Result, error};
use super::{Data, Decoder, reject_unknown_fields};
use crate::LinkId;
use crate::translation::check::read_checked_match_pattern_bindings;
use crate::translation::ir::{Decl, Pattern};
use crate::translation::typed_program_link_rules::{
    read_program_expression_operand_fields, read_program_pattern_binding_form,
    read_program_proposition_binding_form, read_program_proposition_operand_fields,
    read_program_reference_role, read_program_reference_source_field,
    read_program_reference_target_field,
};
use crate::translation::types::Type;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub(super) struct Ownership {
    declarations: BTreeMap<String, LinkId>,
    datatypes: BTreeMap<String, LinkId>,
    declaration_ids: BTreeSet<LinkId>,
    datatype_ids: BTreeSet<LinkId>,
    constructors: HashMap<LinkId, LinkId>,
    models: Vec<Decl>,
    model_nodes: usize,
    model_bytes: usize,
}
type BindingOwners = HashMap<LinkId, (LinkId, usize)>;

pub(super) struct Traversal {
    pub(super) remaining: usize,
    pub(super) identities: BTreeSet<LinkId>,
    pub(super) binding_owners: BindingOwners,
    bytes: usize,
}
impl Traversal {
    pub(super) fn new(remaining: usize) -> Self {
        Self {
            remaining,
            identities: BTreeSet::new(),
            binding_owners: HashMap::new(),
            bytes: 64 * 1024 * 1024,
        }
    }
}

pub(super) fn own_binding(
    owners: &mut BindingOwners,
    binding: LinkId,
    owner: LinkId,
    slot: usize,
) -> Result<()> {
    if owners
        .get(&binding)
        .is_some_and(|existing| *existing != (owner, slot))
    {
        return Err(error("a linked binding record has multiple lexical owners"));
    }
    owners.insert(binding, (owner, slot));
    Ok(())
}
fn visit(remaining: &mut usize) -> Result<()> {
    *remaining = remaining
        .checked_sub(1)
        .ok_or_else(|| error("the linked program exceeds its semantic traversal resource limit"))?;
    Ok(())
}

impl Decoder<'_> {
    pub(super) fn type_kind(&self, id: LinkId) -> &str {
        self.tag(id, "kind")
    }
    pub(super) fn reference_name(&self, label: &str, target: LinkId) -> Result<String> {
        match read_program_reference_target_field(label).as_str() {
            "name" => self.name(target),
            "fullName" => self.full_name(target),
            _ => Err(error("a linked semantic reference has an unsupported role")),
        }
    }
    pub(super) fn ownership(
        &self,
        declarations: &BTreeMap<String, LinkId>,
        remaining: &mut usize,
    ) -> Result<Ownership> {
        let mut ownership = Ownership {
            declarations: declarations.clone(),
            datatypes: BTreeMap::new(),
            declaration_ids: declarations.values().copied().collect(),
            datatype_ids: BTreeSet::new(),
            constructors: HashMap::new(),
            models: Vec::new(),
            model_nodes: 0,
            model_bytes: 0,
        };
        let mut bytes = 64 * 1024 * 1024;
        for (name, id) in declarations {
            if self.kind(*id) != "data" {
                continue;
            }
            if name.is_empty() {
                return Err(error("a linked data type has no name"));
            }
            ownership.datatypes.insert(name.clone(), *id);
            ownership.datatype_ids.insert(*id);
            let constructors = self.array(
                self.field(*id, "ctors")
                    .ok_or_else(|| error("a linked datatype has no constructors"))?,
            )?;
            let mut names = BTreeSet::new();
            for constructor in constructors {
                visit(remaining)?;
                let name = self.name(*constructor)?;
                if name.is_empty() || !names.insert(name) {
                    return Err(error(
                        "a linked datatype has malformed or duplicate constructors",
                    ));
                }
                if ownership
                    .constructors
                    .insert(*constructor, *id)
                    .is_some_and(|owner| owner != *id)
                {
                    return Err(error(
                        "a linked constructor declaration has multiple datatype owners",
                    ));
                }
                let fields = self.array(
                    self.field(*constructor, "fields")
                        .ok_or_else(|| error("a linked constructor has no fields"))?,
                )?;
                let mut field_names = BTreeSet::new();
                for field in fields {
                    visit(remaining)?;
                    let name = self.name(*field)?;
                    if name.is_empty() || !field_names.insert(name) {
                        return Err(error(
                            "a linked constructor has malformed or duplicate fields",
                        ));
                    }
                }
            }
            let before_nodes = *remaining;
            let before_bytes = bytes;
            let mut value = self.render(*id, remaining, &mut bytes)?;
            super::super::source_spans::normalize_source_spans(&mut value)?;
            ownership.model_nodes += before_nodes - *remaining;
            ownership.model_bytes += before_bytes - bytes;
            if value.get("sourceUnitCloneDerivation").is_some() {
                crate::translation::source_unit_clone_derivations::validate_unit_clone_declaration_record(&value)
                    .map_err(|failure| error(failure.to_string()))?;
            }
            if value.get("sourceTypeNamespace").is_some() {
                crate::translation::source_type_namespaces::validate_type_namespace_declaration_record(&value)
                    .map_err(|failure| error(failure.to_string()))?;
            }
            if value.get("sourcePureUnit").is_some() {
                crate::translation::source_pure_unit_calls::validate_pure_unit_declaration_record(
                    &value,
                )
                .map_err(|failure| error(failure.to_string()))?;
            }
            let model: Decl = serde_json::from_value(value.clone())
                .map_err(|failure| error(failure.to_string()))?;
            reject_unknown_fields(
                &value,
                &serde_json::to_value(&model).map_err(|failure| error(failure.to_string()))?,
            )?;
            ownership.models.push(model);
        }
        Ok(ownership)
    }
    pub(super) fn validate_references(&self, ownership: &Ownership) -> Result<()> {
        // Legacy archives can retain nominal spellings as scalar fields.
        // Resolved native Type::Data values bypass source name resolution,
        // so their declaration ownership must still be checked here.
        for id in self.values.keys() {
            if self.type_kind(*id) == "data" {
                let reference = self.references.get(id).and_then(|references| {
                    references.iter().find_map(|(label, target)| {
                        (read_program_reference_role(label) == "datatype").then_some(target)
                    })
                });
                let owns_type = reference.map_or_else(
                    || {
                        let name = self.tag(*id, "name");
                        !name.is_empty() && ownership.datatypes.contains_key(name)
                    },
                    |target| ownership.datatype_ids.contains(target),
                );
                if !owns_type {
                    return Err(error(
                        "a linked data type has no owned datatype declaration",
                    ));
                }
            }
        }
        for (owner, references) in &self.references {
            let kind = self.kind(*owner);
            let type_kind = self.type_kind(*owner);
            let mut fields = BTreeSet::new();
            for (label, target) in references {
                let field = read_program_reference_source_field(label, kind, type_kind);
                if field.is_empty() {
                    return Err(error(
                        "a linked semantic reference has an invalid owner role",
                    ));
                }
                if self.field(*owner, &field).is_some() {
                    return Err(error("a program reference conflicts with a raw name field"));
                }
                if !fields.insert(field) {
                    return Err(error(
                        "a program record contains duplicate reference field aliases",
                    ));
                }
                let role = read_program_reference_role(label);
                if role == "declaration" {
                    if self.kind(*target) != "fn" || !ownership.declaration_ids.contains(target) {
                        return Err(error(
                            "a linked declaration reference is unresolved or has an invalid target",
                        ));
                    }
                } else if role == "datatype" {
                    if !ownership.datatype_ids.contains(target) {
                        return Err(error(
                            "a linked datatype reference does not target an owned datatype",
                        ));
                    }
                } else if role == "constructor" {
                    let datatype = references
                        .iter()
                        .find_map(|(label, target)| {
                            (read_program_reference_role(label) == "datatype").then_some(target)
                        })
                        .or_else(|| ownership.datatypes.get(self.tag(*owner, "data")))
                        .ok_or_else(|| error("a linked constructor has no owned datatype"))?;
                    if ownership.constructors.get(target) != Some(datatype) {
                        return Err(error(
                            "a linked constructor reference belongs to a different datatype",
                        ));
                    }
                }
            }
        }
        Ok(())
    }
    fn pattern_bindings(
        &self,
        pattern: LinkId,
        owner: LinkId,
        scrutinee_type: LinkId,
        ownership: &Ownership,
        traversal: &mut Traversal,
    ) -> Result<Vec<(String, LinkId)>> {
        visit(&mut traversal.remaining)?;
        let mut pattern_value =
            self.render(pattern, &mut traversal.remaining, &mut traversal.bytes)?;
        let form = read_program_pattern_binding_form(self.kind(pattern));
        let records = match form.as_str() {
            "single" => {
                vec![Some(self.field(pattern, "name").ok_or_else(|| {
                    error("a linked pattern has no binding record")
                })?)]
            }
            "fields" => self
                .array(
                    self.field(pattern, "binds")
                        .ok_or_else(|| error("a linked pattern has no binding slots"))?,
                )?
                .iter()
                .map(|id| {
                    if matches!(self.values.get(id), Some(Data::Scalar(Value::Null))) {
                        None
                    } else {
                        Some(*id)
                    }
                })
                .collect(),
            "none" => Vec::new(),
            _ => return Err(error("a linked match contains an unsupported pattern")),
        };
        for id in records.iter().flatten() {
            let Some(Data::Record(fields)) = self.values.get(id) else {
                return Err(error(
                    "a linked pattern binding must be an owned name and type record",
                ));
            };
            if fields.len() != 2
                || !fields.contains_key("name")
                || !fields.contains_key("type")
                || self.references.contains_key(id)
            {
                return Err(error(
                    "a linked pattern binding must be an owned name and type record",
                ));
            }
        }
        restore_pattern(&mut pattern_value)?;
        let checked_pattern: Pattern = serde_json::from_value(pattern_value.clone())
            .map_err(|failure| error(failure.to_string()))?;
        reject_unknown_fields(
            &pattern_value,
            &serde_json::to_value(&checked_pattern)
                .map_err(|failure| error(failure.to_string()))?,
        )?;
        let type_value = self.render(
            scrutinee_type,
            &mut traversal.remaining,
            &mut traversal.bytes,
        )?;
        let scrutinee_type: Type = serde_json::from_value(type_value.clone())
            .map_err(|failure| error(failure.to_string()))?;
        reject_unknown_fields(
            &type_value,
            &serde_json::to_value(&scrutinee_type).map_err(|failure| error(failure.to_string()))?,
        )?;
        // The existing checker takes an owned declaration map. Account for
        // the host boundary copy so small shared case graphs cannot repeatedly
        // copy a large datatype model outside the semantic budget.
        traversal.remaining = traversal
            .remaining
            .checked_sub(ownership.model_nodes)
            .ok_or_else(|| {
                error("the linked program exceeds its semantic traversal resource limit")
            })?;
        traversal.bytes = traversal
            .bytes
            .checked_sub(ownership.model_bytes)
            .ok_or_else(|| {
                error("the linked program exceeds its semantic materialization byte limit")
            })?;
        let expected = read_checked_match_pattern_bindings(
            &checked_pattern,
            &scrutinee_type,
            &ownership.models,
        )
        .map_err(|failure| error(failure.to_string()))?;
        if records.iter().flatten().count() != expected.len() {
            return Err(error("a linked pattern has inconsistent binding slots"));
        }
        let mut expected = expected.into_iter();
        let mut result = Vec::new();
        for (slot, id) in records.iter().enumerate() {
            let Some(id) = id else {
                continue;
            };
            let binding = expected
                .next()
                .ok_or_else(|| error("a linked pattern binding is unresolved"))?;
            let name = self.name(*id)?;
            let value = self.render(
                self.field(*id, "type")
                    .ok_or_else(|| error("a linked pattern binding has no type"))?,
                &mut traversal.remaining,
                &mut traversal.bytes,
            )?;
            let ty: Type = serde_json::from_value(value.clone())
                .map_err(|failure| error(failure.to_string()))?;
            reject_unknown_fields(
                &value,
                &serde_json::to_value(&ty).map_err(|failure| error(failure.to_string()))?,
            )?;
            if name != binding.name || ty != binding.ty {
                return Err(error(
                    "the linked program has inconsistent pattern binding types",
                ));
            }
            own_binding(&mut traversal.binding_owners, *id, owner, slot)?;
            traversal.identities.insert(*id);
            result.push((name, *id));
        }
        Ok(result)
    }
    pub(super) fn bindings(
        &self,
        id: LinkId,
        environment: &BTreeMap<String, LinkId>,
        ownership: &Ownership,
        traversal: &mut Traversal,
        main: bool,
    ) -> Result<()> {
        visit(&mut traversal.remaining)?;
        if main {
            if matches!(self.values.get(&id), Some(Data::Scalar(Value::Null))) {
                return Ok(());
            }
            let mut inner = environment.clone();
            for effect in self.array(
                self.field(id, "effects")
                    .ok_or_else(|| error("a linked main has no effects"))?,
            )? {
                match self.kind(*effect) {
                    "let" => {
                        self.bindings(
                            self.field(*effect, "value")
                                .ok_or_else(|| error("a linked binding has no value"))?,
                            &inner,
                            ownership,
                            traversal,
                            false,
                        )?;
                        own_binding(&mut traversal.binding_owners, *effect, *effect, 0)?;
                        traversal.identities.insert(*effect);
                        inner.insert(self.name(*effect)?, *effect);
                    }
                    "print" => self.bindings(
                        self.field(*effect, "expr")
                            .ok_or_else(|| error("a linked print effect has no expression"))?,
                        &inner,
                        ownership,
                        traversal,
                        false,
                    )?,
                    "assert" => self.bindings(
                        self.field(*effect, "prop")
                            .ok_or_else(|| error("a linked assertion has no proposition"))?,
                        &inner,
                        ownership,
                        traversal,
                        false,
                    )?,
                    _ => {
                        return Err(error(
                            "the Rust typed links adapter does not yet represent this main effect",
                        ));
                    }
                }
            }
            return Ok(());
        }
        let kind = self.kind(id);
        let proposition = self.tag(id, "p");
        if !proposition.is_empty() {
            let form = read_program_proposition_binding_form(proposition);
            if form.is_empty() {
                return Err(error("a linked proposition has no shared operand plan"));
            }
            let mut inner = environment.clone();
            if form == "binders" {
                for (slot, binder) in self
                    .array(
                        self.field(id, "binders").ok_or_else(|| {
                            error("a linked proposition has no owned binder list")
                        })?,
                    )?
                    .iter()
                    .enumerate()
                {
                    own_binding(&mut traversal.binding_owners, *binder, id, slot)?;
                    traversal.identities.insert(*binder);
                    inner.insert(self.name(*binder)?, *binder);
                }
            }
            for field in read_program_proposition_operand_fields(proposition).split_whitespace() {
                self.bindings(
                    self.field(id, field)
                        .ok_or_else(|| error("a linked proposition has no shared operand"))?,
                    &inner,
                    ownership,
                    traversal,
                    false,
                )?;
            }
            return Ok(());
        }
        if kind == "var" {
            let target = self.references.get(&id).and_then(|references| {
                references.iter().find_map(|(label, target)| {
                    (read_program_reference_role(label) == "binding").then_some(target)
                })
            });
            if let Some(target) = target {
                if !environment.values().any(|binding| binding == target) {
                    return Err(error(
                        "the linked variable targets a declaration outside its scope",
                    ));
                }
            } else {
                let name = self.tag(id, "name");
                if !environment.contains_key(name) {
                    return Err(error(format!("the linked variable {name} is unbound")));
                }
            }
        } else if kind == "call" || kind == "functionRef" {
            let has_reference = self.references.get(&id).is_some_and(|references| {
                references
                    .keys()
                    .any(|label| read_program_reference_role(label) == "declaration")
            });
            if !has_reference && !ownership.declarations.contains_key(self.tag(id, "fn")) {
                return Err(error("a linked declaration reference is unresolved"));
            }
        } else if kind == "let" {
            self.bindings(
                self.field(id, "value")
                    .ok_or_else(|| error("a linked binding has no value"))?,
                environment,
                ownership,
                traversal,
                false,
            )?;
            own_binding(&mut traversal.binding_owners, id, id, 0)?;
            traversal.identities.insert(id);
            let mut inner = environment.clone();
            inner.insert(self.name(id)?, id);
            return self.bindings(
                self.field(id, "body")
                    .ok_or_else(|| error("a linked binding has no body"))?,
                &inner,
                ownership,
                traversal,
                false,
            );
        } else if kind == "match" {
            let scrutinee = self
                .field(id, "scrutinee")
                .ok_or_else(|| error("a linked match has no scrutinee"))?;
            self.bindings(scrutinee, environment, ownership, traversal, false)?;
            let ty = self
                .field(scrutinee, "type")
                .ok_or_else(|| error("a linked match scrutinee has no type"))?;
            for case in self.array(
                self.field(id, "cases")
                    .ok_or_else(|| error("a linked match has no cases"))?,
            )? {
                let pattern = self
                    .field(*case, "pattern")
                    .ok_or_else(|| error("a linked case has no pattern"))?;
                let mut inner = environment.clone();
                for (name, binding) in
                    self.pattern_bindings(pattern, *case, ty, ownership, traversal)?
                {
                    inner.insert(name, binding);
                }
                self.bindings(
                    self.field(*case, "body")
                        .ok_or_else(|| error("a linked case has no body"))?,
                    &inner,
                    ownership,
                    traversal,
                    false,
                )?;
            }
            return Ok(());
        }
        for field in read_program_expression_operand_fields(kind).split_whitespace() {
            let child = self
                .field(id, field)
                .ok_or_else(|| error("a linked expression has no shared operand"))?;
            if let Some(Data::Array(values)) = self.values.get(&child) {
                for child in values {
                    self.bindings(*child, environment, ownership, traversal, false)?;
                }
            } else {
                self.bindings(child, environment, ownership, traversal, false)?;
            }
        }
        Ok(())
    }
}

fn binder_name(value: &Value) -> Result<Value> {
    let object = value
        .as_object()
        .ok_or_else(|| error("a linked pattern binding must be an owned name and type record"))?;
    if object.len() != 2
        || !object.contains_key("name")
        || !object.contains_key("type")
        || value["name"].as_str().is_none_or(str::is_empty)
    {
        return Err(error(
            "a linked pattern binding must be an owned name and type record",
        ));
    }
    Ok(value["name"].clone())
}
fn restore_pattern(pattern: &mut Value) -> Result<()> {
    match read_program_pattern_binding_form(pattern["k"].as_str().unwrap_or_default()).as_str() {
        "single" => pattern["name"] = binder_name(&pattern["name"])?,
        "fields" => {
            for slot in pattern["binds"]
                .as_array_mut()
                .ok_or_else(|| error("a linked pattern has no binding slots"))?
            {
                if !slot.is_null() {
                    *slot = binder_name(slot)?;
                }
            }
        }
        "none" => {}
        _ => return Err(error("a linked match contains an unsupported pattern")),
    }
    Ok(())
}
pub(super) fn restore_patterns(value: &mut Value) -> Result<()> {
    if value["k"] == "match" {
        for case in value["cases"]
            .as_array_mut()
            .ok_or_else(|| error("a linked match has no cases"))?
        {
            restore_pattern(&mut case["pattern"])?;
        }
    }
    if let Some(object) = value.as_object_mut() {
        for child in object.values_mut() {
            restore_patterns(child)?;
        }
    } else if let Some(values) = value.as_array_mut() {
        for child in values {
            restore_patterns(child)?;
        }
    }
    Ok(())
}
