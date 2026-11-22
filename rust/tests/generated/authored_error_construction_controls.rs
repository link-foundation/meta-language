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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaScriptErrorAllocationIdentity {
    ErrorAllocationIdentity(self::ml::Big, self::ml_string::Text, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaScriptErrorAllocationDirectory {
    ErrorAllocationDirectory(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorAllocationIdentity>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>),
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f2) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f3) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Text(units) => {
            if (({ let ml_reference_array1 = units.clone(); let ml_reference_projection1 = ml_reference_array1.borrow().len() as f64; ml_reference_projection1 }) > (1048576f64)) {
                self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s5 = self::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(units.clone(), at);
                        match source_ml_source2_s5.clone() {
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1.clone()
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
        self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f4) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f10) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f11) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f12) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f13) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f14, source_ml_source2_f15) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f16) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f17, source_ml_source2_f18) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f19) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f20) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f21) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f22) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f23, source_ml_source2_f24) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f25) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f26, source_ml_source2_f27) => {
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
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f28) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f29) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f30) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f31, source_ml_source2_f32) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f33) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f34, source_ml_source2_f35) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f36) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f37) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f38) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f39, source_ml_source2_f40) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f41) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f42, source_ml_source2_f43) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f44) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f45) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f46) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f47, source_ml_source2_f48) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f49) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f50, source_ml_source2_f51) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f6) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f7, source_ml_source2_f8) => {
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
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f58, source_ml_source2_f59) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, source_ml_source2_f9) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f60) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f61) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f62) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f63) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f64, source_ml_source2_f65) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f66) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f67) => {
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f68) => {
            {
                let source_ml_source2_f92 = source_ml_source2_f68;
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f69) => {
            {
                let source_ml_source2_f101 = source_ml_source2_f69;
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f70) => {
            {
                let source_ml_source2_f110 = source_ml_source2_f70.clone();
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f71) => {
            {
                let units = source_ml_source2_f71.clone();
                { let ml_reference_array2 = units.clone(); let ml_reference_projection2 = ml_reference_array2.borrow().len() as f64; ml_reference_projection2 }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f72, text) => {
            text.length()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f73) => {
            {
                let source_ml_source2_f129 = source_ml_source2_f73.clone();
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f74, source_ml_source2_f75) => {
            {
                let source_ml_source2_f138 = source_ml_source2_f74.clone();
                {
                    let source_ml_source2_f139 = source_ml_source2_f75;
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f140) => {
                {
                    let source_ml_source2_f164 = source_ml_source2_f140;
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f141) => {
                {
                    let source_ml_source2_f173 = source_ml_source2_f141;
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f142) => {
                {
                    let source_ml_source2_f182 = source_ml_source2_f142.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f143) => {
                {
                    let units = source_ml_source2_f143.clone();
                    ({
                        let source_ml_source2_array_read_value196 = units.clone();
                        {
                            let source_ml_source2_array_read_index197 = index;
                            { let ml_reference_array3 = source_ml_source2_array_read_value196.clone(); let ml_reference_position3 = source_ml_source2_array_read_index197; let ml_reference_projection3 = self::ml_array::optional_at(&*ml_reference_array3.borrow(), self::ml_array::number_index(ml_reference_position3)); ml_reference_projection3 }
                        }
                    }).clone().unwrap_or_else(|| (-1f64))
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f144, text) => {
                {
                    let source_ml_source2_string_unit_operand198 = text.clone();
                    {
                        let source_ml_source2_string_unit_operand199 = index;
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
                                    let source_ml_source2_string_unit_operand205 = {
                                        let source_ml_source2_string_range_operand200 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand201 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand202 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position203 = source_ml_source2_string_range_operand201;
                                                    {
                                                        let source_ml_source2_string_range_position204 = source_ml_source2_string_range_operand202;
                                                        source_ml_source2_string_range_operand200.slice(source_ml_source2_string_range_position203, source_ml_source2_string_range_position204)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand206 = 0f64;
                                        source_ml_source2_string_unit_operand205.code_point_at(source_ml_source2_string_unit_operand206)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand198.clone(), source_ml_source2_string_unit_operand199)
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f145) => {
                {
                    let source_ml_source2_f212 = source_ml_source2_f145.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f146, source_ml_source2_f147) => {
                {
                    let source_ml_source2_f221 = source_ml_source2_f146.clone();
                    {
                        let source_ml_source2_f222 = source_ml_source2_f147;
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f223) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f224) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f225) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f226) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f227, source_ml_source2_f228) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f229) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f230, source_ml_source2_f231) => {
                    false
                }
            } && match right.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f232) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f233) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f234) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f235) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f236, source_ml_source2_f237) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f238) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f239, source_ml_source2_f240) => {
                    false
                }
            }) {
                {
                    let source_ml_source2_comparison_operand243 = match left.clone() {
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
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value241) => {
                            Some(source_ml_source2_data_property_value241.clone())
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    {
                        let source_ml_source2_comparison_operand244 = match right.clone() {
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
                            self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value242) => {
                                Some(source_ml_source2_data_property_value242.clone())
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<self::ml_string::Text>
                            }
                        };
                        match (source_ml_source2_comparison_operand243.clone(), source_ml_source2_comparison_operand244.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false }
                    }
                }
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s245 = self::source_ml_source1_temporary_test_java_script_semantic_text_equal_loop2(left.clone(), right.clone(), length, at);
                        match source_ml_source2_s245.clone() {
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Done => {
                                true
                            }
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Return(source_ml_source1_temporary_result2) => {
                                source_ml_source1_temporary_result2
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f246) => {
            {
                let source_ml_source2_f270 = source_ml_source2_f246;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f247) => {
            {
                let source_ml_source2_f279 = source_ml_source2_f247;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f248) => {
            {
                let source_ml_source2_f288 = source_ml_source2_f248.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f249) => {
            {
                let units = source_ml_source2_f249.clone();
                {
                    let text = self::ml_string::Text::from_units(vec![]);
                    {
                        let source_ml_source1_temporary_values3 = units.clone();
                        {
                            let source_ml_source1_temporary_index3 = self::ml::Big::from_i128(0);
                            {
                                let text_2 = self::source_ml_source1_temporary_read_java_script_semantic_text_string_loop4(text.clone(), source_ml_source1_temporary_values3.clone(), source_ml_source1_temporary_index3.clone());
                                text_2.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f250, text_3) => {
            text_3.clone()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f251) => {
            {
                let source_ml_source2_f307 = source_ml_source2_f251.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f252, source_ml_source2_f253) => {
            {
                let source_ml_source2_f316 = source_ml_source2_f252.clone();
                {
                    let source_ml_source2_f317 = source_ml_source2_f253;
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
                                let units_2 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units.clone(), at);
                                units_2.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Null => {
                    {
                        let units_3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_2 = 0f64;
                            {
                                let units_4 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_3.clone(), at_2);
                                units_4.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f318) => {
                    {
                        let units_5 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_3 = 0f64;
                            {
                                let units_6 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_5.clone(), at_3);
                                units_6.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f319) => {
                    {
                        let units_7 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_4 = 0f64;
                            {
                                let units_8 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_7.clone(), at_4);
                                units_8.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f320) => {
                    {
                        let units_9 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_5 = 0f64;
                            {
                                let units_10 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_9.clone(), at_5);
                                units_10.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(units_11) => {
                    units_11.clone()
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f321, source_ml_source2_f322) => {
                    {
                        let units_12 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_6 = 0f64;
                            {
                                let units_13 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_12.clone(), at_6);
                                units_13.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f323) => {
                    {
                        let units_14 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_7 = 0f64;
                            {
                                let units_15 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_14.clone(), at_7);
                                units_15.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f324, source_ml_source2_f325) => {
                    {
                        let units_16 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_8 = 0f64;
                            {
                                let units_17 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_16.clone(), at_8);
                                units_17.clone()
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f326) => {
                {
                    let source_ml_source2_f343 = source_ml_source2_f326;
                    false
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f327) => {
                {
                    let source_ml_source2_f352 = source_ml_source2_f327;
                    false
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f328) => {
                {
                    let source_ml_source2_f361 = source_ml_source2_f328.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f329) => {
                {
                    let source_ml_source2_f370 = source_ml_source2_f329.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f330, source_ml_source2_f331) => {
                {
                    let source_ml_source2_f379 = source_ml_source2_f330.clone();
                    {
                        let source_ml_source2_f380 = source_ml_source2_f331.clone();
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f332) => {
                {
                    let source_ml_source2_f389 = source_ml_source2_f332.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f333, source_ml_source2_f334) => {
                {
                    let source_ml_source2_f398 = source_ml_source2_f333.clone();
                    {
                        let callable = source_ml_source2_f334;
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
            let source_ml_source2_s399 = self::source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop6(properties.clone(), key.clone(), at);
            match source_ml_source2_s399.clone() {
                self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(source_ml_source1_temporary_result6) => {
                    source_ml_source1_temporary_result6
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
        let source_ml_source2_field400 = true;
        {
            let source_ml_source2_field401 = configurable;
            {
                let source_ml_source2_field402 = true;
                {
                    let source_ml_source2_field403 = enumerable;
                    {
                        let source_ml_source2_field404 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                        {
                            let source_ml_source2_field405 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_field406 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                {
                                    let source_ml_source2_field407 = false;
                                    {
                                        let source_ml_source2_field408 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                        {
                                            let source_ml_source2_field409 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field410 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                                {
                                                    let source_ml_source2_field411 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field401, source_ml_source2_field400, source_ml_source2_field403, source_ml_source2_field402, source_ml_source2_field409.clone(), source_ml_source2_field408, source_ml_source2_field411.clone(), source_ml_source2_field410, source_ml_source2_field405.clone(), source_ml_source2_field404, source_ml_source2_field407, source_ml_source2_field406)
                                                }
                                            }
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
        let source_ml_source2_field412 = true;
        {
            let source_ml_source2_field413 = if ({ let ml_reference_value52 = proposed.borrow().clone(); ml_reference_value52 }).1 {
                ({ let ml_reference_value53 = proposed.borrow().clone(); ml_reference_value53 }).0
            } else {
                ({ let ml_reference_value54 = base.borrow().clone(); ml_reference_value54 }).0
            };
            {
                let source_ml_source2_field414 = true;
                {
                    let source_ml_source2_field415 = if ({ let ml_reference_value55 = proposed.borrow().clone(); ml_reference_value55 }).3 {
                        ({ let ml_reference_value56 = proposed.borrow().clone(); ml_reference_value56 }).2
                    } else {
                        ({ let ml_reference_value57 = base.borrow().clone(); ml_reference_value57 }).2
                    };
                    {
                        let source_ml_source2_field416 = ({ let ml_reference_value58 = base.borrow().clone(); ml_reference_value58 }).9;
                        {
                            let source_ml_source2_field417 = if (({ let ml_reference_value59 = base.borrow().clone(); ml_reference_value59 }).9 && ({ let ml_reference_value60 = proposed.borrow().clone(); ml_reference_value60 }).9) {
                                ({ let ml_reference_value61 = proposed.borrow().clone(); ml_reference_value61 }).8
                            } else {
                                ({ let ml_reference_value62 = base.borrow().clone(); ml_reference_value62 }).8
                            };
                            {
                                let source_ml_source2_field418 = ({ let ml_reference_value63 = base.borrow().clone(); ml_reference_value63 }).11;
                                {
                                    let source_ml_source2_field419 = if (({ let ml_reference_value64 = base.borrow().clone(); ml_reference_value64 }).11 && ({ let ml_reference_value65 = proposed.borrow().clone(); ml_reference_value65 }).11) {
                                        ({ let ml_reference_value66 = proposed.borrow().clone(); ml_reference_value66 }).10
                                    } else {
                                        ({ let ml_reference_value67 = base.borrow().clone(); ml_reference_value67 }).10
                                    };
                                    {
                                        let source_ml_source2_field420 = ({ let ml_reference_value68 = base.borrow().clone(); ml_reference_value68 }).5;
                                        {
                                            let source_ml_source2_field421 = if (({ let ml_reference_value69 = base.borrow().clone(); ml_reference_value69 }).5 && ({ let ml_reference_value70 = proposed.borrow().clone(); ml_reference_value70 }).5) {
                                                ({ let ml_reference_value71 = proposed.borrow().clone(); ml_reference_value71 }).4
                                            } else {
                                                ({ let ml_reference_value72 = base.borrow().clone(); ml_reference_value72 }).4
                                            };
                                            {
                                                let source_ml_source2_field422 = ({ let ml_reference_value73 = base.borrow().clone(); ml_reference_value73 }).7;
                                                {
                                                    let source_ml_source2_field423 = if (({ let ml_reference_value74 = base.borrow().clone(); ml_reference_value74 }).7 && ({ let ml_reference_value75 = proposed.borrow().clone(); ml_reference_value75 }).7) {
                                                        ({ let ml_reference_value76 = proposed.borrow().clone(); ml_reference_value76 }).6
                                                    } else {
                                                        ({ let ml_reference_value77 = base.borrow().clone(); ml_reference_value77 }).6
                                                    };
                                                    (source_ml_source2_field413, source_ml_source2_field412, source_ml_source2_field415, source_ml_source2_field414, source_ml_source2_field421.clone(), source_ml_source2_field420, source_ml_source2_field423.clone(), source_ml_source2_field422, source_ml_source2_field417.clone(), source_ml_source2_field416, source_ml_source2_field419, source_ml_source2_field418)
                                                }
                                            }
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
                        let source_ml_source2_concat_value424 = writes.clone();
                        {
                            let source_ml_source2_concat_value425 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])]));
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value79 = source_ml_source2_concat_value424.borrow().clone(); ml_reference_value79 }), { let ml_reference_value80 = source_ml_source2_concat_value425.borrow().clone(); ml_reference_value80 })
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
                            let source_ml_source2_concat_value426 = writes_3.clone();
                            {
                                let source_ml_source2_concat_value427 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value82 = source_ml_source2_concat_value426.borrow().clone(); ml_reference_value82 }), { let ml_reference_value83 = source_ml_source2_concat_value427.borrow().clone(); ml_reference_value83 })
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
                                let source_ml_source2_concat_value428 = writes_5.clone();
                                {
                                    let source_ml_source2_concat_value429 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])]));
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value85 = source_ml_source2_concat_value428.borrow().clone(); ml_reference_value85 }), { let ml_reference_value86 = source_ml_source2_concat_value429.borrow().clone(); ml_reference_value86 })
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
                                    let source_ml_source2_concat_value430 = writes_7.clone();
                                    {
                                        let source_ml_source2_concat_value431 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]));
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value88 = source_ml_source2_concat_value430.borrow().clone(); ml_reference_value88 }), { let ml_reference_value89 = source_ml_source2_concat_value431.borrow().clone(); ml_reference_value89 })
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
                                        let source_ml_source2_concat_value432 = writes_9.clone();
                                        {
                                            let source_ml_source2_concat_value433 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![103, 101, 116])]));
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value91 = source_ml_source2_concat_value432.borrow().clone(); ml_reference_value91 }), { let ml_reference_value92 = source_ml_source2_concat_value433.borrow().clone(); ml_reference_value92 })
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
                                            let source_ml_source2_concat_value434 = writes_11.clone();
                                            {
                                                let source_ml_source2_concat_value435 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 101, 116])]));
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value94 = source_ml_source2_concat_value434.borrow().clone(); ml_reference_value94 }), { let ml_reference_value95 = source_ml_source2_concat_value435.borrow().clone(); ml_reference_value95 })
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
        let source_ml_source2_field436 = disposition.clone();
        {
            let source_ml_source2_field437 = error.clone();
            {
                let source_ml_source2_field438 = descriptor.clone();
                {
                    let source_ml_source2_field439 = writes.clone();
                    (source_ml_source2_field438.clone(), source_ml_source2_field436.clone(), source_ml_source2_field437.clone(), source_ml_source2_field439.clone())
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
                                    let source_ml_source2_s440 = if !(({ let ml_reference_value102 = current.borrow().clone(); ml_reference_value102 }).0) {
                                        if (({ let ml_reference_value103 = proposed.borrow().clone(); ml_reference_value103 }).1 && ({ let ml_reference_value104 = proposed.borrow().clone(); ml_reference_value104 }).0) {
                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                        } else {
                                            if (({ let ml_reference_value105 = proposed.borrow().clone(); ml_reference_value105 }).3 && ((({ let ml_reference_value106 = proposed.borrow().clone(); ml_reference_value106 }).2) != (({ let ml_reference_value107 = current.borrow().clone(); ml_reference_value107 }).2))) {
                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if (({ let ml_reference_value108 = proposed.borrow().clone(); ml_reference_value108 }).5 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value109 = proposed.borrow().clone(); ml_reference_value109 }).4, ({ let ml_reference_value110 = current.borrow().clone(); ml_reference_value110 }).4))) {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                        } else {
                                                            if (({ let ml_reference_value111 = proposed.borrow().clone(); ml_reference_value111 }).7 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value112 = proposed.borrow().clone(); ml_reference_value112 }).6, ({ let ml_reference_value113 = current.borrow().clone(); ml_reference_value113 }).6))) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                            }
                                                        }
                                                    } else {
                                                        if !(({ let ml_reference_value114 = current.borrow().clone(); ml_reference_value114 }).10) {
                                                            if (({ let ml_reference_value115 = proposed.borrow().clone(); ml_reference_value115 }).11 && ({ let ml_reference_value116 = proposed.borrow().clone(); ml_reference_value116 }).10) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                if ({ let ml_reference_value117 = proposed.borrow().clone(); ml_reference_value117 }).9 {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(if self::test_java_script_semantic_same_value(({ let ml_reference_value118 = proposed.borrow().clone(); ml_reference_value118 }).8, ({ let ml_reference_value119 = current.borrow().clone(); ml_reference_value119 }).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }, self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                                } else {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                    };
                                    match source_ml_source2_s440.clone() {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Next => {
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
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(source_ml_source1_temporary_result7) => {
                                            source_ml_source1_temporary_result7.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn find_java_script_heap_directory_object(objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = { let ml_reference_array122 = objects.clone(); let ml_reference_projection122 = ml_reference_array122.borrow().len() as f64; ml_reference_projection122 };
            {
                let source_ml_source2_s441 = self::source_ml_source1_temporary_find_java_script_heap_directory_object_loop8(objects.clone(), identity.clone(), low, high);
                match source_ml_source2_s441.clone() {
                    self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(source_ml_source1_temporary_result8) => {
                        source_ml_source1_temporary_result8
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
            let high = { let ml_reference_array123 = symbols.clone(); let ml_reference_projection123 = ml_reference_array123.borrow().len() as f64; ml_reference_projection123 };
            {
                let source_ml_source2_s442 = self::source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop9(symbols.clone(), identity.clone(), low, high);
                match source_ml_source2_s442.clone() {
                    self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Done => {
                        false
                    }
                    self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(source_ml_source1_temporary_result9) => {
                        source_ml_source1_temporary_result9
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f443) => {
                    {
                        let source_ml_source2_f465 = source_ml_source2_f443;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f444) => {
                    {
                        let source_ml_source2_f473 = source_ml_source2_f444;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f445) => {
                    {
                        let source_ml_source2_f481 = source_ml_source2_f445.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f446) => {
                    {
                        let source_ml_source2_f489 = source_ml_source2_f446.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f447, source_ml_source2_f448) => {
                    {
                        let source_ml_source2_f497 = source_ml_source2_f447.clone();
                        {
                            let source_ml_source2_f498 = source_ml_source2_f448.clone();
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    if self::test_java_script_heap_directory_symbol(match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value500, _, _, _, _) => {
                            source_ml_source2_data_property_value500.clone()
                        }
                    }, identity.clone()) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f449, source_ml_source2_f450) => {
                    {
                        let identity_2 = source_ml_source2_f449.clone();
                        {
                            let callable = source_ml_source2_f450;
                            {
                                let at = self::find_java_script_heap_directory_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value508, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value508.clone()
                                    }
                                }, identity_2.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = {
                                            let source_ml_source2_array_read_value510 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value509, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value509.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_array_read_index511 = at;
                                                { let ml_reference_array124 = source_ml_source2_array_read_value510.clone(); let ml_reference_position124 = source_ml_source2_array_read_index511; let ml_reference_projection124 = self::ml_array::optional_at(&*ml_reference_array124.borrow(), self::ml_array::number_index(ml_reference_position124)); ml_reference_projection124 }
                                            }
                                        };
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value525, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value525
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value526, _, _, _, _) => {
                                                source_ml_source2_data_property_value526
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value527, _) => {
                                                source_ml_source2_data_property_value527
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value528, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value528
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value529, _, _) => {
                                                source_ml_source2_data_property_value529
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value530, _, _) => {
                                                source_ml_source2_data_property_value530
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value531, _, _) => {
                                                source_ml_source2_data_property_value531
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value532, _, _) => {
                                                source_ml_source2_data_property_value532
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value533, _) => {
                                                source_ml_source2_data_property_value533
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value534, _, _, _) => {
                                                source_ml_source2_data_property_value534
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value535, _, _, _) => {
                                                source_ml_source2_data_property_value535
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value536, _, _, _) => {
                                                source_ml_source2_data_property_value536
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value537, _, _, _) => {
                                                source_ml_source2_data_property_value537
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f538) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f539) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f540) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f541) => {
            false
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f542, source_ml_source2_f543) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f544) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f545, source_ml_source2_f546) => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f547) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f548) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f549) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f550) => {
            true
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f551, source_ml_source2_f552) => {
            false
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f553) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f554, source_ml_source2_f555) => {
            true
        }
    }) && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f556) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f557) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f558) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f559) => {
            true
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f560, source_ml_source2_f561) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f562) => {
            false
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f563, source_ml_source2_f564) => {
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

pub fn read_java_script_source_exception_operation(name: self::ml_string::Text) -> self::ml_string::Text {
    if (name == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111, 114])
    } else {
        if (name == (self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]))) {
            self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if (name == (self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) {
                self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
            } else {
                if (name == (self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114]))) {
                    self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
                } else {
                    if (name == (self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]))) {
                        self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])
                    } else {
                        if (name == (self::ml_string::Text::from_units(vec![85, 82, 73, 69, 114, 114, 111, 114]))) {
                            self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 85, 82, 73, 69, 114, 114, 111, 114])
                        } else {
                            if (name == (self::ml_string::Text::from_units(vec![69, 118, 97, 108, 69, 114, 114, 111, 114]))) {
                                self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 118, 97, 108, 69, 114, 114, 111, 114])
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

pub fn read_java_script_source_exception_class(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114]))) {
            self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) {
                self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114]))) {
                    self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]))) {
                        self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 85, 82, 73, 69, 114, 114, 111, 114]))) {
                            self::ml_string::Text::from_units(vec![85, 82, 73, 69, 114, 114, 111, 114])
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 118, 97, 108, 69, 114, 114, 111, 114]))) {
                                self::ml_string::Text::from_units(vec![69, 118, 97, 108, 69, 114, 114, 111, 114])
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

pub fn read_java_script_exception_constructor_operation(name: self::ml_string::Text) -> self::ml_string::Text {
    if ((self::read_java_script_source_exception_operation(name.clone())) == (self::ml_string::Text::from_units(vec![]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116])).concat(&name)
    }
}

