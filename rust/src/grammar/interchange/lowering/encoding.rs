//! The constructs each notation does not carry unchanged and the helper
//! bodies that encode them in constructs it does.

use super::super::super::emit::case_variants;
use super::super::super::{CharClassItem, GrammarExpr, RuleKind};
use super::{GrammarLoweringEncoding, GrammarLoweringError};

/// The highest Unicode code point.
const MAX_CODE_POINT: u32 = 0x0010_FFFF;
/// The most characters a character set is expanded to before it is approximated.
pub const MAX_LOWERED_CHARACTERS: usize = 256;

/// A construct a notation may not carry unchanged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Construct {
    QuotedLiteral,
    LiteralInsensitive,
    CharRange,
    CharClass,
    NegatedClass,
    Any,
    OrderedChoice,
    UnorderedChoice,
    NestedChoice,
    ChoiceWithEmpty,
    ParserCharacters,
    Optional,
    Repeat0,
    Repeat1,
    CountedRepeat,
    UnboundedRepeat,
    And,
    Not,
    LabeledCapture,
    UnlabeledCapture,
    LabelNotIdentifier,
    Empty,
}

impl Construct {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::QuotedLiteral => "quotedLiteral",
            Self::LiteralInsensitive => "literalInsensitive",
            Self::CharRange => "charRange",
            Self::CharClass => "charClass",
            Self::NegatedClass => "negatedClass",
            Self::Any => "any",
            Self::OrderedChoice => "orderedChoice",
            Self::UnorderedChoice => "unorderedChoice",
            Self::NestedChoice => "nestedChoice",
            Self::ChoiceWithEmpty => "choiceWithEmpty",
            Self::ParserCharacters => "parserCharacters",
            Self::Optional => "optional",
            Self::Repeat0 => "repeat0",
            Self::Repeat1 => "repeat1",
            Self::CountedRepeat => "countedRepeat",
            Self::UnboundedRepeat => "unboundedRepeat",
            Self::And => "and",
            Self::Not => "not",
            Self::LabeledCapture => "labeledCapture",
            Self::UnlabeledCapture => "unlabeledCapture",
            Self::LabelNotIdentifier => "labelNotIdentifier",
            Self::Empty => "empty",
        }
    }
}

/// The constructs each notation's emitter and importer do not carry
/// unchanged, as measured by `experiments/lowering-capability-probe.mjs`.
pub(super) fn unsupported(format: &str) -> &'static [Construct] {
    use Construct as C;
    match format {
        "abnf" => &[
            C::NegatedClass,
            C::Any,
            C::OrderedChoice,
            C::And,
            C::Not,
            C::LabeledCapture,
            C::UnlabeledCapture,
            C::Empty,
        ],
        "antlr" => &[
            C::ParserCharacters,
            C::LiteralInsensitive,
            C::OrderedChoice,
            C::CountedRepeat,
            C::UnboundedRepeat,
            C::And,
            C::Not,
            C::UnlabeledCapture,
            C::LabelNotIdentifier,
        ],
        "bnf" => &[
            C::QuotedLiteral,
            C::LiteralInsensitive,
            C::CharRange,
            C::CharClass,
            C::NegatedClass,
            C::Any,
            C::OrderedChoice,
            C::NestedChoice,
            C::Optional,
            C::Repeat0,
            C::Repeat1,
            C::CountedRepeat,
            C::UnboundedRepeat,
            C::And,
            C::Not,
            C::LabeledCapture,
            C::UnlabeledCapture,
        ],
        "ebnf" => &[
            C::LiteralInsensitive,
            C::CharRange,
            C::CharClass,
            C::NegatedClass,
            C::Any,
            C::OrderedChoice,
            C::CountedRepeat,
            C::UnboundedRepeat,
            C::And,
            C::Not,
            C::LabeledCapture,
            C::UnlabeledCapture,
        ],
        "gbnf" => &[
            C::LiteralInsensitive,
            C::Any,
            C::OrderedChoice,
            C::And,
            C::Not,
            C::LabeledCapture,
            C::UnlabeledCapture,
            C::Empty,
        ],
        "lark" => &[
            C::LiteralInsensitive,
            C::CharRange,
            C::Any,
            C::OrderedChoice,
            C::UnboundedRepeat,
            C::And,
            C::Not,
            C::LabeledCapture,
            C::UnlabeledCapture,
        ],
        "pest" => &[
            C::NegatedClass,
            C::UnorderedChoice,
            C::LabeledCapture,
            C::UnlabeledCapture,
            C::Empty,
        ],
        "tree-sitter-json" => &[
            C::LiteralInsensitive,
            C::Any,
            C::OrderedChoice,
            C::ChoiceWithEmpty,
            C::CountedRepeat,
            C::UnboundedRepeat,
            C::And,
            C::Not,
            C::UnlabeledCapture,
        ],
        _ => &[],
    }
}

