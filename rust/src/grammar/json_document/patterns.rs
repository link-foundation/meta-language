//! Adapt the catalog regular-expression syntax to JSON strings and Rust validation.
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::sync::LazyLock;

use regex::{Captures, Regex};

use crate::native_grammar_parser::{NativeNode, parse_native};

use super::super::{ExprBuilder, GrammarExpr};
use super::pattern_policy::{
    bounded_pattern_repetition, consuming_pattern_lower_bound, scalar_from_surrogate_pair,
};
use super::property_names::json_string_character;

const EXPRESSION: ExprBuilder = ExprBuilder;
const SCALARS: [(u32, u32); 2] = [(0, 0xd7ff), (0xe000, 0x10ffff)];
const DIGIT: [(u32, u32); 1] = [(48, 57)];
const WORD: [(u32, u32); 4] = [(48, 57), (65, 90), (95, 95), (97, 122)];
const WHITESPACE: [(u32, u32); 10] = [
    (9, 13),
    (32, 32),
    (160, 160),
    (0x1680, 0x1680),
    (0x2000, 0x200a),
    (0x2028, 0x2029),
    (0x202f, 0x202f),
    (0x205f, 0x205f),
    (0x3000, 0x3000),
    (0xfeff, 0xfeff),
];

type Intervals = Vec<(u32, u32)>;

fn union(mut ranges: Intervals) -> Intervals {
    ranges.sort_unstable();
    let mut result: Intervals = Vec::new();
    for (start, end) in ranges {
        if let Some(previous) = result
            .last_mut()
            .filter(|previous| previous.1 >= start.saturating_sub(1))
        {
            previous.1 = previous.1.max(end);
        } else {
            result.push((start, end));
        }
    }
    result
}

fn complement(ranges: &[(u32, u32)]) -> Intervals {
    let mut result = Vec::new();
    for (start, end) in SCALARS {
        let mut offset = start;
        for &(first, last) in ranges {
            if last < offset || first > end {
                continue;
            }
            if offset < first {
                result.push((offset, first - 1));
            }
            offset = last + 1;
        }
        if offset <= end {
            result.push((offset, end));
        }
    }
    result
}

fn single(value: u32) -> Option<Intervals> {
    char::from_u32(value).map(|_| vec![(value, value)])
}

fn character_ranges(term: &str, spelling: &str) -> Option<Intervals> {
    let mut characters = spelling.chars();
    let first = characters.next()?;
    match term {
        "pattern_character" | "class_character" => single(u32::from(first)),
        "any_character" => Some(complement(&[(10, 10), (13, 13), (0x2028, 0x2029)])),
        "identity_escape" => {
            let escaped = characters.next()?;
            "^$\\.*+?()[]{}|/-"
                .contains(escaped)
                .then_some(vec![(u32::from(escaped), u32::from(escaped))])
        }
        "control_escape" => {
            let value = match characters.next()? {
                '0' => 0,
                'b' => 8,
                't' => 9,
                'n' => 10,
                'v' => 11,
                'f' => 12,
                'r' => 13,
                'x' => u32::from_str_radix(spelling.get(2..)?, 16).ok()?,
                _ => return None,
            };
            single(value)
        }
        "control_letter_escape" => single(u32::from(characters.nth(1)?) % 32),
        "character_class_escape" => {
            let escaped = characters.next()?;
            let ranges = match escaped.to_ascii_lowercase() {
                'd' => &DIGIT[..],
                'w' => &WORD[..],
                's' => &WHITESPACE[..],
                'u' => {
                    let units = spelling.get(2..)?;
                    let digits = units
                        .strip_prefix('{')
                        .and_then(|rest| rest.strip_suffix('}'))
                        .unwrap_or(units);
                    return single(u32::from_str_radix(digits, 16).ok()?);
                }
                _ => return None,
            };
            Some(if escaped.is_ascii_uppercase() {
                complement(ranges)
            } else {
                ranges.to_vec()
            })
        }
        _ => None,
    }
}

fn children(node: &NativeNode) -> impl Iterator<Item = &NativeNode> {
    node.children.iter().map(|(child, _)| child)
}

fn spelling<'a>(source: &'a str, node: &NativeNode) -> &'a str {
    &source[node.start..node.end]
}

fn class_ranges(source: &str, node: &NativeNode) -> Option<Intervals> {
    let mut items = Vec::new();
    for child in children(node) {
        match child.term.as_str() {
            "[" | "]" | "^" => {}
            "class_range" => {
                let [start, _, end] = child.children.as_slice() else {
                    return None;
                };
                let lower = character_ranges(&start.0.term, spelling(source, &start.0))?;
                let upper = character_ranges(&end.0.term, spelling(source, &end.0))?;
                match (lower.as_slice(), upper.as_slice()) {
                    ([(low, low_end)], [(high_start, high)])
                        if low == low_end && high_start == high && low <= high =>
                    {
                        items.push((*low, *high))
                    }
                    _ => return None,
                }
            }
            _ => items.extend(character_ranges(&child.term, spelling(source, child))?),
        }
    }
    let ranges = union(items);
    Some(if spelling(source, node).starts_with("[^") {
        complement(&ranges)
    } else {
        ranges
    })
}

