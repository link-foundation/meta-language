//! Faithful lowering of a grammar into a less expressive grammar notation.
//!
//! Every construct the target notation cannot write so that its own importer
//! reads it back unchanged is moved into a fresh helper rule whose body is an
//! encoding in constructs the target does write: an optional item becomes a
//! choice with the empty expression, a repetition a recursive helper, a
//! character set a choice of its characters, a case-insensitive literal a
//! sequence of per-letter character sets, a capture its item. Each rewrite is
//! a step of the reconstruction metadata, which names the helper, the rule it
//! was lowered from, the construct, whether the encoding is exact (accepts the
//! same texts) or approximate (an ordered choice written as an unordered one, a
//! lookahead written as the empty expression, an arbitrary character limited
//! to printable ASCII), a note and the original expression as native links.
//! Rule names, kinds and documentation the target does not keep are metadata
//! steps as well, and so are the declarations of the grammar feature union
//! (matching, imports, modes, extras, conflicts, macros and scanners) and the
//! rule parameters, channels, modes and actions: no target notation writes
//! them, so the executable does not honor them and a lowering that carries
//! them is approximate. The executable grammar (the emitted text) is therefore
//! distinct from the lossless interchange package (executable plus metadata),
//! and [`check_grammar_lowering`] reconstructs the original from the package
//! and reports every feature the package does not carry. It mirrors
//! `js/src/grammar-lowering.js`.

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

use super::super::emit::{EmitReport, GrammarEmitError};
use super::super::import::GrammarImportError;
use super::super::round_trip::{GrammarEmitFn, GrammarImportFn, GrammarRoundTripError};
use super::super::{Grammar, GrammarExpr, GrammarFormat, GrammarRule, RuleAttributes, RuleKind};
use super::links::render_links_expression;
use super::{grammar_emitter, grammar_importer, render_rule_fields};

mod encoding;
mod metadata;

use encoding::{Construct, constructs_of, encode, kept_kinds, unsupported};
use metadata::render_declarations_text;

pub use encoding::MAX_LOWERED_CHARACTERS;

pub use metadata::{
    GrammarLoweringFailure, GrammarLoweringFailureKind, GrammarLoweringReport,
    check_grammar_lowering, dropped_grammar_features, parse_lowering_metadata, reconstruct_grammar,
    render_lowering_metadata,
};

/// The notations the lowering writes.
pub const GRAMMAR_LOWERING_FORMATS: &[&str] = &[
    "abnf",
    "antlr",
    "bnf",
    "ebnf",
    "gbnf",
    "lark",
    "pest",
    "tree-sitter-json",
];

const HELPER_PREFIX: &str = "lowered";

/// Whether a lowering kept every encoding exact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarLoweringStatus {
    /// Every encoding accepts the same texts as the original.
    Exact,
    /// At least one encoding accepts different texts.
    Approximate,
    /// A check of the lowering failed.
    Broken,
}

impl GrammarLoweringStatus {
    /// Stable tag shared with the JavaScript runtime.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::Broken => "broken",
        }
    }
}

/// Whether one helper accepts the same texts as the expression it replaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarLoweringEncoding {
    /// The helper accepts the same texts.
    Exact,
    /// The helper accepts different texts.
    Approximate,
}

impl GrammarLoweringEncoding {
    /// Stable tag shared with the JavaScript runtime.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
        }
    }
}

/// One step of the reconstruction metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrammarLoweringStep {
    /// The target writes `rule` under the name `value`.
    Rename {
        /// The original rule name.
        rule: String,
        /// The name in the executable grammar.
        value: String,
    },
    /// An expression of `owner` moved into the helper rule `helper`.
    Helper {
        /// The helper rule name.
        helper: String,
        /// The rule the expression was lowered from.
        owner: String,
        /// The construct the target does not write.
        construct: String,
        /// Whether the helper accepts the same texts.
        encoding: GrammarLoweringEncoding,
        /// How the helper encodes the construct.
        note: String,
        /// The expression the helper replaces.
        original: GrammarExpr,
    },
    /// The target does not keep the kind of `rule`.
    Kind {
        /// The original rule name.
        rule: String,
        /// The original rule kind.
        kind: RuleKind,
    },
    /// The target does not keep the documentation of `rule` verbatim.
    Doc {
        /// The original rule name.
        rule: String,
        /// The original documentation.
        doc: String,
    },
    /// The feature union declarations of the grammar, which no target writes
    /// and the executable does not honor.
    Declarations {
        /// The declarations as native links: a grammar link with the
        /// matching, then one link per declaration, each on its own line.
        declarations: String,
    },
    /// The parameters, channel, modes and action of `rule`, which no target
    /// writes and the executable does not honor.
    Attributes {
        /// The original rule name.
        rule: String,
        /// The rule fields as native links, separated by spaces.
        attributes: String,
    },
}

