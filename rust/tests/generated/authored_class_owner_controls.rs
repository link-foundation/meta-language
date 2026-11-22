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


pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

pub fn refuse_required_text_fact() -> self::ml_string::Text {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

pub fn dependency_function1(values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, index: f64) -> f64 {
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

pub fn dependency_function2(values: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, index: f64) -> self::ml_string::Text {
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

pub fn dependency_function3(starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64, field: self::ml_string::Text) -> f64 {
    if (node < (0f64)) {
        (-1f64)
    } else {
        {
            let start = self::dependency_function1(starts.clone(), node);
            {
                let count = self::dependency_function1(counts.clone(), node);
                {
                    let at = start;
                    {
                        let source_ml_source2_s7 = self::source_ml_source1_temporary_dependency_function3_loop1(labels.clone(), targets.clone(), field.clone(), start, count, at);
                        match source_ml_source2_s7.clone() {
                            self::MlSource1TemporaryDependencyFunction3Loop1Result::MlSource1TemporaryDependencyFunction3Loop1Done => {
                                (-1f64)
                            }
                            self::MlSource1TemporaryDependencyFunction3Loop1Result::MlSource1TemporaryDependencyFunction3Loop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn dependency_function4(starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64, index: f64) -> f64 {
    if (((node < (0f64)) || (index < (0f64))) || (index >= (self::dependency_function1(counts.clone(), node)))) {
        (-1f64)
    } else {
        self::dependency_function1(targets.clone(), (self::dependency_function1(starts.clone(), node) + index))
    }
}

pub fn read_collection_construction_kind(texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64) -> self::ml_string::Text {
    {
        let kind = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107, 105, 110, 100]));
        if (kind < (0f64)) {
            self::ml_string::Text::from_units(vec![])
        } else {
            self::dependency_function2(texts.clone(), kind)
        }
    }
}

pub fn plan_collection_construction_inference(texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, kind: self::ml_string::Text, expected: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let context = ((self::read_collection_construction_kind(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), expected)) == kind);
        if (kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, if context {
                1f64
            } else {
                0f64
            }, if context {
                self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), expected, self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116]))
            } else {
                (-1f64)
            }, (-1f64)]))
        } else {
            if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, if context {
                    1f64
                } else {
                    0f64
                }, if context {
                    self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), expected, self::ml_string::Text::from_units(vec![107, 101, 121]))
                } else {
                    (-1f64)
                }, if context {
                    self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), expected, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))
                } else {
                    (-1f64)
                }]))
            } else {
                std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, (-1f64), (-1f64)]))
            }
        }
    }
}

pub fn plan_collection_construction_checked(texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, kind: self::ml_string::Text, items: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let array = ((self::read_collection_construction_kind(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), items)) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])));
        {
            let element = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), items, self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116]));
            if (kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) {
                if array {
                    std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, element, (-1f64)]))
                } else {
                    std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, (-1f64), (-1f64)]))
                }
            } else {
                {
                    let tuple = ((self::read_collection_construction_kind(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), element)) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])));
                    {
                        let elements = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), element, self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 115]));
                        if (((!array || !tuple) || (elements < (0f64))) || ((self::dependency_function1(counts.clone(), elements)) != (2f64))) {
                            std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, (-1f64), (-1f64)]))
                        } else {
                            std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), elements, 0f64), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), elements, 1f64)]))
                        }
                    }
                }
            }
        }
    }
}

pub fn read_collection_construction_key_admission(kind: self::ml_string::Text, primitive: bool) -> f64 {
    if primitive {
        0f64
    } else {
        if (kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) {
            3f64
        } else {
            4f64
        }
    }
}

pub fn read_collection_construction_diagnostic(code: f64, part: self::ml_string::Text) -> self::ml_string::Text {
    if ((part == (self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101]))) && (code == (1f64))) {
        self::ml_string::Text::from_units(vec![83, 101, 116, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 97, 110, 32, 105, 116, 101, 114, 97, 98, 108, 101, 32, 97, 114, 114, 97, 121])
    } else {
        if ((part == (self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101]))) && (code == (2f64))) {
            self::ml_string::Text::from_units(vec![77, 97, 112, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 97, 110, 32, 105, 116, 101, 114, 97, 98, 108, 101, 32, 111, 102, 32, 107, 101, 121, 47, 118, 97, 108, 117, 101, 32, 112, 97, 105, 114, 115])
        } else {
            if ((part == (self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101]))) && (code == (3f64))) {
                self::ml_string::Text::from_units(vec![115, 101, 116, 32, 107, 101, 121, 115, 32, 110, 101, 101, 100, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 102, 111, 114, 32, 115, 116, 114, 117, 99, 116, 117, 114, 101, 100, 32, 118, 97, 108, 117, 101, 115])
            } else {
                if ((part == (self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101]))) && (code == (4f64))) {
                    self::ml_string::Text::from_units(vec![109, 97, 112, 32, 107, 101, 121, 115, 32, 110, 101, 101, 100, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 102, 111, 114, 32, 115, 116, 114, 117, 99, 116, 117, 114, 101, 100, 32, 118, 97, 108, 117, 101, 115])
                } else {
                    if ((part == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]))) && (code == (3f64))) {
                        self::ml_string::Text::from_units(vec![83, 101, 116, 32, 111, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                    } else {
                        if ((part == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]))) && (code == (4f64))) {
                            self::ml_string::Text::from_units(vec![77, 97, 112, 32, 111, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                        } else {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

pub fn read_collection_construction_binding(target: self::ml_string::Text, kind: self::ml_string::Text, role: self::ml_string::Text) -> self::ml_string::Text {
    {
        let set = (kind == (self::ml_string::Text::from_units(vec![115, 101, 116])));
        if (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113]))) {
            if (role == (self::ml_string::Text::from_units(vec![112, 114, 101, 118, 105, 111, 117, 115]))) {
                if set {
                    self::ml_string::Text::from_units(vec![109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115])
                } else {
                    self::ml_string::Text::from_units(vec![40, 102, 115, 116, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 41])
                }
            } else {
                if set {
                    self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])
                } else {
                    self::ml_string::Text::from_units(vec![40, 102, 115, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41])
                }
            }
        } else {
            if (role == (self::ml_string::Text::from_units(vec![112, 114, 101, 118, 105, 111, 117, 115]))) {
                if set {
                    self::ml_string::Text::from_units(vec![109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115])
                } else {
                    self::ml_string::Text::from_units(vec![109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 46, 49])
                }
            } else {
                if set {
                    self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])
                } else {
                    self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 46, 49])
                }
            }
        }
    }
}

