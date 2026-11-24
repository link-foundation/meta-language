use std::collections::{BTreeMap, BTreeSet};

use super::super::{CharClassItem, ExprBuilder, GrammarExpr};

use super::pattern_policy::{intersect_scalar_intervals, surrogate_pair_boundaries};

const EXPRESSION: ExprBuilder = ExprBuilder;
const HEX: &[u8] = b"0123456789abcdef";
const SHORT_ESCAPES: [(char, char); 8] = [
    ('"', '"'),
    ('\\', '\\'),
    ('/', '/'),
    ('\u{8}', 'b'),
    ('\u{c}', 'f'),
    ('\n', 'n'),
    ('\r', 'r'),
    ('\t', 't'),
];

fn hex_digits(first: u32, last: u32) -> GrammarExpr {
    EXPRESSION.char_class(
        false,
        HEX[first as usize..=last as usize]
            .iter()
            .copied()
            .map(char::from)
            .flat_map(|digit| {
                [
                    Some(CharClassItem::Char(digit)),
                    digit
                        .is_ascii_alphabetic()
                        .then(|| CharClassItem::Char(digit.to_ascii_uppercase())),
                ]
                .into_iter()
                .flatten()
            }),
    )
}

fn hex_unit(unit: u32) -> GrammarExpr {
    EXPRESSION.seq(
        std::iter::once(EXPRESSION.term("\\u")).chain((0..4).rev().map(|index| {
            let digit = (unit >> (index * 4)) & 15;
            hex_digits(digit, digit)
        })),
    )
}

fn exact_character(value: char) -> GrammarExpr {
    let mut units = [0; 2];
    let escaped = EXPRESSION.seq(
        value
            .encode_utf16(&mut units)
            .iter()
            .map(|unit| hex_unit(u32::from(*unit))),
    );
    let mut alternatives = vec![escaped];
    if value >= ' ' && value != '"' && value != '\\' {
        alternatives.insert(0, EXPRESSION.term(value.to_string()));
    }
    if let Some((_, escape)) = SHORT_ESCAPES
        .iter()
        .find(|(character, _)| *character == value)
    {
        alternatives.push(EXPRESSION.term(format!("\\{escape}")));
    }
    EXPRESSION.choice_unordered(alternatives)
}

fn hex_interval(first: u32, last: u32, digits: u32) -> GrammarExpr {
    if first == 0 && last == 16_u32.pow(digits) - 1 {
        return EXPRESSION.repeat(hex_digits(0, 15), digits as usize, Some(digits as usize));
    }
    if digits == 1 {
        return hex_digits(first, last);
    }
    let base = 16_u32.pow(digits - 1);
    let lower = first / base;
    let upper = last / base;
    if lower == upper {
        return EXPRESSION.seq([
            hex_digits(lower, lower),
            hex_interval(first % base, last % base, digits - 1),
        ]);
    }
    let mut alternatives = vec![EXPRESSION.seq([
        hex_digits(lower, lower),
        hex_interval(first % base, base - 1, digits - 1),
    ])];
    if lower + 1 < upper {
        let remaining = (digits - 1) as usize;
        alternatives.push(EXPRESSION.seq([
            hex_digits(lower + 1, upper - 1),
            EXPRESSION.repeat(hex_digits(0, 15), remaining, Some(remaining)),
        ]));
    }
    alternatives.push(EXPRESSION.seq([
        hex_digits(upper, upper),
        hex_interval(0, last % base, digits - 1),
    ]));
    EXPRESSION.choice_unordered(alternatives)
}

fn unicode_interval(first: u32, last: u32) -> GrammarExpr {
    EXPRESSION.seq([EXPRESSION.term("\\u"), hex_interval(first, last, 4)])
}

fn surrogate_pair_interval(first: u32, last: u32) -> GrammarExpr {
    let (first_high, first_low, last_high, last_low) =
        surrogate_pair_boundaries(f64::from(first), f64::from(last));
    let (first_high, first_low, last_high, last_low) = (
        first_high as u32,
        first_low as u32,
        last_high as u32,
        last_low as u32,
    );
    if first_high == last_high {
        return EXPRESSION.seq([hex_unit(first_high), unicode_interval(first_low, last_low)]);
    }
    let mut alternatives = vec![
        EXPRESSION.seq([hex_unit(first_high), unicode_interval(first_low, 0xdfff)]),
        EXPRESSION.seq([hex_unit(last_high), unicode_interval(0xdc00, last_low)]),
    ];
    if first_high + 1 < last_high {
        alternatives.push(EXPRESSION.seq([
            unicode_interval(first_high + 1, last_high - 1),
            unicode_interval(0xdc00, 0xdfff),
        ]));
    }
    EXPRESSION.choice_unordered(alternatives)
}

fn intersect(first: u32, last: u32, low: u32, high: u32) -> Option<(u32, u32)> {
    let (start, end) = intersect_scalar_intervals(
        f64::from(first),
        f64::from(last),
        f64::from(low),
        f64::from(high),
    );
    (start <= end).then_some((start as u32, end as u32))
}