fn atom_ranges(source: &str, node: &NativeNode) -> Option<Intervals> {
    if node.term == "character_class" {
        class_ranges(source, node)
    } else {
        character_ranges(&node.term, spelling(source, node))
    }
}

fn normalize_surrogate_pair_escapes(source: &str) -> Cow<'_, str> {
    static PAIR: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\\\\|\\u([dD][89a-bA-B][0-9a-fA-F]{2})\\u([dD][c-fC-F][0-9a-fA-F]{2})")
            .expect("a valid surrogate-pair expression")
    });
    // Consume escaped backslashes before looking for Unicode-mode pairs.
    PAIR.replace_all(source, |captures: &Captures<'_>| {
        if let Some(lead) = captures.get(1) {
            let lead = u32::from_str_radix(lead.as_str(), 16).expect("a hexadecimal lead");
            let trail = u32::from_str_radix(&captures[2], 16).expect("a hexadecimal trail");
            let scalar = scalar_from_surrogate_pair(lead, trail);
            format!(r"\u{{{scalar:x}}}")
        } else {
            captures[0].to_owned()
        }
    })
}

fn regular_expression(source: &str) -> Option<NativeNode> {
    if source.len() > 1024 {
        return None;
    }
    let root = parse_native("native-regex", source);
    (!root.has_error && root.start == 0 && root.end == source.len()).then_some(root)
}

fn render_validation_pattern(source: &str, node: &NativeNode, out: &mut String) -> Option<()> {
    match node.term.as_str() {
        "pattern"
        | "term"
        | "alternation"
        | "anonymous_capturing_group"
        | "non_capturing_group" => {
            for child in children(node) {
                render_validation_pattern(source, child, out)?;
            }
        }
        "|" | "(" | "(?:" | ")" | "start_assertion" | "end_assertion" | "optional"
        | "one_or_more" | "zero_or_more" | "count_quantifier" => {
            out.push_str(spelling(source, node));
        }
        _ => {
            let ranges = atom_ranges(source, node)?;
            if ranges.is_empty() {
                out.push_str(r"[\p{Any}&&\P{Any}]");
                return Some(());
            }
            out.push('[');
            for (first, last) in ranges {
                write!(out, "\\u{{{first:x}}}").ok()?;
                if first != last {
                    write!(out, "-\\u{{{last:x}}}").ok()?;
                }
            }
            out.push(']');
        }
    }
    Some(())
}

pub(super) fn validation_pattern(source: &str) -> Result<Regex, regex::Error> {
    let source = normalize_surrogate_pair_escapes(source);
    if let Some(root) = regular_expression(&source) {
        let mut normalized = String::new();
        if render_validation_pattern(&source, &root, &mut normalized).is_some() {
            return Regex::new(&normalized);
        }
    }
    Regex::new(&source)
}

struct Match {
    nullable: bool,
    consuming: Option<GrammarExpr>,
}

impl Match {
    const NOTHING: Self = Self {
        nullable: false,
        consuming: None,
    };
    const EPSILON: Self = Self {
        nullable: true,
        consuming: None,
    };

    fn complete(self) -> Option<GrammarExpr> {
        if self.nullable {
            Some(
                self.consuming
                    .map_or_else(|| EXPRESSION.empty(), |expr| EXPRESSION.opt(expr)),
            )
        } else {
            self.consuming
        }
    }

    fn concatenate(self, right: Self) -> Self {
        let nullable = self.nullable && right.nullable;
        let mut alternatives = Vec::new();
        if let Some(left) = self.consuming
            && let Some(tail) = (Self {
                nullable: right.nullable,
                consuming: right.consuming.clone(),
            })
            .complete()
        {
            alternatives.push(EXPRESSION.seq([left, tail]));
        }
        if self.nullable
            && let Some(right) = right.consuming
        {
            alternatives.push(right);
        }
        Self {
            nullable,
            consuming: (!alternatives.is_empty())
                .then(|| EXPRESSION.choice_unordered(alternatives)),
        }
    }

    fn quantify(self, spelling: &str) -> Option<Self> {
        let (minimum, maximum) = match spelling.as_bytes().first()? {
            b'+' => (1, None),
            b'*' => (0, None),
            b'?' => (0, Some(1)),
            b'{' => {
                let counted = spelling
                    .trim_end_matches('?')
                    .strip_prefix('{')?
                    .strip_suffix('}')?;
                if let Some((first, last)) = counted.split_once(',') {
                    (
                        first.parse::<u32>().ok()?,
                        if last.is_empty() {
                            None
                        } else {
                            Some(last.parse().ok()?)
                        },
                    )
                } else {
                    let count = counted.parse().ok()?;
                    (count, Some(count))
                }
            }
            _ => return None,
        };
        if !bounded_pattern_repetition(f64::from(minimum), maximum.map_or(-1.0, f64::from)) {
            return None;
        }
        let minimum = consuming_pattern_lower_bound(self.nullable, f64::from(minimum)) as usize;
        let maximum = maximum.map(|maximum| maximum as usize);
        if maximum == Some(0) || (self.consuming.is_none() && minimum == 0) {
            return Some(Self::EPSILON);
        }
        let Some(consuming) = self.consuming else {
            return Some(Self::NOTHING);
        };
        Some(Self {
            nullable: minimum == 0,
            consuming: Some(EXPRESSION.repeat(consuming, 1.max(minimum), maximum)),
        })
    }
}

