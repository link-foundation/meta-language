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


pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

pub fn refuse_required_text_fact() -> self::ml_string::Text {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

pub fn read_required_number_fact(values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, index: f64) -> f64 {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ({ let ml_reference_array1 = values.clone(); let ml_reference_projection1 = ml_reference_array1.borrow().len() as f64; ml_reference_projection1 }))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        ({
            let source_ml_source2_array_read_value1 = values.clone();
            {
                let source_ml_source2_array_read_index2 = index;
                { let ml_reference_array2 = source_ml_source2_array_read_value1.clone(); let ml_reference_position2 = source_ml_source2_array_read_index2; let ml_reference_projection2 = self::ml_array::optional_at(&*ml_reference_array2.borrow(), self::ml_array::number_index(ml_reference_position2)); ml_reference_projection2 }
            }
        }).clone().unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

pub fn read_required_text_fact(values: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, index: f64) -> self::ml_string::Text {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ({ let ml_reference_array3 = values.clone(); let ml_reference_projection3 = ml_reference_array3.borrow().len() as f64; ml_reference_projection3 }))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        ({
            let source_ml_source2_array_read_value3 = values.clone();
            {
                let source_ml_source2_array_read_index4 = index;
                { let ml_reference_array4 = source_ml_source2_array_read_value3.clone(); let ml_reference_position4 = source_ml_source2_array_read_index4; let ml_reference_projection4 = self::ml_array::optional_at(&*ml_reference_array4.borrow(), self::ml_array::number_index(ml_reference_position4)); ml_reference_projection4 }
            }
        }).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

pub fn read_required_boolean_fact(values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, index: f64) -> bool {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ({ let ml_reference_array5 = values.clone(); let ml_reference_projection5 = ml_reference_array5.borrow().len() as f64; ml_reference_projection5 }))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        ({
            let source_ml_source2_array_read_value5 = values.clone();
            {
                let source_ml_source2_array_read_index6 = index;
                { let ml_reference_array6 = source_ml_source2_array_read_value5.clone(); let ml_reference_position6 = source_ml_source2_array_read_index6; let ml_reference_projection6 = self::ml_array::optional_at(&*ml_reference_array6.borrow(), self::ml_array::number_index(ml_reference_position6)); ml_reference_projection6 }
            }
        }).clone().unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

pub fn read_reference_method_result(kind: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    {
        let source_ml_source2_s7 = if (kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
            if ((operation == (self::ml_string::Text::from_units(vec![112, 117, 115, 104]))) || (operation == (self::ml_string::Text::from_units(vec![117, 110, 115, 104, 105, 102, 116])))) {
                self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Return(self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]))
            } else {
                if ((operation == (self::ml_string::Text::from_units(vec![112, 111, 112]))) || (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116])))) {
                    self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Return(self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 32, 101, 108, 101, 109, 101, 110, 116]))
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![114, 101, 118, 101, 114, 115, 101]))) {
                        self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Return(self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114]))
                    } else {
                        self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Next
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Next
        };
        match source_ml_source2_s7.clone() {
            self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Next => {
                if (((kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) && (operation == (self::ml_string::Text::from_units(vec![97, 100, 100])))) || ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && (operation == (self::ml_string::Text::from_units(vec![115, 101, 116]))))) {
                    self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
                } else {
                    {
                        let source_ml_source2_s8 = if ((kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) || (kind == (self::ml_string::Text::from_units(vec![109, 97, 112])))) {
                            if ((operation == (self::ml_string::Text::from_units(vec![104, 97, 115]))) || (operation == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101])))) {
                                self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Return(self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114]))) {
                                    self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Return(self::ml_string::Text::from_units(vec![117, 110, 105, 116]))
                                } else {
                                    if ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && (operation == (self::ml_string::Text::from_units(vec![103, 101, 116])))) {
                                        self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Return(self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 32, 118, 97, 108, 117, 101]))
                                    } else {
                                        self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Next
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Next
                        };
                        match source_ml_source2_s8.clone() {
                            self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Next => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Return(source_ml_source1_temporary_result2) => {
                                source_ml_source1_temporary_result2.clone()
                            }
                        }
                    }
                }
            }
            self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Return(source_ml_source1_temporary_result2_2) => {
                source_ml_source1_temporary_result2_2.clone()
            }
        }
    }
}

pub fn accept_linked_reference_method_operation(expression: self::ml_string::Text, operation: self::ml_string::Text, receiver: self::ml_string::Text, reference: bool) -> bool {
    (((expression == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) && reference) && ((self::read_reference_method_result(receiver.clone(), operation.clone())) != (self::ml_string::Text::from_units(vec![]))))
}

pub fn read_reference_method_refusal(kind: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if (((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) || (kind == (self::ml_string::Text::from_units(vec![115, 101, 116])))) && (((operation == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) || (operation == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])))) || (operation == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))))) {
        self::ml_string::Text::from_units(vec![108, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 111, 114, 100, 101, 114, 101, 100, 32, 101, 110, 116, 114, 105, 101, 115, 32, 97, 110, 100, 32, 116, 111, 109, 98, 115, 116, 111, 110, 101, 115, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 97, 32, 115, 110, 97, 112, 115, 104, 111, 116])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_reference_callback_effect(expression: self::ml_string::Text, operation: self::ml_string::Text) -> f64 {
    if ((expression == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) && (((operation == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101]))) || (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108])))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110]))))) {
        1f64
    } else {
        if ((expression == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) && operation.starts_with(&self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]))) {
            1f64
        } else {
            if ((expression == (self::ml_string::Text::from_units(vec![99, 101, 108, 108, 87, 114, 105, 116, 101]))) || (expression == (self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101, 87, 114, 105, 116, 101])))) {
                1f64
            } else {
                if ((((expression == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) && (operation != (self::ml_string::Text::from_units(vec![104, 97, 115])))) && (operation != (self::ml_string::Text::from_units(vec![103, 101, 116])))) && ((((self::read_reference_method_result(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), operation.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_reference_method_result(self::ml_string::Text::from_units(vec![109, 97, 112]), operation.clone())) != (self::ml_string::Text::from_units(vec![])))) || ((self::read_reference_method_result(self::ml_string::Text::from_units(vec![115, 101, 116]), operation.clone())) != (self::ml_string::Text::from_units(vec![]))))) {
                    1f64
                } else {
                    if ((expression == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) || (expression == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 82, 101, 102])))) {
                        2f64
                    } else {
                        if (expression == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))) {
                            3f64
                        } else {
                            0f64
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_snapshot_callback_parameters(operation: self::ml_string::Text, count: f64) -> bool {
    (count < (if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
        4f64
    } else {
        3f64
    }))
}

pub fn read_reference_callback_operand_fields(expression: self::ml_string::Text, ordinary: self::ml_string::Text) -> self::ml_string::Text {
    if (expression == (self::ml_string::Text::from_units(vec![108, 97, 109, 98, 100, 97]))) {
        self::ml_string::Text::from_units(vec![98, 111, 100, 121])
    } else {
        if (expression == (self::ml_string::Text::from_units(vec![108, 101, 116]))) {
            self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 32, 98, 111, 100, 121])
        } else {
            if (expression == (self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104]))) {
                self::ml_string::Text::from_units(vec![115, 99, 114, 117, 116, 105, 110, 101, 101, 32, 99, 97, 115, 101, 115])
            } else {
                if (expression == (self::ml_string::Text::from_units(vec![111, 117, 116, 67, 111, 110, 115]))) {
                    self::ml_string::Text::from_units(vec![104, 101, 97, 100, 32, 116, 97, 105, 108])
                } else {
                    ordinary.clone()
                }
            }
        }
    }
}

pub fn read_reference_method_required_arguments(kind: self::ml_string::Text, operation: self::ml_string::Text) -> f64 {
    if ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && (operation == (self::ml_string::Text::from_units(vec![115, 101, 116])))) {
        2f64
    } else {
        if ((((operation == (self::ml_string::Text::from_units(vec![97, 100, 100]))) || (operation == (self::ml_string::Text::from_units(vec![103, 101, 116])))) || (operation == (self::ml_string::Text::from_units(vec![104, 97, 115])))) || (operation == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101])))) {
            1f64
        } else {
            0f64
        }
    }
}

pub fn read_reference_method_argument_role(kind: self::ml_string::Text, operation: self::ml_string::Text, position: f64) -> self::ml_string::Text {
    if ((kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) && ((operation == (self::ml_string::Text::from_units(vec![112, 117, 115, 104]))) || (operation == (self::ml_string::Text::from_units(vec![117, 110, 115, 104, 105, 102, 116]))))) {
        self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
    } else {
        if ((((kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) || (kind == (self::ml_string::Text::from_units(vec![109, 97, 112])))) && (position == (0f64))) && (operation != (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114])))) {
            self::ml_string::Text::from_units(vec![107, 101, 121])
        } else {
            if (((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && (operation == (self::ml_string::Text::from_units(vec![115, 101, 116])))) && (position == (1f64))) {
                self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn accept_reference_method_key_kind(kind: self::ml_string::Text) -> bool {
    ((((((kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) || (kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (kind == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))) || (kind == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]))))
}

pub fn read_reference_method_identity(kind: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    {
        let source_ml_source2_s9 = if (kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
            if (operation == (self::ml_string::Text::from_units(vec![112, 117, 115, 104]))) {
                self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![97, 112, 112, 101, 110, 100, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115]))
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![112, 111, 112]))) {
                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 45, 102, 105, 110, 97, 108, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116]))
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116]))) {
                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 45, 105, 110, 105, 116, 105, 97, 108, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116]))
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![117, 110, 115, 104, 105, 102, 116]))) {
                            self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![112, 114, 101, 112, 101, 110, 100, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115]))
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![114, 101, 118, 101, 114, 115, 101]))) {
                                self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![114, 101, 118, 101, 114, 115, 101, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115]))
                            } else {
                                self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Next
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Next
        };
        match source_ml_source2_s9.clone() {
            self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Next => {
                {
                    let source_ml_source2_s10 = if (kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) {
                        if (operation == (self::ml_string::Text::from_units(vec![97, 100, 100]))) {
                            self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text::from_units(vec![97, 100, 100, 45, 115, 101, 116, 45, 109, 101, 109, 98, 101, 114]))
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101]))) {
                                self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 45, 115, 101, 116, 45, 109, 101, 109, 98, 101, 114]))
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114]))) {
                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114, 45, 115, 101, 116, 45, 109, 101, 109, 98, 101, 114, 115]))
                                } else {
                                    if (operation == (self::ml_string::Text::from_units(vec![104, 97, 115]))) {
                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 45, 115, 101, 116, 45, 109, 101, 109, 98, 101, 114, 115, 104, 105, 112]))
                                    } else {
                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Next
                                    }
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Next
                    };
                    match source_ml_source2_s10.clone() {
                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Next => {
                            {
                                let source_ml_source2_s11 = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                    if (operation == (self::ml_string::Text::from_units(vec![115, 101, 116]))) {
                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![115, 101, 116, 45, 109, 97, 112, 45, 101, 110, 116, 114, 121]))
                                    } else {
                                        if (operation == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101]))) {
                                            self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 45, 109, 97, 112, 45, 101, 110, 116, 114, 121]))
                                        } else {
                                            if (operation == (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114]))) {
                                                self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114, 45, 109, 97, 112, 45, 101, 110, 116, 114, 105, 101, 115]))
                                            } else {
                                                if (operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))) {
                                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 45, 109, 97, 112, 45, 101, 110, 116, 114, 121]))
                                                } else {
                                                    if (operation == (self::ml_string::Text::from_units(vec![104, 97, 115]))) {
                                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 45, 109, 97, 112, 45, 109, 101, 109, 98, 101, 114, 115, 104, 105, 112]))
                                                    } else {
                                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Next
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Next
                                };
                                match source_ml_source2_s11.clone() {
                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Next => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(source_ml_source1_temporary_result5) => {
                                        source_ml_source1_temporary_result5.clone()
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(source_ml_source1_temporary_result5_2) => {
                            source_ml_source1_temporary_result5_2.clone()
                        }
                    }
                }
            }
            self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(source_ml_source1_temporary_result5_3) => {
                source_ml_source1_temporary_result5_3.clone()
            }
        }
    }
}

pub fn read_reference_method_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![83, 104, 97, 114, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 115, 32, 101, 118, 97, 108, 117, 97, 116, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 101, 118, 101, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 111, 110, 99, 101, 32, 105, 110, 32, 115, 111, 117, 114, 99, 101, 32, 111, 114, 100, 101, 114, 59, 32, 101, 97, 99, 104, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 109, 117, 116, 97, 116, 105, 111, 110, 32, 99, 104, 97, 110, 103, 101, 115, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 44, 32, 112, 114, 101, 115, 101, 114, 118, 105, 110, 103, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114, 44, 32, 83, 97, 109, 101, 86, 97, 108, 117, 101, 90, 101, 114, 111, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 107, 101, 121, 115, 32, 97, 110, 100, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 32, 114, 101, 115, 117, 108, 116, 46])
}

