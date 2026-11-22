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

pub fn accept_source_private_identifier(name: self::ml_string::Text) -> bool {
    name.starts_with(&self::ml_string::Text::from_units(vec![109, 108, 95]))
}

pub fn read_source_identifier_prefix_ordinal(name: self::ml_string::Text) -> f64 {
    if !(name.starts_with(&self::ml_string::Text::from_units(vec![109, 108, 95, 115, 111, 117, 114, 99, 101]))) {
        (-1f64)
    } else {
        {
            let index = 9f64;
            {
                let ordinal = 0f64;
                if (((({
                    let source_ml_source2_string_unit_operand10 = name.clone();
                    {
                        let source_ml_source2_string_unit_operand11 = index;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand17 = {
                                        let source_ml_source2_string_range_operand12 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand13 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand14 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position15 = source_ml_source2_string_range_operand13;
                                                    {
                                                        let source_ml_source2_string_range_position16 = source_ml_source2_string_range_operand14;
                                                        source_ml_source2_string_range_operand12.slice(source_ml_source2_string_range_position15, source_ml_source2_string_range_position16)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand18 = 0f64;
                                        source_ml_source2_string_unit_operand17.code_point_at(source_ml_source2_string_unit_operand18)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand10.clone(), source_ml_source2_string_unit_operand11)
                    }
                }) < (49f64)) || (({
                    let source_ml_source2_string_unit_operand28 = name.clone();
                    {
                        let source_ml_source2_string_unit_operand29 = index;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand35 = {
                                        let source_ml_source2_string_range_operand30 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand31 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand32 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position33 = source_ml_source2_string_range_operand31;
                                                    {
                                                        let source_ml_source2_string_range_position34 = source_ml_source2_string_range_operand32;
                                                        source_ml_source2_string_range_operand30.slice(source_ml_source2_string_range_position33, source_ml_source2_string_range_position34)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand36 = 0f64;
                                        source_ml_source2_string_unit_operand35.code_point_at(source_ml_source2_string_unit_operand36)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand28.clone(), source_ml_source2_string_unit_operand29)
                    }
                }) > (57f64))) || (index >= (name.length()))) {
                    (-1f64)
                } else {
                    {
                        let source_ml_source2_s37 = self::source_ml_source1_temporary_read_source_identifier_prefix_ordinal_loop1(name.clone(), index, ordinal);
                        match source_ml_source2_s37.clone() {
                            self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Done(index_2, ordinal_2) => {
                                if (({
                                    let source_ml_source2_string_unit_operand47 = name.clone();
                                    {
                                        let source_ml_source2_string_unit_operand48 = index_2;
                                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                            let index_3 = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if ((index_3 < (0f64)) || (index_3 >= (text.length()))) {
                                                f64::NAN
                                            } else {
                                                ({
                                                    let source_ml_source2_string_unit_operand54 = {
                                                        let source_ml_source2_string_range_operand49 = text.clone();
                                                        {
                                                            let source_ml_source2_string_range_operand50 = index_3;
                                                            {
                                                                let source_ml_source2_string_range_operand51 = (index_3 + 1f64);
                                                                {
                                                                    let source_ml_source2_string_range_position52 = source_ml_source2_string_range_operand50;
                                                                    {
                                                                        let source_ml_source2_string_range_position53 = source_ml_source2_string_range_operand51;
                                                                        source_ml_source2_string_range_operand49.slice(source_ml_source2_string_range_position52, source_ml_source2_string_range_position53)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_string_unit_operand55 = 0f64;
                                                        source_ml_source2_string_unit_operand54.code_point_at(source_ml_source2_string_unit_operand55)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source2_string_unit_operand47.clone(), source_ml_source2_string_unit_operand48)
                                    }
                                }) == (95f64)) {
                                    ordinal_2
                                } else {
                                    (-1f64)
                                }
                            }
                            self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_source_identifier_prefix(private_names: bool, ordinals: Vec<f64>) -> self::ml_string::Text {
    if !private_names {
        self::ml_string::Text::from_units(vec![109, 108, 95])
    } else {
        {
            let next = 1f64;
            {
                let previous = 0f64;
                {
                    let index = 0f64;
                    {
                        let next_2 = self::source_ml_source1_temporary_read_source_identifier_prefix_loop2(ordinals, next, previous, index);
                        ((self::ml_string::Text::from_units(vec![109, 108, 95, 115, 111, 117, 114, 99, 101])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(next_2)))).concat(&self::ml_string::Text::from_units(vec![95]))
                    }
                }
            }
        }
    }
}

pub fn render_source_target_identifier(name: self::ml_string::Text) -> self::ml_string::Text {
    if self::accept_source_private_identifier(name.clone()) {
        (self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 95])).concat(&name)
    } else {
        name.clone()
    }
}

pub fn render_source_generated_identifier(prefix: self::ml_string::Text, role: self::ml_string::Text, ordinal: f64) -> self::ml_string::Text {
    if (!(self::ml_math::is_safe_integer(ordinal)) || (ordinal < (0f64))) {
        panic!("{}", "source identifier generation ordinal exceeds its exact range")
    } else {
        (prefix.concat(&if role.starts_with(&self::ml_string::Text::from_units(vec![109, 108, 95])) {
            {
                let source_ml_source2_string_range_operand56 = role.clone();
                {
                    let source_ml_source2_string_range_operand57 = 3f64;
                    {
                        let source_ml_source2_string_range_position58 = source_ml_source2_string_range_operand57;
                        {
                            let source_ml_source2_string_range_position59 = f64::INFINITY;
                            source_ml_source2_string_range_operand56.slice(source_ml_source2_string_range_position58, source_ml_source2_string_range_position59)
                        }
                    }
                }
            }
        } else {
            role.clone()
        })).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(ordinal)))
    }
}

pub fn render_source_cell_parameter(prefix: self::ml_string::Text, name: self::ml_string::Text, reference: bool) -> self::ml_string::Text {
    if reference {
        (prefix.concat(&self::ml_string::Text::from_units(vec![99, 101, 108, 108, 95]))).concat(&name)
    } else {
        name.clone()
    }
}

pub fn dependency_function1(kind: self::ml_string::Text, name: self::ml_string::Text, duplicate: bool, reserved: Vec<self::ml_string::Text>) -> bool {
    if (((kind != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) || (name == (self::ml_string::Text::from_units(vec![])))) || duplicate) {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s60 = self::source_ml_source1_temporary_dependency_function1_loop3(name.clone(), reserved, index);
                match source_ml_source2_s60.clone() {
                    self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Done => {
                        true
                    }
                    self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Return(source_ml_source1_temporary_result3) => {
                        source_ml_source1_temporary_result3
                    }
                }
            }
        }
    }
}

pub fn read_source_library_declaration_binding_kind(keyword: self::ml_string::Text, kind: self::ml_string::Text, name: self::ml_string::Text, callable: bool) -> self::ml_string::Text {
    if !(self::dependency_function1(kind.clone(), name.clone(), false, Vec::<self::ml_string::Text>::new())) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (keyword == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
            self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])
        } else {
            if ((keyword == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]))) && callable) {
                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn accept_java_script_source_function_head_name(kind: self::ml_string::Text) -> bool {
    (kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))
}

pub fn read_source_library_function_declaration_offset(async_: self::ml_string::Text, second: self::ml_string::Text, line_break: bool) -> f64 {
    if (async_ == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
        0f64
    } else {
        if (((async_ == (self::ml_string::Text::from_units(vec![97, 115, 121, 110, 99]))) && (second == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))) && !line_break) {
            1f64
        } else {
            (-1f64)
        }
    }
}

pub fn test_java_script_source_function_expression_term(term: self::ml_string::Text) -> bool {
    ((term == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 95, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110]))) || (term == (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 97, 116, 111, 114, 95, 102, 117, 110, 99, 116, 105, 111, 110]))))
}

