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

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryProperty {
    OrdinaryProperty(Box<self::JavaScriptSemanticValue>, (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)),
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
    ClosureObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, self::ml::Big, f64, Vec<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Option<(Option<self::JavaScriptSemanticValue>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptBoundFunctionObject {
    BoundFunctionObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayObject {
    ArrayObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryObject {
    ClosureObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, self::ml::Big, f64, Vec<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Option<(Option<self::JavaScriptSemanticValue>,)>),
    BoundFunctionObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
    ArrayObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>),
    CollectionIteratorObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, self::ml::Big, f64, self::ml_string::Text, self::ml_string::Text, Option<(Option<bool>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>),
    MapObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, Vec<self::JavaScriptCollectionEntry>),
    SetObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, Vec<self::JavaScriptCollectionEntry>),
    WeakMapObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, Vec<self::JavaScriptWeakCollectionEntry>),
    WeakSetObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, Vec<self::JavaScriptWeakCollectionEntry>),
    OrdinaryObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, Option<(Option<bool>,)>, Option<(Option<bool>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, bool, Vec<self::JavaScriptOrdinaryProperty>),
    HostErrorObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>),
    HostURLObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>),
    HostArrayObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>),
    HostByteBufferObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>),
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
    OrdinaryHeap(Vec<self::JavaScriptOrdinaryObject>, Vec<self::ml::Big>, Option<(Option<self::ml_string::Text>,)>, Option<(Option<Vec<self::JavaScriptSymbolRegistrationFact>>,)>, Option<(Option<self::ml_string::Text>,)>, Option<(Option<Vec<self::JavaScriptSymbolRegistryEntry>>,)>),
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
            if (((units.len() as f64)) > (1048576f64)) {
                self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s5 = self::source_ml_temporary_read_java_script_semantic_value_error_loop1(units, at);
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
                let units = source_ml_source1_f71;
                (units.len() as f64)
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
                    let units = source_ml_source1_f143;
                    (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64))
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f144, text) => {
                {
                    let source_ml_source1_string_unit_operand196 = text.clone();
                    {
                        let source_ml_source1_string_unit_operand197 = index;
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
                                    let source_ml_source1_string_unit_operand203 = {
                                        let source_ml_source1_string_range_operand198 = text_2.clone();
                                        {
                                            let source_ml_source1_string_range_operand199 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand200 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source1_string_range_position201 = source_ml_source1_string_range_operand199;
                                                    {
                                                        let source_ml_source1_string_range_position202 = source_ml_source1_string_range_operand200;
                                                        source_ml_source1_string_range_operand198.slice(source_ml_source1_string_range_position201, source_ml_source1_string_range_position202)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand204 = 0f64;
                                        source_ml_source1_string_unit_operand203.code_point_at(source_ml_source1_string_unit_operand204)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source1_string_unit_operand196.clone(), source_ml_source1_string_unit_operand197)
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f145) => {
                {
                    let source_ml_source1_f210 = source_ml_source1_f145.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f146, source_ml_source1_f147) => {
                {
                    let source_ml_source1_f219 = source_ml_source1_f146.clone();
                    {
                        let source_ml_source1_f220 = source_ml_source1_f147;
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f221) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f222) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f223) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f224) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f225, source_ml_source1_f226) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f227) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f228, source_ml_source1_f229) => {
                    false
                }
            } && match right.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f230) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f231) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f232) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f233) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f234, source_ml_source1_f235) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f236) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f237, source_ml_source1_f238) => {
                    false
                }
            }) {
                {
                    let source_ml_source1_comparison_operand241 = match left.clone() {
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
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value239) => {
                            Some(source_ml_source1_data_property_value239.clone())
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    {
                        let source_ml_source1_comparison_operand242 = match right.clone() {
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
                            self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value240) => {
                                Some(source_ml_source1_data_property_value240.clone())
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<self::ml_string::Text>
                            }
                        };
                        match (source_ml_source1_comparison_operand241.clone(), source_ml_source1_comparison_operand242.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false }
                    }
                }
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s243 = self::source_ml_temporary_test_java_script_semantic_text_equal_loop2(left.clone(), right.clone(), length, at);
                        match source_ml_source1_s243.clone() {
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f244) => {
            {
                let source_ml_source1_f268 = source_ml_source1_f244;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f245) => {
            {
                let source_ml_source1_f277 = source_ml_source1_f245;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f246) => {
            {
                let source_ml_source1_f286 = source_ml_source1_f246.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f247) => {
            {
                let units = source_ml_source1_f247;
                {
                    let text = self::ml_string::Text::from_units(vec![]);
                    {
                        let source_ml_temporary_values3 = units;
                        {
                            let source_ml_temporary_index3 = self::ml::Big::from_i128(0);
                            {
                                let text_2 = self::source_ml_temporary_read_java_script_semantic_text_string_loop4(text.clone(), source_ml_temporary_values3, source_ml_temporary_index3.clone());
                                text_2.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f248, text_3) => {
            text_3.clone()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f249) => {
            {
                let source_ml_source1_f305 = source_ml_source1_f249.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f250, source_ml_source1_f251) => {
            {
                let source_ml_source1_f314 = source_ml_source1_f250.clone();
                {
                    let source_ml_source1_f315 = source_ml_source1_f251;
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
                                let units_2 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units, at);
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
                                let units_4 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_3, at_2);
                                units_4
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f316) => {
                    {
                        let units_5 = Vec::<f64>::new();
                        {
                            let at_3 = 0f64;
                            {
                                let units_6 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_5, at_3);
                                units_6
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f317) => {
                    {
                        let units_7 = Vec::<f64>::new();
                        {
                            let at_4 = 0f64;
                            {
                                let units_8 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_7, at_4);
                                units_8
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f318) => {
                    {
                        let units_9 = Vec::<f64>::new();
                        {
                            let at_5 = 0f64;
                            {
                                let units_10 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_9, at_5);
                                units_10
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(units_11) => {
                    units_11
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f319, source_ml_source1_f320) => {
                    {
                        let units_12 = Vec::<f64>::new();
                        {
                            let at_6 = 0f64;
                            {
                                let units_13 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_12, at_6);
                                units_13
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f321) => {
                    {
                        let units_14 = Vec::<f64>::new();
                        {
                            let at_7 = 0f64;
                            {
                                let units_15 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_14, at_7);
                                units_15
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f322, source_ml_source1_f323) => {
                    {
                        let units_16 = Vec::<f64>::new();
                        {
                            let at_8 = 0f64;
                            {
                                let units_17 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_16, at_8);
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f324) => {
                {
                    let source_ml_source1_f341 = source_ml_source1_f324;
                    false
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f325) => {
                {
                    let source_ml_source1_f350 = source_ml_source1_f325;
                    false
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f326) => {
                {
                    let source_ml_source1_f359 = source_ml_source1_f326.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f327) => {
                {
                    let source_ml_source1_f368 = source_ml_source1_f327;
                    false
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f328, source_ml_source1_f329) => {
                {
                    let source_ml_source1_f377 = source_ml_source1_f328.clone();
                    {
                        let source_ml_source1_f378 = source_ml_source1_f329.clone();
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f330) => {
                {
                    let source_ml_source1_f387 = source_ml_source1_f330.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f331, source_ml_source1_f332) => {
                {
                    let source_ml_source1_f396 = source_ml_source1_f331.clone();
                    {
                        let callable = source_ml_source1_f332;
                        callable
                    }
                }
            }
        }
    }
}

pub fn find_java_script_property_descriptor_entry(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source1_s397 = self::source_ml_temporary_find_java_script_property_descriptor_entry_loop6(properties, key.clone(), at);
            match source_ml_source1_s397.clone() {
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done => {
                    (-1f64)
                }
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(source_ml_temporary_result6) => {
                    source_ml_temporary_result6
                }
            }
        }
    }
}

pub fn read_java_script_property_descriptor_kind(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> self::ml_string::Text {
    {
        let data = ((descriptor.clone()).9 || (descriptor.clone()).11);
        {
            let accessor = ((descriptor.clone()).5 || (descriptor.clone()).7);
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

pub fn test_java_script_property_descriptors_same_value(left: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), right: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> bool {
    !(((((((((((((((left.clone()).1) != ((right.clone()).1)) || ((left.clone()).1 && (((left.clone()).0) != ((right.clone()).0)))) || (((left.clone()).3) != ((right.clone()).3))) || ((left.clone()).3 && (((left.clone()).2) != ((right.clone()).2)))) || (((left.clone()).11) != ((right.clone()).11))) || ((left.clone()).11 && (((left.clone()).10) != ((right.clone()).10)))) || (((left.clone()).9) != ((right.clone()).9))) || ((left.clone()).9 && !(self::test_java_script_semantic_same_value((left.clone()).8, (right.clone()).8)))) || (((left.clone()).5) != ((right.clone()).5))) || ((left.clone()).5 && !(self::test_java_script_semantic_same_value((left.clone()).4, (right.clone()).4)))) || (((left.clone()).7) != ((right.clone()).7))) || ((left.clone()).7 && !(self::test_java_script_semantic_same_value((left.clone()).6, (right.clone()).6)))))
}

pub fn read_java_script_property_descriptor_error(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> self::ml_string::Text {
    if ((self::read_java_script_property_descriptor_kind(descriptor.clone())) == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))) {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        if ((descriptor.clone()).9 && ((self::read_java_script_semantic_value_error((descriptor.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
            self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108, 117, 101])
        } else {
            if ((descriptor.clone()).5 && (((self::read_java_script_semantic_value_error((descriptor.clone()).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).4)))) {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                if ((descriptor.clone()).7 && (((self::read_java_script_semantic_value_error((descriptor.clone()).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).6)))) {
                    self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn test_complete_java_script_property_descriptor(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> bool {
    if ((!((descriptor.clone()).1) || !((descriptor.clone()).3)) || ((self::read_java_script_property_descriptor_error(descriptor.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if (kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97]))) {
                ((descriptor.clone()).9 && (descriptor.clone()).11)
            } else {
                (((kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) && (descriptor.clone()).5) && (descriptor.clone()).7)
            }
        }
    }
}

pub fn create_java_script_property_descriptor(kind: self::ml_string::Text, configurable: bool, enumerable: bool) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source1_field398 = true;
        {
            let source_ml_source1_field399 = configurable;
            {
                let source_ml_source1_field400 = true;
                {
                    let source_ml_source1_field401 = enumerable;
                    {
                        let source_ml_source1_field402 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                        {
                            let source_ml_source1_field403 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field404 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                {
                                    let source_ml_source1_field405 = false;
                                    {
                                        let source_ml_source1_field406 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                        {
                                            let source_ml_source1_field407 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field408 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                                {
                                                    let source_ml_source1_field409 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field399, source_ml_source1_field398, source_ml_source1_field401, source_ml_source1_field400, source_ml_source1_field407.clone(), source_ml_source1_field406, source_ml_source1_field409.clone(), source_ml_source1_field408, source_ml_source1_field403.clone(), source_ml_source1_field402, source_ml_source1_field405, source_ml_source1_field404)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn apply_java_script_property_descriptor_fields(base: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source1_field410 = true;
        {
            let source_ml_source1_field411 = if (proposed.clone()).1 {
                (proposed.clone()).0
            } else {
                (base.clone()).0
            };
            {
                let source_ml_source1_field412 = true;
                {
                    let source_ml_source1_field413 = if (proposed.clone()).3 {
                        (proposed.clone()).2
                    } else {
                        (base.clone()).2
                    };
                    {
                        let source_ml_source1_field414 = (base.clone()).9;
                        {
                            let source_ml_source1_field415 = if ((base.clone()).9 && (proposed.clone()).9) {
                                (proposed.clone()).8
                            } else {
                                (base.clone()).8
                            };
                            {
                                let source_ml_source1_field416 = (base.clone()).11;
                                {
                                    let source_ml_source1_field417 = if ((base.clone()).11 && (proposed.clone()).11) {
                                        (proposed.clone()).10
                                    } else {
                                        (base.clone()).10
                                    };
                                    {
                                        let source_ml_source1_field418 = (base.clone()).5;
                                        {
                                            let source_ml_source1_field419 = if ((base.clone()).5 && (proposed.clone()).5) {
                                                (proposed.clone()).4
                                            } else {
                                                (base.clone()).4
                                            };
                                            {
                                                let source_ml_source1_field420 = (base.clone()).7;
                                                {
                                                    let source_ml_source1_field421 = if ((base.clone()).7 && (proposed.clone()).7) {
                                                        (proposed.clone()).6
                                                    } else {
                                                        (base.clone()).6
                                                    };
                                                    (source_ml_source1_field411, source_ml_source1_field410, source_ml_source1_field413, source_ml_source1_field412, source_ml_source1_field419.clone(), source_ml_source1_field418, source_ml_source1_field421.clone(), source_ml_source1_field420, source_ml_source1_field415.clone(), source_ml_source1_field414, source_ml_source1_field417, source_ml_source1_field416)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_property_descriptor_writes(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> Vec<self::ml_string::Text> {
    {
        let writes = Vec::<self::ml_string::Text>::new();
        {
            let writes_3 = if (descriptor.clone()).1 {
                {
                    let writes_2 = {
                        let source_ml_source1_concat_value422 = writes;
                        {
                            let source_ml_source1_concat_value423 = vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value422), source_ml_source1_concat_value423)
                        }
                    };
                    writes_2
                }
            } else {
                writes
            };
            {
                let writes_5 = if (descriptor.clone()).3 {
                    {
                        let writes_4 = {
                            let source_ml_source1_concat_value424 = writes_3;
                            {
                                let source_ml_source1_concat_value425 = vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])];
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value424), source_ml_source1_concat_value425)
                            }
                        };
                        writes_4
                    }
                } else {
                    writes_3
                };
                {
                    let writes_7 = if (descriptor.clone()).9 {
                        {
                            let writes_6 = {
                                let source_ml_source1_concat_value426 = writes_5;
                                {
                                    let source_ml_source1_concat_value427 = vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])];
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value426), source_ml_source1_concat_value427)
                                }
                            };
                            writes_6
                        }
                    } else {
                        writes_5
                    };
                    {
                        let writes_9 = if (descriptor.clone()).11 {
                            {
                                let writes_8 = {
                                    let source_ml_source1_concat_value428 = writes_7;
                                    {
                                        let source_ml_source1_concat_value429 = vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value428), source_ml_source1_concat_value429)
                                    }
                                };
                                writes_8
                            }
                        } else {
                            writes_7
                        };
                        {
                            let writes_11 = if (descriptor.clone()).5 {
                                {
                                    let writes_10 = {
                                        let source_ml_source1_concat_value430 = writes_9;
                                        {
                                            let source_ml_source1_concat_value431 = vec![self::ml_string::Text::from_units(vec![103, 101, 116])];
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value430), source_ml_source1_concat_value431)
                                        }
                                    };
                                    writes_10
                                }
                            } else {
                                writes_9
                            };
                            {
                                let writes_13 = if (descriptor.clone()).7 {
                                    {
                                        let writes_12 = {
                                            let source_ml_source1_concat_value432 = writes_11;
                                            {
                                                let source_ml_source1_concat_value433 = vec![self::ml_string::Text::from_units(vec![115, 101, 116])];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value432), source_ml_source1_concat_value433)
                                            }
                                        };
                                        writes_12
                                    }
                                } else {
                                    writes_11
                                };
                                writes_13
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn create_java_script_property_transition(disposition: self::ml_string::Text, error: self::ml_string::Text, descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), writes: Vec<self::ml_string::Text>) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>) {
    {
        let source_ml_source1_field434 = disposition.clone();
        {
            let source_ml_source1_field435 = error.clone();
            {
                let source_ml_source1_field436 = descriptor.clone();
                {
                    let source_ml_source1_field437 = writes;
                    (source_ml_source1_field436.clone(), source_ml_source1_field434.clone(), source_ml_source1_field435.clone(), source_ml_source1_field437)
                }
            }
        }
    }
}

pub fn plan_java_script_property_definition(exists: bool, extensible: bool, current: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>) {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), error.clone(), current.clone(), Vec::<self::ml_string::Text>::new())
        } else {
            if (exists && !(self::test_complete_java_script_property_descriptor(current.clone()))) {
                self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 67, 117, 114, 114, 101, 110, 116, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114]), current.clone(), Vec::<self::ml_string::Text>::new())
            } else {
                {
                    let kind = self::read_java_script_property_descriptor_kind(proposed.clone());
                    if !exists {
                        if !extensible {
                            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new())
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
                        if (((((!((proposed.clone()).1) && !((proposed.clone()).3)) && !((proposed.clone()).9)) && !((proposed.clone()).11)) && !((proposed.clone()).5)) && !((proposed.clone()).7)) {
                            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![107, 101, 101, 112]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new())
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(current.clone());
                                {
                                    let source_ml_source1_s438 = if !((current.clone()).0) {
                                        if ((proposed.clone()).1 && (proposed.clone()).0) {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else {
                                            if ((proposed.clone()).3 && (((proposed.clone()).2) != ((current.clone()).2))) {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if ((proposed.clone()).5 && !(self::test_java_script_semantic_same_value((proposed.clone()).4, (current.clone()).4))) {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                        } else {
                                                            if ((proposed.clone()).7 && !(self::test_java_script_semantic_same_value((proposed.clone()).6, (current.clone()).6))) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                            }
                                                        }
                                                    } else {
                                                        if !((current.clone()).10) {
                                                            if ((proposed.clone()).11 && (proposed.clone()).10) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                if (proposed.clone()).9 {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(if self::test_java_script_semantic_same_value((proposed.clone()).8, (current.clone()).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }, self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                                } else {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                    };
                                    match source_ml_source1_s438.clone() {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next => {
                                            {
                                                let base_2 = if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::create_java_script_property_descriptor(kind.clone(), (current.clone()).0, (current.clone()).2)
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
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(source_ml_temporary_result7) => {
                                            source_ml_temporary_result7.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn find_java_script_heap_directory_object(objects: Vec<self::JavaScriptOrdinaryObject>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = (objects.len() as f64);
            {
                let source_ml_source1_s439 = self::source_ml_temporary_find_java_script_heap_directory_object_loop8(objects, identity.clone(), low, high);
                match source_ml_source1_s439.clone() {
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Done => {
                        (-1f64)
                    }
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(source_ml_temporary_result8) => {
                        source_ml_temporary_result8
                    }
                }
            }
        }
    }
}

pub fn test_java_script_heap_directory_symbol(symbols: Vec<self::ml::Big>, identity: self::ml::Big) -> bool {
    {
        let low = 0f64;
        {
            let high = (symbols.len() as f64);
            {
                let source_ml_source1_s440 = self::source_ml_temporary_test_java_script_heap_directory_symbol_loop9(symbols, identity.clone(), low, high);
                match source_ml_source1_s440.clone() {
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(source_ml_temporary_result9) => {
                        source_ml_temporary_result9
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f441) => {
                    {
                        let source_ml_source1_f463 = source_ml_source1_f441;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f442) => {
                    {
                        let source_ml_source1_f471 = source_ml_source1_f442;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f443) => {
                    {
                        let source_ml_source1_f479 = source_ml_source1_f443.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f444) => {
                    {
                        let source_ml_source1_f487 = source_ml_source1_f444;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f445, source_ml_source1_f446) => {
                    {
                        let source_ml_source1_f495 = source_ml_source1_f445.clone();
                        {
                            let source_ml_source1_f496 = source_ml_source1_f446.clone();
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    if self::test_java_script_heap_directory_symbol(match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value498, _, _, _, _) => {
                            source_ml_source1_data_property_value498
                        }
                    }, identity.clone()) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f447, source_ml_source1_f448) => {
                    {
                        let identity_2 = source_ml_source1_f447.clone();
                        {
                            let callable = source_ml_source1_f448;
                            {
                                let at = self::find_java_script_heap_directory_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value506, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value506
                                    }
                                }, identity_2.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value507, _, _, _, _, _) => source_ml_source1_data_property_value507.as_slice() }), self::ml_array::number_index(at));
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source1_data_property_value521, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value521
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source1_data_property_value522, _, _, _, _) => {
                                                source_ml_source1_data_property_value522
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source1_data_property_value523, _) => {
                                                source_ml_source1_data_property_value523
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source1_data_property_value524, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value524
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source1_data_property_value525, _, _) => {
                                                source_ml_source1_data_property_value525
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source1_data_property_value526, _, _) => {
                                                source_ml_source1_data_property_value526
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source1_data_property_value527, _, _) => {
                                                source_ml_source1_data_property_value527
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source1_data_property_value528, _, _) => {
                                                source_ml_source1_data_property_value528
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source1_data_property_value529, _) => {
                                                source_ml_source1_data_property_value529
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source1_data_property_value530, _, _, _) => {
                                                source_ml_source1_data_property_value530
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source1_data_property_value531, _, _, _) => {
                                                source_ml_source1_data_property_value531
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source1_data_property_value532, _, _, _) => {
                                                source_ml_source1_data_property_value532
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source1_data_property_value533, _, _, _) => {
                                                source_ml_source1_data_property_value533
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f534) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f535) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f536) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f537) => {
            false
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f538, source_ml_source1_f539) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f540) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f541, source_ml_source1_f542) => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f543) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f544) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f545) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f546) => {
            true
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f547, source_ml_source1_f548) => {
            false
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f549) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f550, source_ml_source1_f551) => {
            true
        }
    }) && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f552) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f553) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f554) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f555) => {
            true
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f556, source_ml_source1_f557) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f558) => {
            false
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f559, source_ml_source1_f560) => {
            true
        }
    }) {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 101, 120, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 104, 111, 117, 116, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 99, 111, 101, 114, 99, 105, 111, 110])
    } else {
        self::read_java_script_heap_directory_value_error(heap.clone(), key.clone())
    }
}

pub fn find_java_script_heap_directory_property(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    self::find_java_script_property_descriptor_entry(properties, key.clone())
}

pub fn test_java_script_ordinary_class_constructor_mode(mode: self::ml_string::Text) -> bool {
    (mode == (self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115])))
}

pub fn test_java_script_ordinary_class_receiver_result(mode: self::ml_string::Text, result: self::ml_string::Text) -> bool {
    (self::test_java_script_ordinary_class_constructor_mode(mode.clone()) && (result != (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))))
}

pub fn read_java_script_ordinary_class_call_reason(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> self::ml_string::Text {
    {
        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value561, _, _, _, _, _) => source_ml_source1_data_property_value561.as_slice() }), self::ml_array::number_index(self::find_java_script_heap_directory_object(match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value562, _, _, _, _, _) => {
                source_ml_source1_data_property_value562
            }
        }, identity.clone())));
        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::ml_string::Text::from_units(vec![])
        } else {
            match object.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_f563, source_ml_source1_f564, source_ml_source1_f565, source_ml_source1_f566, source_ml_source1_f567, source_ml_source1_f568, source_ml_source1_f569, source_ml_source1_f570, source_ml_source1_f571, source_ml_source1_f572) => {
                    let source_ml_source1_f564 = *source_ml_source1_f564;
                    let source_ml_source1_f571 = *source_ml_source1_f571;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_f573, source_ml_source1_f574, source_ml_source1_f575, source_ml_source1_f576, source_ml_source1_f577, source_ml_source1_f578, source_ml_source1_f579, source_ml_source1_f580) => {
                    let source_ml_source1_f574 = *source_ml_source1_f574;
                    let source_ml_source1_f578 = *source_ml_source1_f578;
                    let source_ml_source1_f579 = *source_ml_source1_f579;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_f581, source_ml_source1_f582, source_ml_source1_f583, source_ml_source1_f584, source_ml_source1_f585) => {
                    let source_ml_source1_f582 = *source_ml_source1_f582;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_f586, source_ml_source1_f587, source_ml_source1_f588, source_ml_source1_f589, source_ml_source1_f590, source_ml_source1_f591, source_ml_source1_f592, source_ml_source1_f593, source_ml_source1_f594, source_ml_source1_f595, source_ml_source1_f596) => {
                    let source_ml_source1_f587 = *source_ml_source1_f587;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_f597, source_ml_source1_f598, source_ml_source1_f599, source_ml_source1_f600, source_ml_source1_f601, source_ml_source1_f602) => {
                    let source_ml_source1_f598 = *source_ml_source1_f598;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_f603, source_ml_source1_f604, source_ml_source1_f605, source_ml_source1_f606, source_ml_source1_f607, source_ml_source1_f608) => {
                    let source_ml_source1_f604 = *source_ml_source1_f604;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_f609, source_ml_source1_f610, source_ml_source1_f611, source_ml_source1_f612, source_ml_source1_f613, source_ml_source1_f614) => {
                    let source_ml_source1_f610 = *source_ml_source1_f610;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_f615, source_ml_source1_f616, source_ml_source1_f617, source_ml_source1_f618, source_ml_source1_f619, source_ml_source1_f620) => {
                    let source_ml_source1_f616 = *source_ml_source1_f616;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_f621, source_ml_source1_f622, source_ml_source1_f623, source_ml_source1_f624, source_ml_source1_f625, source_ml_source1_f626, source_ml_source1_f627, source_ml_source1_f628, properties) => {
                    let source_ml_source1_f622 = *source_ml_source1_f622;
                    {
                        let field = self::ml_array::optional_at(&properties, self::ml_array::number_index(self::find_java_script_heap_directory_property(properties, self::JavaScriptSemanticValue::Text(vec![110f64, 97f64, 109f64, 101f64]))));
                        if (((match (field.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !((match field.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value657) => {
                                source_ml_source1_data_property_value657.clone()
                            }
                        }).9)) || (match field.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value658) => {
                                source_ml_source1_data_property_value658.clone()
                            }
                        }).5) || (match field.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value659) => {
                                source_ml_source1_data_property_value659.clone()
                            }
                        }).7) {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            {
                                let value = (match field.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value660) => {
                                        source_ml_source1_data_property_value660.clone()
                                    }
                                }).8;
                                if (match value.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f661) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Number(source_ml_source1_f662) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f663) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source1_f664) => {
                                        false
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f665, source_ml_source1_f666) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f667) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source1_f668, source_ml_source1_f669) => {
                                        true
                                    }
                                } && match value.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f670) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Number(source_ml_source1_f671) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f672) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source1_f673) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f674, source_ml_source1_f675) => {
                                        false
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f676) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source1_f677, source_ml_source1_f678) => {
                                        true
                                    }
                                }) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    ((self::ml_string::Text::from_units(vec![67, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32])).concat(&self::read_java_script_semantic_text_string(value.clone()))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 105, 110, 118, 111, 107, 101, 100, 32, 119, 105, 116, 104, 111, 117, 116, 32, 39, 110, 101, 119, 39]))
                                }
                            }
                        }
                    }
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_f629, source_ml_source1_f630, source_ml_source1_f631, source_ml_source1_f632, source_ml_source1_f633, source_ml_source1_f634, source_ml_source1_f635) => {
                    let source_ml_source1_f630 = *source_ml_source1_f630;
                    let source_ml_source1_f634 = *source_ml_source1_f634;
                    let source_ml_source1_f635 = *source_ml_source1_f635;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_f636, source_ml_source1_f637, source_ml_source1_f638, source_ml_source1_f639, source_ml_source1_f640, source_ml_source1_f641, source_ml_source1_f642) => {
                    let source_ml_source1_f637 = *source_ml_source1_f637;
                    let source_ml_source1_f641 = *source_ml_source1_f641;
                    let source_ml_source1_f642 = *source_ml_source1_f642;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_f643, source_ml_source1_f644, source_ml_source1_f645, source_ml_source1_f646, source_ml_source1_f647, source_ml_source1_f648, source_ml_source1_f649) => {
                    let source_ml_source1_f644 = *source_ml_source1_f644;
                    let source_ml_source1_f648 = *source_ml_source1_f648;
                    let source_ml_source1_f649 = *source_ml_source1_f649;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_f650, source_ml_source1_f651, source_ml_source1_f652, source_ml_source1_f653, source_ml_source1_f654, source_ml_source1_f655, source_ml_source1_f656) => {
                    let source_ml_source1_f651 = *source_ml_source1_f651;
                    let source_ml_source1_f655 = *source_ml_source1_f655;
                    let source_ml_source1_f656 = *source_ml_source1_f656;
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn read_java_script_ordinary_class_receiver_error(heap: self::JavaScriptOrdinaryHeap, callee: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    {
        let invalid_receiver = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 110, 100, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114]);
        match callee.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Null => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f679) => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f680) => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f681) => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f682) => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f683, source_ml_source1_f684) => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f685) => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Object(callee_identity, callee_callable) => {
                if !callee_callable {
                    invalid_receiver.clone()
                } else {
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Null => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f686) => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f687) => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f688) => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f689) => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f690, source_ml_source1_f691) => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f692) => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Object(identity, callable) => {
                            if callable {
                                invalid_receiver.clone()
                            } else {
                                {
                                    let constructor = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value693, _, _, _, _, _) => source_ml_source1_data_property_value693.as_slice() }), self::ml_array::number_index(self::find_java_script_heap_directory_object(match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value694, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value694
                                        }
                                    }, callee_identity.clone())));
                                    {
                                        let instance = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value695, _, _, _, _, _) => source_ml_source1_data_property_value695.as_slice() }), self::ml_array::number_index(self::find_java_script_heap_directory_object(match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value696, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value696
                                            }
                                        }, identity.clone())));
                                        {
                                            let invalid_kind = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 111, 112, 97, 113, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 107, 105, 110, 100, 115]);
                                            if match (constructor.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                invalid_kind.clone()
                                            } else {
                                                match constructor.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_f697, source_ml_source1_f698, source_ml_source1_f699, source_ml_source1_f700, source_ml_source1_f701, source_ml_source1_f702, source_ml_source1_f703, source_ml_source1_f704, source_ml_source1_f705, source_ml_source1_f706) => {
                                                        let source_ml_source1_f698 = *source_ml_source1_f698;
                                                        let source_ml_source1_f705 = *source_ml_source1_f705;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_f707, source_ml_source1_f708, source_ml_source1_f709, source_ml_source1_f710, source_ml_source1_f711, source_ml_source1_f712, source_ml_source1_f713, source_ml_source1_f714) => {
                                                        let source_ml_source1_f708 = *source_ml_source1_f708;
                                                        let source_ml_source1_f712 = *source_ml_source1_f712;
                                                        let source_ml_source1_f713 = *source_ml_source1_f713;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_f715, source_ml_source1_f716, source_ml_source1_f717, source_ml_source1_f718, source_ml_source1_f719) => {
                                                        let source_ml_source1_f716 = *source_ml_source1_f716;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_f720, source_ml_source1_f721, source_ml_source1_f722, source_ml_source1_f723, source_ml_source1_f724, source_ml_source1_f725, source_ml_source1_f726, source_ml_source1_f727, source_ml_source1_f728, source_ml_source1_f729, source_ml_source1_f730) => {
                                                        let source_ml_source1_f721 = *source_ml_source1_f721;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_f731, source_ml_source1_f732, source_ml_source1_f733, source_ml_source1_f734, source_ml_source1_f735, source_ml_source1_f736) => {
                                                        let source_ml_source1_f732 = *source_ml_source1_f732;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_f737, source_ml_source1_f738, source_ml_source1_f739, source_ml_source1_f740, source_ml_source1_f741, source_ml_source1_f742) => {
                                                        let source_ml_source1_f738 = *source_ml_source1_f738;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_f743, source_ml_source1_f744, source_ml_source1_f745, source_ml_source1_f746, source_ml_source1_f747, source_ml_source1_f748) => {
                                                        let source_ml_source1_f744 = *source_ml_source1_f744;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_f749, source_ml_source1_f750, source_ml_source1_f751, source_ml_source1_f752, source_ml_source1_f753, source_ml_source1_f754) => {
                                                        let source_ml_source1_f750 = *source_ml_source1_f750;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_f755, source_ml_source1_f756, source_ml_source1_f757, source_ml_source1_f758, source_ml_source1_f759, source_ml_source1_f760, source_ml_source1_f761, source_ml_source1_f762, properties) => {
                                                        let source_ml_source1_f756 = *source_ml_source1_f756;
                                                        if match (instance.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            invalid_kind.clone()
                                                        } else {
                                                            match instance.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_f791, source_ml_source1_f792, source_ml_source1_f793, source_ml_source1_f794, source_ml_source1_f795, source_ml_source1_f796, source_ml_source1_f797, source_ml_source1_f798, source_ml_source1_f799, source_ml_source1_f800) => {
                                                                    let source_ml_source1_f792 = *source_ml_source1_f792;
                                                                    let source_ml_source1_f799 = *source_ml_source1_f799;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_f801, source_ml_source1_f802, source_ml_source1_f803, source_ml_source1_f804, source_ml_source1_f805, source_ml_source1_f806, source_ml_source1_f807, source_ml_source1_f808) => {
                                                                    let source_ml_source1_f802 = *source_ml_source1_f802;
                                                                    let source_ml_source1_f806 = *source_ml_source1_f806;
                                                                    let source_ml_source1_f807 = *source_ml_source1_f807;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_f809, source_ml_source1_f810, source_ml_source1_f811, source_ml_source1_f812, source_ml_source1_f813) => {
                                                                    let source_ml_source1_f810 = *source_ml_source1_f810;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_f814, source_ml_source1_f815, source_ml_source1_f816, source_ml_source1_f817, source_ml_source1_f818, source_ml_source1_f819, source_ml_source1_f820, source_ml_source1_f821, source_ml_source1_f822, source_ml_source1_f823, source_ml_source1_f824) => {
                                                                    let source_ml_source1_f815 = *source_ml_source1_f815;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_f825, source_ml_source1_f826, source_ml_source1_f827, source_ml_source1_f828, source_ml_source1_f829, source_ml_source1_f830) => {
                                                                    let source_ml_source1_f826 = *source_ml_source1_f826;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_f831, source_ml_source1_f832, source_ml_source1_f833, source_ml_source1_f834, source_ml_source1_f835, source_ml_source1_f836) => {
                                                                    let source_ml_source1_f832 = *source_ml_source1_f832;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_f837, source_ml_source1_f838, source_ml_source1_f839, source_ml_source1_f840, source_ml_source1_f841, source_ml_source1_f842) => {
                                                                    let source_ml_source1_f838 = *source_ml_source1_f838;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_f843, source_ml_source1_f844, source_ml_source1_f845, source_ml_source1_f846, source_ml_source1_f847, source_ml_source1_f848) => {
                                                                    let source_ml_source1_f844 = *source_ml_source1_f844;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_f849, prototype, source_ml_source1_f850, source_ml_source1_f851, source_ml_source1_f852, source_ml_source1_f853, source_ml_source1_f854, source_ml_source1_f855, source_ml_source1_f856) => {
                                                                    let prototype = *prototype;
                                                                    {
                                                                        let field = self::ml_array::optional_at(&properties, self::ml_array::number_index(self::find_java_script_heap_directory_property(properties, self::JavaScriptSemanticValue::Text(vec![112f64, 114f64, 111f64, 116f64, 111f64, 116f64, 121f64, 112f64, 101f64]))));
                                                                        if (((((!(match (field.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (match field.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value885) => {
                                                                                source_ml_source1_data_property_value885.clone()
                                                                            }
                                                                        }).9) && !((match field.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value886) => {
                                                                                source_ml_source1_data_property_value886.clone()
                                                                            }
                                                                        }).5)) && !((match field.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value887) => {
                                                                                source_ml_source1_data_property_value887.clone()
                                                                            }
                                                                        }).7)) && {
                                                                            let source_ml_source1_s889 = (match field.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value899) => {
                                                                                    source_ml_source1_data_property_value899.clone()
                                                                                }
                                                                            }).8;
                                                                            match source_ml_source1_s889.clone() {
                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f890) => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f891) => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f892) => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f893) => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f894, source_ml_source1_f895) => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f896) => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f897, source_ml_source1_f898) => {
                                                                                    true
                                                                                }
                                                                            }
                                                                        }) && self::test_java_script_semantic_same_value((match field.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value900) => {
                                                                                source_ml_source1_data_property_value900.clone()
                                                                            }
                                                                        }).8, prototype.clone())) {
                                                                            self::ml_string::Text::from_units(vec![])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 99, 108, 97, 115, 115, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 119, 110, 32, 100, 97, 116, 97, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                                        }
                                                                    }
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_f857, source_ml_source1_f858, source_ml_source1_f859, source_ml_source1_f860, source_ml_source1_f861, source_ml_source1_f862, source_ml_source1_f863) => {
                                                                    let source_ml_source1_f858 = *source_ml_source1_f858;
                                                                    let source_ml_source1_f862 = *source_ml_source1_f862;
                                                                    let source_ml_source1_f863 = *source_ml_source1_f863;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_f864, source_ml_source1_f865, source_ml_source1_f866, source_ml_source1_f867, source_ml_source1_f868, source_ml_source1_f869, source_ml_source1_f870) => {
                                                                    let source_ml_source1_f865 = *source_ml_source1_f865;
                                                                    let source_ml_source1_f869 = *source_ml_source1_f869;
                                                                    let source_ml_source1_f870 = *source_ml_source1_f870;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_f871, source_ml_source1_f872, source_ml_source1_f873, source_ml_source1_f874, source_ml_source1_f875, source_ml_source1_f876, source_ml_source1_f877) => {
                                                                    let source_ml_source1_f872 = *source_ml_source1_f872;
                                                                    let source_ml_source1_f876 = *source_ml_source1_f876;
                                                                    let source_ml_source1_f877 = *source_ml_source1_f877;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_f878, source_ml_source1_f879, source_ml_source1_f880, source_ml_source1_f881, source_ml_source1_f882, source_ml_source1_f883, source_ml_source1_f884) => {
                                                                    let source_ml_source1_f879 = *source_ml_source1_f879;
                                                                    let source_ml_source1_f883 = *source_ml_source1_f883;
                                                                    let source_ml_source1_f884 = *source_ml_source1_f884;
                                                                    invalid_kind.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_f763, source_ml_source1_f764, source_ml_source1_f765, source_ml_source1_f766, source_ml_source1_f767, source_ml_source1_f768, source_ml_source1_f769) => {
                                                        let source_ml_source1_f764 = *source_ml_source1_f764;
                                                        let source_ml_source1_f768 = *source_ml_source1_f768;
                                                        let source_ml_source1_f769 = *source_ml_source1_f769;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_f770, source_ml_source1_f771, source_ml_source1_f772, source_ml_source1_f773, source_ml_source1_f774, source_ml_source1_f775, source_ml_source1_f776) => {
                                                        let source_ml_source1_f771 = *source_ml_source1_f771;
                                                        let source_ml_source1_f775 = *source_ml_source1_f775;
                                                        let source_ml_source1_f776 = *source_ml_source1_f776;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_f777, source_ml_source1_f778, source_ml_source1_f779, source_ml_source1_f780, source_ml_source1_f781, source_ml_source1_f782, source_ml_source1_f783) => {
                                                        let source_ml_source1_f778 = *source_ml_source1_f778;
                                                        let source_ml_source1_f782 = *source_ml_source1_f782;
                                                        let source_ml_source1_f783 = *source_ml_source1_f783;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_f784, source_ml_source1_f785, source_ml_source1_f786, source_ml_source1_f787, source_ml_source1_f788, source_ml_source1_f789, source_ml_source1_f790) => {
                                                        let source_ml_source1_f785 = *source_ml_source1_f785;
                                                        let source_ml_source1_f789 = *source_ml_source1_f789;
                                                        let source_ml_source1_f790 = *source_ml_source1_f790;
                                                        invalid_kind.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
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
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(mut units: Vec<f64>, mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f901) => {
                        {
                            let source_ml_source1_f925 = source_ml_source1_f901;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f902) => {
                        {
                            let source_ml_source1_f934 = source_ml_source1_f902;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f903) => {
                        {
                            let source_ml_source1_f943 = source_ml_source1_f903.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Text(units) => {
                        (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f904, source_ml_source1_f905) => {
                        {
                            let source_ml_source1_f953 = source_ml_source1_f904.clone();
                            {
                                let text = source_ml_source1_f905.clone();
                                {
                                    let source_ml_source1_string_unit_operand957 = text.clone();
                                    {
                                        let source_ml_source1_string_unit_operand958 = at;
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
                                                    let source_ml_source1_string_unit_operand964 = {
                                                        let source_ml_source1_string_range_operand959 = text_2.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand960 = index;
                                                            {
                                                                let source_ml_source1_string_range_operand961 = (index + 1f64);
                                                                {
                                                                    let source_ml_source1_string_range_position962 = source_ml_source1_string_range_operand960;
                                                                    {
                                                                        let source_ml_source1_string_range_position963 = source_ml_source1_string_range_operand961;
                                                                        source_ml_source1_string_range_operand959.slice(source_ml_source1_string_range_position962, source_ml_source1_string_range_position963)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_string_unit_operand965 = 0f64;
                                                        source_ml_source1_string_unit_operand964.code_point_at(source_ml_source1_string_unit_operand965)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source1_string_unit_operand957.clone(), source_ml_source1_string_unit_operand958)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f906) => {
                        {
                            let source_ml_source1_f971 = source_ml_source1_f906.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f907, source_ml_source1_f908) => {
                        {
                            let source_ml_source1_f980 = source_ml_source1_f907.clone();
                            {
                                let source_ml_source1_f981 = source_ml_source1_f908;
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f982) => {
                            {
                                let source_ml_source1_f1006 = source_ml_source1_f982;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f983) => {
                            {
                                let source_ml_source1_f1015 = source_ml_source1_f983;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f984) => {
                            {
                                let source_ml_source1_f1024 = source_ml_source1_f984.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units_2) => {
                            (self::ml_array::optional_at(&units_2, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f985, source_ml_source1_f986) => {
                            {
                                let source_ml_source1_f1034 = source_ml_source1_f985.clone();
                                {
                                    let text_2 = source_ml_source1_f986.clone();
                                    {
                                        let source_ml_source1_string_unit_operand1038 = text_2.clone();
                                        {
                                            let source_ml_source1_string_unit_operand1039 = at;
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
                                                        let source_ml_source1_string_unit_operand1045 = {
                                                            let source_ml_source1_string_range_operand1040 = text_3.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand1041 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand1042 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source1_string_range_position1043 = source_ml_source1_string_range_operand1041;
                                                                        {
                                                                            let source_ml_source1_string_range_position1044 = source_ml_source1_string_range_operand1042;
                                                                            source_ml_source1_string_range_operand1040.slice(source_ml_source1_string_range_position1043, source_ml_source1_string_range_position1044)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_string_unit_operand1046 = 0f64;
                                                            source_ml_source1_string_unit_operand1045.code_point_at(source_ml_source1_string_unit_operand1046)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source1_string_unit_operand1038.clone(), source_ml_source1_string_unit_operand1039)
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f987) => {
                            {
                                let source_ml_source1_f1052 = source_ml_source1_f987.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f988, source_ml_source1_f989) => {
                            {
                                let source_ml_source1_f1061 = source_ml_source1_f988.clone();
                                {
                                    let source_ml_source1_f1062 = source_ml_source1_f989;
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

pub fn source_ml_temporary_read_java_script_semantic_text_string_loop4(mut text: self::ml_string::Text, mut source_ml_temporary_values3: Vec<f64>, mut source_ml_temporary_index3: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_temporary_index3 < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))) {
            {
                let unit = self::ml_array::at(&source_ml_temporary_values3, source_ml_temporary_index3.to_index());
                {
                    let text_2 = text.concat(&{
                        let source_ml_source1_character_argument1063 = unit;
                        self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source1_character_argument1063]).into_iter().map(|ml_unit| ml_unit as u16).collect())
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

pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop5(mut value: self::JavaScriptSemanticValue, mut length: f64, mut units: Vec<f64>, mut at: f64) -> Vec<f64> {
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done,
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(f64),
}

pub fn source_ml_temporary_find_java_script_property_descriptor_entry_loop6(mut properties: Vec<self::JavaScriptOrdinaryProperty>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    loop {
        return if (at < ((properties.len() as f64))) {
            {
                let property = self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value(match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1064, _) => {
                        let source_ml_source1_data_property_value1064 = *source_ml_source1_data_property_value1064;
                        source_ml_source1_data_property_value1064.clone()
                    }
                }, key.clone())) {
                    self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(at)
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
            self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin7 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>)),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Done,
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(f64),
}

pub fn source_ml_temporary_find_java_script_heap_directory_object_loop8(mut objects: Vec<self::JavaScriptOrdinaryObject>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value1078, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1078.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value1079, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1079.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value1080, _, _, _, _) => {
                                source_ml_source1_data_property_value1080.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value1081, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1081.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value1082, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1082.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value1083, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1083.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value1084, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1084.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value1085, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1085.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1086, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1086.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value1087, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1087.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value1088, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1088.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value1089, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1089.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value1090, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1090.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value1104, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1104.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value1105, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1105.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value1106, _, _, _, _) => {
                                    source_ml_source1_data_property_value1106.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value1107, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1107.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value1108, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1108.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value1109, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1109.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value1110, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1110.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value1111, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1111.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1112, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1112.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value1113, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1113.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value1114, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1114.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value1115, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1115.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value1116, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1116.clone()
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
            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result {
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Done,
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(bool),
}

pub fn source_ml_temporary_test_java_script_heap_directory_symbol_loop9(mut symbols: Vec<self::ml::Big>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source1_comparison_operand1117 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand1118 = None::<()>;
                            match (source_ml_source1_comparison_operand1117.clone(), source_ml_source1_comparison_operand1118.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(true)
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
            self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Done
        };
    }
}

fn ml_main() {
    assert!(((self::test_java_script_ordinary_class_constructor_mode(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]))) == (true)), "assertion 1");
    assert!(((self::test_java_script_ordinary_class_constructor_mode(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) == (false)), "assertion 2");
    assert!(((self::test_java_script_ordinary_class_constructor_mode(self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114]))) == (false)), "assertion 3");
    assert!(((self::test_java_script_ordinary_class_receiver_result(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))) == (true)), "assertion 4");
    assert!(((self::test_java_script_ordinary_class_receiver_result(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) == (true)), "assertion 5");
    assert!(((self::test_java_script_ordinary_class_receiver_result(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) == (false)), "assertion 6");
    assert!(((self::test_java_script_ordinary_class_receiver_result(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) == (false)), "assertion 7");
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
fn all_literal_ordinary_class_controls() { main(); }
