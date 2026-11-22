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
    Text(Vec<f64>),
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

pub fn test_java_script_standard_array_creation_operation(operation: self::ml_string::Text) -> bool {
    ((operation == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 111, 102]))) || (operation == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109]))))
}

pub fn test_java_script_standard_object_from_entries_operation(operation: self::ml_string::Text) -> bool {
    (operation == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115])))
}

pub fn test_java_script_standard_collection_for_each_operation(operation: self::ml_string::Text) -> bool {
    ((operation == (self::ml_string::Text::from_units(vec![77, 97, 112, 46, 102, 111, 114, 69, 97, 99, 104]))) || (operation == (self::ml_string::Text::from_units(vec![83, 101, 116, 46, 102, 111, 114, 69, 97, 99, 104]))))
}

pub fn test_java_script_standard_object_observation_operation(operation: self::ml_string::Text) -> bool {
    self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 98, 117, 105, 108, 116, 105, 110, 84, 97, 103]), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 118, 97, 108, 117, 101, 79, 102]), self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 46, 118, 97, 108, 117, 101, 79, 102]), self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110, 46, 118, 97, 108, 117, 101, 79, 102]), self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110, 46, 116, 111, 83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 46, 118, 97, 108, 117, 101, 79, 102]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 116, 111, 83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 118, 97, 108, 117, 101, 79, 102]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 114, 101, 103, 117, 108, 97, 114, 69, 120, 112, 114, 101, 115, 115, 105, 111, 110, 83, 108, 111, 116]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 104, 97, 115, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 112, 114, 111, 112, 101, 114, 116, 121, 73, 115, 69, 110, 117, 109, 101, 114, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 104, 97, 115, 79, 119, 110]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 105, 115]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 105, 115, 69, 120, 116, 101, 110, 115, 105, 98, 108, 101]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 105, 115, 83, 101, 97, 108, 101, 100]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 105, 115, 70, 114, 111, 122, 101, 110]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 101, 118, 101, 110, 116, 69, 120, 116, 101, 110, 115, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 115, 101, 97, 108]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 101, 101, 122, 101]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 115, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102])], &operation.clone(), 0f64, false, |a, b| a == b)
}

pub fn test_java_script_standard_array_intrinsic_operation(operation: self::ml_string::Text) -> bool {
    (operation == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 105, 115, 65, 114, 114, 97, 121])))
}

pub fn test_java_script_standard_array_callback_operation(operation: self::ml_string::Text) -> bool {
    self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 111, 114, 69, 97, 99, 104]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 115, 111, 109, 101]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 101, 118, 101, 114, 121]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 105, 110, 100]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 73, 110, 100, 101, 120]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 76, 97, 115, 116]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 76, 97, 115, 116, 73, 110, 100, 101, 120]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 114, 101, 100, 117, 99, 101]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 114, 101, 100, 117, 99, 101, 82, 105, 103, 104, 116])], &operation.clone(), 0f64, false, |a, b| a == b)
}

pub fn test_java_script_standard_array_transform_operation(operation: self::ml_string::Text) -> bool {
    ((operation == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 109, 97, 112]))) || (operation == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 105, 108, 116, 101, 114]))))
}

pub fn test_java_script_standard_array_species_getter_operation(operation: self::ml_string::Text) -> bool {
    (operation == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 115, 112, 101, 99, 105, 101, 115, 32, 103, 101, 116, 116, 101, 114])))
}

pub fn test_java_script_standard_object_enumeration_operation(operation: self::ml_string::Text) -> bool {
    self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 107, 101, 121, 115]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 101, 110, 116, 114, 105, 101, 115])], &operation.clone(), 0f64, false, |a, b| a == b)
}

pub fn test_java_script_standard_property_reflection_operation(operation: self::ml_string::Text) -> bool {
    self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 78, 97, 109, 101, 115]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 83, 121, 109, 98, 111, 108, 115]), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 111, 119, 110, 75, 101, 121, 115]), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114])], &operation.clone(), 0f64, false, |a, b| a == b)
}

pub fn test_java_script_standard_single_descriptor_operation(operation: self::ml_string::Text) -> bool {
    ((operation == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114]))) || (operation == (self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114]))))
}

pub fn test_java_script_standard_property_definitions_operation(operation: self::ml_string::Text) -> bool {
    ((operation == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115]))) || (operation == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97, 116, 101]))))
}

pub fn test_java_script_standard_primitive_method_operation(operation: self::ml_string::Text) -> bool {
    (operation == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104])))
}

pub fn read_java_script_call_integer_width(value: self::ml::Big) -> f64 {
    {
        let rest = if (value < (self::ml::Big::from_i128(0))) {
            value.neg()
        } else {
            value.clone()
        };
        {
            let width = 0f64;
            {
                let source_ml_source2_s1 = self::source_ml_source1_temporary_read_java_script_call_integer_width_loop1(rest.clone(), width);
                match source_ml_source2_s1.clone() {
                    self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Done(width_2) => {
                        width_2
                    }
                    self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Return(source_ml_source1_temporary_result1) => {
                        source_ml_source1_temporary_result1
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_call_text_conversion(value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    {
        let text = self::ml_string::Text::from_units(vec![]);
        {
            let source_ml_source2_s10 = match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    {
                        let text_2 = self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]);
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Next(text_2.clone())
                    }
                }
                self::JavaScriptSemanticValue::Null => {
                    {
                        let text_3 = self::ml_string::Text::from_units(vec![110, 117, 108, 108]);
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Next(text_3.clone())
                    }
                }
                self::JavaScriptSemanticValue::Boolean(value_value) => {
                    {
                        let text_4 = if value_value {
                            self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                        } else {
                            self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                        };
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Next(text_4.clone())
                    }
                }
                self::JavaScriptSemanticValue::Number(value_value_2) => {
                    {
                        let text_5 = {
                            let source_ml_source2_string_argument17 = value_value_2;
                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument17))
                        };
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Next(text_5.clone())
                    }
                }
                self::JavaScriptSemanticValue::Integer(value_value_3) => {
                    if ((self::read_java_script_call_integer_width(value_value_3.clone())) < (0f64)) {
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))))
                    } else {
                        {
                            let text_6 = {
                                let source_ml_source2_string_argument18 = value_value_3.clone();
                                self::ml_string::Text::from_text(&source_ml_source2_string_argument18.to_string())
                            };
                            self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Next(text_6.clone())
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f11) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))))
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f12, source_ml_source2_f13) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))))
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f14) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 115, 116, 114, 105, 110, 103]))))
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f15, source_ml_source2_f16) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))))
                }
            };
            match source_ml_source2_s10.clone() {
                self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Next(text_7) => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index = 0f64;
                            {
                                let units_2 = self::source_ml_source1_temporary_execute_java_script_call_text_conversion_loop3(text_7.clone(), units, index);
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units_2)))
                            }
                        }
                    }
                }
                self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Return(source_ml_source1_temporary_result3) => {
                    let source_ml_source1_temporary_result3 = *source_ml_source1_temporary_result3;
                    source_ml_source1_temporary_result3.clone()
                }
            }
        }
    }
}

pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    ((((((((((((unit >= (9f64)) && (unit <= (13f64))) || (unit == (32f64))) || (unit == (160f64))) || (unit == (5760f64))) || ((unit >= (8192f64)) && (unit <= (8202f64)))) || (unit == (8232f64))) || (unit == (8233f64))) || (unit == (8239f64))) || (unit == (8287f64))) || (unit == (12288f64))) || (unit == (65279f64)))
}

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

pub fn read_java_script_utf8_string_units(bytes: Vec<f64>) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s19 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(bytes.clone(), index);
            match source_ml_source2_s19.clone() {
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s20 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop5(bytes, units, index_2);
                                match source_ml_source2_s20.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Done(units_2) => {
                                        units_2
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Return(source_ml_source1_temporary_result5) => {
                                        source_ml_source1_temporary_result5
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(source_ml_source1_temporary_result7) => {
                    source_ml_source1_temporary_result7
                }
            }
        }
    }
}

pub fn read_string_number_digit(unit: f64) -> f64 {
    self::read_java_script_numeric_digit_unit(unit)
}

pub fn read_string_number_decimal_power(exponent: f64) -> self::ml::Big {
    {
        let value = self::ml::Big::from_i128(1);
        {
            let base = self::ml::Big::from_i128(10);
            {
                let value_2 = self::source_ml_source1_temporary_read_string_number_decimal_power_loop8(exponent, value.clone(), base.clone());
                value_2.clone()
            }
        }
    }
}

pub fn read_string_number_exact_mantissa(value: self::ml::Big) -> f64 {
    {
        let result = 0f64;
        {
            let scale = 1f64;
            {
                let result_2 = self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop9(value.clone(), result, scale);
                result_2
            }
        }
    }
}

pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s21 = self::source_ml_source1_temporary_read_string_number_binary_power_loop10(exponent, result);
            match source_ml_source2_s21.clone() {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop10Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop10Done(exponent_2, result_2) => {
                    {
                        let result_3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop11(exponent_2, result_2);
                        result_3
                    }
                }
            }
        }
    }
}

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
                    let source_ml_source2_s24 = if (numerator >= denominator) {
                        {
                            let scale = denominator.clone();
                            {
                                let source_ml_source2_s25 = self::source_ml_source1_temporary_convert_string_number_rational_loop18(numerator.clone(), exponent, scale.clone());
                                match source_ml_source2_s25.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop18Result::MlSource1TemporaryConvertStringNumberRationalLoop18Done(exponent_2) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin12::MlSource1TemporaryConvertStringNumberRationalJoin12Next(exponent_2)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop18Result::MlSource1TemporaryConvertStringNumberRationalLoop18Return(source_ml_source1_temporary_result18) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin12::MlSource1TemporaryConvertStringNumberRationalJoin12Return(source_ml_source1_temporary_result18)
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let scale_2 = numerator.clone();
                            {
                                let source_ml_source2_s26 = self::source_ml_source1_temporary_convert_string_number_rational_loop19(denominator.clone(), exponent, scale_2.clone());
                                match source_ml_source2_s26.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop19Result::MlSource1TemporaryConvertStringNumberRationalLoop19Done(exponent_3) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin12::MlSource1TemporaryConvertStringNumberRationalJoin12Next(exponent_3)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop19Result::MlSource1TemporaryConvertStringNumberRationalLoop19Return(source_ml_source1_temporary_result19) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin12::MlSource1TemporaryConvertStringNumberRationalJoin12Return(source_ml_source1_temporary_result19)
                                    }
                                }
                            }
                        }
                    };
                    match source_ml_source2_s24.clone() {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin12::MlSource1TemporaryConvertStringNumberRationalJoin12Next(exponent_4) => {
                            {
                                let shift = if (exponent_4 >= ((-1022f64))) {
                                    (52f64 - exponent_4)
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s27 = if (shift >= (0f64)) {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop16(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin13::MlSource1TemporaryConvertStringNumberRationalJoin13Next(numerator_2.clone(), denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop17(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin13::MlSource1TemporaryConvertStringNumberRationalJoin13Next(numerator.clone(), denominator_2.clone())
                                            }
                                        }
                                    };
                                    match source_ml_source2_s27.clone() {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin13::MlSource1TemporaryConvertStringNumberRationalJoin13Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale_3 = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s28 = self::source_ml_source1_temporary_convert_string_number_rational_loop14(remainder.clone(), scale_3.clone(), weight.clone());
                                                                match source_ml_source2_s28.clone() {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop14Result::MlSource1TemporaryConvertStringNumberRationalLoop14Done(scale_4, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s29 = self::source_ml_source1_temporary_convert_string_number_rational_loop15(quotient.clone(), remainder.clone(), scale_4.clone(), weight_2.clone());
                                                                            match source_ml_source2_s29.clone() {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Done(quotient_2, remainder_2) => {
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
                        self::MlSource1TemporaryConvertStringNumberRationalJoin12::MlSource1TemporaryConvertStringNumberRationalJoin12Return(source_ml_source1_temporary_result17) => {
                            source_ml_source1_temporary_result17
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_string_number_infinity(units: Vec<f64>, start: f64, end: f64) -> bool {
    if (((end - start)) != (8f64)) {
        false
    } else {
        {
            let expected = vec![73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64];
            {
                let index = 0f64;
                {
                    let source_ml_source2_s30 = self::source_ml_source1_temporary_accept_string_number_infinity_loop20(units, start, expected, index);
                    match source_ml_source2_s30.clone() {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop20Result::MlSource1TemporaryAcceptStringNumberInfinityLoop20Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop20Result::MlSource1TemporaryAcceptStringNumberInfinityLoop20Return(source_ml_source1_temporary_result20) => {
                            source_ml_source1_temporary_result20
                        }
                    }
                }
            }
        }
    }
}

pub fn convert_java_script_string_number_units(units: Vec<f64>) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s31 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop21(units.clone(), index);
            match source_ml_source2_s31.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop22(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop23(units.clone(), start_2, end);
                                    if (start_2 == end_2) {
                                        0f64
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(start_2))).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let source_ml_source2_s32 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = (start_2 + 1f64);
                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s32.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(start_4, negative_3, signed_3) => {
                                                                if (start_4 == end_2) {
                                                                    f64::NAN
                                                                } else {
                                                                    if self::accept_string_number_infinity(units.clone(), start_4, end_2) {
                                                                        if negative_3 {
                                                                            f64::NEG_INFINITY
                                                                        } else {
                                                                            f64::INFINITY
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let integer_digits = vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)];
                                                                            {
                                                                                let source_ml_source2_s34 = if ((!signed_3 && (((self::ml_array::optional_at(&units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
                                                                                    {
                                                                                        let marker = (self::ml_array::optional_at(&units, self::ml_array::number_index((start_4 + 1f64)))).clone().unwrap_or_else(|| (-1f64));
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
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let index_2 = start_5;
                                                                                                                            {
                                                                                                                                let source_ml_source2_s35 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop32(units.clone(), end_2, integer_digits.clone(), radix, value.clone(), overflow, limit.clone(), base.clone(), index_2);
                                                                                                                                match source_ml_source2_s35.clone() {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Done(value_2, overflow_2) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Return(if overflow_2 {
                                                                                                                                            f64::INFINITY
                                                                                                                                        } else {
                                                                                                                                            self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                        })
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Return(source_ml_source1_temporary_result32) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Return(source_ml_source1_temporary_result32)
                                                                                                                                    }
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Next(start_4)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s34.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Next(start_6) => {
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
                                                                                                                let source_ml_source2_s36 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop26(units.clone(), end_2, at, fraction, digits, significant, dot);
                                                                                                                match source_ml_source2_s36.clone() {
                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Done(at_2, fraction_2, digits_2, significant_2) => {
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
                                                                                                                                            let source_ml_source2_s39 = if ((at_2 < end_2) && ((((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (101f64)) || (((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (69f64)))) {
                                                                                                                                                {
                                                                                                                                                    let at_3 = (at_2 + 1f64);
                                                                                                                                                    {
                                                                                                                                                        let exponent_negative = false;
                                                                                                                                                        {
                                                                                                                                                            let exponent_sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| (-1f64));
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_s40 = if ((exponent_sign == (43f64)) || (exponent_sign == (45f64))) {
                                                                                                                                                                    {
                                                                                                                                                                        let exponent_negative_2 = (exponent_sign == (45f64));
                                                                                                                                                                        {
                                                                                                                                                                            let at_4 = (at_3 + 1f64);
                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30Next(at_4, exponent_negative_2)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30Next(at_3, exponent_negative)
                                                                                                                                                                };
                                                                                                                                                                match source_ml_source2_s40.clone() {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30Next(at_5, exponent_negative_3) => {
                                                                                                                                                                        {
                                                                                                                                                                            let first = at_5;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_s41 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop31(units.clone(), end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                                match source_ml_source2_s41.clone() {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Done(at_6, exponent_2) => {
                                                                                                                                                                                        if (at_6 == first) {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Return(f64::NAN)
                                                                                                                                                                                        } else {
                                                                                                                                                                                            if exponent_negative_3 {
                                                                                                                                                                                                {
                                                                                                                                                                                                    let exponent_3 = (-exponent_2);
                                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_6, exponent_3)
                                                                                                                                                                                                }
                                                                                                                                                                                            } else {
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_6, exponent_2)
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_2, exponent)
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s39.clone() {
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_7, exponent_4) => {
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
                                                                                                                                                                                    let source_ml_source2_s42 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop28(units.clone(), start_6, exponent_5, mantissa_end);
                                                                                                                                                                                    match source_ml_source2_s42.clone() {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let index_3 = start_6;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop29(units, integer_digits, mantissa_end_2, mantissa.clone(), index_3);
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Return(source_ml_source1_temporary_result29) => {
                                                                                                                                                    source_ml_source1_temporary_result29
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Return(source_ml_source1_temporary_result31) => {
                                                                                        source_ml_source1_temporary_result31
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Return(source_ml_source1_temporary_result32_2) => {
                    source_ml_source1_temporary_result32_2
                }
            }
        }
    }
}

pub fn convert_java_script_big_integer_number(value: self::ml::Big) -> f64 {
    if (value < (self::ml::Big::from_i128(0))) {
        (-(self::convert_string_number_rational(value.neg(), self::ml::Big::from_i128(1))))
    } else {
        self::convert_string_number_rational(value.clone(), self::ml::Big::from_i128(1))
    }
}

pub fn convert_java_script_utf8_string_number_units(bytes: Vec<f64>) -> f64 {
    self::convert_java_script_string_number_units(self::read_java_script_utf8_string_units(bytes))
}

pub fn read_java_script_number_builtin_operation(name: self::ml_string::Text) -> self::ml_string::Text {
    if (name == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]))) {
        self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])
    } else {
        if (name == (self::ml_string::Text::from_units(vec![105, 115, 70, 105, 110, 105, 116, 101]))) {
            self::ml_string::Text::from_units(vec![116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 70, 105, 110, 105, 116, 101])
        } else {
            if (name == (self::ml_string::Text::from_units(vec![105, 115, 78, 97, 78]))) {
                self::ml_string::Text::from_units(vec![116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 78, 97, 78])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_java_script_number_conversion_result(operation: self::ml_string::Text) -> self::ml_string::Text {
    if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])))) {
        self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])
    } else {
        if ((operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 70, 105, 110, 105, 116, 101]))) || (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 78, 97, 78])))) {
            self::ml_string::Text::from_units(vec![98, 111, 111, 108])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_java_script_number_conversion_form(kind: self::ml_string::Text) -> self::ml_string::Text {
    if (((((((kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || (kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) || (kind == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))) || (kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))) {
        kind.clone()
    } else {
        self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100])
    }
}

pub fn accept_linked_java_script_number_conversion(kind: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    ((kind == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))) && ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])))))
}

pub fn read_java_script_number_conversion_failure(operation: self::ml_string::Text, kind: self::ml_string::Text) -> self::ml_string::Text {
    if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) && ((kind == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116]))))) {
        self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn require_java_script_number_conversion_units(kind: self::ml_string::Text, operation: self::ml_string::Text, argument: self::ml_string::Text) -> bool {
    (self::accept_linked_java_script_number_conversion(kind.clone(), operation.clone()) && (argument == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
}

pub fn read_java_script_primitive_number_operand(language: self::ml_string::Text, kind: self::ml_string::Text, element: self::ml_string::Text) -> self::ml_string::Text {
    if (language != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (((kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) {
            self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])
        } else {
            if ((kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && (element == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_java_script_primitive_numeric_inference_kind(kind: self::ml_string::Text) -> self::ml_string::Text {
    if (((kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) {
        self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_java_script_primitive_unary_operation(symbol: self::ml_string::Text) -> self::ml_string::Text {
    if (symbol == (self::ml_string::Text::from_units(vec![43]))) {
        self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_java_script_mathematical_conversion_form(language: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if (language != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 65, 114, 103, 117, 109, 101, 110, 116, 115]))) {
            self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121])
        } else {
            if ((operation == (self::ml_string::Text::from_units(vec![109, 97, 120]))) || (operation == (self::ml_string::Text::from_units(vec![109, 105, 110])))) {
                self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 100, 105, 99])
            } else {
                if (((((((operation == (self::ml_string::Text::from_units(vec![97, 98, 115]))) || (operation == (self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114])))) || (operation == (self::ml_string::Text::from_units(vec![99, 101, 105, 108])))) || (operation == (self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99])))) || (operation == (self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100])))) || (operation == (self::ml_string::Text::from_units(vec![115, 105, 103, 110])))) || (operation == (self::ml_string::Text::from_units(vec![115, 113, 114, 116])))) {
                    self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn render_java_script_number_scalar_conversion(format: self::ml_string::Text, operation: self::ml_string::Text, form: self::ml_string::Text, value: self::ml_string::Text, utf16: bool) -> self::ml_string::Text {
    if ((!(self::accept_linked_java_script_number_conversion(self::ml_string::Text::from_units(vec![109, 97, 116, 104]), operation.clone())) || ((self::read_java_script_number_conversion_form(form.clone())) == (self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100])))) || (form == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (format == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) {
                ((self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                ((self::ml_string::Text::from_units(vec![40, 43, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
            }
        } else {
            if ((format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (format == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                if !(self::read_java_script_number_conversion_failure(operation.clone(), form.clone())).0.is_empty() {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    if (form == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                        value.clone()
                    } else {
                        if (form == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                            if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                ((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32, 95, 32, 58, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 40, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 47, 32, 48, 41, 41]))
                            } else {
                                ((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32, 95, 32, 58, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 105, 110, 32, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 110, 97, 110, 41]))
                            }
                        } else {
                            if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
                                if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                    ((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 116, 104, 101, 110, 32, 40, 49, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 41]))
                                } else {
                                    ((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 116, 104, 101, 110, 32, 49, 37, 102, 108, 111, 97, 116, 32, 101, 108, 115, 101, 32, 48, 37, 102, 108, 111, 97, 116, 41]))
                                }
                            } else {
                                if ((form == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (form == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
                                    ((self::ml_string::Text::from_units(vec![40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 73, 110, 116, 101, 103, 101, 114, 32])).concat(&if (form == (self::ml_string::Text::from_units(vec![110, 97, 116]))) {
                                        if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                            ((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![40, 90, 46, 111, 102, 95, 78, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                        }
                                    } else {
                                        value.clone()
                                    })).concat(&self::ml_string::Text::from_units(vec![41]))
                                } else {
                                    if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                        ((self::ml_string::Text::from_units(vec![40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 85, 110, 105, 116, 115, 32, 40, 83, 116, 114, 105, 110, 103, 46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32, 40, 117, 110, 105, 116, 115, 32, 58, 32, 65, 114, 114, 97, 121, 32, 70, 108, 111, 97, 116, 41, 32, 99, 32, 61, 62, 32, 105, 102, 32, 99, 46, 116, 111, 78, 97, 116, 32, 60, 32, 54, 53, 53, 51, 54, 32, 116, 104, 101, 110, 32, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 99, 46, 116, 111, 78, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 53, 50, 57, 54, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 47, 32, 49, 48, 50, 52, 41, 41, 41, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 54, 51, 50, 48, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 37, 32, 49, 48, 50, 52, 41, 41, 41, 32, 35, 91, 93, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                    } else {
                                        ((self::ml_string::Text::from_units(vec![40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 85, 116, 102, 56, 32, 40, 40, 102, 105, 120, 32, 109, 108, 95, 110, 117, 109, 101, 114, 105, 99, 95, 98, 121, 116, 101, 115, 32, 40, 116, 101, 120, 116, 32, 58, 32, 115, 116, 114, 105, 110, 103, 41, 32, 58, 32, 108, 105, 115, 116, 32, 102, 108, 111, 97, 116, 32, 58, 61, 32, 109, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 119, 105, 116, 104, 32, 69, 109, 112, 116, 121, 83, 116, 114, 105, 110, 103, 32, 61, 62, 32, 110, 105, 108, 32, 124, 32, 83, 116, 114, 105, 110, 103, 32, 99, 32, 114, 101, 115, 116, 32, 61, 62, 32, 99, 111, 110, 115, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54, 51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102, 95, 110, 97, 116, 32, 40, 65, 115, 99, 105, 105, 46, 110, 97, 116, 95, 111, 102, 95, 97, 115, 99, 105, 105, 32, 99, 41, 41, 41, 41, 32, 40, 109, 108, 95, 110, 117, 109, 101, 114, 105, 99, 95, 98, 121, 116, 101, 115, 32, 114, 101, 115, 116, 41, 32, 101, 110, 100, 41, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if (format != (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    if (form == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                        value.clone()
                    } else {
                        if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
                            ((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 49, 46, 48, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 48, 46, 48, 102, 54, 52, 32, 125, 41]))
                        } else {
                            if (form == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                                ((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 95, 32, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 102, 54, 52, 58, 58, 78, 65, 78, 32, 125]))
                            } else {
                                if ((form == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (form == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
                                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) {
                                        ((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 95, 32, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 111, 117, 114, 99, 101, 95, 101, 120, 99, 101, 112, 116, 105, 111, 110, 58, 58, 83, 111, 117, 114, 99, 101, 69, 120, 99, 101, 112, 116, 105, 111, 110, 58, 58, 102, 114, 111, 109, 95, 116, 101, 120, 116, 40, 34, 84, 121, 112, 101, 69, 114, 114, 111, 114, 34, 44, 32, 34, 67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114, 34, 41, 46, 114, 97, 105, 115, 101, 40, 41, 32, 125]))
                                    } else {
                                        ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 110, 117, 109, 98, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                    }
                                } else {
                                    {
                                        let units = if utf16 {
                                            ((self::ml_string::Text::from_units(vec![40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 46, 48, 46, 105, 116, 101, 114, 40, 41, 46, 99, 111, 112, 105, 101, 100, 40, 41]))
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 46, 101, 110, 99, 111, 100, 101, 95, 117, 116, 102, 49, 54, 40, 41]))
                                        };
                                        ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 95, 117, 110, 105, 116, 115, 40, 38])).concat(&units)).concat(&self::ml_string::Text::from_units(vec![46, 109, 97, 112, 40, 102, 54, 52, 58, 58, 102, 114, 111, 109, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 102, 54, 52, 62, 62, 40, 41, 41]))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_number_conversion_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![69, 67, 77, 65, 83, 99, 114, 105, 112, 116, 32, 78, 117, 109, 98, 101, 114, 32, 97, 110, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 112, 114, 101, 115, 101, 114, 118, 101, 32, 102, 117, 108, 108, 32, 85, 84, 70, 45, 49, 54, 32, 83, 116, 114, 105, 110, 103, 78, 117, 109, 101, 114, 105, 99, 76, 105, 116, 101, 114, 97, 108, 32, 115, 121, 110, 116, 97, 120, 44, 32, 101, 120, 97, 99, 116, 32, 98, 105, 110, 97, 114, 121, 54, 52, 32, 116, 105, 101, 115, 45, 116, 111, 45, 101, 118, 101, 110, 32, 114, 111, 117, 110, 100, 105, 110, 103, 44, 32, 115, 105, 103, 110, 101, 100, 32, 122, 101, 114, 111, 44, 32, 73, 110, 102, 105, 110, 105, 116, 121, 44, 32, 78, 97, 78, 44, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 97, 98, 115, 101, 110, 99, 101, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 101, 118, 97, 108, 117, 97, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114, 59, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 111, 117, 110, 100, 115, 32, 111, 110, 99, 101, 32, 97, 110, 100, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 116, 104, 114, 111, 119, 115])
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f43) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f44) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f45) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Text(units) => {
            if (((units.len() as f64)) > (1048576f64)) {
                self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s47 = self::source_ml_source1_temporary_read_java_script_semantic_value_error_loop33(units, at);
                        match source_ml_source2_s47.clone() {
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Return(source_ml_source1_temporary_result33) => {
                                source_ml_source1_temporary_result33.clone()
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
        self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f46) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f52) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f53) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f54) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f55) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f56, source_ml_source2_f57) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f58) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f59, source_ml_source2_f60) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f61) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f62) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f63) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f64) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f65, source_ml_source2_f66) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f67) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f68, source_ml_source2_f69) => {
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
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f70) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f71) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f72) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f73, source_ml_source2_f74) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f75) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f76, source_ml_source2_f77) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f78) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f79) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f80) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f81, source_ml_source2_f82) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f83) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f84, source_ml_source2_f85) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f86) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f87) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f88) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f89, source_ml_source2_f90) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f91) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f92, source_ml_source2_f93) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f48) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f49, source_ml_source2_f50) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f94) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f95) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f96) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f97) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f98, source_ml_source2_f99) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f100, source_ml_source2_f101) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, source_ml_source2_f51) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f102) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f103) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f104) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f105) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f106, source_ml_source2_f107) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f108) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f109) => {
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f110) => {
            {
                let source_ml_source2_f134 = source_ml_source2_f110;
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f111) => {
            {
                let source_ml_source2_f143 = source_ml_source2_f111;
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f112) => {
            {
                let source_ml_source2_f152 = source_ml_source2_f112.clone();
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f113) => {
            {
                let units = source_ml_source2_f113;
                (units.len() as f64)
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f114, text) => {
            text.length()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f115) => {
            {
                let source_ml_source2_f171 = source_ml_source2_f115.clone();
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f116, source_ml_source2_f117) => {
            {
                let source_ml_source2_f180 = source_ml_source2_f116.clone();
                {
                    let source_ml_source2_f181 = source_ml_source2_f117;
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f182) => {
                {
                    let source_ml_source2_f206 = source_ml_source2_f182;
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f183) => {
                {
                    let source_ml_source2_f215 = source_ml_source2_f183;
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f184) => {
                {
                    let source_ml_source2_f224 = source_ml_source2_f184.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f185) => {
                {
                    let units = source_ml_source2_f185;
                    (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64))
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f186, text) => {
                {
                    let source_ml_source2_string_unit_operand238 = text.clone();
                    {
                        let source_ml_source2_string_unit_operand239 = index;
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
                                    let source_ml_source2_string_unit_operand245 = {
                                        let source_ml_source2_string_range_operand240 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand241 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand242 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position243 = source_ml_source2_string_range_operand241;
                                                    {
                                                        let source_ml_source2_string_range_position244 = source_ml_source2_string_range_operand242;
                                                        source_ml_source2_string_range_operand240.slice(source_ml_source2_string_range_position243, source_ml_source2_string_range_position244)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand246 = 0f64;
                                        source_ml_source2_string_unit_operand245.code_point_at(source_ml_source2_string_unit_operand246)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand238.clone(), source_ml_source2_string_unit_operand239)
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f187) => {
                {
                    let source_ml_source2_f252 = source_ml_source2_f187.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f188, source_ml_source2_f189) => {
                {
                    let source_ml_source2_f261 = source_ml_source2_f188.clone();
                    {
                        let source_ml_source2_f262 = source_ml_source2_f189;
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f263) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f264) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f265) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f266) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f267, source_ml_source2_f268) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f269) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f270, source_ml_source2_f271) => {
                    false
                }
            } && match right.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f272) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f273) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f274) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f275) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f276, source_ml_source2_f277) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f278) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f279, source_ml_source2_f280) => {
                    false
                }
            }) {
                {
                    let source_ml_source2_comparison_operand283 = match left.clone() {
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
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value281) => {
                            Some(source_ml_source2_data_property_value281.clone())
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    {
                        let source_ml_source2_comparison_operand284 = match right.clone() {
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
                            self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value282) => {
                                Some(source_ml_source2_data_property_value282.clone())
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<self::ml_string::Text>
                            }
                        };
                        match (source_ml_source2_comparison_operand283.clone(), source_ml_source2_comparison_operand284.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false }
                    }
                }
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s285 = self::source_ml_source1_temporary_test_java_script_semantic_text_equal_loop34(left.clone(), right.clone(), length, at);
                        match source_ml_source2_s285.clone() {
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Done => {
                                true
                            }
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Return(source_ml_source1_temporary_result34) => {
                                source_ml_source1_temporary_result34
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f286) => {
            {
                let source_ml_source2_f310 = source_ml_source2_f286;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f287) => {
            {
                let source_ml_source2_f319 = source_ml_source2_f287;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f288) => {
            {
                let source_ml_source2_f328 = source_ml_source2_f288.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f289) => {
            {
                let units = source_ml_source2_f289;
                {
                    let text = self::ml_string::Text::from_units(vec![]);
                    {
                        let source_ml_source1_temporary_values35 = units;
                        {
                            let source_ml_source1_temporary_index35 = self::ml::Big::from_i128(0);
                            {
                                let text_2 = self::source_ml_source1_temporary_read_java_script_semantic_text_string_loop36(text.clone(), source_ml_source1_temporary_values35, source_ml_source1_temporary_index35.clone());
                                text_2.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f290, text_3) => {
            text_3.clone()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f291) => {
            {
                let source_ml_source2_f347 = source_ml_source2_f291.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f292, source_ml_source2_f293) => {
            {
                let source_ml_source2_f356 = source_ml_source2_f292.clone();
                {
                    let source_ml_source2_f357 = source_ml_source2_f293;
                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
                }
            }
        }
    }
}

pub fn construct_java_script_semantic_text_units(value: self::JavaScriptSemanticValue, limit: f64) -> Vec<f64> {
    {
        let length = self::read_java_script_semantic_text_length(value.clone());
        if ((length < (0f64)) || (length > limit)) {
            (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])).0, message_present: true }).raise()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let at = 0f64;
                            {
                                let units_2 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units, at);
                                units_2
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Null => {
                    {
                        let units_3 = Vec::<f64>::new();
                        {
                            let at_2 = 0f64;
                            {
                                let units_4 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_3, at_2);
                                units_4
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f358) => {
                    {
                        let units_5 = Vec::<f64>::new();
                        {
                            let at_3 = 0f64;
                            {
                                let units_6 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_5, at_3);
                                units_6
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f359) => {
                    {
                        let units_7 = Vec::<f64>::new();
                        {
                            let at_4 = 0f64;
                            {
                                let units_8 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_7, at_4);
                                units_8
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f360) => {
                    {
                        let units_9 = Vec::<f64>::new();
                        {
                            let at_5 = 0f64;
                            {
                                let units_10 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_9, at_5);
                                units_10
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(units_11) => {
                    units_11
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f361, source_ml_source2_f362) => {
                    {
                        let units_12 = Vec::<f64>::new();
                        {
                            let at_6 = 0f64;
                            {
                                let units_13 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_12, at_6);
                                units_13
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f363) => {
                    {
                        let units_14 = Vec::<f64>::new();
                        {
                            let at_7 = 0f64;
                            {
                                let units_15 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_14, at_7);
                                units_15
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f364, source_ml_source2_f365) => {
                    {
                        let units_16 = Vec::<f64>::new();
                        {
                            let at_8 = 0f64;
                            {
                                let units_17 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_16, at_8);
                                units_17
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f366) => {
                {
                    let source_ml_source2_f383 = source_ml_source2_f366;
                    false
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f367) => {
                {
                    let source_ml_source2_f392 = source_ml_source2_f367;
                    false
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f368) => {
                {
                    let source_ml_source2_f401 = source_ml_source2_f368.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f369) => {
                {
                    let source_ml_source2_f410 = source_ml_source2_f369;
                    false
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f370, source_ml_source2_f371) => {
                {
                    let source_ml_source2_f419 = source_ml_source2_f370.clone();
                    {
                        let source_ml_source2_f420 = source_ml_source2_f371.clone();
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f372) => {
                {
                    let source_ml_source2_f429 = source_ml_source2_f372.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f373, source_ml_source2_f374) => {
                {
                    let source_ml_source2_f438 = source_ml_source2_f373.clone();
                    {
                        let callable = source_ml_source2_f374;
                        callable
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_call_number_conversion(operation: self::ml_string::Text, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    if (((operation != (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) && (operation != (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) && (operation != (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99])))) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(f64::NAN)))
            }
            self::JavaScriptSemanticValue::Null => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(0f64)))
            }
            self::JavaScriptSemanticValue::Boolean(value_value) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if value_value {
                    1f64
                } else {
                    0f64
                })))
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f439) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
            }
            self::JavaScriptSemanticValue::Integer(value_value_2) => {
                if (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114]))
                    } else {
                        if ((self::read_java_script_call_integer_width(value_value_2.clone())) < (0f64)) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![101, 120, 112, 108, 105, 99, 105, 116, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::convert_java_script_big_integer_number(value_value_2.clone()))))
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(units) => {
                if (((units.len() as f64)) > (4096f64)) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::convert_java_script_string_number_units(units))))
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f440, text) => {
                if ((text.length()) > (4096f64)) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                } else {
                    self::execute_java_script_call_number_conversion(operation.clone(), self::JavaScriptSemanticValue::Text(self::construct_java_script_semantic_text_units(value.clone(), 4096f64)))
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f441) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114]))
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f442, source_ml_source2_f443) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
            }
        }
    }
}

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

pub fn read_utf16_text_integer(value: f64) -> f64 {
    if value.is_nan() {
        0f64
    } else {
        value.trunc()
    }
}

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

pub fn slice_utf16_text(text: Vec<f64>, start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::read_utf16_text_slice_position(start, (text.len() as f64));
        {
            let last = self::read_utf16_text_slice_position(end, (text.len() as f64));
            {
                let result = Vec::<f64>::new();
                {
                    let at = first;
                    {
                        let result_2 = self::source_ml_source1_temporary_slice_utf16_text_loop38(text, last, result, at);
                        result_2
                    }
                }
            }
        }
    }
}

pub fn substring_utf16_text(text: Vec<f64>, start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(start), 0f64), (text.len() as f64));
        {
            let last = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(end), 0f64), (text.len() as f64));
            self::slice_utf16_text(text, self::ml_math::min(first, last), self::ml_math::max(first, last))
        }
    }
}

pub fn character_utf16_text(text: Vec<f64>, position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]
        }
    }
}

pub fn relative_character_utf16_text(text: Vec<f64>, position: f64) -> Vec<Vec<f64>> {
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
                vec![vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]]
            }
        }
    }
}

