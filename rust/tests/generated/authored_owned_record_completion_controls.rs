// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

/// Unbounded integers for the portable core's naturals and integers.
pub mod ml {
    use std::cmp::Ordering;
    use std::fmt;

    /// Sign and magnitude; the magnitude is little-endian base 2^32 without
    /// leading zero limbs, and zero is never negative.
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Big {
        negative: bool,
        magnitude: Vec<u32>,
    }

    fn trim(mut magnitude: Vec<u32>) -> Vec<u32> {
        while magnitude.last() == Some(&0) {
            magnitude.pop();
        }
        magnitude
    }

    fn compare_magnitude(a: &[u32], b: &[u32]) -> Ordering {
        a.len().cmp(&b.len()).then_with(|| a.iter().rev().cmp(b.iter().rev()))
    }

    fn add_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(a.len().max(b.len()) + 1);
        let mut carry = 0u64;
        for index in 0..a.len().max(b.len()) {
            let sum = u64::from(*a.get(index).unwrap_or(&0)) + u64::from(*b.get(index).unwrap_or(&0)) + carry;
            out.push(sum as u32);
            carry = sum >> 32;
        }
        if carry > 0 {
            out.push(carry as u32);
        }
        out
    }

    /// |a| - |b| where |a| >= |b|.
    fn sub_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(a.len());
        let mut borrow = 0i64;
        for (index, limb) in a.iter().enumerate() {
            let mut difference = i64::from(*limb) - i64::from(*b.get(index).unwrap_or(&0)) - borrow;
            borrow = 0;
            if difference < 0 {
                difference += 1 << 32;
                borrow = 1;
            }
            out.push(difference as u32);
        }
        trim(out)
    }

    fn mul_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = vec![0u32; a.len() + b.len()];
        for (i, x) in a.iter().enumerate() {
            let mut carry = 0u64;
            for (j, y) in b.iter().enumerate() {
                let product = u64::from(out[i + j]) + u64::from(*x) * u64::from(*y) + carry;
                out[i + j] = product as u32;
                carry = product >> 32;
            }
            let mut index = i + b.len();
            while carry > 0 {
                let sum = u64::from(out[index]) + carry;
                out[index] = sum as u32;
                carry = sum >> 32;
                index += 1;
            }
        }
        trim(out)
    }

    /// Quotient and remainder of magnitudes; the divisor is not zero.
    fn divmod_magnitude(a: &[u32], b: &[u32]) -> (Vec<u32>, Vec<u32>) {
        let mut quotient = vec![0u32; a.len()];
        if b.len() == 1 {
            let divisor = u64::from(b[0]);
            let mut remainder = 0u64;
            for index in (0..a.len()).rev() {
                let current = (remainder << 32) | u64::from(a[index]);
                quotient[index] = (current / divisor) as u32;
                remainder = current % divisor;
            }
            return (trim(quotient), trim(vec![remainder as u32]));
        }
        let mut remainder: Vec<u32> = Vec::new();
        for bit in (0..a.len() * 32).rev() {
            let mut carry = (a[bit / 32] >> (bit % 32)) & 1;
            for limb in remainder.iter_mut() {
                let next = *limb >> 31;
                *limb = (*limb << 1) | carry;
                carry = next;
            }
            if carry != 0 {
                remainder.push(carry);
            }
            if compare_magnitude(&remainder, b) != Ordering::Less {
                remainder = sub_magnitude(&remainder, b);
                quotient[bit / 32] |= 1 << (bit % 32);
            }
        }
        (trim(quotient), remainder)
    }

    impl Big {
        fn from_parts(negative: bool, magnitude: Vec<u32>) -> Self {
            let magnitude = trim(magnitude);
            Self { negative: negative && !magnitude.is_empty(), magnitude }
        }

        pub fn zero() -> Self {
            Self::from_parts(false, Vec::new())
        }

        pub fn from_i128(value: i128) -> Self {
            let mut big = Self::from_u128(value.unsigned_abs());
            big.negative = value < 0;
            big
        }

        pub fn from_u128(mut value: u128) -> Self {
            let mut magnitude = Vec::new();
            while value > 0 {
                magnitude.push(value as u32);
                value >>= 32;
            }
            Self::from_parts(false, magnitude)
        }

        /// A decimal literal with an optional leading minus sign.
        pub fn parse(text: &str) -> Self {
            let (negative, digits) = match text.strip_prefix('-') {
                Some(rest) => (true, rest),
                None => (false, text),
            };
            let ten = Self::from_i128(10);
            let mut value = Self::zero();
            for digit in digits.chars() {
                let digit = digit.to_digit(10).expect("decimal literal");
                value = value.mul(&ten).add(&Self::from_i128(i128::from(digit)));
            }
            if negative { value.neg() } else { value }
        }

        pub fn is_zero(&self) -> bool {
            self.magnitude.is_empty()
        }

        pub fn is_negative(&self) -> bool {
            self.negative
        }

        /// The array index this names, if it is one.
        pub fn to_index(&self) -> Option<usize> {
            if self.negative || self.magnitude.len() > 2 {
                return None;
            }
            let value = self.magnitude.iter().rev().fold(0u64, |acc, &digit| (acc << 32) | u64::from(digit));
            usize::try_from(value).ok()
        }

        pub fn neg(&self) -> Self {
            Self::from_parts(!self.negative, self.magnitude.clone())
        }

        pub fn add(&self, other: &Self) -> Self {
            if self.negative == other.negative {
                return Self::from_parts(self.negative, add_magnitude(&self.magnitude, &other.magnitude));
            }
            match compare_magnitude(&self.magnitude, &other.magnitude) {
                Ordering::Less => Self::from_parts(other.negative, sub_magnitude(&other.magnitude, &self.magnitude)),
                _ => Self::from_parts(self.negative, sub_magnitude(&self.magnitude, &other.magnitude)),
            }
        }

        pub fn sub(&self, other: &Self) -> Self {
            self.add(&other.neg())
        }

        pub fn mul(&self, other: &Self) -> Self {
            Self::from_parts(self.negative != other.negative, mul_magnitude(&self.magnitude, &other.magnitude))
        }

        /// Division rounding toward zero; the divisor is not zero.
        pub fn divmod_trunc(&self, other: &Self) -> (Self, Self) {
            let (quotient, remainder) = divmod_magnitude(&self.magnitude, &other.magnitude);
            (
                Self::from_parts(self.negative != other.negative, quotient),
                Self::from_parts(self.negative, remainder),
            )
        }
    }

    impl Ord for Big {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self.negative, other.negative) {
                (false, true) => Ordering::Greater,
                (true, false) => Ordering::Less,
                (false, false) => compare_magnitude(&self.magnitude, &other.magnitude),
                (true, true) => compare_magnitude(&other.magnitude, &self.magnitude),
            }
        }
    }

    impl PartialOrd for Big {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl fmt::Display for Big {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_zero() {
                return formatter.write_str("0");
            }
            let billion = [1_000_000_000u32];
            let mut chunks = Vec::new();
            let mut rest = self.magnitude.clone();
            while !rest.is_empty() {
                let (quotient, remainder) = divmod_magnitude(&rest, &billion);
                chunks.push(remainder.first().copied().unwrap_or(0));
                rest = quotient;
            }
            let mut text = String::new();
            if self.negative {
                text.push('-');
            }
            text.push_str(&chunks.pop().unwrap_or(0).to_string());
            for chunk in chunks.iter().rev() {
                text.push_str(&format!("{chunk:09}"));
            }
            formatter.write_str(&text)
        }
    }

    /// Natural subtraction: truncated at zero.
    pub fn nat_sub(a: &Big, b: &Big) -> Big {
        if a > b { a.sub(b) } else { Big::zero() }
    }

    pub fn pred(a: &Big) -> Big {
        a.sub(&Big::from_i128(1))
    }

    /// Integer division with the source's rounding ("trunc", "floor" or
    /// "euclid"); by zero it aborts with the source's message or is total
    /// (x / 0 = 0, x % 0 = x).
    pub fn divide(a: &Big, b: &Big, rounding: &str, by_zero: Option<&str>, remainder: bool) -> Big {
        if b.is_zero() {
            if let Some(message) = by_zero {
                panic!("{message}");
            }
            return if remainder { a.clone() } else { Big::zero() };
        }
        let one = Big::from_i128(1);
        let (mut quotient, rest) = a.divmod_trunc(b);
        if !rest.is_zero() {
            if rounding == "floor" && rest.is_negative() != b.is_negative() {
                quotient = quotient.sub(&one);
            }
            if rounding == "euclid" && rest.is_negative() {
                quotient = if b.is_negative() { quotient.add(&one) } else { quotient.sub(&one) };
            }
        }
        if remainder { a.sub(&quotient.mul(b)) } else { quotient }
    }

    pub fn to_nat_checked(value: Big, message: &str) -> Big {
        if value.is_negative() {
            panic!("{message}");
        }
        value
    }

    pub fn clamp_nat(value: Big) -> Big {
        if value.is_negative() { Big::zero() } else { value }
    }

    /// Bounded domains for executable theorem checks.
    pub fn range(low: i128, high: i128) -> Vec<Big> {
        (low..=high).map(Big::from_i128).collect()
    }
}

