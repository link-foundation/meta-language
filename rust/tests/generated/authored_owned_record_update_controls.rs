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


pub fn read_rust_owned_record_update_version_error(minor: f64) -> self::ml_string::Text {
    if ((self::ml_math::is_integer(minor) && (minor >= (90f64))) && (minor < (100f64))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115, 32, 97, 114, 101, 32, 117, 110, 99, 111, 118, 101, 114, 101, 100, 32, 111, 117, 116, 115, 105, 100, 101, 32, 82, 117, 115, 116, 32, 62, 61, 49, 46, 57, 48, 46, 48, 32, 60, 49, 46, 49, 48, 48, 46, 48])
    }
}

pub fn test_rust_owned_record_field_copy(proven: bool, copy: bool, wrappers: Vec<self::ml_string::Text>) -> bool {
    (proven && ({
        let source_ml_source1_comparison_operand1 = self::ml_array::optional_at(&wrappers, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand2 = self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]);
            match (source_ml_source1_comparison_operand1.clone(), source_ml_source1_comparison_operand2.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    } || ((((wrappers.len() as f64)) == (0f64)) && copy)))
}

pub fn read_rust_owned_record_field_mode(copy: bool, borrowed: bool) -> self::ml_string::Text {
    if copy {
        self::ml_string::Text::from_units(vec![99, 111, 112, 105, 101, 100, 32, 118, 97, 108, 117, 101])
    } else {
        if borrowed {
            self::ml_string::Text::from_units(vec![98, 111, 114, 114, 111, 119, 101, 100, 32, 112, 108, 97, 99, 101])
        } else {
            self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 112, 108, 97, 99, 101])
        }
    }
}

pub fn read_rust_owned_record_field_identity(root: self::ml_string::Text, index: f64) -> self::ml_string::Text {
    if ((((root != (self::ml_string::Text::from_units(vec![]))) && self::ml_math::is_integer(index)) && (index >= (0f64))) && (index < (1024f64))) {
        (((self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32])).concat(&root)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&{
            let source_ml_source1_string_argument3 = index;
            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument3))
        })
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_rust_owned_record_field_root(identity: self::ml_string::Text) -> self::ml_string::Text {
    if !(identity.starts_with(&self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let boundary = {
                let source_ml_source1_string_search_operand4 = identity.clone();
                {
                    let source_ml_source1_string_search_operand5 = self::ml_string::Text::from_units(vec![32]);
                    {
                        let source_ml_source1_string_search_text6 = source_ml_source1_string_search_operand5.clone();
                        {
                            let source_ml_source1_string_search_position7 = f64::NAN;
                            source_ml_source1_string_search_operand4.last_index_of(&source_ml_source1_string_search_text6, source_ml_source1_string_search_position7)
                        }
                    }
                }
            };
            {
                let root = {
                    let source_ml_source1_string_range_operand8 = identity.clone();
                    {
                        let source_ml_source1_string_range_operand9 = 13f64;
                        {
                            let source_ml_source1_string_range_operand10 = boundary;
                            {
                                let source_ml_source1_string_range_position11 = source_ml_source1_string_range_operand9;
                                {
                                    let source_ml_source1_string_range_position12 = source_ml_source1_string_range_operand10;
                                    source_ml_source1_string_range_operand8.slice(source_ml_source1_string_range_position11, source_ml_source1_string_range_position12)
                                }
                            }
                        }
                    }
                };
                {
                    let index = self::ml_string_number::convert_java_script_string_number_units(&({
                        let source_ml_source1_string_range_operand13 = identity.clone();
                        {
                            let source_ml_source1_string_range_operand14 = (boundary + 1f64);
                            {
                                let source_ml_source1_string_range_position15 = source_ml_source1_string_range_operand14;
                                {
                                    let source_ml_source1_string_range_position16 = f64::INFINITY;
                                    source_ml_source1_string_range_operand13.slice(source_ml_source1_string_range_position15, source_ml_source1_string_range_position16)
                                }
                            }
                        }
                    }).0.iter().copied().map(f64::from).collect::<Vec<f64>>());
                    if ((self::read_rust_owned_record_field_identity(root.clone(), index)) == identity) {
                        root.clone()
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    }
}

pub fn read_rust_owned_record_absent_field_state(root: self::ml_string::Text) -> self::ml_string::Text {
    if (root == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))) {
        self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101])
    } else {
        self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
    }
}

