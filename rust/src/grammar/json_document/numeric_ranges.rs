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
    if low.iter().all(|&digit| digit == b'0') && high.iter().all(|&digit| digit == b'9') {
        return EXPRESSION.repeat(digit_range(b'0', b'9'), low.len(), Some(low.len()));
    }
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
    exclude_integers: bool,
) -> Option<Expression> {
    // A zero fractional part represents an integer even with a decimal point.
    let strict_zero = DecimalParts {
        whole: 0,
        fraction: Vec::new(),
        inclusive: false,
    };
    let lower = if exclude_integers && lower.is_none_or(|limit| limit.fraction.is_empty()) {
        Some(&strict_zero)
    } else {
        lower
    };
    let decimals = fraction_digits(lower, upper, 0, false)
        .map(|digits| EXPRESSION.seq([EXPRESSION.term("."), digits]));
    if accepts_zeroes(lower, 0, true) && accepts_zeroes(upper, 0, false) {
        return Some(decimals.map_or_else(|| EXPRESSION.empty(), |part| EXPRESSION.opt(part)));
    }
    decimals
}

fn unsigned_decimal_range(
    lower: Option<Limit>,
    upper: Option<Limit>,
    exclude_integers: bool,
) -> Option<Expression> {
    let first = lower.map(Limit::parts);
    let last = upper.map(Limit::parts);
    let mut parts = Vec::new();
    let low_integer = first.as_ref().map_or(0, |first| first.whole + 1);
    let high_integer = last.as_ref().and_then(|last| last.whole.checked_sub(1));
    if last.is_none() || high_integer.is_some_and(|high| low_integer <= high) {
        parts.push(EXPRESSION.seq([
            unsigned_range(low_integer, high_integer),
            fractional_suffix(None, None, exclude_integers).expect("unbounded fractional suffix"),
        ]));
    }
    if let Some(first) = &first
        && let Some(suffix) = fractional_suffix(
            Some(first),
            last.as_ref().filter(|last| last.whole == first.whole),
            exclude_integers,
        )
    {
        parts.push(EXPRESSION.seq([EXPRESSION.term(first.whole.to_string()), suffix]));
    }
    if let Some(last) = &last
        && first.as_ref().is_none_or(|first| first.whole != last.whole)
        && let Some(suffix) = fractional_suffix(None, Some(last), exclude_integers)
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

fn decimal_limits(schema: &Value) -> (Option<Limit>, Option<Limit>) {
    let number = |name| schema.get(name).and_then(Value::as_f64);
    (
        narrower(number("minimum"), number("exclusiveMinimum"), true),
        narrower(number("maximum"), number("exclusiveMaximum"), false),
    )
}

fn empty_decimal_interval(first: Option<Limit>, last: Option<Limit>) -> bool {
    matches!((first, last), (Some(first), Some(last)) if first.value > last.value
        || (first.value >= last.value && !(first.inclusive && last.inclusive)))
}

fn decimal_range(
    first: Option<Limit>,
    last: Option<Limit>,
    exclude_integers: bool,
) -> DecimalRange {
    if [first, last].into_iter().flatten().any(|limit| {
        limit.value.abs() >= 9_007_199_254_740_991.0 || limit.parts().fraction.len() > 128
    }) {
        return DecimalRange::Unsupported;
    }
    if empty_decimal_interval(first, last) {
        return DecimalRange::Empty;
    }
    let negative =
        if first.is_none_or(|limit| limit.value < 0.0 || (limit.value <= 0.0 && limit.inclusive)) {
            unsigned_decimal_range(
                last.filter(|limit| limit.value <= 0.0).map(Limit::mirrored),
                first.map(Limit::mirrored),
                exclude_integers,
            )
            .map(|digits| EXPRESSION.seq([EXPRESSION.term("-"), digits]))
        } else {
            None
        };
    let positive =
        if last.is_none_or(|limit| limit.value > 0.0 || (limit.value >= 0.0 && limit.inclusive)) {
            unsigned_decimal_range(
                first.filter(|limit| limit.value >= 0.0),
                last,
                exclude_integers,
            )
        } else {
            None
        };
    alternatives(negative.into_iter().chain(positive).collect())
        .map_or(DecimalRange::Empty, DecimalRange::Allowed)
}

pub(super) fn signed_decimal_range(schema: &Value) -> DecimalRange {
    let (first, last) = decimal_limits(schema);
    decimal_range(first, last, false)
}

#[derive(Clone, Copy)]
struct NumericInterval {
    first: Option<Limit>,
    last: Option<Limit>,
}

impl NumericInterval {
    fn contains(self, other: Self) -> bool {
        let includes = |outer: Option<Limit>, inner: Option<Limit>, lower| {
            outer.is_none_or(|outer| {
                inner.is_some_and(|inner| {
                    let position = outer.value.partial_cmp(&inner.value);
                    match position {
                        Some(std::cmp::Ordering::Equal) => outer.inclusive || !inner.inclusive,
                        Some(std::cmp::Ordering::Less) => lower,
                        Some(std::cmp::Ordering::Greater) => !lower,
                        None => false,
                    }
                })
            })
        };
        includes(self.first, other.first, true) && includes(self.last, other.last, false)
    }

    fn integer_expression(self) -> Option<Expression> {
        let spelling = |value| format!("{value:.0}").parse::<i64>().expect("safe endpoint");
        let first = self.first.map(|limit| {
            spelling(if limit.inclusive {
                limit.value.ceil()
            } else {
                limit.value.floor() + 1.0
            })
        });
        let last = self.last.map(|limit| {
            spelling(if limit.inclusive {
                limit.value.floor()
            } else {
                limit.value.ceil() - 1.0
            })
        });
        if matches!((first, last), (Some(first), Some(last)) if first > last) {
            return None;
        }
        Some(EXPRESSION.seq([
            signed_integer_range(first, last),
            EXPRESSION.opt(EXPRESSION.seq([
                EXPRESSION.term("."),
                EXPRESSION.repeat(EXPRESSION.term("0"), 1, None),
            ])),
        ]))
    }
}

pub(super) type NumericMembership = (Vec<usize>, Expression);

pub(super) fn partition_numeric_ranges(
    branches: &[(usize, &str, Value)],
    remaining_work: &mut usize,
) -> Option<Vec<NumericMembership>> {
    let mut ranges = Vec::new();
    let mut endpoints = Vec::new();
    for (index, name, limits) in branches {
        let (first, last) = decimal_limits(limits);
        if !empty_decimal_interval(first, last) {
            ranges.push((*index, *name, NumericInterval { first, last }));
            endpoints.extend([first, last].into_iter().flatten().map(|bound| bound.value));
        }
    }
    endpoints.sort_by(f64::total_cmp);
    endpoints.dedup_by(|left, right| {
        f64::partial_cmp(left, right).is_some_and(std::cmp::Ordering::is_eq)
    });
    let cells = endpoints.len().checked_mul(2)?.checked_add(1)?;
    let mut work = cells.checked_mul(ranges.len().checked_add(32)?)?;
    for &value in &endpoints {
        if value.abs() >= 9_007_199_254_740_991.0 {
            return None;
        }
        let digits = Limit {
            value,
            inclusive: false,
        }
        .parts()
        .fraction
        .len();
        if digits > 128 {
            return None;
        }
        work = work.checked_add(4 * digits)?;
    }
    *remaining_work = remaining_work.checked_sub(work)?;

    let mut variants = Vec::new();
    let mut first = None;
    for value in endpoints {
        let open = Some(Limit {
            value,
            inclusive: false,
        });
        let closed = Some(Limit {
            value,
            inclusive: true,
        });
        numeric_cell_membership(
            &ranges,
            NumericInterval { first, last: open },
            &mut variants,
        )?;
        numeric_cell_membership(
            &ranges,
            NumericInterval {
                first: closed,
                last: closed,
            },
            &mut variants,
        )?;
        first = open;
    }
    numeric_cell_membership(
        &ranges,
        NumericInterval { first, last: None },
        &mut variants,
    )?;
    Some(variants)
}

fn numeric_cell_membership(
    ranges: &[(usize, &str, NumericInterval)],
    cell: NumericInterval,
    variants: &mut Vec<NumericMembership>,
) -> Option<()> {
    let mut integral = Vec::new();
    let mut number = Vec::new();
    for &(index, name, range) in ranges {
        if range.contains(cell) {
            integral.push(index);
            if name == "number" {
                number.push(index);
            }
        }
    }
    let integral_only = integral.len() != number.len();
    if !number.is_empty() {
        let expression = if !integral_only && cell.first.is_none() && cell.last.is_none() {
            DecimalRange::Allowed(EXPRESSION.nt("number"))
        } else {
            decimal_range(cell.first, cell.last, integral_only)
        };
        match expression {
            DecimalRange::Unsupported => return None,
            DecimalRange::Empty => {}
            DecimalRange::Allowed(expression) => variants.push((number, expression)),
        }
    }
    if integral_only && let Some(expression) = cell.integer_expression() {
        variants.push((integral, expression));
    }
    Some(())
}

struct DecimalEndpoint {
    value: f64,
    at: i64,
    after: i64,
}

pub(super) fn exclusive_decimal_range<'schema>(
    branch_limits: impl IntoIterator<Item = &'schema Value>,
    exclude_integers: bool,
) -> DecimalRange {
    let mut endpoints = Vec::new();
    let mut coverage = 0;
    for schema in branch_limits {
        let (first, last) = decimal_limits(schema);
        if empty_decimal_interval(first, last) {
            continue;
        }
        if let Some(first) = first {
            endpoints.push(DecimalEndpoint {
                value: first.value,
                at: i64::from(first.inclusive),
                after: 1,
            });
        } else {
            coverage += 1;
        }
        if let Some(last) = last {
            endpoints.push(DecimalEndpoint {
                value: last.value,
                at: -i64::from(!last.inclusive),
                after: -1,
            });
        }
    }
    endpoints.sort_by(|left, right| left.value.total_cmp(&right.value));
    let mut intervals = Vec::new();
    let mut first = None;
    for changes in endpoints.chunk_by(|left, right| {
        left.value
            .partial_cmp(&right.value)
            .is_some_and(std::cmp::Ordering::is_eq)
    }) {
        // Signed zeroes share an endpoint and therefore one branch count.
        let value = changes[0].value;
        let at = coverage + changes.iter().map(|change| change.at).sum::<i64>();
        let after = coverage + changes.iter().map(|change| change.after).sum::<i64>();
        let open = Some(Limit {
            value,
            inclusive: false,
        });
        let closed = Some(Limit {
            value,
            inclusive: true,
        });
        if coverage == 1 && at != 1 {
            intervals.push((first, open));
        }
        if coverage != 1 && at == 1 {
            first = closed;
        }
        if at == 1 && after != 1 {
            intervals.push((first, closed));
        }
        if at != 1 && after == 1 {
            first = open;
        }
        coverage = after;
    }
    if coverage == 1 {
        intervals.push((first, None));
    }
    if !exclude_integers && matches!(intervals.as_slice(), [(None, None)]) {
        return DecimalRange::Allowed(EXPRESSION.nt("number"));
    }
    let mut parts = Vec::new();
    for (first, last) in intervals {
        match decimal_range(first, last, exclude_integers) {
            DecimalRange::Unsupported => return DecimalRange::Unsupported,
            DecimalRange::Empty => {}
            DecimalRange::Allowed(expression) => parts.push(expression),
        }
    }
    alternatives(parts).map_or(DecimalRange::Empty, DecimalRange::Allowed)
}