pub fn render_collection_construction_type(target: self::ml_string::Text, kind: self::ml_string::Text, first: self::ml_string::Text, second: self::ml_string::Text) -> self::ml_string::Text {
    if (target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
        if (kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) {
            ((self::ml_string::Text::from_units(vec![40, 65, 114, 114, 97, 121, 32])).concat(&first)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            ((((self::ml_string::Text::from_units(vec![40, 65, 114, 114, 97, 121, 32, 40])).concat(&first)).concat(&self::ml_string::Text::from_units(vec![32, 215, 32]))).concat(&second)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
        }
    } else {
        if (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113]))) {
            if (kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) {
                ((self::ml_string::Text::from_units(vec![40, 108, 105, 115, 116, 32])).concat(&first)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                ((((self::ml_string::Text::from_units(vec![40, 108, 105, 115, 116, 32, 40])).concat(&first)).concat(&self::ml_string::Text::from_units(vec![32, 42, 32]))).concat(&second)).concat(&self::ml_string::Text::from_units(vec![41, 41, 37, 116, 121, 112, 101]))
            }
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn render_collection_construction(target: self::ml_string::Text, kind: self::ml_string::Text, key_kind: self::ml_string::Text, values: self::ml_string::Text, collection_type: self::ml_string::Text, element_type: self::ml_string::Text, comparison: self::ml_string::Text) -> self::ml_string::Text {
    {
        let set = (kind == (self::ml_string::Text::from_units(vec![115, 101, 116])));
        if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            ((((self::ml_string::Text::from_units(vec![110, 101, 119, 32])).concat(&if set {
                self::ml_string::Text::from_units(vec![83, 101, 116])
            } else {
                self::ml_string::Text::from_units(vec![77, 97, 112])
            })).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                {
                    let comparator = if (key_kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                        self::ml_string::Text::from_units(vec![124, 97, 58, 32, 38, 102, 54, 52, 44, 32, 98, 58, 32, 38, 102, 54, 52, 124, 32, 97, 32, 61, 61, 32, 98, 32, 124, 124, 32, 40, 97, 46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 38, 38, 32, 98, 46, 105, 115, 95, 110, 97, 110, 40, 41, 41])
                    } else {
                        self::ml_string::Text::from_units(vec![124, 97, 44, 32, 98, 124, 32, 97, 32, 61, 61, 32, 98])
                    };
                    {
                        let values_3 = if (key_kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                            {
                                let values_2 = ((((self::ml_string::Text::from_units(vec![40])).concat(&values)).concat(&self::ml_string::Text::from_units(vec![41, 46, 105, 110, 116, 111, 95, 105, 116, 101, 114, 40, 41, 46, 109, 97, 112, 40, 124, 109, 108, 95, 118, 97, 108, 117, 101, 124, 32]))).concat(&if set {
                                    self::ml_string::Text::from_units(vec![105, 102, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 61, 32, 48, 102, 54, 52, 32, 123, 32, 48, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 125])
                                } else {
                                    self::ml_string::Text::from_units(vec![40, 105, 102, 32, 109, 108, 95, 118, 97, 108, 117, 101, 46, 48, 32, 61, 61, 32, 48, 102, 54, 52, 32, 123, 32, 48, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 118, 97, 108, 117, 101, 46, 48, 32, 125, 44, 32, 109, 108, 95, 118, 97, 108, 117, 101, 46, 49, 41])
                                })).concat(&self::ml_string::Text::from_units(vec![41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 95, 62, 62, 40, 41]));
                                values_2.clone()
                            }
                        } else {
                            values.clone()
                        };
                        ((((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 97, 114, 114, 97, 121, 58, 58])).concat(&kind)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&values_3)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&comparator)).concat(&self::ml_string::Text::from_units(vec![41]))
                    }
                }
            } else {
                {
                    let value = self::read_collection_construction_binding(target.clone(), kind.clone(), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]));
                    {
                        let previous = self::read_collection_construction_binding(target.clone(), kind.clone(), self::ml_string::Text::from_units(vec![112, 114, 101, 118, 105, 111, 117, 115]));
                        if (target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                            {
                                let equal = if (key_kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                                    ((((((((self::ml_string::Text::from_units(vec![40])).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 124, 124, 32, 40]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 78, 97, 78, 32, 38, 38, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 78, 97, 78, 41, 41]))
                                } else {
                                    ((((self::ml_string::Text::from_units(vec![40])).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                };
                                {
                                    let normalized = if (key_kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                                        if set {
                                            self::ml_string::Text::from_units(vec![40, 105, 102, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 32, 61, 61, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 116, 104, 101, 110, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 101, 108, 115, 101, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 41])
                                        } else {
                                            self::ml_string::Text::from_units(vec![40, 40, 105, 102, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 46, 49, 32, 61, 61, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 116, 104, 101, 110, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 101, 108, 115, 101, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 46, 49, 41, 44, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 46, 50, 41])
                                        }
                                    } else {
                                        self::ml_string::Text::from_units(vec![109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108])
                                    };
                                    {
                                        let existing = if set {
                                            self::ml_string::Text::from_units(vec![109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![40, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 46, 109, 97, 112, 32, 40, 102, 117, 110, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 32, 61, 62, 32, 105, 102, 32])).concat(&equal)).concat(&self::ml_string::Text::from_units(vec![32, 116, 104, 101, 110, 32, 40, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 46, 49, 44, 32, 109, 108, 95, 118, 97, 108, 117, 101, 46, 50, 41, 32, 101, 108, 115, 101, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 41, 41]))
                                        };
                                        ((((((((((self::ml_string::Text::from_units(vec![40])).concat(&values)).concat(&self::ml_string::Text::from_units(vec![46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 32, 61, 62, 32, 108, 101, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 58, 61, 32]))).concat(&normalized)).concat(&self::ml_string::Text::from_units(vec![59, 32, 105, 102, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 46, 97, 110, 121, 32, 40, 102, 117, 110, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 32, 61, 62, 32]))).concat(&equal)).concat(&self::ml_string::Text::from_units(vec![41, 32, 116, 104, 101, 110, 32]))).concat(&existing)).concat(&self::ml_string::Text::from_units(vec![32, 101, 108, 115, 101, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 46, 112, 117, 115, 104, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41, 32, 40, 35, 91, 93, 32, 58, 32]))).concat(&collection_type)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                    }
                                }
                            }
                        } else {
                            {
                                let equal_2 = comparison.clone();
                                {
                                    let equal_4 = if (key_kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                                        {
                                            let equal_3 = ((((((self::ml_string::Text::from_units(vec![40, 111, 114, 98, 32])).concat(&equal_2)).concat(&self::ml_string::Text::from_units(vec![32, 40, 97, 110, 100, 98, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 105, 115, 95, 110, 97, 110, 32]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![41, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 105, 115, 95, 110, 97, 110, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41, 41]));
                                            equal_3.clone()
                                        }
                                    } else {
                                        equal_2.clone()
                                    };
                                    {
                                        let normalized_2 = if (key_kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                                            if set {
                                                self::ml_string::Text::from_units(vec![40, 105, 102, 32, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 101, 113, 98, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 32, 48, 37, 102, 108, 111, 97, 116, 32, 116, 104, 101, 110, 32, 48, 37, 102, 108, 111, 97, 116, 32, 101, 108, 115, 101, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 41])
                                            } else {
                                                self::ml_string::Text::from_units(vec![40, 40, 105, 102, 32, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 101, 113, 98, 32, 40, 102, 115, 116, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 41, 32, 48, 37, 102, 108, 111, 97, 116, 32, 116, 104, 101, 110, 32, 48, 37, 102, 108, 111, 97, 116, 32, 101, 108, 115, 101, 32, 102, 115, 116, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 41, 44, 32, 115, 110, 100, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 41])
                                            }
                                        } else {
                                            self::ml_string::Text::from_units(vec![109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108])
                                        };
                                        {
                                            let existing_2 = if set {
                                                self::ml_string::Text::from_units(vec![109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])
                                            } else {
                                                ((self::ml_string::Text::from_units(vec![40, 76, 105, 115, 116, 46, 109, 97, 112, 32, 40, 102, 117, 110, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 32, 61, 62, 32, 105, 102, 32])).concat(&equal_4)).concat(&self::ml_string::Text::from_units(vec![32, 116, 104, 101, 110, 32, 40, 102, 115, 116, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 44, 32, 115, 110, 100, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41, 32, 101, 108, 115, 101, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 41, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 41]))
                                            };
                                            ((((((((((self::ml_string::Text::from_units(vec![40, 76, 105, 115, 116, 46, 102, 111, 108, 100, 95, 108, 101, 102, 116, 32, 40, 102, 117, 110, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 32, 61, 62, 32, 108, 101, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 58, 61, 32])).concat(&normalized_2)).concat(&self::ml_string::Text::from_units(vec![32, 105, 110, 32, 105, 102, 32, 76, 105, 115, 116, 46, 101, 120, 105, 115, 116, 115, 98, 32, 40, 102, 117, 110, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 32, 61, 62, 32]))).concat(&equal_4)).concat(&self::ml_string::Text::from_units(vec![41, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 32, 116, 104, 101, 110, 32]))).concat(&existing_2)).concat(&self::ml_string::Text::from_units(vec![32, 101, 108, 115, 101, 32, 76, 105, 115, 116, 46, 97, 112, 112, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 32, 40, 109, 108, 95, 118, 97, 108, 117, 101, 32, 58, 58, 32, 110, 105, 108, 41, 41, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 40, 64, 110, 105, 108, 32]))).concat(&element_type)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_observation_operands(kind: self::ml_string::Text, proof: self::ml_string::Text) -> self::ml_string::Text {
    if self::ml_array::includes(&{ let ml_reference_value7 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 113]), self::ml_string::Text::from_units(vec![110, 101]), self::ml_string::Text::from_units(vec![108, 116]), self::ml_string::Text::from_units(vec![108, 101]), self::ml_string::Text::from_units(vec![103, 116]), self::ml_string::Text::from_units(vec![103, 101])]))).borrow().clone(); ml_reference_value7 }, &proof.clone(), 0f64, false, |a, b| a == b) {
        self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
    } else {
        if (proof == (self::ml_string::Text::from_units(vec![102, 111, 114, 97, 108, 108]))) {
            self::ml_string::Text::from_units(vec![98, 105, 110, 100, 101, 114, 115])
        } else {
            if ((kind == (self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]))) || (kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 84, 101, 120, 116])))) {
                self::ml_string::Text::from_units(vec![97, 114, 103])
            } else {
                if (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108]))) {
                    self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
                } else {
                    if ((kind == (self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116])))) {
                        self::ml_string::Text::from_units(vec![101, 120, 112, 114])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_observation_operation_operands(kind: self::ml_string::Text, proof: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if ((kind == (self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]))) && self::ml_array::includes(&{ let ml_reference_value8 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 113]), self::ml_string::Text::from_units(vec![110, 101])]))).borrow().clone(); ml_reference_value8 }, &operation.clone(), 0f64, false, |a, b| a == b)) {
        self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
    } else {
        self::read_java_script_closed_class_observation_operands(kind.clone(), proof.clone())
    }
}

pub fn contains_java_script_closed_class_brand(texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, root: f64, declarations: f64, brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> bool {
    if (({ let ml_reference_array9 = brands.clone(); let ml_reference_projection9 = ml_reference_array9.borrow().len() as f64; ml_reference_projection9 }) == (0f64)) {
        false
    } else {
        {
            let pending = std::rc::Rc::new(std::cell::RefCell::new(vec![root]));
            {
                let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                {
                    let source_ml_source2_s8 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, brands.clone(), pending.clone(), seen.clone());
                    match source_ml_source2_s8.clone() {
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done => {
                            false
                        }
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result2) => {
                            source_ml_source1_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

pub fn has_java_script_closed_class_exported_brand(texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, declarations: f64, exports: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> bool {
    if (({ let ml_reference_array10 = exports.clone(); let ml_reference_projection10 = ml_reference_array10.borrow().len() as f64; ml_reference_projection10 }) == (0f64)) {
        false
    } else {
        if (declarations < (0f64)) {
            true
        } else {
            {
                let source_ml_source1_temporary_values8 = exports.clone();
                {
                    let source_ml_source1_temporary_index8 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s9 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop9(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, brands.clone(), source_ml_source1_temporary_values8.clone(), source_ml_source1_temporary_index8.clone());
                        match source_ml_source2_s9.clone() {
                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done => {
                                false
                            }
                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(source_ml_source1_temporary_result9) => {
                                source_ml_source1_temporary_result9
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_record_field_assignment_operator(operator: self::ml_string::Text) -> bool {
    (((((((((operator == (self::ml_string::Text::from_units(vec![61]))) || (operator == (self::ml_string::Text::from_units(vec![43, 61])))) || (operator == (self::ml_string::Text::from_units(vec![45, 61])))) || (operator == (self::ml_string::Text::from_units(vec![42, 61])))) || (operator == (self::ml_string::Text::from_units(vec![47, 61])))) || (operator == (self::ml_string::Text::from_units(vec![37, 61])))) || (operator == (self::ml_string::Text::from_units(vec![42, 42, 61])))) || (operator == (self::ml_string::Text::from_units(vec![43, 43])))) || (operator == (self::ml_string::Text::from_units(vec![45, 45]))))
}

pub fn read_record_field_write_identity(operator: self::ml_string::Text, field: self::ml_string::Text) -> self::ml_string::Text {
    if self::accept_record_field_assignment_operator(operator.clone()) {
        (((self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100])).concat(&operator)).concat(&self::ml_string::Text::from_units(vec![58]))).concat(&field)
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_record_field_write_operator(operation: self::ml_string::Text) -> self::ml_string::Text {
    {
        let operators = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![43, 61]), self::ml_string::Text::from_units(vec![45, 61]), self::ml_string::Text::from_units(vec![42, 61]), self::ml_string::Text::from_units(vec![47, 61]), self::ml_string::Text::from_units(vec![37, 61]), self::ml_string::Text::from_units(vec![42, 42, 61]), self::ml_string::Text::from_units(vec![43, 43]), self::ml_string::Text::from_units(vec![45, 45])]));
        {
            let source_ml_source1_temporary_values12 = operators.clone();
            {
                let source_ml_source1_temporary_index12 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s10 = self::source_ml_source1_temporary_read_record_field_write_operator_loop13(operation.clone(), source_ml_source1_temporary_values12.clone(), source_ml_source1_temporary_index12.clone());
                    match source_ml_source2_s10.clone() {
                        self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Done => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Return(source_ml_source1_temporary_result13) => {
                            source_ml_source1_temporary_result13.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn read_record_field_write_name(operation: self::ml_string::Text) -> self::ml_string::Text {
    {
        let operator = self::read_record_field_write_operator(operation.clone());
        if (operator == (self::ml_string::Text::from_units(vec![]))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let source_ml_source2_string_range_operand11 = operation.clone();
                {
                    let source_ml_source2_string_range_operand12 = (17f64 + operator.length());
                    {
                        let source_ml_source2_string_range_position13 = source_ml_source2_string_range_operand12;
                        {
                            let source_ml_source2_string_range_position14 = f64::INFINITY;
                            source_ml_source2_string_range_operand11.slice(source_ml_source2_string_range_position13, source_ml_source2_string_range_position14)
                        }
                    }
                }
            }
        }
    }
}

pub fn read_record_field_write_argument_count(operator: self::ml_string::Text) -> f64 {
    if ((operator == (self::ml_string::Text::from_units(vec![43, 43]))) || (operator == (self::ml_string::Text::from_units(vec![45, 45])))) {
        0f64
    } else {
        1f64
    }
}

pub fn read_record_field_write_suffix(initial: f64, receiver: self::ml_string::Text, arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> f64 {
    {
        let text = (receiver.concat(&self::ml_string::Text::from_units(vec![32]))).concat(&self::ml_string::join(&{ let ml_reference_value11 = arguments_.borrow().clone(); ml_reference_value11 }, &self::ml_string::Text::from_units(vec![32])));
        {
            let suffix = initial;
            {
                let suffix_2 = self::source_ml_source1_temporary_read_record_field_write_suffix_loop14(text.clone(), suffix);
                suffix_2
            }
        }
    }
}

pub fn accept_linked_record_field_write(expression: self::ml_string::Text, operation: self::ml_string::Text, kind: self::ml_string::Text, reference: bool) -> bool {
    ((((expression == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) && (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])))) && reference) && ((self::read_record_field_write_operator(operation.clone())) != (self::ml_string::Text::from_units(vec![]))))
}

pub fn read_linked_ordered_record_snapshot_operation(expression: self::ml_string::Text, kind: self::ml_string::Text, ordered: bool) -> self::ml_string::Text {
    if (((expression == (self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101, 82, 101, 97, 100]))) && (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])))) && ordered) {
        self::ml_string::Text::from_units(vec![114, 101, 97, 100, 79, 114, 100, 101, 114, 101, 100, 82, 101, 99, 111, 114, 100, 83, 110, 97, 112, 115, 104, 111, 116])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_record_field_write_refusal(operator: self::ml_string::Text, kind: self::ml_string::Text, reference: bool, open: bool, field: bool, value_kind: self::ml_string::Text) -> self::ml_string::Text {
    if !(self::accept_record_field_assignment_operator(operator.clone())) {
        self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 111, 112, 101, 114, 97, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 116, 97, 116, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
    } else {
        if (((kind != (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) || !reference) || open) {
            self::ml_string::Text::from_units(vec![97, 32, 102, 105, 101, 108, 100, 32, 119, 114, 105, 116, 101, 32, 110, 101, 101, 100, 115, 32, 97, 32, 99, 108, 111, 115, 101, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 119, 105, 116, 104, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 115])
        } else {
            if !field {
                self::ml_string::Text::from_units(vec![99, 114, 101, 97, 116, 105, 110, 103, 32, 97, 32, 102, 105, 101, 108, 100, 32, 110, 101, 101, 100, 115, 32, 97, 32, 100, 121, 110, 97, 109, 105, 99, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 97, 110, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110])
            } else {
                if ((operator != (self::ml_string::Text::from_units(vec![61]))) && (value_kind != (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))) {
                    self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 99, 111, 109, 112, 111, 117, 110, 100, 32, 102, 105, 101, 108, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 101, 114, 99, 105, 111, 110, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101, 32, 100, 111, 109, 97, 105, 110])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn read_record_compound_binary_operation(language: self::ml_string::Text, operator: self::ml_string::Text, kind: self::ml_string::Text) -> self::ml_string::Text {
    if ((language != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) || ((kind != (self::ml_string::Text::from_units(vec![105, 110, 116]))) && (kind != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
            if (operator == (self::ml_string::Text::from_units(vec![43, 61]))) {
                self::ml_string::Text::from_units(vec![112, 108, 117, 115])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        } else {
            self::read_record_numeric_compound_binary_operation(operator.clone())
        }
    }
}

pub fn read_record_numeric_compound_binary_operation(operator: self::ml_string::Text) -> self::ml_string::Text {
    if ((operator == (self::ml_string::Text::from_units(vec![43, 61]))) || (operator == (self::ml_string::Text::from_units(vec![43, 43])))) {
        self::ml_string::Text::from_units(vec![112, 108, 117, 115])
    } else {
        if ((operator == (self::ml_string::Text::from_units(vec![45, 61]))) || (operator == (self::ml_string::Text::from_units(vec![45, 45])))) {
            self::ml_string::Text::from_units(vec![115, 117, 98])
        } else {
            if (operator == (self::ml_string::Text::from_units(vec![42, 61]))) {
                self::ml_string::Text::from_units(vec![109, 117, 108])
            } else {
                if (operator == (self::ml_string::Text::from_units(vec![47, 61]))) {
                    self::ml_string::Text::from_units(vec![100, 105, 118])
                } else {
                    if (operator == (self::ml_string::Text::from_units(vec![37, 61]))) {
                        self::ml_string::Text::from_units(vec![114, 101, 109])
                    } else {
                        if (operator == (self::ml_string::Text::from_units(vec![42, 42, 61]))) {
                            self::ml_string::Text::from_units(vec![101, 120, 112, 111, 110, 101, 110, 116, 105, 97, 116, 105, 111, 110])
                        } else {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

pub fn read_optional_record_compound_binary_operation(language: self::ml_string::Text, operator: self::ml_string::Text, kind: self::ml_string::Text, absence: self::ml_string::Text) -> self::ml_string::Text {
    if ((language != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) || ((absence != (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))) && (absence != (self::ml_string::Text::from_units(vec![98, 111, 116, 104]))))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
            self::read_record_numeric_compound_binary_operation(operator.clone())
        } else {
            if (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                self::read_record_compound_binary_operation(language.clone(), operator.clone(), kind.clone())
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_record_field_source_write_refusal(language: self::ml_string::Text, operator: self::ml_string::Text, kind: self::ml_string::Text, reference: bool, open: bool, field: bool, value_kind: self::ml_string::Text) -> self::ml_string::Text {
    {
        let operation = self::read_record_compound_binary_operation(language.clone(), operator.clone(), value_kind.clone());
        if (operation != (self::ml_string::Text::from_units(vec![]))) {
            if (((kind != (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) || !reference) || open) {
                self::ml_string::Text::from_units(vec![97, 32, 102, 105, 101, 108, 100, 32, 119, 114, 105, 116, 101, 32, 110, 101, 101, 100, 115, 32, 97, 32, 99, 108, 111, 115, 101, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 119, 105, 116, 104, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 115])
            } else {
                if !field {
                    self::ml_string::Text::from_units(vec![99, 114, 101, 97, 116, 105, 110, 103, 32, 97, 32, 102, 105, 101, 108, 100, 32, 110, 101, 101, 100, 115, 32, 97, 32, 100, 121, 110, 97, 109, 105, 99, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 97, 110, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        } else {
            self::read_record_field_write_refusal(operator.clone(), kind.clone(), reference, open, field, value_kind.clone())
        }
    }
}

pub fn render_record_field_write(target: self::ml_string::Text, operator: self::ml_string::Text, receiver: self::ml_string::Text, field: self::ml_string::Text, index: self::ml_string::Text, arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, suffix: self::ml_string::Text) -> self::ml_string::Text {
    {
        let owner = (self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 111, 119, 110, 101, 114])).concat(&suffix);
        {
            let value = (self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101])).concat(&suffix);
            {
                let previous = (self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 112, 114, 101, 118, 105, 111, 117, 115])).concat(&suffix);
                {
                    let unary = ((operator == (self::ml_string::Text::from_units(vec![43, 43]))) || (operator == (self::ml_string::Text::from_units(vec![45, 45]))));
                    if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
                        (((((((self::ml_string::Text::from_units(vec![40, 118, 111, 105, 100, 32, 40, 40])).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![41, 91]))).concat(&field)).concat(&self::ml_string::Text::from_units(vec![93]))).concat(&operator)).concat(&if unary {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            ((self::ml_string::Text::from_units(vec![40])).concat(&({
                                let source_ml_source2_array_read_value15 = arguments_.clone();
                                {
                                    let source_ml_source2_array_read_index16 = 0f64;
                                    { let ml_reference_array12 = source_ml_source2_array_read_value15.clone(); let ml_reference_position12 = source_ml_source2_array_read_index16; let ml_reference_projection12 = self::ml_array::optional_at(&*ml_reference_array12.borrow(), self::ml_array::number_index(ml_reference_position12)); ml_reference_projection12 }
                                }
                            }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![41]))
                        })).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                    } else {
                        if (target != (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            {
                                let output = ((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                                {
                                    let output_3 = if (operator != (self::ml_string::Text::from_units(vec![61]))) {
                                        {
                                            let output_2 = output.concat(&((((((self::ml_string::Text::from_units(vec![108, 101, 116, 32])).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 46]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                            output_2.clone()
                                        }
                                    } else {
                                        output.clone()
                                    };
                                    {
                                        let output_5 = if !unary {
                                            {
                                                let output_4 = output_3.concat(&((((self::ml_string::Text::from_units(vec![108, 101, 116, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&({
                                                    let source_ml_source2_array_read_value17 = arguments_.clone();
                                                    {
                                                        let source_ml_source2_array_read_index18 = 0f64;
                                                        { let ml_reference_array13 = source_ml_source2_array_read_value17.clone(); let ml_reference_position13 = source_ml_source2_array_read_index18; let ml_reference_projection13 = self::ml_array::optional_at(&*ml_reference_array13.borrow(), self::ml_array::number_index(ml_reference_position13)); ml_reference_projection13 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                output_4.clone()
                                            }
                                        } else {
                                            output_3.clone()
                                        };
                                        {
                                            let result = if (operator == (self::ml_string::Text::from_units(vec![61]))) {
                                                value.clone()
                                            } else {
                                                if (operator == (self::ml_string::Text::from_units(vec![43, 43]))) {
                                                    previous.concat(&self::ml_string::Text::from_units(vec![32, 43, 32, 49, 46, 48]))
                                                } else {
                                                    if (operator == (self::ml_string::Text::from_units(vec![45, 45]))) {
                                                        previous.concat(&self::ml_string::Text::from_units(vec![32, 45, 32, 49, 46, 48]))
                                                    } else {
                                                        if (operator == (self::ml_string::Text::from_units(vec![42, 42, 61]))) {
                                                            ((((((((((((((self::ml_string::Text::from_units(vec![105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32, 48, 46, 48, 32, 123, 32, 49, 46, 48, 32, 125, 32, 101, 108, 115, 101, 32, 105, 102, 32]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 124, 124, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 124, 124, 32, 40]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![46, 97, 98, 115, 40, 41, 32, 61, 61, 32, 49, 46, 48, 32, 38, 38, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 105, 110, 102, 105, 110, 105, 116, 101, 40, 41, 41, 32, 123, 32, 102, 54, 52, 58, 58, 78, 65, 78, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![46, 112, 111, 119, 102, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125]))
                                                        } else {
                                                            (((previous.concat(&self::ml_string::Text::from_units(vec![32]))).concat(&{
                                                                let source_ml_source2_string_range_operand19 = operator.clone();
                                                                {
                                                                    let source_ml_source2_string_range_operand20 = 0f64;
                                                                    {
                                                                        let source_ml_source2_string_range_operand21 = 1f64;
                                                                        {
                                                                            let source_ml_source2_string_range_position22 = source_ml_source2_string_range_operand20;
                                                                            {
                                                                                let source_ml_source2_string_range_position23 = source_ml_source2_string_range_operand21;
                                                                                source_ml_source2_string_range_operand19.slice(source_ml_source2_string_range_position22, source_ml_source2_string_range_position23)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            })).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&value)
                                                        }
                                                    }
                                                }
                                            };
                                            (((((output_5.concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 46]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&result)).concat(&self::ml_string::Text::from_units(vec![59, 32, 40, 41, 32, 125]))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_optional_record_field_order_followers(numeric_keys: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, field: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let selected = ({
            let source_ml_source2_array_read_value24 = numeric_keys.clone();
            {
                let source_ml_source2_array_read_index25 = field;
                { let ml_reference_array14 = source_ml_source2_array_read_value24.clone(); let ml_reference_position14 = source_ml_source2_array_read_index25; let ml_reference_projection14 = self::ml_array::optional_at(&*ml_reference_array14.borrow(), self::ml_array::number_index(ml_reference_position14)); ml_reference_projection14 }
            }
        }).clone().unwrap_or_else(|| (-1f64));
        {
            let followers = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            if (selected < (0f64)) {
                followers.clone()
            } else {
                {
                    let index = 0f64;
                    {
                        let followers_2 = self::source_ml_source1_temporary_read_optional_record_field_order_followers_loop15(numeric_keys.clone(), selected, followers.clone(), index);
                        followers_2.clone()
                    }
                }
            }
        }
    }
}

pub fn render_optional_rust_record_field_write(receiver: self::ml_string::Text, index: self::ml_string::Text, argument: self::ml_string::Text, suffix: self::ml_string::Text, ordered: bool, followers: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> self::ml_string::Text {
    {
        let owner = (self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 111, 119, 110, 101, 114])).concat(&suffix);
        {
            let value = (self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101])).concat(&suffix);
            {
                let capture = ((((((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&argument)).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                {
                    let storage = ((((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 117, 116, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32]));
                    if !ordered {
                        ((((((capture.concat(&storage)).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 83, 111, 109, 101, 40, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![44, 41, 41, 59, 32, 40, 41, 32, 125]))
                    } else {
                        {
                            let field = (owner.concat(&self::ml_string::Text::from_units(vec![46, 48, 46]))).concat(&index);
                            {
                                let order = owner.concat(&self::ml_string::Text::from_units(vec![46, 49]));
                                {
                                    let position = if (({ let ml_reference_array15 = followers.clone(); let ml_reference_projection15 = ml_reference_array15.borrow().len() as f64; ml_reference_projection15 }) == (0f64)) {
                                        order.concat(&self::ml_string::Text::from_units(vec![46, 108, 101, 110, 40, 41]))
                                    } else {
                                        ((((order.concat(&self::ml_string::Text::from_units(vec![46, 105, 116, 101, 114, 40, 41, 46, 112, 111, 115, 105, 116, 105, 111, 110, 40, 124, 109, 108, 95, 111, 114, 100, 101, 114, 101, 100, 95, 102, 105, 101, 108, 100, 124, 32, 91]))).concat(&self::ml_string::join(&{ let ml_reference_value16 = followers.borrow().clone(); ml_reference_value16 }, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![93, 46, 99, 111, 110, 116, 97, 105, 110, 115, 40, 109, 108, 95, 111, 114, 100, 101, 114, 101, 100, 95, 102, 105, 101, 108, 100, 41, 41, 46, 117, 110, 119, 114, 97, 112, 95, 111, 114, 40]))).concat(&order)).concat(&self::ml_string::Text::from_units(vec![46, 108, 101, 110, 40, 41, 41]))
                                    };
                                    {
                                        let next = (self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 112, 114, 101, 118, 105, 111, 117, 115])).concat(&suffix);
                                        (((((((((((((((((capture.concat(&storage)).concat(&self::ml_string::Text::from_units(vec![105, 102, 32]))).concat(&field)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 110, 111, 110, 101, 40, 41, 32, 123, 32, 108, 101, 116, 32]))).concat(&next)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![59, 32]))).concat(&order)).concat(&self::ml_string::Text::from_units(vec![46, 105, 110, 115, 101, 114, 116, 40]))).concat(&next)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![46, 48, 41, 59, 32, 125, 32]))).concat(&field)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 83, 111, 109, 101, 40, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![44, 41, 41, 59, 32, 40, 41, 32, 125]))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_record_field_write_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![65, 110, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 32, 119, 114, 105, 116, 101, 32, 101, 118, 97, 108, 117, 97, 116, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 111, 110, 99, 101, 44, 32, 114, 101, 97, 100, 115, 32, 97, 32, 99, 111, 109, 112, 111, 117, 110, 100, 32, 111, 112, 101, 114, 97, 110, 100, 32, 98, 101, 102, 111, 114, 101, 32, 116, 104, 101, 32, 114, 105, 103, 104, 116, 45, 104, 97, 110, 100, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 44, 32, 101, 118, 97, 108, 117, 97, 116, 101, 115, 32, 116, 104, 97, 116, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 111, 110, 99, 101, 44, 32, 116, 104, 101, 110, 32, 119, 114, 105, 116, 101, 115, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 102, 105, 101, 108, 100, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 97, 102, 116, 101, 114, 32, 114, 101, 108, 101, 97, 115, 105, 110, 103, 32, 101, 118, 101, 114, 121, 32, 101, 97, 114, 108, 105, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 111, 114, 114, 111, 119, 59, 32, 105, 116, 115, 32, 115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 114, 101, 115, 117, 108, 116, 32, 105, 115, 32, 85, 110, 105, 116, 46])
}

pub fn read_java_script_class_owner_model() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 32, 99, 108, 97, 115, 115, 32, 100, 97, 116, 97, 32, 111, 119, 110, 101, 114, 115, 32, 49])
}

pub fn read_java_script_class_owner_inference(left: self::ml_string::Text, right: self::ml_string::Text, left_open: bool, right_open: bool) -> self::ml_string::Text {
    if (left == right) {
        self::ml_string::Text::from_units(vec![115, 97, 109, 101])
    } else {
        if (((left != (self::ml_string::Text::from_units(vec![]))) && (right == (self::ml_string::Text::from_units(vec![])))) && right_open) {
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116])
        } else {
            if (((right != (self::ml_string::Text::from_units(vec![]))) && (left == (self::ml_string::Text::from_units(vec![])))) && left_open) {
                self::ml_string::Text::from_units(vec![108, 101, 102, 116])
            } else {
                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
            }
        }
    }
}

pub fn read_java_script_class_record_owner(constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, fields: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> self::ml_string::Text {
    {
        let owner = self::ml_string::Text::from_units(vec![]);
        {
            let at = 0f64;
            {
                let source_ml_source2_s26 = self::source_ml_source1_temporary_read_java_script_class_record_owner_loop16(constructors.clone(), brands.clone(), fields.clone(), owner.clone(), at);
                match source_ml_source2_s26.clone() {
                    self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Done(owner_2) => {
                        owner_2.clone()
                    }
                    self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Return(source_ml_source1_temporary_result16) => {
                        source_ml_source1_temporary_result16.clone()
                    }
                }
            }
        }
    }
}

pub fn accept_java_script_class_owner_observation(representation: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    ((representation == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115]))) && (operation == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121]))))
}

pub fn word(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64, name: self::ml_string::Text) -> self::ml_string::Text {
    {
        let value = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, name.clone());
        if ((value >= (0f64)) && ((self::dependency_function2(forms.clone(), value)) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
            self::dependency_function2(texts.clone(), value)
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn exact(starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64, names_2: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> bool {
    if ((node < (0f64)) || ((self::dependency_function1(counts.clone(), node)) != ({ let ml_reference_array17 = names_2.clone(); let ml_reference_projection17 = ml_reference_array17.borrow().len() as f64; ml_reference_projection17 }))) {
        false
    } else {
        {
            let source_ml_source1_temporary_values17 = names_2.clone();
            {
                let source_ml_source1_temporary_index17 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s27 = self::source_ml_source1_temporary_exact_loop18(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, source_ml_source1_temporary_values17.clone(), source_ml_source1_temporary_index17.clone());
                    match source_ml_source2_s27.clone() {
                        self::MlSource1TemporaryExactLoop18Result::MlSource1TemporaryExactLoop18Done => {
                            true
                        }
                        self::MlSource1TemporaryExactLoop18Result::MlSource1TemporaryExactLoop18Return(source_ml_source1_temporary_result18) => {
                            source_ml_source1_temporary_result18
                        }
                    }
                }
            }
        }
    }
}

pub fn strings(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
        if ((node < (0f64)) || ((self::dependency_function2(forms.clone(), node)) != (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![])]))
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s28 = self::source_ml_source1_temporary_strings_loop19(forms.clone(), texts.clone(), starts.clone(), counts.clone(), targets.clone(), node, result.clone(), at);
                    match source_ml_source2_s28.clone() {
                        self::MlSource1TemporaryStringsLoop19Result::MlSource1TemporaryStringsLoop19Done(result_2) => {
                            result_2.clone()
                        }
                        self::MlSource1TemporaryStringsLoop19Result::MlSource1TemporaryStringsLoop19Return(source_ml_source1_temporary_result19) => {
                            source_ml_source1_temporary_result19.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn names(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
        {
            let fields = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
            if (fields < (0f64)) {
                std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![])]))
            } else {
                {
                    let at = 0f64;
                    {
                        let result_2 = self::source_ml_source1_temporary_names_loop20(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), result.clone(), fields, at);
                        result_2.clone()
                    }
                }
            }
        }
    }
}

pub fn carries(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, type_: f64, owner: self::ml_string::Text, depth: f64) -> bool {
    if ((type_ < (0f64)) || (depth > (128f64))) {
        false
    } else {
        if ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]))) == owner) {
            true
        } else {
            if ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, self::ml_string::Text::from_units(vec![107, 105, 110, 100]))) != (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) {
                false
            } else {
                {
                    let elements = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 115]));
                    if (elements < (0f64)) {
                        false
                    } else {
                        {
                            let at = 0f64;
                            {
                                let source_ml_source2_s29 = self::source_ml_source1_temporary_carries_loop21(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner.clone(), depth, elements, at);
                                match source_ml_source2_s29.clone() {
                                    self::MlSource1TemporaryCarriesLoop21Result::MlSource1TemporaryCarriesLoop21Done => {
                                        false
                                    }
                                    self::MlSource1TemporaryCarriesLoop21Result::MlSource1TemporaryCarriesLoop21Return(source_ml_source1_temporary_result21) => {
                                        source_ml_source1_temporary_result21
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn payload(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64, owner: self::ml_string::Text, depth: f64) -> bool {
    if ((node < (0f64)) || (depth > (256f64))) {
        false
    } else {
        {
            let kind = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107]));
            if (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
                ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![116, 121, 112, 101])), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]))) == owner)
            } else {
                if (((kind == (self::ml_string::Text::from_units(vec![108, 101, 116]))) || (kind == (self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116])))) || (kind == (self::ml_string::Text::from_units(vec![99, 101, 108, 108, 87, 114, 105, 116, 101])))) {
                    self::payload(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![98, 111, 100, 121])), owner.clone(), (depth + 1f64))
                } else {
                    if (kind == (self::ml_string::Text::from_units(vec![105, 102]))) {
                        (self::payload(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![116, 104, 101, 110])), owner.clone(), (depth + 1f64)) && self::payload(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![101, 108, 115, 101])), owner.clone(), (depth + 1f64)))
                    } else {
                        false
                    }
                }
            }
        }
    }
}

pub fn fresh(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64, owner: self::ml_string::Text, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, depth: f64) -> f64 {
    if ((node < (0f64)) || (depth > (256f64))) {
        (-1f64)
    } else {
        {
            let type_ = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![116, 121, 112, 101]));
            if !(self::carries(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, owner.clone(), 0f64)) {
                (-2f64)
            } else {
                {
                    let kind = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107]));
                    if (kind == (self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101, 65, 108, 108, 111, 99, 97, 116, 101]))) {
                        {
                            let reference = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]));
                            if ((((reference >= (0f64)) && ((self::dependency_function2(forms.clone(), reference)) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) && ((self::dependency_function2(texts.clone(), reference)) == (self::ml_string::Text::from_units(vec![116, 114, 117, 101])))) && self::payload(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])), owner.clone(), 0f64)) {
                                node
                            } else {
                                (-1f64)
                            }
                        }
                    } else {
                        if (kind == (self::ml_string::Text::from_units(vec![118, 97, 114]))) {
                            {
                                let name = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![110, 97, 109, 101]));
                                {
                                    let at = ({ let ml_reference_array18 = bindings.clone(); let ml_reference_projection18 = ml_reference_array18.borrow().len() as f64; ml_reference_projection18 } - 1f64);
                                    {
                                        let source_ml_source2_s30 = self::source_ml_source1_temporary_fresh_loop22(bindings.clone(), values.clone(), name.clone(), at);
                                        match source_ml_source2_s30.clone() {
                                            self::MlSource1TemporaryFreshLoop22Result::MlSource1TemporaryFreshLoop22Done => {
                                                (-1f64)
                                            }
                                            self::MlSource1TemporaryFreshLoop22Result::MlSource1TemporaryFreshLoop22Return(source_ml_source1_temporary_result22) => {
                                                source_ml_source1_temporary_result22
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if (kind == (self::ml_string::Text::from_units(vec![108, 101, 116]))) {
                                {
                                    let value = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]));
                                    {
                                        let origin = self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), value, owner.clone(), bindings.clone(), values.clone(), (depth + 1f64));
                                        self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![98, 111, 100, 121])), owner.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value31 = bindings.clone();
                                            {
                                                let source_ml_source2_concat_value32 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![110, 97, 109, 101]))]));
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value19 = source_ml_source2_concat_value31.borrow().clone(); ml_reference_value19 }), { let ml_reference_value20 = source_ml_source2_concat_value32.borrow().clone(); ml_reference_value20 })
                                            }
                                        })), std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value33 = values.clone();
                                            {
                                                let source_ml_source2_concat_value34 = std::rc::Rc::new(std::cell::RefCell::new(vec![origin]));
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value21 = source_ml_source2_concat_value33.borrow().clone(); ml_reference_value21 }), { let ml_reference_value22 = source_ml_source2_concat_value34.borrow().clone(); ml_reference_value22 })
                                            }
                                        })), (depth + 1f64))
                                    }
                                }
                            } else {
                                if ((kind == (self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![99, 101, 108, 108, 87, 114, 105, 116, 101])))) {
                                    self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![98, 111, 100, 121])), owner.clone(), bindings.clone(), values.clone(), (depth + 1f64))
                                } else {
                                    if (kind == (self::ml_string::Text::from_units(vec![105, 102]))) {
                                        {
                                            let left = self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![116, 104, 101, 110])), owner.clone(), bindings.clone(), values.clone(), (depth + 1f64));
                                            {
                                                let right = self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![101, 108, 115, 101])), owner.clone(), bindings.clone(), values.clone(), (depth + 1f64));
                                                if ((left >= (0f64)) && (left == right)) {
                                                    left
                                                } else {
                                                    (-1f64)
                                                }
                                            }
                                        }
                                    } else {
                                        if (kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) {
                                            {
                                                let origin_2 = (-2f64);
                                                {
                                                    let items = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![105, 116, 101, 109, 115]));
                                                    if (items < (0f64)) {
                                                        (-1f64)
                                                    } else {
                                                        {
                                                            let at_2 = 0f64;
                                                            {
                                                                let source_ml_source2_s35 = self::source_ml_source1_temporary_fresh_loop23(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner.clone(), bindings.clone(), values.clone(), depth, origin_2, items, at_2);
                                                                match source_ml_source2_s35.clone() {
                                                                    self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Done(origin_3) => {
                                                                        origin_3
                                                                    }
                                                                    self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Return(source_ml_source1_temporary_result23) => {
                                                                        source_ml_source1_temporary_result23
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if (kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101, 73, 110, 100, 101, 120]))) {
                                                self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])), owner.clone(), bindings.clone(), values.clone(), (depth + 1f64))
                                            } else {
                                                (-1f64)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_class_owner_error(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, type_keys: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, root: f64) -> self::ml_string::Text {
    if ((((((({ let ml_reference_array23 = starts.clone(); let ml_reference_projection23 = ml_reference_array23.borrow().len() as f64; ml_reference_projection23 }) > (1048576f64)) || (({ let ml_reference_array24 = starts.clone(); let ml_reference_projection24 = ml_reference_array24.borrow().len() as f64; ml_reference_projection24 }) != ({ let ml_reference_array25 = forms.clone(); let ml_reference_projection25 = ml_reference_array25.borrow().len() as f64; ml_reference_projection25 }))) || (({ let ml_reference_array26 = starts.clone(); let ml_reference_projection26 = ml_reference_array26.borrow().len() as f64; ml_reference_projection26 }) != ({ let ml_reference_array27 = texts.clone(); let ml_reference_projection27 = ml_reference_array27.borrow().len() as f64; ml_reference_projection27 }))) || (({ let ml_reference_array28 = starts.clone(); let ml_reference_projection28 = ml_reference_array28.borrow().len() as f64; ml_reference_projection28 }) != ({ let ml_reference_array29 = counts.clone(); let ml_reference_projection29 = ml_reference_array29.borrow().len() as f64; ml_reference_projection29 }))) || (({ let ml_reference_array30 = starts.clone(); let ml_reference_projection30 = ml_reference_array30.borrow().len() as f64; ml_reference_projection30 }) != ({ let ml_reference_array31 = type_keys.clone(); let ml_reference_projection31 = ml_reference_array31.borrow().len() as f64; ml_reference_projection31 }))) || (({ let ml_reference_array32 = labels.clone(); let ml_reference_projection32 = ml_reference_array32.borrow().len() as f64; ml_reference_projection32 }) != ({ let ml_reference_array33 = targets.clone(); let ml_reference_projection33 = ml_reference_array33.borrow().len() as f64; ml_reference_projection33 }))) {
        self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 115, 104, 105, 112, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 114, 97, 112, 104, 32, 98, 117, 100, 103, 101, 116])
    } else {
        {
            let witness = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), root, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114, 115]));
            {
                let declarations = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), root, self::ml_string::Text::from_units(vec![100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115]));
                {
                    let constructors = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                    {
                        let brands = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                        {
                            let representations = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                            {
                                let owner_nodes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                {
                                    let constructor_nodes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let source_ml_source2_s37 = if (witness >= (0f64)) {
                                            if ((((((((self::dependency_function2(forms.clone(), witness)) != (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) || !(self::exact(starts.clone(), counts.clone(), labels.clone(), targets.clone(), witness, std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 99, 104, 101, 109, 97, 86, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![108, 97, 110, 103, 117, 97, 103, 101]), self::ml_string::Text::from_units(vec![109, 111, 100, 101, 108]), self::ml_string::Text::from_units(vec![111, 119, 110, 101, 114, 115])]))))) || ((self::dependency_function2(forms.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), witness, self::ml_string::Text::from_units(vec![115, 99, 104, 101, 109, 97, 86, 101, 114, 115, 105, 111, 110])))) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) || ((self::dependency_function2(texts.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), witness, self::ml_string::Text::from_units(vec![115, 99, 104, 101, 109, 97, 86, 101, 114, 115, 105, 111, 110])))) != (self::ml_string::Text::from_units(vec![49])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), witness, self::ml_string::Text::from_units(vec![108, 97, 110, 103, 117, 97, 103, 101]))) != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), root, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 76, 97, 110, 103, 117, 97, 103, 101]))) != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), witness, self::ml_string::Text::from_units(vec![109, 111, 100, 101, 108]))) != (self::read_java_script_class_owner_model()))) {
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(self::ml_string::Text::from_units(vec![117, 110, 114, 101, 99, 111, 103, 110, 105, 122, 101, 100, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 119, 105, 116, 110, 101, 115, 115, 32, 104, 101, 97, 100, 101, 114]))
                                            } else {
                                                {
                                                    let owners = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), witness, self::ml_string::Text::from_units(vec![111, 119, 110, 101, 114, 115]));
                                                    if (((((owners < (0f64)) || ((self::dependency_function2(forms.clone(), owners)) != (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) || ((self::dependency_function1(counts.clone(), owners)) < (1f64))) || ((self::dependency_function1(counts.clone(), owners)) > (64f64))) || (declarations < (0f64))) {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 115, 32, 97, 110, 100, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 97, 114, 101, 32, 114, 101, 113, 117, 105, 114, 101, 100]))
                                                    } else {
                                                        {
                                                            let at = 0f64;
                                                            {
                                                                let source_ml_source2_s38 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop46(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructors.clone(), brands.clone(), representations.clone(), owner_nodes.clone(), constructor_nodes.clone(), owners, at);
                                                                match source_ml_source2_s38.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Done(constructors_2, brands_2, representations_2, owner_nodes_2, constructor_nodes_2) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(constructors_2.clone(), brands_2.clone(), representations_2.clone(), owner_nodes_2.clone(), constructor_nodes_2.clone())
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(source_ml_source1_temporary_result46) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(source_ml_source1_temporary_result46.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(constructors.clone(), brands.clone(), representations.clone(), owner_nodes.clone(), constructor_nodes.clone())
                                        };
                                        match source_ml_source2_s37.clone() {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(constructors_3, brands_3, representations_3, owner_nodes_3, constructor_nodes_3) => {
                                                {
                                                    let node = 0f64;
                                                    {
                                                        let source_ml_source2_s39 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop28(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_keys.clone(), constructors_3.clone(), brands_3.clone(), representations_3.clone(), owner_nodes_3.clone(), constructor_nodes_3.clone(), node);
                                                        match source_ml_source2_s39.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Done => {
                                                                if (({ let ml_reference_array34 = constructors_3.clone(); let ml_reference_projection34 = ml_reference_array34.borrow().len() as f64; ml_reference_projection34 }) == (0f64)) {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                } else {
                                                                    {
                                                                        let roots = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), root, self::ml_string::Text::from_units(vec![109, 97, 105, 110]))]));
                                                                        {
                                                                            let scopes = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![])]));
                                                                            {
                                                                                let declaration = 0f64;
                                                                                {
                                                                                    let source_ml_source2_s40 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop29(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, roots.clone(), scopes.clone(), declaration);
                                                                                    match source_ml_source2_s40.clone() {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done(roots_2, scopes_2) => {
                                                                                            {
                                                                                                let scope = 0f64;
                                                                                                {
                                                                                                    let source_ml_source2_s41 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop30(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), constructors_3.clone(), brands_3.clone(), representations_3.clone(), owner_nodes_3.clone(), roots_2.clone(), scopes_2.clone(), scope);
                                                                                                    match source_ml_source2_s41.clone() {
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done => {
                                                                                                            {
                                                                                                                let node_2 = 0f64;
                                                                                                                {
                                                                                                                    let source_ml_source2_s42 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop31(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructors_3.clone(), brands_3.clone(), representations_3.clone(), node_2);
                                                                                                                    match source_ml_source2_s42.clone() {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done => {
                                                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                                                        }
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(source_ml_source1_temporary_result31) => {
                                                                                                                            source_ml_source1_temporary_result31.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(source_ml_source1_temporary_result37) => {
                                                                                                            source_ml_source1_temporary_result37.clone()
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(source_ml_source1_temporary_result41) => {
                                                                source_ml_source1_temporary_result41.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(source_ml_source1_temporary_result45) => {
                                                source_ml_source1_temporary_result45.clone()
                                            }
                                        }
                                    }
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
pub enum MlSource1TemporaryDependencyFunction3Loop1Result {
    MlSource1TemporaryDependencyFunction3Loop1Done,
    MlSource1TemporaryDependencyFunction3Loop1Return(f64),
}

pub fn source_ml_source1_temporary_dependency_function3_loop1(mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut field: self::ml_string::Text, mut start: f64, mut count: f64, mut at: f64) -> self::MlSource1TemporaryDependencyFunction3Loop1Result {
    loop {
        return if (at < ((start + count))) {
            if ((self::dependency_function2(labels.clone(), at)) == field) {
                self::MlSource1TemporaryDependencyFunction3Loop1Result::MlSource1TemporaryDependencyFunction3Loop1Return(self::dependency_function1(targets.clone(), at))
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
            self::MlSource1TemporaryDependencyFunction3Loop1Result::MlSource1TemporaryDependencyFunction3Loop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done,
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(bool),
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut fields: f64, mut at: f64) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), fields))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), fields, at);
                {
                    let name = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![110, 97, 109, 101]));
                    if (name < (0f64)) {
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(true)
                    } else {
                        if self::ml_array::includes(&{ let ml_reference_value35 = brands.borrow().clone(); ml_reference_value35 }, &self::dependency_function2(texts.clone(), name), 0f64, false, |a, b| a == b) {
                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(true)
                        } else {
                            {
                                let pending_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_concat_value43 = pending.clone();
                                    {
                                        let source_ml_source2_concat_value44 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![116, 121, 112, 101]))]));
                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value36 = source_ml_source2_concat_value43.borrow().clone(); ml_reference_value36 }), { let ml_reference_value37 = source_ml_source2_concat_value44.borrow().clone(); ml_reference_value37 })
                                    }
                                }));
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        (pending, at) = (pending_2.clone(), at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(pending.clone())
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop4(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut children: f64, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), children))) {
            {
                let pending_2 = std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_concat_value45 = pending.clone();
                    {
                        let source_ml_source2_concat_value46 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), children, at)]));
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value38 = source_ml_source2_concat_value45.borrow().clone(); ml_reference_value38 }), { let ml_reference_value39 = source_ml_source2_concat_value46.borrow().clone(); ml_reference_value39 })
                    }
                }));
                {
                    let at_2 = (at + 1f64);
                    {
                        (pending, at) = (pending_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            pending.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bool),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(bool),
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop7(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut fields: f64, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index < (self::dependency_function1(counts.clone(), fields))) {
            {
                let pending_2 = std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_concat_value47 = pending.clone();
                    {
                        let source_ml_source2_concat_value48 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), fields, index), self::ml_string::Text::from_units(vec![116, 121, 112, 101]))]));
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value40 = source_ml_source2_concat_value47.borrow().clone(); ml_reference_value40 }), { let ml_reference_value41 = source_ml_source2_concat_value48.borrow().clone(); ml_reference_value41 })
                    }
                }));
                {
                    let index_2 = (index + 1f64);
                    {
                        (pending, index) = (pending_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            pending.clone()
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructors: f64, mut ctor: f64) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result {
    loop {
        return if (ctor < (self::dependency_function1(counts.clone(), constructors))) {
            {
                let fields = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), constructors, ctor), self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
                if (fields < (0f64)) {
                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(true)
                } else {
                    {
                        let index = 0f64;
                        {
                            let pending_2 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop7(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending.clone(), fields, index);
                            {
                                let ctor_2 = (ctor + 1f64);
                                {
                                    (pending, ctor) = (pending_2.clone(), ctor_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(pending.clone())
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut name: f64, mut found: bool, mut at: f64) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), declarations))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), declarations, at);
                {
                    let full_name = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![102, 117, 108, 108, 78, 97, 109, 101]));
                    if ((full_name < (0f64)) || ((self::dependency_function2(texts.clone(), full_name)) != (self::dependency_function2(texts.clone(), name)))) {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let constructors = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![99, 116, 111, 114, 115]));
                            if (constructors < (0f64)) {
                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(true)
                            } else {
                                {
                                    let found_2 = true;
                                    {
                                        let ctor = 0f64;
                                        {
                                            let source_ml_source2_s49 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending.clone(), constructors, ctor);
                                            match source_ml_source2_s49.clone() {
                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(pending_2) => {
                                                    {
                                                        let at_3 = (at + 1f64);
                                                        {
                                                            (pending, found, at) = (pending_2.clone(), found_2, at_3);
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(source_ml_source1_temporary_result6) => {
                                                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(source_ml_source1_temporary_result6)
                                                }
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
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(pending.clone(), found)
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result {
    loop {
        return if { let ml_value = { let ml_reference_array43 = pending.clone(); let ml_reference_projection43 = ml_reference_array43.borrow().len() as f64; ml_reference_projection43 }; ml_value != 0f64 && !ml_value.is_nan() } {
            if ((({ let ml_reference_array44 = seen.clone(); let ml_reference_projection44 = ml_reference_array44.borrow().len() as f64; ml_reference_projection44 }) >= (100000f64)) || (({ let ml_reference_array45 = pending.clone(); let ml_reference_projection45 = ml_reference_array45.borrow().len() as f64; ml_reference_projection45 }) > (100000f64))) {
                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
            } else {
                {
                    let node = self::dependency_function1(pending.clone(), ({ let ml_reference_array46 = pending.clone(); let ml_reference_projection46 = ml_reference_array46.borrow().len() as f64; ml_reference_projection46 } - 1f64));
                    {
                        let pending_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value47 = pending.borrow().clone(); ml_reference_value47 }, 0f64, (-1f64))));
                        if ((node < (0f64)) || (node >= ({ let ml_reference_array48 = starts.clone(); let ml_reference_projection48 = ml_reference_array48.borrow().len() as f64; ml_reference_projection48 }))) {
                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                        } else {
                            if self::ml_array::includes(&{ let ml_reference_value49 = seen.borrow().clone(); ml_reference_value49 }, &node, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) {
                                {
                                    pending = pending_2.clone();
                                    continue;
                                }
                            } else {
                                {
                                    let seen_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_concat_value50 = seen.clone();
                                        {
                                            let source_ml_source2_concat_value51 = std::rc::Rc::new(std::cell::RefCell::new(vec![node]));
                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value50 = source_ml_source2_concat_value50.borrow().clone(); ml_reference_value50 }), { let ml_reference_value51 = source_ml_source2_concat_value51.borrow().clone(); ml_reference_value51 })
                                        }
                                    }));
                                    {
                                        let owner = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107, 105, 110, 100]));
                                        if (owner < (0f64)) {
                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                        } else {
                                            {
                                                let kind = self::dependency_function2(texts.clone(), owner);
                                                if (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
                                                    {
                                                        let fields = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
                                                        if (fields < (0f64)) {
                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                        } else {
                                                            {
                                                                let at = 0f64;
                                                                {
                                                                    let source_ml_source2_s52 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), brands.clone(), pending_2.clone(), fields, at);
                                                                    match source_ml_source2_s52.clone() {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(pending_3) => {
                                                                            {
                                                                                (pending, seen) = (pending_3.clone(), seen_2.clone());
                                                                                continue;
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(source_ml_source1_temporary_result3) => {
                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result3)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if self::ml_array::includes(&{ let ml_reference_value52 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![99, 101, 108, 108])]))).borrow().clone(); ml_reference_value52 }, &kind.clone(), 0f64, false, |a, b| a == b) {
                                                        {
                                                            let pending_4 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                let source_ml_source2_concat_value53 = pending_2.clone();
                                                                {
                                                                    let source_ml_source2_concat_value54 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116]))]));
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value53 = source_ml_source2_concat_value53.borrow().clone(); ml_reference_value53 }), { let ml_reference_value54 = source_ml_source2_concat_value54.borrow().clone(); ml_reference_value54 })
                                                                }
                                                            }));
                                                            {
                                                                (pending, seen) = (pending_4.clone(), seen_2.clone());
                                                                continue;
                                                            }
                                                        }
                                                    } else {
                                                        if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                                            {
                                                                let pending_5 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_concat_value55 = pending_2.clone();
                                                                    {
                                                                        let source_ml_source2_concat_value56 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107, 101, 121])), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))]));
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value55 = source_ml_source2_concat_value55.borrow().clone(); ml_reference_value55 }), { let ml_reference_value56 = source_ml_source2_concat_value56.borrow().clone(); ml_reference_value56 })
                                                                    }
                                                                }));
                                                                {
                                                                    (pending, seen) = (pending_5.clone(), seen_2.clone());
                                                                    continue;
                                                                }
                                                            }
                                                        } else {
                                                            if (((kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 111, 110])))) || (kind == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))) {
                                                                {
                                                                    let children = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, if (kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) {
                                                                        self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 115])
                                                                    } else {
                                                                        if (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 111, 110]))) {
                                                                            self::ml_string::Text::from_units(vec![97, 108, 116, 101, 114, 110, 97, 116, 105, 118, 101, 115])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 115])
                                                                        }
                                                                    });
                                                                    if (children < (0f64)) {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                    } else {
                                                                        {
                                                                            let at_2 = 0f64;
                                                                            {
                                                                                let pending_6 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop4(starts.clone(), counts.clone(), targets.clone(), pending_2.clone(), children, at_2);
                                                                                if (kind == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
                                                                                    {
                                                                                        let pending_7 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                            let source_ml_source2_concat_value57 = pending_6.clone();
                                                                                            {
                                                                                                let source_ml_source2_concat_value58 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![114, 101, 116]))]));
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value57 = source_ml_source2_concat_value57.borrow().clone(); ml_reference_value57 }), { let ml_reference_value58 = source_ml_source2_concat_value58.borrow().clone(); ml_reference_value58 })
                                                                                            }
                                                                                        }));
                                                                                        {
                                                                                            (pending, seen) = (pending_7.clone(), seen_2.clone());
                                                                                            continue;
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        (pending, seen) = (pending_6.clone(), seen_2.clone());
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if (kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97]))) {
                                                                    {
                                                                        let name = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![110, 97, 109, 101]));
                                                                        if ((name < (0f64)) || (declarations < (0f64))) {
                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                        } else {
                                                                            {
                                                                                let found = false;
                                                                                {
                                                                                    let at_3 = 0f64;
                                                                                    {
                                                                                        let source_ml_source2_s59 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, pending_2.clone(), name, found, at_3);
                                                                                        match source_ml_source2_s59.clone() {
                                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(pending_8, found_2) => {
                                                                                                if !found_2 {
                                                                                                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                                                } else {
                                                                                                    {
                                                                                                        (pending, seen) = (pending_8.clone(), seen_2.clone());
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(source_ml_source1_temporary_result5) => {
                                                                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result5)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if !(self::ml_array::includes(&{ let ml_reference_value59 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![117, 110, 105, 116]), self::ml_string::Text::from_units(vec![98, 111, 111, 108]), self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]), self::ml_string::Text::from_units(vec![105, 110, 116]), self::ml_string::Text::from_units(vec![110, 97, 116]), self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])]))).borrow().clone(); ml_reference_value59 }, &kind.clone(), 0f64, false, |a, b| a == b)) {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                    } else {
                                                                        {
                                                                            (pending, seen) = (pending_2.clone(), seen_2.clone());
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
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done,
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(bool),
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done,
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(bool),
}

pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop11(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut params: f64, mut at: f64) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), params))) {
            {
                let parameter = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), params, at);
                {
                    let type_ = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), parameter, self::ml_string::Text::from_units(vec![116, 121, 112, 101]));
                    if self::contains_java_script_closed_class_brand(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, declarations, brands.clone()) {
                        self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(true)
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
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done
        };
    }
}

pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop10(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut exported: self::ml_string::Text, mut found: bool, mut index: f64) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result {
    loop {
        return if (index < (self::dependency_function1(counts.clone(), declarations))) {
            {
                let declaration = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), declarations, index);
                {
                    let name = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, self::ml_string::Text::from_units(vec![110, 97, 109, 101]));
                    if ((name < (0f64)) || ((self::dependency_function2(texts.clone(), name)) != exported)) {
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let kind = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, self::ml_string::Text::from_units(vec![107]));
                            if ((kind < (0f64)) || ((self::dependency_function2(texts.clone(), kind)) != (self::ml_string::Text::from_units(vec![102, 110])))) {
                                {
                                    let index_3 = (index + 1f64);
                                    {
                                        index = index_3;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let params = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 115]));
                                    {
                                        let ret = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, self::ml_string::Text::from_units(vec![114, 101, 116]));
                                        if ((found || (params < (0f64))) || self::contains_java_script_closed_class_brand(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), ret, declarations, brands.clone())) {
                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(true)
                                        } else {
                                            {
                                                let found_2 = true;
                                                {
                                                    let at = 0f64;
                                                    {
                                                        let source_ml_source2_s60 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop11(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, brands.clone(), params, at);
                                                        match source_ml_source2_s60.clone() {
                                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done => {
                                                                {
                                                                    let index_4 = (index + 1f64);
                                                                    {
                                                                        (found, index) = (found_2, index_4);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(source_ml_source1_temporary_result11) => {
                                                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(source_ml_source1_temporary_result11)
                                                            }
                                                        }
                                                    }
                                                }
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
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(found)
        };
    }
}

pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop9(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_values8: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_index8: self::ml::Big) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result {
    loop {
        return if (source_ml_source1_temporary_index8 < ({ let ml_reference_array60 = source_ml_source1_temporary_values8.clone(); let ml_reference_projection60 = self::ml::Big::from_u128(ml_reference_array60.borrow().len() as u128); ml_reference_projection60 })) {
            {
                let exported = {
                    let source_ml_source2_array_read_value61 = source_ml_source1_temporary_values8.clone();
                    {
                        let source_ml_source2_array_read_index62 = source_ml_source1_temporary_index8.clone();
                        { let ml_reference_array61 = source_ml_source2_array_read_value61.clone(); let ml_reference_position61 = source_ml_source2_array_read_index62.clone(); let ml_reference_projection61 = self::ml_array::at(&*ml_reference_array61.borrow(), ml_reference_position61.to_index()); ml_reference_projection61 }
                    }
                };
                {
                    let found = false;
                    {
                        let index = 0f64;
                        {
                            let source_ml_source2_s63 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop10(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, brands.clone(), exported.clone(), found, index);
                            match source_ml_source2_s63.clone() {
                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(found_2) => {
                                    if !found_2 {
                                        self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(true)
                                    } else {
                                        {
                                            let source_ml_source1_temporary_index8_2 = source_ml_source1_temporary_index8.add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_source1_temporary_index8 = source_ml_source1_temporary_index8_2.clone();
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(source_ml_source1_temporary_result10) => {
                                    self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(source_ml_source1_temporary_result10)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result {
    MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Done,
    MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_record_field_write_operator_loop13(mut operation: self::ml_string::Text, mut source_ml_source1_temporary_values12: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_index12: self::ml::Big) -> self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result {
    loop {
        return if (source_ml_source1_temporary_index12 < ({ let ml_reference_array62 = source_ml_source1_temporary_values12.clone(); let ml_reference_projection62 = self::ml::Big::from_u128(ml_reference_array62.borrow().len() as u128); ml_reference_projection62 })) {
            {
                let operator = {
                    let source_ml_source2_array_read_value64 = source_ml_source1_temporary_values12.clone();
                    {
                        let source_ml_source2_array_read_index65 = source_ml_source1_temporary_index12.clone();
                        { let ml_reference_array63 = source_ml_source2_array_read_value64.clone(); let ml_reference_position63 = source_ml_source2_array_read_index65.clone(); let ml_reference_projection63 = self::ml_array::at(&*ml_reference_array63.borrow(), ml_reference_position63.to_index()); ml_reference_projection63 }
                    }
                };
                if operation.starts_with(&((self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100])).concat(&operator)).concat(&self::ml_string::Text::from_units(vec![58]))) {
                    self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Return(operator.clone())
                } else {
                    {
                        let source_ml_source1_temporary_index12_2 = source_ml_source1_temporary_index12.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index12 = source_ml_source1_temporary_index12_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Done
        };
    }
}

pub fn source_ml_source1_temporary_read_record_field_write_suffix_loop14(mut text: self::ml_string::Text, mut suffix: f64) -> f64 {
    loop {
        return if ((text.contains(&(self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 111, 119, 110, 101, 114])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix)))) || text.contains(&(self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix))))) || text.contains(&(self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 112, 114, 101, 118, 105, 111, 117, 115])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix))))) {
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

pub fn source_ml_source1_temporary_read_optional_record_field_order_followers_loop15(mut numeric_keys: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut selected: f64, mut followers: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index < ({ let ml_reference_array64 = numeric_keys.clone(); let ml_reference_projection64 = ml_reference_array64.borrow().len() as f64; ml_reference_projection64 })) {
            {
                let key = ({
                    let source_ml_source2_array_read_value66 = numeric_keys.clone();
                    {
                        let source_ml_source2_array_read_index67 = index;
                        { let ml_reference_array65 = source_ml_source2_array_read_value66.clone(); let ml_reference_position65 = source_ml_source2_array_read_index67; let ml_reference_projection65 = self::ml_array::optional_at(&*ml_reference_array65.borrow(), self::ml_array::number_index(ml_reference_position65)); ml_reference_projection65 }
                    }
                }).clone().unwrap_or_else(|| (-1f64));
                if ((key < (0f64)) || (key > selected)) {
                    {
                        let followers_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value66 = followers.borrow().clone(); ml_reference_value66 }, vec![index])));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (followers, index) = (followers_2.clone(), index_2);
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
            followers.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result {
    MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Done(self::ml_string::Text),
    MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_record_owner_loop16(mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut fields: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owner: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result {
    loop {
        return if (at < ({ let ml_reference_array67 = brands.clone(); let ml_reference_projection67 = ml_reference_array67.borrow().len() as f64; ml_reference_projection67 })) {
            if self::ml_array::includes(&{ let ml_reference_value68 = fields.borrow().clone(); ml_reference_value68 }, &self::dependency_function2(brands.clone(), at), 0f64, false, |a, b| a == b) {
                if (owner != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Return(self::ml_string::Text::from_units(vec![35, 97, 109, 98, 105, 103, 117, 111, 117, 115]))
                } else {
                    {
                        let owner_2 = self::dependency_function2(constructors.clone(), at);
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
            self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Done(owner.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExactLoop18Result {
    MlSource1TemporaryExactLoop18Done,
    MlSource1TemporaryExactLoop18Return(bool),
}

pub fn source_ml_source1_temporary_exact_loop18(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64, mut source_ml_source1_temporary_values17: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_index17: self::ml::Big) -> self::MlSource1TemporaryExactLoop18Result {
    loop {
        return if (source_ml_source1_temporary_index17 < ({ let ml_reference_array69 = source_ml_source1_temporary_values17.clone(); let ml_reference_projection69 = self::ml::Big::from_u128(ml_reference_array69.borrow().len() as u128); ml_reference_projection69 })) {
            {
                let name = {
                    let source_ml_source2_array_read_value68 = source_ml_source1_temporary_values17.clone();
                    {
                        let source_ml_source2_array_read_index69 = source_ml_source1_temporary_index17.clone();
                        { let ml_reference_array70 = source_ml_source2_array_read_value68.clone(); let ml_reference_position70 = source_ml_source2_array_read_index69.clone(); let ml_reference_projection70 = self::ml_array::at(&*ml_reference_array70.borrow(), ml_reference_position70.to_index()); ml_reference_projection70 }
                    }
                };
                if ((self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, name.clone())) < (0f64)) {
                    self::MlSource1TemporaryExactLoop18Result::MlSource1TemporaryExactLoop18Return(false)
                } else {
                    {
                        let source_ml_source1_temporary_index17_2 = source_ml_source1_temporary_index17.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index17 = source_ml_source1_temporary_index17_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExactLoop18Result::MlSource1TemporaryExactLoop18Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStringsLoop19Result {
    MlSource1TemporaryStringsLoop19Done(std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>),
    MlSource1TemporaryStringsLoop19Return(std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>),
}

pub fn source_ml_source1_temporary_strings_loop19(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut at: f64) -> self::MlSource1TemporaryStringsLoop19Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), node))) {
            {
                let value = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), node, at);
                if (((self::dependency_function2(forms.clone(), value)) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || ((self::dependency_function2(texts.clone(), value)) == (self::ml_string::Text::from_units(vec![])))) {
                    self::MlSource1TemporaryStringsLoop19Result::MlSource1TemporaryStringsLoop19Return(std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![])])))
                } else {
                    {
                        let result_2 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_concat_value70 = result.clone();
                            {
                                let source_ml_source2_concat_value71 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function2(texts.clone(), value)]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value71 = source_ml_source2_concat_value70.borrow().clone(); ml_reference_value71 }), { let ml_reference_value72 = source_ml_source2_concat_value71.borrow().clone(); ml_reference_value72 })
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
                }
            }
        } else {
            self::MlSource1TemporaryStringsLoop19Result::MlSource1TemporaryStringsLoop19Done(result.clone())
        };
    }
}

pub fn source_ml_source1_temporary_names_loop20(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut fields: f64, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), fields))) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_concat_value72 = result.clone();
                    {
                        let source_ml_source2_concat_value73 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), fields, at), self::ml_string::Text::from_units(vec![110, 97, 109, 101]))]));
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value73 = source_ml_source2_concat_value72.borrow().clone(); ml_reference_value73 }), { let ml_reference_value74 = source_ml_source2_concat_value73.borrow().clone(); ml_reference_value74 })
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
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCarriesLoop21Result {
    MlSource1TemporaryCarriesLoop21Done,
    MlSource1TemporaryCarriesLoop21Return(bool),
}

