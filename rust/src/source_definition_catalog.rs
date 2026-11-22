//! Discover complete qualified source graphs without loading the whole catalog.
//! Digests authenticate artifacts; formal equivalence requires its own proof.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::source_definition_catalog_rules::{self as rules, ml_string::Text};
use crate::typed_program_links::decode_typed_program_links_with_limits;
use crate::{
    FormalConceptArchive, FormalFunctionSelector, LinkId, LinkNetwork, TypedProgramLinks,
    TypedProgramLinksLimits, encode_formal_function_definition, read_structured_data_lino,
};

mod assets;
include!("../tests/internal/source_definition_catalog.rs");

/// One exact complete source artifact, with a root local to that artifact.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceDefinitionEntry {
    /// Qualified catalog selector, never a semantic equivalence claim.
    pub identifier: String,
    /// Literal paired package asset basename.
    pub asset: String,
    /// Original UTF8 byte count.
    pub bytes: usize,
    /// SHA256 of every original asset byte.
    pub sha256: String,
    /// Exact decimal root in the present safe-number graph profile.
    pub root: String,
    /// Select the reloadable directory of actual source modules.
    #[serde(
        default,
        deserialize_with = "deserialize_source_archive_flag",
        skip_serializing_if = "std::ops::Not::not"
    )]
    pub ordinary_source_archive: bool,
    /// Artifact-qualified identity, distinct from concept meaning.
    #[serde(default)]
    pub definition_identity: String,
}

fn deserialize_source_archive_flag<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    if bool::deserialize(deserializer)? {
        Ok(true)
    } else {
        Err(serde::de::Error::custom(
            "ordinarySourceArchive must be true when present",
        ))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Catalog {
    schema_version: u32,
    model: String,
    entries: Vec<SourceDefinitionEntry>,
}

/// Bounds apply to the selected graph; other definitions remain unloaded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceDefinitionCatalogLimits {
    /// Maximum serialized program bytes, at most 128MiB.
    pub byte_limit: usize,
    /// Maximum graph and structurally owned node counts, at most one million.
    pub node_limit: usize,
    /// Maximum structurally owned value depth, at most 1024.
    pub depth_limit: usize,
}
impl Default for SourceDefinitionCatalogLimits {
    fn default() -> Self {
        Self {
            byte_limit: 16 * 1024 * 1024,
            node_limit: 100_000,
            depth_limit: 256,
        }
    }
}

fn fail(message: impl std::fmt::Display) -> String {
    format!("source definition catalog: {message}")
}
fn texts(values: impl Iterator<Item = String>) -> Vec<Text> {
    values.map(|value| Text::from_text(&value)).collect()
}
fn number(value: usize) -> Result<f64, String> {
    serde_json::Number::from(value)
        .as_f64()
        .ok_or_else(|| fail("unrepresentable host resource value"))
}

/// Read independent catalog metadata without decoding any represented program.
///
/// # Errors
/// Returns malformed data, unknown profiles or shared entry-policy diagnostics.
pub fn source_definition_catalog() -> Result<Vec<SourceDefinitionEntry>, String> {
    let value = read_structured_data_lino(include_str!("data/source-definition-catalog.lino"))
        .map_err(fail)?;
    let mut catalog: Catalog = serde_json::from_value(value).map_err(fail)?;
    if !rules::accept_source_definition_catalog_header(
        f64::from(catalog.schema_version),
        &Text::from_text(&catalog.model),
        number(catalog.entries.len())?,
    ) {
        return Err(fail("unsupported catalog header"));
    }
    let entries = &catalog.entries;
    let code = rules::validate_source_definition_catalog_entries(
        &texts(entries.iter().map(|entry| entry.identifier.clone())),
        &texts(entries.iter().map(|entry| entry.asset.clone())),
        &texts(entries.iter().map(|entry| entry.sha256.clone())),
        &entries
            .iter()
            .map(|entry| number(entry.bytes))
            .collect::<Result<Vec<_>, _>>()?,
        &texts(entries.iter().map(|entry| entry.root.clone())),
    );
    if !matches!(code, 1.0) {
        return Err(fail(format!("invalid catalog entries (reason {code})")));
    }
    for entry in &mut catalog.entries {
        if entry.ordinary_source_archive
            && number(entry.bytes)? > rules::read_source_definition_catalog_byte_limit(true)
        {
            return Err(fail("invalid ordinary source archive entry"));
        }
        entry.definition_identity = rules::read_source_definition_catalog_identity(
            &Text::from_text(&entry.identifier),
            &Text::from_text(&entry.sha256),
        )
        .to_string();
    }
    Ok(catalog.entries)
}

