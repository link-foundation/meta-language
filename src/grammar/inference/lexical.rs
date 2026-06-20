//! Lexical class inference for positive example corpora.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::grammar::{CharClassItem, GrammarExpr, GrammarRule, RuleKind};
use crate::source::ByteRange;

/// Coarse Unicode-aware character category used for token segmentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CharCategory {
    /// Alphabetic character.
    Letter,
    /// Numeric character.
    Digit,
    /// Whitespace character.
    Whitespace,
    /// Meta-notation skeleton delimiter kept as an atomic token.
    Delimiter,
    /// Other ASCII punctuation or symbol character.
    Punctuation,
    /// Any character outside the coarse lexical categories.
    Other,
}

/// Categorises one character into a coarse lexical class.
#[must_use]
pub fn categorise(value: char) -> CharCategory {
    if value.is_whitespace() {
        CharCategory::Whitespace
    } else if is_delimiter(value) {
        CharCategory::Delimiter
    } else if value.is_alphabetic() {
        CharCategory::Letter
    } else if value.is_numeric() {
        CharCategory::Digit
    } else if value.is_ascii_punctuation() {
        CharCategory::Punctuation
    } else {
        CharCategory::Other
    }
}

/// One lossless token produced by lexical segmentation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    /// Token source text.
    pub text: String,
    /// Coarse category that drove segmentation.
    pub category: CharCategory,
    /// Half-open byte span in the original text.
    pub span: ByteRange,
}

/// Configuration for lexical class inference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LexicalConfig {
    /// Maximum number of distinct forms that can remain a closed literal class.
    pub max_closed_forms: usize,
}

impl Default for LexicalConfig {
    fn default() -> Self {
        Self {
            max_closed_forms: 12,
        }
    }
}

/// Deterministic lexical model inferred from a positive corpus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexicalModel {
    /// Distinct characters observed in the training corpus, sorted by scalar value.
    pub alphabet: Vec<char>,
    /// Inferred token-level rules lowered to the grammar IR.
    pub classes: Vec<GrammarRule>,
    /// Configuration used to infer this model.
    pub config: LexicalConfig,
}

impl LexicalModel {
    /// Re-tokenises text with the same category-driven segmentation policy.
    #[must_use]
    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        tokenize_text(text, self.config)
    }
}

/// Infers a lexical model from positive example texts.
#[must_use]
pub fn infer_lexical_classes(corpus: &[&str], config: &LexicalConfig) -> LexicalModel {
    let mut alphabet = BTreeSet::new();
    let mut forms_by_category = BTreeMap::<CharCategory, BTreeMap<String, usize>>::new();

    for text in corpus {
        alphabet.extend(text.chars());

        for token in tokenize_text(text, *config) {
            *forms_by_category
                .entry(token.category)
                .or_default()
                .entry(token.text)
                .or_default() += 1;
        }
    }

    LexicalModel {
        alphabet: alphabet.into_iter().collect(),
        classes: infer_classes(&forms_by_category, *config),
        config: *config,
    }
}

fn tokenize_text(text: &str, _config: LexicalConfig) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut token_start = 0;
    let mut token_end = 0;
    let mut token_category = None;

    for (index, value) in text.char_indices() {
        let category = categorise(value);
        let value_end = index + value.len_utf8();

        let Some(current_category) = token_category else {
            token_start = index;
            token_end = value_end;
            token_category = Some(category);
            continue;
        };

        if continues_token(current_category, category) {
            token_end = value_end;
        } else {
            tokens.push(Token {
                text: text[token_start..token_end].to_string(),
                category: current_category,
                span: ByteRange::new(token_start, token_end),
            });
            token_start = index;
            token_end = value_end;
            token_category = Some(category);
        }
    }

    if let Some(category) = token_category {
        tokens.push(Token {
            text: text[token_start..token_end].to_string(),
            category,
            span: ByteRange::new(token_start, token_end),
        });
    }

    tokens
}

fn continues_token(current: CharCategory, next: CharCategory) -> bool {
    if is_atomic(current) || is_atomic(next) {
        return false;
    }

    current == next || (current == CharCategory::Letter && next == CharCategory::Digit)
}

const fn is_atomic(category: CharCategory) -> bool {
    matches!(
        category,
        CharCategory::Delimiter | CharCategory::Punctuation
    )
}

const fn is_delimiter(value: char) -> bool {
    matches!(value, '(' | ')' | '[' | ']' | '{' | '}' | '\'' | '"' | '`')
}

fn infer_classes(
    forms_by_category: &BTreeMap<CharCategory, BTreeMap<String, usize>>,
    config: LexicalConfig,
) -> Vec<GrammarRule> {
    let mut classes = Vec::new();

    for (category, forms) in forms_by_category {
        match category {
            CharCategory::Delimiter | CharCategory::Punctuation => {
                classes.extend(forms.keys().map(|form| literal_rule(form)));
            }
            CharCategory::Digit | CharCategory::Whitespace => {
                classes.push(open_rule(*category, forms.keys().map(String::as_str)));
            }
            CharCategory::Letter | CharCategory::Other => {
                classes.extend(infer_mixed_category(*category, forms, config));
            }
        }
    }

    classes
}

