//! ECMAScript `Number::toString` for binary64.
//!
//! The JavaScript frontend keeps a Number literal as `String(value)`, the
//! text JavaScript prints, so both runtimes agree on it. Rust's `{:e}` gives
//! the shortest digits that read back as the value; ECMAScript picks the
//! even neighbour where Rust rounds a tie away from zero.
//!
//! The emitted Rust prelude (`ml_number::js_number`) is the same algorithm.

/// `String(x)` for a JavaScript Number `x`.
#[must_use]
pub fn js_number(x: f64) -> String {
    if x.is_nan() {
        return String::from("NaN");
    }
    if x.is_infinite() {
        return String::from(if x > 0.0 { "Infinity" } else { "-Infinity" });
    }
    if x == 0.0 {
        return String::from("0");
    }
    let (mut digits, n) = decimal(&format!("{:e}", x.abs()));
    let (exact, _) = decimal(&format!("{:.800e}", x.abs()));
    let rest = &exact[digits.len().min(exact.len())..];
    if digits.len() > 1 && rest.starts_with('5') && rest[1..].bytes().all(|b| b == b'0') {
        let last = digits.as_bytes()[digits.len() - 1] - b'0';
        let down = exact[..digits.len()].to_string();
        let other = if digits == down {
            increment(&down)
        } else {
            Some(down)
        };
        if let Some(other) = other {
            let back = format!("{}.{}e{}", &other[..1], &other[1..], n - 1);
            if last % 2 == 1 && back.parse::<f64>() == Ok(x.abs()) {
                digits = other.trim_end_matches('0').to_string();
            }
        }
    }
    let k = i32::try_from(digits.len()).unwrap_or(i32::MAX);
    let width = |count: i32| usize::try_from(count).unwrap_or(0);
    let body = if k <= n && n <= 21 {
        format!("{digits}{}", "0".repeat(width(n - k)))
    } else if 0 < n && n <= 21 {
        format!("{}.{}", &digits[..width(n)], &digits[width(n)..])
    } else if -6 < n && n <= 0 {
        format!("0.{}{digits}", "0".repeat(width(-n)))
    } else {
        let sign = if n - 1 < 0 { '-' } else { '+' };
        let power = (n - 1).abs();
        if k == 1 {
            format!("{digits}e{sign}{power}")
        } else {
            format!("{}.{}e{sign}{power}", &digits[..1], &digits[1..])
        }
    };
    if x < 0.0 { format!("-{body}") } else { body }
}

/// `String(Number(text))` for the text of a JavaScript numeric literal
/// without separators or suffix: decimal, or `0x`, `0o`, `0b` integers.
#[must_use]
pub fn js_number_literal(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    let value = if ["0x", "0o", "0b"]
        .iter()
        .any(|prefix| lower.starts_with(prefix))
    {
        super::decimal::Decimal::parse(&lower)?
            .digits
            .parse::<f64>()
            .ok()?
    } else {
        lower.parse::<f64>().ok()?
    };
    Some(js_number(value))
}

/// A decimal digit string plus one, or `None` when it gains a digit.
fn increment(digits: &str) -> Option<String> {
    let mut bytes = digits.as_bytes().to_vec();
    for index in (0..bytes.len()).rev() {
        if bytes[index] == b'9' {
            bytes[index] = b'0';
        } else {
            bytes[index] += 1;
            return String::from_utf8(bytes).ok();
        }
    }
    None
}

/// The significant digits without trailing zeros, and the exponent `n` with
/// value `0.digits × 10^n`, of Rust's `{:e}` text.
fn decimal(text: &str) -> (String, i32) {
    let (mantissa, exponent) = text.split_once('e').unwrap_or((text, "0"));
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let digits = digits.trim_end_matches('0').to_string();
    (
        if digits.is_empty() {
            String::from("0")
        } else {
            digits
        },
        exponent.parse::<i32>().unwrap_or(0) + 1,
    )
}