/// JavaScript's Number operations that Rust's f64 does not share.
pub mod ml_number {
    /// ECMAScript Number::toString: the shortest decimal that reads back as
    /// the value, the even one on a tie, laid out as JavaScript prints it.
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
            if k == 1 {
                format!("{digits}e{sign}{power}")
            } else {
                format!("{}.{}e{sign}{power}", &digits[..1], &digits[1..])
            }
        };
        if x < 0.0 {
            format!("-{body}")
        } else {
            body
        }
    }

    /// What console.log prints: -0 as "-0", where String(-0) is "0".
    pub fn js_console(x: f64) -> String {
        if x == 0.0 && x.is_sign_negative() {
            String::from("-0")
        } else {
            js_number(x)
        }
    }

    /// SameValue, as Object.is and assert.strictEqual compare: NaN equals
    /// NaN, and 0 differs from -0.
    pub fn same_value(a: f64, b: f64) -> bool {
        if a.is_nan() || b.is_nan() {
            a.is_nan() && b.is_nan()
        } else {
            a.to_bits() == b.to_bits()
        }
    }

    /// A decimal digit string plus one, or None when it gains a digit.
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

    /// The significant digits without trailing zeros, and the exponent n with
    /// value 0.digits × 10^n, of Rust's {:e} text.
    fn decimal(text: &str) -> (String, i32) {
        let (mantissa, exponent) = text.split_once('e').expect("exponent");
        let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
        let digits = digits.trim_end_matches('0').to_string();
        let digits = if digits.is_empty() { String::from("0") } else { digits };
        (digits, exponent.parse::<i32>().expect("exponent") + 1)
    }
}

