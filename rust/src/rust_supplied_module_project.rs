//! Supplied Rust module ownership backed by the original concrete archives.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};

use crate::rust_concrete_source_items::{concrete_fields, source_unit};
use crate::rust_concrete_source_rules::{
    test_rust_concrete_entry_owner_kind, test_rust_concrete_module_declaration,
};
use crate::rust_source_project::{create_rust_source_project_packet, limit};
use crate::translation::{
    Language,
    lexer::{Token, TokenKind, tokenize},
};
use crate::{
    LinkId, LinkNetwork, ProgramConcreteSyntaxFact, RustSourceProjectConcreteGraph,
    StructuredDataLinksLimits, TypedProgramLinks, compose_rust_source_project,
    decode_structured_data_links_with_limits, encode_structured_data_links_with_limits,
    read_program_concrete_syntax_facts, read_rust_source_project_concrete_graphs,
};

const MODEL: &str = "Rust supplied concrete module ownership 1";
const DIRECTORY: &str = "Rust module concrete ownership";

/// Checked program and the concrete module witnesses retained in its network.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustSuppliedModuleProject {
    /// Checked semantic graph with retained concrete sources and ownership.
    pub packet: TypedProgramLinks,
    /// Archive root within the same network.
    pub archive_root: LinkId,
    /// Source-relative module ownership and composition ranges.
    pub module_graph: Value,
    /// The composed source and its original ranges.
    pub composition: Value,
    /// The frontend entry treatment for library input.
    pub entry_normalization: String,
}

fn failure(reason: &str) -> String {
    format!("Rust supplied module CST: {reason}")
}

fn source_identifier(path: &str) -> String {
    format!("Rust supplied source {path}")
}

fn options(
    user: &Value,
    capturing: bool,
) -> Result<(Value, StructuredDataLinksLimits, usize), String> {
    let mut options = if capturing {
        user.get("parseOptions")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default()
    } else {
        serde_json::Map::new()
    };
    for (name, default) in [
        ("fileLimit", 1024),
        ("moduleLimit", 4096),
        ("depthLimit", 64),
        ("byteLimit", 16_777_216),
        ("nodeLimit", 1_000_000),
        ("referenceLimit", 4_000_000),
    ] {
        options.insert(name.to_owned(), json!(limit(user, name, default)?));
    }
    options.insert("expansionLimit".to_owned(), options["moduleLimit"].clone());
    options.insert("outputUnitLimit".to_owned(), options["byteLimit"].clone());
    let options = Value::Object(options);
    let limits = StructuredDataLinksLimits {
        node_limit: limit(&options, "nodeLimit", 1_000_000)?,
        byte_limit: limit(&options, "byteLimit", 16_777_216)?,
        ..StructuredDataLinksLimits::default()
    };
    let reference_limit = limit(&options, "referenceLimit", 4_000_000)?;
    Ok((options, limits, reference_limit))
}

struct SourceFacts {
    syntax: Vec<ProgramConcreteSyntaxFact>,
    by_id: BTreeMap<LinkId, usize>,
    fields: BTreeMap<(LinkId, String), Vec<LinkId>>,
    tokens: Vec<Token>,
}

impl SourceFacts {
    fn new(original: &RustSourceProjectConcreteGraph) -> Result<Self, String> {
        let syntax = read_program_concrete_syntax_facts(&original.network, &original.source)
            .map_err(|error| failure(&error.to_string()))?;
        let by_id = syntax
            .iter()
            .enumerate()
            .map(|(index, row)| (row.link_id, index))
            .collect();
        let mut fields: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for (parent, label, child) in concrete_fields(&original.network) {
            fields
                .entry((parent, label.to_owned()))
                .or_default()
                .push(child);
        }
        let tokens = tokenize(&original.source, Language::Rust)
            .map_err(|error| failure(&error.to_string()))?
            .tokens;
        Ok(Self {
            syntax,
            by_id,
            fields,
            tokens,
        })
    }

    fn field(
        &self,
        parent: &ProgramConcreteSyntaxFact,
        label: &str,
    ) -> Result<Option<&ProgramConcreteSyntaxFact>, String> {
        match self
            .fields
            .get(&(parent.link_id, label.to_owned()))
            .map_or(&[][..], Vec::as_slice)
        {
            [] => Ok(None),
            [id] => Ok(self.by_id.get(id).map(|index| &self.syntax[*index])),
            _ => Err(failure("a module field must have one actual CST owner")),
        }
    }

    fn identifier(&self, fact: &ProgramConcreteSyntaxFact) -> Option<&Token> {
        self.tokens.iter().find(|token| {
            token.kind == TokenKind::Identifier
                && token.start == fact.start
                && token.end == fact.end
        })
    }