pub fn read_java_script_exception_constructor_class(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114]))) {
            self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) {
                self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114]))) {
                    self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]))) {
                        self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 85, 82, 73, 69, 114, 114, 111, 114]))) {
                            self::ml_string::Text::from_units(vec![85, 82, 73, 69, 114, 114, 111, 114])
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 118, 97, 108, 69, 114, 114, 111, 114]))) {
                                self::ml_string::Text::from_units(vec![69, 118, 97, 108, 69, 114, 114, 111, 114])
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

pub fn accept_linked_java_script_source_exception(kind: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    ((kind == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))) && ((self::read_java_script_source_exception_class(operation.clone())) != (self::ml_string::Text::from_units(vec![]))))
}

pub fn read_java_script_source_exception_absent_message(absence: self::ml_string::Text) -> self::ml_string::Text {
    if (absence == (self::ml_string::Text::from_units(vec![110, 117, 108, 108]))) {
        self::ml_string::Text::from_units(vec![110, 117, 108, 108])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_java_script_source_exception_message_form(kind: self::ml_string::Text) -> self::ml_string::Text {
    if (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else {
        if (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
            self::ml_string::Text::from_units(vec![101, 109, 112, 116, 121])
        } else {
            if (kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) {
                self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
            } else {
                if ((((kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) || (kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))) || (kind == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
                    self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                } else {
                    self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100])
                }
            }
        }
    }
}

pub fn render_java_script_source_exception_expression(target: self::ml_string::Text, operation: self::ml_string::Text, message: self::ml_string::Text, present: self::ml_string::Text, utf16: bool) -> self::ml_string::Text {
    {
        let name = self::read_java_script_source_exception_class(operation.clone());
        if (name == (self::ml_string::Text::from_units(vec![]))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
                ((((((self::ml_string::Text::from_units(vec![40, 40, 109, 108, 95, 109, 101, 115, 115, 97, 103, 101, 44, 109, 108, 95, 112, 114, 101, 115, 101, 110, 116, 41, 61, 62, 123, 116, 104, 114, 111, 119, 32, 110, 101, 119, 32])).concat(&name)).concat(&self::ml_string::Text::from_units(vec![40, 109, 108, 95, 112, 114, 101, 115, 101, 110, 116, 63, 109, 108, 95, 109, 101, 115, 115, 97, 103, 101, 58, 117, 110, 100, 101, 102, 105, 110, 101, 100, 41, 59, 125, 41, 40]))).concat(&message)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&present)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                    ((((((self::ml_string::Text::from_units(vec![40, 99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 111, 117, 114, 99, 101, 95, 101, 120, 99, 101, 112, 116, 105, 111, 110, 58, 58, 83, 111, 117, 114, 99, 101, 69, 120, 99, 101, 112, 116, 105, 111, 110, 32, 123, 32, 99, 108, 97, 115, 115, 95, 110, 97, 109, 101, 58, 32, 83, 116, 114, 105, 110, 103, 58, 58, 102, 114, 111, 109, 40, 34])).concat(&name)).concat(&self::ml_string::Text::from_units(vec![34, 41, 44, 32, 109, 101, 115, 115, 97, 103, 101, 95, 117, 110, 105, 116, 115, 58, 32]))).concat(&if utf16 {
                        ((self::ml_string::Text::from_units(vec![40])).concat(&message)).concat(&self::ml_string::Text::from_units(vec![41, 46, 48]))
                    } else {
                        ((self::ml_string::Text::from_units(vec![40])).concat(&message)).concat(&self::ml_string::Text::from_units(vec![41, 46, 101, 110, 99, 111, 100, 101, 95, 117, 116, 102, 49, 54, 40, 41, 46, 99, 111, 108, 108, 101, 99, 116, 40, 41]))
                    })).concat(&self::ml_string::Text::from_units(vec![44, 32, 109, 101, 115, 115, 97, 103, 101, 95, 112, 114, 101, 115, 101, 110, 116, 58, 32]))).concat(&present)).concat(&self::ml_string::Text::from_units(vec![32, 125, 41, 46, 114, 97, 105, 115, 101, 40, 41]))
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn read_source_exception_ignored_pattern_fields(fields: f64) -> self::ml_string::Text {
    if (fields == (4f64)) {
        self::ml_string::Text::from_units(vec![32, 95, 32, 95])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn accept_source_exception_payload_fields(output: bool, names: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> bool {
    ((((((((((output && (({ let ml_reference_array125 = names.clone(); let ml_reference_projection125 = ml_reference_array125.borrow().len() as f64; ml_reference_projection125 }) == (4f64))) && (({ let ml_reference_array126 = kinds.clone(); let ml_reference_projection126 = ml_reference_array126.borrow().len() as f64; ml_reference_projection126 }) == (4f64))) && {
        let source_ml_source2_comparison_operand567 = {
            let source_ml_source2_array_read_value565 = names.clone();
            {
                let source_ml_source2_array_read_index566 = 0f64;
                { let ml_reference_array127 = source_ml_source2_array_read_value565.clone(); let ml_reference_position127 = source_ml_source2_array_read_index566; let ml_reference_projection127 = self::ml_array::optional_at(&*ml_reference_array127.borrow(), self::ml_array::number_index(ml_reference_position127)); ml_reference_projection127 }
            }
        };
        {
            let source_ml_source2_comparison_operand568 = self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116]);
            match (source_ml_source2_comparison_operand567.clone(), source_ml_source2_comparison_operand568.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand571 = {
            let source_ml_source2_array_read_value569 = kinds.clone();
            {
                let source_ml_source2_array_read_index570 = 0f64;
                { let ml_reference_array128 = source_ml_source2_array_read_value569.clone(); let ml_reference_position128 = source_ml_source2_array_read_index570; let ml_reference_projection128 = self::ml_array::optional_at(&*ml_reference_array128.borrow(), self::ml_array::number_index(ml_reference_position128)); ml_reference_projection128 }
            }
        };
        {
            let source_ml_source2_comparison_operand572 = self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116]);
            match (source_ml_source2_comparison_operand571.clone(), source_ml_source2_comparison_operand572.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand575 = {
            let source_ml_source2_array_read_value573 = names.clone();
            {
                let source_ml_source2_array_read_index574 = 1f64;
                { let ml_reference_array129 = source_ml_source2_array_read_value573.clone(); let ml_reference_position129 = source_ml_source2_array_read_index574; let ml_reference_projection129 = self::ml_array::optional_at(&*ml_reference_array129.borrow(), self::ml_array::number_index(ml_reference_position129)); ml_reference_projection129 }
            }
        };
        {
            let source_ml_source2_comparison_operand576 = self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101]);
            match (source_ml_source2_comparison_operand575.clone(), source_ml_source2_comparison_operand576.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand579 = {
            let source_ml_source2_array_read_value577 = kinds.clone();
            {
                let source_ml_source2_array_read_index578 = 1f64;
                { let ml_reference_array130 = source_ml_source2_array_read_value577.clone(); let ml_reference_position130 = source_ml_source2_array_read_index578; let ml_reference_projection130 = self::ml_array::optional_at(&*ml_reference_array130.borrow(), self::ml_array::number_index(ml_reference_position130)); ml_reference_projection130 }
            }
        };
        {
            let source_ml_source2_comparison_operand580 = self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]);
            match (source_ml_source2_comparison_operand579.clone(), source_ml_source2_comparison_operand580.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand583 = {
            let source_ml_source2_array_read_value581 = names.clone();
            {
                let source_ml_source2_array_read_index582 = 2f64;
                { let ml_reference_array131 = source_ml_source2_array_read_value581.clone(); let ml_reference_position131 = source_ml_source2_array_read_index582; let ml_reference_projection131 = self::ml_array::optional_at(&*ml_reference_array131.borrow(), self::ml_array::number_index(ml_reference_position131)); ml_reference_projection131 }
            }
        };
        {
            let source_ml_source2_comparison_operand584 = self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 78, 97, 109, 101]);
            match (source_ml_source2_comparison_operand583.clone(), source_ml_source2_comparison_operand584.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand587 = {
            let source_ml_source2_array_read_value585 = kinds.clone();
            {
                let source_ml_source2_array_read_index586 = 2f64;
                { let ml_reference_array132 = source_ml_source2_array_read_value585.clone(); let ml_reference_position132 = source_ml_source2_array_read_index586; let ml_reference_projection132 = self::ml_array::optional_at(&*ml_reference_array132.borrow(), self::ml_array::number_index(ml_reference_position132)); ml_reference_projection132 }
            }
        };
        {
            let source_ml_source2_comparison_operand588 = self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]);
            match (source_ml_source2_comparison_operand587.clone(), source_ml_source2_comparison_operand588.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand591 = {
            let source_ml_source2_array_read_value589 = names.clone();
            {
                let source_ml_source2_array_read_index590 = 3f64;
                { let ml_reference_array133 = source_ml_source2_array_read_value589.clone(); let ml_reference_position133 = source_ml_source2_array_read_index590; let ml_reference_projection133 = self::ml_array::optional_at(&*ml_reference_array133.borrow(), self::ml_array::number_index(ml_reference_position133)); ml_reference_projection133 }
            }
        };
        {
            let source_ml_source2_comparison_operand592 = self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101, 80, 114, 101, 115, 101, 110, 116]);
            match (source_ml_source2_comparison_operand591.clone(), source_ml_source2_comparison_operand592.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand595 = {
            let source_ml_source2_array_read_value593 = kinds.clone();
            {
                let source_ml_source2_array_read_index594 = 3f64;
                { let ml_reference_array134 = source_ml_source2_array_read_value593.clone(); let ml_reference_position134 = source_ml_source2_array_read_index594; let ml_reference_projection134 = self::ml_array::optional_at(&*ml_reference_array134.borrow(), self::ml_array::number_index(ml_reference_position134)); ml_reference_projection134 }
            }
        };
        {
            let source_ml_source2_comparison_operand596 = self::ml_string::Text::from_units(vec![98, 111, 111, 108]);
            match (source_ml_source2_comparison_operand595.clone(), source_ml_source2_comparison_operand596.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    })
}

pub fn read_source_exception_observation_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![109, 108, 95, 115, 111, 117, 114, 99, 101, 95, 101, 120, 99, 101, 112, 116, 105, 111, 110, 95, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 115, 111, 117, 114, 99, 101, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 108, 97, 115, 115, 44, 32, 101, 120, 97, 99, 116, 32, 85, 84, 70, 45, 49, 54, 32, 109, 101, 115, 115, 97, 103, 101, 32, 97, 110, 100, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 112, 114, 101, 115, 101, 110, 99, 101, 59, 32, 109, 97, 105, 110, 32, 112, 114, 111, 106, 101, 99, 116, 115, 32, 116, 104, 101, 32, 100, 105, 115, 112, 108, 97, 121, 32, 109, 101, 115, 115, 97, 103, 101, 32, 97, 116, 32, 105, 116, 115, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 99, 101, 115, 115, 32, 98, 111, 117, 110, 100, 97, 114, 121])
}

pub fn read_source_exception_rocq_output_prelude() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![68, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 109, 108, 95, 101, 109, 105, 116, 95, 115, 111, 117, 114, 99, 101, 95, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 123, 69, 32, 58, 32, 84, 121, 112, 101, 125, 32, 40, 108, 105, 110, 101, 115, 32, 58, 32, 108, 105, 115, 116, 32, 115, 116, 114, 105, 110, 103, 41, 32, 40, 110, 101, 120, 116, 32, 58, 32, 108, 105, 115, 116, 32, 115, 116, 114, 105, 110, 103, 32, 42, 32, 111, 112, 116, 105, 111, 110, 32, 69, 41, 32, 58, 32, 108, 105, 115, 116, 32, 115, 116, 114, 105, 110, 103, 32, 42, 32, 111, 112, 116, 105, 111, 110, 32, 69, 32, 58, 61, 32, 40, 76, 105, 115, 116, 46, 97, 112, 112, 32, 108, 105, 110, 101, 115, 32, 40, 102, 115, 116, 32, 110, 101, 120, 116, 41, 44, 32, 115, 110, 100, 32, 110, 101, 120, 116, 41, 46])
}

pub fn read_java_script_numeric_operation_failure(operation: self::ml_string::Text, left: self::ml_string::Text, right: self::ml_string::Text) -> self::ml_string::Text {
    {
        let first = ((left == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (left == (self::ml_string::Text::from_units(vec![110, 97, 116]))));
        {
            let second = ((right == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (right == (self::ml_string::Text::from_units(vec![110, 97, 116]))));
            if ((first && (right == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))) || ((left == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) && second)) {
                self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100])
            } else {
                if ((first && second) && (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116, 85, 110, 115, 105, 103, 110, 101, 100])))) {
                    self::ml_string::Text::from_units(vec![117, 110, 115, 105, 103, 110, 101, 100])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn read_java_script_numeric_operation_message(failure: self::ml_string::Text) -> self::ml_string::Text {
    if (failure == (self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100]))) {
        self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 109, 105, 120, 32, 66, 105, 103, 73, 110, 116, 32, 97, 110, 100, 32, 111, 116, 104, 101, 114, 32, 116, 121, 112, 101, 115, 44, 32, 117, 115, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 115])
    } else {
        if (failure == (self::ml_string::Text::from_units(vec![117, 110, 115, 105, 103, 110, 101, 100]))) {
            self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 115, 32, 104, 97, 118, 101, 32, 110, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 114, 105, 103, 104, 116, 32, 115, 104, 105, 102, 116, 44, 32, 117, 115, 101, 32, 62, 62, 32, 105, 110, 115, 116, 101, 97, 100])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn test_java_script_error_constructor_mode(mode: self::ml_string::Text) -> bool {
    (mode == (self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114])))
}

pub fn test_java_script_error_intrinsic_mode(mode: self::ml_string::Text) -> bool {
    (mode == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99])))
}

pub fn read_java_script_error_construction_kind_error(error_class: self::ml_string::Text, bind_receiver: bool, mode: self::ml_string::Text) -> self::ml_string::Text {
    if ((self::read_java_script_source_exception_operation(error_class.clone())) == (self::ml_string::Text::from_units(vec![]))) {
        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 108, 97, 115, 115])
    } else {
        if (bind_receiver && !(self::test_java_script_error_constructor_mode(mode.clone()))) {
            self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 114, 105, 118, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 98, 105, 110, 100, 115, 32, 116, 104, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_java_script_error_allocation_directory(heap: self::JavaScriptOrdinaryHeap) -> self::JavaScriptErrorAllocationDirectory {
    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values135 = { let ml_reference_value136 = (match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value597, _, _, _, _, _) => {
            source_ml_source2_data_property_value597.clone()
        }
    }).borrow().clone(); ml_reference_value136 }; let ml_callback135 = std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptErrorAllocationIdentity { self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value598, _, _, _, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value598.clone()
        }
        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value599, _, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value599.clone()
        }
        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value600, _, _, _, _) => {
            source_ml_source2_data_property_value600.clone()
        }
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value601, _, _, _, _, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value601.clone()
        }
        self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value602, _, _, _, _, _) => {
            source_ml_source2_data_property_value602.clone()
        }
        self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value603, _, _, _, _, _) => {
            source_ml_source2_data_property_value603.clone()
        }
        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value604, _, _, _, _, _) => {
            source_ml_source2_data_property_value604.clone()
        }
        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value605, _, _, _, _, _) => {
            source_ml_source2_data_property_value605.clone()
        }
        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value606, _, _, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value606.clone()
        }
        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value607, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value607.clone()
        }
        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value608, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value608.clone()
        }
        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value609, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value609.clone()
        }
        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value610, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value610.clone()
        }
    }, match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
        }
    }, match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value611, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value611
        }
        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value612, _, _, _, _) => {
            source_ml_source2_data_property_value612
        }
        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value613, _) => {
            source_ml_source2_data_property_value613
        }
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value614, _, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value614
        }
        self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value615, _, _) => {
            source_ml_source2_data_property_value615
        }
        self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value616, _, _) => {
            source_ml_source2_data_property_value616
        }
        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value617, _, _) => {
            source_ml_source2_data_property_value617
        }
        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value618, _, _) => {
            source_ml_source2_data_property_value618
        }
        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value619, _) => {
            source_ml_source2_data_property_value619
        }
        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value620, _, _, _) => {
            source_ml_source2_data_property_value620
        }
        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value621, _, _, _) => {
            source_ml_source2_data_property_value621
        }
        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value622, _, _, _) => {
            source_ml_source2_data_property_value622
        }
        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value623, _, _, _) => {
            source_ml_source2_data_property_value623
        }
    }) }); ml_values135.iter().cloned().enumerate().map(|(ml_index135, ml_value135)| ml_callback135(ml_value135)).collect::<Vec<_>>() })), std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(Vec::<self::ml::Big>::new(), { let ml_reference_value137 = (match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value624, _, _, _, _) => {
            source_ml_source2_data_property_value624.clone()
        }
    }).borrow().clone(); ml_reference_value137 }))))
}

