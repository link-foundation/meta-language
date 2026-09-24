//! Editable program representations for JavaScript, Rust, Lean, and Rocq.

mod analysis;
mod edit;
mod module_resolution;
mod snapshot;

pub use snapshot::{construct_program_from_fragments, PROGRAM_SNAPSHOT_SCHEMA_VERSION};

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::{language_support, LinkNetwork, LinkType, ParseConfiguration};
use analysis::{
    effect_markers, extension_markers, module_markers, proof_markers, ranges_overlap,
    resolve_bindings, scope_by_id, semantic_tokens, syntax_facts, unique_facts,
    validate_identifier, SemanticToken,
};
use module_resolution::{module_requests, project_has_module};

/// Schema revision for four-language program representations.
pub const PROGRAM_REPRESENTATION_SCHEMA_VERSION: u32 = 1;

/// Semantic construct categories audited for every four-language frontend.
pub const SEMANTIC_CONSTRUCTS: [&str; 10] = [
    "modules-and-imports",
    "scopes-and-bindings",
    "recursive-definitions",
    "types-and-universes",
    "effects",
    "attributes",
    "macros-and-notation",
    "proof-terms-and-tactics",
    "surface-expansion-elaboration-traces",
    "project-context-and-dependencies",
];

/// Source range in UTF-8 byte offsets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProgramRange {
    start: usize,
    end: usize,
}

impl ProgramRange {
    /// Creates an exact UTF-8 byte range.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Inclusive start byte.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }

    /// Exclusive end byte.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }
}

/// Project files and dependency names available during analysis.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProgramProjectContext {
    root: String,
    files: Vec<String>,
    dependencies: Vec<String>,
    extensions: Vec<String>,
}

impl ProgramProjectContext {
    /// Creates a project context.
    #[must_use]
    pub fn new(root: impl Into<String>, files: Vec<String>, dependencies: Vec<String>) -> Self {
        Self {
            root: root.into(),
            files,
            dependencies,
            extensions: Vec::new(),
        }
    }

    /// Adds project-defined syntax extensions.
    #[must_use]
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }

    /// Project root.
    #[must_use]
    pub fn root(&self) -> &str {
        &self.root
    }

    /// Project files.
    #[must_use]
    pub fn files(&self) -> &[String] {
        &self.files
    }

    /// Declared dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }

    /// Project-defined syntax extensions.
    #[must_use]
    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }
}

/// One lexical scope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramScope {
    id: String,
    parent: Option<String>,
    range: ProgramRange,
    depth: usize,
}

impl ProgramScope {
    /// Stable scope identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Parent scope identifier.
    #[must_use]
    pub fn parent(&self) -> Option<&str> {
        self.parent.as_deref()
    }

    /// Source range governed by the scope.
    #[must_use]
    pub const fn range(&self) -> ProgramRange {
        self.range
    }

    /// Lexical nesting depth.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }
}

/// A resolved declaration and its exact references.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramBinding {
    id: String,
    name: String,
    kind: String,
    scope: String,
    declaration: ProgramRange,
    references: Vec<ProgramRange>,
}

impl ProgramBinding {
    /// Stable binding identifier derived from language and declaration byte.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Declared spelling.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Declaration kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Governing lexical scope.
    #[must_use]
    pub fn scope(&self) -> &str {
        &self.scope
    }

    /// Declaration source range.
    #[must_use]
    pub const fn declaration(&self) -> ProgramRange {
        self.declaration
    }

    /// References resolving to this declaration.
    #[must_use]
    pub fn references(&self) -> &[ProgramRange] {
        &self.references
    }
}

/// Generic semantic fact with source evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramFact {
    kind: String,
    name: String,
    range: ProgramRange,
    phase: Option<String>,
}

impl ProgramFact {
    fn new(kind: impl Into<String>, name: impl Into<String>, range: ProgramRange) -> Self {
        Self {
            kind: kind.into(),
            name: name.into(),
            range,
            phase: None,
        }
    }

    fn with_phase(mut self, phase: &str) -> Self {
        self.phase = Some(phase.to_string());
        self
    }

    /// Fact kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Fact name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Source range.
    #[must_use]
    pub const fn range(&self) -> ProgramRange {
        self.range
    }

    /// Representation phase, when applicable.
    #[must_use]
    pub fn phase(&self) -> Option<&str> {
        self.phase.as_deref()
    }
}

/// CST node to semantic fact source mapping.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramSourceMapping {
    link_id: u64,
    term: String,
    range: ProgramRange,
}

