//! Concept-aligned translation of a grammar's human-facing surface.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use super::{Grammar, GrammarExpr, GrammarRule, GRAMMAR_CONCEPTS};
use crate::{
    LinkMetadata, LinkNetwork, LinkQuery, LinkType, ParseConfiguration, TranslationRule,
    TranslationRuleSet, TranslationTemplate,
};

/// Error raised while translating grammar rule names and documentation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrammarTranslateError {
    /// A rule references a concept absent from the provided rule set.
    UnknownConcept {
        /// Rule whose explicit concept could not be translated.
        rule: String,
        /// Missing concept id.
        concept: String,
    },
    /// Two distinct source rules translate to the same target name.
    NameCollision {
        /// Target language requested by the caller.
        language: String,
        /// Colliding translated rule name.
        name: String,
    },
}

impl fmt::Display for GrammarTranslateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownConcept { rule, concept } => {
                write!(
                    formatter,
                    "rule `{rule}` references unknown grammar concept `{concept}`"
                )
            }
            Self::NameCollision { language, name } => write!(
                formatter,
                "translating grammar surface to `{language}` produced duplicate rule name `{name}`"
            ),
        }
    }
}

impl Error for GrammarTranslateError {}

/// Translates a grammar's rule names and documentation into `target_language`.
///
/// The grammar algebra is preserved: expressions are cloned with only
/// `NonTerminal` references rewritten to match renamed rules. Rules without an
/// explicit concept and without a known source-language surface are left
/// unchanged.
///
/// # Errors
///
/// Returns [`GrammarTranslateError::UnknownConcept`] when an explicitly aligned
/// rule concept cannot be rendered by `rules`, or
/// [`GrammarTranslateError::NameCollision`] when distinct rules would share one
/// translated name.
pub fn translate_grammar_surface(
    grammar: &Grammar,
    target_language: &str,
    rules: &TranslationRuleSet,
) -> Result<Grammar, GrammarTranslateError> {
    let rename_map = translated_rule_names(grammar, target_language, rules)?;
    let mut translated = Grammar::new();

    if let Some(source_format) = grammar.source_format() {
        translated.set_source_format(source_format);
    }
    if let Some(start) = grammar.start() {
        translated.set_start(renamed_name(start, &rename_map));
    }

    for rule in grammar.rules() {
        let mut translated_rule = GrammarRule::new(
            renamed_name(rule.name(), &rename_map),
            rename_expr(rule.expr(), &rename_map),
        )
        .with_kind(rule.kind());

        if let Some(concept) = rule.concept() {
            translated_rule = translated_rule.with_concept(concept);
        }
        if let Some(doc) = rule.doc() {
            translated_rule =
                translated_rule.with_doc(translate_doc_comment(doc, target_language, rules));
        }

        translated.add_rule(translated_rule);
    }

    Ok(translated)
}

/// Builds a default rule set for grammar-construct and common grammar-surface
/// concepts.
#[must_use]
pub fn grammar_concept_translation_rules() -> TranslationRuleSet {
    let grammar_concepts = GRAMMAR_CONCEPTS
        .iter()
        .map(|concept| concept.id)
        .collect::<BTreeSet<_>>();
    let mut rules = TranslationRuleSet::new("grammar-concepts");

    for surface in GRAMMAR_SURFACE_TRANSLATIONS {
        debug_assert!(
            grammar_concepts.contains(surface.concept)
                || surface.concept.starts_with("grammar.")
                || surface.concept.starts_with("grammar::concept::"),
            "grammar translation concept should use a grammar concept namespace"
        );
        rules.add_rule(
            TranslationRule::new(
                surface.concept,
                LinkQuery::by_type(LinkType::Concept).with_term(surface.concept),
            )
            .with_template("English", surface.english)
            .with_template("en", surface.english)
            .with_template("Russian", surface.russian)
            .with_template("ru", surface.russian),
        );
    }

    rules
}

