//! Refine a failed selected call recheck using shared diagnostic instructions.
use super::{TypedProgramLinksError, TypedProgramLinksLimits, checked, codec, error};
use crate::translation::borrowed_call_interface::borrowed_call_policy;
use crate::translation::borrowed_call_interface_rules::{
    accept_rust_borrowed_call_interface_model, read_rust_borrowed_call_target_diagnostic,
};
use crate::translation::ir::{Decl, Node, Program};
use crate::translation::source_header_emission::{
    read_rust_primitive_source_headers, validate_rust_primitive_value_expressions,
};
use crate::{LinkId, LinkNetwork};

fn refine_failure(program: &Program, model: &str) -> Option<String> {
    if !accept_rust_borrowed_call_interface_model(model) {
        return None;
    }
    read_rust_primitive_source_headers(program).ok()?;
    // Existing whole-header and value checks bound these projected facts to
    // the selected primitive interface. They do not replace checked validation.
    for declaration in &program.declarations {
        let Decl::Fn(function) = declaration else {
            return None;
        };
        if let Node::Call { args, .. } = &function.body.node {
            validate_rust_primitive_value_expressions(args).ok()?;
        } else {
            validate_rust_primitive_value_expressions([&function.body]).ok()?;
        }
    }
    let mut diagnostics = Vec::new();
    for declaration in &program.declarations {
        let Decl::Fn(function) = declaration else {
            return None;
        };
        let Node::Call { func, args } = &function.body.node else {
            continue;
        };
        let Some(Decl::Fn(target)) = program.declaration(func) else {
            return None;
        };
        if args.len() != target.params.len()
            || function.body.ty != target.ret
            || function.ret != function.body.ty
        {
            return None;
        }
        let status = if args
            .iter()
            .zip(&target.params)
            .all(|(argument, parameter)| argument.ty == parameter.ty)
        {
            "matched"
        } else {
            "mismatch"
        };
        let diagnostic = read_rust_borrowed_call_target_diagnostic(
            model,
            function.body.kind(),
            target.body.kind(),
            status,
        );
        if !diagnostic.is_empty() {
            diagnostics.push(diagnostic);
        }
    }
    // The linked row order preserves complete-program argument validation
    // before selected target-role validation, independent of declaration order.
    for row in borrowed_call_policy()["targetDiagnostics"].as_array()? {
        let diagnostic = row.get(2)?.as_str()?;
        if diagnostics.iter().any(|value| value == diagnostic) {
            return Some(diagnostic.to_owned());
        }
    }
    None
}

pub(super) fn decode(
    network: &LinkNetwork,
    root: LinkId,
    model: &str,
) -> std::result::Result<Program, (TypedProgramLinksError, bool)> {
    // Structural decoding, ownership and malformed-field refusal run first.
    let program = codec::decode(network, root, TypedProgramLinksLimits::default())
        .map_err(|failure| (failure, false))?;
    if let Err(failure) = checked::validate(&program) {
        return Err(refine_failure(&program, model)
            .map_or((failure, false), |diagnostic| (error(diagnostic), true)));
    }
    Ok(program)
}
