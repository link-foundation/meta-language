use serde_json::Value;

use super::super::{ExprBuilder, GrammarExpr};

type Expression = GrammarExpr;
const EXPRESSION: ExprBuilder = ExprBuilder;

fn digit(character: u8) -> Expression {
    EXPRESSION.term(char::from(character).to_string())
}

fn digit_range(first: u8, last: u8) -> Expression {
    if first == last {
        digit(first)
    } else {
        EXPRESSION.char_range(char::from(first), char::from(last))
    }
}

fn fixed_width(low: &[u8], high: &[u8]) -> Expression {
    let (first, remaining_low) = low.split_first().expect("nonempty number spelling");
    let (last, remaining_high) = high.split_first().expect("nonempty number spelling");
    let width = remaining_low.len();
    if width == 0 {
        return digit_range(*first, *last);
    }
    if first == last {
        return EXPRESSION.seq([digit(*first), fixed_width(remaining_low, remaining_high)]);
    }
    let mut alternatives = vec![EXPRESSION.seq([
        digit(*first),
        fixed_width(remaining_low, &vec![b'9'; width]),
    ])];
    if last - first > 1 {
        alternatives.push(EXPRESSION.seq([
            digit_range(first + 1, last - 1),
            EXPRESSION.repeat(digit_range(b'0', b'9'), width, Some(width)),
        ]));
    }
    alternatives.push(EXPRESSION.seq([
        digit(*last),
        fixed_width(&vec![b'0'; width], remaining_high),
    ]));
    EXPRESSION.choice_unordered(alternatives)
}

fn unsigned_range(min: u64, max: Option<u64>) -> Expression {
    let low = min.to_string();
    let high = max.unwrap_or(min).to_string();
    let widths = (low.len()..=high.len()).map(|width| {
        let start = if width == low.len() {
            low.clone()
        } else {
            format!("1{}", "0".repeat(width - 1))
        };
        let end = if max.is_some() && width == high.len() {
            high.clone()
        } else {
            "9".repeat(width)
        };
        fixed_width(start.as_bytes(), end.as_bytes())
    });
    let longer = max.is_none().then(|| {
        EXPRESSION.seq([
            digit_range(b'1', b'9'),
            EXPRESSION.repeat(digit_range(b'0', b'9'), high.len(), None),
        ])
    });
    EXPRESSION.choice_unordered(widths.chain(longer))
}

pub(super) fn signed_integer_range(first: Option<i64>, last: Option<i64>) -> Expression {
    let mut parts = Vec::new();
    if first.is_none_or(|value| value <= 0) {
        parts.push(EXPRESSION.seq([
            EXPRESSION.term("-"),
            unsigned_range(
                last.filter(|value| *value < 0).map_or(0, i64::unsigned_abs),
                first.map(i64::unsigned_abs),
            ),
        ]));
    }
    if last.is_none_or(|value| value >= 0) {
        parts.push(unsigned_range(
            first
                .filter(|value| *value > 0)
                .map_or(0, i64::unsigned_abs),
            last.map(i64::unsigned_abs),
        ));
    }
    EXPRESSION.choice_unordered(parts)
}

#[derive(Clone, Copy)]
struct Limit {
    value: f64,
    inclusive: bool,
}

impl Limit {
    const fn mirrored(self) -> Self {
        Self {
            value: -self.value,
            ..self
        }
    }

    fn parts(self) -> DecimalParts {
        let spelling = self.value.abs().to_string();
        let (whole, fraction) = spelling.split_once('.').unwrap_or((&spelling, ""));
        DecimalParts {
            whole: whole.parse().expect("bounded decimal integral part"),
            fraction: fraction.trim_end_matches('0').as_bytes().to_vec(),
            inclusive: self.inclusive,
        }
    }
}

struct DecimalParts {
    whole: u64,
    fraction: Vec<u8>,
    inclusive: bool,
}

fn alternatives(parts: Vec<Expression>) -> Option<Expression> {
    (!parts.is_empty()).then(|| EXPRESSION.choice_unordered(parts))
}

fn accepts_zeroes(bound: Option<&DecimalParts>, index: usize, lower: bool) -> bool {
    bound.is_none_or(|bound| {
        let remaining = bound.fraction.get(index..).unwrap_or_default();
        if remaining.iter().all(|digit| *digit == b'0') {
            bound.inclusive
        } else {
            !lower
        }
    })
}

