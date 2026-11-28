//! Host identities over checked IR; all semantic decisions reuse the checker.
use super::super::{
    Result, TypedProgramLinks, error, label_of, metadata, typed_program_links_schema,
};
use crate::translation::check::read_checked_match_pattern_bindings;
use crate::translation::ir::{Decl, Effect, Expr, Node, Program};
use crate::translation::typed_program_link_rules::{
    is_unsafe_program_field, read_program_expression_entry_operand_field,
    read_program_expression_operand_fields, read_program_pattern_binding_form,
    read_program_proposition_binding_form, read_program_proposition_operand_fields,
    read_program_reference_source_field,
};
use crate::{LinkId, LinkNetwork};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};

type Reference = (String, String, String);
type Targets = HashMap<String, Vec<Reference>>;
fn child_path(path: &str, field: impl std::fmt::Display) -> String {
    format!("{path}/{field}")
}
fn reference(
    targets: &mut Targets,
    path: &str,
    field: &str,
    label: &str,
    target: &str,
) -> Result<()> {
    let references = targets.entry(path.to_owned()).or_default();
    if references
        .iter()
        .any(|(owned_field, owned_label, _)| owned_field == field || owned_label == label)
    {
        return Err(error(
            "a typed program reference has duplicate field ownership",
        ));
    }
    references.push((field.to_owned(), label.to_owned(), target.to_owned()));
    Ok(())
}

fn normalize_expression(value: &mut Value, expression: &Expr, program: &Program) -> Result<()> {
    match &expression.node {
        Node::Lambda { body, .. } => {
            normalize_expression(&mut value["body"], body, program)?;
            return Ok(());
        }
        Node::Let {
            value: bound, body, ..
        } => {
            normalize_expression(&mut value["value"], bound, program)?;
            normalize_expression(&mut value["body"], body, program)?;
            return Ok(());
        }
        Node::Match { scrutinee, cases } => {
            normalize_expression(&mut value["scrutinee"], scrutinee, program)?;
            for (index, case) in cases.iter().enumerate() {
                let bindings = read_checked_match_pattern_bindings(
                    &case.pattern,
                    &scrutinee.ty,
                    &program.declarations,
                )
                .map_err(|failure| error(failure.to_string()))?;
                let pattern = &mut value["cases"][index]["pattern"];
                match read_program_pattern_binding_form(pattern["k"].as_str().unwrap_or_default())
                    .as_str()
                {
                    "single" => {
                        let binding = bindings
                            .first()
                            .ok_or_else(|| error("a checked pattern has no binding"))?;
                        pattern["name"] = serde_json::to_value(binding)
                            .map_err(|failure| error(failure.to_string()))?;
                    }
                    "fields" => {
                        let by_name: BTreeMap<_, _> = bindings
                            .iter()
                            .map(|binding| (binding.name.as_str(), binding))
                            .collect();
                        for slot in pattern["binds"]
                            .as_array_mut()
                            .ok_or_else(|| error("a checked pattern has no binding slots"))?
                        {
                            if let Some(name) = slot.as_str() {
                                let binding = by_name.get(name).ok_or_else(|| {
                                    error("a checked pattern binding is unresolved")
                                })?;
                                *slot = serde_json::to_value(binding)
                                    .map_err(|failure| error(failure.to_string()))?;
                            }
                        }
                    }
                    "none" => {}
                    _ => return Err(error("a checked pattern has no shared binding form")),
                }
                normalize_expression(&mut value["cases"][index]["body"], &case.body, program)?;
            }
            return Ok(());
        }
        _ => {}
    }
    let children = expression.children();
    let mut children = children.into_iter();
    for field in read_program_expression_operand_fields(expression.node.kind()).split_whitespace() {
        let entry = read_program_expression_entry_operand_field(expression.node.kind(), field);
        let operand = value
            .get_mut(field)
            .ok_or_else(|| error("a checked expression is missing a shared operand"))?;
        if let Some(values) = operand.as_array_mut() {
            for operand in values {
                let operand = if entry.is_empty() {
                    operand
                } else {
                    operand.get_mut(&entry).ok_or_else(|| {
                        error("a checked expression entry is missing its shared operand")
                    })?
                };
                if operand.is_null() {
                    continue;
                }
                let child = children
                    .next()
                    .ok_or_else(|| error("a shared expression operand is inconsistent"))?;
                normalize_expression(operand, child, program)?;
            }
        } else {
            let child = children
                .next()
                .ok_or_else(|| error("a shared expression operand is inconsistent"))?;
            normalize_expression(operand, child, program)?;
        }
    }
    if children.next().is_some() {
        return Err(error(
            "the Rust typed links adapter has no shared expression operand plan",
        ));
    }
    Ok(())
}

