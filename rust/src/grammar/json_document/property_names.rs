use std::collections::{BTreeMap, BTreeSet};

use super::super::{CharClassItem, ExprBuilder, GrammarExpr};

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
        alternatives.push(EXPRESSION.term(value.to_string()));
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
