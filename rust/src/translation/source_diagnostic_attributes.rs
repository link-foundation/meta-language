//! Authenticate owned diagnostic-only source facets with the shared token policy.
use super::Language;
use super::diagnostics::{Result, TranslationError, type_error};
use super::ir::FnDecl;
use super::lexer::{Source, Token, TokenKind, tokenize};
use super::rust::const_function_rules::{
    accept_rust_const_function_container, accept_rust_const_function_interval,
    accept_rust_const_function_offset,
};
use super::rust_diagnostic_attribute_rules::plan_rust_diagnostic_attribute_prefix;
use super::surface::{SFn, SItem, SProgram};
use serde::{Deserialize, Deserializer};
use serde_json::{Value, json};
use std::sync::LazyLock;

/// The exact policy also consumed by the JavaScript facet adapter.
pub fn policy() -> &'static Value {
    static POLICY: LazyLock<Value> = LazyLock::new(|| {
        crate::read_structured_data_lino(include_str!("../data/rust-diagnostic-attributes.lino"))
            .expect("the shared diagnostic attribute policy is valid")
    });
    &POLICY
}

fn fail(detail: &str) -> TranslationError {
    type_error(format!("Rust diagnostic source attributes: {detail}"), None)
}

/// Preserve a present null so the source proof validator can reject it.
pub fn read_present_facet<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Option<Value>, D::Error> {
    Value::deserialize(deserializer).map(Some)
}

/// Surface transport retains omission independently of a present JSON value.
pub fn read_present_surface_facet<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Option<Box<Value>>, D::Error> {
    Value::deserialize(deserializer).map(|value| Some(Box::new(value)))
}

fn fields(value: &Value, name: &str) -> Result<()> {
    let represented = value.as_object().is_some_and(|record| {
        policy()["fields"][name].as_array().is_some_and(|allowed| {
            record.len() == allowed.len()
                && record
                    .keys()
                    .all(|key| allowed.contains(&Value::String(key.clone())))
        })
    });
    if !accept_rust_const_function_container("record", true, !represented) {
        return Err(fail(
            "an owned descriptor has missing or unrepresented fields",
        ));
    }
    Ok(())
}

fn offset(value: &Value) -> Result<(usize, f64)> {
    let number = value
        .as_f64()
        .filter(|number| {
            accept_rust_const_function_offset(*number, *number == 0.0 && number.is_sign_negative())
        })
        .ok_or_else(|| fail("a source span needs integral absolute UTF16 offsets"))?;
    let integer = number
        .to_string()
        .parse()
        .map_err(|_| fail("a source span needs integral absolute UTF16 offsets"))?;
    Ok((integer, number))
}

fn interval(value: &Value) -> Result<(usize, usize)> {
    fields(value, "span")?;
    let (start, start_number) = offset(&value["start"])?;
    let (end, end_number) = offset(&value["end"])?;
    if !accept_rust_const_function_interval(start_number, end_number, true) {
        return Err(fail("a source span needs integral absolute UTF16 offsets"));
    }
    Ok((start, end))
}

/// Actual lexer spellings, independent of the portable admission decision.
pub const fn token_kind(token: &Token) -> &'static str {
    match token.kind {
        TokenKind::Number => "number",
        TokenKind::Identifier => "identifier",
        TokenKind::Macro => "macro",
        TokenKind::Punct => "punct",
        TokenKind::RegularExpression => "regularExpression",
        TokenKind::String => "string",
        TokenKind::Character => "char",
        TokenKind::ByteCharacter => "byteChar",
        TokenKind::ByteString => "byteString",
        TokenKind::Interpolation => "interpolation",
        TokenKind::Template => "template",
        TokenKind::Eof => "eof",
    }
}

fn parent_record(name: &str, proof: Option<&Value>) -> Value {
    json!({"k":"fn","name":name,"sourceConstFunction":proof})
}

fn token_fact(token: &Token) -> Value {
    json!({"kind":token_kind(token),"value":token.value,"raw":token.raw,
        "span":{"start":token.start,"end":token.end}})
}

