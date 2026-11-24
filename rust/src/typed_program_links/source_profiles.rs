use super::*;

/// Select independent source profiles while keeping caller provenance separate.
///
/// # Errors
/// Propagates source, model, checker and linked-adapter diagnostics.
pub fn create_typed_program_links_with_rust_source_profiles(
    source: &str,
    language: &str,
    array_result_model: Option<&str>,
    completion_model: Option<&str>,
    source_signature_model: Option<&str>,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    create_typed_program_links_with_scalar_loop_profiles(
        source,
        language,
        array_result_model,
        completion_model,
        source_signature_model,
        (None, None),
        provenance,
    )
}

/// Import Rust under an explicit closed primitive scalar loop interpretation.
///
/// # Errors
/// Refuses non-Rust sources, unknown models and unproved loop semantics.
pub fn create_typed_program_links_with_rust_scalar_loop_model(
    source: &str,
    language: &str,
    model: &str,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    create_typed_program_links_with_scalar_loop_profiles(
        source,
        language,
        None,
        None,
        None,
        (Some(model), None),
        provenance,
    )
}

/// Import actual emitted JavaScript loops under the Number/Boolean interface.
///
/// # Errors
/// Refuses other languages, unknown models and unproved snapshot semantics.
pub fn create_typed_program_links_with_javascript_scalar_loop_model(
    source: &str,
    language: &str,
    model: &str,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    create_typed_program_links_with_scalar_loop_profiles(
        source,
        language,
        None,
        None,
        None,
        (None, Some(model)),
        provenance,
    )
}

/// Select independent existing source profiles and one scalar loop language model.
///
/// The loop selection pairs Rust and JavaScript models; a selected model must
/// match the actual source language. Caller provenance stays separate.
///
/// # Errors
/// Refuses conflicting domains, unknown models and unproved source semantics.
pub fn create_typed_program_links_with_scalar_loop_profiles(
    source: &str,
    language: &str,
    array_result_model: Option<&str>,
    completion_model: Option<&str>,
    source_signature_model: Option<&str>,
    scalar_loop_models: (Option<&str>, Option<&str>),
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    let (rust_scalar_loop_model, javascript_scalar_loop_model) = scalar_loop_models;
    let canonical = match language.to_ascii_lowercase().as_str() {
        "rust" | "rs" => "Rust",
        "javascript" | "js" | "mjs" => "JavaScript",
        _ => {
            return Err(error(
                "typed linked source import currently reads JavaScript and Rust",
            ));
        }
    };
    if canonical != "Rust" {
        if array_result_model.is_some() {
            return Err(error("a Rust array result model requires Rust source"));
        }
        if completion_model.is_some() {
            return Err(error("a Rust completion model requires Rust source"));
        }
        if source_signature_model.is_some() {
            return Err(error("a Rust source signature model requires Rust source"));
        }
        if rust_scalar_loop_model.is_some() {
            return Err(error("a Rust scalar loop model requires Rust source"));
        }
    }
    if javascript_scalar_loop_model.is_some() && canonical != "JavaScript" {
        return Err(error(
            "a JavaScript scalar loop model requires JavaScript source",
        ));
    }
    let surface = if canonical == "Rust" {
        parse_rust_with_loop_profiles(
            source,
            array_result_model.unwrap_or(""),
            completion_model.unwrap_or(""),
            source_signature_model.unwrap_or(""),
            rust_scalar_loop_model.unwrap_or(""),
        )
    } else if let Some(model) = javascript_scalar_loop_model {
        parse_javascript_with_scalar_loop_model(source, model)
    } else {
        parse_javascript(source)
    }
    .map_err(|failure| error(failure.to_string()))?;
    let program = check_program(&surface).map_err(|failure| error(failure.to_string()))?;
    if array_result_model.is_none()
        && completion_model.is_none()
        && source_signature_model.is_none()
        && rust_scalar_loop_model.is_none()
        && javascript_scalar_loop_model.is_none()
    {
        return encode_typed_program_links_with_provenance(&program, provenance);
    }
    let mut profile = serde_json::Map::new();
    profile.insert("language".into(), Value::String(canonical.into()));
    if let Some(model) = array_result_model {
        profile.insert("rustArrayResultModel".into(), Value::String(model.into()));
    }
    if let Some(model) = completion_model {
        profile.insert("rustCompletionModel".into(), Value::String(model.into()));
    }
    if let Some(model) = source_signature_model {
        profile.insert(
            "rustSourceSignatureModel".into(),
            Value::String(model.into()),
        );
    }
    if let Some(model) = rust_scalar_loop_model {
        profile.insert("rustScalarLoopModel".into(), Value::String(model.into()));
    }
    if let Some(model) = javascript_scalar_loop_model {
        profile.insert(
            "javaScriptScalarLoopModel".into(),
            Value::String(model.into()),
        );
    }
    let selected_provenance = serde_json::json!({
        "sourceProfile": profile,
        "userProvenance": provenance,
    });
    encode_typed_program_links_with_provenance(&program, Some(&selected_provenance))
}
