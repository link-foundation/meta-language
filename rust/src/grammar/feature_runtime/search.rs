//! Grammar search over the original UTF-8 source.

use super::search_policy::{inside_grammar_search_word, skip_grammar_search_start};
use std::cell::Cell;
use std::collections::BTreeMap;

use super::{
    FeatureGrammarParser, GrammarRuntimeError, PARSE_STACK, SyntaxTree, decorate_syntax_tree,
    decorator_error, parse_program,
};
use crate::LinkNetwork;
use crate::link_network::text_location::{CharacterClass, character_class};
use crate::query::{QueryCaptures, QueryPredicate, QueryPredicateHost};
use crate::transform::{SourceTextPredicateHost, captured_text};

/// How search chooses among successful results at one source position.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GrammarMatchPolicy {
    /// Choose the farthest end among the results admitted by the grammar.
    #[default]
    LeftmostLongest,
    /// Choose the first result admitted by the grammar.
    PegFirst,
}

/// Options for scanning text with a compiled grammar.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GrammarSearchOptions {
    /// Rule to scan with. The parser's selected start rule is the default.
    pub rule: Option<String>,
    /// Also test starts within a previous match.
    pub overlapping: bool,
    /// Refuse matches which start or end inside a Unicode word.
    pub word_boundaries: bool,
    /// Selection among multiple ends at one start position.
    pub match_policy: GrammarMatchPolicy,
    /// CST node kinds whose source text and spans should be returned.
    pub captures: Vec<String>,
}

/// A captured part of the original source, using UTF-8 byte offsets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarSearchSpan {
    /// First byte of the capture.
    pub start: usize,
    /// Byte after the capture.
    pub end: usize,
    /// Exact original spelling.
    pub text: String,
}

/// One nonempty recognition of the selected rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarSearchMatch {
    /// Exact original extent of the recognition.
    pub span: GrammarSearchSpan,
    /// Concrete syntax with the original source's coordinates.
    pub tree: SyntaxTree,
    /// Requested CST kinds in tree order, with repeated captures retained.
    pub captures: BTreeMap<String, Vec<GrammarSearchSpan>>,
    /// Source registry for mapped embedded syntax.
    pub source_planes: Option<Vec<serde_json::Value>>,
}

/// A predicate host for `#grammar-match?` and standard source text predicates.
pub struct GrammarSearchPredicate<'a> {
    parser: &'a FeatureGrammarParser,
    options: GrammarSearchOptions,
}

impl QueryPredicateHost for GrammarSearchPredicate<'_> {
    fn evaluate(
        &self,
        predicate: &QueryPredicate,
        captures: &QueryCaptures,
        network: &LinkNetwork,
    ) -> bool {
        if predicate.name() != "grammar-match?" {
            return SourceTextPredicateHost.evaluate(predicate, captures, network);
        }
        let [argument] = predicate.arguments() else {
            return false;
        };
        let Some(name) = argument.capture_name() else {
            return false;
        };
        captured_text(network, captures.first(name)).is_some_and(|text| {
            self.parser
                .find_all(&text, &self.options)
                .is_ok_and(|matches| !matches.is_empty())
        })
    }
}

fn is_word_character(character: char) -> bool {
    matches!(
        character_class(character),
        Some(CharacterClass::Word | CharacterClass::Connector)
    )
}

pub(super) struct SearchAttempt<'a> {
    source: &'a str,
    word_boundaries: bool,
    pub(super) begin: usize,
    pub(super) match_policy: GrammarMatchPolicy,
    pub(super) steps: Cell<usize>,
}

impl SearchAttempt<'_> {
    pub(super) fn accepts_boundary(&self, position: usize) -> bool {
        self.source.is_char_boundary(position)
            && (!self.word_boundaries
                || !inside_grammar_search_word(
                    self.source[..position]
                        .chars()
                        .next_back()
                        .is_some_and(is_word_character),
                    self.source[position..]
                        .chars()
                        .next()
                        .is_some_and(is_word_character),
                ))
    }
}

fn search_span(source: &str, start: usize, end: usize) -> GrammarSearchSpan {
    GrammarSearchSpan {
        start,
        end,
        text: source[start..end].to_owned(),
    }
}