pub fn read_java_script_error_prior_directory_error(heap: self::JavaScriptOrdinaryHeap, before: self::JavaScriptErrorAllocationDirectory) -> self::ml_string::Text {
    if ((({ let ml_reference_array138 = match before.clone() {
        self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(source_ml_source2_data_property_value626, _) => {
            source_ml_source2_data_property_value626.clone()
        }
    }; let ml_reference_projection138 = ml_reference_array138.borrow().len() as f64; ml_reference_projection138 }) > (4096f64)) || (({ let ml_reference_array139 = match before.clone() {
        self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(_, source_ml_source2_data_property_value628) => {
            source_ml_source2_data_property_value628.clone()
        }
    }; let ml_reference_projection139 = ml_reference_array139.borrow().len() as f64; ml_reference_projection139 }) > (16384f64))) {
        self::ml_string::Text::from_units(vec![116, 104, 101, 32, 112, 114, 105, 111, 114, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116])
    } else {
        {
            let previous = self::ml::Big::from_i128(0);
            {
                let source_ml_source1_temporary_values10 = match before.clone() {
                    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(source_ml_source2_data_property_value629, _) => {
                        source_ml_source2_data_property_value629.clone()
                    }
                };
                {
                    let source_ml_source1_temporary_index10 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s630 = self::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop12(heap.clone(), previous.clone(), source_ml_source1_temporary_values10.clone(), source_ml_source1_temporary_index10.clone());
                        match source_ml_source2_s630.clone() {
                            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Done(previous_2) => {
                                {
                                    let previous_3 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_temporary_values11 = match before.clone() {
                                            self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(_, source_ml_source2_data_property_value631) => {
                                                source_ml_source2_data_property_value631.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_index11 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s632 = self::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop13(heap.clone(), before.clone(), previous_3.clone(), source_ml_source1_temporary_values11.clone(), source_ml_source1_temporary_index11.clone());
                                                match source_ml_source2_s632.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Done => {
                                                        self::ml_string::Text::from_units(vec![])
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(source_ml_source1_temporary_result13) => {
                                                        source_ml_source1_temporary_result13.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(source_ml_source1_temporary_result13_2) => {
                                source_ml_source1_temporary_result13_2.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_error_construction_operands_error(heap: self::JavaScriptOrdinaryHeap, constructor: self::JavaScriptSemanticValue, new_target: self::JavaScriptSemanticValue, prototype: self::JavaScriptSemanticValue, message: self::JavaScriptSemanticValue, present: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    {
        let source_ml_source1_temporary_values14 = std::rc::Rc::new(std::cell::RefCell::new(vec![constructor.clone(), new_target.clone(), prototype.clone(), message.clone(), present.clone()]));
        {
            let source_ml_source1_temporary_index14 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s633 = self::source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop15(heap.clone(), source_ml_source1_temporary_values14.clone(), source_ml_source1_temporary_index14.clone());
                match source_ml_source2_s633.clone() {
                    self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Done => {
                        {
                            let invalid_constructor = self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]);
                            if (match constructor.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f634) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source2_f635) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f636) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source2_f637) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f638, source_ml_source2_f639) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f640) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source2_f641, source_ml_source2_f642) => {
                                    false
                                }
                            } || !(match match constructor.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    None::<bool>
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    None::<bool>
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    None::<bool>
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    None::<bool>
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    None::<bool>
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    None::<bool>
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    None::<bool>
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    None::<bool>
                                }
                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value643) => {
                                    Some(source_ml_source2_data_property_value643)
                                }
                            } { Some(ml_present) => ml_present, None => false })) {
                                invalid_constructor.clone()
                            } else {
                                match new_target.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f644) => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Number(source_ml_source2_f645) => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f646) => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f647) => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f648, source_ml_source2_f649) => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f650) => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Object(identity, callable) => {
                                        if !callable {
                                            invalid_constructor.clone()
                                        } else {
                                            if match {
                                                let source_ml_source2_logical661 = match prototype.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f651) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Number(source_ml_source2_f652) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f653) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f654) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f655, source_ml_source2_f656) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f657) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f658, source_ml_source2_f659) => {
                                                        false
                                                    }
                                                };
                                                if source_ml_source2_logical661 {
                                                    Some(source_ml_source2_logical661)
                                                } else {
                                                    match prototype.clone() {
                                                        self::JavaScriptSemanticValue::Undefined => {
                                                            None::<bool>
                                                        }
                                                        self::JavaScriptSemanticValue::Null => {
                                                            None::<bool>
                                                        }
                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                            None::<bool>
                                                        }
                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                            None::<bool>
                                                        }
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            None::<bool>
                                                        }
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            None::<bool>
                                                        }
                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                            None::<bool>
                                                        }
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            None::<bool>
                                                        }
                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value660) => {
                                                            Some(source_ml_source2_data_property_value660)
                                                        }
                                                    }
                                                }
                                            } { Some(ml_present) => ml_present, None => false } {
                                                self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 69, 114, 114, 111, 114, 32, 115, 117, 98, 99, 108, 97, 115, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 79, 98, 106, 101, 99, 116])
                                            } else {
                                                if ((match message.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f662) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Number(source_ml_source2_f663) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f664) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f665) => {
                                                        false
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f666, source_ml_source2_f667) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f668) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f669, source_ml_source2_f670) => {
                                                        true
                                                    }
                                                } && match message.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f671) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Number(source_ml_source2_f672) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f673) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f674) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f675, source_ml_source2_f676) => {
                                                        false
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f677) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f678, source_ml_source2_f679) => {
                                                        true
                                                    }
                                                }) || match present.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f680) => {
                                                        false
                                                    }
                                                    self::JavaScriptSemanticValue::Number(source_ml_source2_f681) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f682) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f683) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f684, source_ml_source2_f685) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f686) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f687, source_ml_source2_f688) => {
                                                        true
                                                    }
                                                }) {
                                                    self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 85, 84, 70, 49, 54, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 105, 110, 100, 101, 112, 101, 110, 100, 101, 110, 116, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101])
                                                } else {
                                                    {
                                                        let object = {
                                                            let source_ml_source2_array_read_value691 = match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value689, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value689.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_array_read_index692 = self::find_java_script_heap_directory_object(match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value690, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value690.clone()
                                                                    }
                                                                }, identity.clone());
                                                                { let ml_reference_array140 = source_ml_source2_array_read_value691.clone(); let ml_reference_position140 = source_ml_source2_array_read_index692; let ml_reference_projection140 = self::ml_array::optional_at(&*ml_reference_array140.borrow(), self::ml_array::number_index(ml_reference_position140)); ml_reference_projection140 }
                                                            }
                                                        };
                                                        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::ml_string::Text::from_units(vec![110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 104, 101, 97, 112])
                                                        } else {
                                                            {
                                                                let property = {
                                                                    let source_ml_source2_array_read_value719 = match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value693, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value693.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value694, _, _, _) => {
                                                                            source_ml_source2_data_property_value694.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value695) => {
                                                                            source_ml_source2_data_property_value695.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value696, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value696.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value697, _) => {
                                                                            source_ml_source2_data_property_value697.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value698, _) => {
                                                                            source_ml_source2_data_property_value698.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value699, _) => {
                                                                            source_ml_source2_data_property_value699.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value700, _) => {
                                                                            source_ml_source2_data_property_value700.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value701) => {
                                                                            source_ml_source2_data_property_value701.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value702, _, _) => {
                                                                            source_ml_source2_data_property_value702.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value703, _, _) => {
                                                                            source_ml_source2_data_property_value703.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value704, _, _) => {
                                                                            source_ml_source2_data_property_value704.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value705, _, _) => {
                                                                            source_ml_source2_data_property_value705.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_array_read_index720 = self::find_java_script_heap_directory_property(match object.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value706, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value706.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value707, _, _, _) => {
                                                                                source_ml_source2_data_property_value707.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value708) => {
                                                                                source_ml_source2_data_property_value708.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value709, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value709.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value710, _) => {
                                                                                source_ml_source2_data_property_value710.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value711, _) => {
                                                                                source_ml_source2_data_property_value711.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value712, _) => {
                                                                                source_ml_source2_data_property_value712.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value713, _) => {
                                                                                source_ml_source2_data_property_value713.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value714) => {
                                                                                source_ml_source2_data_property_value714.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value715, _, _) => {
                                                                                source_ml_source2_data_property_value715.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value716, _, _) => {
                                                                                source_ml_source2_data_property_value716.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value717, _, _) => {
                                                                                source_ml_source2_data_property_value717.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value718, _, _) => {
                                                                                source_ml_source2_data_property_value718.clone()
                                                                            }
                                                                        }, self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![112f64, 114f64, 111f64, 116f64, 111f64, 116f64, 121f64, 112f64, 101f64]))));
                                                                        { let ml_reference_array141 = source_ml_source2_array_read_value719.clone(); let ml_reference_position141 = source_ml_source2_array_read_index720; let ml_reference_projection141 = self::ml_array::optional_at(&*ml_reference_array141.borrow(), self::ml_array::number_index(ml_reference_position141)); ml_reference_projection141 }
                                                                    }
                                                                };
                                                                if ((match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(({ let ml_reference_value142 = (match property.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value721) => {
                                                                        source_ml_source2_data_property_value721.clone()
                                                                    }
                                                                }).borrow().clone(); ml_reference_value142 }).9)) || !(self::test_java_script_semantic_same_value(({ let ml_reference_value143 = (match property.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value722) => {
                                                                        source_ml_source2_data_property_value722.clone()
                                                                    }
                                                                }).borrow().clone(); ml_reference_value143 }).8, prototype.clone()))) {
                                                                    self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 112, 114, 111, 102, 105, 108, 101, 32, 110, 101, 101, 100, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 111, 119, 110, 32, 100, 97, 116, 97, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 97, 110, 32, 105, 110, 118, 101, 110, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 111, 114, 32, 103, 101, 116, 116, 101, 114])
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
                    self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Return(source_ml_source1_temporary_result15) => {
                        source_ml_source1_temporary_result15.clone()
                    }
                }
            }
        }
    }
}