/// Match every JSON spelling of one Unicode scalar in the given intervals.
pub(super) fn json_string_character(ranges: &[(u32, u32)]) -> Option<GrammarExpr> {
    let mut direct = Vec::new();
    let mut escaped = Vec::new();
    for &(first, last) in ranges {
        if first == last {
            escaped.push(exact_character(char::from_u32(first)?));
            continue;
        }
        for (low, high) in [(32, 33), (35, 91), (93, 0xd7ff), (0xe000, 0x10ffff)] {
            if let Some((start, end)) = intersect(first, last, low, high) {
                direct.push(CharClassItem::Range(
                    char::from_u32(start)?,
                    char::from_u32(end)?,
                ));
            }
        }
        for (low, high) in [(0, 0xd7ff), (0xe000, 0xffff), (0x10000, 0x10ffff)] {
            if let Some((start, end)) = intersect(first, last, low, high) {
                escaped.push(if start < 0x10000 {
                    unicode_interval(start, end)
                } else {
                    surrogate_pair_interval(start, end)
                });
            }
        }
        for (value, escape) in SHORT_ESCAPES {
            if (first..=last).contains(&u32::from(value)) {
                escaped.push(EXPRESSION.term(format!("\\{escape}")));
            }
        }
    }
    if !direct.is_empty() {
        escaped.insert(0, EXPRESSION.char_class(false, direct));
    }
    (!escaped.is_empty()).then(|| EXPRESSION.choice_unordered(escaped))
}

fn units_except(first: u32, last: u32, excluded: impl Iterator<Item = u32>) -> Option<GrammarExpr> {
    let mut intervals = Vec::new();
    let mut start = first;
    for unit in excluded {
        if unit < start || unit > last {
            continue;
        }
        if unit > start {
            intervals.push(hex_interval(start, unit - 1, 4));
        }
        start = unit + 1;
    }
    if start <= last {
        intervals.push(hex_interval(start, last, 4));
    }
    (!intervals.is_empty()).then(|| {
        EXPRESSION.seq([
            EXPRESSION.term("\\u"),
            EXPRESSION.choice_unordered(intervals),
        ])
    })
}

fn different_character(excluded: &BTreeMap<char, PropertyName>) -> GrammarExpr {
    let direct = EXPRESSION.char_class(
        true,
        [
            CharClassItem::Char('"'),
            CharClassItem::Char('\\'),
            CharClassItem::Range('\0', '\u{1f}'),
        ]
        .into_iter()
        .chain(excluded.keys().copied().map(CharClassItem::Char)),
    );
    let abbreviations: Vec<_> = SHORT_ESCAPES
        .into_iter()
        .filter(|(value, _)| !excluded.contains_key(value))
        .map(|(_, escape)| CharClassItem::Char(escape))
        .collect();
    let mut basic_plane = BTreeSet::new();
    let mut pairs = BTreeMap::<u32, BTreeSet<u32>>::new();
    for value in excluded.keys() {
        let mut units = [0; 2];
        match value.encode_utf16(&mut units) {
            [unit] => {
                basic_plane.insert(u32::from(*unit));
            }
            [high, low] => {
                pairs
                    .entry(u32::from(*high))
                    .or_default()
                    .insert(u32::from(*low));
            }
            _ => unreachable!("one Unicode scalar"),
        }
    }
    let mut alternatives = vec![direct];
    alternatives.extend(units_except(0, 0xd7ff, basic_plane.iter().copied()));
    alternatives.extend(units_except(0xe000, 0xffff, basic_plane.iter().copied()));
    if !abbreviations.is_empty() {
        alternatives.push(EXPRESSION.seq([
            EXPRESSION.term("\\"),
            EXPRESSION.char_class(false, abbreviations),
        ]));
    }
    if let Some(high) = units_except(0xd800, 0xdbff, pairs.keys().copied()) {
        let low = units_except(0xdc00, 0xdfff, std::iter::empty()).expect("low-surrogate range");
        alternatives.push(EXPRESSION.seq([high, low]));
    }
    for (high, lows) in pairs {
        if let Some(low) = units_except(0xdc00, 0xdfff, lows.into_iter()) {
            alternatives.push(EXPRESSION.seq([hex_unit(high), low]));
        }
    }
    EXPRESSION.choice_unordered(alternatives)
}

#[derive(Default)]
struct PropertyName {
    terminal: bool,
    children: BTreeMap<char, Self>,
}

impl PropertyName {
    fn remaining(&self, arbitrary_tail: &GrammarExpr) -> GrammarExpr {
        let mut alternatives =
            vec![EXPRESSION.seq([different_character(&self.children), arbitrary_tail.clone()])];
        if !self.terminal {
            alternatives.push(EXPRESSION.empty());
        }
        alternatives.extend(self.children.iter().map(|(value, child)| {
            EXPRESSION.seq([exact_character(*value), child.remaining(arbitrary_tail)])
        }));
        EXPRESSION.choice_unordered(alternatives)
    }
}

/// Match a JSON key that decodes to something other than any declared key.
pub(super) fn additional_property_name(keys: &[&str]) -> GrammarExpr {
    let mut root = PropertyName::default();
    for key in keys {
        let mut node = &mut root;
        for value in key.chars() {
            node = node.children.entry(value).or_default();
        }
        node.terminal = true;
    }
    let arbitrary_tail = EXPRESSION.rep0(different_character(&BTreeMap::new()));
    EXPRESSION.seq([
        EXPRESSION.term("\""),
        root.remaining(&arbitrary_tail),
        EXPRESSION.term("\""),
    ])
}
