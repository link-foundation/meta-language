//! Editable source-free programs in the shared typed links schema.
//! The codec adapts identities and data; semantic checking and emission reuse
//! the existing portable core.
use crate::translation::Language;
use crate::translation::check::check_program;
use crate::translation::emit_common::{Assumption, Encoding, Mapping, TheoremRecord};
use crate::translation::emit_javascript::{
    JavaScriptEmissionOptions, emit_javascript_with_options,
};
use crate::translation::emit_rust::{RustEmissionOptions, emit_rust_with_array_consumption};
use crate::translation::ir::Program;
use crate::translation::javascript::{parse_javascript, parse_javascript_with_scalar_loop_model};
use crate::translation::rust::{
    parse_rust, parse_rust_with_const_function_model, parse_rust_with_empty_option_source_model,
    parse_rust_with_loop_profiles, parse_rust_with_owned_local_state_model,
    parse_rust_with_primitive_owner_model, parse_rust_with_pure_unit_call_model,
    parse_rust_with_runtime_shell_model, parse_rust_with_type_namespace_model,
};
use crate::{LinkId, LinkMetadata, LinkNetwork, LinkType};
use serde_json::Value;
use std::fmt;
use std::sync::OnceLock;

mod borrowed_call_diagnostics;
mod callables;
mod checked;
mod codec;
pub mod program_numeric_metadata;
pub mod program_numeric_metadata_rules;
mod projection;
pub mod source_span_rules;
pub mod source_spans;

/// A structural, unsupported or semantic defect in a linked program.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedProgramLinksError {
    /// The concrete diagnostic.
    pub message: String,
}
impl fmt::Display for TypedProgramLinksError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}
impl std::error::Error for TypedProgramLinksError {}
type Result<T> = std::result::Result<T, TypedProgramLinksError>;
fn error(message: impl Into<String>) -> TypedProgramLinksError {
    TypedProgramLinksError {
        message: message.into(),
    }
}

/// A semantic graph with an explicit program root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedProgramLinks {
    /// Values, ordered containers, fields and binding identities.
    pub network: LinkNetwork,
    /// The schema root.
    pub root: LinkId,
}
/// Bounds on structural traversal; semantic references do not recurse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypedProgramLinksLimits {
    /// Maximum structurally owned values.
    pub node_limit: usize,
    /// Maximum structural depth.
    pub depth_limit: usize,
}
impl Default for TypedProgramLinksLimits {
    fn default() -> Self {
        Self {
            node_limit: 100_000,
            depth_limit: 256,
        }
    }
}
/// Code and the existing portable-core translation contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedProgramLinksEmission {
    /// Executable target code.
    pub code: String,
    /// Target language.
    pub language: Language,
    /// Source span and target name mappings.
    pub mappings: Vec<Mapping>,
    /// Explicit target assumptions.
    pub assumptions: Vec<Assumption>,
    /// Representation choices.
    pub encodings: Vec<Encoding>,
    /// Proof obligations.
    pub theorems: Vec<TheoremRecord>,
}

/// Optional explicit models for emission from the current checked links.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TypedProgramLinksEmissionOptions {
    /// Selected primitive immutable-reference source-header interpretation.
    pub rust_source_header_emission_model: Option<String>,
    /// Canonical aliases for checked JavaScript local binding scopes.
    pub java_script_scoped_binding_emission_model: Option<String>,
}

