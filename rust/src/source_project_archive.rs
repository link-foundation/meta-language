//! Standard source/parser/graph transport over complete shared archive storage.
use crate::source_project_archive_rules::{self as rules, ml_string::Text};
use crate::{
    Link, LinkId, LinkMetadata, LinkNetwork, LinkType, MetaLanguageArchive,
    MetaLanguageArchiveEntry, MetaLanguageArchiveLimits, NetworkProjection, ParseConfiguration,
    StructuredDataLinksLimits, create_meta_language_archive,
    decode_structured_data_links_with_limits, encode_structured_data_links_with_limits,
    read_meta_language_archive,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, sync::Arc};
fn fail(message: &str) -> String {
    format!("source project archive: {message}")
}
fn text(value: &str) -> Text {
    Text::from_text(value)
}
fn vocabulary(role: &str) -> String {
    rules::read_source_project_archive_vocabulary(&text(role)).to_string()
}
fn bound(options: &Value, name: &str, default: usize) -> Result<usize, String> {
    match options.get(name) {
        None | Some(Value::Null) => Ok(default),
        Some(value) => value
            .as_f64()
            .filter(|value| {
                value.is_finite()
                    && value.fract() == 0.0
                    && *value >= 1.0
                    && *value <= 9_007_199_254_740_991.0
                    && *value <= usize::MAX as f64
            })
            .map(|value| value as usize)
            .ok_or_else(|| fail(&format!("invalid {name}"))),
    }
}
fn limits(options: &Value) -> Result<MetaLanguageArchiveLimits, String> {
    if !options.is_object() {
        return Err(fail("expected a resource options record"));
    }
    Ok(MetaLanguageArchiveLimits {
        node_limit: bound(options, "nodeLimit", 1000000)?,
        reference_limit: bound(options, "referenceLimit", 4000000)?,
        byte_limit: bound(options, "byteLimit", 128 * 1024 * 1024)?,
    })
}
fn projection_option(options: &Value, fallback: &str) -> Result<String, String> {
    match options.get("projection") {
        None | Some(Value::Null) => Ok(fallback.to_owned()),
        Some(value) => value
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| fail("unknown source syntax projection")),
    }
}

