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

pub mod ml_string_number {
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

use self::ml;

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
pub fn read_java_script_numeric_digit_unit(unit: f64) -> f64 {
    if ((unit >= (48f64)) && (unit <= (57f64))) {
        (unit - 48f64)
    } else {
        if ((unit >= (65f64)) && (unit <= (70f64))) {
            (unit - 55f64)
        } else {
            if ((unit >= (97f64)) && (unit <= (102f64))) {
                (unit - 87f64)
            } else {
                (-1f64)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_utf8_string_units(bytes: &[f64]) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s1 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop1(bytes, index);
            match source_ml_source2_s1.clone() {
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s2 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop2(bytes, units, index_2);
                                match source_ml_source2_s2.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Done(units_2) => {
                                        units_2
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_source1_temporary_result2) => {
                                        source_ml_source1_temporary_result2
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Return(source_ml_source1_temporary_result4) => {
                    source_ml_source1_temporary_result4
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_digit(unit: f64) -> f64 {
    self::read_java_script_numeric_digit_unit(unit)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_decimal_power(exponent: f64) -> self::ml::Big {
    {
        let value = self::ml::Big::from_i128(1);
        {
            let base = self::ml::Big::from_i128(10);
            {
                let value_2 = self::source_ml_source1_temporary_read_string_number_decimal_power_loop5(exponent, value.clone(), base.clone());
                value_2.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_exact_mantissa(value: self::ml::Big) -> f64 {
    {
        let result = 0f64;
        {
            let scale = 1f64;
            {
                let result_2 = self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop6(value.clone(), result, scale);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop7(exponent, result);
            match source_ml_source2_s3.clone() {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Done(exponent_2, result_2) => {
                    {
                        let result_3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop8(exponent_2, result_2);
                        result_3
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_string_number_rational(numerator: self::ml::Big, denominator: self::ml::Big) -> f64 {
    if ((numerator < (self::ml::Big::from_i128(0))) || (denominator <= (self::ml::Big::from_i128(0)))) {
        f64::NAN
    } else {
        if (numerator == (self::ml::Big::from_i128(0))) {
            0f64
        } else {
            {
                let exponent = 0f64;
                {
                    let source_ml_source2_s6 = if (numerator >= denominator) {
                        {
                            let scale = denominator.clone();
                            {
                                let source_ml_source2_s7 = self::source_ml_source1_temporary_convert_string_number_rational_loop15(numerator.clone(), exponent, scale.clone());
                                match source_ml_source2_s7.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Done(exponent_2) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Next(exponent_2)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Return(source_ml_source1_temporary_result15) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Return(source_ml_source1_temporary_result15)
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let scale_2 = numerator.clone();
                            {
                                let source_ml_source2_s8 = self::source_ml_source1_temporary_convert_string_number_rational_loop16(denominator.clone(), exponent, scale_2.clone());
                                match source_ml_source2_s8.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop16Result::MlSource1TemporaryConvertStringNumberRationalLoop16Done(exponent_3) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Next(exponent_3)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop16Result::MlSource1TemporaryConvertStringNumberRationalLoop16Return(source_ml_source1_temporary_result16) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Return(source_ml_source1_temporary_result16)
                                    }
                                }
                            }
                        }
                    };
                    match source_ml_source2_s6.clone() {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Next(exponent_4) => {
                            {
                                let shift = if (exponent_4 >= ((-1022f64))) {
                                    (52f64 - exponent_4)
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s9 = if (shift >= (0f64)) {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop13(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin10::MlSource1TemporaryConvertStringNumberRationalJoin10Next(numerator_2.clone(), denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop14(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin10::MlSource1TemporaryConvertStringNumberRationalJoin10Next(numerator.clone(), denominator_2.clone())
                                            }
                                        }
                                    };
                                    match source_ml_source2_s9.clone() {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin10::MlSource1TemporaryConvertStringNumberRationalJoin10Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale_3 = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s10 = self::source_ml_source1_temporary_convert_string_number_rational_loop11(remainder.clone(), scale_3.clone(), weight.clone());
                                                                match source_ml_source2_s10.clone() {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop11Result::MlSource1TemporaryConvertStringNumberRationalLoop11Done(scale_4, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s11 = self::source_ml_source1_temporary_convert_string_number_rational_loop12(quotient.clone(), remainder.clone(), scale_4.clone(), weight_2.clone());
                                                                            match source_ml_source2_s11.clone() {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Done(quotient_2, remainder_2) => {
                                                                                    {
                                                                                        let twice = remainder_2.mul(&self::ml::Big::from_i128(2));
                                                                                        {
                                                                                            let quotient_4 = if ((twice > denominator_3) || ((twice == denominator_3) && ((self::ml::divide(&quotient_2, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))))) {
                                                                                                {
                                                                                                    let quotient_3 = quotient_2.add(&self::ml::Big::from_i128(1));
                                                                                                    quotient_3.clone()
                                                                                                }
                                                                                            } else {
                                                                                                quotient_2.clone()
                                                                                            };
                                                                                            (self::read_string_number_exact_mantissa(quotient_4.clone()) * self::read_string_number_binary_power((-shift)))
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
                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Return(source_ml_source1_temporary_result14) => {
                            source_ml_source1_temporary_result14
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_string_number_infinity(units: &[f64], start: f64, end: f64) -> bool {
    if (((end - start)) != (8f64)) {
        false
    } else {
        {
            let expected = vec![73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64];
            {
                let index = 0f64;
                {
                    let source_ml_source2_s12 = self::source_ml_source1_temporary_accept_string_number_infinity_loop17(units, start, &(expected), index);
                    match source_ml_source2_s12.clone() {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result::MlSource1TemporaryAcceptStringNumberInfinityLoop17Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result::MlSource1TemporaryAcceptStringNumberInfinityLoop17Return(source_ml_source1_temporary_result17) => {
                            source_ml_source1_temporary_result17
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_string_number_units(units: &[f64]) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s13 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop18(units, index);
            match source_ml_source2_s13.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop19(units, start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop20(units, start_2, end);
                                    if (start_2 == end_2) {
                                        0f64
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = (self::ml_array::optional_at(units, self::ml_array::number_index(start_2))).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let source_ml_source2_s14 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = (start_2 + 1f64);
                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s14.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(start_4, negative_3, signed_3) => {
                                                                if (start_4 == end_2) {
                                                                    f64::NAN
                                                                } else {
                                                                    if self::accept_string_number_infinity(units, start_4, end_2) {
                                                                        if negative_3 {
                                                                            f64::NEG_INFINITY
                                                                        } else {
                                                                            f64::INFINITY
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let integer_digits = vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)];
                                                                            {
                                                                                let source_ml_source2_s16 = if ((!signed_3 && (((self::ml_array::optional_at(units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
                                                                                    {
                                                                                        let marker = (self::ml_array::optional_at(units, self::ml_array::number_index((start_4 + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                                                                                        {
                                                                                            let radix = if ((marker == (120f64)) || (marker == (88f64))) {
                                                                                                16f64
                                                                                            } else {
                                                                                                if ((marker == (98f64)) || (marker == (66f64))) {
                                                                                                    2f64
                                                                                                } else {
                                                                                                    if ((marker == (111f64)) || (marker == (79f64))) {
                                                                                                        8f64
                                                                                                    } else {
                                                                                                        0f64
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            if (radix != (0f64)) {
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let overflow = false;
                                                                                                        {
                                                                                                            let limit = self::read_string_number_decimal_power(310f64);
                                                                                                            {
                                                                                                                let base = if (radix == (16f64)) {
                                                                                                                    self::ml::Big::from_i128(16)
                                                                                                                } else {
                                                                                                                    if (radix == (8f64)) {
                                                                                                                        self::ml::Big::from_i128(8)
                                                                                                                    } else {
                                                                                                                        self::ml::Big::from_i128(2)
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let start_5 = (start_4 + 2f64);
                                                                                                                    if (start_5 == end_2) {
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let index_2 = start_5;
                                                                                                                            {
                                                                                                                                let source_ml_source2_s17 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop29(units, end_2, integer_digits.clone(), radix, value.clone(), overflow, limit.clone(), base.clone(), index_2);
                                                                                                                                match source_ml_source2_s17.clone() {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done(value_2, overflow_2) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(if overflow_2 {
                                                                                                                                            f64::INFINITY
                                                                                                                                        } else {
                                                                                                                                            self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                        })
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(source_ml_source1_temporary_result29) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(source_ml_source1_temporary_result29)
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(start_4)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s16.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(start_6) => {
                                                                                        {
                                                                                            let at = start_6;
                                                                                            {
                                                                                                let fraction = 0f64;
                                                                                                {
                                                                                                    let digits = 0f64;
                                                                                                    {
                                                                                                        let significant = 0f64;
                                                                                                        {
                                                                                                            let dot = false;
                                                                                                            {
                                                                                                                let source_ml_source2_s18 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop23(units, end_2, at, fraction, digits, significant, dot);
                                                                                                                match source_ml_source2_s18.clone() {
                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Done(at_2, fraction_2, digits_2, significant_2) => {
                                                                                                                        if (digits_2 == (0f64)) {
                                                                                                                            f64::NAN
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let decimal_end = at_2;
                                                                                                                                {
                                                                                                                                    let exponent = 0f64;
                                                                                                                                    {
                                                                                                                                        let exponent_limit = ((units.len() as f64) + 10000f64);
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s21 = if ((at_2 < end_2) && ((((self::ml_array::optional_at(units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (101f64)) || (((self::ml_array::optional_at(units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (69f64)))) {
                                                                                                                                                {
                                                                                                                                                    let at_3 = (at_2 + 1f64);
                                                                                                                                                    {
                                                                                                                                                        let exponent_negative = false;
                                                                                                                                                        {
                                                                                                                                                            let exponent_sign = (self::ml_array::optional_at(units, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| (-1f64));
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_s22 = if ((exponent_sign == (43f64)) || (exponent_sign == (45f64))) {
                                                                                                                                                                    {
                                                                                                                                                                        let exponent_negative_2 = (exponent_sign == (45f64));
                                                                                                                                                                        {
                                                                                                                                                                            let at_4 = (at_3 + 1f64);
                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_4, exponent_negative_2)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_3, exponent_negative)
                                                                                                                                                                };
                                                                                                                                                                match source_ml_source2_s22.clone() {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_5, exponent_negative_3) => {
                                                                                                                                                                        {
                                                                                                                                                                            let first = at_5;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_s23 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop28(units, end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                                match source_ml_source2_s23.clone() {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(at_6, exponent_2) => {
                                                                                                                                                                                        if (at_6 == first) {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Return(f64::NAN)
                                                                                                                                                                                        } else {
                                                                                                                                                                                            if exponent_negative_3 {
                                                                                                                                                                                                {
                                                                                                                                                                                                    let exponent_3 = (-exponent_2);
                                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_6, exponent_3)
                                                                                                                                                                                                }
                                                                                                                                                                                            } else {
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_6, exponent_2)
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
                                                                                                                                            } else {
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_2, exponent)
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s21.clone() {
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_7, exponent_4) => {
                                                                                                                                                    if (at_7 != end_2) {
                                                                                                                                                        f64::NAN
                                                                                                                                                    } else {
                                                                                                                                                        if (significant_2 == (0f64)) {
                                                                                                                                                            if negative_3 {
                                                                                                                                                                (-0f64)
                                                                                                                                                            } else {
                                                                                                                                                                0f64
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            {
                                                                                                                                                                let exponent_5 = (exponent_4 - fraction_2);
                                                                                                                                                                {
                                                                                                                                                                    let order = (significant_2 + exponent_5);
                                                                                                                                                                    if (order > (310f64)) {
                                                                                                                                                                        if negative_3 {
                                                                                                                                                                            f64::NEG_INFINITY
                                                                                                                                                                        } else {
                                                                                                                                                                            f64::INFINITY
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        if (order < ((-325f64))) {
                                                                                                                                                                            if negative_3 {
                                                                                                                                                                                (-0f64)
                                                                                                                                                                            } else {
                                                                                                                                                                                0f64
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            {
                                                                                                                                                                                let mantissa_end = decimal_end;
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_s24 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop25(units, start_6, exponent_5, mantissa_end);
                                                                                                                                                                                    match source_ml_source2_s24.clone() {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let index_3 = start_6;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop26(units, integer_digits, mantissa_end_2, mantissa.clone(), index_3);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let numerator = if (exponent_6 >= (0f64)) {
                                                                                                                                                                                                                mantissa_2.mul(&self::read_string_number_decimal_power(exponent_6))
                                                                                                                                                                                                            } else {
                                                                                                                                                                                                                mantissa_2.clone()
                                                                                                                                                                                                            };
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let denominator = if (exponent_6 < (0f64)) {
                                                                                                                                                                                                                    self::read_string_number_decimal_power((-exponent_6))
                                                                                                                                                                                                                } else {
                                                                                                                                                                                                                    self::ml::Big::from_i128(1)
                                                                                                                                                                                                                };
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let value_3 = self::convert_string_number_rational(numerator.clone(), denominator.clone());
                                                                                                                                                                                                                    if negative_3 {
                                                                                                                                                                                                                        (-value_3)
                                                                                                                                                                                                                    } else {
                                                                                                                                                                                                                        value_3
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Return(source_ml_source1_temporary_result26) => {
                                                                                                                                                    source_ml_source1_temporary_result26
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
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(source_ml_source1_temporary_result28) => {
                                                                                        source_ml_source1_temporary_result28
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
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Return(source_ml_source1_temporary_result29_2) => {
                    source_ml_source1_temporary_result29_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_big_integer_number(value: self::ml::Big) -> f64 {
    if (value < (self::ml::Big::from_i128(0))) {
        (-(self::convert_string_number_rational(value.neg(), self::ml::Big::from_i128(1))))
    } else {
        self::convert_string_number_rational(value.clone(), self::ml::Big::from_i128(1))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_utf8_string_number_units(bytes: &[f64]) -> f64 {
    self::convert_java_script_string_number_units(&(self::read_java_script_utf8_string_units(bytes)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_builtin_operation(name: &str) -> String {
    if (name == "Number") {
        String::from("convertJavaScriptNumber")
    } else {
        if (name == "isFinite") {
            String::from("testCoercedJavaScriptNumberFinite")
        } else {
            if (name == "isNaN") {
                String::from("testCoercedJavaScriptNumberNaN")
            } else {
                String::from("")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_result(operation: &str) -> String {
    if ((operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber")) {
        String::from("float")
    } else {
        if ((operation == "testCoercedJavaScriptNumberFinite") || (operation == "testCoercedJavaScriptNumberNaN")) {
            String::from("bool")
        } else {
            String::from("")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_form(kind: &str) -> String {
    if (((((((kind == "float") || (kind == "bool")) || (kind == "string")) || (kind == "unit")) || (kind == "int")) || (kind == "nat")) || (kind == "option")) {
        kind.to_owned()
    } else {
        String::from("unsupported")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_java_script_number_conversion(kind: &str, operation: &str) -> bool {
    ((kind == "math") && ((operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_failure(operation: &str, kind: &str) -> String {
    if ((operation == "coerceJavaScriptNumber") && ((kind == "int") || (kind == "nat"))) {
        String::from("Cannot convert a BigInt value to a number")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_number_conversion_units(kind: &str, operation: &str, argument: &str) -> bool {
    (self::accept_linked_java_script_number_conversion(kind, operation) && (argument == "string"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_number_operand(language: &str, kind: &str, element: &str) -> String {
    if (language != "JavaScript") {
        String::from("")
    } else {
        if (((kind == "bool") || (kind == "string")) || (kind == "unit")) {
            String::from("coerceJavaScriptNumber")
        } else {
            if ((kind == "option") && (element == "string")) {
                String::from("coerceJavaScriptNumber")
            } else {
                String::from("")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_numeric_inference_kind(kind: &str) -> String {
    if (((kind == "bool") || (kind == "string")) || (kind == "unit")) {
        String::from("float")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_unary_operation(symbol: &str) -> String {
    if (symbol == "+") {
        String::from("coerceJavaScriptNumber")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_mathematical_conversion_form(language: &str, operation: &str) -> String {
    if (language != "JavaScript") {
        String::from("")
    } else {
        if (operation == "powerJavaScriptNumberArguments") {
            String::from("binary")
        } else {
            if ((operation == "max") || (operation == "min")) {
                String::from("variadic")
            } else {
                if (((((((operation == "abs") || (operation == "floor")) || (operation == "ceil")) || (operation == "trunc")) || (operation == "round")) || (operation == "sign")) || (operation == "sqrt")) {
                    String::from("unary")
                } else {
                    String::from("")
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_number_scalar_conversion(format: &str, operation: &str, form: &str, value: &str, utf16: bool) -> String {
    if ((!(self::accept_linked_java_script_number_conversion("math", operation)) || ((self::read_java_script_number_conversion_form(form)) == "unsupported")) || (form == "option")) {
        String::from("")
    } else {
        if (format == "JavaScript") {
            if (operation == "convertJavaScriptNumber") {
                format!("{}{}", (format!("{}{}", (String::from("Number(")), value.to_owned())), (String::from(")")))
            } else {
                format!("{}{}", (format!("{}{}", (String::from("(+(")), value.to_owned())), (String::from("))")))
            }
        } else {
            if ((format == "Lean") || (format == "Rocq")) {
                if !(self::read_java_script_number_conversion_failure(operation, form)).is_empty() {
                    String::from("")
                } else {
                    if (form == "float") {
                        value.to_owned()
                    } else {
                        if (form == "unit") {
                            if (format == "Lean") {
                                format!("{}{}", (format!("{}{}", (String::from("(let _ := ")), value.to_owned())), (String::from("; ((0 : Float) / 0))")))
                            } else {
                                format!("{}{}", (format!("{}{}", (String::from("(let _ := ")), value.to_owned())), (String::from(" in PrimFloat.nan)")))
                            }
                        } else {
                            if (form == "bool") {
                                if (format == "Lean") {
                                    format!("{}{}", (format!("{}{}", (String::from("(if ")), value.to_owned())), (String::from(" then (1 : Float) else (0 : Float))")))
                                } else {
                                    format!("{}{}", (format!("{}{}", (String::from("(if ")), value.to_owned())), (String::from(" then 1%float else 0%float)")))
                                }
                            } else {
                                if ((form == "int") || (form == "nat")) {
                                    format!("{}{}", (format!("{}{}", (String::from("(MLStringNumber.convertInteger ")), (if (form == "nat") {
                                        if (format == "Lean") {
                                            format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat ")), value.to_owned())), (String::from(")")))
                                        } else {
                                            format!("{}{}", (format!("{}{}", (String::from("(Z.of_N ")), value.to_owned())), (String::from(")")))
                                        }
                                    } else {
                                        value.to_owned()
                                    }))), (String::from(")")))
                                } else {
                                    if (format == "Lean") {
                                        format!("{}{}", (format!("{}{}", (String::from("(MLStringNumber.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] ")), value.to_owned())), (String::from("))")))
                                    } else {
                                        format!("{}{}", (format!("{}{}", (String::from("(MLStringNumber.convertUtf8 ((fix ml_numeric_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_numeric_bytes rest) end) ")), value.to_owned())), (String::from("))")))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if (format != "Rust") {
                    String::from("")
                } else {
                    if (form == "float") {
                        value.to_owned()
                    } else {
                        if (form == "bool") {
                            format!("{}{}", (format!("{}{}", (String::from("(if ")), value.to_owned())), (String::from(" { 1.0f64 } else { 0.0f64 })")))
                        } else {
                            if (form == "unit") {
                                format!("{}{}", (format!("{}{}", (String::from("{ let _ = ")), value.to_owned())), (String::from("; f64::NAN }")))
                            } else {
                                if ((form == "int") || (form == "nat")) {
                                    if (operation == "coerceJavaScriptNumber") {
                                        format!("{}{}", (format!("{}{}", (String::from("{ let _ = ")), value.to_owned())), (String::from("; crate::ml_source_exception::SourceException::from_text(\"TypeError\", \"Cannot convert a BigInt value to a number\").raise() }")))
                                    } else {
                                        format!("{}{}", (format!("{}{}", (String::from("crate::ml_string_number::convert_java_script_big_integer_number(")), value.to_owned())), (String::from(")")))
                                    }
                                } else {
                                    {
                                        let units = if utf16 {
                                            format!("{}{}", (format!("{}{}", (String::from("(")), value.to_owned())), (String::from(").0.iter().copied()")))
                                        } else {
                                            format!("{}{}", (format!("{}{}", (String::from("(")), value.to_owned())), (String::from(").encode_utf16()")))
                                        };
                                        format!("{}{}", (format!("{}{}", (String::from("crate::ml_string_number::convert_java_script_string_number_units(&")), units)), (String::from(".map(f64::from).collect::<Vec<f64>>())")))
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
pub fn read_java_script_number_conversion_statement() -> String {
    String::from("ECMAScript Number and ToNumber preserve full UTF-16 StringNumericLiteral syntax, exact binary64 ties-to-even rounding, signed zero, Infinity, NaN, primitive absence and argument evaluation order; explicit BigInt conversion rounds once and implicit BigInt conversion throws")
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Done(Vec<f64>),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done(f64, f64),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(bytes: &[f64], mut index: f64, mut scalar: f64, mut remaining: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    loop {
        return if (remaining > (0f64)) {
            {
                let next = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((next < (128f64)) || (next > (191f64))) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(vec![(-1f64)])
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
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done(index, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3 {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(f64, f64, f64),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop2(bytes: &[f64], mut units: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result {
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
                                let source_ml_source2_s25 = if ((scalar >= (194f64)) && (scalar <= (223f64))) {
                                    {
                                        let scalar_2 = (scalar - 192f64);
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_2, remaining_2, minimum_2)
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
                                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_3, remaining_3, minimum_3)
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
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_4, remaining_4, minimum_4)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (scalar >= (128f64)) {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Return(vec![(-1f64)])
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar, remaining, minimum)
                                            }
                                        }
                                    }
                                };
                                match source_ml_source2_s25.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source2_s26 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(bytes, index_2, scalar_5, remaining_5);
                                            match source_ml_source2_s26.clone() {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done(index_3, scalar_6) => {
                                                    if (((scalar_6 < minimum_5) || (scalar_6 > (1114111f64))) || ((scalar_6 >= (55296f64)) && (scalar_6 <= (57343f64)))) {
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Return(vec![(-1f64)])
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
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(source_ml_source1_temporary_result4) => {
                                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_source1_temporary_result4)
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Return(source_ml_source1_temporary_result4_2) => {
                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_source1_temporary_result4_2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Done(units)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Done,
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop1(bytes: &[f64], mut index: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (255f64))) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Return(vec![(-1f64)])
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
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop5(mut exponent: f64, mut value: self::ml::Big, mut base: self::ml::Big) -> self::ml::Big {
    loop {
        return if (exponent > (0f64)) {
            {
                let value_3 = if (((exponent % 2f64)) == (1f64)) {
                    {
                        let value_2 = value.mul(&base);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let exponent_2 = ((exponent / 2f64)).floor();
                    if (exponent_2 > (0f64)) {
                        {
                            let base_2 = base.mul(&base);
                            {
                                (exponent, value, base) = (exponent_2, value_3.clone(), base_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (exponent, value) = (exponent_2, value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop6(mut value: self::ml::Big, mut result: f64, mut scale: f64) -> f64 {
    loop {
        return if (value > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let result_2 = (result + scale);
                        result_2
                    }
                } else {
                    result
                };
                {
                    let value_2 = self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let scale_2 = (scale * 2f64);
                        {
                            (value, result, scale) = (value_2.clone(), result_3, scale_2);
                            continue;
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
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop8(mut exponent: f64, mut result: f64) -> f64 {
    loop {
        return if (exponent < (0f64)) {
            {
                let result_2 = (result / 2f64);
                {
                    let exponent_2 = (exponent + 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
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
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop7Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop7(mut exponent: f64, mut result: f64) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result {
    loop {
        return if (exponent > (0f64)) {
            {
                let result_2 = (result * 2f64);
                {
                    let exponent_2 = (exponent - 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Done(exponent, result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop12Result {
    MlSource1TemporaryConvertStringNumberRationalLoop12Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop12(mut quotient: self::ml::Big, mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop12Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let scale_2 = self::ml::divide(&scale, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = self::ml::divide(&weight, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remainder >= scale_2) {
                        {
                            let remainder_2 = remainder.sub(&scale_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (quotient, remainder, scale, weight) = (quotient_2.clone(), remainder_2.clone(), scale_2.clone(), weight_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            (scale, weight) = (scale_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop11Result {
    MlSource1TemporaryConvertStringNumberRationalLoop11Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop11(mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop11Result {
    loop {
        return if (scale <= remainder) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (scale, weight) = (scale_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop11Result::MlSource1TemporaryConvertStringNumberRationalLoop11Done(scale.clone(), weight.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin10 {
    MlSource1TemporaryConvertStringNumberRationalJoin10Next(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop13(mut numerator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < shift) {
            {
                let numerator_2 = numerator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (numerator, index) = (numerator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            numerator.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop14(mut denominator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < ((-shift))) {
            {
                let denominator_2 = denominator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (denominator, index) = (denominator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            denominator.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin9 {
    MlSource1TemporaryConvertStringNumberRationalJoin9Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin9Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop15Result {
    MlSource1TemporaryConvertStringNumberRationalLoop15Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop15Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop15(mut numerator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop15Result {
    loop {
        return if (numerator >= (scale.mul(&self::ml::Big::from_i128(2)))) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent + 1f64);
                    if (exponent_2 >= (1024f64)) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Return(f64::INFINITY)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop16Result {
    MlSource1TemporaryConvertStringNumberRationalLoop16Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop16Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop16(mut denominator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop16Result {
    loop {
        return if (scale < denominator) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent - 1f64);
                    if (exponent_2 < ((-1075f64))) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop16Result::MlSource1TemporaryConvertStringNumberRationalLoop16Return(0f64)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop16Result::MlSource1TemporaryConvertStringNumberRationalLoop16Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptStringNumberInfinityLoop17Result {
    MlSource1TemporaryAcceptStringNumberInfinityLoop17Done,
    MlSource1TemporaryAcceptStringNumberInfinityLoop17Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_string_number_infinity_loop17(units: &[f64], mut start: f64, expected: &[f64], mut index: f64) -> self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result {
    loop {
        return if (index < ((expected.len() as f64))) {
            if (((self::ml_array::optional_at(units, self::ml_array::number_index((start + index)))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(expected, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-2f64)))) {
                self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result::MlSource1TemporaryAcceptStringNumberInfinityLoop17Return(false)
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
            self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result::MlSource1TemporaryAcceptStringNumberInfinityLoop17Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop19(units: &[f64], mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((start < end) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(units, self::ml_array::number_index(start))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let start_2 = (start + 1f64);
                {
                    start = start_2;
                    continue;
                }
            }
        } else {
            start
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop20(units: &[f64], mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((end > start) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(units, self::ml_array::number_index((end - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let end_2 = (end - 1f64);
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop26(units: &[f64], mut integer_digits: Vec<self::ml::Big>, mut mantissa_end: f64, mut mantissa: self::ml::Big, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < mantissa_end) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if (unit != (46f64)) {
                    {
                        let mantissa_2 = (mantissa.mul(&self::ml::Big::from_i128(10))).add(&(self::ml_array::optional_at(&integer_digits, self::ml_array::number_index((unit - 48f64)))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (mantissa, index) = (mantissa_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let index_3 = (index + 1f64);
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            mantissa.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop25(units: &[f64], mut start: f64, mut exponent: f64, mut mantissa_end: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result {
    loop {
        return if (mantissa_end > start) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index((mantissa_end - 1f64)))).clone().unwrap_or_else(|| (-1f64));
                if (unit == (46f64)) {
                    {
                        let mantissa_end_2 = (mantissa_end - 1f64);
                        {
                            mantissa_end = mantissa_end_2;
                            continue;
                        }
                    }
                } else {
                    if (unit != (48f64)) {
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(exponent, mantissa_end)
                    } else {
                        {
                            let mantissa_end_3 = (mantissa_end - 1f64);
                            {
                                let exponent_2 = (exponent + 1f64);
                                {
                                    (exponent, mantissa_end) = (exponent_2, mantissa_end_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(exponent, mantissa_end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(f64, f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop28(units: &[f64], mut end: f64, mut at: f64, mut exponent: f64, mut exponent_limit: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result {
    loop {
        return if (at < end) {
            {
                let digit = ((self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)) - 48f64);
                if ((digit < (0f64)) || (digit > (9f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(at, exponent)
                } else {
                    {
                        let exponent_3 = if (exponent < exponent_limit) {
                            {
                                let exponent_2 = self::ml_math::min(exponent_limit, ((exponent * 10f64) + digit));
                                exponent_2
                            }
                        } else {
                            exponent
                        };
                        {
                            let at_2 = (at + 1f64);
                            {
                                (at, exponent) = (at_2, exponent_3);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(at, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Done(f64, f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop23(units: &[f64], mut end: f64, mut at: f64, mut fraction: f64, mut digits: f64, mut significant: f64, mut dot: bool) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result {
    loop {
        return if (at < end) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((unit >= (48f64)) && (unit <= (57f64))) {
                    {
                        let digits_2 = (digits + 1f64);
                        {
                            let fraction_3 = if dot {
                                {
                                    let fraction_2 = (fraction + 1f64);
                                    fraction_2
                                }
                            } else {
                                fraction
                            };
                            {
                                let significant_3 = if ((unit != (48f64)) || (significant > (0f64))) {
                                    {
                                        let significant_2 = (significant + 1f64);
                                        significant_2
                                    }
                                } else {
                                    significant
                                };
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        (at, fraction, digits, significant) = (at_2, fraction_3, digits_2, significant_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if ((unit == (46f64)) && !dot) {
                        {
                            let dot_2 = true;
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (at, dot) = (at_3, dot_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Done(at, fraction, digits, significant)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Done(at, fraction, digits, significant)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done(self::ml::Big, bool),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop29(units: &[f64], mut end: f64, mut integer_digits: Vec<self::ml::Big>, mut radix: f64, mut value: self::ml::Big, mut overflow: bool, mut limit: self::ml::Big, mut base: self::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_string_number_digit((self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(f64::NAN)
                } else {
                    if !overflow {
                        {
                            let value_2 = (value.mul(&base)).add(&(self::ml_array::optional_at(&integer_digits, self::ml_array::number_index(digit))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
                            {
                                let overflow_2 = (value_2 > limit);
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        (value, overflow, index) = (value_2.clone(), overflow_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let index_3 = (index + 1f64);
                            {
                                index = index_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done(value.clone(), overflow)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Done,
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop18(units: &[f64], mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result {
    loop {
        return if (index < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Done
        };
    }
}
}


pub fn find_java_script_class_group_end(values: Vec<self::ml_string::Text>, open: f64) -> f64 {
    {
        let first = (self::ml_array::optional_at(&values, self::ml_array::number_index(open))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        {
            let last = if (first == (self::ml_string::Text::from_units(vec![40]))) {
                self::ml_string::Text::from_units(vec![41])
            } else {
                if (first == (self::ml_string::Text::from_units(vec![91]))) {
                    self::ml_string::Text::from_units(vec![93])
                } else {
                    if (first == (self::ml_string::Text::from_units(vec![123]))) {
                        self::ml_string::Text::from_units(vec![125])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            };
            if (last == (self::ml_string::Text::from_units(vec![]))) {
                (-1f64)
            } else {
                {
                    let expected = vec![last.clone()];
                    {
                        let at = (open + 1f64);
                        {
                            let source_ml_source2_s1 = self::source_ml_source1_temporary_find_java_script_class_group_end_loop1(values, open, expected, at);
                            match source_ml_source2_s1.clone() {
                                self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Done => {
                                    (-1f64)
                                }
                                self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return(source_ml_source1_temporary_result1) => {
                                    source_ml_source1_temporary_result1
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_parameter_positions(values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>, open: f64, close: f64) -> Vec<f64> {
    self::read_java_script_closed_class_parameter_positions_with_collections(values, kinds, open, close, false)
}

pub fn read_java_script_closed_class_collection_parameter_positions(values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>, open: f64, close: f64) -> Vec<f64> {
    self::read_java_script_closed_class_parameter_positions_with_collections(values, kinds, open, close, true)
}

pub fn read_java_script_closed_class_parameter_positions_with_collections(values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>, open: f64, close: f64, collections: bool) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = (open + 1f64);
            {
                let source_ml_source2_s2 = self::source_ml_source1_temporary_read_java_script_closed_class_parameter_positions_with_collections_loop2(values, kinds, close, collections, result, at);
                match source_ml_source2_s2.clone() {
                    self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Done(result_2) => {
                        result_2
                    }
                    self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(source_ml_source1_temporary_result2) => {
                        source_ml_source1_temporary_result2
                    }
                }
            }
        }
    }
}

pub fn render_java_script_closed_class_parameter_arguments(source: self::ml_string::Text, values: Vec<self::ml_string::Text>, starts: Vec<f64>, ends: Vec<f64>, open: f64, close: f64) -> self::ml_string::Text {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let at = (open + 1f64);
            {
                let result_2 = self::source_ml_source1_temporary_render_java_script_closed_class_parameter_arguments_loop4(source.clone(), values, starts, ends, close, result, at);
                self::ml_string::join(&result_2, &self::ml_string::Text::from_units(vec![44, 32]))
            }
        }
    }
}

pub fn find_java_script_closed_class_initialization_start(values: Vec<self::ml_string::Text>, first: f64, last: f64) -> f64 {
    {
        let depth = 0f64;
        {
            let at = first;
            {
                let source_ml_source2_s3 = self::source_ml_source1_temporary_find_java_script_closed_class_initialization_start_loop5(values, first, last, depth, at);
                match source_ml_source2_s3.clone() {
                    self::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Result::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Done => {
                        last
                    }
                    self::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Result::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Return(source_ml_source1_temporary_result5) => {
                        source_ml_source1_temporary_result5
                    }
                }
            }
        }
    }
}

pub fn find_java_script_closed_class_field_end(values: Vec<self::ml_string::Text>, first: f64, last: f64) -> f64 {
    if (((((!(self::ml_math::is_safe_integer(first)) || !(self::ml_math::is_safe_integer(last))) || (first < (0f64))) || (last < first)) || (last > ((values.len() as f64)))) || (((values.len() as f64)) > (8192f64))) {
        (-1f64)
    } else {
        {
            let at = first;
            {
                let source_ml_source2_s4 = self::source_ml_source1_temporary_find_java_script_closed_class_field_end_loop6(values, last, at);
                match source_ml_source2_s4.clone() {
                    self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Return(source_ml_source1_temporary_result6) => {
                        source_ml_source1_temporary_result6
                    }
                }
            }
        }
    }
}

pub fn render_java_script_closed_class_field_documentation(comment: self::ml_string::Text) -> self::ml_string::Text {
    {
        let result = self::ml_string::Text::from_units(vec![]);
        {
            let previous = 0f64;
            {
                let at = {
                    let source_ml_source2_string_search_operand5 = comment.clone();
                    {
                        let source_ml_source2_string_search_operand6 = self::ml_string::Text::from_units(vec![64, 116, 121, 112, 101]);
                        {
                            let source_ml_source2_string_search_text7 = source_ml_source2_string_search_operand6.clone();
                            {
                                let source_ml_source2_string_search_position8 = 0f64;
                                source_ml_source2_string_search_operand5.index_of(&source_ml_source2_string_search_text7, source_ml_source2_string_search_position8)
                            }
                        }
                    }
                };
                {
                    let source_ml_source2_s9 = self::source_ml_source1_temporary_render_java_script_closed_class_field_documentation_loop8(comment.clone(), result.clone(), previous, at);
                    match source_ml_source2_s9.clone() {
                        self::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Result::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Done(result_2, previous_2) => {
                            result_2.concat(&{
                                let source_ml_source2_string_range_operand10 = comment.clone();
                                {
                                    let source_ml_source2_string_range_operand11 = previous_2;
                                    {
                                        let source_ml_source2_string_range_position12 = source_ml_source2_string_range_operand11;
                                        {
                                            let source_ml_source2_string_range_position13 = f64::INFINITY;
                                            source_ml_source2_string_range_operand10.slice(source_ml_source2_string_range_position12, source_ml_source2_string_range_position13)
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_prefix_end(row: Vec<f64>) -> f64 {
    {
        let end = (self::ml_array::optional_at(&row, self::ml_array::number_index(8f64))).clone().unwrap_or_else(|| (-1f64));
        {
            let fields = (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64);
            if (((((((row.len() as f64)) < (9f64)) || !(self::ml_math::is_safe_integer(fields))) || (fields < (0f64))) || (fields > (8192f64))) || (((9f64 + (3f64 * fields))) > ((row.len() as f64)))) {
                (-1f64)
            } else {
                {
                    let at = 9f64;
                    {
                        let end_2 = self::source_ml_source1_temporary_read_java_script_closed_class_prefix_end_loop10(row, end, fields, at);
                        end_2
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_assigned_field_position(values: Vec<self::ml_string::Text>, row: Vec<f64>, name: self::ml_string::Text) -> f64 {
    {
        let at = 9f64;
        {
            let source_ml_source2_s14 = self::source_ml_source1_temporary_read_java_script_closed_class_assigned_field_position_loop11(values, row, name.clone(), at);
            match source_ml_source2_s14.clone() {
                self::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Return(source_ml_source1_temporary_result11) => {
                    source_ml_source1_temporary_result11
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_directory(values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>, first: f64) -> Vec<f64> {
    if (((((self::ml_array::optional_at(&values, self::ml_array::number_index(first))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) || (((self::ml_array::optional_at(&kinds, self::ml_array::number_index((first + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((first + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![123])))) {
        vec![(-1f64), first]
    } else {
        {
            let end = self::find_java_script_class_group_end(values.clone(), (first + 2f64));
            if (end < (0f64)) {
                vec![(-1f64), (first + 2f64)]
            } else {
                {
                    let constructor = vec![(-1f64), (-1f64), (-1f64), (-1f64), (-1f64)];
                    {
                        let methods = Vec::<f64>::new();
                        {
                            let fields = Vec::<f64>::new();
                            {
                                let names = Vec::<self::ml_string::Text>::new();
                                {
                                    let at = (first + 3f64);
                                    {
                                        let source_ml_source2_s15 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop12(values.clone(), kinds.clone(), end, constructor, methods, fields, names, at);
                                        match source_ml_source2_s15.clone() {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Done(constructor_2, methods_2, fields_2, names_2) => {
                                                {
                                                    let body_start = (self::ml_array::optional_at(&constructor_2, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let body_end = (self::ml_array::optional_at(&constructor_2, self::ml_array::number_index(4f64))).clone().unwrap_or_else(|| (-1f64));
                                                        {
                                                            let initialization = if (body_start < (0f64)) {
                                                                (-1f64)
                                                            } else {
                                                                self::find_java_script_closed_class_initialization_start(values.clone(), (body_start + 1f64), body_end)
                                                            };
                                                            {
                                                                let cursor = initialization;
                                                                {
                                                                    let source_ml_source2_s16 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop13(values.clone(), kinds, fields_2, names_2, body_start, body_end, cursor);
                                                                    match source_ml_source2_s16.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Done(fields_3) => {
                                                                            {
                                                                                let source_ml_source2_s18 = if (body_start >= (0f64)) {
                                                                                    {
                                                                                        let at_2 = (body_start + 1f64);
                                                                                        {
                                                                                            let source_ml_source2_s19 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop15(values, body_end, at_2);
                                                                                            match source_ml_source2_s19.clone() {
                                                                                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Done => {
                                                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Next
                                                                                                }
                                                                                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Return(source_ml_source1_temporary_result15) => {
                                                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Return(source_ml_source1_temporary_result15)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Next
                                                                                };
                                                                                match source_ml_source2_s18.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Next => {
                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(vec![first, end], constructor_2), vec![((fields_3.len() as f64) / 3f64), initialization]), fields_3), methods_2)
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Return(source_ml_source1_temporary_result14) => {
                                                                                        source_ml_source1_temporary_result14
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Return(source_ml_source1_temporary_result15_2) => {
                                                                            source_ml_source1_temporary_result15_2
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(source_ml_source1_temporary_result19) => {
                                                source_ml_source1_temporary_result19
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

pub fn read_java_script_closed_class_refusal(code: f64) -> self::ml_string::Text {
    if (code == ((-1f64))) {
        self::ml_string::Text::from_units(vec![97, 32, 99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 110, 101, 101, 100, 115, 32, 97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 110, 97, 109, 101, 100, 32, 98, 97, 115, 101, 45, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 97, 108, 97, 110, 99, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 103, 114, 111, 117, 112, 115])
    } else {
        if (code == ((-2f64))) {
            self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114, 115, 44, 32, 112, 114, 105, 118, 97, 116, 101, 32, 110, 97, 109, 101, 115, 44, 32, 105, 110, 104, 101, 114, 105, 116, 97, 110, 99, 101, 44, 32, 115, 116, 97, 116, 105, 99, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 109, 101, 109, 98, 101, 114, 32, 111, 119, 110, 101, 114, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 102, 117, 108, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 109, 111, 100, 101, 108])
        } else {
            if (code == ((-3f64))) {
                self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 99, 108, 97, 115, 115, 32, 112, 114, 111, 102, 105, 108, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 102, 105, 120, 101, 100, 32, 105, 100, 101, 110, 116, 105, 102, 105, 101, 114, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 119, 105, 116, 104, 32, 108, 105, 116, 101, 114, 97, 108, 32, 100, 101, 102, 97, 117, 108, 116, 115, 59, 32, 112, 97, 116, 116, 101, 114, 110, 115, 32, 97, 110, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 116, 32, 111, 114, 32, 105, 110, 115, 116, 97, 110, 99, 101, 45, 114, 101, 97, 100, 105, 110, 103, 32, 100, 101, 102, 97, 117, 108, 116, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116])
            } else {
                if (code == ((-11f64))) {
                    self::ml_string::Text::from_units(vec![97, 32, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 100, 101, 102, 97, 117, 108, 116, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 108, 111, 99, 97, 108, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 119, 105, 116, 104, 32, 116, 104, 101, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 111, 119, 110, 101, 114])
                } else {
                    if (code == ((-4f64))) {
                        self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 110, 100, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 101, 116, 104, 111, 100])
                    } else {
                        if (code == ((-5f64))) {
                            self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 115, 32, 110, 101, 101, 100, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 105, 99, 111, 108, 111, 110, 45, 116, 101, 114, 109, 105, 110, 97, 116, 101, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 112, 114, 101, 102, 105, 120, 32, 119, 105, 116, 104, 111, 117, 116, 32, 105, 110, 115, 116, 97, 110, 99, 101, 32, 101, 115, 99, 97, 112, 101, 32, 111, 114, 32, 114, 101, 97, 100, 115, 32, 98, 101, 102, 111, 114, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110])
                        } else {
                            if (code == ((-6f64))) {
                                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 116, 117, 114, 110, 32, 111, 118, 101, 114, 114, 105, 100, 101, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115])
                            } else {
                                if (code == ((-7f64))) {
                                    self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 118, 97, 108, 117, 101, 115, 44, 32, 114, 101, 102, 108, 101, 99, 116, 105, 118, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 44, 32, 100, 101, 116, 97, 99, 104, 101, 100, 32, 109, 101, 116, 104, 111, 100, 115, 44, 32, 109, 101, 116, 104, 111, 100, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 97, 110, 100, 32, 111, 98, 115, 101, 114, 118, 97, 98, 108, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 32, 102, 117, 108, 108, 32, 111, 98, 106, 101, 99, 116, 32, 109, 111, 100, 101, 108])
                                } else {
                                    if (code == ((-8f64))) {
                                        self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 97, 117, 116, 104, 111, 114, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 111, 117, 114, 99, 101, 44, 32, 116, 111, 107, 101, 110, 44, 32, 110, 101, 115, 116, 105, 110, 103, 32, 111, 114, 32, 99, 108, 97, 115, 115, 45, 99, 111, 117, 110, 116, 32, 98, 117, 100, 103, 101, 116])
                                    } else {
                                        if (code == ((-10f64))) {
                                            self::ml_string::Text::from_units(vec![112, 117, 98, 108, 105, 99, 32, 102, 105, 101, 108, 100, 115, 32, 110, 101, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 105, 99, 111, 108, 111, 110, 45, 116, 101, 114, 109, 105, 110, 97, 116, 101, 100, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 115, 32, 119, 105, 116, 104, 32, 112, 114, 111, 118, 101, 110, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 115, 32, 97, 110, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 105, 110, 103])
                                        } else {
                                            if (code == ((-9f64))) {
                                                self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 109, 101, 109, 98, 101, 114, 32, 109, 111, 100, 105, 102, 105, 101, 114, 32, 107, 101, 121, 119, 111, 114, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 101, 115, 99, 97, 112, 101, 100, 32, 69, 67, 77, 65, 83, 99, 114, 105, 112, 116, 32, 115, 111, 117, 114, 99, 101, 32, 115, 112, 101, 108, 108, 105, 110, 103])
                                            } else {
                                                self::ml_string::Text::from_units(vec![])
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

pub fn render_java_script_closed_class_range(source: self::ml_string::Text, values: Vec<self::ml_string::Text>, starts: Vec<f64>, ends: Vec<f64>, first: f64, last: f64, receiver: self::ml_string::Text, class_names: Vec<self::ml_string::Text>, static_calls: Vec<self::ml_string::Text>) -> self::ml_string::Text {
    if (first >= last) {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let result = self::ml_string::Text::from_units(vec![]);
            {
                let previous = (self::ml_array::optional_at(&starts, self::ml_array::number_index(first))).clone().unwrap_or_else(|| 0f64);
                {
                    let at = first;
                    {
                        let source_ml_source2_s20 = self::source_ml_source1_temporary_render_java_script_closed_class_range_loop21(source.clone(), values, starts, ends.clone(), last, receiver.clone(), class_names, static_calls, result.clone(), previous, at);
                        match source_ml_source2_s20.clone() {
                            self::MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Result::MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Done(result_2, previous_2) => {
                                result_2.concat(&{
                                    let source_ml_source2_string_range_operand21 = source.clone();
                                    {
                                        let source_ml_source2_string_range_operand22 = previous_2;
                                        {
                                            let source_ml_source2_string_range_operand23 = (self::ml_array::optional_at(&ends, self::ml_array::number_index((last - 1f64)))).clone().unwrap_or_else(|| previous_2);
                                            {
                                                let source_ml_source2_string_range_position24 = source_ml_source2_string_range_operand22;
                                                {
                                                    let source_ml_source2_string_range_position25 = source_ml_source2_string_range_operand23;
                                                    source_ml_source2_string_range_operand21.slice(source_ml_source2_string_range_position24, source_ml_source2_string_range_position25)
                                                }
                                            }
                                        }
                                    }
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_methods(values: Vec<self::ml_string::Text>, directory: Vec<f64>) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let first = (9f64 + (3f64 * (self::ml_array::optional_at(&directory, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
            {
                let at = first;
                {
                    let result_2 = self::source_ml_source1_temporary_read_java_script_closed_class_methods_loop22(values, directory, result, at);
                    result_2
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_static_calls(values: Vec<self::ml_string::Text>, directory: Vec<Vec<f64>>, prefix: self::ml_string::Text) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let ordinal = 0f64;
            {
                let source_ml_source1_temporary_values23 = directory;
                {
                    let source_ml_source1_temporary_index23 = self::ml::Big::from_i128(0);
                    {
                        let result_2 = self::source_ml_source1_temporary_read_java_script_closed_class_static_calls_loop24(values, prefix.clone(), result, ordinal, source_ml_source1_temporary_values23, source_ml_source1_temporary_index23.clone());
                        result_2
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_static_target(values: Vec<self::ml_string::Text>, names: Vec<self::ml_string::Text>, calls: Vec<self::ml_string::Text>, at: f64) -> self::ml_string::Text {
    {
        let previous = (self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        if (((((!(self::ml_array::includes(&names, &(self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) || (previous == (self::ml_string::Text::from_units(vec![110, 101, 119])))) || (previous == (self::ml_string::Text::from_units(vec![46])))) || (previous == (self::ml_string::Text::from_units(vec![63, 46])))) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![46])))) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 3f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![40])))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let key = (((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))).concat(&self::ml_string::Text::from_units(vec![46]))).concat(&(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])));
                {
                    let index = 0f64;
                    {
                        let source_ml_source2_s26 = self::source_ml_source1_temporary_read_java_script_closed_class_static_target_loop26(calls, key.clone(), index);
                        match source_ml_source2_s26.clone() {
                            self::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Result::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Result::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Return(source_ml_source1_temporary_result26) => {
                                source_ml_source1_temporary_result26.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_collection_default_class_owner_error(values: Vec<self::ml_string::Text>, directory: Vec<Vec<f64>>) -> f64 {
    {
        let names = Vec::<self::ml_string::Text>::new();
        {
            let source_ml_source1_temporary_values27 = directory.clone();
            {
                let source_ml_source1_temporary_index27 = self::ml::Big::from_i128(0);
                {
                    let names_2 = self::source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop29(values.clone(), names, source_ml_source1_temporary_values27, source_ml_source1_temporary_index27.clone());
                    {
                        let source_ml_source1_temporary_values28 = directory;
                        {
                            let source_ml_source1_temporary_index28 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s27 = self::source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop30(values, names_2, source_ml_source1_temporary_values28, source_ml_source1_temporary_index28.clone());
                                match source_ml_source2_s27.clone() {
                                    self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Done => {
                                        (-1f64)
                                    }
                                    self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Return(source_ml_source1_temporary_result30) => {
                                        source_ml_source1_temporary_result30
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

pub fn accept_java_script_closed_class_function_export(values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>, at: f64) -> bool {
    {
        let previous = (self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        ((((((((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![101, 120, 112, 111, 114, 116]))) && (previous != (self::ml_string::Text::from_units(vec![46])))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))) && (((self::ml_array::optional_at(&kinds, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 3f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40]))))
    }
}

pub fn read_java_script_closed_class_function_exports(values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_read_java_script_closed_class_function_exports_loop34(values, kinds, result, at);
                result_2
            }
        }
    }
}

pub fn read_java_script_closed_class_observation_error(values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>, directory: Vec<Vec<f64>>, static_calls: Vec<self::ml_string::Text>) -> f64 {
    {
        let names = Vec::<self::ml_string::Text>::new();
        {
            let methods = Vec::<self::ml_string::Text>::new();
            {
                let source_ml_source1_temporary_values35 = directory.clone();
                {
                    let source_ml_source1_temporary_index35 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s28 = self::source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop37(values.clone(), names, methods, source_ml_source1_temporary_values35, source_ml_source1_temporary_index35.clone());
                        match source_ml_source2_s28.clone() {
                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Done(names_2, methods_2) => {
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_source2_s29 = self::source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop38(values, kinds, directory, static_calls, names_2, methods_2, at);
                                        match source_ml_source2_s29.clone() {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Done => {
                                                (-1f64)
                                            }
                                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(source_ml_source1_temporary_result38) => {
                                                source_ml_source1_temporary_result38
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Return(source_ml_source1_temporary_result40) => {
                                source_ml_source1_temporary_result40
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_java_script_closed_class(source: self::ml_string::Text, values: Vec<self::ml_string::Text>, starts: Vec<f64>, ends: Vec<f64>, row: Vec<f64>, prefix: self::ml_string::Text, ordinal: f64, class_names: Vec<self::ml_string::Text>, static_calls: Vec<self::ml_string::Text>, prototype: bool) -> self::ml_string::Text {
    {
        let name = (self::ml_array::optional_at(&values, self::ml_array::number_index(((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64) + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        {
            let receiver = (prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95]))).concat(&{
                let source_ml_source2_string_argument30 = ordinal;
                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument30))
            });
            {
                let brand = (prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95]))).concat(&{
                    let source_ml_source2_string_argument31 = ordinal;
                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument31))
                });
                {
                    let brand_value = (prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 98, 114, 97, 110, 100, 95, 118, 97, 108, 117, 101, 95]))).concat(&{
                        let source_ml_source2_string_argument32 = ordinal;
                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument32))
                    });
                    {
                        let field_count = (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64);
                        {
                            let methods = (9f64 + (field_count * 3f64));
                            {
                                let parameter_first = ((self::ml_array::optional_at(&row, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| (-1f64)) + 1f64);
                                {
                                    let parameter_last = (self::ml_array::optional_at(&row, self::ml_array::number_index(4f64))).clone().unwrap_or_else(|| (-1f64));
                                    {
                                        let parameters = if (parameter_last < (0f64)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::render_java_script_closed_class_range(source.clone(), values.clone(), starts.clone(), ends.clone(), parameter_first, parameter_last, self::ml_string::Text::from_units(vec![]), class_names.clone(), static_calls.clone())
                                        };
                                        {
                                            let stubs = ((self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32])).concat(&brand_value)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 59, 32, 125, 10]));
                                            {
                                                let initializations = self::ml_string::Text::from_units(vec![]);
                                                {
                                                    let constructor_fields = self::ml_string::Text::from_units(vec![]);
                                                    {
                                                        let field_names = Vec::<self::ml_string::Text>::new();
                                                        {
                                                            let fields = ((brand.concat(&self::ml_string::Text::from_units(vec![58, 32]))).concat(&brand_value)).concat(&self::ml_string::Text::from_units(vec![40, 41]));
                                                            {
                                                                let at = 9f64;
                                                                {
                                                                    let source_ml_source2_s33 = self::source_ml_source1_temporary_render_java_script_closed_class_loop43(source.clone(), values.clone(), starts.clone(), ends.clone(), row.clone(), prefix.clone(), ordinal, class_names.clone(), static_calls.clone(), methods, stubs.clone(), initializations.clone(), constructor_fields.clone(), field_names, fields.clone(), at);
                                                                    match source_ml_source2_s33.clone() {
                                                                        self::MlSource1TemporaryRenderJavaScriptClosedClassLoop43Result::MlSource1TemporaryRenderJavaScriptClosedClassLoop43Done(stubs_2, initializations_2, constructor_fields_2, fields_2) => {
                                                                            {
                                                                                let assignments = self::ml_string::Text::from_units(vec![]);
                                                                                {
                                                                                    let at_2 = methods;
                                                                                    {
                                                                                        let source_ml_source2_s34 = self::source_ml_source1_temporary_render_java_script_closed_class_loop44(source.clone(), values.clone(), starts.clone(), ends.clone(), row.clone(), prefix.clone(), ordinal, class_names.clone(), static_calls.clone(), prototype, receiver.clone(), stubs_2.clone(), fields_2.clone(), assignments.clone(), at_2);
                                                                                        match source_ml_source2_s34.clone() {
                                                                                            self::MlSource1TemporaryRenderJavaScriptClosedClassLoop44Result::MlSource1TemporaryRenderJavaScriptClosedClassLoop44Done(stubs_3, fields_3, assignments_2) => {
                                                                                                {
                                                                                                    let constructor_body = (self::ml_array::optional_at(&row, self::ml_array::number_index(5f64))).clone().unwrap_or_else(|| (-1f64));
                                                                                                    {
                                                                                                        let constructor_end = (self::ml_array::optional_at(&row, self::ml_array::number_index(6f64))).clone().unwrap_or_else(|| (-1f64));
                                                                                                        {
                                                                                                            let initialization = (self::ml_array::optional_at(&row, self::ml_array::number_index(8f64))).clone().unwrap_or_else(|| (-1f64));
                                                                                                            {
                                                                                                                let prologue = if (constructor_body < (0f64)) {
                                                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                                                } else {
                                                                                                                    self::render_java_script_closed_class_range(source.clone(), values.clone(), starts.clone(), ends.clone(), (constructor_body + 1f64), initialization, self::ml_string::Text::from_units(vec![]), class_names.clone(), static_calls.clone())
                                                                                                                };
                                                                                                                {
                                                                                                                    let remainder = if (constructor_body < (0f64)) {
                                                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                                                    } else {
                                                                                                                        self::render_java_script_closed_class_range(source.clone(), values, starts.clone(), ends.clone(), self::read_java_script_closed_class_prefix_end(row.clone()), constructor_end, receiver.clone(), class_names, static_calls)
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let constructor_name = (self::ml_array::optional_at(&row, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| (-1f64));
                                                                                                                        {
                                                                                                                            let documentation = if (constructor_name < (0f64)) {
                                                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                                                            } else {
                                                                                                                                {
                                                                                                                                    let source_ml_source2_string_range_operand35 = source.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_string_range_operand36 = (self::ml_array::optional_at(&ends, self::ml_array::number_index((constructor_name - 1f64)))).clone().unwrap_or_else(|| 0f64);
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_string_range_operand37 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(constructor_name))).clone().unwrap_or_else(|| 0f64);
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_string_range_position38 = source_ml_source2_string_range_operand36;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_string_range_position39 = source_ml_source2_string_range_operand37;
                                                                                                                                                    source_ml_source2_string_range_operand35.slice(source_ml_source2_string_range_position38, source_ml_source2_string_range_position39)
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            ((((((((((((((((((((stubs_3.concat(&self::ml_string::Text::from_units(vec![10]))).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&name)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&parameters)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 10]))).concat(&initializations_2)).concat(&prologue)).concat(&self::ml_string::Text::from_units(vec![10]))).concat(&constructor_fields_2)).concat(&self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 123, 32]))).concat(&fields_3)).concat(&self::ml_string::Text::from_units(vec![32, 125, 59, 10]))).concat(&assignments_2)).concat(&remainder)).concat(&self::ml_string::Text::from_units(vec![10, 114, 101, 116, 117, 114, 110, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![59, 10, 125, 10]))
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
                }
            }
        }
    }
}

pub fn require_java_script_closed_class_prototype_methods(values: Vec<self::ml_string::Text>, directory: Vec<Vec<f64>>) -> bool {
    {
        let names = Vec::<self::ml_string::Text>::new();
        {
            let source_ml_source1_temporary_values47 = directory.clone();
            {
                let source_ml_source1_temporary_index47 = self::ml::Big::from_i128(0);
                {
                    let names_2 = self::source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop49(values.clone(), names, source_ml_source1_temporary_values47, source_ml_source1_temporary_index47.clone());
                    {
                        let source_ml_source1_temporary_values48 = directory;
                        {
                            let source_ml_source1_temporary_index48 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s40 = self::source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop50(values, names_2, source_ml_source1_temporary_values48, source_ml_source1_temporary_index48.clone());
                                match source_ml_source2_s40.clone() {
                                    self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Done => {
                                        false
                                    }
                                    self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Return(source_ml_source1_temporary_result50) => {
                                        source_ml_source1_temporary_result50
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

pub fn read_java_script_closed_class_method_target(methods: Vec<self::ml_string::Text>, receiver_fields: Vec<self::ml_string::Text>, method: self::ml_string::Text) -> self::ml_string::Text {
    if ((((methods.len() as f64) % 3f64)) != (0f64)) {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let target = self::ml_string::Text::from_units(vec![]);
            {
                let at = 0f64;
                {
                    let source_ml_source2_s41 = self::source_ml_source1_temporary_read_java_script_closed_class_method_target_loop53(methods, receiver_fields, method.clone(), target.clone(), at);
                    match source_ml_source2_s41.clone() {
                        self::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Result::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Done(target_2) => {
                            target_2.clone()
                        }
                        self::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Result::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Return(source_ml_source1_temporary_result53) => {
                            source_ml_source1_temporary_result53.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_java_script_closed_class_method_definition(brand: self::ml_string::Text, method: self::ml_string::Text, helper: self::ml_string::Text, parameter: self::ml_string::Text, receiver_fields: Vec<self::ml_string::Text>, constructor: bool) -> bool {
    {
        let marker = {
            let source_ml_source2_string_search_operand42 = brand.clone();
            {
                let source_ml_source2_string_search_operand43 = self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95]);
                {
                    let source_ml_source2_string_search_text44 = source_ml_source2_string_search_operand43.clone();
                    {
                        let source_ml_source2_string_search_position45 = f64::NAN;
                        source_ml_source2_string_search_operand42.last_index_of(&source_ml_source2_string_search_text44, source_ml_source2_string_search_position45)
                    }
                }
            }
        };
        if (marker < (0f64)) {
            false
        } else {
            {
                let prefix = {
                    let source_ml_source2_string_range_operand46 = brand.clone();
                    {
                        let source_ml_source2_string_range_operand47 = 0f64;
                        {
                            let source_ml_source2_string_range_operand48 = marker;
                            {
                                let source_ml_source2_string_range_position49 = source_ml_source2_string_range_operand47;
                                {
                                    let source_ml_source2_string_range_position50 = source_ml_source2_string_range_operand48;
                                    source_ml_source2_string_range_operand46.slice(source_ml_source2_string_range_position49, source_ml_source2_string_range_position50)
                                }
                            }
                        }
                    }
                };
                {
                    let ordinal = {
                        let source_ml_source2_string_range_operand51 = brand.clone();
                        {
                            let source_ml_source2_string_range_operand52 = (marker + 15f64);
                            {
                                let source_ml_source2_string_range_position53 = source_ml_source2_string_range_operand52;
                                {
                                    let source_ml_source2_string_range_position54 = f64::INFINITY;
                                    source_ml_source2_string_range_operand51.slice(source_ml_source2_string_range_position53, source_ml_source2_string_range_position54)
                                }
                            }
                        }
                    };
                    ((((((ordinal != (self::ml_string::Text::from_units(vec![]))) && (method != (self::ml_string::Text::from_units(vec![])))) && helper.starts_with(&((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95]))).concat(&ordinal)).concat(&self::ml_string::Text::from_units(vec![95])))) && (parameter == ((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95]))).concat(&ordinal)))) && self::ml_array::includes(&receiver_fields, &brand.clone(), 0f64, false, |a, b| a == b)) && constructor)
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_owner_row(row: Vec<self::ml_string::Text>) -> Vec<f64> {
    if (((row.len() as f64)) < (5f64)) {
        vec![(-1f64)]
    } else {
        {
            let fields = self::ml_string_number::convert_java_script_string_number_units(&((self::ml_array::optional_at(&row, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))).0.iter().copied().map(f64::from).collect::<Vec<f64>>());
            {
                let methods = self::ml_string_number::convert_java_script_string_number_units(&((self::ml_array::optional_at(&row, self::ml_array::number_index(4f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))).0.iter().copied().map(f64::from).collect::<Vec<f64>>());
                if ((((((((!(self::ml_math::is_safe_integer(fields)) || (fields < (0f64))) || (fields > (8192f64))) || (({
                    let source_ml_source2_string_argument56 = fields;
                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument56))
                }) != ((self::ml_array::optional_at(&row, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))))) || !(self::ml_math::is_safe_integer(methods))) || (methods < (0f64))) || (methods > (8192f64))) || (({
                    let source_ml_source2_string_argument58 = methods;
                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument58))
                }) != ((self::ml_array::optional_at(&row, self::ml_array::number_index(4f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))))) || (((row.len() as f64)) != (((5f64 + fields) + (2f64 * methods))))) {
                    vec![(-1f64)]
                } else {
                    vec![0f64, 5f64, fields, (5f64 + fields), methods]
                }
            }
        }
    }
}

pub fn lower_java_script_closed_class_source(source: self::ml_string::Text, values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>, starts: Vec<f64>, ends: Vec<f64>, prefix: self::ml_string::Text) -> (f64, Vec<self::ml_string::Text>, bool, self::ml_string::Text, Vec<f64>, Vec<Vec<self::ml_string::Text>>, Vec<self::ml_string::Text>, Vec<self::ml_string::Text>, self::ml_string::Text) {
    if (((!(self::accept_java_script_closed_class_source_size(source.length(), (values.len() as f64))) || (((kinds.len() as f64)) != ((values.len() as f64)))) || (((starts.len() as f64)) != ((values.len() as f64)))) || (((ends.len() as f64)) != ((values.len() as f64)))) {
        {
            let source_ml_source2_field59 = source.clone();
            {
                let source_ml_source2_field60 = Vec::<f64>::new();
                {
                    let source_ml_source2_field61 = Vec::<self::ml_string::Text>::new();
                    {
                        let source_ml_source2_field62 = Vec::<self::ml_string::Text>::new();
                        {
                            let source_ml_source2_field63 = Vec::<self::ml_string::Text>::new();
                            {
                                let source_ml_source2_field64 = Vec::<Vec<self::ml_string::Text>>::new();
                                {
                                    let source_ml_source2_field65 = self::read_java_script_closed_class_refusal((-8f64));
                                    {
                                        let source_ml_source2_field66 = 0f64;
                                        {
                                            let source_ml_source2_field67 = false;
                                            (source_ml_source2_field66, source_ml_source2_field61, source_ml_source2_field67, source_ml_source2_field65.clone(), source_ml_source2_field60, source_ml_source2_field64, source_ml_source2_field63, source_ml_source2_field62, source_ml_source2_field59.clone())
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
        {
            let syntax = Vec::<self::ml_string::Text>::new();
            {
                let at = 0f64;
                {
                    let syntax_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop55(values, kinds.clone(), syntax, at);
                    {
                        let values_2 = syntax_2;
                        {
                            let directory = Vec::<Vec<f64>>::new();
                            {
                                let names = Vec::<self::ml_string::Text>::new();
                                {
                                    let depth = 0f64;
                                    {
                                        let declarations_finished = false;
                                        {
                                            let error = 0f64;
                                            {
                                                let error_at = 0f64;
                                                {
                                                    let error_3 = if ((((((source.length()) > (131072f64)) || (((values_2.len() as f64)) > (8192f64))) || (((kinds.len() as f64)) != ((values_2.len() as f64)))) || (((starts.len() as f64)) != ((values_2.len() as f64)))) || (((ends.len() as f64)) != ((values_2.len() as f64)))) {
                                                        {
                                                            let error_2 = (-8f64);
                                                            error_2
                                                        }
                                                    } else {
                                                        error
                                                    };
                                                    {
                                                        let at_2 = 0f64;
                                                        {
                                                            let source_ml_source2_s68 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop56(values_2.clone(), kinds.clone(), directory, names, depth, declarations_finished, error_3, error_at, at_2);
                                                            match source_ml_source2_s68.clone() {
                                                                self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory_2, names_2, error_4, error_at_2) => {
                                                                    {
                                                                        let source_ml_source2_s70 = if ((error_4 == (0f64)) && (((directory_2.len() as f64)) > (0f64))) {
                                                                            {
                                                                                let at_3 = 0f64;
                                                                                {
                                                                                    let source_ml_source2_s71 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop66(source.clone(), values_2.clone(), kinds.clone(), starts.clone(), ends.clone(), names_2.clone(), error_4, error_at_2, at_3);
                                                                                    match source_ml_source2_s71.clone() {
                                                                                        self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Done(error_5, error_at_3) => {
                                                                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57Next(error_5, error_at_3)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57Next(error_4, error_at_2)
                                                                        };
                                                                        match source_ml_source2_s70.clone() {
                                                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57Next(error_6, error_at_4) => {
                                                                                {
                                                                                    let static_calls = self::read_java_script_closed_class_static_calls(values_2.clone(), directory_2.clone(), prefix.clone());
                                                                                    {
                                                                                        let source_ml_source2_s72 = if ((error_6 == (0f64)) && (((directory_2.len() as f64)) > (0f64))) {
                                                                                            {
                                                                                                let owner = self::read_java_script_closed_class_collection_default_class_owner_error(values_2.clone(), directory_2.clone());
                                                                                                if (owner >= (0f64)) {
                                                                                                    {
                                                                                                        let error_7 = (-11f64);
                                                                                                        {
                                                                                                            let error_at_5 = owner;
                                                                                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58Next(error_7, error_at_5)
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58Next(error_6, error_at_4)
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58Next(error_6, error_at_4)
                                                                                        };
                                                                                        match source_ml_source2_s72.clone() {
                                                                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58Next(error_8, error_at_6) => {
                                                                                                {
                                                                                                    let source_ml_source2_s73 = if ((error_8 == (0f64)) && (((directory_2.len() as f64)) > (0f64))) {
                                                                                                        {
                                                                                                            let observation = self::read_java_script_closed_class_observation_error(values_2.clone(), kinds, directory_2.clone(), static_calls.clone());
                                                                                                            if (observation >= (0f64)) {
                                                                                                                {
                                                                                                                    let error_9 = (-7f64);
                                                                                                                    {
                                                                                                                        let error_at_7 = observation;
                                                                                                                        self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59Next(error_9, error_at_7)
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59Next(error_8, error_at_6)
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59Next(error_8, error_at_6)
                                                                                                    };
                                                                                                    match source_ml_source2_s73.clone() {
                                                                                                        self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59Next(error_10, error_at_8) => {
                                                                                                            if (error_10 != (0f64)) {
                                                                                                                {
                                                                                                                    let source_ml_source2_field74 = source.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_field75 = Vec::<f64>::new();
                                                                                                                        {
                                                                                                                            let source_ml_source2_field76 = Vec::<self::ml_string::Text>::new();
                                                                                                                            {
                                                                                                                                let source_ml_source2_field77 = Vec::<self::ml_string::Text>::new();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_field78 = Vec::<self::ml_string::Text>::new();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_field79 = Vec::<Vec<self::ml_string::Text>>::new();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_field80 = self::read_java_script_closed_class_refusal(error_10);
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_field81 = error_at_8;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_field82 = false;
                                                                                                                                                    (source_ml_source2_field81, source_ml_source2_field76, source_ml_source2_field82, source_ml_source2_field80.clone(), source_ml_source2_field75, source_ml_source2_field79, source_ml_source2_field78, source_ml_source2_field77, source_ml_source2_field74.clone())
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
                                                                                                                if (((directory_2.len() as f64)) == (0f64)) {
                                                                                                                    {
                                                                                                                        let source_ml_source2_field83 = source.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_field84 = Vec::<f64>::new();
                                                                                                                            {
                                                                                                                                let source_ml_source2_field85 = Vec::<self::ml_string::Text>::new();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_field86 = Vec::<self::ml_string::Text>::new();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_field87 = Vec::<self::ml_string::Text>::new();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_field88 = Vec::<Vec<self::ml_string::Text>>::new();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_field89 = self::ml_string::Text::from_units(vec![]);
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_field90 = 0f64;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_field91 = false;
                                                                                                                                                        (source_ml_source2_field90, source_ml_source2_field85, source_ml_source2_field91, source_ml_source2_field89.clone(), source_ml_source2_field84, source_ml_source2_field88, source_ml_source2_field87, source_ml_source2_field86, source_ml_source2_field83.clone())
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
                                                                                                                    {
                                                                                                                        let text = self::ml_string::Text::from_units(vec![]);
                                                                                                                        {
                                                                                                                            let intervals = Vec::<f64>::new();
                                                                                                                            {
                                                                                                                                let previous = 0f64;
                                                                                                                                {
                                                                                                                                    let ordinal = 0f64;
                                                                                                                                    {
                                                                                                                                        let brands = Vec::<self::ml_string::Text>::new();
                                                                                                                                        {
                                                                                                                                            let public_fields = Vec::<self::ml_string::Text>::new();
                                                                                                                                            {
                                                                                                                                                let prototype_methods = Vec::<self::ml_string::Text>::new();
                                                                                                                                                {
                                                                                                                                                    let owner_rows = Vec::<Vec<self::ml_string::Text>>::new();
                                                                                                                                                    {
                                                                                                                                                        let prototype = self::require_java_script_closed_class_prototype_methods(values_2.clone(), directory_2.clone());
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source1_temporary_values54 = directory_2;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_temporary_index54 = self::ml::Big::from_i128(0);
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_s92 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop60(source.clone(), values_2.clone(), starts.clone(), ends.clone(), prefix.clone(), names_2.clone(), static_calls.clone(), text.clone(), intervals, previous, ordinal, brands, public_fields, prototype_methods, owner_rows, prototype, source_ml_source1_temporary_values54, source_ml_source1_temporary_index54.clone());
                                                                                                                                                                    match source_ml_source2_s92.clone() {
                                                                                                                                                                        self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Done(text_2, intervals_2, previous_2, brands_2, public_fields_2, prototype_methods_2, owner_rows_2) => {
                                                                                                                                                                            {
                                                                                                                                                                                let first = 0f64;
                                                                                                                                                                                {
                                                                                                                                                                                    let first_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop61(values_2.clone(), starts.clone(), previous_2, first);
                                                                                                                                                                                    {
                                                                                                                                                                                        let rest = if (first_2 >= ((values_2.len() as f64))) {
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source2_string_range_operand93 = source.clone();
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_string_range_operand94 = previous_2;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_string_range_position95 = source_ml_source2_string_range_operand94;
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source2_string_range_position96 = f64::INFINITY;
                                                                                                                                                                                                            source_ml_source2_string_range_operand93.slice(source_ml_source2_string_range_position95, source_ml_source2_string_range_position96)
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        } else {
                                                                                                                                                                                            (({
                                                                                                                                                                                                let source_ml_source2_string_range_operand97 = source.clone();
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_string_range_operand98 = previous_2;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_string_range_operand99 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(first_2))).clone().unwrap_or_else(|| previous_2);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source2_string_range_position100 = source_ml_source2_string_range_operand98;
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let source_ml_source2_string_range_position101 = source_ml_source2_string_range_operand99;
                                                                                                                                                                                                                source_ml_source2_string_range_operand97.slice(source_ml_source2_string_range_position100, source_ml_source2_string_range_position101)
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }).concat(&self::render_java_script_closed_class_range(source.clone(), values_2.clone(), starts, ends.clone(), first_2, (values_2.len() as f64), self::ml_string::Text::from_units(vec![]), names_2, static_calls))).concat(&{
                                                                                                                                                                                                let source_ml_source2_string_range_operand102 = source.clone();
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_string_range_operand103 = (self::ml_array::optional_at(&ends, self::ml_array::number_index(((values_2.len() as f64) - 1f64)))).clone().unwrap_or_else(|| source.length());
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_string_range_position104 = source_ml_source2_string_range_operand103;
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source2_string_range_position105 = f64::INFINITY;
                                                                                                                                                                                                            source_ml_source2_string_range_operand102.slice(source_ml_source2_string_range_position104, source_ml_source2_string_range_position105)
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            })
                                                                                                                                                                                        };
                                                                                                                                                                                        {
                                                                                                                                                                                            let intervals_3 = {
                                                                                                                                                                                                let source_ml_source2_concat_value106 = intervals_2;
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_concat_value107 = vec![text_2.length(), (text_2.length() + rest.length()), previous_2, source.length()];
                                                                                                                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value106), source_ml_source2_concat_value107)
                                                                                                                                                                                                }
                                                                                                                                                                                            };
                                                                                                                                                                                            {
                                                                                                                                                                                                let text_3 = text_2.concat(&rest);
                                                                                                                                                                                                if ((text_3.length()) > (1048576f64)) {
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_field108 = source.clone();
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source2_field109 = Vec::<f64>::new();
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let source_ml_source2_field110 = Vec::<self::ml_string::Text>::new();
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let source_ml_source2_field111 = Vec::<self::ml_string::Text>::new();
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let source_ml_source2_field112 = Vec::<self::ml_string::Text>::new();
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            let source_ml_source2_field113 = Vec::<Vec<self::ml_string::Text>>::new();
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let source_ml_source2_field114 = self::read_java_script_closed_class_refusal((-8f64));
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    let source_ml_source2_field115 = 0f64;
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        let source_ml_source2_field116 = false;
                                                                                                                                                                                                                                        (source_ml_source2_field115, source_ml_source2_field110, source_ml_source2_field116, source_ml_source2_field114.clone(), source_ml_source2_field109, source_ml_source2_field113, source_ml_source2_field112, source_ml_source2_field111, source_ml_source2_field108.clone())
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
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_field117 = text_3.clone();
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source2_field118 = intervals_3;
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let source_ml_source2_field119 = brands_2;
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let source_ml_source2_field120 = public_fields_2;
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let source_ml_source2_field121 = prototype_methods_2;
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            let source_ml_source2_field122 = owner_rows_2;
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let source_ml_source2_field123 = self::ml_string::Text::from_units(vec![]);
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    let source_ml_source2_field124 = 0f64;
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        let source_ml_source2_field125 = true;
                                                                                                                                                                                                                                        (source_ml_source2_field124, source_ml_source2_field119, source_ml_source2_field125, source_ml_source2_field123.clone(), source_ml_source2_field118, source_ml_source2_field122, source_ml_source2_field121, source_ml_source2_field120, source_ml_source2_field117.clone())
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
                    }
                }
            }
        }
    }
}

pub fn accept_java_script_closed_class_source_size(units: f64, tokens: f64) -> bool {
    (((((self::ml_math::is_safe_integer(units) && self::ml_math::is_safe_integer(tokens)) && (units >= (0f64))) && (units <= (131072f64))) && (tokens >= (0f64))) && (tokens <= (8192f64)))
}

pub fn read_java_script_closed_class_context_error(shared: bool, externals: f64, constants: f64, bindings: f64, known: bool, module: bool) -> self::ml_string::Text {
    if (((((shared && (externals == (0f64))) && (constants == (0f64))) && (bindings == (0f64))) && known) && !module) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![116, 104, 101, 32, 99, 108, 111, 115, 101, 100, 32, 112, 114, 111, 102, 105, 108, 101, 32, 110, 101, 101, 100, 115, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115, 32, 97, 110, 100, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 108, 111, 99, 97, 108, 32, 109, 111, 100, 117, 108, 101, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 117, 110, 112, 114, 111, 118, 101, 100, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 99, 111, 110, 115, 117, 109, 101, 114, 115])
    }
}

pub fn read_java_script_closed_class_use_error(kind: self::ml_string::Text, operation: self::ml_string::Text, instance: bool, private_field: bool, callable: bool, direct: bool) -> self::ml_string::Text {
    if !instance {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (((kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]))) && !private_field) && (!callable || direct)) {
            self::ml_string::Text::from_units(vec![])
        } else {
            if ((kind == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) && operation.starts_with(&self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 105, 110, 115, 116, 97, 110, 99, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 114, 101, 99, 116, 32, 112, 117, 98, 108, 105, 99, 32, 102, 105, 101, 108, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 45, 114, 101, 116, 97, 105, 110, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 32, 99, 97, 108, 108, 115, 59, 32, 112, 114, 105, 118, 97, 116, 101, 32, 115, 108, 111, 116, 115, 44, 32, 100, 101, 116, 97, 99, 104, 101, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 115, 32, 97, 110, 100, 32, 105, 110, 100, 105, 114, 101, 99, 116, 32, 114, 101, 102, 108, 101, 99, 116, 105, 118, 101, 32, 108, 111, 111, 107, 117, 112, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 32, 102, 117, 108, 108, 32, 111, 98, 106, 101, 99, 116, 32, 109, 111, 100, 101, 108])
            }
        }
    }
}

pub fn accept_java_script_closed_class_own_field(receiver_fields: Vec<self::ml_string::Text>, public_fields: Vec<self::ml_string::Text>, field: self::ml_string::Text) -> bool {
    if ((((public_fields.len() as f64) % 2f64)) != (0f64)) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source2_s126 = self::source_ml_source1_temporary_accept_java_script_closed_class_own_field_loop68(receiver_fields, public_fields, field.clone(), at);
                match source_ml_source2_s126.clone() {
                    self::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Done => {
                        false
                    }
                    self::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Return(source_ml_source1_temporary_result68) => {
                        source_ml_source1_temporary_result68
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_own_field_use_error(kind: self::ml_string::Text, operation: self::ml_string::Text, instance: bool, private_field: bool, callable: bool, direct: bool, own_field: bool) -> self::ml_string::Text {
    if (((instance && (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100])))) && !private_field) && own_field) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::read_java_script_closed_class_use_error(kind.clone(), operation.clone(), instance, private_field, callable, direct)
    }
}

pub fn project_java_script_class_source_interval(intervals: Vec<f64>, first: f64, last: f64) -> Vec<f64> {
    {
        let low = (-1f64);
        {
            let high = (-1f64);
            {
                let at = 0f64;
                {
                    let source_ml_source2_s127 = self::source_ml_source1_temporary_project_java_script_class_source_interval_loop69(intervals, first, last, low, high, at);
                    match source_ml_source2_s127.clone() {
                        self::MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Result::MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Done(low_2, high_2) => {
                            if (low_2 < (0f64)) {
                                Vec::<f64>::new()
                            } else {
                                vec![low_2, high_2]
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result {
    MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Done,
    MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return(f64),
}

pub fn source_ml_source1_temporary_find_java_script_class_group_end_loop1(mut values: Vec<self::ml_string::Text>, mut open: f64, mut expected: Vec<self::ml_string::Text>, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result {
    loop {
        return if ((at < ((values.len() as f64))) && (((at - open)) <= (8192f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let closing = if (value == (self::ml_string::Text::from_units(vec![40]))) {
                        self::ml_string::Text::from_units(vec![41])
                    } else {
                        if (value == (self::ml_string::Text::from_units(vec![91]))) {
                            self::ml_string::Text::from_units(vec![93])
                        } else {
                            if (value == (self::ml_string::Text::from_units(vec![123]))) {
                                self::ml_string::Text::from_units(vec![125])
                            } else {
                                self::ml_string::Text::from_units(vec![])
                            }
                        }
                    };
                    if (closing != (self::ml_string::Text::from_units(vec![]))) {
                        if (((expected.len() as f64)) >= (128f64)) {
                            self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return((-1f64))
                        } else {
                            {
                                let expected_2 = {
                                    let source_ml_source2_concat_value128 = expected;
                                    {
                                        let source_ml_source2_concat_value129 = vec![closing.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value128), source_ml_source2_concat_value129)
                                    }
                                };
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        (expected, at) = (expected_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        if (((value == (self::ml_string::Text::from_units(vec![41]))) || (value == (self::ml_string::Text::from_units(vec![93])))) || (value == (self::ml_string::Text::from_units(vec![125])))) {
                            if (value != ((self::ml_array::optional_at(&expected, self::ml_array::number_index(((expected.len() as f64) - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))) {
                                self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return((-1f64))
                            } else {
                                {
                                    let expected_3 = self::ml_array::slice(&expected, 0f64, (-1f64));
                                    if (((expected_3.len() as f64)) == (0f64)) {
                                        self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return(at)
                                    } else {
                                        {
                                            let at_3 = (at + 1f64);
                                            {
                                                (expected, at) = (expected_3, at_3);
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
            self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result {
    MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Done(Vec<f64>),
    MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3 {
    MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(f64),
    MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_parameter_positions_with_collections_loop2(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut close: f64, mut collections: bool, mut result: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result {
    loop {
        return if (at < close) {
            if (((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) {
                self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(vec![(-1f64), at])
            } else {
                {
                    let result_2 = {
                        let source_ml_source2_concat_value130 = result;
                        {
                            let source_ml_source2_concat_value131 = vec![at];
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value130), source_ml_source2_concat_value131)
                        }
                    };
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_source2_s132 = if (((self::ml_array::optional_at(&values, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) {
                                {
                                    let at_3 = (at_2 + 1f64);
                                    {
                                        let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                        if ((value == (self::ml_string::Text::from_units(vec![43]))) || (value == (self::ml_string::Text::from_units(vec![45])))) {
                                            {
                                                let at_4 = (at_3 + 1f64);
                                                if ((((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at_4))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) && !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![78, 97, 78]), self::ml_string::Text::from_units(vec![73, 110, 102, 105, 110, 105, 116, 121])], &(self::ml_array::optional_at(&values, self::ml_array::number_index(at_4))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b))) {
                                                    self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Return(vec![(-1f64), at_4])
                                                } else {
                                                    {
                                                        let at_5 = (at_4 + 1f64);
                                                        self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_5)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (((((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) || (((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![116, 114, 117, 101]), self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101]), self::ml_string::Text::from_units(vec![110, 117, 108, 108]), self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]), self::ml_string::Text::from_units(vec![78, 97, 78]), self::ml_string::Text::from_units(vec![73, 110, 102, 105, 110, 105, 116, 121])], &value.clone(), 0f64, false, |a, b| a == b)) {
                                                {
                                                    let at_6 = (at_3 + 1f64);
                                                    self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_6)
                                                }
                                            } else {
                                                if (((value == (self::ml_string::Text::from_units(vec![91]))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![93])))) || ((value == (self::ml_string::Text::from_units(vec![123]))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![125]))))) {
                                                    {
                                                        let at_7 = (at_3 + 2f64);
                                                        self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_7)
                                                    }
                                                } else {
                                                    if ((((collections && (value == (self::ml_string::Text::from_units(vec![110, 101, 119])))) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])], &(self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 3f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![41])))) {
                                                        {
                                                            let at_8 = (at_3 + 4f64);
                                                            self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_8)
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Return(vec![(-1f64), at_3])
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_2)
                            };
                            match source_ml_source2_s132.clone() {
                                self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_9) => {
                                    if (at_9 < close) {
                                        if (((self::ml_array::optional_at(&values, self::ml_array::number_index(at_9))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![44]))) {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(vec![(-1f64), at_9])
                                        } else {
                                            {
                                                let at_10 = (at_9 + 1f64);
                                                {
                                                    (result, at) = (result_2, at_10);
                                                    continue;
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            (result, at) = (result_2, at_9);
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Return(source_ml_source1_temporary_result3) => {
                                    self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(source_ml_source1_temporary_result3)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Done(result)
        };
    }
}

pub fn source_ml_source1_temporary_render_java_script_closed_class_parameter_arguments_loop4(mut source: self::ml_string::Text, mut values: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut close: f64, mut result: Vec<self::ml_string::Text>, mut at: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (at < close) {
            {
                let result_2 = {
                    let source_ml_source2_concat_value138 = result;
                    {
                        let source_ml_source2_concat_value139 = vec![{
                            let source_ml_source2_string_range_operand133 = source.clone();
                            {
                                let source_ml_source2_string_range_operand134 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                                {
                                    let source_ml_source2_string_range_operand135 = (self::ml_array::optional_at(&ends, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                                    {
                                        let source_ml_source2_string_range_position136 = source_ml_source2_string_range_operand134;
                                        {
                                            let source_ml_source2_string_range_position137 = source_ml_source2_string_range_operand135;
                                            source_ml_source2_string_range_operand133.slice(source_ml_source2_string_range_position136, source_ml_source2_string_range_position137)
                                        }
                                    }
                                }
                            }
                        }];
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value138), source_ml_source2_concat_value139)
                    }
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        let at_8 = if (((self::ml_array::optional_at(&values, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) {
                            {
                                let at_3 = (at_2 + 1f64);
                                {
                                    let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                    if ((value == (self::ml_string::Text::from_units(vec![91]))) || (value == (self::ml_string::Text::from_units(vec![123])))) {
                                        {
                                            let at_4 = (at_3 + 2f64);
                                            at_4
                                        }
                                    } else {
                                        if ((value == (self::ml_string::Text::from_units(vec![43]))) || (value == (self::ml_string::Text::from_units(vec![45])))) {
                                            {
                                                let at_5 = (at_3 + 2f64);
                                                at_5
                                            }
                                        } else {
                                            if ((((value == (self::ml_string::Text::from_units(vec![110, 101, 119]))) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])], &(self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 3f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![41])))) {
                                                {
                                                    let at_6 = (at_3 + 4f64);
                                                    at_6
                                                }
                                            } else {
                                                {
                                                    let at_7 = (at_3 + 1f64);
                                                    at_7
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            at_2
                        };
                        if (at_8 < close) {
                            {
                                let at_9 = (at_8 + 1f64);
                                {
                                    (result, at) = (result_2, at_9);
                                    continue;
                                }
                            }
                        } else {
                            {
                                (result, at) = (result_2, at_8);
                                continue;
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
pub enum MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Result {
    MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Done,
    MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Return(f64),
}

pub fn source_ml_source1_temporary_find_java_script_closed_class_initialization_start_loop5(mut values: Vec<self::ml_string::Text>, mut first: f64, mut last: f64, mut depth: f64, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Result {
    loop {
        return if (at < last) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let previous = (self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if (((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (previous != (self::ml_string::Text::from_units(vec![46])))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) {
                        {
                            let boundary = (((at == first) || (previous == (self::ml_string::Text::from_units(vec![59])))) || (previous == (self::ml_string::Text::from_units(vec![125]))));
                            self::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Result::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Return(if ((((depth == (0f64)) && boundary) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 3f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61])))) {
                                at
                            } else {
                                first
                            })
                        }
                    } else {
                        {
                            let depth_3 = if (((value == (self::ml_string::Text::from_units(vec![40]))) || (value == (self::ml_string::Text::from_units(vec![91])))) || (value == (self::ml_string::Text::from_units(vec![123])))) {
                                {
                                    let depth_2 = (depth + 1f64);
                                    depth_2
                                }
                            } else {
                                depth
                            };
                            if (((value == (self::ml_string::Text::from_units(vec![41]))) || (value == (self::ml_string::Text::from_units(vec![93])))) || (value == (self::ml_string::Text::from_units(vec![125])))) {
                                {
                                    let depth_4 = (depth_3 - 1f64);
                                    {
                                        let at_2 = (at + 1f64);
                                        {
                                            (depth, at) = (depth_4, at_2);
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at_3 = (at + 1f64);
                                    {
                                        (depth, at) = (depth_3, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Result::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result {
    MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Done,
    MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Result {
    MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Done,
    MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Return(f64),
}

pub fn source_ml_source1_temporary_find_java_script_closed_class_field_end_loop7(mut values: Vec<self::ml_string::Text>, mut end: f64, mut inner: f64) -> self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Result {
    loop {
        return if (inner < end) {
            {
                let token = (self::ml_array::optional_at(&values, self::ml_array::number_index(inner))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![116, 104, 105, 115]), self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115]), self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116]), self::ml_string::Text::from_units(vec![121, 105, 101, 108, 100]), self::ml_string::Text::from_units(vec![61, 62]), self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])], &token.clone(), 0f64, false, |a, b| a == b) {
                    self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Return((-1f64))
                } else {
                    {
                        let inner_2 = (inner + 1f64);
                        {
                            inner = inner_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Done
        };
    }
}

pub fn source_ml_source1_temporary_find_java_script_closed_class_field_end_loop6(mut values: Vec<self::ml_string::Text>, mut last: f64, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result {
    loop {
        return if (at < last) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if (value == (self::ml_string::Text::from_units(vec![59]))) {
                    self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Return(at)
                } else {
                    if ((((((((value == (self::ml_string::Text::from_units(vec![44]))) || (value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))) || (value == (self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114])))) || (value == (self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115])))) || (value == (self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116])))) || (value == (self::ml_string::Text::from_units(vec![121, 105, 101, 108, 100])))) || (value == (self::ml_string::Text::from_units(vec![61, 62])))) || (value == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))) {
                        self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Return((-1f64))
                    } else {
                        if (((value == (self::ml_string::Text::from_units(vec![40]))) || (value == (self::ml_string::Text::from_units(vec![91])))) || (value == (self::ml_string::Text::from_units(vec![123])))) {
                            {
                                let end = self::find_java_script_class_group_end(values.clone(), at);
                                if ((end < (0f64)) || (end >= last)) {
                                    self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Return((-1f64))
                                } else {
                                    {
                                        let inner = (at + 1f64);
                                        {
                                            let source_ml_source2_s140 = self::source_ml_source1_temporary_find_java_script_closed_class_field_end_loop7(values.clone(), end, inner);
                                            match source_ml_source2_s140.clone() {
                                                self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Done => {
                                                    {
                                                        let at_2 = end;
                                                        {
                                                            let at_3 = (at_2 + 1f64);
                                                            {
                                                                at = at_3;
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Return(source_ml_source1_temporary_result7) => {
                                                    self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Return(source_ml_source1_temporary_result7)
                                                }
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
            self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Result {
    MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Done(self::ml_string::Text, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9 {
    MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(self::ml_string::Text, f64),
}

pub fn source_ml_source1_temporary_render_java_script_closed_class_field_documentation_loop8(mut comment: self::ml_string::Text, mut result: self::ml_string::Text, mut previous: f64, mut at: f64) -> self::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Result {
    loop {
        return if (at >= (0f64)) {
            {
                let before = if (at == (0f64)) {
                    self::ml_string::Text::from_units(vec![32])
                } else {
                    {
                        let source_ml_source2_string_unit_operand141 = comment.clone();
                        {
                            let source_ml_source2_string_unit_operand142 = (at - 1f64);
                            (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                let index = if position.is_nan() {
                                    0f64
                                } else {
                                    position.trunc()
                                };
                                if ((index < (0f64)) || (index >= (text.length()))) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    {
                                        let source_ml_source2_string_range_operand143 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand144 = index;
                                            {
                                                let source_ml_source2_string_range_operand145 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position146 = source_ml_source2_string_range_operand144;
                                                    {
                                                        let source_ml_source2_string_range_position147 = source_ml_source2_string_range_operand145;
                                                        source_ml_source2_string_range_operand143.slice(source_ml_source2_string_range_position146, source_ml_source2_string_range_position147)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } }))(source_ml_source2_string_unit_operand141.clone(), source_ml_source2_string_unit_operand142)
                        }
                    }
                };
                {
                    let after = {
                        let source_ml_source2_string_unit_operand148 = comment.clone();
                        {
                            let source_ml_source2_string_unit_operand149 = (at + 5f64);
                            (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                let index = if position.is_nan() {
                                    0f64
                                } else {
                                    position.trunc()
                                };
                                if ((index < (0f64)) || (index >= (text.length()))) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    {
                                        let source_ml_source2_string_range_operand150 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand151 = index;
                                            {
                                                let source_ml_source2_string_range_operand152 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position153 = source_ml_source2_string_range_operand151;
                                                    {
                                                        let source_ml_source2_string_range_position154 = source_ml_source2_string_range_operand152;
                                                        source_ml_source2_string_range_operand150.slice(source_ml_source2_string_range_position153, source_ml_source2_string_range_position154)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } }))(source_ml_source2_string_unit_operand148.clone(), source_ml_source2_string_unit_operand149)
                        }
                    };
                    {
                        let source_ml_source2_s160 = if (self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![32]), self::ml_string::Text::from_units(vec![9]), self::ml_string::Text::from_units(vec![13]), self::ml_string::Text::from_units(vec![10]), self::ml_string::Text::from_units(vec![42])], &before.clone(), 0f64, false, |a, b| a == b) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![32]), self::ml_string::Text::from_units(vec![9]), self::ml_string::Text::from_units(vec![13]), self::ml_string::Text::from_units(vec![10]), self::ml_string::Text::from_units(vec![123]), self::ml_string::Text::from_units(vec![])], &after.clone(), 0f64, false, |a, b| a == b)) {
                            {
                                let result_2 = result.concat(&({
                                    let source_ml_source2_string_range_operand161 = comment.clone();
                                    {
                                        let source_ml_source2_string_range_operand162 = previous;
                                        {
                                            let source_ml_source2_string_range_operand163 = at;
                                            {
                                                let source_ml_source2_string_range_position164 = source_ml_source2_string_range_operand162;
                                                {
                                                    let source_ml_source2_string_range_position165 = source_ml_source2_string_range_operand163;
                                                    source_ml_source2_string_range_operand161.slice(source_ml_source2_string_range_position164, source_ml_source2_string_range_position165)
                                                }
                                            }
                                        }
                                    }
                                }).concat(&self::ml_string::Text::from_units(vec![64, 114, 101, 116, 117, 114, 110, 115])));
                                {
                                    let previous_2 = (at + 5f64);
                                    self::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(result_2.clone(), previous_2)
                                }
                            }
                        } else {
                            self::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(result.clone(), previous)
                        };
                        match source_ml_source2_s160.clone() {
                            self::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(result_3, previous_3) => {
                                {
                                    let at_2 = {
                                        let source_ml_source2_string_search_operand166 = comment.clone();
                                        {
                                            let source_ml_source2_string_search_operand167 = self::ml_string::Text::from_units(vec![64, 116, 121, 112, 101]);
                                            {
                                                let source_ml_source2_string_search_operand168 = (at + 5f64);
                                                {
                                                    let source_ml_source2_string_search_text169 = source_ml_source2_string_search_operand167.clone();
                                                    {
                                                        let source_ml_source2_string_search_position170 = source_ml_source2_string_search_operand168;
                                                        source_ml_source2_string_search_operand166.index_of(&source_ml_source2_string_search_text169, source_ml_source2_string_search_position170)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        (result, previous, at) = (result_3.clone(), previous_3, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Result::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Done(result.clone(), previous)
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_prefix_end_loop10(mut row: Vec<f64>, mut end: f64, mut fields: f64, mut at: f64) -> f64 {
    loop {
        return if (at < ((9f64 + (fields * 3f64)))) {
            if (((self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) {
                {
                    let end_2 = ((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| 0f64) + 1f64);
                    {
                        let at_2 = (at + 3f64);
                        {
                            (end, at) = (end_2, at_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 3f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            end
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result {
    MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Done,
    MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_assigned_field_position_loop11(mut values: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut name: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result {
    loop {
        return if (at < ((9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64))))) {
            if ((((self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == name)) {
                self::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Return(at)
            } else {
                {
                    let at_2 = (at + 3f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14 {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Next,
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Result {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Done,
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop15(mut values: Vec<self::ml_string::Text>, mut body_end: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Result {
    loop {
        return if (at < body_end) {
            if (((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]))) {
                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Return(vec![(-6f64), at])
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
            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Done(Vec<f64>),
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop16(mut values: Vec<self::ml_string::Text>, mut fields: Vec<f64>, mut name: self::ml_string::Text, mut declaration: bool, mut field: f64) -> bool {
    loop {
        return if (field < ((fields.len() as f64))) {
            if ((((self::ml_array::optional_at(&fields, self::ml_array::number_index(field))).clone().unwrap_or_else(|| 0f64)) < (0f64)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index(((-((self::ml_array::optional_at(&fields, self::ml_array::number_index(field))).clone().unwrap_or_else(|| 0f64))) - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == name)) {
                {
                    let declaration_2 = true;
                    {
                        let field_2 = (field + 3f64);
                        {
                            (declaration, field) = (declaration_2, field_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let field_3 = (field + 3f64);
                    {
                        field = field_3;
                        continue;
                    }
                }
            }
        } else {
            declaration
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Done(f64),
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Result {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Done,
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop19(mut values: Vec<self::ml_string::Text>, mut group: f64, mut inner: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Result {
    loop {
        return if (inner < group) {
            if (((((self::ml_array::optional_at(&values, self::ml_array::number_index(inner))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((inner - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((inner - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![63, 46])))) {
                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Return(vec![(-5f64), inner])
            } else {
                {
                    let inner_2 = (inner + 1f64);
                    {
                        inner = inner_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop18(mut values: Vec<self::ml_string::Text>, mut body_end: f64, mut finish: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result {
    loop {
        return if (finish < body_end) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(finish))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if (value == (self::ml_string::Text::from_units(vec![59]))) {
                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Done(finish)
                } else {
                    if (value == (self::ml_string::Text::from_units(vec![44]))) {
                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Return(vec![(-5f64), finish])
                    } else {
                        if (((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((finish - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((finish - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![63, 46])))) {
                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Return(vec![(-5f64), finish])
                        } else {
                            if (((value == (self::ml_string::Text::from_units(vec![40]))) || (value == (self::ml_string::Text::from_units(vec![91])))) || (value == (self::ml_string::Text::from_units(vec![123])))) {
                                {
                                    let group = self::find_java_script_class_group_end(values.clone(), finish);
                                    if ((group < (0f64)) || (group >= body_end)) {
                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Return(vec![(-5f64), finish])
                                    } else {
                                        {
                                            let inner = (finish + 1f64);
                                            {
                                                let source_ml_source2_s171 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop19(values.clone(), group, inner);
                                                match source_ml_source2_s171.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Done => {
                                                        {
                                                            let finish_2 = group;
                                                            {
                                                                let finish_3 = (finish_2 + 1f64);
                                                                {
                                                                    finish = finish_3;
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Return(source_ml_source1_temporary_result19) => {
                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Return(source_ml_source1_temporary_result19)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let finish_4 = (finish + 1f64);
                                    {
                                        finish = finish_4;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Done(finish)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Result {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Done,
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop17(mut values: Vec<self::ml_string::Text>, mut fields: Vec<f64>, mut cursor: f64, mut name: self::ml_string::Text, mut previous: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Result {
    loop {
        return if (previous < ((fields.len() as f64))) {
            if ((((self::ml_array::optional_at(&fields, self::ml_array::number_index(previous))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((self::ml_array::optional_at(&fields, self::ml_array::number_index(previous))).clone().unwrap_or_else(|| 0f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == name)) {
                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Return(vec![(-4f64), (cursor + 2f64)])
            } else {
                {
                    let previous_2 = (previous + 3f64);
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop13(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut fields: Vec<f64>, mut names: Vec<self::ml_string::Text>, mut body_start: f64, mut body_end: f64, mut cursor: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result {
    loop {
        return if ((((((body_start >= (0f64)) && (cursor < body_end)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index(cursor))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((cursor + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&kinds, self::ml_array::number_index((cursor + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((cursor + 3f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61])))) {
            {
                let name = (self::ml_array::optional_at(&values, self::ml_array::number_index((cursor + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let declaration = false;
                    {
                        let field = 0f64;
                        {
                            let declaration_2 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop16(values.clone(), fields.clone(), name.clone(), declaration, field);
                            if ((name == (self::ml_string::Text::from_units(vec![36]))) || (self::ml_array::includes(&names, &(self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 58])).concat(&name), 0f64, false, |a, b| a == b) && !declaration_2)) {
                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Return(vec![(-4f64), (cursor + 2f64)])
                            } else {
                                {
                                    let previous = 0f64;
                                    {
                                        let source_ml_source2_s172 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop17(values.clone(), fields.clone(), cursor, name.clone(), previous);
                                        match source_ml_source2_s172.clone() {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Done => {
                                                {
                                                    let start = (cursor + 4f64);
                                                    {
                                                        let finish = start;
                                                        {
                                                            let source_ml_source2_s173 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop18(values.clone(), body_end, finish);
                                                            match source_ml_source2_s173.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Done(finish_2) => {
                                                                    if ((finish_2 == start) || (finish_2 >= body_end)) {
                                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Return(vec![(-5f64), start])
                                                                    } else {
                                                                        {
                                                                            let fields_2 = {
                                                                                let source_ml_source2_concat_value174 = fields;
                                                                                {
                                                                                    let source_ml_source2_concat_value175 = vec![(cursor + 2f64), start, finish_2];
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value174), source_ml_source2_concat_value175)
                                                                                }
                                                                            };
                                                                            {
                                                                                let cursor_2 = (finish_2 + 1f64);
                                                                                {
                                                                                    (fields, cursor) = (fields_2, cursor_2);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Return(source_ml_source1_temporary_result18) => {
                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Return(source_ml_source1_temporary_result18)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Return(source_ml_source1_temporary_result19) => {
                                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Return(source_ml_source1_temporary_result19)
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
            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Done(fields)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Done(Vec<f64>, Vec<f64>, Vec<f64>, Vec<self::ml_string::Text>),
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20 {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20Next(Vec<f64>, Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop12(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut end: f64, mut constructor: Vec<f64>, mut methods: Vec<f64>, mut fields: Vec<f64>, mut names: Vec<self::ml_string::Text>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result {
    loop {
        return if (at < end) {
            if (((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![59]))) {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            } else {
                {
                    let static_member = (((((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99]))) && (((self::ml_array::optional_at(&kinds, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40]))));
                    {
                        let position = if static_member {
                            (at + 1f64)
                        } else {
                            at
                        };
                        {
                            let name = (self::ml_array::optional_at(&values, self::ml_array::number_index(position))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            {
                                let owner = (if static_member {
                                    self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99, 58])
                                } else {
                                    self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 58])
                                }).concat(&name);
                                {
                                    let field = ((((self::ml_array::optional_at(&values, self::ml_array::number_index((position + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((position + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![59]))));
                                    if ((!static_member && field) && (((self::ml_array::optional_at(&kinds, self::ml_array::number_index(position))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) {
                                        if (self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]), self::ml_string::Text::from_units(vec![95, 95, 112, 114, 111, 116, 111, 95, 95]), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101]), self::ml_string::Text::from_units(vec![36])], &name.clone(), 0f64, false, |a, b| a == b) || self::ml_array::includes(&names, &(self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 58])).concat(&name), 0f64, false, |a, b| a == b)) {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![(-2f64), position])
                                        } else {
                                            {
                                                let initial = if (((self::ml_array::optional_at(&values, self::ml_array::number_index((position + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) {
                                                    (position + 2f64)
                                                } else {
                                                    (position + 1f64)
                                                };
                                                {
                                                    let finish = self::find_java_script_closed_class_field_end(values.clone(), initial, end);
                                                    if ((finish < (0f64)) || ((((self::ml_array::optional_at(&values, self::ml_array::number_index((position + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) && (finish == initial))) {
                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![(-10f64), initial])
                                                    } else {
                                                        {
                                                            let fields_2 = {
                                                                let source_ml_source2_concat_value176 = fields;
                                                                {
                                                                    let source_ml_source2_concat_value177 = vec![((-position) - 1f64), initial, finish];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value176), source_ml_source2_concat_value177)
                                                                }
                                                            };
                                                            {
                                                                let names_2 = {
                                                                    let source_ml_source2_concat_value178 = names;
                                                                    {
                                                                        let source_ml_source2_concat_value179 = vec![(self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 58])).concat(&name)];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value178), source_ml_source2_concat_value179)
                                                                    }
                                                                };
                                                                {
                                                                    let at_3 = (finish + 1f64);
                                                                    {
                                                                        (fields, names, at) = (fields_2, names_2, at_3);
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
                                        if (((((((((self::ml_array::optional_at(&kinds, self::ml_array::number_index(position))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) || (name == (self::ml_string::Text::from_units(vec![95, 95, 112, 114, 111, 116, 111, 95, 95])))) || (name == (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101])))) || (name == (self::ml_string::Text::from_units(vec![36])))) || (static_member && (name == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))))) || self::ml_array::includes(&names, &owner.clone(), 0f64, false, |a, b| a == b)) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((position + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![40])))) {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![(-2f64), at])
                                        } else {
                                            {
                                                let parameters = self::find_java_script_class_group_end(values.clone(), (position + 1f64));
                                                if ((parameters < (0f64)) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((parameters + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![123])))) {
                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![(-2f64), at])
                                                } else {
                                                    {
                                                        let literal_positions = self::read_java_script_closed_class_parameter_positions(values.clone(), kinds.clone(), (position + 1f64), parameters);
                                                        {
                                                            let positions = if (((self::ml_array::optional_at(&literal_positions, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64)) < (0f64)) {
                                                                self::read_java_script_closed_class_collection_parameter_positions(values.clone(), kinds.clone(), (position + 1f64), parameters)
                                                            } else {
                                                                literal_positions
                                                            };
                                                            if (((self::ml_array::optional_at(&positions, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64)) < (0f64)) {
                                                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![(-3f64), (self::ml_array::optional_at(&positions, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| at)])
                                                            } else {
                                                                {
                                                                    let body = self::find_java_script_class_group_end(values.clone(), (parameters + 1f64));
                                                                    if ((body < (0f64)) || (body > end)) {
                                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![(-2f64), at])
                                                                    } else {
                                                                        {
                                                                            let names_3 = {
                                                                                let source_ml_source2_concat_value180 = names;
                                                                                {
                                                                                    let source_ml_source2_concat_value181 = vec![owner.clone()];
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value180), source_ml_source2_concat_value181)
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_s184 = if ((name == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && !static_member) {
                                                                                    {
                                                                                        let constructor_2 = vec![position, (position + 1f64), parameters, (parameters + 1f64), body];
                                                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20Next(constructor_2, methods)
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        let methods_2 = {
                                                                                            let source_ml_source2_concat_value185 = methods;
                                                                                            {
                                                                                                let source_ml_source2_concat_value186 = vec![if static_member {
                                                                                                    ((-position) - 1f64)
                                                                                                } else {
                                                                                                    position
                                                                                                }, (position + 1f64), parameters, (parameters + 1f64), body];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value185), source_ml_source2_concat_value186)
                                                                                            }
                                                                                        };
                                                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20Next(constructor, methods_2)
                                                                                    }
                                                                                };
                                                                                match source_ml_source2_s184.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20Next(constructor_3, methods_3) => {
                                                                                        {
                                                                                            let at_4 = (body + 1f64);
                                                                                            {
                                                                                                (constructor, methods, names, at) = (constructor_3, methods_3, names_3, at_4);
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
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Done(constructor, methods, fields, names)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Result {
    MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Done(self::ml_string::Text, f64),
}

pub fn source_ml_source1_temporary_render_java_script_closed_class_range_loop21(mut source: self::ml_string::Text, mut values: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut last: f64, mut receiver: self::ml_string::Text, mut class_names: Vec<self::ml_string::Text>, mut static_calls: Vec<self::ml_string::Text>, mut result: self::ml_string::Text, mut previous: f64, mut at: f64) -> self::MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Result {
    loop {
        return if (at < last) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let target = self::read_java_script_closed_class_static_target(values.clone(), class_names.clone(), static_calls.clone(), at);
                    if (target != (self::ml_string::Text::from_units(vec![]))) {
                        {
                            let result_2 = result.concat(&({
                                let source_ml_source2_string_range_operand187 = source.clone();
                                {
                                    let source_ml_source2_string_range_operand188 = previous;
                                    {
                                        let source_ml_source2_string_range_operand189 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(at))).clone().unwrap_or_else(|| previous);
                                        {
                                            let source_ml_source2_string_range_position190 = source_ml_source2_string_range_operand188;
                                            {
                                                let source_ml_source2_string_range_position191 = source_ml_source2_string_range_operand189;
                                                source_ml_source2_string_range_operand187.slice(source_ml_source2_string_range_position190, source_ml_source2_string_range_position191)
                                            }
                                        }
                                    }
                                }
                            }).concat(&target));
                            {
                                let previous_2 = (self::ml_array::optional_at(&ends, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| previous);
                                {
                                    let at_2 = (at + 2f64);
                                    {
                                        let at_3 = (at_2 + 1f64);
                                        {
                                            (result, previous, at) = (result_2.clone(), previous_2, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let this_reference = ((((receiver != (self::ml_string::Text::from_units(vec![]))) && (value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![63, 46]))));
                            {
                                let construction = (((value == (self::ml_string::Text::from_units(vec![110, 101, 119]))) && self::ml_array::includes(&class_names, &(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40]))));
                                if (this_reference || construction) {
                                    {
                                        let result_3 = result.concat(&({
                                            let source_ml_source2_string_range_operand192 = source.clone();
                                            {
                                                let source_ml_source2_string_range_operand193 = previous;
                                                {
                                                    let source_ml_source2_string_range_operand194 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(at))).clone().unwrap_or_else(|| previous);
                                                    {
                                                        let source_ml_source2_string_range_position195 = source_ml_source2_string_range_operand193;
                                                        {
                                                            let source_ml_source2_string_range_position196 = source_ml_source2_string_range_operand194;
                                                            source_ml_source2_string_range_operand192.slice(source_ml_source2_string_range_position195, source_ml_source2_string_range_position196)
                                                        }
                                                    }
                                                }
                                            }
                                        }).concat(&if this_reference {
                                            receiver.clone()
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        }));
                                        {
                                            let previous_3 = (self::ml_array::optional_at(&ends, self::ml_array::number_index(at))).clone().unwrap_or_else(|| previous);
                                            {
                                                let at_4 = (at + 1f64);
                                                {
                                                    (result, previous, at) = (result_3.clone(), previous_3, at_4);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at_5 = (at + 1f64);
                                        {
                                            at = at_5;
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
            self::MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Result::MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Done(result.clone(), previous)
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_methods_loop22(mut values: Vec<self::ml_string::Text>, mut directory: Vec<f64>, mut result: Vec<self::ml_string::Text>, mut at: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (at < ((directory.len() as f64))) {
            if (((self::ml_array::optional_at(&directory, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) {
                {
                    let result_2 = {
                        let source_ml_source2_concat_value197 = result;
                        {
                            let source_ml_source2_concat_value198 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index((self::ml_array::optional_at(&directory, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value197), source_ml_source2_concat_value198)
                        }
                    };
                    {
                        let at_2 = (at + 5f64);
                        {
                            (result, at) = (result_2, at_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 5f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_static_calls_loop25(mut values: Vec<self::ml_string::Text>, mut prefix: self::ml_string::Text, mut result: Vec<self::ml_string::Text>, mut ordinal: f64, mut row: Vec<f64>, mut name: self::ml_string::Text, mut at: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (at < ((row.len() as f64))) {
            {
                let position = (self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                if (position < (0f64)) {
                    {
                        let result_2 = {
                            let source_ml_source2_concat_value201 = result;
                            {
                                let source_ml_source2_concat_value202 = vec![(name.concat(&self::ml_string::Text::from_units(vec![46]))).concat(&(self::ml_array::optional_at(&values, self::ml_array::number_index(((-position) - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 115, 116, 97, 116, 105, 99, 95]))).concat(&{
                                    let source_ml_source2_string_argument199 = ordinal;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument199))
                                })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                    let source_ml_source2_string_argument200 = at;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument200))
                                })];
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value201), source_ml_source2_concat_value202)
                            }
                        };
                        {
                            let at_2 = (at + 5f64);
                            {
                                (result, at) = (result_2, at_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let at_3 = (at + 5f64);
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_static_calls_loop24(mut values: Vec<self::ml_string::Text>, mut prefix: self::ml_string::Text, mut result: Vec<self::ml_string::Text>, mut ordinal: f64, mut source_ml_source1_temporary_values23: Vec<Vec<f64>>, mut source_ml_source1_temporary_index23: self::ml::Big) -> Vec<self::ml_string::Text> {
    loop {
        return if (source_ml_source1_temporary_index23 < (self::ml::Big::from_u128(source_ml_source1_temporary_values23.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values23, source_ml_source1_temporary_index23.to_index());
                {
                    let name = (self::ml_array::optional_at(&values, self::ml_array::number_index(((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64) + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let at = (9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
                        {
                            let result_2 = self::source_ml_source1_temporary_read_java_script_closed_class_static_calls_loop25(values.clone(), prefix.clone(), result, ordinal, row, name.clone(), at);
                            {
                                let ordinal_2 = (ordinal + 1f64);
                                {
                                    let source_ml_source1_temporary_index23_2 = source_ml_source1_temporary_index23.add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, ordinal, source_ml_source1_temporary_index23) = (result_2, ordinal_2, source_ml_source1_temporary_index23_2.clone());
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Result {
    MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Done,
    MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_static_target_loop26(mut calls: Vec<self::ml_string::Text>, mut key: self::ml_string::Text, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Result {
    loop {
        return if (index < ((calls.len() as f64))) {
            if (((self::ml_array::optional_at(&calls, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == key) {
                self::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Result::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Return((self::ml_array::optional_at(&calls, self::ml_array::number_index((index + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
            } else {
                {
                    let index_2 = (index + 2f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Result::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop29(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_values27: Vec<Vec<f64>>, mut source_ml_source1_temporary_index27: self::ml::Big) -> Vec<self::ml_string::Text> {
    loop {
        return if (source_ml_source1_temporary_index27 < (self::ml::Big::from_u128(source_ml_source1_temporary_values27.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values27, source_ml_source1_temporary_index27.to_index());
                {
                    let names_2 = {
                        let source_ml_source2_concat_value203 = names;
                        {
                            let source_ml_source2_concat_value204 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index(((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64) + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value203), source_ml_source2_concat_value204)
                        }
                    };
                    {
                        let source_ml_source1_temporary_index27_2 = source_ml_source1_temporary_index27.add(&self::ml::Big::from_i128(1));
                        {
                            (names, source_ml_source1_temporary_index27) = (names_2, source_ml_source1_temporary_index27_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            names
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result {
    MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Done,
    MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop31(mut row: Vec<f64>, mut parameters: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((row.len() as f64))) {
            {
                let parameters_2 = {
                    let source_ml_source2_concat_value205 = parameters;
                    {
                        let source_ml_source2_concat_value206 = vec![(self::ml_array::optional_at(&row, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64)), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| (-1f64))];
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value205), source_ml_source2_concat_value206)
                    }
                };
                {
                    let at_2 = (at + 5f64);
                    {
                        (parameters, at) = (parameters_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            parameters
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result {
    MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Done,
    MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result {
    MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Done,
    MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop33(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut parameters: Vec<f64>, mut pair: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result {
    loop {
        return if (at < ((self::ml_array::optional_at(&parameters, self::ml_array::number_index((pair + 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
            if ((((((self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![110, 101, 119])))) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])], &(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) && self::ml_array::includes(&names, &(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) {
                self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Return((at + 1f64))
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
            self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop32(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut parameters: Vec<f64>, mut pair: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result {
    loop {
        return if (pair < ((parameters.len() as f64))) {
            {
                let at = ((self::ml_array::optional_at(&parameters, self::ml_array::number_index(pair))).clone().unwrap_or_else(|| (-1f64)) + 1f64);
                {
                    let source_ml_source2_s207 = self::source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop33(values.clone(), names.clone(), parameters.clone(), pair, at);
                    match source_ml_source2_s207.clone() {
                        self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Done => {
                            {
                                let pair_2 = (pair + 2f64);
                                {
                                    pair = pair_2;
                                    continue;
                                }
                            }
                        }
                        self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Return(source_ml_source1_temporary_result33) => {
                            self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Return(source_ml_source1_temporary_result33)
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop30(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_values28: Vec<Vec<f64>>, mut source_ml_source1_temporary_index28: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result {
    loop {
        return if (source_ml_source1_temporary_index28 < (self::ml::Big::from_u128(source_ml_source1_temporary_values28.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values28, source_ml_source1_temporary_index28.to_index());
                {
                    let parameters = vec![(self::ml_array::optional_at(&row, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| (-1f64)), (self::ml_array::optional_at(&row, self::ml_array::number_index(4f64))).clone().unwrap_or_else(|| (-1f64))];
                    {
                        let at = (9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
                        {
                            let parameters_2 = self::source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop31(row, parameters, at);
                            {
                                let pair = 0f64;
                                {
                                    let source_ml_source2_s208 = self::source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop32(values.clone(), names.clone(), parameters_2, pair);
                                    match source_ml_source2_s208.clone() {
                                        self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Done => {
                                            {
                                                let source_ml_source1_temporary_index28_2 = source_ml_source1_temporary_index28.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index28 = source_ml_source1_temporary_index28_2.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Return(source_ml_source1_temporary_result32) => {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Return(source_ml_source1_temporary_result32)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_function_exports_loop34(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut result: Vec<self::ml_string::Text>, mut at: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (at < ((values.len() as f64))) {
            if ((((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && self::accept_java_script_closed_class_function_export(values.clone(), kinds.clone(), at)) {
                {
                    let result_2 = {
                        let source_ml_source2_concat_value209 = result;
                        {
                            let source_ml_source2_concat_value210 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value209), source_ml_source2_concat_value210)
                        }
                    };
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
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
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
pub enum MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result {
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Done,
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39 {
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Next,
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Result {
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Done,
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop40(mut at: f64, mut source_ml_source1_temporary_values36: Vec<Vec<f64>>, mut source_ml_source1_temporary_index36: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Result {
    loop {
        return if (source_ml_source1_temporary_index36 < (self::ml::Big::from_u128(source_ml_source1_temporary_values36.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values36, source_ml_source1_temporary_index36.to_index());
                if ((at > ((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64))) && (at < ((self::ml_array::optional_at(&row, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| 0f64)))) {
                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(at)
                } else {
                    {
                        let source_ml_source1_temporary_index36_2 = source_ml_source1_temporary_index36.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index36 = source_ml_source1_temporary_index36_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop38(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut directory: Vec<Vec<f64>>, mut static_calls: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut methods: Vec<self::ml_string::Text>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let kind = (self::ml_array::optional_at(&kinds, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let previous = (self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let next = (self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            if ((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116]), self::ml_string::Text::from_units(vec![74, 83, 79, 78]), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108]), self::ml_string::Text::from_units(vec![80, 114, 111, 120, 121]), self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![101, 118, 97, 108]), self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115]), self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]), self::ml_string::Text::from_units(vec![100, 101, 101, 112, 69, 113, 117, 97, 108]), self::ml_string::Text::from_units(vec![110, 111, 116, 68, 101, 101, 112, 69, 113, 117, 97, 108]), self::ml_string::Text::from_units(vec![100, 101, 101, 112, 83, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108]), self::ml_string::Text::from_units(vec![110, 111, 116, 68, 101, 101, 112, 83, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108])], &value.clone(), 0f64, false, |a, b| a == b)) {
                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                            } else {
                                if (((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && (value == (self::ml_string::Text::from_units(vec![101, 120, 112, 111, 114, 116])))) && !(self::accept_java_script_closed_class_function_export(values.clone(), kinds.clone(), at))) {
                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                } else {
                                    if (((value == (self::ml_string::Text::from_units(vec![46, 46, 46]))) || (value == (self::ml_string::Text::from_units(vec![95, 95, 112, 114, 111, 116, 111, 95, 95])))) || (value == (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101])))) {
                                        self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                    } else {
                                        {
                                            let source_ml_source2_s212 = if (value == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
                                                {
                                                    let source_ml_source1_temporary_values36 = directory.clone();
                                                    {
                                                        let source_ml_source1_temporary_index36 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s213 = self::source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop40(at, source_ml_source1_temporary_values36, source_ml_source1_temporary_index36.clone());
                                                            match source_ml_source2_s213.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Done => {
                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Next
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(source_ml_source1_temporary_result40) => {
                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Return(source_ml_source1_temporary_result40)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Next
                                            };
                                            match source_ml_source2_s212.clone() {
                                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Next => {
                                                    if (((((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && self::ml_array::includes(&names, &value.clone(), 0f64, false, |a, b| a == b)) && (previous != (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))) && ((previous != (self::ml_string::Text::from_units(vec![110, 101, 119]))) || (next != (self::ml_string::Text::from_units(vec![40]))))) && ((self::read_java_script_closed_class_static_target(values.clone(), names.clone(), static_calls.clone(), at)) == (self::ml_string::Text::from_units(vec![])))) {
                                                        self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                                    } else {
                                                        if (((((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (previous != (self::ml_string::Text::from_units(vec![46])))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) && (next != (self::ml_string::Text::from_units(vec![46])))) && (next != (self::ml_string::Text::from_units(vec![91])))) {
                                                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                                        } else {
                                                            if (((previous == (self::ml_string::Text::from_units(vec![46]))) || (previous == (self::ml_string::Text::from_units(vec![63, 46])))) && ((self::ml_array::includes(&methods, &value.clone(), 0f64, false, |a, b| a == b) && (next != (self::ml_string::Text::from_units(vec![40])))) || (value == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))))) {
                                                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                                            } else {
                                                                if (((previous == (self::ml_string::Text::from_units(vec![91]))) && (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && self::ml_array::includes(&methods, &{
                                                                    let source_ml_source2_string_range_operand214 = value.clone();
                                                                    {
                                                                        let source_ml_source2_string_range_operand215 = 1f64;
                                                                        {
                                                                            let source_ml_source2_string_range_position216 = source_ml_source2_string_range_operand215;
                                                                            {
                                                                                let source_ml_source2_string_range_position217 = f64::INFINITY;
                                                                                source_ml_source2_string_range_operand214.slice(source_ml_source2_string_range_position216, source_ml_source2_string_range_position217)
                                                                            }
                                                                        }
                                                                    }
                                                                }, 0f64, false, |a, b| a == b)) {
                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
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
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Return(source_ml_source1_temporary_result39) => {
                                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(source_ml_source1_temporary_result39)
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
            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Result {
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Done(Vec<self::ml_string::Text>, Vec<self::ml_string::Text>),
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Result {
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Done,
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Result {
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Done,
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop42(mut values: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut at: f64, mut inner: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Result {
    loop {
        return if (inner < ((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 4f64)))).clone().unwrap_or_else(|| 0f64))) {
            if (((((self::ml_array::optional_at(&values, self::ml_array::number_index(inner))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((inner - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((inner - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![63, 46])))) {
                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Return(inner)
            } else {
                {
                    let inner_2 = (inner + 1f64);
                    {
                        inner = inner_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop41(mut values: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Result {
    loop {
        return if (at < ((row.len() as f64))) {
            if (((self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)) < (0f64)) {
                {
                    let inner = ((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 3f64)))).clone().unwrap_or_else(|| 0f64) + 1f64);
                    {
                        let source_ml_source2_s218 = self::source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop42(values.clone(), row.clone(), at, inner);
                        match source_ml_source2_s218.clone() {
                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Done => {
                                {
                                    let at_2 = (at + 5f64);
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Return(source_ml_source1_temporary_result42) => {
                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Return(source_ml_source1_temporary_result42)
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 5f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop37(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut methods: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_values35: Vec<Vec<f64>>, mut source_ml_source1_temporary_index35: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Result {
    loop {
        return if (source_ml_source1_temporary_index35 < (self::ml::Big::from_u128(source_ml_source1_temporary_values35.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values35, source_ml_source1_temporary_index35.to_index());
                {
                    let names_2 = {
                        let source_ml_source2_concat_value219 = names;
                        {
                            let source_ml_source2_concat_value220 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index(((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64) + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value219), source_ml_source2_concat_value220)
                        }
                    };
                    {
                        let methods_2 = {
                            let source_ml_source2_concat_value221 = methods;
                            {
                                let source_ml_source2_concat_value222 = self::read_java_script_closed_class_methods(values.clone(), row.clone());
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value221), source_ml_source2_concat_value222)
                            }
                        };
                        {
                            let at = (9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
                            {
                                let source_ml_source2_s223 = self::source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop41(values.clone(), row, at);
                                match source_ml_source2_s223.clone() {
                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Done => {
                                        {
                                            let source_ml_source1_temporary_index35_2 = source_ml_source1_temporary_index35.add(&self::ml::Big::from_i128(1));
                                            {
                                                (names, methods, source_ml_source1_temporary_index35) = (names_2, methods_2, source_ml_source1_temporary_index35_2.clone());
                                                continue;
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Return(source_ml_source1_temporary_result41) => {
                                        self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Return(source_ml_source1_temporary_result41)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Done(names, methods)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassLoop44Result {
    MlSource1TemporaryRenderJavaScriptClosedClassLoop44Done(self::ml_string::Text, self::ml_string::Text, self::ml_string::Text),
}

pub fn source_ml_source1_temporary_render_java_script_closed_class_loop44(mut source: self::ml_string::Text, mut values: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut row: Vec<f64>, mut prefix: self::ml_string::Text, mut ordinal: f64, mut class_names: Vec<self::ml_string::Text>, mut static_calls: Vec<self::ml_string::Text>, mut prototype: bool, mut receiver: self::ml_string::Text, mut stubs: self::ml_string::Text, mut fields: self::ml_string::Text, mut assignments: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryRenderJavaScriptClosedClassLoop44Result {
    loop {
        return if (at < ((row.len() as f64))) {
            {
                let member = (self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let position = if (member < (0f64)) {
                        ((-member) - 1f64)
                    } else {
                        member
                    };
                    {
                        let method = (self::ml_array::optional_at(&values, self::ml_array::number_index(position))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let args = self::render_java_script_closed_class_range(source.clone(), values.clone(), starts.clone(), ends.clone(), ((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| 0f64) + 1f64), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| 0f64), self::ml_string::Text::from_units(vec![]), class_names.clone(), static_calls.clone());
                            {
                                let arguments_ = self::render_java_script_closed_class_parameter_arguments(source.clone(), values.clone(), starts.clone(), ends.clone(), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| 0f64), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| 0f64));
                                {
                                    let body = self::render_java_script_closed_class_range(source.clone(), values.clone(), starts.clone(), ends.clone(), ((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 3f64)))).clone().unwrap_or_else(|| 0f64) + 1f64), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 4f64)))).clone().unwrap_or_else(|| 0f64), if (member < (0f64)) {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        receiver.clone()
                                    }, class_names.clone(), static_calls.clone());
                                    {
                                        let stub = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 117, 110, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 95]))).concat(&{
                                            let source_ml_source2_string_argument224 = ordinal;
                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument224))
                                        })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                            let source_ml_source2_string_argument225 = at;
                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument225))
                                        });
                                        {
                                            let declaration = if (member < (0f64)) {
                                                (position - 1f64)
                                            } else {
                                                position
                                            };
                                            {
                                                let documentation = {
                                                    let source_ml_source2_string_range_operand226 = source.clone();
                                                    {
                                                        let source_ml_source2_string_range_operand227 = (self::ml_array::optional_at(&ends, self::ml_array::number_index((declaration - 1f64)))).clone().unwrap_or_else(|| 0f64);
                                                        {
                                                            let source_ml_source2_string_range_operand228 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(declaration))).clone().unwrap_or_else(|| 0f64);
                                                            {
                                                                let source_ml_source2_string_range_position229 = source_ml_source2_string_range_operand227;
                                                                {
                                                                    let source_ml_source2_string_range_position230 = source_ml_source2_string_range_operand228;
                                                                    source_ml_source2_string_range_operand226.slice(source_ml_source2_string_range_position229, source_ml_source2_string_range_position230)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                if (member < (0f64)) {
                                                    {
                                                        let stubs_2 = stubs.concat(&((((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&prefix)).concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 115, 116, 97, 116, 105, 99, 95]))).concat(&{
                                                            let source_ml_source2_string_argument231 = ordinal;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument231))
                                                        })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                            let source_ml_source2_string_argument232 = at;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument232))
                                                        })).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32]))).concat(&body)).concat(&self::ml_string::Text::from_units(vec![32, 125, 10])));
                                                        {
                                                            let at_2 = (at + 5f64);
                                                            {
                                                                (stubs, at) = (stubs_2.clone(), at_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if prototype {
                                                        {
                                                            let stubs_3 = stubs.concat(&(((((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&prefix)).concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95]))).concat(&{
                                                                let source_ml_source2_string_argument233 = ordinal;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument233))
                                                            })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                let source_ml_source2_string_argument234 = at;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument234))
                                                            })).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&receiver)).concat(&if (args == (self::ml_string::Text::from_units(vec![]))) {
                                                                self::ml_string::Text::from_units(vec![])
                                                            } else {
                                                                (self::ml_string::Text::from_units(vec![44, 32])).concat(&args)
                                                            })).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32]))).concat(&body)).concat(&self::ml_string::Text::from_units(vec![32, 125, 10])));
                                                            {
                                                                let at_3 = (at + 5f64);
                                                                {
                                                                    (stubs, at) = (stubs_3.clone(), at_3);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {
                                                            let environment = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 95]))).concat(&{
                                                                let source_ml_source2_string_argument235 = ordinal;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument235))
                                                            })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                let source_ml_source2_string_argument236 = at;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument236))
                                                            });
                                                            {
                                                                let stubs_4 = stubs.concat(&((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&stub)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 32]))).concat(&stub)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&arguments_)).concat(&self::ml_string::Text::from_units(vec![41, 59, 32, 125, 10])));
                                                                {
                                                                    let stubs_5 = stubs_4.concat(&((((((((self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32])).concat(&environment)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 32, 40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 61, 62, 32, 123, 32]))).concat(&body)).concat(&self::ml_string::Text::from_units(vec![32, 125, 59, 32, 125, 10])));
                                                                    {
                                                                        let fields_2 = fields.concat(&(((self::ml_string::Text::from_units(vec![44, 32])).concat(&method)).concat(&self::ml_string::Text::from_units(vec![58, 32]))).concat(&stub));
                                                                        {
                                                                            let assignments_2 = assignments.concat(&((((((((self::ml_string::Text::from_units(vec![10])).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![46]))).concat(&method)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&environment)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![41, 59, 10])));
                                                                            {
                                                                                let at_4 = (at + 5f64);
                                                                                {
                                                                                    (stubs, fields, assignments, at) = (stubs_5.clone(), fields_2.clone(), assignments_2.clone(), at_4);
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
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryRenderJavaScriptClosedClassLoop44Result::MlSource1TemporaryRenderJavaScriptClosedClassLoop44Done(stubs.clone(), fields.clone(), assignments.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassLoop43Result {
    MlSource1TemporaryRenderJavaScriptClosedClassLoop43Done(self::ml_string::Text, self::ml_string::Text, self::ml_string::Text, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassJoin46 {
    MlSource1TemporaryRenderJavaScriptClosedClassJoin46Next(self::ml_string::Text, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassJoin45 {
    MlSource1TemporaryRenderJavaScriptClosedClassJoin45Next(self::ml_string::Text, self::ml_string::Text, self::ml_string::Text),
}

pub fn source_ml_source1_temporary_render_java_script_closed_class_loop43(mut source: self::ml_string::Text, mut values: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut row: Vec<f64>, mut prefix: self::ml_string::Text, mut ordinal: f64, mut class_names: Vec<self::ml_string::Text>, mut static_calls: Vec<self::ml_string::Text>, mut methods: f64, mut stubs: self::ml_string::Text, mut initializations: self::ml_string::Text, mut constructor_fields: self::ml_string::Text, mut field_names: Vec<self::ml_string::Text>, mut fields: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryRenderJavaScriptClosedClassLoop43Result {
    loop {
        return if (at < methods) {
            {
                let position = (self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let field = (self::ml_array::optional_at(&values, self::ml_array::number_index(if (position < (0f64)) {
                        ((-position) - 1f64)
                    } else {
                        position
                    }))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let value = if (((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| 0f64)) == ((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| 0f64))) {
                            self::ml_string::Text::from_units(vec![40, 118, 111, 105, 100, 32, 48, 41])
                        } else {
                            self::render_java_script_closed_class_range(source.clone(), values.clone(), starts.clone(), ends.clone(), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| 0f64), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| 0f64), self::ml_string::Text::from_units(vec![]), class_names.clone(), static_calls.clone())
                        };
                        {
                            let source_ml_source2_s248 = if (position < (0f64)) {
                                {
                                    let initializer = (((prefix.concat(&self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 95, 99, 108, 97, 115, 115, 95, 102, 105, 101, 108, 100, 95]))).concat(&{
                                        let source_ml_source2_string_argument249 = ordinal;
                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument249))
                                    })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                        let source_ml_source2_string_argument250 = at;
                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument250))
                                    });
                                    {
                                        let capture = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101, 95]))).concat(&{
                                            let source_ml_source2_string_argument251 = ordinal;
                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument251))
                                        })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                            let source_ml_source2_string_argument252 = at;
                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument252))
                                        });
                                        {
                                            let documentation = self::render_java_script_closed_class_field_documentation({
                                                let source_ml_source2_string_range_operand253 = source.clone();
                                                {
                                                    let source_ml_source2_string_range_operand254 = (self::ml_array::optional_at(&ends, self::ml_array::number_index(((-position) - 2f64)))).clone().unwrap_or_else(|| 0f64);
                                                    {
                                                        let source_ml_source2_string_range_operand255 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(((-position) - 1f64)))).clone().unwrap_or_else(|| 0f64);
                                                        {
                                                            let source_ml_source2_string_range_position256 = source_ml_source2_string_range_operand254;
                                                            {
                                                                let source_ml_source2_string_range_position257 = source_ml_source2_string_range_operand255;
                                                                source_ml_source2_string_range_operand253.slice(source_ml_source2_string_range_position256, source_ml_source2_string_range_position257)
                                                            }
                                                        }
                                                    }
                                                }
                                            });
                                            {
                                                let stubs_2 = stubs.concat(&((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&initializer)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 32, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 59, 32, 125, 10])));
                                                {
                                                    let initializations_2 = initializations.concat(&((((self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 32])).concat(&capture)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&initializer)).concat(&self::ml_string::Text::from_units(vec![40, 41, 59, 10])));
                                                    {
                                                        let value_2 = capture.clone();
                                                        {
                                                            let assigned = self::read_java_script_closed_class_assigned_field_position(values.clone(), row.clone(), field.clone());
                                                            if (assigned >= (0f64)) {
                                                                {
                                                                    let value_3 = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 97, 115, 115, 105, 103, 110, 101, 100, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101, 95]))).concat(&{
                                                                        let source_ml_source2_string_argument258 = ordinal;
                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument258))
                                                                    })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                        let source_ml_source2_string_argument259 = assigned;
                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument259))
                                                                    });
                                                                    self::MlSource1TemporaryRenderJavaScriptClosedClassJoin45::MlSource1TemporaryRenderJavaScriptClosedClassJoin45Next(stubs_2.clone(), initializations_2.clone(), value_3.clone())
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryRenderJavaScriptClosedClassJoin45::MlSource1TemporaryRenderJavaScriptClosedClassJoin45Next(stubs_2.clone(), initializations_2.clone(), value_2.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryRenderJavaScriptClosedClassJoin45::MlSource1TemporaryRenderJavaScriptClosedClassJoin45Next(stubs.clone(), initializations.clone(), value.clone())
                            };
                            match source_ml_source2_s248.clone() {
                                self::MlSource1TemporaryRenderJavaScriptClosedClassJoin45::MlSource1TemporaryRenderJavaScriptClosedClassJoin45Next(stubs_3, initializations_3, value_4) => {
                                    {
                                        let source_ml_source2_s262 = if (position >= (0f64)) {
                                            {
                                                let capture_2 = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 97, 115, 115, 105, 103, 110, 101, 100, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101, 95]))).concat(&{
                                                    let source_ml_source2_string_argument263 = ordinal;
                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument263))
                                                })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                    let source_ml_source2_string_argument264 = at;
                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument264))
                                                });
                                                {
                                                    let constructor_fields_2 = constructor_fields.concat(&((((self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 32])).concat(&capture_2)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 40]))).concat(&value_4)).concat(&self::ml_string::Text::from_units(vec![41, 59, 10])));
                                                    {
                                                        let value_5 = capture_2.clone();
                                                        self::MlSource1TemporaryRenderJavaScriptClosedClassJoin46::MlSource1TemporaryRenderJavaScriptClosedClassJoin46Next(constructor_fields_2.clone(), value_5.clone())
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryRenderJavaScriptClosedClassJoin46::MlSource1TemporaryRenderJavaScriptClosedClassJoin46Next(constructor_fields.clone(), value_4.clone())
                                        };
                                        match source_ml_source2_s262.clone() {
                                            self::MlSource1TemporaryRenderJavaScriptClosedClassJoin46::MlSource1TemporaryRenderJavaScriptClosedClassJoin46Next(constructor_fields_3, value_6) => {
                                                if self::ml_array::includes(&field_names, &field.clone(), 0f64, false, |a, b| a == b) {
                                                    {
                                                        let at_2 = (at + 3f64);
                                                        {
                                                            (stubs, initializations, constructor_fields, at) = (stubs_3.clone(), initializations_3.clone(), constructor_fields_3.clone(), at_2);
                                                            continue;
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let field_names_2 = {
                                                            let source_ml_source2_concat_value265 = field_names;
                                                            {
                                                                let source_ml_source2_concat_value266 = vec![field.clone()];
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value265), source_ml_source2_concat_value266)
                                                            }
                                                        };
                                                        {
                                                            let fields_2 = fields.concat(&((((self::ml_string::Text::from_units(vec![44, 32])).concat(&field)).concat(&self::ml_string::Text::from_units(vec![58, 32, 40]))).concat(&value_6)).concat(&self::ml_string::Text::from_units(vec![41])));
                                                            {
                                                                let at_3 = (at + 3f64);
                                                                {
                                                                    (stubs, initializations, constructor_fields, field_names, fields, at) = (stubs_3.clone(), initializations_3.clone(), constructor_fields_3.clone(), field_names_2, fields_2.clone(), at_3);
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
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryRenderJavaScriptClosedClassLoop43Result::MlSource1TemporaryRenderJavaScriptClosedClassLoop43Done(stubs.clone(), initializations.clone(), constructor_fields.clone(), fields.clone())
        };
    }
}

pub fn source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop49(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_values47: Vec<Vec<f64>>, mut source_ml_source1_temporary_index47: self::ml::Big) -> Vec<self::ml_string::Text> {
    loop {
        return if (source_ml_source1_temporary_index47 < (self::ml::Big::from_u128(source_ml_source1_temporary_values47.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values47, source_ml_source1_temporary_index47.to_index());
                {
                    let names_2 = {
                        let source_ml_source2_concat_value267 = names;
                        {
                            let source_ml_source2_concat_value268 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index(((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64) + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value267), source_ml_source2_concat_value268)
                        }
                    };
                    {
                        let source_ml_source1_temporary_index47_2 = source_ml_source1_temporary_index47.add(&self::ml::Big::from_i128(1));
                        {
                            (names, source_ml_source1_temporary_index47) = (names_2, source_ml_source1_temporary_index47_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            names
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result {
    MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Done,
    MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result {
    MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Done,
    MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result {
    MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Done,
    MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Return(bool),
}

pub fn source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop52(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut method: f64, mut at: f64) -> self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result {
    loop {
        return if (at < ((self::ml_array::optional_at(&row, self::ml_array::number_index((method + 4f64)))).clone().unwrap_or_else(|| 0f64))) {
            if (((((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![110, 101, 119]))) && self::ml_array::includes(&names, &(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40])))) {
                self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Return(true)
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
            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Done
        };
    }
}

pub fn source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop51(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut method: f64) -> self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result {
    loop {
        return if (method < ((row.len() as f64))) {
            if (((self::ml_array::optional_at(&row, self::ml_array::number_index(method))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) {
                {
                    let at = ((self::ml_array::optional_at(&row, self::ml_array::number_index((method + 3f64)))).clone().unwrap_or_else(|| 0f64) + 1f64);
                    {
                        let source_ml_source2_s269 = self::source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop52(values.clone(), names.clone(), row.clone(), method, at);
                        match source_ml_source2_s269.clone() {
                            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Done => {
                                {
                                    let method_2 = (method + 5f64);
                                    {
                                        method = method_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Return(source_ml_source1_temporary_result52) => {
                                self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Return(source_ml_source1_temporary_result52)
                            }
                        }
                    }
                }
            } else {
                {
                    let method_3 = (method + 5f64);
                    {
                        method = method_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Done
        };
    }
}

pub fn source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop50(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_values48: Vec<Vec<f64>>, mut source_ml_source1_temporary_index48: self::ml::Big) -> self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result {
    loop {
        return if (source_ml_source1_temporary_index48 < (self::ml::Big::from_u128(source_ml_source1_temporary_values48.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values48, source_ml_source1_temporary_index48.to_index());
                {
                    let method = (9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
                    {
                        let source_ml_source2_s270 = self::source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop51(values.clone(), names.clone(), row, method);
                        match source_ml_source2_s270.clone() {
                            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Done => {
                                {
                                    let source_ml_source1_temporary_index48_2 = source_ml_source1_temporary_index48.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index48 = source_ml_source1_temporary_index48_2.clone();
                                        continue;
                                    }
                                }
                            }
                            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Return(source_ml_source1_temporary_result51) => {
                                self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Return(source_ml_source1_temporary_result51)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Result {
    MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Done(self::ml_string::Text),
    MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_method_target_loop53(mut methods: Vec<self::ml_string::Text>, mut receiver_fields: Vec<self::ml_string::Text>, mut method: self::ml_string::Text, mut target: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Result {
    loop {
        return if (at < ((methods.len() as f64))) {
            if (self::ml_array::includes(&receiver_fields, &(self::ml_array::optional_at(&methods, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b) && (((self::ml_array::optional_at(&methods, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == method)) {
                if (target != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Result::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Return(self::ml_string::Text::from_units(vec![]))
                } else {
                    {
                        let target_2 = (self::ml_array::optional_at(&methods, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let at_2 = (at + 3f64);
                            {
                                (target, at) = (target_2.clone(), at_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 3f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Result::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Done(target.clone())
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop55(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut syntax: Vec<self::ml_string::Text>, mut at: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let syntax_2 = {
                    let source_ml_source2_concat_value271 = syntax;
                    {
                        let source_ml_source2_concat_value272 = vec![if ((((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) || (((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))) {
                            (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))
                        } else {
                            (self::ml_string::Text::from_units(vec![35])).concat(&(self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        }];
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value271), source_ml_source2_concat_value272)
                    }
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        (syntax, at) = (syntax_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            syntax
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop61(mut values: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut previous: f64, mut first: f64) -> f64 {
    loop {
        return if ((first < ((values.len() as f64))) && (((self::ml_array::optional_at(&starts, self::ml_array::number_index(first))).clone().unwrap_or_else(|| 0f64)) < previous)) {
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Result {
    MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Done(self::ml_string::Text, Vec<f64>, f64, Vec<self::ml_string::Text>, Vec<self::ml_string::Text>, Vec<self::ml_string::Text>, Vec<Vec<self::ml_string::Text>>),
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop62(mut values: Vec<self::ml_string::Text>, mut prefix: self::ml_string::Text, mut ordinal: f64, mut prototype_methods: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut brand: self::ml_string::Text, mut method: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (method < ((row.len() as f64))) {
            if (((self::ml_array::optional_at(&row, self::ml_array::number_index(method))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) {
                {
                    let prototype_methods_2 = {
                        let source_ml_source2_concat_value275 = prototype_methods;
                        {
                            let source_ml_source2_concat_value276 = vec![brand.clone(), (self::ml_array::optional_at(&values, self::ml_array::number_index((self::ml_array::optional_at(&row, self::ml_array::number_index(method))).clone().unwrap_or_else(|| 0f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95]))).concat(&{
                                let source_ml_source2_string_argument273 = ordinal;
                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument273))
                            })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                let source_ml_source2_string_argument274 = method;
                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument274))
                            })];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value275), source_ml_source2_concat_value276)
                        }
                    };
                    {
                        let method_2 = (method + 5f64);
                        {
                            (prototype_methods, method) = (prototype_methods_2, method_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let method_3 = (method + 5f64);
                    {
                        method = method_3;
                        continue;
                    }
                }
            }
        } else {
            prototype_methods
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop63(mut values: Vec<self::ml_string::Text>, mut public_fields: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut brand: self::ml_string::Text, mut field: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (field < ((9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64))))) {
            {
                let position = (self::ml_array::optional_at(&row, self::ml_array::number_index(field))).clone().unwrap_or_else(|| 0f64);
                {
                    let public_fields_2 = {
                        let source_ml_source2_concat_value277 = public_fields;
                        {
                            let source_ml_source2_concat_value278 = vec![brand.clone(), (self::ml_array::optional_at(&values, self::ml_array::number_index(if (position < (0f64)) {
                                ((-position) - 1f64)
                            } else {
                                position
                            }))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value277), source_ml_source2_concat_value278)
                        }
                    };
                    {
                        let field_2 = (field + 3f64);
                        {
                            (public_fields, field) = (public_fields_2, field_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            public_fields
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop64(mut values: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut own_fields: Vec<self::ml_string::Text>, mut field: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (field < ((9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64))))) {
            {
                let position = (self::ml_array::optional_at(&row, self::ml_array::number_index(field))).clone().unwrap_or_else(|| 0f64);
                {
                    let name = (self::ml_array::optional_at(&values, self::ml_array::number_index(if (position < (0f64)) {
                        ((-position) - 1f64)
                    } else {
                        position
                    }))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if !(self::ml_array::includes(&own_fields, &name.clone(), 0f64, false, |a, b| a == b)) {
                        {
                            let own_fields_2 = {
                                let source_ml_source2_concat_value279 = own_fields;
                                {
                                    let source_ml_source2_concat_value280 = vec![name.clone()];
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value279), source_ml_source2_concat_value280)
                                }
                            };
                            {
                                let field_2 = (field + 3f64);
                                {
                                    (own_fields, field) = (own_fields_2, field_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let field_3 = (field + 3f64);
                            {
                                field = field_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            own_fields
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop65(mut values: Vec<self::ml_string::Text>, mut prefix: self::ml_string::Text, mut ordinal: f64, mut prototype: bool, mut row: Vec<f64>, mut own_methods: Vec<self::ml_string::Text>, mut method: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (method < ((row.len() as f64))) {
            if (((self::ml_array::optional_at(&row, self::ml_array::number_index(method))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) {
                {
                    let own_methods_2 = {
                        let source_ml_source2_concat_value283 = own_methods;
                        {
                            let source_ml_source2_concat_value284 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index((self::ml_array::optional_at(&row, self::ml_array::number_index(method))).clone().unwrap_or_else(|| 0f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), (((prefix.concat(&if prototype {
                                self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95])
                            } else {
                                self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 95])
                            })).concat(&{
                                let source_ml_source2_string_argument281 = ordinal;
                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument281))
                            })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                let source_ml_source2_string_argument282 = method;
                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument282))
                            })];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value283), source_ml_source2_concat_value284)
                        }
                    };
                    {
                        let method_2 = (method + 5f64);
                        {
                            (own_methods, method) = (own_methods_2, method_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let method_3 = (method + 5f64);
                    {
                        method = method_3;
                        continue;
                    }
                }
            }
        } else {
            own_methods
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop60(mut source: self::ml_string::Text, mut values: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut prefix: self::ml_string::Text, mut names: Vec<self::ml_string::Text>, mut static_calls: Vec<self::ml_string::Text>, mut text: self::ml_string::Text, mut intervals: Vec<f64>, mut previous: f64, mut ordinal: f64, mut brands: Vec<self::ml_string::Text>, mut public_fields: Vec<self::ml_string::Text>, mut prototype_methods: Vec<self::ml_string::Text>, mut owner_rows: Vec<Vec<self::ml_string::Text>>, mut prototype: bool, mut source_ml_source1_temporary_values54: Vec<Vec<f64>>, mut source_ml_source1_temporary_index54: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Result {
    loop {
        return if (source_ml_source1_temporary_index54 < (self::ml::Big::from_u128(source_ml_source1_temporary_values54.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values54, source_ml_source1_temporary_index54.to_index());
                {
                    let first = (self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64);
                    {
                        let last = (self::ml_array::optional_at(&row, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| 0f64);
                        {
                            let before = {
                                let source_ml_source2_string_range_operand285 = source.clone();
                                {
                                    let source_ml_source2_string_range_operand286 = previous;
                                    {
                                        let source_ml_source2_string_range_operand287 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(first))).clone().unwrap_or_else(|| previous);
                                        {
                                            let source_ml_source2_string_range_position288 = source_ml_source2_string_range_operand286;
                                            {
                                                let source_ml_source2_string_range_position289 = source_ml_source2_string_range_operand287;
                                                source_ml_source2_string_range_operand285.slice(source_ml_source2_string_range_position288, source_ml_source2_string_range_position289)
                                            }
                                        }
                                    }
                                }
                            };
                            {
                                let intervals_2 = {
                                    let source_ml_source2_concat_value290 = intervals;
                                    {
                                        let source_ml_source2_concat_value291 = vec![text.length(), (text.length() + before.length()), previous, (self::ml_array::optional_at(&starts, self::ml_array::number_index(first))).clone().unwrap_or_else(|| previous)];
                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value290), source_ml_source2_concat_value291)
                                    }
                                };
                                {
                                    let text_2 = text.concat(&before);
                                    {
                                        let replacement = self::render_java_script_closed_class(source.clone(), values.clone(), starts.clone(), ends.clone(), row.clone(), prefix.clone(), ordinal, names.clone(), static_calls.clone(), prototype);
                                        {
                                            let intervals_3 = {
                                                let source_ml_source2_concat_value292 = intervals_2;
                                                {
                                                    let source_ml_source2_concat_value293 = vec![text_2.length(), (text_2.length() + replacement.length()), (self::ml_array::optional_at(&starts, self::ml_array::number_index(first))).clone().unwrap_or_else(|| 0f64), (self::ml_array::optional_at(&ends, self::ml_array::number_index(last))).clone().unwrap_or_else(|| 0f64)];
                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value292), source_ml_source2_concat_value293)
                                                }
                                            };
                                            {
                                                let text_3 = text_2.concat(&replacement);
                                                {
                                                    let previous_2 = (self::ml_array::optional_at(&ends, self::ml_array::number_index(last))).clone().unwrap_or_else(|| previous);
                                                    {
                                                        let brand = (prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95]))).concat(&{
                                                            let source_ml_source2_string_argument294 = ordinal;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument294))
                                                        });
                                                        {
                                                            let brands_2 = {
                                                                let source_ml_source2_concat_value295 = brands;
                                                                {
                                                                    let source_ml_source2_concat_value296 = vec![brand.clone()];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value295), source_ml_source2_concat_value296)
                                                                }
                                                            };
                                                            {
                                                                let prototype_methods_3 = if prototype {
                                                                    {
                                                                        let method = (9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
                                                                        {
                                                                            let prototype_methods_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop62(values.clone(), prefix.clone(), ordinal, prototype_methods, row.clone(), brand.clone(), method);
                                                                            prototype_methods_2
                                                                        }
                                                                    }
                                                                } else {
                                                                    prototype_methods
                                                                };
                                                                {
                                                                    let field = 9f64;
                                                                    {
                                                                        let public_fields_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop63(values.clone(), public_fields, row.clone(), brand.clone(), field);
                                                                        {
                                                                            let own_fields = Vec::<self::ml_string::Text>::new();
                                                                            {
                                                                                let own_methods = Vec::<self::ml_string::Text>::new();
                                                                                {
                                                                                    let field_2 = 9f64;
                                                                                    {
                                                                                        let own_fields_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop64(values.clone(), row.clone(), own_fields, field_2);
                                                                                        {
                                                                                            let method_2 = (9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
                                                                                            {
                                                                                                let own_methods_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop65(values.clone(), prefix.clone(), ordinal, prototype, row, own_methods, method_2);
                                                                                                {
                                                                                                    let owner_rows_2 = {
                                                                                                        let source_ml_source2_concat_value302 = owner_rows;
                                                                                                        {
                                                                                                            let source_ml_source2_concat_value303 = vec![{
                                                                                                                let source_ml_source2_concat_value299 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index((first + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), brand.clone(), if prototype {
                                                                                                                    self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 104, 101, 108, 112, 101, 114, 115])
                                                                                                                } else {
                                                                                                                    self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115])
                                                                                                                }, {
                                                                                                                    let source_ml_source2_string_argument297 = (own_fields_2.len() as f64);
                                                                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument297))
                                                                                                                }, {
                                                                                                                    let source_ml_source2_string_argument298 = ((own_methods_2.len() as f64) / 2f64);
                                                                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument298))
                                                                                                                }];
                                                                                                                {
                                                                                                                    let source_ml_source2_concat_value300 = own_fields_2;
                                                                                                                    {
                                                                                                                        let source_ml_source2_concat_value301 = own_methods_2;
                                                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value299), source_ml_source2_concat_value300), source_ml_source2_concat_value301)
                                                                                                                    }
                                                                                                                }
                                                                                                            }];
                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<Vec<self::ml_string::Text>>::new(), source_ml_source2_concat_value302), source_ml_source2_concat_value303)
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let ordinal_2 = (ordinal + 1f64);
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index54_2 = source_ml_source1_temporary_index54.add(&self::ml::Big::from_i128(1));
                                                                                                            {
                                                                                                                (text, intervals, previous, ordinal, brands, public_fields, prototype_methods, owner_rows, source_ml_source1_temporary_index54) = (text_3.clone(), intervals_3, previous_2, ordinal_2, brands_2, public_fields_2, prototype_methods_3, owner_rows_2, source_ml_source1_temporary_index54_2.clone());
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
        } else {
            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Done(text.clone(), intervals, previous, brands, public_fields, prototype_methods, owner_rows)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59 {
    MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58 {
    MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57 {
    MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Result {
    MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Done(f64, f64),
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop66(mut source: self::ml_string::Text, mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut names: Vec<self::ml_string::Text>, mut error: f64, mut error_at: f64, mut at: f64) -> self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let previous = (self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if ((((previous != (self::ml_string::Text::from_units(vec![46]))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) && ((((value == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) || (value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))) || ((value == (self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99]))) && (((self::ml_array::optional_at(&kinds, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))))) || ((value == (self::ml_string::Text::from_units(vec![110, 101, 119]))) && self::ml_array::includes(&names, &(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)))) && (({
                        let source_ml_source2_string_range_operand309 = source.clone();
                        {
                            let source_ml_source2_string_range_operand310 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                            {
                                let source_ml_source2_string_range_operand311 = (self::ml_array::optional_at(&ends, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                                {
                                    let source_ml_source2_string_range_position312 = source_ml_source2_string_range_operand310;
                                    {
                                        let source_ml_source2_string_range_position313 = source_ml_source2_string_range_operand311;
                                        source_ml_source2_string_range_operand309.slice(source_ml_source2_string_range_position312, source_ml_source2_string_range_position313)
                                    }
                                }
                            }
                        }
                    }) != value)) {
                        {
                            let error_2 = (-9f64);
                            {
                                let error_at_2 = at;
                                self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Done(error_2, error_at_2)
                            }
                        }
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
            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Done(error, error_at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result {
    MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(Vec<Vec<f64>>, Vec<self::ml_string::Text>, f64, f64),
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop67(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut end: f64) -> f64 {
    loop {
        return if ((end < ((values.len() as f64))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index(end))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![59])))) {
            if (((self::ml_array::optional_at(&kinds, self::ml_array::number_index(end))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                end
            } else {
                {
                    let end_2 = (end + 1f64);
                    {
                        end = end_2;
                        continue;
                    }
                }
            }
        } else {
            end
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop56(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut directory: Vec<Vec<f64>>, mut names: Vec<self::ml_string::Text>, mut depth: f64, mut declarations_finished: bool, mut error: f64, mut error_at: f64, mut at: f64) -> self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result {
    loop {
        return if ((at < ((values.len() as f64))) && (error == (0f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if (((((value == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) && (((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![63, 46])))) && ((((self::ml_array::optional_at(&kinds, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![123]))))) {
                    if (((directory.len() as f64)) >= (64f64)) {
                        {
                            let error_2 = (-8f64);
                            {
                                let error_at_2 = at;
                                self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_2, error_at_2)
                            }
                        }
                    } else {
                        if ((depth != (0f64)) || declarations_finished) {
                            {
                                let error_3 = (-7f64);
                                {
                                    let error_at_3 = at;
                                    self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_3, error_at_3)
                                }
                            }
                        } else {
                            {
                                let row = self::read_java_script_closed_class_directory(values.clone(), kinds.clone(), at);
                                if (((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| (-1f64))) < (0f64)) {
                                    {
                                        let error_4 = (self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| (-1f64));
                                        {
                                            let error_at_4 = (self::ml_array::optional_at(&row, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| at);
                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_4, error_at_4)
                                        }
                                    }
                                } else {
                                    {
                                        let name = (self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                        if self::ml_array::includes(&names, &name.clone(), 0f64, false, |a, b| a == b) {
                                            {
                                                let error_5 = (-7f64);
                                                {
                                                    let error_at_5 = (at + 1f64);
                                                    self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_5, error_at_5)
                                                }
                                            }
                                        } else {
                                            {
                                                let names_2 = {
                                                    let source_ml_source2_concat_value314 = names;
                                                    {
                                                        let source_ml_source2_concat_value315 = vec![name.clone()];
                                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value314), source_ml_source2_concat_value315)
                                                    }
                                                };
                                                {
                                                    let directory_2 = {
                                                        let source_ml_source2_concat_value316 = directory;
                                                        {
                                                            let source_ml_source2_concat_value317 = vec![row.clone()];
                                                            self::ml_array::append(self::ml_array::append(Vec::<Vec<f64>>::new(), source_ml_source2_concat_value316), source_ml_source2_concat_value317)
                                                        }
                                                    };
                                                    {
                                                        let at_2 = (self::ml_array::optional_at(&row, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| at);
                                                        {
                                                            let at_3 = (at_2 + 1f64);
                                                            {
                                                                (directory, names, at) = (directory_2, names_2, at_3);
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
                    }
                } else {
                    if ((depth == (0f64)) && (value == (self::ml_string::Text::from_units(vec![105, 109, 112, 111, 114, 116])))) {
                        {
                            let end = at;
                            {
                                let end_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop67(values.clone(), kinds.clone(), end);
                                if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![35, 110, 111, 100, 101, 58, 97, 115, 115, 101, 114, 116]), self::ml_string::Text::from_units(vec![35, 110, 111, 100, 101, 58, 97, 115, 115, 101, 114, 116, 47, 115, 116, 114, 105, 99, 116]), self::ml_string::Text::from_units(vec![35, 97, 115, 115, 101, 114, 116]), self::ml_string::Text::from_units(vec![35, 97, 115, 115, 101, 114, 116, 47, 115, 116, 114, 105, 99, 116])], &(self::ml_array::optional_at(&values, self::ml_array::number_index(end_2))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) {
                                    {
                                        let error_6 = (-7f64);
                                        {
                                            let error_at_6 = at;
                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_6, error_at_6)
                                        }
                                    }
                                } else {
                                    {
                                        let at_4 = end_2;
                                        {
                                            let at_5 = (at_4 + 1f64);
                                            {
                                                at = at_5;
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let declarations_finished_3 = if (((depth == (0f64)) && (value != (self::ml_string::Text::from_units(vec![59])))) && (value != (self::ml_string::Text::from_units(vec![35, 117, 115, 101, 32, 115, 116, 114, 105, 99, 116])))) {
                                {
                                    let declarations_finished_2 = true;
                                    declarations_finished_2
                                }
                            } else {
                                declarations_finished
                            };
                            {
                                let depth_3 = if (((value == (self::ml_string::Text::from_units(vec![40]))) || (value == (self::ml_string::Text::from_units(vec![91])))) || (value == (self::ml_string::Text::from_units(vec![123])))) {
                                    {
                                        let depth_2 = (depth + 1f64);
                                        depth_2
                                    }
                                } else {
                                    depth
                                };
                                if (((value == (self::ml_string::Text::from_units(vec![41]))) || (value == (self::ml_string::Text::from_units(vec![93])))) || (value == (self::ml_string::Text::from_units(vec![125])))) {
                                    {
                                        let depth_4 = (depth_3 - 1f64);
                                        {
                                            let at_6 = (at + 1f64);
                                            {
                                                (depth, declarations_finished, at) = (depth_4, declarations_finished_3, at_6);
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at_7 = (at + 1f64);
                                        {
                                            (depth, declarations_finished, at) = (depth_3, declarations_finished_3, at_7);
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
            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error, error_at)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result {
    MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Done,
    MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Return(bool),
}

pub fn source_ml_source1_temporary_accept_java_script_closed_class_own_field_loop68(mut receiver_fields: Vec<self::ml_string::Text>, mut public_fields: Vec<self::ml_string::Text>, mut field: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result {
    loop {
        return if (at < ((public_fields.len() as f64))) {
            if (self::ml_array::includes(&receiver_fields, &(self::ml_array::optional_at(&public_fields, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b) && (((self::ml_array::optional_at(&public_fields, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == field)) {
                self::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Return(true)
            } else {
                {
                    let at_2 = (at + 2f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Result {
    MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Done(f64, f64),
}

pub fn source_ml_source1_temporary_project_java_script_class_source_interval_loop69(mut intervals: Vec<f64>, mut first: f64, mut last: f64, mut low: f64, mut high: f64, mut at: f64) -> self::MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Result {
    loop {
        return if (at < ((intervals.len() as f64))) {
            {
                let start = (self::ml_array::optional_at(&intervals, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let end = (self::ml_array::optional_at(&intervals, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| 0f64);
                    if (((end < first) || (start > last)) || (start == end)) {
                        {
                            let at_2 = (at + 4f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let original_start = (self::ml_array::optional_at(&intervals, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| 0f64);
                            {
                                let original_end = (self::ml_array::optional_at(&intervals, self::ml_array::number_index((at + 3f64)))).clone().unwrap_or_else(|| 0f64);
                                {
                                    let low_2 = if (low < (0f64)) {
                                        original_start
                                    } else {
                                        self::ml_math::min(low, original_start)
                                    };
                                    {
                                        let high_2 = self::ml_math::max(high, original_end);
                                        {
                                            let at_3 = (at + 4f64);
                                            {
                                                (low, high, at) = (low_2, high_2, at_3);
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
        } else {
            self::MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Result::MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Done(low, high)
        };
    }
}

fn ml_main() {
    assert!(((self::read_java_script_closed_class_parameter_positions(vec![self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![110, 101, 119]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![41]), self::ml_string::Text::from_units(vec![41])], vec![self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])], 0f64, 7f64)) == (vec![(-1f64), 3f64])), "assertion 1");
    assert!(((self::read_java_script_closed_class_collection_parameter_positions(vec![self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![110, 101, 119]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![41]), self::ml_string::Text::from_units(vec![41])], vec![self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])], 0f64, 7f64)) == (vec![1f64])), "assertion 2");
    assert!(((self::read_java_script_closed_class_collection_parameter_positions(vec![self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![110, 101, 119]), self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![41]), self::ml_string::Text::from_units(vec![41])], vec![self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])], 0f64, 7f64)) == (vec![1f64])), "assertion 3");
    assert!(((self::read_java_script_closed_class_collection_parameter_positions(vec![self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![110, 101, 119]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![105, 116, 101, 109, 115]), self::ml_string::Text::from_units(vec![41]), self::ml_string::Text::from_units(vec![41])], vec![self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])], 0f64, 8f64)) == (vec![(-1f64), 3f64])), "assertion 4");
    assert!(((self::accept_java_script_closed_class_own_field(vec![self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121, 48]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116])], vec![self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121, 48]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116])], self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116]))) == (true)), "assertion 5");
    assert!(((self::accept_java_script_closed_class_own_field(vec![self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121, 49]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116])], vec![self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121, 48]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116])], self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116]))) == (false)), "assertion 6");
    assert!(((self::accept_java_script_closed_class_own_field(vec![self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104])], vec![self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121, 48]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116])], self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]))) == (false)), "assertion 7");
    assert!(((self::read_java_script_closed_class_own_field_use_error(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]), self::ml_string::Text::from_units(vec![]), true, false, true, false, true)) == (self::ml_string::Text::from_units(vec![]))), "assertion 8");
    assert!(((self::read_java_script_closed_class_use_error(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]), self::ml_string::Text::from_units(vec![]), true, false, true, false)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 105, 110, 115, 116, 97, 110, 99, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 114, 101, 99, 116, 32, 112, 117, 98, 108, 105, 99, 32, 102, 105, 101, 108, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 45, 114, 101, 116, 97, 105, 110, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 32, 99, 97, 108, 108, 115, 59, 32, 112, 114, 105, 118, 97, 116, 101, 32, 115, 108, 111, 116, 115, 44, 32, 100, 101, 116, 97, 99, 104, 101, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 115, 32, 97, 110, 100, 32, 105, 110, 100, 105, 114, 101, 99, 116, 32, 114, 101, 102, 108, 101, 99, 116, 105, 118, 101, 32, 108, 111, 111, 107, 117, 112, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 32, 102, 117, 108, 108, 32, 111, 98, 106, 101, 99, 116, 32, 109, 111, 100, 101, 108]))), "assertion 9");
    assert!(((self::read_java_script_closed_class_method_target(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 48]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 49]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 49])], vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]))) == (self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 48]))), "assertion 10");
    assert!(((self::read_java_script_closed_class_method_target(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 48]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 49]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 49])], vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 49]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]))) == (self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 49]))), "assertion 11");
    assert!(((self::read_java_script_closed_class_method_target(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 48])], vec![self::ml_string::Text::from_units(vec![100, 105, 102, 102, 101, 114, 101, 110, 116]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 12");
    assert!(((self::read_java_script_closed_class_method_target(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 48])], vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], self::ml_string::Text::from_units(vec![100, 101, 116, 97, 99, 104, 101, 100]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 13");
    assert!(((self::read_java_script_closed_class_method_target(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 48]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 49])], vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48])], self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 14");
    assert!(((self::accept_java_script_closed_class_method_definition(self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 48, 95, 57]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95, 48]), vec![self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], true)) == (true)), "assertion 15");
    assert!(((self::accept_java_script_closed_class_method_definition(self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 49, 95, 57]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95, 48]), vec![self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], true)) == (false)), "assertion 16");
    assert!(((self::accept_java_script_closed_class_method_definition(self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 48, 95, 57]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95, 48]), vec![self::ml_string::Text::from_units(vec![117, 115, 101, 100])], true)) == (false)), "assertion 17");
    assert!(((self::accept_java_script_closed_class_method_definition(self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 48, 95, 57]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95, 48]), vec![self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], false)) == (false)), "assertion 18");
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
fn all_literal_closed_class_default_controls() { main(); }