pub fn render_reference_method_expression(target: self::ml_string::Text, kind: self::ml_string::Text, operation: self::ml_string::Text, receiver: self::ml_string::Text, arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, float_key: bool, suffix: self::ml_string::Text) -> self::ml_string::Text {
    if ((self::read_reference_method_result(kind.clone(), operation.clone())) == (self::ml_string::Text::from_units(vec![]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            ((((((self::ml_string::Text::from_units(vec![40])).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![41, 46]))).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&self::ml_string::join(&{ let ml_reference_value7 = arguments_.borrow().clone(); ml_reference_value7 }, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if (target != (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                {
                    let owner = (self::ml_string::Text::from_units(vec![109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 111, 119, 110, 101, 114])).concat(&suffix);
                    {
                        let values = (self::ml_string::Text::from_units(vec![109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 118, 97, 108, 117, 101, 115])).concat(&suffix);
                        {
                            let prefix = ((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                            {
                                let index = 0f64;
                                {
                                    let prefix_2 = self::source_ml_source1_temporary_render_reference_method_expression_loop6(arguments_.clone(), suffix.clone(), prefix.clone(), index);
                                    {
                                        let first = ((self::ml_string::Text::from_units(vec![109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![95, 48]));
                                        {
                                            let second = ((self::ml_string::Text::from_units(vec![109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![95, 49]));
                                            if (kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
                                                if ((operation == (self::ml_string::Text::from_units(vec![112, 117, 115, 104]))) || (operation == (self::ml_string::Text::from_units(vec![117, 110, 115, 104, 105, 102, 116])))) {
                                                    {
                                                        let names = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                        {
                                                            let index_2 = 0f64;
                                                            {
                                                                let names_2 = self::source_ml_source1_temporary_render_reference_method_expression_loop7(arguments_.clone(), suffix.clone(), names.clone(), index_2);
                                                                {
                                                                    let changes = if (operation == (self::ml_string::Text::from_units(vec![112, 117, 115, 104]))) {
                                                                        values.concat(&self::ml_string::Text::from_units(vec![46, 101, 120, 116, 101, 110, 100, 40, 109, 108, 95, 105, 110, 115, 101, 114, 116, 101, 100, 41, 59]))
                                                                    } else {
                                                                        values.concat(&self::ml_string::Text::from_units(vec![46, 115, 112, 108, 105, 99, 101, 40, 48, 46, 46, 48, 44, 32, 109, 108, 95, 105, 110, 115, 101, 114, 116, 101, 100, 41, 59]))
                                                                    };
                                                                    (((((((((((prefix_2.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 105, 110, 115, 101, 114, 116, 101, 100, 32, 61, 32, 118, 101, 99, 33, 91]))).concat(&self::ml_string::join(&{ let ml_reference_value8 = names_2.borrow().clone(); ml_reference_value8 }, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![93, 59, 32, 108, 101, 116, 32, 109, 117, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 108, 101, 110, 103, 116, 104, 32, 61, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![46, 108, 101, 110, 40, 41, 46, 99, 104, 101, 99, 107, 101, 100, 95, 97, 100, 100, 40, 109, 108, 95, 105, 110, 115, 101, 114, 116, 101, 100, 46, 108, 101, 110, 40, 41, 41, 46, 102, 105, 108, 116, 101, 114, 40, 124, 108, 101, 110, 103, 116, 104, 124, 32, 42, 108, 101, 110, 103, 116, 104, 32, 60, 61, 32, 52, 50, 57, 52, 57, 54, 55, 50, 57, 53, 117, 115, 105, 122, 101, 41, 46, 101, 120, 112, 101, 99, 116, 40, 34, 73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 34, 41, 59, 32]))).concat(&changes)).concat(&self::ml_string::Text::from_units(vec![32, 109, 108, 95, 108, 101, 110, 103, 116, 104, 32, 97, 115, 32, 102, 54, 52, 32, 125]))
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if (operation == (self::ml_string::Text::from_units(vec![112, 111, 112]))) {
                                                        ((prefix_2.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 114, 101, 115, 117, 108, 116, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 46, 112, 111, 112, 40, 41, 59, 32, 109, 108, 95, 114, 101, 115, 117, 108, 116, 32, 125]))
                                                    } else {
                                                        if (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116]))) {
                                                            ((((((((prefix_2.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 117, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32, 105, 102, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 101, 109, 112, 116, 121, 40, 41, 32, 123, 32, 78, 111, 110, 101, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 83, 111, 109, 101, 40]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![46, 114, 101, 109, 111, 118, 101, 40, 48, 41, 41, 32, 125, 32, 125]))
                                                        } else {
                                                            (((prefix_2.concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 46, 114, 101, 118, 101, 114, 115, 101, 40, 41, 59, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                                        }
                                                    }
                                                }
                                            } else {
                                                if (operation == (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114]))) {
                                                    (prefix_2.concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 46, 99, 108, 101, 97, 114, 40, 41, 59, 32, 40, 41, 32, 125]))
                                                } else {
                                                    {
                                                        let key = (self::ml_string::Text::from_units(vec![109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 107, 101, 121])).concat(&suffix);
                                                        {
                                                            let prefix_3 = prefix_2.concat(&((((self::ml_string::Text::from_units(vec![108, 101, 116, 32])).concat(&key)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&if float_key {
                                                                ((((self::ml_string::Text::from_units(vec![105, 102, 32])).concat(&first)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32, 48, 46, 48, 32, 123, 32, 48, 46, 48, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32]))).concat(&first)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                                            } else {
                                                                first.clone()
                                                            })).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                            {
                                                                let pattern = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                                                    self::ml_string::Text::from_units(vec![40, 109, 108, 95, 107, 101, 121, 44, 32, 95, 41])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![109, 108, 95, 107, 101, 121])
                                                                };
                                                                {
                                                                    let equality = if float_key {
                                                                        ((((self::ml_string::Text::from_units(vec![40, 109, 108, 95, 107, 101, 121, 32, 61, 61, 32, 38])).concat(&key)).concat(&self::ml_string::Text::from_units(vec![32, 124, 124, 32, 40, 109, 108, 95, 107, 101, 121, 46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 38, 38, 32]))).concat(&key)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 110, 97, 110, 40, 41, 41, 41]))
                                                                    } else {
                                                                        (self::ml_string::Text::from_units(vec![109, 108, 95, 107, 101, 121, 32, 61, 61, 32, 38])).concat(&key)
                                                                    };
                                                                    {
                                                                        let position = ((((values.concat(&self::ml_string::Text::from_units(vec![46, 105, 116, 101, 114, 40, 41, 46, 112, 111, 115, 105, 116, 105, 111, 110, 40, 124]))).concat(&pattern)).concat(&self::ml_string::Text::from_units(vec![124, 32]))).concat(&equality)).concat(&self::ml_string::Text::from_units(vec![41]));
                                                                        if (operation == (self::ml_string::Text::from_units(vec![104, 97, 115]))) {
                                                                            ((((((prefix_3.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 59, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 115, 111, 109, 101, 40, 41, 32, 125]))
                                                                        } else {
                                                                            if (operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))) {
                                                                                ((((((((prefix_3.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 59, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![46, 109, 97, 112, 40, 124, 105, 110, 100, 101, 120, 124, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![91, 105, 110, 100, 101, 120, 93, 46, 49, 46, 99, 108, 111, 110, 101, 40, 41, 41, 32, 125]))
                                                                            } else {
                                                                                if (operation == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101]))) {
                                                                                    ((((((((prefix_3.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 117, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32, 109, 97, 116, 99, 104, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40, 105, 110, 100, 101, 120, 41, 32, 61, 62, 32, 123, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![46, 114, 101, 109, 111, 118, 101, 40, 105, 110, 100, 101, 120, 41, 59, 32, 116, 114, 117, 101, 32, 125, 44, 32, 78, 111, 110, 101, 32, 61, 62, 32, 102, 97, 108, 115, 101, 32, 125, 32, 125]))
                                                                                } else {
                                                                                    {
                                                                                        let insertion = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                                                                            ((((values.concat(&self::ml_string::Text::from_units(vec![46, 112, 117, 115, 104, 40, 40]))).concat(&key)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&second)).concat(&self::ml_string::Text::from_units(vec![41, 41, 59]))
                                                                                        } else {
                                                                                            ((values.concat(&self::ml_string::Text::from_units(vec![46, 112, 117, 115, 104, 40]))).concat(&key)).concat(&self::ml_string::Text::from_units(vec![41, 59]))
                                                                                        };
                                                                                        {
                                                                                            let replacement = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                                                                                ((values.concat(&self::ml_string::Text::from_units(vec![91, 105, 110, 100, 101, 120, 93, 46, 49, 32, 61, 32]))).concat(&second)).concat(&self::ml_string::Text::from_units(vec![59]))
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![108, 101, 116, 32, 95, 32, 61, 32, 105, 110, 100, 101, 120, 59])
                                                                                            };
                                                                                            ((((((((((((prefix_3.concat(&self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 117, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32, 109, 97, 116, 99, 104, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40, 105, 110, 100, 101, 120, 41, 32, 61, 62, 32, 123, 32]))).concat(&replacement)).concat(&self::ml_string::Text::from_units(vec![32, 125, 44, 32, 78, 111, 110, 101, 32, 61, 62, 32, 123, 32]))).concat(&insertion)).concat(&self::ml_string::Text::from_units(vec![32, 125, 32, 125, 32, 125, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
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

pub fn read_array_callback_result_form(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
        self::ml_string::Text::from_units(vec![109, 97, 112, 112, 101, 100, 32, 97, 114, 114, 97, 121])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 116, 101, 114]))) {
            self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100]))) {
                self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 32, 101, 108, 101, 109, 101, 110, 116])
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 73, 110, 100, 101, 120]))) {
                    self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])
                } else {
                    if ((operation == (self::ml_string::Text::from_units(vec![115, 111, 109, 101]))) || (operation == (self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121])))) {
                        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112]))) {
                            self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107, 32, 97, 114, 114, 97, 121])
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![102, 111, 114, 69, 97, 99, 104]))) {
                                self::ml_string::Text::from_units(vec![117, 110, 105, 116])
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
                                    self::ml_string::Text::from_units(vec![97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])
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

pub fn read_array_callback_expected_result_form(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (((((operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 116, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100])))) || (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 73, 110, 100, 101, 120])))) || (operation == (self::ml_string::Text::from_units(vec![115, 111, 109, 101])))) || (operation == (self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121])))) {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
            self::ml_string::Text::from_units(vec![97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112]))) {
                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_array_callback_maximum_arguments(operation: self::ml_string::Text) -> f64 {
    if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
        2f64
    } else {
        if ((self::read_array_callback_result_form(operation.clone())) == (self::ml_string::Text::from_units(vec![]))) {
            (-1f64)
        } else {
            1f64
        }
    }
}

pub fn read_array_callback_signature_error(operation: self::ml_string::Text, kind: self::ml_string::Text, declared_count: f64, provided_count: f64, parameters_match: bool, result_matches: bool, expected_result: self::ml_string::Text) -> self::ml_string::Text {
    if (kind != (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
        ((self::ml_string::Text::from_units(vec![46])).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
    } else {
        if ((declared_count > provided_count) || !parameters_match) {
            ((self::ml_string::Text::from_units(vec![46])).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 100, 105, 102, 102, 101, 114]))
        } else {
            if ((expected_result != (self::ml_string::Text::from_units(vec![]))) && !result_matches) {
                (((self::ml_string::Text::from_units(vec![46])).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 114, 101, 115, 117, 108, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32]))).concat(&expected_result)
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_array_callback_parameter_role(operation: self::ml_string::Text, position: f64) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
        if (position == (0f64)) {
            self::ml_string::Text::from_units(vec![97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])
        } else {
            if (position == (1f64)) {
                self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
            } else {
                if (position == (2f64)) {
                    self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
                } else {
                    if (position == (3f64)) {
                        self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    } else {
        if (position == (0f64)) {
            self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
        } else {
            if (position == (1f64)) {
                self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
            } else {
                if (position == (2f64)) {
                    self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn render_array_callback_argument(operation: self::ml_string::Text, position: f64, suffix: self::ml_string::Text, clone_element: bool) -> self::ml_string::Text {
    {
        let role = self::read_array_callback_parameter_role(operation.clone(), position);
        if (role == (self::ml_string::Text::from_units(vec![97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114]))) {
            (self::ml_string::Text::from_units(vec![109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])).concat(&suffix)
        } else {
            if (role == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116]))) {
                ((self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])).concat(&suffix)).concat(&if clone_element {
                    self::ml_string::Text::from_units(vec![46, 99, 108, 111, 110, 101, 40, 41])
                } else {
                    self::ml_string::Text::from_units(vec![])
                })
            } else {
                if (role == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120]))) {
                    ((self::ml_string::Text::from_units(vec![40, 109, 108, 95, 105, 110, 100, 101, 120])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![32, 97, 115, 32, 102, 54, 52, 41]))
                } else {
                    if (role == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114]))) {
                        ((self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 115])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![46, 99, 108, 111, 110, 101, 40, 41]))
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    }
}

pub fn render_array_callback_call(operation: self::ml_string::Text, parameter_count: f64, suffix: self::ml_string::Text, clone_element: bool) -> self::ml_string::Text {
    {
        let arguments_text = self::ml_string::Text::from_units(vec![]);
        {
            let position = 0f64;
            {
                let maximum = if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
                    4f64
                } else {
                    3f64
                };
                {
                    let arguments_text_2 = self::source_ml_source1_temporary_render_array_callback_call_loop8(operation.clone(), parameter_count, suffix.clone(), clone_element, arguments_text.clone(), position, maximum);
                    ((((self::ml_string::Text::from_units(vec![109, 108, 95, 99, 97, 108, 108, 98, 97, 99, 107])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&arguments_text_2)).concat(&self::ml_string::Text::from_units(vec![41]))
                }
            }
        }
    }
}

pub fn render_array_callback_expression(operation: self::ml_string::Text, object: self::ml_string::Text, callback: self::ml_string::Text, parameter_count: f64, suffix: self::ml_string::Text, initial: self::ml_string::Text, has_initial: bool) -> self::ml_string::Text {
    {
        let values = (self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 115])).concat(&suffix);
        {
            let index = (self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 100, 101, 120])).concat(&suffix);
            {
                let value = (self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])).concat(&suffix);
                {
                    let call = self::render_array_callback_call(operation.clone(), parameter_count, suffix.clone(), false);
                    {
                        let head = ((((((((self::ml_string::Text::from_units(vec![108, 101, 116, 32])).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&object)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 99, 97, 108, 108, 98, 97, 99, 107]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&callback)).concat(&self::ml_string::Text::from_units(vec![59]));
                        {
                            let iterator = values.concat(&self::ml_string::Text::from_units(vec![46, 105, 116, 101, 114, 40, 41, 46, 99, 108, 111, 110, 101, 100, 40, 41, 46, 101, 110, 117, 109, 101, 114, 97, 116, 101, 40, 41]));
                            if ((operation == (self::ml_string::Text::from_units(vec![115, 111, 109, 101]))) || (operation == (self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121])))) {
                                ((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46]))).concat(&if (operation == (self::ml_string::Text::from_units(vec![115, 111, 109, 101]))) {
                                    self::ml_string::Text::from_units(vec![97, 110, 121])
                                } else {
                                    self::ml_string::Text::from_units(vec![97, 108, 108])
                                })).concat(&self::ml_string::Text::from_units(vec![40, 124, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125]))
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 73, 110, 100, 101, 120]))) {
                                    ((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46, 112, 111, 115, 105, 116, 105, 111, 110, 40, 124, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![41, 46, 109, 97, 112, 95, 111, 114, 40, 45, 49, 102, 54, 52, 44, 32, 124, 109, 108, 95, 112, 111, 115, 105, 116, 105, 111, 110, 124, 32, 109, 108, 95, 112, 111, 115, 105, 116, 105, 111, 110, 32, 97, 115, 32, 102, 54, 52, 41, 32, 125]))
                                } else {
                                    if (operation == (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112]))) {
                                        ((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46, 102, 108, 97, 116, 95, 109, 97, 112, 40, 124, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 95, 62, 62, 40, 41, 32, 125]))
                                    } else {
                                        if ((operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 116, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100])))) {
                                            {
                                                let predicate = self::render_array_callback_call(operation.clone(), parameter_count, suffix.clone(), true);
                                                if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 116, 101, 114]))) {
                                                    ((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46, 102, 105, 108, 116, 101, 114, 95, 109, 97, 112, 40, 124, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32, 105, 102, 32]))).concat(&predicate)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 78, 111, 110, 101, 32, 125, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 95, 62, 62, 40, 41, 32, 125]))
                                                } else {
                                                    ((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46, 102, 105, 110, 100, 95, 109, 97, 112, 40, 124, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32, 105, 102, 32]))).concat(&predicate)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 78, 111, 110, 101, 32, 125, 41, 32, 125]))
                                                }
                                            }
                                        } else {
                                            if (operation == (self::ml_string::Text::from_units(vec![102, 111, 114, 69, 97, 99, 104]))) {
                                                ((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32, 102, 111, 114, 32, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 32, 105, 110, 32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![59, 32, 125, 32, 40, 41, 32, 125]))
                                            } else {
                                                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
                                                    if !has_initial {
                                                        ((((((((((((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32, 108, 101, 116, 32, 109, 117, 116, 32, 109, 108, 95, 105, 116, 101, 114, 97, 116, 111, 114]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 105, 110, 105, 116, 105, 97, 108]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 109, 108, 95, 105, 116, 101, 114, 97, 116, 111, 114]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![46, 110, 101, 120, 116, 40, 41, 46, 109, 97, 112, 40, 124, 40, 95, 44, 32, 109, 108, 95, 102, 105, 114, 115, 116, 41, 124, 32, 109, 108, 95, 102, 105, 114, 115, 116, 41, 46, 101, 120, 112, 101, 99, 116, 40, 34, 82, 101, 100, 117, 99, 101, 32, 111, 102, 32, 101, 109, 112, 116, 121, 32, 97, 114, 114, 97, 121, 32, 119, 105, 116, 104, 32, 110, 111, 32, 105, 110, 105, 116, 105, 97, 108, 32, 118, 97, 108, 117, 101, 34, 41, 59, 32, 109, 108, 95, 105, 116, 101, 114, 97, 116, 111, 114]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![46, 102, 111, 108, 100, 40, 109, 108, 95, 105, 110, 105, 116, 105, 97, 108]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![44, 32, 124, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![44, 32, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125]))
                                                    } else {
                                                        ((((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46, 102, 111, 108, 100, 40]))).concat(&initial)).concat(&self::ml_string::Text::from_units(vec![44, 32, 124, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![44, 32, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125]))
                                                    }
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
}

pub fn read_array_callback_argument_count_error(operation: self::ml_string::Text, count: f64) -> self::ml_string::Text {
    {
        let maximum = self::read_array_callback_maximum_arguments(operation.clone());
        if ((maximum < (0f64)) || ((count >= (1f64)) && (count <= maximum))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let expected = if (maximum == (1f64)) {
                    self::ml_string::Text::from_units(vec![49])
                } else {
                    self::ml_string::Text::from_units(vec![49, 32, 116, 111, 32, 50])
                };
                ((((self::ml_string::Text::from_units(vec![46])).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 101, 120, 112, 101, 99, 116, 115, 32]))).concat(&expected)).concat(&self::ml_string::Text::from_units(vec![32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
            }
        }
    }
}

pub fn read_array_callback_checked_result_context(operation: self::ml_string::Text) -> self::ml_string::Text {
    {
        let expected = self::read_array_callback_expected_result_form(operation.clone());
        if (expected == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            expected.clone()
        }
    }
}

pub fn read_array_callback_result_error(operation: self::ml_string::Text, result_kind: self::ml_string::Text) -> self::ml_string::Text {
    if ((operation == (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112]))) && (result_kind != (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) {
        self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 109, 117, 115, 116, 32, 114, 101, 116, 117, 114, 110, 32, 97, 110, 32, 97, 114, 114, 97, 121])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_array_callback_checked_argument_position(operation: self::ml_string::Text, has_initial: bool, position: f64) -> f64 {
    if ((operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) && has_initial) {
        if (position == (0f64)) {
            1f64
        } else {
            if (position == (1f64)) {
                0f64
            } else {
                (-1f64)
            }
        }
    } else {
        if (position == (0f64)) {
            0f64
        } else {
            (-1f64)
        }
    }
}

pub fn render_array_callback_reference_expression(operation: self::ml_string::Text, object: self::ml_string::Text, callback: self::ml_string::Text, parameter_count: f64, suffix: self::ml_string::Text, reference_result: bool) -> self::ml_string::Text {
    if ((operation != (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112]))) || !reference_result) {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let values = (self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 115])).concat(&suffix);
            {
                let index = (self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 100, 101, 120])).concat(&suffix);
                {
                    let value = (self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])).concat(&suffix);
                    {
                        let result = (self::ml_string::Text::from_units(vec![109, 108, 95, 99, 97, 108, 108, 98, 97, 99, 107, 95, 118, 97, 108, 117, 101, 115])).concat(&suffix);
                        {
                            let call = self::render_array_callback_call(operation.clone(), parameter_count, suffix.clone(), false);
                            {
                                let head = ((((((((self::ml_string::Text::from_units(vec![108, 101, 116, 32])).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&object)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 99, 97, 108, 108, 98, 97, 99, 107]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&callback)).concat(&self::ml_string::Text::from_units(vec![59]));
                                {
                                    let iterator = values.concat(&self::ml_string::Text::from_units(vec![46, 105, 116, 101, 114, 40, 41, 46, 99, 108, 111, 110, 101, 100, 40, 41, 46, 101, 110, 117, 109, 101, 114, 97, 116, 101, 40, 41]));
                                    ((((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32, 115, 116, 100, 58, 58, 114, 99, 58, 58, 82, 99, 58, 58, 110, 101, 119, 40, 115, 116, 100, 58, 58, 99, 101, 108, 108, 58, 58, 82, 101, 102, 67, 101, 108, 108, 58, 58, 110, 101, 119, 40]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46, 102, 108, 97, 116, 95, 109, 97, 112, 40, 124, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32, 123, 32, 108, 101, 116, 32]))).concat(&result)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 46, 99, 108, 111, 110, 101, 40, 41, 59, 32]))).concat(&result)).concat(&self::ml_string::Text::from_units(vec![32, 125, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 95, 62, 62, 40, 41, 41, 41, 32, 125]))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_array_state_method_result(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101]))) {
        self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 100, 32, 97, 114, 114, 97, 121])
    } else {
        if ((operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110])))) {
            self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn accept_linked_array_state_method(expression: self::ml_string::Text, operation: self::ml_string::Text, kind: self::ml_string::Text, reference: bool) -> bool {
    ((((expression == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) && (kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) && reference) && ((self::read_array_state_method_result(operation.clone())) != (self::ml_string::Text::from_units(vec![]))))
}

pub fn read_array_state_method_required_arguments(operation: self::ml_string::Text) -> f64 {
    if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) {
        1f64
    } else {
        0f64
    }
}

pub fn read_array_state_method_argument_role(operation: self::ml_string::Text, position: f64) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101]))) {
        if (position < (2f64)) {
            self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 105, 110, 100, 101, 120])
        } else {
            self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
        }
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) {
            if (position == (0f64)) {
                self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
            } else {
                if (position == (1f64)) {
                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 105, 110, 100, 101, 120])
                } else {
                    if (position == (2f64)) {
                        self::ml_string::Text::from_units(vec![101, 110, 100, 32, 105, 110, 100, 101, 120])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110]))) {
                if (position < (2f64)) {
                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 105, 110, 100, 101, 120])
                } else {
                    if (position == (2f64)) {
                        self::ml_string::Text::from_units(vec![101, 110, 100, 32, 105, 110, 100, 101, 120])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_array_state_method_argument_profile(kind: self::ml_string::Text, element: self::ml_string::Text, absence: self::ml_string::Text) -> self::ml_string::Text {
    if (((kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || (kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) {
        kind.clone()
    } else {
        if (((kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && (((element == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || (element == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))) || (element == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))) && (((absence == (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))) || (absence == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))) || (absence == (self::ml_string::Text::from_units(vec![98, 111, 116, 104]))))) {
            (((self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 32])).concat(&absence)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&element)
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_array_state_method_suffix(initial: f64, receiver: self::ml_string::Text, arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> f64 {
    {
        let text = (receiver.concat(&self::ml_string::Text::from_units(vec![32]))).concat(&self::ml_string::join(&{ let ml_reference_value9 = arguments_.borrow().clone(); ml_reference_value9 }, &self::ml_string::Text::from_units(vec![32])));
        {
            let suffix = initial;
            {
                let suffix_2 = self::source_ml_source1_temporary_read_array_state_method_suffix_loop9(text.clone(), suffix);
                suffix_2
            }
        }
    }
}

pub fn render_array_state_numeric_value(profile: self::ml_string::Text, value: self::ml_string::Text, undefined_value: self::ml_string::Text) -> self::ml_string::Text {
    if (profile == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
        value.clone()
    } else {
        if (profile == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
            ((self::ml_string::Text::from_units(vec![105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 49, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 48, 102, 54, 52, 32, 125]))
        } else {
            if (profile == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                undefined_value.clone()
            } else {
                {
                    let parts = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_string_unit_operand12 = profile.clone();
                        {
                            let source_ml_source2_string_unit_operand13 = self::ml_string::Text::from_units(vec![32]);
                            {
                                let source_ml_source2_string_split_limit14 = 4294967295f64;
                                {
                                    let source_ml_source2_string_split_separator15 = source_ml_source2_string_unit_operand13.clone();
                                    (|ml_text:&self::ml_string::Text,ml_separator:Option<&self::ml_string::Text>,ml_limit:f64|{self::ml_utf16_text::split_utf16_text(&(ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()), &(ml_separator.map(|ml_value|ml_value.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()).unwrap_or_default()), ml_limit, ml_separator.is_none()).into_iter().map(|ml_units|self::ml_string::Text::from_units(ml_units.into_iter().map(|ml_unit|ml_unit as u16).collect())).collect::<Vec<_>>()})(&(source_ml_source2_string_unit_operand12), Some(&source_ml_source2_string_split_separator15), source_ml_source2_string_split_limit14)
                                }
                            }
                        }
                    }));
                    {
                        let absence = ({
                            let source_ml_source2_array_read_value16 = parts.clone();
                            {
                                let source_ml_source2_array_read_index17 = 1f64;
                                { let ml_reference_array10 = source_ml_source2_array_read_value16.clone(); let ml_reference_position10 = source_ml_source2_array_read_index17; let ml_reference_projection10 = self::ml_array::optional_at(&*ml_reference_array10.borrow(), self::ml_array::number_index(ml_reference_position10)); ml_reference_projection10 }
                            }
                        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let element = ({
                                let source_ml_source2_array_read_value18 = parts.clone();
                                {
                                    let source_ml_source2_array_read_index19 = 2f64;
                                    { let ml_reference_array11 = source_ml_source2_array_read_value18.clone(); let ml_reference_position11 = source_ml_source2_array_read_index19; let ml_reference_projection11 = self::ml_array::optional_at(&*ml_reference_array11.borrow(), self::ml_array::number_index(ml_reference_position11)); ml_reference_projection11 }
                                }
                            }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            {
                                let present = if (element == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                                    self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110, 116])
                                } else {
                                    if (element == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
                                        self::ml_string::Text::from_units(vec![105, 102, 32, 109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110, 116, 32, 123, 32, 49, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 48, 102, 54, 52, 32, 125])
                                    } else {
                                        self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78])
                                    }
                                };
                                if (absence == (self::ml_string::Text::from_units(vec![98, 111, 116, 104]))) {
                                    ((((((self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 99, 114, 97, 116, 101, 58, 58, 77, 108, 79, 112, 116, 105, 111, 110, 97, 108, 58, 58, 86, 97, 108, 117, 101, 40, 109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110, 116, 41, 32, 61, 62, 32]))).concat(&present)).concat(&self::ml_string::Text::from_units(vec![44, 32, 99, 114, 97, 116, 101, 58, 58, 77, 108, 79, 112, 116, 105, 111, 110, 97, 108, 58, 58, 78, 117, 108, 108, 32, 61, 62, 32, 48, 102, 54, 52, 44, 32, 99, 114, 97, 116, 101, 58, 58, 77, 108, 79, 112, 116, 105, 111, 110, 97, 108, 58, 58, 85, 110, 100, 101, 102, 105, 110, 101, 100, 32, 61, 62, 32]))).concat(&undefined_value)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                } else {
                                    ((((((self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40, 109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110, 116, 41, 32, 61, 62, 32]))).concat(&present)).concat(&self::ml_string::Text::from_units(vec![44, 32, 78, 111, 110, 101, 32, 61, 62, 32]))).concat(&if (absence == (self::ml_string::Text::from_units(vec![110, 117, 108, 108]))) {
                                        self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                                    } else {
                                        undefined_value.clone()
                                    })).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_array_state_relative_index(value: self::ml_string::Text, length: self::ml_string::Text) -> self::ml_string::Text {
    (((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 95, 118, 97, 108, 117, 101, 58, 32, 102, 54, 52, 32, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 32, 61, 32, 105, 102, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 95, 118, 97, 108, 117, 101, 46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 123, 32, 48, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 95, 118, 97, 108, 117, 101, 46, 116, 114, 117, 110, 99, 40, 41, 32, 125, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 98, 111, 117, 110, 100, 32, 61, 32]))).concat(&length)).concat(&self::ml_string::Text::from_units(vec![32, 97, 115, 32, 102, 54, 52, 59, 32, 40, 105, 102, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 32, 60, 32, 48, 102, 54, 52, 32, 123, 32, 40, 109, 108, 95, 98, 111, 117, 110, 100, 32, 43, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 41, 46, 99, 108, 97, 109, 112, 40, 48, 102, 54, 52, 44, 32, 109, 108, 95, 98, 111, 117, 110, 100, 41, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 46, 109, 105, 110, 40, 109, 108, 95, 98, 111, 117, 110, 100, 41, 32, 125, 41, 32, 97, 115, 32, 117, 115, 105, 122, 101, 32, 125]))
}

pub fn read_array_state_method_identity(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101]))) {
        self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) {
            self::ml_string::Text::from_units(vec![102, 105, 108, 108, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110]))) {
                self::ml_string::Text::from_units(vec![99, 111, 112, 121, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115, 45, 119, 105, 116, 104, 105, 110])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_array_state_method_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![68, 101, 110, 115, 101, 32, 115, 104, 97, 114, 101, 100, 32, 97, 114, 114, 97, 121, 32, 114, 97, 110, 103, 101, 32, 109, 117, 116, 97, 116, 105, 111, 110, 115, 32, 101, 118, 97, 108, 117, 97, 116, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 101, 118, 101, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 111, 110, 99, 101, 32, 105, 110, 32, 111, 114, 100, 101, 114, 44, 32, 100, 105, 115, 116, 105, 110, 103, 117, 105, 115, 104, 32, 109, 105, 115, 115, 105, 110, 103, 32, 97, 110, 100, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 44, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 32, 78, 117, 109, 98, 101, 114, 32, 105, 110, 100, 105, 99, 101, 115, 32, 119, 105, 116, 104, 111, 117, 116, 32, 110, 97, 114, 114, 111, 119, 105, 110, 103, 32, 98, 101, 102, 111, 114, 101, 32, 98, 111, 117, 110, 100, 115, 44, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 97, 110, 100, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 101, 108, 101, 109, 101, 110, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 115, 44, 32, 97, 110, 100, 32, 99, 111, 112, 121, 32, 111, 118, 101, 114, 108, 97, 112, 112, 105, 110, 103, 32, 114, 97, 110, 103, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 100, 105, 114, 101, 99, 116, 105, 111, 110, 59, 32, 115, 112, 108, 105, 99, 101, 32, 114, 101, 116, 117, 114, 110, 115, 32, 97, 32, 102, 114, 101, 115, 104, 32, 114, 101, 109, 111, 118, 101, 100, 32, 97, 114, 114, 97, 121, 46])
}

pub fn render_array_state_method_expression(target: self::ml_string::Text, operation: self::ml_string::Text, receiver: self::ml_string::Text, arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, profiles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, types: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, suffix: self::ml_string::Text) -> self::ml_string::Text {
    if ((self::read_array_state_method_result(operation.clone())) == (self::ml_string::Text::from_units(vec![]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            ((((((self::ml_string::Text::from_units(vec![40])).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![41, 46]))).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&self::ml_string::join(&{ let ml_reference_value12 = arguments_.borrow().clone(); ml_reference_value12 }, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if (target != (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                {
                    let owner = (self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 111, 119, 110, 101, 114])).concat(&suffix);
                    {
                        let output = ((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                        {
                            let index = 0f64;
                            {
                                let output_2 = self::source_ml_source1_temporary_render_array_state_method_expression_loop10(arguments_.clone(), types.clone(), suffix.clone(), output.clone(), index);
                                {
                                    let names = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                    {
                                        let index_2 = 0f64;
                                        {
                                            let names_2 = self::source_ml_source1_temporary_render_array_state_method_expression_loop11(arguments_.clone(), suffix.clone(), names.clone(), index_2);
                                            {
                                                let output_3 = output_2.concat(&((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 61, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 46, 108, 101, 110, 40, 41, 59, 32])));
                                                if (operation == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101]))) {
                                                    {
                                                        let begin = if (({ let ml_reference_array13 = arguments_.clone(); let ml_reference_projection13 = ml_reference_array13.borrow().len() as f64; ml_reference_projection13 }) == (0f64)) {
                                                            self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                                                        } else {
                                                            self::render_array_state_numeric_value(({
                                                                let source_ml_source2_array_read_value20 = profiles.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index21 = 0f64;
                                                                    { let ml_reference_array14 = source_ml_source2_array_read_value20.clone(); let ml_reference_position14 = source_ml_source2_array_read_index21; let ml_reference_projection14 = self::ml_array::optional_at(&*ml_reference_array14.borrow(), self::ml_array::number_index(ml_reference_position14)); ml_reference_projection14 }
                                                                }
                                                            }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), ({
                                                                let source_ml_source2_array_read_value22 = names_2.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index23 = 0f64;
                                                                    { let ml_reference_array15 = source_ml_source2_array_read_value22.clone(); let ml_reference_position15 = source_ml_source2_array_read_index23; let ml_reference_projection15 = self::ml_array::optional_at(&*ml_reference_array15.borrow(), self::ml_array::number_index(ml_reference_position15)); ml_reference_projection15 }
                                                                }
                                                            }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78]))
                                                        };
                                                        {
                                                            let output_4 = output_3.concat(&((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 61, 32])).concat(&self::render_array_state_relative_index(begin.clone(), self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104])))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                            {
                                                                let count = if (({ let ml_reference_array16 = arguments_.clone(); let ml_reference_projection16 = ml_reference_array16.borrow().len() as f64; ml_reference_projection16 }) == (0f64)) {
                                                                    self::ml_string::Text::from_units(vec![48, 117, 115, 105, 122, 101])
                                                                } else {
                                                                    if (({ let ml_reference_array17 = arguments_.clone(); let ml_reference_projection17 = ml_reference_array17.borrow().len() as f64; ml_reference_projection17 }) == (1f64)) {
                                                                        self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116])
                                                                    } else {
                                                                        ((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 108, 95, 100, 101, 108, 101, 116, 101, 95, 118, 97, 108, 117, 101, 58, 32, 102, 54, 52, 32, 61, 32])).concat(&self::render_array_state_numeric_value(({
                                                                            let source_ml_source2_array_read_value24 = profiles.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index25 = 1f64;
                                                                                { let ml_reference_array18 = source_ml_source2_array_read_value24.clone(); let ml_reference_position18 = source_ml_source2_array_read_index25; let ml_reference_projection18 = self::ml_array::optional_at(&*ml_reference_array18.borrow(), self::ml_array::number_index(ml_reference_position18)); ml_reference_projection18 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), ({
                                                                            let source_ml_source2_array_read_value26 = names_2.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index27 = 1f64;
                                                                                { let ml_reference_array19 = source_ml_source2_array_read_value26.clone(); let ml_reference_position19 = source_ml_source2_array_read_index27; let ml_reference_projection19 = self::ml_array::optional_at(&*ml_reference_array19.borrow(), self::ml_array::number_index(ml_reference_position19)); ml_reference_projection19 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78])))).concat(&self::ml_string::Text::from_units(vec![59, 32, 109, 108, 95, 100, 101, 108, 101, 116, 101, 95, 118, 97, 108, 117, 101, 46, 116, 114, 117, 110, 99, 40, 41, 46, 109, 97, 120, 40, 48, 102, 54, 52, 41, 46, 109, 105, 110, 40, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 41, 32, 97, 115, 32, 102, 54, 52, 41, 32, 97, 115, 32, 117, 115, 105, 122, 101, 32, 125]))
                                                                    }
                                                                };
                                                                {
                                                                    let inserted = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    {
                                                                        let index_3 = 2f64;
                                                                        {
                                                                            let inserted_2 = self::source_ml_source1_temporary_render_array_state_method_expression_loop12(arguments_.clone(), names_2.clone(), inserted.clone(), index_3);
                                                                            (((((((((output_4.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 61, 32]))).concat(&count)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 115, 101, 114, 116, 101, 100, 32, 61, 32, 118, 101, 99, 33, 91]))).concat(&self::ml_string::join(&{ let ml_reference_value20 = inserted_2.borrow().clone(); ml_reference_value20 }, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![93, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 95, 32, 61, 32, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 41, 46, 99, 104, 101, 99, 107, 101, 100, 95, 97, 100, 100, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 115, 101, 114, 116, 101, 100, 46, 108, 101, 110, 40, 41, 41, 46, 102, 105, 108, 116, 101, 114, 40, 124, 108, 101, 110, 103, 116, 104, 124, 32, 42, 108, 101, 110, 103, 116, 104, 32, 60, 61, 32, 52, 50, 57, 52, 57, 54, 55, 50, 57, 53, 117, 115, 105, 122, 101, 41, 46, 101, 120, 112, 101, 99, 116, 40, 34, 73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 34, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 114, 101, 109, 111, 118, 101, 100, 58, 32, 86, 101, 99, 60, 95, 62, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 46, 115, 112, 108, 105, 99, 101, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 44, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 115, 101, 114, 116, 101, 100, 41, 46, 99, 111, 108, 108, 101, 99, 116, 40, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 114, 99, 58, 58, 82, 99, 58, 58, 110, 101, 119, 40, 115, 116, 100, 58, 58, 99, 101, 108, 108, 58, 58, 82, 101, 102, 67, 101, 108, 108, 58, 58, 110, 101, 119, 40, 109, 108, 95, 114, 101, 109, 111, 118, 101, 100, 41, 41, 32, 125]))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let first = if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) {
                                                            1f64
                                                        } else {
                                                            0f64
                                                        };
                                                        {
                                                            let start_value = if (({ let ml_reference_array21 = arguments_.clone(); let ml_reference_projection21 = ml_reference_array21.borrow().len() as f64; ml_reference_projection21 }) <= first) {
                                                                self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                                                            } else {
                                                                self::render_array_state_numeric_value(({
                                                                    let source_ml_source2_array_read_value28 = profiles.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index29 = first;
                                                                        { let ml_reference_array22 = source_ml_source2_array_read_value28.clone(); let ml_reference_position22 = source_ml_source2_array_read_index29; let ml_reference_projection22 = self::ml_array::optional_at(&*ml_reference_array22.borrow(), self::ml_array::number_index(ml_reference_position22)); ml_reference_projection22 }
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), ({
                                                                    let source_ml_source2_array_read_value30 = names_2.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index31 = first;
                                                                        { let ml_reference_array23 = source_ml_source2_array_read_value30.clone(); let ml_reference_position23 = source_ml_source2_array_read_index31; let ml_reference_projection23 = self::ml_array::optional_at(&*ml_reference_array23.borrow(), self::ml_array::number_index(ml_reference_position23)); ml_reference_projection23 }
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78]))
                                                            };
                                                            {
                                                                let output_5 = output_3.concat(&((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 61, 32])).concat(&self::render_array_state_relative_index(start_value.clone(), self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104])))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                                {
                                                                    let end_value = if (({ let ml_reference_array24 = arguments_.clone(); let ml_reference_projection24 = ml_reference_array24.borrow().len() as f64; ml_reference_projection24 }) <= (2f64)) {
                                                                        self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 97, 115, 32, 102, 54, 52])
                                                                    } else {
                                                                        self::render_array_state_numeric_value(({
                                                                            let source_ml_source2_array_read_value32 = profiles.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index33 = 2f64;
                                                                                { let ml_reference_array25 = source_ml_source2_array_read_value32.clone(); let ml_reference_position25 = source_ml_source2_array_read_index33; let ml_reference_projection25 = self::ml_array::optional_at(&*ml_reference_array25.borrow(), self::ml_array::number_index(ml_reference_position25)); ml_reference_projection25 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), ({
                                                                            let source_ml_source2_array_read_value34 = names_2.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index35 = 2f64;
                                                                                { let ml_reference_array26 = source_ml_source2_array_read_value34.clone(); let ml_reference_position26 = source_ml_source2_array_read_index35; let ml_reference_projection26 = self::ml_array::optional_at(&*ml_reference_array26.borrow(), self::ml_array::number_index(ml_reference_position26)); ml_reference_projection26 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 97, 115, 32, 102, 54, 52]))
                                                                    };
                                                                    {
                                                                        let end = ((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 32, 61, 32])).concat(&self::render_array_state_relative_index(end_value.clone(), self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104])))).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                                                                        if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) {
                                                                            ((((((((output_5.concat(&end)).concat(&self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 117, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![105, 102, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 60, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 32, 123, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 93, 46, 102, 105, 108, 108, 40]))).concat(&({
                                                                                let source_ml_source2_array_read_value36 = names_2.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index37 = 0f64;
                                                                                    { let ml_reference_array27 = source_ml_source2_array_read_value36.clone(); let ml_reference_position27 = source_ml_source2_array_read_index37; let ml_reference_projection27 = self::ml_array::optional_at(&*ml_reference_array27.borrow(), self::ml_array::number_index(ml_reference_position27)); ml_reference_projection27 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![41, 59, 32, 125, 32, 125, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                                                        } else {
                                                                            {
                                                                                let from_value = if (({ let ml_reference_array28 = arguments_.clone(); let ml_reference_projection28 = ml_reference_array28.borrow().len() as f64; ml_reference_projection28 }) <= (1f64)) {
                                                                                    self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                                                                                } else {
                                                                                    self::render_array_state_numeric_value(({
                                                                                        let source_ml_source2_array_read_value38 = profiles.clone();
                                                                                        {
                                                                                            let source_ml_source2_array_read_index39 = 1f64;
                                                                                            { let ml_reference_array29 = source_ml_source2_array_read_value38.clone(); let ml_reference_position29 = source_ml_source2_array_read_index39; let ml_reference_projection29 = self::ml_array::optional_at(&*ml_reference_array29.borrow(), self::ml_array::number_index(ml_reference_position29)); ml_reference_projection29 }
                                                                                        }
                                                                                    }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), ({
                                                                                        let source_ml_source2_array_read_value40 = names_2.clone();
                                                                                        {
                                                                                            let source_ml_source2_array_read_index41 = 1f64;
                                                                                            { let ml_reference_array30 = source_ml_source2_array_read_value40.clone(); let ml_reference_position30 = source_ml_source2_array_read_index41; let ml_reference_projection30 = self::ml_array::optional_at(&*ml_reference_array30.borrow(), self::ml_array::number_index(ml_reference_position30)); ml_reference_projection30 }
                                                                                        }
                                                                                    }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78]))
                                                                                };
                                                                                (((((((((((output_5.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 61, 32]))).concat(&self::render_array_state_relative_index(from_value.clone(), self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104])))).concat(&self::ml_string::Text::from_units(vec![59, 32]))).concat(&end)).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 61, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 46, 115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 115, 117, 98, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 41, 46, 109, 105, 110, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 117, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32, 105, 102, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 60, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 38, 38, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 60, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 123, 32]))).concat(&self::ml_string::Text::from_units(vec![102, 111, 114, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 32, 105, 110, 32, 40, 48, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 41, 46, 114, 101, 118, 40, 41, 32, 123, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 32, 61, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 46, 99, 108, 111, 110, 101, 40, 41, 59, 32, 125, 32]))).concat(&self::ml_string::Text::from_units(vec![125, 32, 101, 108, 115, 101, 32, 123, 32, 102, 111, 114, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 32, 105, 110, 32, 48, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 123, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 32, 61, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 46, 99, 108, 111, 110, 101, 40, 41, 59, 32, 125, 32, 125, 32, 125, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
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

pub fn read_collection_access_result(kind: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if ((kind != (self::ml_string::Text::from_units(vec![109, 97, 112]))) && (kind != (self::ml_string::Text::from_units(vec![115, 101, 116])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![104, 97, 115]))) {
            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
        } else {
            if ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && (operation == (self::ml_string::Text::from_units(vec![103, 101, 116])))) {
                self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 32, 118, 97, 108, 117, 101])
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) {
                    self::ml_string::Text::from_units(vec![107, 101, 121, 115])
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))) {
                        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))) {
                            self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])
                        } else {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

pub fn read_collection_access_argument_count(operation: self::ml_string::Text) -> f64 {
    if ((operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))) || (operation == (self::ml_string::Text::from_units(vec![104, 97, 115])))) {
        1f64
    } else {
        0f64
    }
}

pub fn render_collection_size_expression(target: self::ml_string::Text, object: self::ml_string::Text) -> self::ml_string::Text {
    if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        ((self::ml_string::Text::from_units(vec![40])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![41, 46, 115, 105, 122, 101]))
    } else {
        if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
            ((self::ml_string::Text::from_units(vec![40])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![46, 108, 101, 110, 40, 41, 32, 97, 115, 32, 102, 54, 52, 41]))
        } else {
            if (target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                ((self::ml_string::Text::from_units(vec![40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![46, 115, 105, 122, 101, 41]))
            } else {
                if (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113]))) {
                    ((self::ml_string::Text::from_units(vec![40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54, 51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102, 95, 110, 97, 116, 32, 40, 76, 105, 115, 116, 46, 108, 101, 110, 103, 116, 104, 32])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![41, 41, 41, 41]))
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn render_collection_read_expression(target: self::ml_string::Text, kind: self::ml_string::Text, operation: self::ml_string::Text, object: self::ml_string::Text, search: self::ml_string::Text, float_key: bool, comparison: self::ml_string::Text, key_type: self::ml_string::Text, value_type: self::ml_string::Text) -> self::ml_string::Text {
    if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        ((((((self::ml_string::Text::from_units(vec![40])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![41, 46]))).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&search)).concat(&self::ml_string::Text::from_units(vec![41]))
    } else {
        if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
            {
                let equal = if float_key {
                    self::ml_string::Text::from_units(vec![40, 109, 108, 95, 107, 101, 121, 32, 61, 61, 32, 38, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32, 124, 124, 32, 40, 109, 108, 95, 107, 101, 121, 46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 38, 38, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 46, 105, 115, 95, 110, 97, 110, 40, 41, 41, 41])
                } else {
                    self::ml_string::Text::from_units(vec![109, 108, 95, 107, 101, 121, 32, 61, 61, 32, 38, 109, 108, 95, 115, 101, 97, 114, 99, 104])
                };
                {
                    let bindings = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                        self::ml_string::Text::from_units(vec![40, 109, 108, 95, 107, 101, 121, 44, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41])
                    } else {
                        self::ml_string::Text::from_units(vec![109, 108, 95, 107, 101, 121])
                    };
                    {
                        let action = if (operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))) {
                            ((((self::ml_string::Text::from_units(vec![102, 105, 110, 100, 95, 109, 97, 112, 40, 124])).concat(&bindings)).concat(&self::ml_string::Text::from_units(vec![124, 32, 105, 102, 32]))).concat(&equal)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40, 109, 108, 95, 118, 97, 108, 117, 101, 46, 99, 108, 111, 110, 101, 40, 41, 41, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 78, 111, 110, 101, 32, 125, 41]))
                        } else {
                            ((((self::ml_string::Text::from_units(vec![97, 110, 121, 40, 124])).concat(&bindings)).concat(&self::ml_string::Text::from_units(vec![124, 32]))).concat(&equal)).concat(&self::ml_string::Text::from_units(vec![41]))
                        };
                        ((((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 108, 95, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 61, 32])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32, 61, 32]))).concat(&search)).concat(&self::ml_string::Text::from_units(vec![59, 32, 109, 108, 95, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 46, 105, 116, 101, 114, 40, 41, 46]))).concat(&action)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                    }
                }
            }
        } else {
            if (target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                {
                    let key = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                        self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 46, 49])
                    } else {
                        self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])
                    };
                    {
                        let equal_2 = if float_key {
                            ((((self::ml_string::Text::from_units(vec![40])).concat(&key)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32, 124, 124, 32, 40]))).concat(&key)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 78, 97, 78, 32, 38, 38, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 46, 105, 115, 78, 97, 78, 41, 41]))
                        } else {
                            ((self::ml_string::Text::from_units(vec![40])).concat(&key)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 41]))
                        };
                        {
                            let result = if (operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))) {
                                ((self::ml_string::Text::from_units(vec![40, 40, 109, 108, 95, 118, 97, 108, 117, 101, 115, 46, 116, 111, 76, 105, 115, 116, 46, 102, 105, 110, 100, 63, 32, 40, 102, 117, 110, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 62, 32])).concat(&equal_2)).concat(&self::ml_string::Text::from_units(vec![41, 41, 46, 109, 97, 112, 32, 40, 102, 117, 110, 32, 109, 108, 95, 112, 97, 105, 114, 32, 61, 62, 32, 109, 108, 95, 112, 97, 105, 114, 46, 50, 41, 41]))
                            } else {
                                ((self::ml_string::Text::from_units(vec![40, 109, 108, 95, 118, 97, 108, 117, 101, 115, 46, 97, 110, 121, 32, 40, 102, 117, 110, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 62, 32])).concat(&equal_2)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                            };
                            ((((((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 115, 32, 58, 61, 32])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32, 58, 61, 32]))).concat(&search)).concat(&self::ml_string::Text::from_units(vec![59, 32]))).concat(&result)).concat(&self::ml_string::Text::from_units(vec![41]))
                        }
                    }
                }
            } else {
                if (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113]))) {
                    {
                        let key_2 = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                            self::ml_string::Text::from_units(vec![40, 102, 115, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41])
                        } else {
                            self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])
                        };
                        {
                            let equal_3 = if float_key {
                                ((((self::ml_string::Text::from_units(vec![40, 111, 114, 98, 32])).concat(&comparison)).concat(&self::ml_string::Text::from_units(vec![32, 40, 97, 110, 100, 98, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 105, 115, 95, 110, 97, 110, 32]))).concat(&key_2)).concat(&self::ml_string::Text::from_units(vec![41, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 105, 115, 95, 110, 97, 110, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 41, 41, 41]))
                            } else {
                                comparison.clone()
                            };
                            {
                                let result_2 = if (operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))) {
                                    ((((((self::ml_string::Text::from_units(vec![40, 111, 112, 116, 105, 111, 110, 95, 109, 97, 112, 32, 40, 64, 115, 110, 100, 32])).concat(&key_type)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&value_type)).concat(&self::ml_string::Text::from_units(vec![41, 32, 40, 76, 105, 115, 116, 46, 102, 105, 110, 100, 32, 40, 102, 117, 110, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 62, 32]))).concat(&equal_3)).concat(&self::ml_string::Text::from_units(vec![41, 32, 109, 108, 95, 118, 97, 108, 117, 101, 115, 41, 41]))
                                } else {
                                    ((self::ml_string::Text::from_units(vec![40, 76, 105, 115, 116, 46, 101, 120, 105, 115, 116, 115, 98, 32, 40, 102, 117, 110, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 62, 32])).concat(&equal_3)).concat(&self::ml_string::Text::from_units(vec![41, 32, 109, 108, 95, 118, 97, 108, 117, 101, 115, 41]))
                                };
                                ((((((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 115, 32, 58, 61, 32])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![32, 105, 110, 32, 108, 101, 116, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32, 58, 61, 32]))).concat(&search)).concat(&self::ml_string::Text::from_units(vec![32, 105, 110, 32]))).concat(&result_2)).concat(&self::ml_string::Text::from_units(vec![41]))
                            }
                        }
                    }
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn read_disjoint_collection_iteration_source_operation() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![114, 101, 97, 100, 67, 108, 111, 115, 101, 100, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 105, 111, 110])
}

