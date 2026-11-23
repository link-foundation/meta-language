use std::cmp::Ordering;

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