impl ProgramSourceMapping {
    /// Source CST link id.
    #[must_use]
    pub const fn link_id(&self) -> u64 {
        self.link_id
    }

    /// Grammar term.
    #[must_use]
    pub fn term(&self) -> &str {
        &self.term
    }

    /// Mapped source range.
    #[must_use]
    pub const fn range(&self) -> ProgramRange {
        self.range
    }
}

/// Parse diagnostic retained in the representation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramDiagnostic {
    kind: &'static str,
    term: String,
    range: ProgramRange,
}

impl ProgramDiagnostic {
    /// Diagnostic kind.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        self.kind
    }

    /// Grammar term carrying the diagnostic.
    #[must_use]
    pub fn term(&self) -> &str {
        &self.term
    }

    /// Diagnostic source range.
    #[must_use]
    pub const fn range(&self) -> ProgramRange {
        self.range
    }
}

/// Representation state for one required semantic construct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgramConstructStatus {
    /// The source contains the construct and it has structured evidence.
    Represented,
    /// The source contains no instance of the construct.
    NotPresent,
    /// The language does not define this construct class.
    NotApplicable,
    /// The runtime has not produced the required semantic structure.
    Unavailable,
}

/// One semantic construct coverage record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramConstruct {
    kind: &'static str,
    status: ProgramConstructStatus,
    evidence: Vec<ProgramFact>,
    rationale: Option<String>,
}

impl ProgramConstruct {
    /// Construct category.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        self.kind
    }

    /// Representation state.
    #[must_use]
    pub const fn status(&self) -> ProgramConstructStatus {
        self.status
    }

    /// Source-backed evidence.
    #[must_use]
    pub fn evidence(&self) -> &[ProgramFact] {
        &self.evidence
    }

    /// Rationale for a non-represented state.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }
}

/// Semantic analysis or binding-safe edit failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgramRepresentationError {
    /// No four-language frontend is registered.
    UnsupportedLanguage(String),
    /// Binding id is absent.
    UnknownBinding(String),
    /// Replacement is not a valid identifier.
    InvalidIdentifier {
        language: String,
        identifier: String,
    },
    /// Rename would capture another binding.
    CaptureConflict { identifier: String, offset: usize },
    /// A range is reversed, out of bounds, or splits a UTF-8 code point.
    InvalidRange { start: usize, end: usize },
    /// A move destination falls strictly inside the moved range.
    DestinationInsideRange,
    /// The edited result does not parse cleanly.
    InvalidEdit,
    /// A serialized program snapshot is malformed or inconsistent.
    InvalidSnapshot(String),
}

impl fmt::Display for ProgramRepresentationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedLanguage(language) => {
                write!(
                    formatter,
                    "program semantics are not registered for {language}"
                )
            }
            Self::UnknownBinding(binding) => write!(formatter, "unknown binding {binding}"),
            Self::InvalidIdentifier {
                language,
                identifier,
            } => write!(
                formatter,
                "{identifier:?} is not a valid {language} identifier"
            ),
            Self::CaptureConflict { identifier, offset } => write!(
                formatter,
                "rename would capture {identifier} at {offset} (binding conflict)"
            ),
            Self::InvalidRange { start, end } => {
                write!(formatter, "invalid UTF-8 source range {start}..{end}")
            }
            Self::DestinationInsideRange => {
                formatter.write_str("move destination is inside the moved range")
            }
            Self::InvalidEdit => formatter.write_str("structured edit does not reparse cleanly"),
            Self::InvalidSnapshot(reason) => {
                write!(formatter, "invalid program snapshot: {reason}")
            }
        }
    }
}

impl Error for ProgramRepresentationError {}

/// CST-backed resolved program model.
#[derive(Clone, Debug)]
pub struct ProgramRepresentation {
    schema_version: u32,
    language: &'static str,
    source: String,
    project: ProgramProjectContext,
    network: LinkNetwork,
    scopes: Vec<ProgramScope>,
    bindings: Vec<ProgramBinding>,
    unresolved_references: Vec<ProgramFact>,
    source_mappings: Vec<ProgramSourceMapping>,
    modules: Vec<ProgramFact>,
    types: Vec<ProgramFact>,
    extensions: Vec<ProgramFact>,
    proofs: Vec<ProgramFact>,
    diagnostics: Vec<ProgramDiagnostic>,
    constructs: Vec<ProgramConstruct>,
}

