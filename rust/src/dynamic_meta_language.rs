//! Immutable linked snapshots with persistent dynamic language extension.
//! Compilation and proof decisions delegate to the existing shared algorithms.
use crate::dynamic_meta_language_rules::{
    accept_dynamic_syntax_projection, read_dynamic_concrete_syntax_version,
    read_dynamic_meta_language_identifier,
};
use crate::grammar::feature_runtime::{
    FeatureParseOptions, ParseOutcome, SyntaxTree, check_grammar_selection_mapped_tree,
};
use crate::{
    FormalConceptArchive, GlobalConceptOptions, GlobalConceptSource, GlobalConceptSpaceParser,
    MetaLanguageArchive, MetaLanguageArchiveEntry, MetaLanguageArchiveLimits, StructuredDataLinks,
    StructuredDataLinksLimits, create_discovered_concept_links, create_meta_language_archive,
    encode_structured_data_links_with_limits, extend_meta_language_archive,
    read_discovered_concept_links, read_meta_language_archive,
};
use serde_json::{Value, json};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;

mod contexts;
mod extension;
mod public_tree;
mod settings;

/// Creation and persistent extension settings; absent budgets inherit defaults.
#[derive(Clone, Debug, Default)]
pub struct DynamicMetaLanguageOptions {
    /// Complete semantic records, formal premises and required correspondences.
    pub concepts: GlobalConceptOptions,
    /// Additional complete linked entry points.
    pub entries: Vec<MetaLanguageArchiveEntry>,
    /// Explicit archive budgets, inherited during extension when absent.
    pub limits: Option<MetaLanguageArchiveLimits>,
    /// Explicit structured-value depth, inherited when absent.
    pub depth_limit: Option<usize>,
    /// Permit explicitly selected source-grammar replacements.
    pub replace: bool,
}

/// Parse options retain concrete syntax unless abstraction is explicitly chosen.
#[derive(Clone, Debug)]
pub struct DynamicParseOptions {
    /// Ordinary native parser options with protected archive context.
    pub parser: FeatureParseOptions,
    /// Concrete syntax or an explicitly supplied abstract projection.
    pub projection: String,
    /// Optional stable name for the saved complete concrete tree.
    pub identifier: Option<String>,
    /// Explicitly replace an existing concrete snapshot with that name.
    pub replace_snapshot: bool,
}
impl Default for DynamicParseOptions {
    fn default() -> Self {
        Self {
            parser: FeatureParseOptions::default(),
            projection: "concrete syntax".to_owned(),
            identifier: None,
            replace_snapshot: false,
        }
    }
}

/// Complete grammar result and its optional saved concrete and abstract views.
#[derive(Clone, Debug)]
pub struct DynamicParseResult {
    /// Actual grammar acceptance, full concrete tree and all diagnostics.
    pub outcome: ParseOutcome,
    /// Explicitly selected view.
    pub projection: String,
    /// Complete linked archive, when a concrete tree was built.
    pub archive: Option<MetaLanguageArchive>,
    /// Actual retained concrete syntax data graph.
    pub concrete: Option<StructuredDataLinks>,
    /// Optional caller projection; the original concrete tree stays in outcome.
    pub abstract_tree: Option<Value>,
}

/// Derived parsers read a private immutable linked snapshot. Exported graphs
/// remain editable through a new session or persistent extension.
#[derive(Clone, Debug)]
pub struct DynamicMetaLanguage {
    archive: MetaLanguageArchive,
    entries: Vec<MetaLanguageArchiveEntry>,
    sources: Vec<GlobalConceptSource>,
    space: Value,
    formal_definitions: BTreeMap<String, FormalConceptArchive>,
    configuration: Value,
    limits: MetaLanguageArchiveLimits,
    depth_limit: usize,
    definition_context: String,
    configuration_context: String,
    definition_fingerprint: String,
    configuration_fingerprint: String,
    default_parsers: RefCell<BTreeMap<String, Rc<GlobalConceptSpaceParser>>>,
}
pub(super) fn fail(message: &str) -> String {
    format!("dynamic meta-language: {message}")
}

fn data_limits(limits: MetaLanguageArchiveLimits, depth_limit: usize) -> StructuredDataLinksLimits {
    StructuredDataLinksLimits {
        node_limit: limits.node_limit,
        byte_limit: limits.byte_limit,
        depth_limit,
    }
}