pub fn test_java_script_source_keyword_role(identifier: bool, keyword: bool) -> bool {
    (!identifier && keyword)
}

pub fn read_java_script_source_function_scope_form(term: self::ml_string::Text) -> self::ml_string::Text {
    if self::test_java_script_source_function_expression_term(term.clone()) {
        self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110])
    } else {
        if ((term == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]))) || (term == (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 97, 116, 111, 114, 95, 102, 117, 110, 99, 116, 105, 111, 110, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110])))) {
            self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110])
        } else {
            if (term == (self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 95, 102, 117, 110, 99, 116, 105, 111, 110]))) {
                self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 102, 117, 110, 99, 116, 105, 111, 110])
            } else {
                if (term == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 95, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))) {
                    self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn accept_java_script_source_simple_parameter_list(terms: Vec<self::ml_string::Text>) -> bool {
    {
        let source_ml_source1_temporary_values4 = terms;
        {
            let source_ml_source1_temporary_index4 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s61 = self::source_ml_source1_temporary_accept_java_script_source_simple_parameter_list_loop5(source_ml_source1_temporary_values4, source_ml_source1_temporary_index4.clone());
                match source_ml_source2_s61.clone() {
                    self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Done => {
                        true
                    }
                    self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Return(source_ml_source1_temporary_result5) => {
                        source_ml_source1_temporary_result5
                    }
                }
            }
        }
    }
}

pub fn test_java_script_source_parameter_variable_dependency(first: self::ml_string::Text, second: self::ml_string::Text, body_parent: bool) -> bool {
    (body_parent && (((first == (self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 101, 116, 101, 114]))) && (second == (self::ml_string::Text::from_units(vec![118, 97, 114])))) || ((first == (self::ml_string::Text::from_units(vec![118, 97, 114]))) && (second == (self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 101, 116, 101, 114]))))))
}

pub fn test_java_script_source_variable_redeclaration(kind: self::ml_string::Text, existing: bool) -> bool {
    ((kind == (self::ml_string::Text::from_units(vec![118, 97, 114]))) && existing)
}

pub fn test_java_script_source_loop_scope_term(term: self::ml_string::Text) -> bool {
    (((term == (self::ml_string::Text::from_units(vec![102, 111, 114, 95, 115, 116, 97, 116, 101, 109, 101, 110, 116]))) || (term == (self::ml_string::Text::from_units(vec![102, 111, 114, 95, 105, 110, 95, 115, 116, 97, 116, 101, 109, 101, 110, 116])))) || (term == (self::ml_string::Text::from_units(vec![102, 111, 114, 95, 111, 102, 95, 115, 116, 97, 116, 101, 109, 101, 110, 116]))))
}

pub fn test_java_script_source_catch_scope_term(term: self::ml_string::Text) -> bool {
    (term == (self::ml_string::Text::from_units(vec![99, 97, 116, 99, 104, 95, 99, 108, 97, 117, 115, 101])))
}

pub fn test_java_script_source_class_expression_term(term: self::ml_string::Text) -> bool {
    (term == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))
}

pub fn test_java_script_source_variable_environment_term(term: self::ml_string::Text) -> bool {
    ((term == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 115, 116, 97, 116, 105, 99, 95, 98, 108, 111, 99, 107]))) || ((self::read_java_script_source_function_scope_form(term.clone())) != (self::ml_string::Text::from_units(vec![]))))
}

