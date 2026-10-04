//! The concepts of the native grammars (requirement
//! I195-GRAMMAR-SHARED-CONCEPTS): every rule of every native grammar names a
//! canonical concept record with `(concept ID)`, and the record carries a
//! `native:<language>` source alias for each rule that names it. A concept
//! that two or more native grammars name is shared; a concept that one grammar
//! names is specific to that language. A construct translates from one native
//! grammar to another through its concept record alone, with no rule for the
//! pair of languages. This is the port of `js/src/grammar-concepts.js`.

use std::collections::{BTreeMap, BTreeSet};

use crate::language_catalog::{native_grammar, native_grammars};
use crate::native_grammar_parser::{native_grammar_text, native_parser};
use crate::{ConceptRecord, FeatureParseOptions, SyntaxTree, concept_records, parse_grammar_links};

const NATIVE_SOURCE_PREFIX: &str = "native:";

/// A rule of a native grammar and the concept it names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeRuleConcept {
    /// The rule name.
    pub rule: String,
    /// The concept identity the rule names, or `None` when it names none.
    pub concept: Option<String>,
}

/// What is wrong with the concept of a native grammar rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeConceptProblemKind {
    /// The rule names no concept.
    RuleWithoutConcept,
    /// No record has the concept the rule names.
    ConceptWithoutRecord,
    /// The record of the rule's concept has no source alias for the rule.
    RuleWithoutAlias,
    /// A record has a source alias for a rule that does not name it.
    AliasWithoutRule,
    /// The rule names a concept in another language's namespace.
    NamespaceMismatch,
}

impl NativeConceptProblemKind {
    /// The kebab-case name the JavaScript runtime uses.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuleWithoutConcept => "rule-without-concept",
            Self::ConceptWithoutRecord => "concept-without-record",
            Self::RuleWithoutAlias => "rule-without-alias",
            Self::AliasWithoutRule => "alias-without-rule",
            Self::NamespaceMismatch => "namespace-mismatch",
        }
    }
}

/// One problem `check_native_grammar_concepts` finds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeConceptProblem {
    /// What is wrong.
    pub kind: NativeConceptProblemKind,
    /// The native grammar id, such as `native-json`.
    pub grammar: String,
    /// The rule name.
    pub rule: String,
    /// The concept identity involved, if any.
    pub concept: Option<String>,
}

/// The id of every native grammar of the catalog, sorted.
#[must_use]
pub fn native_grammar_ids() -> Vec<&'static str> {
    let mut ids: Vec<&'static str> = native_grammars()
        .iter()
        .map(|grammar| grammar.id.as_str())
        .collect();
    ids.sort_unstable();
    ids
}

/// The language of the native grammar `id`, its file name: `json` for
/// `native-json`.
#[must_use]
pub fn native_grammar_language(id: &str) -> Option<&'static str> {
    let file = native_grammar(id)?.file.as_str();
    let name = file.rsplit('/').next().unwrap_or(file);
    Some(name.strip_suffix(".lino").unwrap_or(name))
}

/// The source concept records name the rules of the native grammar `id`
/// under: `native:json`.
#[must_use]
pub fn native_grammar_source(id: &str) -> Option<String> {
    native_grammar_language(id).map(|language| format!("{NATIVE_SOURCE_PREFIX}{language}"))
}

/// Every rule of the native grammar `id` with the concept it names, in
/// grammar order; empty for an id that names no native grammar.
#[must_use]
pub fn native_grammar_rule_concepts(id: &str) -> Vec<NativeRuleConcept> {
    native_grammar_text(id)
        .and_then(|text| parse_grammar_links(text).ok())
        .map(|grammar| {
            grammar
                .rules()
                .iter()
                .map(|rule| NativeRuleConcept {
                    rule: rule.name.clone(),
                    concept: rule.concept.clone(),
                })
                .collect()
        })
        .unwrap_or_default()
}

// The language a concept identity is specific to by its namespace:
// `grammar.diff.hunk` belongs to diff.
fn namespace_language<'a>(concept: &'a str, languages: &BTreeSet<&str>) -> Option<&'a str> {
    let parts: Vec<&str> = concept.split('.').collect();
    (parts.len() > 2 && languages.contains(parts[1])).then(|| parts[1])
}