struct Lowering<'a, Register> {
    source: &'a str,
    register_pattern_rule: Register,
    characters: BTreeMap<Intervals, Option<GrammarExpr>>,
}

impl<Register: FnMut(GrammarExpr) -> GrammarExpr> Lowering<'_, Register> {
    fn atom(&mut self, node: &NativeNode, depth: usize) -> Option<Match> {
        if matches!(
            node.term.as_str(),
            "anonymous_capturing_group" | "non_capturing_group"
        ) {
            let inner = children(node).find(|child| child.term == "pattern")?;
            let result = self.pattern(inner, false, depth + 1)?;
            // Retain one consuming production when enclosing expressions
            // repeat or reference the group more than once.
            return Some(Match {
                nullable: result.nullable,
                consuming: result.consuming.map(&mut self.register_pattern_rule),
            });
        }
        let ranges = atom_ranges(self.source, node)?;
        let consuming = self
            .characters
            .entry(ranges.clone())
            .or_insert_with(|| json_string_character(&ranges).map(&mut self.register_pattern_rule))
            .clone();
        Some(Match {
            nullable: false,
            consuming,
        })
    }

    fn branch(&mut self, nodes: &[&NativeNode], outer: bool, depth: usize) -> Option<Match> {
        let start = nodes
            .first()
            .is_some_and(|node| node.term == "start_assertion");
        let end = nodes
            .last()
            .is_some_and(|node| node.term == "end_assertion");
        if !outer && (start || end) {
            return None;
        }
        let mut body = nodes[usize::from(start)..nodes.len() - usize::from(end)]
            .iter()
            .peekable();
        let mut result = Match::EPSILON;
        while let Some(node) = body.next() {
            let mut atom = self.atom(node, depth)?;
            if let Some(next) = body.next_if(|next| {
                matches!(
                    next.term.as_str(),
                    "optional" | "one_or_more" | "zero_or_more" | "count_quantifier"
                )
            }) {
                atom = atom.quantify(spelling(self.source, next))?;
            }
            result = result.concatenate(atom);
        }
        let longer = || Match {
            nullable: true,
            consuming: Some(EXPRESSION.repeat(EXPRESSION.nt("json_pattern_character"), 1, None)),
        };
        if outer && !start {
            result = longer().concatenate(result);
        }
        if outer && !end {
            result = result.concatenate(longer());
        }
        Some(result)
    }

    fn pattern(&mut self, node: &NativeNode, outer: bool, depth: usize) -> Option<Match> {
        if depth > 32 {
            return None;
        }
        let body = children(node).next()?;
        if body.term == "term" {
            return self.branch(&children(body).collect::<Vec<_>>(), outer, depth);
        }
        if body.term != "alternation" {
            return None;
        }
        let mut parts = vec![Vec::new()];
        for child in children(body) {
            match child.term.as_str() {
                "|" => parts.push(Vec::new()),
                "term" => parts.last_mut()?.extend(children(child)),
                _ => return None,
            }
        }
        let mut nullable = false;
        let mut alternatives = Vec::new();
        for part in parts {
            let result = self.branch(&part, outer, depth)?;
            nullable |= result.nullable;
            alternatives.extend(result.consuming);
        }
        Some(Match {
            nullable,
            consuming: (!alternatives.is_empty())
                .then(|| EXPRESSION.choice_unordered(alternatives)),
        })
    }
}

pub(super) enum PatternLowering {
    Match(GrammarExpr),
    Impossible,
}

/// Lower a supported pattern even when its language is empty.
pub(super) fn lower_json_string_pattern(
    source: &str,
    register_pattern_rule: impl FnMut(GrammarExpr) -> GrammarExpr,
) -> Option<PatternLowering> {
    if source.len() > 1024 {
        return None;
    }
    if source.is_empty() {
        return Some(PatternLowering::Match(EXPRESSION.repeat(
            EXPRESSION.nt("json_pattern_character"),
            0,
            None,
        )));
    }
    let source = normalize_surrogate_pair_escapes(source);
    let root = regular_expression(&source)?;
    Lowering {
        source: &source,
        register_pattern_rule,
        characters: BTreeMap::new(),
    }
    .pattern(&root, true, 0)
    .map(|result| {
        result
            .complete()
            .map_or(PatternLowering::Impossible, PatternLowering::Match)
    })
}

pub(super) fn any_json_pattern_character() -> GrammarExpr {
    json_string_character(&SCALARS).expect("the Unicode scalar domain is nonempty")
}
