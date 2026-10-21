//! Native fact transport for the shared owned-identifier policy. Rendering uses
//! the existing automatically translated scoped-binding decision unchanged.
use super::{Result, ident, unsupported};
use serde_json::Value;
use std::sync::OnceLock;

static POLICY: OnceLock<std::result::Result<Value, String>> = OnceLock::new();
static SOURCE: OnceLock<std::result::Result<Value, String>> = OnceLock::new();

fn read(
    slot: &'static OnceLock<std::result::Result<Value, String>>,
    text: &'static str,
) -> Result<&'static Value> {
    match slot.get_or_init(|| crate::read_structured_data_lino(text)) {
        Ok(value) => Ok(value),
        Err(detail) => Err(unsupported("JavaScript identifier model", detail, None)),
    }
}

fn policy() -> Result<&'static Value> {
    let policy = read(
        &POLICY,
        include_str!("../../data/javascript-identifier-models.lino"),
    )?;
    let source = read(
        &SOURCE,
        include_str!("../../data/javascript-scoped-binding-emission.lino"),
    )?;
    if policy["schemaVersion"] != 1
        || policy["reservedPrefix"] != "ml_"
        || policy["renderer"]["operation"] != "renderJavaScriptScopedBindingIdentifier"
        || policy["renderer"]["preserve"] != false
        || policy["renderer"]["sourcePolicy"]["file"]
            != "js/src/translation/data/javascript-scoped-binding-emission.lino"
        || policy["renderer"]["sourcePolicy"]["model"] != source["model"]
        || policy["renderer"]["sourcePolicy"]["prefix"] != source["prefix"]
    {
        return Err(unsupported(
            "JavaScript identifier model",
            "the installed identifier policy must retain its exact shared renderer authority",
            None,
        ));
    }
    Ok(policy)
}

fn ordinary_ident(name: &str) -> String {
    let normalized = ident(name);
    // select() validated this immutable installed policy before any allocator
    // receives this function pointer. This is a factual prefix projection only.
    let prefix =
        policy().expect("the selected immutable identifier policy stays valid")["reservedPrefix"]
            .as_str()
            .expect("the selected prefix is text");
    if normalized.starts_with(prefix) {
        super::super::scoped_binding_emission_rules::render_java_script_scoped_binding_identifier(
            &normalized,
            false,
        )
    } else {
        normalized
    }
}

pub(super) fn select(model: Option<&str>) -> Result<fn(&str) -> String> {
    let Some(model) = model else {
        return Ok(ident);
    };
    let policy = policy()?;
    let expected = policy["model"].as_str().ok_or_else(|| {
        unsupported(
            "JavaScript identifier model",
            "the installed model identity must be text",
            None,
        )
    })?;
    if model != expected {
        return Err(unsupported(
            "JavaScript identifier model",
            &format!("select {expected}"),
            None,
        ));
    }
    Ok(ordinary_ident)
}