fn translated_rule_names(
    grammar: &Grammar,
    target_language: &str,
    rules: &TranslationRuleSet,
) -> Result<BTreeMap<String, String>, GrammarTranslateError> {
    let mut rename_map = BTreeMap::new();
    let mut used_names = BTreeMap::<String, String>::new();

    for rule in grammar.rules() {
        let resolved = resolve_rule_concept(rule, rules);
        let translated_name = match resolved {
            Some(ResolvedConcept {
                concept,
                explicit: true,
            }) => translate_concept_surface(&concept, target_language, rules).ok_or_else(|| {
                GrammarTranslateError::UnknownConcept {
                    rule: rule.name().to_string(),
                    concept,
                }
            })?,
            Some(ResolvedConcept {
                concept,
                explicit: false,
            }) => translate_concept_surface(&concept, target_language, rules)
                .unwrap_or_else(|| rule.name().to_string()),
            None => rule.name().to_string(),
        };

        if let Some(previous_rule) = used_names.insert(translated_name.clone(), rule.name().into())
        {
            if previous_rule != rule.name() {
                return Err(GrammarTranslateError::NameCollision {
                    language: target_language.to_string(),
                    name: translated_name,
                });
            }
        }
        rename_map.insert(rule.name().to_string(), translated_name);
    }

    Ok(rename_map)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ResolvedConcept {
    concept: String,
    explicit: bool,
}

fn resolve_rule_concept(rule: &GrammarRule, rules: &TranslationRuleSet) -> Option<ResolvedConcept> {
    rule.concept()
        .map(|concept| ResolvedConcept {
            concept: concept.to_string(),
            explicit: true,
        })
        .or_else(|| infer_concept_from_surface(rule.name(), rules))
}

fn infer_concept_from_surface(name: &str, rules: &TranslationRuleSet) -> Option<ResolvedConcept> {
    for rule in rules.rules() {
        let Some(concept) = concept_id_for_translation_rule(rule) else {
            continue;
        };
        if concept == name
            || rule
                .templates()
                .values()
                .any(|template| template.source() == name)
        {
            return Some(ResolvedConcept {
                concept: concept.to_string(),
                explicit: false,
            });
        }
    }

    None
}

fn concept_id_for_translation_rule(rule: &TranslationRule) -> Option<&str> {
    rule.query()
        .term_filter()
        .or_else(|| concept_like_rule_name(rule.name()))
}

fn concept_like_rule_name(name: &str) -> Option<&str> {
    (name.starts_with("grammar.") || name.starts_with("grammar::concept::")).then_some(name)
}

fn translate_concept_surface(
    concept: &str,
    target_language: &str,
    rules: &TranslationRuleSet,
) -> Option<String> {
    render_concept_with_rules(concept, target_language, rules).or_else(|| {
        rules
            .rules()
            .iter()
            .find(|rule| concept_id_for_translation_rule(rule) == Some(concept))
            .and_then(|rule| template_for_language(rule.templates(), target_language))
            .map(ToString::to_string)
    })
}

fn render_concept_with_rules(
    concept: &str,
    target_language: &str,
    rules: &TranslationRuleSet,
) -> Option<String> {
    let mut network = LinkNetwork::new();
    let concept_link = network.intern_concept(concept, None);
    network.insert_link(
        [concept_link],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term(concept),
    );

    for language in language_lookup_order(target_language) {
        let rendered =
            network.reconstruct_text_as_with_rules(&language, ParseConfiguration::default(), rules);
        if !rendered.is_empty() {
            return Some(rendered);
        }
    }

    None
}

fn template_for_language<'a>(
    templates: &'a BTreeMap<String, TranslationTemplate>,
    language: &str,
) -> Option<&'a str> {
    language_lookup_order(language)
        .into_iter()
        .find_map(|candidate| templates.get(&candidate).map(TranslationTemplate::source))
}

fn language_lookup_order(language: &str) -> Vec<String> {
    match canonical_language(language) {
        Some(("English", "en")) => vec!["English".to_string(), "en".to_string()],
        Some(("Russian", "ru")) => vec!["Russian".to_string(), "ru".to_string()],
        _ => vec![language.to_string()],
    }
}

fn canonical_language(language: &str) -> Option<(&'static str, &'static str)> {
    match language.to_ascii_lowercase().as_str() {
        "english" | "en" => Some(("English", "en")),
        "russian" | "ru" => Some(("Russian", "ru")),
        _ => None,
    }
}

fn renamed_name(name: &str, rename_map: &BTreeMap<String, String>) -> String {
    rename_map
        .get(name)
        .cloned()
        .unwrap_or_else(|| name.to_string())
}

