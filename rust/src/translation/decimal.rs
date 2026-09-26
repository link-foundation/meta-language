//! Arbitrary-precision integer literals as decimal strings.
//!
//! The translator only compares, negates and re-bases literals (the JavaScript runtime uses
//! `BigInt` for the same steps), so a signed decimal string is enough.

use std::cmp::Ordering;
use std::fmt::Write as _;

/// A signed integer of any size, kept as its canonical decimal digits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decimal {
    pub negative: bool,
    /// Canonical digits: no leading zeros, `"0"` for zero.
    pub digits: String,
}

impl Decimal {
    /// Parses what `BigInt(text)` accepts for the translator's literals:
    /// decimal digits, or `0x`/`0o`/`0b` followed by digits of that radix.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim();
        let (negative, body) = text
            .strip_prefix('-')
            .map_or((false, text), |rest| (true, rest));
        let lower = body.to_ascii_lowercase();
        let (radix, digits) = [("0x", 16), ("0o", 8), ("0b", 2)]
            .into_iter()
            .find_map(|(prefix, radix)| lower.strip_prefix(prefix).map(|rest| (radix, rest)))
            .unwrap_or((10, lower.as_str()));
        if digits.is_empty() || (radix != 10 && negative) {
            return None;
        }
        // Little-endian base-10^9 limbs.
        let mut limbs: Vec<u64> = vec![0];
        for ch in digits.chars() {
            let digit = u64::from(ch.to_digit(radix)?);
            let mut carry = digit;
            for limb in &mut limbs {
                let value = *limb * u64::from(radix) + carry;
                *limb = value % 1_000_000_000;
                carry = value / 1_000_000_000;
            }
            while carry > 0 {
                limbs.push(carry % 1_000_000_000);
                carry /= 1_000_000_000;
            }
        }
        let mut out = String::new();
        for (index, limb) in limbs.iter().rev().enumerate() {
            if index == 0 {
                out.push_str(&limb.to_string());
            } else {
                let _ = write!(out, "{limb:09}");
            }
        }
        let zero = out == "0";
        Some(Self {
            negative: negative && !zero,
            digits: out,
        })
    }

    #[must_use]
    pub fn from_i128(value: i128) -> Self {
        Self {
            negative: value < 0,
            digits: value.unsigned_abs().to_string(),
        }
    }

    #[must_use]
    pub fn from_u128(value: u128) -> Self {
        Self {
            negative: false,
            digits: value.to_string(),
        }
    }

    #[must_use]
    pub fn negate(mut self) -> Self {
        if self.digits != "0" {
            self.negative = !self.negative;
        }
        self
    }

    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.negative
    }

    /// The value as an `i128`, when it fits.
    #[must_use]
    pub fn to_i128(&self) -> Option<i128> {
        let text = self.to_string();
        text.parse().ok()
    }
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            formatter.write_str("-")?;
        }
        formatter.write_str(&self.digits)
    }
}

fn magnitude(left: &str, right: &str) -> Ordering {
    left.len().cmp(&right.len()).then_with(|| left.cmp(right))
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.negative, other.negative) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (false, false) => magnitude(&self.digits, &other.digits),
            (true, true) => magnitude(&other.digits, &self.digits),
        }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::Decimal;

    #[test]
    fn parses_and_compares() {
        assert_eq!(Decimal::parse("0x1f").unwrap().to_string(), "31");
        assert_eq!(Decimal::parse("0b101").unwrap().to_string(), "5");
        assert_eq!(Decimal::parse("007").unwrap().to_string(), "7");
        assert_eq!(Decimal::parse("-0").unwrap().to_string(), "0");
        let big = Decimal::parse("1219326311370217952237463801111263526900").unwrap();
        assert_eq!(big.to_string(), "1219326311370217952237463801111263526900");
        assert!(big > Decimal::from_u128(u128::MAX));
        assert!(Decimal::parse("-5").unwrap() < Decimal::parse("-4").unwrap());
        assert!(Decimal::parse("-5").unwrap() < Decimal::parse("0").unwrap());
        assert_eq!(Decimal::parse("12a"), None);
    }
}