fn validate_facet(facet: &Value, entry: &Value, module_path: &[Value]) -> Result<Vec<f64>> {
    fields(facet, "facet")?;
    if ["model", "language", "edition"]
        .iter()
        .any(|name| facet[*name] != policy()["facet"][*name])
    {
        return Err(fail("the registered source model or Rust edition changed"));
    }
    let proof = &entry["sourceConstFunction"];
    if entry["k"] != "fn" || !proof.is_object() {
        return Err(fail("an actual const function proof is required"));
    }
    let attachment = &facet["attachment"];
    fields(attachment, "attachment")?;
    if !attachment["sourceName"].is_string()
        || attachment["sourceName"] != entry["name"]
        || attachment["modulePath"].as_array().is_none_or(|path| {
            path.as_slice() != module_path || path.iter().any(|name| !name.is_string())
        })
    {
        return Err(fail(
            "the attribute prefix belongs to a different function owner",
        ));
    }
    if interval(&attachment["constKeywordSpan"])? != interval(&proof["keywordSpan"])?
        || interval(&attachment["functionKeywordSpan"])? != interval(&proof["functionKeywordSpan"])?
    {
        return Err(fail(
            "the attribute attachment differs from the actual const header",
        ));
    }
    let (start, end) = interval(&facet["span"])?;
    let raw = facet["raw"]
        .as_str()
        .ok_or_else(|| fail("the complete raw prefix must end at the actual const keyword"))?;
    let stored = facet["tokens"]
        .as_array()
        .ok_or_else(|| fail("the complete raw prefix must end at the actual const keyword"))?;
    let width = raw.encode_utf16().count();
    if width != end - start || end != interval(&proof["keywordSpan"])?.0 {
        return Err(fail(
            "the complete raw prefix must end at the actual const keyword",
        ));
    }
    let const_kind = policy()["constKeyword"]["kind"]
        .as_str()
        .expect("a shared token kind");
    let const_value = policy()["constKeyword"]["value"]
        .as_str()
        .expect("a shared const spelling");
    let text = format!("{raw}{const_value}");
    let mut actual = tokenize(&text, Language::Rust)?.tokens;
    actual.retain(|token| token.kind != TokenKind::Eof);
    let following = actual.pop();
    if following.as_ref().is_none_or(|token| {
        token_kind(token) != const_kind
            || token.value != const_value
            || token.raw != const_value
            || token.start != width
            || token.end != width + const_value.encode_utf16().count()
    }) {
        return Err(fail(
            "the retained prefix absorbs or changes the actual following const token",
        ));
    }
    if actual.len() != stored.len() || actual.first().is_none_or(|token| token.start != 0) {
        return Err(fail(
            "the facet must retain every actual token from its first outer attribute",
        ));
    }
    let raw_source = Source::new(raw);
    for (original, token) in actual.iter().zip(stored) {
        fields(token, "token")?;
        let (token_start, token_end) = interval(&token["span"])?;
        if token["kind"].as_str() != Some(token_kind(original))
            || token["value"].as_str() != Some(original.value.as_str())
            || token["raw"].as_str() != Some(original.raw.as_str())
            || token_start != start + original.start
            || token_end != start + original.end
            || token["raw"].as_str()
                != Some(raw_source.slice(original.start, original.end).as_str())
        {
            return Err(fail(
                "stored token kind value spelling or absolute UTF16 span differs from the actual prefix",
            ));
        }
    }
    let kinds = actual
        .iter()
        .map(|token| token_kind(token).to_owned())
        .collect::<Vec<_>>();
    let values = actual
        .iter()
        .map(|token| token.value.clone())
        .collect::<Vec<_>>();
    let plan = plan_rust_diagnostic_attribute_prefix(&kinds, &values);
    let length = serde_json::Number::from(actual.len())
        .as_f64()
        .expect("a token length is numeric");
    if plan.len() != 4 || plan[0].to_bits() != length.to_bits() || plan[2] < 1.0 {
        return Err(fail(
            "the complete prefix has no admitted diagnostic attribute interpretation",
        ));
    }
    Ok(plan)
}

/// Bind a prefix to the actual source slice and already parsed const function.
pub fn create_facet(
    source: &str,
    tokens: &[Token],
    first_attribute: usize,
    const_keyword: usize,
    function: &SFn,
    module_path: &[String],
) -> Result<Value> {
    if first_attribute >= const_keyword || const_keyword >= tokens.len() {
        return Err(fail(
            "actual complete source and token positions are required",
        ));
    }
    let first = &tokens[first_attribute];
    let keyword = &tokens[const_keyword];
    let proof = function
        .source_const_function
        .as_deref()
        .ok_or_else(|| fail("the prefix must be attached to the original actual const token"))?;
    if token_kind(keyword)
        != policy()["constKeyword"]["kind"]
            .as_str()
            .expect("a shared token kind")
        || keyword.value
            != policy()["constKeyword"]["value"]
                .as_str()
                .expect("a shared const spelling")
        || (keyword.start, keyword.end) != interval(&proof["keywordSpan"])?
    {
        return Err(fail(
            "the prefix must be attached to the original actual const token",
        ));
    }
    let source = Source::new(source);
    let facet = json!({
        "model":policy()["facet"]["model"],"language":policy()["facet"]["language"],"edition":policy()["facet"]["edition"],
        "span":{"start":first.start,"end":keyword.start},
        "raw":source.slice(first.start,keyword.start),
        "tokens":tokens[first_attribute..const_keyword].iter().map(token_fact).collect::<Vec<_>>(),
        "attachment":{"sourceName":function.name,"modulePath":module_path,
            "constKeywordSpan":proof["keywordSpan"],"functionKeywordSpan":proof["functionKeywordSpan"]}
    });
    let entry = parent_record(&function.name, function.source_const_function.as_deref());
    let module_path = module_path
        .iter()
        .map(|name| Value::String(name.clone()))
        .collect::<Vec<_>>();
    validate_facet(&facet, &entry, &module_path)?;
    Ok(facet)
}