impl ProgramRepresentation {
    fn analyze(
        source: &str,
        language: &str,
        project: ProgramProjectContext,
    ) -> Result<Self, ProgramRepresentationError> {
        let support = language_support(language)
            .ok_or_else(|| ProgramRepresentationError::UnsupportedLanguage(language.to_string()))?;
        let network = LinkNetwork::parse(source, support.name, ParseConfiguration::default());
        let source_mappings = syntax_facts(&network);
        let tokens = semantic_tokens(source, support.name, &source_mappings);
        let (scopes, bindings, unresolved_references) =
            resolve_bindings(&tokens, source.len(), support.name);
        let modules = module_facts(&tokens, &source_mappings, source, support.name, &project);
        let types = type_facts(&tokens, &source_mappings, support.name);
        let extensions = extension_facts(&tokens, &source_mappings, source, support.name);
        let proofs = proof_facts(&tokens, &source_mappings, support.name);
        let mut diagnostics = diagnostic_facts(&network);
        diagnostics.extend(project_diagnostics(&modules, &project));
        let mut program = Self {
            schema_version: PROGRAM_REPRESENTATION_SCHEMA_VERSION,
            language: support.name,
            source: source.to_string(),
            project,
            network,
            scopes,
            bindings,
            unresolved_references,
            source_mappings,
            modules,
            types,
            extensions,
            proofs,
            diagnostics,
            constructs: Vec::new(),
        };
        program.constructs = construct_facts(&program);
        Ok(program)
    }

    /// Schema revision.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Canonical language name.
    #[must_use]
    pub const fn language(&self) -> &'static str {
        self.language
    }

    /// Exact source text.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Project context used during analysis.
    #[must_use]
    pub const fn project(&self) -> &ProgramProjectContext {
        &self.project
    }

    /// Concrete syntax network enriched by this representation.
    #[must_use]
    pub const fn network(&self) -> &LinkNetwork {
        &self.network
    }

    /// Lexical scopes inferred from parsed source.
    #[must_use]
    pub fn scopes(&self) -> &[ProgramScope] {
        &self.scopes
    }

    /// Local bindings inferred from parsed source.
    #[must_use]
    pub fn bindings(&self) -> &[ProgramBinding] {
        &self.bindings
    }

    /// Identifier occurrences without a local declaration.
    #[must_use]
    pub fn unresolved_references(&self) -> &[ProgramFact] {
        &self.unresolved_references
    }

    /// CST source mappings.
    #[must_use]
    pub fn source_mappings(&self) -> &[ProgramSourceMapping] {
        &self.source_mappings
    }

    /// Module and import facts.
    #[must_use]
    pub fn modules(&self) -> &[ProgramFact] {
        &self.modules
    }

    /// Type and universe facts.
    #[must_use]
    pub fn types(&self) -> &[ProgramFact] {
        &self.types
    }

    /// Attribute, macro, notation, and syntax-extension facts.
    #[must_use]
    pub fn extensions(&self) -> &[ProgramFact] {
        &self.extensions
    }

    /// Proof and tactic facts.
    #[must_use]
    pub fn proofs(&self) -> &[ProgramFact] {
        &self.proofs
    }

    /// Parse diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[ProgramDiagnostic] {
        &self.diagnostics
    }

    /// Semantic construct coverage.
    #[must_use]
    pub fn constructs(&self) -> &[ProgramConstruct] {
        &self.constructs
    }

    /// Emits exact source from the retained token network.
    #[must_use]
    pub fn emit(&self) -> String {
        self.network.reconstruct_text()
    }
}

/// Analyzes a four-language program with project context.
pub fn analyze_program(
    source: &str,
    language: &str,
    project: ProgramProjectContext,
) -> Result<ProgramRepresentation, ProgramRepresentationError> {
    ProgramRepresentation::analyze(source, language, project)
}

