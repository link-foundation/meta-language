//! Editable source-free programs in the shared typed links schema.
//! The codec adapts identities and data; semantic checking and emission reuse
//! the existing portable core.
use crate::translation::Language;
use crate::translation::check::check_program;
use crate::translation::emit_common::{Assumption, Encoding, Mapping, TheoremRecord};
use crate::translation::emit_javascript::emit_javascript;
use crate::translation::emit_rust::emit_rust;
use crate::translation::ir::Program;
use crate::translation::javascript::parse_javascript;
use crate::translation::rust::parse_rust;
use crate::{LinkId, LinkMetadata, LinkNetwork, LinkType};
use serde_json::Value;
use std::fmt;
use std::sync::OnceLock;

mod checked;
mod codec;
mod projection;

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
fn label_of(name: &str) -> String {
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
fn field_of(label: &str) -> String {
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

pub(crate) fn read_typed_program_binding_identities(
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
/// Emit target-native code from a reloaded semantic graph.
///
/// # Errors
/// Refuses invalid graph edits and propagates target obligations.
pub fn emit_typed_program_links(
    network: &LinkNetwork,
    root: LinkId,
    target: &str,
) -> Result<TypedProgramLinksEmission> {
    let program = decode_typed_program_links(network, root)?;
    let emitted = match target.to_ascii_lowercase().as_str() {
        "javascript" | "js" | "mjs" => emit_javascript(&program),
        "rust" | "rs" => emit_rust(&program),
        _ => {
            return Err(error(
                "typed linked emission currently writes JavaScript and Rust",
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