pub fn test_rust_source_opaque_literal_term(term: self::ml_string::Text) -> bool {
    ((((term == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 95, 108, 105, 116, 101, 114, 97, 108]))) || (term == (self::ml_string::Text::from_units(vec![114, 97, 119, 95, 115, 116, 114, 105, 110, 103, 95, 108, 105, 116, 101, 114, 97, 108])))) || (term == (self::ml_string::Text::from_units(vec![99, 104, 97, 114, 97, 99, 116, 101, 114, 95, 108, 105, 116, 101, 114, 97, 108])))) || (term == (self::ml_string::Text::from_units(vec![99, 104, 97, 114, 95, 108, 105, 116, 101, 114, 97, 108]))))
}

pub fn test_java_script_source_opaque_literal_term(term: self::ml_string::Text) -> bool {
    (((term == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || (term == (self::ml_string::Text::from_units(vec![114, 101, 103, 117, 108, 97, 114, 95, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110])))) || (term == (self::ml_string::Text::from_units(vec![114, 101, 103, 101, 120]))))
}

pub fn decode_java_script_identifier_escape(units: Vec<f64>, start: f64) -> Vec<f64> {
    if ((((!(self::ml_math::is_safe_integer(start)) || (start < (0f64))) || (((start + 2f64)) > ((units.len() as f64)))) || {
        let source_ml_source2_comparison_operand62 = self::ml_array::optional_at(&units, self::ml_array::number_index(start));
        {
            let source_ml_source2_comparison_operand63 = 92f64;
            !(match (source_ml_source2_comparison_operand62.clone(), source_ml_source2_comparison_operand63) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) || {
        let source_ml_source2_comparison_operand64 = self::ml_array::optional_at(&units, self::ml_array::number_index((start + 1f64)));
        {
            let source_ml_source2_comparison_operand65 = 117f64;
            !(match (source_ml_source2_comparison_operand64.clone(), source_ml_source2_comparison_operand65) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) {
        vec![(-1f64), self::ml_math::min((units.len() as f64), (start + 2f64))]
    } else {
        {
            let braced = {
                let source_ml_source2_comparison_operand66 = self::ml_array::optional_at(&units, self::ml_array::number_index((start + 2f64)));
                {
                    let source_ml_source2_comparison_operand67 = 123f64;
                    match (source_ml_source2_comparison_operand66.clone(), source_ml_source2_comparison_operand67) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                }
            };
            {
                let index = (start + if braced {
                    3f64
                } else {
                    2f64
                });
                {
                    let digits = 0f64;
                    {
                        let scalar = 0f64;
                        {
                            let source_ml_source2_s68 = self::source_ml_source1_temporary_decode_java_script_identifier_escape_loop6(units.clone(), braced, index, digits, scalar);
                            match source_ml_source2_s68.clone() {
                                self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Done(index_2, digits_2, scalar_2) => {
                                    {
                                        let source_ml_source2_s71 = if braced {
                                            if ((digits_2 == (0f64)) || {
                                                let source_ml_source2_comparison_operand72 = self::ml_array::optional_at(&units, self::ml_array::number_index(index_2));
                                                {
                                                    let source_ml_source2_comparison_operand73 = 125f64;
                                                    !(match (source_ml_source2_comparison_operand72.clone(), source_ml_source2_comparison_operand73) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                }
                                            }) {
                                                self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Return(vec![(-1f64), self::ml_math::min((units.len() as f64), (index_2 + 1f64))])
                                            } else {
                                                {
                                                    let index_3 = (index_2 + 1f64);
                                                    self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Next(index_3)
                                                }
                                            }
                                        } else {
                                            if (digits_2 != (4f64)) {
                                                self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Return(vec![(-1f64), self::ml_math::min((units.len() as f64), (index_2 + 1f64))])
                                            } else {
                                                self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Next(index_2)
                                            }
                                        };
                                        match source_ml_source2_s71.clone() {
                                            self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Next(index_4) => {
                                                if ((scalar_2 >= (55296f64)) && (scalar_2 <= (57343f64))) {
                                                    vec![(-1f64), index_4]
                                                } else {
                                                    vec![index_4, scalar_2]
                                                }
                                            }
                                            self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Return(source_ml_source1_temporary_result7) => {
                                                source_ml_source1_temporary_result7
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Return(source_ml_source1_temporary_result7_2) => {
                                    source_ml_source1_temporary_result7_2
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_java_script_reserved_identifier(name: self::ml_string::Text) -> bool {
    self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116]), self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107]), self::ml_string::Text::from_units(vec![99, 97, 115, 101]), self::ml_string::Text::from_units(vec![99, 97, 116, 99, 104]), self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]), self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101]), self::ml_string::Text::from_units(vec![100, 101, 98, 117, 103, 103, 101, 114]), self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]), self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101]), self::ml_string::Text::from_units(vec![100, 111]), self::ml_string::Text::from_units(vec![101, 108, 115, 101]), self::ml_string::Text::from_units(vec![101, 110, 117, 109]), self::ml_string::Text::from_units(vec![101, 120, 112, 111, 114, 116]), self::ml_string::Text::from_units(vec![101, 120, 116, 101, 110, 100, 115]), self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101]), self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121]), self::ml_string::Text::from_units(vec![102, 111, 114]), self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![105, 102]), self::ml_string::Text::from_units(vec![105, 109, 112, 111, 114, 116]), self::ml_string::Text::from_units(vec![105, 110]), self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102]), self::ml_string::Text::from_units(vec![110, 101, 119]), self::ml_string::Text::from_units(vec![110, 117, 108, 108]), self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]), self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]), self::ml_string::Text::from_units(vec![115, 119, 105, 116, 99, 104]), self::ml_string::Text::from_units(vec![116, 104, 105, 115]), self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]), self::ml_string::Text::from_units(vec![116, 114, 117, 101]), self::ml_string::Text::from_units(vec![116, 114, 121]), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 111, 102]), self::ml_string::Text::from_units(vec![118, 97, 114]), self::ml_string::Text::from_units(vec![118, 111, 105, 100]), self::ml_string::Text::from_units(vec![119, 104, 105, 108, 101]), self::ml_string::Text::from_units(vec![119, 105, 116, 104]), self::ml_string::Text::from_units(vec![121, 105, 101, 108, 100]), self::ml_string::Text::from_units(vec![108, 101, 116]), self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99]), self::ml_string::Text::from_units(vec![105, 109, 112, 108, 101, 109, 101, 110, 116, 115]), self::ml_string::Text::from_units(vec![105, 110, 116, 101, 114, 102, 97, 99, 101]), self::ml_string::Text::from_units(vec![112, 97, 99, 107, 97, 103, 101]), self::ml_string::Text::from_units(vec![112, 114, 105, 118, 97, 116, 101]), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 101, 99, 116, 101, 100]), self::ml_string::Text::from_units(vec![112, 117, 98, 108, 105, 99])], &name.clone(), 0f64, false, |a, b| a == b)
}