pub fn read_java_script_error_allocation_result_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue, prototype: self::JavaScriptSemanticValue, before: self::JavaScriptErrorAllocationDirectory) -> self::ml_string::Text {
    {
        let prior = self::read_java_script_error_prior_directory_error(heap.clone(), before.clone());
        if (prior != (self::ml_string::Text::from_units(vec![]))) {
            prior.clone()
        } else {
            {
                let error = self::read_java_script_heap_directory_value_error(heap.clone(), value.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    {
                        let invalid_result = self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 103, 108, 111, 98, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                        match value.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Null => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f723) => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source2_f724) => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source2_f725) => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source2_f726) => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f727, source_ml_source2_f728) => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f729) => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Object(identity, callable) => {
                                if ((callable || { let ml_values144 = { let ml_reference_value145 = (match before.clone() {
                                    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(source_ml_source2_data_property_value730, _) => {
                                        source_ml_source2_data_property_value730.clone()
                                    }
                                }).borrow().clone(); ml_reference_value145 }; let ml_callback144 = { let identity = identity.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorAllocationIdentity| -> bool { ((match row.clone() {
                                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value732, _, _) => {
                                        source_ml_source2_data_property_value732.clone()
                                    }
                                }) == identity) }) }; ml_values144.iter().cloned().enumerate().any(|(ml_index144, ml_value144)| ml_callback144(ml_value144)) }) || self::ml_array::includes(&{ let ml_reference_value146 = (match before.clone() {
                                    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(_, source_ml_source2_data_property_value734) => {
                                        source_ml_source2_data_property_value734.clone()
                                    }
                                }).borrow().clone(); ml_reference_value146 }, &identity.clone(), 0f64, false, |a, b| a == b)) {
                                    invalid_result.clone()
                                } else {
                                    {
                                        let object = {
                                            let source_ml_source2_array_read_value737 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value735, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value735.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_array_read_index738 = self::find_java_script_heap_directory_object(match heap.clone() {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value736, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value736.clone()
                                                    }
                                                }, identity.clone());
                                                { let ml_reference_array147 = source_ml_source2_array_read_value737.clone(); let ml_reference_position147 = source_ml_source2_array_read_index738; let ml_reference_projection147 = self::ml_array::optional_at(&*ml_reference_array147.borrow(), self::ml_array::number_index(ml_reference_position147)); ml_reference_projection147 }
                                            }
                                        };
                                        {
                                            let invalid_result_prototype = self::ml_string::Text::from_units(vec![116, 104, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 112, 97, 113, 117, 101, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 114, 105, 118, 101, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]);
                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                invalid_result_prototype.clone()
                                            } else {
                                                if (match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_f739, source_ml_source2_f740, source_ml_source2_f741, source_ml_source2_f742, source_ml_source2_f743, source_ml_source2_f744, source_ml_source2_f745, source_ml_source2_f746, source_ml_source2_f747, source_ml_source2_f748) => {
                                                        let source_ml_source2_f740 = *source_ml_source2_f740;
                                                        let source_ml_source2_f747 = *source_ml_source2_f747;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_f749, source_ml_source2_f750, source_ml_source2_f751, source_ml_source2_f752, source_ml_source2_f753, source_ml_source2_f754, source_ml_source2_f755, source_ml_source2_f756) => {
                                                        let source_ml_source2_f750 = *source_ml_source2_f750;
                                                        let source_ml_source2_f754 = *source_ml_source2_f754;
                                                        let source_ml_source2_f755 = *source_ml_source2_f755;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_f757, source_ml_source2_f758, source_ml_source2_f759, source_ml_source2_f760, source_ml_source2_f761) => {
                                                        let source_ml_source2_f758 = *source_ml_source2_f758;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_f762, source_ml_source2_f763, source_ml_source2_f764, source_ml_source2_f765, source_ml_source2_f766, source_ml_source2_f767, source_ml_source2_f768, source_ml_source2_f769, source_ml_source2_f770, source_ml_source2_f771, source_ml_source2_f772) => {
                                                        let source_ml_source2_f763 = *source_ml_source2_f763;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_f773, source_ml_source2_f774, source_ml_source2_f775, source_ml_source2_f776, source_ml_source2_f777, source_ml_source2_f778) => {
                                                        let source_ml_source2_f774 = *source_ml_source2_f774;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_f779, source_ml_source2_f780, source_ml_source2_f781, source_ml_source2_f782, source_ml_source2_f783, source_ml_source2_f784) => {
                                                        let source_ml_source2_f780 = *source_ml_source2_f780;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_f785, source_ml_source2_f786, source_ml_source2_f787, source_ml_source2_f788, source_ml_source2_f789, source_ml_source2_f790) => {
                                                        let source_ml_source2_f786 = *source_ml_source2_f786;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_f791, source_ml_source2_f792, source_ml_source2_f793, source_ml_source2_f794, source_ml_source2_f795, source_ml_source2_f796) => {
                                                        let source_ml_source2_f792 = *source_ml_source2_f792;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_f797, source_ml_source2_f798, source_ml_source2_f799, source_ml_source2_f800, source_ml_source2_f801, source_ml_source2_f802, source_ml_source2_f803, source_ml_source2_f804, source_ml_source2_f805) => {
                                                        let source_ml_source2_f798 = *source_ml_source2_f798;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_f806, source_ml_source2_f807, source_ml_source2_f808, source_ml_source2_f809, source_ml_source2_f810, source_ml_source2_f811, source_ml_source2_f812) => {
                                                        let source_ml_source2_f807 = *source_ml_source2_f807;
                                                        let source_ml_source2_f811 = *source_ml_source2_f811;
                                                        let source_ml_source2_f812 = *source_ml_source2_f812;
                                                        true
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_f813, source_ml_source2_f814, source_ml_source2_f815, source_ml_source2_f816, source_ml_source2_f817, source_ml_source2_f818, source_ml_source2_f819) => {
                                                        let source_ml_source2_f814 = *source_ml_source2_f814;
                                                        let source_ml_source2_f818 = *source_ml_source2_f818;
                                                        let source_ml_source2_f819 = *source_ml_source2_f819;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_f820, source_ml_source2_f821, source_ml_source2_f822, source_ml_source2_f823, source_ml_source2_f824, source_ml_source2_f825, source_ml_source2_f826) => {
                                                        let source_ml_source2_f821 = *source_ml_source2_f821;
                                                        let source_ml_source2_f825 = *source_ml_source2_f825;
                                                        let source_ml_source2_f826 = *source_ml_source2_f826;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_f827, source_ml_source2_f828, source_ml_source2_f829, source_ml_source2_f830, source_ml_source2_f831, source_ml_source2_f832, source_ml_source2_f833) => {
                                                        let source_ml_source2_f828 = *source_ml_source2_f828;
                                                        let source_ml_source2_f832 = *source_ml_source2_f832;
                                                        let source_ml_source2_f833 = *source_ml_source2_f833;
                                                        false
                                                    }
                                                } && self::test_java_script_semantic_same_value(match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value834, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value834 = *source_ml_source2_data_property_value834;
                                                        source_ml_source2_data_property_value834.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value835, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value835 = *source_ml_source2_data_property_value835;
                                                        source_ml_source2_data_property_value835.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value836, _, _, _) => {
                                                        let source_ml_source2_data_property_value836 = *source_ml_source2_data_property_value836;
                                                        source_ml_source2_data_property_value836.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value837, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value837 = *source_ml_source2_data_property_value837;
                                                        source_ml_source2_data_property_value837.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value838, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value838 = *source_ml_source2_data_property_value838;
                                                        source_ml_source2_data_property_value838.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value839, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value839 = *source_ml_source2_data_property_value839;
                                                        source_ml_source2_data_property_value839.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value840, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value840 = *source_ml_source2_data_property_value840;
                                                        source_ml_source2_data_property_value840.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value841, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value841 = *source_ml_source2_data_property_value841;
                                                        source_ml_source2_data_property_value841.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value842, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value842 = *source_ml_source2_data_property_value842;
                                                        source_ml_source2_data_property_value842.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value843, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value843 = *source_ml_source2_data_property_value843;
                                                        source_ml_source2_data_property_value843.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source2_data_property_value844, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value844 = *source_ml_source2_data_property_value844;
                                                        source_ml_source2_data_property_value844.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source2_data_property_value845, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value845 = *source_ml_source2_data_property_value845;
                                                        source_ml_source2_data_property_value845.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source2_data_property_value846, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value846 = *source_ml_source2_data_property_value846;
                                                        source_ml_source2_data_property_value846.clone()
                                                    }
                                                }, prototype.clone())) {
                                                    self::ml_string::Text::from_units(vec![])
                                                } else {
                                                    invalid_result_prototype.clone()
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ({ let ml_reference_array148 = units.clone(); let ml_reference_projection148 = ml_reference_array148.borrow().len() as f64; ml_reference_projection148 })) {
            {
                let unit = ({
                    let source_ml_source2_array_read_value847 = units.clone();
                    {
                        let source_ml_source2_array_read_index848 = at;
                        { let ml_reference_array149 = source_ml_source2_array_read_value847.clone(); let ml_reference_position149 = source_ml_source2_array_read_index848; let ml_reference_projection149 = self::ml_array::optional_at(&*ml_reference_array149.borrow(), self::ml_array::number_index(ml_reference_position149)); ml_reference_projection149 }
                    }
                }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}

pub fn source_ml_source1_temporary_test_java_script_semantic_text_equal_loop2(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut at: f64) -> self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f849) => {
                        {
                            let source_ml_source2_f873 = source_ml_source2_f849;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f850) => {
                        {
                            let source_ml_source2_f882 = source_ml_source2_f850;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f851) => {
                        {
                            let source_ml_source2_f891 = source_ml_source2_f851.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Text(units) => {
                        ({
                            let source_ml_source2_array_read_value897 = units.clone();
                            {
                                let source_ml_source2_array_read_index898 = at;
                                { let ml_reference_array150 = source_ml_source2_array_read_value897.clone(); let ml_reference_position150 = source_ml_source2_array_read_index898; let ml_reference_projection150 = self::ml_array::optional_at(&*ml_reference_array150.borrow(), self::ml_array::number_index(ml_reference_position150)); ml_reference_projection150 }
                            }
                        }).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f852, source_ml_source2_f853) => {
                        {
                            let source_ml_source2_f903 = source_ml_source2_f852.clone();
                            {
                                let text = source_ml_source2_f853.clone();
                                {
                                    let source_ml_source2_string_unit_operand907 = text.clone();
                                    {
                                        let source_ml_source2_string_unit_operand908 = at;
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
                                                    let source_ml_source2_string_unit_operand914 = {
                                                        let source_ml_source2_string_range_operand909 = text_2.clone();
                                                        {
                                                            let source_ml_source2_string_range_operand910 = index;
                                                            {
                                                                let source_ml_source2_string_range_operand911 = (index + 1f64);
                                                                {
                                                                    let source_ml_source2_string_range_position912 = source_ml_source2_string_range_operand910;
                                                                    {
                                                                        let source_ml_source2_string_range_position913 = source_ml_source2_string_range_operand911;
                                                                        source_ml_source2_string_range_operand909.slice(source_ml_source2_string_range_position912, source_ml_source2_string_range_position913)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_string_unit_operand915 = 0f64;
                                                        source_ml_source2_string_unit_operand914.code_point_at(source_ml_source2_string_unit_operand915)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source2_string_unit_operand907.clone(), source_ml_source2_string_unit_operand908)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f854) => {
                        {
                            let source_ml_source2_f921 = source_ml_source2_f854.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f855, source_ml_source2_f856) => {
                        {
                            let source_ml_source2_f930 = source_ml_source2_f855.clone();
                            {
                                let source_ml_source2_f931 = source_ml_source2_f856;
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f932) => {
                            {
                                let source_ml_source2_f956 = source_ml_source2_f932;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f933) => {
                            {
                                let source_ml_source2_f965 = source_ml_source2_f933;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f934) => {
                            {
                                let source_ml_source2_f974 = source_ml_source2_f934.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units_2) => {
                            ({
                                let source_ml_source2_array_read_value980 = units_2.clone();
                                {
                                    let source_ml_source2_array_read_index981 = at;
                                    { let ml_reference_array151 = source_ml_source2_array_read_value980.clone(); let ml_reference_position151 = source_ml_source2_array_read_index981; let ml_reference_projection151 = self::ml_array::optional_at(&*ml_reference_array151.borrow(), self::ml_array::number_index(ml_reference_position151)); ml_reference_projection151 }
                                }
                            }).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f935, source_ml_source2_f936) => {
                            {
                                let source_ml_source2_f986 = source_ml_source2_f935.clone();
                                {
                                    let text_2 = source_ml_source2_f936.clone();
                                    {
                                        let source_ml_source2_string_unit_operand990 = text_2.clone();
                                        {
                                            let source_ml_source2_string_unit_operand991 = at;
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
                                                        let source_ml_source2_string_unit_operand997 = {
                                                            let source_ml_source2_string_range_operand992 = text_3.clone();
                                                            {
                                                                let source_ml_source2_string_range_operand993 = index;
                                                                {
                                                                    let source_ml_source2_string_range_operand994 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source2_string_range_position995 = source_ml_source2_string_range_operand993;
                                                                        {
                                                                            let source_ml_source2_string_range_position996 = source_ml_source2_string_range_operand994;
                                                                            source_ml_source2_string_range_operand992.slice(source_ml_source2_string_range_position995, source_ml_source2_string_range_position996)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_string_unit_operand998 = 0f64;
                                                            source_ml_source2_string_unit_operand997.code_point_at(source_ml_source2_string_unit_operand998)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source2_string_unit_operand990.clone(), source_ml_source2_string_unit_operand991)
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f937) => {
                            {
                                let source_ml_source2_f1004 = source_ml_source2_f937.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f938, source_ml_source2_f939) => {
                            {
                                let source_ml_source2_f1013 = source_ml_source2_f938.clone();
                                {
                                    let source_ml_source2_f1014 = source_ml_source2_f939;
                                    (-1f64)
                                }
                            }
                        }
                    };
                    if (a != b) {
                        self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Return(false)
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
            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_semantic_text_string_loop4(mut text: self::ml_string::Text, mut source_ml_source1_temporary_values3: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index3: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_source1_temporary_index3 < ({ let ml_reference_array152 = source_ml_source1_temporary_values3.clone(); let ml_reference_projection152 = self::ml::Big::from_u128(ml_reference_array152.borrow().len() as u128); ml_reference_projection152 })) {
            {
                let unit = {
                    let source_ml_source2_array_read_value1015 = source_ml_source1_temporary_values3.clone();
                    {
                        let source_ml_source2_array_read_index1016 = source_ml_source1_temporary_index3.clone();
                        { let ml_reference_array153 = source_ml_source2_array_read_value1015.clone(); let ml_reference_position153 = source_ml_source2_array_read_index1016.clone(); let ml_reference_projection153 = self::ml_array::at(&*ml_reference_array153.borrow(), ml_reference_position153.to_index()); ml_reference_projection153 }
                    }
                };
                {
                    let text_2 = text.concat(&{
                        let source_ml_source2_character_argument1017 = unit;
                        self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source2_character_argument1017]).into_iter().map(|ml_unit| ml_unit as u16).collect())
                    });
                    {
                        let source_ml_source1_temporary_index3_2 = source_ml_source1_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_source1_temporary_index3) = (text_2.clone(), source_ml_source1_temporary_index3_2.clone());
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

pub fn source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(mut value: self::JavaScriptSemanticValue, mut length: f64, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < length) {
            {
                let units_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value154 = units.borrow().clone(); ml_reference_value154 }, vec![self::read_java_script_semantic_text_unit(value.clone(), at)])));
                {
                    let at_2 = (at + 1f64);
                    {
                        (units, at) = (units_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            units.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done,
    MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(f64),
}

pub fn source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop6(mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    loop {
        return if (at < ({ let ml_reference_array155 = properties.clone(); let ml_reference_projection155 = ml_reference_array155.borrow().len() as f64; ml_reference_projection155 })) {
            {
                let property = {
                    let source_ml_source2_array_read_value1018 = properties.clone();
                    {
                        let source_ml_source2_array_read_index1019 = at;
                        { let ml_reference_array156 = source_ml_source2_array_read_value1018.clone(); let ml_reference_position156 = source_ml_source2_array_read_index1019; let ml_reference_projection156 = self::ml_array::optional_at(&*ml_reference_array156.borrow(), self::ml_array::number_index(ml_reference_position156)); ml_reference_projection156 }
                    }
                };
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value(match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value1020, _) => {
                        let source_ml_source2_data_property_value1020 = *source_ml_source2_data_property_value1020;
                        source_ml_source2_data_property_value1020.clone()
                    }
                }, key.clone())) {
                    self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(at)
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
            self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7 {
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Next,
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Done,
    MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(f64),
}

pub fn source_ml_source1_temporary_find_java_script_heap_directory_object_loop8(mut objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = {
                        let source_ml_source2_array_read_value1021 = objects.clone();
                        {
                            let source_ml_source2_array_read_index1022 = middle;
                            { let ml_reference_array157 = source_ml_source2_array_read_value1021.clone(); let ml_reference_position157 = source_ml_source2_array_read_index1022; let ml_reference_projection157 = self::ml_array::optional_at(&*ml_reference_array157.borrow(), self::ml_array::number_index(ml_reference_position157)); ml_reference_projection157 }
                        }
                    };
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value1036, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1036.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value1037, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1037.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value1038, _, _, _, _) => {
                                source_ml_source2_data_property_value1038.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value1039, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1039.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value1040, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1040.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value1041, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1041.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value1042, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1042.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value1043, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1043.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value1044, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1044.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value1045, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1045.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value1046, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1046.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value1047, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1047.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value1048, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1048.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value1062, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1062.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value1063, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1063.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value1064, _, _, _, _) => {
                                    source_ml_source2_data_property_value1064.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value1065, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1065.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value1066, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1066.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value1067, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1067.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value1068, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1068.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value1069, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1069.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value1070, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1070.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value1071, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1071.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value1072, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1072.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value1073, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1073.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value1074, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1074.clone()
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
            self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result {
    MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Done,
    MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(bool),
}

pub fn source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop9(mut symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = {
                        let source_ml_source2_array_read_value1075 = symbols.clone();
                        {
                            let source_ml_source2_array_read_index1076 = middle;
                            { let ml_reference_array158 = source_ml_source2_array_read_value1075.clone(); let ml_reference_position158 = source_ml_source2_array_read_index1076; let ml_reference_projection158 = self::ml_array::optional_at(&*ml_reference_array158.borrow(), self::ml_array::number_index(ml_reference_position158)); ml_reference_projection158 }
                        }
                    };
                    if {
                        let source_ml_source2_comparison_operand1077 = symbol.clone();
                        {
                            let source_ml_source2_comparison_operand1078 = None::<()>;
                            match (source_ml_source2_comparison_operand1077.clone(), source_ml_source2_comparison_operand1078.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                        }
                    } {
                        self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(true)
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
            self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result {
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Done,
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop13(mut heap: self::JavaScriptOrdinaryHeap, mut before: self::JavaScriptErrorAllocationDirectory, mut previous: self::ml::Big, mut source_ml_source1_temporary_values11: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut source_ml_source1_temporary_index11: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result {
    loop {
        return if (source_ml_source1_temporary_index11 < ({ let ml_reference_array159 = source_ml_source1_temporary_values11.clone(); let ml_reference_projection159 = self::ml::Big::from_u128(ml_reference_array159.borrow().len() as u128); ml_reference_projection159 })) {
            {
                let symbol = {
                    let source_ml_source2_array_read_value1079 = source_ml_source1_temporary_values11.clone();
                    {
                        let source_ml_source2_array_read_index1080 = source_ml_source1_temporary_index11.clone();
                        { let ml_reference_array160 = source_ml_source2_array_read_value1079.clone(); let ml_reference_position160 = source_ml_source2_array_read_index1080.clone(); let ml_reference_projection160 = self::ml_array::at(&*ml_reference_array160.borrow(), ml_reference_position160.to_index()); ml_reference_projection160 }
                    }
                };
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || { let ml_values161 = { let ml_reference_value162 = (match before.clone() {
                    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(source_ml_source2_data_property_value1081, _) => {
                        source_ml_source2_data_property_value1081.clone()
                    }
                }).borrow().clone(); ml_reference_value162 }; let ml_callback161 = { let symbol = symbol.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorAllocationIdentity| -> bool { ((match row.clone() {
                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value1083, _, _) => {
                        source_ml_source2_data_property_value1083.clone()
                    }
                }) == symbol) }) }; ml_values161.iter().cloned().enumerate().any(|(ml_index161, ml_value161)| ml_callback161(ml_value161)) }) {
                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(self::ml_string::Text::from_units(vec![112, 114, 105, 111, 114, 32, 83, 121, 109, 98, 111, 108, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = symbol.clone();
                        if !(self::ml_array::includes(&{ let ml_reference_value163 = (match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value1085, _, _, _, _) => {
                                source_ml_source2_data_property_value1085.clone()
                            }
                        }).borrow().clone(); ml_reference_value163 }, &symbol.clone(), 0f64, false, |a, b| a == b)) {
                            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                        } else {
                            {
                                let source_ml_source1_temporary_index11_2 = source_ml_source1_temporary_index11.add(&self::ml::Big::from_i128(1));
                                {
                                    (previous, source_ml_source1_temporary_index11) = (previous_2.clone(), source_ml_source1_temporary_index11_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result {
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop12(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_source1_temporary_values10: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorAllocationIdentity>>>, mut source_ml_source1_temporary_index10: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result {
    loop {
        return if (source_ml_source1_temporary_index10 < ({ let ml_reference_array164 = source_ml_source1_temporary_values10.clone(); let ml_reference_projection164 = self::ml::Big::from_u128(ml_reference_array164.borrow().len() as u128); ml_reference_projection164 })) {
            {
                let row = {
                    let source_ml_source2_array_read_value1086 = source_ml_source1_temporary_values10.clone();
                    {
                        let source_ml_source2_array_read_index1087 = source_ml_source1_temporary_index10.clone();
                        { let ml_reference_array165 = source_ml_source2_array_read_value1086.clone(); let ml_reference_position165 = source_ml_source2_array_read_index1087.clone(); let ml_reference_projection165 = self::ml_array::at(&*ml_reference_array165.borrow(), ml_reference_position165.to_index()); ml_reference_projection165 }
                    }
                };
                if (((match row.clone() {
                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value1089, _, _) => {
                        source_ml_source2_data_property_value1089.clone()
                    }
                }) <= previous) || ((match row.clone() {
                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value1091, _, _) => {
                        source_ml_source2_data_property_value1091.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(self::ml_string::Text::from_units(vec![112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 115, 116, 114, 105, 99, 116, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 111, 114, 100, 101, 114]))
                } else {
                    {
                        let previous_2 = match row.clone() {
                            self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value1092, _, _) => {
                                source_ml_source2_data_property_value1092.clone()
                            }
                        };
                        {
                            let object = {
                                let source_ml_source2_array_read_value1096 = match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value1093, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1093.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_array_read_index1097 = self::find_java_script_heap_directory_object(match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value1094, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1094.clone()
                                        }
                                    }, match row.clone() {
                                        self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value1095, _, _) => {
                                            source_ml_source2_data_property_value1095.clone()
                                        }
                                    });
                                    { let ml_reference_array166 = source_ml_source2_array_read_value1096.clone(); let ml_reference_position166 = source_ml_source2_array_read_index1097; let ml_reference_projection166 = self::ml_array::optional_at(&*ml_reference_array166.borrow(), self::ml_array::number_index(ml_reference_position166)); ml_reference_projection166 }
                                }
                            };
                            if ((match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                }
                            }) != (match row.clone() {
                                self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(_, source_ml_source2_data_property_value1099, _) => {
                                    source_ml_source2_data_property_value1099.clone()
                                }
                            }))) || ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value1114, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1114
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value1115, _, _, _, _) => {
                                    source_ml_source2_data_property_value1115
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value1116, _) => {
                                    source_ml_source2_data_property_value1116
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value1117, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1117
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value1118, _, _) => {
                                    source_ml_source2_data_property_value1118
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value1119, _, _) => {
                                    source_ml_source2_data_property_value1119
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value1120, _, _) => {
                                    source_ml_source2_data_property_value1120
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value1121, _, _) => {
                                    source_ml_source2_data_property_value1121
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value1122, _) => {
                                    source_ml_source2_data_property_value1122
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value1123, _, _, _) => {
                                    source_ml_source2_data_property_value1123
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value1124, _, _, _) => {
                                    source_ml_source2_data_property_value1124
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value1125, _, _, _) => {
                                    source_ml_source2_data_property_value1125
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value1126, _, _, _) => {
                                    source_ml_source2_data_property_value1126
                                }
                            }) != (match row.clone() {
                                self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(_, _, source_ml_source2_data_property_value1127) => {
                                    source_ml_source2_data_property_value1127
                                }
                            }))) {
                                self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                            } else {
                                {
                                    let source_ml_source1_temporary_index10_2 = source_ml_source1_temporary_index10.add(&self::ml::Big::from_i128(1));
                                    {
                                        (previous, source_ml_source1_temporary_index10) = (previous_2.clone(), source_ml_source1_temporary_index10_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result {
    MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Done,
    MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop15(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values14: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_source1_temporary_index14: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result {
    loop {
        return if (source_ml_source1_temporary_index14 < ({ let ml_reference_array167 = source_ml_source1_temporary_values14.clone(); let ml_reference_projection167 = self::ml::Big::from_u128(ml_reference_array167.borrow().len() as u128); ml_reference_projection167 })) {
            {
                let value = {
                    let source_ml_source2_array_read_value1128 = source_ml_source1_temporary_values14.clone();
                    {
                        let source_ml_source2_array_read_index1129 = source_ml_source1_temporary_index14.clone();
                        { let ml_reference_array168 = source_ml_source2_array_read_value1128.clone(); let ml_reference_position168 = source_ml_source2_array_read_index1129.clone(); let ml_reference_projection168 = self::ml_array::at(&*ml_reference_array168.borrow(), ml_reference_position168.to_index()); ml_reference_projection168 }
                    }
                };
                {
                    let error = self::read_java_script_heap_directory_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index14_2 = source_ml_source1_temporary_index14.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index14 = source_ml_source1_temporary_index14_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Done
        };
    }
}

fn ml_main() {
    assert!(((self::test_java_script_error_constructor_mode(self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114]))) == (true)), "assertion 1");
    assert!(((self::test_java_script_error_constructor_mode(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) == (false)), "assertion 2");
    assert!(((self::test_java_script_error_intrinsic_mode(self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]))) == (true)), "assertion 3");
    assert!(((self::test_java_script_error_intrinsic_mode(self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114]))) == (false)), "assertion 4");
    assert!(((self::read_java_script_error_construction_kind_error(self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]), true, self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 5");
    assert!(((self::read_java_script_error_construction_kind_error(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), false, self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 6");
    assert!(((self::read_java_script_error_construction_kind_error(self::ml_string::Text::from_units(vec![67, 117, 115, 116, 111, 109]), false, self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 108, 97, 115, 115]))), "assertion 7");
    assert!(((self::read_java_script_error_construction_kind_error(self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]), true, self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) == (self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 114, 105, 118, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 98, 105, 110, 100, 115, 32, 116, 104, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114]))), "assertion 8");
    assert!(((self::read_java_script_error_allocation_result_error({
        let source_ml_source2_constructor_field1140 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
            let source_ml_source2_constructor_field1130 = self::ml::Big::from_i128(1);
            {
                let source_ml_source2_constructor_field1131 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1132 = true;
                    {
                        let source_ml_source2_constructor_field1133 = false;
                        {
                            let source_ml_source2_constructor_field1134 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1130.clone(), Box::new(source_ml_source2_constructor_field1131.clone()), source_ml_source2_constructor_field1132, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1133, source_ml_source2_constructor_field1134.clone())
                        }
                    }
                }
            }
        }, {
            let source_ml_source2_constructor_field1135 = self::ml::Big::from_i128(2);
            {
                let source_ml_source2_constructor_field1136 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1137 = true;
                    {
                        let source_ml_source2_constructor_field1138 = true;
                        {
                            let source_ml_source2_constructor_field1139 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1135.clone(), Box::new(source_ml_source2_constructor_field1136.clone()), source_ml_source2_constructor_field1137, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1138, source_ml_source2_constructor_field1139.clone())
                        }
                    }
                }
            }
        }, self::JavaScriptOrdinaryObject::HostErrorObject(self::ml::Big::from_i128(7), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false)), true, false, std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new())), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)))]));
        {
            let source_ml_source2_constructor_field1141 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]));
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_constructor_field1140.clone(), source_ml_source2_constructor_field1141.clone(), None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,)>)
        }
    }, self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(7), false), self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false), self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(1), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), false), self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(2), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), true)])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]))))) == (self::ml_string::Text::from_units(vec![]))), "assertion 9");
    assert!(((self::read_java_script_error_allocation_result_error({
        let source_ml_source2_constructor_field1152 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
            let source_ml_source2_constructor_field1142 = self::ml::Big::from_i128(1);
            {
                let source_ml_source2_constructor_field1143 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1144 = true;
                    {
                        let source_ml_source2_constructor_field1145 = false;
                        {
                            let source_ml_source2_constructor_field1146 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1142.clone(), Box::new(source_ml_source2_constructor_field1143.clone()), source_ml_source2_constructor_field1144, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1145, source_ml_source2_constructor_field1146.clone())
                        }
                    }
                }
            }
        }, {
            let source_ml_source2_constructor_field1147 = self::ml::Big::from_i128(2);
            {
                let source_ml_source2_constructor_field1148 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1149 = true;
                    {
                        let source_ml_source2_constructor_field1150 = true;
                        {
                            let source_ml_source2_constructor_field1151 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1147.clone(), Box::new(source_ml_source2_constructor_field1148.clone()), source_ml_source2_constructor_field1149, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1150, source_ml_source2_constructor_field1151.clone())
                        }
                    }
                }
            }
        }, self::JavaScriptOrdinaryObject::HostErrorObject(self::ml::Big::from_i128(7), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false)), true, false, std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new())), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)))]));
        {
            let source_ml_source2_constructor_field1153 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]));
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_constructor_field1152.clone(), source_ml_source2_constructor_field1153.clone(), None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,)>)
        }
    }, self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false), self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false), self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(1), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), false), self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(2), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), true)])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]))))) == (self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 103, 108, 111, 98, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]))), "assertion 10");
    assert!(((self::read_java_script_error_prior_directory_error({
        let source_ml_source2_constructor_field1159 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
            let source_ml_source2_constructor_field1154 = self::ml::Big::from_i128(1);
            {
                let source_ml_source2_constructor_field1155 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1156 = true;
                    {
                        let source_ml_source2_constructor_field1157 = false;
                        {
                            let source_ml_source2_constructor_field1158 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1154.clone(), Box::new(source_ml_source2_constructor_field1155.clone()), source_ml_source2_constructor_field1156, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1157, source_ml_source2_constructor_field1158.clone())
                        }
                    }
                }
            }
        }, self::JavaScriptOrdinaryObject::HostErrorObject(self::ml::Big::from_i128(7), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false)), true, false, std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new())), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)))]));
        {
            let source_ml_source2_constructor_field1160 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]));
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_constructor_field1159.clone(), source_ml_source2_constructor_field1160.clone(), None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,)>)
        }
    }, self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(1), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), false), self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(2), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), true)])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]))))) == (self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))), "assertion 11");
    assert!(((self::read_java_script_error_prior_directory_error({
        let source_ml_source2_constructor_field1171 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
            let source_ml_source2_constructor_field1161 = self::ml::Big::from_i128(1);
            {
                let source_ml_source2_constructor_field1162 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1163 = true;
                    {
                        let source_ml_source2_constructor_field1164 = true;
                        {
                            let source_ml_source2_constructor_field1165 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1161.clone(), Box::new(source_ml_source2_constructor_field1162.clone()), source_ml_source2_constructor_field1163, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1164, source_ml_source2_constructor_field1165.clone())
                        }
                    }
                }
            }
        }, {
            let source_ml_source2_constructor_field1166 = self::ml::Big::from_i128(2);
            {
                let source_ml_source2_constructor_field1167 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1168 = true;
                    {
                        let source_ml_source2_constructor_field1169 = true;
                        {
                            let source_ml_source2_constructor_field1170 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1166.clone(), Box::new(source_ml_source2_constructor_field1167.clone()), source_ml_source2_constructor_field1168, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1169, source_ml_source2_constructor_field1170.clone())
                        }
                    }
                }
            }
        }, self::JavaScriptOrdinaryObject::HostErrorObject(self::ml::Big::from_i128(7), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false)), true, false, std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new())), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)))]));
        {
            let source_ml_source2_constructor_field1172 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]));
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_constructor_field1171.clone(), source_ml_source2_constructor_field1172.clone(), None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,)>)
        }
    }, self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(1), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), false), self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(2), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), true)])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]))))) == (self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))), "assertion 12");
    assert!(((self::read_java_script_error_prior_directory_error({
        let source_ml_source2_constructor_field1183 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
            let source_ml_source2_constructor_field1173 = self::ml::Big::from_i128(1);
            {
                let source_ml_source2_constructor_field1174 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1175 = true;
                    {
                        let source_ml_source2_constructor_field1176 = false;
                        {
                            let source_ml_source2_constructor_field1177 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1173.clone(), Box::new(source_ml_source2_constructor_field1174.clone()), source_ml_source2_constructor_field1175, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1176, source_ml_source2_constructor_field1177.clone())
                        }
                    }
                }
            }
        }, {
            let source_ml_source2_constructor_field1178 = self::ml::Big::from_i128(2);
            {
                let source_ml_source2_constructor_field1179 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1180 = true;
                    {
                        let source_ml_source2_constructor_field1181 = true;
                        {
                            let source_ml_source2_constructor_field1182 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1178.clone(), Box::new(source_ml_source2_constructor_field1179.clone()), source_ml_source2_constructor_field1180, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1181, source_ml_source2_constructor_field1182.clone())
                        }
                    }
                }
            }
        }, self::JavaScriptOrdinaryObject::HostErrorObject(self::ml::Big::from_i128(7), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false)), true, false, std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new())), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)))]));
        {
            let source_ml_source2_constructor_field1184 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()));
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_constructor_field1183.clone(), source_ml_source2_constructor_field1184.clone(), None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,)>)
        }
    }, self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(1), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), false), self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(2), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), true)])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]))))) == (self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))), "assertion 13");
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
fn all_literal_error_construction_controls() { main(); }