/// Checks the concepts of every rule of every native grammar.
///
/// Every rule names a concept that has a record, the records' native source
/// aliases are exactly the rules that name them, and a concept in a language
/// namespace (`grammar.diff.…`) is named by that language alone.
#[must_use]
pub fn check_native_grammar_concepts() -> Vec<NativeConceptProblem> {
    check_native_grammar_concepts_in(
        concept_records(),
        &native_grammar_ids(),
        &native_grammar_rule_concepts,
    )
}

/// `check_native_grammar_concepts` over the records `records`, the native
/// grammars `grammars` and the rule concepts `rule_concepts` gives each.
#[must_use]
pub fn check_native_grammar_concepts_in(
    records: &[ConceptRecord],
    grammars: &[&str],
    rule_concepts: &dyn Fn(&str) -> Vec<NativeRuleConcept>,
) -> Vec<NativeConceptProblem> {
    let by_id: BTreeMap<&str, &ConceptRecord> = records
        .iter()
        .map(|record| (record.id.as_str(), record))
        .collect();
    let languages: BTreeSet<&str> = grammars
        .iter()
        .filter_map(|id| native_grammar_language(id))
        .collect();
    let mut problems = Vec::new();
    for &grammar in grammars {
        let (Some(source), Some(language)) = (
            native_grammar_source(grammar),
            native_grammar_language(grammar),
        ) else {
            continue;
        };
        let problem = |kind, rule: &str, concept: Option<&str>| NativeConceptProblem {
            kind,
            grammar: grammar.to_owned(),
            rule: rule.to_owned(),
            concept: concept.map(str::to_owned),
        };
        let mut named = BTreeMap::new();
        for NativeRuleConcept { rule, concept } in rule_concepts(grammar) {
            let Some(concept) = concept else {
                problems.push(problem(
                    NativeConceptProblemKind::RuleWithoutConcept,
                    &rule,
                    None,
                ));
                continue;
            };
            let Some(record) = by_id.get(concept.as_str()) else {
                problems.push(problem(
                    NativeConceptProblemKind::ConceptWithoutRecord,
                    &rule,
                    Some(&concept),
                ));
                named.insert(rule, concept);
                continue;
            };
            if !record
                .source_aliases
                .iter()
                .any(|alias| alias.source == source && alias.name == rule)
            {
                problems.push(problem(
                    NativeConceptProblemKind::RuleWithoutAlias,
                    &rule,
                    Some(&concept),
                ));
            }
            if namespace_language(&concept, &languages).is_some_and(|owner| owner != language) {
                problems.push(problem(
                    NativeConceptProblemKind::NamespaceMismatch,
                    &rule,
                    Some(&concept),
                ));
            }
            named.insert(rule, concept);
        }
        for record in records {
            for alias in &record.source_aliases {
                if alias.source == source && named.get(&alias.name) != Some(&record.id) {
                    problems.push(problem(
                        NativeConceptProblemKind::AliasWithoutRule,
                        &alias.name,
                        Some(&record.id),
                    ));
                }
            }
        }
    }
    problems
}

/// A rule that names a concept two or more native grammars name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedRule {
    /// The rule name.
    pub rule: String,
    /// The shared concept identity.
    pub concept: String,
    /// The languages whose native grammars name the concept, sorted.
    pub languages: Vec<String>,
}

/// A rule that names a concept only its own native grammar names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecificRule {
    /// The rule name.
    pub rule: String,
    /// The language-specific concept identity.
    pub concept: String,
}

/// The reuse of one native grammar: its rules split into shared and
/// language-specific ones.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarConceptReuse {
    /// The native grammar id.
    pub grammar: String,
    /// The language of the grammar.
    pub language: String,
    /// The number of rules.
    pub rules: usize,
    /// The rules that name a shared concept.
    pub shared: Vec<SharedRule>,
    /// The rules that name a language-specific concept.
    pub specific: Vec<SpecificRule>,
}

/// A concept the native grammars name, with the languages that name it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConceptReuse {
    /// The concept identity.
    pub concept: String,
    /// The languages whose native grammars name the concept, sorted.
    pub languages: Vec<String>,
    /// Whether two or more languages name the concept.
    pub shared: bool,
}

/// The per-language reuse report of the native grammars.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeConceptReuse {
    /// The reuse of each native grammar, by grammar id.
    pub grammars: Vec<GrammarConceptReuse>,
    /// Every concept the native grammars name, by identity.
    pub concepts: Vec<ConceptReuse>,
}