/// Prepare actual discovered grammar/definition graphs and their configuration.
///
/// # Errors
/// Refuses malformed sources, false correspondences and resource violations.
pub fn create_dynamic_meta_language(
    sources: &[GlobalConceptSource],
    options: &DynamicMetaLanguageOptions,
) -> Result<DynamicMetaLanguage, String> {
    let limits = options.limits.unwrap_or_default();
    let depth = options.depth_limit.unwrap_or(256);
    let discovered = create_discovered_concept_links(sources, &options.concepts)?;
    let configuration = settings::configuration(&options.concepts)?;
    let encoded =
        encode_structured_data_links_with_limits(&configuration, data_limits(limits, depth))?;
    let mut entries = vec![
        MetaLanguageArchiveEntry {
            identifier: read_dynamic_meta_language_identifier("definitions"),
            kind: "discovered concepts".to_owned(),
            network: Arc::new(discovered.network),
            root: discovered.root,
        },
        MetaLanguageArchiveEntry {
            identifier: read_dynamic_meta_language_identifier("configuration"),
            kind: "language configuration".to_owned(),
            network: Arc::new(encoded.network),
            root: encoded.root,
        },
    ];
    entries.extend(options.entries.clone());
    let definition = entries[0].clone();
    let configuration = entries[1].clone();
    contexts::pin(&mut entries, &definition, "definition snapshot");
    contexts::pin(&mut entries, &configuration, "configuration snapshot");
    let archive = create_meta_language_archive(&entries, limits)?;
    DynamicMetaLanguage::new(&archive, options)
}

impl DynamicMetaLanguage {
    /// Eagerly reload complete linked language definitions and formal premises.
    ///
    /// # Errors
    /// Refuses malformed roots, invalid contexts and false archived proofs.
    pub fn new(
        archive: &MetaLanguageArchive,
        options: &DynamicMetaLanguageOptions,
    ) -> Result<Self, String> {
        let limits = options.limits.unwrap_or_default();
        let depth_limit = options.depth_limit.unwrap_or(256);
        if depth_limit == 0 || depth_limit as u128 > 9_007_199_254_740_991 {
            return Err(fail("invalid resource limit"));
        }
        let mut entries =
            read_meta_language_archive(&archive.network, archive.root, limits)?.entries;
        let definition = entries
            .iter()
            .find(|entry| entry.identifier == read_dynamic_meta_language_identifier("definitions"))
            .cloned()
            .ok_or_else(|| fail("missing language definitions or configuration"))?;
        let settings = entries
            .iter()
            .find(|entry| {
                entry.identifier == read_dynamic_meta_language_identifier("configuration")
            })
            .cloned()
            .ok_or_else(|| fail("missing language definitions or configuration"))?;
        if definition.kind != "discovered concepts"
            || !["language configuration", "decorators"].contains(&settings.kind.as_str())
        {
            return Err(fail("missing language definitions or configuration"));
        }
        let configuration = contexts::data(&settings, limits, depth_limit)?;
        if configuration["schemaVersion"].as_f64() != Some(1.0)
            || !configuration["records"].is_array()
            || !configuration["equivalences"].is_array()
        {
            return Err(fail("invalid language configuration"));
        }
        let definition_network = crate::meta_language_archive::select_network(
            &definition.network,
            definition.root,
            limits.node_limit,
        )?;
        let discovered = read_discovered_concept_links(&definition_network, definition.root)?;
        let count = entries.len();
        let definition_context = contexts::pin(&mut entries, &definition, "definition snapshot");
        let configuration_context =
            contexts::pin(&mut entries, &settings, "configuration snapshot");
        let snapshot = if entries.len() == count {
            archive.clone()
        } else {
            create_meta_language_archive(&entries, limits)?
        };
        let definition_fingerprint = contexts::fingerprint(&definition, limits)?;
        let configuration_fingerprint = contexts::fingerprint(&settings, limits)?;
        contexts::check(&entries, limits, depth_limit)?;
        Ok(Self {
            archive: snapshot,
            entries,
            sources: discovered.sources,
            space: discovered.space,
            formal_definitions: discovered.formal_definitions,
            configuration,
            limits,
            depth_limit,
            definition_context,
            configuration_context,
            definition_fingerprint,
            configuration_fingerprint,
            default_parsers: RefCell::new(BTreeMap::new()),
        })
    }

    /// Read the complete authoritative archive; clone it to make edits.
    #[must_use]
    pub const fn archive(&self) -> &MetaLanguageArchive {
        &self.archive
    }
    /// Read every linked root selection.
    #[must_use]
    pub fn entries(&self) -> &[MetaLanguageArchiveEntry] {
        &self.entries
    }
    /// Read complete source-qualified native grammars.
    #[must_use]
    pub fn sources(&self) -> &[GlobalConceptSource] {
        &self.sources
    }
    /// Read the validated common concept space.
    #[must_use]
    pub const fn space(&self) -> &Value {
        &self.space
    }
    /// Read all formal alternatives, including unresolved interpretations.
    #[must_use]
    pub const fn formal_definitions(&self) -> &BTreeMap<String, FormalConceptArchive> {
        &self.formal_definitions
    }
    /// Read stored record/correspondence configuration.
    #[must_use]
    pub const fn configuration(&self) -> &Value {
        &self.configuration
    }

