//! Concept-ontology alignment for grammar algebra nodes.

use super::{Grammar, GrammarExpr, GrammarRule};
use crate::link_network::LinkNetwork;

/// A grammar-algebra concept with a stable id and per-format surface forms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GrammarConcept {
    /// Stable language-free concept identifier.
    pub id: &'static str,
    /// Human-readable definition of the grammar construct.
    pub definition: &'static str,
    /// Concrete syntax examples keyed by grammar format.
    pub syntax: &'static [(&'static str, &'static str)],
}

/// Grammar construct concepts aligned with [`GrammarExpr`] variants and rules.
pub const GRAMMAR_CONCEPTS: &[GrammarConcept] = &[
    GrammarConcept {
        id: "grammar.rule",
        definition: "A named grammar production that binds a non-terminal to an expression.",
        syntax: &[
            ("bnf", "::="),
            ("ebnf", "="),
            ("abnf", "="),
            ("peg", "="),
            ("meta-language", "rule"),
        ],
    },
    GrammarConcept {
        id: "grammar.sequence",
        definition: "An ordered concatenation of grammar expressions.",
        syntax: &[
            ("bnf", "a b"),
            ("ebnf", "a , b"),
            ("peg", "a b"),
            ("gbnf", "a b"),
            ("meta-language", "sequence"),
        ],
    },
    GrammarConcept {
        id: "grammar.ordered-choice",
        definition: "A prioritized grammar alternative where earlier matches win.",
        syntax: &[("peg", "/"), ("meta-language", "ordered choice")],
    },
    GrammarConcept {
        id: "grammar.unordered-choice",
        definition: "A grammar alternative whose branches are not semantically prioritized.",
        syntax: &[
            ("bnf", "|"),
            ("ebnf", "|"),
            ("abnf", "/"),
            ("antlr", "|"),
            ("lark", "|"),
            ("meta-language", "choice"),
        ],
    },
    GrammarConcept {
        id: "grammar.repetition",
        definition: "A counted repetition with explicit minimum and optional maximum bounds.",
        syntax: &[
            ("ebnf", "{ }"),
            ("abnf", "m*n"),
            ("gbnf", "{m,n}"),
            ("meta-language", "repeat"),
        ],
    },
    GrammarConcept {
        id: "grammar.zero-or-more",
        definition: "A repetition that accepts zero or more occurrences.",
        syntax: &[
            ("peg", "*"),
            ("ebnf", "{ }"),
            ("antlr", "*"),
            ("lark", "*"),
            ("gbnf", "*"),
        ],
    },
    GrammarConcept {
        id: "grammar.one-or-more",
        definition: "A repetition that accepts one or more occurrences.",
        syntax: &[
            ("peg", "+"),
            ("antlr", "+"),
            ("lark", "+"),
            ("gbnf", "+"),
            ("meta-language", "one or more"),
        ],
    },
    GrammarConcept {
        id: "grammar.optional",
        definition: "An expression that may be present or absent.",
        syntax: &[
            ("ebnf", "[ ]"),
            ("abnf", "[ ]"),
            ("peg", "?"),
            ("antlr", "?"),
            ("lark", "?"),
        ],
    },
    GrammarConcept {
        id: "grammar.terminal",
        definition: "A literal terminal token matched directly in the input.",
        syntax: &[
            ("bnf", "\"lit\""),
            ("ebnf", "\"lit\""),
            ("abnf", "%i\"lit\""),
            ("peg", "\"lit\""),
            ("lark", "\"lit\""),
            ("tree-sitter", "token"),
        ],
    },
    GrammarConcept {
        id: "grammar.non-terminal",
        definition: "A reference to another named grammar rule.",
        syntax: &[
            ("bnf", "<name>"),
            ("ebnf", "name"),
            ("peg", "name"),
            ("antlr", "name"),
            ("lark", "name"),
            ("meta-language", "non-terminal"),
        ],
    },
    GrammarConcept {
        id: "grammar.char-class",
        definition: "A set of characters accepted at one input position.",
        syntax: &[
            ("peg", "[a-z]"),
            ("antlr", "[a-z]"),
            ("lark", "/[a-z]/"),
            ("gbnf", "[^...]"),
            ("meta-language", "character class"),
        ],
    },
    GrammarConcept {
        id: "grammar.char-range",
        definition: "An inclusive range between two character endpoints.",
        syntax: &[
            ("abnf", "%x30-39"),
            ("lark", "\"a\"..\"z\""),
            ("meta-language", "'a'..='z'"),
        ],
    },
    GrammarConcept {
        id: "grammar.any-char",
        definition: "A wildcard grammar expression that accepts any single character.",
        syntax: &[
            ("peg", "."),
            ("lark", "."),
            ("gbnf", "."),
            ("meta-language", "any"),
        ],
    },
    GrammarConcept {
        id: "grammar.positive-predicate",
        definition:
            "A positive lookahead predicate that tests an expression without consuming input.",
        syntax: &[("peg", "&e"), ("meta-language", "and predicate")],
    },
    GrammarConcept {
        id: "grammar.negative-predicate",
        definition: "A negative lookahead predicate that rejects when an expression would match.",
        syntax: &[("peg", "!e"), ("meta-language", "not predicate")],
    },
    GrammarConcept {
        id: "grammar.capture",
        definition: "A labelled or anonymous capture of a grammar subexpression.",
        syntax: &[
            ("meta-language", "name:e"),
            ("antlr", "label=e"),
            ("lark", "name:e"),
        ],
    },
    GrammarConcept {
        id: "grammar.empty",
        definition: "A grammar expression that accepts the empty string.",
        syntax: &[
            ("bnf", "empty alternative"),
            ("ebnf", "empty alternative"),
            ("meta-language", "empty"),
        ],
    },
];