pub fn validate_checked_program(program: &Program) -> Result<()> {
    checked::validate(program)
}
/// Exact schema shared by both packages.
#[must_use]
pub fn typed_program_links_schema() -> &'static Value {
    static SCHEMA: OnceLock<Value> = OnceLock::new();
    SCHEMA.get_or_init(|| {
        crate::read_structured_data_lino(include_str!("data/typed-program-links-schema.lino"))
            .expect("generated typed schema is structured Links data")
    })
}
fn metadata(term: &str, definition: Option<&str>) -> LinkMetadata {
    let result = LinkMetadata::new()
        .with_link_type(LinkType::Semantic)
        .with_named(true)
        .with_language(typed_program_links_schema()["language"].as_str().unwrap())
        .with_term(term);
    if let Some(definition) = definition {
        result.with_definition(definition)
    } else {
        result
    }
}
pub fn label_of(name: &str) -> String {
    typed_program_links_schema()["fields"][name]
        .as_str()
        .map_or_else(
            || {
                let mut label = String::new();
                for character in name.chars() {
                    if character.is_ascii_uppercase() {
                        label.push(' ');
                        label.push(character.to_ascii_lowercase());
                    } else {
                        label.push(character);
                    }
                }
                label
            },
            str::to_owned,
        )
}
pub fn field_of(label: &str) -> String {
    if let Some((field, _)) = typed_program_links_schema()["fields"]
        .as_object()
        .unwrap()
        .iter()
        .find(|(_, value)| value.as_str() == Some(label))
    {
        return field.clone();
    }
    let mut name = String::new();
    let mut uppercase = false;
    for character in label.chars() {
        if character == ' ' {
            uppercase = true;
        } else {
            name.push(if uppercase {
                character.to_ascii_uppercase()
            } else {
                character
            });
            uppercase = false;
        }
    }
    name
}
/// Encode checked IR into inspectable links.
///
/// # Errors
/// Refuses unsupported constructs or inconsistent checked semantics.
pub fn encode_typed_program_links(program: &Program) -> Result<TypedProgramLinks> {
    encode_typed_program_links_with_provenance(program, None)
}
/// Encode checked IR with separate structured provenance.
///
/// # Errors
/// As [`encode_typed_program_links`].
pub fn encode_typed_program_links_with_provenance(
    program: &Program,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    checked::validate(program)?;
    codec::encode(program, provenance)
}
/// Encode exact scalar/type/source-span value sharing. Declaration, expression
/// and lexical binder identities remain independently represented.
///
/// # Errors
/// Refuses unknown profiles and every existing checked-program defect.
pub fn encode_typed_program_links_with_value_sharing(
    program: &Program,
    provenance: Option<&Value>,
    profile: &str,
) -> Result<TypedProgramLinks> {
    let definition =
        crate::read_structured_data_lino(include_str!("data/typed-program-value-sharing.lino"))
            .map_err(|failure| error(failure.to_string()))?;
    if definition["model"].as_str() != Some(profile)
        || definition["schemaVersion"] != serde_json::json!(1)
        || definition["scalarValues"] != serde_json::json!(true)
        || definition["typeRecords"] != serde_json::json!(true)
        || definition["sourceSpans"] != serde_json::json!(true)
        || definition["occurrences"] != serde_json::json!(false)
    {
        return Err(error("unregistered typed program value sharing profile"));
    }
    checked::validate(program)?;
    codec::encode_values(program, provenance)
}
/// Decode and recheck a graph with the default structural bounds.
///
/// # Errors
/// Refuses cycles, dangling references, scope violations, invalid values,
/// unsupported constructs and inconsistent checked semantics.
pub fn decode_typed_program_links(network: &LinkNetwork, root: LinkId) -> Result<Program> {
    decode_typed_program_links_with_limits(network, root, TypedProgramLinksLimits::default())
}
/// Decode using caller-specified structural bounds.
///
/// # Errors
/// As [`decode_typed_program_links`], including a resource diagnostic.
pub fn decode_typed_program_links_with_limits(
    network: &LinkNetwork,
    root: LinkId,
    limits: TypedProgramLinksLimits,
) -> Result<Program> {
    let program = codec::decode(network, root, limits)?;
    checked::validate(&program)?;
    Ok(program)
}