fn marker_owners(value: &Value) -> Result<Vec<&Value>> {
    let mut pending = vec![(value, 0)];
    let mut owners = Vec::new();
    let mut visits = 0;
    let mut limited = false;
    while let Some((value, depth)) = pending.pop() {
        if !value.is_object() && !value.is_array() {
            continue;
        }
        visits += 1;
        if depth > 256 || visits > 100_000 {
            limited = true;
            continue;
        }
        if let Some(record) = value.as_object() {
            if record.contains_key("sourceDiagnosticAttributes") {
                owners.push(value);
            }
            pending.extend(record.values().map(|child| (child, depth + 1)));
        } else if let Some(items) = value.as_array() {
            pending.extend(items.iter().map(|child| (child, depth + 1)));
        }
    }
    if !owners.is_empty() && limited {
        return Err(fail(
            "the source descriptor exceeds its semantic traversal limit",
        ));
    }
    Ok(owners)
}

/// Validate actual raw archive records before serde can erase unknown fields.
pub fn validate_records(value: &Value) -> Result<()> {
    let owners = marker_owners(value)?;
    if owners.is_empty() {
        return Ok(());
    }
    let entries = value["declarations"]
        .as_array()
        .ok_or_else(|| fail("the source facet requires actual checked Rust declarations"))?;
    for owner in &owners {
        if !entries
            .iter()
            .any(|entry| std::ptr::eq(*owner, entry) && entry["k"] == "fn")
        {
            return Err(fail(
                "a source facet is not attached to an actual function item",
            ));
        }
    }
    if value["sourceLanguage"] != policy()["facet"]["language"] {
        return Err(fail(
            "the source facet requires actual checked Rust declarations",
        ));
    }
    for entry in owners {
        let path = entry["modulePath"]
            .as_array()
            .filter(|path| path.iter().all(Value::is_string))
            .ok_or_else(|| fail("the facet does not target its actual declaration binding"))?;
        let name = entry["name"]
            .as_str()
            .ok_or_else(|| fail("the facet does not target its actual declaration binding"))?;
        let full_name = path
            .iter()
            .filter_map(Value::as_str)
            .chain(std::iter::once(name))
            .collect::<Vec<_>>()
            .join(".");
        if entry["fullName"].as_str() != Some(full_name.as_str()) {
            return Err(fail(
                "the facet does not target its actual declaration binding",
            ));
        }
        validate_facet(&entry["sourceDiagnosticAttributes"], entry, path)?;
    }
    Ok(())
}

/// Check represented Surface fields before lowering; external raw Surface serde
/// remains its existing host boundary, separate from the guarded archive codec.
pub fn validate_surface(surface: &SProgram) -> Result<()> {
    let mut pending = surface
        .items
        .iter()
        .map(|item| (item, Vec::<String>::new()))
        .collect::<Vec<_>>();
    let mut functions = Vec::new();
    while let Some((item, path)) = pending.pop() {
        match item {
            SItem::Fn(function) if function.source_diagnostic_attributes.is_some() => {
                functions.push((function, path));
            }
            SItem::Module(module) => {
                let mut path = path;
                path.push(module.name.clone());
                pending.extend(module.items.iter().map(|item| (item, path.clone())));
            }
            _ => {}
        }
    }
    if functions.is_empty() {
        return Ok(());
    }
    if surface.language != Language::Rust {
        return Err(fail(
            "the source facet requires actual checked Rust declarations",
        ));
    }
    for (function, path) in functions {
        let entry = parent_record(&function.name, function.source_const_function.as_deref());
        let path = path.into_iter().map(Value::String).collect::<Vec<_>>();
        validate_facet(
            function
                .source_diagnostic_attributes
                .as_deref()
                .expect("a selected facet"),
            &entry,
            &path,
        )?;
    }
    Ok(())
}

/// Materialize only a validated prefix, with both spellings from shared data.
pub fn render_prefix(function: &FnDecl, source: String) -> Result<String> {
    let Some(facet) = function.source_diagnostic_attributes.as_ref() else {
        return Ok(source);
    };
    let entry = parent_record(&function.name, function.source_const_function.as_ref());
    let path = function
        .module_path
        .iter()
        .map(|name| Value::String(name.clone()))
        .collect::<Vec<_>>();
    let plan = validate_facet(facet, &entry, &path)?;
    let raw = facet["raw"].as_str().expect("a validated source prefix");
    if plan[3] < 0.0 {
        return Ok(format!("{raw}{source}"));
    }
    let prefix = policy()["emission"]["canonicalPublicPrefix"]
        .as_str()
        .expect("a shared canonical public prefix");
    let visibility = policy()["emission"]["visibilityPrefix"]
        .as_str()
        .expect("a shared visibility prefix");
    if !source.starts_with(prefix) {
        return Err(fail("the canonical const emitter header changed"));
    }
    Ok(format!("{raw}{}", &source[visibility.len()..]))
}
