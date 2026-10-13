//! Checked native ownership adaptation for JavaScript scoped local aliases.

use super::diagnostics::{Result, unsupported};
use super::ir::{Decl, Effect, Expr, FnDecl, Main, Node, Program};
use super::scoped_binding_emission_rules::*;
use super::source_loops::validate_source_loop;
use super::types::Type;
use crate::{LinkId, LinkNetwork, LinkType};

fn fail(detail: &str) -> super::diagnostics::TranslationError {
    unsupported("JavaScript scoped binding emission", detail, None)
}

pub(crate) fn validate_model(model: &str) -> Result<()> {
    if !accept_java_script_scoped_binding_emission_model(model) {
        return Err(unsupported(
            "JavaScript scoped binding emission model",
            "select owned JavaScript scoped bindings 1",
            None,
        ));
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
pub(crate) fn validate_program(program: &Program, model: &str) -> Result<()> {
    validate_model(model)?;
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
pub(crate) fn validate_origin_context(
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

pub(crate) fn function_role(program: &Program, function: &FnDecl) -> Result<bool> {
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

pub(crate) fn main_role(program: &Program, main: &Main) -> Result<()> {
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