pub fn read_disjoint_collection_spread_source_operation() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![114, 101, 97, 100, 67, 108, 111, 115, 101, 100, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 83, 112, 114, 101, 97, 100])
}

pub fn read_disjoint_collection_iteration_marker(ordinal: f64, operation: self::ml_string::Text) -> self::ml_string::Text {
    (((self::ml_string::Text::from_units(vec![35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32])).concat(&{
        let source_ml_source2_string_argument42 = ordinal;
        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument42))
    })).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&operation)
}

pub fn read_disjoint_collection_spread_marker(ordinal: f64, operation: self::ml_string::Text) -> self::ml_string::Text {
    (((self::ml_string::Text::from_units(vec![35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 112, 114, 101, 97, 100, 32])).concat(&{
        let source_ml_source2_string_argument43 = ordinal;
        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument43))
    })).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&operation)
}

pub fn read_disjoint_collection_iteration_marker_role(operation: self::ml_string::Text) -> self::ml_string::Text {
    if operation.starts_with(&self::ml_string::Text::from_units(vec![35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32])) {
        self::ml_string::Text::from_units(vec![108, 111, 111, 112])
    } else {
        if operation.starts_with(&self::ml_string::Text::from_units(vec![35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 112, 114, 101, 97, 100, 32])) {
            self::ml_string::Text::from_units(vec![101, 97, 103, 101, 114])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn accept_disjoint_collection_spread_receiver(shape: self::ml_string::Text) -> bool {
    ((shape == (self::ml_string::Text::from_units(vec![110, 97, 109, 101]))) || (shape == (self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100]))))
}

pub fn accept_disjoint_collection_iteration_scope(known: bool, externals: f64, literals: f64, bindings: f64, directory: bool, publication: bool) -> bool {
    (((((known && (externals == (0f64))) && (literals == (0f64))) && (bindings == (0f64))) && !directory) && !publication)
}

pub fn accept_disjoint_collection_iteration_operation(kind: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    (((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) || (kind == (self::ml_string::Text::from_units(vec![115, 101, 116])))) && (operation == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))))
}