/// The per-language reuse report of the native grammars.
///
/// For every native grammar, its rules split into those that name a shared
/// concept (one two or more native grammars name, listed with those
/// languages) and those that name a concept specific to that language; and
/// every concept the native grammars name with the languages that name it.
#[must_use]
pub fn native_grammar_concept_reuse() -> NativeConceptReuse {
    let grammars = native_grammar_ids();
    let language_of: BTreeMap<String, &str> = grammars
        .iter()
        .filter_map(|id| Some((native_grammar_source(id)?, native_grammar_language(id)?)))
        .collect();
    let mut languages: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for record in concept_records() {
        let named: BTreeSet<&str> = record
            .source_aliases
            .iter()
            .filter_map(|alias| language_of.get(&alias.source).copied())
            .collect();
        if !named.is_empty() {
            languages.insert(&record.id, named.into_iter().map(str::to_owned).collect());
        }
    }
    let report = grammars
        .iter()
        .map(|&grammar| {
            let mut shared = Vec::new();
            let mut specific = Vec::new();
            for NativeRuleConcept { rule, concept } in native_grammar_rule_concepts(grammar) {
                let concept = concept.unwrap_or_default();
                match languages.get(concept.as_str()) {
                    Some(named) if named.len() > 1 => shared.push(SharedRule {
                        rule,
                        concept,
                        languages: named.clone(),
                    }),
                    _ => specific.push(SpecificRule { rule, concept }),
                }
            }
            GrammarConceptReuse {
                grammar: grammar.to_owned(),
                language: native_grammar_language(grammar)
                    .unwrap_or_default()
                    .to_owned(),
                rules: shared.len() + specific.len(),
                shared,
                specific,
            }
        })
        .collect();
    let concepts = languages
        .into_iter()
        .map(|(concept, named)| ConceptReuse {
            concept: concept.to_owned(),
            shared: named.len() > 1,
            languages: named,
        })
        .collect();
    NativeConceptReuse {
        grammars: report,
        concepts,
    }
}

/// How a construct translates from one native grammar to another.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructTranslationRelation {
    /// The target grammar names one rule with the construct's concept.
    Translated,
    /// The target grammar names no rule with the construct's concept.
    Untranslatable,
    /// Several records name the rule, or the target names several rules.
    Ambiguous,
    /// No record names the rule.
    Unknown,
}

impl ConstructTranslationRelation {
    /// The name the JavaScript runtime uses.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Translated => "translated",
            Self::Untranslatable => "untranslatable",
            Self::Ambiguous => "ambiguous",
            Self::Unknown => "unknown",
        }
    }
}

/// The translation of one construct between native grammars.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructTranslation {
    /// How the construct translates.
    pub relation: ConstructTranslationRelation,
    /// The concept the translation went through.
    pub concept: Option<String>,
    /// The rules of the target grammar that name the concept.
    pub rules: Vec<String>,
}

/// Translates the rule `rule` of the native grammar `from` to `to`.
///
/// The translation goes through the concept record whose source alias names
/// the rule, then that record's alias in the target grammar. No rule for the
/// pair of languages takes part.
#[must_use]
pub fn translate_native_construct(from: &str, rule: &str, to: &str) -> ConstructTranslation {
    translate_native_construct_in(concept_records(), from, rule, to)
}

/// `translate_native_construct` over the records `records`.
#[must_use]
pub fn translate_native_construct_in(
    records: &[ConceptRecord],
    from: &str,
    rule: &str,
    to: &str,
) -> ConstructTranslation {
    let unknown = ConstructTranslation {
        relation: ConstructTranslationRelation::Unknown,
        concept: None,
        rules: Vec::new(),
    };
    let (Some(source), Some(target)) = (native_grammar_source(from), native_grammar_source(to))
    else {
        return unknown;
    };
    let meanings: Vec<&ConceptRecord> = records
        .iter()
        .filter(|record| {
            record
                .source_aliases
                .iter()
                .any(|alias| alias.source == source && alias.name == rule)
        })
        .collect();
    let record = match meanings.as_slice() {
        [] => return unknown,
        [record] => record,
        _ => {
            return ConstructTranslation {
                relation: ConstructTranslationRelation::Ambiguous,
                ..unknown
            };
        }
    };
    let rules: Vec<String> = record
        .source_aliases
        .iter()
        .filter(|alias| alias.source == target)
        .map(|alias| alias.name.clone())
        .collect();
    let relation = match rules.len() {
        1 => ConstructTranslationRelation::Translated,
        0 => ConstructTranslationRelation::Untranslatable,
        _ => ConstructTranslationRelation::Ambiguous,
    };
    ConstructTranslation {
        relation,
        concept: Some(record.id.clone()),
        rules,
    }
}