    fn has_main(&self) -> Result<bool, String> {
        for function in self
            .syntax
            .iter()
            .filter(|node| node.term.as_deref() == Some("function_item"))
        {
            if self
                .field(function, "name")?
                .and_then(|name| self.identifier(name))
                .is_some_and(|token| token.value == "main")
                && !self.syntax.iter().any(|parent| {
                    parent
                        .term
                        .as_deref()
                        .is_some_and(test_rust_concrete_entry_owner_kind)
                        && parent.start < function.start
                        && parent.end >= function.end
                })
            {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

fn module_witnesses(
    composition: &Value,
    original_facts: &BTreeMap<&str, SourceFacts>,
) -> Result<Vec<Value>, String> {
    let mut modules = Vec::new();
    let mut namespaces = BTreeSet::new();
    for actual in composition["modules"]
        .as_array()
        .expect("composition has modules")
    {
        let path = actual["path"].as_str().expect("composed module has a path");
        let provider = actual["provider"]
            .as_str()
            .expect("composed module has a provider");
        let source = original_facts
            .get(path)
            .ok_or_else(|| failure("composition refers to a foreign concrete source"))?;
        let module_start = actual["startUnit"].as_f64().expect("module start");
        let module_end = actual["endUnit"].as_f64().expect("module end");
        let mut candidates = source.syntax.iter().filter(|node| {
            test_rust_concrete_module_declaration(
                node.term.as_deref().unwrap_or_default(),
                source_unit(node.start),
                source_unit(node.end),
                module_start,
                module_end,
            )
        });
        let unique = || failure("composition has no unique actual module CST declaration");
        let declaration = candidates.next().ok_or_else(unique)?;
        if candidates.next().is_some() {
            return Err(unique());
        }
        let invalid_name =
            || failure("composed namespace differs from the actual module name field");
        let name = source
            .field(declaration, "name")?
            .ok_or_else(invalid_name)?;
        let token = source.identifier(name).ok_or_else(invalid_name)?;
        let module = actual["module"].as_str().expect("module namespace");
        let namespace = module.split("::").collect::<Vec<_>>();
        if namespace.first() != Some(&"crate")
            || namespace.last().copied() != Some(token.value.as_str())
        {
            return Err(invalid_name());
        }
        if !namespaces.insert(module) {
            return Err(failure(&format!(
                "duplicate actual module namespace: {module}"
            )));
        }
        let inline = source.field(declaration, "body")?.is_some();
        if inline != (path == provider) {
            return Err(failure(
                "module provider disagrees with its actual inline body field",
            ));
        }
        let mut witness = json!({"namespace":namespace,"parent":namespace[..namespace.len()-1],
            "name":token.value,"source":source_identifier(path),"sourceLink":declaration.link_id.as_u64(),
            "declaration":{"start":declaration.start,"end":declaration.end},
            "nameSpan":{"start":name.start,"end":name.end},"kind":if inline {"inline"} else {"external"}});
        if !inline {
            witness["target"] = json!(source_identifier(provider));
        }
        modules.push(witness);
    }
    Ok(modules)
}

fn module_graph(
    composition: &Value,
    crate_root: &str,
    library: bool,
    normalization: &str,
    original_facts: &BTreeMap<&str, SourceFacts>,
) -> Result<Value, String> {
    let source = composition["source"]
        .as_str()
        .expect("composition has source");
    let mappings = composition["mappings"]
        .as_array()
        .expect("composition has mappings");
    let wanted = mappings
        .iter()
        .flat_map(|row| {
            ["outputStart", "outputEnd"]
                .map(|field| row[field].as_u64().expect("output byte coordinate") as usize)
        })
        .collect::<BTreeSet<_>>();
    let mut offsets = BTreeMap::new();
    let mut units = 0;
    for (byte, character) in source.char_indices() {
        if wanted.contains(&byte) {
            offsets.insert(byte, units);
        }
        units += character.len_utf16();
    }
    offsets.insert(source.len(), units);
    if wanted.iter().any(|byte| !offsets.contains_key(byte)) {
        return Err(failure(
            "composition output coordinates split an actual UTF8 scalar",
        ));
    }
    let segments = mappings.iter().map(|row| {
        let start = row["outputStart"].as_u64().expect("output start") as usize;
        let end = row["outputEnd"].as_u64().expect("output end") as usize;
        let path = row["path"].as_str().expect("mapping path");
        let role = row["kind"].as_str().expect("mapping kind");
        json!({"generatedStart":offsets[&start],"generatedEnd":offsets[&end],
            "generatedByteStart":start,"generatedByteEnd":end,"source":source_identifier(path),
            "start":row["startUnit"],"end":row["endUnit"],"role":if role=="source" {"original"} else {role}})
    }).collect::<Vec<_>>();
    let unused = composition["unusedSources"]
        .as_array()
        .expect("unused source list");
    let files = composition["files"]
        .as_array()
        .expect("composition has files")
        .iter()
        .map(|file| {
            let path = file["path"].as_str().expect("file path");
            json!({"path":path,"identifier":source_identifier(path),"bytes":file["sourceBytes"],
            "sha256":file["sourceSha256"],"selected":!unused.contains(&file["path"])})
        })
        .collect::<Vec<_>>();
    Ok(
        json!({"model":MODEL,"crateRoot":crate_root,"library":library,"entryNormalization":normalization,
        "files":files,"modules":module_witnesses(composition, original_facts)?,"segments":segments}),
    )
}

fn unique_field(network: &LinkNetwork, root: LinkId, label: &str) -> Result<LinkId, String> {
    let mut fields = concrete_fields(network)
        .filter_map(|(parent, name, child)| (parent == root && name == label).then_some(child));
    let error = || failure(&format!("{label} must retain its actual unique root"));
    let field = fields.next().ok_or_else(error)?;
    if fields.next().is_some() {
        return Err(error());
    }
    Ok(field)
}

fn facts(
    originals: &[RustSourceProjectConcreteGraph],
) -> Result<BTreeMap<&str, SourceFacts>, String> {
    originals
        .iter()
        .map(|original| Ok((original.path.as_str(), SourceFacts::new(original)?)))
        .collect()
}

/// Capture a checked supplied-file program and retain its actual module declarations and ranges.
///
/// # Errors
/// Refuses invalid sources or bounds, ambiguous module owners, or missing archive links.
pub fn capture_rust_supplied_module_project(
    inputs: &Value,
    user: &Value,
) -> Result<RustSuppliedModuleProject, String> {
    let crate_root = user["crateRoot"]
        .as_str()
        .ok_or_else(|| failure("unknown source entry"))?;
    let library = match user.get("library") {
        None => false,
        Some(value) => value
            .as_bool()
            .ok_or_else(|| failure("library mode must be Boolean"))?,
    };
    let (mut options, limits, reference_limit) = options(user, true)?;
    options["library"] = json!(library);
    let (mut packet, composition, entry_normalization) =
        create_rust_source_project_packet(inputs, crate_root, &options)?;
    let originals =
        read_rust_source_project_concrete_graphs(&packet.network, packet.root, &options)?;
    let module_graph = module_graph(
        &composition,
        crate_root,
        library,
        &entry_normalization,
        &facts(&originals)?,
    )?;
    let encoded = encode_structured_data_links_with_limits(&module_graph, limits)?;
    let links = encoded.network.links().collect::<Vec<_>>();
    if packet
        .network
        .links()
        .count()
        .saturating_add(links.len())
        .saturating_add(2)
        > limits.node_limit
    {
        return Err(failure("module ownership graph exceeds its node budget"));
    }
    let references = packet
        .network
        .links()
        .chain(encoded.network.links())
        .fold(3usize, |sum, link| {
            sum.saturating_add(link.references().len())
        });
    if references > reference_limit {
        return Err(failure(
            "module ownership graph exceeds its reference budget",
        ));
    }
    let ids = links
        .iter()
        .map(|link| {
            (
                link.id(),
                packet.network.insert_link([], link.metadata().clone()),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for link in links {
        let references = link
            .references()
            .iter()
            .map(|id| {
                ids.get(id)
                    .copied()
                    .ok_or_else(|| failure("dangling module ownership reference"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        packet.network.set_references(ids[&link.id()], &references);
    }
    packet
        .network
        .insert_field(packet.root, DIRECTORY, ids[&encoded.root]);
    let archive_root = unique_field(
        &packet.network,
        packet.root,
        "source project concrete archive",
    )?;
    Ok(RustSuppliedModuleProject {
        packet,
        archive_root,
        module_graph,
        composition,
        entry_normalization,
    })
}

/// Read and revalidate a module directory directly from the archived concrete sources.
///
/// # Errors
/// Refuses detached or modified owners, archives, ranges or source digests.
pub fn read_rust_supplied_module_ownership(
    network: &LinkNetwork,
    root: LinkId,
    user: &Value,
) -> Result<Value, String> {
    let (options, limits, _) = options(user, false)?;
    let directory = unique_field(network, root, DIRECTORY)?;
    let graph = decode_structured_data_links_with_limits(network, directory, limits)?;
    if graph["model"] != MODEL {
        return Err(failure("unknown module ownership model or source entry"));
    }
    let crate_root = graph["crateRoot"]
        .as_str()
        .ok_or_else(|| failure("unknown source entry"))?;
    let library = graph["library"]
        .as_bool()
        .ok_or_else(|| failure("library mode must be Boolean"))?;
    let originals = read_rust_source_project_concrete_graphs(network, root, &options)?;
    let facts = facts(&originals)?;
    let inputs = originals
        .iter()
        .map(|original| json!({"path":original.path,"source":original.source}))
        .collect::<Vec<_>>();
    let composition = compose_rust_source_project(&json!(inputs), crate_root, &options)?;
    let original = facts
        .get(crate_root)
        .ok_or_else(|| failure("missing original crate root"))?;
    let normalization = if library && !original.has_main()? {
        "empty frontend entry for a source library"
    } else {
        "none"
    };
    if module_graph(&composition, crate_root, library, normalization, &facts)? != graph {
        return Err(failure(
            "module ownership directory differs from its actual current concrete sources",
        ));
    }
    Ok(graph)
}