pub fn accept_java_script_source_keyword_token(name: self::ml_string::Text, raw_: self::ml_string::Text) -> bool {
    (!(raw_.contains(&self::ml_string::Text::from_units(vec![92]))) || !((self::accept_java_script_reserved_identifier(name.clone()) || self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 115]), self::ml_string::Text::from_units(vec![97, 115, 121, 110, 99]), self::ml_string::Text::from_units(vec![102, 114, 111, 109]), self::ml_string::Text::from_units(vec![103, 101, 116]), self::ml_string::Text::from_units(vec![109, 101, 116, 97]), self::ml_string::Text::from_units(vec![111, 102]), self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116])], &name.clone(), 0f64, false, |a, b| a == b))))
}

pub fn read_java_script_source_token_goal_value(kind: self::ml_string::Text, name: self::ml_string::Text, raw_: self::ml_string::Text) -> self::ml_string::Text {
    if ((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && !(self::accept_java_script_source_keyword_token(name.clone(), raw_.clone()))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        name.clone()
    }
}

pub fn read_java_script_escaped_identifier_error(name: self::ml_string::Text, raw_: self::ml_string::Text, binding: bool) -> self::ml_string::Text {
    if !(raw_.contains(&self::ml_string::Text::from_units(vec![92]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if self::accept_java_script_reserved_identifier(name.clone()) {
            (self::ml_string::Text::from_units(vec![101, 115, 99, 97, 112, 101, 100, 32, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 32, 105, 100, 101, 110, 116, 105, 102, 105, 101, 114, 32, 105, 115, 32, 114, 101, 115, 101, 114, 118, 101, 100, 58, 32])).concat(&name)
        } else {
            if (binding && ((name == (self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115]))) || (name == (self::ml_string::Text::from_units(vec![101, 118, 97, 108]))))) {
                (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 115, 116, 114, 105, 99, 116, 32, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 100, 101, 110, 116, 105, 102, 105, 101, 114, 58, 32])).concat(&name)
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn accept_java_script_binding_identifier_context(context: self::ml_string::Text) -> bool {
    self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 101, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 98, 105, 110, 100, 105, 110, 103]), self::ml_string::Text::from_units(vec![110, 97, 109, 101, 115, 112, 97, 99, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]), self::ml_string::Text::from_units(vec![108, 101, 116]), self::ml_string::Text::from_units(vec![102, 111, 114, 8230, 111, 102])], &context.clone(), 0f64, false, |a, b| a == b)
}

pub fn accept_java_script_number_identifier_boundary(identifier: bool, following: f64) -> bool {
    ((!identifier && (following != (92f64))) && !(((following >= (48f64)) && (following <= (57f64)))))
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

pub fn dependency_function2(values: Vec<f64>, index: f64) -> f64 {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

pub fn dependency_function3(values: Vec<self::ml_string::Text>, index: f64) -> self::ml_string::Text {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

pub fn read_required_boolean_fact(values: Vec<bool>, index: f64) -> bool {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

pub fn read_source_library_class_profile_error(profile: self::ml_string::Text, dependencies: bool, aliases: bool, shared: bool) -> self::ml_string::Text {
    if (profile == (self::ml_string::Text::from_units(vec![]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if ((((profile == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 108, 111, 99, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 49]))) && dependencies) && aliases) && shared) {
            self::ml_string::Text::from_units(vec![])
        } else {
            self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 121, 44, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 32, 97, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101, 115])
        }
    }
}

pub fn read_source_library_re_export_profile_error(profile: self::ml_string::Text, dependencies: bool, aliases: bool) -> self::ml_string::Text {
    if ((profile == (self::ml_string::Text::from_units(vec![]))) || (((profile == (self::ml_string::Text::from_units(vec![114, 101, 115, 111, 108, 118, 101, 100, 32, 110, 97, 109, 101, 100, 32, 109, 111, 100, 117, 108, 101, 32, 112, 114, 111, 118, 105, 100, 101, 114, 115, 32, 49]))) && dependencies) && aliases)) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100, 32, 114, 101, 45, 101, 120, 112, 111, 114, 116, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 32, 97, 110, 100, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 101, 115])
    }
}