pub fn read_typed_program_binding_identities(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<Vec<LinkId>> {
    let (program, identities) =
        codec::decode_with_binding_identities(network, root, TypedProgramLinksLimits::default())?;
    checked::validate(&program)?;
    Ok(identities)
}
/// Import JavaScript or Rust through its existing typed frontend.
///
/// # Errors
/// Propagates frontend/checker and supported-adapter diagnostics.
pub fn create_typed_program_links(source: &str, language: &str) -> Result<TypedProgramLinks> {
    let surface = match language.to_ascii_lowercase().as_str() {
        "javascript" | "js" | "mjs" => parse_javascript(source),
        "rust" | "rs" => parse_rust(source),
        _ => {
            return Err(error(
                "typed linked source import currently reads JavaScript and Rust",
            ));
        }
    }
    .map_err(|failure| error(failure.to_string()))?;
    encode_typed_program_links(
        &check_program(&surface).map_err(|failure| error(failure.to_string()))?,
    )
}
/// Import Rust with an explicit versioned array result model and provenance.
///
/// The selected source profile is serialized separately from caller provenance.
/// An empty model retains the legacy source admission; the fresh scalar model
/// checks constructor tails before projection into immutable array values.
///
/// # Errors
/// Refuses non-Rust source languages, unknown models and unproved result tails,
/// and propagates frontend/checker and supported-adapter diagnostics.
pub fn create_typed_program_links_with_rust_array_result_model(
    source: &str,
    language: &str,
    model: &str,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    create_typed_program_links_with_rust_source_models(
        source,
        language,
        Some(model),
        None,
        provenance,
    )
}

/// Import under the explicit closed guarded-return source profile.
///
/// # Errors
/// Refuses non-Rust sources, unknown profiles and unsupported completions.
pub fn create_typed_program_links_with_rust_completion_model(
    source: &str,
    language: &str,
    model: &str,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    create_typed_program_links_with_rust_source_models(
        source,
        language,
        None,
        Some(model),
        provenance,
    )
}

/// Import with independent source models and inspectable caller provenance.
///
/// Only selected models are stored. Selecting either Rust model for JavaScript
/// is refused; no selection preserves its original source-admission contract.
///
/// # Errors
/// Propagates language, model, source, checker and linked-adapter diagnostics.
pub fn create_typed_program_links_with_rust_source_models(
    source: &str,
    language: &str,
    array_result_model: Option<&str>,
    completion_model: Option<&str>,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    create_typed_program_links_with_rust_source_profiles(
        source,
        language,
        array_result_model,
        completion_model,
        None,
        provenance,
    )
}

/// Import Rust with an inspectable owned source function signature witness.
///
/// # Errors
/// Refuses non-Rust sources, unknown models and unsupported header facts.
pub fn create_typed_program_links_with_rust_source_signature_model(
    source: &str,
    language: &str,
    model: &str,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    create_typed_program_links_with_rust_source_profiles(
        source,
        language,
        None,
        None,
        Some(model),
        provenance,
    )
}

/// Import a complete primitive leaf program with owned standard binary64 paths.
///
/// Canonical primitive types stay separate from the actual qualified path,
/// import resolution and source-signature reference evidence. Caller provenance
/// is stored separately from these checked source facts.
///
/// # Errors
/// Refuses non-Rust sources, unknown models and unproved owners or leaf contexts.
pub fn create_typed_program_links_with_rust_primitive_owner_model(
    source: &str,
    language: &str,
    model: &str,
    source_signature_model: Option<&str>,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    if !matches!(language.to_ascii_lowercase().as_str(), "rust" | "rs") {
        return Err(error("a Rust primitive owner model requires Rust source"));
    }
    let surface =
        parse_rust_with_primitive_owner_model(source, model, source_signature_model.unwrap_or(""))
            .map_err(|failure| error(failure.to_string()))?;
    let program = check_program(&surface).map_err(|failure| error(failure.to_string()))?;
    let mut profile = serde_json::Map::new();
    profile.insert("language".into(), Value::String("Rust".into()));
    profile.insert(
        "rustPrimitiveOwnerModel".into(),
        Value::String(model.into()),
    );
    if let Some(model) = source_signature_model {
        profile.insert(
            "rustSourceSignatureModel".into(),
            Value::String(model.into()),
        );
    }
    let selected = serde_json::json!({
        "sourceProfile": profile,
        "userProvenance": provenance,
    });
    encode_typed_program_links_with_provenance(&program, Some(&selected))
}

/// Import an independently selected complete pure Unit program with owned call sites.
///
/// # Errors
/// Refuses non-Rust sources, unknown models and unsupported source context.
pub fn create_typed_program_links_with_rust_pure_unit_call_model(
    source: &str,
    language: &str,
    model: &str,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    if !matches!(language.to_ascii_lowercase().as_str(), "rust" | "rs") {
        return Err(error("a Rust pure Unit-call model requires Rust source"));
    }
    let surface = parse_rust_with_pure_unit_call_model(source, model)
        .map_err(|failure| error(failure.to_string()))?;
    let program = check_program(&surface).map_err(|failure| error(failure.to_string()))?;
    let selected = serde_json::json!({"sourceProfile":{"language":"Rust","rustPureUnitCallModel":model},"userProvenance":provenance});
    encode_typed_program_links_with_provenance(&program, Some(&selected))
}

/// Import a complete closed emitted Rust runtime with inspectable owned operations.
///
/// # Errors
/// Refuses non-Rust sources, unknown models and unproved runtime assumptions.
pub fn create_typed_program_links_with_rust_runtime_shell_model(
    source: &str,
    language: &str,
    model: &str,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    if !matches!(language.to_ascii_lowercase().as_str(), "rust" | "rs") {
        return Err(error("a Rust runtime shell model requires Rust source"));
    }
    let surface = parse_rust_with_runtime_shell_model(source, model)
        .map_err(|failure| error(failure.to_string()))?;
    let program = check_program(&surface).map_err(|failure| error(failure.to_string()))?;
    let selected = serde_json::json!({"sourceProfile":{"language":"Rust","rustRuntimeShellModel":model},"userProvenance":provenance});
    encode_typed_program_links_with_provenance(&program, Some(&selected))
}

/// Import actual standard Rust shared-array handles through the checked link model.
///
/// # Errors
/// Refuses non-Rust sources, unknown models and unproved handle operations.
pub fn create_typed_program_links_with_rust_shared_array_handle_source_model(
    source: &str,
    language: &str,
    model: &str,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    if !matches!(language.to_ascii_lowercase().as_str(), "rust" | "rs") {
        return Err(error(
            "a Rust shared array handle source model requires Rust source",
        ));
    }
    let surface =
        crate::translation::rust::parse_rust_with_shared_array_handle_source_model(source, model)
            .map_err(|failure| error(failure.to_string()))?;
    let program = check_program(&surface).map_err(|failure| error(failure.to_string()))?;
    let selected = serde_json::json!({"sourceProfile":{"language":"Rust","rustSharedArrayHandleSourceModel":model},"userProvenance":provenance});
    encode_typed_program_links_with_provenance(&program, Some(&selected))
}

/// Import owned const qualification on closed primitive Rust predicates.
///
/// # Errors
/// Refuses non-Rust sources, unknown models and unproved predicate context.
pub fn create_typed_program_links_with_rust_const_function_model(
    source: &str,
    language: &str,
    model: &str,
    source_signature_model: Option<&str>,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    if !matches!(language.to_ascii_lowercase().as_str(), "rust" | "rs") {
        return Err(error("a Rust const function model requires Rust source"));
    }
    let surface =
        parse_rust_with_const_function_model(source, model, source_signature_model.unwrap_or(""))
            .map_err(|failure| error(failure.to_string()))?;
    let program = check_program(&surface).map_err(|failure| error(failure.to_string()))?;
    let mut profile = serde_json::Map::new();
    profile.insert("language".into(), Value::String("Rust".into()));
    profile.insert("rustConstFunctionModel".into(), Value::String(model.into()));
    if let Some(model) = source_signature_model {
        profile.insert(
            "rustSourceSignatureModel".into(),
            Value::String(model.into()),
        );
    }
    let selected = serde_json::json!({"sourceProfile":profile,"userProvenance":provenance});
    encode_typed_program_links_with_provenance(&program, Some(&selected))
}

/// Import explicit owned String moves or canonical local copies as editable links.
///
/// # Errors
/// Refuses non-Rust source, unknown models and unproved ownership or local state.
pub fn create_typed_program_links_with_rust_owned_local_state_model(
    source: &str,
    language: &str,
    model: &str,
    source_signature_model: Option<&str>,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    if !matches!(language.to_ascii_lowercase().as_str(), "rust" | "rs") {
        return Err(error("a Rust owned local state model requires Rust source"));
    }
    let surface = parse_rust_with_owned_local_state_model(
        source,
        model,
        source_signature_model.unwrap_or(""),
    )
    .map_err(|failure| error(failure.to_string()))?;
    let program = check_program(&surface).map_err(|failure| error(failure.to_string()))?;
    let mut profile = serde_json::Map::new();
    profile.insert("language".into(), Value::String("Rust".into()));
    profile.insert(
        "rustOwnedLocalStateModel".into(),
        Value::String(model.into()),
    );
    if let Some(model) = source_signature_model {
        profile.insert(
            "rustSourceSignatureModel".into(),
            Value::String(model.into()),
        );
    }
    let selected = serde_json::json!({"sourceProfile":profile,"userProvenance":provenance});
    encode_typed_program_links_with_provenance(&program, Some(&selected))
}

/// Import Rust with owned type spellings rechecked against actual scoped declarations.
///
/// The selected profile captures the source signature independently of caller
/// provenance. Emitted definition replay does not admit arbitrary runtime shells.
///
/// # Errors
/// Refuses non-Rust sources, unknown models and unproved source type bindings.
pub fn create_typed_program_links_with_rust_type_namespace_model(
    source: &str,
    language: &str,
    model: &str,
    source_signature_model: Option<&str>,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    if !matches!(language.to_ascii_lowercase().as_str(), "rust" | "rs") {
        return Err(error("a Rust type namespace model requires Rust source"));
    }
    let surface =
        parse_rust_with_type_namespace_model(source, model, source_signature_model.unwrap_or(""))
            .map_err(|failure| error(failure.to_string()))?;
    let program = check_program(&surface).map_err(|failure| error(failure.to_string()))?;
    let mut profile = serde_json::Map::new();
    profile.insert("language".into(), Value::String("Rust".into()));
    profile.insert("rustTypeNamespaceModel".into(), Value::String(model.into()));
    if let Some(model) = source_signature_model {
        profile.insert(
            "rustSourceSignatureModel".into(),
            Value::String(model.into()),
        );
    }
    let selected = serde_json::json!({"sourceProfile":profile,"userProvenance":provenance});
    encode_typed_program_links_with_provenance(&program, Some(&selected))
}

/// Import the independently selected closed empty primitive option source model.
///
/// The original source factories keep their existing type admission. The owned
/// empty result interpretation and caller provenance remain separate records.
///
/// # Errors
/// Refuses non-Rust sources, unknown models and unproved owners or empty bodies.
pub fn create_typed_program_links_with_rust_empty_option_source_model(
    source: &str,
    language: &str,
    model: &str,
    provenance: Option<&Value>,
) -> Result<TypedProgramLinks> {
    if !matches!(language.to_ascii_lowercase().as_str(), "rust" | "rs") {
        return Err(error(
            "a Rust empty option source model requires Rust source",
        ));
    }
    let surface = parse_rust_with_empty_option_source_model(source, model)
        .map_err(|failure| error(failure.to_string()))?;
    let program = check_program(&surface).map_err(|failure| error(failure.to_string()))?;
    let selected = serde_json::json!({
        "sourceProfile":{"language":"Rust","rustEmptyOptionSourceModel":model},
        "userProvenance":provenance,
    });
    encode_typed_program_links_with_provenance(&program, Some(&selected))
}

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

/// Emit target-native code from a reloaded semantic graph.
///
/// # Errors
/// Refuses invalid graph edits and propagates target obligations.
pub fn emit_typed_program_links(
    network: &LinkNetwork,
    root: LinkId,
    target: &str,
) -> Result<TypedProgramLinksEmission> {
    emit_typed_program_links_with_options(
        network,
        root,
        target,
        &TypedProgramLinksEmissionOptions::default(),
    )
}

/// Emit from checked links with an explicitly selected target-interface model.
///
/// # Errors
/// As [`emit_typed_program_links`], including unsupported selected interfaces.
pub fn emit_typed_program_links_with_options(
    network: &LinkNetwork,
    root: LinkId,
    target: &str,
    options: &TypedProgramLinksEmissionOptions,
) -> Result<TypedProgramLinksEmission> {
    let source_header_model = options.rust_source_header_emission_model.as_deref();
    let rust_target = matches!(target.to_ascii_lowercase().as_str(), "rust" | "rs");
    if rust_target
        && options.java_script_scoped_binding_emission_model.is_none()
        && let Some(model) = source_header_model
    {
        crate::translation::source_header_emission::validate_rust_source_header_emission_model(
            model,
        )
        .map_err(|failure| error(failure.to_string()))?;
        if !crate::translation::rust_scalar_source_interface_rules::accept_rust_scalar_source_interface_model(model)
            && !crate::translation::rust_scalar_mathematical_interface_rules::accept_rust_scalar_mathematical_interface_model(model)
            && !crate::translation::rust_textual_source_interface_rules::accept_rust_textual_source_interface_model(model)
            && !crate::translation::rust_string_predicate_interface_rules::accept_rust_string_predicate_interface_model(model)
            && !crate::translation::rust_constructed_string_interface_rules::accept_rust_constructed_string_interface_model(model)
            && !crate::translation::rust_displayed_string_interface_rules::accept_rust_displayed_string_interface_model(model) {
            crate::shared_numeric_function_model::validate_shared_numeric_origin_context(network, root)
                .map_err(|failure| error(format!("Rust source header emission: {failure}")))?;
        }
    }
    if matches!(
        target.to_ascii_lowercase().as_str(),
        "javascript" | "js" | "mjs"
    ) && options.rust_source_header_emission_model.is_none()
        && let Some(model) = options.java_script_scoped_binding_emission_model.as_deref()
    {
        crate::translation::scoped_binding_emission::validate_raw_program(network, root, model)
            .map_err(|failure| error(failure.to_string()))?;
    }
    let decoded = if rust_target
        && options.java_script_scoped_binding_emission_model.is_none()
        && let Some(model) = source_header_model.filter(|model| {
            crate::translation::borrowed_call_interface_rules::accept_rust_borrowed_call_interface_model(model)
        })
    {
        borrowed_call_diagnostics::decode(network, root, model)
    } else {
        decode_typed_program_links(network, root).map_err(|failure| (failure, false))
    };
    let program = decoded.map_err(|(failure, refined)| {
        if !refined
            && rust_target
            && options.java_script_scoped_binding_emission_model.is_none()
            && source_header_model.is_some()
        {
            error(format!("Rust source header emission: {failure}"))
        } else {
            failure
        }
    })?;
    let emitted = match target.to_ascii_lowercase().as_str() {
        "javascript" | "js" | "mjs" => {
            if source_header_model.is_some() {
                return Err(error(
                    "a Rust source header emission model requires Rust target",
                ));
            }
            let scoped_binding_model = options.java_script_scoped_binding_emission_model.as_deref();
            if let Some(model) = scoped_binding_model {
                crate::translation::scoped_binding_emission::validate_program(&program, model)
                    .map_err(|failure| error(failure.to_string()))?;
                crate::translation::scoped_binding_emission::validate_origin_context(
                    network, root, &program,
                )
                .map_err(|failure| error(failure.to_string()))?;
            }
            emit_javascript_with_options(
                &program,
                JavaScriptEmissionOptions {
                    scoped_binding_model,
                    identifier_model: None,
                },
            )
        }
        "rust" | "rs" => {
            if options.java_script_scoped_binding_emission_model.is_some() {
                return Err(error(
                    "a JavaScript scoped binding emission model requires JavaScript target",
                ));
            }
            emit_rust_with_array_consumption(
                &program,
                RustEmissionOptions {
                    source_header_model,
                },
            )
        }
        "lean" | "rocq" => {
            if source_header_model.is_some() {
                return Err(error(
                    "a Rust source header emission model requires Rust target",
                ));
            }
            if options.java_script_scoped_binding_emission_model.is_some() {
                return Err(error(
                    "a JavaScript scoped binding emission model requires JavaScript target",
                ));
            }
            if target.eq_ignore_ascii_case("lean") {
                crate::translation::emit_lean::emit_lean(&program)
            } else {
                crate::translation::emit_rocq::emit_rocq(&program)
            }
        }
        _ => {
            return Err(error(
                "typed linked emission writes JavaScript, Rust, Lean and Rocq",
            ));
        }
    }
    .map_err(|failure| error(failure.to_string()))?;
    Ok(TypedProgramLinksEmission {
        code: emitted.text,
        language: emitted.language,
        mappings: emitted.mappings,
        assumptions: emitted.assumptions,
        encodings: emitted.encodings,
        theorems: emitted.theorems,
    })
}