/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if x < 0.0 && x >= -0.5 {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 {
            floor + 1.0
        } else {
            floor
        }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where f64::signum does not.
    pub fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            x
        }
    }

    /// Math.max of two Numbers: NaN when either is, and 0 above -0.
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() {
                b
            } else {
                a
            }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() {
                a
            } else {
                b
            }
        } else {
            f64::NAN
        }
    }

    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9007199254740991.0
    }
}

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
    }

    /// The index a Number names: a non-negative integer, -0 included.
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9007199254740992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize }
    }
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len()); let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> f64 {
        if values.is_empty() { return -1.0; }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) { return -1.0; }
            let start = if from < 0.0 { (values.len() as f64 + from) as usize } else { from.min((values.len() - 1) as f64) as usize };
            (0..=start).rev().find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 { (values.len() as f64 + from).max(0.0) as usize } else { from.min(values.len() as f64) as usize };
            (start..values.len()).find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        }
    }
    pub fn includes<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values { if !out.iter().any(|previous| equal(previous, &value)) { out.push(value); } }
        out
    }
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) { out[index].1 = value; }
            else { out.push((key, value)); }
        }
        out
    }
}

pub fn accept_rust_completion_model(model: String) -> bool {
    ((model == "") || (model == "closed guarded function returns 1"))
}

pub fn accept_rust_return_ownership(proven: bool, indirected: bool) -> bool {
    (proven && !indirected)
}

