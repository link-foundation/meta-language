//! Owned source-header validation and native graph/value adaptation.
//!
//! Shared generated decisions and renderers supply the selected syntax model.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde_json::Value;

use super::diagnostics::{Result, unsupported};
use super::emit_rust::indent;
use super::ir::{Decl, Expr, Node, Param, Program, Semantics};
use super::source_header_emission_rules::{
    accept_rust_source_header_emission_context, accept_rust_source_header_emission_declaration,
    accept_rust_source_header_emission_declaration_fields,
    accept_rust_source_header_emission_expression,
    accept_rust_source_header_emission_expression_field, accept_rust_source_header_emission_model,
    accept_rust_source_header_emission_parameter, accept_rust_source_header_emission_position,
    accept_rust_source_header_emission_semantics, accept_rust_source_header_lifetime_character,
    read_rust_source_header_primitive, render_rust_source_header_copy,
    render_rust_source_header_dereference, render_rust_source_header_function,
    render_rust_source_header_lifetime, render_rust_source_header_parameter,
    render_rust_source_header_reference,
};
use super::source_signatures::{SourceSignature, validate_source_signature};
use super::surface::UnaryOp;
use super::types::Type;
use crate::shared_numeric_function_rules::{
    accept_shared_numeric_binary, accept_shared_numeric_expression_shape,
    accept_shared_numeric_type_shape, accept_shared_numeric_unary,
};

fn fail(detail: &str) -> super::diagnostics::TranslationError {
    unsupported("Rust source header emission", detail, None)
}

fn value_type(ty: &Type) -> Result<()> {
    if !accept_shared_numeric_type_shape(ty.kind(), matches!(ty, Type::Float | Type::Bool)) {
        return Err(fail(
            "only primitive binary64 and Boolean value types are admitted",
        ));
    }
    Ok(())
}

pub(crate) fn source_header_policy() -> &'static Value {
    static POLICY: OnceLock<Value> = OnceLock::new();
    POLICY.get_or_init(|| {
        crate::read_structured_data_lino(include_str!("../data/rust-source-header-emission.lino"))
            .expect("generated source header emission policy is structured links data")
    })
}

const fn semantics_name(semantics: Option<Semantics>) -> &'static str {
    match semantics {
        None => "",
        Some(Semantics::Exact) => "exact",
        Some(Semantics::Checked) => "checked",
        Some(Semantics::Truncated) => "truncated",
        Some(Semantics::Ieee) => "ieee",
    }
}

pub fn validate_rust_source_header_emission_model(model: &str) -> Result<()> {
    if !accept_rust_source_header_emission_model(model)
        && !super::borrowed_call_interface_rules::accept_rust_borrowed_call_interface_model(model)
        && !super::rust_scalar_source_interface_rules::accept_rust_scalar_source_interface_model(
            model,
        )
        && !super::rust_scalar_mathematical_interface_rules::accept_rust_scalar_mathematical_interface_model(model)
        && !super::rust_textual_source_interface_rules::accept_rust_textual_source_interface_model(model)
        && !super::rust_string_predicate_interface_rules::accept_rust_string_predicate_interface_model(model)
        && !super::rust_constructed_string_interface_rules::accept_rust_constructed_string_interface_model(model)
        && !super::rust_displayed_string_interface_rules::accept_rust_displayed_string_interface_model(model)
    {
        return Err(unsupported(
            "Rust source header emission model",
            "select primitive borrowed Rust function header 1",
            None,
        ));
    }
    Ok(())
}

/// Validate every original declaration before changing any function interface.
///
/// # Errors
/// Refuses unsupported models, context, owners, interfaces and body operations.
pub fn read_rust_source_header_emission(
    program: &Program,
    model: &str,
) -> Result<BTreeMap<String, SourceSignature>> {
    if !accept_rust_source_header_emission_model(model) {
        return Err(unsupported(
            "Rust source header emission model",
            "select primitive borrowed Rust function header 1",
            None,
        ));
    }
    validate_rust_source_header_emission_model(model)?;
    let headers = read_rust_primitive_source_headers(program)?;
    validate_rust_primitive_value_expressions(program.declarations.iter().filter_map(
        |declaration| {
            if let Decl::Fn(function) = declaration {
                Some(&function.body)
            } else {
                None
            }
        },
    ))?;
    Ok(headers)
}

