//! Source, lexer, typed-program and graph transport for generated composition.
use crate::rust_project_source_rules::{self as rules, ml_string::Text};
use crate::translation::{
    Language,
    check::check_program,
    lexer::{TokenKind, tokenize},
    rust::parse_rust,
    surface::SProgram,
};
use crate::{LinkNetwork, LinkType, TypedProgramLinks, encode_typed_program_links_with_provenance};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

fn limit(options: &Value, name: &str, default: usize) -> Result<usize, String> {
    if !options.is_object() {
        return Err("Rust source project: expected a resource options record".to_owned());
    }
    match options.get(name) {
        None | Some(Value::Null) => Ok(default),
        Some(value) => value
            .as_f64()
            .filter(|value| {
                value.is_finite()
                    && value.fract() == 0.0
                    && *value > 0.0
                    && *value <= 9_007_199_254_740_991.0
                    && *value <= usize::MAX as f64
            })
            .map(|value| value as usize)
            .ok_or_else(|| format!("Rust source project: invalid {name}")),
    }
}
fn text(value: &str) -> Text {
    Text::from_text(value)
}
fn scalar(value: &Text) -> Result<String, String> {
    String::from_utf16(&value.0)
        .map_err(|_| "Rust source project: generated source is not UTF8".to_owned())
}
fn byte_offset(source: &str, unit: usize) -> Result<usize, String> {
    let mut count = 0;
    for (offset, character) in source.char_indices() {
        if count == unit {
            return Ok(offset);
        }
        count += character.len_utf16();
        if count > unit {
            return Err("Rust source project: token range splits a scalar".to_owned());
        }
    }
    if count == unit {
        Ok(source.len())
    } else {
        Err("Rust source project: token range exceeds its original source".to_owned())
    }
}

