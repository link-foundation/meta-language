//! The native executor of the grammar feature union, the Rust port of
//! `js/src/grammar-runtime.js`.
//!
//! Compile a grammar once, then parse sources into lossless concrete syntax
//! trees. Loading (`load.rs`, `compile.rs`)
//! resolves imports, macros and parameterized rules and checks every
//! scanner, action and predicate; the executor (`executor.rs`, `rules.rs`)
//! runs it within explicit resource limits; `tree.rs` copies and renders the
//! result. docs/grammar/feature-union.md is the specification both
//! runtimes follow.

mod compile;
mod executor;
mod lexing;
mod load;
mod operations;
mod ordering;
mod precedence;
mod program;
mod results;
mod rules;
mod text;
mod tree;

use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::fmt;

pub use load::GrammarRuntimeError;
pub use operations::OperationValue;
pub use tree::{Ambiguity, LeafText, SyntaxAttributes, SyntaxTree};

use super::Grammar;
use executor::Executor;
use operations::Abort;
use program::Compiled;
use results::{KeywordLexing, Outcome, Shared, TokenOrder};

/// The default bound on nested rule calls.
const DEFAULT_MAX_DEPTH: usize = 1000;
/// The default bound on memoized rule calls.
const DEFAULT_MEMO_LIMIT: usize = 1_000_000;
/// The stack of the thread a parse runs on; the executor's frame bound keeps
/// a parse well inside it.
const PARSE_STACK: usize = 256 * 1024 * 1024;

/// The limits and policies of a parse. Every field left `None` keeps the
/// value the parser was compiled with, and then the default.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FeatureParseOptions {
    /// The rule to parse from instead of the grammar's start rule.
    pub start_rule: Option<String>,
    /// The bound on nested rule calls (default 1000).
    pub max_depth: Option<usize>,
    /// The bound on evaluation steps (default `100000 + 1000 * input length`).
    pub step_limit: Option<usize>,
    /// The bound on memoized rule calls (default 1000000).
    pub memo_limit: Option<usize>,
    /// Whether a reported ambiguity rejects the input (default false).
    pub reject_ambiguity: Option<bool>,
    /// Whether a tree with ERROR or MISSING nodes is accepted (default false).
    pub accept_recovery: Option<bool>,
    /// Whether a failed parse is repaired into a tree with ERROR or MISSING
    /// nodes instead of rejected without a tree (default false).
    pub error_recovery: Option<bool>,
    /// The bound on repair points of automatic recovery (default 32).
    pub max_repairs: Option<usize>,
}

impl FeatureParseOptions {
    fn over(&self, base: &Self) -> Self {
        Self {
            start_rule: self.start_rule.clone().or_else(|| base.start_rule.clone()),
            max_depth: self.max_depth.or(base.max_depth),
            step_limit: self.step_limit.or(base.step_limit),
            memo_limit: self.memo_limit.or(base.memo_limit),
            reject_ambiguity: self.reject_ambiguity.or(base.reject_ambiguity),
            accept_recovery: self.accept_recovery.or(base.accept_recovery),
            error_recovery: self.error_recovery.or(base.error_recovery),
            max_repairs: self.max_repairs.or(base.max_repairs),
        }
    }
}

/// Why a parse rejects its input, and where.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseRejection {
    /// `syntax`, `nestingDepth`, `stepLimit`, `recovered` or `ambiguity`.
    pub reason: &'static str,
    /// The byte offset, for a positioned rejection.
    pub offset: Option<usize>,
    /// The 1-based line of the offset.
    pub line: Option<usize>,
    /// The 1-based column of the offset, in code points.
    pub column: Option<usize>,
    /// What a syntax error expected at the offset, sorted.
    pub expected: Option<Vec<String>>,
    /// The limit a resource rejection reached.
    pub limit: Option<usize>,
}

impl ParseRejection {
    const fn limited(reason: &'static str, limit: usize) -> Self {
        Self {
            reason,
            offset: None,
            line: None,
            column: None,
            expected: None,
            limit: Some(limit),
        }
    }

    fn positioned(reason: &'static str, bytes: &[u8], offset: usize) -> Self {
        let (line, column) = text::line_and_column(bytes, offset);
        Self {
            reason,
            offset: Some(offset),
            line: Some(line),
            column: Some(column),
            expected: None,
            limit: None,
        }
    }
}