pub fn read_disjoint_collection_iteration_item_form(kind: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if self::accept_disjoint_collection_iteration_operation(kind.clone(), operation.clone()) {
        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])
    } else {
        {
            let form = self::read_collection_access_result(kind.clone(), operation.clone());
            if ((form == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) || (form == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])))) {
                form.clone()
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_disjoint_collection_iteration_identity_kind(kind: self::ml_string::Text) -> bool {
    self::ml_array::includes(&{ let ml_reference_value31 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]), self::ml_string::Text::from_units(vec![100, 97, 116, 97]), self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![99, 101, 108, 108]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value31 }, &kind.clone(), 0f64, false, |a, b| a == b)
}

pub fn read_disjoint_collection_iteration_method_effect(kind: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if ((((self::read_reference_method_result(kind.clone(), operation.clone())) != (self::ml_string::Text::from_units(vec![]))) && (operation != (self::ml_string::Text::from_units(vec![103, 101, 116])))) && (operation != (self::ml_string::Text::from_units(vec![104, 97, 115])))) {
        self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])
    } else {
        if ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) || (kind == (self::ml_string::Text::from_units(vec![115, 101, 116])))) {
            if (self::ml_array::includes(&{ let ml_reference_value32 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![104, 97, 115]), self::ml_string::Text::from_units(vec![107, 101, 121, 115]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])]))).borrow().clone(); ml_reference_value32 }, &operation.clone(), 0f64, false, |a, b| a == b) || ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && (operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))))) {
                self::ml_string::Text::from_units(vec![114, 101, 97, 100])
            } else {
                self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
            }
        } else {
            if (kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
                if self::ml_array::includes(&{ let ml_reference_value33 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 111, 114, 116]), self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101]), self::ml_string::Text::from_units(vec![102, 105, 108, 108]), self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110])]))).borrow().clone(); ml_reference_value33 }, &operation.clone(), 0f64, false, |a, b| a == b) {
                    self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])
                } else {
                    if (self::read_disjoint_collection_iteration_callback(operation.clone()) || self::ml_array::includes(&{ let ml_reference_value34 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116]), self::ml_string::Text::from_units(vec![106, 111, 105, 110]), self::ml_string::Text::from_units(vec![105, 110, 99, 108, 117, 100, 101, 115]), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 79, 102]), self::ml_string::Text::from_units(vec![108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102]), self::ml_string::Text::from_units(vec![97, 116]), self::ml_string::Text::from_units(vec![102, 108, 97, 116]), self::ml_string::Text::from_units(vec![116, 111, 82, 101, 118, 101, 114, 115, 101, 100]), self::ml_string::Text::from_units(vec![116, 111, 83, 111, 114, 116, 101, 100])]))).borrow().clone(); ml_reference_value34 }, &operation.clone(), 0f64, false, |a, b| a == b)) {
                        self::ml_string::Text::from_units(vec![114, 101, 97, 100])
                    } else {
                        self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
                    }
                }
            } else {
                if (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
                    if ((self::read_disjoint_collection_iteration_write_field(operation.clone())) >= (0f64)) {
                        self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 99, 111, 114, 100, 80, 114, 111, 112, 101, 114, 116, 121, 79, 114, 100, 101, 114]))) {
                            self::ml_string::Text::from_units(vec![114, 101, 97, 100])
                        } else {
                            self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
                        }
                    }
                } else {
                    self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
                }
            }
        }
    }
}

pub fn accept_disjoint_collection_iteration_read_expression(shape: self::ml_string::Text) -> bool {
    self::ml_array::includes(&{ let ml_reference_value35 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 116]), self::ml_string::Text::from_units(vec![105, 102]), self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104]), self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116]), self::ml_string::Text::from_units(vec![99, 111, 97, 108, 101, 115, 99, 101]), self::ml_string::Text::from_units(vec![115, 111, 109, 101]), self::ml_string::Text::from_units(vec![102, 108, 97, 116, 116, 101, 110, 79, 112, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112, 79, 112, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![108, 105, 116]), self::ml_string::Text::from_units(vec![117, 110, 105, 116]), self::ml_string::Text::from_units(vec![110, 111, 110, 101]), self::ml_string::Text::from_units(vec![97, 98, 111, 114, 116]), self::ml_string::Text::from_units(vec![105, 109, 112, 111, 115, 115, 105, 98, 108, 101]), self::ml_string::Text::from_units(vec![108, 97, 109, 98, 100, 97]), self::ml_string::Text::from_units(vec![99, 101, 108, 108, 82, 101, 97, 100]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 83, 105, 122, 101]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 82, 101, 97, 100]), self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121]), self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]), self::ml_string::Text::from_units(vec![110, 117, 108, 108, 105, 115, 104, 78, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 73, 110, 100, 101, 120, 78, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![116, 114, 117, 116, 104, 121]), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 84, 121, 112, 101, 79, 102]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 84, 101, 120, 116]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 80, 114, 101, 100, 105, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 70, 105, 101, 108, 100]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 80, 114, 111, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 73, 110, 100, 101, 120]), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 80, 111, 105, 110, 116, 115]), self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 77, 97, 112]), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 84, 101, 115, 116]), self::ml_string::Text::from_units(vec![99, 97, 115, 116]), self::ml_string::Text::from_units(vec![97, 112, 112, 101, 110, 100]), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])]))).borrow().clone(); ml_reference_value35 }, &shape.clone(), 0f64, false, |a, b| a == b)
}

pub fn read_disjoint_collection_iteration_write_field(operation: self::ml_string::Text) -> f64 {
    {
        let prefix = self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]);
        if !(operation.starts_with(&prefix)) {
            (-1f64)
        } else {
            {
                let text = {
                    let source_ml_source2_string_range_operand44 = operation.clone();
                    {
                        let source_ml_source2_string_range_operand45 = prefix.length();
                        {
                            let source_ml_source2_string_range_position46 = source_ml_source2_string_range_operand45;
                            {
                                let source_ml_source2_string_range_position47 = f64::INFINITY;
                                source_ml_source2_string_range_operand44.slice(source_ml_source2_string_range_position46, source_ml_source2_string_range_position47)
                            }
                        }
                    }
                };
                {
                    let index = self::ml_string_number::convert_java_script_string_number_units(&(text.clone()).0.iter().copied().map(f64::from).collect::<Vec<f64>>());
                    if ((self::ml_math::is_integer(index) && (index >= (0f64))) && (({
                        let source_ml_source2_string_argument49 = index;
                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument49))
                    }) == text)) {
                        index
                    } else {
                        (-1f64)
                    }
                }
            }
        }
    }
}

pub fn read_disjoint_collection_iteration_callback(operation: self::ml_string::Text) -> bool {
    ((((self::read_array_callback_result_form(operation.clone())) != (self::ml_string::Text::from_units(vec![]))) || (operation == (self::ml_string::Text::from_units(vec![115, 111, 114, 116])))) || (operation == (self::ml_string::Text::from_units(vec![116, 111, 83, 111, 114, 116, 101, 100]))))
}

pub fn accept_disjoint_collection_iteration_callback_head(role: self::ml_string::Text) -> bool {
    (((role == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) || (role == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101])))) || (role == (self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 98, 108, 101]))))
}

pub fn add_fact(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, fact: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (((self::read_required_number_fact(facts.clone(), 0f64)) == ((-1f64))) || self::ml_array::includes(&{ let ml_reference_value36 = facts.borrow().clone(); ml_reference_value36 }, &fact, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
        facts.clone()
    } else {
        std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_concat_value50 = facts.clone();
            {
                let source_ml_source2_concat_value51 = std::rc::Rc::new(std::cell::RefCell::new(vec![fact]));
                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value37 = source_ml_source2_concat_value50.borrow().clone(); ml_reference_value37 }), { let ml_reference_value38 = source_ml_source2_concat_value51.borrow().clone(); ml_reference_value38 })
            }
        }))
    }
}

pub fn add_value(values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, value: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if self::ml_array::includes(&{ let ml_reference_value39 = values.borrow().clone(); ml_reference_value39 }, &value, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) {
        values.clone()
    } else {
        std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_concat_value52 = values.clone();
            {
                let source_ml_source2_concat_value53 = std::rc::Rc::new(std::cell::RefCell::new(vec![value]));
                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value40 = source_ml_source2_concat_value52.borrow().clone(); ml_reference_value40 }), { let ml_reference_value41 = source_ml_source2_concat_value53.borrow().clone(); ml_reference_value41 })
            }
        }))
    }
}

pub fn read_roots(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, tag: f64, node: f64, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let source_ml_source1_temporary_values13 = facts.clone();
            {
                let source_ml_source1_temporary_index13 = self::ml::Big::from_i128(0);
                {
                    let result_2 = self::source_ml_source1_temporary_read_roots_loop14(tag, node, width, span, result.clone(), source_ml_source1_temporary_values13.clone(), source_ml_source1_temporary_index13.clone());
                    result_2.clone()
                }
            }
        }
    }
}

pub fn add_roots(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, tag: f64, node: f64, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let present = self::read_roots(facts.clone(), tag, node, width, span);
        {
            let source_ml_source1_temporary_values15 = roots.clone();
            {
                let source_ml_source1_temporary_index15 = self::ml::Big::from_i128(0);
                {
                    let present_2 = self::source_ml_source1_temporary_add_roots_loop17(present.clone(), source_ml_source1_temporary_values15.clone(), source_ml_source1_temporary_index15.clone());
                    if (({ let ml_reference_array42 = present_2.clone(); let ml_reference_projection42 = ml_reference_array42.borrow().len() as f64; ml_reference_projection42 }) > (32f64)) {
                        std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]))
                    } else {
                        {
                            let result = facts.clone();
                            {
                                let source_ml_source1_temporary_values16 = roots.clone();
                                {
                                    let source_ml_source1_temporary_index16 = self::ml::Big::from_i128(0);
                                    {
                                        let result_2 = self::source_ml_source1_temporary_add_roots_loop18(tag, node, width, span, result.clone(), source_ml_source1_temporary_values16.clone(), source_ml_source1_temporary_index16.clone());
                                        result_2.clone()
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

pub fn edges(starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64, label: self::ml_string::Text) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let at = self::read_required_number_fact(starts.clone(), node);
            {
                let result_2 = self::source_ml_source1_temporary_edges_loop19(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, label.clone(), result.clone(), at);
                result_2.clone()
            }
        }
    }
}

pub fn read_field(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, field: f64, field_width: f64, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let source_ml_source1_temporary_values20 = roots.clone();
            {
                let source_ml_source1_temporary_index20 = self::ml::Big::from_i128(0);
                {
                    let result_2 = self::source_ml_source1_temporary_read_field_loop22(facts.clone(), field, field_width, width, span, result.clone(), source_ml_source1_temporary_values20.clone(), source_ml_source1_temporary_index20.clone());
                    result_2.clone()
                }
            }
        }
    }
}

pub fn add_field(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, field: f64, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, field_width: f64, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = facts.clone();
        {
            let source_ml_source1_temporary_values24 = owners.clone();
            {
                let source_ml_source1_temporary_index24 = self::ml::Big::from_i128(0);
                {
                    let result_2 = self::source_ml_source1_temporary_add_field_loop26(field, roots.clone(), field_width, width, span, result.clone(), source_ml_source1_temporary_values24.clone(), source_ml_source1_temporary_index24.clone());
                    result_2.clone()
                }
            }
        }
    }
}

pub fn copy_fields(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, destination: f64, field_width: f64, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = facts.clone();
        {
            let source_ml_source1_temporary_values28 = roots.clone();
            {
                let source_ml_source1_temporary_index28 = self::ml::Big::from_i128(0);
                {
                    let result_2 = self::source_ml_source1_temporary_copy_fields_loop30(facts.clone(), destination, field_width, width, span, result.clone(), source_ml_source1_temporary_values28.clone(), source_ml_source1_temporary_index28.clone());
                    result_2.clone()
                }
            }
        }
    }
}

pub fn called(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, source: f64, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    self::read_roots(facts.clone(), 3f64, source, width, span)
}

pub fn reachable(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, initial: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = initial.clone();
        {
            let progress = true;
            {
                let result_2 = self::source_ml_source1_temporary_reachable_loop34(facts.clone(), width, span, result.clone(), progress);
                result_2.clone()
            }
        }
    }
}

