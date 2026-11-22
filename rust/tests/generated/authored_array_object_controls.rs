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

/// JavaScript strings retain every UTF-16 code unit, including isolated surrogates.
pub mod ml_string {
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Text(pub Vec<u16>);

    impl std::fmt::Display for Text {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.0))
        }
    }

    impl Text {
        pub fn from_text(text: &str) -> Self { Self(text.encode_utf16().collect()) }
        pub fn from_units(units: Vec<u16>) -> Self { Self(units) }
        pub fn length(&self) -> f64 { self.0.len() as f64 }
        pub fn concat(&self, other: &Self) -> Self {
            let mut units = self.0.clone(); units.extend_from_slice(&other.0); Self(units)
        }
        pub fn starts_with(&self, search: &Self) -> bool { self.0.starts_with(&search.0) }
        pub fn ends_with(&self, search: &Self) -> bool { self.0.ends_with(&search.0) }
        pub fn contains(&self, search: &Self) -> bool { self.index_of(search, 0.0) >= 0.0 }
        pub fn slice(&self, start: f64, end: f64) -> Self {
            let first = relative(start, self.0.len()); let last = relative(end, self.0.len());
            Self(self.0[first..last.max(first)].to_vec())
        }
        pub fn index_of(&self, search: &Self, position: f64) -> f64 {
            let start = absolute(position, self.0.len());
            if search.0.is_empty() { return start as f64; }
            if search.0.len() > self.0.len() { return -1.0; }
            (start..=self.0.len() - search.0.len()).find(|&index| self.0[index..].starts_with(&search.0)).map_or(-1.0, |index| index as f64)
        }
        pub fn last_index_of(&self, search: &Self, position: f64) -> f64 {
            let position = if position.is_nan() { f64::INFINITY } else { position };
            let start = absolute(position, self.0.len());
            if search.0.is_empty() { return start as f64; }
            if search.0.len() > self.0.len() { return -1.0; }
            (0..=start.min(self.0.len() - search.0.len())).rev().find(|&index| self.0[index..].starts_with(&search.0)).map_or(-1.0, |index| index as f64)
        }
        pub fn split(&self, separator: Option<&Self>, limit: f64) -> Vec<Self> {
            let maximum = uint32(limit) as usize;
            if maximum == 0 { return Vec::new(); }
            let separator = match separator { Some(value) => value, None => return vec![self.clone()] };
            if separator.0.is_empty() { return self.0.iter().take(maximum).map(|&unit| Self(vec![unit])).collect(); }
            let mut out = Vec::new(); let mut start = 0;
            loop {
                let index = self.index_of(separator, start as f64);
                if index < 0.0 { break; }
                out.push(Self(self.0[start..index as usize].to_vec()));
                if out.len() == maximum { return out; }
                start = index as usize + separator.0.len();
            }
            out.push(Self(self.0[start..].to_vec())); out
        }
        pub fn code_point_at(&self, position: f64) -> Option<f64> {
            let index = integer(position);
            if index < 0.0 || index >= self.0.len() as f64 { return None; }
            let index = index as usize; let first = self.0[index];
            if (0xd800..=0xdbff).contains(&first) {
                if let Some(&second) = self.0.get(index + 1) {
                    if (0xdc00..=0xdfff).contains(&second) { return Some(((first as u32 - 0xd800) * 1024 + second as u32 - 0xdc00 + 0x10000) as f64); }
                }
            }
            Some(first as f64)
        }
        pub fn code_points(&self) -> Vec<Self> {
            let mut out = Vec::new(); let mut index = 0;
            while index < self.0.len() {
                let first = self.0[index];
                let width = if (0xd800..=0xdbff).contains(&first) && self.0.get(index + 1).map_or(false, |second| (0xdc00..=0xdfff).contains(second)) { 2 } else { 1 };
                out.push(Self(self.0[index..index + width].to_vec())); index += width;
            }
            out
        }
        pub fn repeat(&self, count: f64) -> Self {
            let count = integer(count);
            if count < 0.0 || !count.is_finite() { panic!("Invalid count value: {}", count); }
            if self.0.is_empty() { return Self(Vec::new()); }
            let size = count * self.0.len() as f64;
            if size > 536870888.0 { panic!("Invalid string length"); }
            Self(self.0.repeat(count as usize))
        }
        pub fn pad(&self, length: f64, fill: &Self, start: bool) -> Self {
            let length = integer(length).max(0.0);
            if length <= self.0.len() as f64 || fill.0.is_empty() { return self.clone(); }
            if length > 536870888.0 { panic!("Invalid string length"); }
            let count = length as usize - self.0.len();
            let padding: Vec<u16> = fill.0.iter().copied().cycle().take(count).collect();
            let pad = Self(padding); if start { pad.concat(self) } else { self.concat(&pad) }
        }
        pub fn replace(&self, search: &Self, replacement: &Self, all: bool) -> Self {
            let mut out = Vec::new(); let mut start = 0; let mut from = 0;
            loop {
                let index = self.index_of(search, from as f64);
                if index < 0.0 { break; }
                let index = index as usize;
                out.extend_from_slice(&self.0[start..index]);
                substitute(&mut out, &self.0, &search.0, &replacement.0, index);
                start = index + search.0.len();
                if !all || (search.0.is_empty() && index == self.0.len()) { break; }
                from = start + usize::from(search.0.is_empty());
            }
            out.extend_from_slice(&self.0[start..]); Self(out)
        }
        pub fn trim(&self, start: bool, end: bool, rust: bool) -> Self {
            let whitespace = |unit: u16| char::from_u32(unit as u32).map_or(false, |c| if rust { c.is_whitespace() } else { (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}' });
            let mut first = 0; let mut last = self.0.len();
            if start { while first < last && whitespace(self.0[first]) { first += 1; } }
            if end { while last > first && whitespace(self.0[last - 1]) { last -= 1; } }
            Self(self.0[first..last].to_vec())
        }
        pub fn map_case(&self, upper: bool) -> Self {
            let mut out = Vec::new(); let mut run = String::new();
            let flush = |run: &mut String, out: &mut Vec<u16>| { let mapped = if upper { run.to_uppercase() } else { run.to_lowercase() }; out.extend(mapped.encode_utf16()); run.clear(); };
            for scalar in char::decode_utf16(self.0.iter().copied()) {
                match scalar { Ok(c) => run.push(c), Err(error) => { flush(&mut run, &mut out); out.push(error.unpaired_surrogate()); } }
            }
            flush(&mut run, &mut out); Self(out)
        }
    }
    pub fn join(values: &[Text], separator: &Text) -> Text {
        let mut units = Vec::new();
        for (index, value) in values.iter().enumerate() { if index != 0 { units.extend_from_slice(&separator.0); } units.extend_from_slice(&value.0); }
        Text(units)
    }
    pub fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    pub fn absolute(value: f64, length: usize) -> usize { integer(value).max(0.0).min(length as f64) as usize }
    pub fn relative(value: f64, length: usize) -> usize { let value = integer(value); if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize } }
    fn uint32(value: f64) -> u32 { if !value.is_finite() || value == 0.0 { 0 } else { value.trunc().rem_euclid(4294967296.0) as u32 } }
    fn substitute(out: &mut Vec<u16>, text: &[u16], search: &[u16], replacement: &[u16], position: usize) {
        let mut index = 0;
        while index < replacement.len() {
            if replacement[index] == 36 && index + 1 < replacement.len() {
                match replacement[index + 1] {
                    36 => { out.push(36); index += 2; continue; }
                    38 => { out.extend_from_slice(search); index += 2; continue; }
                    96 => { out.extend_from_slice(&text[..position]); index += 2; continue; }
                    39 => { out.extend_from_slice(&text[position + search.len()..]); index += 2; continue; }
                    _ => {}
                }
            }
            out.push(replacement[index]); index += 1;
        }
    }
}

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self { class_name: class_name.to_owned(), message_units: message.encode_utf16().collect(), message_present: true }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