pub fn code_point_utf16_text(text: Vec<f64>, position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            {
                let first = (self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(&text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
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

pub fn points_utf16_text(text: Vec<f64>) -> Vec<Vec<f64>> {
    {
        let result = Vec::<Vec<f64>>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_points_utf16_text_loop39(text, result, at);
                result_2
            }
        }
    }
}

pub fn split_utf16_text(text: Vec<f64>, separator: Vec<f64>, limit: f64, missing: bool) -> Vec<Vec<f64>> {
    {
        let count = self::normalize_java_script_unsigned_integer(limit);
        {
            let result = Vec::<Vec<f64>>::new();
            if (count == (0f64)) {
                result
            } else {
                if missing {
                    vec![self::slice_utf16_text(text.clone(), 0f64, (text.len() as f64))]
                } else {
                    if (((separator.len() as f64)) == (0f64)) {
                        {
                            let at = 0f64;
                            {
                                let result_2 = self::source_ml_source1_temporary_split_utf16_text_loop40(text, count, result, at);
                                result_2
                            }
                        }
                    } else {
                        {
                            let start = 0f64;
                            {
                                let source_ml_source2_s444 = self::source_ml_source1_temporary_split_utf16_text_loop41(text.clone(), separator, count, result, start);
                                match source_ml_source2_s444.clone() {
                                    self::MlSource1TemporarySplitUtf16TextLoop41Result::MlSource1TemporarySplitUtf16TextLoop41Done(result_3, start_2) => {
                                        {
                                            let result_4 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text.clone(), start_2, (text.len() as f64))], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                            result_4
                                        }
                                    }
                                    self::MlSource1TemporarySplitUtf16TextLoop41Result::MlSource1TemporarySplitUtf16TextLoop41Return(source_ml_source1_temporary_result41) => {
                                        source_ml_source1_temporary_result41
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn match_utf16_text(text: Vec<f64>, search: Vec<f64>, start: f64) -> bool {
    if ((start < (0f64)) || (((start + (search.len() as f64))) > ((text.len() as f64)))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source2_s445 = self::source_ml_source1_temporary_match_utf16_text_loop42(text, search, start, at);
                match source_ml_source2_s445.clone() {
                    self::MlSource1TemporaryMatchUtf16TextLoop42Result::MlSource1TemporaryMatchUtf16TextLoop42Done => {
                        true
                    }
                    self::MlSource1TemporaryMatchUtf16TextLoop42Result::MlSource1TemporaryMatchUtf16TextLoop42Return(source_ml_source1_temporary_result42) => {
                        source_ml_source1_temporary_result42
                    }
                }
            }
        }
    }
}

pub fn index_utf16_text(text: Vec<f64>, search: Vec<f64>, position: f64) -> f64 {
    {
        let start = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(position), 0f64), (text.len() as f64));
        {
            let at = start;
            {
                let source_ml_source2_s446 = self::source_ml_source1_temporary_index_utf16_text_loop43(text, search, at);
                match source_ml_source2_s446.clone() {
                    self::MlSource1TemporaryIndexUtf16TextLoop43Result::MlSource1TemporaryIndexUtf16TextLoop43Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryIndexUtf16TextLoop43Result::MlSource1TemporaryIndexUtf16TextLoop43Return(source_ml_source1_temporary_result43) => {
                        source_ml_source1_temporary_result43
                    }
                }
            }
        }
    }
}

pub fn last_index_utf16_text(text: Vec<f64>, search: Vec<f64>, position: f64) -> f64 {
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
                    let source_ml_source2_s447 = self::source_ml_source1_temporary_last_index_utf16_text_loop44(text, search, at);
                    match source_ml_source2_s447.clone() {
                        self::MlSource1TemporaryLastIndexUtf16TextLoop44Result::MlSource1TemporaryLastIndexUtf16TextLoop44Done => {
                            (-1f64)
                        }
                        self::MlSource1TemporaryLastIndexUtf16TextLoop44Result::MlSource1TemporaryLastIndexUtf16TextLoop44Return(source_ml_source1_temporary_result44) => {
                            source_ml_source1_temporary_result44
                        }
                    }
                }
            }
        }
    }
}

pub fn compare_utf16_text(left: Vec<f64>, right: Vec<f64>) -> f64 {
    {
        let count = self::ml_math::min((left.len() as f64), (right.len() as f64));
        {
            let at = 0f64;
            {
                let source_ml_source2_s448 = self::source_ml_source1_temporary_compare_utf16_text_loop45(left.clone(), right.clone(), count, at);
                match source_ml_source2_s448.clone() {
                    self::MlSource1TemporaryCompareUtf16TextLoop45Result::MlSource1TemporaryCompareUtf16TextLoop45Done => {
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
                    self::MlSource1TemporaryCompareUtf16TextLoop45Result::MlSource1TemporaryCompareUtf16TextLoop45Return(source_ml_source1_temporary_result45) => {
                        source_ml_source1_temporary_result45
                    }
                }
            }
        }
    }
}

pub fn starts_utf16_text(text: Vec<f64>, search: Vec<f64>) -> bool {
    self::match_utf16_text(text, search, 0f64)
}

pub fn ends_utf16_text(text: Vec<f64>, search: Vec<f64>) -> bool {
    self::match_utf16_text(text.clone(), search.clone(), ((text.len() as f64) - (search.len() as f64)))
}

pub fn includes_utf16_text(text: Vec<f64>, search: Vec<f64>) -> bool {
    ((self::index_utf16_text(text, search, 0f64)) >= (0f64))
}

pub fn join_utf16_text(values: Vec<Vec<f64>>, separator: Vec<f64>) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_join_utf16_text_loop46(values, separator, result, at);
                result_2
            }
        }
    }
}