pub fn accept_source_library_re_export_name(kind: self::ml_string::Text, name: self::ml_string::Text) -> bool {
    (((self::dependency_function1(kind.clone(), name.clone(), false, Vec::<self::ml_string::Text>::new()) && !(self::accept_java_script_reserved_identifier(name.clone()))) && (name != (self::ml_string::Text::from_units(vec![101, 118, 97, 108])))) && (name != (self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115]))))
}

pub fn accept_source_library_entry_export_owner(exported: self::ml_string::Text, compiled: self::ml_string::Text) -> bool {
    ((exported != (self::ml_string::Text::from_units(vec![]))) && (exported == compiled))
}

pub fn read_source_library_class_binding_kind(keyword: self::ml_string::Text, kind: self::ml_string::Text, name: self::ml_string::Text, callable: bool) -> self::ml_string::Text {
    if (((((keyword == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) && self::dependency_function1(kind.clone(), name.clone(), false, Vec::<self::ml_string::Text>::new())) && !(self::accept_java_script_reserved_identifier(name.clone()))) && (name != (self::ml_string::Text::from_units(vec![101, 118, 97, 108])))) && (name != (self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115])))) {
        self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])
    } else {
        self::read_source_library_declaration_binding_kind(keyword.clone(), kind.clone(), name.clone(), callable)
    }
}

pub fn accept_source_library_class_export(keyword: self::ml_string::Text, kind: self::ml_string::Text, name: self::ml_string::Text, callable: bool) -> bool {
    ((self::read_source_library_class_binding_kind(keyword.clone(), kind.clone(), name.clone(), callable)) != (self::ml_string::Text::from_units(vec![])))
}

pub fn accept_source_library_class_declaration_term(term: self::ml_string::Text, kind: self::ml_string::Text) -> bool {
    if (kind == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) {
        (term == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110])))
    } else {
        if (kind == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
            ((term == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]))) || (term == (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 97, 116, 111, 114, 95, 102, 117, 110, 99, 116, 105, 111, 110, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]))))
        } else {
            ((kind == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]))) && (term == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]))))
        }
    }
}

