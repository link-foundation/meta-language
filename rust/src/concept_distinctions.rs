//! Which concepts are shared across languages, and which must stay distinct.
//!
//! Two source spellings name one concept only under a one-to-one
//! correspondence of meaning: each spelling resolves to exactly one concept
//! record or foundation model, and it is the same one. The same spelling with
//! different meanings, such as `|` in pest (ordered choice) and in BNF
//! (unordered choice), or `panic!` in Rust (abort) and in Lean (a default
//! value), never merges. Ordered and unordered choice, lexical and syntactic
//! precedence, binding and assignment, and the integer, overflow, effect,
//! universe, proof and logic models the foundation register keeps apart are
//! required distinctions the check below enforces. The JavaScript twin is
//! `js/src/concept-distinctions.js`.

use std::collections::BTreeSet;

use crate::concept_records::{ConceptRecord, SourceAlias, concept_records};
use crate::foundation_models::{FoundationRegister, foundation_register};
use crate::grammar::{Grammar, GrammarExpr, RuleKind};

/// Two concepts that must never be merged, with the reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequiredDistinction {
    /// The identities of the two concepts.
    pub concepts: [&'static str; 2],
    /// What distinguishes them.
    pub reason: &'static str,
}

/// The concept pairs that stay distinct in every grammar and program.
pub const REQUIRED_CONCEPT_DISTINCTIONS: &[RequiredDistinction] = &[
    RequiredDistinction {
        concepts: ["grammar.ordered-choice", "grammar.unordered-choice"],
        reason: "An ordered choice commits to the first alternative that matches, while an unordered choice accepts every alternative that matches, so the two accept different languages.",
    },
    RequiredDistinction {
        concepts: ["grammar.lexical-precedence", "grammar.syntactic-precedence"],
        reason: "Lexical precedence decides which token the lexer produces, while syntactic precedence decides between parse alternatives over tokens already produced.",
    },
    RequiredDistinction {
        concepts: ["binding", "assignment"],
        reason: "A binding introduces a name for a value in a scope, while an assignment updates the value stored in a location that already exists.",
    },
    RequiredDistinction {
        concepts: ["grammar.list", "grammar.linked-list"],
        reason: "A list rule is a delimited sequence of items, while a linked list is built from pairs and may end in a dotted pair, so a linked list has no counterpart among plain sequences.",
    },
    RequiredDistinction {
        concepts: ["grammar.string", "grammar.symbol"],
        reason: "A string literal denotes text, while a symbol denotes an interned identifier that compares by identity, so the same characters read as different data.",
    },
    RequiredDistinction {
        concepts: ["grammar.member", "grammar.ini.setting"],
        reason: "An object member pairs a name with any nested value, while an INI setting is one key and value line of a section whose value is raw text.",
    },
    RequiredDistinction {
        concepts: ["grammar.object", "grammar.racket.hash-table"],
        reason: "An object maps names to values with no stated key comparison, while a Racket hash table maps any datum keys under its equality, so the two key sets differ.",
    },
    RequiredDistinction {
        concepts: ["grammar.value", "grammar.datum"],
        reason: "A value of a data grammar is data only, while a datum is the external representation of a program value, which the reader may also read as code.",
    },
    RequiredDistinction {
        concepts: ["grammar.identifier", "grammar.identifier-name"],
        reason: "An identifier rule accepts letters, digits and underscores that start with a letter, while an ECMAScript identifier name also accepts dollar signs, Unicode letters and escapes.",
    },
    RequiredDistinction {
        concepts: ["grammar.document", "grammar.program"],
        reason: "A document is the whole text of a data or document format, while a program is source text the reader reads as code.",
    },
    RequiredDistinction {
        concepts: ["grammar.boolean-value", "grammar.true-value"],
        reason: "A boolean value rule accepts both truth values, while a true value rule accepts the literal true alone, so one cannot replace the other.",
    },
    RequiredDistinction {
        concepts: ["grammar.program", "grammar.module"],
        reason: "A program is source text the reader reads as code as a whole, while a TypeScript module declaration is one statement that opens a namespace inside a program.",
    },
    RequiredDistinction {
        concepts: ["grammar.true-constant", "grammar.true-value"],
        reason: "A Lean true constant accepts the proposition True as well as the Boolean literal true, while a true value rule accepts the literal true alone.",
    },
    RequiredDistinction {
        concepts: ["grammar.symbol", "grammar.keyword"],
        reason: "A symbol may name a variable, while a keyword evaluates to itself and is never a variable.",
    },
    RequiredDistinction {
        concepts: ["grammar.comment", "grammar.block-comment"],
        reason: "A comment rule covers every comment its format allows, while a block comment runs between nesting delimiters, so it does not end at the end of its line.",
    },
];

