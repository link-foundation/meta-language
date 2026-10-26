//! Native projection of owned IR into the automatically translated interface.
use super::diagnostics::{Result, unsupported};
use super::emit_rust::indent;
use super::ir::{Decl, Node, Param, Program};
use super::rust_scalar_mathematical_interface_rules::{
    accept_rust_scalar_mathematical_expression, accept_rust_scalar_mathematical_interface_model,
    accept_rust_scalar_mathematical_operation,
};
use super::rust_scalar_source_interface_rules::{
    accept_rust_scalar_source_budget, accept_rust_scalar_source_call,
    accept_rust_scalar_source_context, accept_rust_scalar_source_declaration,
    accept_rust_scalar_source_declaration_field, accept_rust_scalar_source_expression,
    accept_rust_scalar_source_interface_model, accept_rust_scalar_source_position,
    read_rust_scalar_source_type, read_rust_scalar_value_type,
    render_rust_scalar_source_conversion,
};
use super::rust_textual_source_interface_rules::{
    accept_rust_textual_source_binary, accept_rust_textual_source_interface_model,
    accept_rust_textual_source_operation, read_rust_textual_source_type,
    render_rust_textual_source_input, render_rust_textual_source_slice_argument,
};
use super::source_signatures::{SourceSignature, SourceTypeSignature, validate_source_signature};
use super::types::Type;
use std::collections::{BTreeMap, BTreeSet};

fn fail(detail: &str) -> super::diagnostics::TranslationError {
    unsupported("Rust scalar source interface", detail, None)
}

fn count(length: usize) -> f64 {
    u32::try_from(length).map_or(f64::INFINITY, f64::from)
}

fn value_type(ty: &Type, strings: bool) -> Result<String> {
    if strings && matches!(ty, Type::String) {
        return Ok("String".to_owned());
    }
    let (bits, signed) = match ty {
        Type::Fixed { bits, signed } => (f64::from(*bits), *signed),
        _ => (0.0, false),
    };
    let name = read_rust_scalar_value_type(ty.kind(), bits, signed);
    if name.is_empty() {
        return Err(fail(
            "every interface and expression needs an actual scalar value type",
        ));
    }
    Ok(name)
}

fn source_type(position: &SourceTypeSignature, strings: bool) -> Result<String> {
    let value = value_type(&position.ty, strings)?;
    if value == "String" {
        return Ok(read_rust_textual_source_type(
            &position.owner.kind,
            &position.owner.path.join("::"),
            count(position.owner.arguments.len()),
        ));
    }
    Ok(read_rust_scalar_source_type(
        &value,
        &position.owner.kind,
        position.owner.path.first().map_or("", String::as_str),
        count(position.owner.path.len()),
        count(position.owner.arguments.len()),
    ))
}

