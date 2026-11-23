use std::cmp::Ordering;

use super::super::{CharClassItem, ExprBuilder, GrammarExpr};

/// A JSON number normalized to a sign, decimal position and significant digits.
pub(super) struct NumberSpelling {
    sign: i8,
    position: i64,
    digits: String,
}

impl NumberSpelling {
    pub(super) fn new(source: &str) -> Self {
        let negative = source.starts_with('-');
        let unsigned = source.strip_prefix('-').unwrap_or(source);
        let (mantissa, exponent) = unsigned.split_once(['e', 'E']).unwrap_or((unsigned, "0"));
        let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let mut coefficient = String::with_capacity(whole.len() + fraction.len());
        coefficient.push_str(whole);
        coefficient.push_str(fraction);
        let digits = coefficient.trim_start_matches('0');
        if digits.is_empty() {
            return Self {
                sign: 0,
                position: 0,
                digits: String::new(),
            };
        }
        let first = coefficient.len() - digits.len();
        let position = whole.len() as i64 - first as i64;
        let exponent = exponent.parse::<i64>().unwrap_or_else(|_| {
            if exponent.starts_with('-') {
                i64::MIN
            } else {
                i64::MAX
            }
        });
        Self {
            sign: if negative { -1 } else { 1 },
            position: position.saturating_add(exponent),
            digits: digits.trim_end_matches('0').to_owned(),
        }
    }

    pub(super) fn is_integer(&self) -> bool {
        self.position >= self.digits.len() as i64
    }

    pub(super) fn is_zero(&self) -> bool {
        self.sign == 0
    }

    pub(super) fn constant_expression(&self) -> GrammarExpr {
        let expression = ExprBuilder;
        let zero = expression.term("0");
        let leading_zeroes = expression.rep0(zero.clone());
        let exponent_marker = expression.char_class(false, ['e', 'E'].map(CharClassItem::Char));
        let exponent_sign = expression.char_class(false, ['+', '-'].map(CharClassItem::Char));
        let zero_fraction =
            expression.opt(expression.seq([expression.term("."), expression.rep1(zero.clone())]));
        if self.is_zero() {
            return expression.seq([
                expression.opt(expression.term("-")),
                zero,
                zero_fraction,
                expression.opt(expression.seq([
                    exponent_marker,
                    expression.opt(exponent_sign),
                    expression.rep1(expression.char_range('0', '9')),
                ])),
            ]);
        }

        let length = self.digits.len() as i64;
        let positions = std::iter::once(self.position)
            .chain((-1..=length + 1).filter(|&index| index != self.position));
        let forms = positions.map(|index| {
            let exponent = self.position - index;
            let mantissa = if index >= length {
                expression.seq([
                    expression.term(format!(
                        "{}{}",
                        self.digits,
                        "0".repeat((index - length) as usize)
                    )),
                    zero_fraction.clone(),
                ])
            } else {
                let decimal = if index > 0 {
                    let (whole, fraction) = self.digits.split_at(index as usize);
                    format!("{whole}.{fraction}")
                } else {
                    format!("0.{}{}", "0".repeat((-index) as usize), self.digits)
                };
                expression.seq([expression.term(decimal), leading_zeroes.clone()])
            };
            let scale = if exponent == 0 {
                expression.opt(expression.seq([
                    exponent_marker.clone(),
                    expression.opt(exponent_sign.clone()),
                    expression.rep1(zero.clone()),
                ]))
            } else {
                let sign = if exponent < 0 {
                    expression.term("-")
                } else {
                    expression.opt(expression.term("+"))
                };
                expression.seq([
                    exponent_marker.clone(),
                    sign,
                    leading_zeroes.clone(),
                    expression.term(exponent.unsigned_abs().to_string()),
                ])
            };
            expression.seq([mantissa, scale])
        });
        expression.seq([
            if self.sign < 0 {
                expression.term("-")
            } else {
                expression.empty()
            },
            expression.choice_unordered(forms),
        ])
    }

    pub(super) fn compare(&self, other: &Self) -> Ordering {
        let sign = self.sign.cmp(&other.sign);
        if sign != Ordering::Equal {
            return sign;
        }
        let magnitude = self
            .position
            .cmp(&other.position)
            .then_with(|| self.digits.cmp(&other.digits));
        if self.sign < 0 {
            magnitude.reverse()
        } else {
            magnitude
        }
    }
}