/// The rule kinds each notation writes and reads back unchanged, as measured
/// by `experiments/lowering-kind-probe.mjs`; other rules are lowered to
/// normal rules with a kind step.
pub(super) fn kept_kinds(format: &str) -> &'static [RuleKind] {
    match format {
        "pest" => &[
            RuleKind::Normal,
            RuleKind::Atomic,
            RuleKind::Silent,
            RuleKind::Token,
        ],
        "tree-sitter-json" => &[RuleKind::Normal, RuleKind::Token],
        _ => &[RuleKind::Normal],
    }
}

fn is_identifier(label: &str) -> bool {
    let mut bytes = label.bytes();
    bytes
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

/// The constructs of `expr` as a whole rule body unless `inner`, outside a
/// lexer rule unless `lexical`.
pub(super) fn constructs_of(expr: &GrammarExpr, inner: bool, lexical: bool) -> Vec<Construct> {
    let character = match expr {
        GrammarExpr::TerminalInsensitive(value) => !value.is_empty(),
        GrammarExpr::CharRange(..) | GrammarExpr::CharClass { .. } | GrammarExpr::AnyChar => true,
        _ => false,
    };
    let mut constructs = Vec::new();
    if !lexical && character {
        constructs.push(Construct::ParserCharacters);
    }
    match expr {
        GrammarExpr::Terminal(value) if value.contains('"') && value.contains('\'') => {
            constructs.push(Construct::QuotedLiteral);
        }
        GrammarExpr::TerminalInsensitive(value) if !value.is_empty() => {
            constructs.push(Construct::LiteralInsensitive);
        }
        GrammarExpr::CharRange(..) => constructs.push(Construct::CharRange),
        GrammarExpr::CharClass { negated, .. } => constructs.push(if *negated {
            Construct::NegatedClass
        } else {
            Construct::CharClass
        }),
        GrammarExpr::AnyChar => constructs.push(Construct::Any),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => {
            constructs.push(if *ordered {
                Construct::OrderedChoice
            } else {
                Construct::UnorderedChoice
            });
            if alternatives.contains(&GrammarExpr::Empty) {
                constructs.push(Construct::ChoiceWithEmpty);
            }
            if inner {
                constructs.push(Construct::NestedChoice);
            }
        }
        GrammarExpr::Optional(_) => constructs.push(Construct::Optional),
        GrammarExpr::ZeroOrMore(_) => constructs.push(Construct::Repeat0),
        GrammarExpr::OneOrMore(_) => constructs.push(Construct::Repeat1),
        GrammarExpr::And(_) => constructs.push(Construct::And),
        GrammarExpr::Not(_) => constructs.push(Construct::Not),
        GrammarExpr::Empty => constructs.push(Construct::Empty),
        GrammarExpr::Repeat { max, .. } => constructs.push(if max.is_none() {
            Construct::UnboundedRepeat
        } else {
            Construct::CountedRepeat
        }),
        GrammarExpr::Capture { label: None, .. } => constructs.push(Construct::UnlabeledCapture),
        GrammarExpr::Capture {
            label: Some(label), ..
        } => {
            constructs.push(Construct::LabeledCapture);
            if !is_identifier(label) {
                constructs.push(Construct::LabelNotIdentifier);
            }
        }
        _ => {}
    }
    constructs
}

fn literal(value: impl Into<String>) -> GrammarExpr {
    GrammarExpr::Terminal(value.into())
}

fn unordered(mut items: Vec<GrammarExpr>) -> GrammarExpr {
    if items.len() == 1 {
        items.remove(0)
    } else {
        GrammarExpr::Choice {
            ordered: false,
            alternatives: items,
        }
    }
}

fn seq(mut items: Vec<GrammarExpr>) -> GrammarExpr {
    if items.len() == 1 {
        items.remove(0)
    } else {
        GrammarExpr::Sequence(items)
    }
}

fn to_char(point: u32) -> char {
    char::from_u32(point).unwrap_or(char::REPLACEMENT_CHARACTER)
}

/// The expression a notation reads back as matching the empty text.
fn empty_encoding(format: &str) -> GrammarExpr {
    if format == "abnf" {
        GrammarExpr::TerminalInsensitive(String::new())
    } else if unsupported(format).contains(&Construct::Empty) {
        literal("")
    } else {
        GrammarExpr::Empty
    }
}

/// The code point ranges of a character set's items.
fn item_ranges(items: &[CharClassItem]) -> Vec<(u32, u32)> {
    items
        .iter()
        .map(|item| match item {
            CharClassItem::Char(value) => (u32::from(*value), u32::from(*value)),
            CharClassItem::Range(start, end) => (u32::from(*start), u32::from(*end)),
        })
        .collect()
}

/// Sorted, merged, non-overlapping ranges.
fn merge_ranges(mut ranges: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    ranges.sort_by_key(|range| range.0);
    let mut merged: Vec<(u32, u32)> = Vec::new();
    for (start, end) in ranges {
        match merged.last_mut() {
            Some(last) if start <= last.1 + 1 => last.1 = last.1.max(end),
            _ => merged.push((start, end)),
        }
    }
    merged
}

/// The ranges of every character outside `ranges`, surrogate code points left out.
fn complement_ranges(mut ranges: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    ranges.push((0xD800, 0xDFFF));
    let mut complement = Vec::new();
    let mut next = 0;
    for (start, end) in merge_ranges(ranges) {
        if start > next {
            complement.push((next, start - 1));
        }
        next = end + 1;
    }
    if next <= MAX_CODE_POINT {
        complement.push((next, MAX_CODE_POINT));
    }
    complement
}

/// A character set as constructs `format` writes: one character class where
/// the notation has classes and a choice of single characters otherwise. A
/// set too large to spell character by character is limited to printable
/// ASCII. Returns the expression and whether it is exact.
fn character_set_encoding(
    format: &str,
    ranges: &[(u32, u32)],
) -> Result<(GrammarExpr, bool), GrammarLoweringError> {
    if !unsupported(format).contains(&Construct::CharClass) {
        let items: Vec<CharClassItem> = ranges
            .iter()
            .map(|&(start, end)| {
                if start == end {
                    CharClassItem::Char(to_char(start))
                } else {
                    CharClassItem::Range(to_char(start), to_char(end))
                }
            })
            .collect();
        if let [CharClassItem::Char(value)] = items.as_slice() {
            return Ok((literal(value.to_string()), true));
        }
        return Ok((
            GrammarExpr::CharClass {
                negated: false,
                items,
            },
            true,
        ));
    }
    let size: u32 = ranges.iter().map(|(start, end)| end - start + 1).sum();
    let exact = usize::try_from(size).is_ok_and(|size| size <= MAX_LOWERED_CHARACTERS);
    let characters: Vec<char> = if exact {
        ranges
            .iter()
            .flat_map(|&(start, end)| (start..=end).filter_map(char::from_u32))
            .collect()
    } else {
        (0x20..0x7F)
            .filter(|point| {
                ranges
                    .iter()
                    .any(|(start, end)| point >= start && point <= end)
            })
            .map(to_char)
            .collect()
    };
    if characters.is_empty() {
        return Err(GrammarLoweringError::Unencodable(format.to_owned()));
    }
    Ok((
        unordered(
            characters
                .into_iter()
                .map(|value| literal(value.to_string()))
                .collect(),
        ),
        exact,
    ))
}

/// Whether ordered and unordered readings of a choice accept the same texts.
fn choice_order_irrelevant(items: &[GrammarExpr]) -> bool {
    let values: Option<Vec<&str>> = items
        .iter()
        .map(|item| match item {
            GrammarExpr::Terminal(value) if !value.is_empty() => Some(value.as_str()),
            _ => None,
        })
        .collect();
    values.is_some_and(|values| {
        values.iter().enumerate().all(|(index, value)| {
            values
                .iter()
                .enumerate()
                .all(|(at, other)| at == index || !other.starts_with(value))
        })
    })
}

type Encoding = (GrammarExpr, GrammarLoweringEncoding, &'static str);

const fn exact(body: GrammarExpr, note: &'static str) -> Encoding {
    (body, GrammarLoweringEncoding::Exact, note)
}

const fn approximate(body: GrammarExpr, note: &'static str) -> Encoding {
    (body, GrammarLoweringEncoding::Approximate, note)
}

fn quoted_literal_runs(value: &str) -> GrammarExpr {
    let mut runs: Vec<String> = Vec::new();
    for character in value.chars() {
        if let Some(last) = runs.last_mut() {
            let candidate = format!("{last}{character}");
            if !(candidate.contains('"') && candidate.contains('\'')) {
                *last = candidate;
                continue;
            }
        }
        runs.push(character.to_string());
    }
    seq(runs.into_iter().map(literal).collect())
}

fn case_insensitive_sequence(value: &str) -> GrammarExpr {
    let mut items: Vec<GrammarExpr> = Vec::new();
    for character in value.chars() {
        if let Some(variants) = case_variants(character) {
            items.push(GrammarExpr::CharClass {
                negated: false,
                items: variants.into_iter().map(CharClassItem::Char).collect(),
            });
        } else if let Some(GrammarExpr::Terminal(last)) = items.last_mut() {
            last.push(character);
        } else {
            items.push(literal(character.to_string()));
        }
    }
    seq(items)
}

fn counted_repeat(item: &GrammarExpr, min: usize, max: Option<usize>) -> GrammarExpr {
    let mut items = vec![item.clone(); min];
    let rest = max.map_or_else(
        || Some(GrammarExpr::ZeroOrMore(Box::new(item.clone()))),
        |max| {
            (0..max.saturating_sub(min)).fold(None, |rest, _| {
                Some(GrammarExpr::Optional(Box::new(rest.map_or_else(
                    || item.clone(),
                    |rest| seq(vec![item.clone(), rest]),
                ))))
            })
        },
    );
    items.extend(rest);
    if items.is_empty() {
        GrammarExpr::Empty
    } else {
        seq(items)
    }
}

/// The helper body encoding `expr`'s `construct` in `format`, whether it is
/// exact and a note.
pub(super) fn encode(
    format: &str,
    construct: Construct,
    expr: &GrammarExpr,
    helper: &str,
) -> Result<Encoding, GrammarLoweringError> {
    let encoding = match (construct, expr) {
        (Construct::QuotedLiteral, GrammarExpr::Terminal(value)) => exact(
            quoted_literal_runs(value),
            "a literal with both quote characters as a sequence of literals",
        ),
        (Construct::LiteralInsensitive, GrammarExpr::TerminalInsensitive(value)) => exact(
            case_insensitive_sequence(value),
            "a case-insensitive literal as per-letter character sets",
        ),
        (Construct::CharRange | Construct::CharClass, _) => {
            let ranges = match expr {
                GrammarExpr::CharRange(start, end) => vec![(u32::from(*start), u32::from(*end))],
                GrammarExpr::CharClass { items, .. } => item_ranges(items),
                _ => Vec::new(),
            };
            match character_set_encoding(format, &merge_ranges(ranges))? {
                (body, true) => exact(body, "a character set as the characters or ranges it holds"),
                (body, false) => approximate(body, "a character set limited to printable ASCII"),
            }
        }
        (Construct::NegatedClass, GrammarExpr::CharClass { items, .. }) if format == "pest" => {
            exact(
                seq(vec![
                    GrammarExpr::Not(Box::new(GrammarExpr::CharClass {
                        negated: false,
                        items: items.clone(),
                    })),
                    GrammarExpr::AnyChar,
                ]),
                "a negated character class as a negative lookahead and any character",
            )
        }
        (Construct::NegatedClass, GrammarExpr::CharClass { items, .. }) => {
            match character_set_encoding(format, &complement_ranges(item_ranges(items)))? {
                (body, true) => exact(
                    body,
                    "a negated character class as the ranges of its complement",
                ),
                (body, false) => {
                    approximate(body, "a negated character class limited to printable ASCII")
                }
            }
        }
        (Construct::Any, _) => match character_set_encoding(format, &[(0, MAX_CODE_POINT)])? {
            (body, true) => exact(body, "any character as the range of every code point"),
            (body, false) => approximate(body, "any character limited to printable ASCII"),
        },
        (
            Construct::OrderedChoice | Construct::UnorderedChoice,
            GrammarExpr::Choice { alternatives, .. },
        ) => {
            let body = GrammarExpr::Choice {
                ordered: construct == Construct::UnorderedChoice,
                alternatives: alternatives.clone(),
            };
            if choice_order_irrelevant(alternatives) {
                exact(
                    body,
                    "a choice of literals none of which starts another, whose order is irrelevant",
                )
            } else if construct == Construct::OrderedChoice {
                approximate(body, "an ordered choice written as an unordered one")
            } else {
                approximate(body, "an unordered choice written as an ordered one")
            }
        }
        (Construct::NestedChoice, _) => exact(expr.clone(), "a nested choice as a rule of its own"),
        (
            Construct::ChoiceWithEmpty,
            GrammarExpr::Choice {
                ordered,
                alternatives,
            },
        ) => {
            let mut items: Vec<GrammarExpr> = alternatives
                .iter()
                .filter(|item| **item != GrammarExpr::Empty)
                .cloned()
                .collect();
            let item = if items.len() == 1 {
                items.remove(0)
            } else {
                GrammarExpr::Choice {
                    ordered: *ordered,
                    alternatives: items,
                }
            };
            exact(
                GrammarExpr::Optional(Box::new(item)),
                "a choice with the empty expression as an optional choice",
            )
        }
        (Construct::ParserCharacters, _) => exact(
            expr.clone(),
            "a character-level construct as a lexer rule of its own",
        ),
        (Construct::Optional, GrammarExpr::Optional(item)) => exact(
            unordered(vec![(**item).clone(), GrammarExpr::Empty]),
            "an optional item as a choice with the empty expression",
        ),
        (Construct::Repeat0, GrammarExpr::ZeroOrMore(item)) => exact(
            unordered(vec![
                seq(vec![
                    (**item).clone(),
                    GrammarExpr::NonTerminal(helper.to_owned()),
                ]),
                GrammarExpr::Empty,
            ]),
            "a repetition as a right-recursive rule",
        ),
        (Construct::Repeat1, GrammarExpr::OneOrMore(item)) => exact(
            unordered(vec![
                seq(vec![
                    (**item).clone(),
                    GrammarExpr::NonTerminal(helper.to_owned()),
                ]),
                (**item).clone(),
            ]),
            "a repetition as a right-recursive rule",
        ),
        (
            Construct::CountedRepeat | Construct::UnboundedRepeat,
            GrammarExpr::Repeat {
                expr: item,
                min,
                max,
            },
        ) => exact(
            counted_repeat(item, *min, *max),
            "a counted repetition as copies and nested optional items",
        ),
        (Construct::And, _) => approximate(
            empty_encoding(format),
            "a positive lookahead written as the empty expression",
        ),
        (Construct::Not, _) => approximate(
            empty_encoding(format),
            "a negative lookahead written as the empty expression",
        ),
        (
            Construct::LabeledCapture | Construct::UnlabeledCapture | Construct::LabelNotIdentifier,
            GrammarExpr::Capture { expr: item, .. },
        ) => exact((**item).clone(), "a capture as a rule of its own"),
        (Construct::Empty, _) => exact(
            empty_encoding(format),
            "the empty expression as the empty literal",
        ),
        _ => unreachable!("constructs_of pairs every construct with its expression"),
    };
    Ok(encoding)
}
