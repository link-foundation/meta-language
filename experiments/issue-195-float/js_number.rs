// ECMAScript Number::toString for f64 from Rust's shortest round-trip digits;
// checked against V8 on bits.txt / expected.txt.
//   rustc -O js_number.rs -o /tmp/js_number && /tmp/js_number < bits.txt | diff - expected.txt
use std::io::BufRead;

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
    // Rust's shortest digits round a tie away from zero; ECMAScript takes the even neighbour.
    let (exact, _) = decimal(&format!("{:.800e}", x.abs()));
    let rest = &exact[digits.len().min(exact.len())..];
    if digits.len() > 1 && rest.starts_with('5') && rest[1..].bytes().all(|b| b == b'0') {
        let last = digits.as_bytes()[digits.len() - 1] - b'0';
        let down = exact[..digits.len()].to_string();
        let other = if digits == down { increment(&down) } else { Some(down) };
        if let Some(other) = other {
            let back = format!("{}.{}e{}", &other[..1], &other[1..], n - 1);
            if last % 2 == 1 && back.parse::<f64>() == Ok(x.abs()) {
                digits = other.trim_end_matches('0').to_string();
            }
        }
    }
    let k = digits.len() as i32;
    let body = if k <= n && n <= 21 {
        format!("{digits}{}", "0".repeat((n - k) as usize))
    } else if 0 < n && n <= 21 {
        format!("{}.{}", &digits[..n as usize], &digits[n as usize..])
    } else if -6 < n && n <= 0 {
        format!("0.{}{digits}", "0".repeat((-n) as usize))
    } else {
        let sign = if n - 1 < 0 { '-' } else { '+' };
        let power = (n - 1).abs();
        if k == 1 { format!("{digits}e{sign}{power}") } else { format!("{}.{}e{sign}{power}", &digits[..1], &digits[1..]) }
    };
    if x < 0.0 { format!("-{body}") } else { body }
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

/// The significant digits without trailing zeros, and the decimal exponent
/// `n` with value `0.digits × 10^n`, of Rust's `{:e}` text.
fn decimal(text: &str) -> (String, i32) {
    let (mantissa, exponent) = text.split_once('e').expect("exponent");
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let digits = digits.trim_end_matches('0').to_string();
    (if digits.is_empty() { String::from("0") } else { digits }, exponent.parse::<i32>().expect("exponent") + 1)
}

fn main() {
    for line in std::io::stdin().lock().lines() {
        let bits: u64 = line.unwrap().trim().parse().unwrap();
        println!("{}", js_number(f64::from_bits(bits)));
    }
}
