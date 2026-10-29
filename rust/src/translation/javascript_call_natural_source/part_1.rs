// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1;

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

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSemanticValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    Integer(self::ml::Big),
    Text(Vec<f64>),
    Symbol(self::ml::Big),
    Object(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallInstruction {
    Constant(f64, Box<self::JavaScriptSemanticValue>),
    Copy(f64, f64),
    Argument(f64, f64),
    Jump(f64),
    Branch(f64, f64, f64),
    Call(f64, f64, f64, Vec<f64>),
    Return(f64),
    Throw(f64),
    PushHandler(f64, f64),
    PopHandler,
    NumberBinary(f64, f64, f64, self::ml_string::Text),
    SameValue(f64, f64, f64),
    StrictEqual(f64, f64, f64),
    RequireWrite(f64),
    Primitive(f64, self::ml_string::Text, Vec<f64>),
    OrdinaryGet(f64, f64, f64, f64),
    OrdinarySet(f64, f64, f64, f64, f64),
    PropertyKey(f64, f64, self::ml::Big),
    PropertyDefinition(f64, f64, f64, f64, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallFunction {
    CallFunction(self::ml::Big, self::ml::Big, self::ml_string::Text, Box<self::JavaScriptSemanticValue>, f64, f64, Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptCallInstruction>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallProgram {
    CallProgram(Vec<self::JavaScriptCallFunction>, Vec<self::ml::Big>, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallPrimitiveResult {
    PrimitiveValue(Box<self::JavaScriptSemanticValue>),
    PrimitiveInvalid(self::ml_string::Text),
    PrimitiveError(self::ml_string::Text, self::ml_string::Text),
    PrimitiveResourceLimit(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSource {
    SourceConstant(Box<self::JavaScriptSemanticValue>),
    SourceReceiver,
    SourceArgument(f64),
    SourceThrow(Box<self::JavaScriptCallSource>),
    SourceTry(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceBinding(f64),
    SourceLet(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceIf(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceNumberBinary(self::ml_string::Text, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceSameValue(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceStrictEqual(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourcePrimitive(self::ml_string::Text, Vec<self::JavaScriptCallSource>),
    SourceRecur(Vec<self::JavaScriptCallSource>),
    SourceSet(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceGet(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceCall(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Vec<self::JavaScriptCallSource>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceBinding {
    SourceBindingRegister(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceBuilder {
    SourceBuilder(Vec<self::JavaScriptCallInstruction>, f64, f64, f64, f64, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceResult {
    SourceResult(Box<self::JavaScriptCallSourceBuilder>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceCompilation {
    SourceCompiled(Vec<self::JavaScriptCallInstruction>, f64, self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_value_error(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Text(units) => {
            if (((units.len() as f64)) > (1048576f64)) {
                self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s5 = self::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(&(units), at);
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
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if ((identity > (self::ml::Big::from_i128(0))) && (identity <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, _) => {
            if ((identity_2 > (self::ml::Big::from_i128(0))) && (identity_2 <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
        _ => {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_code_coordinate(value: f64, limit: f64) -> bool {
    (((self::ml_math::is_safe_integer(value) && (value >= (0f64))) && (value < limit)) && ((value != (0f64)) || (((1f64 / value)) > (0f64))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_constant_error(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    self::read_java_script_semantic_value_error(value.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_arity(operation: &self::ml_string::Text) -> f64 {
    if ((((*operation) == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 105, 110, 116, 101, 103, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115])))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
        1f64
    } else {
        if self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116]), self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100])], operation, 0f64, false, |a, b| a == b) {
            2f64
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_exception_operation(name: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*name) == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111, 114])
    } else {
        if ((*name) == (self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]))) {
            self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if ((*name) == (self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) {
                self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
            } else {
                if ((*name) == (self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114]))) {
                    self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
                } else {
                    if ((*name) == (self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]))) {
                        self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])
                    } else {
                        if ((*name) == (self::ml_string::Text::from_units(vec![85, 82, 73, 69, 114, 114, 111, 114]))) {
                            self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 85, 82, 73, 69, 114, 114, 111, 114])
                        } else {
                            if ((*name) == (self::ml_string::Text::from_units(vec![69, 118, 97, 108, 69, 114, 114, 111, 114]))) {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_exception_class(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114])
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114]))) {
            self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) {
                self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
            } else {
                if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114]))) {
                    self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
                } else {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]))) {
                        self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 85, 82, 73, 69, 114, 114, 111, 114]))) {
                            self::ml_string::Text::from_units(vec![85, 82, 73, 69, 114, 114, 111, 114])
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 118, 97, 108, 69, 114, 114, 111, 114]))) {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_unary_number_operation(operation: &self::ml_string::Text) -> bool {
    self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 101, 105, 108, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 98, 101, 114, 32, 115, 105, 103, 110]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 113, 117, 97, 114, 101, 32, 114, 111, 111, 116]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114])], operation, 0f64, false, |a, b| a == b)
}
