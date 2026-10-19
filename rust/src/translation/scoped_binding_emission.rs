//! Checked native ownership adaptation for JavaScript scoped local aliases.

use super::diagnostics::{Result, unsupported};
use super::ir::{Decl, Effect, Expr, FnDecl, Main, Node, Program};
use super::scoped_binding_emission_rules::{
    accept_java_script_scoped_binding_boolean_field,
    accept_java_script_scoped_binding_defined_field,
    accept_java_script_scoped_binding_emission_absent_expression,
    accept_java_script_scoped_binding_emission_absent_value_type,
    accept_java_script_scoped_binding_emission_context,
    accept_java_script_scoped_binding_emission_declaration,
    accept_java_script_scoped_binding_emission_effect,
    accept_java_script_scoped_binding_emission_expression,
    accept_java_script_scoped_binding_emission_model,
    accept_java_script_scoped_binding_emission_reference_context,
    accept_java_script_scoped_binding_emission_role,
    accept_java_script_scoped_binding_emission_value_type,
    accept_java_script_scoped_binding_empty_sequence, accept_java_script_scoped_binding_own_field,
    accept_java_script_scoped_binding_source_facet,
};
use super::source_loops::validate_source_loop;
use super::types::Type;
use crate::{LinkId, LinkNetwork, LinkType};
use std::collections::{HashMap, HashSet};

fn fail(detail: &str) -> super::diagnostics::TranslationError {
    unsupported("JavaScript scoped binding emission", detail, None)
}

pub fn validate_model(model: &str) -> Result<()> {
    if !accept_java_script_scoped_binding_emission_model(model) {
        return Err(unsupported(
            "JavaScript scoped binding emission model",
            "select owned JavaScript scoped bindings 1",
            None,
        ));
    }
    Ok(())
}