    /// Compile an archived grammar with resolution from this same snapshot.
    ///
    /// # Errors
    /// Refuses unknown source identities, protected overrides or invalid rules.
    pub fn parser(
        &self,
        source_identifier: &str,
        options: FeatureParseOptions,
    ) -> Result<Rc<GlobalConceptSpaceParser>, String> {
        let defaults = options == FeatureParseOptions::default();
        if defaults {
            if let Some(parser) = self.default_parsers.borrow().get(source_identifier) {
                return Ok(Rc::clone(parser));
            }
        }
        let parser = Rc::new(
            GlobalConceptSpaceParser::from_validated_sources(
                &self.sources,
                self.space.clone(),
                source_identifier,
                options,
            )
            .map_err(|error| error.to_string())?,
        );
        if defaults {
            self.default_parsers
                .borrow_mut()
                .insert(source_identifier.to_owned(), Rc::clone(&parser));
        }
        Ok(parser)
    }

    /// Parse concrete syntax by default and retain its complete linked product.
    /// An explicit abstraction receives an immutable concrete tree.
    ///
    /// # Errors
    /// Refuses invalid projections, archived contexts or resource violations.
    pub fn parse(
        &self,
        source_identifier: &str,
        input: &[u8],
        options: &DynamicParseOptions,
        project_abstract_syntax: Option<&dyn Fn(&SyntaxTree, &Value) -> Result<Value, String>>,
    ) -> Result<DynamicParseResult, String> {
        if !accept_dynamic_syntax_projection(&options.projection) {
            return Err(fail("unknown syntax projection"));
        }
        if options.projection == "abstract syntax" && project_abstract_syntax.is_none() {
            return Err(fail("abstract syntax needs an explicit projection"));
        }
        let outcome = self
            .parser(source_identifier, options.parser.clone())?
            .parse_tree(input, &FeatureParseOptions::default())
            .map_err(|error| error.to_string())?;
        let Some(tree) = outcome.tree.as_ref() else {
            return Ok(DynamicParseResult {
                outcome,
                projection: options.projection.clone(),
                archive: None,
                concrete: None,
                abstract_tree: None,
            });
        };
        let saved = public_tree::outcome(&outcome, self.depth_limit)?;
        if outcome.source_planes.is_some() {
            check_grammar_selection_mapped_tree(&saved, &options.parser)
                .map_err(|error| error.to_string())?;
        }
        let value = json!({"schemaVersion":read_dynamic_concrete_syntax_version(),"sourceIdentifier":source_identifier,
            "projection":"concrete syntax","definitionContext":self.definition_context,"configurationContext":self.configuration_context,
            "definitionContextFingerprint":self.definition_fingerprint,"configurationContextFingerprint":self.configuration_fingerprint,
            "parserOptions":settings::parser(&options.parser),"outcome":saved});
        let concrete = encode_structured_data_links_with_limits(
            &value,
            data_limits(self.limits, self.depth_limit),
        )?;
        let identifier = options.identifier.clone().unwrap_or_else(|| {
            crate::dynamic_meta_language_rules::read_dynamic_concrete_identifier(
                &self
                    .entries
                    .iter()
                    .map(|entry| entry.identifier.clone())
                    .collect::<Vec<_>>(),
                source_identifier,
            )
        });
        if self
            .entries
            .iter()
            .any(|entry| entry.identifier == identifier && entry.kind != "concrete syntax")
        {
            return Err(fail(
                "concrete syntax identifier collides with another entry point",
            ));
        }
        let entry = MetaLanguageArchiveEntry {
            identifier,
            kind: "concrete syntax".to_owned(),
            network: Arc::new(concrete.network.clone()),
            root: concrete.root,
        };
        let archive = extend_meta_language_archive(
            &self.archive,
            &[entry],
            options.replace_snapshot,
            &[],
            self.limits,
        )?;
        let abstract_tree = if options.projection == "abstract syntax" {
            Some(project_abstract_syntax.ok_or_else(|| {
                fail("abstract syntax needs an explicit projection")
            })?(tree, &self.space)?)
        } else {
            None
        };
        Ok(DynamicParseResult {
            outcome,
            projection: options.projection.clone(),
            archive: Some(archive),
            concrete: Some(concrete),
            abstract_tree,
        })
    }
}