/// A node of a construct tree: a node of a native parse tree whose kind is a
/// rule of the grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructTree {
    /// The rule name.
    pub kind: String,
    /// The concept the rule names.
    pub concept: String,
    /// The constructs inside it.
    pub children: Vec<Self>,
}

/// The construct tree of `source` under the native grammar `id`.
///
/// It keeps the nodes of the parse tree whose kind is a rule of the grammar,
/// without trivia, literals and token kinds that are not rules. `None` when
/// the grammar does not accept the input.
#[must_use]
pub fn native_construct_tree(id: &str, source: &str) -> Option<ConstructTree> {
    let concepts: BTreeMap<String, String> = native_grammar_rule_concepts(id)
        .into_iter()
        .filter_map(|NativeRuleConcept { rule, concept }| Some((rule, concept?)))
        .collect();
    let outcome = native_parser(id)?
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .ok()?;
    let tree = outcome.tree?;
    if !matches!(tree, SyntaxTree::Node { .. }) {
        return None;
    }
    constructs(&tree, &concepts)?.into_iter().next()
}

fn constructs(
    node: &SyntaxTree,
    concepts: &BTreeMap<String, String>,
) -> Option<Vec<ConstructTree>> {
    let (kind, children) = match node {
        SyntaxTree::Node { kind, children, .. } => {
            let mut inner = Vec::new();
            for child in children {
                inner.extend(constructs(child, concepts)?);
            }
            (Some(kind), inner)
        }
        SyntaxTree::Token {
            kind,
            trivia: false,
            ..
        } => (kind.as_ref(), Vec::new()),
        SyntaxTree::Token { .. } | SyntaxTree::Embed { .. } => (None, Vec::new()),
        SyntaxTree::Error { .. } | SyntaxTree::Missing { .. } => return None,
    };
    match kind.and_then(|kind| Some((kind, concepts.get(kind)?))) {
        Some((kind, concept)) => Some(vec![ConstructTree {
            kind: kind.clone(),
            concept: concept.clone(),
            children,
        }]),
        None => Some(children),
    }
}

/// A construct of a tree that does not translate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructTranslationProblem {
    /// The rule name in the source grammar.
    pub kind: String,
    /// The concept of the construct, if a record names it.
    pub concept: Option<String>,
    /// Why it does not translate.
    pub relation: ConstructTranslationRelation,
}

/// The translation of a construct tree between native grammars.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructTreeTranslation {
    /// The tree with the target rule names, or `None` when a construct does
    /// not translate.
    pub tree: Option<ConstructTree>,
    /// Every construct that does not translate, children first.
    pub problems: Vec<ConstructTranslationProblem>,
}

/// Translates a construct tree of the native grammar `from` to the native
/// grammar `to`, every node through `translate_native_construct`.
#[must_use]
pub fn translate_native_construct_tree(
    tree: &ConstructTree,
    from: &str,
    to: &str,
) -> ConstructTreeTranslation {
    fn translate(
        node: &ConstructTree,
        from: &str,
        to: &str,
        problems: &mut Vec<ConstructTranslationProblem>,
    ) -> Option<ConstructTree> {
        let translation = translate_native_construct(from, &node.kind, to);
        let children: Vec<Option<ConstructTree>> = node
            .children
            .iter()
            .map(|child| translate(child, from, to, problems))
            .collect();
        if translation.relation != ConstructTranslationRelation::Translated {
            problems.push(ConstructTranslationProblem {
                kind: node.kind.clone(),
                concept: translation.concept,
                relation: translation.relation,
            });
            return None;
        }
        Some(ConstructTree {
            kind: translation.rules[0].clone(),
            concept: translation.concept.unwrap_or_default(),
            children: children.into_iter().collect::<Option<Vec<_>>>()?,
        })
    }
    let mut problems = Vec::new();
    let translated = translate(tree, from, to, &mut problems);
    ConstructTreeTranslation {
        tree: if problems.is_empty() {
            translated
        } else {
            None
        },
        problems,
    }
}