fn normalize_proposition(value: &mut Value, program: &Program) -> Result<()> {
    let kind = value["p"]
        .as_str()
        .ok_or_else(|| error("a checked assertion has no proposition kind"))?;
    let form = read_program_proposition_binding_form(kind);
    if form.is_empty() {
        return Err(error("a checked proposition has no shared operand plan"));
    }
    let fields = read_program_proposition_operand_fields(kind);
    for field in fields.split_whitespace() {
        let operand = value
            .get_mut(field)
            .ok_or_else(|| error("a checked proposition is missing a shared operand"))?;
        if form == "expressions" {
            let expression: Expr = serde_json::from_value(operand.clone())
                .map_err(|failure| error(failure.to_string()))?;
            normalize_expression(operand, &expression, program)?;
        } else {
            normalize_proposition(operand, program)?;
        }
    }
    Ok(())
}

fn semantic_targets(
    value: &Value,
    path: &str,
    declarations: &BTreeMap<String, String>,
    constructors: &BTreeMap<(String, String), String>,
    targets: &mut Targets,
    remaining: &mut usize,
) -> Result<()> {
    *remaining = remaining
        .checked_sub(1)
        .ok_or_else(|| error("the linked program exceeds its reference resource limit"))?;
    if let Some(object) = value.as_object() {
        let kind = value["k"].as_str().unwrap_or_default();
        let type_kind = value["kind"].as_str().unwrap_or_default();
        for (role, label) in typed_program_links_schema()["references"]
            .as_object()
            .unwrap()
        {
            if role == "binding" {
                continue;
            }
            let label = label.as_str().unwrap();
            let field = read_program_reference_source_field(label, kind, type_kind);
            if field.is_empty() {
                continue;
            }
            let name = value[&field]
                .as_str()
                .ok_or_else(|| error("a checked program reference has no name"))?;
            let target = if role == "constructor" {
                let data = value["data"]
                    .as_str()
                    .ok_or_else(|| error("a checked constructor has no datatype"))?;
                constructors.get(&(data.to_owned(), name.to_owned()))
            } else {
                declarations.get(name)
            };
            reference(
                targets,
                path,
                &field,
                label,
                target.ok_or_else(|| error("a checked program reference has no owned target"))?,
            )?;
        }
        for (field, child) in object {
            semantic_targets(
                child,
                &child_path(path, field),
                declarations,
                constructors,
                targets,
                remaining,
            )?;
        }
    } else if let Some(values) = value.as_array() {
        for (index, child) in values.iter().enumerate() {
            semantic_targets(
                child,
                &child_path(path, index),
                declarations,
                constructors,
                targets,
                remaining,
            )?;
        }
    }
    Ok(())
}

