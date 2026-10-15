//! Inspect raw owned fields and match every fact to the actual checked body.
use super::expression::{Scope, Slot, Validator};
use super::records::{Records, key};
use super::{
    Result, Value, allowed, array, contains, exact, fail, interval, owned_local_position,
    pair_type, policy, same_order, value_type,
};
use crate::translation::source_signatures::{SourceSignature, validate_source_signature};
use crate::translation::types::Type;
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn validate(program: &Value, ordered: bool) -> Result<()> {
    let Some(entries) = program["declarations"].as_array() else {
        return Ok(());
    };
    if !entries
        .iter()
        .any(|entry| entry.get("sourceLocalState").is_some())
    {
        return Ok(());
    }
    super::shapes::validate(program)?;
    let context_error = "the profile requires a complete flat value program without imports, effects or other runtime interpretations";
    if program["sourceLanguage"] != "Rust"
        || array(&program["items"], context_error)?
            .iter()
            .any(|item| item["k"] != "decl")
        || program["main"]
            .get("effects")
            .is_some_and(|value| value.as_array().is_none_or(|values| !values.is_empty()))
        || ["imports", "externals"].iter().any(|field| {
            program
                .get(*field)
                .is_some_and(|value| value.as_array().is_none_or(|values| !values.is_empty()))
        })
        || [
            "referenceSemantics",
            "utf16Strings",
            "outputThreaded",
            "abortsThreaded",
        ]
        .iter()
        .any(|field| {
            program
                .get(*field)
                .is_some_and(|value| value != &Value::Bool(false))
        })
    {
        return Err(fail(context_error));
    }
    for entry in entries {
        validate_entry(entry, ordered)?;
    }
    Ok(())
}