/// Select the actual qualified identity without any alias or language fallback.
///
/// # Errors
/// Returns catalog diagnostics; an unknown identity returns `Ok(None)`.
pub fn source_definition_entry(identifier: &str) -> Result<Option<SourceDefinitionEntry>, String> {
    let entries = source_definition_catalog()?;
    let index = rules::select_source_definition_catalog_entry(
        &texts(entries.iter().map(|entry| entry.identifier.clone())),
        &Text::from_text(identifier),
    );
    if index < 0.0 {
        return Ok(None);
    }
    let index = crate::translation::frontend_rules::ml_array::number_index(index)
        .ok_or_else(|| fail("invalid selected catalog position"))?;
    Ok(entries.into_iter().nth(index))
}

/// Authenticate and statically recheck one complete editable source graph.
///
/// # Errors
/// Returns unknown selectors, changed assets/roots, resource or typed errors.
pub fn create_source_definition_program_links(
    identifier: &str,
) -> Result<TypedProgramLinks, String> {
    let entry = source_definition_entry(identifier)?
        .ok_or_else(|| fail("unknown qualified program identity"))?;
    let archive = entry.ordinary_source_archive;
    let index = crate::translation::frontend_rules::ml_array::number_index;
    let limits = SourceDefinitionCatalogLimits {
        byte_limit: index(rules::read_source_definition_catalog_byte_limit(archive))
            .ok_or_else(|| fail("invalid default byte budget"))?,
        node_limit: index(rules::read_source_definition_catalog_node_limit(archive))
            .ok_or_else(|| fail("invalid default node budget"))?,
        ..SourceDefinitionCatalogLimits::default()
    };
    load_source_definition_program_links(&entry, limits)
}

/// As the default constructor, with explicit graph bounds.
///
/// # Errors
/// Returns invalid bounds and every diagnostic of the default constructor.
pub fn create_source_definition_program_links_with_limits(
    identifier: &str,
    limits: SourceDefinitionCatalogLimits,
) -> Result<TypedProgramLinks, String> {
    let entry = source_definition_entry(identifier)?
        .ok_or_else(|| fail("unknown qualified program identity"))?;
    load_source_definition_program_links(&entry, limits)
}

fn load_source_definition_program_links(
    entry: &SourceDefinitionEntry,
    limits: SourceDefinitionCatalogLimits,
) -> Result<TypedProgramLinks, String> {
    if !rules::accept_source_definition_catalog_limits(
        number(limits.byte_limit)?,
        number(limits.node_limit)?,
        number(limits.depth_limit)?,
    ) {
        return Err(fail("invalid resource limit"));
    }
    if entry.bytes > limits.byte_limit {
        return Err(fail("program byte limit exceeded"));
    }
    let source = assets::source_asset(&entry.asset).ok_or_else(|| fail("unknown program asset"))?;
    if source.len() != entry.bytes
        || format!("{:x}", Sha256::digest(source.as_bytes())) != entry.sha256
    {
        return Err(fail("program asset identity changed"));
    }
    let network = if entry.ordinary_source_archive {
        LinkNetwork::from_bounded_lino(source, limits.byte_limit, limits.node_limit, 4_000_000)
    } else {
        LinkNetwork::from_lino(source)
    }
    .map_err(fail)?;
    if network.links().count() > limits.node_limit {
        return Err(fail("program node limit exceeded"));
    }
    let roots = network
        .links()
        .filter(|link| {
            link.metadata().language() == Some("meta-language typed program")
                && link.metadata().term() == Some("typed program")
                && link.metadata().definition() == Some("1")
        })
        .map(crate::Link::id)
        .collect::<Vec<_>>();
    let root = LinkId::from_u64(entry.root.parse::<u64>().map_err(fail)?);
    if roots != [root] {
        return Err(fail("program root identity changed"));
    }
    if entry.ordinary_source_archive {
        let label = rules::read_ordinary_source_archive_field().to_string();
        if !network.links().any(|relation| {
            relation.metadata().link_type() == Some(crate::LinkType::Field)
                && matches!(relation.references(), [parent, field, _]
                    if *parent == root
                        && network.link(*field).and_then(|link| link.metadata().term()) == Some(label.as_str()))
        }) {
            return Err(fail("ordinary source archive directory is missing"));
        }
    }
    decode_typed_program_links_with_limits(
        &network,
        root,
        TypedProgramLinksLimits {
            node_limit: limits.node_limit,
            depth_limit: limits.depth_limit,
        },
    )
    .map_err(fail)?;
    Ok(TypedProgramLinks { network, root })
}

/// Select an owned function and its complete recursive formal context.
/// Existing formal-model refusals remain binding for catalog entries.
///
/// # Errors
/// Returns loading, selector and unsupported formal-model diagnostics.
pub fn create_source_definition_formal_function_links(
    identifier: &str,
    declaration: FormalFunctionSelector<'_>,
) -> Result<FormalConceptArchive, String> {
    let program = create_source_definition_program_links(identifier)?;
    let entry = source_definition_entry(identifier)?
        .ok_or_else(|| fail("unknown qualified program identity"))?;
    let provenance: Value = serde_json::json!({"sourceDefinition":entry});
    encode_formal_function_definition(
        &program.network,
        program.root,
        declaration,
        Some(&provenance),
    )
}
