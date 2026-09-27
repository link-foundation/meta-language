//! Project-aware semantics: resolves an entry program's module requests to the
//! project's source files, reads the project manifests, and links the entry's
//! imports, references, attributes, macros, notations and tactics to the
//! declarations they name in other files. Every symbol is identified as
//! `<file>#<qualified name>`. The JavaScript runtime implements the same
//! analysis in js/src/program-project.js.

mod javascript;
mod lean;
mod rocq;
mod rust_lang;
mod tree;

use std::collections::BTreeMap;
use std::rc::Rc;

use super::analysis::syntax_facts;
use super::{ProgramBinding, ProgramDiagnostic, ProgramFact, ProgramRange};
use crate::{LinkNetwork, ParseConfiguration};
use tree::{SyntaxTree, Target};

const TYPE_KINDS: [&str; 9] = [
    "class",
    "struct",
    "enum",
    "trait",
    "type",
    "structure",
    "inductive",
    "constructor",
    "universe",
];
const EFFECT_TRAITS: [&str; 4] = ["async", "generator", "io", "proof-state"];
const PROOF_KINDS: [&str; 3] = ["theorem", "lemma", "tactic"];
// Tactic steps, and the checked assertions of languages without proof terms.
const PROOF_ROLES: [&str; 3] = ["tactic", "assertion", "const-assertion"];

/// One project file by project-relative path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramProjectSource {
    path: String,
    source: String,
}

impl ProgramProjectSource {
    /// Creates a project file.
    #[must_use]
    pub fn new(path: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            source: source.into(),
        }
    }

    /// Project-relative path.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// File contents.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }
}

/// Project files and dependency names available during analysis.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProgramProjectContext {
    root: String,
    files: Vec<String>,
    dependencies: Vec<String>,
    extensions: Vec<String>,
    entry: Option<String>,
    sources: Vec<ProgramProjectSource>,
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
            entry: None,
            sources: Vec::new(),
        }
    }

    /// Adds project-defined syntax extensions.
    #[must_use]
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }

    /// Names the analyzed program's path within the project and the project's
    /// files (manifests and modules), enabling project-aware semantics.
    #[must_use]
    pub fn with_entry(
        mut self,
        entry: impl Into<String>,
        sources: Vec<ProgramProjectSource>,
    ) -> Self {
        self.entry = Some(entry.into());
        self.sources = sources;
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

    /// The analyzed program's path within the project.
    #[must_use]
    pub fn entry(&self) -> Option<&str> {
        self.entry.as_deref()
    }

    /// The project's files by project-relative path.
    #[must_use]
    pub fn sources(&self) -> &[ProgramProjectSource] {
        &self.sources
    }
}

/// An entry module request resolved to a project file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramProjectModule {
    request: String,
    module: String,
    file: String,
    range: ProgramRange,
}

impl ProgramProjectModule {
    /// The request as written.
    #[must_use]
    pub fn request(&self) -> &str {
        &self.request
    }

    /// The resolved module.
    #[must_use]
    pub fn module(&self) -> &str {
        &self.module
    }

    /// The project file defining the module.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    /// Entry range of the request.
    #[must_use]
    pub const fn range(&self) -> ProgramRange {
        self.range
    }
}

/// An entry range linked to the declaration it names, identified as
/// `<file>#<qualified name>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramProjectReference {
    role: &'static str,
    name: String,
    range: ProgramRange,
    symbol: String,
    target_kind: String,
    traits: Vec<String>,
    file: String,
    declaration: ProgramRange,
}

impl ProgramProjectReference {
    /// How the entry uses the declaration: `import`, `reference`, `attribute`,
    /// `macro`, `notation`, `template-tag`, `tactic`, `assertion` or
    /// `const-assertion`.
    #[must_use]
    pub const fn role(&self) -> &'static str {
        self.role
    }

    /// The name as written.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Entry range of the use.
    #[must_use]
    pub const fn range(&self) -> ProgramRange {
        self.range
    }

    /// Declaration identity.
    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Declaration kind.
    #[must_use]
    pub fn target_kind(&self) -> &str {
        &self.target_kind
    }

    /// Declaration traits (`recursive`, `async`, `const`, `io`, ...).
    #[must_use]
    pub fn traits(&self) -> &[String] {
        &self.traits
    }

    /// The project file declaring the target.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    /// Declaration range within its file.
    #[must_use]
    pub const fn declaration(&self) -> ProgramRange {
        self.declaration
    }
}

/// A macro, notation or tagged template use with the source it expands to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramExpansion {
    name: String,
    kind: &'static str,
    range: ProgramRange,
    expansion: String,
    target: String,
}