pub fn trim_utf16_text(text: Vec<f64>, start: bool, end: bool) -> Vec<f64> {
    {
        let first = 0f64;
        {
            let last = (text.len() as f64);
            {
                let first_3 = if start {
                    {
                        let first_2 = self::source_ml_source1_temporary_trim_utf16_text_loop49(text.clone(), first, last);
                        first_2
                    }
                } else {
                    first
                };
                {
                    let last_3 = if end {
                        {
                            let last_2 = self::source_ml_source1_temporary_trim_utf16_text_loop50(text.clone(), first_3, last);
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

pub fn character_units_utf16_text(values: Vec<f64>) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_character_units_utf16_text_loop51(values, result, at);
                result_2
            }
        }
    }
}

pub fn code_point_units_utf16_text(values: Vec<f64>) -> Vec<f64> {
    {
        let result = vec![(-1f64)];
        {
            let at = 0f64;
            {
                let source_ml_source2_s449 = self::source_ml_source1_temporary_code_point_units_utf16_text_loop52(values, result, at);
                match source_ml_source2_s449.clone() {
                    self::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Done(result_2) => {
                        result_2
                    }
                    self::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Return(source_ml_source1_temporary_result52) => {
                        source_ml_source1_temporary_result52
                    }
                }
            }
        }
    }
}

pub fn read_utf8_text_units(bytes: Vec<f64>) -> Vec<f64> {
    self::read_java_script_utf8_string_units(bytes)
}

pub fn display_points_utf16_text(text: Vec<f64>) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_display_points_utf16_text_loop53(text, result, at);
                result_2
            }
        }
    }
}