fn rename_expr(expr: &GrammarExpr, rename_map: &BTreeMap<String, String>) -> GrammarExpr {
    match expr {
        GrammarExpr::NonTerminal(name) => GrammarExpr::NonTerminal(renamed_name(name, rename_map)),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => GrammarExpr::Choice {
            ordered: *ordered,
            alternatives: alternatives
                .iter()
                .map(|expr| rename_expr(expr, rename_map))
                .collect(),
        },
        GrammarExpr::Sequence(items) => GrammarExpr::Sequence(
            items
                .iter()
                .map(|expr| rename_expr(expr, rename_map))
                .collect(),
        ),
        GrammarExpr::Optional(expr) => {
            GrammarExpr::Optional(Box::new(rename_expr(expr, rename_map)))
        }
        GrammarExpr::ZeroOrMore(expr) => {
            GrammarExpr::ZeroOrMore(Box::new(rename_expr(expr, rename_map)))
        }
        GrammarExpr::OneOrMore(expr) => {
            GrammarExpr::OneOrMore(Box::new(rename_expr(expr, rename_map)))
        }
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::Repeat {
            expr: Box::new(rename_expr(expr, rename_map)),
            min: *min,
            max: *max,
        },
        GrammarExpr::And(expr) => GrammarExpr::And(Box::new(rename_expr(expr, rename_map))),
        GrammarExpr::Not(expr) => GrammarExpr::Not(Box::new(rename_expr(expr, rename_map))),
        GrammarExpr::Capture { label, expr } => GrammarExpr::Capture {
            label: label.clone(),
            expr: Box::new(rename_expr(expr, rename_map)),
        },
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => expr.clone(),
    }
}

fn translate_doc_comment(doc: &str, target_language: &str, rules: &TranslationRuleSet) -> String {
    for source_language in doc_source_languages(target_language) {
        let network = LinkNetwork::parse(doc, source_language, ParseConfiguration::default());
        let rendered = network.reconstruct_text_as_with_rules(
            target_language,
            ParseConfiguration::default(),
            rules,
        );
        if rendered != doc {
            return rendered;
        }
    }

    let replaced = replace_known_concept_surfaces(doc, target_language, rules);
    if replaced == doc {
        doc.to_string()
    } else {
        replaced
    }
}

fn doc_source_languages(target_language: &str) -> Vec<&'static str> {
    match canonical_language(target_language) {
        Some(("Russian", _)) => vec!["English"],
        Some(("English", _)) => vec!["Russian"],
        _ => vec!["English", "Russian"],
    }
}

fn replace_known_concept_surfaces(
    text: &str,
    target_language: &str,
    rules: &TranslationRuleSet,
) -> String {
    let mut replacements = rules
        .rules()
        .iter()
        .filter_map(|rule| {
            template_for_language(rule.templates(), target_language).map(|target| (rule, target))
        })
        .flat_map(|(rule, target)| {
            rule.templates()
                .values()
                .filter_map(move |template| replacement_pair(template.source(), target))
        })
        .collect::<Vec<_>>();

    replacements.sort_by_key(|replacement| Reverse(replacement.source.len()));
    replacements.dedup();

    replacements
        .into_iter()
        .fold(text.to_string(), |current, pair| {
            replace_surface(&current, &pair.source, &pair.target)
        })
}