/// Constructs a program and rejects syntax that cannot be represented cleanly.
pub fn construct_program(
    source: &str,
    language: &str,
    project: ProgramProjectContext,
) -> Result<ProgramRepresentation, ProgramRepresentationError> {
    let program = ProgramRepresentation::analyze(source, language, project)?;
    if !program.network.verify_full_match(None).is_clean() {
        return Err(ProgramRepresentationError::InvalidEdit);
    }
    Ok(program)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TokenKind {
    Identifier,
    Keyword,
    Punctuation,
}

fn module_facts(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    source: &str,
    language: &str,
    project: &ProgramProjectContext,
) -> Vec<ProgramFact> {
    let markers = module_markers(language);
    let mut facts = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        if !markers.contains(&token.text.as_str()) {
            continue;
        }
        let names = tokens[index + 1..]
            .iter()
            .take_while(|candidate| {
                candidate.text != ";" && !markers.contains(&candidate.text.as_str())
            })
            .filter(|candidate| candidate.kind == TokenKind::Identifier)
            .take(3)
            .map(|candidate| candidate.text.as_str())
            .collect::<Vec<_>>()
            .join(".");
        facts.push(ProgramFact::new(&token.text, names, token.range));
    }
    facts.extend(project.dependencies.iter().map(|dependency| {
        ProgramFact::new(
            "declared-project-dependency",
            dependency,
            ProgramRange::default(),
        )
    }));
    for request in module_requests(syntax, source, language) {
        if project_has_module(project, &request.name, language) {
            facts.push(ProgramFact::new(
                "recognized-toolchain-module",
                &request.name,
                request.range,
            ));
        }
        facts.push(request);
    }
    unique_facts(facts)
}

fn type_facts(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    language: &str,
) -> Vec<ProgramFact> {
    let mut facts = syntax
        .iter()
        .filter(|mapping| {
            let term = mapping.term.to_ascii_lowercase();
            term.contains("type") || term.contains("universe")
        })
        .map(|mapping| {
            ProgramFact::new("syntax-type", &mapping.term, mapping.range).with_phase("surface")
        })
        .collect::<Vec<_>>();
    for (index, token) in tokens.iter().enumerate() {
        if token.text == ":" {
            if let Some(value) = tokens.get(index + 1) {
                facts.push(
                    ProgramFact::new("annotation", &value.text, value.range).with_phase("surface"),
                );
            }
        }
        if matches!(token.text.as_str(), "universe" | "Universe" | "Type") {
            facts.push(
                ProgramFact::new(
                    "universe",
                    tokens
                        .get(index + 1)
                        .map_or(&token.text, |value| &value.text),
                    token.range,
                )
                .with_phase("surface"),
            );
        }
    }
    if language == "JavaScript" {
        facts.push(
            ProgramFact::new("dynamic-type", "ECMAScript value", ProgramRange::default())
                .with_phase("runtime"),
        );
    }
    unique_facts(facts)
}

fn extension_facts(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    source: &str,
    language: &str,
) -> Vec<ProgramFact> {
    let markers = extension_markers(language);
    let mut facts = tokens
        .iter()
        .filter(|token| markers.contains(&token.text.as_str()))
        .map(|token| ProgramFact::new(&token.text, &token.text, token.range))
        .collect::<Vec<_>>();
    facts.extend(syntax.iter().filter_map(|mapping| {
        let term = mapping.term.to_ascii_lowercase();
        [
            "macro",
            "attribute",
            "decorator",
            "quotation",
            "template",
            "notation",
        ]
        .iter()
        .any(|needle| term.contains(needle))
        .then(|| ProgramFact::new(&mapping.term, &mapping.term, mapping.range))
    }));
    if language == "JavaScript" {
        if let Some(start) = source.find("\"use strict\"") {
            facts.push(ProgramFact::new(
                "directive",
                "use strict",
                ProgramRange::new(start, start + 12),
            ));
        }
    }
    unique_facts(facts)
}

fn proof_facts(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    language: &str,
) -> Vec<ProgramFact> {
    if matches!(language, "JavaScript" | "Rust") {
        return Vec::new();
    }
    let markers = proof_markers(language);
    let facts = tokens
        .iter()
        .filter(|token| markers.contains(&token.text.as_str()))
        .map(|token| ProgramFact::new(&token.text, &token.text, token.range))
        .chain(syntax.iter().filter_map(|mapping| {
            let term = mapping.term.to_ascii_lowercase();
            ["theorem", "proof", "tactic"]
                .iter()
                .any(|needle| term.contains(needle))
                .then(|| ProgramFact::new(&mapping.term, &mapping.term, mapping.range))
        }))
        .collect();
    unique_facts(facts)
}

fn diagnostic_facts(network: &LinkNetwork) -> Vec<ProgramDiagnostic> {
    network
        .verify_full_match(None)
        .issues()
        .iter()
        .filter_map(|issue| {
            let link = network.link(issue.link_id())?;
            let span = link.metadata().span();
            Some(ProgramDiagnostic {
                kind: if link.metadata().flags().is_missing() {
                    "missing"
                } else {
                    "parse-error"
                },
                term: link.metadata().term().unwrap_or_default().to_string(),
                range: span.map_or_else(ProgramRange::default, |span| {
                    ProgramRange::new(span.byte_range().start(), span.byte_range().end())
                }),
            })
        })
        .collect()
}