pub fn display_utf8_from_utf16_text(text: Vec<f64>) -> Vec<f64> {
    {
        let bytes = Vec::<f64>::new();
        {
            let points = self::display_points_utf16_text(text);
            {
                let at = 0f64;
                {
                    let bytes_2 = self::source_ml_source1_temporary_display_utf8_from_utf16_text_loop55(bytes, points, at);
                    bytes_2
                }
            }
        }
    }
}

pub fn plan_utf16_text_substitution(replacement: Vec<f64>) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let start = 0f64;
            {
                let at = 0f64;
                {
                    let source_ml_source2_s450 = self::source_ml_source1_temporary_plan_utf16_text_substitution_loop56(replacement.clone(), result, start, at);
                    match source_ml_source2_s450.clone() {
                        self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Result::MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Done(result_2, start_2) => {
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

pub fn replace_utf16_text(text: Vec<f64>, search: Vec<f64>, replacement: Vec<f64>, all: bool) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let plan = self::plan_utf16_text_substitution(replacement.clone());
            {
                let start = 0f64;
                {
                    let from = 0f64;
                    {
                        let searching = true;
                        {
                            let source_ml_source2_s451 = self::source_ml_source1_temporary_replace_utf16_text_loop57(text.clone(), search, replacement, all, result, plan, start, from, searching);
                            match source_ml_source2_s451.clone() {
                                self::MlSource1TemporaryReplaceUtf16TextLoop57Result::MlSource1TemporaryReplaceUtf16TextLoop57Done(result_2, start_2) => {
                                    {
                                        let at = start_2;
                                        {
                                            let result_3 = self::source_ml_source1_temporary_replace_utf16_text_loop58(text, result_2, at);
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

pub fn test_java_script_primitive_method_operation(operation: self::ml_string::Text) -> bool {
    self::test_java_script_standard_primitive_method_operation(operation.clone())
}

pub fn execute_java_script_primitive_method(receiver: self::JavaScriptSemanticValue, operation: self::ml_string::Text, args: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    if !(self::test_java_script_primitive_method_operation(operation.clone())) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114]))
    } else {
        if (((args.len() as f64)) > (4096f64)) {
            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 109, 101, 116, 104, 111, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 97, 108, 108, 32, 98, 117, 100, 103, 101, 116]))
        } else {
            if (match receiver.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    true
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f452) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f453) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f454) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f455) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f456, source_ml_source2_f457) => {
                    false
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f458) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f459, source_ml_source2_f460) => {
                    false
                }
            } || match receiver.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    true
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f461) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f462) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f463) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f464) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f465, source_ml_source2_f466) => {
                    false
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f467) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f468, source_ml_source2_f469) => {
                    false
                }
            }) {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104, 32, 99, 97, 108, 108, 101, 100, 32, 111, 110, 32, 110, 117, 108, 108, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
            } else {
                match receiver.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        {
                            let receiver_error = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error.clone())
                            } else {
                                {
                                    let text = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                                            let value = *value;
                                            self::execute_java_script_primitive_prefix_search(value.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f479) => {
                                            text.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f480, source_ml_source2_f481) => {
                                            text.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f482) => {
                                            text.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Null => {
                        {
                            let receiver_error_2 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_2.clone())
                            } else {
                                {
                                    let text_2 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_2.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_2) => {
                                            let value_2 = *value_2;
                                            self::execute_java_script_primitive_prefix_search(value_2.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f483) => {
                                            text_2.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f484, source_ml_source2_f485) => {
                                            text_2.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f486) => {
                                            text_2.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f470) => {
                        {
                            let receiver_error_3 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_3.clone())
                            } else {
                                {
                                    let text_3 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_3.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_3) => {
                                            let value_3 = *value_3;
                                            self::execute_java_script_primitive_prefix_search(value_3.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f487) => {
                                            text_3.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f488, source_ml_source2_f489) => {
                                            text_3.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f490) => {
                                            text_3.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f471) => {
                        {
                            let receiver_error_4 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_4 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_4.clone())
                            } else {
                                {
                                    let text_4 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_4.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_4) => {
                                            let value_4 = *value_4;
                                            self::execute_java_script_primitive_prefix_search(value_4.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f491) => {
                                            text_4.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f492, source_ml_source2_f493) => {
                                            text_4.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f494) => {
                                            text_4.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f472) => {
                        {
                            let receiver_error_5 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_5 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_5.clone())
                            } else {
                                {
                                    let text_5 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_5.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_5) => {
                                            let value_5 = *value_5;
                                            self::execute_java_script_primitive_prefix_search(value_5.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f495) => {
                                            text_5.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f496, source_ml_source2_f497) => {
                                            text_5.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f498) => {
                                            text_5.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f473) => {
                        {
                            let receiver_error_6 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_6 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_6.clone())
                            } else {
                                {
                                    let text_6 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_6.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_6) => {
                                            let value_6 = *value_6;
                                            self::execute_java_script_primitive_prefix_search(value_6.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f499) => {
                                            text_6.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f500, source_ml_source2_f501) => {
                                            text_6.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f502) => {
                                            text_6.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f474, source_ml_source2_f475) => {
                        {
                            let receiver_error_7 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_7 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_7.clone())
                            } else {
                                {
                                    let text_7 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_7.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_7) => {
                                            let value_7 = *value_7;
                                            self::execute_java_script_primitive_prefix_search(value_7.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f503) => {
                                            text_7.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f504, source_ml_source2_f505) => {
                                            text_7.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f506) => {
                                            text_7.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f476) => {
                        {
                            let receiver_error_8 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_8 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_8.clone())
                            } else {
                                {
                                    let text_8 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_8.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_8) => {
                                            let value_8 = *value_8;
                                            self::execute_java_script_primitive_prefix_search(value_8.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f507) => {
                                            text_8.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f508, source_ml_source2_f509) => {
                                            text_8.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f510) => {
                                            text_8.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f477, source_ml_source2_f478) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 83, 116, 114, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_primitive_prefix_search(receiver_text: self::JavaScriptSemanticValue, args: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let search = (self::ml_array::optional_at(&args, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        match search.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                {
                    let search_error = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error.clone())
                    } else {
                        {
                            let converted = self::execute_java_script_call_text_conversion(search.clone());
                            match converted.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                                    let value = *value;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f520) => {
                                    converted.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f521, source_ml_source2_f522) => {
                                    converted.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f523) => {
                                    converted.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Null => {
                {
                    let search_error_2 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_2 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_2.clone())
                    } else {
                        {
                            let converted_2 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_2.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_2) => {
                                    let value_2 = *value_2;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_2.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f524) => {
                                    converted_2.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f525, source_ml_source2_f526) => {
                                    converted_2.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f527) => {
                                    converted_2.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f511) => {
                {
                    let search_error_3 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_3 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_3.clone())
                    } else {
                        {
                            let converted_3 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_3.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_3) => {
                                    let value_3 = *value_3;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_3.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f528) => {
                                    converted_3.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f529, source_ml_source2_f530) => {
                                    converted_3.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f531) => {
                                    converted_3.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f512) => {
                {
                    let search_error_4 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_4 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_4.clone())
                    } else {
                        {
                            let converted_4 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_4.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_4) => {
                                    let value_4 = *value_4;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_4.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f532) => {
                                    converted_4.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f533, source_ml_source2_f534) => {
                                    converted_4.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f535) => {
                                    converted_4.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f513) => {
                {
                    let search_error_5 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_5 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_5.clone())
                    } else {
                        {
                            let converted_5 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_5.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_5) => {
                                    let value_5 = *value_5;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_5.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f536) => {
                                    converted_5.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f537, source_ml_source2_f538) => {
                                    converted_5.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f539) => {
                                    converted_5.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f514) => {
                {
                    let search_error_6 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_6 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_6.clone())
                    } else {
                        {
                            let converted_6 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_6.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_6) => {
                                    let value_6 = *value_6;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_6.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f540) => {
                                    converted_6.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f541, source_ml_source2_f542) => {
                                    converted_6.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f543) => {
                                    converted_6.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f515, source_ml_source2_f516) => {
                {
                    let search_error_7 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_7 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_7.clone())
                    } else {
                        {
                            let converted_7 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_7.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_7) => {
                                    let value_7 = *value_7;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_7.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f544) => {
                                    converted_7.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f545, source_ml_source2_f546) => {
                                    converted_7.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f547) => {
                                    converted_7.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f517) => {
                {
                    let search_error_8 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_8 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_8.clone())
                    } else {
                        {
                            let converted_8 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_8.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_8) => {
                                    let value_8 = *value_8;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_8.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f548) => {
                                    converted_8.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f549, source_ml_source2_f550) => {
                                    converted_8.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f551) => {
                                    converted_8.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f518, source_ml_source2_f519) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 73, 115, 82, 101, 103, 69, 120, 112, 32, 97, 110, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108, 115]))
            }
        }
    }
}

pub fn execute_java_script_primitive_prefix_position(receiver_text: self::JavaScriptSemanticValue, search_text: self::JavaScriptSemanticValue, position: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    match position.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            {
                let position_error = self::read_java_script_semantic_value_error(position.clone());
                if (position_error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error.clone())
                } else {
                    {
                        let number = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value) => {
                                let position_value = *position_value;
                                match position_value.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f565) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value) => {
                                        {
                                            let length = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start = self::ml_math::min(length, self::ml_math::max(0f64, self::read_utf16_text_integer(value)));
                                                    if (search_length > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches = (((start + search_length)) <= length);
                                                            {
                                                                let at = 0f64;
                                                                {
                                                                    let matches_2 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length, start, matches, at);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_2)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f566) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f567) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f568, source_ml_source2_f569) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f570) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f571, source_ml_source2_f572) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f561) => {
                                number.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f562, source_ml_source2_f563) => {
                                number.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f564) => {
                                number.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Null => {
            {
                let position_error_2 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_2 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_2.clone())
                } else {
                    {
                        let number_2 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_2.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_2) => {
                                let position_value_2 = *position_value_2;
                                match position_value_2.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f577) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_2) => {
                                        {
                                            let length_2 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_2 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_2 = self::ml_math::min(length_2, self::ml_math::max(0f64, self::read_utf16_text_integer(value_2)));
                                                    if (search_length_2 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_3 = (((start_2 + search_length_2)) <= length_2);
                                                            {
                                                                let at_2 = 0f64;
                                                                {
                                                                    let matches_4 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_2, start_2, matches_3, at_2);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_4)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f578) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f579) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f580, source_ml_source2_f581) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f582) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f583, source_ml_source2_f584) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f573) => {
                                number_2.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f574, source_ml_source2_f575) => {
                                number_2.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f576) => {
                                number_2.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f552) => {
            {
                let position_error_3 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_3 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_3.clone())
                } else {
                    {
                        let number_3 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_3.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_3) => {
                                let position_value_3 = *position_value_3;
                                match position_value_3.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f589) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_3) => {
                                        {
                                            let length_3 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_3 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_3 = self::ml_math::min(length_3, self::ml_math::max(0f64, self::read_utf16_text_integer(value_3)));
                                                    if (search_length_3 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_5 = (((start_3 + search_length_3)) <= length_3);
                                                            {
                                                                let at_3 = 0f64;
                                                                {
                                                                    let matches_6 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_3, start_3, matches_5, at_3);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_6)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f590) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f591) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f592, source_ml_source2_f593) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f594) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f595, source_ml_source2_f596) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f585) => {
                                number_3.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f586, source_ml_source2_f587) => {
                                number_3.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f588) => {
                                number_3.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f553) => {
            {
                let position_error_4 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_4 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_4.clone())
                } else {
                    {
                        let number_4 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_4.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_4) => {
                                let position_value_4 = *position_value_4;
                                match position_value_4.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f601) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_4) => {
                                        {
                                            let length_4 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_4 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_4 = self::ml_math::min(length_4, self::ml_math::max(0f64, self::read_utf16_text_integer(value_4)));
                                                    if (search_length_4 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_7 = (((start_4 + search_length_4)) <= length_4);
                                                            {
                                                                let at_4 = 0f64;
                                                                {
                                                                    let matches_8 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_4, start_4, matches_7, at_4);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_8)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f602) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f603) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f604, source_ml_source2_f605) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f606) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f607, source_ml_source2_f608) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f597) => {
                                number_4.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f598, source_ml_source2_f599) => {
                                number_4.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f600) => {
                                number_4.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f554) => {
            {
                let position_error_5 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_5 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_5.clone())
                } else {
                    {
                        let number_5 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_5.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_5) => {
                                let position_value_5 = *position_value_5;
                                match position_value_5.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f613) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_5) => {
                                        {
                                            let length_5 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_5 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_5 = self::ml_math::min(length_5, self::ml_math::max(0f64, self::read_utf16_text_integer(value_5)));
                                                    if (search_length_5 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_9 = (((start_5 + search_length_5)) <= length_5);
                                                            {
                                                                let at_5 = 0f64;
                                                                {
                                                                    let matches_10 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_5, start_5, matches_9, at_5);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_10)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f614) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f615) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f616, source_ml_source2_f617) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f618) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f619, source_ml_source2_f620) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f609) => {
                                number_5.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f610, source_ml_source2_f611) => {
                                number_5.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f612) => {
                                number_5.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f555) => {
            {
                let position_error_6 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_6 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_6.clone())
                } else {
                    {
                        let number_6 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_6.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_6) => {
                                let position_value_6 = *position_value_6;
                                match position_value_6.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f625) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_6) => {
                                        {
                                            let length_6 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_6 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_6 = self::ml_math::min(length_6, self::ml_math::max(0f64, self::read_utf16_text_integer(value_6)));
                                                    if (search_length_6 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_11 = (((start_6 + search_length_6)) <= length_6);
                                                            {
                                                                let at_6 = 0f64;
                                                                {
                                                                    let matches_12 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_6, start_6, matches_11, at_6);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_12)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f626) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f627) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f628, source_ml_source2_f629) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f630) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f631, source_ml_source2_f632) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f621) => {
                                number_6.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f622, source_ml_source2_f623) => {
                                number_6.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f624) => {
                                number_6.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f556, source_ml_source2_f557) => {
            {
                let position_error_7 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_7 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_7.clone())
                } else {
                    {
                        let number_7 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_7.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_7) => {
                                let position_value_7 = *position_value_7;
                                match position_value_7.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f637) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_7) => {
                                        {
                                            let length_7 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_7 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_7 = self::ml_math::min(length_7, self::ml_math::max(0f64, self::read_utf16_text_integer(value_7)));
                                                    if (search_length_7 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_13 = (((start_7 + search_length_7)) <= length_7);
                                                            {
                                                                let at_7 = 0f64;
                                                                {
                                                                    let matches_14 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_7, start_7, matches_13, at_7);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_14)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f638) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f639) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f640, source_ml_source2_f641) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f642) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f643, source_ml_source2_f644) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f633) => {
                                number_7.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f634, source_ml_source2_f635) => {
                                number_7.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f636) => {
                                number_7.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f558) => {
            {
                let position_error_8 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_8 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_8.clone())
                } else {
                    {
                        let number_8 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_8.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_8) => {
                                let position_value_8 = *position_value_8;
                                match position_value_8.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f649) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_8) => {
                                        {
                                            let length_8 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_8 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_8 = self::ml_math::min(length_8, self::ml_math::max(0f64, self::read_utf16_text_integer(value_8)));
                                                    if (search_length_8 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_15 = (((start_8 + search_length_8)) <= length_8);
                                                            {
                                                                let at_8 = 0f64;
                                                                {
                                                                    let matches_16 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_8, start_8, matches_15, at_8);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_16)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f650) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f651) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f652, source_ml_source2_f653) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f654) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f655, source_ml_source2_f656) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f645) => {
                                number_8.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f646, source_ml_source2_f647) => {
                                number_8.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f648) => {
                                number_8.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f559, source_ml_source2_f560) => {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Result {
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Done(f64),
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_call_integer_width_loop1(mut rest: self::ml::Big, mut width: f64) -> self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Result {
    loop {
        return if (rest > (self::ml::Big::from_i128(0))) {
            if (width >= (4096f64)) {
                self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Return((-1f64))
            } else {
                {
                    let rest_2 = self::ml::divide(&rest, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let width_2 = (width + 1f64);
                        {
                            (rest, width) = (rest_2.clone(), width_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Done(width)
        };
    }
}

pub fn source_ml_source1_temporary_execute_java_script_call_text_conversion_loop3(mut text: self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (text.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source2_string_unit_operand657 = text.clone();
                    {
                        let source_ml_source2_string_unit_operand658 = index;
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
                                    let source_ml_source2_string_unit_operand664 = {
                                        let source_ml_source2_string_range_operand659 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand660 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand661 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position662 = source_ml_source2_string_range_operand660;
                                                    {
                                                        let source_ml_source2_string_range_position663 = source_ml_source2_string_range_operand661;
                                                        source_ml_source2_string_range_operand659.slice(source_ml_source2_string_range_position662, source_ml_source2_string_range_position663)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand665 = 0f64;
                                        source_ml_source2_string_unit_operand664.code_point_at(source_ml_source2_string_unit_operand665)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand657.clone(), source_ml_source2_string_unit_operand658)
                    }
                }], units); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (units, index) = (units_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            units
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2 {
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Next(self::ml_string::Text),
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Return(Box<self::JavaScriptCallPrimitiveResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Done(Vec<f64>),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Done(f64, f64),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop7(mut bytes: Vec<f64>, mut index: f64, mut scalar: f64, mut remaining: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Result {
    loop {
        return if (remaining > (0f64)) {
            {
                let next = (self::ml_array::optional_at(&bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((next < (128f64)) || (next > (191f64))) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Return(vec![(-1f64)])
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
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Done(index, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6 {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Next(f64, f64, f64),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop5(mut bytes: Vec<f64>, mut units: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let scalar = (self::ml_array::optional_at(&bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                {
                    let index_2 = (index + 1f64);
                    {
                        let remaining = 0f64;
                        {
                            let minimum = 0f64;
                            {
                                let source_ml_source2_s666 = if ((scalar >= (194f64)) && (scalar <= (223f64))) {
                                    {
                                        let scalar_2 = (scalar - 192f64);
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar_2, remaining_2, minimum_2)
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
                                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar_3, remaining_3, minimum_3)
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
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar_4, remaining_4, minimum_4)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (scalar >= (128f64)) {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Return(vec![(-1f64)])
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar, remaining, minimum)
                                            }
                                        }
                                    }
                                };
                                match source_ml_source2_s666.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source2_s667 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop7(bytes.clone(), index_2, scalar_5, remaining_5);
                                            match source_ml_source2_s667.clone() {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Done(index_3, scalar_6) => {
                                                    if (((scalar_6 < minimum_5) || (scalar_6 > (1114111f64))) || ((scalar_6 >= (55296f64)) && (scalar_6 <= (57343f64)))) {
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Return(vec![(-1f64)])
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
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Return(source_ml_source1_temporary_result7) => {
                                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Return(source_ml_source1_temporary_result7)
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Return(source_ml_source1_temporary_result7_2) => {
                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Return(source_ml_source1_temporary_result7_2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Done(units)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done,
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(mut bytes: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(&bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (255f64))) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(vec![(-1f64)])
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
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done
        };
    }
}

pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop8(mut exponent: f64, mut value: self::ml::Big, mut base: self::ml::Big) -> self::ml::Big {
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

pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop9(mut value: self::ml::Big, mut result: f64, mut scale: f64) -> f64 {
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

pub fn source_ml_source1_temporary_read_string_number_binary_power_loop11(mut exponent: f64, mut result: f64) -> f64 {
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
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop10Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop10Done(f64, f64),
}

pub fn source_ml_source1_temporary_read_string_number_binary_power_loop10(mut exponent: f64, mut result: f64) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop10Result {
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
            self::MlSource1TemporaryReadStringNumberBinaryPowerLoop10Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop10Done(exponent, result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop15Result {
    MlSource1TemporaryConvertStringNumberRationalLoop15Done(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop15(mut quotient: self::ml::Big, mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop15Result {
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
            self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop14Result {
    MlSource1TemporaryConvertStringNumberRationalLoop14Done(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop14(mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop14Result {
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
            self::MlSource1TemporaryConvertStringNumberRationalLoop14Result::MlSource1TemporaryConvertStringNumberRationalLoop14Done(scale.clone(), weight.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin13 {
    MlSource1TemporaryConvertStringNumberRationalJoin13Next(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop16(mut numerator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
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

pub fn source_ml_source1_temporary_convert_string_number_rational_loop17(mut denominator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
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
pub enum MlSource1TemporaryConvertStringNumberRationalJoin12 {
    MlSource1TemporaryConvertStringNumberRationalJoin12Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin12Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop18Result {
    MlSource1TemporaryConvertStringNumberRationalLoop18Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop18Return(f64),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop18(mut numerator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop18Result {
    loop {
        return if (numerator >= (scale.mul(&self::ml::Big::from_i128(2)))) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent + 1f64);
                    if (exponent_2 >= (1024f64)) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop18Result::MlSource1TemporaryConvertStringNumberRationalLoop18Return(f64::INFINITY)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop18Result::MlSource1TemporaryConvertStringNumberRationalLoop18Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop19Result {
    MlSource1TemporaryConvertStringNumberRationalLoop19Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop19Return(f64),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop19(mut denominator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop19Result {
    loop {
        return if (scale < denominator) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent - 1f64);
                    if (exponent_2 < ((-1075f64))) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop19Result::MlSource1TemporaryConvertStringNumberRationalLoop19Return(0f64)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop19Result::MlSource1TemporaryConvertStringNumberRationalLoop19Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptStringNumberInfinityLoop20Result {
    MlSource1TemporaryAcceptStringNumberInfinityLoop20Done,
    MlSource1TemporaryAcceptStringNumberInfinityLoop20Return(bool),
}

pub fn source_ml_source1_temporary_accept_string_number_infinity_loop20(mut units: Vec<f64>, mut start: f64, mut expected: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryAcceptStringNumberInfinityLoop20Result {
    loop {
        return if (index < ((expected.len() as f64))) {
            if (((self::ml_array::optional_at(&units, self::ml_array::number_index((start + index)))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(&expected, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-2f64)))) {
                self::MlSource1TemporaryAcceptStringNumberInfinityLoop20Result::MlSource1TemporaryAcceptStringNumberInfinityLoop20Return(false)
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
            self::MlSource1TemporaryAcceptStringNumberInfinityLoop20Result::MlSource1TemporaryAcceptStringNumberInfinityLoop20Done
        };
    }
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop22(mut units: Vec<f64>, mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((start < end) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(&units, self::ml_array::number_index(start))).clone().unwrap_or_else(|| (-1f64)))) {
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

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop23(mut units: Vec<f64>, mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((end > start) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(&units, self::ml_array::number_index((end - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
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

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop29(mut units: Vec<f64>, mut integer_digits: Vec<self::ml::Big>, mut mantissa_end: f64, mut mantissa: self::ml::Big, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < mantissa_end) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
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
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(f64, f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop28(mut units: Vec<f64>, mut start: f64, mut exponent: f64, mut mantissa_end: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result {
    loop {
        return if (mantissa_end > start) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index((mantissa_end - 1f64)))).clone().unwrap_or_else(|| (-1f64));
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
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(exponent, mantissa_end)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(exponent, mantissa_end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(f64, f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Done(f64, f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop31(mut units: Vec<f64>, mut end: f64, mut at: f64, mut exponent: f64, mut exponent_limit: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Result {
    loop {
        return if (at < end) {
            {
                let digit = ((self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)) - 48f64);
                if ((digit < (0f64)) || (digit > (9f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Done(at, exponent)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Done(at, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30Next(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Done(f64, f64, f64, f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop26(mut units: Vec<f64>, mut end: f64, mut at: f64, mut fraction: f64, mut digits: f64, mut significant: f64, mut dot: bool) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Result {
    loop {
        return if (at < end) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
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
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Done(at, fraction, digits, significant)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Done(at, fraction, digits, significant)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Next(f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Done(self::ml::Big, bool),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Return(f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop32(mut units: Vec<f64>, mut end: f64, mut integer_digits: Vec<self::ml::Big>, mut radix: f64, mut value: self::ml::Big, mut overflow: bool, mut limit: self::ml::Big, mut base: self::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_string_number_digit((self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Done(value.clone(), overflow)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done,
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Return(f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop21(mut units: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result {
    loop {
        return if (index < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop33(mut units: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Result {
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Done,
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Return(bool),
}

pub fn source_ml_source1_temporary_test_java_script_semantic_text_equal_loop34(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut at: f64) -> self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Result {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f668) => {
                        {
                            let source_ml_source2_f692 = source_ml_source2_f668;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f669) => {
                        {
                            let source_ml_source2_f701 = source_ml_source2_f669;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f670) => {
                        {
                            let source_ml_source2_f710 = source_ml_source2_f670.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Text(units) => {
                        (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f671, source_ml_source2_f672) => {
                        {
                            let source_ml_source2_f720 = source_ml_source2_f671.clone();
                            {
                                let text = source_ml_source2_f672.clone();
                                {
                                    let source_ml_source2_string_unit_operand724 = text.clone();
                                    {
                                        let source_ml_source2_string_unit_operand725 = at;
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
                                                    let source_ml_source2_string_unit_operand731 = {
                                                        let source_ml_source2_string_range_operand726 = text_2.clone();
                                                        {
                                                            let source_ml_source2_string_range_operand727 = index;
                                                            {
                                                                let source_ml_source2_string_range_operand728 = (index + 1f64);
                                                                {
                                                                    let source_ml_source2_string_range_position729 = source_ml_source2_string_range_operand727;
                                                                    {
                                                                        let source_ml_source2_string_range_position730 = source_ml_source2_string_range_operand728;
                                                                        source_ml_source2_string_range_operand726.slice(source_ml_source2_string_range_position729, source_ml_source2_string_range_position730)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_string_unit_operand732 = 0f64;
                                                        source_ml_source2_string_unit_operand731.code_point_at(source_ml_source2_string_unit_operand732)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source2_string_unit_operand724.clone(), source_ml_source2_string_unit_operand725)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f673) => {
                        {
                            let source_ml_source2_f738 = source_ml_source2_f673.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f674, source_ml_source2_f675) => {
                        {
                            let source_ml_source2_f747 = source_ml_source2_f674.clone();
                            {
                                let source_ml_source2_f748 = source_ml_source2_f675;
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f749) => {
                            {
                                let source_ml_source2_f773 = source_ml_source2_f749;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f750) => {
                            {
                                let source_ml_source2_f782 = source_ml_source2_f750;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f751) => {
                            {
                                let source_ml_source2_f791 = source_ml_source2_f751.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units_2) => {
                            (self::ml_array::optional_at(&units_2, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f752, source_ml_source2_f753) => {
                            {
                                let source_ml_source2_f801 = source_ml_source2_f752.clone();
                                {
                                    let text_2 = source_ml_source2_f753.clone();
                                    {
                                        let source_ml_source2_string_unit_operand805 = text_2.clone();
                                        {
                                            let source_ml_source2_string_unit_operand806 = at;
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
                                                        let source_ml_source2_string_unit_operand812 = {
                                                            let source_ml_source2_string_range_operand807 = text_3.clone();
                                                            {
                                                                let source_ml_source2_string_range_operand808 = index;
                                                                {
                                                                    let source_ml_source2_string_range_operand809 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source2_string_range_position810 = source_ml_source2_string_range_operand808;
                                                                        {
                                                                            let source_ml_source2_string_range_position811 = source_ml_source2_string_range_operand809;
                                                                            source_ml_source2_string_range_operand807.slice(source_ml_source2_string_range_position810, source_ml_source2_string_range_position811)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_string_unit_operand813 = 0f64;
                                                            source_ml_source2_string_unit_operand812.code_point_at(source_ml_source2_string_unit_operand813)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source2_string_unit_operand805.clone(), source_ml_source2_string_unit_operand806)
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f754) => {
                            {
                                let source_ml_source2_f819 = source_ml_source2_f754.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f755, source_ml_source2_f756) => {
                            {
                                let source_ml_source2_f828 = source_ml_source2_f755.clone();
                                {
                                    let source_ml_source2_f829 = source_ml_source2_f756;
                                    (-1f64)
                                }
                            }
                        }
                    };
                    if (a != b) {
                        self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Return(false)
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
            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_semantic_text_string_loop36(mut text: self::ml_string::Text, mut source_ml_source1_temporary_values35: Vec<f64>, mut source_ml_source1_temporary_index35: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_source1_temporary_index35 < (self::ml::Big::from_u128(source_ml_source1_temporary_values35.len() as u128))) {
            {
                let unit = self::ml_array::at(&source_ml_source1_temporary_values35, source_ml_source1_temporary_index35.to_index());
                {
                    let text_2 = text.concat(&{
                        let source_ml_source2_character_argument830 = unit;
                        self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source2_character_argument830]).into_iter().map(|ml_unit| ml_unit as u16).collect())
                    });
                    {
                        let source_ml_source1_temporary_index35_2 = source_ml_source1_temporary_index35.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_source1_temporary_index35) = (text_2.clone(), source_ml_source1_temporary_index35_2.clone());
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

pub fn source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(mut value: self::JavaScriptSemanticValue, mut length: f64, mut units: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < length) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_java_script_semantic_text_unit(value.clone(), at)], units); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (units, at) = (units_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            units
        };
    }
}

pub fn source_ml_source1_temporary_slice_utf16_text_loop38(mut text: Vec<f64>, mut last: f64, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
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

pub fn source_ml_source1_temporary_points_utf16_text_loop39(mut text: Vec<f64>, mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let first = (self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(&text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
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

pub fn source_ml_source1_temporary_split_utf16_text_loop40(mut text: Vec<f64>, mut count: f64, mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if ((at < ((text.len() as f64))) && (((result.len() as f64)) < count)) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]], result); self::ml_array::append(ml_append_left, ml_append_right) };
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
pub enum MlSource1TemporarySplitUtf16TextLoop41Result {
    MlSource1TemporarySplitUtf16TextLoop41Done(Vec<Vec<f64>>, f64),
    MlSource1TemporarySplitUtf16TextLoop41Return(Vec<Vec<f64>>),
}

pub fn source_ml_source1_temporary_split_utf16_text_loop41(mut text: Vec<f64>, mut separator: Vec<f64>, mut count: f64, mut result: Vec<Vec<f64>>, mut start: f64) -> self::MlSource1TemporarySplitUtf16TextLoop41Result {
    loop {
        return {
            let found = self::index_utf16_text(text.clone(), separator.clone(), start);
            if (found < (0f64)) {
                self::MlSource1TemporarySplitUtf16TextLoop41Result::MlSource1TemporarySplitUtf16TextLoop41Done(result, start)
            } else {
                {
                    let result_2 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text.clone(), start, found)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                    if (((result_2.len() as f64)) == count) {
                        self::MlSource1TemporarySplitUtf16TextLoop41Result::MlSource1TemporarySplitUtf16TextLoop41Return(result_2)
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
pub enum MlSource1TemporaryMatchUtf16TextLoop42Result {
    MlSource1TemporaryMatchUtf16TextLoop42Done,
    MlSource1TemporaryMatchUtf16TextLoop42Return(bool),
}

pub fn source_ml_source1_temporary_match_utf16_text_loop42(mut text: Vec<f64>, mut search: Vec<f64>, mut start: f64, mut at: f64) -> self::MlSource1TemporaryMatchUtf16TextLoop42Result {
    loop {
        return if (at < ((search.len() as f64))) {
            if {
                let source_ml_source2_comparison_operand831 = self::ml_array::optional_at(&text, self::ml_array::number_index((start + at)));
                {
                    let source_ml_source2_comparison_operand832 = self::ml_array::optional_at(&search, self::ml_array::number_index(at));
                    !(match (source_ml_source2_comparison_operand831.clone(), source_ml_source2_comparison_operand832.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlSource1TemporaryMatchUtf16TextLoop42Result::MlSource1TemporaryMatchUtf16TextLoop42Return(false)
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
            self::MlSource1TemporaryMatchUtf16TextLoop42Result::MlSource1TemporaryMatchUtf16TextLoop42Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryIndexUtf16TextLoop43Result {
    MlSource1TemporaryIndexUtf16TextLoop43Done,
    MlSource1TemporaryIndexUtf16TextLoop43Return(f64),
}

pub fn source_ml_source1_temporary_index_utf16_text_loop43(mut text: Vec<f64>, mut search: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryIndexUtf16TextLoop43Result {
    loop {
        return if (at <= (((text.len() as f64) - (search.len() as f64)))) {
            if self::match_utf16_text(text.clone(), search.clone(), at) {
                self::MlSource1TemporaryIndexUtf16TextLoop43Result::MlSource1TemporaryIndexUtf16TextLoop43Return(at)
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
            self::MlSource1TemporaryIndexUtf16TextLoop43Result::MlSource1TemporaryIndexUtf16TextLoop43Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLastIndexUtf16TextLoop44Result {
    MlSource1TemporaryLastIndexUtf16TextLoop44Done,
    MlSource1TemporaryLastIndexUtf16TextLoop44Return(f64),
}

pub fn source_ml_source1_temporary_last_index_utf16_text_loop44(mut text: Vec<f64>, mut search: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryLastIndexUtf16TextLoop44Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text.clone(), search.clone(), at) {
                self::MlSource1TemporaryLastIndexUtf16TextLoop44Result::MlSource1TemporaryLastIndexUtf16TextLoop44Return(at)
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
            self::MlSource1TemporaryLastIndexUtf16TextLoop44Result::MlSource1TemporaryLastIndexUtf16TextLoop44Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompareUtf16TextLoop45Result {
    MlSource1TemporaryCompareUtf16TextLoop45Done,
    MlSource1TemporaryCompareUtf16TextLoop45Return(f64),
}

pub fn source_ml_source1_temporary_compare_utf16_text_loop45(mut left: Vec<f64>, mut right: Vec<f64>, mut count: f64, mut at: f64) -> self::MlSource1TemporaryCompareUtf16TextLoop45Result {
    loop {
        return if (at < count) {
            {
                let a = (self::ml_array::optional_at(&left, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let b = (self::ml_array::optional_at(&right, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlSource1TemporaryCompareUtf16TextLoop45Result::MlSource1TemporaryCompareUtf16TextLoop45Return(if (a < b) {
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
            self::MlSource1TemporaryCompareUtf16TextLoop45Result::MlSource1TemporaryCompareUtf16TextLoop45Done
        };
    }
}

pub fn source_ml_source1_temporary_join_utf16_text_loop47(mut separator: Vec<f64>, mut result: Vec<f64>, mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((separator.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(&separator, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
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

pub fn source_ml_source1_temporary_join_utf16_text_loop48(mut result: Vec<f64>, mut value: Vec<f64>, mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((value.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(&value, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
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

pub fn source_ml_source1_temporary_join_utf16_text_loop46(mut values: Vec<Vec<f64>>, mut separator: Vec<f64>, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let result_3 = if (at != (0f64)) {
                    {
                        let offset = 0f64;
                        {
                            let result_2 = self::source_ml_source1_temporary_join_utf16_text_loop47(separator.clone(), result, offset);
                            result_2
                        }
                    }
                } else {
                    result
                };
                {
                    let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| Vec::<f64>::new());
                    {
                        let offset_2 = 0f64;
                        {
                            let result_4 = self::source_ml_source1_temporary_join_utf16_text_loop48(result_3, value, offset_2);
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

pub fn source_ml_source1_temporary_trim_utf16_text_loop49(mut text: Vec<f64>, mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((first < last) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(&text, self::ml_array::number_index(first))).clone().unwrap_or_else(|| (-1f64)))) {
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

pub fn source_ml_source1_temporary_trim_utf16_text_loop50(mut text: Vec<f64>, mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((last > first) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(&text, self::ml_array::number_index((last - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
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

pub fn source_ml_source1_temporary_character_units_utf16_text_loop51(mut values: Vec<f64>, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
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
pub enum MlSource1TemporaryCodePointUnitsUtf16TextLoop52Result {
    MlSource1TemporaryCodePointUnitsUtf16TextLoop52Done(Vec<f64>),
    MlSource1TemporaryCodePointUnitsUtf16TextLoop52Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_code_point_units_utf16_text_loop52(mut values: Vec<f64>, mut result: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value > (1114111f64))) {
                    self::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Return(vec![at])
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
            self::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDisplayPointsUtf16TextJoin54 {
    MlSource1TemporaryDisplayPointsUtf16TextJoin54Next(f64, f64),
}

pub fn source_ml_source1_temporary_display_points_utf16_text_loop53(mut text: Vec<f64>, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
                {
                    let source_ml_source2_s833 = if ((point >= (55296f64)) && (point <= (56319f64))) {
                        {
                            let low = (self::ml_array::optional_at(&text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                            if ((low >= (56320f64)) && (low <= (57343f64))) {
                                {
                                    let point_2 = (((65536f64 + ((point - 55296f64) * 1024f64)) + low) - 56320f64);
                                    {
                                        let at_2 = (at + 1f64);
                                        self::MlSource1TemporaryDisplayPointsUtf16TextJoin54::MlSource1TemporaryDisplayPointsUtf16TextJoin54Next(at_2, point_2)
                                    }
                                }
                            } else {
                                {
                                    let point_3 = 65533f64;
                                    self::MlSource1TemporaryDisplayPointsUtf16TextJoin54::MlSource1TemporaryDisplayPointsUtf16TextJoin54Next(at, point_3)
                                }
                            }
                        }
                    } else {
                        if ((point >= (56320f64)) && (point <= (57343f64))) {
                            {
                                let point_4 = 65533f64;
                                self::MlSource1TemporaryDisplayPointsUtf16TextJoin54::MlSource1TemporaryDisplayPointsUtf16TextJoin54Next(at, point_4)
                            }
                        } else {
                            self::MlSource1TemporaryDisplayPointsUtf16TextJoin54::MlSource1TemporaryDisplayPointsUtf16TextJoin54Next(at, point)
                        }
                    };
                    match source_ml_source2_s833.clone() {
                        self::MlSource1TemporaryDisplayPointsUtf16TextJoin54::MlSource1TemporaryDisplayPointsUtf16TextJoin54Next(at_3, point_5) => {
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

pub fn source_ml_source1_temporary_display_utf8_from_utf16_text_loop55(mut bytes: Vec<f64>, mut points: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((points.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(&points, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
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
pub enum MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Result {
    MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Done(Vec<f64>, f64),
}

pub fn source_ml_source1_temporary_plan_utf16_text_substitution_loop56(mut replacement: Vec<f64>, mut result: Vec<f64>, mut start: f64, mut at: f64) -> self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Result {
    loop {
        return if (at < ((replacement.len() as f64))) {
            {
                let next = (self::ml_array::optional_at(&replacement, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                if ((((self::ml_array::optional_at(&replacement, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)) == (36f64)) && ((((next == (36f64)) || (next == (38f64))) || (next == (96f64))) || (next == (39f64)))) {
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
            self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Result::MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Done(result, start)
        };
    }
}

pub fn source_ml_source1_temporary_replace_utf16_text_loop58(mut text: Vec<f64>, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
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
pub enum MlSource1TemporaryReplaceUtf16TextLoop57Result {
    MlSource1TemporaryReplaceUtf16TextLoop57Done(Vec<f64>, f64),
}

pub fn source_ml_source1_temporary_replace_utf16_text_loop59(mut text: Vec<f64>, mut result: Vec<f64>, mut found: f64, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < found) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
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

pub fn source_ml_source1_temporary_replace_utf16_text_loop61(mut result: Vec<f64>, mut last: f64, mut source: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(&source, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
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

pub fn source_ml_source1_temporary_replace_utf16_text_loop60(mut text: Vec<f64>, mut search: Vec<f64>, mut replacement: Vec<f64>, mut result: Vec<f64>, mut plan: Vec<f64>, mut found: f64, mut instruction: f64) -> Vec<f64> {
    loop {
        return if (instruction < ((plan.len() as f64))) {
            {
                let kind = (self::ml_array::optional_at(&plan, self::ml_array::number_index(instruction))).clone().unwrap_or_else(|| 0f64);
                {
                    let first = if (kind == (0f64)) {
                        (self::ml_array::optional_at(&plan, self::ml_array::number_index((instruction + 1f64)))).clone().unwrap_or_else(|| 0f64)
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
                            (self::ml_array::optional_at(&plan, self::ml_array::number_index((instruction + 2f64)))).clone().unwrap_or_else(|| 0f64)
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
                                replacement.clone()
                            } else {
                                text.clone()
                            };
                            {
                                let at = first;
                                {
                                    let result_2 = self::source_ml_source1_temporary_replace_utf16_text_loop61(result, last, source, at);
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

pub fn source_ml_source1_temporary_replace_utf16_text_loop57(mut text: Vec<f64>, mut search: Vec<f64>, mut replacement: Vec<f64>, mut all: bool, mut result: Vec<f64>, mut plan: Vec<f64>, mut start: f64, mut from: f64, mut searching: bool) -> self::MlSource1TemporaryReplaceUtf16TextLoop57Result {
    loop {
        return if searching {
            {
                let found = self::index_utf16_text(text.clone(), search.clone(), from);
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
                            let result_2 = self::source_ml_source1_temporary_replace_utf16_text_loop59(text.clone(), result, found, at);
                            {
                                let instruction = 0f64;
                                {
                                    let result_3 = self::source_ml_source1_temporary_replace_utf16_text_loop60(text.clone(), search.clone(), replacement.clone(), result_2, plan.clone(), found, instruction);
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
            self::MlSource1TemporaryReplaceUtf16TextLoop57Result::MlSource1TemporaryReplaceUtf16TextLoop57Done(result, start)
        };
    }
}

pub fn source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(mut receiver_text: self::JavaScriptSemanticValue, mut search_text: self::JavaScriptSemanticValue, mut search_length: f64, mut start: f64, mut matches: bool, mut at: f64) -> bool {
    loop {
        return if (matches && (at < search_length)) {
            if ((self::read_java_script_semantic_text_unit(receiver_text.clone(), (start + at))) != (self::read_java_script_semantic_text_unit(search_text.clone(), at))) {
                {
                    let matches_2 = false;
                    {
                        let at_2 = (at + 1f64);
                        {
                            (matches, at) = (matches_2, at_2);
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
            matches
        };
    }
}

fn ml_main() {
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Text(vec![97f64, 98f64, 99f64]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Text(vec![98f64]), self::JavaScriptSemanticValue::Number(1f64)])) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(true))))), "assertion 1");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![97, 98, 99])), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Text(vec![98f64]), self::JavaScriptSemanticValue::Number(2f64)])) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(false))))), "assertion 2");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Text(vec![97f64, 98f64, 99f64]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Text(vec![97f64]), self::JavaScriptSemanticValue::Number(f64::NEG_INFINITY), self::JavaScriptSemanticValue::Number(19f64)])) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(true))))), "assertion 3");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Text(vec![97f64, 98f64, 99f64]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Text(Vec::<f64>::new()), self::JavaScriptSemanticValue::Number(f64::INFINITY)])) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(true))))), "assertion 4");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Text(vec![97f64, 98f64, 99f64]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Text(vec![97f64]), self::JavaScriptSemanticValue::Number(f64::NAN)])) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(true))))), "assertion 5");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Text(vec![97f64, 98f64, 99f64]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Text(vec![97f64]), self::JavaScriptSemanticValue::Number((-0f64))])) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(true))))), "assertion 6");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Undefined, self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), Vec::<self::JavaScriptSemanticValue>::new())) == (self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104, 32, 99, 97, 108, 108, 101, 100, 32, 111, 110, 32, 110, 117, 108, 108, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100])))), "assertion 7");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Text(vec![97f64]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false)])) == (self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 73, 115, 82, 101, 103, 69, 120, 112, 32, 97, 110, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108, 115])))), "assertion 8");
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
fn all_literal_primitive_method_controls() { main(); }