impl fmt::Display for ParseRejection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let place = match (self.line, self.column) {
            (Some(line), Some(column)) => format!(" at line {line} column {column}"),
            _ => String::new(),
        };
        let limit = self.limit.unwrap_or_default();
        match self.reason {
            "syntax" => {
                let expected = self.expected.as_deref().unwrap_or_default();
                let expected = if expected.is_empty() {
                    "nothing more".to_owned()
                } else {
                    expected.join(", ")
                };
                write!(formatter, "syntax error{place}: expected {expected}")
            }
            "nestingDepth" => write!(formatter, "the input nests deeper than {limit} rules"),
            "stepLimit" => write!(formatter, "the parse needed more than {limit} steps"),
            "recovered" => write!(formatter, "the input needed error recovery{place}"),
            "ambiguity" => write!(formatter, "the input is ambiguous{place}"),
            reason => write!(formatter, "the input is rejected ({reason})"),
        }
    }
}

/// The outcome of one parse: `ok` only without a rejection; the tree is kept
/// whenever one was built.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseOutcome {
    /// Whether the input is accepted.
    pub ok: bool,
    /// The concrete syntax tree, when one was built.
    pub tree: Option<SyntaxTree>,
    /// The ambiguous nodes outside the declared conflicts.
    pub ambiguities: Vec<Ambiguity>,
    /// Why the input is rejected.
    pub rejection: Option<ParseRejection>,
}

/// A source the grammar rejects.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarParseError {
    /// The reason and position.
    pub rejection: ParseRejection,
}

impl fmt::Display for GrammarParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rejection.fmt(formatter)
    }
}

impl std::error::Error for GrammarParseError {}

/// Returns the grammar an import or an embedded language names.
pub type GrammarResolver<'a> = &'a dyn Fn(&str) -> Option<Grammar>;

/// A grammar compiled for the native executor.
#[derive(Debug)]
pub struct FeatureGrammarParser {
    compiled: Compiled,
    options: FeatureParseOptions,
}

/// Compiles `grammar` for the native executor. `resolve_grammar(name)`
/// returns the grammar an import or an embedded language names; `options`
/// are the defaults of every parse.
///
/// # Errors
///
/// A [`GrammarRuntimeError`] when an import, a macro, a parameterized rule,
/// an operation, a mode, a Unicode property or an embedded language does not
/// load; its `reason` names which.
pub fn compile_feature_grammar(
    grammar: &Grammar,
    resolve_grammar: Option<GrammarResolver<'_>>,
    options: FeatureParseOptions,
) -> Result<FeatureGrammarParser, GrammarRuntimeError> {
    Ok(FeatureGrammarParser {
        compiled: load::load(grammar, resolve_grammar)?,
        options,
    })
}

impl FeatureGrammarParser {
    /// Parses `source` into an outcome that keeps the tree, the ambiguities
    /// and the rejection.
    ///
    /// # Errors
    ///
    /// A [`GrammarRuntimeError`] with reason `reference` when the start rule
    /// is not a rule of the grammar.
    pub fn parse_tree(
        &self,
        source: &[u8],
        options: &FeatureParseOptions,
    ) -> Result<ParseOutcome, GrammarRuntimeError> {
        let options = options.over(&self.options);
        let program = &self.compiled.programs[self.compiled.main];
        let start_name = options
            .start_rule
            .clone()
            .or_else(|| program.start.clone())
            .unwrap_or_default();
        let Some(&start_rule) = program.rule_index.get(&start_name) else {
            return Err(GrammarRuntimeError {
                reason: "reference",
                message: format!("undefined start rule {start_name}"),
            });
        };
        let compiled = &self.compiled;
        Ok(std::thread::scope(|scope| {
            std::thread::Builder::new()
                .stack_size(PARSE_STACK)
                .spawn_scoped(scope, || {
                    parse_program(compiled, start_rule, source, &options)
                })
                .map_or_else(
                    |_| parse_program(compiled, start_rule, source, &options),
                    |handle| {
                        handle
                            .join()
                            .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
                    },
                )
        }))
    }

    /// Parses `source` into its concrete syntax tree.
    ///
    /// # Errors
    ///
    /// A [`GrammarParseError`] when the input is rejected; a rejected start
    /// rule is reported as a syntax rejection at offset 0.
    pub fn parse(
        &self,
        source: &[u8],
        options: &FeatureParseOptions,
    ) -> Result<SyntaxTree, GrammarParseError> {
        let outcome = self
            .parse_tree(source, options)
            .map_err(|_| GrammarParseError {
                rejection: ParseRejection::positioned("syntax", source, 0),
            })?;
        match (outcome.rejection, outcome.tree) {
            (None, Some(tree)) => Ok(tree),
            (rejection, _) => Err(GrammarParseError {
                rejection: rejection
                    .unwrap_or_else(|| ParseRejection::positioned("syntax", source, 0)),
            }),
        }
    }
}

/// The default bound on repair points of automatic recovery.
const DEFAULT_MAX_REPAIRS: usize = 32;