/// Admit every owned header and direct call before changing any interface.
pub fn read_rust_scalar_source_interface(
    program: &Program,
    model: &str,
) -> Result<BTreeMap<String, SourceSignature>> {
    let strings = accept_rust_textual_source_interface_model(model);
    let mathematical = strings || accept_rust_scalar_mathematical_interface_model(model);
    if !accept_rust_scalar_source_interface_model(model) && !mathematical {
        return Err(fail("select retained scalar Rust function interface 1"));
    }
    super::source_primitive_owners::validate_source_primitive_owners(program)?;
    super::source_type_namespaces::validate_source_type_namespaces(program)?;
    let forbidden = !program.imports.is_empty()
        || !program.externals.is_empty()
        || program.main.as_ref().is_some_and(|main| {
            !main.effects.is_empty()
                || main.sequential_async
                || main.source_runtime_worker.is_some()
        })
        || program.reference_semantics
        || program.utf16_strings.unwrap_or(false)
        || program.output_threaded
        || program.aborts_threaded
        || program.source_runtime_shell.is_some()
        || program.source_pure_unit_calls.is_some();
    if !accept_rust_scalar_source_context(program.source_language.as_str(), forbidden) {
        return Err(fail(
            "only closed synchronous Rust scalar modules without imports or runtime effects are admitted",
        ));
    }
    let policy = super::source_header_emission::source_header_policy();
    let mut headers = BTreeMap::new();
    let mut pending = Vec::new();
    for declaration in &program.declarations {
        let Decl::Fn(function) = declaration else {
            return Err(fail(
                "every interface needs its actual original owned function declaration",
            ));
        };
        if !accept_rust_scalar_source_declaration(
            declaration.kind(),
            function.generated,
            !headers.contains_key(&function.full_name),
        ) {
            return Err(fail(
                "every interface needs its actual original owned function declaration",
            ));
        }
        let fields = serde_json::to_value(declaration).map_err(|_| {
            fail("the declaration contains unrepresented source interface qualifiers")
        })?;
        if fields.as_object().unwrap().keys().any(|field| {
            !accept_rust_scalar_source_declaration_field(
                field,
                policy["declarationFields"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|allowed| allowed.as_str() == Some(field.as_str())),
            )
        }) {
            return Err(fail(
                "the declaration contains unrepresented source interface qualifiers",
            ));
        }
        let signature = function.source_signature.as_ref().ok_or_else(|| {
            fail("every original function needs its retained version-two source signature")
        })?;
        validate_source_signature(
            signature,
            &function
                .params
                .iter()
                .map(|parameter| parameter.ty.clone())
                .collect::<Vec<_>>(),
            &function.ret,
            program.source_language.as_str(),
        )?;
        for lifetime in &signature.lifetime_bindings {
            if lifetime.is_empty() || lifetime.bytes().enumerate().any(|(index, character)| {
                !super::source_header_emission_rules::accept_rust_source_header_lifetime_character(
                    f64::from(character),
                    index == 0,
                )
            }) {
                return Err(fail("source lifetimes need represented ASCII identifiers"));
            }
        }
        for (index, parameter) in function.params.iter().enumerate() {
            if parameter.guard.is_some()
                || parameter.default_value.is_some()
                || fields["params"][index]
                    .as_object()
                    .unwrap()
                    .keys()
                    .any(|field| {
                        !policy["parameterFields"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .any(|allowed| allowed.as_str() == Some(field.as_str()))
                    })
            {
                return Err(fail(
                    "parameter guards, defaults and additional qualifiers need their own interface interpretation",
                ));
            }
        }
        for (position, result) in signature
            .parameters
            .iter()
            .map(|position| (position, false))
            .chain(std::iter::once((&signature.result, true)))
        {
            if !accept_rust_scalar_source_position(
                &signature.model,
                &source_type(position, strings)?,
                position
                    .wrappers
                    .iter()
                    .any(|wrapper| wrapper != "reference"),
                result && !position.wrappers.is_empty(),
            ) {
                return Err(fail(
                    "version-two scalar owners, immutable-reference inputs and direct results are required",
                ));
            }
        }
        headers.insert(function.full_name.clone(), signature.clone());
        if !accept_rust_scalar_source_budget(count(pending.len().saturating_add(1))) {
            return Err(fail(
                "the closed scalar call graph exceeds its expression budget",
            ));
        }
        pending.push(&function.body);
    }
    let mut visited = BTreeSet::new();
    let mut at = 0;
    while at < pending.len() {
        let node = pending[at];
        at += 1;
        if !visited.insert(std::ptr::from_ref(node)) {
            continue;
        }
        if !accept_rust_scalar_source_budget(count(visited.len())) {
            return Err(fail(
                "the closed scalar call graph exceeds its expression budget",
            ));
        }
        value_type(&node.ty, strings)?;
        // Native variants exhaustively represent these fields. Linked decoding
        // checks raw field inventories before constructing this IR.
        if !(if mathematical {
            accept_rust_scalar_mathematical_expression(node.kind())
        } else {
            accept_rust_scalar_source_expression(node.kind())
        }) {
            return Err(fail(
                "the expression requires an additional source interface interpretation",
            ));
        }
        match &node.node {
            Node::Math { op, args } => {
                if !(if strings {
                    accept_rust_textual_source_operation(op, count(args.len()), node.ty.kind())
                } else {
                    accept_rust_scalar_mathematical_operation(op, count(args.len()), node.ty.kind())
                }) {
                    return Err(fail(
                        "the mathematical operation needs its admitted scalar result and complete operands",
                    ));
                }
            }
            Node::Call { func, args } => {
                let target = match program.declaration(func) {
                    Some(Decl::Fn(target)) => Some(target),
                    _ => None,
                };
                let signature = headers.get(func);
                let arguments = target.is_some_and(|target| {
                    args.len() == target.params.len()
                        && args
                            .iter()
                            .zip(&target.params)
                            .all(|(argument, parameter)| argument.ty == parameter.ty)
                });
                let result = target.is_some_and(|target| node.ty == target.ret);
                let static_borrow = signature.is_some_and(|signature| {
                    signature.parameters.iter().any(|position| {
                        position
                            .lifetimes
                            .iter()
                            .flatten()
                            .any(|lifetime| lifetime == "static")
                    })
                });
                if !accept_rust_scalar_source_call(
                    target.is_some_and(|target| target.full_name == *func) && signature.is_some(),
                    arguments,
                    result,
                    static_borrow,
                ) {
                    return Err(fail(
                        "calls need owned scalar targets with matching checked types and nonstatic copied borrows",
                    ));
                }
            }
            Node::Binary { op, domain, .. } => {
                let domain = domain.as_ref().ok_or_else(|| {
                    fail("every interface and expression needs an actual scalar value type")
                })?;
                value_type(domain, strings)?;
                if strings && !accept_rust_textual_source_binary(domain.kind(), op.name()) {
                    return Err(fail(
                        "string ordering needs its own source interface interpretation",
                    ));
                }
            }
            Node::Cast { from, to, .. } => {
                value_type(from, false)?;
                value_type(to, false)?;
            }
            _ => {}
        }
        let children = node.children();
        if !accept_rust_scalar_source_budget(count(pending.len().saturating_add(children.len()))) {
            return Err(fail(
                "the closed scalar call graph exceeds its expression budget",
            ));
        }
        pending.extend(children);
    }
    Ok(headers)
}

/// Adapt actual scalar endpoints using shared syntax and conversion definitions.
pub fn render_rust_scalar_source_header(
    signature: &SourceSignature,
    params: &[Param],
    name: &str,
    body: &str,
) -> Result<String> {
    use super::borrowed_call_interface_rules::render_rust_borrowed_call_header_reference;
    use super::source_header_emission_rules::{
        render_rust_source_header_copy, render_rust_source_header_dereference,
        render_rust_source_header_function, render_rust_source_header_lifetime,
        render_rust_source_header_parameter,
    };
    let mut copies = Vec::new();
    let mut parameters = Vec::new();
    for (parameter, position) in params.iter().zip(&signature.parameters) {
        let source = source_type(position, true)?;
        let mut ty = source.clone();
        let mut value = parameter.name.clone();
        for layer in (0..position.wrappers.len()).rev() {
            ty = render_rust_borrowed_call_header_reference(
                position.lifetimes[layer].as_deref().unwrap_or(""),
                &ty,
                layer < position.wrappers.len() - 1,
            );
            value = render_rust_source_header_dereference(&value);
        }
        let converted = if matches!(position.ty, Type::String) && !position.wrappers.is_empty() {
            render_rust_textual_source_input(&value)
        } else {
            render_rust_scalar_source_conversion(&value, &source, false)
        };
        if !position.wrappers.is_empty() || converted != value {
            copies.push(render_rust_source_header_copy(&parameter.name, &converted));
        }
        parameters.push(render_rust_source_header_parameter(&parameter.name, &ty));
    }
    let result = source_type(&signature.result, true)?;
    copies.push(render_rust_scalar_source_conversion(body, &result, true));
    Ok(render_rust_source_header_function(
        name,
        &signature
            .lifetime_bindings
            .iter()
            .map(|lifetime| render_rust_source_header_lifetime(lifetime))
            .collect::<Vec<_>>()
            .join(", "),
        &parameters.join(", "),
        &result,
        &indent(&copies.join("\n"), 1),
    ))
}

/// Materialize each argument once in order, convert, borrow and restore core.
pub fn render_rust_scalar_source_call(
    signature: &SourceSignature,
    head: &str,
    values: &[String],
    names: &[String],
) -> Result<String> {
    use super::borrowed_call_interface_rules::{
        render_rust_borrowed_call_block, render_rust_borrowed_call_invocation,
        render_rust_borrowed_call_reference,
    };
    use super::source_header_emission_rules::render_rust_source_header_copy;
    let mut bindings = Vec::new();
    let mut arguments = Vec::new();
    for ((value, name), position) in values.iter().zip(names).zip(&signature.parameters) {
        bindings.push(render_rust_source_header_copy(
            name,
            &render_rust_scalar_source_conversion(value, &source_type(position, true)?, true),
        ));
        let slice = source_type(position, true)? == "str";
        let mut argument = if slice {
            render_rust_textual_source_slice_argument(name)
        } else {
            name.clone()
        };
        for _ in position.wrappers.iter().skip(usize::from(slice)) {
            argument = render_rust_borrowed_call_reference(&argument);
        }
        arguments.push(argument);
    }
    bindings.push(render_rust_scalar_source_conversion(
        &render_rust_borrowed_call_invocation(head, &arguments.join(", ")),
        &source_type(&signature.result, true)?,
        false,
    ));
    Ok(render_rust_borrowed_call_block(&indent(
        &bindings.join("\n"),
        1,
    )))
}