pub fn read_rust_conditional_completion(then_kind: String, otherwise_kind: String, has_else: bool, continuation: bool) -> String {
    if ((((then_kind == "returned") && (otherwise_kind == "")) && (has_else == (false))) && (continuation == (true))) {
        String::from("guarded continuation")
    } else {
        if ((((then_kind == "returned") && (otherwise_kind == "returned")) && (has_else == (true))) && (continuation == (false))) {
            String::from("returned conditional")
        } else {
            if ((((then_kind == "value") && (otherwise_kind == "value")) && (has_else == (true))) && (continuation == (false))) {
                String::from("conditional value")
            } else {
                if ((((then_kind == "returned") && (otherwise_kind == "returned")) && (has_else == (true))) && (continuation == (true))) {
                    String::from("unreachable continuation")
                } else {
                    if ((((then_kind == "value") && (otherwise_kind == "value")) && (has_else == (true))) && (continuation == (true))) {
                        String::from("discarded conditional value")
                    } else {
                        if ((((then_kind == "returned") && (otherwise_kind == "aborted")) && (has_else == (true))) && (continuation == (false))) {
                            String::from("returned conditional")
                        } else {
                            if ((((then_kind == "returned") && (otherwise_kind == "aborted")) && (has_else == (true))) && (continuation == (true))) {
                                String::from("unreachable continuation")
                            } else {
                                if ((((then_kind == "aborted") && (otherwise_kind == "returned")) && (has_else == (true))) && (continuation == (false))) {
                                    String::from("returned conditional")
                                } else {
                                    if ((((then_kind == "aborted") && (otherwise_kind == "returned")) && (has_else == (true))) && (continuation == (true))) {
                                        String::from("unreachable continuation")
                                    } else {
                                        if ((((then_kind == "aborted") && (otherwise_kind == "aborted")) && (has_else == (true))) && (continuation == (false))) {
                                            String::from("aborted conditional")
                                        } else {
                                            if ((((then_kind == "aborted") && (otherwise_kind == "aborted")) && (has_else == (true))) && (continuation == (true))) {
                                                String::from("unreachable continuation")
                                            } else {
                                                if ((((then_kind == "value") && (otherwise_kind == "aborted")) && (has_else == (true))) && (continuation == (false))) {
                                                    String::from("conditional value")
                                                } else {
                                                    if ((((then_kind == "value") && (otherwise_kind == "aborted")) && (has_else == (true))) && (continuation == (true))) {
                                                        String::from("discarded conditional value")
                                                    } else {
                                                        if ((((then_kind == "aborted") && (otherwise_kind == "value")) && (has_else == (true))) && (continuation == (false))) {
                                                            String::from("conditional value")
                                                        } else {
                                                            if ((((then_kind == "aborted") && (otherwise_kind == "value")) && (has_else == (true))) && (continuation == (true))) {
                                                                String::from("discarded conditional value")
                                                            } else {
                                                                if ((((then_kind == "aborted") && (otherwise_kind == "")) && (has_else == (false))) && (continuation == (true))) {
                                                                    String::from("guarded continuation")
                                                                } else {
                                                                    String::from("unsupported completion")
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_completion_kind(action: String, then_kind: String, otherwise_kind: String) -> String {
    if (((action == "returned conditional") && (then_kind == "returned")) && (otherwise_kind == "returned")) {
        String::from("returned")
    } else {
        if (((action == "conditional value") && (then_kind == "value")) && (otherwise_kind == "value")) {
            String::from("value")
        } else {
            if ((action == "returned conditional") && (((then_kind == "returned") && (otherwise_kind == "aborted")) || ((then_kind == "aborted") && (otherwise_kind == "returned")))) {
                String::from("returned")
            } else {
                if (((action == "aborted conditional") && (then_kind == "aborted")) && (otherwise_kind == "aborted")) {
                    String::from("aborted")
                } else {
                    if (((((action == "conditional value") && (then_kind == "value")) && (otherwise_kind == "aborted")) || (((action == "conditional value") && (then_kind == "aborted")) && (otherwise_kind == "value"))) || (((action == "guarded continuation") && (then_kind == "returned")) && (otherwise_kind == "value"))) {
                        String::from("value")
                    } else {
                        if (((action == "guarded continuation") && (then_kind == "returned")) && ((otherwise_kind == "returned") || (otherwise_kind == "aborted"))) {
                            String::from("returned")
                        } else {
                            if (((action == "guarded continuation") && (then_kind == "aborted")) && (otherwise_kind == "value")) {
                                String::from("value")
                            } else {
                                if (((action == "guarded continuation") && (then_kind == "aborted")) && (otherwise_kind == "returned")) {
                                    String::from("returned")
                                } else {
                                    if (((action == "guarded continuation") && (then_kind == "aborted")) && (otherwise_kind == "aborted")) {
                                        String::from("aborted")
                                    } else {
                                        String::from("unsupported completion")
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_rust_completion_panic_arguments(model: String, macro_name: String, first_kind: String, first_value: String, second_kind: String, count: String) -> bool {
    ((((((model == "closed guarded function returns 1") && (macro_name == "panic")) && (first_kind == "str")) && (first_value == "{}")) && (second_kind == "str")) && (count == "2"))
}

pub fn accept_rust_ordinary_completion_profiles(profiles: Vec<String>) -> bool {
    {
        let index = 0f64;
        {
            let source_ml_source1_s1 = self::source_ml_temporary_accept_rust_ordinary_completion_profiles_loop1(profiles, index);
            match source_ml_source1_s1.clone() {
                self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Done => {
                    true
                }
                self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Return(source_ml_temporary_result1) => {
                    source_ml_temporary_result1
                }
            }
        }
    }
}

pub fn read_rust_function_return_presence(values: Vec<String>) -> f64 {
    if {
        let source_ml_source1_comparison_operand2 = self::ml_array::optional_at(&values, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand3 = String::from("{");
            !(match (source_ml_source1_comparison_operand2.clone(), source_ml_source1_comparison_operand3.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    } {
        0f64
    } else {
        {
            let depth = 0f64;
            {
                let index = 0f64;
                {
                    let source_ml_source1_s4 = self::source_ml_temporary_read_rust_function_return_presence_loop2(values, depth, index);
                    match source_ml_source1_s4.clone() {
                        self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Done => {
                            (-1f64)
                        }
                        self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Return(source_ml_temporary_result2) => {
                            source_ml_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_ordinary_conditional_completion(first: String, second: String, has_else: bool, continuation: bool, terminated: bool) -> String {
    {
        let original = self::read_rust_conditional_completion(first.clone(), second.clone(), has_else, continuation);
        if (original != "unsupported completion") {
            original.clone()
        } else {
            {
                let first_known = ((((first == "returned") || (first == "aborted")) || (first == "value")) || (first == "mixed"));
                {
                    let second_known = ((((second == "returned") || (second == "aborted")) || (second == "value")) || (second == "mixed"));
                    if ((!continuation && !has_else) && first_known) {
                        String::from("implicit Unit fallback")
                    } else {
                        if ((((!continuation && has_else) && !terminated) && first_known) && second_known) {
                            String::from("terminal mixed conditional")
                        } else {
                            original.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_ordinary_completion_kind(action: String, first: String, second: String) -> String {
    if (action == "terminal mixed conditional") {
        String::from("mixed")
    } else {
        if (action == "implicit Unit fallback") {
            if (first == "value") {
                String::from("value")
            } else {
                String::from("mixed")
            }
        } else {
            if ((action == "guarded continuation") && ((second == "mixed") || ((first == "returned") && (second == "value")))) {
                String::from("mixed")
            } else {
                self::read_rust_completion_kind(action.clone(), first.clone(), second.clone())
            }
        }
    }
}

pub fn accept_rust_ordinary_else_if(first: String) -> bool {
    ((first == "returned") || (first == "aborted"))
}

pub fn read_rust_unit_statement_disposition(kind: String, collecting: bool, proven: bool, type_: String, indirected: bool) -> String {
    if (kind == "abort") {
        String::from("abort")
    } else {
        if ((proven && (type_ == "unit")) && !indirected) {
            String::from("bind")
        } else {
            if (collecting && (kind == "app")) {
                String::from("bind")
            } else {
                String::from("refuse")
            }
        }
    }
}

pub fn read_rust_function_macro_disposition(name: String) -> String {
    if (name == "println") {
        String::from("print")
    } else {
        String::from("")
    }
}

pub fn find_rust_terminal_return_boundary(values: Vec<String>, start: f64) -> f64 {
    if ((!(self::ml_math::is_safe_integer(start)) || (start < (0f64))) || (start >= ((values.len() as f64)))) {
        (-1f64)
    } else {
        {
            let stack = Vec::<String>::new();
            {
                let at = (start + 1f64);
                {
                    let source_ml_source1_s5 = self::source_ml_temporary_find_rust_terminal_return_boundary_loop3(values, stack, at);
                    match source_ml_source1_s5.clone() {
                        self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Done => {
                            (-2f64)
                        }
                        self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(source_ml_temporary_result3) => {
                            source_ml_temporary_result3
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_ordinary_completion_construction(kind: String, terminal: bool) -> String {
    if ((kind == "break") || (kind == "continue")) {
        String::from("loop signal")
    } else {
        if (kind == "loop") {
            String::from("finite loop")
        } else {
            if (kind == "abort") {
                String::from("value")
            } else {
                if (kind == "return") {
                    if terminal {
                        String::from("value")
                    } else {
                        String::from("completed")
                    }
                } else {
                    if (kind == "value") {
                        if terminal {
                            String::from("value")
                        } else {
                            String::from("continuation")
                        }
                    } else {
                        if (kind == "binding") {
                            String::from("binding")
                        } else {
                            if (kind == "conditional") {
                                String::from("conditional")
                            } else {
                                if (kind == "match") {
                                    String::from("match")
                                } else {
                                    if (kind == "sequence") {
                                        String::from("join")
                                    } else {
                                        String::from("")
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_ordinary_completion_alternative_kind(first: String, second: String) -> String {
    if ((((((first == "broken") || (first == "continued")) || (first == "completed loop alternatives")) || (second == "broken")) || (second == "continued")) || (second == "completed loop alternatives")) {
        if (first == second) {
            first.clone()
        } else {
            if (self::accept_rust_ordinary_completed_alternative(first.clone()) && self::accept_rust_ordinary_completed_alternative(second.clone())) {
                String::from("completed loop alternatives")
            } else {
                String::from("mixed")
            }
        }
    } else {
        if ((first == "aborted") && (second == "aborted")) {
            String::from("aborted")
        } else {
            if (((first == "returned") || (first == "aborted")) && ((second == "returned") || (second == "aborted"))) {
                String::from("returned")
            } else {
                if ((first == "value") && (second == "value")) {
                    String::from("value")
                } else {
                    String::from("mixed")
                }
            }
        }
    }
}

pub fn accept_rust_ordinary_completed_alternative(kind: String) -> bool {
    (((((kind == "returned") || (kind == "aborted")) || (kind == "broken")) || (kind == "continued")) || (kind == "completed loop alternatives"))
}

pub fn test_rust_ordinary_completion_falls_through(kind: String) -> bool {
    !(self::accept_rust_ordinary_completed_alternative(kind.clone()))
}

pub fn read_rust_ordinary_match_completion_presence(kinds: Vec<String>, values: Vec<String>) -> f64 {
    if (((((kinds.len() as f64)) != ((values.len() as f64))) || {
        let source_ml_source1_comparison_operand6 = self::ml_array::optional_at(&values, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand7 = String::from("match");
            !(match (source_ml_source1_comparison_operand6.clone(), source_ml_source1_comparison_operand7.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) || {
        let source_ml_source1_comparison_operand8 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand9 = String::from("identifier");
            !(match (source_ml_source1_comparison_operand8.clone(), source_ml_source1_comparison_operand9.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) {
        0f64
    } else {
        {
            let depth = 0f64;
            {
                let opened = false;
                {
                    let selected = false;
                    {
                        let index = 1f64;
                        {
                            let source_ml_source1_s10 = self::source_ml_temporary_read_rust_ordinary_match_completion_presence_loop5(kinds, values, depth, opened, selected, index);
                            match source_ml_source1_s10.clone() {
                                self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Result::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Done => {
                                    (-1f64)
                                }
                                self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Result::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Return(source_ml_temporary_result5) => {
                                    source_ml_temporary_result5
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_ordinary_match_arm_count_error(count: f64) -> String {
    if (((count >= (1f64)) && (count <= (256f64))) && (((count % 1f64)) == (0f64))) {
        String::from("")
    } else {
        String::from("the complete ordinary match arms exceed their source budget")
    }
}

pub fn test_rust_ordinary_completion_transferred_result(transfers: Vec<bool>) -> bool {
    if (((transfers.len() as f64)) == (0f64)) {
        false
    } else {
        {
            let source_ml_temporary_values7 = transfers;
            {
                let source_ml_temporary_index7 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s11 = self::source_ml_temporary_test_rust_ordinary_completion_transferred_result_loop8(source_ml_temporary_values7, source_ml_temporary_index7.clone());
                    match source_ml_source1_s11.clone() {
                        self::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Result::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Done => {
                            true
                        }
                        self::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Result::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Return(source_ml_temporary_result8) => {
                            source_ml_temporary_result8
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_ordinary_completion_snapshot_error(entries: f64, added_entries: f64, text: f64, added_text: f64) -> String {
    if ((((((((((((entries % 1f64)) != (0f64)) || (((added_entries % 1f64)) != (0f64))) || (((text % 1f64)) != (0f64))) || (((added_text % 1f64)) != (0f64))) || (entries < (0f64))) || (added_entries < (0f64))) || (text < (0f64))) || (added_text < (0f64))) || (entries > ((100000f64 - added_entries)))) || (text > ((16777216f64 - added_text)))) {
        String::from("the actual completion exit snapshots exceed their finite source budget")
    } else {
        String::from("")
    }
}

pub fn read_rust_ordinary_completion_sequence_kind(first: String, second: String) -> String {
    if (first == "value") {
        second.clone()
    } else {
        if (((first == "broken") || (first == "continued")) || (first == "completed loop alternatives")) {
            first.clone()
        } else {
            if (((second == "broken") || (second == "continued")) || (second == "completed loop alternatives")) {
                String::from("completed loop alternatives")
            } else {
                if ((first == "returned") || (first == "aborted")) {
                    first.clone()
                } else {
                    if ((second == "returned") || (second == "aborted")) {
                        String::from("returned")
                    } else {
                        String::from("mixed")
                    }
                }
            }
        }
    }
}

pub fn read_rust_function_completion_presence(values: Vec<String>) -> f64 {
    if {
        let source_ml_source1_comparison_operand12 = self::ml_array::optional_at(&values, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand13 = String::from("{");
            !(match (source_ml_source1_comparison_operand12.clone(), source_ml_source1_comparison_operand13.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    } {
        0f64
    } else {
        {
            let depth = 0f64;
            {
                let index = 0f64;
                {
                    let source_ml_source1_s14 = self::source_ml_temporary_read_rust_function_completion_presence_loop9(values, depth, index);
                    match source_ml_source1_s14.clone() {
                        self::MlTemporaryReadRustFunctionCompletionPresenceLoop9Result::MlTemporaryReadRustFunctionCompletionPresenceLoop9Done => {
                            (-1f64)
                        }
                        self::MlTemporaryReadRustFunctionCompletionPresenceLoop9Result::MlTemporaryReadRustFunctionCompletionPresenceLoop9Return(source_ml_temporary_result9) => {
                            source_ml_temporary_result9
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_ordinary_statement_disposition(kind: String, collecting: bool, proven: bool, type_: String, indirected: bool) -> String {
    {
        let original = self::read_rust_unit_statement_disposition(kind.clone(), collecting, proven, type_.clone(), indirected);
        if (original != "refuse") {
            original.clone()
        } else {
            if (!proven || indirected) {
                String::from("refuse")
            } else {
                if ((((((type_ == "bool") || (type_ == "fixed")) || (type_ == "float")) || (type_ == "int")) || (type_ == "nat")) || (type_ == "string")) {
                    String::from("bind")
                } else {
                    String::from("refuse")
                }
            }
        }
    }
}

pub fn read_rust_implicit_block_statement_disposition(kind: String, block: bool, continuation: bool, collecting: bool, proven: bool, type_: String, indirected: bool) -> String {
    if (!continuation || !((((block || (kind == "if")) || (kind == "match")) || (kind == "match1")))) {
        String::from("")
    } else {
        if indirected {
            String::from("refuse")
        } else {
            if (collecting || (proven && (type_ == "unit"))) {
                String::from("bind")
            } else {
                String::from("refuse")
            }
        }
    }
}

pub fn accept_rust_block_expression_statement_boundary(opening: String, closing: String) -> bool {
    (self::ml_array::includes(&vec![String::from("if"), String::from("match"), String::from("{")], &opening.clone(), 0f64, false, |a, b| a == b) && (closing == "}"))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result {
    MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Done,
    MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Return(bool),
}

pub fn source_ml_temporary_accept_rust_ordinary_completion_profiles_loop1(mut profiles: Vec<String>, mut index: f64) -> self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result {
    loop {
        return if (index < ((profiles.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand15 = self::ml_array::optional_at(&profiles, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand16 = String::from("");
                    !(match (source_ml_source1_comparison_operand15.clone(), source_ml_source1_comparison_operand16.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            } {
                self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFunctionReturnPresenceLoop2Result {
    MlTemporaryReadRustFunctionReturnPresenceLoop2Done,
    MlTemporaryReadRustFunctionReturnPresenceLoop2Return(f64),
}

pub fn source_ml_temporary_read_rust_function_return_presence_loop2(mut values: Vec<String>, mut depth: f64, mut index: f64) -> self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result {
    loop {
        return if (index < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| String::from(""));
                if (value == "return") {
                    self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Return(1f64)
                } else {
                    {
                        let depth_3 = if (value == "{") {
                            {
                                let depth_2 = (depth + 1f64);
                                depth_2
                            }
                        } else {
                            depth
                        };
                        if (value == "}") {
                            {
                                let depth_4 = (depth_3 - 1f64);
                                if (depth_4 == (0f64)) {
                                    self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Return(0f64)
                                } else {
                                    {
                                        let index_2 = (index + 1f64);
                                        {
                                            (depth, index) = (depth_4, index_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let index_3 = (index + 1f64);
                                {
                                    (depth, index) = (depth_3, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindRustTerminalReturnBoundaryLoop3Result {
    MlTemporaryFindRustTerminalReturnBoundaryLoop3Done,
    MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(f64),
}

pub fn source_ml_temporary_find_rust_terminal_return_boundary_loop4(mut values: Vec<String>, mut end: f64) -> f64 {
    loop {
        return if {
            let source_ml_source1_comparison_operand17 = self::ml_array::optional_at(&values, self::ml_array::number_index(end));
            {
                let source_ml_source1_comparison_operand18 = String::from(";");
                match (source_ml_source1_comparison_operand17.clone(), source_ml_source1_comparison_operand18.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        } {
            {
                let end_2 = (end + 1f64);
                {
                    end = end_2;
                    continue;
                }
            }
        } else {
            end
        };
    }
}

pub fn source_ml_temporary_find_rust_terminal_return_boundary_loop3(mut values: Vec<String>, mut stack: Vec<String>, mut at: f64) -> self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| String::from(""));
                if (((value == "(") || (value == "[")) || (value == "{")) {
                    {
                        let stack_2 = { let (ml_append_right, ml_append_left) = (vec![value.clone()], stack); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let at_2 = (at + 1f64);
                            {
                                (stack, at) = (stack_2, at_2);
                                continue;
                            }
                        }
                    }
                } else {
                    if ((((stack.len() as f64)) == (0f64)) && ((value == ";") || (value == "}"))) {
                        {
                            let end = at;
                            {
                                let end_2 = self::source_ml_temporary_find_rust_terminal_return_boundary_loop4(values.clone(), end);
                                if (end_2 >= ((values.len() as f64))) {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return((-2f64))
                                } else {
                                    if {
                                        let source_ml_source1_comparison_operand19 = self::ml_array::optional_at(&values, self::ml_array::number_index(end_2));
                                        {
                                            let source_ml_source1_comparison_operand20 = String::from("}");
                                            match (source_ml_source1_comparison_operand19.clone(), source_ml_source1_comparison_operand20.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        }
                                    } {
                                        self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(end_2)
                                    } else {
                                        self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return((-1f64))
                                    }
                                }
                            }
                        }
                    } else {
                        if (((value == ")") || (value == "]")) || (value == "}")) {
                            {
                                let previous = (self::ml_array::optional_at(&stack, self::ml_array::number_index(((stack.len() as f64) - 1f64)))).clone().unwrap_or_else(|| String::from(""));
                                if ((((value == ")") && (previous != "(")) || ((value == "]") && (previous != "["))) || ((value == "}") && (previous != "{"))) {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return((-1f64))
                                } else {
                                    {
                                        let stack_3 = self::ml_array::slice(&stack, 0f64, (-1f64));
                                        {
                                            let at_3 = (at + 1f64);
                                            {
                                                (stack, at) = (stack_3, at_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let at_4 = (at + 1f64);
                                {
                                    at = at_4;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Result {
    MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Done,
    MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6 {
    MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6Next(f64, bool),
}

pub fn source_ml_temporary_read_rust_ordinary_match_completion_presence_loop5(mut kinds: Vec<String>, mut values: Vec<String>, mut depth: f64, mut opened: bool, mut selected: bool, mut index: f64) -> self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Result {
    loop {
        return if (index < ((values.len() as f64))) {
            {
                let source_ml_source1_s25 = if ({
                    let source_ml_source1_comparison_operand26 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(index));
                    {
                        let source_ml_source1_comparison_operand27 = String::from("punct");
                        match (source_ml_source1_comparison_operand26.clone(), source_ml_source1_comparison_operand27.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    }
                } && {
                    let source_ml_source1_comparison_operand28 = self::ml_array::optional_at(&values, self::ml_array::number_index(index));
                    {
                        let source_ml_source1_comparison_operand29 = String::from("{");
                        match (source_ml_source1_comparison_operand28.clone(), source_ml_source1_comparison_operand29.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    }
                }) {
                    {
                        let depth_2 = (depth + 1f64);
                        {
                            let opened_2 = true;
                            self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6::MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6Next(depth_2, opened_2)
                        }
                    }
                } else {
                    self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6::MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6Next(depth, opened)
                };
                match source_ml_source1_s25.clone() {
                    self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6::MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6Next(depth_3, opened_3) => {
                        {
                            let selected_3 = if (opened_3 && (({
                                let source_ml_source1_comparison_operand30 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand31 = String::from("identifier");
                                    match (source_ml_source1_comparison_operand30.clone(), source_ml_source1_comparison_operand31.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            } && (({
                                let source_ml_source1_comparison_operand32 = self::ml_array::optional_at(&values, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand33 = String::from("return");
                                    match (source_ml_source1_comparison_operand32.clone(), source_ml_source1_comparison_operand33.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            } || {
                                let source_ml_source1_comparison_operand34 = self::ml_array::optional_at(&values, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand35 = String::from("break");
                                    match (source_ml_source1_comparison_operand34.clone(), source_ml_source1_comparison_operand35.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            }) || {
                                let source_ml_source1_comparison_operand36 = self::ml_array::optional_at(&values, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand37 = String::from("continue");
                                    match (source_ml_source1_comparison_operand36.clone(), source_ml_source1_comparison_operand37.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            })) || ({
                                let source_ml_source1_comparison_operand38 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand39 = String::from("punct");
                                    match (source_ml_source1_comparison_operand38.clone(), source_ml_source1_comparison_operand39.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            } && {
                                let source_ml_source1_comparison_operand40 = self::ml_array::optional_at(&values, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand41 = String::from("?");
                                    match (source_ml_source1_comparison_operand40.clone(), source_ml_source1_comparison_operand41.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            }))) {
                                {
                                    let selected_2 = true;
                                    selected_2
                                }
                            } else {
                                selected
                            };
                            if (({
                                let source_ml_source1_comparison_operand42 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand43 = String::from("punct");
                                    match (source_ml_source1_comparison_operand42.clone(), source_ml_source1_comparison_operand43.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            } && {
                                let source_ml_source1_comparison_operand44 = self::ml_array::optional_at(&values, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand45 = String::from("}");
                                    match (source_ml_source1_comparison_operand44.clone(), source_ml_source1_comparison_operand45.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            }) && opened_3) {
                                {
                                    let depth_4 = (depth_3 - 1f64);
                                    if (depth_4 == (0f64)) {
                                        self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Result::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Return(if selected_3 {
                                            1f64
                                        } else {
                                            0f64
                                        })
                                    } else {
                                        {
                                            let index_2 = (index + 1f64);
                                            {
                                                (depth, opened, selected, index) = (depth_4, opened_3, selected_3, index_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index_3 = (index + 1f64);
                                    {
                                        (depth, opened, selected, index) = (depth_3, opened_3, selected_3, index_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Result::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Result {
    MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Done,
    MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Return(bool),
}

pub fn source_ml_temporary_test_rust_ordinary_completion_transferred_result_loop8(mut source_ml_temporary_values7: Vec<bool>, mut source_ml_temporary_index7: self::ml::Big) -> self::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Result {
    loop {
        return if (source_ml_temporary_index7 < (self::ml::Big::from_u128(source_ml_temporary_values7.len() as u128))) {
            {
                let transfer = self::ml_array::at(&source_ml_temporary_values7, source_ml_temporary_index7.to_index());
                if !transfer {
                    self::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Result::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Return(false)
                } else {
                    {
                        let source_ml_temporary_index7_2 = source_ml_temporary_index7.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index7 = source_ml_temporary_index7_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Result::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFunctionCompletionPresenceLoop9Result {
    MlTemporaryReadRustFunctionCompletionPresenceLoop9Done,
    MlTemporaryReadRustFunctionCompletionPresenceLoop9Return(f64),
}

pub fn source_ml_temporary_read_rust_function_completion_presence_loop9(mut values: Vec<String>, mut depth: f64, mut index: f64) -> self::MlTemporaryReadRustFunctionCompletionPresenceLoop9Result {
    loop {
        return if (index < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| String::from(""));
                if (((value == "return") || (value == "abort")) || (value == "try")) {
                    self::MlTemporaryReadRustFunctionCompletionPresenceLoop9Result::MlTemporaryReadRustFunctionCompletionPresenceLoop9Return(1f64)
                } else {
                    {
                        let depth_3 = if (value == "{") {
                            {
                                let depth_2 = (depth + 1f64);
                                depth_2
                            }
                        } else {
                            depth
                        };
                        if (value == "}") {
                            {
                                let depth_4 = (depth_3 - 1f64);
                                if (depth_4 == (0f64)) {
                                    self::MlTemporaryReadRustFunctionCompletionPresenceLoop9Result::MlTemporaryReadRustFunctionCompletionPresenceLoop9Return(0f64)
                                } else {
                                    {
                                        let index_2 = (index + 1f64);
                                        {
                                            (depth, index) = (depth_4, index_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let index_3 = (index + 1f64);
                                {
                                    (depth, index) = (depth_3, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFunctionCompletionPresenceLoop9Result::MlTemporaryReadRustFunctionCompletionPresenceLoop9Done
        };
    }
}

fn ml_main() {
    assert!(((self::test_rust_ordinary_completion_falls_through(String::from("value"))) == (true)), "assertion 1");
    assert!(((self::test_rust_ordinary_completion_falls_through(String::from("mixed"))) == (true)), "assertion 2");
    assert!(((self::test_rust_ordinary_completion_falls_through(String::from("returned"))) == (false)), "assertion 3");
    assert!(((self::test_rust_ordinary_completion_falls_through(String::from("aborted"))) == (false)), "assertion 4");
    assert!(((self::read_rust_ordinary_completion_snapshot_error(99999f64, 1f64, 16777215f64, 1f64)) == ""), "assertion 5");
    assert!(((self::read_rust_ordinary_completion_snapshot_error(100000f64, 1f64, 0f64, 0f64)) == "the actual completion exit snapshots exceed their finite source budget"), "assertion 6");
    assert!(((self::read_rust_ordinary_completion_snapshot_error(0f64, 0f64, 16777216f64, 1f64)) == "the actual completion exit snapshots exceed their finite source budget"), "assertion 7");
    assert!(((self::test_rust_ordinary_completion_transferred_result(Vec::<bool>::new())) == (false)), "assertion 8");
    assert!(((self::test_rust_ordinary_completion_transferred_result(vec![true])) == (true)), "assertion 9");
    assert!(((self::test_rust_ordinary_completion_transferred_result(vec![true, false])) == (false)), "assertion 10");
    assert!(((self::read_rust_ordinary_completion_construction(String::from("match"), true)) == "match"), "assertion 11");
    assert!(((self::read_rust_ordinary_completion_construction(String::from("match"), false)) == "match"), "assertion 12");
    assert!(self::ml_number::same_value(self::read_rust_ordinary_match_completion_presence(vec![String::from("identifier"), String::from("identifier"), String::from("punct"), String::from("identifier"), String::from("punct")], vec![String::from("match"), String::from("value"), String::from("{"), String::from("return"), String::from("}")]), 1f64), "assertion 13");
    assert!(self::ml_number::same_value(self::read_rust_ordinary_match_completion_presence(vec![String::from("identifier"), String::from("identifier"), String::from("punct"), String::from("string"), String::from("punct")], vec![String::from("match"), String::from("value"), String::from("{"), String::from("return"), String::from("}")]), 0f64), "assertion 14");
    assert!(self::ml_number::same_value(self::read_rust_ordinary_match_completion_presence(vec![String::from("identifier"), String::from("identifier"), String::from("punct"), String::from("identifier"), String::from("punct")], vec![String::from("match"), String::from("value"), String::from("{"), String::from("r#return"), String::from("}")]), 0f64), "assertion 15");
    assert!(self::ml_number::same_value(self::read_rust_ordinary_match_completion_presence(vec![String::from("identifier"), String::from("identifier"), String::from("punct"), String::from("punct"), String::from("punct")], vec![String::from("match"), String::from("value"), String::from("{"), String::from("?"), String::from("}")]), 1f64), "assertion 16");
    assert!(self::ml_number::same_value(self::read_rust_ordinary_match_completion_presence(vec![String::from("identifier"), String::from("identifier"), String::from("punct"), String::from("identifier")], vec![String::from("match"), String::from("value"), String::from("{"), String::from("return")]), (-1f64)), "assertion 17");
    assert!(self::ml_number::same_value(self::read_rust_ordinary_match_completion_presence(vec![String::from("identifier")], vec![String::from("r#match")]), 0f64), "assertion 18");
    assert!(((self::read_rust_ordinary_match_arm_count_error(1f64)) == ""), "assertion 19");
    assert!(((self::read_rust_ordinary_match_arm_count_error(256f64)) == ""), "assertion 20");
    assert!(((self::read_rust_ordinary_match_arm_count_error(257f64)) == "the complete ordinary match arms exceed their source budget"), "assertion 21");
    assert!(((self::read_rust_ordinary_match_arm_count_error(0f64)) == "the complete ordinary match arms exceed their source budget"), "assertion 22");
    assert!(((self::read_rust_ordinary_completion_alternative_kind(String::from("broken"), String::from("value"))) == "mixed"), "assertion 23");
    assert!(((self::read_rust_ordinary_completion_alternative_kind(String::from("returned"), String::from("continued"))) == "completed loop alternatives"), "assertion 24");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_owned_record_completion_controls() { main(); }