pub fn read_source_library_class_declaration_owner(names: Vec<self::ml_string::Text>, starts: Vec<f64>, ends: Vec<f64>, witnessed: Vec<self::ml_string::Text>, first: f64, last: f64) -> self::ml_string::Text {
    {
        let owner = self::ml_string::Text::from_units(vec![]);
        if ((((((names.len() as f64)) != ((starts.len() as f64))) || (((names.len() as f64)) != ((ends.len() as f64)))) || (first < (0f64))) || (last < first)) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s74 = self::source_ml_source1_temporary_read_source_library_class_declaration_owner_loop8(names, starts, ends, witnessed, first, last, owner.clone(), at);
                    match source_ml_source2_s74.clone() {
                        self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Done(owner_2) => {
                            owner_2.clone()
                        }
                        self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Return(source_ml_source1_temporary_result8) => {
                            source_ml_source1_temporary_result8.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_source_library_class_helper_call(imported_owners: Vec<self::ml_string::Text>, constructor: self::ml_string::Text, helpers: Vec<self::ml_string::Text>, helper: self::ml_string::Text, receiver_owner: self::ml_string::Text) -> bool {
    ((((constructor != (self::ml_string::Text::from_units(vec![]))) && (constructor == receiver_owner)) && self::ml_array::includes(&imported_owners, &constructor.clone(), 0f64, false, |a, b| a == b)) && self::ml_array::includes(&helpers, &helper.clone(), 0f64, false, |a, b| a == b))
}

pub fn accept_source_library_class_static_call(imported_owners: Vec<self::ml_string::Text>, constructor: self::ml_string::Text, helpers: Vec<self::ml_string::Text>, helper: self::ml_string::Text) -> bool {
    (((constructor != (self::ml_string::Text::from_units(vec![]))) && self::ml_array::includes(&imported_owners, &constructor.clone(), 0f64, false, |a, b| a == b)) && self::ml_array::includes(&helpers, &helper.clone(), 0f64, false, |a, b| a == b))
}

pub fn project_source_library_class_edit_interval(starts: Vec<f64>, ends: Vec<f64>, lengths: Vec<f64>, first: f64, last: f64) -> Vec<f64> {
    if (((((((((starts.len() as f64)) != ((ends.len() as f64))) || (((starts.len() as f64)) != ((lengths.len() as f64)))) || (((starts.len() as f64)) > (32768f64))) || !(self::ml_math::is_safe_integer(first))) || !(self::ml_math::is_safe_integer(last))) || (first < (0f64))) || (last < first)) {
        Vec::<f64>::new()
    } else {
        {
            let delta = 0f64;
            {
                let previous = 0f64;
                {
                    let low = (-1f64);
                    {
                        let high = (-1f64);
                        {
                            let at = 0f64;
                            {
                                let source_ml_source2_s75 = self::source_ml_source1_temporary_project_source_library_class_edit_interval_loop9(starts, ends, lengths, first, last, delta, previous, low, high, at);
                                match source_ml_source2_s75.clone() {
                                    self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Done(delta_2, low_2, high_2) => {
                                        {
                                            let low_4 = if (low_2 < (0f64)) {
                                                {
                                                    let low_3 = (first - delta_2);
                                                    low_3
                                                }
                                            } else {
                                                low_2
                                            };
                                            {
                                                let high_4 = if (high_2 < (0f64)) {
                                                    {
                                                        let high_3 = (last - delta_2);
                                                        high_3
                                                    }
                                                } else {
                                                    high_2
                                                };
                                                if ((low_4 < (0f64)) || (high_4 < low_4)) {
                                                    Vec::<f64>::new()
                                                } else {
                                                    vec![low_4, high_4]
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Return(source_ml_source1_temporary_result9) => {
                                        source_ml_source1_temporary_result9
                                    }
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
pub enum MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result {
    MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Done(f64, f64),
    MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Return(f64),
}

pub fn source_ml_source1_temporary_read_source_identifier_prefix_ordinal_loop1(mut name: self::ml_string::Text, mut index: f64, mut ordinal: f64) -> self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result {
    loop {
        return if (index < (name.length())) {
            {
                let unit = {
                    let source_ml_source2_string_unit_operand76 = name.clone();
                    {
                        let source_ml_source2_string_unit_operand77 = index;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand83 = {
                                        let source_ml_source2_string_range_operand78 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand79 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand80 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position81 = source_ml_source2_string_range_operand79;
                                                    {
                                                        let source_ml_source2_string_range_position82 = source_ml_source2_string_range_operand80;
                                                        source_ml_source2_string_range_operand78.slice(source_ml_source2_string_range_position81, source_ml_source2_string_range_position82)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand84 = 0f64;
                                        source_ml_source2_string_unit_operand83.code_point_at(source_ml_source2_string_unit_operand84)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand76.clone(), source_ml_source2_string_unit_operand77)
                    }
                };
                if ((unit < (48f64)) || (unit > (57f64))) {
                    self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Done(index, ordinal)
                } else {
                    {
                        let ordinal_2 = (((ordinal * 10f64) + unit) - 48f64);
                        if !(self::ml_math::is_safe_integer(ordinal_2)) {
                            self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Return((-1f64))
                        } else {
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (index, ordinal) = (index_2, ordinal_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Done(index, ordinal)
        };
    }
}

pub fn source_ml_source1_temporary_read_source_identifier_prefix_loop2(mut ordinals: Vec<f64>, mut next: f64, mut previous: f64, mut index: f64) -> f64 {
    loop {
        return if (index < ((ordinals.len() as f64))) {
            {
                let ordinal = (self::ml_array::optional_at(&ordinals, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(ordinal)) || (ordinal < (1f64))) || (ordinal < previous)) {
                    panic!("{}", "source identifier ordinals need a sorted positive integer column")
                } else {
                    {
                        let previous_2 = ordinal;
                        if (ordinal == next) {
                            {
                                let next_2 = (next + 1f64);
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        (next, previous, index) = (next_2, previous_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let index_3 = (index + 1f64);
                                {
                                    (previous, index) = (previous_2, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            next
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDependencyFunction1Loop3Result {
    MlSource1TemporaryDependencyFunction1Loop3Done,
    MlSource1TemporaryDependencyFunction1Loop3Return(bool),
}

pub fn source_ml_source1_temporary_dependency_function1_loop3(mut name: self::ml_string::Text, mut reserved: Vec<self::ml_string::Text>, mut index: f64) -> self::MlSource1TemporaryDependencyFunction1Loop3Result {
    loop {
        return if (index < ((reserved.len() as f64))) {
            {
                let word = self::ml_array::optional_at(&reserved, self::ml_array::number_index(index));
                if {
                    let source_ml_source2_comparison_operand85 = word.clone();
                    {
                        let source_ml_source2_comparison_operand86 = None::<()>;
                        match (source_ml_source2_comparison_operand85.clone(), source_ml_source2_comparison_operand86.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                    }
                } {
                    panic!("{}", "source binding reserved names need a complete text column")
                } else {
                    if ((word.clone().expect("a proven present value")) == name) {
                        self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Return(false)
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
            }
        } else {
            self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result {
    MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Done,
    MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Return(bool),
}

pub fn source_ml_source1_temporary_accept_java_script_source_simple_parameter_list_loop5(mut source_ml_source1_temporary_values4: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_index4: self::ml::Big) -> self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result {
    loop {
        return if (source_ml_source1_temporary_index4 < (self::ml::Big::from_u128(source_ml_source1_temporary_values4.len() as u128))) {
            {
                let term = self::ml_array::at(&source_ml_source1_temporary_values4, source_ml_source1_temporary_index4.to_index());
                if self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 95, 112, 97, 116, 116, 101, 114, 110]), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 95, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 95, 112, 97, 116, 116, 101, 114, 110]), self::ml_string::Text::from_units(vec![114, 101, 115, 116, 95, 112, 97, 116, 116, 101, 114, 110]), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 95, 112, 97, 116, 116, 101, 114, 110]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 95, 112, 97, 116, 116, 101, 114, 110])], &term.clone(), 0f64, false, |a, b| a == b) {
                    self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Return(false)
                } else {
                    {
                        let source_ml_source1_temporary_index4_2 = source_ml_source1_temporary_index4.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index4 = source_ml_source1_temporary_index4_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7 {
    MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Next(f64),
    MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result {
    MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Done(f64, f64, f64),
    MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_decode_java_script_identifier_escape_loop6(mut units: Vec<f64>, mut braced: bool, mut index: f64, mut digits: f64, mut scalar: f64) -> self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result {
    loop {
        return if ((index < ((units.len() as f64))) && (braced || (digits < (4f64)))) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                {
                    let digit = (-1f64);
                    {
                        let digit_5 = if ((unit >= (48f64)) && (unit <= (57f64))) {
                            {
                                let digit_2 = (unit - 48f64);
                                digit_2
                            }
                        } else {
                            if ((unit >= (65f64)) && (unit <= (70f64))) {
                                {
                                    let digit_3 = (unit - 55f64);
                                    digit_3
                                }
                            } else {
                                if ((unit >= (97f64)) && (unit <= (102f64))) {
                                    {
                                        let digit_4 = (unit - 87f64);
                                        digit_4
                                    }
                                } else {
                                    digit
                                }
                            }
                        };
                        if (digit_5 < (0f64)) {
                            self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Done(index, digits, scalar)
                        } else {
                            {
                                let scalar_2 = ((scalar * 16f64) + digit_5);
                                {
                                    let digits_2 = (digits + 1f64);
                                    {
                                        let index_2 = (index + 1f64);
                                        if (scalar_2 > (1114111f64)) {
                                            self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Return(vec![(-1f64), index_2])
                                        } else {
                                            {
                                                (index, digits, scalar) = (index_2, digits_2, scalar_2);
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
            self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Done(index, digits, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result {
    MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Done(self::ml_string::Text),
    MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_source_library_class_declaration_owner_loop8(mut names: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut witnessed: Vec<self::ml_string::Text>, mut first: f64, mut last: f64, mut owner: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result {
    loop {
        return if (at < ((names.len() as f64))) {
            if (((first >= (self::dependency_function2(starts.clone(), at))) && (last <= (self::dependency_function2(ends.clone(), at)))) && self::ml_array::includes(&witnessed, &self::dependency_function3(names.clone(), at), 0f64, false, |a, b| a == b)) {
                if (owner != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Return(self::ml_string::Text::from_units(vec![35, 97, 109, 98, 105, 103, 117, 111, 117, 115]))
                } else {
                    {
                        let owner_2 = self::dependency_function3(names.clone(), at);
                        {
                            let at_2 = (at + 1f64);
                            {
                                (owner, at) = (owner_2.clone(), at_2);
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
            self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Done(owner.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result {
    MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Done(f64, f64, f64),
    MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_project_source_library_class_edit_interval_loop9(mut starts: Vec<f64>, mut ends: Vec<f64>, mut lengths: Vec<f64>, mut first: f64, mut last: f64, mut delta: f64, mut previous: f64, mut low: f64, mut high: f64, mut at: f64) -> self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result {
    loop {
        return if (at < ((starts.len() as f64))) {
            {
                let start = self::dependency_function2(starts.clone(), at);
                {
                    let end = self::dependency_function2(ends.clone(), at);
                    {
                        let length = self::dependency_function2(lengths.clone(), at);
                        if (((((!(self::ml_math::is_safe_integer(start)) || !(self::ml_math::is_safe_integer(end))) || !(self::ml_math::is_safe_integer(length))) || (start < previous)) || (end < start)) || (length < (0f64))) {
                            self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Return(Vec::<f64>::new())
                        } else {
                            {
                                let changed_start = (start + delta);
                                {
                                    let changed_end = (changed_start + length);
                                    {
                                        let low_3 = if ((low < (0f64)) && (first < changed_start)) {
                                            {
                                                let low_2 = (first - delta);
                                                low_2
                                            }
                                        } else {
                                            low
                                        };
                                        {
                                            let high_3 = if ((high < (0f64)) && (last < changed_start)) {
                                                {
                                                    let high_2 = (last - delta);
                                                    high_2
                                                }
                                            } else {
                                                high
                                            };
                                            {
                                                let low_5 = if (((first >= changed_start) && (first <= changed_end)) && (low_3 < (0f64))) {
                                                    {
                                                        let low_4 = start;
                                                        low_4
                                                    }
                                                } else {
                                                    low_3
                                                };
                                                {
                                                    let high_5 = if (((last >= changed_start) && (last <= changed_end)) && (high_3 < (0f64))) {
                                                        {
                                                            let high_4 = end;
                                                            high_4
                                                        }
                                                    } else {
                                                        high_3
                                                    };
                                                    {
                                                        let delta_2 = (delta + (length - (end - start)));
                                                        {
                                                            let previous_2 = end;
                                                            {
                                                                let at_2 = (at + 1f64);
                                                                {
                                                                    (delta, previous, low, high, at) = (delta_2, previous_2, low_5, high_5, at_2);
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
            self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Done(delta, low, high)
        };
    }
}

fn ml_main() {
    assert!(((self::read_source_library_class_profile_error(self::ml_string::Text::from_units(vec![]), false, false, false)) == (self::ml_string::Text::from_units(vec![]))), "assertion 1");
    assert!(((self::read_source_library_class_profile_error(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 108, 111, 99, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 49]), true, true, true)) == (self::ml_string::Text::from_units(vec![]))), "assertion 2");
    assert!(((self::read_source_library_class_profile_error(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 108, 111, 99, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 49]), false, true, true)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 121, 44, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 32, 97, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101, 115]))), "assertion 3");
    assert!(((self::read_source_library_class_profile_error(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 108, 111, 99, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 49]), true, false, true)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 121, 44, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 32, 97, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101, 115]))), "assertion 4");
    assert!(((self::read_source_library_class_profile_error(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 108, 111, 99, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 49]), true, true, false)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 121, 44, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 32, 97, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101, 115]))), "assertion 5");
    assert!(((self::read_source_library_class_profile_error(self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110, 32, 99, 108, 97, 115, 115, 32, 112, 114, 111, 102, 105, 108, 101]), true, true, true)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 121, 44, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 32, 97, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101, 115]))), "assertion 6");
    assert!(((self::read_source_library_class_binding_kind(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), false)) == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))), "assertion 7");
    assert!(((self::read_source_library_class_binding_kind(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![114, 117, 110]), true)) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]))), "assertion 8");
    assert!(((self::read_source_library_class_binding_kind(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]), false)) == (self::ml_string::Text::from_units(vec![]))), "assertion 9");
    assert!(((self::accept_source_library_class_export(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), false)) == (true)), "assertion 10");
    assert!(((self::accept_source_library_class_export(self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), false)) == (false)), "assertion 11");
    assert!(((self::accept_source_library_class_declaration_term(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) == (true)), "assertion 12");
    assert!(((self::accept_source_library_class_declaration_term(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) == (false)), "assertion 13");
    assert!(((self::read_source_library_class_declaration_owner(vec![self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101])], vec![10f64], vec![90f64], vec![self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101])], 20f64, 80f64)) == (self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))), "assertion 14");
    assert!(((self::read_source_library_class_declaration_owner(vec![self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101])], vec![10f64], vec![90f64], vec![self::ml_string::Text::from_units(vec![79, 116, 104, 101, 114])], 20f64, 80f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 15");
    assert!(((self::accept_source_library_class_helper_call(vec![self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101])], self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), vec![self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 72, 101, 108, 112, 101, 114])], self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 72, 101, 108, 112, 101, 114]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))) == (true)), "assertion 16");
    assert!(((self::accept_source_library_class_helper_call(Vec::<self::ml_string::Text>::new(), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), vec![self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 72, 101, 108, 112, 101, 114])], self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 72, 101, 108, 112, 101, 114]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))) == (false)), "assertion 17");
    assert!(((self::accept_source_library_class_helper_call(vec![self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101])], self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), vec![self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 72, 101, 108, 112, 101, 114])], self::ml_string::Text::from_units(vec![100, 105, 102, 102, 101, 114, 101, 110, 116]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))) == (false)), "assertion 18");
    assert!(((self::accept_source_library_class_static_call(vec![self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101])], self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), vec![self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 83, 116, 97, 116, 105, 99])], self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 83, 116, 97, 116, 105, 99]))) == (true)), "assertion 19");
    assert!(((self::accept_source_library_class_static_call(Vec::<self::ml_string::Text>::new(), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), vec![self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 83, 116, 97, 116, 105, 99])], self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 83, 116, 97, 116, 105, 99]))) == (false)), "assertion 20");
    assert!(((self::project_source_library_class_edit_interval(vec![4f64], vec![9f64], vec![2f64], 7f64, 9f64)) == (vec![10f64, 12f64])), "assertion 21");
    assert!(((self::project_source_library_class_edit_interval(vec![4f64], vec![9f64], vec![2f64], 4f64, 5f64)) == (vec![4f64, 9f64])), "assertion 22");
    assert!(((self::project_source_library_class_edit_interval(vec![4f64], vec![10f64], vec![0f64], 4f64, 6f64)) == (vec![4f64, 12f64])), "assertion 23");
    assert!(((self::project_source_library_class_edit_interval(vec![9f64, 4f64], vec![10f64, 6f64], vec![1f64, 1f64], 1f64, 3f64)) == (Vec::<f64>::new())), "assertion 24");
    assert!(((self::read_source_library_re_export_profile_error(self::ml_string::Text::from_units(vec![]), false, false)) == (self::ml_string::Text::from_units(vec![]))), "assertion 25");
    assert!(((self::read_source_library_re_export_profile_error(self::ml_string::Text::from_units(vec![114, 101, 115, 111, 108, 118, 101, 100, 32, 110, 97, 109, 101, 100, 32, 109, 111, 100, 117, 108, 101, 32, 112, 114, 111, 118, 105, 100, 101, 114, 115, 32, 49]), true, true)) == (self::ml_string::Text::from_units(vec![]))), "assertion 26");
    assert!(((self::read_source_library_re_export_profile_error(self::ml_string::Text::from_units(vec![114, 101, 115, 111, 108, 118, 101, 100, 32, 110, 97, 109, 101, 100, 32, 109, 111, 100, 117, 108, 101, 32, 112, 114, 111, 118, 105, 100, 101, 114, 115, 32, 49]), false, true)) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100, 32, 114, 101, 45, 101, 120, 112, 111, 114, 116, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 32, 97, 110, 100, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 101, 115]))), "assertion 27");
    assert!(((self::read_source_library_re_export_profile_error(self::ml_string::Text::from_units(vec![114, 101, 115, 111, 108, 118, 101, 100, 32, 110, 97, 109, 101, 100, 32, 109, 111, 100, 117, 108, 101, 32, 112, 114, 111, 118, 105, 100, 101, 114, 115, 32, 49]), true, false)) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100, 32, 114, 101, 45, 101, 120, 112, 111, 114, 116, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 32, 97, 110, 100, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 101, 115]))), "assertion 28");
    assert!(((self::read_source_library_re_export_profile_error(self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]), true, true)) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100, 32, 114, 101, 45, 101, 120, 112, 111, 114, 116, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 32, 97, 110, 100, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 101, 115]))), "assertion 29");
    assert!(((self::accept_source_library_re_export_name(self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))) == (true)), "assertion 30");
    assert!(((self::accept_source_library_re_export_name(self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) == (false)), "assertion 31");
    assert!(((self::accept_source_library_re_export_name(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))) == (false)), "assertion 32");
    assert!(((self::accept_source_library_entry_export_owner(self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))) == (true)), "assertion 33");
    assert!(((self::accept_source_library_entry_export_owner(self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), self::ml_string::Text::from_units(vec![112, 114, 105, 118, 97, 116, 101, 80, 114, 111, 118, 105, 100, 101, 114]))) == (false)), "assertion 34");
    assert!(((self::accept_source_library_entry_export_owner(self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]))) == (false)), "assertion 35");
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
fn all_literal_source_library_class_controls() { main(); }