pub fn source_ml_source1_temporary_carries_loop21(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut owner: self::ml_string::Text, mut depth: f64, mut elements: f64, mut at: f64) -> self::MlSource1TemporaryCarriesLoop21Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), elements))) {
            if self::carries(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), elements, at), owner.clone(), (depth + 1f64)) {
                self::MlSource1TemporaryCarriesLoop21Result::MlSource1TemporaryCarriesLoop21Return(true)
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
            self::MlSource1TemporaryCarriesLoop21Result::MlSource1TemporaryCarriesLoop21Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFreshLoop22Result {
    MlSource1TemporaryFreshLoop22Done,
    MlSource1TemporaryFreshLoop22Return(f64),
}

pub fn source_ml_source1_temporary_fresh_loop22(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut name: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryFreshLoop22Result {
    loop {
        return if (at >= (0f64)) {
            if ((self::dependency_function2(bindings.clone(), at)) == name) {
                self::MlSource1TemporaryFreshLoop22Result::MlSource1TemporaryFreshLoop22Return(self::dependency_function1(values.clone(), at))
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
            self::MlSource1TemporaryFreshLoop22Result::MlSource1TemporaryFreshLoop22Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFreshLoop23Result {
    MlSource1TemporaryFreshLoop23Done(f64),
    MlSource1TemporaryFreshLoop23Return(f64),
}

pub fn source_ml_source1_temporary_fresh_loop23(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut owner: self::ml_string::Text, mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut depth: f64, mut origin: f64, mut items: f64, mut at: f64) -> self::MlSource1TemporaryFreshLoop23Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), items))) {
            {
                let value = self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), items, at), owner.clone(), bindings.clone(), values.clone(), (depth + 1f64));
                if ((value == ((-1f64))) || (((value >= (0f64)) && (origin >= (0f64))) && (value != origin))) {
                    self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Return((-1f64))
                } else {
                    if (value >= (0f64)) {
                        {
                            let origin_2 = value;
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (origin, at) = (origin_2, at_2);
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
                }
            }
        } else {
            self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Done(origin)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop33(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut operand: f64, mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), operand))) {
            {
                let values_2 = std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_concat_value74 = values.clone();
                    {
                        let source_ml_source2_concat_value75 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), operand, at)]));
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value75 = source_ml_source2_concat_value74.borrow().clone(); ml_reference_value75 }), { let ml_reference_value76 = source_ml_source2_concat_value75.borrow().clone(); ml_reference_value76 })
                    }
                }));
                {
                    let at_2 = (at + 1f64);
                    {
                        (values, at) = (values_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            values.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop37(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut actual: f64, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ((self::dependency_function1(starts.clone(), actual) + self::dependency_function1(counts.clone(), actual)))) {
            if !(self::ml_array::includes(&{ let ml_reference_value77 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 112, 97, 110]), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114])]))).borrow().clone(); ml_reference_value77 }, &self::dependency_function2(labels.clone(), at), 0f64, false, |a, b| a == b)) {
                {
                    let pending_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value76 = pending.clone();
                        {
                            let source_ml_source2_concat_value77 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function1(targets.clone(), at)]));
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value78 = source_ml_source2_concat_value76.borrow().clone(); ml_reference_value78 }), { let ml_reference_value79 = source_ml_source2_concat_value77.borrow().clone(); ml_reference_value79 })
                        }
                    }));
                    {
                        let at_2 = (at + 1f64);
                        {
                            (pending, at) = (pending_2.clone(), at_2);
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
            pending.clone()
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop36(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut identity: bool, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result {
    loop {
        return if { let ml_value = { let ml_reference_array81 = pending.clone(); let ml_reference_projection81 = ml_reference_array81.borrow().len() as f64; ml_reference_projection81 }; ml_value != 0f64 && !ml_value.is_nan() } {
            {
                let actual = self::dependency_function1(pending.clone(), ({ let ml_reference_array82 = pending.clone(); let ml_reference_projection82 = ml_reference_array82.borrow().len() as f64; ml_reference_projection82 } - 1f64));
                {
                    let pending_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value83 = pending.borrow().clone(); ml_reference_value83 }, 0f64, (-1f64))));
                    if ((actual < (0f64)) || self::ml_array::includes(&{ let ml_reference_value84 = seen.borrow().clone(); ml_reference_value84 }, &actual, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                        {
                            pending = pending_2.clone();
                            continue;
                        }
                    } else {
                        if (({ let ml_reference_array85 = seen.clone(); let ml_reference_projection85 = ml_reference_array85.borrow().len() as f64; ml_reference_projection85 }) > (100000f64)) {
                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let seen_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_concat_value78 = seen.clone();
                                    {
                                        let source_ml_source2_concat_value79 = std::rc::Rc::new(std::cell::RefCell::new(vec![actual]));
                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value86 = source_ml_source2_concat_value78.borrow().clone(); ml_reference_value86 }), { let ml_reference_value87 = source_ml_source2_concat_value79.borrow().clone(); ml_reference_value87 })
                                    }
                                }));
                                {
                                    let owner = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), actual, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]));
                                    {
                                        let position = self::ml_array::index_of(&{ let ml_reference_value88 = constructors.borrow().clone(); ml_reference_value88 }, &owner.clone(), 0f64, false, |a, b| a == b);
                                        if ((position >= (0f64)) && !(self::accept_java_script_class_owner_observation(self::dependency_function2(representations.clone(), position), if identity {
                                            self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
                                        } else {
                                            self::ml_string::Text::from_units(vec![115, 116, 114, 117, 99, 116, 117, 114, 97, 108])
                                        }))) {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 99, 108, 97, 115, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 100, 111, 101, 115, 32, 110, 111, 116, 32, 97, 100, 109, 105, 116, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32, 100, 101, 98, 117, 103, 32, 111, 114, 32, 112, 114, 111, 111, 102, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110]))
                                        } else {
                                            {
                                                let at = self::dependency_function1(starts.clone(), actual);
                                                {
                                                    let pending_3 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop37(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending_2.clone(), actual, at);
                                                    {
                                                        (pending, seen) = (pending_3.clone(), seen_2.clone());
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop35(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut identity: bool, mut type_: f64, mut owner: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result {
    loop {
        return if (owner < ({ let ml_reference_array89 = constructors.clone(); let ml_reference_projection89 = ml_reference_array89.borrow().len() as f64; ml_reference_projection89 })) {
            if (!(self::accept_java_script_class_owner_observation(self::dependency_function2(representations.clone(), owner), if identity {
                self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
            } else {
                self::ml_string::Text::from_units(vec![115, 116, 114, 117, 99, 116, 117, 114, 97, 108])
            })) && self::contains_java_script_closed_class_brand(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, declarations, std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function2(brands.clone(), owner)])))) {
                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Return(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 99, 108, 97, 115, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 100, 111, 101, 115, 32, 110, 111, 116, 32, 97, 100, 109, 105, 116, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32, 100, 101, 98, 117, 103, 32, 111, 114, 32, 112, 114, 111, 111, 102, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110]))
            } else {
                {
                    let owner_2 = (owner + 1f64);
                    {
                        owner = owner_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop34(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut identity: bool, mut source_ml_source1_temporary_values26: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index26: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result {
    loop {
        return if (source_ml_source1_temporary_index26 < ({ let ml_reference_array90 = source_ml_source1_temporary_values26.clone(); let ml_reference_projection90 = self::ml::Big::from_u128(ml_reference_array90.borrow().len() as u128); ml_reference_projection90 })) {
            {
                let value = {
                    let source_ml_source2_array_read_value80 = source_ml_source1_temporary_values26.clone();
                    {
                        let source_ml_source2_array_read_index81 = source_ml_source1_temporary_index26.clone();
                        { let ml_reference_array91 = source_ml_source2_array_read_value80.clone(); let ml_reference_position91 = source_ml_source2_array_read_index81.clone(); let ml_reference_projection91 = self::ml_array::at(&*ml_reference_array91.borrow(), ml_reference_position91.to_index()); ml_reference_projection91 }
                    }
                };
                {
                    let type_ = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), value, self::ml_string::Text::from_units(vec![116, 121, 112, 101]));
                    if (type_ < (0f64)) {
                        {
                            let source_ml_source1_temporary_index26_2 = source_ml_source1_temporary_index26.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index26 = source_ml_source1_temporary_index26_2.clone();
                                continue;
                            }
                        }
                    } else {
                        {
                            let owner = 0f64;
                            {
                                let source_ml_source2_s82 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop35(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructors.clone(), brands.clone(), representations.clone(), identity, type_, owner);
                                match source_ml_source2_s82.clone() {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Done => {
                                        {
                                            let pending = std::rc::Rc::new(std::cell::RefCell::new(vec![type_]));
                                            {
                                                let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                {
                                                    let source_ml_source2_s83 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop36(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), constructors.clone(), representations.clone(), identity, pending.clone(), seen.clone());
                                                    match source_ml_source2_s83.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Done => {
                                                            {
                                                                let source_ml_source1_temporary_index26_3 = source_ml_source1_temporary_index26.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index26 = source_ml_source1_temporary_index26_3.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(source_ml_source1_temporary_result36) => {
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Return(source_ml_source1_temporary_result36.clone())
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Return(source_ml_source1_temporary_result37) => {
                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Return(source_ml_source1_temporary_result37.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop32(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut node: f64, mut identity: bool, mut source_ml_source1_temporary_values25: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_index25: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result {
    loop {
        return if (source_ml_source1_temporary_index25 < ({ let ml_reference_array92 = source_ml_source1_temporary_values25.clone(); let ml_reference_projection92 = self::ml::Big::from_u128(ml_reference_array92.borrow().len() as u128); ml_reference_projection92 })) {
            {
                let key = {
                    let source_ml_source2_array_read_value84 = source_ml_source1_temporary_values25.clone();
                    {
                        let source_ml_source2_array_read_index85 = source_ml_source1_temporary_index25.clone();
                        { let ml_reference_array93 = source_ml_source2_array_read_value84.clone(); let ml_reference_position93 = source_ml_source2_array_read_index85.clone(); let ml_reference_projection93 = self::ml_array::at(&*ml_reference_array93.borrow(), ml_reference_position93.to_index()); ml_reference_projection93 }
                    }
                };
                if !key.0.is_empty() {
                    {
                        let operand = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, key.clone());
                        if (operand < (0f64)) {
                            {
                                let source_ml_source1_temporary_index25_2 = source_ml_source1_temporary_index25.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index25 = source_ml_source1_temporary_index25_2.clone();
                                    continue;
                                }
                            }
                        } else {
                            {
                                let values = if ((self::dependency_function2(forms.clone(), operand)) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![operand]))
                                };
                                {
                                    let values_3 = if ((self::dependency_function2(forms.clone(), operand)) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
                                        {
                                            let at = 0f64;
                                            {
                                                let values_2 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop33(starts.clone(), counts.clone(), targets.clone(), operand, values.clone(), at);
                                                values_2.clone()
                                            }
                                        }
                                    } else {
                                        values.clone()
                                    };
                                    {
                                        let source_ml_source1_temporary_values26 = values_3.clone();
                                        {
                                            let source_ml_source1_temporary_index26 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s86 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop34(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructors.clone(), brands.clone(), representations.clone(), identity, source_ml_source1_temporary_values26.clone(), source_ml_source1_temporary_index26.clone());
                                                match source_ml_source2_s86.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Done => {
                                                        {
                                                            let source_ml_source1_temporary_index25_3 = source_ml_source1_temporary_index25.add(&self::ml::Big::from_i128(1));
                                                            {
                                                                source_ml_source1_temporary_index25 = source_ml_source1_temporary_index25_3.clone();
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Return(source_ml_source1_temporary_result34) => {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Return(source_ml_source1_temporary_result34.clone())
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
                        let source_ml_source1_temporary_index25_4 = source_ml_source1_temporary_index25.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index25 = source_ml_source1_temporary_index25_4.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop31(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut node: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result {
    loop {
        return if (node < ({ let ml_reference_array94 = starts.clone(); let ml_reference_projection94 = ml_reference_array94.borrow().len() as f64; ml_reference_projection94 })) {
            {
                let kind = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107]));
                {
                    let proof = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![112]));
                    {
                        let operation = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![111, 112]));
                        {
                            let reference = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]));
                            {
                                let identity = (((kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108]))) || ((kind == (self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]))) && self::ml_array::includes(&{ let ml_reference_value95 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 113]), self::ml_string::Text::from_units(vec![110, 101])]))).borrow().clone(); ml_reference_value95 }, &operation.clone(), 0f64, false, |a, b| a == b))) || ((self::ml_array::includes(&{ let ml_reference_value96 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 113]), self::ml_string::Text::from_units(vec![110, 101])]))).borrow().clone(); ml_reference_value96 }, &proof.clone(), 0f64, false, |a, b| a == b) && (reference >= (0f64))) && ((self::dependency_function2(texts.clone(), reference)) == (self::ml_string::Text::from_units(vec![116, 114, 117, 101])))));
                                {
                                    let source_ml_source1_temporary_values25 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_string_unit_operand87 = self::read_java_script_closed_class_observation_operation_operands(kind.clone(), proof.clone(), operation.clone());
                                        {
                                            let source_ml_source2_string_unit_operand88 = self::ml_string::Text::from_units(vec![32]);
                                            {
                                                let source_ml_source2_string_split_limit89 = 4294967295f64;
                                                {
                                                    let source_ml_source2_string_split_separator90 = source_ml_source2_string_unit_operand88.clone();
                                                    (|ml_text:&self::ml_string::Text,ml_separator:Option<&self::ml_string::Text>,ml_limit:f64|{self::ml_utf16_text::split_utf16_text(&(ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()), &(ml_separator.map(|ml_value|ml_value.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()).unwrap_or_default()), ml_limit, ml_separator.is_none()).into_iter().map(|ml_units|self::ml_string::Text::from_units(ml_units.into_iter().map(|ml_unit|ml_unit as u16).collect())).collect::<Vec<_>>()})(&(source_ml_source2_string_unit_operand87), Some(&source_ml_source2_string_split_separator90), source_ml_source2_string_split_limit89)
                                                }
                                            }
                                        }
                                    }));
                                    {
                                        let source_ml_source1_temporary_index25 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s91 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop32(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructors.clone(), brands.clone(), representations.clone(), node, identity, source_ml_source1_temporary_values25.clone(), source_ml_source1_temporary_index25.clone());
                                            match source_ml_source2_s91.clone() {
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Done => {
                                                    {
                                                        let node_2 = (node + 1f64);
                                                        {
                                                            node = node_2;
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Return(source_ml_source1_temporary_result32) => {
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(source_ml_source1_temporary_result32.clone())
                                                }
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop40(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64, mut kind: self::ml_string::Text, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ((self::dependency_function1(starts.clone(), node) + self::dependency_function1(counts.clone(), node)))) {
            if !(self::ml_array::includes(&{ let ml_reference_value97 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![116, 121, 112, 101]), self::ml_string::Text::from_units(vec![100, 111, 109, 97, 105, 110]), self::ml_string::Text::from_units(vec![115, 112, 97, 110]), self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101, 80, 97, 116, 104])]))).borrow().clone(); ml_reference_value97 }, &self::dependency_function2(labels.clone(), at), 0f64, false, |a, b| a == b)) {
                {
                    let pending_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value92 = pending.clone();
                        {
                            let source_ml_source2_concat_value93 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function1(targets.clone(), at), if ((kind == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))) && ((self::dependency_function2(labels.clone(), at)) == (self::ml_string::Text::from_units(vec![102, 110])))) {
                                1f64
                            } else {
                                0f64
                            }]));
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value98 = source_ml_source2_concat_value92.borrow().clone(); ml_reference_value98 }), { let ml_reference_value99 = source_ml_source2_concat_value93.borrow().clone(); ml_reference_value99 })
                        }
                    }));
                    {
                        let at_2 = (at + 1f64);
                        {
                            (pending, at) = (pending_2.clone(), at_2);
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
            pending.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39 {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Next,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop41(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owner: self::ml_string::Text, mut direct: f64, mut node: f64, mut kind: self::ml_string::Text, mut object: f64, mut instance: f64, mut name: self::ml_string::Text, mut methods: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), methods))) {
            {
                let method = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), methods, at);
                if ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), method, self::ml_string::Text::from_units(vec![110, 97, 109, 101]))) != name) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                } else {
                    if ((kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]))) && (((self::dependency_function2(representations.clone(), instance)) != (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115]))) || (direct != (1f64)))) {
                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 114, 101, 99, 101, 105, 118, 101, 114, 45, 114, 101, 116, 97, 105, 110, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 100, 101, 116, 97, 99, 104, 101, 100, 32, 115, 108, 111, 116, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 115]))
                    } else {
                        if (kind == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) {
                            {
                                let args = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![97, 114, 103, 115]));
                                {
                                    let value = if (args < (0f64)) {
                                        (-1f64)
                                    } else {
                                        self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), args, 0f64)
                                    };
                                    {
                                        let helper = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), method, self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114]));
                                        {
                                            let call_args = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), value, self::ml_string::Text::from_units(vec![97, 114, 103, 115]));
                                            {
                                                let receiver = if (call_args < (0f64)) {
                                                    (-1f64)
                                                } else {
                                                    self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), call_args, 0f64)
                                                };
                                                if (((((((owner != (self::dependency_function2(constructors.clone(), instance))) || ((self::dependency_function2(representations.clone(), instance)) != (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), value, self::ml_string::Text::from_units(vec![107]))) != (self::ml_string::Text::from_units(vec![99, 97, 108, 108])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), value, self::ml_string::Text::from_units(vec![102, 110]))) != helper)) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), object, self::ml_string::Text::from_units(vec![107]))) != (self::ml_string::Text::from_units(vec![118, 97, 114])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), receiver, self::ml_string::Text::from_units(vec![107]))) != (self::ml_string::Text::from_units(vec![118, 97, 114])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), object, self::ml_string::Text::from_units(vec![110, 97, 109, 101]))) != (self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), receiver, self::ml_string::Text::from_units(vec![110, 97, 109, 101]))))) {
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 32, 115, 108, 111, 116, 115, 32, 97, 114, 101, 32, 105, 110, 115, 116, 97, 108, 108, 101, 100, 32, 111, 110, 108, 121, 32, 98, 121, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 102, 97, 99, 116, 111, 114, 121, 32, 105, 110, 115, 105, 100, 101, 32, 116, 104, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop38(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owner_nodes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut owner: self::ml_string::Text, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result {
    loop {
        return if { let ml_value = { let ml_reference_array101 = pending.clone(); let ml_reference_projection101 = ml_reference_array101.borrow().len() as f64; ml_reference_projection101 }; ml_value != 0f64 && !ml_value.is_nan() } {
            {
                let direct = self::dependency_function1(pending.clone(), ({ let ml_reference_array102 = pending.clone(); let ml_reference_projection102 = ml_reference_array102.borrow().len() as f64; ml_reference_projection102 } - 1f64));
                {
                    let node = self::dependency_function1(pending.clone(), ({ let ml_reference_array103 = pending.clone(); let ml_reference_projection103 = ml_reference_array103.borrow().len() as f64; ml_reference_projection103 } - 2f64));
                    {
                        let pending_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value104 = pending.borrow().clone(); ml_reference_value104 }, 0f64, (-2f64))));
                        if ((node < (0f64)) || self::ml_array::includes(&{ let ml_reference_value105 = seen.borrow().clone(); ml_reference_value105 }, &((node * 2f64) + direct), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                            {
                                pending = pending_2.clone();
                                continue;
                            }
                        } else {
                            if (({ let ml_reference_array106 = seen.clone(); let ml_reference_projection106 = ml_reference_array106.borrow().len() as f64; ml_reference_projection106 }) > (100000f64)) {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 98, 111, 100, 121, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let seen_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_concat_value94 = seen.clone();
                                        {
                                            let source_ml_source2_concat_value95 = std::rc::Rc::new(std::cell::RefCell::new(vec![((node * 2f64) + direct)]));
                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value107 = source_ml_source2_concat_value94.borrow().clone(); ml_reference_value107 }), { let ml_reference_value108 = source_ml_source2_concat_value95.borrow().clone(); ml_reference_value108 })
                                        }
                                    }));
                                    {
                                        let kind = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107]));
                                        {
                                            let type_ = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![116, 121, 112, 101]));
                                            {
                                                let class_owner = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]));
                                                if (((kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) && (class_owner != (self::ml_string::Text::from_units(vec![])))) && (class_owner != owner)) {
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 100, 97, 116, 97, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 115, 32, 98, 101, 108, 111, 110, 103, 32, 111, 110, 108, 121, 32, 116, 111, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                                } else {
                                                    {
                                                        let object = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]));
                                                        {
                                                            let object_type = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), object, self::ml_string::Text::from_units(vec![116, 121, 112, 101]));
                                                            {
                                                                let instance = self::ml_array::index_of(&{ let ml_reference_value109 = constructors.borrow().clone(); ml_reference_value109 }, &self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), object_type, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114])), 0f64, false, |a, b| a == b);
                                                                {
                                                                    let source_ml_source2_s97 = if (instance >= (0f64)) {
                                                                        {
                                                                            let definition = self::dependency_function1(owner_nodes.clone(), instance);
                                                                            {
                                                                                let brand = self::dependency_function2(brands.clone(), instance);
                                                                                {
                                                                                    let name = if (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]))) {
                                                                                        self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100]))
                                                                                    } else {
                                                                                        if (kind == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) {
                                                                                            self::read_record_field_write_name(self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![111, 112])))
                                                                                        } else {
                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                        }
                                                                                    };
                                                                                    if (name == brand) {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Return(self::ml_string::Text::from_units(vec![97, 32, 115, 121, 110, 116, 104, 101, 116, 105, 99, 32, 99, 108, 97, 115, 115, 32, 98, 114, 97, 110, 100, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 111, 98, 115, 101, 114, 118, 97, 98, 108, 101, 32, 112, 117, 98, 108, 105, 99, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100]))
                                                                                    } else {
                                                                                        {
                                                                                            let methods = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), definition, self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 115]));
                                                                                            {
                                                                                                let at = 0f64;
                                                                                                {
                                                                                                    let source_ml_source2_s98 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop41(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), constructors.clone(), representations.clone(), owner.clone(), direct, node, kind.clone(), object, instance, name.clone(), methods, at);
                                                                                                    match source_ml_source2_s98.clone() {
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Done => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Next
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(source_ml_source1_temporary_result41) => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Return(source_ml_source1_temporary_result41.clone())
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
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Next
                                                                    };
                                                                    match source_ml_source2_s97.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Next => {
                                                                            {
                                                                                let at_2 = self::dependency_function1(starts.clone(), node);
                                                                                {
                                                                                    let pending_3 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop40(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending_2.clone(), node, kind.clone(), at_2);
                                                                                    {
                                                                                        (pending, seen) = (pending_3.clone(), seen_2.clone());
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Return(source_ml_source1_temporary_result40) => {
                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(source_ml_source1_temporary_result40.clone())
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop30(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owner_nodes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut scopes: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut scope: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result {
    loop {
        return if (scope < ({ let ml_reference_array110 = roots.clone(); let ml_reference_projection110 = ml_reference_array110.borrow().len() as f64; ml_reference_projection110 })) {
            {
                let owner = self::dependency_function2(scopes.clone(), scope);
                {
                    let pending = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function1(roots.clone(), scope), 0f64]));
                    {
                        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let source_ml_source2_s99 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop38(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), constructors.clone(), brands.clone(), representations.clone(), owner_nodes.clone(), owner.clone(), pending.clone(), seen.clone());
                            match source_ml_source2_s99.clone() {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Done => {
                                    {
                                        let scope_2 = (scope + 1f64);
                                        {
                                            scope = scope_2;
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(source_ml_source1_temporary_result38) => {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(source_ml_source1_temporary_result38.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop29(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut scopes: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut declaration: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result {
    loop {
        return if ((declarations >= (0f64)) && (declaration < (self::dependency_function1(counts.clone(), declarations)))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), declarations, declaration);
                {
                    let roots_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value100 = roots.clone();
                        {
                            let source_ml_source2_concat_value101 = std::rc::Rc::new(std::cell::RefCell::new(vec![row]));
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value111 = source_ml_source2_concat_value100.borrow().clone(); ml_reference_value111 }), { let ml_reference_value112 = source_ml_source2_concat_value101.borrow().clone(); ml_reference_value112 })
                        }
                    }));
                    {
                        let scopes_2 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_concat_value102 = scopes.clone();
                            {
                                let source_ml_source2_concat_value103 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![102, 117, 108, 108, 78, 97, 109, 101]))]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value113 = source_ml_source2_concat_value102.borrow().clone(); ml_reference_value113 }), { let ml_reference_value114 = source_ml_source2_concat_value103.borrow().clone(); ml_reference_value114 })
                            }
                        }));
                        {
                            let declaration_2 = (declaration + 1f64);
                            {
                                (roots, scopes, declaration) = (roots_2.clone(), scopes_2.clone(), declaration_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done(roots.clone(), scopes.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop42(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut methods: f64, mut complete: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), methods))) {
            {
                let complete_2 = std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_concat_value104 = complete.clone();
                    {
                        let source_ml_source2_concat_value105 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), methods, at), self::ml_string::Text::from_units(vec![110, 97, 109, 101]))]));
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value115 = source_ml_source2_concat_value104.borrow().clone(); ml_reference_value115 }), { let ml_reference_value116 = source_ml_source2_concat_value105.borrow().clone(); ml_reference_value116 })
                    }
                }));
                {
                    let at_2 = (at + 1f64);
                    {
                        (complete, at) = (complete_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            complete.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop45(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut type_keys: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut expected_fields: f64, mut actual: f64, mut name: self::ml_string::Text, mut other: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result {
    loop {
        return if (other < (self::dependency_function1(counts.clone(), expected_fields))) {
            {
                let expected = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), expected_fields, other);
                if (((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), expected, self::ml_string::Text::from_units(vec![110, 97, 109, 101]))) == name) && ((self::dependency_function2(type_keys.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), actual, self::ml_string::Text::from_units(vec![116, 121, 112, 101])))) != (self::dependency_function2(type_keys.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), expected, self::ml_string::Text::from_units(vec![116, 121, 112, 101])))))) {
                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 102, 105, 101, 108, 100, 32, 99, 104, 97, 110, 103, 101, 100, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 116, 121, 112, 101]))
                } else {
                    {
                        let other_2 = (other + 1f64);
                        {
                            other = other_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop44(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut type_keys: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut actual_fields: f64, mut expected_fields: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), actual_fields))) {
            {
                let actual = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), actual_fields, at);
                {
                    let name = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), actual, self::ml_string::Text::from_units(vec![110, 97, 109, 101]));
                    {
                        let other = 0f64;
                        {
                            let source_ml_source2_s106 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop45(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_keys.clone(), expected_fields, actual, name.clone(), other);
                            match source_ml_source2_s106.clone() {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Done => {
                                    {
                                        let at_2 = (at + 1f64);
                                        {
                                            at = at_2;
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Return(source_ml_source1_temporary_result45) => {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Return(source_ml_source1_temporary_result45.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop43(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut position: f64, mut branded_fields: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), branded_fields))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), branded_fields, at);
                if (((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![110, 97, 109, 101]))) == (self::dependency_function2(brands.clone(), position))) && ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![116, 121, 112, 101])), self::ml_string::Text::from_units(vec![107, 105, 110, 100]))) != (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) {
                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 98, 114, 97, 110, 100, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 85, 110, 105, 116, 32, 102, 105, 101, 108, 100]))
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop28(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut type_keys: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owner_nodes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructor_nodes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result {
    loop {
        return if (node < ({ let ml_reference_array117 = starts.clone(); let ml_reference_projection117 = ml_reference_array117.borrow().len() as f64; ml_reference_projection117 })) {
            if ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107, 105, 110, 100]))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
                {
                    let owner = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]));
                    {
                        let fields = self::names(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node);
                        {
                            let position = self::ml_array::index_of(&{ let ml_reference_value118 = constructors.borrow().clone(); ml_reference_value118 }, &owner.clone(), 0f64, false, |a, b| a == b);
                            {
                                let present = { let ml_values119 = { let ml_reference_value120 = brands.borrow().clone(); ml_reference_value120 }; let ml_callback119 = { let fields = fields.clone(); std::rc::Rc::new(move |brand: self::ml_string::Text| -> bool { self::ml_array::includes(&{ let ml_reference_value121 = fields.borrow().clone(); ml_reference_value121 }, &brand.clone(), 0f64, false, |a, b| a == b) }) }; ml_values119.iter().cloned().enumerate().any(|(ml_index119, ml_value119)| ml_callback119(ml_value119)) };
                                if (((self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]))) >= (0f64)) && (owner == (self::ml_string::Text::from_units(vec![])))) {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 116, 121, 112, 101, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 32, 110, 111, 110, 101, 109, 112, 116, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))
                                } else {
                                    if ((owner == (self::ml_string::Text::from_units(vec![]))) && !present) {
                                        {
                                            let node_2 = (node + 1f64);
                                            {
                                                node = node_2;
                                                continue;
                                            }
                                        }
                                    } else {
                                        if (position < (0f64)) {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 116, 121, 112, 101, 32, 99, 97, 110, 110, 111, 116, 32, 100, 114, 111, 112, 32, 105, 116, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 119, 105, 116, 110, 101, 115, 115]))
                                        } else {
                                            {
                                                let definition = self::dependency_function1(owner_nodes.clone(), position);
                                                {
                                                    let expected = std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source2_concat_value107 = self::strings(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), definition, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                                        {
                                                            let source_ml_source2_concat_value108 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function2(brands.clone(), position)]));
                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value122 = source_ml_source2_concat_value107.borrow().clone(); ml_reference_value122 }), { let ml_reference_value123 = source_ml_source2_concat_value108.borrow().clone(); ml_reference_value123 })
                                                        }
                                                    }));
                                                    {
                                                        let methods = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), definition, self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 115]));
                                                        {
                                                            let complete = expected.clone();
                                                            {
                                                                let complete_3 = if ((self::dependency_function2(representations.clone(), position)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115]))) {
                                                                    {
                                                                        let at = 0f64;
                                                                        {
                                                                            let complete_2 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop42(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), methods, complete.clone(), at);
                                                                            complete_2.clone()
                                                                        }
                                                                    }
                                                                } else {
                                                                    complete.clone()
                                                                };
                                                                if ((({ let ml_reference_array124 = fields.clone(); let ml_reference_projection124 = ml_reference_array124.borrow().len() as f64; ml_reference_projection124 }) != ({ let ml_reference_array125 = complete_3.clone(); let ml_reference_projection125 = ml_reference_array125.borrow().len() as f64; ml_reference_projection125 })) || { let ml_values126 = { let ml_reference_value127 = complete_3.borrow().clone(); ml_reference_value127 }; let ml_callback126 = { let fields = fields.clone(); std::rc::Rc::new(move |name: self::ml_string::Text| -> bool { !(self::ml_array::includes(&{ let ml_reference_value128 = fields.borrow().clone(); ml_reference_value128 }, &name.clone(), 0f64, false, |a, b| a == b)) }) }; ml_values126.iter().cloned().enumerate().any(|(ml_index126, ml_value126)| ml_callback126(ml_value126)) }) {
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 114, 101, 99, 111, 114, 100, 32, 99, 111, 110, 116, 114, 97, 100, 105, 99, 116, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100, 115]))
                                                                } else {
                                                                    {
                                                                        let branded_fields = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
                                                                        {
                                                                            let at_2 = 0f64;
                                                                            {
                                                                                let source_ml_source2_s109 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop43(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), brands.clone(), position, branded_fields, at_2);
                                                                                match source_ml_source2_s109.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Done => {
                                                                                        {
                                                                                            let reference = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function1(constructor_nodes.clone(), position), self::ml_string::Text::from_units(vec![114, 101, 116]));
                                                                                            if ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), reference, self::ml_string::Text::from_units(vec![107, 105, 110, 100]))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
                                                                                                {
                                                                                                    let actual_fields = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
                                                                                                    {
                                                                                                        let expected_fields = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), reference, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
                                                                                                        {
                                                                                                            let at_3 = 0f64;
                                                                                                            {
                                                                                                                let source_ml_source2_s110 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop44(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_keys.clone(), actual_fields, expected_fields, at_3);
                                                                                                                match source_ml_source2_s110.clone() {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Done => {
                                                                                                                        {
                                                                                                                            let node_3 = (node + 1f64);
                                                                                                                            {
                                                                                                                                node = node_3;
                                                                                                                                continue;
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Return(source_ml_source1_temporary_result44) => {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(source_ml_source1_temporary_result44.clone())
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
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Return(source_ml_source1_temporary_result45) => {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(source_ml_source1_temporary_result45.clone())
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
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
                    let node_5 = (node + 1f64);
                    {
                        node = node_5;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27 {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Done(std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop48(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructor: self::ml_string::Text, mut actual: f64, mut declaration: f64) -> f64 {
    loop {
        return if (declaration < (self::dependency_function1(counts.clone(), declarations))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), declarations, declaration);
                if (((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![102, 117, 108, 108, 78, 97, 109, 101]))) == constructor) && ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![107]))) == (self::ml_string::Text::from_units(vec![102, 110])))) {
                    {
                        let actual_2 = row;
                        {
                            let declaration_2 = (declaration + 1f64);
                            {
                                (actual, declaration) = (actual_2, declaration_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let declaration_3 = (declaration + 1f64);
                        {
                            declaration = declaration_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            actual
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop50(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructor: self::ml_string::Text, mut helper: self::ml_string::Text, mut found: bool, mut declaration: f64) -> bool {
    loop {
        return if (declaration < (self::dependency_function1(counts.clone(), declarations))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), declarations, declaration);
                if (((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![102, 117, 108, 108, 78, 97, 109, 101]))) != helper) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![107]))) != (self::ml_string::Text::from_units(vec![102, 110])))) {
                    {
                        let declaration_2 = (declaration + 1f64);
                        {
                            declaration = declaration_2;
                            continue;
                        }
                    }
                } else {
                    {
                        let params = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 115]));
                        {
                            let parameter = if (params < (0f64)) {
                                (-1f64)
                            } else {
                                self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), params, 0f64)
                            };
                            {
                                let type_ = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), parameter, self::ml_string::Text::from_units(vec![116, 121, 112, 101]));
                                if ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]))) == constructor) {
                                    {
                                        let found_2 = true;
                                        {
                                            let declaration_3 = (declaration + 1f64);
                                            {
                                                (found, declaration) = (found_2, declaration_3);
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let declaration_4 = (declaration + 1f64);
                                        {
                                            declaration = declaration_4;
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
            found
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop49(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructor: self::ml_string::Text, mut source_ml_source1_temporary_values24: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_index24: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Result {
    loop {
        return if (source_ml_source1_temporary_index24 < ({ let ml_reference_array129 = source_ml_source1_temporary_values24.clone(); let ml_reference_projection129 = self::ml::Big::from_u128(ml_reference_array129.borrow().len() as u128); ml_reference_projection129 })) {
            {
                let helper = {
                    let source_ml_source2_array_read_value111 = source_ml_source1_temporary_values24.clone();
                    {
                        let source_ml_source2_array_read_index112 = source_ml_source1_temporary_index24.clone();
                        { let ml_reference_array130 = source_ml_source2_array_read_value111.clone(); let ml_reference_position130 = source_ml_source2_array_read_index112.clone(); let ml_reference_projection130 = self::ml_array::at(&*ml_reference_array130.borrow(), ml_reference_position130.to_index()); ml_reference_projection130 }
                    }
                };
                {
                    let found = false;
                    {
                        let declaration = 0f64;
                        {
                            let found_2 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop50(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructor.clone(), helper.clone(), found, declaration);
                            if !found_2 {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Return(self::ml_string::Text::from_units(vec![97, 32, 99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 32, 104, 101, 108, 112, 101, 114, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101]))
                            } else {
                                {
                                    let source_ml_source1_temporary_index24_2 = source_ml_source1_temporary_index24.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index24 = source_ml_source1_temporary_index24_2.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Done(std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>),
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop47(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut fields: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut methods: f64, mut method_names: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut helpers: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut method: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Result {
    loop {
        return if (method < (self::dependency_function1(counts.clone(), methods))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), methods, method);
                {
                    let name = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![110, 97, 109, 101]));
                    {
                        let helper = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114]));
                        if (((((!(self::exact(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![110, 97, 109, 101]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114])])))) || (name == (self::ml_string::Text::from_units(vec![])))) || (helper == (self::ml_string::Text::from_units(vec![])))) || self::ml_array::includes(&{ let ml_reference_value131 = fields.borrow().clone(); ml_reference_value131 }, &name.clone(), 0f64, false, |a, b| a == b)) || self::ml_array::includes(&{ let ml_reference_value132 = method_names.borrow().clone(); ml_reference_value132 }, &name.clone(), 0f64, false, |a, b| a == b)) || self::ml_array::includes(&{ let ml_reference_value133 = helpers.borrow().clone(); ml_reference_value133 }, &helper.clone(), 0f64, false, |a, b| a == b)) {
                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 32, 111, 119, 110, 101, 114, 115, 32, 97, 114, 101, 32, 99, 111, 110, 116, 114, 97, 100, 105, 99, 116, 111, 114, 121]))
                        } else {
                            {
                                let method_names_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_concat_value113 = method_names.clone();
                                    {
                                        let source_ml_source2_concat_value114 = std::rc::Rc::new(std::cell::RefCell::new(vec![name.clone()]));
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value134 = source_ml_source2_concat_value113.borrow().clone(); ml_reference_value134 }), { let ml_reference_value135 = source_ml_source2_concat_value114.borrow().clone(); ml_reference_value135 })
                                    }
                                }));
                                {
                                    let helpers_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_concat_value115 = helpers.clone();
                                        {
                                            let source_ml_source2_concat_value116 = std::rc::Rc::new(std::cell::RefCell::new(vec![helper.clone()]));
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value136 = source_ml_source2_concat_value115.borrow().clone(); ml_reference_value136 }), { let ml_reference_value137 = source_ml_source2_concat_value116.borrow().clone(); ml_reference_value137 })
                                        }
                                    }));
                                    {
                                        let method_2 = (method + 1f64);
                                        {
                                            (method_names, helpers, method) = (method_names_2.clone(), helpers_2.clone(), method_2);
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Done(helpers.clone())
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop46(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owner_nodes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructor_nodes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut owners: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), owners))) {
            {
                let owner = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), owners, at);
                {
                    let constructor = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner, self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]));
                    {
                        let brand = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner, self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100]));
                        {
                            let representation = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner, self::ml_string::Text::from_units(vec![114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110]));
                            if (((((!(self::exact(starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner, std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100]), self::ml_string::Text::from_units(vec![114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]), self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 115])])))) || (constructor == (self::ml_string::Text::from_units(vec![])))) || (brand == (self::ml_string::Text::from_units(vec![])))) || self::ml_array::includes(&{ let ml_reference_value138 = constructors.borrow().clone(); ml_reference_value138 }, &constructor.clone(), 0f64, false, |a, b| a == b)) || self::ml_array::includes(&{ let ml_reference_value139 = brands.borrow().clone(); ml_reference_value139 }, &brand.clone(), 0f64, false, |a, b| a == b)) || !(self::ml_array::includes(&{ let ml_reference_value140 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 104, 101, 108, 112, 101, 114, 115]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115])]))).borrow().clone(); ml_reference_value140 }, &representation.clone(), 0f64, false, |a, b| a == b))) {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 98, 114, 97, 110, 100, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                            } else {
                                {
                                    let fields = self::strings(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                    if ((((({ let ml_reference_array141 = fields.clone(); let ml_reference_projection141 = ml_reference_array141.borrow().len() as f64; ml_reference_projection141 }) > (8192f64)) || self::ml_array::includes(&{ let ml_reference_value142 = fields.borrow().clone(); ml_reference_value142 }, &self::ml_string::Text::from_units(vec![]), 0f64, false, |a, b| a == b)) || (((({ let ml_reference_value143 = (std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::set({ let ml_reference_value144 = fields.borrow().clone(); ml_reference_value144 }, |a, b| a == b)))).borrow().clone(); ml_reference_value143 }).len() as f64)) != ({ let ml_reference_array145 = fields.clone(); let ml_reference_projection145 = ml_reference_array145.borrow().len() as f64; ml_reference_projection145 }))) || self::ml_array::includes(&{ let ml_reference_value146 = fields.borrow().clone(); ml_reference_value146 }, &brand.clone(), 0f64, false, |a, b| a == b)) {
                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 110, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116]))
                                    } else {
                                        {
                                            let methods = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner, self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 115]));
                                            if (((methods < (0f64)) || ((self::dependency_function2(forms.clone(), methods)) != (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) || ((self::dependency_function1(counts.clone(), methods)) > (8192f64))) {
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 104, 101, 108, 112, 101, 114, 115]))
                                            } else {
                                                {
                                                    let method_names = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                    {
                                                        let helpers = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                        {
                                                            let method = 0f64;
                                                            {
                                                                let source_ml_source2_s117 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop47(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), fields.clone(), methods, method_names.clone(), helpers.clone(), method);
                                                                match source_ml_source2_s117.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Done(helpers_2) => {
                                                                        {
                                                                            let actual = (-1f64);
                                                                            {
                                                                                let declaration = 0f64;
                                                                                {
                                                                                    let actual_2 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop48(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructor.clone(), actual, declaration);
                                                                                    if (actual_2 < (0f64)) {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text::from_units(vec![97, 32, 99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]))
                                                                                    } else {
                                                                                        {
                                                                                            let ret = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), actual_2, self::ml_string::Text::from_units(vec![114, 101, 116]));
                                                                                            {
                                                                                                let body = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), actual_2, self::ml_string::Text::from_units(vec![98, 111, 100, 121]));
                                                                                                {
                                                                                                    let reference = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), ret, self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]));
                                                                                                    if (((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), ret, self::ml_string::Text::from_units(vec![107, 105, 110, 100]))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) && (((reference < (0f64)) || ((self::dependency_function2(forms.clone(), reference)) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) || ((self::dependency_function2(texts.clone(), reference)) != (self::ml_string::Text::from_units(vec![116, 114, 117, 101]))))) {
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text::from_units(vec![97, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 115, 104, 97, 114, 101, 100, 32, 105, 110, 115, 116, 97, 110, 99, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))
                                                                                                    } else {
                                                                                                        if (!(self::carries(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), ret, constructor.clone(), 0f64)) || ((self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), body, constructor.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())), 0f64)) < (0f64))) {
                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 109, 117, 115, 116, 32, 114, 101, 116, 117, 114, 110, 32, 105, 116, 115, 32, 111, 119, 110, 32, 102, 114, 101, 115, 104, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                                                                                                        } else {
                                                                                                            {
                                                                                                                let source_ml_source1_temporary_values24 = helpers_2.clone();
                                                                                                                {
                                                                                                                    let source_ml_source1_temporary_index24 = self::ml::Big::from_i128(0);
                                                                                                                    {
                                                                                                                        let source_ml_source2_s118 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop49(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructor.clone(), source_ml_source1_temporary_values24.clone(), source_ml_source1_temporary_index24.clone());
                                                                                                                        match source_ml_source2_s118.clone() {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Done => {
                                                                                                                                {
                                                                                                                                    let constructors_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                        let source_ml_source2_concat_value119 = constructors.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_concat_value120 = std::rc::Rc::new(std::cell::RefCell::new(vec![constructor.clone()]));
                                                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value147 = source_ml_source2_concat_value119.borrow().clone(); ml_reference_value147 }), { let ml_reference_value148 = source_ml_source2_concat_value120.borrow().clone(); ml_reference_value148 })
                                                                                                                                        }
                                                                                                                                    }));
                                                                                                                                    {
                                                                                                                                        let brands_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                            let source_ml_source2_concat_value121 = brands.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_concat_value122 = std::rc::Rc::new(std::cell::RefCell::new(vec![brand.clone()]));
                                                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value149 = source_ml_source2_concat_value121.borrow().clone(); ml_reference_value149 }), { let ml_reference_value150 = source_ml_source2_concat_value122.borrow().clone(); ml_reference_value150 })
                                                                                                                                            }
                                                                                                                                        }));
                                                                                                                                        {
                                                                                                                                            let representations_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                                let source_ml_source2_concat_value123 = representations.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_concat_value124 = std::rc::Rc::new(std::cell::RefCell::new(vec![representation.clone()]));
                                                                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value151 = source_ml_source2_concat_value123.borrow().clone(); ml_reference_value151 }), { let ml_reference_value152 = source_ml_source2_concat_value124.borrow().clone(); ml_reference_value152 })
                                                                                                                                                }
                                                                                                                                            }));
                                                                                                                                            {
                                                                                                                                                let owner_nodes_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                                    let source_ml_source2_concat_value125 = owner_nodes.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_concat_value126 = std::rc::Rc::new(std::cell::RefCell::new(vec![owner]));
                                                                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value153 = source_ml_source2_concat_value125.borrow().clone(); ml_reference_value153 }), { let ml_reference_value154 = source_ml_source2_concat_value126.borrow().clone(); ml_reference_value154 })
                                                                                                                                                    }
                                                                                                                                                }));
                                                                                                                                                {
                                                                                                                                                    let constructor_nodes_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                                        let source_ml_source2_concat_value127 = constructor_nodes.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_concat_value128 = std::rc::Rc::new(std::cell::RefCell::new(vec![actual_2]));
                                                                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value155 = source_ml_source2_concat_value127.borrow().clone(); ml_reference_value155 }), { let ml_reference_value156 = source_ml_source2_concat_value128.borrow().clone(); ml_reference_value156 })
                                                                                                                                                        }
                                                                                                                                                    }));
                                                                                                                                                    {
                                                                                                                                                        let at_2 = (at + 1f64);
                                                                                                                                                        {
                                                                                                                                                            (constructors, brands, representations, owner_nodes, constructor_nodes, at) = (constructors_2.clone(), brands_2.clone(), representations_2.clone(), owner_nodes_2.clone(), constructor_nodes_2.clone(), at_2);
                                                                                                                                                            continue;
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Return(source_ml_source1_temporary_result49) => {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(source_ml_source1_temporary_result49.clone())
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Return(source_ml_source1_temporary_result50) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(source_ml_source1_temporary_result50.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Done(constructors.clone(), brands.clone(), representations.clone(), owner_nodes.clone(), constructor_nodes.clone())
        };
    }
}

fn ml_main() {
    assert!(((self::accept_java_script_class_owner_observation(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121]))) == (true)), "assertion 1");
    assert!(((self::accept_java_script_class_owner_observation(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115]), self::ml_string::Text::from_units(vec![115, 116, 114, 117, 99, 116, 117, 114, 97, 108]))) == (false)), "assertion 2");
    assert!(((self::accept_java_script_class_owner_observation(self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 104, 101, 108, 112, 101, 114, 115]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121]))) == (false)), "assertion 3");
    assert!(((self::accept_java_script_class_owner_observation(self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 104, 101, 108, 112, 101, 114, 115]), self::ml_string::Text::from_units(vec![115, 116, 114, 117, 99, 116, 117, 114, 97, 108]))) == (false)), "assertion 4");
    assert!(((self::accept_java_script_class_owner_observation(self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121]))) == (false)), "assertion 5");
    assert!(((self::read_java_script_class_owner_inference(self::ml_string::Text::from_units(vec![66, 111, 120]), self::ml_string::Text::from_units(vec![]), false, true)) == (self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]))), "assertion 6");
    assert!(((self::read_java_script_class_owner_inference(self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![66, 111, 120]), true, false)) == (self::ml_string::Text::from_units(vec![108, 101, 102, 116]))), "assertion 7");
    assert!(((self::read_java_script_class_owner_inference(self::ml_string::Text::from_units(vec![66, 111, 120]), self::ml_string::Text::from_units(vec![79, 116, 104, 101, 114]), false, false)) == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]))), "assertion 8");
    assert!(((self::read_java_script_class_record_owner(std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![66, 111, 120]), self::ml_string::Text::from_units(vec![79, 116, 104, 101, 114])])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 49])])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])])))) == (self::ml_string::Text::from_units(vec![66, 111, 120]))), "assertion 9");
    assert!(((self::read_java_script_class_record_owner(std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![66, 111, 120]), self::ml_string::Text::from_units(vec![79, 116, 104, 101, 114])])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 49])])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 49])])))) == (self::ml_string::Text::from_units(vec![35, 97, 109, 98, 105, 103, 117, 111, 117, 115]))), "assertion 10");
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
fn all_literal_class_owner_controls() { main(); }