pub fn separate_roots(left: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, right: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> bool {
    (((((({ let ml_reference_array43 = left.clone(); let ml_reference_projection43 = ml_reference_array43.borrow().len() as f64; ml_reference_projection43 }) > (0f64)) && (({ let ml_reference_array44 = right.clone(); let ml_reference_projection44 = ml_reference_array44.borrow().len() as f64; ml_reference_projection44 }) > (0f64))) && !(self::ml_array::includes(&{ let ml_reference_value45 = left.borrow().clone(); ml_reference_value45 }, &(-1f64), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) && !(self::ml_array::includes(&{ let ml_reference_value46 = right.borrow().clone(); ml_reference_value46 }, &(-1f64), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) && !({ let ml_values47 = { let ml_reference_value48 = left.borrow().clone(); ml_reference_value48 }; let ml_callback47 = { let right = right.clone(); std::rc::Rc::new(move |root: f64| -> bool { self::ml_array::includes(&{ let ml_reference_value49 = right.borrow().clone(); ml_reference_value49 }, &root, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) }) }; ml_values47.iter().cloned().enumerate().any(|(ml_index47, ml_value47)| ml_callback47(ml_value47)) }))
}

pub fn accepts_effects(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, scopes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, source: f64, allocation: bool, main_: f64, width: f64, span: f64) -> bool {
    {
        let roots = self::read_roots(facts.clone(), 1f64, source, width, span);
        if (allocation && ((({ let ml_reference_array50 = roots.clone(); let ml_reference_projection50 = ml_reference_array50.borrow().len() as f64; ml_reference_projection50 }) == (0f64)) || { let ml_values51 = { let ml_reference_value52 = roots.borrow().clone(); ml_reference_value52 }; let ml_callback51 = { let roles = roles.clone(); let kinds = kinds.clone(); let source = source; std::rc::Rc::new(move |root: f64| -> bool { (((root < (0f64)) || ((self::text_fact(roles.clone(), root)) != (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110])))) || ((self::text_fact(kinds.clone(), root)) != (self::text_fact(kinds.clone(), source)))) }) }; ml_values51.iter().cloned().enumerate().any(|(ml_index51, ml_value51)| ml_callback51(ml_value51)) })) {
            false
        } else {
            {
                let reached = self::reachable(facts.clone(), scopes.clone(), width, span);
                {
                    let source_ml_source1_temporary_values37 = reached.clone();
                    {
                        let source_ml_source1_temporary_index37 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s54 = self::source_ml_source1_temporary_accepts_effects_loop38(functions.clone(), bodies.clone(), main_, source_ml_source1_temporary_values37.clone(), source_ml_source1_temporary_index37.clone());
                            match source_ml_source2_s54.clone() {
                                self::MlSource1TemporaryAcceptsEffectsLoop38Result::MlSource1TemporaryAcceptsEffectsLoop38Done => {
                                    {
                                        let node = 0f64;
                                        {
                                            let source_ml_source2_s55 = self::source_ml_source1_temporary_accepts_effects_loop39(facts.clone(), roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), source, width, span, roots.clone(), reached.clone(), node);
                                            match source_ml_source2_s55.clone() {
                                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Done => {
                                                    true
                                                }
                                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(source_ml_source1_temporary_result39) => {
                                                    source_ml_source1_temporary_result39
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryAcceptsEffectsLoop38Result::MlSource1TemporaryAcceptsEffectsLoop38Return(source_ml_source1_temporary_result40) => {
                                    source_ml_source1_temporary_result40
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn bind_calls(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, owner: f64, callees: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, callback: self::ml_string::Text, receiver: f64, kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, main_: f64, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = facts.clone();
        {
            let source_ml_source1_temporary_values41 = callees.clone();
            {
                let source_ml_source1_temporary_index41 = self::ml::Big::from_i128(0);
                {
                    let result_2 = self::source_ml_source1_temporary_bind_calls_loop43(owner, arguments_.clone(), callback.clone(), receiver, kinds.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), bodies.clone(), main_, width, span, result.clone(), source_ml_source1_temporary_values41.clone(), source_ml_source1_temporary_index41.clone());
                    result_2.clone()
                }
            }
        }
    }
}

pub fn prove_disjoint_collection_iterations(roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, main_: f64, snapshots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, callbacks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let size = { let ml_reference_array53 = roles.clone(); let ml_reference_projection53 = ml_reference_array53.borrow().len() as f64; ml_reference_projection53 };
        {
            let width = (size + 1f64);
            {
                let field_width = ({ let ml_reference_array54 = labels.clone(); let ml_reference_projection54 = ml_reference_array54.borrow().len() as f64; ml_reference_projection54 } + 1f64);
                {
                    let span = (((size * field_width) * width) + 1f64);
                    if (((((((((((((((size < (1f64)) || (size > (4096f64))) || (({ let ml_reference_array55 = labels.clone(); let ml_reference_projection55 = ml_reference_array55.borrow().len() as f64; ml_reference_projection55 }) > (32768f64))) || (({ let ml_reference_array56 = operations.clone(); let ml_reference_projection56 = ml_reference_array56.borrow().len() as f64; ml_reference_projection56 }) != size)) || (({ let ml_reference_array57 = kinds.clone(); let ml_reference_projection57 = ml_reference_array57.borrow().len() as f64; ml_reference_projection57 }) != size)) || (({ let ml_reference_array58 = owners.clone(); let ml_reference_projection58 = ml_reference_array58.borrow().len() as f64; ml_reference_projection58 }) != size)) || (({ let ml_reference_array59 = starts.clone(); let ml_reference_projection59 = ml_reference_array59.borrow().len() as f64; ml_reference_projection59 }) != size)) || (({ let ml_reference_array60 = counts.clone(); let ml_reference_projection60 = ml_reference_array60.borrow().len() as f64; ml_reference_projection60 }) != size)) || (({ let ml_reference_array61 = labels.clone(); let ml_reference_projection61 = ml_reference_array61.borrow().len() as f64; ml_reference_projection61 }) != ({ let ml_reference_array62 = targets.clone(); let ml_reference_projection62 = ml_reference_array62.borrow().len() as f64; ml_reference_projection62 }))) || (({ let ml_reference_array63 = functions.clone(); let ml_reference_projection63 = ml_reference_array63.borrow().len() as f64; ml_reference_projection63 }) != ({ let ml_reference_array64 = parameter_starts.clone(); let ml_reference_projection64 = ml_reference_array64.borrow().len() as f64; ml_reference_projection64 }))) || (({ let ml_reference_array65 = functions.clone(); let ml_reference_projection65 = ml_reference_array65.borrow().len() as f64; ml_reference_projection65 }) != ({ let ml_reference_array66 = parameter_counts.clone(); let ml_reference_projection66 = ml_reference_array66.borrow().len() as f64; ml_reference_projection66 }))) || (({ let ml_reference_array67 = functions.clone(); let ml_reference_projection67 = ml_reference_array67.borrow().len() as f64; ml_reference_projection67 }) != ({ let ml_reference_array68 = bodies.clone(); let ml_reference_projection68 = ml_reference_array68.borrow().len() as f64; ml_reference_projection68 }))) || !(self::ml_array::includes(&{ let ml_reference_value69 = functions.borrow().clone(); ml_reference_value69 }, &main_, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) || ((({ let ml_reference_array70 = snapshots.clone(); let ml_reference_projection70 = ml_reference_array70.borrow().len() as f64; ml_reference_projection70 } % 2f64)) != (0f64))) || ((({ let ml_reference_array71 = callbacks.clone(); let ml_reference_projection71 = ml_reference_array71.borrow().len() as f64; ml_reference_projection71 } % 2f64)) != (0f64))) {
                        std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))
                    } else {
                        {
                            let node = 0f64;
                            {
                                let source_ml_source2_s56 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop54(owners.clone(), starts.clone(), counts.clone(), targets.clone(), size, node);
                                match source_ml_source2_s56.clone() {
                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Done => {
                                        if ({ let ml_values72 = { let ml_reference_value73 = targets.borrow().clone(); ml_reference_value73 }; let ml_callback72 = { let size = size; std::rc::Rc::new(move |target: f64| -> bool { ((!(self::ml_math::is_integer(target)) || (target < (0f64))) || (target >= size)) }) }; ml_values72.iter().cloned().enumerate().any(|(ml_index72, ml_value72)| ml_callback72(ml_value72)) } || { let ml_values74 = { let ml_reference_value75 = parameters.borrow().clone(); ml_reference_value75 }; let ml_callback74 = { let size = size; std::rc::Rc::new(move |target: f64| -> bool { ((!(self::ml_math::is_integer(target)) || (target < (0f64))) || (target >= size)) }) }; ml_values74.iter().cloned().enumerate().any(|(ml_index74, ml_value74)| ml_callback74(ml_value74)) }) {
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))
                                        } else {
                                            {
                                                let at = 0f64;
                                                {
                                                    let source_ml_source2_s57 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop55(functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), bodies.clone(), size, at);
                                                    match source_ml_source2_s57.clone() {
                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Done => {
                                                            if ({ let ml_values76 = { let ml_reference_value77 = snapshots.borrow().clone(); ml_reference_value77 }; let ml_callback76 = { let size = size; std::rc::Rc::new(move |node_2: f64| -> bool { ((!(self::ml_math::is_integer(node_2)) || (node_2 < (0f64))) || (node_2 >= size)) }) }; ml_values76.iter().cloned().enumerate().any(|(ml_index76, ml_value76)| ml_callback76(ml_value76)) } || { let ml_values78 = { let ml_reference_value79 = callbacks.borrow().clone(); ml_reference_value79 }; let ml_callback78 = { let size = size; std::rc::Rc::new(move |node_2: f64| -> bool { ((!(self::ml_math::is_integer(node_2)) || (node_2 < (0f64))) || (node_2 >= size)) }) }; ml_values78.iter().cloned().enumerate().any(|(ml_index78, ml_value78)| ml_callback78(ml_value78)) }) {
                                                                std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))
                                                            } else {
                                                                {
                                                                    let facts = std::rc::Rc::new(std::cell::RefCell::new(vec![((4f64 * span) + main_)]));
                                                                    {
                                                                        let work = 0f64;
                                                                        {
                                                                            let settled = false;
                                                                            {
                                                                                let seeded = false;
                                                                                {
                                                                                    let pass = 0f64;
                                                                                    {
                                                                                        let source_ml_source2_s58 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop56(roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), bodies.clone(), main_, snapshots.clone(), callbacks.clone(), size, width, field_width, span, facts.clone(), work, settled, seeded, pass);
                                                                                        match source_ml_source2_s58.clone() {
                                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Done(facts_2, settled_2) => {
                                                                                                if !settled_2 {
                                                                                                    std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))
                                                                                                } else {
                                                                                                    {
                                                                                                        let accepted = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                                                        {
                                                                                                            let at_2 = 0f64;
                                                                                                            {
                                                                                                                let source_ml_source2_s59 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop57(roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), bodies.clone(), main_, snapshots.clone(), size, width, span, facts_2.clone(), accepted.clone(), at_2);
                                                                                                                match source_ml_source2_s59.clone() {
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Done(accepted_2) => {
                                                                                                                        {
                                                                                                                            let safe = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                                                                            {
                                                                                                                                let at_3 = 0f64;
                                                                                                                                {
                                                                                                                                    let safe_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop58(roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), bodies.clone(), main_, callbacks.clone(), width, span, facts_2.clone(), safe.clone(), at_3);
                                                                                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                        let source_ml_source2_concat_value60 = std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, { let ml_reference_array80 = accepted_2.clone(); let ml_reference_projection80 = ml_reference_array80.borrow().len() as f64; ml_reference_projection80 }]));
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_concat_value61 = accepted_2.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_concat_value62 = std::rc::Rc::new(std::cell::RefCell::new(vec![{ let ml_reference_array81 = safe_2.clone(); let ml_reference_projection81 = ml_reference_array81.borrow().len() as f64; ml_reference_projection81 }]));
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_concat_value63 = safe_2.clone();
                                                                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value82 = source_ml_source2_concat_value60.borrow().clone(); ml_reference_value82 }), { let ml_reference_value83 = source_ml_source2_concat_value61.borrow().clone(); ml_reference_value83 }), { let ml_reference_value84 = source_ml_source2_concat_value62.borrow().clone(); ml_reference_value84 }), { let ml_reference_value85 = source_ml_source2_concat_value63.borrow().clone(); ml_reference_value85 })
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }))
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(source_ml_source1_temporary_result58) => {
                                                                                                                        source_ml_source1_temporary_result58.clone()
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Return(source_ml_source1_temporary_result63) => {
                                                                                                source_ml_source1_temporary_result63.clone()
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
                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Return(source_ml_source1_temporary_result78) => {
                                                            source_ml_source1_temporary_result78.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Return(source_ml_source1_temporary_result78_2) => {
                                        source_ml_source1_temporary_result78_2.clone()
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

pub fn first_number(values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> f64 {
    if (({ let ml_reference_array86 = values.clone(); let ml_reference_projection86 = ml_reference_array86.borrow().len() as f64; ml_reference_projection86 }) == (0f64)) {
        (-1f64)
    } else {
        self::read_required_number_fact(values.clone(), 0f64)
    }
}

pub fn text_fact(values: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, index: f64) -> self::ml_string::Text {
    if ((index < (0f64)) || (index >= ({ let ml_reference_array87 = values.clone(); let ml_reference_projection87 = ml_reference_array87.borrow().len() as f64; ml_reference_projection87 }))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::read_required_text_fact(values.clone(), index)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodResultJoin2 {
    MlSource1TemporaryReadReferenceMethodResultJoin2Next,
    MlSource1TemporaryReadReferenceMethodResultJoin2Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodResultJoin1 {
    MlSource1TemporaryReadReferenceMethodResultJoin1Next,
    MlSource1TemporaryReadReferenceMethodResultJoin1Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodIdentityJoin5 {
    MlSource1TemporaryReadReferenceMethodIdentityJoin5Next,
    MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodIdentityJoin4 {
    MlSource1TemporaryReadReferenceMethodIdentityJoin4Next,
    MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodIdentityJoin3 {
    MlSource1TemporaryReadReferenceMethodIdentityJoin3Next,
    MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_render_reference_method_expression_loop6(mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut suffix: self::ml_string::Text, mut prefix: self::ml_string::Text, mut index: f64) -> self::ml_string::Text {
    loop {
        return if (index < ({ let ml_reference_array88 = arguments_.clone(); let ml_reference_projection88 = ml_reference_array88.borrow().len() as f64; ml_reference_projection88 })) {
            {
                let prefix_2 = prefix.concat(&((((((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(index)))).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&({
                    let source_ml_source2_array_read_value64 = arguments_.clone();
                    {
                        let source_ml_source2_array_read_index65 = index;
                        { let ml_reference_array89 = source_ml_source2_array_read_value64.clone(); let ml_reference_position89 = source_ml_source2_array_read_index65; let ml_reference_projection89 = self::ml_array::optional_at(&*ml_reference_array89.borrow(), self::ml_array::number_index(ml_reference_position89)); ml_reference_projection89 }
                    }
                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (prefix, index) = (prefix_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            prefix.clone()
        };
    }
}

pub fn source_ml_source1_temporary_render_reference_method_expression_loop7(mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut suffix: self::ml_string::Text, mut names: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    loop {
        return if (index < ({ let ml_reference_array90 = arguments_.clone(); let ml_reference_projection90 = ml_reference_array90.borrow().len() as f64; ml_reference_projection90 })) {
            {
                let names_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value91 = names.borrow().clone(); ml_reference_value91 }, vec![(((self::ml_string::Text::from_units(vec![109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(index)))])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (names, index) = (names_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            names.clone()
        };
    }
}

pub fn source_ml_source1_temporary_render_array_callback_call_loop8(mut operation: self::ml_string::Text, mut parameter_count: f64, mut suffix: self::ml_string::Text, mut clone_element: bool, mut arguments_text: self::ml_string::Text, mut position: f64, mut maximum: f64) -> self::ml_string::Text {
    loop {
        return if ((position < parameter_count) && (position < maximum)) {
            {
                let arguments_text_3 = if (position > (0f64)) {
                    {
                        let arguments_text_2 = arguments_text.concat(&self::ml_string::Text::from_units(vec![44, 32]));
                        arguments_text_2.clone()
                    }
                } else {
                    arguments_text.clone()
                };
                {
                    let arguments_text_4 = arguments_text_3.concat(&self::render_array_callback_argument(operation.clone(), position, suffix.clone(), clone_element));
                    {
                        let position_2 = (position + 1f64);
                        {
                            (arguments_text, position) = (arguments_text_4.clone(), position_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            arguments_text.clone()
        };
    }
}

pub fn source_ml_source1_temporary_read_array_state_method_suffix_loop9(mut text: self::ml_string::Text, mut suffix: f64) -> f64 {
    loop {
        return if (text.contains(&(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 111, 119, 110, 101, 114])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix)))) || text.contains(&((self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix)))).concat(&self::ml_string::Text::from_units(vec![95])))) {
            {
                let suffix_2 = (suffix + 1f64);
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        } else {
            suffix
        };
    }
}

pub fn source_ml_source1_temporary_render_array_state_method_expression_loop10(mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut types: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut suffix: self::ml_string::Text, mut output: self::ml_string::Text, mut index: f64) -> self::ml_string::Text {
    loop {
        return if (index < ({ let ml_reference_array92 = arguments_.clone(); let ml_reference_projection92 = ml_reference_array92.borrow().len() as f64; ml_reference_projection92 })) {
            {
                let output_2 = output.concat(&((((((((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(index)))).concat(&self::ml_string::Text::from_units(vec![58, 32]))).concat(&({
                    let source_ml_source2_array_read_value66 = types.clone();
                    {
                        let source_ml_source2_array_read_index67 = index;
                        { let ml_reference_array93 = source_ml_source2_array_read_value66.clone(); let ml_reference_position93 = source_ml_source2_array_read_index67; let ml_reference_projection93 = self::ml_array::optional_at(&*ml_reference_array93.borrow(), self::ml_array::number_index(ml_reference_position93)); ml_reference_projection93 }
                    }
                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&({
                    let source_ml_source2_array_read_value68 = arguments_.clone();
                    {
                        let source_ml_source2_array_read_index69 = index;
                        { let ml_reference_array94 = source_ml_source2_array_read_value68.clone(); let ml_reference_position94 = source_ml_source2_array_read_index69; let ml_reference_projection94 = self::ml_array::optional_at(&*ml_reference_array94.borrow(), self::ml_array::number_index(ml_reference_position94)); ml_reference_projection94 }
                    }
                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (output, index) = (output_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            output.clone()
        };
    }
}

pub fn source_ml_source1_temporary_render_array_state_method_expression_loop11(mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut suffix: self::ml_string::Text, mut names: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    loop {
        return if (index < ({ let ml_reference_array95 = arguments_.clone(); let ml_reference_projection95 = ml_reference_array95.borrow().len() as f64; ml_reference_projection95 })) {
            {
                let names_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value96 = names.borrow().clone(); ml_reference_value96 }, vec![(((self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(index)))])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (names, index) = (names_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            names.clone()
        };
    }
}

pub fn source_ml_source1_temporary_render_array_state_method_expression_loop12(mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut names: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut inserted: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    loop {
        return if (index < ({ let ml_reference_array97 = arguments_.clone(); let ml_reference_projection97 = ml_reference_array97.borrow().len() as f64; ml_reference_projection97 })) {
            {
                let inserted_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value98 = inserted.borrow().clone(); ml_reference_value98 }, vec![({
                    let source_ml_source2_array_read_value70 = names.clone();
                    {
                        let source_ml_source2_array_read_index71 = index;
                        { let ml_reference_array99 = source_ml_source2_array_read_value70.clone(); let ml_reference_position99 = source_ml_source2_array_read_index71; let ml_reference_projection99 = self::ml_array::optional_at(&*ml_reference_array99.borrow(), self::ml_array::number_index(ml_reference_position99)); ml_reference_projection99 }
                    }
                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (inserted, index) = (inserted_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            inserted.clone()
        };
    }
}

pub fn source_ml_source1_temporary_read_roots_loop14(mut tag: f64, mut node: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values13: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index13: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index13 < ({ let ml_reference_array100 = source_ml_source1_temporary_values13.clone(); let ml_reference_projection100 = self::ml::Big::from_u128(ml_reference_array100.borrow().len() as u128); ml_reference_projection100 })) {
            {
                let fact = {
                    let source_ml_source2_array_read_value72 = source_ml_source1_temporary_values13.clone();
                    {
                        let source_ml_source2_array_read_index73 = source_ml_source1_temporary_index13.clone();
                        { let ml_reference_array101 = source_ml_source2_array_read_value72.clone(); let ml_reference_position101 = source_ml_source2_array_read_index73.clone(); let ml_reference_projection101 = self::ml_array::at(&*ml_reference_array101.borrow(), ml_reference_position101.to_index()); ml_reference_projection101 }
                    }
                };
                if ((((fact / span)).floor()) == tag) {
                    {
                        let value = (fact % span);
                        if ((((value / width)).floor()) == node) {
                            {
                                let result_2 = self::add_value(result.clone(), ((value % width) - 1f64));
                                {
                                    let source_ml_source1_temporary_index13_2 = source_ml_source1_temporary_index13.add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index13) = (result_2.clone(), source_ml_source1_temporary_index13_2.clone());
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source1_temporary_index13_3 = source_ml_source1_temporary_index13.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index13 = source_ml_source1_temporary_index13_3.clone();
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index13_4 = source_ml_source1_temporary_index13.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index13 = source_ml_source1_temporary_index13_4.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_add_roots_loop17(mut present: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values15: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index15: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index15 < ({ let ml_reference_array102 = source_ml_source1_temporary_values15.clone(); let ml_reference_projection102 = self::ml::Big::from_u128(ml_reference_array102.borrow().len() as u128); ml_reference_projection102 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value74 = source_ml_source1_temporary_values15.clone();
                    {
                        let source_ml_source2_array_read_index75 = source_ml_source1_temporary_index15.clone();
                        { let ml_reference_array103 = source_ml_source2_array_read_value74.clone(); let ml_reference_position103 = source_ml_source2_array_read_index75.clone(); let ml_reference_projection103 = self::ml_array::at(&*ml_reference_array103.borrow(), ml_reference_position103.to_index()); ml_reference_projection103 }
                    }
                };
                {
                    let present_2 = self::add_value(present.clone(), root);
                    {
                        let source_ml_source1_temporary_index15_2 = source_ml_source1_temporary_index15.add(&self::ml::Big::from_i128(1));
                        {
                            (present, source_ml_source1_temporary_index15) = (present_2.clone(), source_ml_source1_temporary_index15_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            present.clone()
        };
    }
}

pub fn source_ml_source1_temporary_add_roots_loop18(mut tag: f64, mut node: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values16: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index16: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index16 < ({ let ml_reference_array104 = source_ml_source1_temporary_values16.clone(); let ml_reference_projection104 = self::ml::Big::from_u128(ml_reference_array104.borrow().len() as u128); ml_reference_projection104 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value76 = source_ml_source1_temporary_values16.clone();
                    {
                        let source_ml_source2_array_read_index77 = source_ml_source1_temporary_index16.clone();
                        { let ml_reference_array105 = source_ml_source2_array_read_value76.clone(); let ml_reference_position105 = source_ml_source2_array_read_index77.clone(); let ml_reference_projection105 = self::ml_array::at(&*ml_reference_array105.borrow(), ml_reference_position105.to_index()); ml_reference_projection105 }
                    }
                };
                {
                    let result_2 = self::add_fact(result.clone(), ((((tag * span) + (node * width)) + root) + 1f64));
                    {
                        let source_ml_source1_temporary_index16_2 = source_ml_source1_temporary_index16.add(&self::ml::Big::from_i128(1));
                        {
                            (result, source_ml_source1_temporary_index16) = (result_2.clone(), source_ml_source1_temporary_index16_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_edges_loop19(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64, mut label: self::ml_string::Text, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ((self::read_required_number_fact(starts.clone(), node) + self::read_required_number_fact(counts.clone(), node)))) {
            if ((self::text_fact(labels.clone(), at)) == label) {
                {
                    let result_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value78 = result.clone();
                        {
                            let source_ml_source2_concat_value79 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::read_required_number_fact(targets.clone(), at)]));
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value106 = source_ml_source2_concat_value78.borrow().clone(); ml_reference_value106 }), { let ml_reference_value107 = source_ml_source2_concat_value79.borrow().clone(); ml_reference_value107 })
                        }
                    }));
                    {
                        let at_2 = (at + 1f64);
                        {
                            (result, at) = (result_2.clone(), at_2);
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
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_read_field_loop23(mut field: f64, mut field_width: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut root: f64, mut source_ml_source1_temporary_values21: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index21: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index21 < ({ let ml_reference_array108 = source_ml_source1_temporary_values21.clone(); let ml_reference_projection108 = self::ml::Big::from_u128(ml_reference_array108.borrow().len() as u128); ml_reference_projection108 })) {
            {
                let fact = {
                    let source_ml_source2_array_read_value80 = source_ml_source1_temporary_values21.clone();
                    {
                        let source_ml_source2_array_read_index81 = source_ml_source1_temporary_index21.clone();
                        { let ml_reference_array109 = source_ml_source2_array_read_value80.clone(); let ml_reference_position109 = source_ml_source2_array_read_index81.clone(); let ml_reference_projection109 = self::ml_array::at(&*ml_reference_array109.borrow(), ml_reference_position109.to_index()); ml_reference_projection109 }
                    }
                };
                if ((((fact / span)).floor()) == (2f64)) {
                    {
                        let value = (fact % span);
                        {
                            let owner = ((value / (field_width * width))).floor();
                            {
                                let actual = (((value / width)).floor() % field_width);
                                if ((owner == root) && ((actual == (0f64)) || (actual == field))) {
                                    {
                                        let result_2 = self::add_value(result.clone(), ((value % width) - 1f64));
                                        {
                                            let source_ml_source1_temporary_index21_2 = source_ml_source1_temporary_index21.add(&self::ml::Big::from_i128(1));
                                            {
                                                (result, source_ml_source1_temporary_index21) = (result_2.clone(), source_ml_source1_temporary_index21_2.clone());
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let source_ml_source1_temporary_index21_3 = source_ml_source1_temporary_index21.add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index21 = source_ml_source1_temporary_index21_3.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index21_4 = source_ml_source1_temporary_index21.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index21 = source_ml_source1_temporary_index21_4.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_read_field_loop22(mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut field: f64, mut field_width: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values20: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index20: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index20 < ({ let ml_reference_array110 = source_ml_source1_temporary_values20.clone(); let ml_reference_projection110 = self::ml::Big::from_u128(ml_reference_array110.borrow().len() as u128); ml_reference_projection110 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value82 = source_ml_source1_temporary_values20.clone();
                    {
                        let source_ml_source2_array_read_index83 = source_ml_source1_temporary_index20.clone();
                        { let ml_reference_array111 = source_ml_source2_array_read_value82.clone(); let ml_reference_position111 = source_ml_source2_array_read_index83.clone(); let ml_reference_projection111 = self::ml_array::at(&*ml_reference_array111.borrow(), ml_reference_position111.to_index()); ml_reference_projection111 }
                    }
                };
                if (root < (0f64)) {
                    {
                        let result_2 = self::add_value(result.clone(), (-1f64));
                        {
                            let source_ml_source1_temporary_index20_2 = source_ml_source1_temporary_index20.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_source1_temporary_index20) = (result_2.clone(), source_ml_source1_temporary_index20_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_values21 = facts.clone();
                        {
                            let source_ml_source1_temporary_index21 = self::ml::Big::from_i128(0);
                            {
                                let result_3 = self::source_ml_source1_temporary_read_field_loop23(field, field_width, width, span, result.clone(), root, source_ml_source1_temporary_values21.clone(), source_ml_source1_temporary_index21.clone());
                                {
                                    let source_ml_source1_temporary_index20_3 = source_ml_source1_temporary_index20.add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index20) = (result_3.clone(), source_ml_source1_temporary_index20_3.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_add_field_loop27(mut field: f64, mut field_width: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut owner: f64, mut source_ml_source1_temporary_values25: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index25: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index25 < ({ let ml_reference_array112 = source_ml_source1_temporary_values25.clone(); let ml_reference_projection112 = self::ml::Big::from_u128(ml_reference_array112.borrow().len() as u128); ml_reference_projection112 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value84 = source_ml_source1_temporary_values25.clone();
                    {
                        let source_ml_source2_array_read_index85 = source_ml_source1_temporary_index25.clone();
                        { let ml_reference_array113 = source_ml_source2_array_read_value84.clone(); let ml_reference_position113 = source_ml_source2_array_read_index85.clone(); let ml_reference_projection113 = self::ml_array::at(&*ml_reference_array113.borrow(), ml_reference_position113.to_index()); ml_reference_projection113 }
                    }
                };
                {
                    let result_2 = self::add_fact(result.clone(), ((((2f64 * span) + (((owner * field_width) + field) * width)) + root) + 1f64));
                    {
                        let source_ml_source1_temporary_index25_2 = source_ml_source1_temporary_index25.add(&self::ml::Big::from_i128(1));
                        {
                            (result, source_ml_source1_temporary_index25) = (result_2.clone(), source_ml_source1_temporary_index25_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_add_field_loop26(mut field: f64, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut field_width: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values24: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index24: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index24 < ({ let ml_reference_array114 = source_ml_source1_temporary_values24.clone(); let ml_reference_projection114 = self::ml::Big::from_u128(ml_reference_array114.borrow().len() as u128); ml_reference_projection114 })) {
            {
                let owner = {
                    let source_ml_source2_array_read_value86 = source_ml_source1_temporary_values24.clone();
                    {
                        let source_ml_source2_array_read_index87 = source_ml_source1_temporary_index24.clone();
                        { let ml_reference_array115 = source_ml_source2_array_read_value86.clone(); let ml_reference_position115 = source_ml_source2_array_read_index87.clone(); let ml_reference_projection115 = self::ml_array::at(&*ml_reference_array115.borrow(), ml_reference_position115.to_index()); ml_reference_projection115 }
                    }
                };
                if (owner >= (0f64)) {
                    {
                        let source_ml_source1_temporary_values25 = roots.clone();
                        {
                            let source_ml_source1_temporary_index25 = self::ml::Big::from_i128(0);
                            {
                                let result_2 = self::source_ml_source1_temporary_add_field_loop27(field, field_width, width, span, result.clone(), owner, source_ml_source1_temporary_values25.clone(), source_ml_source1_temporary_index25.clone());
                                {
                                    let source_ml_source1_temporary_index24_2 = source_ml_source1_temporary_index24.add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index24) = (result_2.clone(), source_ml_source1_temporary_index24_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index24_3 = source_ml_source1_temporary_index24.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index24 = source_ml_source1_temporary_index24_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_copy_fields_loop31(mut destination: f64, mut field_width: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut root: f64, mut source_ml_source1_temporary_values29: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index29: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index29 < ({ let ml_reference_array116 = source_ml_source1_temporary_values29.clone(); let ml_reference_projection116 = self::ml::Big::from_u128(ml_reference_array116.borrow().len() as u128); ml_reference_projection116 })) {
            {
                let fact = {
                    let source_ml_source2_array_read_value88 = source_ml_source1_temporary_values29.clone();
                    {
                        let source_ml_source2_array_read_index89 = source_ml_source1_temporary_index29.clone();
                        { let ml_reference_array117 = source_ml_source2_array_read_value88.clone(); let ml_reference_position117 = source_ml_source2_array_read_index89.clone(); let ml_reference_projection117 = self::ml_array::at(&*ml_reference_array117.borrow(), ml_reference_position117.to_index()); ml_reference_projection117 }
                    }
                };
                if ((((fact / span)).floor()) == (2f64)) {
                    {
                        let value = (fact % span);
                        if ((((value / (field_width * width))).floor()) == root) {
                            {
                                let result_2 = self::add_fact(result.clone(), (((2f64 * span) + ((destination * field_width) * width)) + (value % (field_width * width))));
                                {
                                    let source_ml_source1_temporary_index29_2 = source_ml_source1_temporary_index29.add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index29) = (result_2.clone(), source_ml_source1_temporary_index29_2.clone());
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source1_temporary_index29_3 = source_ml_source1_temporary_index29.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index29 = source_ml_source1_temporary_index29_3.clone();
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index29_4 = source_ml_source1_temporary_index29.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index29 = source_ml_source1_temporary_index29_4.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_copy_fields_loop30(mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut destination: f64, mut field_width: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values28: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index28: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index28 < ({ let ml_reference_array118 = source_ml_source1_temporary_values28.clone(); let ml_reference_projection118 = self::ml::Big::from_u128(ml_reference_array118.borrow().len() as u128); ml_reference_projection118 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value90 = source_ml_source1_temporary_values28.clone();
                    {
                        let source_ml_source2_array_read_index91 = source_ml_source1_temporary_index28.clone();
                        { let ml_reference_array119 = source_ml_source2_array_read_value90.clone(); let ml_reference_position119 = source_ml_source2_array_read_index91.clone(); let ml_reference_projection119 = self::ml_array::at(&*ml_reference_array119.borrow(), ml_reference_position119.to_index()); ml_reference_projection119 }
                    }
                };
                if (root < (0f64)) {
                    {
                        let result_2 = self::add_field(result.clone(), std::rc::Rc::new(std::cell::RefCell::new(vec![destination])), 0f64, std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])), field_width, width, span);
                        {
                            let source_ml_source1_temporary_index28_2 = source_ml_source1_temporary_index28.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_source1_temporary_index28) = (result_2.clone(), source_ml_source1_temporary_index28_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_values29 = facts.clone();
                        {
                            let source_ml_source1_temporary_index29 = self::ml::Big::from_i128(0);
                            {
                                let result_3 = self::source_ml_source1_temporary_copy_fields_loop31(destination, field_width, width, span, result.clone(), root, source_ml_source1_temporary_values29.clone(), source_ml_source1_temporary_index29.clone());
                                {
                                    let source_ml_source1_temporary_index28_3 = source_ml_source1_temporary_index28.add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index28) = (result_3.clone(), source_ml_source1_temporary_index28_3.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReachableLoop35Result {
    MlSource1TemporaryReachableLoop35Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReachableLoop36Result {
    MlSource1TemporaryReachableLoop36Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bool),
}

pub fn source_ml_source1_temporary_reachable_loop36(mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut progress: bool, mut source_ml_source1_temporary_values33: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index33: self::ml::Big) -> self::MlSource1TemporaryReachableLoop36Result {
    loop {
        return if (source_ml_source1_temporary_index33 < ({ let ml_reference_array120 = source_ml_source1_temporary_values33.clone(); let ml_reference_projection120 = self::ml::Big::from_u128(ml_reference_array120.borrow().len() as u128); ml_reference_projection120 })) {
            {
                let target = {
                    let source_ml_source2_array_read_value92 = source_ml_source1_temporary_values33.clone();
                    {
                        let source_ml_source2_array_read_index93 = source_ml_source1_temporary_index33.clone();
                        { let ml_reference_array121 = source_ml_source2_array_read_value92.clone(); let ml_reference_position121 = source_ml_source2_array_read_index93.clone(); let ml_reference_projection121 = self::ml_array::at(&*ml_reference_array121.borrow(), ml_reference_position121.to_index()); ml_reference_projection121 }
                    }
                };
                if !(self::ml_array::includes(&{ let ml_reference_value122 = result.borrow().clone(); ml_reference_value122 }, &target, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                    {
                        let result_2 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_concat_value94 = result.clone();
                            {
                                let source_ml_source2_concat_value95 = std::rc::Rc::new(std::cell::RefCell::new(vec![target]));
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value123 = source_ml_source2_concat_value94.borrow().clone(); ml_reference_value123 }), { let ml_reference_value124 = source_ml_source2_concat_value95.borrow().clone(); ml_reference_value124 })
                            }
                        }));
                        {
                            let progress_2 = true;
                            {
                                let source_ml_source1_temporary_index33_2 = source_ml_source1_temporary_index33.add(&self::ml::Big::from_i128(1));
                                {
                                    (result, progress, source_ml_source1_temporary_index33) = (result_2.clone(), progress_2, source_ml_source1_temporary_index33_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index33_3 = source_ml_source1_temporary_index33.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index33 = source_ml_source1_temporary_index33_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReachableLoop36Result::MlSource1TemporaryReachableLoop36Done(result.clone(), progress)
        };
    }
}

pub fn source_ml_source1_temporary_reachable_loop35(mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut progress: bool, mut source_ml_source1_temporary_values32: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index32: self::ml::Big) -> self::MlSource1TemporaryReachableLoop35Result {
    loop {
        return if (source_ml_source1_temporary_index32 < ({ let ml_reference_array125 = source_ml_source1_temporary_values32.clone(); let ml_reference_projection125 = self::ml::Big::from_u128(ml_reference_array125.borrow().len() as u128); ml_reference_projection125 })) {
            {
                let owner = {
                    let source_ml_source2_array_read_value96 = source_ml_source1_temporary_values32.clone();
                    {
                        let source_ml_source2_array_read_index97 = source_ml_source1_temporary_index32.clone();
                        { let ml_reference_array126 = source_ml_source2_array_read_value96.clone(); let ml_reference_position126 = source_ml_source2_array_read_index97.clone(); let ml_reference_projection126 = self::ml_array::at(&*ml_reference_array126.borrow(), ml_reference_position126.to_index()); ml_reference_projection126 }
                    }
                };
                {
                    let source_ml_source1_temporary_values33 = self::called(facts.clone(), owner, width, span);
                    {
                        let source_ml_source1_temporary_index33 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s98 = self::source_ml_source1_temporary_reachable_loop36(result.clone(), progress, source_ml_source1_temporary_values33.clone(), source_ml_source1_temporary_index33.clone());
                            match source_ml_source2_s98.clone() {
                                self::MlSource1TemporaryReachableLoop36Result::MlSource1TemporaryReachableLoop36Done(result_2, progress_2) => {
                                    {
                                        let source_ml_source1_temporary_index32_2 = source_ml_source1_temporary_index32.add(&self::ml::Big::from_i128(1));
                                        {
                                            (result, progress, source_ml_source1_temporary_index32) = (result_2.clone(), progress_2, source_ml_source1_temporary_index32_2.clone());
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
            self::MlSource1TemporaryReachableLoop35Result::MlSource1TemporaryReachableLoop35Done(result.clone(), progress)
        };
    }
}

pub fn source_ml_source1_temporary_reachable_loop34(mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut progress: bool) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if progress {
            {
                let progress_2 = false;
                {
                    let source_ml_source1_temporary_values32 = result.clone();
                    {
                        let source_ml_source1_temporary_index32 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s99 = self::source_ml_source1_temporary_reachable_loop35(facts.clone(), width, span, result.clone(), progress_2, source_ml_source1_temporary_values32.clone(), source_ml_source1_temporary_index32.clone());
                            match source_ml_source2_s99.clone() {
                                self::MlSource1TemporaryReachableLoop35Result::MlSource1TemporaryReachableLoop35Done(result_2, progress_3) => {
                                    {
                                        (result, progress) = (result_2.clone(), progress_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptsEffectsLoop39Result {
    MlSource1TemporaryAcceptsEffectsLoop39Done,
    MlSource1TemporaryAcceptsEffectsLoop39Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptsEffectsJoin40 {
    MlSource1TemporaryAcceptsEffectsJoin40Next,
    MlSource1TemporaryAcceptsEffectsJoin40Return(bool),
}

pub fn source_ml_source1_temporary_accepts_effects_loop39(mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source: f64, mut width: f64, mut span: f64, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut reached: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64) -> self::MlSource1TemporaryAcceptsEffectsLoop39Result {
    loop {
        return if (node < ({ let ml_reference_array127 = roles.clone(); let ml_reference_projection127 = ml_reference_array127.borrow().len() as f64; ml_reference_projection127 })) {
            if self::ml_array::includes(&{ let ml_reference_value128 = reached.borrow().clone(); ml_reference_value128 }, &self::read_required_number_fact(owners.clone(), node), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) {
                if (((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))) || ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![99, 101, 108, 108, 32, 119, 114, 105, 116, 101])))) {
                    self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                } else {
                    {
                        let source_ml_source2_s100 = if (((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) || ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])))) {
                            {
                                let actual = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![99, 97, 108, 108, 101, 101]));
                                {
                                    let callees = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let callees_4 = if ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                            {
                                                let callees_2 = actual.clone();
                                                callees_2.clone()
                                            }
                                        } else {
                                            if (({ let ml_reference_array129 = actual.clone(); let ml_reference_projection129 = ml_reference_array129.borrow().len() as f64; ml_reference_projection129 }) != (0f64)) {
                                                {
                                                    let callees_3 = self::read_roots(facts.clone(), 1f64, self::read_required_number_fact(actual.clone(), 0f64), width, span);
                                                    callees_3.clone()
                                                }
                                            } else {
                                                callees.clone()
                                            }
                                        };
                                        if ((({ let ml_reference_array130 = callees_4.clone(); let ml_reference_projection130 = ml_reference_array130.borrow().len() as f64; ml_reference_projection130 }) == (0f64)) || { let ml_values131 = { let ml_reference_value132 = callees_4.borrow().clone(); ml_reference_value132 }; let ml_callback131 = { let functions = functions.clone(); std::rc::Rc::new(move |target: f64| -> bool { !(self::ml_array::includes(&{ let ml_reference_value133 = functions.borrow().clone(); ml_reference_value133 }, &target, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values131.iter().cloned().enumerate().any(|(ml_index131, ml_value131)| ml_callback131(ml_value131)) }) {
                                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Return(false)
                                        } else {
                                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Next
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Next
                        };
                        match source_ml_source2_s100.clone() {
                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Next => {
                                if ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) {
                                    {
                                        let object = self::first_number(self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])));
                                        {
                                            let effect = self::read_disjoint_collection_iteration_method_effect(self::text_fact(kinds.clone(), object), self::text_fact(operations.clone(), node));
                                            if (effect == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))) {
                                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                                            } else {
                                                if ((effect == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && !(self::ml_array::includes(&{ let ml_reference_value134 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![99, 101, 108, 108])]))).borrow().clone(); ml_reference_value134 }, &self::text_fact(kinds.clone(), object), 0f64, false, |a, b| a == b))) {
                                                    self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                                                } else {
                                                    if ((effect == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && (((self::text_fact(kinds.clone(), object)) == (self::text_fact(kinds.clone(), source))) && !(self::separate_roots(roots.clone(), self::read_roots(facts.clone(), 1f64, object, width, span))))) {
                                                        self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                                                    } else {
                                                        {
                                                            let arguments_ = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116]));
                                                            if (self::read_disjoint_collection_iteration_callback(self::text_fact(operations.clone(), node)) && (({ let ml_reference_array135 = arguments_.clone(); let ml_reference_projection135 = ml_reference_array135.borrow().len() as f64; ml_reference_projection135 }) != (0f64))) {
                                                                {
                                                                    let callees_5 = self::read_roots(facts.clone(), 1f64, self::read_required_number_fact(arguments_.clone(), 0f64), width, span);
                                                                    if ((({ let ml_reference_array136 = callees_5.clone(); let ml_reference_projection136 = ml_reference_array136.borrow().len() as f64; ml_reference_projection136 }) == (0f64)) || { let ml_values137 = { let ml_reference_value138 = callees_5.borrow().clone(); ml_reference_value138 }; let ml_callback137 = { let functions = functions.clone(); std::rc::Rc::new(move |target: f64| -> bool { !(self::ml_array::includes(&{ let ml_reference_value139 = functions.borrow().clone(); ml_reference_value139 }, &target, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values137.iter().cloned().enumerate().any(|(ml_index137, ml_value137)| ml_callback137(ml_value137)) }) {
                                                                        self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                                                                    } else {
                                                                        {
                                                                            let node_2 = (node + 1f64);
                                                                            {
                                                                                node = node_2;
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    let node_3 = (node + 1f64);
                                                                    {
                                                                        node = node_3;
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
                                    {
                                        let node_4 = (node + 1f64);
                                        {
                                            node = node_4;
                                            continue;
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Return(source_ml_source1_temporary_result40) => {
                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(source_ml_source1_temporary_result40)
                            }
                        }
                    }
                }
            } else {
                {
                    let node_5 = (node + 1f64);
                    {
                        node = node_5;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptsEffectsLoop38Result {
    MlSource1TemporaryAcceptsEffectsLoop38Done,
    MlSource1TemporaryAcceptsEffectsLoop38Return(bool),
}

pub fn source_ml_source1_temporary_accepts_effects_loop38(mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut main_: f64, mut source_ml_source1_temporary_values37: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index37: self::ml::Big) -> self::MlSource1TemporaryAcceptsEffectsLoop38Result {
    loop {
        return if (source_ml_source1_temporary_index37 < ({ let ml_reference_array140 = source_ml_source1_temporary_values37.clone(); let ml_reference_projection140 = self::ml::Big::from_u128(ml_reference_array140.borrow().len() as u128); ml_reference_projection140 })) {
            {
                let scope = {
                    let source_ml_source2_array_read_value101 = source_ml_source1_temporary_values37.clone();
                    {
                        let source_ml_source2_array_read_index102 = source_ml_source1_temporary_index37.clone();
                        { let ml_reference_array141 = source_ml_source2_array_read_value101.clone(); let ml_reference_position141 = source_ml_source2_array_read_index102.clone(); let ml_reference_projection141 = self::ml_array::at(&*ml_reference_array141.borrow(), ml_reference_position141.to_index()); ml_reference_projection141 }
                    }
                };
                {
                    let position = self::ml_array::index_of(&{ let ml_reference_value142 = functions.borrow().clone(); ml_reference_value142 }, &scope, 0f64, false, |a, b| a == b);
                    if ((position < (0f64)) || (((self::read_required_number_fact(bodies.clone(), position)) < (0f64)) && (scope != main_))) {
                        self::MlSource1TemporaryAcceptsEffectsLoop38Result::MlSource1TemporaryAcceptsEffectsLoop38Return(false)
                    } else {
                        {
                            let source_ml_source1_temporary_index37_2 = source_ml_source1_temporary_index37.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index37 = source_ml_source1_temporary_index37_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptsEffectsLoop38Result::MlSource1TemporaryAcceptsEffectsLoop38Done
        };
    }
}

pub fn source_ml_source1_temporary_bind_calls_loop45(mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values42: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index42: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index42 < ({ let ml_reference_array143 = source_ml_source1_temporary_values42.clone(); let ml_reference_projection143 = self::ml::Big::from_u128(ml_reference_array143.borrow().len() as u128); ml_reference_projection143 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value103 = source_ml_source1_temporary_values42.clone();
                    {
                        let source_ml_source2_array_read_index104 = source_ml_source1_temporary_index42.clone();
                        { let ml_reference_array144 = source_ml_source2_array_read_value103.clone(); let ml_reference_position144 = source_ml_source2_array_read_index104.clone(); let ml_reference_projection144 = self::ml_array::at(&*ml_reference_array144.borrow(), ml_reference_position144.to_index()); ml_reference_projection144 }
                    }
                };
                {
                    let roots_2 = self::add_value(roots.clone(), root);
                    {
                        let source_ml_source1_temporary_index42_2 = source_ml_source1_temporary_index42.add(&self::ml::Big::from_i128(1));
                        {
                            (roots, source_ml_source1_temporary_index42) = (roots_2.clone(), source_ml_source1_temporary_index42_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            roots.clone()
        };
    }
}

pub fn source_ml_source1_temporary_bind_calls_loop44(mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut callback: self::ml_string::Text, mut receiver: f64, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut callee: f64, mut position: f64, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < (self::read_required_number_fact(parameter_counts.clone(), position))) {
            {
                let parameter = self::read_required_number_fact(parameters.clone(), (self::read_required_number_fact(parameter_starts.clone(), position) + at));
                {
                    let roots = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let roots_7 = if (callback == (self::ml_string::Text::from_units(vec![]))) {
                            {
                                let roots_2 = if (at < ({ let ml_reference_array145 = arguments_.clone(); let ml_reference_projection145 = ml_reference_array145.borrow().len() as f64; ml_reference_projection145 })) {
                                    self::read_roots(result.clone(), 1f64, self::read_required_number_fact(arguments_.clone(), at), width, span)
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                };
                                roots_2.clone()
                            }
                        } else {
                            if self::read_disjoint_collection_iteration_identity_kind(self::text_fact(kinds.clone(), parameter)) {
                                if ((callback == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) && (at == (0f64))) {
                                    {
                                        let roots_3 = if (({ let ml_reference_array146 = arguments_.clone(); let ml_reference_projection146 = ml_reference_array146.borrow().len() as f64; ml_reference_projection146 }) > (1f64)) {
                                            self::read_roots(result.clone(), 1f64, self::read_required_number_fact(arguments_.clone(), 1f64), width, span)
                                        } else {
                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                        };
                                        {
                                            let source_ml_source1_temporary_values42 = self::read_roots(result.clone(), 5f64, callee, width, span);
                                            {
                                                let source_ml_source1_temporary_index42 = self::ml::Big::from_i128(0);
                                                {
                                                    let roots_4 = self::source_ml_source1_temporary_bind_calls_loop45(roots_3.clone(), source_ml_source1_temporary_values42.clone(), source_ml_source1_temporary_index42.clone());
                                                    roots_4.clone()
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if (at == (if (callback == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
                                        3f64
                                    } else {
                                        2f64
                                    })) {
                                        {
                                            let roots_5 = self::read_roots(result.clone(), 1f64, receiver, width, span);
                                            roots_5.clone()
                                        }
                                    } else {
                                        {
                                            let roots_6 = std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]));
                                            roots_6.clone()
                                        }
                                    }
                                }
                            } else {
                                roots.clone()
                            }
                        };
                        {
                            let result_2 = self::add_roots(result.clone(), 1f64, parameter, roots_7.clone(), width, span);
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2.clone(), at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_bind_calls_loop43(mut owner: f64, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut callback: self::ml_string::Text, mut receiver: f64, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut main_: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values41: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index41: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index41 < ({ let ml_reference_array147 = source_ml_source1_temporary_values41.clone(); let ml_reference_projection147 = self::ml::Big::from_u128(ml_reference_array147.borrow().len() as u128); ml_reference_projection147 })) {
            {
                let callee = {
                    let source_ml_source2_array_read_value105 = source_ml_source1_temporary_values41.clone();
                    {
                        let source_ml_source2_array_read_index106 = source_ml_source1_temporary_index41.clone();
                        { let ml_reference_array148 = source_ml_source2_array_read_value105.clone(); let ml_reference_position148 = source_ml_source2_array_read_index106.clone(); let ml_reference_projection148 = self::ml_array::at(&*ml_reference_array148.borrow(), ml_reference_position148.to_index()); ml_reference_projection148 }
                    }
                };
                {
                    let position = self::ml_array::index_of(&{ let ml_reference_value149 = functions.borrow().clone(); ml_reference_value149 }, &callee, 0f64, false, |a, b| a == b);
                    if (position < (0f64)) {
                        {
                            let source_ml_source1_temporary_index41_2 = source_ml_source1_temporary_index41.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index41 = source_ml_source1_temporary_index41_2.clone();
                                continue;
                            }
                        }
                    } else {
                        {
                            let result_2 = self::add_fact(result.clone(), ((4f64 * span) + callee));
                            {
                                let result_3 = self::add_roots(result_2.clone(), 3f64, owner, std::rc::Rc::new(std::cell::RefCell::new(vec![callee])), width, span);
                                {
                                    let result_5 = if (((self::read_required_number_fact(bodies.clone(), position)) < (0f64)) && (callee != main_)) {
                                        {
                                            let result_4 = self::add_roots(result_3.clone(), 5f64, callee, std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])), width, span);
                                            result_4.clone()
                                        }
                                    } else {
                                        result_3.clone()
                                    };
                                    {
                                        let at = 0f64;
                                        {
                                            let result_6 = self::source_ml_source1_temporary_bind_calls_loop44(arguments_.clone(), callback.clone(), receiver, kinds.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), width, span, result_5.clone(), callee, position, at);
                                            {
                                                let source_ml_source1_temporary_index41_3 = source_ml_source1_temporary_index41.add(&self::ml::Big::from_i128(1));
                                                {
                                                    (result, source_ml_source1_temporary_index41) = (result_6.clone(), source_ml_source1_temporary_index41_3.clone());
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
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop58(mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut main_: f64, mut callbacks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut safe: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array150 = callbacks.clone(); let ml_reference_projection150 = ml_reference_array150.borrow().len() as f64; ml_reference_projection150 })) {
            {
                let node = self::read_required_number_fact(callbacks.clone(), at);
                {
                    let source = self::read_required_number_fact(callbacks.clone(), (at + 1f64));
                    {
                        let callback = self::first_number(self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116])));
                        if ((callback < (0f64)) || !(self::accept_disjoint_collection_iteration_callback_head(self::text_fact(roles.clone(), callback)))) {
                            {
                                let at_2 = (at + 2f64);
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        } else {
                            {
                                let scopes = self::read_roots(facts.clone(), 1f64, callback, width, span);
                                if ((({ let ml_reference_array151 = scopes.clone(); let ml_reference_projection151 = ml_reference_array151.borrow().len() as f64; ml_reference_projection151 }) != (0f64)) && self::accepts_effects(facts.clone(), roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), bodies.clone(), scopes.clone(), source, false, main_, width, span)) {
                                    {
                                        let safe_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value107 = safe.clone();
                                            {
                                                let source_ml_source2_concat_value108 = std::rc::Rc::new(std::cell::RefCell::new(vec![node]));
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value152 = source_ml_source2_concat_value107.borrow().clone(); ml_reference_value152 }), { let ml_reference_value153 = source_ml_source2_concat_value108.borrow().clone(); ml_reference_value153 })
                                            }
                                        }));
                                        {
                                            let at_3 = (at + 2f64);
                                            {
                                                (safe, at) = (safe_2.clone(), at_3);
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at_4 = (at + 2f64);
                                        {
                                            at = at_4;
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
            safe.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop57Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop59(mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut snapshot: f64, mut aliases: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (node < size) {
            if ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))) {
                {
                    let child = self::first_number(self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                    if ((child >= (0f64)) && self::ml_array::includes(&{ let ml_reference_value154 = (self::read_roots(facts.clone(), 1f64, child, width, span)).borrow().clone(); ml_reference_value154 }, &snapshot, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                        {
                            let aliases_2 = self::add_value(aliases.clone(), node);
                            {
                                let node_2 = (node + 1f64);
                                {
                                    (aliases, node) = (aliases_2.clone(), node_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let node_3 = (node + 1f64);
                            {
                                node = node_3;
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let node_4 = (node + 1f64);
                    {
                        node = node_4;
                        continue;
                    }
                }
            }
        } else {
            aliases.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop62(mut aliases: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut passes_alias: bool, mut source_ml_source1_temporary_values52: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index52: self::ml::Big) -> bool {
    loop {
        return if (source_ml_source1_temporary_index52 < ({ let ml_reference_array155 = source_ml_source1_temporary_values52.clone(); let ml_reference_projection155 = self::ml::Big::from_u128(ml_reference_array155.borrow().len() as u128); ml_reference_projection155 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value109 = source_ml_source1_temporary_values52.clone();
                    {
                        let source_ml_source2_array_read_index110 = source_ml_source1_temporary_index52.clone();
                        { let ml_reference_array156 = source_ml_source2_array_read_value109.clone(); let ml_reference_position156 = source_ml_source2_array_read_index110.clone(); let ml_reference_projection156 = self::ml_array::at(&*ml_reference_array156.borrow(), ml_reference_position156.to_index()); ml_reference_projection156 }
                    }
                };
                if self::ml_array::includes(&{ let ml_reference_value157 = aliases.borrow().clone(); ml_reference_value157 }, &root, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) {
                    {
                        let passes_alias_2 = true;
                        passes_alias_2
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index52_2 = source_ml_source1_temporary_index52.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index52 = source_ml_source1_temporary_index52_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            passes_alias
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop61(mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut aliases: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut passes_alias: bool, mut source_ml_source1_temporary_values51: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index51: self::ml::Big) -> bool {
    loop {
        return if (source_ml_source1_temporary_index51 < ({ let ml_reference_array158 = source_ml_source1_temporary_values51.clone(); let ml_reference_projection158 = self::ml::Big::from_u128(ml_reference_array158.borrow().len() as u128); ml_reference_projection158 })) {
            {
                let argument = {
                    let source_ml_source2_array_read_value111 = source_ml_source1_temporary_values51.clone();
                    {
                        let source_ml_source2_array_read_index112 = source_ml_source1_temporary_index51.clone();
                        { let ml_reference_array159 = source_ml_source2_array_read_value111.clone(); let ml_reference_position159 = source_ml_source2_array_read_index112.clone(); let ml_reference_projection159 = self::ml_array::at(&*ml_reference_array159.borrow(), ml_reference_position159.to_index()); ml_reference_projection159 }
                    }
                };
                {
                    let source_ml_source1_temporary_values52 = self::read_roots(facts.clone(), 1f64, argument, width, span);
                    {
                        let source_ml_source1_temporary_index52 = self::ml::Big::from_i128(0);
                        {
                            let passes_alias_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop62(aliases.clone(), passes_alias, source_ml_source1_temporary_values52.clone(), source_ml_source1_temporary_index52.clone());
                            if passes_alias_2 {
                                passes_alias_2
                            } else {
                                {
                                    let source_ml_source1_temporary_index51_2 = source_ml_source1_temporary_index51.add(&self::ml::Big::from_i128(1));
                                    {
                                        (passes_alias, source_ml_source1_temporary_index51) = (passes_alias_2, source_ml_source1_temporary_index51_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            passes_alias
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop63(mut scopes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values53: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index53: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index53 < ({ let ml_reference_array160 = source_ml_source1_temporary_values53.clone(); let ml_reference_projection160 = self::ml::Big::from_u128(ml_reference_array160.borrow().len() as u128); ml_reference_projection160 })) {
            {
                let callee = {
                    let source_ml_source2_array_read_value113 = source_ml_source1_temporary_values53.clone();
                    {
                        let source_ml_source2_array_read_index114 = source_ml_source1_temporary_index53.clone();
                        { let ml_reference_array161 = source_ml_source2_array_read_value113.clone(); let ml_reference_position161 = source_ml_source2_array_read_index114.clone(); let ml_reference_projection161 = self::ml_array::at(&*ml_reference_array161.borrow(), ml_reference_position161.to_index()); ml_reference_projection161 }
                    }
                };
                {
                    let scopes_2 = self::add_value(scopes.clone(), callee);
                    {
                        let source_ml_source1_temporary_index53_2 = source_ml_source1_temporary_index53.add(&self::ml::Big::from_i128(1));
                        {
                            (scopes, source_ml_source1_temporary_index53) = (scopes_2.clone(), source_ml_source1_temporary_index53_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            scopes.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop60(mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut aliases: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut scopes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (node < size) {
            if (((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) || ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])))) {
                {
                    let actual = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116]));
                    {
                        let passes_alias = false;
                        {
                            let source_ml_source1_temporary_values51 = actual.clone();
                            {
                                let source_ml_source1_temporary_index51 = self::ml::Big::from_i128(0);
                                {
                                    let passes_alias_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop61(width, span, facts.clone(), aliases.clone(), passes_alias, source_ml_source1_temporary_values51.clone(), source_ml_source1_temporary_index51.clone());
                                    if passes_alias_2 {
                                        {
                                            let targets_ = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![99, 97, 108, 108, 101, 101]));
                                            {
                                                let callees = if ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                    targets_.clone()
                                                } else {
                                                    if { let ml_value = { let ml_reference_array163 = targets_.clone(); let ml_reference_projection163 = ml_reference_array163.borrow().len() as f64; ml_reference_projection163 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                        self::read_roots(facts.clone(), 1f64, self::read_required_number_fact(targets_.clone(), 0f64), width, span)
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_temporary_values53 = callees.clone();
                                                    {
                                                        let source_ml_source1_temporary_index53 = self::ml::Big::from_i128(0);
                                                        {
                                                            let scopes_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop63(scopes.clone(), source_ml_source1_temporary_values53.clone(), source_ml_source1_temporary_index53.clone());
                                                            {
                                                                let node_2 = (node + 1f64);
                                                                {
                                                                    (scopes, node) = (scopes_2.clone(), node_2);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let node_3 = (node + 1f64);
                                            {
                                                node = node_3;
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
                {
                    let node_4 = (node + 1f64);
                    {
                        node = node_4;
                        continue;
                    }
                }
            }
        } else {
            scopes.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop57(mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut main_: f64, mut snapshots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut accepted: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result {
    loop {
        return if (at < ({ let ml_reference_array164 = snapshots.clone(); let ml_reference_projection164 = ml_reference_array164.borrow().len() as f64; ml_reference_projection164 })) {
            {
                let snapshot = self::read_required_number_fact(snapshots.clone(), at);
                {
                    let source = self::read_required_number_fact(snapshots.clone(), (at + 1f64));
                    {
                        let aliases = std::rc::Rc::new(std::cell::RefCell::new(vec![snapshot]));
                        {
                            let scopes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                            if ((self::text_fact(roles.clone(), snapshot)) == (self::ml_string::Text::from_units(vec![101, 97, 103, 101, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116]))) {
                                if !(self::accepts_effects(facts.clone(), roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), bodies.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())), source, true, main_, width, span)) {
                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, snapshot])))
                                } else {
                                    {
                                        let accepted_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value115 = accepted.clone();
                                            {
                                                let source_ml_source2_concat_value116 = std::rc::Rc::new(std::cell::RefCell::new(vec![snapshot]));
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value165 = source_ml_source2_concat_value115.borrow().clone(); ml_reference_value165 }), { let ml_reference_value166 = source_ml_source2_concat_value116.borrow().clone(); ml_reference_value166 })
                                            }
                                        }));
                                        {
                                            let at_2 = (at + 2f64);
                                            {
                                                (accepted, at) = (accepted_2.clone(), at_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let node = 0f64;
                                    {
                                        let aliases_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop59(roles.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), size, width, span, facts.clone(), snapshot, aliases.clone(), node);
                                        {
                                            let node_2 = 0f64;
                                            {
                                                let scopes_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop60(roles.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), size, width, span, facts.clone(), aliases_2.clone(), scopes.clone(), node_2);
                                                if ((({ let ml_reference_array167 = scopes_2.clone(); let ml_reference_projection167 = ml_reference_array167.borrow().len() as f64; ml_reference_projection167 }) == (0f64)) || !(self::accepts_effects(facts.clone(), roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), bodies.clone(), scopes_2.clone(), source, true, main_, width, span))) {
                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, snapshot])))
                                                } else {
                                                    {
                                                        let accepted_3 = std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_concat_value117 = accepted.clone();
                                                            {
                                                                let source_ml_source2_concat_value118 = std::rc::Rc::new(std::cell::RefCell::new(vec![snapshot]));
                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value168 = source_ml_source2_concat_value117.borrow().clone(); ml_reference_value168 }), { let ml_reference_value169 = source_ml_source2_concat_value118.borrow().clone(); ml_reference_value169 })
                                                            }
                                                        }));
                                                        {
                                                            let at_3 = (at + 2f64);
                                                            {
                                                                (accepted, at) = (accepted_3.clone(), at_3);
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
        } else {
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Done(accepted.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop56Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bool),
    MlSource1TemporaryProveDisjointCollectionIterationsLoop56Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop65(mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array170 = functions.clone(); let ml_reference_projection170 = ml_reference_array170.borrow().len() as f64; ml_reference_projection170 })) {
            if (((self::read_required_number_fact(bodies.clone(), at)) >= (0f64)) && self::ml_array::includes(&{ let ml_reference_value171 = facts.borrow().clone(); ml_reference_value171 }, &((4f64 * span) + self::read_required_number_fact(functions.clone(), at)), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                {
                    let facts_2 = self::add_roots(facts.clone(), 5f64, self::read_required_number_fact(functions.clone(), at), self::read_roots(facts.clone(), 1f64, self::read_required_number_fact(bodies.clone(), at), width, span), width, span);
                    {
                        let at_2 = (at + 1f64);
                        {
                            (facts, at) = (facts_2.clone(), at_2);
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
            facts.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin66 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin66Next(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bool),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop69(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut position: f64, mut parameter: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (parameter < ((self::read_required_number_fact(parameter_starts.clone(), position) + self::read_required_number_fact(parameter_counts.clone(), position)))) {
            if self::read_disjoint_collection_iteration_identity_kind(self::text_fact(kinds.clone(), self::read_required_number_fact(parameters.clone(), parameter))) {
                {
                    let facts_2 = self::add_roots(facts.clone(), 1f64, self::read_required_number_fact(parameters.clone(), parameter), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])), width, span);
                    {
                        let parameter_2 = (parameter + 1f64);
                        {
                            (facts, parameter) = (facts_2.clone(), parameter_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let parameter_3 = (parameter + 1f64);
                    {
                        parameter = parameter_3;
                        continue;
                    }
                }
            }
        } else {
            facts.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop68(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut list: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array172 = list.clone(); let ml_reference_projection172 = ml_reference_array172.borrow().len() as f64; ml_reference_projection172 })) {
            {
                let owner = self::read_required_number_fact(owners.clone(), self::read_required_number_fact(list.clone(), at));
                if ((owner >= (0f64)) && !(self::ml_array::includes(&{ let ml_reference_value173 = facts.borrow().clone(); ml_reference_value173 }, &((4f64 * span) + owner), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) {
                    {
                        let facts_2 = self::add_fact(facts.clone(), ((4f64 * span) + owner));
                        {
                            let position = self::ml_array::index_of(&{ let ml_reference_value174 = functions.borrow().clone(); ml_reference_value174 }, &owner, 0f64, false, |a, b| a == b);
                            if (position >= (0f64)) {
                                {
                                    let parameter = self::read_required_number_fact(parameter_starts.clone(), position);
                                    {
                                        let facts_3 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop69(kinds.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), width, span, facts_2.clone(), position, parameter);
                                        {
                                            let at_2 = (at + 2f64);
                                            {
                                                (facts, at) = (facts_3.clone(), at_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at_3 = (at + 2f64);
                                    {
                                        (facts, at) = (facts_2.clone(), at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let at_4 = (at + 2f64);
                        {
                            at = at_4;
                            continue;
                        }
                    }
                }
            }
        } else {
            facts.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop67(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values50: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>>, mut source_ml_source1_temporary_index50: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index50 < ({ let ml_reference_array175 = source_ml_source1_temporary_values50.clone(); let ml_reference_projection175 = self::ml::Big::from_u128(ml_reference_array175.borrow().len() as u128); ml_reference_projection175 })) {
            {
                let list = {
                    let source_ml_source2_array_read_value119 = source_ml_source1_temporary_values50.clone();
                    {
                        let source_ml_source2_array_read_index120 = source_ml_source1_temporary_index50.clone();
                        { let ml_reference_array176 = source_ml_source2_array_read_value119.clone(); let ml_reference_position176 = source_ml_source2_array_read_index120.clone(); let ml_reference_projection176 = self::ml_array::at(&*ml_reference_array176.borrow(), ml_reference_position176.to_index()); ml_reference_projection176 }
                    }
                };
                {
                    let at = 0f64;
                    {
                        let facts_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop68(kinds.clone(), owners.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), width, span, facts.clone(), list.clone(), at);
                        {
                            let source_ml_source1_temporary_index50_2 = source_ml_source1_temporary_index50.add(&self::ml::Big::from_i128(1));
                            {
                                (facts, source_ml_source1_temporary_index50) = (facts_2.clone(), source_ml_source1_temporary_index50_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            facts.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop64Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, f64),
    MlSource1TemporaryProveDisjointCollectionIterationsLoop64Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop72(mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values47: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index47: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index47 < ({ let ml_reference_array177 = source_ml_source1_temporary_values47.clone(); let ml_reference_projection177 = self::ml::Big::from_u128(ml_reference_array177.borrow().len() as u128); ml_reference_projection177 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value121 = source_ml_source1_temporary_values47.clone();
                    {
                        let source_ml_source2_array_read_index122 = source_ml_source1_temporary_index47.clone();
                        { let ml_reference_array178 = source_ml_source2_array_read_value121.clone(); let ml_reference_position178 = source_ml_source2_array_read_index122.clone(); let ml_reference_projection178 = self::ml_array::at(&*ml_reference_array178.borrow(), ml_reference_position178.to_index()); ml_reference_projection178 }
                    }
                };
                {
                    let roots_2 = self::add_value(roots.clone(), root);
                    {
                        let source_ml_source1_temporary_index47_2 = source_ml_source1_temporary_index47.add(&self::ml::Big::from_i128(1));
                        {
                            (roots, source_ml_source1_temporary_index47) = (roots_2.clone(), source_ml_source1_temporary_index47_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            roots.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop71(mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values46: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index46: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index46 < ({ let ml_reference_array179 = source_ml_source1_temporary_values46.clone(); let ml_reference_projection179 = self::ml::Big::from_u128(ml_reference_array179.borrow().len() as u128); ml_reference_projection179 })) {
            {
                let child = {
                    let source_ml_source2_array_read_value123 = source_ml_source1_temporary_values46.clone();
                    {
                        let source_ml_source2_array_read_index124 = source_ml_source1_temporary_index46.clone();
                        { let ml_reference_array180 = source_ml_source2_array_read_value123.clone(); let ml_reference_position180 = source_ml_source2_array_read_index124.clone(); let ml_reference_projection180 = self::ml_array::at(&*ml_reference_array180.borrow(), ml_reference_position180.to_index()); ml_reference_projection180 }
                    }
                };
                {
                    let source_ml_source1_temporary_values47 = self::read_roots(facts.clone(), 1f64, child, width, span);
                    {
                        let source_ml_source1_temporary_index47 = self::ml::Big::from_i128(0);
                        {
                            let roots_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop72(roots.clone(), source_ml_source1_temporary_values47.clone(), source_ml_source1_temporary_index47.clone());
                            {
                                let source_ml_source1_temporary_index46_2 = source_ml_source1_temporary_index46.add(&self::ml::Big::from_i128(1));
                                {
                                    (roots, source_ml_source1_temporary_index46) = (roots_2.clone(), source_ml_source1_temporary_index46_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            roots.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop76(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut field_width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64, mut role: self::ml_string::Text, mut object: f64, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ((self::read_required_number_fact(starts.clone(), node) + self::read_required_number_fact(counts.clone(), node)))) {
            if ((self::text_fact(labels.clone(), at)).starts_with(&self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 58])) || (self::text_fact(labels.clone(), at)).starts_with(&self::ml_string::Text::from_units(vec![105, 116, 101, 109, 58]))) {
                {
                    let actual = if (role == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) {
                        self::read_roots(facts.clone(), 1f64, object, width, span)
                    } else {
                        std::rc::Rc::new(std::cell::RefCell::new(vec![node]))
                    };
                    {
                        let facts_2 = self::add_field(facts.clone(), actual.clone(), (self::ml_array::index_of(&{ let ml_reference_value181 = labels.borrow().clone(); ml_reference_value181 }, &self::text_fact(labels.clone(), at), 0f64, false, |a, b| a == b) + 1f64), self::read_roots(facts.clone(), 1f64, self::read_required_number_fact(targets.clone(), at), width, span), field_width, width, span);
                        {
                            let at_2 = (at + 1f64);
                            {
                                (facts, at) = (facts_2.clone(), at_2);
                                continue;
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
        } else {
            facts.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin75 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, f64),
    MlSource1TemporaryProveDisjointCollectionIterationsJoin75Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin74 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin73 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin73Next(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop78(mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values49: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index49: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index49 < ({ let ml_reference_array182 = source_ml_source1_temporary_values49.clone(); let ml_reference_projection182 = self::ml::Big::from_u128(ml_reference_array182.borrow().len() as u128); ml_reference_projection182 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value125 = source_ml_source1_temporary_values49.clone();
                    {
                        let source_ml_source2_array_read_index126 = source_ml_source1_temporary_index49.clone();
                        { let ml_reference_array183 = source_ml_source2_array_read_value125.clone(); let ml_reference_position183 = source_ml_source2_array_read_index126.clone(); let ml_reference_projection183 = self::ml_array::at(&*ml_reference_array183.borrow(), ml_reference_position183.to_index()); ml_reference_projection183 }
                    }
                };
                {
                    let roots_2 = self::add_value(roots.clone(), root);
                    {
                        let source_ml_source1_temporary_index49_2 = source_ml_source1_temporary_index49.add(&self::ml::Big::from_i128(1));
                        {
                            (roots, source_ml_source1_temporary_index49) = (roots_2.clone(), source_ml_source1_temporary_index49_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            roots.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop77(mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values48: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index48: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index48 < ({ let ml_reference_array184 = source_ml_source1_temporary_values48.clone(); let ml_reference_projection184 = self::ml::Big::from_u128(ml_reference_array184.borrow().len() as u128); ml_reference_projection184 })) {
            {
                let target = {
                    let source_ml_source2_array_read_value127 = source_ml_source1_temporary_values48.clone();
                    {
                        let source_ml_source2_array_read_index128 = source_ml_source1_temporary_index48.clone();
                        { let ml_reference_array185 = source_ml_source2_array_read_value127.clone(); let ml_reference_position185 = source_ml_source2_array_read_index128.clone(); let ml_reference_projection185 = self::ml_array::at(&*ml_reference_array185.borrow(), ml_reference_position185.to_index()); ml_reference_projection185 }
                    }
                };
                {
                    let source_ml_source1_temporary_values49 = self::read_roots(facts.clone(), 5f64, target, width, span);
                    {
                        let source_ml_source1_temporary_index49 = self::ml::Big::from_i128(0);
                        {
                            let roots_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop78(roots.clone(), source_ml_source1_temporary_values49.clone(), source_ml_source1_temporary_index49.clone());
                            {
                                let source_ml_source1_temporary_index48_2 = source_ml_source1_temporary_index48.add(&self::ml::Big::from_i128(1));
                                {
                                    (roots, source_ml_source1_temporary_index48) = (roots_2.clone(), source_ml_source1_temporary_index48_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            roots.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin70 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin70Next(f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryProveDisjointCollectionIterationsJoin70Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop64(mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut main_: f64, mut size: f64, mut width: f64, mut field_width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut work: f64, mut node: f64) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result {
    loop {
        return if (node < size) {
            {
                let work_2 = (work + (({ let ml_reference_array186 = facts.clone(); let ml_reference_projection186 = ml_reference_array186.borrow().len() as f64; ml_reference_projection186 } + 1f64) * (self::read_required_number_fact(counts.clone(), node) + 4f64)));
                if ((((self::read_required_number_fact(facts.clone(), 0f64)) == ((-1f64))) || (work_2 > (268435456f64))) || (({ let ml_reference_array187 = facts.clone(); let ml_reference_projection187 = ml_reference_array187.borrow().len() as f64; ml_reference_projection187 }) > (65536f64))) {
                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Return(std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))
                } else {
                    {
                        let facts_3 = if ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
                            {
                                let facts_2 = self::add_roots(facts.clone(), 1f64, node, std::rc::Rc::new(std::cell::RefCell::new(vec![node])), width, span);
                                facts_2.clone()
                            }
                        } else {
                            facts.clone()
                        };
                        if !(self::ml_array::includes(&{ let ml_reference_value188 = facts_3.borrow().clone(); ml_reference_value188 }, &((4f64 * span) + self::read_required_number_fact(owners.clone(), node)), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                            {
                                let node_2 = (node + 1f64);
                                {
                                    (facts, work, node) = (facts_3.clone(), work_2, node_2);
                                    continue;
                                }
                            }
                        } else {
                            {
                                let role = self::text_fact(roles.clone(), node);
                                {
                                    let object = self::first_number(self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])));
                                    {
                                        let value = self::first_number(self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                        {
                                            let arguments_ = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116]));
                                            {
                                                let roots = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                {
                                                    let roots_3 = if (((((((role == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))) || (role == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])))) || (role == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) || (role == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) || (role == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])))) || (role == (self::ml_string::Text::from_units(vec![115, 110, 97, 112, 115, 104, 111, 116])))) || (role == (self::ml_string::Text::from_units(vec![101, 97, 103, 101, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116])))) {
                                                        {
                                                            let roots_2 = std::rc::Rc::new(std::cell::RefCell::new(vec![node]));
                                                            roots_2.clone()
                                                        }
                                                    } else {
                                                        roots.clone()
                                                    };
                                                    {
                                                        let roots_5 = if (((role == (self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 98, 108, 101]))) || (role == (self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103])))) || (role == (self::ml_string::Text::from_units(vec![114, 101, 97, 100])))) {
                                                            {
                                                                let roots_4 = if (value < (0f64)) {
                                                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                                                } else {
                                                                    self::read_roots(facts_3.clone(), 1f64, value, width, span)
                                                                };
                                                                roots_4.clone()
                                                            }
                                                        } else {
                                                            roots_3.clone()
                                                        };
                                                        {
                                                            let source_ml_source2_s129 = if (role == (self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100]))) {
                                                                {
                                                                    let actual = self::read_roots(facts_3.clone(), 1f64, object, width, span);
                                                                    {
                                                                        let work_3 = (work_2 + (({ let ml_reference_array189 = facts_3.clone(); let ml_reference_projection189 = ml_reference_array189.borrow().len() as f64; ml_reference_projection189 } + 1f64) * { let ml_reference_array190 = actual.clone(); let ml_reference_projection190 = ml_reference_array190.borrow().len() as f64; ml_reference_projection190 }));
                                                                        if (work_3 > (268435456f64)) {
                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsJoin70::MlSource1TemporaryProveDisjointCollectionIterationsJoin70Return(std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))
                                                                        } else {
                                                                            {
                                                                                let roots_6 = self::read_field(facts_3.clone(), actual.clone(), (self::ml_array::index_of(&{ let ml_reference_value191 = labels.borrow().clone(); ml_reference_value191 }, &self::text_fact(operations.clone(), node), 0f64, false, |a, b| a == b) + 1f64), field_width, width, span);
                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin70::MlSource1TemporaryProveDisjointCollectionIterationsJoin70Next(work_3, roots_6.clone())
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin70::MlSource1TemporaryProveDisjointCollectionIterationsJoin70Next(work_2, roots_5.clone())
                                                            };
                                                            match source_ml_source2_s129.clone() {
                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin70::MlSource1TemporaryProveDisjointCollectionIterationsJoin70Next(work_4, roots_7) => {
                                                                    {
                                                                        let roots_9 = if (role == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))) {
                                                                            {
                                                                                let roots_8 = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![99, 97, 108, 108, 101, 101]));
                                                                                roots_8.clone()
                                                                            }
                                                                        } else {
                                                                            roots_7.clone()
                                                                        };
                                                                        {
                                                                            let roots_11 = if (role == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                                {
                                                                                    let source_ml_source1_temporary_values46 = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116]));
                                                                                    {
                                                                                        let source_ml_source1_temporary_index46 = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            let roots_10 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop71(width, span, facts_3.clone(), roots_9.clone(), source_ml_source1_temporary_values46.clone(), source_ml_source1_temporary_index46.clone());
                                                                                            roots_10.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                roots_9.clone()
                                                                            };
                                                                            {
                                                                                let source_ml_source2_s130 = if ((role == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) || (role == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])))) {
                                                                                    {
                                                                                        let callee = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![99, 97, 108, 108, 101, 101]));
                                                                                        {
                                                                                            let actual_2 = callee.clone();
                                                                                            {
                                                                                                let actual_4 = if (role == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))) {
                                                                                                    {
                                                                                                        let actual_3 = if { let ml_value = { let ml_reference_array193 = callee.clone(); let ml_reference_projection193 = ml_reference_array193.borrow().len() as f64; ml_reference_projection193 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                                            self::read_roots(facts_3.clone(), 1f64, self::read_required_number_fact(callee.clone(), 0f64), width, span)
                                                                                                        } else {
                                                                                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                                                                                        };
                                                                                                        actual_3.clone()
                                                                                                    }
                                                                                                } else {
                                                                                                    actual_2.clone()
                                                                                                };
                                                                                                {
                                                                                                    let facts_4 = self::bind_calls(facts_3.clone(), self::read_required_number_fact(owners.clone(), node), actual_4.clone(), arguments_.clone(), self::ml_string::Text::from_units(vec![]), (-1f64), kinds.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), bodies.clone(), main_, width, span);
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_values48 = actual_4.clone();
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index48 = self::ml::Big::from_i128(0);
                                                                                                            {
                                                                                                                let roots_12 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop77(width, span, facts_4.clone(), roots_11.clone(), source_ml_source1_temporary_values48.clone(), source_ml_source1_temporary_index48.clone());
                                                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin73::MlSource1TemporaryProveDisjointCollectionIterationsJoin73Next(facts_4.clone(), roots_12.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin73::MlSource1TemporaryProveDisjointCollectionIterationsJoin73Next(facts_3.clone(), roots_11.clone())
                                                                                };
                                                                                match source_ml_source2_s130.clone() {
                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin73::MlSource1TemporaryProveDisjointCollectionIterationsJoin73Next(facts_5, roots_13) => {
                                                                                        {
                                                                                            let source_ml_source2_s131 = if (role == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) {
                                                                                                {
                                                                                                    let roots_16 = if (((self::read_reference_method_result(self::text_fact(kinds.clone(), object), self::text_fact(operations.clone(), node))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114]))) || (((self::text_fact(kinds.clone(), object)) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) && (((self::read_array_state_method_result(self::text_fact(operations.clone(), node))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114]))) || ((self::text_fact(operations.clone(), node)) == (self::ml_string::Text::from_units(vec![115, 111, 114, 116])))))) {
                                                                                                        {
                                                                                                            let roots_14 = self::read_roots(facts_5.clone(), 1f64, object, width, span);
                                                                                                            roots_14.clone()
                                                                                                        }
                                                                                                    } else {
                                                                                                        if self::read_disjoint_collection_iteration_identity_kind(self::text_fact(kinds.clone(), node)) {
                                                                                                            {
                                                                                                                let roots_15 = std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]));
                                                                                                                roots_15.clone()
                                                                                                            }
                                                                                                        } else {
                                                                                                            roots_13.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    if (self::read_disjoint_collection_iteration_callback(self::text_fact(operations.clone(), node)) && (({ let ml_reference_array194 = arguments_.clone(); let ml_reference_projection194 = ml_reference_array194.borrow().len() as f64; ml_reference_projection194 }) != (0f64))) {
                                                                                                        {
                                                                                                            let facts_6 = self::bind_calls(facts_5.clone(), self::read_required_number_fact(owners.clone(), node), self::read_roots(facts_5.clone(), 1f64, self::read_required_number_fact(arguments_.clone(), 0f64), width, span), arguments_.clone(), self::text_fact(operations.clone(), node), object, kinds.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), bodies.clone(), main_, width, span);
                                                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsJoin74::MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(facts_6.clone(), roots_16.clone())
                                                                                                        }
                                                                                                    } else {
                                                                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsJoin74::MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(facts_5.clone(), roots_16.clone())
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin74::MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(facts_5.clone(), roots_13.clone())
                                                                                            };
                                                                                            match source_ml_source2_s131.clone() {
                                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin74::MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(facts_7, roots_17) => {
                                                                                                    {
                                                                                                        let roots_19 = if (((role == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))) || (role == (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 114, 101, 97, 100])))) && self::read_disjoint_collection_iteration_identity_kind(self::text_fact(kinds.clone(), node))) {
                                                                                                            {
                                                                                                                let roots_18 = std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]));
                                                                                                                roots_18.clone()
                                                                                                            }
                                                                                                        } else {
                                                                                                            roots_17.clone()
                                                                                                        };
                                                                                                        {
                                                                                                            let facts_9 = if ((role != (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) && self::read_disjoint_collection_iteration_identity_kind(self::text_fact(kinds.clone(), node))) {
                                                                                                                {
                                                                                                                    let facts_8 = self::add_roots(facts_7.clone(), 1f64, node, roots_19.clone(), width, span);
                                                                                                                    facts_8.clone()
                                                                                                                }
                                                                                                            } else {
                                                                                                                facts_7.clone()
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_s132 = if (role == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))) {
                                                                                                                    {
                                                                                                                        let actual_5 = self::read_roots(facts_9.clone(), 1f64, value, width, span);
                                                                                                                        {
                                                                                                                            let work_5 = (work_4 + (({ let ml_reference_array195 = facts_9.clone(); let ml_reference_projection195 = ml_reference_array195.borrow().len() as f64; ml_reference_projection195 } + 1f64) * { let ml_reference_array196 = actual_5.clone(); let ml_reference_projection196 = ml_reference_array196.borrow().len() as f64; ml_reference_projection196 }));
                                                                                                                            if (work_5 > (268435456f64)) {
                                                                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Return(std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))
                                                                                                                            } else {
                                                                                                                                {
                                                                                                                                    let facts_10 = self::copy_fields(facts_9.clone(), actual_5.clone(), node, field_width, width, span);
                                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(facts_10.clone(), work_5)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(facts_9.clone(), work_4)
                                                                                                                };
                                                                                                                match source_ml_source2_s132.clone() {
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(facts_11, work_6) => {
                                                                                                                        if ((((role == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) || (role == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) || (role == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])))) || (role == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])))) {
                                                                                                                            {
                                                                                                                                let at = self::read_required_number_fact(starts.clone(), node);
                                                                                                                                {
                                                                                                                                    let facts_12 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop76(starts.clone(), counts.clone(), labels.clone(), targets.clone(), width, field_width, span, facts_11.clone(), node, role.clone(), object, at);
                                                                                                                                    {
                                                                                                                                        let node_3 = (node + 1f64);
                                                                                                                                        {
                                                                                                                                            (facts, work, node) = (facts_12.clone(), work_6, node_3);
                                                                                                                                            continue;
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let node_4 = (node + 1f64);
                                                                                                                                {
                                                                                                                                    (facts, work, node) = (facts_11.clone(), work_6, node_4);
                                                                                                                                    continue;
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Return(source_ml_source1_temporary_result76) => {
                                                                                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Return(source_ml_source1_temporary_result76.clone())
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
                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin70::MlSource1TemporaryProveDisjointCollectionIterationsJoin70Return(source_ml_source1_temporary_result78) => {
                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Return(source_ml_source1_temporary_result78.clone())
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
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Done(facts.clone(), work)
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop56(mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut main_: f64, mut snapshots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut callbacks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut width: f64, mut field_width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut work: f64, mut settled: bool, mut seeded: bool, mut pass: f64) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result {
    loop {
        return if ((pass < (128f64)) && !settled) {
            {
                let before = { let ml_reference_array197 = facts.clone(); let ml_reference_projection197 = ml_reference_array197.borrow().len() as f64; ml_reference_projection197 };
                {
                    let node = 0f64;
                    {
                        let source_ml_source2_s133 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop64(roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), bodies.clone(), main_, size, width, field_width, span, facts.clone(), work, node);
                        match source_ml_source2_s133.clone() {
                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Done(facts_2, work_2) => {
                                {
                                    let at = 0f64;
                                    {
                                        let facts_3 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop65(functions.clone(), bodies.clone(), width, span, facts_2.clone(), at);
                                        if (({ let ml_reference_array198 = facts_3.clone(); let ml_reference_projection198 = ml_reference_array198.borrow().len() as f64; ml_reference_projection198 }) == before) {
                                            {
                                                let source_ml_source2_s134 = if !seeded {
                                                    {
                                                        let seeded_2 = true;
                                                        {
                                                            let source_ml_source1_temporary_values50 = std::rc::Rc::new(std::cell::RefCell::new(vec![snapshots.clone(), callbacks.clone()]));
                                                            {
                                                                let source_ml_source1_temporary_index50 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let facts_4 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop67(kinds.clone(), owners.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), width, span, facts_3.clone(), source_ml_source1_temporary_values50.clone(), source_ml_source1_temporary_index50.clone());
                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin66::MlSource1TemporaryProveDisjointCollectionIterationsJoin66Next(facts_4.clone(), seeded_2)
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin66::MlSource1TemporaryProveDisjointCollectionIterationsJoin66Next(facts_3.clone(), seeded)
                                                };
                                                match source_ml_source2_s134.clone() {
                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin66::MlSource1TemporaryProveDisjointCollectionIterationsJoin66Next(facts_5, seeded_3) => {
                                                        {
                                                            let settled_2 = (({ let ml_reference_array199 = facts_5.clone(); let ml_reference_projection199 = ml_reference_array199.borrow().len() as f64; ml_reference_projection199 }) == before);
                                                            {
                                                                let pass_2 = (pass + 1f64);
                                                                {
                                                                    (facts, work, settled, seeded, pass) = (facts_5.clone(), work_2, settled_2, seeded_3, pass_2);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let pass_3 = (pass + 1f64);
                                                {
                                                    (facts, work, pass) = (facts_3.clone(), work_2, pass_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Return(source_ml_source1_temporary_result69) => {
                                self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Return(source_ml_source1_temporary_result69.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Done(facts.clone(), settled)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop55Done,
    MlSource1TemporaryProveDisjointCollectionIterationsLoop55Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop55(mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut at: f64) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result {
    loop {
        return if (at < ({ let ml_reference_array200 = functions.clone(); let ml_reference_projection200 = ml_reference_array200.borrow().len() as f64; ml_reference_projection200 })) {
            if ((((((((((!(self::ml_math::is_integer(self::read_required_number_fact(functions.clone(), at))) || !(self::ml_math::is_integer(self::read_required_number_fact(parameter_starts.clone(), at)))) || !(self::ml_math::is_integer(self::read_required_number_fact(parameter_counts.clone(), at)))) || !(self::ml_math::is_integer(self::read_required_number_fact(bodies.clone(), at)))) || ((self::read_required_number_fact(functions.clone(), at)) < (0f64))) || ((self::read_required_number_fact(functions.clone(), at)) >= size)) || ((self::read_required_number_fact(parameter_starts.clone(), at)) < (0f64))) || ((self::read_required_number_fact(parameter_counts.clone(), at)) < (0f64))) || (((self::read_required_number_fact(parameter_starts.clone(), at) + self::read_required_number_fact(parameter_counts.clone(), at))) > ({ let ml_reference_array201 = parameters.clone(); let ml_reference_projection201 = ml_reference_array201.borrow().len() as f64; ml_reference_projection201 }))) || ((self::read_required_number_fact(bodies.clone(), at)) < ((-1f64)))) || ((self::read_required_number_fact(bodies.clone(), at)) >= size)) {
                self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Return(std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))
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
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop54Done,
    MlSource1TemporaryProveDisjointCollectionIterationsLoop54Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop54(mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut node: f64) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result {
    loop {
        return if (node < size) {
            if (((((((!(self::ml_math::is_integer(self::read_required_number_fact(starts.clone(), node))) || !(self::ml_math::is_integer(self::read_required_number_fact(counts.clone(), node)))) || !(self::ml_math::is_integer(self::read_required_number_fact(owners.clone(), node)))) || ((self::read_required_number_fact(starts.clone(), node)) < (0f64))) || ((self::read_required_number_fact(counts.clone(), node)) < (0f64))) || (((self::read_required_number_fact(starts.clone(), node) + self::read_required_number_fact(counts.clone(), node))) > ({ let ml_reference_array202 = targets.clone(); let ml_reference_projection202 = ml_reference_array202.borrow().len() as f64; ml_reference_projection202 }))) || ((self::read_required_number_fact(owners.clone(), node)) < ((-1f64)))) || ((self::read_required_number_fact(owners.clone(), node)) >= size)) {
                self::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Return(std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))
            } else {
                {
                    let node_2 = (node + 1f64);
                    {
                        node = node_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Done
        };
    }
}

fn ml_main() {
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![115, 101, 116]))) == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))), "assertion 1");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![97, 100, 100]))) == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))), "assertion 2");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))), "assertion 3");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100, 48]))) == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))), "assertion 4");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![103, 101, 116]))) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100]))), "assertion 5");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![109, 97, 112]))) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100]))), "assertion 6");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![99, 101, 108, 108]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))), "assertion 7");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![102, 117, 116, 117, 114, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]), self::ml_string::Text::from_units(vec![115, 101, 116]))) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))), "assertion 8");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![102, 117, 116, 117, 114, 101, 79, 112, 101, 114, 97, 116, 105, 111, 110]))) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))), "assertion 9");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![115, 101, 116]))) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))), "assertion 10");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![103, 101, 116]))) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))), "assertion 11");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100, 48, 48]))) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))), "assertion 12");
    assert!(((self::accept_disjoint_collection_iteration_read_expression(self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]))) == (true)), "assertion 13");
    assert!(((self::accept_disjoint_collection_iteration_read_expression(self::ml_string::Text::from_units(vec![99, 101, 108, 108, 82, 101, 97, 100]))) == (true)), "assertion 14");
    assert!(((self::accept_disjoint_collection_iteration_read_expression(self::ml_string::Text::from_units(vec![99, 101, 108, 108, 87, 114, 105, 116, 101]))) == (false)), "assertion 15");
    assert!(((self::accept_disjoint_collection_iteration_read_expression(self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101, 87, 114, 105, 116, 101]))) == (false)), "assertion 16");
    assert!(((self::accept_disjoint_collection_iteration_read_expression(self::ml_string::Text::from_units(vec![109, 97, 116, 104]))) == (false)), "assertion 17");
    assert!(((self::accept_disjoint_collection_iteration_read_expression(self::ml_string::Text::from_units(vec![102, 117, 116, 117, 114, 101, 67, 104, 101, 99, 107, 101, 100, 79, 112, 101, 114, 97, 116, 105, 111, 110]))) == (false)), "assertion 18");
    assert!(((self::accept_disjoint_collection_iteration_callback_head(self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))) == (true)), "assertion 19");
    assert!(((self::accept_disjoint_collection_iteration_callback_head(self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) == (false)), "assertion 20");
    assert!(((self::accept_disjoint_collection_iteration_scope(true, 0f64, 0f64, 0f64, false, false)) == (true)), "assertion 21");
    assert!(((self::accept_disjoint_collection_iteration_scope(true, 0f64, 0f64, 0f64, false, true)) == (false)), "assertion 22");
    assert!(((self::accept_disjoint_collection_iteration_operation(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))) == (true)), "assertion 23");
    assert!(((self::accept_disjoint_collection_iteration_operation(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))) == (false)), "assertion 24");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))), "assertion 25");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))), "assertion 26");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))), "assertion 27");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))), "assertion 28");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))), "assertion 29");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))), "assertion 30");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 31");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 32");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![103, 101, 116]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 33");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![102, 117, 116, 117, 114, 101, 73, 116, 101, 114, 97, 116, 111, 114]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 34");
    assert!(((self::read_disjoint_collection_iteration_marker_role(self::ml_string::Text::from_units(vec![35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 48, 32, 118, 97, 108, 117, 101, 115]))) == (self::ml_string::Text::from_units(vec![108, 111, 111, 112]))), "assertion 35");
    assert!(((self::read_disjoint_collection_iteration_marker_role(self::ml_string::Text::from_units(vec![35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 112, 114, 101, 97, 100, 32, 48, 32, 118, 97, 108, 117, 101, 115]))) == (self::ml_string::Text::from_units(vec![101, 97, 103, 101, 114]))), "assertion 36");
    assert!(((self::read_disjoint_collection_iteration_marker_role(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 37");
    assert!(((self::accept_disjoint_collection_spread_receiver(self::ml_string::Text::from_units(vec![110, 97, 109, 101]))) == (true)), "assertion 38");
    assert!(((self::accept_disjoint_collection_spread_receiver(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100]))) == (true)), "assertion 39");
    assert!(((self::accept_disjoint_collection_spread_receiver(self::ml_string::Text::from_units(vec![97, 112, 112]))) == (false)), "assertion 40");
    assert!(((self::accept_disjoint_collection_spread_receiver(self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) == (false)), "assertion 41");
    assert!(((self::accept_disjoint_collection_spread_receiver(self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) == (false)), "assertion 42");
    assert!(((self::accept_disjoint_collection_spread_receiver(self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120]))) == (false)), "assertion 43");
    assert!(((self::accept_disjoint_collection_iteration_scope(true, 1f64, 0f64, 0f64, false, false)) == (false)), "assertion 44");
    assert!(((self::accept_disjoint_collection_iteration_scope(true, 0f64, 0f64, 1f64, false, false)) == (false)), "assertion 45");
    assert!(((self::accept_disjoint_collection_iteration_scope(false, 0f64, 0f64, 0f64, false, false)) == (false)), "assertion 46");
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
fn all_literal_disjoint_collection_iteration_controls() { main(); }