fn replacement_pair(source: &str, target: &str) -> Option<ReplacementPair> {
    (!source.is_empty() && source != target && !source.contains('{') && !target.contains('{')).then(
        || ReplacementPair {
            source: source.to_string(),
            target: target.to_string(),
        },
    )
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ReplacementPair {
    source: String,
    target: String,
}

fn replace_surface(text: &str, source: &str, target: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut index = 0;

    while index < text.len() {
        if text[index..].starts_with(source)
            && text.is_char_boundary(index + source.len())
            && has_surface_boundaries(text, source, index, index + source.len())
        {
            output.push_str(target);
            index += source.len();
            continue;
        }

        let character = text[index..]
            .chars()
            .next()
            .expect("index should be inside the string");
        output.push(character);
        index += character.len_utf8();
    }

    output
}

fn has_surface_boundaries(text: &str, source: &str, start: usize, end: usize) -> bool {
    let first = source.chars().next();
    let last = source.chars().next_back();
    let before = text[..start].chars().next_back();
    let after = text[end..].chars().next();

    let start_ok = first.map_or(true, |character| !is_word_character(character))
        || !before.is_some_and(is_word_character);
    let end_ok = last.map_or(true, |character| !is_word_character(character))
        || !after.is_some_and(is_word_character);

    start_ok && end_ok
}

fn is_word_character(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SurfaceTranslation {
    concept: &'static str,
    english: &'static str,
    russian: &'static str,
}

const GRAMMAR_SURFACE_TRANSLATIONS: &[SurfaceTranslation] = &[
    SurfaceTranslation {
        concept: "grammar.rule",
        english: "rule",
        russian: "правило",
    },
    SurfaceTranslation {
        concept: "grammar.sequence",
        english: "sequence",
        russian: "последовательность",
    },
    SurfaceTranslation {
        concept: "grammar.ordered-choice",
        english: "ordered choice",
        russian: "упорядоченный выбор",
    },
    SurfaceTranslation {
        concept: "grammar.unordered-choice",
        english: "choice",
        russian: "выбор",
    },
    SurfaceTranslation {
        concept: "grammar.repetition",
        english: "repetition",
        russian: "повторение",
    },
    SurfaceTranslation {
        concept: "grammar.zero-or-more",
        english: "zero or more",
        russian: "ноль или более",
    },
    SurfaceTranslation {
        concept: "grammar.one-or-more",
        english: "one or more",
        russian: "один или более",
    },
    SurfaceTranslation {
        concept: "grammar.optional",
        english: "optional",
        russian: "необязательный",
    },
    SurfaceTranslation {
        concept: "grammar.terminal",
        english: "terminal",
        russian: "терминал",
    },
    SurfaceTranslation {
        concept: "grammar.non-terminal",
        english: "non-terminal",
        russian: "нетерминал",
    },
    SurfaceTranslation {
        concept: "grammar.char-class",
        english: "character class",
        russian: "класс символов",
    },
    SurfaceTranslation {
        concept: "grammar.char-range",
        english: "character range",
        russian: "диапазон символов",
    },
    SurfaceTranslation {
        concept: "grammar.any-char",
        english: "any character",
        russian: "любой символ",
    },
    SurfaceTranslation {
        concept: "grammar.positive-predicate",
        english: "positive predicate",
        russian: "положительный предикат",
    },
    SurfaceTranslation {
        concept: "grammar.negative-predicate",
        english: "negative predicate",
        russian: "отрицательный предикат",
    },
    SurfaceTranslation {
        concept: "grammar.capture",
        english: "capture",
        russian: "захват",
    },
    SurfaceTranslation {
        concept: "grammar.empty",
        english: "empty",
        russian: "пусто",
    },
    SurfaceTranslation {
        concept: "grammar.expression",
        english: "expression",
        russian: "выражение",
    },
    SurfaceTranslation {
        concept: "grammar.term",
        english: "term",
        russian: "слагаемое",
    },
    SurfaceTranslation {
        concept: "grammar.factor",
        english: "factor",
        russian: "множитель",
    },
    SurfaceTranslation {
        concept: "grammar.number",
        english: "number",
        russian: "число",
    },
    SurfaceTranslation {
        concept: "grammar.digit",
        english: "digit",
        russian: "цифра",
    },
    SurfaceTranslation {
        concept: "grammar.letter",
        english: "letter",
        russian: "буква",
    },
    SurfaceTranslation {
        concept: "grammar.identifier",
        english: "identifier",
        russian: "идентификатор",
    },
    SurfaceTranslation {
        concept: "grammar.statement",
        english: "statement",
        russian: "оператор",
    },
    SurfaceTranslation {
        concept: "grammar.item",
        english: "item",
        russian: "элемент",
    },
    SurfaceTranslation {
        concept: "grammar.list",
        english: "list",
        russian: "список",
    },
    SurfaceTranslation {
        concept: "grammar.value",
        english: "value",
        russian: "значение",
    },
    SurfaceTranslation {
        concept: "grammar.name",
        english: "name",
        russian: "имя",
    },
    SurfaceTranslation {
        concept: "grammar.object",
        english: "object",
        russian: "объект",
    },
    SurfaceTranslation {
        concept: "grammar.member",
        english: "member",
        russian: "член",
    },
    SurfaceTranslation {
        concept: "grammar.string",
        english: "string",
        russian: "строка",
    },
    SurfaceTranslation {
        concept: "grammar.boolean",
        english: "boolean",
        russian: "логическое",
    },
    SurfaceTranslation {
        concept: "grammar.null",
        english: "null",
        russian: "нуль",
    },
    SurfaceTranslation {
        concept: "grammar::concept::expression",
        english: "expression",
        russian: "выражение",
    },
    SurfaceTranslation {
        concept: "grammar::concept::term",
        english: "term",
        russian: "слагаемое",
    },
    SurfaceTranslation {
        concept: "grammar::concept::factor",
        english: "factor",
        russian: "множитель",
    },
];