/// The reconstruction metadata of a lowering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarLoweringMetadata {
    /// The target notation, one of [`GRAMMAR_LOWERING_FORMATS`].
    pub format: String,
    /// Whether every encoding is exact.
    pub status: GrammarLoweringStatus,
    /// The source format of the original grammar.
    pub source: Option<GrammarFormat>,
    /// The original start rule.
    pub start: String,
    /// The original rule names in grammar order.
    pub order: Vec<String>,
    /// The steps in the order they were taken.
    pub steps: Vec<GrammarLoweringStep>,
}

/// A grammar lowered into one notation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarLowering {
    /// The target notation.
    pub format: String,
    /// Whether every encoding is exact.
    pub status: GrammarLoweringStatus,
    /// The lowered grammar under the names the target gives its rules.
    pub grammar: Grammar,
    /// The executable text the target emitter writes for it.
    pub executable: String,
    /// The emitter's fidelity report for the executable.
    pub report: EmitReport,
    /// The grammar the target importer reads from the executable.
    pub imported: Grammar,
    /// The reconstruction steps.
    pub steps: Vec<GrammarLoweringStep>,
    /// The reconstruction metadata as links.
    pub metadata: String,
}

/// The emitter, importer, samples and metadata edit a lowering runs with.
#[derive(Clone, Copy, Default)]
pub struct GrammarLoweringOptions<'a> {
    /// Replaces the target's own emitter.
    pub emit: Option<GrammarEmitFn<'a>>,
    /// Replaces the target's own importer.
    pub import: Option<GrammarImportFn<'a>>,
    /// Texts an exact lowering must accept.
    pub accepts: &'a [String],
    /// Texts an exact lowering must reject.
    pub rejects: &'a [String],
    /// Rewrites the metadata before the reconstruction, for negative controls.
    pub edit_metadata: Option<&'a dyn Fn(&str) -> String>,
}

/// A lowering or reconstruction that cannot be carried out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrammarLoweringError {
    /// The format is not one of [`GRAMMAR_LOWERING_FORMATS`].
    UnsupportedFormat(String),
    /// The grammar has no start rule.
    NoStartRule,
    /// A character set holds no character the format can spell.
    Unencodable(String),
    /// The executable grammar has no rule of this name.
    MissingRule(String),
    /// The importer rejected the executable or the metadata is malformed.
    Import(GrammarImportError),
    /// The emitter rejected the lowered grammar.
    Emit(GrammarEmitError),
    /// A rule definition could not be normalized for comparison.
    Definition(GrammarRoundTripError),
}

