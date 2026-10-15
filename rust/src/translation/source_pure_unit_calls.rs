//! Inspectable pure Unit source facets checked against existing owned IR nodes.
//! Model and admitted syntax decisions reuse automatically translated policies.
use super::diagnostics::{Result, type_error};
use super::ir::Program;
use super::rust::pure_unit_call_rules::{
    accept_rust_pure_unit_body_form, accept_rust_pure_unit_call_model,
    accept_rust_pure_unit_context, accept_rust_pure_unit_header_form, accept_rust_pure_unit_name,
    accept_rust_pure_unit_name_character, accept_rust_pure_unit_path_form,
    accept_rust_pure_unit_visibility,
};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::sync::LazyLock;

pub(super) fn policy() -> &'static Value {
    static POLICY: LazyLock<Value> = LazyLock::new(|| {
        crate::read_structured_data_lino(include_str!("../data/rust-pure-unit-calls.lino"))
            .expect("the shared pure Unit policy is valid")
    });
    &POLICY
}
fn fail(detail: &str) -> super::diagnostics::TranslationError {
    type_error(format!("Rust pure Unit-call proof: {detail}"), None)
}
fn fields(role: &str) -> Vec<&'static str> {
    policy()["fields"][role]
        .as_array()
        .unwrap()
        .iter()
        .map(|field| field.as_str().unwrap())
        .collect()
}
fn allowed(value: &Value, role: &str) -> Result<()> {
    let fields = fields(role);
    if value
        .as_object()
        .is_none_or(|record| record.keys().any(|name| !fields.contains(&name.as_str())))
    {
        return Err(fail("unrepresented own fields"));
    }
    Ok(())
}
fn exact(value: &Value, role: &str) -> Result<()> {
    allowed(value, role)?;
    if value
        .as_object()
        .is_none_or(|record| record.len() != fields(role).len())
    {
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
fn source_offset(value: &Value) -> Option<u64> {
    let number = value.as_f64()?;
    if !crate::typed_program_links::source_span_rules::accept_source_span_offset(
        number,
        number == 0.0 && number.is_sign_negative(),
    ) {
        return None;
    }
    // The shared policy proves a finite, exact safe unsigned integer. Preserve
    // that meaning for decimal and exponent aliases without changing literals.
    number.to_string().parse().ok()
}
fn span(value: &Value) -> Result<(u64, u64)> {
    if value.as_object().is_none_or(|record| {
        record.len() != 2 || !record.contains_key("start") || !record.contains_key("end")
    }) {
        return Err(fail("unrecognized or missing owned fields"));
    }
    match (source_offset(&value["start"]), source_offset(&value["end"])) {
        (Some(start), Some(end)) if end > start => Ok((start, end)),
        _ => Err(fail("an actual source span is malformed")),
    }
}
const fn inside(inner: (u64, u64), outer: (u64, u64)) -> bool {
    inner.0 >= outer.0 && inner.1 <= outer.1
}
fn name_accepted(value: &Value) -> bool {
    value.as_str().is_some_and(|name| {
        accept_rust_pure_unit_name(name)
            && name.chars().enumerate().all(|(index, character)| {
                accept_rust_pure_unit_name_character(f64::from(u32::from(character)), index == 0)
            })
    })
}
fn model(proof: &Value) -> Result<&str> {
    let model = proof["model"].as_str().unwrap_or_default();
    if model.is_empty() || proof["language"] != "Rust" || !accept_rust_pure_unit_call_model(model) {
        return Err(fail("the selected interpretation is invalid"));
    }
    Ok(model)
}
/// Validate a selected raw declaration before serde can erase fields or nulls.
pub fn validate_pure_unit_declaration_record(entry: &Value) -> Result<()> {
    allowed(entry, "declaration")?;
    let entry_span = span(&entry["span"])?;
    if entry["k"] != "fn"
        || !name_accepted(&entry["name"])
        || entry["fullName"] != entry["name"]
        || entry["modulePath"] != json!([])
        || entry["params"] != json!([])
        || entry["recursive"] != false
        || !entry["decreasing"].is_null()
        || ["mutual", "generated"].iter().any(|field| {
            entry
                .get(*field)
                .is_some_and(|value| value != &Value::Bool(false))
        })
    {
        return Err(fail(
            "a helper does not own the declared flat zero-argument identity",
        ));
    }
    exact(&entry["ret"], "unitType")?;
    exact(&entry["body"], "body")?;
    exact(&entry["body"]["type"], "unitType")?;
    if entry["ret"]["kind"] != "unit"
        || entry["body"]["k"] != "unit"
        || entry["body"]["type"]["kind"] != "unit"
    {
        return Err(fail("a helper must return the exact pure Unit value"));
    }
    let proof = &entry["sourcePureUnit"];
    let selected = model(proof)?;
    let public = accept_rust_pure_unit_visibility(selected, "public");
    exact(
        proof,
        if public {
            "publicFunctionProof"
        } else {
            "functionProof"
        },
    )?;
    let header = span(&proof["headerSpan"])?;
    let body = span(&proof["bodySpan"])?;
    if !accept_rust_pure_unit_header_form(proof["headerForm"].as_str().unwrap_or_default())
        || !accept_rust_pure_unit_body_form(proof["bodyForm"].as_str().unwrap_or_default())
        || header.0 != entry_span.0
        || body.1 != entry_span.1
        || header.1 > body.0
        || !inside(header, entry_span)
        || !inside(body, entry_span)
    {
        return Err(fail(
            "a helper header or complete body site contradicts its actual owned declaration",
        ));
    }
    if public {
        let visibility = &proof["visibility"];
        exact(visibility, "visibility")?;
        let site = span(&visibility["span"])?;
        if !accept_rust_pure_unit_visibility(
            selected,
            visibility["form"].as_str().unwrap_or_default(),
        ) || site.0 != header.0
            || site.1 - site.0 != policy()["publicVisibilityWidth"].as_u64().unwrap()
            || site.1 >= header.1
        {
            return Err(fail(
                "the actual bare public visibility does not belong to the complete owned header",
            ));
        }
    }
    Ok(())
}
/// Check complete raw records before the checked-program deserialize boundary.
pub fn validate_pure_unit_records(value: &Value) -> Result<()> {
    let entries = value["declarations"].as_array();
    let effects = value["main"]["effects"].as_array();
    if value.get("sourcePureUnitCalls").is_none()
        && !entries.is_some_and(|entries| {
            entries
                .iter()
                .any(|entry| entry.get("sourcePureUnit").is_some())
        })
        && !effects.is_some_and(|effects| {
            effects
                .iter()
                .any(|effect| effect.get("sourcePureUnitCall").is_some())
        })
    {
        return Ok(());
    }
    let entries = array(
        &value["declarations"],
        "the selected interpretation requires a complete flat pure Unit program without other effects",
    )?;
    let items = array(
        &value["items"],
        "the selected interpretation requires a complete flat pure Unit program without other effects",
    )?;
    let effects = array(
        &value["main"]["effects"],
        "the selected interpretation requires a complete flat pure Unit program without other effects",
    )?;
    let forbidden = value["main"].is_null()
        || ["imports", "externals"].iter().any(|field| {
            value
                .get(*field)
                .is_some_and(|value| value.as_array().is_none_or(|entries| !entries.is_empty()))
        })
        || [
            "outputThreaded",
            "abortsThreaded",
            "referenceSemantics",
            "utf16Strings",
        ]
        .iter()
        .any(|field| {
            value
                .get(*field)
                .is_some_and(|value| value != &Value::Bool(false))
        })
        || items.iter().any(|item| item["k"] != "decl")
        || entries.len() != items.len();
    if !accept_rust_pure_unit_context(
        value["sourceLanguage"].as_str().unwrap_or_default(),
        forbidden,
    ) {
        return Err(fail(
            "the selected interpretation requires a complete flat pure Unit program without other effects",
        ));
    }
    allowed(value, "program")?;
    exact(&value["sourcePureUnitCalls"], "marker")?;
    let selected = model(&value["sourcePureUnitCalls"])?;
    exact(&value["main"], "main")?;
    let main_span = span(&value["main"]["span"])?;
    let mut names = HashSet::new();
    for entry in entries {
        validate_pure_unit_declaration_record(entry)?;
        if entry["sourcePureUnit"]["model"] != selected
            || !names.insert(entry["name"].as_str().unwrap())
        {
            return Err(fail(
                "a helper does not own the declared flat zero-argument identity",
            ));
        }
    }
    let mut item_names = HashSet::new();
    for item in items {
        if item.as_object().is_none_or(|record| record.len() != 2)
            || item["k"] != "decl"
            || !item["fullName"]
                .as_str()
                .is_some_and(|name| names.contains(name) && item_names.insert(name))
        {
            return Err(fail(
                "the ordered items do not contain each actual declaration exactly once",
            ));
        }
    }
    let mut locals = HashSet::new();
    for effect in effects {
        exact(effect, "effect")?;
        let site = span(&effect["span"])?;
        if effect["k"] != "let"
            || !name_accepted(&effect["name"])
            || !effect["name"]
                .as_str()
                .is_some_and(|name| !names.contains(name) && locals.insert(name))
            || !inside(site, main_span)
        {
            return Err(fail(
                "a Unit call statement has no distinct unreserved local binding or actual main site",
            ));
        }
        let call = &effect["value"];
        exact(call, "call")?;
        exact(&call["type"], "unitType")?;
        span(&call["span"])?;
        if call["k"] != "call"
            || call["args"] != json!([])
            || call["type"]["kind"] != "unit"
            || call["span"] != effect["span"]
            || !call["fn"].as_str().is_some_and(|name| names.contains(name))
        {
            return Err(fail(
                "a main statement must call the exact owned zero-argument pure Unit declaration",
            ));
        }
        let proof = &effect["sourcePureUnitCall"];
        exact(proof, "callProof")?;
        span(&proof["span"])?;
        if proof["model"] != selected
            || proof["language"] != "Rust"
            || !accept_rust_pure_unit_path_form(proof["pathForm"].as_str().unwrap_or_default())
            || proof["span"] != effect["span"]
        {
            return Err(fail(
                "a Unit call statement has invalid selected source facts",
            ));
        }
    }
    Ok(())
}
/// Revalidate typed native programs with the same raw owned-record checks.
pub fn validate_source_pure_unit_calls(program: &Program) -> Result<()> {
    validate_pure_unit_records(
        &serde_json::to_value(program).map_err(|failure| fail(&failure.to_string()))?,
    )
}