/// The foundation model pairs that stay distinct; the register records the reason for each.
pub const REQUIRED_FOUNDATION_DISTINCTIONS: &[[&str; 2]] = &[
    [
        "integer-model.natural-number",
        "integer-model.unbounded-integer",
    ],
    ["overflow-model.abort", "overflow-model.wrapping"],
    [
        "effect-model.abort",
        "effect-model.panic-with-default-value",
    ],
    [
        "universe-model.non-cumulative-sort-hierarchy",
        "universe-model.cumulative-type-hierarchy",
    ],
    [
        "proof-system.bounded-property-check",
        "proof-system.lean-kernel",
    ],
    [
        "logic-model.constructive-propositions",
        "logic-model.classical-propositions",
    ],
];

/// How two source spellings relate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorrespondenceRelation {
    /// Both spellings mean exactly one concept, and it is the same concept.
    Shared,
    /// Both spellings mean exactly one concept, and the concepts differ.
    Distinct,
    /// A spelling means more than one concept, so no correspondence follows from the spelling alone.
    Ambiguous,
    /// A spelling means no recorded concept.
    Unknown,
}

impl CorrespondenceRelation {
    /// The relation's name, the same string the JavaScript runtime reports.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shared => "shared",
            Self::Distinct => "distinct",
            Self::Ambiguous => "ambiguous",
            Self::Unknown => "unknown",
        }
    }
}

/// The relation between two source spellings and what justifies it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConceptCorrespondence {
    /// How the two spellings relate.
    pub relation: CorrespondenceRelation,
    /// The concepts and models the first spelling means.
    pub first: Vec<String>,
    /// The concepts and models the second spelling means.
    pub second: Vec<String>,
    /// The concept both spellings mean, when the relation is shared.
    pub shared: Option<String>,
    /// The definition of the shared concept, or the recorded reason two distinct concepts differ.
    pub justification: Option<String>,
    /// The kind of the recorded correspondence between two distinct foundation models.
    pub correspondence: Option<String>,
}

/// A way the records fail to keep required distinctions or justify a shared concept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConceptDistinctionProblem {
    /// The problem kind, the same string the JavaScript runtime reports, such as `indistinct-concepts`.
    pub kind: &'static str,
    /// The concept, model or pair the problem is about.
    pub subject: String,
    /// A readable explanation.
    pub message: String,
}

/// One use of precedence in a grammar and the concept it expresses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrecedenceUse {
    /// The rule the precedence appears in.
    pub rule: String,
    /// The precedence label, such as `prec=1` or `prec_left=2`.
    pub label: String,
    /// `grammar.lexical-precedence` inside a token, `grammar.syntactic-precedence` elsewhere.
    pub concept: &'static str,
}

fn within_matches(id: &str, within: Option<&str>) -> bool {
    within.is_none_or(|prefix| {
        id == prefix
            || id
                .strip_prefix(prefix)
                .is_some_and(|rest| rest.starts_with('.'))
    })
}

/// Returns the concepts and foundation models a source spelling means, optionally only those under `within`.
///
/// A concept record matches an alias with the same source and name; a
/// foundation model matches when its alias for the source lists the name among
/// its comma-separated spellings.
#[must_use]
pub fn source_meanings_in(
    records: &[ConceptRecord],
    register: &FoundationRegister,
    alias: &SourceAlias,
    within: Option<&str>,
) -> Vec<String> {
    let concepts = records.iter().filter(|record| {
        record
            .source_aliases
            .iter()
            .any(|candidate| candidate.source == alias.source && candidate.name == alias.name)
    });
    let models = register.models.iter().filter(|model| {
        model.source_aliases.iter().any(|candidate| {
            candidate.source == alias.source
                && candidate
                    .name
                    .split(", ")
                    .any(|spelling| spelling == alias.name)
        })
    });
    concepts
        .map(|record| record.id.clone())
        .chain(models.map(|model| model.id.clone()))
        .filter(|id| within_matches(id, within))
        .collect()
}

fn same_pair(pair: &[impl AsRef<str>], first: &str, second: &str) -> bool {
    pair.len() == 2
        && ((pair[0].as_ref() == first && pair[1].as_ref() == second)
            || (pair[0].as_ref() == second && pair[1].as_ref() == first))
}

