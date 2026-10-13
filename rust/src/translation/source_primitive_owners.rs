//! Host record adapters for the inspectable standard primitive source proof.
//! Types and binder scope remain in the existing checker; shared generated
//! decisions select the admitted primitive owners, values and expressions.
use super::diagnostics::{Result, type_error};
use super::ir::{Decl, Item, Program};
use super::rust::primitive_owner_rules::{
    accept_rust_primitive_owner_expression, accept_rust_primitive_owner_value_type,
    accept_rust_standard_primitive_owner, read_rust_primitive_owner_constant_value,
};
use serde_json::{Value, json};
use std::sync::LazyLock;

#[must_use]
pub(crate) fn policy() -> &'static Value {
    static POLICY: LazyLock<Value> = LazyLock::new(|| {
        crate::read_structured_data_lino(include_str!("../data/rust-primitive-owners.lino"))
            .expect("the shared primitive owner policy is valid")
    });
    &POLICY
}
fn fail(detail: &str) -> super::diagnostics::TranslationError {
    type_error(format!("Rust primitive owner proof: {detail}"), None)
}
fn exact(value: &Value, fields: &[&str]) -> Result<()> {
    if value.as_object().is_none_or(|record| {
        record.len() != fields.len() || fields.iter().any(|name| !record.contains_key(*name))
    }) {
        return Err(fail("unrecognized or missing owned fields"));
    }
    Ok(())
}
fn array<'a>(value: &'a Value, detail: &str) -> Result<&'a [Value]> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| fail(detail))
}
fn spelling(value: &Value) -> Result<String> {
    let parts = array(value, "an actual source owner path is malformed")?;
    if parts
        .iter()
        .any(|part| part.as_str().is_none_or(str::is_empty))
    {
        return Err(fail("an actual source owner path is malformed"));
    }
    Ok(parts
        .iter()
        .map(|part| part.as_str().unwrap())
        .collect::<Vec<_>>()
        .join("::"))
}
fn owner(position: &Value, imports: &[Value]) -> Result<()> {
    exact(position, &["sourcePath", "resolvedPath", "alias"])?;
    spelling(&position["sourcePath"])?;
    let path = spelling(&position["resolvedPath"])?;
    if position["resolvedPath"] != json!(path.split("::").collect::<Vec<_>>())
        || (path != "f64" && !accept_rust_standard_primitive_owner(&path, false, false))
    {
        return Err(fail("an actual owner is not a standard primitive reexport"));
    }
    if position["alias"].is_null() {
        if position["sourcePath"] != position["resolvedPath"] {
            return Err(fail("an unaliased source path differs from its owner"));
        }
    } else if position["alias"].as_str().is_none()
        || position["sourcePath"] != json!([position["alias"]])
        || !imports.iter().any(|alias| {
            alias["name"] == position["alias"] && alias["target"] == position["resolvedPath"]
        })
    {
        return Err(fail(
            "the actual imported source owner differs from its owned alias resolution",
        ));
    }
    Ok(())
}
fn body_sites(body: &Value) -> Result<Vec<(&Value, Vec<Value>)>> {
    let mut pending = vec![(body, Vec::new())];
    let mut sites = Vec::new();
    let mut visits = 0;
    while let Some((value, path)) = pending.pop() {
        visits += 1;
        if visits > 100_000 {
            return Err(fail("the body exceeds its owned-site limit"));
        }
        if value["k"] == "lit" {
            sites.push((value, path.clone()));
        }
        if let Some(record) = value.as_object() {
            for (key, child) in record {
                if ["span", "type", "domain", "from", "to"].contains(&key.as_str()) {
                    continue;
                }
                if let Some(children) = child.as_array() {
                    for (index, child) in children.iter().enumerate().rev() {
                        if child.is_object() {
                            let mut path = path.clone();
                            path.extend([json!(key), json!(index)]);
                            pending.push((child, path));
                        }
                    }
                } else if child.is_object() {
                    let mut path = path.clone();
                    path.push(json!(key));
                    pending.push((child, path));
                }
            }
        }
    }
    Ok(sites)
}
pub(crate) fn prepare_primitive_owner_proof(
    proof: &Value,
    body: &super::ir::Expr,
) -> Result<Value> {
    let mut result = proof.clone();
    let body = serde_json::to_value(body).map_err(|_| fail("a checked leaf body is required"))?;
    let sites = body_sites(&body)?;
    let constants = result["constants"]
        .as_array_mut()
        .ok_or_else(|| fail("owner positions differ from their declaration"))?;
    for constant in constants {
        if !constant["path"].is_null() {
            continue;
        }
        let expected =
            read_rust_primitive_owner_constant_value(constant["name"].as_str().unwrap_or_default());
        let matches = sites
            .iter()
            .filter(|(site, _)| {
                site["span"] == constant["span"]
                    && site["type"]["kind"] == "float"
                    && site["value"] == expected
            })
            .collect::<Vec<_>>();
        if matches.len() != 1 {
            return Err(fail(
                "a source constant must own exactly one checked literal site",
            ));
        }
        constant["path"] = json!(matches[0].1);
    }
    Ok(result)
}
fn position(value: &Value, ty: &Value, signature: Option<&Value>, imports: &[Value]) -> Result<()> {
    if !accept_rust_primitive_owner_value_type(ty["kind"].as_str().unwrap_or_default())
        || ty.as_object().is_none_or(|record| record.len() != 1)
    {
        return Err(fail("an interface contains a nonprimitive value type"));
    }
    if ty["kind"] != "float" {
        if !value.is_null() {
            return Err(fail(
                "binary64 source evidence belongs to a different parameter",
            ));
        }
        return Ok(());
    }
    exact(
        value,
        &[
            "sourcePath",
            "resolvedPath",
            "alias",
            "wrappers",
            "lifetimes",
        ],
    )?;
    owner(
        &json!({"sourcePath":value["sourcePath"],"resolvedPath":value["resolvedPath"],"alias":value["alias"]}),
        imports,
    )?;
    let wrappers = array(
        &value["wrappers"],
        "source owner reference layers are inconsistent",
    )?;
    let lifetimes = array(
        &value["lifetimes"],
        "source owner reference layers are inconsistent",
    )?;
    if wrappers.iter().any(|layer| layer != "reference")
        || lifetimes.len() != wrappers.len()
        || lifetimes
            .iter()
            .any(|lifetime| !lifetime.is_null() && lifetime != "static" && lifetime != "_")
    {
        return Err(fail("source owner reference layers are inconsistent"));
    }
    if signature.is_some_and(|signature| {
        signature["wrappers"] != value["wrappers"]
            || signature["lifetimes"] != value["lifetimes"]
            || signature["owner"]["kind"] != "primitive"
            || signature["owner"]["path"] != json!(["f64"])
    }) {
        return Err(fail(
            "the canonical signature disagrees with its actual qualified source owner",
        ));
    }
    Ok(())
}
/// Validate raw field shapes before native serde can normalize unknown fields.
pub(crate) fn validate_primitive_owner_records(value: &Value) -> Result<()> {
    let declarations = array(&value["declarations"], "owner declarations are not ordered")?;
    if !declarations
        .iter()
        .any(|entry| entry.get("sourcePrimitiveOwners").is_some())
    {
        return Ok(());
    }
    let mut import_table: Option<&Value> = None;
    for entry in declarations {
        if entry.as_object().is_none_or(|record| {
            record.keys().any(|key| {
                !policy()["declarationFields"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|field| field.as_str() == Some(key.as_str()))
            })
        }) {
            return Err(fail("a primitive function has unrepresented own fields"));
        }
        if !entry["modulePath"].is_array()
            || entry["recursive"] != Value::Bool(false)
            || !entry["decreasing"].is_null()
            || entry
                .get("mutual")
                .is_some_and(|value| value != &Value::Bool(false))
        {
            return Err(fail(
                "the declared function context contradicts its primitive leaf ownership",
            ));
        }
        if entry
            .get("generated")
            .is_some_and(|value| value != &Value::Bool(false))
            || entry.get("sourceLoop").is_some()
        {
            return Err(fail(
                "the selected model requires a complete flat primitive leaf program without declared types, imports or effects",
            ));
        }
        if entry
            .get("sourceSignature")
            .is_some_and(|signature| !signature.is_object())
        {
            return Err(fail(
                "a present source signature must retain its owned record",
            ));
        }
        let proof = &entry["sourcePrimitiveOwners"];
        exact(
            proof,
            &[
                "model",
                "language",
                "namespace",
                "parameters",
                "result",
                "constants",
            ],
        )?;
        if proof["model"] != policy()["model"] || proof["language"] != "Rust" {
            return Err(fail("unknown selected source interpretation"));
        }
        exact(
            &proof["namespace"],
            &["declaredTypeOwners", "externalPrimitiveRoots", "imports"],
        )?;
        if proof["namespace"]["declaredTypeOwners"] != json!([])
            || proof["namespace"]["externalPrimitiveRoots"] != json!(["std", "core"])
        {
            return Err(fail(
                "the actual declared-type namespace contradicts its primitive premise",
            ));
        }
        let imports = array(
            &proof["namespace"]["imports"],
            "the actual declared-type namespace contradicts its primitive premise",
        )?;
        if import_table.is_some_and(|table| table != &proof["namespace"]["imports"]) {
            return Err(fail(
                "function owners disagree about their shared import namespace",
            ));
        }
        import_table = Some(&proof["namespace"]["imports"]);
        let mut names = std::collections::HashSet::new();
        for alias in imports {
            exact(alias, &["name", "target"])?;
            let name = alias["name"].as_str().unwrap_or_default();
            let path = spelling(&alias["target"])?;
            if !super::rust::primitive_owners::accept_import_name(name)
                || !names.insert(name)
                || alias["target"] != json!(path.split("::").collect::<Vec<_>>())
                || !accept_rust_standard_primitive_owner(&path, false, false)
            {
                return Err(fail(
                    "an import alias has no admitted standard primitive target",
                ));
            }
        }
        let parameters = array(
            &entry["params"],
            "owner positions differ from their declaration",
        )?;
        let positions = array(
            &proof["parameters"],
            "owner positions differ from their declaration",
        )?;
        if parameters.len() != positions.len() {
            return Err(fail("owner positions differ from their declaration"));
        }
        for (index, (parameter, source)) in parameters.iter().zip(positions).enumerate() {
            if parameter.as_object().is_none_or(|record| {
                record.keys().any(|key| {
                    !policy()["parameterFields"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|field| field.as_str() == Some(key.as_str()))
                })
            }) {
                return Err(fail("a primitive parameter has unrepresented own fields"));
            }
            if !parameter["guard"].is_null() || !parameter["default"].is_null() {
                return Err(fail(
                    "parameter guards, defaults and optionality need a separate owner interface proof",
                ));
            }
            position(
                source,
                &parameter["type"],
                entry
                    .get("sourceSignature")
                    .map(|signature| &signature["parameters"][index]),
                imports,
            )?;
        }
        position(
            &proof["result"],
            &entry["ret"],
            entry
                .get("sourceSignature")
                .map(|signature| &signature["result"]),
            imports,
        )?;
        if proof["result"]["wrappers"]
            .as_array()
            .is_some_and(|wrappers| !wrappers.is_empty())
        {
            return Err(fail(
                "borrowed results need a separate region and source owner interpretation",
            ));
        }
        let sites = body_sites(&entry["body"])?;
        let mut used = std::collections::HashSet::new();
        for constant in array(
            &proof["constants"],
            "owner positions differ from their declaration",
        )? {
            exact(constant, &["owner", "name", "span", "path"])?;
            owner(&constant["owner"], imports)?;
            exact(&constant["span"], &["start", "end"])?;
            let start = constant["span"]["start"].as_u64();
            let end = constant["span"]["end"].as_u64();
            if !start
                .zip(end)
                .is_some_and(|(start, end)| start < end && end <= 9_007_199_254_740_991)
            {
                return Err(fail("an actual constant source span is malformed"));
            }
            let expected = read_rust_primitive_owner_constant_value(
                constant["name"].as_str().unwrap_or_default(),
            );
            let path = array(
                &constant["path"],
                "a constant has an invalid or duplicate owned body site",
            )?;
            if expected.is_empty()
                || !used.insert(constant["path"].to_string())
                || path.iter().any(|part| {
                    !part.is_string()
                        && !part
                            .as_i64()
                            .is_some_and(|value| (0..=9_007_199_254_740_991).contains(&value))
                })
            {
                return Err(fail(
                    "a constant has an invalid or duplicate owned body site",
                ));
            }
            if !sites.iter().any(|(site, site_path)| {
                site_path.as_slice() == path
                    && site["type"]["kind"] == "float"
                    && site["value"] == expected
                    && site["span"] == constant["span"]
            }) {
                return Err(fail(
                    "the source constant disagrees with its checked body literal",
                ));
            }
        }
        let mut pending = vec![&entry["body"]];
        let mut visits = 0;
        while let Some(node) = pending.pop() {
            visits += 1;
            if visits > 100_000 {
                return Err(fail("the primitive leaf body exceeds its resource limit"));
            }
            if !accept_rust_primitive_owner_expression(node["k"].as_str().unwrap_or_default())
                || !accept_rust_primitive_owner_value_type(
                    node["type"]["kind"].as_str().unwrap_or_default(),
                )
                || node["type"]
                    .as_object()
                    .is_none_or(|record| record.len() != 1)
            {
                return Err(fail(
                    "the source profile requires primitive leaf value expressions",
                ));
            }
            if node.as_object().is_none_or(|record| {
                record.keys().any(|key| {
                    !policy()["expressionFields"][node["k"].as_str().unwrap()]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|field| field.as_str() == Some(key.as_str()))
                })
            }) {
                return Err(fail(
                    "a primitive body expression has unrepresented own fields",
                ));
            }
            for key in [
                "arg", "left", "right", "value", "body", "cond", "then", "else",
            ] {
                if node[key].is_object() {
                    pending.push(&node[key]);
                }
            }
            if let Some(args) = node["args"].as_array() {
                pending.extend(args);
            }
        }
    }
    Ok(())
}
/// Check actual declaration and context premises after ordinary semantic typing.
pub(crate) fn validate_source_primitive_owners(program: &Program) -> Result<()> {
    if !program.declarations.iter().any(
        |entry| matches!(entry,Decl::Fn(function)if function.source_primitive_owners.is_some()),
    ) {
        return Ok(());
    }
    if program.source_language!=super::Language::Rust||!program.imports.is_empty()||!program.externals.is_empty()||program.main.as_ref().is_some_and(|main|!main.effects.is_empty())||program.output_threaded||program.aborts_threaded||program.items.iter().any(|item|matches!(item,Item::Module{..}))||program.declarations.iter().any(|entry|!matches!(entry,Decl::Fn(function)if function.module_path.is_empty()&&!function.generated&&function.source_loop.is_none())){return Err(fail("the selected model requires a complete flat primitive leaf program without declared types, imports or effects"));}
    validate_primitive_owner_records(
        &serde_json::to_value(program).map_err(|_| fail("a checked leaf program is required"))?,
    )
}