impl ProgramExpansion {
    /// The macro, notation or tag name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// `macro-rules`, `notation` or `tagged-template`.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        self.kind
    }

    /// Entry range of the use.
    #[must_use]
    pub const fn range(&self) -> ProgramRange {
        self.range
    }

    /// Expanded source.
    #[must_use]
    pub fn expansion(&self) -> &str {
        &self.expansion
    }

    /// Symbol of the expanded definition.
    #[must_use]
    pub fn target(&self) -> &str {
        &self.target
    }
}

/// Results of the project-aware analysis.
#[derive(Clone, Debug, Default)]
pub(super) struct ProjectResult {
    pub(super) modules: Vec<ProgramProjectModule>,
    pub(super) facts: Vec<ProgramFact>,
    pub(super) references: Vec<ProgramProjectReference>,
    pub(super) expansions: Vec<ProgramExpansion>,
    pub(super) diagnostics: Vec<ProgramDiagnostic>,
    /// Module requests the language-neutral request scan does not see.
    pub(super) requests: Vec<ProgramFact>,
    /// Requests resolved to project files, as (name, start).
    pub(super) resolved_requests: Vec<(String, usize)>,
}

/// Analyzes an entry program within its project. Returns empty results
/// unless the project names an entry file.
pub(super) fn project_semantics(
    language: &'static str,
    source: &str,
    mappings: &[super::ProgramSourceMapping],
    bindings: &[ProgramBinding],
    unresolved: &[ProgramFact],
    project: &ProgramProjectContext,
) -> ProjectResult {
    let Some(entry) = project.entry().filter(|entry| !entry.is_empty()) else {
        return ProjectResult::default();
    };
    let mut context = ProjectContext::new(language, entry, source, mappings, project);
    context.bindings = bindings;
    context.unresolved = unresolved;
    match language {
        "JavaScript" => javascript::analyze(&mut context),
        "Rust" => rust_lang::analyze(&mut context),
        "Lean" => lean::analyze(&mut context),
        "Rocq" => rocq::analyze(&mut context),
        _ => {}
    }
    for file in &project.sources {
        if context
            .parsed
            .get(&file.path)
            .is_some_and(|parsed| !parsed.clean)
        {
            context.result.diagnostics.push(ProgramDiagnostic {
                kind: "project-parse-error",
                term: file.path.clone(),
                range: ProgramRange::default(),
            });
        }
    }
    context.result
}

/// A parsed project file.
#[derive(Debug)]
struct ParsedFile {
    path: String,
    source: String,
    tree: SyntaxTree,
    clean: bool,
}

struct ProjectContext<'a> {
    language: &'static str,
    bindings: &'a [ProgramBinding],
    unresolved: &'a [ProgramFact],
    entry: Rc<ParsedFile>,
    sources: BTreeMap<&'a str, &'a str>,
    parsed: BTreeMap<String, Rc<ParsedFile>>,
    result: ProjectResult,
}

impl<'a> ProjectContext<'a> {
    fn new(
        language: &'static str,
        entry: &str,
        source: &str,
        mappings: &[super::ProgramSourceMapping],
        project: &'a ProgramProjectContext,
    ) -> Self {
        let mut context = Self {
            language,
            bindings: &[],
            unresolved: &[],
            entry: Rc::new(ParsedFile {
                path: entry.to_string(),
                source: source.to_string(),
                tree: SyntaxTree::build(mappings, source),
                clean: true,
            }),
            sources: project
                .sources
                .iter()
                .map(|file| (file.path.as_str(), file.source.as_str()))
                .collect(),
            parsed: BTreeMap::new(),
            result: ProjectResult::default(),
        };
        for file in &project.sources {
            context.fact(
                "project-file",
                &file.path,
                &file.path,
                (0, file.source.len()),
            );
        }
        context
    }

    fn has(&self, path: &str) -> bool {
        path == self.entry.path || self.sources.contains_key(path)
    }

    fn text(&self, path: &str) -> Option<&str> {
        if path == self.entry.path {
            return Some(&self.entry.source);
        }
        self.sources.get(path).copied()
    }

    /// Parses a project file once; `None` when the project has no such file.
    fn load(&mut self, path: &str) -> Option<Rc<ParsedFile>> {
        self.load_as(path, self.language)
    }

    fn load_as(&mut self, path: &str, language: &str) -> Option<Rc<ParsedFile>> {
        if path == self.entry.path {
            return Some(Rc::clone(&self.entry));
        }
        let source = *self.sources.get(path)?;
        let parsed = self.parsed.entry(path.to_string()).or_insert_with(|| {
            let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
            Rc::new(ParsedFile {
                path: path.to_string(),
                source: source.to_string(),
                tree: SyntaxTree::build(&syntax_facts(&network), source),
                clean: network.verify_full_match(None).is_clean(),
            })
        });
        Some(Rc::clone(parsed))
    }