/// Concept id for a [`GrammarExpr`] variant.
#[must_use]
pub const fn grammar_expr_concept_id(expr: &GrammarExpr) -> &'static str {
    match expr {
        GrammarExpr::Empty => "grammar.empty",
        GrammarExpr::Terminal(_) | GrammarExpr::TerminalInsensitive(_) => "grammar.terminal",
        GrammarExpr::CharRange(_, _) => "grammar.char-range",
        GrammarExpr::CharClass { .. } => "grammar.char-class",
        GrammarExpr::AnyChar => "grammar.any-char",
        GrammarExpr::NonTerminal(_) => "grammar.non-terminal",
        GrammarExpr::Choice { ordered: true, .. } => "grammar.ordered-choice",
        GrammarExpr::Choice { ordered: false, .. } => "grammar.unordered-choice",
        GrammarExpr::Sequence(_) => "grammar.sequence",
        GrammarExpr::Optional(_) => "grammar.optional",
        GrammarExpr::ZeroOrMore(_) => "grammar.zero-or-more",
        GrammarExpr::OneOrMore(_) => "grammar.one-or-more",
        GrammarExpr::Repeat { .. } => "grammar.repetition",
        GrammarExpr::And(_) => "grammar.positive-predicate",
        GrammarExpr::Not(_) => "grammar.negative-predicate",
        GrammarExpr::Capture { .. } => "grammar.capture",
    }
}

/// Concept id for a rule: its explicit concept, or its top-level expression concept.
#[must_use]
pub fn rule_concept_id(rule: &GrammarRule) -> Option<&str> {
    rule.concept()
        .map_or_else(|| Some(grammar_expr_concept_id(rule.expr())), Some)
}

/// Fills missing rule-level concept alignments from each rule's top-level expression.
pub fn annotate_grammar_concepts(grammar: &mut Grammar) {
    for rule in &mut grammar.rules {
        if rule.concept.is_none() {
            rule.concept = Some(grammar_expr_concept_id(&rule.expr).to_owned());
        }
    }
}

impl LinkNetwork {
    /// Seeds only the grammar-construct concept ontology layer.
    #[must_use]
    pub fn seed_grammar_concept_ontology(&mut self) -> usize {
        for concept in GRAMMAR_CONCEPTS {
            let concept_link = self.intern_concept(concept.id, Some(concept.definition));

            for (language, syntax) in concept.syntax {
                self.insert_concept_syntax_mapping(
                    concept_link,
                    concept.id,
                    language,
                    syntax,
                    true,
                );
            }
        }

        GRAMMAR_CONCEPTS.len()
    }
}
