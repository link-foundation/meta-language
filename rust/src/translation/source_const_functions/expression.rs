//! Actual primitive parameter occurrences interpreted by shared const rules.
use super::{Result, Value, fail, fields, interval};
use crate::translation::rust::const_function_rules::{
    accept_rust_const_function_binary, accept_rust_const_function_finite_literal,
    accept_rust_const_function_type,
};
use crate::translation::rust_const_branch_rules::{
    accept_rust_const_branch_conditional, accept_rust_const_branch_kind,
    accept_rust_const_branch_model, accept_rust_const_branch_operation,
    accept_rust_const_branch_type,
};
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

fn literal(node: &Value) -> Result<()> {
    let valid = if node["type"]["kind"] == "bool" {
        node["value"].is_boolean()
    } else if let Some(value) = node["value"].as_str() {
        static SPELLING: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
                r"^(?:-?(?:[0-9]+(?:\.[0-9]+)?)(?:[eE][+-]?[0-9]+)?|NaN|Infinity|-Infinity)$",
            )
            .expect("the shared primitive number spelling is valid")
        });
        SPELLING.is_match(value)
            && (matches!(value, "NaN" | "Infinity" | "-Infinity")
                || value
                    .parse::<f64>()
                    .is_ok_and(accept_rust_const_function_finite_literal))
    } else {
        false
    };
    if !valid {
        return Err(fail("a literal has an invalid primitive value"));
    }
    Ok(())
}

pub(super) fn validate(
    node: &Value,
    environment: &HashMap<String, Value>,
    selected: &Value,
    body: (u64, u64),
    depth: usize,
    visits: &mut usize,
) -> Result<()> {
    *visits += 1;
    if *visits > 100_000 || depth > 256 {
        return Err(fail("the predicate exceeds its semantic traversal limit"));
    }
    let kind = node["k"].as_str().unwrap_or_default();
    let branching = accept_rust_const_branch_model(selected["model"].as_str().unwrap_or_default());
    let allowed = selected["fields"]["expressions"]
        .get(kind)
        .ok_or_else(|| fail("the expression has no primitive const predicate interpretation"))?;
    if branching && !accept_rust_const_branch_kind(kind) {
        return Err(fail(
            "the expression has no primitive const predicate interpretation",
        ));
    }
    fields(node, allowed, false)?;
    fields(&node["type"], &serde_json::json!(["kind"]), true)?;
    let ty = node["type"]["kind"].as_str().unwrap_or_default();
    if !(if branching {
        accept_rust_const_branch_type(ty)
    } else {
        accept_rust_const_function_type(ty)
    }) {
        return Err(fail("the expression changes its primitive value domain"));
    }
    if !node["span"].is_null() {
        let site = interval(&node["span"], false)?;
        if site.0 < body.0 || site.1 > body.1 {
            return Err(fail("an expression escaped its actual owned source body"));
        }
    }
    match kind {
        "lit" => literal(node)?,
        "var" => {
            if !node["name"]
                .as_str()
                .is_some_and(|name| environment.get(name) == Some(&node["type"]))
            {
                return Err(fail("a value lacks its actual primitive parameter owner"));
            }
        }
        "unary" => {
            validate(&node["arg"], environment, selected, body, depth + 1, visits)?;
            if node["op"] != "not"
                || node["type"]["kind"] != "bool"
                || node["arg"]["type"]["kind"] != "bool"
                || branching
                    && !accept_rust_const_branch_operation(
                        kind,
                        node["op"].as_str().unwrap_or_default(),
                    )
            {
                return Err(fail("only primitive Boolean negation is admitted"));
            }
        }
        "if" => {
            for field in ["cond", "then", "else"] {
                validate(&node[field], environment, selected, body, depth + 1, visits)?;
            }
            if !branching
                || !accept_rust_const_branch_conditional(
                    ty,
                    node["cond"]["type"]["kind"].as_str().unwrap_or_default(),
                    node["then"]["type"]["kind"].as_str().unwrap_or_default(),
                    node["else"]["type"]["kind"].as_str().unwrap_or_default(),
                )
            {
                return Err(fail(
                    "the conditional needs actual primitive Boolean condition and result branches",
                ));
            }
        }
        _ => {
            validate(
                &node["left"],
                environment,
                selected,
                body,
                depth + 1,
                visits,
            )?;
            validate(
                &node["right"],
                environment,
                selected,
                body,
                depth + 1,
                visits,
            )?;
            let domain = &node["left"]["type"];
            let domain_kind = domain["kind"].as_str().unwrap_or_default();
            if node["type"]["kind"] != "bool"
                || domain != &node["right"]["type"]
                || !(if branching {
                    accept_rust_const_branch_operation(
                        kind,
                        node["op"].as_str().unwrap_or_default(),
                    )
                } else {
                    accept_rust_const_function_binary(
                        domain_kind,
                        node["op"].as_str().unwrap_or_default(),
                    )
                })
                || node.get("domain").is_some_and(|value| value != domain)
                || node.get("semantics").is_some_and(|value| {
                    value.as_str()
                        != Some(if domain_kind == "float" {
                            "ieee"
                        } else {
                            "exact"
                        })
                })
            {
                return Err(fail(
                    "the comparison has an unproved primitive operation or domain",
                ));
            }
        }
    }
    Ok(())
}