pub mod ml_utf16_text {
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    ((((((((((((unit >= (9f64)) && (unit <= (13f64))) || (unit == (32f64))) || (unit == (160f64))) || (unit == (5760f64))) || ((unit >= (8192f64)) && (unit <= (8202f64)))) || (unit == (8232f64))) || (unit == (8233f64))) || (unit == (8239f64))) || (unit == (8287f64))) || (unit == (12288f64))) || (unit == (65279f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_utf8_string_units(bytes: &[f64]) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source1_s1 = self::source_ml_temporary_read_java_script_utf8_string_units_loop1(bytes, index);
            match source_ml_source1_s1.clone() {
                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source1_s2 = self::source_ml_temporary_read_java_script_utf8_string_units_loop2(bytes, units, index_2);
                                match source_ml_source1_s2.clone() {
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Done(units_2) => {
                                        units_2
                                    }
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_temporary_result2) => {
                                        source_ml_temporary_result2
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Return(source_ml_temporary_result4) => {
                    source_ml_temporary_result4
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_unsigned_integer(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integral = if (value < (0f64)) {
                (-(((-value)).floor()))
            } else {
                value.floor()
            };
            {
                let remainder = (integral % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_integer(value: f64) -> f64 {
    if value.is_nan() {
        0f64
    } else {
        value.trunc()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_slice_position(value: f64, length: f64) -> f64 {
    {
        let integer = self::read_utf16_text_integer(value);
        if (integer < (0f64)) {
            self::ml_math::max((length + integer), 0f64)
        } else {
            self::ml_math::min(integer, length)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn slice_utf16_text(text: &[f64], start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::read_utf16_text_slice_position(start, (text.len() as f64));
        {
            let last = self::read_utf16_text_slice_position(end, (text.len() as f64));
            {
                let result = Vec::<f64>::new();
                {
                    let at = first;
                    {
                        let result_2 = self::source_ml_temporary_slice_utf16_text_loop5(text, last, result, at);
                        result_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn substring_utf16_text(text: &[f64], start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(start), 0f64), (text.len() as f64));
        {
            let last = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(end), 0f64), (text.len() as f64));
            self::slice_utf16_text(text, self::ml_math::min(first, last), self::ml_math::max(first, last))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn character_utf16_text(text: &[f64], position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn relative_character_utf16_text(text: &[f64], position: f64) -> Vec<Vec<f64>> {
    {
        let integer = self::read_utf16_text_integer(position);
        {
            let at = if (integer < (0f64)) {
                ((text.len() as f64) + integer)
            } else {
                integer
            };
            if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
                Vec::<Vec<f64>>::new()
            } else {
                vec![vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn code_point_utf16_text(text: &[f64], position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            {
                let first = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                    vec![if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        (((65536f64 + ((first - 55296f64) * 1024f64)) + second) - 56320f64)
                    } else {
                        first
                    }]
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn points_utf16_text(text: &[f64]) -> Vec<Vec<f64>> {
    {
        let result = Vec::<Vec<f64>>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_points_utf16_text_loop6(text, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn split_utf16_text(text: &[f64], separator: &[f64], limit: f64, missing: bool) -> Vec<Vec<f64>> {
    {
        let count = self::normalize_java_script_unsigned_integer(limit);
        {
            let result = Vec::<Vec<f64>>::new();
            if (count == (0f64)) {
                result
            } else {
                if missing {
                    vec![self::slice_utf16_text(text, 0f64, (text.len() as f64))]
                } else {
                    if (((separator.len() as f64)) == (0f64)) {
                        {
                            let at = 0f64;
                            {
                                let result_2 = self::source_ml_temporary_split_utf16_text_loop7(text, count, result, at);
                                result_2
                            }
                        }
                    } else {
                        {
                            let start = 0f64;
                            {
                                let source_ml_source1_s3 = self::source_ml_temporary_split_utf16_text_loop8(text, separator, count, result, start);
                                match source_ml_source1_s3.clone() {
                                    self::MlTemporarySplitUtf16TextLoop8Result::MlTemporarySplitUtf16TextLoop8Done(result_3, start_2) => {
                                        {
                                            let result_4 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text, start_2, (text.len() as f64))], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                            result_4
                                        }
                                    }
                                    self::MlTemporarySplitUtf16TextLoop8Result::MlTemporarySplitUtf16TextLoop8Return(source_ml_temporary_result8) => {
                                        source_ml_temporary_result8
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn match_utf16_text(text: &[f64], search: &[f64], start: f64) -> bool {
    if ((start < (0f64)) || (((start + (search.len() as f64))) > ((text.len() as f64)))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source1_s4 = self::source_ml_temporary_match_utf16_text_loop9(text, search, start, at);
                match source_ml_source1_s4.clone() {
                    self::MlTemporaryMatchUtf16TextLoop9Result::MlTemporaryMatchUtf16TextLoop9Done => {
                        true
                    }
                    self::MlTemporaryMatchUtf16TextLoop9Result::MlTemporaryMatchUtf16TextLoop9Return(source_ml_temporary_result9) => {
                        source_ml_temporary_result9
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn index_utf16_text(text: &[f64], search: &[f64], position: f64) -> f64 {
    {
        let start = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(position), 0f64), (text.len() as f64));
        {
            let at = start;
            {
                let source_ml_source1_s5 = self::source_ml_temporary_index_utf16_text_loop10(text, search, at);
                match source_ml_source1_s5.clone() {
                    self::MlTemporaryIndexUtf16TextLoop10Result::MlTemporaryIndexUtf16TextLoop10Done => {
                        (-1f64)
                    }
                    self::MlTemporaryIndexUtf16TextLoop10Result::MlTemporaryIndexUtf16TextLoop10Return(source_ml_temporary_result10) => {
                        source_ml_temporary_result10
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn last_index_utf16_text(text: &[f64], search: &[f64], position: f64) -> f64 {
    {
        let integer = if position.is_nan() {
            f64::INFINITY
        } else {
            self::read_utf16_text_integer(position)
        };
        {
            let start = self::ml_math::min(self::ml_math::max(integer, 0f64), ((text.len() as f64) - (search.len() as f64)));
            {
                let at = start;
                {
                    let source_ml_source1_s6 = self::source_ml_temporary_last_index_utf16_text_loop11(text, search, at);
                    match source_ml_source1_s6.clone() {
                        self::MlTemporaryLastIndexUtf16TextLoop11Result::MlTemporaryLastIndexUtf16TextLoop11Done => {
                            (-1f64)
                        }
                        self::MlTemporaryLastIndexUtf16TextLoop11Result::MlTemporaryLastIndexUtf16TextLoop11Return(source_ml_temporary_result11) => {
                            source_ml_temporary_result11
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_utf16_text(left: &[f64], right: &[f64]) -> f64 {
    {
        let count = self::ml_math::min((left.len() as f64), (right.len() as f64));
        {
            let at = 0f64;
            {
                let source_ml_source1_s7 = self::source_ml_temporary_compare_utf16_text_loop12(left, right, count, at);
                match source_ml_source1_s7.clone() {
                    self::MlTemporaryCompareUtf16TextLoop12Result::MlTemporaryCompareUtf16TextLoop12Done => {
                        if (((left.len() as f64)) == ((right.len() as f64))) {
                            0f64
                        } else {
                            if (((left.len() as f64)) < ((right.len() as f64))) {
                                (-1f64)
                            } else {
                                1f64
                            }
                        }
                    }
                    self::MlTemporaryCompareUtf16TextLoop12Result::MlTemporaryCompareUtf16TextLoop12Return(source_ml_temporary_result12) => {
                        source_ml_temporary_result12
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn starts_utf16_text(text: &[f64], search: &[f64]) -> bool {
    self::match_utf16_text(text, search, 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ends_utf16_text(text: &[f64], search: &[f64]) -> bool {
    self::match_utf16_text(text, search, ((text.len() as f64) - (search.len() as f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn includes_utf16_text(text: &[f64], search: &[f64]) -> bool {
    ((self::index_utf16_text(text, search, 0f64)) >= (0f64))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn join_utf16_text(values: &[Vec<f64>], separator: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_join_utf16_text_loop13(values, separator, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn trim_utf16_text(text: &[f64], start: bool, end: bool) -> Vec<f64> {
    {
        let first = 0f64;
        {
            let last = (text.len() as f64);
            {
                let first_3 = if start {
                    {
                        let first_2 = self::source_ml_temporary_trim_utf16_text_loop16(text, first, last);
                        first_2
                    }
                } else {
                    first
                };
                {
                    let last_3 = if end {
                        {
                            let last_2 = self::source_ml_temporary_trim_utf16_text_loop17(text, first_3, last);
                            last_2
                        }
                    } else {
                        last
                    };
                    self::slice_utf16_text(text, first_3, last_3)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn character_units_utf16_text(values: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_character_units_utf16_text_loop18(values, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn code_point_units_utf16_text(values: &[f64]) -> Vec<f64> {
    {
        let result = vec![(-1f64)];
        {
            let at = 0f64;
            {
                let source_ml_source1_s8 = self::source_ml_temporary_code_point_units_utf16_text_loop19(values, result, at);
                match source_ml_source1_s8.clone() {
                    self::MlTemporaryCodePointUnitsUtf16TextLoop19Result::MlTemporaryCodePointUnitsUtf16TextLoop19Done(result_2) => {
                        result_2
                    }
                    self::MlTemporaryCodePointUnitsUtf16TextLoop19Result::MlTemporaryCodePointUnitsUtf16TextLoop19Return(source_ml_temporary_result19) => {
                        source_ml_temporary_result19
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf8_text_units(bytes: &[f64]) -> Vec<f64> {
    self::read_java_script_utf8_string_units(bytes)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn display_points_utf16_text(text: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_display_points_utf16_text_loop20(text, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn display_utf8_from_utf16_text(text: &[f64]) -> Vec<f64> {
    {
        let bytes = Vec::<f64>::new();
        {
            let points = self::display_points_utf16_text(text);
            {
                let at = 0f64;
                {
                    let bytes_2 = self::source_ml_temporary_display_utf8_from_utf16_text_loop22(bytes, &(points), at);
                    bytes_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_utf16_text_substitution(replacement: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let start = 0f64;
            {
                let at = 0f64;
                {
                    let source_ml_source1_s9 = self::source_ml_temporary_plan_utf16_text_substitution_loop23(replacement, result, start, at);
                    match source_ml_source1_s9.clone() {
                        self::MlTemporaryPlanUtf16TextSubstitutionLoop23Result::MlTemporaryPlanUtf16TextSubstitutionLoop23Done(result_2, start_2) => {
                            {
                                let result_4 = if (start_2 < ((replacement.len() as f64))) {
                                    {
                                        let result_3 = { let (ml_append_right, ml_append_left) = (vec![0f64, start_2, (replacement.len() as f64)], result_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                        result_3
                                    }
                                } else {
                                    result_2
                                };
                                result_4
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn replace_utf16_text(text: &[f64], search: &[f64], replacement: &[f64], all: bool) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let plan = self::plan_utf16_text_substitution(replacement);
            {
                let start = 0f64;
                {
                    let from = 0f64;
                    {
                        let searching = true;
                        {
                            let source_ml_source1_s10 = self::source_ml_temporary_replace_utf16_text_loop24(text, search, replacement, all, result, &(plan), start, from, searching);
                            match source_ml_source1_s10.clone() {
                                self::MlTemporaryReplaceUtf16TextLoop24Result::MlTemporaryReplaceUtf16TextLoop24Done(result_2, start_2) => {
                                    {
                                        let at = start_2;
                                        {
                                            let result_3 = self::source_ml_temporary_replace_utf16_text_loop25(text, result_2, at);
                                            result_3
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result {
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Done(Vec<f64>),
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Done(f64, f64),
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_utf8_string_units_loop4(bytes: &[f64], mut index: f64, mut scalar: f64, mut remaining: f64) -> self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    loop {
        return if (remaining > (0f64)) {
            {
                let next = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((next < (128f64)) || (next > (191f64))) {
                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(vec![(-1f64)])
                } else {
                    {
                        let scalar_2 = (((scalar * 64f64) + next) - 128f64);
                        {
                            let index_2 = (index + 1f64);
                            {
                                let remaining_2 = (remaining - 1f64);
                                {
                                    (index, scalar, remaining) = (index_2, scalar_2, remaining_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Done(index, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsJoin3 {
    MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Next(f64, f64, f64),
    MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_utf8_string_units_loop2(bytes: &[f64], mut units: Vec<f64>, mut index: f64) -> self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let scalar = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                {
                    let index_2 = (index + 1f64);
                    {
                        let remaining = 0f64;
                        {
                            let minimum = 0f64;
                            {
                                let source_ml_source1_s11 = if ((scalar >= (194f64)) && (scalar <= (223f64))) {
                                    {
                                        let scalar_2 = (scalar - 192f64);
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_2, remaining_2, minimum_2)
                                            }
                                        }
                                    }
                                } else {
                                    if ((scalar >= (224f64)) && (scalar <= (239f64))) {
                                        {
                                            let scalar_3 = (scalar - 224f64);
                                            {
                                                let remaining_3 = 2f64;
                                                {
                                                    let minimum_3 = 2048f64;
                                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_3, remaining_3, minimum_3)
                                                }
                                            }
                                        }
                                    } else {
                                        if ((scalar >= (240f64)) && (scalar <= (244f64))) {
                                            {
                                                let scalar_4 = (scalar - 240f64);
                                                {
                                                    let remaining_4 = 3f64;
                                                    {
                                                        let minimum_4 = 65536f64;
                                                        self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_4, remaining_4, minimum_4)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (scalar >= (128f64)) {
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Return(vec![(-1f64)])
                                            } else {
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar, remaining, minimum)
                                            }
                                        }
                                    }
                                };
                                match source_ml_source1_s11.clone() {
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source1_s12 = self::source_ml_temporary_read_java_script_utf8_string_units_loop4(bytes, index_2, scalar_5, remaining_5);
                                            match source_ml_source1_s12.clone() {
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Done(index_3, scalar_6) => {
                                                    if (((scalar_6 < minimum_5) || (scalar_6 > (1114111f64))) || ((scalar_6 >= (55296f64)) && (scalar_6 <= (57343f64)))) {
                                                        self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Return(vec![(-1f64)])
                                                    } else {
                                                        if (scalar_6 <= (65535f64)) {
                                                            {
                                                                let units_2 = { let (ml_append_right, ml_append_left) = (vec![scalar_6], units); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    (units, index) = (units_2, index_3);
                                                                    continue;
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let units_3 = { let (ml_append_right, ml_append_left) = (vec![(55296f64 + (((scalar_6 - 65536f64) / 1024f64)).floor())], units); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let units_4 = { let (ml_append_right, ml_append_left) = (vec![(56320f64 + ((scalar_6 - 65536f64) % 1024f64))], units_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        (units, index) = (units_4, index_3);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(source_ml_temporary_result4) => {
                                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_temporary_result4)
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Return(source_ml_temporary_result4_2) => {
                                        self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_temporary_result4_2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Done(units)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Result {
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Done,
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_utf8_string_units_loop1(bytes: &[f64], mut index: f64) -> self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (255f64))) {
                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Return(vec![(-1f64)])
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_slice_utf16_text_loop5(text: &[f64], mut last: f64, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_points_utf16_text_loop6(text: &[f64], mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let first = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                    if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![first, second]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    let at_3 = (at_2 + 1f64);
                                    {
                                        (result, at) = (result_2, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = { let (ml_append_right, ml_append_left) = (vec![vec![first]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_4 = (at + 1f64);
                                {
                                    (result, at) = (result_3, at_4);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_split_utf16_text_loop7(text: &[f64], mut count: f64, mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if ((at < ((text.len() as f64))) && (((result.len() as f64)) < count)) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporarySplitUtf16TextLoop8Result {
    MlTemporarySplitUtf16TextLoop8Done(Vec<Vec<f64>>, f64),
    MlTemporarySplitUtf16TextLoop8Return(Vec<Vec<f64>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_split_utf16_text_loop8(text: &[f64], separator: &[f64], mut count: f64, mut result: Vec<Vec<f64>>, mut start: f64) -> self::MlTemporarySplitUtf16TextLoop8Result {
    loop {
        return {
            let found = self::index_utf16_text(text, separator, start);
            if (found < (0f64)) {
                self::MlTemporarySplitUtf16TextLoop8Result::MlTemporarySplitUtf16TextLoop8Done(result, start)
            } else {
                {
                    let result_2 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text, start, found)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                    if (((result_2.len() as f64)) == count) {
                        self::MlTemporarySplitUtf16TextLoop8Result::MlTemporarySplitUtf16TextLoop8Return(result_2)
                    } else {
                        {
                            let start_2 = (found + (separator.len() as f64));
                            {
                                (result, start) = (result_2, start_2);
                                continue;
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryMatchUtf16TextLoop9Result {
    MlTemporaryMatchUtf16TextLoop9Done,
    MlTemporaryMatchUtf16TextLoop9Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_match_utf16_text_loop9(text: &[f64], search: &[f64], mut start: f64, mut at: f64) -> self::MlTemporaryMatchUtf16TextLoop9Result {
    loop {
        return if (at < ((search.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand13 = self::ml_array::optional_at(text, self::ml_array::number_index((start + at)));
                {
                    let source_ml_source1_comparison_operand14 = self::ml_array::optional_at(search, self::ml_array::number_index(at));
                    !(match (source_ml_source1_comparison_operand13.clone(), source_ml_source1_comparison_operand14.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlTemporaryMatchUtf16TextLoop9Result::MlTemporaryMatchUtf16TextLoop9Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryMatchUtf16TextLoop9Result::MlTemporaryMatchUtf16TextLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryIndexUtf16TextLoop10Result {
    MlTemporaryIndexUtf16TextLoop10Done,
    MlTemporaryIndexUtf16TextLoop10Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_index_utf16_text_loop10(text: &[f64], search: &[f64], mut at: f64) -> self::MlTemporaryIndexUtf16TextLoop10Result {
    loop {
        return if (at <= (((text.len() as f64) - (search.len() as f64)))) {
            if self::match_utf16_text(text, search, at) {
                self::MlTemporaryIndexUtf16TextLoop10Result::MlTemporaryIndexUtf16TextLoop10Return(at)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryIndexUtf16TextLoop10Result::MlTemporaryIndexUtf16TextLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLastIndexUtf16TextLoop11Result {
    MlTemporaryLastIndexUtf16TextLoop11Done,
    MlTemporaryLastIndexUtf16TextLoop11Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_last_index_utf16_text_loop11(text: &[f64], search: &[f64], mut at: f64) -> self::MlTemporaryLastIndexUtf16TextLoop11Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text, search, at) {
                self::MlTemporaryLastIndexUtf16TextLoop11Result::MlTemporaryLastIndexUtf16TextLoop11Return(at)
            } else {
                {
                    let at_2 = (at - 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryLastIndexUtf16TextLoop11Result::MlTemporaryLastIndexUtf16TextLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareUtf16TextLoop12Result {
    MlTemporaryCompareUtf16TextLoop12Done,
    MlTemporaryCompareUtf16TextLoop12Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_utf16_text_loop12(left: &[f64], right: &[f64], mut count: f64, mut at: f64) -> self::MlTemporaryCompareUtf16TextLoop12Result {
    loop {
        return if (at < count) {
            {
                let a = (self::ml_array::optional_at(left, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let b = (self::ml_array::optional_at(right, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlTemporaryCompareUtf16TextLoop12Result::MlTemporaryCompareUtf16TextLoop12Return(if (a < b) {
                            (-1f64)
                        } else {
                            1f64
                        })
                    } else {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompareUtf16TextLoop12Result::MlTemporaryCompareUtf16TextLoop12Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop14(separator: &[f64], mut result: Vec<f64>, mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((separator.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(separator, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop15(mut result: Vec<f64>, value: &[f64], mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((value.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(value, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop13(values: &[Vec<f64>], separator: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let result_3 = if (at != (0f64)) {
                    {
                        let offset = 0f64;
                        {
                            let result_2 = self::source_ml_temporary_join_utf16_text_loop14(separator, result, offset);
                            result_2
                        }
                    }
                } else {
                    result
                };
                {
                    let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| Vec::<f64>::new());
                    {
                        let offset_2 = 0f64;
                        {
                            let result_4 = self::source_ml_temporary_join_utf16_text_loop15(result_3, &(value), offset_2);
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_4, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_trim_utf16_text_loop16(text: &[f64], mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((first < last) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(text, self::ml_array::number_index(first))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let first_2 = (first + 1f64);
                {
                    first = first_2;
                    continue;
                }
            }
        } else {
            first
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_trim_utf16_text_loop17(text: &[f64], mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((last > first) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(text, self::ml_array::number_index((last - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let last_2 = (last - 1f64);
                {
                    last = last_2;
                    continue;
                }
            }
        } else {
            last
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_character_units_utf16_text_loop18(values: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let integer = if value.is_finite() {
                        value.trunc()
                    } else {
                        0f64
                    };
                    {
                        let residue = (integer % 65536f64);
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (residue < (0f64)) {
                                (residue + 65536f64)
                            } else {
                                if (residue == (0f64)) {
                                    0f64
                                } else {
                                    residue
                                }
                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCodePointUnitsUtf16TextLoop19Result {
    MlTemporaryCodePointUnitsUtf16TextLoop19Done(Vec<f64>),
    MlTemporaryCodePointUnitsUtf16TextLoop19Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_code_point_units_utf16_text_loop19(values: &[f64], mut result: Vec<f64>, mut at: f64) -> self::MlTemporaryCodePointUnitsUtf16TextLoop19Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value > (1114111f64))) {
                    self::MlTemporaryCodePointUnitsUtf16TextLoop19Result::MlTemporaryCodePointUnitsUtf16TextLoop19Return(vec![at])
                } else {
                    if (value < (65536f64)) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (value == (0f64)) {
                                0f64
                            } else {
                                value
                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = { let (ml_append_right, ml_append_left) = (vec![(55296f64 + (((value - 65536f64) / 1024f64)).floor()), (56320f64 + ((value - 65536f64) % 1024f64))], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (result, at) = (result_3, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCodePointUnitsUtf16TextLoop19Result::MlTemporaryCodePointUnitsUtf16TextLoop19Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDisplayPointsUtf16TextJoin21 {
    MlTemporaryDisplayPointsUtf16TextJoin21Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_display_points_utf16_text_loop20(text: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
                {
                    let source_ml_source1_s15 = if ((point >= (55296f64)) && (point <= (56319f64))) {
                        {
                            let low = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                            if ((low >= (56320f64)) && (low <= (57343f64))) {
                                {
                                    let point_2 = (((65536f64 + ((point - 55296f64) * 1024f64)) + low) - 56320f64);
                                    {
                                        let at_2 = (at + 1f64);
                                        self::MlTemporaryDisplayPointsUtf16TextJoin21::MlTemporaryDisplayPointsUtf16TextJoin21Next(at_2, point_2)
                                    }
                                }
                            } else {
                                {
                                    let point_3 = 65533f64;
                                    self::MlTemporaryDisplayPointsUtf16TextJoin21::MlTemporaryDisplayPointsUtf16TextJoin21Next(at, point_3)
                                }
                            }
                        }
                    } else {
                        if ((point >= (56320f64)) && (point <= (57343f64))) {
                            {
                                let point_4 = 65533f64;
                                self::MlTemporaryDisplayPointsUtf16TextJoin21::MlTemporaryDisplayPointsUtf16TextJoin21Next(at, point_4)
                            }
                        } else {
                            self::MlTemporaryDisplayPointsUtf16TextJoin21::MlTemporaryDisplayPointsUtf16TextJoin21Next(at, point)
                        }
                    };
                    match source_ml_source1_s15.clone() {
                        self::MlTemporaryDisplayPointsUtf16TextJoin21::MlTemporaryDisplayPointsUtf16TextJoin21Next(at_3, point_5) => {
                            {
                                let result_2 = { let (ml_append_right, ml_append_left) = (vec![point_5], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_4 = (at_3 + 1f64);
                                    {
                                        (result, at) = (result_2, at_4);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_display_utf8_from_utf16_text_loop22(mut bytes: Vec<f64>, points: &[f64], mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((points.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(points, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
                if (point < (128f64)) {
                    {
                        let bytes_2 = { let (ml_append_right, ml_append_left) = (vec![point], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let at_2 = (at + 1f64);
                            {
                                (bytes, at) = (bytes_2, at_2);
                                continue;
                            }
                        }
                    }
                } else {
                    if (point < (2048f64)) {
                        {
                            let bytes_3 = { let (ml_append_right, ml_append_left) = (vec![(192f64 + ((point / 64f64)).floor()), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (bytes, at) = (bytes_3, at_3);
                                    continue;
                                }
                            }
                        }
                    } else {
                        if (point < (65536f64)) {
                            {
                                let bytes_4 = { let (ml_append_right, ml_append_left) = (vec![(224f64 + ((point / 4096f64)).floor()), (128f64 + (((point / 64f64)).floor() % 64f64)), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_4 = (at + 1f64);
                                    {
                                        (bytes, at) = (bytes_4, at_4);
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let bytes_5 = { let (ml_append_right, ml_append_left) = (vec![(240f64 + ((point / 262144f64)).floor()), (128f64 + (((point / 4096f64)).floor() % 64f64)), (128f64 + (((point / 64f64)).floor() % 64f64)), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_5 = (at + 1f64);
                                    {
                                        (bytes, at) = (bytes_5, at_5);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            bytes
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanUtf16TextSubstitutionLoop23Result {
    MlTemporaryPlanUtf16TextSubstitutionLoop23Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_utf16_text_substitution_loop23(replacement: &[f64], mut result: Vec<f64>, mut start: f64, mut at: f64) -> self::MlTemporaryPlanUtf16TextSubstitutionLoop23Result {
    loop {
        return if (at < ((replacement.len() as f64))) {
            {
                let next = (self::ml_array::optional_at(replacement, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                if ((((self::ml_array::optional_at(replacement, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)) == (36f64)) && ((((next == (36f64)) || (next == (38f64))) || (next == (96f64))) || (next == (39f64)))) {
                    {
                        let result_3 = if (start < at) {
                            {
                                let result_2 = { let (ml_append_right, ml_append_left) = (vec![0f64, start, at], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                result_2
                            }
                        } else {
                            result
                        };
                        {
                            let result_6 = if (next == (36f64)) {
                                {
                                    let result_4 = { let (ml_append_right, ml_append_left) = (vec![0f64, at, (at + 1f64)], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                    result_4
                                }
                            } else {
                                {
                                    let result_5 = { let (ml_append_right, ml_append_left) = (vec![if (next == (38f64)) {
                                        1f64
                                    } else {
                                        if (next == (96f64)) {
                                            2f64
                                        } else {
                                            3f64
                                        }
                                    }, 0f64, 0f64], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                    result_5
                                }
                            };
                            {
                                let at_2 = (at + 2f64);
                                {
                                    let start_2 = at_2;
                                    {
                                        (result, start, at) = (result_6, start_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let at_3 = (at + 1f64);
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanUtf16TextSubstitutionLoop23Result::MlTemporaryPlanUtf16TextSubstitutionLoop23Done(result, start)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop25(text: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReplaceUtf16TextLoop24Result {
    MlTemporaryReplaceUtf16TextLoop24Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop26(text: &[f64], mut result: Vec<f64>, mut found: f64, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < found) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop28(mut result: Vec<f64>, mut last: f64, source: &[f64], mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(source, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop27(text: &[f64], search: &[f64], replacement: &[f64], mut result: Vec<f64>, plan: &[f64], mut found: f64, mut instruction: f64) -> Vec<f64> {
    loop {
        return if (instruction < ((plan.len() as f64))) {
            {
                let kind = (self::ml_array::optional_at(plan, self::ml_array::number_index(instruction))).clone().unwrap_or_else(|| 0f64);
                {
                    let first = if (kind == (0f64)) {
                        (self::ml_array::optional_at(plan, self::ml_array::number_index((instruction + 1f64)))).clone().unwrap_or_else(|| 0f64)
                    } else {
                        if (kind == (2f64)) {
                            0f64
                        } else {
                            if (kind == (1f64)) {
                                found
                            } else {
                                (found + (search.len() as f64))
                            }
                        }
                    };
                    {
                        let last = if (kind == (0f64)) {
                            (self::ml_array::optional_at(plan, self::ml_array::number_index((instruction + 2f64)))).clone().unwrap_or_else(|| 0f64)
                        } else {
                            if (kind == (2f64)) {
                                found
                            } else {
                                if (kind == (1f64)) {
                                    (found + (search.len() as f64))
                                } else {
                                    (text.len() as f64)
                                }
                            }
                        };
                        {
                            let source = if (kind == (0f64)) {
                                replacement.to_vec()
                            } else {
                                text.to_vec()
                            };
                            {
                                let at = first;
                                {
                                    let result_2 = self::source_ml_temporary_replace_utf16_text_loop28(result, last, &(source), at);
                                    {
                                        let instruction_2 = (instruction + 3f64);
                                        {
                                            (result, instruction) = (result_2, instruction_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop24(text: &[f64], search: &[f64], replacement: &[f64], mut all: bool, mut result: Vec<f64>, plan: &[f64], mut start: f64, mut from: f64, mut searching: bool) -> self::MlTemporaryReplaceUtf16TextLoop24Result {
    loop {
        return if searching {
            {
                let found = self::index_utf16_text(text, search, from);
                if (found < (0f64)) {
                    {
                        let searching_2 = false;
                        {
                            searching = searching_2;
                            continue;
                        }
                    }
                } else {
                    {
                        let at = start;
                        {
                            let result_2 = self::source_ml_temporary_replace_utf16_text_loop26(text, result, found, at);
                            {
                                let instruction = 0f64;
                                {
                                    let result_3 = self::source_ml_temporary_replace_utf16_text_loop27(text, search, replacement, result_2, plan, found, instruction);
                                    {
                                        let start_2 = (found + (search.len() as f64));
                                        {
                                            let searching_3 = (all && !(((((search.len() as f64)) == (0f64)) && (found == ((text.len() as f64))))));
                                            {
                                                let from_2 = (start_2 + if (((search.len() as f64)) == (0f64)) {
                                                    1f64
                                                } else {
                                                    0f64
                                                });
                                                {
                                                    (result, start, from, searching) = (result_3, start_2, from_2, searching_3);
                                                    continue;
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
        } else {
            self::MlTemporaryReplaceUtf16TextLoop24Result::MlTemporaryReplaceUtf16TextLoop24Done(result, start)
        };
    }
}
}


#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSemanticValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    Integer(self::ml::Big),
    Text(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    SourceText(self::ml_string::Text, self::ml_string::Text),
    Symbol(self::ml::Big),
    Object(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallPrimitiveResult {
    PrimitiveValue(Box<self::JavaScriptSemanticValue>),
    PrimitiveInvalid(self::ml_string::Text),
    PrimitiveError(self::ml_string::Text, self::ml_string::Text),
    PrimitiveResourceLimit(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryProperty {
    OrdinaryProperty(Box<self::JavaScriptSemanticValue>, std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionEntry {
    CollectionEntry(Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptWeakCollectionEntry {
    WeakCollectionEntry(Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptClosureObject {
    ClosureObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, self::ml::Big, f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, Box<self::JavaScriptSemanticValue>, Option<(Option<self::JavaScriptSemanticValue>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptBoundFunctionObject {
    BoundFunctionObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayObject {
    ArrayObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryObject {
    ClosureObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, self::ml::Big, f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, Box<self::JavaScriptSemanticValue>, Option<(Option<self::JavaScriptSemanticValue>,)>),
    BoundFunctionObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>),
    ArrayObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>),
    CollectionIteratorObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, self::ml::Big, f64, self::ml_string::Text, self::ml_string::Text, Option<(Option<bool>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>),
    MapObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>),
    SetObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>),
    WeakMapObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>),
    WeakSetObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>),
    OrdinaryObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, Option<(Option<bool>,)>, Option<(Option<bool>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>),
    HostErrorObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>),
    HostURLObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>),
    HostArrayObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>),
    HostByteBufferObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSymbolRegistrationFact {
    SymbolRegistrationFact(self::ml::Big, bool, Option<(Option<self::JavaScriptSemanticValue>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSymbolRegistryEntry {
    SymbolRegistryEntry(self::ml::Big, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryHeap {
    OrdinaryHeap(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, Option<(Option<self::ml_string::Text>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,)>, Option<(Option<self::ml_string::Text>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayPropertyPlan {
    ArrayPropertyPlan(Box<self::JavaScriptOrdinaryObject>, bool, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>),
}

pub fn read_java_script_source_text_profile_error(profile: self::ml_string::Text, length: f64) -> self::ml_string::Text {
    if (profile != (self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]))) {
        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101])
    } else {
        if (((!(self::ml_math::is_safe_integer(length)) || (length < (0f64))) || (length > (16777216f64))) || ((length == (0f64)) && (((1f64 / length)) < (0f64)))) {
            self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 49, 54, 55, 55, 55, 50, 49, 54, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_java_script_semantic_value_error(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Null => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f2) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f3) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Text(units) => {
            if (({ let ml_reference_array1 = units.clone(); let ml_reference_projection1 = ml_reference_array1.borrow().len() as f64; ml_reference_projection1 }) > (1048576f64)) {
                self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s5 = self::source_ml_temporary_read_java_script_semantic_value_error_loop1(units.clone(), at);
                        match source_ml_source1_s5.clone() {
                            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(source_ml_temporary_result1) => {
                                source_ml_temporary_result1.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(profile, text) => {
            if ((self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 83, 116, 114, 105, 110, 103, 32, 118, 97, 108, 117, 101])
            } else {
                self::read_java_script_source_text_profile_error(profile.clone(), text.length())
            }
        }
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if ((identity > (self::ml::Big::from_i128(0))) && (identity <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f4) => {
            if ((identity_2 > (self::ml::Big::from_i128(0))) && (identity_2 <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
    }
}

pub fn test_java_script_semantic_same_value(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_java_script_semantic_value_error(right.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![83, 97, 109, 101, 86, 97, 108, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 119, 111, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 117, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 109, 111, 100, 101, 108])).0, message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f10) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f11) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f12) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f13) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f14, source_ml_source1_f15) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f16) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f17, source_ml_source1_f18) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Null => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f19) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f20) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f21) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f22) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f23, source_ml_source1_f24) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f25) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f26, source_ml_source1_f27) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Boolean(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(value) => {
                        (left_value == value)
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f28) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f29) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f30) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f31, source_ml_source1_f32) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f33) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f34, source_ml_source1_f35) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Number(left_value_2) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f36) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f37) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f38) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f39, source_ml_source1_f40) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f41) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f42, source_ml_source1_f43) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(left_value_3) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f44) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f45) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f46) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f47, source_ml_source1_f48) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f49) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f50, source_ml_source1_f51) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f6) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f7, source_ml_source1_f8) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(left_identity) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f52) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f53) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f54) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f55) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f56, source_ml_source1_f57) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f58, source_ml_source1_f59) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, source_ml_source1_f9) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f60) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f61) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f62) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f63) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f64, source_ml_source1_f65) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f66) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f67) => {
                        (left_identity_2 == identity_2)
                    }
                }
            }
        }
    }
}

pub fn read_java_script_semantic_text_length(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Null => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f68) => {
            {
                let source_ml_source1_f92 = source_ml_source1_f68;
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f69) => {
            {
                let source_ml_source1_f101 = source_ml_source1_f69;
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f70) => {
            {
                let source_ml_source1_f110 = source_ml_source1_f70.clone();
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f71) => {
            {
                let units = source_ml_source1_f71.clone();
                { let ml_reference_array2 = units.clone(); let ml_reference_projection2 = ml_reference_array2.borrow().len() as f64; ml_reference_projection2 }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f72, text) => {
            text.length()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f73) => {
            {
                let source_ml_source1_f129 = source_ml_source1_f73.clone();
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f74, source_ml_source1_f75) => {
            {
                let source_ml_source1_f138 = source_ml_source1_f74.clone();
                {
                    let source_ml_source1_f139 = source_ml_source1_f75;
                    (-1f64)
                }
            }
        }
    }
}

pub fn read_java_script_semantic_text_unit(value: self::JavaScriptSemanticValue, index: f64) -> f64 {
    if ((!(self::ml_math::is_safe_integer(index)) || (index < (0f64))) || (index >= (self::read_java_script_semantic_text_length(value.clone())))) {
        (-1f64)
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                (-1f64)
            }
            self::JavaScriptSemanticValue::Null => {
                (-1f64)
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f140) => {
                {
                    let source_ml_source1_f164 = source_ml_source1_f140;
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f141) => {
                {
                    let source_ml_source1_f173 = source_ml_source1_f141;
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f142) => {
                {
                    let source_ml_source1_f182 = source_ml_source1_f142.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f143) => {
                {
                    let units = source_ml_source1_f143.clone();
                    ({
                        let source_ml_source1_array_read_value196 = units.clone();
                        {
                            let source_ml_source1_array_read_index197 = index;
                            { let ml_reference_array3 = source_ml_source1_array_read_value196.clone(); let ml_reference_position3 = source_ml_source1_array_read_index197; let ml_reference_projection3 = self::ml_array::optional_at(&*ml_reference_array3.borrow(), self::ml_array::number_index(ml_reference_position3)); ml_reference_projection3 }
                        }
                    }).clone().unwrap_or_else(|| (-1f64))
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f144, text) => {
                {
                    let source_ml_source1_string_unit_operand198 = text.clone();
                    {
                        let source_ml_source1_string_unit_operand199 = index;
                        (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text_2.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source1_string_unit_operand205 = {
                                        let source_ml_source1_string_range_operand200 = text_2.clone();
                                        {
                                            let source_ml_source1_string_range_operand201 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand202 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source1_string_range_position203 = source_ml_source1_string_range_operand201;
                                                    {
                                                        let source_ml_source1_string_range_position204 = source_ml_source1_string_range_operand202;
                                                        source_ml_source1_string_range_operand200.slice(source_ml_source1_string_range_position203, source_ml_source1_string_range_position204)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand206 = 0f64;
                                        source_ml_source1_string_unit_operand205.code_point_at(source_ml_source1_string_unit_operand206)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source1_string_unit_operand198.clone(), source_ml_source1_string_unit_operand199)
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f145) => {
                {
                    let source_ml_source1_f212 = source_ml_source1_f145.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f146, source_ml_source1_f147) => {
                {
                    let source_ml_source1_f221 = source_ml_source1_f146.clone();
                    {
                        let source_ml_source1_f222 = source_ml_source1_f147;
                        (-1f64)
                    }
                }
            }
        }
    }
}

pub fn test_java_script_semantic_text_equal(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    {
        let length = self::read_java_script_semantic_text_length(left.clone());
        if ((length < (0f64)) || (length != (self::read_java_script_semantic_text_length(right.clone())))) {
            false
        } else {
            if (match left.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f223) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f224) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f225) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f226) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f227, source_ml_source1_f228) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f229) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f230, source_ml_source1_f231) => {
                    false
                }
            } && match right.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f232) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f233) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f234) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f235) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f236, source_ml_source1_f237) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f238) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f239, source_ml_source1_f240) => {
                    false
                }
            }) {
                {
                    let source_ml_source1_comparison_operand243 = match left.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Null => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Number(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value241) => {
                            Some(source_ml_source1_data_property_value241.clone())
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    {
                        let source_ml_source1_comparison_operand244 = match right.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Null => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value242) => {
                                Some(source_ml_source1_data_property_value242.clone())
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<self::ml_string::Text>
                            }
                        };
                        match (source_ml_source1_comparison_operand243.clone(), source_ml_source1_comparison_operand244.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false }
                    }
                }
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s245 = self::source_ml_temporary_test_java_script_semantic_text_equal_loop2(left.clone(), right.clone(), length, at);
                        match source_ml_source1_s245.clone() {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done => {
                                true
                            }
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(source_ml_temporary_result2) => {
                                source_ml_temporary_result2
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_semantic_text_string(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Null => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f246) => {
            {
                let source_ml_source1_f270 = source_ml_source1_f246;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f247) => {
            {
                let source_ml_source1_f279 = source_ml_source1_f247;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f248) => {
            {
                let source_ml_source1_f288 = source_ml_source1_f248.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f249) => {
            {
                let units = source_ml_source1_f249.clone();
                {
                    let text = self::ml_string::Text::from_units(vec![]);
                    {
                        let source_ml_temporary_values3 = units.clone();
                        {
                            let source_ml_temporary_index3 = self::ml::Big::from_i128(0);
                            {
                                let text_2 = self::source_ml_temporary_read_java_script_semantic_text_string_loop4(text.clone(), source_ml_temporary_values3.clone(), source_ml_temporary_index3.clone());
                                text_2.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f250, text_3) => {
            text_3.clone()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f251) => {
            {
                let source_ml_source1_f307 = source_ml_source1_f251.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f252, source_ml_source1_f253) => {
            {
                let source_ml_source1_f316 = source_ml_source1_f252.clone();
                {
                    let source_ml_source1_f317 = source_ml_source1_f253;
                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
                }
            }
        }
    }
}

pub fn construct_java_script_semantic_text_units(value: self::JavaScriptSemanticValue, limit: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let length = self::read_java_script_semantic_text_length(value.clone());
        if ((length < (0f64)) || (length > limit)) {
            (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])).0, message_present: true }).raise()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    {
                        let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_s326 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units.clone(), at);
                                match source_ml_source1_s326.clone() {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units.clone()
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Null => {
                    {
                        let units_2 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_2 = 0f64;
                            {
                                let source_ml_source1_s327 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_2.clone(), at_2);
                                match source_ml_source1_s327.clone() {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_2.clone()
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f318) => {
                    {
                        let units_3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_3 = 0f64;
                            {
                                let source_ml_source1_s328 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_3.clone(), at_3);
                                match source_ml_source1_s328.clone() {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_3.clone()
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f319) => {
                    {
                        let units_4 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_4 = 0f64;
                            {
                                let source_ml_source1_s329 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_4.clone(), at_4);
                                match source_ml_source1_s329.clone() {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_4.clone()
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f320) => {
                    {
                        let units_5 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_5 = 0f64;
                            {
                                let source_ml_source1_s330 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_5.clone(), at_5);
                                match source_ml_source1_s330.clone() {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_5.clone()
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(units_6) => {
                    units_6.clone()
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f321, source_ml_source1_f322) => {
                    {
                        let units_7 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_6 = 0f64;
                            {
                                let source_ml_source1_s331 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_7.clone(), at_6);
                                match source_ml_source1_s331.clone() {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_7.clone()
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f323) => {
                    {
                        let units_8 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_7 = 0f64;
                            {
                                let source_ml_source1_s332 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_8.clone(), at_7);
                                match source_ml_source1_s332.clone() {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_8.clone()
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f324, source_ml_source1_f325) => {
                    {
                        let units_9 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_8 = 0f64;
                            {
                                let source_ml_source1_s333 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_9.clone(), at_8);
                                match source_ml_source1_s333.clone() {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_9.clone()
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

pub fn construct_java_script_source_text(text: self::ml_string::Text) -> self::JavaScriptSemanticValue {
    if ((self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 83, 116, 114, 105, 110, 103, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
    } else {
        {
            let profile = self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]);
            {
                let error = self::read_java_script_source_text_profile_error(profile.clone(), text.length());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (error.clone()).0, message_present: true }).raise()
                } else {
                    self::JavaScriptSemanticValue::SourceText(profile.clone(), text.clone())
                }
            }
        }
    }
}

pub fn test_java_script_semantic_accessor(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
        false
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                true
            }
            self::JavaScriptSemanticValue::Null => {
                false
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f334) => {
                {
                    let source_ml_source1_f351 = source_ml_source1_f334;
                    false
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f335) => {
                {
                    let source_ml_source1_f360 = source_ml_source1_f335;
                    false
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f336) => {
                {
                    let source_ml_source1_f369 = source_ml_source1_f336.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f337) => {
                {
                    let source_ml_source1_f378 = source_ml_source1_f337.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f338, source_ml_source1_f339) => {
                {
                    let source_ml_source1_f387 = source_ml_source1_f338.clone();
                    {
                        let source_ml_source1_f388 = source_ml_source1_f339.clone();
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f340) => {
                {
                    let source_ml_source1_f397 = source_ml_source1_f340.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f341, source_ml_source1_f342) => {
                {
                    let source_ml_source1_f406 = source_ml_source1_f341.clone();
                    {
                        let callable = source_ml_source1_f342;
                        callable
                    }
                }
            }
        }
    }
}

pub fn find_java_script_property_descriptor_entry(properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, key: self::JavaScriptSemanticValue) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source1_s407 = self::source_ml_temporary_find_java_script_property_descriptor_entry_loop7(properties.clone(), key.clone(), at);
            match source_ml_source1_s407.clone() {
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Done => {
                    (-1f64)
                }
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Return(source_ml_temporary_result7) => {
                    source_ml_temporary_result7
                }
            }
        }
    }
}

pub fn read_java_script_property_descriptor_kind(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::ml_string::Text {
    {
        let data = (({ let ml_reference_value4 = descriptor.borrow().clone(); ml_reference_value4 }).9 || ({ let ml_reference_value5 = descriptor.borrow().clone(); ml_reference_value5 }).11);
        {
            let accessor = (({ let ml_reference_value6 = descriptor.borrow().clone(); ml_reference_value6 }).5 || ({ let ml_reference_value7 = descriptor.borrow().clone(); ml_reference_value7 }).7);
            if (data && accessor) {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else {
                if data {
                    self::ml_string::Text::from_units(vec![100, 97, 116, 97])
                } else {
                    if accessor {
                        self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
                    } else {
                        self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
                    }
                }
            }
        }
    }
}

pub fn test_java_script_property_descriptors_same_value(left: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, right: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> bool {
    !((((((((((((((({ let ml_reference_value8 = left.borrow().clone(); ml_reference_value8 }).1) != (({ let ml_reference_value9 = right.borrow().clone(); ml_reference_value9 }).1)) || (({ let ml_reference_value10 = left.borrow().clone(); ml_reference_value10 }).1 && ((({ let ml_reference_value11 = left.borrow().clone(); ml_reference_value11 }).0) != (({ let ml_reference_value12 = right.borrow().clone(); ml_reference_value12 }).0)))) || ((({ let ml_reference_value13 = left.borrow().clone(); ml_reference_value13 }).3) != (({ let ml_reference_value14 = right.borrow().clone(); ml_reference_value14 }).3))) || (({ let ml_reference_value15 = left.borrow().clone(); ml_reference_value15 }).3 && ((({ let ml_reference_value16 = left.borrow().clone(); ml_reference_value16 }).2) != (({ let ml_reference_value17 = right.borrow().clone(); ml_reference_value17 }).2)))) || ((({ let ml_reference_value18 = left.borrow().clone(); ml_reference_value18 }).11) != (({ let ml_reference_value19 = right.borrow().clone(); ml_reference_value19 }).11))) || (({ let ml_reference_value20 = left.borrow().clone(); ml_reference_value20 }).11 && ((({ let ml_reference_value21 = left.borrow().clone(); ml_reference_value21 }).10) != (({ let ml_reference_value22 = right.borrow().clone(); ml_reference_value22 }).10)))) || ((({ let ml_reference_value23 = left.borrow().clone(); ml_reference_value23 }).9) != (({ let ml_reference_value24 = right.borrow().clone(); ml_reference_value24 }).9))) || (({ let ml_reference_value25 = left.borrow().clone(); ml_reference_value25 }).9 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value26 = left.borrow().clone(); ml_reference_value26 }).8, ({ let ml_reference_value27 = right.borrow().clone(); ml_reference_value27 }).8)))) || ((({ let ml_reference_value28 = left.borrow().clone(); ml_reference_value28 }).5) != (({ let ml_reference_value29 = right.borrow().clone(); ml_reference_value29 }).5))) || (({ let ml_reference_value30 = left.borrow().clone(); ml_reference_value30 }).5 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value31 = left.borrow().clone(); ml_reference_value31 }).4, ({ let ml_reference_value32 = right.borrow().clone(); ml_reference_value32 }).4)))) || ((({ let ml_reference_value33 = left.borrow().clone(); ml_reference_value33 }).7) != (({ let ml_reference_value34 = right.borrow().clone(); ml_reference_value34 }).7))) || (({ let ml_reference_value35 = left.borrow().clone(); ml_reference_value35 }).7 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value36 = left.borrow().clone(); ml_reference_value36 }).6, ({ let ml_reference_value37 = right.borrow().clone(); ml_reference_value37 }).6)))))
}

pub fn read_java_script_property_descriptor_error(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::ml_string::Text {
    if ((self::read_java_script_property_descriptor_kind(descriptor.clone())) == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))) {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        if (({ let ml_reference_value38 = descriptor.borrow().clone(); ml_reference_value38 }).9 && ((self::read_java_script_semantic_value_error(({ let ml_reference_value39 = descriptor.borrow().clone(); ml_reference_value39 }).8)) != (self::ml_string::Text::from_units(vec![])))) {
            self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108, 117, 101])
        } else {
            if (({ let ml_reference_value40 = descriptor.borrow().clone(); ml_reference_value40 }).5 && (((self::read_java_script_semantic_value_error(({ let ml_reference_value41 = descriptor.borrow().clone(); ml_reference_value41 }).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor(({ let ml_reference_value42 = descriptor.borrow().clone(); ml_reference_value42 }).4)))) {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                if (({ let ml_reference_value43 = descriptor.borrow().clone(); ml_reference_value43 }).7 && (((self::read_java_script_semantic_value_error(({ let ml_reference_value44 = descriptor.borrow().clone(); ml_reference_value44 }).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor(({ let ml_reference_value45 = descriptor.borrow().clone(); ml_reference_value45 }).6)))) {
                    self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn test_complete_java_script_property_descriptor(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> bool {
    if ((!(({ let ml_reference_value46 = descriptor.borrow().clone(); ml_reference_value46 }).1) || !(({ let ml_reference_value47 = descriptor.borrow().clone(); ml_reference_value47 }).3)) || ((self::read_java_script_property_descriptor_error(descriptor.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if (kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97]))) {
                (({ let ml_reference_value48 = descriptor.borrow().clone(); ml_reference_value48 }).9 && ({ let ml_reference_value49 = descriptor.borrow().clone(); ml_reference_value49 }).11)
            } else {
                (((kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) && ({ let ml_reference_value50 = descriptor.borrow().clone(); ml_reference_value50 }).5) && ({ let ml_reference_value51 = descriptor.borrow().clone(); ml_reference_value51 }).7)
            }
        }
    }
}

pub fn create_java_script_property_descriptor(kind: self::ml_string::Text, configurable: bool, enumerable: bool) -> std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source1_field408 = true;
        {
            let source_ml_source1_field409 = configurable;
            {
                let source_ml_source1_field410 = true;
                {
                    let source_ml_source1_field411 = enumerable;
                    {
                        let source_ml_source1_field412 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                        {
                            let source_ml_source1_field413 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field414 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                {
                                    let source_ml_source1_field415 = false;
                                    {
                                        let source_ml_source1_field416 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                        {
                                            let source_ml_source1_field417 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field418 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                                {
                                                    let source_ml_source1_field419 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field409, source_ml_source1_field408, source_ml_source1_field411, source_ml_source1_field410, source_ml_source1_field417.clone(), source_ml_source1_field416, source_ml_source1_field419.clone(), source_ml_source1_field418, source_ml_source1_field413.clone(), source_ml_source1_field412, source_ml_source1_field415, source_ml_source1_field414)
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
    }))
}

pub fn apply_java_script_property_descriptor_fields(base: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source1_field420 = true;
        {
            let source_ml_source1_field421 = if ({ let ml_reference_value52 = proposed.borrow().clone(); ml_reference_value52 }).1 {
                ({ let ml_reference_value53 = proposed.borrow().clone(); ml_reference_value53 }).0
            } else {
                ({ let ml_reference_value54 = base.borrow().clone(); ml_reference_value54 }).0
            };
            {
                let source_ml_source1_field422 = true;
                {
                    let source_ml_source1_field423 = if ({ let ml_reference_value55 = proposed.borrow().clone(); ml_reference_value55 }).3 {
                        ({ let ml_reference_value56 = proposed.borrow().clone(); ml_reference_value56 }).2
                    } else {
                        ({ let ml_reference_value57 = base.borrow().clone(); ml_reference_value57 }).2
                    };
                    {
                        let source_ml_source1_field424 = ({ let ml_reference_value58 = base.borrow().clone(); ml_reference_value58 }).9;
                        {
                            let source_ml_source1_field425 = if (({ let ml_reference_value59 = base.borrow().clone(); ml_reference_value59 }).9 && ({ let ml_reference_value60 = proposed.borrow().clone(); ml_reference_value60 }).9) {
                                ({ let ml_reference_value61 = proposed.borrow().clone(); ml_reference_value61 }).8
                            } else {
                                ({ let ml_reference_value62 = base.borrow().clone(); ml_reference_value62 }).8
                            };
                            {
                                let source_ml_source1_field426 = ({ let ml_reference_value63 = base.borrow().clone(); ml_reference_value63 }).11;
                                {
                                    let source_ml_source1_field427 = if (({ let ml_reference_value64 = base.borrow().clone(); ml_reference_value64 }).11 && ({ let ml_reference_value65 = proposed.borrow().clone(); ml_reference_value65 }).11) {
                                        ({ let ml_reference_value66 = proposed.borrow().clone(); ml_reference_value66 }).10
                                    } else {
                                        ({ let ml_reference_value67 = base.borrow().clone(); ml_reference_value67 }).10
                                    };
                                    {
                                        let source_ml_source1_field428 = ({ let ml_reference_value68 = base.borrow().clone(); ml_reference_value68 }).5;
                                        {
                                            let source_ml_source1_field429 = if (({ let ml_reference_value69 = base.borrow().clone(); ml_reference_value69 }).5 && ({ let ml_reference_value70 = proposed.borrow().clone(); ml_reference_value70 }).5) {
                                                ({ let ml_reference_value71 = proposed.borrow().clone(); ml_reference_value71 }).4
                                            } else {
                                                ({ let ml_reference_value72 = base.borrow().clone(); ml_reference_value72 }).4
                                            };
                                            {
                                                let source_ml_source1_field430 = ({ let ml_reference_value73 = base.borrow().clone(); ml_reference_value73 }).7;
                                                {
                                                    let source_ml_source1_field431 = if (({ let ml_reference_value74 = base.borrow().clone(); ml_reference_value74 }).7 && ({ let ml_reference_value75 = proposed.borrow().clone(); ml_reference_value75 }).7) {
                                                        ({ let ml_reference_value76 = proposed.borrow().clone(); ml_reference_value76 }).6
                                                    } else {
                                                        ({ let ml_reference_value77 = base.borrow().clone(); ml_reference_value77 }).6
                                                    };
                                                    (source_ml_source1_field421, source_ml_source1_field420, source_ml_source1_field423, source_ml_source1_field422, source_ml_source1_field429.clone(), source_ml_source1_field428, source_ml_source1_field431.clone(), source_ml_source1_field430, source_ml_source1_field425.clone(), source_ml_source1_field424, source_ml_source1_field427, source_ml_source1_field426)
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
    }))
}

pub fn read_java_script_property_descriptor_writes(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    {
        let writes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
        {
            let writes_3 = if ({ let ml_reference_value78 = descriptor.borrow().clone(); ml_reference_value78 }).1 {
                {
                    let writes_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source1_concat_value432 = writes.clone();
                        {
                            let source_ml_source1_concat_value433 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])]));
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value79 = source_ml_source1_concat_value432.borrow().clone(); ml_reference_value79 }), { let ml_reference_value80 = source_ml_source1_concat_value433.borrow().clone(); ml_reference_value80 })
                        }
                    }));
                    writes_2.clone()
                }
            } else {
                writes.clone()
            };
            {
                let writes_5 = if ({ let ml_reference_value81 = descriptor.borrow().clone(); ml_reference_value81 }).3 {
                    {
                        let writes_4 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source1_concat_value434 = writes_3.clone();
                            {
                                let source_ml_source1_concat_value435 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value82 = source_ml_source1_concat_value434.borrow().clone(); ml_reference_value82 }), { let ml_reference_value83 = source_ml_source1_concat_value435.borrow().clone(); ml_reference_value83 })
                            }
                        }));
                        writes_4.clone()
                    }
                } else {
                    writes_3.clone()
                };
                {
                    let writes_7 = if ({ let ml_reference_value84 = descriptor.borrow().clone(); ml_reference_value84 }).9 {
                        {
                            let writes_6 = std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source1_concat_value436 = writes_5.clone();
                                {
                                    let source_ml_source1_concat_value437 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])]));
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value85 = source_ml_source1_concat_value436.borrow().clone(); ml_reference_value85 }), { let ml_reference_value86 = source_ml_source1_concat_value437.borrow().clone(); ml_reference_value86 })
                                }
                            }));
                            writes_6.clone()
                        }
                    } else {
                        writes_5.clone()
                    };
                    {
                        let writes_9 = if ({ let ml_reference_value87 = descriptor.borrow().clone(); ml_reference_value87 }).11 {
                            {
                                let writes_8 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source1_concat_value438 = writes_7.clone();
                                    {
                                        let source_ml_source1_concat_value439 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]));
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value88 = source_ml_source1_concat_value438.borrow().clone(); ml_reference_value88 }), { let ml_reference_value89 = source_ml_source1_concat_value439.borrow().clone(); ml_reference_value89 })
                                    }
                                }));
                                writes_8.clone()
                            }
                        } else {
                            writes_7.clone()
                        };
                        {
                            let writes_11 = if ({ let ml_reference_value90 = descriptor.borrow().clone(); ml_reference_value90 }).5 {
                                {
                                    let writes_10 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_concat_value440 = writes_9.clone();
                                        {
                                            let source_ml_source1_concat_value441 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![103, 101, 116])]));
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value91 = source_ml_source1_concat_value440.borrow().clone(); ml_reference_value91 }), { let ml_reference_value92 = source_ml_source1_concat_value441.borrow().clone(); ml_reference_value92 })
                                        }
                                    }));
                                    writes_10.clone()
                                }
                            } else {
                                writes_9.clone()
                            };
                            {
                                let writes_13 = if ({ let ml_reference_value93 = descriptor.borrow().clone(); ml_reference_value93 }).7 {
                                    {
                                        let writes_12 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source1_concat_value442 = writes_11.clone();
                                            {
                                                let source_ml_source1_concat_value443 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 101, 116])]));
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value94 = source_ml_source1_concat_value442.borrow().clone(); ml_reference_value94 }), { let ml_reference_value95 = source_ml_source1_concat_value443.borrow().clone(); ml_reference_value95 })
                                            }
                                        }));
                                        writes_12.clone()
                                    }
                                } else {
                                    writes_11.clone()
                                };
                                writes_13.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn create_java_script_property_transition(disposition: self::ml_string::Text, error: self::ml_string::Text, descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, writes: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source1_field444 = disposition.clone();
        {
            let source_ml_source1_field445 = error.clone();
            {
                let source_ml_source1_field446 = descriptor.clone();
                {
                    let source_ml_source1_field447 = writes.clone();
                    (source_ml_source1_field446.clone(), source_ml_source1_field444.clone(), source_ml_source1_field445.clone(), source_ml_source1_field447.clone())
                }
            }
        }
    }))
}

pub fn plan_java_script_property_definition(exists: bool, extensible: bool, current: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>> {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), error.clone(), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
        } else {
            if (exists && !(self::test_complete_java_script_property_descriptor(current.clone()))) {
                self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 67, 117, 114, 114, 101, 110, 116, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
            } else {
                {
                    let kind = self::read_java_script_property_descriptor_kind(proposed.clone());
                    if !exists {
                        if !extensible {
                            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            {
                                let base = self::create_java_script_property_descriptor(kind.clone(), false, false);
                                {
                                    let descriptor = self::apply_java_script_property_descriptor_fields(base.clone(), proposed.clone());
                                    self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![99, 114, 101, 97, 116, 101]), self::ml_string::Text::from_units(vec![]), descriptor.clone(), self::read_java_script_property_descriptor_writes(descriptor.clone()))
                                }
                            }
                        }
                    } else {
                        if (((((!(({ let ml_reference_value96 = proposed.borrow().clone(); ml_reference_value96 }).1) && !(({ let ml_reference_value97 = proposed.borrow().clone(); ml_reference_value97 }).3)) && !(({ let ml_reference_value98 = proposed.borrow().clone(); ml_reference_value98 }).9)) && !(({ let ml_reference_value99 = proposed.borrow().clone(); ml_reference_value99 }).11)) && !(({ let ml_reference_value100 = proposed.borrow().clone(); ml_reference_value100 }).5)) && !(({ let ml_reference_value101 = proposed.borrow().clone(); ml_reference_value101 }).7)) {
                            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![107, 101, 101, 112]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(current.clone());
                                {
                                    let source_ml_source1_s448 = if !(({ let ml_reference_value102 = current.borrow().clone(); ml_reference_value102 }).0) {
                                        if (({ let ml_reference_value103 = proposed.borrow().clone(); ml_reference_value103 }).1 && ({ let ml_reference_value104 = proposed.borrow().clone(); ml_reference_value104 }).0) {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                        } else {
                                            if (({ let ml_reference_value105 = proposed.borrow().clone(); ml_reference_value105 }).3 && ((({ let ml_reference_value106 = proposed.borrow().clone(); ml_reference_value106 }).2) != (({ let ml_reference_value107 = current.borrow().clone(); ml_reference_value107 }).2))) {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if (({ let ml_reference_value108 = proposed.borrow().clone(); ml_reference_value108 }).5 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value109 = proposed.borrow().clone(); ml_reference_value109 }).4, ({ let ml_reference_value110 = current.borrow().clone(); ml_reference_value110 }).4))) {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                        } else {
                                                            if (({ let ml_reference_value111 = proposed.borrow().clone(); ml_reference_value111 }).7 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value112 = proposed.borrow().clone(); ml_reference_value112 }).6, ({ let ml_reference_value113 = current.borrow().clone(); ml_reference_value113 }).6))) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Next
                                                            }
                                                        }
                                                    } else {
                                                        if !(({ let ml_reference_value114 = current.borrow().clone(); ml_reference_value114 }).10) {
                                                            if (({ let ml_reference_value115 = proposed.borrow().clone(); ml_reference_value115 }).11 && ({ let ml_reference_value116 = proposed.borrow().clone(); ml_reference_value116 }).10) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                if ({ let ml_reference_value117 = proposed.borrow().clone(); ml_reference_value117 }).9 {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(if self::test_java_script_semantic_same_value(({ let ml_reference_value118 = proposed.borrow().clone(); ml_reference_value118 }).8, ({ let ml_reference_value119 = current.borrow().clone(); ml_reference_value119 }).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }, self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                                } else {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Next
                                    };
                                    match source_ml_source1_s448.clone() {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Next => {
                                            {
                                                let base_2 = if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::create_java_script_property_descriptor(kind.clone(), ({ let ml_reference_value120 = current.borrow().clone(); ml_reference_value120 }).0, ({ let ml_reference_value121 = current.borrow().clone(); ml_reference_value121 }).2)
                                                } else {
                                                    current.clone()
                                                };
                                                {
                                                    let descriptor_2 = self::apply_java_script_property_descriptor_fields(base_2.clone(), proposed.clone());
                                                    {
                                                        let conversion = ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original));
                                                        self::create_java_script_property_transition(if conversion {
                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![117, 112, 100, 97, 116, 101])
                                                        }, self::ml_string::Text::from_units(vec![]), descriptor_2.clone(), self::read_java_script_property_descriptor_writes(if conversion {
                                                            descriptor_2.clone()
                                                        } else {
                                                            proposed.clone()
                                                        }))
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(source_ml_temporary_result8) => {
                                            source_ml_temporary_result8.clone()
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

pub fn read_java_script_literal_property_index(key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> f64 {
    if ((({ let ml_reference_array122 = key.clone(); let ml_reference_projection122 = ml_reference_array122.borrow().len() as f64; ml_reference_projection122 }) == (0f64)) || (({ let ml_reference_array123 = key.clone(); let ml_reference_projection123 = ml_reference_array123.borrow().len() as f64; ml_reference_projection123 }) > (10f64))) {
        (-1f64)
    } else {
        if ((({ let ml_reference_array124 = key.clone(); let ml_reference_projection124 = ml_reference_array124.borrow().len() as f64; ml_reference_projection124 }) > (1f64)) && {
            let source_ml_source1_comparison_operand451 = {
                let source_ml_source1_array_read_value449 = key.clone();
                {
                    let source_ml_source1_array_read_index450 = 0f64;
                    { let ml_reference_array125 = source_ml_source1_array_read_value449.clone(); let ml_reference_position125 = source_ml_source1_array_read_index450; let ml_reference_projection125 = self::ml_array::optional_at(&*ml_reference_array125.borrow(), self::ml_array::number_index(ml_reference_position125)); ml_reference_projection125 }
                }
            };
            {
                let source_ml_source1_comparison_operand452 = 48f64;
                match (source_ml_source1_comparison_operand451.clone(), source_ml_source1_comparison_operand452) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) {
            (-1f64)
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s453 = self::source_ml_temporary_read_java_script_literal_property_index_loop9(key.clone(), value, at);
                        match source_ml_source1_s453.clone() {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Done(value_2) => {
                                if (value_2 <= (4294967294f64)) {
                                    value_2
                                } else {
                                    (-1f64)
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Return(source_ml_temporary_result9) => {
                                source_ml_temporary_result9
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_literal_property_form(kind: self::ml_string::Text, index: f64) -> self::ml_string::Text {
    if ((((kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) || (kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && (index >= (0f64))) {
        self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
    } else {
        self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])
    }
}

pub fn read_java_script_string_index_number_form(kind: self::ml_string::Text, element: self::ml_string::Text) -> self::ml_string::Text {
    if (kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else {
        if ((kind == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
        } else {
            if ((kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) || ((kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && (element == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))) {
                self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
            } else {
                if ((kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && (((element == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || (element == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || (element == (self::ml_string::Text::from_units(vec![110, 97, 116]))))) {
                    self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
                } else {
                    self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
                }
            }
        }
    }
}

pub fn find_java_script_heap_directory_object(objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = { let ml_reference_array126 = objects.clone(); let ml_reference_projection126 = ml_reference_array126.borrow().len() as f64; ml_reference_projection126 };
            {
                let source_ml_source1_s454 = self::source_ml_temporary_find_java_script_heap_directory_object_loop10(objects.clone(), identity.clone(), low, high);
                match source_ml_source1_s454.clone() {
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Done => {
                        (-1f64)
                    }
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Return(source_ml_temporary_result10) => {
                        source_ml_temporary_result10
                    }
                }
            }
        }
    }
}

pub fn test_java_script_heap_directory_symbol(symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, identity: self::ml::Big) -> bool {
    {
        let low = 0f64;
        {
            let high = { let ml_reference_array127 = symbols.clone(); let ml_reference_projection127 = ml_reference_array127.borrow().len() as f64; ml_reference_projection127 };
            {
                let source_ml_source1_s455 = self::source_ml_temporary_test_java_script_heap_directory_symbol_loop11(symbols.clone(), identity.clone(), low, high);
                match source_ml_source1_s455.clone() {
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Return(source_ml_temporary_result11) => {
                        source_ml_temporary_result11
                    }
                }
            }
        }
    }
}

pub fn read_java_script_heap_directory_value_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    {
        let error = self::read_java_script_semantic_value_error(value.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptSemanticValue::Null => {
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f456) => {
                    {
                        let source_ml_source1_f478 = source_ml_source1_f456;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f457) => {
                    {
                        let source_ml_source1_f486 = source_ml_source1_f457;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f458) => {
                    {
                        let source_ml_source1_f494 = source_ml_source1_f458.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f459) => {
                    {
                        let source_ml_source1_f502 = source_ml_source1_f459.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f460, source_ml_source1_f461) => {
                    {
                        let source_ml_source1_f510 = source_ml_source1_f460.clone();
                        {
                            let source_ml_source1_f511 = source_ml_source1_f461.clone();
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    if self::test_java_script_heap_directory_symbol(match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value513, _, _, _, _) => {
                            source_ml_source1_data_property_value513.clone()
                        }
                    }, identity.clone()) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f462, source_ml_source1_f463) => {
                    {
                        let identity_2 = source_ml_source1_f462.clone();
                        {
                            let callable = source_ml_source1_f463;
                            {
                                let at = self::find_java_script_heap_directory_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value521, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value521.clone()
                                    }
                                }, identity_2.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = {
                                            let source_ml_source1_array_read_value523 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value522, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value522.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_array_read_index524 = at;
                                                { let ml_reference_array128 = source_ml_source1_array_read_value523.clone(); let ml_reference_position128 = source_ml_source1_array_read_index524; let ml_reference_projection128 = self::ml_array::optional_at(&*ml_reference_array128.borrow(), self::ml_array::number_index(ml_reference_position128)); ml_reference_projection128 }
                                            }
                                        };
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source1_data_property_value538, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value538
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source1_data_property_value539, _, _, _, _) => {
                                                source_ml_source1_data_property_value539
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source1_data_property_value540, _) => {
                                                source_ml_source1_data_property_value540
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source1_data_property_value541, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value541
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source1_data_property_value542, _, _) => {
                                                source_ml_source1_data_property_value542
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source1_data_property_value543, _, _) => {
                                                source_ml_source1_data_property_value543
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source1_data_property_value544, _, _) => {
                                                source_ml_source1_data_property_value544
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source1_data_property_value545, _, _) => {
                                                source_ml_source1_data_property_value545
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source1_data_property_value546, _) => {
                                                source_ml_source1_data_property_value546
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source1_data_property_value547, _, _, _) => {
                                                source_ml_source1_data_property_value547
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source1_data_property_value548, _, _, _) => {
                                                source_ml_source1_data_property_value548
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source1_data_property_value549, _, _, _) => {
                                                source_ml_source1_data_property_value549
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source1_data_property_value550, _, _, _) => {
                                                source_ml_source1_data_property_value550
                                            }
                                        }) == callable)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121])
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

pub fn read_java_script_heap_directory_key_error(heap: self::JavaScriptOrdinaryHeap, key: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    if ((match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f551) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f552) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f553) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f554) => {
            false
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f555, source_ml_source1_f556) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f557) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f558, source_ml_source1_f559) => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f560) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f561) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f562) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f563) => {
            true
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f564, source_ml_source1_f565) => {
            false
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f566) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f567, source_ml_source1_f568) => {
            true
        }
    }) && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f569) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f570) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f571) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f572) => {
            true
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f573, source_ml_source1_f574) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f575) => {
            false
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f576, source_ml_source1_f577) => {
            true
        }
    }) {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 101, 120, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 104, 111, 117, 116, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 99, 111, 101, 114, 99, 105, 111, 110])
    } else {
        self::read_java_script_heap_directory_value_error(heap.clone(), key.clone())
    }
}

pub fn find_java_script_heap_directory_property(properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, key: self::JavaScriptSemanticValue) -> f64 {
    self::find_java_script_property_descriptor_entry(properties.clone(), key.clone())
}

pub fn read_java_script_array_length_key() -> self::JavaScriptSemanticValue {
    self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64])))
}

pub fn test_java_script_array_length_key(key: self::JavaScriptSemanticValue) -> bool {
    self::test_java_script_semantic_same_value(key.clone(), self::read_java_script_array_length_key())
}

pub fn read_java_script_array_index(key: self::JavaScriptSemanticValue) -> f64 {
    {
        let length = self::read_java_script_semantic_text_length(key.clone());
        if ((length < (0f64)) || (length > (10f64))) {
            (-1f64)
        } else {
            self::read_java_script_literal_property_index(self::construct_java_script_semantic_text_units(key.clone(), 10f64))
        }
    }
}

pub fn find_java_script_array_property(properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, key: self::JavaScriptSemanticValue) -> f64 {
    self::find_java_script_heap_directory_property(properties.clone(), key.clone())
}

pub fn read_java_script_array_length(object: self::JavaScriptOrdinaryObject) -> f64 {
    {
        let property = {
            let source_ml_source1_array_read_value604 = match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value578, _, _, _, _, _) => {
                    source_ml_source1_data_property_value578.clone()
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value579, _, _, _) => {
                    source_ml_source1_data_property_value579.clone()
                }
                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value580) => {
                    source_ml_source1_data_property_value580.clone()
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value581, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value581.clone()
                }
                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value582, _) => {
                    source_ml_source1_data_property_value582.clone()
                }
                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value583, _) => {
                    source_ml_source1_data_property_value583.clone()
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value584, _) => {
                    source_ml_source1_data_property_value584.clone()
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value585, _) => {
                    source_ml_source1_data_property_value585.clone()
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value586) => {
                    source_ml_source1_data_property_value586.clone()
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value587, _, _) => {
                    source_ml_source1_data_property_value587.clone()
                }
                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value588, _, _) => {
                    source_ml_source1_data_property_value588.clone()
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value589, _, _) => {
                    source_ml_source1_data_property_value589.clone()
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value590, _, _) => {
                    source_ml_source1_data_property_value590.clone()
                }
            };
            {
                let source_ml_source1_array_read_index605 = self::find_java_script_array_property(match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value591, _, _, _, _, _) => {
                        source_ml_source1_data_property_value591.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value592, _, _, _) => {
                        source_ml_source1_data_property_value592.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value593) => {
                        source_ml_source1_data_property_value593.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value594, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value594.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value595, _) => {
                        source_ml_source1_data_property_value595.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value596, _) => {
                        source_ml_source1_data_property_value596.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value597, _) => {
                        source_ml_source1_data_property_value597.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value598, _) => {
                        source_ml_source1_data_property_value598.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value599) => {
                        source_ml_source1_data_property_value599.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value600, _, _) => {
                        source_ml_source1_data_property_value600.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value601, _, _) => {
                        source_ml_source1_data_property_value601.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value602, _, _) => {
                        source_ml_source1_data_property_value602.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value603, _, _) => {
                        source_ml_source1_data_property_value603.clone()
                    }
                }, self::read_java_script_array_length_key());
                { let ml_reference_array129 = source_ml_source1_array_read_value604.clone(); let ml_reference_position129 = source_ml_source1_array_read_index605; let ml_reference_projection129 = self::ml_array::optional_at(&*ml_reference_array129.borrow(), self::ml_array::number_index(ml_reference_position129)); ml_reference_projection129 }
            }
        };
        if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            (-1f64)
        } else {
            self::read_java_script_array_length_value(({ let ml_reference_value130 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value606) => {
                    source_ml_source1_data_property_value606.clone()
                }
            }).borrow().clone(); ml_reference_value130 }).8)
        }
    }
}

pub fn read_java_script_array_length_value(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Null => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f607) => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Number(value_value) => {
            value_value
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f608) => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f609) => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f610, source_ml_source1_f611) => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f612) => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f613, source_ml_source1_f614) => {
            (-1f64)
        }
    }
}

pub fn test_java_script_array_length(length: f64) -> bool {
    (((self::ml_math::is_integer(length) && (length >= (0f64))) && (length <= (4294967295f64))) && !({
        let source_ml_source1_same_value_operand615 = length;
        {
            let source_ml_source1_same_value_operand616 = (-0f64);
            (((source_ml_source1_same_value_operand615 == source_ml_source1_same_value_operand616) && ((source_ml_source1_same_value_operand615 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand615)) == ((1f64 / source_ml_source1_same_value_operand616))))) || ((source_ml_source1_same_value_operand615 != source_ml_source1_same_value_operand615) && (source_ml_source1_same_value_operand616 != source_ml_source1_same_value_operand616)))
        }
    }))
}

pub fn read_java_script_array_object_error(object: self::JavaScriptOrdinaryObject) -> self::ml_string::Text {
    if match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source1_data_property_value617, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value617
        }
        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source1_data_property_value618, _, _, _, _) => {
            source_ml_source1_data_property_value618
        }
        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source1_data_property_value619, _) => {
            source_ml_source1_data_property_value619
        }
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source1_data_property_value620, _, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value620
        }
        self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source1_data_property_value621, _, _) => {
            source_ml_source1_data_property_value621
        }
        self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source1_data_property_value622, _, _) => {
            source_ml_source1_data_property_value622
        }
        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source1_data_property_value623, _, _) => {
            source_ml_source1_data_property_value623
        }
        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source1_data_property_value624, _, _) => {
            source_ml_source1_data_property_value624
        }
        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source1_data_property_value625, _) => {
            source_ml_source1_data_property_value625
        }
        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source1_data_property_value626, _, _, _) => {
            source_ml_source1_data_property_value626
        }
        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source1_data_property_value627, _, _, _) => {
            source_ml_source1_data_property_value627
        }
        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source1_data_property_value628, _, _, _) => {
            source_ml_source1_data_property_value628
        }
        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source1_data_property_value629, _, _, _) => {
            source_ml_source1_data_property_value629
        }
    } {
        self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 101, 120, 111, 116, 105, 99, 32, 111, 98, 106, 101, 99, 116, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])
    } else {
        {
            let property = {
                let source_ml_source1_array_read_value656 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value630, _, _, _, _, _) => {
                        source_ml_source1_data_property_value630.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value631, _, _, _) => {
                        source_ml_source1_data_property_value631.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value632) => {
                        source_ml_source1_data_property_value632.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value633, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value633.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value634, _) => {
                        source_ml_source1_data_property_value634.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value635, _) => {
                        source_ml_source1_data_property_value635.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value636, _) => {
                        source_ml_source1_data_property_value636.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value637, _) => {
                        source_ml_source1_data_property_value637.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value638) => {
                        source_ml_source1_data_property_value638.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value639, _, _) => {
                        source_ml_source1_data_property_value639.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value640, _, _) => {
                        source_ml_source1_data_property_value640.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value641, _, _) => {
                        source_ml_source1_data_property_value641.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value642, _, _) => {
                        source_ml_source1_data_property_value642.clone()
                    }
                };
                {
                    let source_ml_source1_array_read_index657 = self::find_java_script_array_property(match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value643, _, _, _, _, _) => {
                            source_ml_source1_data_property_value643.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value644, _, _, _) => {
                            source_ml_source1_data_property_value644.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value645) => {
                            source_ml_source1_data_property_value645.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value646, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value646.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value647, _) => {
                            source_ml_source1_data_property_value647.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value648, _) => {
                            source_ml_source1_data_property_value648.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value649, _) => {
                            source_ml_source1_data_property_value649.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value650, _) => {
                            source_ml_source1_data_property_value650.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value651) => {
                            source_ml_source1_data_property_value651.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value652, _, _) => {
                            source_ml_source1_data_property_value652.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value653, _, _) => {
                            source_ml_source1_data_property_value653.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value654, _, _) => {
                            source_ml_source1_data_property_value654.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value655, _, _) => {
                            source_ml_source1_data_property_value655.clone()
                        }
                    }, self::read_java_script_array_length_key());
                    { let ml_reference_array131 = source_ml_source1_array_read_value656.clone(); let ml_reference_position131 = source_ml_source1_array_read_index657; let ml_reference_projection131 = self::ml_array::optional_at(&*ml_reference_array131.borrow(), self::ml_array::number_index(ml_reference_position131)); ml_reference_projection131 }
                }
            };
            {
                let invalid = self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 101, 110, 117, 109, 101, 114, 97, 98, 108, 101, 32, 110, 111, 110, 99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101, 32, 105, 110, 116, 101, 103, 114, 97, 108, 32, 108, 101, 110, 103, 116, 104, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121]);
                if (((((match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(({ let ml_reference_value132 = (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value658) => {
                        source_ml_source1_data_property_value658.clone()
                    }
                }).borrow().clone(); ml_reference_value132 }).9)) || ({ let ml_reference_value133 = (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value659) => {
                        source_ml_source1_data_property_value659.clone()
                    }
                }).borrow().clone(); ml_reference_value133 }).5) || ({ let ml_reference_value134 = (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value660) => {
                        source_ml_source1_data_property_value660.clone()
                    }
                }).borrow().clone(); ml_reference_value134 }).7) || ({ let ml_reference_value135 = (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value661) => {
                        source_ml_source1_data_property_value661.clone()
                    }
                }).borrow().clone(); ml_reference_value135 }).2) || ({ let ml_reference_value136 = (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value662) => {
                        source_ml_source1_data_property_value662.clone()
                    }
                }).borrow().clone(); ml_reference_value136 }).0) {
                    invalid.clone()
                } else {
                    {
                        let length = self::read_java_script_array_length_value(({ let ml_reference_value137 = (match property.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value663) => {
                                source_ml_source1_data_property_value663.clone()
                            }
                        }).borrow().clone(); ml_reference_value137 }).8);
                        if !(self::test_java_script_array_length(length)) {
                            invalid.clone()
                        } else {
                            {
                                let source_ml_temporary_values12 = match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value664, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value664.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value665, _, _, _) => {
                                        source_ml_source1_data_property_value665.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value666) => {
                                        source_ml_source1_data_property_value666.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value667, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value667.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value668, _) => {
                                        source_ml_source1_data_property_value668.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value669, _) => {
                                        source_ml_source1_data_property_value669.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value670, _) => {
                                        source_ml_source1_data_property_value670.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value671, _) => {
                                        source_ml_source1_data_property_value671.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value672) => {
                                        source_ml_source1_data_property_value672.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value673, _, _) => {
                                        source_ml_source1_data_property_value673.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value674, _, _) => {
                                        source_ml_source1_data_property_value674.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value675, _, _) => {
                                        source_ml_source1_data_property_value675.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value676, _, _) => {
                                        source_ml_source1_data_property_value676.clone()
                                    }
                                };
                                {
                                    let source_ml_temporary_index12 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s677 = self::source_ml_temporary_read_java_script_array_object_error_loop13(length, source_ml_temporary_values12.clone(), source_ml_temporary_index12.clone());
                                        match source_ml_source1_s677.clone() {
                                            self::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Done => {
                                                self::ml_string::Text::from_units(vec![])
                                            }
                                            self::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Return(source_ml_temporary_result13) => {
                                                source_ml_temporary_result13.clone()
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

pub fn construct_java_script_array_object(identity: self::ml::Big, prototype: self::JavaScriptSemanticValue, length: f64) -> self::JavaScriptOrdinaryObject {
    {
        let descriptor = self::create_java_script_property_descriptor(self::ml_string::Text::from_units(vec![100, 97, 116, 97]), true, false);
        {
            let source_ml_temporary_construct_java_script_array_object_written14 = { let ml_field_owner138 = descriptor.clone(); let ml_field_value138 = false; ml_field_owner138.borrow_mut().0 = ml_field_value138; () };
            {
                let source_ml_temporary_construct_java_script_array_object_written15 = { let ml_field_owner139 = descriptor.clone(); let ml_field_value139 = true; ml_field_owner139.borrow_mut().10 = ml_field_value139; () };
                {
                    let source_ml_temporary_construct_java_script_array_object_written16 = { let ml_field_owner140 = descriptor.clone(); let ml_field_value140 = self::JavaScriptSemanticValue::Number(length); ml_field_owner140.borrow_mut().8 = ml_field_value140; () };
                    self::JavaScriptOrdinaryObject::ArrayObject(identity.clone(), Box::new(prototype.clone()), true, false, std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(self::read_java_script_array_length_key()), descriptor.clone())])))
                }
            }
        }
    }
}

pub fn plan_java_script_array_ordinary_definition(object: self::JavaScriptOrdinaryObject, key: self::JavaScriptSemanticValue, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::JavaScriptArrayPropertyPlan {
    {
        let at = self::find_java_script_array_property(match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value678, _, _, _, _, _) => {
                source_ml_source1_data_property_value678.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value679, _, _, _) => {
                source_ml_source1_data_property_value679.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value680) => {
                source_ml_source1_data_property_value680.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value681, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value681.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value682, _) => {
                source_ml_source1_data_property_value682.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value683, _) => {
                source_ml_source1_data_property_value683.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value684, _) => {
                source_ml_source1_data_property_value684.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value685, _) => {
                source_ml_source1_data_property_value685.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value686) => {
                source_ml_source1_data_property_value686.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value687, _, _) => {
                source_ml_source1_data_property_value687.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value688, _, _) => {
                source_ml_source1_data_property_value688.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value689, _, _) => {
                source_ml_source1_data_property_value689.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value690, _, _) => {
                source_ml_source1_data_property_value690.clone()
            }
        }, key.clone());
        {
            let property = {
                let source_ml_source1_array_read_value704 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value691, _, _, _, _, _) => {
                        source_ml_source1_data_property_value691.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value692, _, _, _) => {
                        source_ml_source1_data_property_value692.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value693) => {
                        source_ml_source1_data_property_value693.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value694, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value694.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value695, _) => {
                        source_ml_source1_data_property_value695.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value696, _) => {
                        source_ml_source1_data_property_value696.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value697, _) => {
                        source_ml_source1_data_property_value697.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value698, _) => {
                        source_ml_source1_data_property_value698.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value699) => {
                        source_ml_source1_data_property_value699.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value700, _, _) => {
                        source_ml_source1_data_property_value700.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value701, _, _) => {
                        source_ml_source1_data_property_value701.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value702, _, _) => {
                        source_ml_source1_data_property_value702.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value703, _, _) => {
                        source_ml_source1_data_property_value703.clone()
                    }
                };
                {
                    let source_ml_source1_array_read_index705 = at;
                    { let ml_reference_array141 = source_ml_source1_array_read_value704.clone(); let ml_reference_position141 = source_ml_source1_array_read_index705; let ml_reference_projection141 = self::ml_array::optional_at(&*ml_reference_array141.borrow(), self::ml_array::number_index(ml_reference_position141)); ml_reference_projection141 }
                }
            };
            {
                let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::create_java_script_property_descriptor(self::ml_string::Text::from_units(vec![100, 97, 116, 97]), false, false)
                } else {
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value706) => {
                            source_ml_source1_data_property_value706.clone()
                        }
                    }
                };
                {
                    let plan = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, source_ml_source1_data_property_value707, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value707
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, source_ml_source1_data_property_value708, _, _, _, _, _) => {
                            source_ml_source1_data_property_value708
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, source_ml_source1_data_property_value709, _, _) => {
                            source_ml_source1_data_property_value709
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source1_data_property_value710, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value710
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, source_ml_source1_data_property_value711, _, _, _) => {
                            source_ml_source1_data_property_value711
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, source_ml_source1_data_property_value712, _, _, _) => {
                            source_ml_source1_data_property_value712
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, source_ml_source1_data_property_value713, _, _, _) => {
                            source_ml_source1_data_property_value713
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, source_ml_source1_data_property_value714, _, _, _) => {
                            source_ml_source1_data_property_value714
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value715, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value715
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, source_ml_source1_data_property_value716, _, _, _, _) => {
                            source_ml_source1_data_property_value716
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, source_ml_source1_data_property_value717, _, _, _, _) => {
                            source_ml_source1_data_property_value717
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, source_ml_source1_data_property_value718, _, _, _, _) => {
                            source_ml_source1_data_property_value718
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, source_ml_source1_data_property_value719, _, _, _, _) => {
                            source_ml_source1_data_property_value719
                        }
                    }, current.clone(), proposed.clone());
                    if ((({ let ml_reference_value142 = plan.borrow().clone(); ml_reference_value142 }).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, ({ let ml_reference_value143 = plan.borrow().clone(); ml_reference_value143 }).2, self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    } else {
                        if ((({ let ml_reference_value144 = plan.borrow().clone(); ml_reference_value144 }).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), true, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            if ((at < (0f64)) && (({ let ml_reference_array145 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value733, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value733.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value734, _, _, _) => {
                                    source_ml_source1_data_property_value734.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value735) => {
                                    source_ml_source1_data_property_value735.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value736, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value736.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value737, _) => {
                                    source_ml_source1_data_property_value737.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value738, _) => {
                                    source_ml_source1_data_property_value738.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value739, _) => {
                                    source_ml_source1_data_property_value739.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value740, _) => {
                                    source_ml_source1_data_property_value740.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value741) => {
                                    source_ml_source1_data_property_value741.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value742, _, _) => {
                                    source_ml_source1_data_property_value742.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value743, _, _) => {
                                    source_ml_source1_data_property_value743.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value744, _, _) => {
                                    source_ml_source1_data_property_value744.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value745, _, _) => {
                                    source_ml_source1_data_property_value745.clone()
                                }
                            }; let ml_reference_projection145 = ml_reference_array145.borrow().len() as f64; ml_reference_projection145 }) >= (4096f64))) {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 115, 111, 117, 114, 99, 101]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                            } else {
                                {
                                    let replacement = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), ({ let ml_reference_value146 = plan.borrow().clone(); ml_reference_value146 }).0);
                                    {
                                        let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                        {
                                            let index = 0f64;
                                            {
                                                let source_ml_temporary_values17 = match object.clone() {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value746, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value746.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value747, _, _, _) => {
                                                        source_ml_source1_data_property_value747.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value748) => {
                                                        source_ml_source1_data_property_value748.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value749, _, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value749.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value750, _) => {
                                                        source_ml_source1_data_property_value750.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value751, _) => {
                                                        source_ml_source1_data_property_value751.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value752, _) => {
                                                        source_ml_source1_data_property_value752.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value753, _) => {
                                                        source_ml_source1_data_property_value753.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value754) => {
                                                        source_ml_source1_data_property_value754.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value755, _, _) => {
                                                        source_ml_source1_data_property_value755.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value756, _, _) => {
                                                        source_ml_source1_data_property_value756.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value757, _, _) => {
                                                        source_ml_source1_data_property_value757.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value758, _, _) => {
                                                        source_ml_source1_data_property_value758.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index17 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s759 = self::source_ml_temporary_plan_java_script_array_ordinary_definition_loop18(at, replacement.clone(), properties.clone(), index, source_ml_temporary_values17.clone(), source_ml_temporary_index17.clone());
                                                        match source_ml_source1_s759.clone() {
                                                            self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Result::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Done => {
                                                                {
                                                                    let source_ml_source1_s760 = if (at < (0f64)) {
                                                                        {
                                                                            let source_ml_temporary_plan_java_script_array_ordinary_definition_ignored20 = { let ml_method_owner147 = properties.clone(); let ml_method_argument147_0 = replacement.clone(); let ml_inserted = vec![ml_method_argument147_0]; let mut ml_method_values147 = ml_method_owner147.borrow_mut(); let ml_length = ml_method_values147.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values147.extend(ml_inserted); ml_length as f64 };
                                                                            self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19Next
                                                                        }
                                                                    } else {
                                                                        self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19Next
                                                                    };
                                                                    match source_ml_source1_s760.clone() {
                                                                        self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19Next => {
                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(match object.clone() {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field761, source_ml_source1_nominal_field762, source_ml_source1_nominal_field763, source_ml_source1_nominal_field764, source_ml_source1_nominal_field765, source_ml_source1_nominal_field766, source_ml_source1_nominal_field767, source_ml_source1_nominal_field768, source_ml_source1_nominal_field769, source_ml_source1_nominal_field770) => {
                                                                                    let source_ml_source1_nominal_field762 = *source_ml_source1_nominal_field762;
                                                                                    let source_ml_source1_nominal_field769 = *source_ml_source1_nominal_field769;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override771 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field761.clone(), Box::new(source_ml_source1_nominal_field762.clone()), source_ml_source1_nominal_field763, source_ml_source1_nominal_field764, source_ml_source1_nominal_override771.clone(), source_ml_source1_nominal_field766.clone(), source_ml_source1_nominal_field767, source_ml_source1_nominal_field768.clone(), Box::new(source_ml_source1_nominal_field769.clone()), source_ml_source1_nominal_field770.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field772, source_ml_source1_nominal_field773, source_ml_source1_nominal_field774, source_ml_source1_nominal_field775, source_ml_source1_nominal_field776, source_ml_source1_nominal_field777, source_ml_source1_nominal_field778, source_ml_source1_nominal_field779) => {
                                                                                    let source_ml_source1_nominal_field773 = *source_ml_source1_nominal_field773;
                                                                                    let source_ml_source1_nominal_field777 = *source_ml_source1_nominal_field777;
                                                                                    let source_ml_source1_nominal_field778 = *source_ml_source1_nominal_field778;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override780 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field772.clone(), Box::new(source_ml_source1_nominal_field773.clone()), source_ml_source1_nominal_field774, source_ml_source1_nominal_field775, source_ml_source1_nominal_override780.clone(), Box::new(source_ml_source1_nominal_field777.clone()), Box::new(source_ml_source1_nominal_field778.clone()), source_ml_source1_nominal_field779.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field781, source_ml_source1_nominal_field782, source_ml_source1_nominal_field783, source_ml_source1_nominal_field784, source_ml_source1_nominal_field785) => {
                                                                                    let source_ml_source1_nominal_field782 = *source_ml_source1_nominal_field782;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override786 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field781.clone(), Box::new(source_ml_source1_nominal_field782.clone()), source_ml_source1_nominal_field783, source_ml_source1_nominal_field784, source_ml_source1_nominal_override786.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field787, source_ml_source1_nominal_field788, source_ml_source1_nominal_field789, source_ml_source1_nominal_field790, source_ml_source1_nominal_field791, source_ml_source1_nominal_field792, source_ml_source1_nominal_field793, source_ml_source1_nominal_field794, source_ml_source1_nominal_field795, source_ml_source1_nominal_field796, source_ml_source1_nominal_field797) => {
                                                                                    let source_ml_source1_nominal_field788 = *source_ml_source1_nominal_field788;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override798 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field787.clone(), Box::new(source_ml_source1_nominal_field788.clone()), source_ml_source1_nominal_field789, source_ml_source1_nominal_field790, source_ml_source1_nominal_override798.clone(), source_ml_source1_nominal_field792.clone(), source_ml_source1_nominal_field793, source_ml_source1_nominal_field794.clone(), source_ml_source1_nominal_field795.clone(), source_ml_source1_nominal_field796.clone(), source_ml_source1_nominal_field797.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field799, source_ml_source1_nominal_field800, source_ml_source1_nominal_field801, source_ml_source1_nominal_field802, source_ml_source1_nominal_field803, source_ml_source1_nominal_field804) => {
                                                                                    let source_ml_source1_nominal_field800 = *source_ml_source1_nominal_field800;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override805 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field799.clone(), Box::new(source_ml_source1_nominal_field800.clone()), source_ml_source1_nominal_field801, source_ml_source1_nominal_field802, source_ml_source1_nominal_override805.clone(), source_ml_source1_nominal_field804.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field806, source_ml_source1_nominal_field807, source_ml_source1_nominal_field808, source_ml_source1_nominal_field809, source_ml_source1_nominal_field810, source_ml_source1_nominal_field811) => {
                                                                                    let source_ml_source1_nominal_field807 = *source_ml_source1_nominal_field807;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override812 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field806.clone(), Box::new(source_ml_source1_nominal_field807.clone()), source_ml_source1_nominal_field808, source_ml_source1_nominal_field809, source_ml_source1_nominal_override812.clone(), source_ml_source1_nominal_field811.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field813, source_ml_source1_nominal_field814, source_ml_source1_nominal_field815, source_ml_source1_nominal_field816, source_ml_source1_nominal_field817, source_ml_source1_nominal_field818) => {
                                                                                    let source_ml_source1_nominal_field814 = *source_ml_source1_nominal_field814;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override819 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field813.clone(), Box::new(source_ml_source1_nominal_field814.clone()), source_ml_source1_nominal_field815, source_ml_source1_nominal_field816, source_ml_source1_nominal_override819.clone(), source_ml_source1_nominal_field818.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field820, source_ml_source1_nominal_field821, source_ml_source1_nominal_field822, source_ml_source1_nominal_field823, source_ml_source1_nominal_field824, source_ml_source1_nominal_field825) => {
                                                                                    let source_ml_source1_nominal_field821 = *source_ml_source1_nominal_field821;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override826 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field820.clone(), Box::new(source_ml_source1_nominal_field821.clone()), source_ml_source1_nominal_field822, source_ml_source1_nominal_field823, source_ml_source1_nominal_override826.clone(), source_ml_source1_nominal_field825.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field827, source_ml_source1_nominal_field828, source_ml_source1_nominal_field829, source_ml_source1_nominal_field830, source_ml_source1_nominal_field831, source_ml_source1_nominal_field832, source_ml_source1_nominal_field833, source_ml_source1_nominal_field834, source_ml_source1_nominal_field835) => {
                                                                                    let source_ml_source1_nominal_field828 = *source_ml_source1_nominal_field828;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override836 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field827.clone(), Box::new(source_ml_source1_nominal_field828.clone()), source_ml_source1_nominal_field829, source_ml_source1_nominal_field830.clone(), source_ml_source1_nominal_field831.clone(), source_ml_source1_nominal_field832.clone(), source_ml_source1_nominal_field833.clone(), source_ml_source1_nominal_field834, source_ml_source1_nominal_override836.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field837, source_ml_source1_nominal_field838, source_ml_source1_nominal_field839, source_ml_source1_nominal_field840, source_ml_source1_nominal_field841, source_ml_source1_nominal_field842, source_ml_source1_nominal_field843) => {
                                                                                    let source_ml_source1_nominal_field838 = *source_ml_source1_nominal_field838;
                                                                                    let source_ml_source1_nominal_field842 = *source_ml_source1_nominal_field842;
                                                                                    let source_ml_source1_nominal_field843 = *source_ml_source1_nominal_field843;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override844 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field837.clone(), Box::new(source_ml_source1_nominal_field838.clone()), source_ml_source1_nominal_field839, source_ml_source1_nominal_field840, source_ml_source1_nominal_override844.clone(), Box::new(source_ml_source1_nominal_field842.clone()), Box::new(source_ml_source1_nominal_field843.clone()))
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field845, source_ml_source1_nominal_field846, source_ml_source1_nominal_field847, source_ml_source1_nominal_field848, source_ml_source1_nominal_field849, source_ml_source1_nominal_field850, source_ml_source1_nominal_field851) => {
                                                                                    let source_ml_source1_nominal_field846 = *source_ml_source1_nominal_field846;
                                                                                    let source_ml_source1_nominal_field850 = *source_ml_source1_nominal_field850;
                                                                                    let source_ml_source1_nominal_field851 = *source_ml_source1_nominal_field851;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override852 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field845.clone(), Box::new(source_ml_source1_nominal_field846.clone()), source_ml_source1_nominal_field847, source_ml_source1_nominal_field848, source_ml_source1_nominal_override852.clone(), Box::new(source_ml_source1_nominal_field850.clone()), Box::new(source_ml_source1_nominal_field851.clone()))
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field853, source_ml_source1_nominal_field854, source_ml_source1_nominal_field855, source_ml_source1_nominal_field856, source_ml_source1_nominal_field857, source_ml_source1_nominal_field858, source_ml_source1_nominal_field859) => {
                                                                                    let source_ml_source1_nominal_field854 = *source_ml_source1_nominal_field854;
                                                                                    let source_ml_source1_nominal_field858 = *source_ml_source1_nominal_field858;
                                                                                    let source_ml_source1_nominal_field859 = *source_ml_source1_nominal_field859;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override860 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field853.clone(), Box::new(source_ml_source1_nominal_field854.clone()), source_ml_source1_nominal_field855, source_ml_source1_nominal_field856, source_ml_source1_nominal_override860.clone(), Box::new(source_ml_source1_nominal_field858.clone()), Box::new(source_ml_source1_nominal_field859.clone()))
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field861, source_ml_source1_nominal_field862, source_ml_source1_nominal_field863, source_ml_source1_nominal_field864, source_ml_source1_nominal_field865, source_ml_source1_nominal_field866, source_ml_source1_nominal_field867) => {
                                                                                    let source_ml_source1_nominal_field862 = *source_ml_source1_nominal_field862;
                                                                                    let source_ml_source1_nominal_field866 = *source_ml_source1_nominal_field866;
                                                                                    let source_ml_source1_nominal_field867 = *source_ml_source1_nominal_field867;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override868 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field861.clone(), Box::new(source_ml_source1_nominal_field862.clone()), source_ml_source1_nominal_field863, source_ml_source1_nominal_field864, source_ml_source1_nominal_override868.clone(), Box::new(source_ml_source1_nominal_field866.clone()), Box::new(source_ml_source1_nominal_field867.clone()))
                                                                                    }
                                                                                }
                                                                            }), true, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), ({ let ml_reference_value148 = plan.borrow().clone(); ml_reference_value148 }).3)
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
    }
}

pub fn plan_java_script_array_length_definition(object: self::JavaScriptOrdinaryObject, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, unsigned_length: f64, number_length: f64) -> self::JavaScriptArrayPropertyPlan {
    if !(self::test_java_script_array_length(unsigned_length)) {
        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 105, 103, 110, 101, 100, 51, 50, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
    } else {
        if (unsigned_length != number_length) {
            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
        } else {
            {
                let key = self::read_java_script_array_length_key();
                {
                    let at = self::find_java_script_array_property(match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value869, _, _, _, _, _) => {
                            source_ml_source1_data_property_value869.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value870, _, _, _) => {
                            source_ml_source1_data_property_value870.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value871) => {
                            source_ml_source1_data_property_value871.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value872, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value872.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value873, _) => {
                            source_ml_source1_data_property_value873.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value874, _) => {
                            source_ml_source1_data_property_value874.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value875, _) => {
                            source_ml_source1_data_property_value875.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value876, _) => {
                            source_ml_source1_data_property_value876.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value877) => {
                            source_ml_source1_data_property_value877.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value878, _, _) => {
                            source_ml_source1_data_property_value878.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value879, _, _) => {
                            source_ml_source1_data_property_value879.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value880, _, _) => {
                            source_ml_source1_data_property_value880.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value881, _, _) => {
                            source_ml_source1_data_property_value881.clone()
                        }
                    }, key.clone());
                    {
                        let old = {
                            let source_ml_source1_array_read_value895 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value882, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value882.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value883, _, _, _) => {
                                    source_ml_source1_data_property_value883.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value884) => {
                                    source_ml_source1_data_property_value884.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value885, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value885.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value886, _) => {
                                    source_ml_source1_data_property_value886.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value887, _) => {
                                    source_ml_source1_data_property_value887.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value888, _) => {
                                    source_ml_source1_data_property_value888.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value889, _) => {
                                    source_ml_source1_data_property_value889.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value890) => {
                                    source_ml_source1_data_property_value890.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value891, _, _) => {
                                    source_ml_source1_data_property_value891.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value892, _, _) => {
                                    source_ml_source1_data_property_value892.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value893, _, _) => {
                                    source_ml_source1_data_property_value893.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value894, _, _) => {
                                    source_ml_source1_data_property_value894.clone()
                                }
                            };
                            {
                                let source_ml_source1_array_read_index896 = at;
                                { let ml_reference_array149 = source_ml_source1_array_read_value895.clone(); let ml_reference_position149 = source_ml_source1_array_read_index896; let ml_reference_projection149 = self::ml_array::optional_at(&*ml_reference_array149.borrow(), self::ml_array::number_index(ml_reference_position149)); ml_reference_projection149 }
                            }
                        };
                        if match (old.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 112, 114, 101, 115, 101, 110, 116]), self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            {
                                let length = self::read_java_script_array_length(object.clone());
                                {
                                    let descriptor = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_spread897 = { let ml_reference_value150 = proposed.borrow().clone(); ml_reference_value150 };
                                        {
                                            let source_ml_source1_spread_field898 = (source_ml_source1_spread897.clone()).0;
                                            {
                                                let source_ml_source1_spread_field899 = (source_ml_source1_spread897.clone()).1;
                                                {
                                                    let source_ml_source1_spread_field900 = (source_ml_source1_spread897.clone()).2;
                                                    {
                                                        let source_ml_source1_spread_field901 = (source_ml_source1_spread897.clone()).3;
                                                        {
                                                            let source_ml_source1_spread_field902 = (source_ml_source1_spread897.clone()).4;
                                                            {
                                                                let source_ml_source1_spread_field903 = (source_ml_source1_spread897.clone()).5;
                                                                {
                                                                    let source_ml_source1_spread_field904 = (source_ml_source1_spread897.clone()).6;
                                                                    {
                                                                        let source_ml_source1_spread_field905 = (source_ml_source1_spread897.clone()).7;
                                                                        {
                                                                            let source_ml_source1_spread_field906 = (source_ml_source1_spread897.clone()).8;
                                                                            {
                                                                                let source_ml_source1_spread_field907 = (source_ml_source1_spread897.clone()).9;
                                                                                {
                                                                                    let source_ml_source1_spread_field908 = (source_ml_source1_spread897.clone()).10;
                                                                                    {
                                                                                        let source_ml_source1_spread_field909 = (source_ml_source1_spread897.clone()).11;
                                                                                        {
                                                                                            let source_ml_source1_field910 = true;
                                                                                            {
                                                                                                let source_ml_source1_field911 = self::JavaScriptSemanticValue::Number(unsigned_length);
                                                                                                (source_ml_source1_spread_field898, source_ml_source1_spread_field899, source_ml_source1_spread_field900, source_ml_source1_spread_field901, source_ml_source1_spread_field902.clone(), source_ml_source1_spread_field903, source_ml_source1_spread_field904.clone(), source_ml_source1_spread_field905, source_ml_source1_field911.clone(), source_ml_source1_field910, source_ml_source1_spread_field908, source_ml_source1_spread_field909)
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
                                    }));
                                    if (unsigned_length >= length) {
                                        self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), descriptor.clone())
                                    } else {
                                        if !(({ let ml_reference_value151 = (match old.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value912) => {
                                                source_ml_source1_data_property_value912.clone()
                                            }
                                        }).borrow().clone(); ml_reference_value151 }).10) {
                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                                        } else {
                                            {
                                                let new_writable = (!(({ let ml_reference_value152 = descriptor.borrow().clone(); ml_reference_value152 }).11) || ({ let ml_reference_value153 = descriptor.borrow().clone(); ml_reference_value153 }).10);
                                                {
                                                    let temporary = if new_writable {
                                                        descriptor.clone()
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_spread913 = { let ml_reference_value154 = descriptor.borrow().clone(); ml_reference_value154 };
                                                            {
                                                                let source_ml_source1_spread_field914 = (source_ml_source1_spread913.clone()).0;
                                                                {
                                                                    let source_ml_source1_spread_field915 = (source_ml_source1_spread913.clone()).1;
                                                                    {
                                                                        let source_ml_source1_spread_field916 = (source_ml_source1_spread913.clone()).2;
                                                                        {
                                                                            let source_ml_source1_spread_field917 = (source_ml_source1_spread913.clone()).3;
                                                                            {
                                                                                let source_ml_source1_spread_field918 = (source_ml_source1_spread913.clone()).4;
                                                                                {
                                                                                    let source_ml_source1_spread_field919 = (source_ml_source1_spread913.clone()).5;
                                                                                    {
                                                                                        let source_ml_source1_spread_field920 = (source_ml_source1_spread913.clone()).6;
                                                                                        {
                                                                                            let source_ml_source1_spread_field921 = (source_ml_source1_spread913.clone()).7;
                                                                                            {
                                                                                                let source_ml_source1_spread_field922 = (source_ml_source1_spread913.clone()).8;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field923 = (source_ml_source1_spread913.clone()).9;
                                                                                                    {
                                                                                                        let source_ml_source1_spread_field924 = (source_ml_source1_spread913.clone()).10;
                                                                                                        {
                                                                                                            let source_ml_source1_spread_field925 = (source_ml_source1_spread913.clone()).11;
                                                                                                            {
                                                                                                                let source_ml_source1_field926 = true;
                                                                                                                (source_ml_source1_spread_field914, source_ml_source1_spread_field915, source_ml_source1_spread_field916, source_ml_source1_spread_field917, source_ml_source1_spread_field918.clone(), source_ml_source1_spread_field919, source_ml_source1_spread_field920.clone(), source_ml_source1_spread_field921, source_ml_source1_spread_field922.clone(), source_ml_source1_spread_field923, source_ml_source1_field926, source_ml_source1_spread_field925)
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
                                                        }))
                                                    };
                                                    {
                                                        let initial = self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), temporary.clone());
                                                        if !(match initial.clone() {
                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source1_data_property_value927, _, _, _) => {
                                                                source_ml_source1_data_property_value927
                                                            }
                                                        }) {
                                                            initial.clone()
                                                        } else {
                                                            {
                                                                let blocked = (-1f64);
                                                                {
                                                                    let source_ml_temporary_values22 = match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value928, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value928.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value929, _, _, _) => {
                                                                            source_ml_source1_data_property_value929.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value930) => {
                                                                            source_ml_source1_data_property_value930.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value931, _, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value931.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value932, _) => {
                                                                            source_ml_source1_data_property_value932.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value933, _) => {
                                                                            source_ml_source1_data_property_value933.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value934, _) => {
                                                                            source_ml_source1_data_property_value934.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value935, _) => {
                                                                            source_ml_source1_data_property_value935.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value936) => {
                                                                            source_ml_source1_data_property_value936.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value937, _, _) => {
                                                                            source_ml_source1_data_property_value937.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value938, _, _) => {
                                                                            source_ml_source1_data_property_value938.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value939, _, _) => {
                                                                            source_ml_source1_data_property_value939.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value940, _, _) => {
                                                                            source_ml_source1_data_property_value940.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_temporary_index22 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let blocked_2 = self::source_ml_temporary_plan_java_script_array_length_definition_loop24(unsigned_length, blocked, source_ml_temporary_values22.clone(), source_ml_temporary_index22.clone());
                                                                            {
                                                                                let final_length = if (blocked_2 < (0f64)) {
                                                                                    unsigned_length
                                                                                } else {
                                                                                    (blocked_2 + 1f64)
                                                                                };
                                                                                {
                                                                                    let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                                    {
                                                                                        let source_ml_temporary_values23 = match match initial.clone() {
                                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value941, _, _, _, _) => {
                                                                                                let source_ml_source1_data_property_value941 = *source_ml_source1_data_property_value941;
                                                                                                source_ml_source1_data_property_value941.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value942, _, _, _, _, _) => {
                                                                                                source_ml_source1_data_property_value942.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value943, _, _, _) => {
                                                                                                source_ml_source1_data_property_value943.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value944) => {
                                                                                                source_ml_source1_data_property_value944.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value945, _, _, _, _, _, _) => {
                                                                                                source_ml_source1_data_property_value945.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value946, _) => {
                                                                                                source_ml_source1_data_property_value946.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value947, _) => {
                                                                                                source_ml_source1_data_property_value947.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value948, _) => {
                                                                                                source_ml_source1_data_property_value948.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value949, _) => {
                                                                                                source_ml_source1_data_property_value949.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value950) => {
                                                                                                source_ml_source1_data_property_value950.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value951, _, _) => {
                                                                                                source_ml_source1_data_property_value951.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value952, _, _) => {
                                                                                                source_ml_source1_data_property_value952.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value953, _, _) => {
                                                                                                source_ml_source1_data_property_value953.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value954, _, _) => {
                                                                                                source_ml_source1_data_property_value954.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_temporary_index23 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source1_s955 = self::source_ml_temporary_plan_java_script_array_length_definition_loop25(new_writable, final_length, properties.clone(), source_ml_temporary_values23.clone(), source_ml_temporary_index23.clone());
                                                                                                match source_ml_source1_s955.clone() {
                                                                                                    self::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Result::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Done => {
                                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(match match initial.clone() {
                                                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value956, _, _, _, _) => {
                                                                                                                let source_ml_source1_data_property_value956 = *source_ml_source1_data_property_value956;
                                                                                                                source_ml_source1_data_property_value956.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field957, source_ml_source1_nominal_field958, source_ml_source1_nominal_field959, source_ml_source1_nominal_field960, source_ml_source1_nominal_field961, source_ml_source1_nominal_field962, source_ml_source1_nominal_field963, source_ml_source1_nominal_field964, source_ml_source1_nominal_field965, source_ml_source1_nominal_field966) => {
                                                                                                                let source_ml_source1_nominal_field958 = *source_ml_source1_nominal_field958;
                                                                                                                let source_ml_source1_nominal_field965 = *source_ml_source1_nominal_field965;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override967 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field957.clone(), Box::new(source_ml_source1_nominal_field958.clone()), source_ml_source1_nominal_field959, source_ml_source1_nominal_field960, source_ml_source1_nominal_override967.clone(), source_ml_source1_nominal_field962.clone(), source_ml_source1_nominal_field963, source_ml_source1_nominal_field964.clone(), Box::new(source_ml_source1_nominal_field965.clone()), source_ml_source1_nominal_field966.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field968, source_ml_source1_nominal_field969, source_ml_source1_nominal_field970, source_ml_source1_nominal_field971, source_ml_source1_nominal_field972, source_ml_source1_nominal_field973, source_ml_source1_nominal_field974, source_ml_source1_nominal_field975) => {
                                                                                                                let source_ml_source1_nominal_field969 = *source_ml_source1_nominal_field969;
                                                                                                                let source_ml_source1_nominal_field973 = *source_ml_source1_nominal_field973;
                                                                                                                let source_ml_source1_nominal_field974 = *source_ml_source1_nominal_field974;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override976 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field968.clone(), Box::new(source_ml_source1_nominal_field969.clone()), source_ml_source1_nominal_field970, source_ml_source1_nominal_field971, source_ml_source1_nominal_override976.clone(), Box::new(source_ml_source1_nominal_field973.clone()), Box::new(source_ml_source1_nominal_field974.clone()), source_ml_source1_nominal_field975.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field977, source_ml_source1_nominal_field978, source_ml_source1_nominal_field979, source_ml_source1_nominal_field980, source_ml_source1_nominal_field981) => {
                                                                                                                let source_ml_source1_nominal_field978 = *source_ml_source1_nominal_field978;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override982 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field977.clone(), Box::new(source_ml_source1_nominal_field978.clone()), source_ml_source1_nominal_field979, source_ml_source1_nominal_field980, source_ml_source1_nominal_override982.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field983, source_ml_source1_nominal_field984, source_ml_source1_nominal_field985, source_ml_source1_nominal_field986, source_ml_source1_nominal_field987, source_ml_source1_nominal_field988, source_ml_source1_nominal_field989, source_ml_source1_nominal_field990, source_ml_source1_nominal_field991, source_ml_source1_nominal_field992, source_ml_source1_nominal_field993) => {
                                                                                                                let source_ml_source1_nominal_field984 = *source_ml_source1_nominal_field984;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override994 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field983.clone(), Box::new(source_ml_source1_nominal_field984.clone()), source_ml_source1_nominal_field985, source_ml_source1_nominal_field986, source_ml_source1_nominal_override994.clone(), source_ml_source1_nominal_field988.clone(), source_ml_source1_nominal_field989, source_ml_source1_nominal_field990.clone(), source_ml_source1_nominal_field991.clone(), source_ml_source1_nominal_field992.clone(), source_ml_source1_nominal_field993.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field995, source_ml_source1_nominal_field996, source_ml_source1_nominal_field997, source_ml_source1_nominal_field998, source_ml_source1_nominal_field999, source_ml_source1_nominal_field1000) => {
                                                                                                                let source_ml_source1_nominal_field996 = *source_ml_source1_nominal_field996;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1001 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field995.clone(), Box::new(source_ml_source1_nominal_field996.clone()), source_ml_source1_nominal_field997, source_ml_source1_nominal_field998, source_ml_source1_nominal_override1001.clone(), source_ml_source1_nominal_field1000.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1002, source_ml_source1_nominal_field1003, source_ml_source1_nominal_field1004, source_ml_source1_nominal_field1005, source_ml_source1_nominal_field1006, source_ml_source1_nominal_field1007) => {
                                                                                                                let source_ml_source1_nominal_field1003 = *source_ml_source1_nominal_field1003;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1008 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1002.clone(), Box::new(source_ml_source1_nominal_field1003.clone()), source_ml_source1_nominal_field1004, source_ml_source1_nominal_field1005, source_ml_source1_nominal_override1008.clone(), source_ml_source1_nominal_field1007.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1009, source_ml_source1_nominal_field1010, source_ml_source1_nominal_field1011, source_ml_source1_nominal_field1012, source_ml_source1_nominal_field1013, source_ml_source1_nominal_field1014) => {
                                                                                                                let source_ml_source1_nominal_field1010 = *source_ml_source1_nominal_field1010;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1015 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1009.clone(), Box::new(source_ml_source1_nominal_field1010.clone()), source_ml_source1_nominal_field1011, source_ml_source1_nominal_field1012, source_ml_source1_nominal_override1015.clone(), source_ml_source1_nominal_field1014.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1016, source_ml_source1_nominal_field1017, source_ml_source1_nominal_field1018, source_ml_source1_nominal_field1019, source_ml_source1_nominal_field1020, source_ml_source1_nominal_field1021) => {
                                                                                                                let source_ml_source1_nominal_field1017 = *source_ml_source1_nominal_field1017;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1022 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1016.clone(), Box::new(source_ml_source1_nominal_field1017.clone()), source_ml_source1_nominal_field1018, source_ml_source1_nominal_field1019, source_ml_source1_nominal_override1022.clone(), source_ml_source1_nominal_field1021.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1023, source_ml_source1_nominal_field1024, source_ml_source1_nominal_field1025, source_ml_source1_nominal_field1026, source_ml_source1_nominal_field1027, source_ml_source1_nominal_field1028, source_ml_source1_nominal_field1029, source_ml_source1_nominal_field1030, source_ml_source1_nominal_field1031) => {
                                                                                                                let source_ml_source1_nominal_field1024 = *source_ml_source1_nominal_field1024;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1032 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1023.clone(), Box::new(source_ml_source1_nominal_field1024.clone()), source_ml_source1_nominal_field1025, source_ml_source1_nominal_field1026.clone(), source_ml_source1_nominal_field1027.clone(), source_ml_source1_nominal_field1028.clone(), source_ml_source1_nominal_field1029.clone(), source_ml_source1_nominal_field1030, source_ml_source1_nominal_override1032.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1033, source_ml_source1_nominal_field1034, source_ml_source1_nominal_field1035, source_ml_source1_nominal_field1036, source_ml_source1_nominal_field1037, source_ml_source1_nominal_field1038, source_ml_source1_nominal_field1039) => {
                                                                                                                let source_ml_source1_nominal_field1034 = *source_ml_source1_nominal_field1034;
                                                                                                                let source_ml_source1_nominal_field1038 = *source_ml_source1_nominal_field1038;
                                                                                                                let source_ml_source1_nominal_field1039 = *source_ml_source1_nominal_field1039;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1040 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1033.clone(), Box::new(source_ml_source1_nominal_field1034.clone()), source_ml_source1_nominal_field1035, source_ml_source1_nominal_field1036, source_ml_source1_nominal_override1040.clone(), Box::new(source_ml_source1_nominal_field1038.clone()), Box::new(source_ml_source1_nominal_field1039.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1041, source_ml_source1_nominal_field1042, source_ml_source1_nominal_field1043, source_ml_source1_nominal_field1044, source_ml_source1_nominal_field1045, source_ml_source1_nominal_field1046, source_ml_source1_nominal_field1047) => {
                                                                                                                let source_ml_source1_nominal_field1042 = *source_ml_source1_nominal_field1042;
                                                                                                                let source_ml_source1_nominal_field1046 = *source_ml_source1_nominal_field1046;
                                                                                                                let source_ml_source1_nominal_field1047 = *source_ml_source1_nominal_field1047;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1048 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1041.clone(), Box::new(source_ml_source1_nominal_field1042.clone()), source_ml_source1_nominal_field1043, source_ml_source1_nominal_field1044, source_ml_source1_nominal_override1048.clone(), Box::new(source_ml_source1_nominal_field1046.clone()), Box::new(source_ml_source1_nominal_field1047.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1049, source_ml_source1_nominal_field1050, source_ml_source1_nominal_field1051, source_ml_source1_nominal_field1052, source_ml_source1_nominal_field1053, source_ml_source1_nominal_field1054, source_ml_source1_nominal_field1055) => {
                                                                                                                let source_ml_source1_nominal_field1050 = *source_ml_source1_nominal_field1050;
                                                                                                                let source_ml_source1_nominal_field1054 = *source_ml_source1_nominal_field1054;
                                                                                                                let source_ml_source1_nominal_field1055 = *source_ml_source1_nominal_field1055;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1056 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1049.clone(), Box::new(source_ml_source1_nominal_field1050.clone()), source_ml_source1_nominal_field1051, source_ml_source1_nominal_field1052, source_ml_source1_nominal_override1056.clone(), Box::new(source_ml_source1_nominal_field1054.clone()), Box::new(source_ml_source1_nominal_field1055.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1057, source_ml_source1_nominal_field1058, source_ml_source1_nominal_field1059, source_ml_source1_nominal_field1060, source_ml_source1_nominal_field1061, source_ml_source1_nominal_field1062, source_ml_source1_nominal_field1063) => {
                                                                                                                let source_ml_source1_nominal_field1058 = *source_ml_source1_nominal_field1058;
                                                                                                                let source_ml_source1_nominal_field1062 = *source_ml_source1_nominal_field1062;
                                                                                                                let source_ml_source1_nominal_field1063 = *source_ml_source1_nominal_field1063;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1064 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1057.clone(), Box::new(source_ml_source1_nominal_field1058.clone()), source_ml_source1_nominal_field1059, source_ml_source1_nominal_field1060, source_ml_source1_nominal_override1064.clone(), Box::new(source_ml_source1_nominal_field1062.clone()), Box::new(source_ml_source1_nominal_field1063.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                        }), (blocked_2 < (0f64)), self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                            let source_ml_source1_concat_value1068 = match initial.clone() {
                                                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value1065) => {
                                                                                                                    source_ml_source1_data_property_value1065.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value1069 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 105, 110, 100, 105, 99, 101, 115]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                                                {
                                                                                                                    let source_ml_source1_concat_value1070 = if (!new_writable && !(self::ml_array::includes(&{ let ml_reference_value155 = (match initial.clone() {
                                                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value1067) => {
                                                                                                                            source_ml_source1_data_property_value1067.clone()
                                                                                                                        }
                                                                                                                    }).borrow().clone(); ml_reference_value155 }, &self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]), 0f64, false, |a, b| a == b))) {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]))
                                                                                                                    } else {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))
                                                                                                                    };
                                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value156 = source_ml_source1_concat_value1068.borrow().clone(); ml_reference_value156 }), { let ml_reference_value157 = source_ml_source1_concat_value1069.borrow().clone(); ml_reference_value157 }), { let ml_reference_value158 = source_ml_source1_concat_value1070.borrow().clone(); ml_reference_value158 })
                                                                                                                }
                                                                                                            }
                                                                                                        })))
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
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn plan_java_script_array_property_definition(object: self::JavaScriptOrdinaryObject, key: self::JavaScriptSemanticValue, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::JavaScriptArrayPropertyPlan {
    if self::test_java_script_array_length_key(key.clone()) {
        if !(({ let ml_reference_value159 = proposed.borrow().clone(); ml_reference_value159 }).9) {
            self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), proposed.clone())
        } else {
            {
                let value = ({ let ml_reference_value160 = proposed.borrow().clone(); ml_reference_value160 }).8;
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1071) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        self::plan_java_script_array_length_definition(object.clone(), proposed.clone(), self::convert_java_script_array_unsigned_length(value_value), value_value)
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1072) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f1073) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f1074, source_ml_source1_f1075) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1076) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1077, source_ml_source1_f1078) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                }
            }
        }
    } else {
        {
            let index = self::read_java_script_array_index(key.clone());
            {
                let length = self::read_java_script_array_length(object.clone());
                {
                    let length_property = {
                        let source_ml_source1_array_read_value1105 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1079, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1079.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1080, _, _, _) => {
                                source_ml_source1_data_property_value1080.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1081) => {
                                source_ml_source1_data_property_value1081.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1082, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1082.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1083, _) => {
                                source_ml_source1_data_property_value1083.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1084, _) => {
                                source_ml_source1_data_property_value1084.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1085, _) => {
                                source_ml_source1_data_property_value1085.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1086, _) => {
                                source_ml_source1_data_property_value1086.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1087) => {
                                source_ml_source1_data_property_value1087.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1088, _, _) => {
                                source_ml_source1_data_property_value1088.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1089, _, _) => {
                                source_ml_source1_data_property_value1089.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1090, _, _) => {
                                source_ml_source1_data_property_value1090.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1091, _, _) => {
                                source_ml_source1_data_property_value1091.clone()
                            }
                        };
                        {
                            let source_ml_source1_array_read_index1106 = self::find_java_script_array_property(match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1092, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1092.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1093, _, _, _) => {
                                    source_ml_source1_data_property_value1093.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1094) => {
                                    source_ml_source1_data_property_value1094.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1095, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1095.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1096, _) => {
                                    source_ml_source1_data_property_value1096.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1097, _) => {
                                    source_ml_source1_data_property_value1097.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1098, _) => {
                                    source_ml_source1_data_property_value1098.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1099, _) => {
                                    source_ml_source1_data_property_value1099.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1100) => {
                                    source_ml_source1_data_property_value1100.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1101, _, _) => {
                                    source_ml_source1_data_property_value1101.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1102, _, _) => {
                                    source_ml_source1_data_property_value1102.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1103, _, _) => {
                                    source_ml_source1_data_property_value1103.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1104, _, _) => {
                                    source_ml_source1_data_property_value1104.clone()
                                }
                            }, self::read_java_script_array_length_key());
                            { let ml_reference_array161 = source_ml_source1_array_read_value1105.clone(); let ml_reference_position161 = source_ml_source1_array_read_index1106; let ml_reference_projection161 = self::ml_array::optional_at(&*ml_reference_array161.borrow(), self::ml_array::number_index(ml_reference_position161)); ml_reference_projection161 }
                        }
                    };
                    if ((index >= length) && (match (length_property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(({ let ml_reference_value162 = (match length_property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1107) => {
                            source_ml_source1_data_property_value1107.clone()
                        }
                    }).borrow().clone(); ml_reference_value162 }).10))) {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    } else {
                        {
                            let plan = self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), proposed.clone());
                            if ((!(match plan.clone() {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source1_data_property_value1108, _, _, _) => {
                                    source_ml_source1_data_property_value1108
                                }
                            }) || (index < length)) || (index < (0f64))) {
                                plan.clone()
                            } else {
                                {
                                    let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                    {
                                        let source_ml_temporary_values27 = match match plan.clone() {
                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value1109, _, _, _, _) => {
                                                let source_ml_source1_data_property_value1109 = *source_ml_source1_data_property_value1109;
                                                source_ml_source1_data_property_value1109.clone()
                                            }
                                        } {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1110, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value1110.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1111, _, _, _) => {
                                                source_ml_source1_data_property_value1111.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1112) => {
                                                source_ml_source1_data_property_value1112.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1113, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value1113.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1114, _) => {
                                                source_ml_source1_data_property_value1114.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1115, _) => {
                                                source_ml_source1_data_property_value1115.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1116, _) => {
                                                source_ml_source1_data_property_value1116.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1117, _) => {
                                                source_ml_source1_data_property_value1117.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1118) => {
                                                source_ml_source1_data_property_value1118.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1119, _, _) => {
                                                source_ml_source1_data_property_value1119.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1120, _, _) => {
                                                source_ml_source1_data_property_value1120.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1121, _, _) => {
                                                source_ml_source1_data_property_value1121.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1122, _, _) => {
                                                source_ml_source1_data_property_value1122.clone()
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index27 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_s1123 = self::source_ml_temporary_plan_java_script_array_property_definition_loop28(index, properties.clone(), source_ml_temporary_values27.clone(), source_ml_temporary_index27.clone());
                                                match source_ml_source1_s1123.clone() {
                                                    self::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Result::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Done => {
                                                        {
                                                            let source_ml_source1_nominal_spread1236 = plan.clone();
                                                            {
                                                                let source_ml_source1_nominal_field1237 = match source_ml_source1_nominal_spread1236.clone() {
                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value1238, _, _, _, _) => {
                                                                        let source_ml_source1_data_property_value1238 = *source_ml_source1_data_property_value1238;
                                                                        source_ml_source1_data_property_value1238.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1239 = match source_ml_source1_nominal_spread1236.clone() {
                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source1_data_property_value1240, _, _, _) => {
                                                                            source_ml_source1_data_property_value1240
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1241 = match source_ml_source1_nominal_spread1236.clone() {
                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, source_ml_source1_data_property_value1242, _, _) => {
                                                                                source_ml_source1_data_property_value1242.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field1243 = match source_ml_source1_nominal_spread1236.clone() {
                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, source_ml_source1_data_property_value1244, _) => {
                                                                                    source_ml_source1_data_property_value1244.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field1245 = match source_ml_source1_nominal_spread1236.clone() {
                                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value1246) => {
                                                                                        source_ml_source1_data_property_value1246.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_override1247 = match match plan.clone() {
                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value1124, _, _, _, _) => {
                                                                                            let source_ml_source1_data_property_value1124 = *source_ml_source1_data_property_value1124;
                                                                                            source_ml_source1_data_property_value1124.clone()
                                                                                        }
                                                                                    } {
                                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field1125, source_ml_source1_nominal_field1126, source_ml_source1_nominal_field1127, source_ml_source1_nominal_field1128, source_ml_source1_nominal_field1129, source_ml_source1_nominal_field1130, source_ml_source1_nominal_field1131, source_ml_source1_nominal_field1132, source_ml_source1_nominal_field1133, source_ml_source1_nominal_field1134) => {
                                                                                            let source_ml_source1_nominal_field1126 = *source_ml_source1_nominal_field1126;
                                                                                            let source_ml_source1_nominal_field1133 = *source_ml_source1_nominal_field1133;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1135 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field1125.clone(), Box::new(source_ml_source1_nominal_field1126.clone()), source_ml_source1_nominal_field1127, source_ml_source1_nominal_field1128, source_ml_source1_nominal_override1135.clone(), source_ml_source1_nominal_field1130.clone(), source_ml_source1_nominal_field1131, source_ml_source1_nominal_field1132.clone(), Box::new(source_ml_source1_nominal_field1133.clone()), source_ml_source1_nominal_field1134.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field1136, source_ml_source1_nominal_field1137, source_ml_source1_nominal_field1138, source_ml_source1_nominal_field1139, source_ml_source1_nominal_field1140, source_ml_source1_nominal_field1141, source_ml_source1_nominal_field1142, source_ml_source1_nominal_field1143) => {
                                                                                            let source_ml_source1_nominal_field1137 = *source_ml_source1_nominal_field1137;
                                                                                            let source_ml_source1_nominal_field1141 = *source_ml_source1_nominal_field1141;
                                                                                            let source_ml_source1_nominal_field1142 = *source_ml_source1_nominal_field1142;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1144 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field1136.clone(), Box::new(source_ml_source1_nominal_field1137.clone()), source_ml_source1_nominal_field1138, source_ml_source1_nominal_field1139, source_ml_source1_nominal_override1144.clone(), Box::new(source_ml_source1_nominal_field1141.clone()), Box::new(source_ml_source1_nominal_field1142.clone()), source_ml_source1_nominal_field1143.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field1145, source_ml_source1_nominal_field1146, source_ml_source1_nominal_field1147, source_ml_source1_nominal_field1148, source_ml_source1_nominal_field1149) => {
                                                                                            let source_ml_source1_nominal_field1146 = *source_ml_source1_nominal_field1146;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1150 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field1145.clone(), Box::new(source_ml_source1_nominal_field1146.clone()), source_ml_source1_nominal_field1147, source_ml_source1_nominal_field1148, source_ml_source1_nominal_override1150.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field1151, source_ml_source1_nominal_field1152, source_ml_source1_nominal_field1153, source_ml_source1_nominal_field1154, source_ml_source1_nominal_field1155, source_ml_source1_nominal_field1156, source_ml_source1_nominal_field1157, source_ml_source1_nominal_field1158, source_ml_source1_nominal_field1159, source_ml_source1_nominal_field1160, source_ml_source1_nominal_field1161) => {
                                                                                            let source_ml_source1_nominal_field1152 = *source_ml_source1_nominal_field1152;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1162 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field1151.clone(), Box::new(source_ml_source1_nominal_field1152.clone()), source_ml_source1_nominal_field1153, source_ml_source1_nominal_field1154, source_ml_source1_nominal_override1162.clone(), source_ml_source1_nominal_field1156.clone(), source_ml_source1_nominal_field1157, source_ml_source1_nominal_field1158.clone(), source_ml_source1_nominal_field1159.clone(), source_ml_source1_nominal_field1160.clone(), source_ml_source1_nominal_field1161.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field1163, source_ml_source1_nominal_field1164, source_ml_source1_nominal_field1165, source_ml_source1_nominal_field1166, source_ml_source1_nominal_field1167, source_ml_source1_nominal_field1168) => {
                                                                                            let source_ml_source1_nominal_field1164 = *source_ml_source1_nominal_field1164;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1169 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field1163.clone(), Box::new(source_ml_source1_nominal_field1164.clone()), source_ml_source1_nominal_field1165, source_ml_source1_nominal_field1166, source_ml_source1_nominal_override1169.clone(), source_ml_source1_nominal_field1168.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1170, source_ml_source1_nominal_field1171, source_ml_source1_nominal_field1172, source_ml_source1_nominal_field1173, source_ml_source1_nominal_field1174, source_ml_source1_nominal_field1175) => {
                                                                                            let source_ml_source1_nominal_field1171 = *source_ml_source1_nominal_field1171;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1176 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1170.clone(), Box::new(source_ml_source1_nominal_field1171.clone()), source_ml_source1_nominal_field1172, source_ml_source1_nominal_field1173, source_ml_source1_nominal_override1176.clone(), source_ml_source1_nominal_field1175.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1177, source_ml_source1_nominal_field1178, source_ml_source1_nominal_field1179, source_ml_source1_nominal_field1180, source_ml_source1_nominal_field1181, source_ml_source1_nominal_field1182) => {
                                                                                            let source_ml_source1_nominal_field1178 = *source_ml_source1_nominal_field1178;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1183 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1177.clone(), Box::new(source_ml_source1_nominal_field1178.clone()), source_ml_source1_nominal_field1179, source_ml_source1_nominal_field1180, source_ml_source1_nominal_override1183.clone(), source_ml_source1_nominal_field1182.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1184, source_ml_source1_nominal_field1185, source_ml_source1_nominal_field1186, source_ml_source1_nominal_field1187, source_ml_source1_nominal_field1188, source_ml_source1_nominal_field1189) => {
                                                                                            let source_ml_source1_nominal_field1185 = *source_ml_source1_nominal_field1185;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1190 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1184.clone(), Box::new(source_ml_source1_nominal_field1185.clone()), source_ml_source1_nominal_field1186, source_ml_source1_nominal_field1187, source_ml_source1_nominal_override1190.clone(), source_ml_source1_nominal_field1189.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1191, source_ml_source1_nominal_field1192, source_ml_source1_nominal_field1193, source_ml_source1_nominal_field1194, source_ml_source1_nominal_field1195, source_ml_source1_nominal_field1196, source_ml_source1_nominal_field1197, source_ml_source1_nominal_field1198, source_ml_source1_nominal_field1199) => {
                                                                                            let source_ml_source1_nominal_field1192 = *source_ml_source1_nominal_field1192;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1200 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1191.clone(), Box::new(source_ml_source1_nominal_field1192.clone()), source_ml_source1_nominal_field1193, source_ml_source1_nominal_field1194.clone(), source_ml_source1_nominal_field1195.clone(), source_ml_source1_nominal_field1196.clone(), source_ml_source1_nominal_field1197.clone(), source_ml_source1_nominal_field1198, source_ml_source1_nominal_override1200.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1201, source_ml_source1_nominal_field1202, source_ml_source1_nominal_field1203, source_ml_source1_nominal_field1204, source_ml_source1_nominal_field1205, source_ml_source1_nominal_field1206, source_ml_source1_nominal_field1207) => {
                                                                                            let source_ml_source1_nominal_field1202 = *source_ml_source1_nominal_field1202;
                                                                                            let source_ml_source1_nominal_field1206 = *source_ml_source1_nominal_field1206;
                                                                                            let source_ml_source1_nominal_field1207 = *source_ml_source1_nominal_field1207;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1208 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1201.clone(), Box::new(source_ml_source1_nominal_field1202.clone()), source_ml_source1_nominal_field1203, source_ml_source1_nominal_field1204, source_ml_source1_nominal_override1208.clone(), Box::new(source_ml_source1_nominal_field1206.clone()), Box::new(source_ml_source1_nominal_field1207.clone()))
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1209, source_ml_source1_nominal_field1210, source_ml_source1_nominal_field1211, source_ml_source1_nominal_field1212, source_ml_source1_nominal_field1213, source_ml_source1_nominal_field1214, source_ml_source1_nominal_field1215) => {
                                                                                            let source_ml_source1_nominal_field1210 = *source_ml_source1_nominal_field1210;
                                                                                            let source_ml_source1_nominal_field1214 = *source_ml_source1_nominal_field1214;
                                                                                            let source_ml_source1_nominal_field1215 = *source_ml_source1_nominal_field1215;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1216 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1209.clone(), Box::new(source_ml_source1_nominal_field1210.clone()), source_ml_source1_nominal_field1211, source_ml_source1_nominal_field1212, source_ml_source1_nominal_override1216.clone(), Box::new(source_ml_source1_nominal_field1214.clone()), Box::new(source_ml_source1_nominal_field1215.clone()))
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1217, source_ml_source1_nominal_field1218, source_ml_source1_nominal_field1219, source_ml_source1_nominal_field1220, source_ml_source1_nominal_field1221, source_ml_source1_nominal_field1222, source_ml_source1_nominal_field1223) => {
                                                                                            let source_ml_source1_nominal_field1218 = *source_ml_source1_nominal_field1218;
                                                                                            let source_ml_source1_nominal_field1222 = *source_ml_source1_nominal_field1222;
                                                                                            let source_ml_source1_nominal_field1223 = *source_ml_source1_nominal_field1223;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1224 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1217.clone(), Box::new(source_ml_source1_nominal_field1218.clone()), source_ml_source1_nominal_field1219, source_ml_source1_nominal_field1220, source_ml_source1_nominal_override1224.clone(), Box::new(source_ml_source1_nominal_field1222.clone()), Box::new(source_ml_source1_nominal_field1223.clone()))
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1225, source_ml_source1_nominal_field1226, source_ml_source1_nominal_field1227, source_ml_source1_nominal_field1228, source_ml_source1_nominal_field1229, source_ml_source1_nominal_field1230, source_ml_source1_nominal_field1231) => {
                                                                                            let source_ml_source1_nominal_field1226 = *source_ml_source1_nominal_field1226;
                                                                                            let source_ml_source1_nominal_field1230 = *source_ml_source1_nominal_field1230;
                                                                                            let source_ml_source1_nominal_field1231 = *source_ml_source1_nominal_field1231;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1232 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1225.clone(), Box::new(source_ml_source1_nominal_field1226.clone()), source_ml_source1_nominal_field1227, source_ml_source1_nominal_field1228, source_ml_source1_nominal_override1232.clone(), Box::new(source_ml_source1_nominal_field1230.clone()), Box::new(source_ml_source1_nominal_field1231.clone()))
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_override1248 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                            let source_ml_source1_concat_value1234 = match plan.clone() {
                                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value1233) => {
                                                                                                    source_ml_source1_data_property_value1233.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_concat_value1235 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value163 = source_ml_source1_concat_value1234.borrow().clone(); ml_reference_value163 }), { let ml_reference_value164 = source_ml_source1_concat_value1235.borrow().clone(); ml_reference_value164 })
                                                                                            }
                                                                                        }));
                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(source_ml_source1_nominal_override1247.clone()), source_ml_source1_nominal_field1239, source_ml_source1_nominal_field1241.clone(), source_ml_source1_nominal_field1243.clone(), source_ml_source1_nominal_override1248.clone())
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
                }
            }
        }
    }
}

pub fn convert_java_script_array_unsigned_length(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integer = value.trunc();
            {
                let remainder = (integer % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ({ let ml_reference_array165 = units.clone(); let ml_reference_projection165 = ml_reference_array165.borrow().len() as f64; ml_reference_projection165 })) {
            {
                let unit = ({
                    let source_ml_source1_array_read_value1249 = units.clone();
                    {
                        let source_ml_source1_array_read_index1250 = at;
                        { let ml_reference_array166 = source_ml_source1_array_read_value1249.clone(); let ml_reference_position166 = source_ml_source1_array_read_index1250; let ml_reference_projection166 = self::ml_array::optional_at(&*ml_reference_array166.borrow(), self::ml_array::number_index(ml_reference_position166)); ml_reference_projection166 }
                    }
                }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}

pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop2(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut at: f64) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    loop {
        return if (at < length) {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        (-1f64)
                    }
                    self::JavaScriptSemanticValue::Null => {
                        (-1f64)
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1251) => {
                        {
                            let source_ml_source1_f1275 = source_ml_source1_f1251;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f1252) => {
                        {
                            let source_ml_source1_f1284 = source_ml_source1_f1252;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1253) => {
                        {
                            let source_ml_source1_f1293 = source_ml_source1_f1253.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Text(units) => {
                        ({
                            let source_ml_source1_array_read_value1299 = units.clone();
                            {
                                let source_ml_source1_array_read_index1300 = at;
                                { let ml_reference_array167 = source_ml_source1_array_read_value1299.clone(); let ml_reference_position167 = source_ml_source1_array_read_index1300; let ml_reference_projection167 = self::ml_array::optional_at(&*ml_reference_array167.borrow(), self::ml_array::number_index(ml_reference_position167)); ml_reference_projection167 }
                            }
                        }).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f1254, source_ml_source1_f1255) => {
                        {
                            let source_ml_source1_f1305 = source_ml_source1_f1254.clone();
                            {
                                let text = source_ml_source1_f1255.clone();
                                {
                                    let source_ml_source1_string_unit_operand1309 = text.clone();
                                    {
                                        let source_ml_source1_string_unit_operand1310 = at;
                                        (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 { {
                                            let index = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if ((index < (0f64)) || (index >= (text_2.length()))) {
                                                f64::NAN
                                            } else {
                                                ({
                                                    let source_ml_source1_string_unit_operand1316 = {
                                                        let source_ml_source1_string_range_operand1311 = text_2.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand1312 = index;
                                                            {
                                                                let source_ml_source1_string_range_operand1313 = (index + 1f64);
                                                                {
                                                                    let source_ml_source1_string_range_position1314 = source_ml_source1_string_range_operand1312;
                                                                    {
                                                                        let source_ml_source1_string_range_position1315 = source_ml_source1_string_range_operand1313;
                                                                        source_ml_source1_string_range_operand1311.slice(source_ml_source1_string_range_position1314, source_ml_source1_string_range_position1315)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_string_unit_operand1317 = 0f64;
                                                        source_ml_source1_string_unit_operand1316.code_point_at(source_ml_source1_string_unit_operand1317)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source1_string_unit_operand1309.clone(), source_ml_source1_string_unit_operand1310)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1256) => {
                        {
                            let source_ml_source1_f1323 = source_ml_source1_f1256.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1257, source_ml_source1_f1258) => {
                        {
                            let source_ml_source1_f1332 = source_ml_source1_f1257.clone();
                            {
                                let source_ml_source1_f1333 = source_ml_source1_f1258;
                                (-1f64)
                            }
                        }
                    }
                };
                {
                    let b = match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            (-1f64)
                        }
                        self::JavaScriptSemanticValue::Null => {
                            (-1f64)
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1334) => {
                            {
                                let source_ml_source1_f1358 = source_ml_source1_f1334;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1335) => {
                            {
                                let source_ml_source1_f1367 = source_ml_source1_f1335;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1336) => {
                            {
                                let source_ml_source1_f1376 = source_ml_source1_f1336.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units_2) => {
                            ({
                                let source_ml_source1_array_read_value1382 = units_2.clone();
                                {
                                    let source_ml_source1_array_read_index1383 = at;
                                    { let ml_reference_array168 = source_ml_source1_array_read_value1382.clone(); let ml_reference_position168 = source_ml_source1_array_read_index1383; let ml_reference_projection168 = self::ml_array::optional_at(&*ml_reference_array168.borrow(), self::ml_array::number_index(ml_reference_position168)); ml_reference_projection168 }
                                }
                            }).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f1337, source_ml_source1_f1338) => {
                            {
                                let source_ml_source1_f1388 = source_ml_source1_f1337.clone();
                                {
                                    let text_2 = source_ml_source1_f1338.clone();
                                    {
                                        let source_ml_source1_string_unit_operand1392 = text_2.clone();
                                        {
                                            let source_ml_source1_string_unit_operand1393 = at;
                                            (std::rc::Rc::new(move |text_3: self::ml_string::Text, position: f64| -> f64 { {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if ((index < (0f64)) || (index >= (text_3.length()))) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                        let source_ml_source1_string_unit_operand1399 = {
                                                            let source_ml_source1_string_range_operand1394 = text_3.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand1395 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand1396 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source1_string_range_position1397 = source_ml_source1_string_range_operand1395;
                                                                        {
                                                                            let source_ml_source1_string_range_position1398 = source_ml_source1_string_range_operand1396;
                                                                            source_ml_source1_string_range_operand1394.slice(source_ml_source1_string_range_position1397, source_ml_source1_string_range_position1398)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_string_unit_operand1400 = 0f64;
                                                            source_ml_source1_string_unit_operand1399.code_point_at(source_ml_source1_string_unit_operand1400)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source1_string_unit_operand1392.clone(), source_ml_source1_string_unit_operand1393)
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1339) => {
                            {
                                let source_ml_source1_f1406 = source_ml_source1_f1339.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1340, source_ml_source1_f1341) => {
                            {
                                let source_ml_source1_f1415 = source_ml_source1_f1340.clone();
                                {
                                    let source_ml_source1_f1416 = source_ml_source1_f1341;
                                    (-1f64)
                                }
                            }
                        }
                    };
                    if (a != b) {
                        self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(false)
                    } else {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done
        };
    }
}

pub fn source_ml_temporary_read_java_script_semantic_text_string_loop4(mut text: self::ml_string::Text, mut source_ml_temporary_values3: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_temporary_index3: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_temporary_index3 < ({ let ml_reference_array169 = source_ml_temporary_values3.clone(); let ml_reference_projection169 = self::ml::Big::from_u128(ml_reference_array169.borrow().len() as u128); ml_reference_projection169 })) {
            {
                let unit = {
                    let source_ml_source1_array_read_value1417 = source_ml_temporary_values3.clone();
                    {
                        let source_ml_source1_array_read_index1418 = source_ml_temporary_index3.clone();
                        { let ml_reference_array170 = source_ml_source1_array_read_value1417.clone(); let ml_reference_position170 = source_ml_source1_array_read_index1418.clone(); let ml_reference_projection170 = self::ml_array::at(&*ml_reference_array170.borrow(), ml_reference_position170.to_index()); ml_reference_projection170 }
                    }
                };
                {
                    let text_2 = text.concat(&{
                        let source_ml_source1_character_argument1419 = unit;
                        self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source1_character_argument1419]).into_iter().map(|ml_unit| ml_unit as u16).collect())
                    });
                    {
                        let source_ml_temporary_index3_2 = source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_temporary_index3) = (text_2.clone(), source_ml_temporary_index3_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            text.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result {
    MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done,
}

pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop5(mut value: self::JavaScriptSemanticValue, mut length: f64, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result {
    loop {
        return if (at < length) {
            {
                let source_ml_temporary_construct_java_script_semantic_text_units_ignored6 = { let ml_method_owner171 = units.clone(); let ml_method_argument171_0 = self::read_java_script_semantic_text_unit(value.clone(), at); let ml_inserted = vec![ml_method_argument171_0]; let mut ml_method_values171 = ml_method_owner171.borrow_mut(); let ml_length = ml_method_values171.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values171.extend(ml_inserted); ml_length as f64 };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Result {
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Done,
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Return(f64),
}

pub fn source_ml_temporary_find_java_script_property_descriptor_entry_loop7(mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Result {
    loop {
        return if (at < ({ let ml_reference_array172 = properties.clone(); let ml_reference_projection172 = ml_reference_array172.borrow().len() as f64; ml_reference_projection172 })) {
            {
                let property = {
                    let source_ml_source1_array_read_value1420 = properties.clone();
                    {
                        let source_ml_source1_array_read_index1421 = at;
                        { let ml_reference_array173 = source_ml_source1_array_read_value1420.clone(); let ml_reference_position173 = source_ml_source1_array_read_index1421; let ml_reference_projection173 = self::ml_array::optional_at(&*ml_reference_array173.borrow(), self::ml_array::number_index(ml_reference_position173)); ml_reference_projection173 }
                    }
                };
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value(match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1422, _) => {
                        let source_ml_source1_data_property_value1422 = *source_ml_source1_data_property_value1422;
                        source_ml_source1_data_property_value1422.clone()
                    }
                }, key.clone())) {
                    self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Return(at)
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin8 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Return(f64),
}

pub fn source_ml_temporary_read_java_script_literal_property_index_loop9(mut key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut value: f64, mut at: f64) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Result {
    loop {
        return if (at < ({ let ml_reference_array174 = key.clone(); let ml_reference_projection174 = ml_reference_array174.borrow().len() as f64; ml_reference_projection174 })) {
            {
                let digit = (match {
                    let source_ml_source1_array_read_value1423 = key.clone();
                    {
                        let source_ml_source1_array_read_index1424 = at;
                        { let ml_reference_array175 = source_ml_source1_array_read_value1423.clone(); let ml_reference_position175 = source_ml_source1_array_read_index1424; let ml_reference_projection175 = self::ml_array::optional_at(&*ml_reference_array175.borrow(), self::ml_array::number_index(ml_reference_position175)); ml_reference_projection175 }
                    }
                } { Some(ml_present) => ml_present, None => f64::NAN } - 48f64);
                if ((!(self::ml_math::is_integer(digit)) || (digit < (0f64))) || (digit > (9f64))) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Return((-1f64))
                } else {
                    {
                        let value_2 = ((value * 10f64) + digit);
                        {
                            let at_2 = (at + 1f64);
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result {
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Done,
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Return(f64),
}

pub fn source_ml_temporary_find_java_script_heap_directory_object_loop10(mut objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = {
                        let source_ml_source1_array_read_value1425 = objects.clone();
                        {
                            let source_ml_source1_array_read_index1426 = middle;
                            { let ml_reference_array176 = source_ml_source1_array_read_value1425.clone(); let ml_reference_position176 = source_ml_source1_array_read_index1426; let ml_reference_projection176 = self::ml_array::optional_at(&*ml_reference_array176.borrow(), self::ml_array::number_index(ml_reference_position176)); ml_reference_projection176 }
                        }
                    };
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value1440, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1440.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value1441, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1441.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value1442, _, _, _, _) => {
                                source_ml_source1_data_property_value1442.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value1443, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1443.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value1444, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1444.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value1445, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1445.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value1446, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1446.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value1447, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1447.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1448, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1448.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value1449, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1449.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value1450, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1450.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value1451, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1451.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value1452, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1452.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value1466, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1466.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value1467, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1467.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value1468, _, _, _, _) => {
                                    source_ml_source1_data_property_value1468.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value1469, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1469.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value1470, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1470.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value1471, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1471.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value1472, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1472.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value1473, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1473.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1474, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1474.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value1475, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1475.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value1476, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1476.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value1477, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1477.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value1478, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1478.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result {
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Done,
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Return(bool),
}

pub fn source_ml_temporary_test_java_script_heap_directory_symbol_loop11(mut symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = {
                        let source_ml_source1_array_read_value1479 = symbols.clone();
                        {
                            let source_ml_source1_array_read_index1480 = middle;
                            { let ml_reference_array177 = source_ml_source1_array_read_value1479.clone(); let ml_reference_position177 = source_ml_source1_array_read_index1480; let ml_reference_projection177 = self::ml_array::optional_at(&*ml_reference_array177.borrow(), self::ml_array::number_index(ml_reference_position177)); ml_reference_projection177 }
                        }
                    };
                    if {
                        let source_ml_source1_comparison_operand1481 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand1482 = None::<()>;
                            match (source_ml_source1_comparison_operand1481.clone(), source_ml_source1_comparison_operand1482.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Return(true)
                        } else {
                            if ((symbol.clone().expect("a proven present value")) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptArrayObjectErrorLoop13Result {
    MlTemporaryReadJavaScriptArrayObjectErrorLoop13Done,
    MlTemporaryReadJavaScriptArrayObjectErrorLoop13Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_array_object_error_loop13(mut length: f64, mut source_ml_temporary_values12: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index12: self::ml::Big) -> self::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Result {
    loop {
        return if (source_ml_temporary_index12 < ({ let ml_reference_array178 = source_ml_temporary_values12.clone(); let ml_reference_projection178 = self::ml::Big::from_u128(ml_reference_array178.borrow().len() as u128); ml_reference_projection178 })) {
            {
                let entry = {
                    let source_ml_source1_array_read_value1483 = source_ml_temporary_values12.clone();
                    {
                        let source_ml_source1_array_read_index1484 = source_ml_temporary_index12.clone();
                        { let ml_reference_array179 = source_ml_source1_array_read_value1483.clone(); let ml_reference_position179 = source_ml_source1_array_read_index1484.clone(); let ml_reference_projection179 = self::ml_array::at(&*ml_reference_array179.borrow(), ml_reference_position179.to_index()); ml_reference_projection179 }
                    }
                };
                {
                    let index = self::read_java_script_array_index(match entry.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1485, _) => {
                            let source_ml_source1_data_property_value1485 = *source_ml_source1_data_property_value1485;
                            source_ml_source1_data_property_value1485.clone()
                        }
                    });
                    if (index >= length) {
                        self::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Return(self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 98, 101, 108, 111, 119, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 110, 103, 116, 104]))
                    } else {
                        {
                            let source_ml_temporary_index12_2 = source_ml_temporary_index12.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index12 = source_ml_temporary_index12_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19 {
    MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Result {
    MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Done,
}

pub fn source_ml_temporary_plan_java_script_array_ordinary_definition_loop18(mut at: f64, mut replacement: self::JavaScriptOrdinaryProperty, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut index: f64, mut source_ml_temporary_values17: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index17: self::ml::Big) -> self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Result {
    loop {
        return if (source_ml_temporary_index17 < ({ let ml_reference_array180 = source_ml_temporary_values17.clone(); let ml_reference_projection180 = self::ml::Big::from_u128(ml_reference_array180.borrow().len() as u128); ml_reference_projection180 })) {
            {
                let entry = {
                    let source_ml_source1_array_read_value1486 = source_ml_temporary_values17.clone();
                    {
                        let source_ml_source1_array_read_index1487 = source_ml_temporary_index17.clone();
                        { let ml_reference_array181 = source_ml_source1_array_read_value1486.clone(); let ml_reference_position181 = source_ml_source1_array_read_index1487.clone(); let ml_reference_projection181 = self::ml_array::at(&*ml_reference_array181.borrow(), ml_reference_position181.to_index()); ml_reference_projection181 }
                    }
                };
                {
                    let source_ml_temporary_plan_java_script_array_ordinary_definition_ignored21 = { let ml_method_owner182 = properties.clone(); let ml_method_argument182_0 = if (index == at) {
                        replacement.clone()
                    } else {
                        entry.clone()
                    }; let ml_inserted = vec![ml_method_argument182_0]; let mut ml_method_values182 = ml_method_owner182.borrow_mut(); let ml_length = ml_method_values182.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values182.extend(ml_inserted); ml_length as f64 };
                    {
                        let index_2 = (index + 1f64);
                        {
                            let source_ml_temporary_index17_2 = source_ml_temporary_index17.add(&self::ml::Big::from_i128(1));
                            {
                                (index, source_ml_temporary_index17) = (index_2, source_ml_temporary_index17_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Result::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Done
        };
    }
}

pub fn source_ml_temporary_plan_java_script_array_length_definition_loop24(mut unsigned_length: f64, mut blocked: f64, mut source_ml_temporary_values22: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index22: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index22 < ({ let ml_reference_array183 = source_ml_temporary_values22.clone(); let ml_reference_projection183 = self::ml::Big::from_u128(ml_reference_array183.borrow().len() as u128); ml_reference_projection183 })) {
            {
                let property = {
                    let source_ml_source1_array_read_value1488 = source_ml_temporary_values22.clone();
                    {
                        let source_ml_source1_array_read_index1489 = source_ml_temporary_index22.clone();
                        { let ml_reference_array184 = source_ml_source1_array_read_value1488.clone(); let ml_reference_position184 = source_ml_source1_array_read_index1489.clone(); let ml_reference_projection184 = self::ml_array::at(&*ml_reference_array184.borrow(), ml_reference_position184.to_index()); ml_reference_projection184 }
                    }
                };
                {
                    let index = self::read_java_script_array_index(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1490, _) => {
                            let source_ml_source1_data_property_value1490 = *source_ml_source1_data_property_value1490;
                            source_ml_source1_data_property_value1490.clone()
                        }
                    });
                    if (((index >= unsigned_length) && !(({ let ml_reference_value185 = (match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1491) => {
                            source_ml_source1_data_property_value1491.clone()
                        }
                    }).borrow().clone(); ml_reference_value185 }).0)) && (index > blocked)) {
                        {
                            let blocked_2 = index;
                            {
                                let source_ml_temporary_index22_2 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (blocked, source_ml_temporary_index22) = (blocked_2, source_ml_temporary_index22_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_temporary_index22_3 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index22 = source_ml_temporary_index22_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            blocked
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Result {
    MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Done,
}

pub fn source_ml_temporary_plan_java_script_array_length_definition_loop25(mut new_writable: bool, mut final_length: f64, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_values23: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index23: self::ml::Big) -> self::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Result {
    loop {
        return if (source_ml_temporary_index23 < ({ let ml_reference_array186 = source_ml_temporary_values23.clone(); let ml_reference_projection186 = self::ml::Big::from_u128(ml_reference_array186.borrow().len() as u128); ml_reference_projection186 })) {
            {
                let property = {
                    let source_ml_source1_array_read_value1492 = source_ml_temporary_values23.clone();
                    {
                        let source_ml_source1_array_read_index1493 = source_ml_temporary_index23.clone();
                        { let ml_reference_array187 = source_ml_source1_array_read_value1492.clone(); let ml_reference_position187 = source_ml_source1_array_read_index1493.clone(); let ml_reference_projection187 = self::ml_array::at(&*ml_reference_array187.borrow(), ml_reference_position187.to_index()); ml_reference_projection187 }
                    }
                };
                {
                    let index = self::read_java_script_array_index(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1494, _) => {
                            let source_ml_source1_data_property_value1494 = *source_ml_source1_data_property_value1494;
                            source_ml_source1_data_property_value1494.clone()
                        }
                    });
                    if ((index < (0f64)) || (index < final_length)) {
                        {
                            let source_ml_temporary_plan_java_script_array_length_definition_ignored26 = { let ml_method_owner188 = properties.clone(); let ml_method_argument188_0 = if self::test_java_script_array_length_key(match property.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1495, _) => {
                                    let source_ml_source1_data_property_value1495 = *source_ml_source1_data_property_value1495;
                                    source_ml_source1_data_property_value1495.clone()
                                }
                            }) {
                                {
                                    let source_ml_source1_nominal_spread1512 = property.clone();
                                    {
                                        let source_ml_source1_nominal_field1513 = match source_ml_source1_nominal_spread1512.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1514, _) => {
                                                let source_ml_source1_data_property_value1514 = *source_ml_source1_data_property_value1514;
                                                source_ml_source1_data_property_value1514.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1515 = match source_ml_source1_nominal_spread1512.clone() {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1516) => {
                                                    source_ml_source1_data_property_value1516.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_override1517 = std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source1_spread1497 = { let ml_reference_value189 = (match property.clone() {
                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1496) => {
                                                            source_ml_source1_data_property_value1496.clone()
                                                        }
                                                    }).borrow().clone(); ml_reference_value189 };
                                                    {
                                                        let source_ml_source1_spread_field1498 = (source_ml_source1_spread1497.clone()).0;
                                                        {
                                                            let source_ml_source1_spread_field1499 = (source_ml_source1_spread1497.clone()).1;
                                                            {
                                                                let source_ml_source1_spread_field1500 = (source_ml_source1_spread1497.clone()).2;
                                                                {
                                                                    let source_ml_source1_spread_field1501 = (source_ml_source1_spread1497.clone()).3;
                                                                    {
                                                                        let source_ml_source1_spread_field1502 = (source_ml_source1_spread1497.clone()).4;
                                                                        {
                                                                            let source_ml_source1_spread_field1503 = (source_ml_source1_spread1497.clone()).5;
                                                                            {
                                                                                let source_ml_source1_spread_field1504 = (source_ml_source1_spread1497.clone()).6;
                                                                                {
                                                                                    let source_ml_source1_spread_field1505 = (source_ml_source1_spread1497.clone()).7;
                                                                                    {
                                                                                        let source_ml_source1_spread_field1506 = (source_ml_source1_spread1497.clone()).8;
                                                                                        {
                                                                                            let source_ml_source1_spread_field1507 = (source_ml_source1_spread1497.clone()).9;
                                                                                            {
                                                                                                let source_ml_source1_spread_field1508 = (source_ml_source1_spread1497.clone()).10;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field1509 = (source_ml_source1_spread1497.clone()).11;
                                                                                                    {
                                                                                                        let source_ml_source1_field1510 = self::JavaScriptSemanticValue::Number(final_length);
                                                                                                        {
                                                                                                            let source_ml_source1_field1511 = new_writable;
                                                                                                            (source_ml_source1_spread_field1498, source_ml_source1_spread_field1499, source_ml_source1_spread_field1500, source_ml_source1_spread_field1501, source_ml_source1_spread_field1502.clone(), source_ml_source1_spread_field1503, source_ml_source1_spread_field1504.clone(), source_ml_source1_spread_field1505, source_ml_source1_field1510.clone(), source_ml_source1_spread_field1507, source_ml_source1_field1511, source_ml_source1_spread_field1509)
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
                                                }));
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source1_nominal_field1513.clone()), source_ml_source1_nominal_override1517.clone())
                                            }
                                        }
                                    }
                                }
                            } else {
                                property.clone()
                            }; let ml_inserted = vec![ml_method_argument188_0]; let mut ml_method_values188 = ml_method_owner188.borrow_mut(); let ml_length = ml_method_values188.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values188.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_temporary_index23_2 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_temporary_index23 = source_ml_temporary_index23_2.clone();
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_temporary_index23_3 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index23 = source_ml_temporary_index23_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Result::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Result {
    MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Done,
}

pub fn source_ml_temporary_plan_java_script_array_property_definition_loop28(mut index: f64, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_values27: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index27: self::ml::Big) -> self::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Result {
    loop {
        return if (source_ml_temporary_index27 < ({ let ml_reference_array190 = source_ml_temporary_values27.clone(); let ml_reference_projection190 = self::ml::Big::from_u128(ml_reference_array190.borrow().len() as u128); ml_reference_projection190 })) {
            {
                let property = {
                    let source_ml_source1_array_read_value1518 = source_ml_temporary_values27.clone();
                    {
                        let source_ml_source1_array_read_index1519 = source_ml_temporary_index27.clone();
                        { let ml_reference_array191 = source_ml_source1_array_read_value1518.clone(); let ml_reference_position191 = source_ml_source1_array_read_index1519.clone(); let ml_reference_projection191 = self::ml_array::at(&*ml_reference_array191.borrow(), ml_reference_position191.to_index()); ml_reference_projection191 }
                    }
                };
                {
                    let source_ml_temporary_plan_java_script_array_property_definition_ignored29 = { let ml_method_owner192 = properties.clone(); let ml_method_argument192_0 = if self::test_java_script_array_length_key(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1520, _) => {
                            let source_ml_source1_data_property_value1520 = *source_ml_source1_data_property_value1520;
                            source_ml_source1_data_property_value1520.clone()
                        }
                    }) {
                        {
                            let source_ml_source1_nominal_spread1536 = property.clone();
                            {
                                let source_ml_source1_nominal_field1537 = match source_ml_source1_nominal_spread1536.clone() {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1538, _) => {
                                        let source_ml_source1_data_property_value1538 = *source_ml_source1_data_property_value1538;
                                        source_ml_source1_data_property_value1538.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1539 = match source_ml_source1_nominal_spread1536.clone() {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1540) => {
                                            source_ml_source1_data_property_value1540.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_override1541 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source1_spread1522 = { let ml_reference_value193 = (match property.clone() {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1521) => {
                                                    source_ml_source1_data_property_value1521.clone()
                                                }
                                            }).borrow().clone(); ml_reference_value193 };
                                            {
                                                let source_ml_source1_spread_field1523 = (source_ml_source1_spread1522.clone()).0;
                                                {
                                                    let source_ml_source1_spread_field1524 = (source_ml_source1_spread1522.clone()).1;
                                                    {
                                                        let source_ml_source1_spread_field1525 = (source_ml_source1_spread1522.clone()).2;
                                                        {
                                                            let source_ml_source1_spread_field1526 = (source_ml_source1_spread1522.clone()).3;
                                                            {
                                                                let source_ml_source1_spread_field1527 = (source_ml_source1_spread1522.clone()).4;
                                                                {
                                                                    let source_ml_source1_spread_field1528 = (source_ml_source1_spread1522.clone()).5;
                                                                    {
                                                                        let source_ml_source1_spread_field1529 = (source_ml_source1_spread1522.clone()).6;
                                                                        {
                                                                            let source_ml_source1_spread_field1530 = (source_ml_source1_spread1522.clone()).7;
                                                                            {
                                                                                let source_ml_source1_spread_field1531 = (source_ml_source1_spread1522.clone()).8;
                                                                                {
                                                                                    let source_ml_source1_spread_field1532 = (source_ml_source1_spread1522.clone()).9;
                                                                                    {
                                                                                        let source_ml_source1_spread_field1533 = (source_ml_source1_spread1522.clone()).10;
                                                                                        {
                                                                                            let source_ml_source1_spread_field1534 = (source_ml_source1_spread1522.clone()).11;
                                                                                            {
                                                                                                let source_ml_source1_field1535 = self::JavaScriptSemanticValue::Number((index + 1f64));
                                                                                                (source_ml_source1_spread_field1523, source_ml_source1_spread_field1524, source_ml_source1_spread_field1525, source_ml_source1_spread_field1526, source_ml_source1_spread_field1527.clone(), source_ml_source1_spread_field1528, source_ml_source1_spread_field1529.clone(), source_ml_source1_spread_field1530, source_ml_source1_field1535.clone(), source_ml_source1_spread_field1532, source_ml_source1_spread_field1533, source_ml_source1_spread_field1534)
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
                                        }));
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source1_nominal_field1537.clone()), source_ml_source1_nominal_override1541.clone())
                                    }
                                }
                            }
                        }
                    } else {
                        property.clone()
                    }; let ml_inserted = vec![ml_method_argument192_0]; let mut ml_method_values192 = ml_method_owner192.borrow_mut(); let ml_length = ml_method_values192.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values192.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_temporary_index27_2 = source_ml_temporary_index27.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index27 = source_ml_temporary_index27_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Result::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Done
        };
    }
}

fn ml_main() {
    assert!(((self::test_java_script_array_length(0f64)) == (true)), "assertion 1");
    assert!(((self::test_java_script_array_length(4294967295f64)) == (true)), "assertion 2");
    assert!(((self::test_java_script_array_length(4294967296f64)) == (false)), "assertion 3");
    assert!(((self::test_java_script_array_length((-0f64))) == (false)), "assertion 4");
    assert!(((self::test_java_script_array_length(f64::NAN)) == (false)), "assertion 5");
    assert!(self::ml_number::same_value(self::read_java_script_array_index(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![48f64])))), 0f64), "assertion 6");
    assert!(self::ml_number::same_value(self::read_java_script_array_index(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![52, 50, 57, 52, 57, 54, 55, 50, 57, 52]))), 4294967294f64), "assertion 7");
    assert!(self::ml_number::same_value(self::read_java_script_array_index(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![52, 50, 57, 52, 57, 54, 55, 50, 57, 53]))), (-1f64)), "assertion 8");
    assert!(self::ml_number::same_value(self::read_java_script_array_index(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![48, 49]))), (-1f64)), "assertion 9");
    assert!(self::ml_number::same_value(self::convert_java_script_array_unsigned_length((-1f64)), 4294967295f64), "assertion 10");
    assert!(self::ml_number::same_value(self::convert_java_script_array_unsigned_length((-0f64)), 0f64), "assertion 11");
    assert!(self::ml_number::same_value(self::convert_java_script_array_unsigned_length(f64::INFINITY), 0f64), "assertion 12");
    assert!(self::ml_number::same_value(self::convert_java_script_array_unsigned_length(4294967297f64), 1f64), "assertion 13");
    assert!(((self::test_java_script_array_length_key(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))) == (true)), "assertion 14");
    assert!(((self::test_java_script_array_length_key(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![48f64]))))) == (false)), "assertion 15");
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
fn all_literal_array_object_controls() { main(); }