/// Compose actual supplied modules with explicit ranges and original sources.
///
/// # Errors
/// Refuses missing/ambiguous modules, configurations without an interpretation,
/// invalid input, nonterminating module files and exceeded resource budgets.
pub fn compose_rust_source_project(
    inputs: &Value,
    entry: &str,
    options: &Value,
) -> Result<Value, String> {
    if !options.is_object() {
        return Err("Rust source project: expected a resource options record".to_owned());
    }
    let inputs = inputs
        .as_array()
        .filter(|rows| !rows.is_empty())
        .ok_or_else(|| {
            "Rust source project: expected a bounded nonempty source inventory".to_owned()
        })?;
    if inputs.len() > limit(options, "fileLimit", 10000)? {
        return Err("Rust source project: expected a bounded nonempty source inventory".to_owned());
    }
    let mut files = Vec::new();
    let mut byte_count = 0usize;
    for row in inputs {
        let path = row["path"].as_str().ok_or_else(|| {
            "Rust source project: expected original UTF8 Rust source files".to_owned()
        })?;
        let source = row["source"].as_str().ok_or_else(|| {
            "Rust source project: expected original UTF8 Rust source files".to_owned()
        })?;
        byte_count = byte_count
            .checked_add(source.len())
            .ok_or_else(|| "Rust source project: input byte limit exceeded".to_owned())?;
        files.push(json!({"path":path,"source":source,"sourceBytes":source.len(),"sourceSha256":format!("{:x}",Sha256::digest(source.as_bytes()))}));
    }
    if byte_count > limit(options, "byteLimit", 128 * 1024 * 1024)? {
        return Err("Rust source project: input byte limit exceeded".to_owned());
    }
    files.sort_by(|a, b| {
        let result = crate::project_translation_rules::compare_project_translation_identifiers(
            &crate::project_translation_rules::ml_string::Text::from_text(
                a["path"].as_str().unwrap(),
            ),
            &crate::project_translation_rules::ml_string::Text::from_text(
                b["path"].as_str().unwrap(),
            ),
        );
        if result < 0.0 {
            std::cmp::Ordering::Less
        } else if result > 0.0 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    let mut paths = Vec::new();
    let mut sources = Vec::new();
    let mut kinds = Vec::new();
    let mut values = Vec::new();
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    let mut token_count = 0usize;
    for file in &files {
        paths.push(text(file["path"].as_str().unwrap()));
        sources.push(text(file["source"].as_str().unwrap()));
        let tokens = tokenize(file["source"].as_str().unwrap(), Language::Rust)
            .map_err(|error| error.to_string())?
            .tokens;
        token_count = token_count
            .checked_add(tokens.len())
            .ok_or_else(|| "Rust source project: token limit exceeded".to_owned())?;
        if token_count > limit(options, "tokenLimit", 2000000)? {
            return Err("Rust source project: token limit exceeded".to_owned());
        }
        kinds.push(
            tokens
                .iter()
                .map(|token| {
                    text(match token.kind {
                        TokenKind::Identifier if token.value == "mod" && token.raw == "mod" => {
                            "keyword"
                        }
                        TokenKind::Identifier => "identifier",
                        TokenKind::Punct => "punct",
                        TokenKind::String => "string",
                        TokenKind::Eof => "eof",
                        _ => "atom",
                    })
                })
                .collect::<Vec<_>>(),
        );
        values.push(
            tokens
                .iter()
                .map(|token| text(&token.value))
                .collect::<Vec<_>>(),
        );
        starts.push(
            tokens
                .iter()
                .map(|token| token.start as f64)
                .collect::<Vec<_>>(),
        );
        ends.push(
            tokens
                .iter()
                .map(|token| token.end as f64)
                .collect::<Vec<_>>(),
        );
    }
    let depth = limit(options, "depthLimit", 128)? as f64;
    let expansions = limit(options, "expansionLimit", 4096)? as f64;
    let output = limit(options, "outputUnitLimit", 128 * 1024 * 1024)? as f64;
    let events = std::panic::catch_unwind(|| {
        rules::expand_rust_project_source_events(
            &paths,
            &sources,
            &kinds,
            &values,
            &starts,
            &ends,
            &text(entry),
            depth,
            expansions,
            output,
        )
    })
    .map_err(|payload| {
        if let Some((class_name, message_units, _)) =
            payload.downcast_ref::<(String, Vec<u16>, bool)>()
        {
            format!(
                "{}: {}",
                class_name,
                String::from_utf16_lossy(message_units)
            )
        } else {
            "Rust source project: unexpected source composition failure".to_owned()
        }
    })?;
    let by_path = files
        .iter()
        .map(|file| {
            (
                file["path"].as_str().unwrap(),
                file["source"].as_str().unwrap(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut source = String::new();
    let mut mappings = Vec::new();
    let mut modules = Vec::new();
    let mut used = std::collections::BTreeSet::new();
    for row in events.chunks_exact(7) {
        let kind = scalar(&row[0])?;
        let path = scalar(&row[1])?;
        let first = scalar(&row[2])?
            .parse::<usize>()
            .map_err(|_| "Rust source project: invalid original source range".to_owned())?;
        let last = scalar(&row[3])?
            .parse::<usize>()
            .map_err(|_| "Rust source project: invalid original source range".to_owned())?;
        let fragment = scalar(&row[4])?;
        let module = scalar(&row[5])?;
        let provider = scalar(&row[6])?;
        let original = by_path
            .get(path.as_str())
            .ok_or_else(|| "Rust source project: unknown original source owner".to_owned())?;
        let start = byte_offset(original, first)?;
        let end = byte_offset(original, last)?;
        if kind == "file" {
            used.insert(path);
            continue;
        }
        if kind == "module" {
            modules.push(json!({"path":path,"start":start,"end":end,"startUnit":first,"endUnit":last,"module":module,"provider":provider}));
            continue;
        }
        let output_start = source.len();
        source.push_str(&fragment);
        mappings.push(json!({"kind":kind,"path":path,"start":start,"end":end,"startUnit":first,"endUnit":last,"outputStart":output_start,"outputEnd":source.len(),"module":module,"provider":provider}));
    }
    let unused = files
        .iter()
        .filter(|file| !used.contains(file["path"].as_str().unwrap()))
        .map(|file| file["path"].clone())
        .collect::<Vec<_>>();
    Ok(
        json!({"schemaVersion":1,"model":"complete Rust source module composition 1","entry":entry,"source":source,"files":files,"modules":modules,"mappings":mappings,"unusedSources":unused}),
    )
}

/// Ordinary source surface and its complete source composition receipt.
///
/// # Errors
/// Retains ordinary syntax, source ownership and admission failures.
pub fn parse_rust_source_project(
    inputs: &Value,
    entry: &str,
    options: &Value,
) -> Result<(Value, SProgram, String), String> {
    let composition = compose_rust_source_project(inputs, entry, options)?;
    let source = composition["source"].as_str().unwrap();
    let retain = match options.get("retainSourceTypeOwners") {
        None => false,
        Some(Value::Bool(value)) => *value,
        _ => return Err("Rust source project: retainSourceTypeOwners must be Boolean".to_owned()),
    };
    let source_version = match options.get("sourceVersion") {
        None => None,
        Some(Value::String(value)) => Some(
            crate::translation::rust::source_versions::canonical(value)
                .map_err(|error| error.to_string())?,
        ),
        _ => return Err("Rust sourceVersion must be a string".to_owned()),
    };
    let parse = |source: &str| {
        if let Some(version) = source_version.as_deref() {
            if retain {
                crate::translation::rust::parse_rust_with_source_type_owners_and_version(
                    source, version,
                )
            } else {
                crate::translation::rust::parse_rust_with_source_version(source, version)
            }
        } else if retain {
            crate::translation::rust::parse_rust_with_source_type_owners(source)
        } else {
            parse_rust(source)
        }
    };
    let (surface, normalization) = match parse(source) {
        Ok(surface) => (surface, "none"),
        Err(error)
            if options["library"].as_bool() == Some(true)
                && error.kind == crate::translation::diagnostics::ErrorKind::Syntax
                && error.reason == "a Rust program needs fn main" =>
        {
            (
                parse(&format!("{source}\nfn main(){{}}\n")).map_err(|error| error.to_string())?,
                "empty frontend entry for a source library",
            )
        }
        Err(error) => return Err(error.to_string()),
    };
    Ok((composition, surface, normalization.to_owned()))
}

/// Checked editable program with every complete original concrete graph.
///
/// # Errors
/// Refuses semantic errors, unclean source grammars and exhausted graph bounds.
pub fn create_rust_source_project_program_links(
    inputs: &Value,
    entry: &str,
    options: &Value,
) -> Result<TypedProgramLinks, String> {
    let (composition, surface, normalization) = parse_rust_source_project(inputs, entry, options)?;
    let program = check_program(&surface).map_err(|error| error.to_string())?;
    let mut provenance = json!({"sourceProject":composition,"entryNormalization":normalization,"userProvenance":options.get("provenance").unwrap_or(&Value::Null)});
    if let Some(version) = options.get("sourceVersion").and_then(Value::as_str) {
        let version = crate::translation::rust::source_versions::canonical(version)
            .map_err(|error| error.to_string())?;
        provenance["sourceProfile"] = json!({"language":"Rust","rustSourceVersion":version});
    }
    let mut packet = encode_typed_program_links_with_provenance(&program, Some(&provenance))
        .map_err(|error| error.to_string())?;
    let node_limit = limit(options, "nodeLimit", 1000000)?;
    let inputs = composition["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|file| json!({"path":file["path"],"source":file["source"],"language":"Rust"}))
        .collect::<Vec<_>>();
    let mut configuration = options.clone();
    let fields = configuration
        .as_object_mut()
        .ok_or_else(|| "Rust source project: expected a resource options record".to_owned())?;
    fields.insert("requireClean".to_owned(), json!(true));
    fields.insert("projection".to_owned(), json!("ConcreteSyntax"));
    let archive = crate::create_source_project_archive(&json!(inputs), &configuration)?;
    let links = archive.network.links().collect::<Vec<_>>();
    if packet
        .network
        .links()
        .count()
        .saturating_add(links.len())
        .saturating_add(2)
        > node_limit
    {
        return Err("Rust source project: concrete graph node limit exceeded".to_owned());
    }
    let mut ids = BTreeMap::new();
    for link in &links {
        ids.insert(
            link.id(),
            packet.network.insert_link([], link.metadata().clone()),
        );
    }
    for link in &links {
        let references = link
            .references()
            .iter()
            .map(|id| {
                ids.get(id).copied().ok_or_else(|| {
                    "Rust source project: dangling original concrete reference".to_owned()
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        packet.network.set_references(ids[&link.id()], &references);
    }
    packet.network.insert_field(
        packet.root,
        "source project concrete archive",
        ids[&archive.root],
    );
    Ok(packet)
}

/// A source-qualified complete concrete graph reconstructed from its links.
#[derive(Clone, Debug)]
pub struct RustSourceProjectConcreteGraph {
    /// Original project-relative filename.
    pub path: String,
    /// Exact original UTF8 bytes reconstructed from concrete tokens and trivia.
    pub source: String,
    /// Original graph members with their archived coordinates and registrations.
    pub network: LinkNetwork,
}
fn fields(network: &LinkNetwork, parent: crate::LinkId, label: &str) -> Vec<crate::LinkId> {
    network
        .links()
        .filter_map(|link| {
            let references = link.references();
            if link.metadata().link_type() != Some(LinkType::Field)
                || references.len() != 3
                || references[0] != parent
                || network.link(references[1])?.metadata().term() != Some(label)
            {
                None
            } else {
                Some(references[2])
            }
        })
        .collect()
}
/// Reconstruct concrete files from either codec, without files or a parser.
///
/// # Errors
/// Refuses missing/duplicate members, foreign references, lost registrations,
/// changed source digests, unclean concrete graphs and resource exhaustion.
pub fn read_rust_source_project_concrete_graphs(
    network: &LinkNetwork,
    root: crate::LinkId,
    options: &Value,
) -> Result<Vec<RustSourceProjectConcreteGraph>, String> {
    if network.link(root).is_none()
        || network.links().count() > limit(options, "nodeLimit", 1000000)?
    {
        return Err("Rust source project: invalid source graph root or node limit".to_owned());
    }
    let current = fields(network, root, "source project concrete archive");
    if !current.is_empty() {
        if current.len() != 1 || !fields(network, root, "source concrete graph").is_empty() {
            return Err("Rust source project: ambiguous source concrete archive".to_owned());
        }
        let mut configuration = options.clone();
        configuration
            .as_object_mut()
            .ok_or_else(|| "Rust source project: expected a resource options record".to_owned())?
            .insert("projection".to_owned(), json!("ConcreteSyntax"));
        return crate::read_source_project_archive(network, current[0], &configuration)?
            .sources
            .into_iter()
            .map(|file| {
                if file.language != "Rust" || !file.network.verify_full_match(None).is_clean() {
                    return Err("Rust source project: changed original concrete source".to_owned());
                }
                Ok(RustSourceProjectConcreteGraph {
                    path: file.path,
                    source: file.source,
                    network: (*file.network).clone(),
                })
            })
            .collect();
    }
    // Retain the complete preceding layout under its original reader roles.
    let mut seen = std::collections::BTreeSet::new();
    let mut results = Vec::new();
    for id in fields(network, root, "source concrete graph") {
        let inventory = network
            .link(id)
            .ok_or_else(|| "Rust source project: invalid source concrete inventory".to_owned())?;
        let path = inventory
            .metadata()
            .definition()
            .ok_or_else(|| "Rust source project: invalid source concrete inventory".to_owned())?;
        if inventory.metadata().term() != Some("complete original source concrete graph")
            || inventory.metadata().language() != Some("Rust source project")
            || !seen.insert(path.to_owned())
        {
            return Err("Rust source project: invalid source concrete inventory".to_owned());
        }
        let members = inventory.references();
        let ids = members
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        if ids.len() != members.len() {
            return Err("Rust source project: duplicate source concrete member".to_owned());
        }
        let mut concrete = LinkNetwork::new();
        for id in members {
            let link = network.link(*id).ok_or_else(|| {
                "Rust source project: foreign source concrete reference".to_owned()
            })?;
            if link.references().iter().any(|target| !ids.contains(target)) {
                return Err("Rust source project: foreign source concrete reference".to_owned());
            }
            concrete
                .links
                .insert(*id, std::sync::Arc::new(link.clone()));
            concrete.next_id = concrete.next_id.max(u128::from(id.as_u64()) + 1);
        }
        let registrations = fields(network, id, "registered concrete points");
        let digests = fields(network, id, "original source digest");
        if registrations.len() != 1 || digests.len() != 1 {
            return Err(
                "Rust source project: missing source graph registrations or digest".to_owned(),
            );
        }
        let points = network
            .link(registrations[0])
            .ok_or_else(|| "Rust source project: invalid source graph registrations".to_owned())?
            .references();
        if points
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != points.len()
        {
            return Err("Rust source project: invalid source graph registrations".to_owned());
        }
        for id in points {
            let term = concrete
                .link(*id)
                .and_then(|link| link.metadata().term())
                .ok_or_else(|| {
                    "Rust source project: foreign or ambiguous source graph registration".to_owned()
                })?
                .to_owned();
            if concrete
                .terms
                .insert(std::sync::Arc::from(term), *id)
                .is_some()
            {
                return Err(
                    "Rust source project: foreign or ambiguous source graph registration"
                        .to_owned(),
                );
            }
        }
        let source = concrete.reconstruct_text();
        let digest = format!("{:x}", Sha256::digest(source.as_bytes()));
        if network
            .link(digests[0])
            .and_then(|link| link.metadata().definition())
            != Some(digest.as_str())
            || !concrete.verify_full_match(None).is_clean()
        {
            return Err("Rust source project: changed original concrete source".to_owned());
        }
        results.push(RustSourceProjectConcreteGraph {
            path: path.to_owned(),
            source,
            network: concrete,
        });
    }
    if results.is_empty() {
        return Err("Rust source project: missing source concrete inventories".to_owned());
    }
    Ok(results)
}