fn meaning_definition(
    records: &[ConceptRecord],
    register: &FoundationRegister,
    id: &str,
) -> Option<String> {
    records
        .iter()
        .find(|record| record.id == id)
        .map(|record| record.definition.clone())
        .or_else(|| {
            register
                .models
                .iter()
                .find(|model| model.id == id)
                .map(|model| model.definition.clone())
        })
}

/// Relates two source spellings through the given records and foundation register.
#[must_use]
pub fn concept_correspondence_in(
    records: &[ConceptRecord],
    register: &FoundationRegister,
    first: &SourceAlias,
    second: &SourceAlias,
    within: Option<&str>,
) -> ConceptCorrespondence {
    let first_meanings = source_meanings_in(records, register, first, within);
    let second_meanings = source_meanings_in(records, register, second, within);
    let mut result = ConceptCorrespondence {
        relation: CorrespondenceRelation::Unknown,
        first: first_meanings.clone(),
        second: second_meanings.clone(),
        shared: None,
        justification: None,
        correspondence: None,
    };
    if first_meanings.is_empty() || second_meanings.is_empty() {
        return result;
    }
    if first_meanings.len() > 1 || second_meanings.len() > 1 {
        result.relation = CorrespondenceRelation::Ambiguous;
        return result;
    }
    let (one, other) = (&first_meanings[0], &second_meanings[0]);
    if one == other {
        result.relation = CorrespondenceRelation::Shared;
        result.shared = Some(one.clone());
        result.justification = meaning_definition(records, register, one);
        return result;
    }
    result.relation = CorrespondenceRelation::Distinct;
    result.justification = REQUIRED_CONCEPT_DISTINCTIONS
        .iter()
        .find(|entry| same_pair(&entry.concepts, one, other))
        .map(|entry| entry.reason.to_owned())
        .or_else(|| {
            register
                .distinctions
                .iter()
                .find(|entry| same_pair(&entry.models, one, other))
                .map(|entry| entry.reason.clone())
        });
    result.correspondence = register
        .correspondences
        .iter()
        .find(|entry| same_pair(&[&entry.from, &entry.to], one, other))
        .map(|entry| entry.kind.clone());
    result
}

/// Relates two source spellings through the embedded concept records and foundation register.
#[must_use]
pub fn concept_correspondence(
    first: &SourceAlias,
    second: &SourceAlias,
    within: Option<&str>,
) -> ConceptCorrespondence {
    concept_correspondence_in(
        concept_records(),
        foundation_register(),
        first,
        second,
        within,
    )
}

fn shared_sources(aliases: &[SourceAlias]) -> usize {
    aliases
        .iter()
        .map(|alias| alias.source.as_str())
        .filter(|source| *source != "meta-language" && !source.ends_with(" grammar surface"))
        .collect::<BTreeSet<_>>()
        .len()
}

/// A justification is a sentence of at least three words.
fn is_justification(text: &str) -> bool {
    let text = text.trim();
    text.ends_with('.') && text.split_whitespace().count() >= 3
}