/// Append new source identities without reparsing or replacing any old source.
///
/// # Errors
/// Refuses collisions, invalid new sources and exhausted combined bounds.
pub fn extend_source_project_archive(
    network: &LinkNetwork,
    root: LinkId,
    additions: &Value,
    options: &Value,
) -> Result<MetaLanguageArchive, String> {
    let selected = limits(options)?;
    let previous = read_source_project_archive(network, root, options)?;
    let additions = additions
        .as_array()
        .ok_or_else(|| fail("invalid original source input"))?;
    let paths = additions
        .iter()
        .map(|row| {
            row["path"]
                .as_str()
                .map(text)
                .ok_or_else(|| fail("invalid original source input"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let error = rules::read_source_project_extension_error(
        &previous
            .sources
            .iter()
            .map(|file| text(&file.path))
            .collect::<Vec<_>>(),
        &paths,
    )
    .to_string();
    if !error.is_empty() {
        return Err(fail(&error));
    }
    if previous.sources.len().saturating_add(additions.len()) > bound(options, "fileLimit", 10000)?
    {
        return Err(fail("expected a bounded nonempty source inventory"));
    }
    let added = create_source_project_archive(&Value::Array(additions.clone()), options)?;
    let incoming = read_source_project_archive(&added.network, added.root, options)?;
    let mut sources = previous.sources;
    sources.extend(incoming.sources);
    sources.sort_by(|a, b| {
        let value = crate::project_translation_rules::compare_project_translation_identifiers(
            &crate::project_translation_rules::ml_string::Text::from_text(&a.path),
            &crate::project_translation_rules::ml_string::Text::from_text(&b.path),
        );
        if value < 0.0 {
            std::cmp::Ordering::Less
        } else if value > 0.0 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    let mut bytes = 0usize;
    for file in &sources {
        bytes = bytes
            .checked_add(file.bytes)
            .ok_or_else(|| fail("source byte limit exceeded"))?;
    }
    if bytes > selected.byte_limit {
        return Err(fail("source byte limit exceeded"));
    }
    let files=sources.iter().map(|file|json!({"bytes":file.bytes,"identifier":file.identifier,"language":file.language,"path":file.path,"sha256":file.sha256})).collect::<Vec<_>>();
    let projection =
        projection_option(options, previous.directory["projection"].as_str().unwrap())?;
    let directory = encode_structured_data_links_with_limits(
        &json!({"files":files,"model":vocabulary("model"),"projection":projection,"schemaVersion":1}),
        data_limits(selected),
    )?;
    let mut entries = vec![MetaLanguageArchiveEntry {
        identifier: vocabulary("directory"),
        kind: "language configuration".to_owned(),
        network: Arc::new(directory.network),
        root: directory.root,
    }];
    entries.extend(sources.into_iter().map(|file| MetaLanguageArchiveEntry {
        identifier: file.identifier,
        kind: "concrete syntax".to_owned(),
        network: file.network,
        root: file.root,
    }));
    create_meta_language_archive(&entries, selected)
}
fn data_limits(limits: MetaLanguageArchiveLimits) -> StructuredDataLinksLimits {
    StructuredDataLinksLimits {
        node_limit: limits.node_limit,
        depth_limit: 256,
        byte_limit: limits.byte_limit,
    }
}
fn digest(source: &str) -> String {
    format!("{:x}", Sha256::digest(source.as_bytes()))
}
fn root_positions(network: &LinkNetwork) -> Vec<LinkId> {
    let links = network.links().collect::<Vec<_>>();
    let mut firsts = Vec::new();
    let mut counts = Vec::new();
    let mut targets = Vec::new();
    for link in &links {
        firsts.push(targets.len() as f64);
        counts.push(link.references().len() as f64);
        targets.extend(link.references().iter().map(|id| id.as_u64() as f64));
    }
    rules::select_source_project_syntax_root_positions(
        &links
            .iter()
            .map(|link| {
                text(if link.metadata().link_type() == Some(LinkType::Syntax) {
                    "syntax"
                } else {
                    "other"
                })
            })
            .collect::<Vec<_>>(),
        &links
            .iter()
            .map(|link| link.id().as_u64() as f64)
            .collect::<Vec<_>>(),
        &firsts,
        &counts,
        &targets,
    )
    .into_iter()
    .map(|position| links[position as usize].id())
    .collect()
}
fn projection(value: &str) -> Result<NetworkProjection, String> {
    if !rules::accept_source_project_syntax_projection(&text(value)) {
        return Err(fail("unknown source syntax projection"));
    }
    Ok(if value == "AbstractSyntax" {
        NetworkProjection::AbstractSyntax
    } else {
        NetworkProjection::ConcreteSyntax
    })
}

/// Archive actual complete catalog-language concrete graphs, including recovery
/// nodes. An abstract view retains its full original concrete graph.
///
/// # Errors
/// Refuses malformed source inventories, unknown source languages, missing
/// concrete trees, lost original bytes and exhausted archive bounds.
pub fn create_source_project_archive(
    inputs: &Value,
    options: &Value,
) -> Result<MetaLanguageArchive, String> {
    let selected = limits(options)?;
    let projection_name = projection_option(options, "ConcreteSyntax")?;
    projection(&projection_name)?;
    let inputs = inputs
        .as_array()
        .filter(|rows| !rows.is_empty())
        .ok_or_else(|| fail("expected a bounded nonempty source inventory"))?;
    if inputs.len() > bound(options, "fileLimit", 10000)? {
        return Err(fail("expected a bounded nonempty source inventory"));
    }
    let mut records = Vec::new();
    let mut paths = BTreeSet::new();
    let mut byte_count = 0usize;
    for row in inputs {
        let path = row["path"]
            .as_str()
            .filter(|path| rules::accept_source_project_path(&text(path)))
            .ok_or_else(|| fail("invalid original source input"))?;
        let source = row["source"]
            .as_str()
            .ok_or_else(|| fail("invalid original source input"))?;
        let language = row["language"]
            .as_str()
            .and_then(crate::language_catalog::canonical_language_name)
            .ok_or_else(|| fail("unknown source language"))?;
        if !paths.insert(path) {
            return Err(fail("duplicate source project path"));
        }
        byte_count = byte_count
            .checked_add(source.len())
            .ok_or_else(|| fail("source byte limit exceeded"))?;
        records.push((path, source, language));
    }
    if byte_count > selected.byte_limit {
        return Err(fail("source byte limit exceeded"));
    }
    records.sort_by(|a, b| {
        let value = crate::project_translation_rules::compare_project_translation_identifiers(
            &crate::project_translation_rules::ml_string::Text::from_text(a.0),
            &crate::project_translation_rules::ml_string::Text::from_text(b.0),
        );
        if value < 0.0 {
            std::cmp::Ordering::Less
        } else if value > 0.0 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    let mut entries = Vec::new();
    let mut files = Vec::new();
    for (path, source, language) in records {
        let mut network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        let roots = root_positions(&network);
        if roots.is_empty() {
            return Err(fail(&format!(
                "the selected parser did not produce a concrete syntax tree: {path}"
            )));
        }
        if network.reconstruct_text() != source {
            return Err(fail(&format!(
                "concrete syntax lost original source bytes: {path}"
            )));
        }
        if options["requireClean"].as_bool() == Some(true)
            && !network.verify_full_match(None).is_clean()
        {
            return Err(fail(&format!("unclean original concrete syntax: {path}")));
        }
        let root = network.insert_link(
            [],
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_language(language)
                .with_term(vocabulary("root"))
                .with_definition(path),
        );
        network.set_references(root, &roots);
        let identifier = vocabulary("sourcePrefix") + path;
        entries.push(MetaLanguageArchiveEntry {
            identifier: identifier.clone(),
            kind: "concrete syntax".to_owned(),
            network: Arc::new(network),
            root,
        });
        files.push(json!({"bytes":source.len(),"identifier":identifier,"language":language,"path":path,"sha256":digest(source)}));
    }
    let directory = encode_structured_data_links_with_limits(
        &json!({"files":files,"model":vocabulary("model"),"projection":projection_name,"schemaVersion":1}),
        data_limits(selected),
    )?;
    entries.insert(
        0,
        MetaLanguageArchiveEntry {
            identifier: vocabulary("directory"),
            kind: "language configuration".to_owned(),
            network: Arc::new(directory.network),
            root: directory.root,
        },
    );
    create_meta_language_archive(&entries, selected)
}

/// A full original graph, original bytes and a derived concrete/abstract view.
#[derive(Clone, Debug)]
pub struct SourceProjectArchiveSource {
    /// Original project-relative source identity.
    pub path: String,
    /// Original selected canonical language, without current-catalog redispatch.
    pub language: String,
    /// Qualified archive entry identity.
    pub identifier: String,
    /// Exact original source byte count.
    pub bytes: usize,
    /// Original source digest.
    pub sha256: String,
    /// Source bytes reconstructed from concrete tokens and trivia.
    pub source: String,
    /// Complete graph, registrations, syntax state and allocator history.
    pub network: Arc<LinkNetwork>,
    /// Explicit source root linking all original concrete roots.
    pub root: LinkId,
    /// Derived view selection; concrete syntax remains stored in full.
    pub projection: NetworkProjection,
    /// Selected original links; every reference resolves in the complete graph.
    pub view: Vec<Link>,
}
/// Eagerly reconstructed complete sources and their original directory.
#[derive(Clone, Debug)]
pub struct SourceProjectArchiveView {
    /// Checked source identities, byte digests and original view selection.
    pub directory: Value,
    /// Every original graph with exact source and a derived view.
    pub sources: Vec<SourceProjectArchiveSource>,
}
fn file_shape(file: &Value) -> bool {
    file.as_object().is_some_and(|fields| {
        fields.len() == 5
            && ["path", "language", "identifier", "sha256"]
                .iter()
                .all(|name| file[*name].is_string())
            && file["bytes"].is_number()
    })
}
/// Reconstruct every source group without files, parsers or a current catalog.
///
/// # Errors
/// Refuses unknown directory versions, foreign entries, lost source bytes,
/// changed roots, invalid metadata and exhausted graph reconstruction bounds.
pub fn read_source_project_archive(
    network: &LinkNetwork,
    root: LinkId,
    options: &Value,
) -> Result<SourceProjectArchiveView, String> {
    let selected = limits(options)?;
    let archive = read_meta_language_archive(network, root, selected)?;
    let directories = archive
        .entries
        .iter()
        .filter(|entry| entry.identifier == vocabulary("directory"))
        .collect::<Vec<_>>();
    if directories.len() != 1 || directories[0].kind != "language configuration" {
        return Err(fail("missing source project directory"));
    }
    let directory = decode_structured_data_links_with_limits(
        &directories[0].network,
        directories[0].root,
        data_limits(selected),
    )?;
    let files = directory["files"]
        .as_array()
        .ok_or_else(|| fail("invalid source project directory"))?;
    if directory.as_object().is_none_or(|fields| fields.len() != 4)
        || !directory["model"].is_string()
        || !directory["schemaVersion"].is_number()
        || !directory["projection"].is_string()
        || files.iter().any(|file| !file_shape(file))
    {
        return Err(fail("invalid source project directory"));
    }
    let error = rules::read_source_project_directory_error(
        &text(directory["model"].as_str().unwrap()),
        directory["schemaVersion"].as_f64().unwrap(),
        &text(directory["projection"].as_str().unwrap()),
        &files
            .iter()
            .map(|file| text(file["path"].as_str().unwrap()))
            .collect::<Vec<_>>(),
        &files
            .iter()
            .map(|file| text(file["language"].as_str().unwrap()))
            .collect::<Vec<_>>(),
        &files
            .iter()
            .map(|file| text(file["identifier"].as_str().unwrap()))
            .collect::<Vec<_>>(),
        &files
            .iter()
            .map(|file| file["bytes"].as_f64().unwrap())
            .collect::<Vec<_>>(),
        &files
            .iter()
            .map(|file| text(file["sha256"].as_str().unwrap()))
            .collect::<Vec<_>>(),
    )
    .to_string();
    if !error.is_empty() {
        return Err(fail(&error));
    }
    let selected_projection = projection(&projection_option(
        options,
        directory["projection"].as_str().unwrap(),
    )?)?;
    if archive.entries.len() != files.len() + 1 {
        return Err(fail("foreign source project entry"));
    }
    let mut sources = Vec::new();
    for file in files {
        let entry = archive
            .entries
            .iter()
            .find(|entry| Some(entry.identifier.as_str()) == file["identifier"].as_str())
            .ok_or_else(|| fail("changed source concrete graph or directory"))?;
        let chosen = entry
            .network
            .link(entry.root)
            .ok_or_else(|| fail("changed source concrete graph or directory"))?;
        let source = entry.network.reconstruct_text();
        if entry.kind != "concrete syntax"
            || chosen.metadata().term() != Some(vocabulary("root").as_str())
            || chosen.metadata().language() != file["language"].as_str()
            || chosen.metadata().definition() != file["path"].as_str()
            || source.len() as f64 != file["bytes"].as_f64().unwrap()
            || digest(&source) != file["sha256"].as_str().unwrap()
        {
            return Err(fail("changed source concrete graph or directory"));
        }
        if chosen.references().is_empty()
            || chosen.references().iter().any(|id| {
                entry
                    .network
                    .link(*id)
                    .is_none_or(|link| link.metadata().link_type() != Some(LinkType::Syntax))
            })
        {
            return Err(fail("invalid original source concrete roots"));
        }
        let expected_roots = root_positions(&entry.network);
        if chosen.references() != expected_roots.as_slice() {
            return Err(fail("changed original source concrete roots"));
        }
        let view = entry
            .network
            .projected_links(selected_projection)
            .cloned()
            .collect();
        sources.push(SourceProjectArchiveSource {
            path: file["path"].as_str().unwrap().to_owned(),
            language: file["language"].as_str().unwrap().to_owned(),
            identifier: entry.identifier.clone(),
            bytes: source.len(),
            sha256: digest(&source),
            source,
            network: Arc::clone(&entry.network),
            root: entry.root,
            projection: selected_projection,
            view,
        });
    }
    Ok(SourceProjectArchiveView { directory, sources })
}