fn project_diagnostics(
    modules: &[ProgramFact],
    _project: &ProgramProjectContext,
) -> Vec<ProgramDiagnostic> {
    let recognized = modules
        .iter()
        .filter(|fact| fact.kind == "recognized-toolchain-module")
        .map(|fact| fact.name.as_str())
        .collect::<std::collections::HashSet<_>>();
    modules
        .iter()
        .filter(|fact| fact.kind == "module-import" && !recognized.contains(fact.name.as_str()))
        .map(|fact| ProgramDiagnostic {
            kind: "missing-project-context",
            term: fact.name.clone(),
            range: fact.range,
        })
        .collect()
}

fn construct_facts(program: &ProgramRepresentation) -> Vec<ProgramConstruct> {
    SEMANTIC_CONSTRUCTS
        .iter()
        .map(|kind| {
            if *kind == "surface-expansion-elaboration-traces" {
                return ProgramConstruct {
                    kind,
                    status: ProgramConstructStatus::Unavailable,
                    evidence: Vec::new(),
                    rationale: Some(
                        "no macro expansion or elaboration trace has been produced".to_string(),
                    ),
                };
            }
            if *kind == "proof-terms-and-tactics"
                && matches!(program.language, "JavaScript" | "Rust")
            {
                return ProgramConstruct {
                    kind,
                    status: ProgramConstructStatus::NotApplicable,
                    evidence: Vec::new(),
                    rationale: Some(format!(
                        "{} defines no proof/tactic sublanguage",
                        program.language
                    )),
                };
            }
            let evidence = construct_evidence(program, kind);
            let represented = !evidence.is_empty();
            ProgramConstruct {
                kind,
                status: if represented {
                    ProgramConstructStatus::Represented
                } else {
                    ProgramConstructStatus::NotPresent
                },
                evidence,
                rationale: (!represented).then(|| {
                    "the analyzed source contains no instance of this construct".to_string()
                }),
            }
        })
        .collect()
}

fn construct_evidence(program: &ProgramRepresentation, kind: &str) -> Vec<ProgramFact> {
    match kind {
        "modules-and-imports" => program.modules.clone(),
        "scopes-and-bindings" => program
            .bindings
            .iter()
            .map(|binding| ProgramFact::new(&binding.kind, &binding.name, binding.declaration))
            .collect(),
        "recursive-definitions" => program
            .bindings
            .iter()
            .filter(|binding| {
                matches!(
                    binding.kind.as_str(),
                    "function" | "Fixpoint" | "CoFixpoint" | "def"
                ) && !binding.references.is_empty()
            })
            .map(|binding| ProgramFact::new("recursive", &binding.name, binding.declaration))
            .collect(),
        "types-and-universes" => program.types.clone(),
        "effects" => effect_facts(program),
        "attributes" => program
            .extensions
            .iter()
            .filter(|fact| {
                let kind = fact.kind.to_ascii_lowercase();
                ["attribute", "directive", "allow", "local", "simp", "#"]
                    .iter()
                    .any(|needle| kind.contains(needle))
            })
            .cloned()
            .collect(),
        "macros-and-notation" => program
            .extensions
            .iter()
            .filter(|fact| {
                let kind = fact.kind.to_ascii_lowercase();
                [
                    "macro", "notation", "template", "tagged", "prefix", "postfix", "infix",
                    "syntax",
                ]
                .iter()
                .any(|needle| kind.contains(needle))
            })
            .cloned()
            .collect(),
        "proof-terms-and-tactics" => program.proofs.clone(),
        "project-context-and-dependencies" => program
            .project
            .files
            .iter()
            .chain(&program.project.dependencies)
            .map(|name| ProgramFact::new("project", name, ProgramRange::default()))
            .collect(),
        _ => Vec::new(),
    }
}

fn effect_facts(program: &ProgramRepresentation) -> Vec<ProgramFact> {
    let mut facts = Vec::new();
    for marker in effect_markers(program.language) {
        facts.extend(program.source.match_indices(marker).map(|(start, value)| {
            ProgramFact::new(
                "effect",
                value,
                ProgramRange::new(start, start + value.len()),
            )
        }));
    }
    unique_facts(facts)
}