/// Read the actual owned primitive headers; body admission is a separate proof.
pub fn read_rust_primitive_source_headers(
    program: &Program,
) -> Result<BTreeMap<String, SourceSignature>> {
    super::source_primitive_owners::validate_source_primitive_owners(program)?;
    super::source_type_namespaces::validate_source_type_namespaces(program)?;
    let forbidden = !program.imports.is_empty()
        || !program.externals.is_empty()
        || program
            .main
            .as_ref()
            .is_some_and(|main| !main.effects.is_empty())
        || program.output_threaded
        || program.aborts_threaded;
    if !accept_rust_source_header_emission_context(program.source_language.as_str(), forbidden) {
        return Err(fail(
            "only Rust leaf value functions without imports, callers or effects are admitted",
        ));
    }
    let mut headers = BTreeMap::new();
    for declaration in &program.declarations {
        let Decl::Fn(function) = declaration else {
            return Err(fail(
                "only flat original leaf function declarations are admitted",
            ));
        };
        let fields = serde_json::to_value(declaration)
            .map_err(|_| fail("the declaration needs its actual checked field inventory"))?;
        let unrepresented = fields.as_object().unwrap().keys().any(|field| {
            !source_header_policy()["declarationFields"]
                .as_array()
                .unwrap()
                .iter()
                .any(|allowed| allowed.as_str() == Some(field.as_str()))
        });
        if !accept_rust_source_header_emission_declaration_fields(unrepresented) {
            return Err(fail(
                "function declarations cannot carry unrepresented source interface fields",
            ));
        }
        if !accept_rust_source_header_emission_declaration(
            declaration.kind(),
            !function.module_path.is_empty(),
            function.generated,
        ) {
            return Err(fail(
                "only flat original leaf function declarations are admitted",
            ));
        }
        let signature = function.source_signature.as_ref().ok_or_else(|| {
            fail("every original function needs its owned source signature facet")
        })?;
        let parameter_types = function
            .params
            .iter()
            .map(|parameter| parameter.ty.clone())
            .collect::<Vec<_>>();
        validate_source_signature(
            signature,
            &parameter_types,
            &function.ret,
            program.source_language.as_str(),
        )?;
        for lifetime in &signature.lifetime_bindings {
            if lifetime.is_empty()
                || lifetime.bytes().enumerate().any(|(index, character)| {
                    !accept_rust_source_header_lifetime_character(f64::from(character), index == 0)
                })
            {
                return Err(fail(
                    "the selected header lifetime profile requires an ASCII identifier",
                ));
            }
        }
        value_type(&function.ret)?;
        for (index, parameter) in function.params.iter().enumerate() {
            value_type(&parameter.ty)?;
            let parameter_fields = fields["params"][index]
                .as_object()
                .ok_or_else(|| fail("the parameter needs its actual checked field inventory"))?;
            let unrepresented = parameter_fields.keys().any(|field| {
                !source_header_policy()["parameterFields"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|allowed| allowed.as_str() == Some(field.as_str()))
            });
            if !accept_rust_source_header_emission_parameter(
                parameter.guard.is_some(),
                parameter.default_value.is_some(),
                unrepresented,
            ) {
                return Err(fail(
                    "parameter defaults, guards and unrepresented fields need a separate interface proof",
                ));
            }
        }
        for (position, result) in signature
            .parameters
            .iter()
            .map(|position| (position, false))
            .chain(std::iter::once((&signature.result, true)))
        {
            value_type(&position.ty)?;
            let path = if position.owner.path.len() == 1 {
                position.owner.path[0].as_str()
            } else {
                ""
            };
            if !accept_rust_source_header_emission_position(
                position.ty.kind(),
                &position.owner.kind,
                path,
                !position.owner.arguments.is_empty(),
                position.wrappers.iter().any(|wrapper| {
                    !source_header_policy()["parameterWrappers"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|allowed| allowed.as_str() == Some(wrapper.as_str()))
                }),
                result && !position.wrappers.is_empty(),
            ) {
                return Err(fail(
                    "headers require primitive f64 or bool owners, immutable-reference inputs and direct results",
                ));
            }
        }
        headers.insert(function.full_name.clone(), signature.clone());
    }
    Ok(headers)
}