fn bindings(
    value: &Value,
    path: &str,
    environment: &BTreeMap<String, String>,
    targets: &mut Targets,
    remaining: &mut usize,
) -> Result<()> {
    *remaining = remaining
        .checked_sub(1)
        .ok_or_else(|| error("the linked program exceeds its binding resource limit"))?;
    let kind = value["k"].as_str().unwrap_or_default();
    if let Some(proposition) = value["p"].as_str() {
        let form = read_program_proposition_binding_form(proposition);
        if form.is_empty() {
            return Err(error("a checked proposition has no shared operand plan"));
        }
        let mut inner = environment.clone();
        if form == "binders" {
            for (index, binder) in value["binders"]
                .as_array()
                .ok_or_else(|| error("a checked proposition has no binders"))?
                .iter()
                .enumerate()
            {
                let name = binder["name"]
                    .as_str()
                    .ok_or_else(|| error("a checked proposition binder has no name"))?;
                inner.insert(
                    name.to_owned(),
                    child_path(&child_path(path, "binders"), index),
                );
            }
        }
        for field in read_program_proposition_operand_fields(proposition).split_whitespace() {
            bindings(
                &value[field],
                &child_path(path, field),
                &inner,
                targets,
                remaining,
            )?;
        }
        return Ok(());
    }
    if kind == "var" {
        let name = value["name"]
            .as_str()
            .ok_or_else(|| error("a checked variable has no name"))?;
        let target = environment
            .get(name)
            .ok_or_else(|| error("a checked variable is outside its scope"))?;
        reference(
            targets,
            path,
            "name",
            typed_program_links_schema()["references"]["binding"]
                .as_str()
                .unwrap(),
            target,
        )?;
    } else if kind == "lambda" {
        let parameters = value["params"]
            .as_array()
            .ok_or_else(|| error("a checked lambda has no owned parameters"))?;
        let mut inner = environment.clone();
        for index in super::super::callables::parameter_ordinals(value) {
            let parameter = &parameters[index];
            let name = parameter["name"]
                .as_str()
                .ok_or_else(|| error("a checked callable parameter has no name"))?;
            inner.insert(
                name.to_owned(),
                child_path(&child_path(path, "params"), index),
            );
        }
        return bindings(
            &value["body"],
            &child_path(path, "body"),
            &inner,
            targets,
            remaining,
        );
    } else if kind == "let" {
        bindings(
            &value["value"],
            &child_path(path, "value"),
            environment,
            targets,
            remaining,
        )?;
        let mut inner = environment.clone();
        let name = value["name"]
            .as_str()
            .ok_or_else(|| error("a checked binding has no name"))?;
        inner.insert(name.to_owned(), path.to_owned());
        return bindings(
            &value["body"],
            &child_path(path, "body"),
            &inner,
            targets,
            remaining,
        );
    } else if kind == "match" {
        bindings(
            &value["scrutinee"],
            &child_path(path, "scrutinee"),
            environment,
            targets,
            remaining,
        )?;
        for (index, case) in value["cases"]
            .as_array()
            .ok_or_else(|| error("a checked match has no cases"))?
            .iter()
            .enumerate()
        {
            let case_path = child_path(&child_path(path, "cases"), index);
            let pattern_path = child_path(&case_path, "pattern");
            let pattern = &case["pattern"];
            let mut inner = environment.clone();
            match read_program_pattern_binding_form(pattern["k"].as_str().unwrap_or_default())
                .as_str()
            {
                "single" => {
                    let name = pattern["name"]["name"]
                        .as_str()
                        .ok_or_else(|| error("a checked pattern binder has no name"))?;
                    inner.insert(name.to_owned(), child_path(&pattern_path, "name"));
                }
                "fields" => {
                    for (slot, binding) in pattern["binds"]
                        .as_array()
                        .ok_or_else(|| error("a checked pattern has no binding slots"))?
                        .iter()
                        .enumerate()
                    {
                        if binding.is_null() {
                            continue;
                        }
                        let name = binding["name"]
                            .as_str()
                            .ok_or_else(|| error("a checked pattern binder has no name"))?;
                        inner.insert(
                            name.to_owned(),
                            child_path(&child_path(&pattern_path, "binds"), slot),
                        );
                    }
                }
                "none" => {}
                _ => return Err(error("a checked pattern has no shared binding form")),
            }
            bindings(
                &case["body"],
                &child_path(&case_path, "body"),
                &inner,
                targets,
                remaining,
            )?;
        }
        return Ok(());
    }
    for field in read_program_expression_operand_fields(kind).split_whitespace() {
        let entry = read_program_expression_entry_operand_field(kind, field);
        if let Some(values) = value[field].as_array() {
            for (index, child) in values.iter().enumerate() {
                let mut path = child_path(&child_path(path, field), index);
                let child = if entry.is_empty() {
                    child
                } else {
                    path = child_path(&path, &entry);
                    child.get(&entry).ok_or_else(|| {
                        error("a checked expression entry is missing its shared operand")
                    })?
                };
                if child.is_null() {
                    continue;
                }
                bindings(child, &path, environment, targets, remaining)?;
            }
        } else {
            bindings(
                &value[field],
                &child_path(path, field),
                environment,
                targets,
                remaining,
            )?;
        }
    }
    Ok(())
}