fn fraction_digits(
    lower: Option<&DecimalParts>,
    upper: Option<&DecimalParts>,
    index: usize,
    consumed: bool,
) -> Option<Expression> {
    let arbitrary = digit_range(b'0', b'9');
    let least = usize::from(!consumed);
    if lower.is_none() && upper.is_none() {
        return Some(EXPRESSION.repeat(arbitrary, least, None));
    }
    let mut parts = Vec::new();
    if consumed && accepts_zeroes(lower, index, true) && accepts_zeroes(upper, index, false) {
        parts.push(EXPRESSION.empty());
    }
    let length = |bound: Option<&DecimalParts>| bound.map_or(0, |bound| bound.fraction.len());
    if index >= length(lower).max(length(upper)) {
        if let Some(upper) = upper {
            return (upper.inclusive && lower.is_none_or(|lower| lower.inclusive))
                .then(|| EXPRESSION.repeat(digit(b'0'), least, None));
        }
        if lower.is_some_and(|lower| lower.inclusive) {
            return Some(EXPRESSION.repeat(arbitrary, least, None));
        }
        return Some(EXPRESSION.seq([
            EXPRESSION.rep0(digit(b'0')),
            digit_range(b'1', b'9'),
            EXPRESSION.rep0(arbitrary),
        ]));
    }
    let at = |bound: &DecimalParts| bound.fraction.get(index).copied().unwrap_or(b'0');
    let first = lower.map_or(b'0', at);
    let last = upper.map_or(b'9', at);
    if first > last {
        return None;
    }
    if lower.is_some()
        && let Some(tail) = fraction_digits(lower, upper.filter(|_| first == last), index + 1, true)
    {
        parts.push(EXPRESSION.seq([digit(first), tail]));
    }
    if upper.is_some()
        && (lower.is_none() || first != last)
        && let Some(tail) = fraction_digits(None, upper, index + 1, true)
    {
        parts.push(EXPRESSION.seq([digit(last), tail]));
    }
    let inner_first = first + u8::from(lower.is_some());
    let inner_last = last - u8::from(upper.is_some());
    if inner_first <= inner_last {
        parts.push(EXPRESSION.seq([
            digit_range(inner_first, inner_last),
            EXPRESSION.rep0(arbitrary),
        ]));
    }
    alternatives(parts)
}

fn fractional_suffix(
    lower: Option<&DecimalParts>,
    upper: Option<&DecimalParts>,
) -> Option<Expression> {
    let decimals = fraction_digits(lower, upper, 0, false)
        .map(|digits| EXPRESSION.seq([EXPRESSION.term("."), digits]));
    if accepts_zeroes(lower, 0, true) && accepts_zeroes(upper, 0, false) {
        return Some(decimals.map_or_else(|| EXPRESSION.empty(), |part| EXPRESSION.opt(part)));
    }
    decimals
}

fn unsigned_decimal_range(lower: Option<Limit>, upper: Option<Limit>) -> Option<Expression> {
    let first = lower.map(Limit::parts);
    let last = upper.map(Limit::parts);
    let mut parts = Vec::new();
    let low_integer = first.as_ref().map_or(0, |first| first.whole + 1);
    let high_integer = last.as_ref().and_then(|last| last.whole.checked_sub(1));
    if last.is_none() || high_integer.is_some_and(|high| low_integer <= high) {
        parts.push(EXPRESSION.seq([
            unsigned_range(low_integer, high_integer),
            fractional_suffix(None, None).expect("unbounded fractional suffix"),
        ]));
    }
    if let Some(first) = &first
        && let Some(suffix) = fractional_suffix(
            Some(first),
            last.as_ref().filter(|last| last.whole == first.whole),
        )
    {
        parts.push(EXPRESSION.seq([EXPRESSION.term(first.whole.to_string()), suffix]));
    }
    if let Some(last) = &last
        && first.as_ref().is_none_or(|first| first.whole != last.whole)
        && let Some(suffix) = fractional_suffix(None, Some(last))
    {
        parts.push(EXPRESSION.seq([EXPRESSION.term(last.whole.to_string()), suffix]));
    }
    alternatives(parts)
}

fn narrower(inclusive: Option<f64>, exclusive: Option<f64>, lower: bool) -> Option<Limit> {
    if let Some(value) = inclusive
        && exclusive.is_none_or(|exclusive| {
            if lower {
                value > exclusive
            } else {
                value < exclusive
            }
        })
    {
        return Some(Limit {
            value,
            inclusive: true,
        });
    }
    exclusive.map(|value| Limit {
        value,
        inclusive: false,
    })
}

pub(super) enum DecimalRange {
    Unsupported,
    Empty,
    Allowed(Expression),
}

pub(super) fn signed_decimal_range(schema: &Value) -> DecimalRange {
    let number = |name| schema.get(name).and_then(Value::as_f64);
    let first = narrower(number("minimum"), number("exclusiveMinimum"), true);
    let last = narrower(number("maximum"), number("exclusiveMaximum"), false);
    if [first, last].into_iter().flatten().any(|limit| {
        limit.value.abs() >= 9_007_199_254_740_991.0 || limit.parts().fraction.len() > 128
    }) {
        return DecimalRange::Unsupported;
    }
    if let (Some(first), Some(last)) = (first, last)
        && (first.value > last.value
            || (first.value >= last.value && !(first.inclusive && last.inclusive)))
    {
        return DecimalRange::Empty;
    }
    let negative =
        if first.is_none_or(|limit| limit.value < 0.0 || (limit.value <= 0.0 && limit.inclusive)) {
            unsigned_decimal_range(
                last.filter(|limit| limit.value <= 0.0).map(Limit::mirrored),
                first.map(Limit::mirrored),
            )
            .map(|digits| EXPRESSION.seq([EXPRESSION.term("-"), digits]))
        } else {
            None
        };
    let positive =
        if last.is_none_or(|limit| limit.value > 0.0 || (limit.value >= 0.0 && limit.inclusive)) {
            unsigned_decimal_range(first.filter(|limit| limit.value >= 0.0), last)
        } else {
            None
        };
    alternatives(negative.into_iter().chain(positive).collect())
        .map_or(DecimalRange::Empty, DecimalRange::Allowed)
}
