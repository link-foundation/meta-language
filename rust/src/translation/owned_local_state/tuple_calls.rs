//! Bind real declaration references and argument roles to the shared call rule.
use super::{Result, Value, array, fail, tuple_call_policy};
use crate::translation::rust::owned_tuple_call_rules::accept_rust_owned_tuple_call_initializer;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn declarations(program: &Value, entries: &[Value]) -> Result<BTreeMap<String, Value>> {
    let mut declarations = BTreeMap::new();
    for entry in entries {
        let name = entry["name"]
            .as_str()
            .ok_or_else(|| fail("the linked declaration mapping is inconsistent"))?;
        if entry["fullName"] != name
            || entry["modulePath"] != serde_json::json!([])
            || declarations
                .insert(name.to_owned(), entry.clone())
                .is_some()
        {
            return Err(fail("the linked declaration mapping is inconsistent"));
        }
    }
    let mut seen = BTreeSet::new();
    for item in array(
        &program["items"],
        "the linked declaration mapping and ordered items disagree",
    )? {
        let name = item["fullName"]
            .as_str()
            .ok_or_else(|| fail("the linked declaration item has no declaration"))?;
        if item["k"] != "decl" || !declarations.contains_key(name) || !seen.insert(name) {
            return Err(fail(
                "the linked ordered items contain a duplicate or unowned declaration",
            ));
        }
    }
    if seen.len() != declarations.len() {
        return Err(fail(
            "the linked declaration mapping and ordered items disagree",
        ));
    }
    Ok(declarations)
}

pub(super) fn parameters(parameters: &Value) -> bool {
    parameters.as_array().is_some_and(|parameters| {
        parameters.len() == 2
            && parameters[0]["type"]["kind"] == "string"
            && parameters[1]["type"]["kind"] == "fixed"
            && parameters[1]["type"]["bits"] == tuple_call_policy()["helper"]["integerBits"]
            && parameters[1]["type"]["signed"] == tuple_call_policy()["helper"]["integerSigned"]
    })
}

pub(super) fn target(
    declarations: &BTreeMap<String, Value>,
    entry: &Value,
    node: &Value,
    fact: &Value,
) -> Result<()> {
    let target = node["fn"].as_str().and_then(|name| declarations.get(name));
    let target_match = target.is_some_and(|target| {
        target["fullName"] != entry["fullName"]
            && target["k"] == "fn"
            && target["recursive"] == false
            && target["sourceLocalState"]["model"] == tuple_call_policy()["model"]
            && target["ret"]["kind"] == "tuple"
            && target["body"]["k"] == "tuple"
            && target["sourceLocalState"]["initializers"]
                .as_array()
                .is_some_and(Vec::is_empty)
            && parameters(&target["params"])
    });
    let arguments_match = target.filter(|_| target_match).is_some_and(|target| {
        match (node["args"].as_array(), target["params"].as_array()) {
            (Some(arguments), Some(parameters)) => {
                arguments.len() == parameters.len()
                    && arguments
                        .iter()
                        .zip(parameters)
                        .all(|(argument, parameter)| argument["type"] == parameter["type"])
                    && node["type"] == target["ret"]
            }
            _ => false,
        }
    });
    let fields_match = fact.is_object()
        && fact["target"] == node["fn"]
        && fact["fieldModes"] == tuple_call_policy()["fieldModes"];
    if !accept_rust_owned_tuple_call_initializer(target_match, arguments_match, fields_match) {
        return Err(fail(
            "the tuple call must retain its actual closed declaration target, argument domains and ordered field modes",
        ));
    }
    Ok(())
}