/// Validate primitive roots without changing their checked declarations.
pub fn validate_rust_primitive_value_expressions<'a>(
    roots: impl IntoIterator<Item = &'a Expr>,
) -> Result<()> {
    let mut pending = roots.into_iter().collect::<Vec<_>>();
    // Serialize each root once. Per-subtree serialization would make long
    // binding chains quadratic; this host walk only inspects actual keys.
    let actual = pending
        .iter()
        .map(serde_json::to_value)
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| fail("a primitive body needs its actual checked field inventory"))?;
    let mut field_nodes = actual.iter().collect::<Vec<_>>();
    let mut field_at = 0;
    while field_at < field_nodes.len() {
        if field_at >= 100_000 {
            return Err(fail("the leaf body exceeds its expression resource limit"));
        }
        let node = field_nodes[field_at];
        field_at += 1;
        let fields = node.as_object().unwrap();
        let kind = node["k"].as_str().unwrap();
        let qualified = node.get("rounding").is_some_and(|value| !value.is_null())
            || node.get("byZero").is_some_and(|value| !value.is_null());
        if !accept_rust_source_header_emission_expression(kind)
            || !accept_shared_numeric_expression_shape(kind, qualified)
        {
            continue;
        }
        if fields
            .keys()
            .any(|field| !accept_rust_source_header_emission_expression_field(kind, field))
        {
            return Err(fail(
                "primitive value expressions cannot carry unrepresented source interface fields",
            ));
        }
        if let Some(semantics) = node.get("semantics")
            && !semantics
                .as_str()
                .is_some_and(accept_rust_source_header_emission_semantics)
        {
            return Err(fail(
                "primitive value expressions require represented arithmetic semantics",
            ));
        }
        for (field, value) in fields {
            if ["type", "domain", "span"].contains(&field.as_str()) {
                continue;
            }
            if value.get("k").is_some() {
                field_nodes.push(value);
            } else if let Some(values) = value.as_array() {
                field_nodes.extend(values.iter().filter(|value| value.get("k").is_some()));
            }
        }
    }
    let mut at = 0;
    while at < pending.len() {
        if at >= 100_000 {
            return Err(fail("the leaf body exceeds its expression resource limit"));
        }
        let node = pending[at];
        at += 1;
        value_type(&node.ty)?;
        let qualified = matches!(&node.node, Node::Binary { rounding, by_zero, .. } if rounding.is_some() || by_zero.is_some());
        if !accept_rust_source_header_emission_expression(node.kind())
            || !accept_shared_numeric_expression_shape(node.kind(), qualified)
        {
            return Err(fail(
                "calls and effects require a distinct interface transformation proof",
            ));
        }
        match &node.node {
            Node::Unary { op, semantics, .. } => {
                let operation = match op {
                    UnaryOp::Neg => "neg",
                    UnaryOp::Not => "not",
                };
                if !accept_shared_numeric_unary(
                    operation,
                    node.ty.kind(),
                    semantics_name(*semantics),
                ) {
                    return Err(fail(
                        "the unary operation has no admitted primitive value interpretation",
                    ));
                }
            }
            Node::Binary {
                op,
                domain,
                semantics,
                ..
            } => {
                let domain = domain.as_ref().ok_or_else(|| {
                    fail("only primitive binary64 and Boolean value types are admitted")
                })?;
                value_type(domain)?;
                if !accept_shared_numeric_binary(
                    op.name(),
                    domain.kind(),
                    semantics_name(*semantics),
                ) {
                    return Err(fail(
                        "the binary operation has no admitted primitive value interpretation",
                    ));
                }
            }
            _ => {}
        }
        pending.extend(node.children());
    }
    Ok(())
}

/// Render actual ordered header facts around the checked, renamed value body.
#[must_use]
pub fn render_rust_source_header_emission(
    signature: &SourceSignature,
    parameters: &[Param],
    name: &str,
    body: &str,
) -> String {
    render_rust_source_header_emission_with_reference(
        signature,
        parameters,
        name,
        body,
        |lifetime, inner, _| render_rust_source_header_reference(lifetime, inner),
    )
}

/// Render exact source headers with the selected shared reference spelling.
#[must_use]
pub fn render_rust_source_header_emission_with_reference(
    signature: &SourceSignature,
    parameters: &[Param],
    name: &str,
    body: &str,
    reference: fn(&str, &str, bool) -> String,
) -> String {
    let mut copies = Vec::new();
    let parameters = parameters
        .iter()
        .zip(&signature.parameters)
        .map(|(parameter, position)| {
            let mut ty = read_rust_source_header_primitive(position.ty.kind());
            let mut dereferenced = parameter.name.clone();
            for layer in (0..position.wrappers.len()).rev() {
                ty = reference(
                    position.lifetimes[layer].as_deref().unwrap_or(""),
                    &ty,
                    layer < position.wrappers.len() - 1,
                );
                dereferenced = render_rust_source_header_dereference(&dereferenced);
            }
            if !position.wrappers.is_empty() {
                copies.push(render_rust_source_header_copy(
                    &parameter.name,
                    &dereferenced,
                ));
            }
            render_rust_source_header_parameter(&parameter.name, &ty)
        })
        .collect::<Vec<_>>();
    copies.push(body.to_owned());
    let lifetimes = signature
        .lifetime_bindings
        .iter()
        .map(|name| render_rust_source_header_lifetime(name))
        .collect::<Vec<_>>();
    render_rust_source_header_function(
        name,
        &lifetimes.join(", "),
        &parameters.join(", "),
        &read_rust_source_header_primitive(signature.result.ty.kind()),
        &indent(&copies.join("\n"), 1),
    )
}