/// Automatic error recovery after a failed parse: each round reparses with
/// one more repair point, the farthest offset where an element failed without
/// a repair, until a parse completes. After `max_repairs` rounds, or when no
/// new point appears, the last round's partial tree stands, the rest of the
/// input an ERROR leaf. Each round has its own step budget.
fn repair_parse(
    failed: Outcome,
    max_repairs: usize,
    attempt: impl Fn(Option<&HashSet<usize>>) -> Result<Outcome, Abort>,
) -> Result<Outcome, Abort> {
    let mut points = HashSet::new();
    let mut outcome = failed;
    while points.len() < max_repairs {
        let Outcome::Failed {
            farthest,
            element_farthest,
            ..
        } = &outcome
        else {
            break;
        };
        if !points.insert(element_farthest.unwrap_or(*farthest)) {
            break;
        }
        outcome = attempt(Some(&points))?;
        if matches!(outcome, Outcome::Parsed(_)) {
            return Ok(outcome);
        }
    }
    Ok(match outcome {
        Outcome::Failed {
            partial: Some(partial),
            ..
        } => Outcome::Parsed(partial),
        outcome => outcome,
    })
}

fn parse_program(
    compiled: &Compiled,
    start_rule: usize,
    bytes: &[u8],
    options: &FeatureParseOptions,
) -> ParseOutcome {
    let max_depth = options.max_depth.unwrap_or(DEFAULT_MAX_DEPTH);
    let limit = options
        .step_limit
        .unwrap_or_else(|| 100_000 + 1000 * bytes.len());
    let refused = |rejection, tree| ParseOutcome {
        ok: false,
        tree,
        ambiguities: Vec::new(),
        rejection: Some(rejection),
    };
    // Under `(matching longest)` the input is parsed again while the tree
    // takes a token rule's leaf where a keyword a lexer prefers matched.
    let tokens = compiled.programs[compiled.main]
        .token_ranks
        .as_ref()
        .map(|ranks| TokenOrder {
            ranks,
            bytes,
            orders: &compiled.programs[compiled.main].precedence_orders,
        });
    let keywords = tokens.map(|_| RefCell::new(KeywordLexing::default()));
    let attempt = |points: Option<&HashSet<usize>>| {
        let shared = Shared {
            steps: Cell::new(0),
            limit,
            frames: Cell::new(0),
            memo_limit: options.memo_limit.unwrap_or(DEFAULT_MEMO_LIMIT),
        };
        let mut executor = Executor::new(
            compiled,
            compiled.main,
            bytes,
            0,
            bytes.len(),
            &shared,
            max_depth,
        );
        executor.repair_points = points.cloned();
        executor.keywords = keywords.as_ref();
        executor.run(start_rule)
    };
    let recovery = options.error_recovery == Some(true);
    let outcome = loop {
        // With no repair point yet, recovery only notes where elements fail.
        let outcome =
            attempt(recovery.then(HashSet::new).as_ref()).and_then(|outcome| match outcome {
                Outcome::Failed { .. } if recovery => repair_parse(
                    outcome,
                    options.max_repairs.unwrap_or(DEFAULT_MAX_REPAIRS),
                    attempt,
                ),
                outcome => Ok(outcome),
            });
        if let (Ok(Outcome::Parsed(root)), Some(keywords), Some(tokens)) =
            (&outcome, &keywords, tokens)
            && keywords.borrow_mut().conflicts(root, tokens)
        {
            continue;
        }
        break outcome;
    };
    let root = match outcome {
        Err(Abort::StepLimit) => {
            return refused(ParseRejection::limited("stepLimit", limit), None);
        }
        Err(Abort::NestingTooDeep) => {
            let tree = tree::error_tree(bytes, 0, bytes.len(), Some("nestingDepth"));
            return refused(
                ParseRejection::limited("nestingDepth", max_depth),
                Some(tree),
            );
        }
        Ok(Outcome::Failed {
            farthest, expected, ..
        }) => {
            let mut rejection = ParseRejection::positioned("syntax", bytes, farthest);
            rejection.expected = Some(expected);
            return refused(rejection, None);
        }
        Ok(Outcome::Parsed(root)) => root,
    };
    let tree = tree::public_tree(&root, bytes);
    let mut ambiguities = Vec::new();
    tree::collect_ambiguities(&root, compiled, compiled.main, &mut ambiguities);
    let recovered = tree
        .first_recovery()
        .filter(|_| options.accept_recovery != Some(true))
        .map(|recovery| ParseRejection::positioned("recovered", bytes, recovery.start()));
    let rejection = recovered.or_else(|| {
        ambiguities
            .first()
            .filter(|_| options.reject_ambiguity == Some(true))
            .map(|first| ParseRejection::positioned("ambiguity", bytes, first.start))
    });
    ParseOutcome {
        ok: rejection.is_none(),
        tree: Some(tree),
        ambiguities,
        rejection,
    }
}