impl fmt::Display for GrammarLoweringError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedFormat(format) => write!(
                formatter,
                "the lowering does not support the format {format}"
            ),
            Self::NoStartRule => formatter.write_str("the grammar has no start rule"),
            Self::Unencodable(format) => write!(
                formatter,
                "a character set without printable ASCII characters cannot be written in {format}"
            ),
            Self::MissingRule(name) => {
                write!(formatter, "the executable grammar has no rule {name}")
            }
            Self::Import(error) => write!(formatter, "{error}"),
            Self::Emit(error) => write!(formatter, "{error}"),
            Self::Definition(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for GrammarLoweringError {}

impl From<GrammarImportError> for GrammarLoweringError {
    fn from(error: GrammarImportError) -> Self {
        Self::Import(error)
    }
}

impl From<GrammarEmitError> for GrammarLoweringError {
    fn from(error: GrammarEmitError) -> Self {
        Self::Emit(error)
    }
}

impl From<GrammarRoundTripError> for GrammarLoweringError {
    fn from(error: GrammarRoundTripError) -> Self {
        Self::Definition(error)
    }
}

fn check_format(format: &str) -> Result<(), GrammarLoweringError> {
    if GRAMMAR_LOWERING_FORMATS.contains(&format) {
        Ok(())
    } else {
        Err(GrammarLoweringError::UnsupportedFormat(format.to_owned()))
    }
}

/// Rebuilds `expr` with `map` applied to its direct sub-expressions.
pub(super) fn map_children<E>(
    expr: &GrammarExpr,
    map: &mut impl FnMut(&GrammarExpr) -> Result<GrammarExpr, E>,
) -> Result<GrammarExpr, E> {
    let mut boxed = |item: &GrammarExpr| map(item).map(Box::new);
    Ok(match expr {
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => GrammarExpr::Choice {
            ordered: *ordered,
            alternatives: alternatives
                .iter()
                .map(&mut *map)
                .collect::<Result<_, _>>()?,
        },
        GrammarExpr::Sequence(items) => {
            GrammarExpr::Sequence(items.iter().map(&mut *map).collect::<Result<_, _>>()?)
        }
        GrammarExpr::Optional(item) => GrammarExpr::Optional(boxed(item)?),
        GrammarExpr::ZeroOrMore(item) => GrammarExpr::ZeroOrMore(boxed(item)?),
        GrammarExpr::OneOrMore(item) => GrammarExpr::OneOrMore(boxed(item)?),
        GrammarExpr::And(item) => GrammarExpr::And(boxed(item)?),
        GrammarExpr::Not(item) => GrammarExpr::Not(boxed(item)?),
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::Repeat {
            expr: boxed(expr)?,
            min: *min,
            max: *max,
        },
        GrammarExpr::Capture { label, expr } => GrammarExpr::Capture {
            label: label.clone(),
            expr: boxed(expr)?,
        },
        _ => expr.clone(),
    })
}

/// Builds a grammar from its parts.
pub(super) fn assemble(
    start: &str,
    rules: Vec<GrammarRule>,
    source: Option<GrammarFormat>,
) -> Grammar {
    let mut grammar = Grammar::new().with_start(start);
    for rule in rules {
        grammar.add_rule(rule);
    }
    if let Some(source) = source {
        grammar.set_source_format(source);
    }
    grammar
}

fn rule(name: &str, kind: RuleKind, expr: GrammarExpr, doc: Option<String>) -> GrammarRule {
    GrammarRule {
        name: name.to_owned(),
        expr,
        kind,
        concept: None,
        doc,
        attributes: RuleAttributes::default(),
    }
}

/// The helper rules and steps of one lowering.
struct Lowerer<'a> {
    format: &'a str,
    unsupported: &'static [Construct],
    taken: HashSet<String>,
    counter: usize,
    helpers: Vec<GrammarRule>,
    memo: HashMap<String, String>,
    steps: Vec<GrammarLoweringStep>,
}

impl Lowerer<'_> {
    /// A fresh helper name; a lexer helper gets an upper-case name, which
    /// ANTLR and Lark read as a token rule.
    fn helper_name(&mut self, lexical: bool) -> String {
        let name = loop {
            self.counter += 1;
            let name = format!("{HELPER_PREFIX}{}", self.counter);
            if !self.taken.contains(&name) {
                break name;
            }
        };
        self.taken.insert(name.clone());
        if lexical { name.to_uppercase() } else { name }
    }

    /// Lowers `expr` of `owner`; `lexical` marks a lexer rule body, where
    /// character-level constructs are matched, and ANTLR's parser rules get
    /// them through a lexer helper.
    fn lower(
        &mut self,
        expr: &GrammarExpr,
        owner: &str,
        inner: bool,
        lexical: bool,
    ) -> Result<GrammarExpr, GrammarLoweringError> {
        let Some(construct) = constructs_of(expr, inner, lexical)
            .into_iter()
            .find(|construct| self.unsupported.contains(construct))
        else {
            return map_children(expr, &mut |item| self.lower(item, owner, true, lexical));
        };
        let helper_lexical = lexical || construct == Construct::ParserCharacters;
        let key = format!(
            "{} {helper_lexical} {}",
            construct.as_str(),
            render_links_expression(expr)
        );
        if let Some(helper) = self.memo.get(&key) {
            return Ok(GrammarExpr::NonTerminal(helper.clone()));
        }
        let helper = self.helper_name(helper_lexical);
        self.memo.insert(key, helper.clone());
        let (body, encoding, note) = encode(self.format, construct, expr, &helper)?;
        self.steps.push(GrammarLoweringStep::Helper {
            helper: helper.clone(),
            owner: owner.to_owned(),
            construct: construct.as_str().to_owned(),
            encoding,
            note: note.to_owned(),
            original: expr.clone(),
        });
        let index = self.helpers.len();
        let kind = if helper_lexical {
            RuleKind::Token
        } else {
            RuleKind::Normal
        };
        self.helpers
            .push(rule(&helper, kind, GrammarExpr::Empty, None));
        self.helpers[index].expr = self.lower(&body, &helper, false, helper_lexical)?;
        Ok(GrammarExpr::NonTerminal(helper))
    }
}