fn validate_entry(entry: &Value, ordered: bool) -> Result<()> {
    // Dynamic proof types retain their original Number and property order.
    // Read only a private fixed-width comparison view before typed serde.
    let mut view = entry.clone();
    crate::typed_program_links::program_numeric_metadata::normalize_program_integral_metadata(
        &mut view,
    )
    .map_err(|failure| fail(&failure.to_string()))?;
    let entry = &view;
    let policy = policy();
    if !allowed(entry, &policy["declarationFields"]) {
        return Err(fail("a function has unrepresented own fields"));
    }
    if entry["k"] != "fn"
        || entry["modulePath"] != json!([])
        || entry["recursive"] != false
        || ["generated", "mutual"].iter().any(|field| {
            entry
                .get(*field)
                .is_some_and(|value| value != &Value::Bool(false))
        })
        || !entry["decreasing"].is_null()
        || entry.get("sourceLoop").is_some()
        || entry.get("sourcePrimitiveOwners").is_some()
        || contains(&policy["reservedOwners"], &entry["name"])
    {
        return Err(fail(
            "a function has an unsupported declaration or standard owner context",
        ));
    }
    let proof = &entry["sourceLocalState"];
    exact(proof, &policy["fields"]["proof"])?;
    let function_span = interval(&entry["span"])?;
    let copies = proof["model"] == policy["models"]["copies"];
    if (!copies && proof["model"] != policy["models"]["moves"])
        || proof["language"] != "Rust"
        || proof["sourceName"] != entry["name"]
        || !same_order(&proof["interpretation"], &policy["interpretation"], ordered)
    {
        return Err(fail(
            "the owned model or explicit value observation boundary is inconsistent",
        ));
    }
    let parameters = array(&entry["params"], "a parameter has unrepresented own fields")?;
    for parameter in parameters {
        if !allowed(parameter, &policy["parameterFields"]) {
            return Err(fail("a parameter has unrepresented own fields"));
        }
    }
    let signature: SourceSignature = serde_json::from_value(entry["sourceSignature"].clone())
        .map_err(|_| {
            fail("the actual standard interface and signed i32 domain must remain unchanged")
        })?;
    let parameter_types = parameters
        .iter()
        .map(|parameter| serde_json::from_value::<Type>(parameter["type"].clone()))
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| {
            fail("the actual standard interface and signed i32 domain must remain unchanged")
        })?;
    let result: Type = serde_json::from_value(entry["ret"].clone()).map_err(|_| {
        fail("the actual standard interface and signed i32 domain must remain unchanged")
    })?;
    validate_source_signature(&signature, &parameter_types, &result, "Rust")?;
    let raw_signature = &entry["sourceSignature"];
    let raw_positions = array(
        &raw_signature["parameters"],
        "the actual standard interface and signed i32 domain must remain unchanged",
    )?;
    if entry["ret"] != json!({"kind":"string"})
        || !signature.lifetime_bindings.is_empty()
        || signature
            .parameters
            .iter()
            .chain(std::iter::once(&signature.result))
            .any(|position| !owned_local_position(position))
        || raw_positions
            .iter()
            .chain(std::iter::once(&raw_signature["result"]))
            .any(|position| !value_type(&position["type"]))
        || parameters.iter().any(|parameter| {
            !value_type(&parameter["type"])
                || !parameter["guard"].is_null()
                || !parameter["default"].is_null()
                || parameter.get("optional").is_some()
        })
    {
        return Err(fail(
            "the actual standard interface and signed i32 domain must remain unchanged",
        ));
    }
    let facts = array(
        &proof["bindings"],
        "state facts must retain their ordered roles",
    )?;
    let read_facts = array(
        &proof["reads"],
        "state facts must retain their ordered roles",
    )?;
    let initializers = array(
        &proof["initializers"],
        "state facts must retain their ordered roles",
    )?;
    let constructors = array(
        &proof["literalConstructors"],
        "state facts must retain their ordered roles",
    )?;
    let records = Records::collect(&entry["body"])?;
    let mut bindings = BTreeMap::new();
    let mut reads = BTreeMap::new();
    let mut names = BTreeSet::new();
    for parameter in parameters {
        let name = parameter["name"].as_str().ok_or_else(|| {
            fail("parameters must retain their distinct unshadowed source owners")
        })?;
        if !names.insert(name.to_owned()) || contains(&policy["reservedOwners"], &parameter["name"])
        {
            return Err(fail(
                "parameters must retain their distinct unshadowed source owners",
            ));
        }
    }
    if records.bindings.len() != facts.len() || records.reads.len() != read_facts.len() {
        return Err(fail(
            "the owned state proof must cover every actual binder and String occurrence",
        ));
    }
    for (binding, record) in facts.iter().zip(&records.bindings) {
        exact(binding, &policy["fields"]["binding"])?;
        let path_key = key(&binding["path"])?;
        if !same_order(&binding["path"], &json!(record.path), true)
            || record.node["name"] != binding["name"]
            || bindings.contains_key(&path_key)
            || record.node["value"]["type"] != binding["type"]
            || !binding["mutable"].is_boolean()
            || !contains(&policy["roles"], &binding["role"])
            || binding["sourceName"].as_str().is_none_or(str::is_empty)
            || contains(&policy["reservedOwners"], &binding["sourceName"])
        {
            return Err(fail(
                "a local version contradicts its actual binding, order or value domain",
            ));
        }
        let replacement = binding["role"] == "replacement" || binding["role"] == "join";
        if copies && (binding["mutable"] == true || replacement) {
            return Err(fail(
                "canonical copies do not claim original mutable source bindings",
            ));
        }
        if !value_type(&binding["type"]) && !pair_type(&binding["type"]) {
            return Err(fail(
                "a binding changes its actual text or signed i32 domain",
            ));
        }
        if !replacement
            && (binding["sourceName"] != binding["name"]
                || !names.insert(
                    binding["sourceName"]
                        .as_str()
                        .unwrap_or_default()
                        .to_owned(),
                ))
        {
            return Err(fail(
                "an original local owner must match its distinct actual source binder",
            ));
        }
        bindings.insert(path_key, binding.clone());
    }
    for (read, record) in read_facts.iter().zip(&records.reads) {
        exact(read, &policy["fields"]["read"])?;
        let path_key = key(&read["path"])?;
        if !same_order(&read["path"], &json!(record.path), true)
            || read["binding"] != record.binding
            || reads.contains_key(&path_key)
            || read["type"] != record.node["type"]
            || !same_order(&read["span"], &record.node["span"], ordered)
            || !contains(&policy["readModes"], &read["mode"])
            || !copies && read["mode"] != "move"
        {
            return Err(fail(
                "an owned move or copy disagrees with its actual occurrence and interpretation",
            ));
        }
        reads.insert(path_key, read.clone());
    }
    if !copies
        && (initializers.len() != 1
            || !facts.iter().any(|binding| binding["role"] == "replacement"))
    {
        return Err(fail(
            "the original model requires one actual mutable tuple initialization and replacement",
        ));
    }
    if copies && !initializers.is_empty() {
        return Err(fail(
            "canonical immutable output does not counterfeit original tuple state",
        ));
    }
    if records.constructors.len() != constructors.len() {
        return Err(fail(
            "each owned String literal needs its actual standard constructor site",
        ));
    }
    for (constructor, record) in constructors.iter().zip(&records.constructors) {
        exact(constructor, &policy["fields"]["constructor"])?;
        let owner = Value::Object(
            constructor
                .as_object()
                .unwrap()
                .iter()
                .filter(|(field, _)| !["span", "callSpan", "path"].contains(&field.as_str()))
                .map(|(field, value)| (field.clone(), value.clone()))
                .collect(),
        );
        let span = interval(&constructor["span"])?;
        let call_span = interval(&constructor["callSpan"])?;
        if !same_order(&owner, &policy["constructor"], ordered)
            || !same_order(&constructor["path"], &json!(record.path), true)
            || !same_order(&record.node["span"], &constructor["span"], ordered)
            || call_span.0 >= span.0
            || call_span.1 <= span.1
            || call_span.0 < function_span.0
            || call_span.1 > function_span.1
        {
            return Err(fail(
                "the standard constructor owner or its actual literal site changed",
            ));
        }
    }
    for initializer in initializers {
        exact(initializer, &policy["fields"]["initializer"])?;
        let binding = bindings.get(&key(&initializer["path"])?);
        let fields = array(
            &initializer["fields"],
            "the initializer must own its once-evaluated tuple and ordered field slots",
        )?;
        if binding.is_none_or(|binding| {
            binding["name"] != initializer["binding"] || binding["role"] != "tuple initializer"
        }) || fields.len() != 2
            || fields[0] == fields[1]
        {
            return Err(fail(
                "the initializer must own its once-evaluated tuple and ordered field slots",
            ));
        }
    }
    let mut scope = Scope::new();
    for parameter in parameters {
        scope.insert(
            parameter["name"].as_str().unwrap_or_default().to_owned(),
            Slot::new(parameter["type"].clone()),
        );
    }
    Validator::new(proof, bindings, reads).expression(
        &entry["body"],
        &[],
        &mut scope,
        &BTreeMap::new(),
        None,
    )
}