/// Checks that the records keep every required distinction and justify every shared concept.
///
/// A required pair must name two recorded concepts (or models) with different
/// definitions (or properties) that do not record one another as a former name
/// or as what they represent, and every required foundation pair must carry a
/// distinction with its reason in the register. A concept or model several
/// sources share must state the meaning that justifies sharing it. The kinds
/// match the JavaScript runtime's `checkConceptDistinctions`; an empty result
/// means the distinctions hold.
#[must_use]
pub fn check_concept_distinctions(
    records: &[ConceptRecord],
    register: &FoundationRegister,
) -> Vec<ConceptDistinctionProblem> {
    let mut problems = Vec::new();
    let mut report = |kind, subject: &str, message: String| {
        problems.push(ConceptDistinctionProblem {
            kind,
            subject: subject.to_owned(),
            message,
        });
    };
    for entry in REQUIRED_CONCEPT_DISTINCTIONS {
        let subject = entry.concepts.join(" / ");
        let found: Vec<_> = entry
            .concepts
            .iter()
            .map(|id| records.iter().find(|record| record.id == *id))
            .collect();
        for (id, record) in entry.concepts.iter().zip(&found) {
            if record.is_none() {
                report(
                    "unknown-concept",
                    &subject,
                    format!("{subject} must stay distinct, but {id} is not recorded"),
                );
            }
        }
        let [Some(first), Some(second)] = found[..] else {
            continue;
        };
        if first.definition.trim().to_lowercase() == second.definition.trim().to_lowercase() {
            report(
                "indistinct-concepts",
                &subject,
                format!("{subject} record the same definition, which merges two meanings"),
            );
        }
        for (record, other) in [(first, second), (second, first)] {
            if record.former_names.contains(&other.id)
                || record.represents.as_deref() == Some(other.id.as_str())
            {
                report(
                    "conflated-distinction",
                    &subject,
                    format!(
                        "{} records {} as itself, which merges two meanings",
                        record.id, other.id
                    ),
                );
            }
        }
    }
    for pair in REQUIRED_FOUNDATION_DISTINCTIONS {
        let subject = pair.join(" / ");
        let found: Vec<_> = pair
            .iter()
            .map(|id| register.models.iter().find(|model| model.id == *id))
            .collect();
        for (id, model) in pair.iter().zip(&found) {
            if model.is_none() {
                report(
                    "unknown-model",
                    &subject,
                    format!("{subject} must stay distinct, but {id} is not recorded"),
                );
            }
        }
        if let [Some(first), Some(second)] = found[..]
            && first.properties == second.properties
        {
            report(
                "indistinct-concepts",
                &subject,
                format!("{subject} record the same properties, which merges two meanings"),
            );
        }
        let recorded = register.distinctions.iter().any(|entry| {
            same_pair(&entry.models, pair[0], pair[1]) && is_justification(&entry.reason)
        });
        if !recorded {
            report(
                "unrecorded-distinction",
                &subject,
                format!("the foundation register records no reason why {subject} differ"),
            );
        }
    }
    for record in records {
        if shared_sources(&record.source_aliases) > 1
            && (!is_justification(&record.definition) || record.constraints.is_empty())
        {
            report(
                "unjustified-sharing",
                &record.id,
                format!(
                    "{} is shared by several sources but states no meaning that justifies the correspondence",
                    record.id
                ),
            );
        }
    }
    for model in &register.models {
        if shared_sources(&model.source_aliases) > 1
            && (!is_justification(&model.definition) || model.properties.is_empty())
        {
            report(
                "unjustified-sharing",
                &model.id,
                format!(
                    "{} is shared by several languages but states no meaning that justifies the correspondence",
                    model.id
                ),
            );
        }
    }
    problems
}

fn is_precedence_label(label: &str) -> bool {
    ["prec", "prec_left", "prec_right", "prec_dynamic"]
        .iter()
        .any(|name| {
            label == *name
                || label
                    .strip_prefix(name)
                    .is_some_and(|rest| rest.starts_with('='))
        })
}

fn collect_precedence(
    rule: &str,
    expr: &GrammarExpr,
    in_token: bool,
    uses: &mut Vec<PrecedenceUse>,
) {
    match expr {
        GrammarExpr::Capture { label, expr } => {
            let label = label.as_deref().unwrap_or_default();
            if is_precedence_label(label) {
                let lexical = in_token && !label.starts_with("prec_dynamic");
                uses.push(PrecedenceUse {
                    rule: rule.to_owned(),
                    label: label.to_owned(),
                    concept: if lexical {
                        "grammar.lexical-precedence"
                    } else {
                        "grammar.syntactic-precedence"
                    },
                });
            }
            let token = in_token || label == "token" || label == "immediate_token";
            collect_precedence(rule, expr, token, uses);
        }
        GrammarExpr::Choice { alternatives, .. } => {
            for alternative in alternatives {
                collect_precedence(rule, alternative, in_token, uses);
            }
        }
        GrammarExpr::Sequence(items) => {
            for item in items {
                collect_precedence(rule, item, in_token, uses);
            }
        }
        GrammarExpr::Optional(item)
        | GrammarExpr::ZeroOrMore(item)
        | GrammarExpr::OneOrMore(item)
        | GrammarExpr::Repeat { expr: item, .. }
        | GrammarExpr::And(item)
        | GrammarExpr::Not(item) => collect_precedence(rule, item, in_token, uses),
        _ => {}
    }
}

/// Returns every precedence use in `grammar` with the concept it expresses.
///
/// Precedence inside a token rule or a `token` capture decides between tokens
/// (lexical precedence); anywhere else, and dynamic precedence everywhere, it
/// decides between parse alternatives (syntactic precedence). Both runtimes
/// read tree-sitter's `prec` the same way, so the two never merge.
#[must_use]
pub fn grammar_precedence_concepts(grammar: &Grammar) -> Vec<PrecedenceUse> {
    let mut uses = Vec::new();
    for rule in grammar.rules() {
        collect_precedence(
            rule.name(),
            rule.expr(),
            rule.kind() == RuleKind::Token,
            &mut uses,
        );
    }
    uses
}
