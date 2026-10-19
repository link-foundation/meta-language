//! Whole-source closed runtime interpretation over shared linked syntax.
//! Native positional references project real owned `LinkIds`, never copied names.
use super::Language;
use super::diagnostics::{Result, type_error, unsupported};
use super::ir::{Decl, Program};
use super::surface::SProgram;
use serde_json::{Value, json};
use std::sync::LazyLock;
mod emission;
mod parser;
pub use parser::Parsed;
pub fn policy() -> &'static Value {
    static POLICY: LazyLock<Value> = LazyLock::new(|| {
        crate::read_structured_data_lino(include_str!("../data/rust-runtime-shell.lino"))
            .expect("the shared closed runtime policy is valid")
    });
    &POLICY
}
fn fail(detail: &str) -> super::diagnostics::TranslationError {
    type_error(format!("Rust runtime shell proof: {detail}"), None)
}
pub fn position(value: &Value, field: &str) -> Result<usize> {
    if value
        .as_object()
        .is_none_or(|record| record.len() != 1 || !record.contains_key(field))
    {
        return Err(fail(
            "an owned structural reference has unrepresented fields",
        ));
    }
    value[field]
        .as_u64()
        .and_then(|index| usize::try_from(index).ok())
        .ok_or_else(|| fail("an owned structural position is invalid"))
}
pub fn source_interval(value: &Value, empty: bool) -> Result<(u64, u64)> {
    if value.as_object().is_none_or(|record| {
        record.len() != 2 || !record.contains_key("start") || !record.contains_key("end")
    }) {
        return Err(fail("an actual source interval has unrepresented fields"));
    }
    let offset = |value: &Value| -> Option<u64> {
        let number = value.as_f64()?;
        if !crate::typed_program_links::source_span_rules::accept_source_span_offset(
            number,
            number == 0.0 && number.is_sign_negative(),
        ) {
            return None;
        }
        number.to_string().parse().ok()
    };
    match (offset(&value["start"]), offset(&value["end"])) {
        (Some(start), Some(end)) if if empty { end >= start } else { end > start } => {
            Ok((start, end))
        }
        _ => Err(fail("an actual owned source interval is malformed")),
    }
}
fn fields(value: &Value, names: &[&str], exact: bool) -> Result<()> {
    if value.as_object().is_none_or(|record| {
        record.keys().any(|name| !names.contains(&name.as_str()))
            || exact && record.len() != names.len()
    }) {
        return Err(fail("unrecognized or missing owned fields"));
    }
    Ok(())
}
fn role(value: &Value, name: &str, exact: bool) -> Result<()> {
    let names = policy()["fields"][name]
        .as_array()
        .unwrap()
        .iter()
        .map(|field| field.as_str().unwrap())
        .collect::<Vec<_>>();
    fields(value, &names, exact)
}
fn bounded(value: &Value) -> Result<()> {
    let mut pending = vec![(value, 0)];
    let mut count = 0usize;
    while let Some((value, depth)) = pending.pop() {
        count += 1;
        if count > 100_000 || depth > 256 {
            return Err(fail("the complete owned graph exceeds its traversal limit"));
        }
        if let Some(record) = value.as_object() {
            if let Some(span) = record.get("span") {
                source_interval(span, true)?;
            }
            pending.extend(record.values().map(|child| (child, depth + 1)));
        } else if let Some(values) = value.as_array() {
            pending.extend(values.iter().map(|child| (child, depth + 1)));
        }
    }
    Ok(())
}
fn identifier(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().enumerate().all(|(index, byte)| {
            byte.is_ascii_alphabetic() || byte == b'_' || index > 0 && byte.is_ascii_digit()
        })
}
fn tokens(proof: &Value, program: &Program) -> Result<()> {
    let tokens = proof["tokens"]
        .as_array()
        .filter(|tokens| !tokens.is_empty() && tokens.len() <= 100_000)
        .ok_or_else(|| fail("the whole artifact requires its ordered complete source tokens"))?;
    let outer = source_interval(&proof["span"], true)?;
    let mut previous = 0;
    let mut previous_word = false;
    for (index, token) in tokens.iter().enumerate() {
        let kind = token["kind"].as_str().unwrap_or_default();
        let reference = if token.get("declaration").is_some() {
            Some("declaration")
        } else if token.get("binding").is_some() {
            Some("binding")
        } else {
            None
        };
        if !matches!(kind, "identifier" | "number" | "string" | "punct" | "eof")
            || reference.is_some() && kind != "identifier"
        {
            return Err(fail("an owned token has an unrecognized lexical role"));
        }
        if let Some(reference) = reference {
            fields(token, &["kind", reference, "alias", "span"], true)?;
        } else if kind == "number" {
            fields(
                token,
                &["kind", "value", "suffix", "floating", "span"],
                true,
            )?;
        } else {
            fields(token, &["kind", "value", "span"], true)?;
        }
        let (start, end) = source_interval(&token["span"], kind == "eof")?;
        if start < previous || end > outer.1 {
            return Err(fail(
                "source tokens are overlapping or escape the complete artifact",
            ));
        }
        let word = matches!(kind, "identifier" | "number");
        if previous == start && previous_word && word {
            return Err(fail(
                "distinct source word tokens require an actual lexical separator",
            ));
        }
        previous = end;
        previous_word = word;
        if let Some(reference) = reference {
            let alias = token["alias"]
                .as_str()
                .ok_or_else(|| fail("a bound occurrence has no actual source alias"))?;
            if !identifier(alias)
                || u64::try_from(alias.encode_utf16().count()).ok() != Some(end - start)
            {
                return Err(fail(
                    "a bound occurrence lost its actual source alias interval",
                ));
            }
            if reference == "declaration" {
                let target = position(&token[reference], "declarationPosition")?;
                if !matches!(program.declarations.get(target), Some(Decl::Fn(_))) {
                    return Err(fail(
                        "a source occurrence references an unowned declaration",
                    ));
                }
            } else {
                let target = position(&token[reference], "bindingPosition")?;
                if tokens.get(target).is_none_or(|target| {
                    target["kind"] != "identifier" || target.get("value").is_none()
                }) {
                    return Err(fail(
                        "a source occurrence references an unowned lexical binder",
                    ));
                }
            }
        } else {
            let value = token["value"]
                .as_str()
                .ok_or_else(|| fail("the actual token scalar has a different lexical domain"))?;
            if kind == "number" && (!token["suffix"].is_string() || !token["floating"].is_boolean())
            {
                return Err(fail(
                    "the actual numeric token has a different lexical domain",
                ));
            }
            let width = value.encode_utf16().count()
                + if kind == "number" {
                    token["suffix"].as_str().unwrap().encode_utf16().count()
                } else {
                    0
                };
            if matches!(kind, "identifier" | "punct" | "number")
                && u64::try_from(width).ok() != Some(end - start)
            {
                return Err(fail(
                    "a lexical token does not own its actual source interval",
                ));
            }
            if kind == "string"
                && u64::try_from(width + 2)
                    .ok()
                    .is_none_or(|width| end - start < width)
            {
                return Err(fail(
                    "an actual fixed ASCII string must own its scalar contents and both delimiters",
                ));
            }
            if kind == "eof"
                && (index + 1 != tokens.len()
                    || !value.is_empty()
                    || start != outer.1
                    || end != outer.1)
            {
                return Err(fail(
                    "the complete artifact has a malformed final lexical boundary",
                ));
            }
        }
    }
    if tokens.last().is_none_or(|token| token["kind"] != "eof") {
        return Err(fail("the complete artifact has no final lexical boundary"));
    }
    Ok(())
}
pub fn operation_path(root: &Value, label: &str) -> Result<Vec<usize>> {
    fn visit(node: &Value, label: &str, path: &mut Vec<usize>, found: &mut Vec<Vec<usize>>) {
        if node["kind"] == "source operation" {
            if node["op"] == label {
                found.push(path.clone());
            }
            if let Some(parts) = node["parts"].as_array() {
                for (index, child) in parts.iter().enumerate() {
                    path.push(index);
                    visit(child, label, path, found);
                    path.pop();
                }
            }
        }
    }
    let mut found = Vec::new();
    visit(root, label, &mut Vec::new(), &mut found);
    if found.len() != 1 {
        return Err(fail(
            "the runtime assumption has no unique actual operation",
        ));
    }
    Ok(found.remove(0))
}
fn assumptions(parsed: &Parsed) -> Result<Value> {
    policy()["assumptions"].as_array().unwrap().iter().map(|assumption|Ok(json!({"id":assumption["id"],"meaning":assumption["meaning"],"operationNode":{"operationPosition":operation_path(&parsed.operations,assumption["operation"].as_str().unwrap())?}}))).collect::<Result<Vec<_>>>().map(Value::Array)
}
fn normalize_core(value: &mut Value) {
    let record = value.as_object_mut().unwrap();
    for field in [
        "imports",
        "externals",
        "outputThreaded",
        "abortsThreaded",
        "referenceSemantics",
        "utf16Strings",
        "sourceRuntimeShell",
    ] {
        record.remove(field);
    }
    if let Some(main) = value["main"].as_object_mut() {
        main.remove("sourceRuntimeWorker");
    }
    if let Some(entries) = value["declarations"].as_array_mut() {
        for entry in entries {
            if let Some(record) = entry.as_object_mut() {
                for field in ["mutual", "generated"] {
                    if record.get(field) == Some(&Value::Bool(false)) {
                        record.remove(field);
                    }
                }
            }
        }
    }
}
/// Reject raw extra/null/role facts before the checked-program serde boundary.
pub fn validate_records(value: &Value) -> Result<Option<Parsed>> {
    let rooted = value.get("sourceRuntimeShell").is_some();
    let marked = value["main"].get("sourceRuntimeWorker").is_some();
    if !rooted && !marked {
        return Ok(None);
    }
    bounded(value)?;
    let mut normalized = value.clone();
    crate::typed_program_links::source_spans::normalize_source_spans(&mut normalized)
        .map_err(|failure| fail(&failure.to_string()))?;
    let value = &normalized;
    if !rooted {
        return Err(fail(
            "the owned runtime worker lost its complete source interpretation",
        ));
    }
    role(value, "program", false)?;
    let proof = &value["sourceRuntimeShell"];
    role(proof, "proof", true)?;
    let interval = source_interval(&proof["span"], true)?;
    if proof["model"] != policy()["model"]
        || proof["language"] != policy()["language"]
        || proof["observationModel"] != policy()["observationModel"]
        || interval.0 != 0
    {
        return Err(fail(
            "the complete runtime interpretation is unknown or inconsistent",
        ));
    }
    if value["schemaVersion"] != 1
        || value["sourceLanguage"] != "Rust"
        || ["imports", "externals"].iter().any(|field| {
            value
                .get(*field)
                .is_some_and(|value| value.as_array().is_none_or(|rows| !rows.is_empty()))
        })
        || [
            "outputThreaded",
            "abortsThreaded",
            "referenceSemantics",
            "utf16Strings",
        ]
        .iter()
        .any(|field| value.get(*field).is_some_and(|value| value != false))
    {
        return Err(fail(
            "the complete runtime requires a closed pure Rust program",
        ));
    }
    for entry in value["declarations"]
        .as_array()
        .ok_or_else(|| fail("the runtime requires owned declarations"))?
    {
        role(entry, "declaration", false)?;
        for field in ["mutual", "generated"] {
            if entry.get(field).is_some_and(|value| value != false) {
                return Err(fail(
                    "a helper acquired an unproved generated or recursive context",
                ));
            }
        }
    }
    role(&value["main"], "main", true)?;
    for effect in value["main"]["effects"]
        .as_array()
        .ok_or_else(|| fail("the worker requires ordered Unit calls"))?
    {
        role(effect, "effect", true)?;
    }
    let program: Program =
        serde_json::from_value(value.clone()).map_err(|failure| fail(&failure.to_string()))?;
    tokens(proof, &program)?;
    let parsed = parser::read_syntax(proof["tokens"].as_array().unwrap(), Some(&program))?;
    if proof["operations"] != parsed.operations {
        return Err(fail(
            "the represented operation graph does not belong to the actual whole-artifact syntax",
        ));
    }
    if proof["assumptions"] != assumptions(&parsed)? {
        return Err(fail(
            "the conditional runtime assumptions do not cover the actual owned operations",
        ));
    }
    let context = &value["main"]["sourceRuntimeWorker"];
    role(context, "workerContext", true)?;
    if context["model"] != policy()["model"]
        || context["observationModel"] != policy()["observationModel"]
        || context["span"] != parsed.surface["main"]["span"]
    {
        return Err(fail(
            "the owned worker context differs from its complete runtime interpretation",
        ));
    }
    let surface: SProgram = serde_json::from_value(parsed.surface.clone())
        .map_err(|failure| fail(&failure.to_string()))?;
    let expected = super::check::check_program(&surface)?;
    let mut expected =
        serde_json::to_value(expected).map_err(|failure| fail(&failure.to_string()))?;
    let mut actual = value.clone();
    normalize_core(&mut actual);
    normalize_core(&mut expected);
    let expected_entries = expected["declarations"].as_array().unwrap();
    let actual_entries = actual["declarations"].as_array().unwrap();
    let mut ordered = Vec::new();
    for entry in expected_entries {
        let matches = actual_entries
            .iter()
            .filter(|actual| actual["fullName"] == entry["fullName"])
            .collect::<Vec<_>>();
        if matches.len() != 1 {
            return Err(fail("a helper has no unique actual declaration identity"));
        }
        ordered.push(matches[0].clone());
    }
    if ordered.len() != actual_entries.len() {
        return Err(fail(
            "the runtime declaration mapping includes an unowned helper",
        ));
    }
    actual["declarations"] = Value::Array(ordered);
    if actual != expected {
        return Err(fail(
            "the checked Unit program differs from its complete owned runtime syntax or bindings",
        ));
    }
    for row in &parsed.helpers {
        let target = position(
            &proof["tokens"][row.name]["declaration"],
            "declarationPosition",
        )?;
        if value["declarations"][target]["span"] != row.node["span"] {
            return Err(fail(
                "a helper header references a different actual declaration site",
            ));
        }
    }
    Ok(Some(parsed))
}
pub fn validate_program(program: &Program) -> Result<Option<Parsed>> {
    validate_records(&serde_json::to_value(program).map_err(|failure| fail(&failure.to_string()))?)
}
/// Capture real source ownership after the existing core checker creates items.
pub fn capture_program(program: &mut Program, source: &Value) -> Result<()> {
    let mut proof = source.clone();
    let raw = proof["tokens"]
        .as_array()
        .ok_or_else(|| fail("the complete source tokens are missing"))?;
    let parsed = parser::read_syntax(raw, None)?;
    let tokens = proof["tokens"].as_array_mut().unwrap();
    for row in &parsed.helpers {
        let name = tokens[row.name]["value"].as_str().unwrap();
        let index = program
            .declarations
            .iter()
            .position(|entry| entry.full_name() == name)
            .ok_or_else(|| fail("a source helper has no checked declaration"))?;
        let token = tokens[row.name].as_object_mut().unwrap();
        let alias = token.remove("value").unwrap();
        token.insert("alias".into(), alias);
        token.insert("declaration".into(), json!({"declarationPosition":index}));
    }
    for (use_site, target) in &parsed.helper_uses {
        let reference = tokens[*target]["declaration"].clone();
        let token = tokens[*use_site].as_object_mut().unwrap();
        let alias = token.remove("value").unwrap();
        token.insert("alias".into(), alias);
        token.insert("declaration".into(), reference);
    }
    for (use_site, target) in &parsed.local_uses {
        let token = tokens[*use_site].as_object_mut().unwrap();
        let alias = token.remove("value").unwrap();
        token.insert("alias".into(), alias);
        token.insert("binding".into(), json!({"bindingPosition":target}));
    }
    let parsed = parser::read_syntax(tokens, Some(program))?;
    proof["operations"] = parsed.operations.clone();
    proof["assumptions"] = assumptions(&parsed)?;
    let main = program
        .main
        .as_mut()
        .ok_or_else(|| fail("the complete source has no owned worker"))?;
    main.source_runtime_worker = Some(
        json!({"model":policy()["model"],"observationModel":policy()["observationModel"],"span":main.span}),
    );
    program.source_runtime_shell = Some(proof);
    validate_program(program)?;
    Ok(())
}
/// Parse the whole real source under the independently selected versioned model.
pub fn parse_source(source: &str, model: &str) -> Result<SProgram> {
    if !super::runtime_shell_rules::accept_rust_runtime_shell_model(model) {
        return Err(unsupported(
            "Rust runtime shell model",
            "select the legacy model or closed pure Rust unit runtime shell 1",
            None,
        ));
    }
    if model.is_empty() {
        return super::rust::parse_rust(source);
    }
    let lexical = super::lexer::tokenize(source, Language::Rust)?;
    let mut tokens = Vec::new();
    for token in lexical.tokens {
        let kind = match token.kind {
            super::lexer::TokenKind::Identifier => "identifier",
            super::lexer::TokenKind::Number => "number",
            super::lexer::TokenKind::String => "string",
            super::lexer::TokenKind::Punct => "punct",
            super::lexer::TokenKind::Eof => "eof",
            _ => {
                return Err(unsupported(
                    "closed Rust runtime shell",
                    "this lexical construct needs a separate runtime ownership proof",
                    Some(token.span()),
                ));
            }
        };
        let mut value =
            json!({"kind":kind,"value":token.value,"span":{"start":token.start,"end":token.end}});
        if kind == "number" {
            value["suffix"] = json!(token.suffix);
            value["floating"] = json!(token.floating);
        }
        tokens.push(value);
    }
    let parsed = parser::read_syntax(&tokens, None)?;
    let mut surface = parsed.surface;
    surface["sourceRuntimeShell"] = json!({"model":policy()["model"],"language":policy()["language"],"observationModel":policy()["observationModel"],"span":{"start":0,"end":source.encode_utf16().count()},"tokens":tokens,"operations":parsed.operations,"assumptions":[]});
    serde_json::from_value(surface).map_err(|failure| fail(&failure.to_string()))
}
pub use emission::{emit_rust, record_assumptions};