pub fn read_rust_owned_record_ultimate_root(identity: self::ml_string::Text) -> self::ml_string::Text {
    {
        let root = self::read_rust_owned_record_field_root(identity.clone());
        if (root == (self::ml_string::Text::from_units(vec![]))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let depth = 0f64;
                {
                    let source_ml_source1_s17 = self::source_ml_temporary_read_rust_owned_record_ultimate_root_loop1(root.clone(), depth);
                    match source_ml_source1_s17.clone() {
                        self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Done => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn test_rust_owned_record_field_descendant(root: self::ml_string::Text, identity: self::ml_string::Text) -> bool {
    {
        let parent = self::read_rust_owned_record_field_root(identity.clone());
        {
            let depth = 0f64;
            {
                let source_ml_source1_s18 = self::source_ml_temporary_test_rust_owned_record_field_descendant_loop2(root.clone(), parent.clone(), depth);
                match source_ml_source1_s18.clone() {
                    self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Done => {
                        false
                    }
                    self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Return(source_ml_temporary_result2) => {
                        source_ml_temporary_result2
                    }
                }
            }
        }
    }
}

pub fn read_rust_owned_record_whole_error(root: self::ml_string::Text, identities: Vec<self::ml_string::Text>, states: Vec<self::ml_string::Text>) -> self::ml_string::Text {
    if (((identities.len() as f64)) != ((states.len() as f64))) {
        self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 97, 118, 97, 105, 108, 97, 98, 105, 108, 105, 116, 121, 32, 110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 115, 116, 97, 116, 101])
    } else {
        if (root == (self::ml_string::Text::from_units(vec![]))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_temporary_values3 = identities;
                    {
                        let source_ml_temporary_index3 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s19 = self::source_ml_temporary_read_rust_owned_record_whole_error_loop4(root.clone(), states, index, source_ml_temporary_values3, source_ml_temporary_index3.clone());
                            match source_ml_source1_s19.clone() {
                                self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Done => {
                                    self::ml_string::Text::from_units(vec![])
                                }
                                self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Return(source_ml_temporary_result4) => {
                                    source_ml_temporary_result4.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_owned_record_field_error(root: self::ml_string::Text, field: self::ml_string::Text) -> self::ml_string::Text {
    if ((root == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))) && (field == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![97, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 114, 111, 111, 116, 32, 97, 110, 100, 32, 117, 110, 109, 111, 118, 101, 100, 32, 102, 105, 101, 108, 100])
    }
}

pub fn read_rust_owned_record_update_error(profile: self::ml_string::Text, owner: bool, layers: f64, fields: f64, positions: f64) -> self::ml_string::Text {
    if ((((((((profile == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]))) || (profile == (self::ml_string::Text::from_units(vec![99, 111, 112, 121])))) && owner) && (layers == (0f64))) && self::ml_math::is_integer(fields)) && (fields >= (0f64))) && (fields <= (1024f64))) && (positions == fields)) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 114, 117, 99, 116, 111, 114, 45, 102, 114, 101, 101, 32, 115, 116, 114, 117, 99, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 102, 105, 101, 108, 100, 32, 108, 97, 121, 111, 117, 116])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result {
    MlTemporaryReadRustOwnedRecordUltimateRootLoop1Done,
    MlTemporaryReadRustOwnedRecordUltimateRootLoop1Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_rust_owned_record_ultimate_root_loop1(mut root: self::ml_string::Text, mut depth: f64) -> self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result {
    loop {
        return if (depth < (1024f64)) {
            {
                let parent = self::read_rust_owned_record_field_root(root.clone());
                if (parent == (self::ml_string::Text::from_units(vec![]))) {
                    self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Return(root.clone())
                } else {
                    {
                        let root_2 = parent.clone();
                        {
                            let depth_2 = (depth + 1f64);
                            {
                                (root, depth) = (root_2.clone(), depth_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result {
    MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Done,
    MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Return(bool),
}

pub fn source_ml_temporary_test_rust_owned_record_field_descendant_loop2(mut root: self::ml_string::Text, mut parent: self::ml_string::Text, mut depth: f64) -> self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result {
    loop {
        return if ((depth < (1024f64)) && (parent != (self::ml_string::Text::from_units(vec![])))) {
            if (parent == root) {
                self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Return(true)
            } else {
                {
                    let parent_2 = self::read_rust_owned_record_field_root(parent.clone());
                    {
                        let depth_2 = (depth + 1f64);
                        {
                            (parent, depth) = (parent_2.clone(), depth_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result {
    MlTemporaryReadRustOwnedRecordWholeErrorLoop4Done,
    MlTemporaryReadRustOwnedRecordWholeErrorLoop4Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_rust_owned_record_whole_error_loop4(mut root: self::ml_string::Text, mut states: Vec<self::ml_string::Text>, mut index: f64, mut source_ml_temporary_values3: Vec<self::ml_string::Text>, mut source_ml_temporary_index3: self::ml::Big) -> self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result {
    loop {
        return if (source_ml_temporary_index3 < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))) {
            {
                let identity = self::ml_array::at(&source_ml_temporary_values3, source_ml_temporary_index3.to_index());
                if (self::test_rust_owned_record_field_descendant(root.clone(), identity.clone()) && {
                    let source_ml_source1_comparison_operand20 = self::ml_array::optional_at(&states, self::ml_array::number_index(index));
                    {
                        let source_ml_source1_comparison_operand21 = self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]);
                        !(match (source_ml_source1_comparison_operand20.clone(), source_ml_source1_comparison_operand21.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                    }
                }) {
                    self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Return(self::ml_string::Text::from_units(vec![97, 32, 112, 97, 114, 116, 105, 97, 108, 108, 121, 32, 109, 111, 118, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 117, 115, 101, 100, 32, 97, 115, 32, 97, 32, 119, 104, 111, 108, 101, 32, 118, 97, 108, 117, 101]))
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            let source_ml_temporary_index3_2 = source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                            {
                                (index, source_ml_temporary_index3) = (index_2, source_ml_temporary_index3_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Done
        };
    }
}

fn ml_main() {
    assert!(((self::read_rust_owned_record_update_version_error(90f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 1");
    assert!(((self::read_rust_owned_record_update_version_error(99f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 2");
    assert!(((self::read_rust_owned_record_update_version_error(89f64)) == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115, 32, 97, 114, 101, 32, 117, 110, 99, 111, 118, 101, 114, 101, 100, 32, 111, 117, 116, 115, 105, 100, 101, 32, 82, 117, 115, 116, 32, 62, 61, 49, 46, 57, 48, 46, 48, 32, 60, 49, 46, 49, 48, 48, 46, 48]))), "assertion 3");
    assert!(((self::read_rust_owned_record_update_version_error(100f64)) == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115, 32, 97, 114, 101, 32, 117, 110, 99, 111, 118, 101, 114, 101, 100, 32, 111, 117, 116, 115, 105, 100, 101, 32, 82, 117, 115, 116, 32, 62, 61, 49, 46, 57, 48, 46, 48, 32, 60, 49, 46, 49, 48, 48, 46, 48]))), "assertion 4");
    assert!(((self::read_rust_owned_record_field_identity(self::ml_string::Text::from_units(vec![55]), 0f64)) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48]))), "assertion 5");
    assert!(((self::read_rust_owned_record_field_identity(self::ml_string::Text::from_units(vec![55]), 1023f64)) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 49, 48, 50, 51]))), "assertion 6");
    assert!(((self::read_rust_owned_record_field_identity(self::ml_string::Text::from_units(vec![55]), 1024f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 7");
    assert!(((self::read_rust_owned_record_field_identity(self::ml_string::Text::from_units(vec![]), 0f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 8");
    assert!(((self::read_rust_owned_record_field_root(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48]))) == (self::ml_string::Text::from_units(vec![55]))), "assertion 9");
    assert!(((self::read_rust_owned_record_field_root(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48, 48]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 10");
    assert!(((self::read_rust_owned_record_field_root(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 49, 48, 50, 52]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 11");
    assert!(((self::read_rust_owned_record_field_root(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 55]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 12");
    assert!(((self::read_rust_owned_record_field_root(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48, 32, 49]))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48]))), "assertion 13");
    assert!(((self::read_rust_owned_record_ultimate_root(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48, 32, 49]))) == (self::ml_string::Text::from_units(vec![55]))), "assertion 14");
    assert!(((self::test_rust_owned_record_field_descendant(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48, 32, 49]))) == (true)), "assertion 15");
    assert!(((self::test_rust_owned_record_field_descendant(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 48, 32, 48]))) == (false)), "assertion 16");
    assert!(((self::read_rust_owned_record_whole_error(self::ml_string::Text::from_units(vec![55]), vec![self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48])], vec![self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101])])) == (self::ml_string::Text::from_units(vec![]))), "assertion 17");
    assert!(((self::read_rust_owned_record_whole_error(self::ml_string::Text::from_units(vec![55]), vec![self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48])], vec![self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100])])) == (self::ml_string::Text::from_units(vec![97, 32, 112, 97, 114, 116, 105, 97, 108, 108, 121, 32, 109, 111, 118, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 117, 115, 101, 100, 32, 97, 115, 32, 97, 32, 119, 104, 111, 108, 101, 32, 118, 97, 108, 117, 101]))), "assertion 18");
    assert!(((self::read_rust_owned_record_whole_error(self::ml_string::Text::from_units(vec![55]), vec![self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48, 32, 49])], vec![self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100])])) == (self::ml_string::Text::from_units(vec![97, 32, 112, 97, 114, 116, 105, 97, 108, 108, 121, 32, 109, 111, 118, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 117, 115, 101, 100, 32, 97, 115, 32, 97, 32, 119, 104, 111, 108, 101, 32, 118, 97, 108, 117, 101]))), "assertion 19");
    assert!(((self::read_rust_owned_record_whole_error(self::ml_string::Text::from_units(vec![55]), vec![self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 48, 32, 48])], vec![self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100])])) == (self::ml_string::Text::from_units(vec![]))), "assertion 20");
    assert!(((self::read_rust_owned_record_field_error(self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 21");
    assert!(((self::read_rust_owned_record_field_error(self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100]))) == (self::ml_string::Text::from_units(vec![97, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 114, 111, 111, 116, 32, 97, 110, 100, 32, 117, 110, 109, 111, 118, 101, 100, 32, 102, 105, 101, 108, 100]))), "assertion 22");
    assert!(((self::read_rust_owned_record_field_error(self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100]), self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))) == (self::ml_string::Text::from_units(vec![97, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 114, 111, 111, 116, 32, 97, 110, 100, 32, 117, 110, 109, 111, 118, 101, 100, 32, 102, 105, 101, 108, 100]))), "assertion 23");
    assert!(((self::read_rust_owned_record_absent_field_state(self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))) == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))), "assertion 24");
    assert!(((self::read_rust_owned_record_absent_field_state(self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100]))) == (self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116]))), "assertion 25");
    assert!(((self::read_rust_owned_record_update_error(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]), true, 0f64, 2f64, 2f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 26");
    assert!(((self::read_rust_owned_record_update_error(self::ml_string::Text::from_units(vec![99, 111, 112, 121]), true, 0f64, 2f64, 2f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 27");
    assert!(((self::read_rust_owned_record_update_error(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]), false, 0f64, 2f64, 2f64)) == (self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 114, 117, 99, 116, 111, 114, 45, 102, 114, 101, 101, 32, 115, 116, 114, 117, 99, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 102, 105, 101, 108, 100, 32, 108, 97, 121, 111, 117, 116]))), "assertion 28");
    assert!(((self::read_rust_owned_record_update_error(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]), true, 1f64, 2f64, 2f64)) == (self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 114, 117, 99, 116, 111, 114, 45, 102, 114, 101, 101, 32, 115, 116, 114, 117, 99, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 102, 105, 101, 108, 100, 32, 108, 97, 121, 111, 117, 116]))), "assertion 29");
    assert!(((self::test_rust_owned_record_field_copy(true, true, Vec::<self::ml_string::Text>::new())) == (true)), "assertion 30");
    assert!(((self::test_rust_owned_record_field_copy(true, false, vec![self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101])])) == (true)), "assertion 31");
    assert!(((self::test_rust_owned_record_field_copy(true, false, vec![self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]), self::ml_string::Text::from_units(vec![98, 111, 120])])) == (true)), "assertion 32");
    assert!(((self::test_rust_owned_record_field_copy(true, true, vec![self::ml_string::Text::from_units(vec![98, 111, 120])])) == (false)), "assertion 33");
    assert!(((self::test_rust_owned_record_field_copy(false, true, Vec::<self::ml_string::Text>::new())) == (false)), "assertion 34");
    assert!(((self::read_rust_owned_record_field_mode(true, true)) == (self::ml_string::Text::from_units(vec![99, 111, 112, 105, 101, 100, 32, 118, 97, 108, 117, 101]))), "assertion 35");
    assert!(((self::read_rust_owned_record_field_mode(false, true)) == (self::ml_string::Text::from_units(vec![98, 111, 114, 114, 111, 119, 101, 100, 32, 112, 108, 97, 99, 101]))), "assertion 36");
    assert!(((self::read_rust_owned_record_field_mode(false, false)) == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 112, 108, 97, 99, 101]))), "assertion 37");
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
fn all_literal_owned_record_update_controls() { main(); }
