//! Thin owned-IR adapter for the shared primitive borrowed-call policy.
use super::borrowed_call_interface_rules::{
    accept_rust_borrowed_call_interface_model, accept_rust_borrowed_call_inventory,
    accept_rust_borrowed_call_shape, accept_rust_borrowed_call_target,
    accept_rust_borrowed_call_type, render_rust_borrowed_call_block,
    render_rust_borrowed_call_invocation, render_rust_borrowed_call_reference,
};
use super::diagnostics::{Result, unsupported};
use super::emit_rust::indent;
use super::ir::{Decl, Node, Program};
use super::source_header_emission::{
    read_rust_primitive_source_headers, validate_rust_primitive_value_expressions,
};
use super::source_header_emission_rules::render_rust_source_header_copy;
use super::source_signatures::SourceSignature;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

fn fail(detail: &str) -> super::diagnostics::TranslationError {
    unsupported("Rust borrowed call interface", detail, None)
}

#[must_use]
pub fn borrowed_call_policy() -> &'static Value {
    static POLICY: OnceLock<Value> = OnceLock::new();
    POLICY.get_or_init(|| {
        crate::read_structured_data_lino(include_str!("../data/rust-borrowed-call-interface.lino"))
            .expect("generated borrowed call policy is structured links data")
    })
}

/// Admission keeps actual header and checked callee identities together.
pub struct BorrowedCallInterface {
    pub headers: BTreeMap<String, SourceSignature>,
    pub targets: BTreeMap<String, SourceSignature>,
}

/// Admit a complete primitive leaf/direct-caller program before rendering.
pub fn read_rust_borrowed_call_interface(
    program: &Program,
    model: &str,
) -> Result<BorrowedCallInterface> {
    if !accept_rust_borrowed_call_interface_model(model) {
        return Err(unsupported(
            "Rust source header emission model",
            "select closed primitive borrowed Rust call interface 1",
            None,
        ));
    }
    let headers = read_rust_primitive_source_headers(program)?;
    let mut leaves = BTreeSet::new();
    let mut callers = Vec::new();
    let mut owned = BTreeSet::new();
    for declaration in &program.declarations {
        let Decl::Fn(function) = declaration else {
            unreachable!("primitive header admission requires function declarations")
        };
        if !owned.insert(&function.full_name) || function.full_name != function.name {
            return Err(fail(
                "the target must be an actual owned declaration identity",
            ));
        }
        if function.body.kind() == borrowed_call_policy()["callerKind"].as_str().unwrap() {
            callers.push(function);
        } else {
            validate_rust_primitive_value_expressions([&function.body])?;
            leaves.insert(function.full_name.as_str());
        }
    }
    if !accept_rust_borrowed_call_inventory(!callers.is_empty()) {
        return Err(fail(
            "the closed interface requires an actual direct caller",
        ));
    }
    let mut targets = BTreeMap::new();
    for caller in callers {
        let Node::Call { func, args } = &caller.body.node else {
            unreachable!("the generated caller-kind policy selects checked calls")
        };
        let target = program.declaration(func);
        let signature = headers.get(func);
        let static_lifetime = signature.is_some_and(|signature| {
            signature.parameters.iter().any(|position| {
                position.lifetimes.iter().flatten().any(|lifetime| {
                    borrowed_call_policy()["forbiddenTargetLifetimes"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|forbidden| forbidden.as_str() == Some(lifetime.as_str()))
                })
            })
        });
        if !accept_rust_borrowed_call_target(
            target.is_some_and(|target| target.full_name() == func) && signature.is_some(),
            leaves.contains(func.as_str()),
            static_lifetime,
        ) {
            return Err(fail(
                "every caller must target an owned pure leaf without a static borrowed input",
            ));
        }
        let Some(Decl::Fn(target)) = target else {
            unreachable!("primitive header admission proves the callee function")
        };
        // Node::Call has an exhaustive field set. Raw linked decoding rejects
        // unrepresented call fields before constructing the native IR.
        if !accept_rust_borrowed_call_shape(
            caller.body.kind(),
            false,
            args.len() == target.params.len(),
        ) {
            return Err(fail(
                "the actual checked call must retain its ordered arguments without unrepresented fields",
            ));
        }
        validate_rust_primitive_value_expressions(args)?;
        let argument_types = args
            .iter()
            .zip(&target.params)
            .all(|(argument, parameter)| argument.ty == parameter.ty);
        let result_types = caller.body.ty == target.ret && caller.ret == target.ret;
        if !accept_rust_borrowed_call_type(argument_types, result_types) {
            return Err(fail(
                "the caller and target must retain their checked primitive argument and result types",
            ));
        }
        targets.insert(func.clone(), signature.unwrap().clone());
    }
    Ok(BorrowedCallInterface { headers, targets })
}

/// Materialize ordered scalar values once, then recreate exact reference layers.
#[must_use]
pub fn render_rust_borrowed_call(
    signature: &SourceSignature,
    head: &str,
    arguments: &[String],
    names: &[String],
) -> String {
    let mut lines = arguments
        .iter()
        .zip(names)
        .map(|(value, name)| render_rust_source_header_copy(name, value))
        .collect::<Vec<_>>();
    let arguments = names
        .iter()
        .zip(&signature.parameters)
        .map(|(name, position)| {
            let mut value = name.clone();
            for _ in &position.wrappers {
                value = render_rust_borrowed_call_reference(&value);
            }
            value
        })
        .collect::<Vec<_>>();
    lines.push(render_rust_borrowed_call_invocation(
        head,
        &arguments.join(", "),
    ));
    render_rust_borrowed_call_block(&indent(&lines.join("\n"), 1))
}