/// Admit actual linked record fields before the native serde projection.
pub fn validate_raw_program(network: &LinkNetwork, root: LinkId, model: &str) -> Result<()> {
    use super::typed_program_link_rules::{
        accept_program_schema_root, read_program_expression_operand_fields,
        read_program_reference_role, read_program_reference_source_field,
    };
    validate_model(model)?;
    let link = network
        .link(root)
        .ok_or_else(|| fail("the program root is dangling"))?;
    if !accept_program_schema_root(
        link.metadata().term().unwrap_or(""),
        link.metadata().definition().unwrap_or(""),
        &link.references().len().to_string(),
    ) {
        return Err(fail("the program root has no checked schema role"));
    }
    let mut fields: HashMap<LinkId, Vec<(&str, LinkId)>> = HashMap::new();
    for relation in network.links() {
        if relation.metadata().link_type() != Some(LinkType::Field) {
            continue;
        }
        if let [parent, label, child] = relation.references() {
            let label = network
                .link(*label)
                .and_then(|link| link.metadata().term())
                .ok_or_else(|| fail("a linked field has no actual label"))?;
            fields.entry(*parent).or_default().push((label, *child));
        }
    }
    let field = |id: LinkId, name: &str| -> Option<LinkId> {
        let label = crate::typed_program_links::label_of(name);
        fields
            .get(&id)?
            .iter()
            .find_map(|(actual, child)| (*actual == label).then_some(*child))
    };
    let kind = |id: LinkId| -> &str {
        field(id, "k")
            .and_then(|id| network.link(id))
            .and_then(|link| link.metadata().definition())
            .unwrap_or("")
    };
    let sequence = |id: LinkId| -> Result<Vec<LinkId>> {
        let link = network
            .link(id)
            .ok_or_else(|| fail("an owned sequence is dangling"))?;
        if link.metadata().term()
            != crate::typed_program_links::typed_program_links_schema()["containers"]["array"]
                .as_str()
        {
            return Err(fail(
                "an owned sequence must retain its actual ordered container",
            ));
        }
        Ok(link.references().to_vec())
    };
    let mut pending = vec![("program", link.references()[0], 0usize)];
    let mut seen = HashSet::new();
    let mut visits = 0usize;
    while let Some((role, id, depth)) = pending.pop() {
        visits += 1;
        if visits > 100_000 || depth > 256 {
            return Err(fail("the raw ownership walk exceeds its resource limit"));
        }
        if !seen.insert((role, id)) {
            continue;
        }
        let node = network
            .link(id)
            .ok_or_else(|| fail("an owned record is dangling"))?;
        if node.metadata().term()
            != crate::typed_program_links::typed_program_links_schema()["containers"]["record"]
                .as_str()
        {
            return Err(fail(
                "an owned record must retain its actual record container",
            ));
        }
        let kind = if matches!(role, "declaration" | "effect" | "expression") {
            kind(id)
        } else {
            ""
        };
        for (label, child) in fields.get(&id).into_iter().flatten() {
            let name = if read_program_reference_role(label).is_empty() {
                crate::typed_program_links::field_of(label)
            } else {
                read_program_reference_source_field(label, kind, "")
            };
            if !accept_java_script_scoped_binding_own_field(role, kind, &name) {
                return Err(fail(&format!(
                    "a checked {role} has unrepresented own fields"
                )));
            }
            let value = network
                .link(*child)
                .ok_or_else(|| fail("an owned field value is dangling"))?;
            if !accept_java_script_scoped_binding_defined_field(
                value.metadata().term() == Some("undefined value"),
            ) {
                return Err(fail(&format!(
                    "a checked {role} has an explicitly undefined field"
                )));
            }
            if accept_java_script_scoped_binding_boolean_field(role, kind, &name)
                && (value.metadata().term() != Some("boolean value")
                    || !matches!(value.metadata().definition(), Some("true" | "false")))
            {
                return Err(fail(&format!(
                    "a checked {role} requires exact Boolean fields"
                )));
            }
            if role == "program"
                && !accept_java_script_scoped_binding_empty_sequence(&name, true,
                    value.metadata().term() == crate::typed_program_links::typed_program_links_schema()["containers"]["array"].as_str(),
                    value.references().is_empty())
            {
                return Err(fail("the closed program context requires actual empty import and external sequences"));
            }
            if role == "declaration"
                && accept_java_script_scoped_binding_source_facet(&name, true)
                && !accept_java_script_scoped_binding_source_facet(&name,
                    network.link(*child).and_then(|link| link.metadata().term())
                        == crate::typed_program_links::typed_program_links_schema()["containers"]["record"].as_str())
            {
                return Err(fail("an owned source facet must retain its actual checked record"));
            }
        }
        let mut push = |role, child| pending.push((role, child, depth + 1));
        match role {
            "program" => {
                if let Some(main) = field(id, "main") {
                    let main_value = network.link(main).ok_or_else(|| fail("main is dangling"))?;
                    if main_value.metadata().term() != Some("null value") {
                        push("main", main);
                    }
                }
                if let Some(items) = field(id, "items") {
                    for declaration in sequence(items)? {
                        push("declaration", declaration);
                    }
                }
                if let Some(directory) = field(id, "declarations") {
                    let directory = network
                        .link(directory)
                        .ok_or_else(|| fail("the declaration mapping is dangling"))?;
                    for entry in directory.references() {
                        let entry = network
                            .link(*entry)
                            .ok_or_else(|| fail("a declaration entry is dangling"))?;
                        let [_, declaration] = entry.references() else {
                            return Err(fail("a declaration entry must retain its actual pair"));
                        };
                        push("declaration", *declaration);
                    }
                }
            }
            "main" => {
                if let Some(effects) = field(id, "effects") {
                    for effect in sequence(effects)? {
                        push("effect", effect);
                    }
                }
            }
            "declaration" => {
                if let Some(parameters) = field(id, "params") {
                    for parameter in sequence(parameters)? {
                        push("parameter", parameter);
                    }
                }
                if let Some(body) = field(id, "body") {
                    push("expression", body);
                }
            }
            "effect" => {
                if let Some(value) = field(id, if kind == "let" { "value" } else { "expr" }) {
                    push("expression", value);
                }
            }
            "expression" => {
                for operand in read_program_expression_operand_fields(kind).split_whitespace() {
                    if let Some(value) = field(id, operand) {
                        if operand == "args" {
                            for value in sequence(value)? {
                                push("expression", value);
                            }
                        } else {
                            push("expression", value);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn value_type(ty: &Type) -> Result<()> {
    let primitive = accept_java_script_scoped_binding_emission_value_type(
        ty.kind(),
        matches!(ty, Type::Float | Type::Bool | Type::String | Type::Unit),
    );
    let absent = if let Type::Option { element, absence } = ty {
        accept_java_script_scoped_binding_emission_absent_value_type(
            ty.kind(),
            element.kind(),
            absence,
            matches!(element.as_ref(), Type::Float),
        )
    } else {
        false
    };
    if !primitive && !absent {
        return Err(fail(
            "only primitive scalar values and the exact undefined optional value type are admitted",
        ));
    }
    Ok(())
}

/// Validate the complete checked primitive scope before allocating aliases.
pub fn validate_program(program: &Program, model: &str) -> Result<()> {
    validate_model(model)?;
    super::source_empty_options::validate_source_empty_options(program)?;
    super::source_type_namespaces::validate_source_type_namespaces(program)?;
    crate::typed_program_links::validate_checked_program(program)
        .map_err(|error| super::diagnostics::type_error(error.to_string(), None))?;
    let forbidden = !program.imports.is_empty()
        || !program.externals.is_empty()
        || program.output_threaded
        || program.aborts_threaded;
    if !accept_java_script_scoped_binding_emission_context(
        program.source_language.as_str(),
        forbidden,
    ) {
        return Err(fail(
            "only the closed scalar value context has an owned local interpretation",
        ));
    }
    let mut pending: Vec<&Expr> = Vec::new();
    for declaration in &program.declarations {
        let Decl::Fn(function) = declaration else {
            return Err(fail(
                "only flat original value functions own these local scopes",
            ));
        };
        if !accept_java_script_scoped_binding_emission_declaration(
            declaration.kind(),
            !function.module_path.is_empty(),
            function.generated,
        ) {
            return Err(fail(
                "only flat original value functions own these local scopes",
            ));
        }
        if function.source_loop.is_some() {
            validate_source_loop(function, program.source_language.as_str())?;
        }
        value_type(&function.ret)?;
        for parameter in &function.params {
            value_type(&parameter.ty)?;
            if parameter.guard.is_some() || parameter.default_value.is_some() {
                return Err(fail(
                    "parameter guards, defaults and unrepresented facets require another scope proof",
                ));
            }
        }
        pending.push(&function.body);
    }
    if let Some(main) = &program.main {
        for effect in &main.effects {
            let (kind, value) = match effect {
                Effect::Let { value, .. } => ("let", value),
                Effect::Print { expr, .. } => ("print", expr),
                Effect::Assert { .. } | Effect::Output { .. } | Effect::Unwrap { .. } => {
                    return Err(fail(
                        "only ordered value bindings and prints are admitted in main",
                    ));
                }
            };
            if !accept_java_script_scoped_binding_emission_effect(kind) {
                return Err(fail(
                    "only ordered value bindings and prints are admitted in main",
                ));
            }
            pending.push(value);
        }
    }
    let mut at = 0;
    while at < pending.len() {
        if at >= 100_000 {
            return Err(fail("the scoped value walk exceeds its resource limit"));
        }
        let node = pending[at];
        at += 1;
        value_type(&node.ty)?;
        if !accept_java_script_scoped_binding_emission_expression(node.kind()) {
            return Err(fail("the construct has no admitted local ownership role"));
        }
        match &node.node {
            Node::None { absence } => {
                if !accept_java_script_scoped_binding_emission_absent_expression(
                    node.ty.kind(),
                    absence,
                ) {
                    return Err(fail(
                        "only the exact undefined empty expression is admitted",
                    ));
                }
            }
            Node::Call { func, .. } if program.declaration(func).is_none() => {
                return Err(fail(
                    "each call must resolve to an actual owned declaration",
                ));
            }
            Node::Binary { domain, .. } => {
                value_type(
                    domain
                        .as_ref()
                        .ok_or_else(|| fail("only primitive scalar value types are admitted"))?,
                )?;
            }
            _ => {}
        }
        pending.extend(node.children());
    }
    Ok(())
}

fn flag_form<'a>(network: &'a LinkNetwork, body: LinkId, field: &str) -> Result<Option<&'a str>> {
    let label = crate::typed_program_links::label_of(field);
    let scalar = network.links().find_map(|link| match link.references() {
        [parent, name, child]
            if *parent == body
                && link.metadata().link_type() == Some(LinkType::Field)
                && network.link(*name).and_then(|name| name.metadata().term())
                    == Some(label.as_str()) =>
        {
            Some(*child)
        }
        _ => None,
    });
    let Some(id) = scalar else {
        return Ok(None);
    };
    let scalar = network
        .link(id)
        .ok_or_else(|| fail("a context premise is dangling"))?;
    if scalar.metadata().term() != Some("boolean value") || !scalar.references().is_empty() {
        return Err(fail("a context premise is not a Boolean scalar"));
    }
    Ok(Some(scalar.metadata().definition().unwrap_or("other")))
}

/// Preserve origin flags that the native Program projection cannot represent.
pub fn validate_origin_context(
    network: &LinkNetwork,
    root: LinkId,
    program: &Program,
) -> Result<()> {
    let body = *network
        .link(root)
        .and_then(|link| link.references().first())
        .ok_or_else(|| fail("the program root has no checked body"))?;
    let form = flag_form(network, body, "referenceSemantics")?;
    let snapshots = program.source_language == super::Language::JavaScript
        && program
            .declarations
            .iter()
            .any(|entry| matches!(entry, Decl::Fn(function) if function.source_loop.is_some()));
    // validate_program rechecks each sourceLoop before this premise is admitted.
    if !accept_java_script_scoped_binding_emission_reference_context(
        form.is_some(),
        form.unwrap_or("other"),
        snapshots,
    ) || ["outputThreaded", "abortsThreaded", "utf16Strings"]
        .iter()
        .any(|field| {
            flag_form(network, body, field)
                .map_or(true, |form| form.is_some_and(|form| form != "false"))
        })
    {
        return Err(fail(
            "only the closed scalar value context has an owned local interpretation",
        ));
    }
    Ok(())
}

pub fn function_role(program: &Program, function: &FnDecl) -> Result<bool> {
    if !program
        .declarations
        .iter()
        .any(|entry| matches!(entry, Decl::Fn(actual) if std::ptr::eq(actual, function)))
    {
        return Err(fail(
            "the alias does not belong to its actual checked declaration or main scope",
        ));
    }
    let preserve = function.source_loop.is_some();
    let (role, owner) = if preserve {
        ("scalar loop binding", "scalar loop declaration")
    } else {
        ("function binding", "function declaration")
    };
    if !accept_java_script_scoped_binding_emission_role(role, owner) {
        return Err(fail(
            "the alias does not belong to an admitted binding scope",
        ));
    }
    Ok(preserve)
}

pub fn main_role(program: &Program, main: &Main) -> Result<()> {
    if !program
        .main
        .as_ref()
        .is_some_and(|actual| std::ptr::eq(actual, main))
    {
        return Err(fail(
            "the alias does not belong to its actual checked declaration or main scope",
        ));
    }
    if !accept_java_script_scoped_binding_emission_role("main binding", "main effect container") {
        return Err(fail(
            "the alias does not belong to an admitted binding scope",
        ));
    }
    Ok(())
}