fn search_captures(
    tree: &SyntaxTree,
    source: &str,
    names: &[String],
) -> BTreeMap<String, Vec<GrammarSearchSpan>> {
    let mut captures: BTreeMap<String, Vec<GrammarSearchSpan>> = names
        .iter()
        .cloned()
        .map(|name| (name, Vec::new()))
        .collect();
    let mut pending = vec![tree];
    while let Some(node) = pending.pop() {
        let name = match node {
            SyntaxTree::Node { kind, children, .. } => {
                pending.extend(children.iter().rev());
                Some(kind)
            }
            SyntaxTree::Token { kind, .. } => kind.as_ref(),
            SyntaxTree::Embed {
                source_children,
                root,
                ..
            } => {
                if let Some(children) = source_children {
                    pending.extend(children.iter().rev());
                } else {
                    pending.push(root);
                }
                None
            }
            SyntaxTree::Missing { .. } | SyntaxTree::Error { .. } => None,
        };
        let (start, end) = (node.start(), node.end());
        if let Some(found) = name.and_then(|name| captures.get_mut(name))
            && source.get(start..end).is_some()
        {
            found.push(search_span(source, start, end));
        }
    }
    captures
}

impl FeatureGrammarParser {
    /// Build a predicate host which runs grammar search on the captured source.
    #[must_use]
    pub const fn search_predicate(
        &self,
        options: GrammarSearchOptions,
    ) -> GrammarSearchPredicate<'_> {
        GrammarSearchPredicate {
            parser: self,
            options,
        }
    }

    /// Scan original text using an existing compiled grammar.
    ///
    /// Empty results are ignored. A candidate inside a word can be rejected
    /// independently at both endpoints. Search uses the parser's resource
    /// limits, with one step budget for the scan.
    ///
    /// # Errors
    ///
    /// Returns a reference error for an unknown rule or the first execution
    /// error encountered during the scan.
    pub fn find_all(
        &self,
        source: &str,
        options: &GrammarSearchOptions,
    ) -> Result<Vec<GrammarSearchMatch>, GrammarRuntimeError> {
        let program = &self.compiled.programs[self.compiled.main];
        let rule = options
            .rule
            .as_ref()
            .or(self.options.start_rule.as_ref())
            .or(program.start.as_ref());
        let name = rule.map_or("", String::as_str);
        let Some(&start_rule) = program.rule_index.get(name) else {
            return Err(GrammarRuntimeError {
                reason: "reference",
                message: format!("undefined start rule {name}"),
            });
        };
        let parse_options = super::FeatureParseOptions {
            error_recovery: Some(false),
            ..self.options.clone()
        };
        let bytes = source.as_bytes();
        let scan = || {
            let mut attempt = SearchAttempt {
                source,
                word_boundaries: options.word_boundaries,
                begin: 0,
                match_policy: options.match_policy,
                steps: Cell::new(0),
            };
            let mut matches = Vec::new();
            let mut next = 0;
            for (begin, _) in source.char_indices() {
                if skip_grammar_search_start(begin < next, options.overlapping)
                    || !attempt.accepts_boundary(begin)
                {
                    continue;
                }
                attempt.begin = begin;
                let outcome = parse_program(
                    &self.compiled,
                    start_rule,
                    bytes,
                    &parse_options,
                    Some(&attempt),
                )?;
                if let Some(rejection) = &outcome.rejection {
                    if matches!(
                        rejection.reason,
                        "stepLimit" | "memoryBudget" | "nestingDepth"
                    ) {
                        return Err(GrammarRuntimeError {
                            reason: rejection.reason,
                            message: rejection.to_string(),
                        });
                    }
                    continue;
                }
                let Some(tree) = outcome.tree else { continue };
                let end = tree.end();
                let tree = decorate_syntax_tree(&tree, &parse_options.decorators, bytes)
                    .map_err(decorator_error)?;
                matches.push(GrammarSearchMatch {
                    span: search_span(source, begin, end),
                    captures: search_captures(&tree, source, &options.captures),
                    tree,
                    source_planes: outcome.source_planes,
                });
                next = end;
            }
            Ok(matches)
        };
        std::thread::scope(|scope| {
            std::thread::Builder::new()
                .stack_size(PARSE_STACK)
                .spawn_scoped(scope, scan)
                .map_or_else(
                    |_| scan(),
                    |handle| {
                        handle
                            .join()
                            .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
                    },
                )
        })
    }
}