fn infer_mixed_category(
    category: CharCategory,
    forms: &BTreeMap<String, usize>,
    config: LexicalConfig,
) -> Vec<GrammarRule> {
    let has_repeated_forms = forms.values().any(|count| *count > 1);
    let has_singleton_forms = forms.values().any(|count| *count == 1);

    if forms.len() <= config.max_closed_forms && !(has_repeated_forms && has_singleton_forms) {
        return forms.keys().map(|form| literal_rule(form)).collect();
    }

    let mut rules = Vec::new();
    let mut open_forms = Vec::new();
    let mut closed_forms = 0;

    for (form, count) in forms {
        if *count > 1 && closed_forms < config.max_closed_forms {
            rules.push(literal_rule(form));
            closed_forms += 1;
        } else {
            open_forms.push(form.as_str());
        }
    }

    if !open_forms.is_empty() {
        rules.push(open_rule(category, open_forms));
    }

    rules
}

fn literal_rule(form: &str) -> GrammarRule {
    GrammarRule::new(literal_rule_name(form), GrammarExpr::terminal(form))
        .with_kind(RuleKind::Token)
}

fn literal_rule_name(form: &str) -> String {
    let mut name = String::from("literal");

    for value in form.chars() {
        name.push('_');
        if value.is_ascii_alphanumeric() {
            name.push(value.to_ascii_lowercase());
        } else {
            name.push('u');
            write!(name, "{:04x}", u32::from(value)).expect("writing to a String cannot fail");
        }
    }

    name
}

fn open_rule<'a>(category: CharCategory, forms: impl IntoIterator<Item = &'a str>) -> GrammarRule {
    GrammarRule::new(open_rule_name(category), open_expr(category, forms))
        .with_kind(RuleKind::Token)
}

const fn open_rule_name(category: CharCategory) -> &'static str {
    match category {
        CharCategory::Letter => "identifier",
        CharCategory::Digit => "integer",
        CharCategory::Whitespace => "whitespace",
        CharCategory::Delimiter => "delimiter",
        CharCategory::Punctuation => "punctuation",
        CharCategory::Other => "other",
    }
}

fn open_expr<'a>(category: CharCategory, forms: impl IntoIterator<Item = &'a str>) -> GrammarExpr {
    let forms = forms.into_iter().collect::<Vec<_>>();

    match category {
        CharCategory::Letter => identifier_expr(&forms),
        CharCategory::Digit if all_ascii_digits(&forms) => {
            GrammarExpr::one_or_more(GrammarExpr::char_range('0', '9'))
        }
        CharCategory::Digit | CharCategory::Whitespace | CharCategory::Other => {
            GrammarExpr::one_or_more(char_set_expr(&all_chars(&forms)))
        }
        CharCategory::Delimiter | CharCategory::Punctuation => {
            GrammarExpr::choice(false, forms.into_iter().map(GrammarExpr::terminal))
        }
    }
}

fn identifier_expr(forms: &[&str]) -> GrammarExpr {
    let mut first_chars = BTreeSet::new();
    let mut rest_chars = BTreeSet::new();

    for form in forms {
        let mut chars = form.chars();
        if let Some(first) = chars.next() {
            first_chars.insert(first);
            rest_chars.extend(chars);
        }
    }

    let first = char_set_expr(&first_chars);
    if rest_chars.is_empty() {
        first
    } else {
        GrammarExpr::sequence([first, GrammarExpr::zero_or_more(char_set_expr(&rest_chars))])
    }
}

fn all_ascii_digits(forms: &[&str]) -> bool {
    forms
        .iter()
        .flat_map(|form| form.chars())
        .all(|value| value.is_ascii_digit())
}

fn all_chars(forms: &[&str]) -> BTreeSet<char> {
    forms.iter().flat_map(|form| form.chars()).collect()
}

fn char_set_expr(chars: &BTreeSet<char>) -> GrammarExpr {
    GrammarExpr::char_class(false, char_class_items(chars))
}

fn char_class_items(chars: &BTreeSet<char>) -> Vec<CharClassItem> {
    let mut items = Vec::new();
    let mut chars = chars.iter().copied();
    let Some(mut range_start) = chars.next() else {
        return items;
    };
    let mut range_end = range_start;

    for value in chars {
        if u32::from(value) != u32::from(range_end).saturating_add(1) {
            push_char_class_item(&mut items, range_start, range_end);
            range_start = value;
        }
        range_end = value;
    }

    push_char_class_item(&mut items, range_start, range_end);
    items
}

fn push_char_class_item(items: &mut Vec<CharClassItem>, start: char, end: char) {
    if start == end {
        items.push(CharClassItem::char(start));
    } else {
        items.push(CharClassItem::range(start, end));
    }
}