/// Lowers `grammar` into `format`.
///
/// The result holds the lowered grammar, the executable text the target
/// emitter writes for it, the steps and the metadata links that reconstruct
/// the original from the executable, and whether every encoding is exact. The options' `emit` and `import` replace the format's own pair.
///
/// # Errors
///
/// Returns a [`GrammarLoweringError`] for an unsupported format, a grammar
/// without a start rule, a character set the format cannot spell, or an
/// emitter or importer error.
pub fn lower_grammar(
    grammar: &Grammar,
    format: &str,
    options: &GrammarLoweringOptions<'_>,
) -> Result<GrammarLowering, GrammarLoweringError> {
    check_format(format)?;
    let start = grammar
        .start_rule()
        .ok_or(GrammarLoweringError::NoStartRule)?;
    let own_emit = grammar_emitter(format)
        .ok_or_else(|| GrammarLoweringError::UnsupportedFormat(format.to_owned()))?;
    let own_import = grammar_importer(format)
        .ok_or_else(|| GrammarLoweringError::UnsupportedFormat(format.to_owned()))?;
    let emit: GrammarEmitFn<'_> = options.emit.unwrap_or(&own_emit);
    let import: GrammarImportFn<'_> = options.import.unwrap_or(&own_import);

    let mut state = Lowerer {
        format,
        unsupported: unsupported(format),
        taken: grammar
            .rule_names()
            .iter()
            .map(|name| name.to_lowercase())
            .collect(),
        counter: 0,
        helpers: Vec::new(),
        memo: HashMap::new(),
        steps: Vec::new(),
    };
    let kept = kept_kinds(format);
    let mut rules = Vec::new();
    for original in grammar.rules() {
        let expr = state.lower(&original.expr, &original.name, false, false)?;
        // A tree-sitter token cannot refer to other rules, so a token rule
        // that needs a helper becomes a normal rule with a kind step.
        let refers_to_helper = format == "tree-sitter-json"
            && render_links_expression(&expr) != render_links_expression(&original.expr);
        let kind = if kept.contains(&original.kind) && !refers_to_helper {
            original.kind
        } else {
            RuleKind::Normal
        };
        rules.push(rule(&original.name, kind, expr, original.doc.clone()));
    }
    rules.append(&mut state.helpers);
    // The lowered grammar keeps the original source format, so the emitter
    // reports every construct it cannot write for a grammar of that format.
    let lowered = assemble(&start.name, rules, grammar.source_format());

    // A first emission shows the names the target gives the rules (GBNF
    // starts at root, ANTLR and Lark adjust case), matched by position, which
    // every emitter keeps. The lowered grammar takes those names, so the
    // target writes it without renaming, and the renames become steps.
    let first = import(&emit(&lowered)?.0)?;
    let first = first.rule_names();
    let renames: Vec<(String, String)> = lowered
        .rule_names()
        .iter()
        .zip(&first)
        .filter(|(name, executable)| name != executable)
        .map(|(name, executable)| ((*name).to_owned(), (*executable).to_owned()))
        .collect();
    let named = |name: &str| {
        renames
            .iter()
            .find(|(rule, _)| rule == name)
            .map_or_else(|| name.to_owned(), |(_, value)| value.clone())
    };
    let mut executable_rules: Vec<GrammarRule> = lowered
        .rules()
        .iter()
        .map(|lowered_rule| {
            let expr = rename_references(&lowered_rule.expr, &named);
            rule(
                &named(&lowered_rule.name),
                lowered_rule.kind,
                expr,
                lowered_rule.doc.clone(),
            )
        })
        .collect();
    let executable_start = named(&start.name);
    let mut executable = assemble(
        &executable_start,
        executable_rules.clone(),
        grammar.source_format(),
    );
    let (mut text, mut report) = emit(&executable)?;
    let mut imported = import(&text)?;
    // Documentation the target does not read back verbatim (ANTLR keeps the
    // comment markers, BNF has no comments) leaves the executable and is
    // carried by a doc step instead.
    let mut unkept = false;
    for executable_rule in &mut executable_rules {
        let kept_doc = imported
            .rule(&executable_rule.name)
            .is_some_and(|back| back.doc == executable_rule.doc);
        if executable_rule.doc.is_some() && !kept_doc {
            executable_rule.doc = None;
            unkept = true;
        }
    }
    if unkept {
        executable = assemble(&executable_start, executable_rules, grammar.source_format());
        (text, report) = emit(&executable)?;
        imported = import(&text)?;
    }

    let mut steps: Vec<GrammarLoweringStep> = renames
        .iter()
        .map(|(rule, value)| GrammarLoweringStep::Rename {
            rule: rule.clone(),
            value: value.clone(),
        })
        .collect();
    steps.append(&mut state.steps);
    for original in grammar.rules() {
        let Some(back) = imported.rule(&named(&original.name)) else {
            continue;
        };
        if back.kind != original.kind {
            steps.push(GrammarLoweringStep::Kind {
                rule: original.name.clone(),
                kind: original.kind,
            });
        }
        if let Some(doc) = &original.doc
            && back.doc.as_ref() != Some(doc)
        {
            steps.push(GrammarLoweringStep::Doc {
                rule: original.name.clone(),
                doc: doc.clone(),
            });
        }
    }

    // The feature union declarations and rule fields have no form in any
    // target notation: they become steps, and the executable does not honor
    // them.
    if let Some(declarations) = render_declarations_text(grammar.declarations()) {
        steps.push(GrammarLoweringStep::Declarations { declarations });
    }
    for original in grammar.rules() {
        let fields = render_rule_fields(&original.attributes);
        if !fields.is_empty() {
            steps.push(GrammarLoweringStep::Attributes {
                rule: original.name.clone(),
                attributes: fields.join(" "),
            });
        }
    }

    let approximate = steps.iter().any(|step| {
        matches!(
            step,
            GrammarLoweringStep::Helper {
                encoding: GrammarLoweringEncoding::Approximate,
                ..
            } | GrammarLoweringStep::Declarations { .. }
                | GrammarLoweringStep::Attributes { .. }
        )
    });
    let status = if approximate {
        GrammarLoweringStatus::Approximate
    } else {
        GrammarLoweringStatus::Exact
    };
    let metadata = GrammarLoweringMetadata {
        format: format.to_owned(),
        status,
        source: grammar.source_format(),
        start: start.name.clone(),
        order: grammar
            .rule_names()
            .into_iter()
            .map(str::to_owned)
            .collect(),
        steps,
    };
    Ok(GrammarLowering {
        format: format.to_owned(),
        status,
        grammar: executable,
        executable: text,
        report,
        imported,
        metadata: render_lowering_metadata(&metadata),
        steps: metadata.steps,
    })
}

/// `expr` with every reference renamed by `named`.
fn rename_references(expr: &GrammarExpr, named: &impl Fn(&str) -> String) -> GrammarExpr {
    match expr {
        GrammarExpr::NonTerminal(name) => GrammarExpr::NonTerminal(named(name)),
        _ => map_children::<std::convert::Infallible>(expr, &mut |item| {
            Ok(rename_references(item, named))
        })
        .unwrap_or_else(|never| match never {}),
    }
}