struct Encoder {
    network: LinkNetwork,
    identities: HashMap<String, LinkId>,
    types: HashMap<String, LinkId>,
    targets: Targets,
    pending: Vec<(LinkId, String, String)>,
    runtime_value: Option<Value>,
}
impl Encoder {
    fn ordered_items(
        &mut self,
        values: &[Value],
        path: &str,
        declarations: &BTreeMap<String, LinkId>,
        seen: &mut BTreeSet<LinkId>,
    ) -> Result<LinkId> {
        let list = self.container("array");
        let mut children = Vec::new();
        for (index, item) in values.iter().enumerate() {
            let item_path = child_path(path, index);
            if item["k"] == "decl" {
                let id = item["fullName"]
                    .as_str()
                    .and_then(|name| declarations.get(name))
                    .copied()
                    .ok_or_else(|| error("a program item has no owned declaration"))?;
                if !seen.insert(id) {
                    return Err(error(
                        "the linked ordered items repeat an owned declaration",
                    ));
                }
                children.push(id);
            } else if item["k"] == "module" {
                let id = self.container("record");
                self.identities.insert(item_path.clone(), id);
                for (field, value) in item
                    .as_object()
                    .ok_or_else(|| error("an owned module needs its actual record"))?
                {
                    let child_path = child_path(&item_path, field);
                    let child = if field == "items" {
                        let values = value
                            .as_array()
                            .ok_or_else(|| error("an owned module needs its ordered items"))?;
                        self.ordered_items(values, &child_path, declarations, seen)?
                    } else {
                        self.value(value, &child_path)?
                    };
                    self.network.insert_field(id, &label_of(field), child);
                }
                children.push(id);
            } else {
                return Err(error("a program item has no owned declaration or module"));
            }
        }
        self.network.set_references(list, &children);
        Ok(list)
    }
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
        if self.runtime_value.is_some()
            && path.starts_with("/sourceRuntimeShell/")
            && let Some(target) = super::runtime_shell::reference_path(value)?
        {
            let value = self
                .runtime_value
                .as_ref()
                .and_then(|root| root.pointer(&target))
                .cloned()
                .ok_or_else(|| {
                    error("a runtime structural reference has no actual owned target")
                })?;
            return self.value(&value, &target);
        }
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
                if let Some((_, label, target)) = self
                    .targets
                    .get(path)
                    .and_then(|references| references.iter().find(|(field, _, _)| field == name))
                {
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
    let mut value = serde_json::to_value(program).map_err(|failure| error(failure.to_string()))?;
    for (index, declaration) in program.declarations.iter().enumerate() {
        if let Decl::Fn(function) = declaration {
            normalize_expression(
                &mut value["declarations"][index]["body"],
                &function.body,
                program,
            )?;
        }
    }
    if let Some(main) = &program.main {
        for (index, effect) in main.effects.iter().enumerate() {
            let (field, expression) = match effect {
                Effect::Print { expr, .. } => ("expr", expr),
                Effect::Let { value, .. } => ("value", value),
                Effect::Assert { .. } => {
                    normalize_proposition(&mut value["main"]["effects"][index]["prop"], program)?;
                    continue;
                }
                _ => {
                    return Err(error(
                        "the Rust typed links adapter does not yet represent this main effect",
                    ));
                }
            };
            normalize_expression(
                &mut value["main"]["effects"][index][field],
                expression,
                program,
            )?;
        }
    }
    super::owned_local_state::prepare_order(&mut value)?;
    let declarations = value["declarations"]
        .as_array()
        .ok_or_else(|| error("a checked program has no declarations"))?;
    let mut names = BTreeMap::new();
    let mut constructors = BTreeMap::new();
    for (index, declaration) in declarations.iter().enumerate() {
        let path = child_path("/declarations", index);
        let name = declaration["fullName"]
            .as_str()
            .ok_or_else(|| error("a checked declaration has no name"))?;
        if names.insert(name.to_owned(), path.clone()).is_some() {
            return Err(error("a checked program has duplicate declarations"));
        }
        if declaration["k"] == "data" {
            for (index, constructor) in declaration["ctors"]
                .as_array()
                .ok_or_else(|| error("a checked datatype has no constructors"))?
                .iter()
                .enumerate()
            {
                let constructor_name = constructor["name"]
                    .as_str()
                    .ok_or_else(|| error("a checked constructor has no name"))?;
                if constructors
                    .insert(
                        (name.to_owned(), constructor_name.to_owned()),
                        child_path(&child_path(&path, "ctors"), index),
                    )
                    .is_some()
                {
                    return Err(error("a checked datatype has duplicate constructors"));
                }
            }
        }
    }
    let mut targets = HashMap::new();
    let mut reference_remaining = 100_000;
    semantic_targets(
        &value,
        "",
        &names,
        &constructors,
        &mut targets,
        &mut reference_remaining,
    )?;
    let mut remaining = 100_000;
    for (index, declaration) in declarations.iter().enumerate() {
        if declaration["k"] != "fn" {
            continue;
        }
        let path = child_path("/declarations", index);
        let mut environment = BTreeMap::new();
        for (index, parameter) in declaration["params"]
            .as_array()
            .ok_or_else(|| error("a checked function has no parameters"))?
            .iter()
            .enumerate()
        {
            let name = parameter["name"]
                .as_str()
                .ok_or_else(|| error("a checked parameter has no name"))?;
            environment.insert(
                name.to_owned(),
                child_path(&child_path(&path, "params"), index),
            );
        }
        bindings(
            &declaration["body"],
            &child_path(&path, "body"),
            &environment,
            &mut targets,
            &mut remaining,
        )?;
    }
    if let Some(effects) = value["main"]["effects"].as_array() {
        let mut environment = BTreeMap::new();
        for (index, effect) in effects.iter().enumerate() {
            let path = child_path("/main/effects", index);
            let field = if effect["k"] == "let" {
                "value"
            } else if effect["k"] == "assert" {
                "prop"
            } else {
                "expr"
            };
            bindings(
                &effect[field],
                &child_path(&path, field),
                &environment,
                &mut targets,
                &mut remaining,
            )?;
            if effect["k"] == "let" {
                let name = effect["name"]
                    .as_str()
                    .ok_or_else(|| error("a checked binding has no name"))?;
                environment.insert(name.to_owned(), path);
            }
        }
    }
    let mut encoder = Encoder {
        network: LinkNetwork::new(),
        identities: HashMap::new(),
        types: HashMap::new(),
        targets,
        pending: Vec::new(),
        runtime_value: program.source_runtime_shell.as_ref().map(|_| value.clone()),
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
    let mut seen_items = BTreeSet::new();
    let items = encoder.ordered_items(
        value["items"]
            .as_array()
            .ok_or_else(|| error("a checked program needs its ordered items"))?,
        "/items",
        &declaration_ids,
        &mut seen_items,
    )?;
    if seen_items.len() != declaration_ids.len() {
        return Err(error(
            "the linked declaration mapping and ordered items disagree",
        ));
    }
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