    fn fact(&mut self, kind: &str, name: &str, file: &str, range: (usize, usize)) {
        let mut fact = ProgramFact::new(kind, name, ProgramRange::new(range.0, range.1));
        fact.file = Some(file.to_string());
        self.result.facts.push(fact);
    }

    fn module(&mut self, request: &str, module: &str, file: &str, range: (usize, usize)) {
        self.result.modules.push(ProgramProjectModule {
            request: request.to_string(),
            module: module.to_string(),
            file: file.to_string(),
            range: ProgramRange::new(range.0, range.1),
        });
        self.result
            .resolved_requests
            .push((request.to_string(), range.0));
    }

    /// A module request the language-neutral request scan does not see.
    fn request(&mut self, name: &str, range: (usize, usize)) {
        self.result.requests.push(ProgramFact::new(
            "module-import",
            name,
            ProgramRange::new(range.0, range.1),
        ));
    }

    /// Links an entry range to a declaration.
    fn reference(
        &mut self,
        role: &'static str,
        name: &str,
        range: (usize, usize),
        target: &Target,
    ) {
        self.result.references.push(ProgramProjectReference {
            role,
            name: name.to_string(),
            range: ProgramRange::new(range.0, range.1),
            symbol: target.symbol.clone(),
            target_kind: target.kind.clone(),
            traits: target.traits.clone(),
            file: target.file.clone(),
            declaration: ProgramRange::new(target.start, target.end),
        });
    }

    fn expansion(
        &mut self,
        name: &str,
        kind: &'static str,
        range: (usize, usize),
        expansion: String,
        target: &str,
    ) {
        self.result.expansions.push(ProgramExpansion {
            name: name.to_string(),
            kind,
            range: ProgramRange::new(range.0, range.1),
            expansion,
            target: target.to_string(),
        });
    }

    fn diagnose(&mut self, kind: &'static str, term: String, range: (usize, usize)) {
        self.result.diagnostics.push(ProgramDiagnostic {
            kind,
            term,
            range: ProgramRange::new(range.0, range.1),
        });
    }
}

/// Construct evidence contributed by the project analysis.
pub(super) fn project_evidence(
    project: &ProgramProjectContext,
    result: &ProjectResult,
    construct: &str,
) -> Vec<ProgramFact> {
    let entry = project.entry().unwrap_or_default();
    let located = |kind: String, name: &str, range: ProgramRange, file: &str| {
        let mut fact = ProgramFact::new(kind, name, range);
        fact.file = Some(file.to_string());
        fact
    };
    let references = |predicate: &dyn Fn(&ProgramProjectReference) -> bool| {
        result
            .references
            .iter()
            .filter(|reference| predicate(reference))
            .map(|reference| {
                located(
                    format!(
                        "project-reference:{}:{}",
                        reference.role, reference.target_kind
                    ),
                    &reference.symbol,
                    reference.range,
                    entry,
                )
            })
            .collect::<Vec<_>>()
    };
    let has_trait = |reference: &ProgramProjectReference, traits: &[&str]| {
        reference
            .traits
            .iter()
            .any(|value| traits.contains(&value.as_str()))
    };
    match construct {
        "modules-and-imports" => result
            .modules
            .iter()
            .map(|module| {
                located(
                    "project-module".to_string(),
                    &module.module,
                    module.range,
                    entry,
                )
            })
            .chain(references(&|reference| reference.role == "import"))
            .collect(),
        "scopes-and-bindings" => references(&|reference| reference.role == "reference"),
        "recursive-definitions" => references(&|reference| has_trait(reference, &["recursive"])),
        "types-and-universes" => references(&|reference| {
            TYPE_KINDS.contains(&reference.target_kind.as_str())
                || has_trait(reference, &["universe-polymorphic"])
        }),
        "effects" => references(&|reference| has_trait(reference, &EFFECT_TRAITS)),
        "attributes" => references(&|reference| reference.role == "attribute"),
        "macros-and-notation" => {
            references(&|reference| matches!(reference.role, "macro" | "notation" | "template-tag"))
        }
        "proof-terms-and-tactics" => references(&|reference| {
            PROOF_ROLES.contains(&reference.role)
                || PROOF_KINDS.contains(&reference.target_kind.as_str())
        }),
        "surface-expansion-elaboration-traces" => result
            .expansions
            .iter()
            .map(|expansion| {
                located(
                    format!("expansion:{}", expansion.kind),
                    &expansion.target,
                    expansion.range,
                    entry,
                )
            })
            .collect(),
        "project-context-and-dependencies" => result.facts.clone(),
        _ => Vec::new(),
    }
}
