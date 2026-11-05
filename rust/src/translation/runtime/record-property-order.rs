pub mod ml_record_property_order {
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    ((((((((((((unit >= (9f64)) && (unit <= (13f64))) || (unit == (32f64))) || (unit == (160f64))) || (unit == (5760f64))) || ((unit >= (8192f64)) && (unit <= (8202f64)))) || (unit == (8232f64))) || (unit == (8233f64))) || (unit == (8239f64))) || (unit == (8287f64))) || (unit == (12288f64))) || (unit == (65279f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn starts_utf16_text(text: &[f64], search: &[f64]) -> bool {
    self::match_utf16_text(text, search, 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn ends_utf16_text(text: &[f64], search: &[f64]) -> bool {
    self::match_utf16_text(text, search, ((text.len() as f64) - (search.len() as f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn includes_utf16_text(text: &[f64], search: &[f64]) -> bool {
    ((self::index_utf16_text(text, search, 0f64)) >= (0f64))
}

/// Execute the checked shared source algorithm.
#[must_use]
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf8_text_units(bytes: &[f64]) -> Vec<f64> {
    self::read_java_script_utf8_string_units(bytes)
}

/// Execute the checked shared source algorithm.
#[must_use]
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
                        let source_ml_source1_s10 = self::source_ml_temporary_replace_utf16_text_loop24(text, search, replacement, all, result, &(plan), start, from);
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop28(text: &[f64], replacement: &[f64], mut result: Vec<f64>, mut kind: f64, mut last: f64, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (kind == (0f64)) {
                    (self::ml_array::optional_at(replacement, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)
                } else {
                    (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)
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
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
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
                            let at = first;
                            {
                                let result_2 = self::source_ml_temporary_replace_utf16_text_loop28(text, replacement, result, kind, last, at);
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
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop24(text: &[f64], search: &[f64], replacement: &[f64], mut all: bool, mut result: Vec<f64>, plan: &[f64], mut start: f64, mut from: f64) -> self::MlTemporaryReplaceUtf16TextLoop24Result {
    loop {
        return {
            let found = self::index_utf16_text(text, search, from);
            if (found < (0f64)) {
                self::MlTemporaryReplaceUtf16TextLoop24Result::MlTemporaryReplaceUtf16TextLoop24Done(result, start)
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
                                    if (!all || ((((search.len() as f64)) == (0f64)) && (found == ((text.len() as f64))))) {
                                        self::MlTemporaryReplaceUtf16TextLoop24Result::MlTemporaryReplaceUtf16TextLoop24Done(result_3, start_2)
                                    } else {
                                        {
                                            let from_2 = (start_2 + if (((search.len() as f64)) == (0f64)) {
                                                1f64
                                            } else {
                                                0f64
                                            });
                                            {
                                                (result, start, from) = (result_3, start_2, from_2);
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
        };
    }
}
}


/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> self::ml_string::Text {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[self::ml_string::Text], index: f64) -> self::ml_string::Text {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_record_name() -> Vec<f64> {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 110, 97, 109, 101, 32, 114, 111, 119])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_record_name(names: &[Vec<f64>], index: f64) -> Vec<f64> {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((names.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 110, 97, 109, 101, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(names, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_record_name())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_value_order(count: f64, reverse: bool) -> Vec<f64> {
    if (!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) {
        vec![(-1f64)]
    } else {
        {
            let order = Vec::<f64>::new();
            {
                let index = 0f64;
                {
                    let order_2 = self::source_ml_temporary_read_record_value_order_loop1(count, reverse, order, index);
                    order_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_projection() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 118, 97, 108, 117, 101, 115])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_projection_error(projection: &self::ml_string::Text, names: &[Vec<f64>], entries: &[bool]) -> self::ml_string::Text {
    if ((*projection) != (self::read_record_projection())) {
        self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110, 32, 99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110])
    } else {
        if (((names.len() as f64)) != ((entries.len() as f64))) {
            self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 104, 97, 115, 32, 105, 110, 99, 111, 110, 115, 105, 115, 116, 101, 110, 116, 32, 101, 110, 116, 114, 121, 32, 102, 97, 99, 116, 115])
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_s1 = self::source_ml_temporary_read_record_projection_error_loop2(names, entries, index);
                    match source_ml_source1_s1.clone() {
                        self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Done => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(source_ml_temporary_result2) => {
                            source_ml_temporary_result2.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_record_field_name(name: &[f64]) -> bool {
    {
        let index = 0f64;
        {
            let source_ml_source1_s2 = self::source_ml_temporary_accept_record_field_name_loop3(name, index);
            match source_ml_source1_s2.clone() {
                self::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Done => {
                    true
                }
                self::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Return(source_ml_temporary_result3) => {
                    source_ml_temporary_result3
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_record_field_names(left: &[f64], right: &[f64]) -> f64 {
    {
        let end = if (((left.len() as f64)) < ((right.len() as f64))) {
            (left.len() as f64)
        } else {
            (right.len() as f64)
        };
        {
            let index = 0f64;
            {
                let source_ml_source1_s3 = self::source_ml_temporary_compare_record_field_names_loop4(left, right, end, index);
                match source_ml_source1_s3.clone() {
                    self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Done => {
                        if (((left.len() as f64)) < ((right.len() as f64))) {
                            (-1f64)
                        } else {
                            if (((left.len() as f64)) > ((right.len() as f64))) {
                                1f64
                            } else {
                                0f64
                            }
                        }
                    }
                    self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Return(source_ml_temporary_result4) => {
                        source_ml_temporary_result4
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_position(fields: &[Vec<f64>], name: &[f64]) -> f64 {
    if !(self::accept_record_field_name(name)) {
        (-1f64)
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s4 = self::source_ml_temporary_read_record_field_position_loop5(fields, index);
                match source_ml_source1_s4.clone() {
                    self::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Done => {
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source1_s5 = self::source_ml_temporary_read_record_field_position_loop6(fields, name, index_2);
                                match source_ml_source1_s5.clone() {
                                    self::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Done => {
                                        (-1f64)
                                    }
                                    self::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Return(source_ml_temporary_result6) => {
                                        source_ml_temporary_result6
                                    }
                                }
                            }
                        }
                    }
                    self::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Return(source_ml_temporary_result6_2) => {
                        source_ml_temporary_result6_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_sorted_field_positions(fields: &[Vec<f64>]) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source1_s6 = self::source_ml_temporary_read_record_sorted_field_positions_loop7(fields, index);
            match source_ml_source1_s6.clone() {
                self::MlTemporaryReadRecordSortedFieldPositionsLoop7Result::MlTemporaryReadRecordSortedFieldPositionsLoop7Done => {
                    {
                        let sorted = self::read_record_value_order((fields.len() as f64), false);
                        {
                            let width = 1f64;
                            {
                                let sorted_2 = self::source_ml_temporary_read_record_sorted_field_positions_loop8(fields, sorted, width);
                                sorted_2
                            }
                        }
                    }
                }
                self::MlTemporaryReadRecordSortedFieldPositionsLoop7Result::MlTemporaryReadRecordSortedFieldPositionsLoop7Return(source_ml_temporary_result10) => {
                    source_ml_temporary_result10
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_order(fields: &[Vec<f64>]) -> Vec<f64> {
    {
        let sorted = self::read_record_sorted_field_positions(fields);
        if ((((sorted.len() as f64)) > (0f64)) && ((self::read_required_number_fact(&(sorted), 0f64)) < (0f64))) {
            vec![(-1f64)]
        } else {
            {
                let selected = Vec::<f64>::new();
                {
                    let index = 0f64;
                    {
                        let selected_2 = self::source_ml_temporary_read_record_field_order_loop11(fields, &(sorted), selected, index);
                        selected_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_insertion_order(fields: &[Vec<f64>]) -> Vec<f64> {
    {
        let sorted = self::read_record_sorted_field_positions(fields);
        if ((((sorted.len() as f64)) > (0f64)) && ((self::read_required_number_fact(&(sorted), 0f64)) < (0f64))) {
            vec![(-1f64)]
        } else {
            {
                let first = Vec::<f64>::new();
                {
                    let last = Vec::<f64>::new();
                    {
                        let start = 0f64;
                        {
                            let index = 0f64;
                            {
                                let source_ml_source1_s7 = self::source_ml_temporary_read_record_field_insertion_order_loop12(fields, &(sorted), first, last, start, index);
                                match source_ml_source1_s7.clone() {
                                    self::MlTemporaryReadRecordFieldInsertionOrderLoop12Result::MlTemporaryReadRecordFieldInsertionOrderLoop12Done(first_2, last_2) => {
                                        {
                                            let selected = Vec::<f64>::new();
                                            {
                                                let index_2 = 0f64;
                                                {
                                                    let selected_2 = self::source_ml_temporary_read_record_field_insertion_order_loop13(fields, &(first_2), &(last_2), selected, index_2);
                                                    selected_2
                                                }
                                            }
                                        }
                                    }
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
pub fn read_record_spread_role(kind: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*kind) == (self::ml_string::Text::from_units(vec![118, 97, 114]))) {
        self::ml_string::Text::from_units(vec![111, 112, 101, 110])
    } else {
        if ((*kind) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
            self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])
        } else {
            self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_type_constraint_step(left: &[Vec<f64>], right: &[Vec<f64>], left_open: bool, right_open: bool, phase: f64, index: f64) -> Vec<f64> {
    if (phase < (1f64)) {
        if (index >= ((left.len() as f64))) {
            vec![5f64, (-1f64), (-1f64), 1f64, 0f64]
        } else {
            {
                let matching = self::read_record_field_position(right, &(self::read_required_record_name(left, index)));
                if (matching >= (0f64)) {
                    vec![0f64, index, matching, 0f64, (index + 1f64)]
                } else {
                    vec![if right_open {
                        1f64
                    } else {
                        3f64
                    }, index, (-1f64), 0f64, (index + 1f64)]
                }
            }
        }
    } else {
        if (index >= ((right.len() as f64))) {
            vec![6f64, (-1f64), (-1f64), 1f64, index]
        } else {
            {
                let matching_2 = self::read_record_field_position(left, &(self::read_required_record_name(right, index)));
                if (matching_2 >= (0f64)) {
                    vec![5f64, matching_2, index, 1f64, (index + 1f64)]
                } else {
                    vec![if left_open {
                        2f64
                    } else {
                        4f64
                    }, (-1f64), index, 1f64, (index + 1f64)]
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_type_constraint_error(action: f64, name: &self::ml_string::Text) -> self::ml_string::Text {
    if ((action == (3f64)) || (action == (4f64))) {
        ((self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32])).concat(name)).concat(&self::ml_string::Text::from_units(vec![32, 105, 115, 32, 109, 105, 115, 115, 105, 110, 103]))
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_method_error(kind: &self::ml_string::Text, operation: &self::ml_string::Text, parameters: f64, arguments_count: f64) -> self::ml_string::Text {
    if ((*kind) != (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
        (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 104, 97, 115, 32, 110, 111, 32, 109, 101, 116, 104, 111, 100, 32])).concat(operation)
    } else {
        if (arguments_count != parameters) {
            ((((self::ml_string::Text::from_units(vec![46])).concat(operation)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 101, 120, 112, 101, 99, 116, 115, 32]))).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(parameters)))).concat(&self::ml_string::Text::from_units(vec![32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_inference_step(fields: &[Vec<f64>], name: &[f64], open: bool) -> Vec<f64> {
    {
        let index = self::read_record_field_position(fields, name);
        if (index >= (0f64)) {
            vec![0f64, index]
        } else {
            vec![if open {
                1f64
            } else {
                2f64
            }, (-1f64)]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn record_type_fact_field(starts: &[f64], counts: &[f64], labels: &[self::ml_string::Text], targets: &[f64], owner: f64, name: &self::ml_string::Text) -> f64 {
    if (owner < (0f64)) {
        (-1f64)
    } else {
        {
            let start = self::read_required_number_fact(starts, owner);
            {
                let end = (start + self::read_required_number_fact(counts, owner));
                {
                    let edge = start;
                    {
                        let source_ml_source1_s8 = self::source_ml_temporary_record_type_fact_field_loop15(labels, targets, name, end, edge);
                        match source_ml_source1_s8.clone() {
                            self::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Done => {
                                (-1f64)
                            }
                            self::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Return(source_ml_temporary_result15) => {
                                source_ml_temporary_result15
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
pub fn record_type_fact_kind(texts: &[self::ml_string::Text], starts: &[f64], counts: &[f64], labels: &[self::ml_string::Text], targets: &[f64], owner: f64) -> self::ml_string::Text {
    {
        let kind = self::record_type_fact_field(starts, counts, labels, targets, owner, &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])));
        if (kind < (0f64)) {
            self::ml_string::Text::from_units(vec![])
        } else {
            self::read_required_text_fact(texts, kind)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn record_type_fact_elements(starts: &[f64], counts: &[f64], targets: &[f64], owner: f64) -> Vec<f64> {
    {
        let values = Vec::<f64>::new();
        if (owner < (0f64)) {
            values
        } else {
            {
                let start = self::read_required_number_fact(starts, owner);
                {
                    let end = (start + self::read_required_number_fact(counts, owner));
                    {
                        let edge = start;
                        {
                            let values_2 = self::source_ml_temporary_record_type_fact_elements_loop16(targets, values, end, edge);
                            values_2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_type_owner_presence(texts: &[self::ml_string::Text], numbers: &[f64], starts: &[f64], counts: &[f64], labels: &[self::ml_string::Text], targets: &[f64], bindings: &[f64], root: f64, owner: f64) -> bool {
    {
        let pending = vec![root];
        {
            let seen = Vec::<f64>::new();
            {
                let position = 0f64;
                {
                    let source_ml_source1_s9 = self::source_ml_temporary_read_record_type_owner_presence_loop17(texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position);
                    match source_ml_source1_s9.clone() {
                        self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Done => {
                            false
                        }
                        self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Return(source_ml_temporary_result17) => {
                            source_ml_temporary_result17
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_literal_property_index(key: &[f64]) -> f64 {
    if ((((key.len() as f64)) == (0f64)) || (((key.len() as f64)) > (10f64))) {
        (-1f64)
    } else {
        if ((((key.len() as f64)) > (1f64)) && {
            let source_ml_source1_comparison_operand10 = self::ml_array::optional_at(key, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand11 = 48f64;
                match (source_ml_source1_comparison_operand10.clone(), source_ml_source1_comparison_operand11) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) {
            (-1f64)
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s12 = self::source_ml_temporary_read_java_script_literal_property_index_loop22(key, value, at);
                        match source_ml_source1_s12.clone() {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Done(value_2) => {
                                if (value_2 <= (4294967294f64)) {
                                    value_2
                                } else {
                                    (-1f64)
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Return(source_ml_temporary_result22) => {
                                source_ml_temporary_result22
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
pub fn read_java_script_literal_property_form(kind: &self::ml_string::Text, index: f64) -> self::ml_string::Text {
    if (((((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) || ((*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) || ((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && (index >= (0f64))) {
        self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
    } else {
        self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_index_number_form(kind: &self::ml_string::Text, element: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else {
        if (((*kind) == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || ((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
        } else {
            if (((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) || (((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && ((*element) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))) {
                self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
            } else {
                if (((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && ((((*element) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || ((*element) == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || ((*element) == (self::ml_string::Text::from_units(vec![110, 97, 116]))))) {
                    self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
                } else {
                    self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_record_order_name() -> Vec<f64> {
    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_record_order_position() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 102, 105, 101, 108, 100, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_name(names: &[Vec<f64>], index: f64) -> Vec<f64> {
    (self::ml_array::optional_at(names, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_java_script_record_order_name())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_position(positions: &[f64], index: f64) -> f64 {
    (self::ml_array::optional_at(positions, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_java_script_record_order_position())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_order_names(names: &[Vec<f64>]) -> bool {
    {
        let index = 0f64;
        {
            let source_ml_source1_s13 = self::source_ml_temporary_accept_java_script_record_order_names_loop23(names, index);
            match source_ml_source1_s13.clone() {
                self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Done => {
                    true
                }
                self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(source_ml_temporary_result23) => {
                    source_ml_temporary_result23
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_order_permutation(count: f64, order: &[f64]) -> bool {
    if (((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || ((count == (0f64)) && (((1f64 / count)) < (0f64)))) || (((order.len() as f64)) != count)) {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s14 = self::source_ml_temporary_accept_java_script_record_order_permutation_loop24(count, order, index);
                match source_ml_source1_s14.clone() {
                    self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Done => {
                        true
                    }
                    self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Return(source_ml_temporary_result24) => {
                        source_ml_temporary_result24
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_creation_error(names: &[Vec<f64>], insertion: &[f64]) -> self::ml_string::Text {
    if (!(self::accept_java_script_record_order_names(names)) || !(self::accept_java_script_record_order_permutation((names.len() as f64), insertion))) {
        self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 115, 101, 113, 117, 101, 110, 99, 101])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_record_property_order(names: &[Vec<f64>], insertion: &[f64]) -> Vec<f64> {
    {
        let error = self::read_java_script_record_order_creation_error(names, insertion);
        if !error.0.is_empty() {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (error.clone()).0, message_present: true }).raise()
        } else {
            {
                let numeric = Vec::<f64>::new();
                {
                    let field = 0f64;
                    {
                        let numeric_2 = self::source_ml_temporary_create_java_script_record_property_order_loop26(names, numeric, field);
                        {
                            let result = numeric_2;
                            {
                                let index = 0f64;
                                {
                                    let result_2 = self::source_ml_temporary_create_java_script_record_property_order_loop28(names, insertion, result, index);
                                    result_2
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
pub fn accept_java_script_record_property_order(names: &[Vec<f64>], order: &[f64]) -> bool {
    if (!(self::accept_java_script_record_order_names(names)) || !(self::accept_java_script_record_order_permutation((names.len() as f64), order))) {
        false
    } else {
        {
            let expected = self::create_java_script_record_property_order(names, order);
            {
                let index = 0f64;
                {
                    let source_ml_source1_s15 = self::source_ml_temporary_accept_java_script_record_property_order_loop29(order, &(expected), index);
                    match source_ml_source1_s15.clone() {
                        self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Done => {
                            true
                        }
                        self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Return(source_ml_temporary_result29) => {
                            source_ml_temporary_result29
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_merge_error(names: &[Vec<f64>], chunks: &[Vec<f64>]) -> self::ml_string::Text {
    if !(self::accept_java_script_record_order_names(names)) {
        self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101, 115])
    } else {
        {
            let insertion = Vec::<f64>::new();
            {
                let chunk = 0f64;
                {
                    let source_ml_source1_s16 = self::source_ml_temporary_read_java_script_record_order_merge_error_loop30(names, chunks, insertion, chunk);
                    match source_ml_source1_s16.clone() {
                        self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Done(insertion_2) => {
                            self::read_java_script_record_order_creation_error(names, &(insertion_2))
                        }
                        self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Return(source_ml_temporary_result30) => {
                            source_ml_temporary_result30.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn merge_java_script_record_property_order(names: &[Vec<f64>], chunks: &[Vec<f64>]) -> Vec<f64> {
    {
        let error = self::read_java_script_record_order_merge_error(names, chunks);
        if !error.0.is_empty() {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (error.clone()).0, message_present: true }).raise()
        } else {
            {
                let insertion = Vec::<f64>::new();
                {
                    let chunk = 0f64;
                    {
                        let insertion_2 = self::source_ml_temporary_merge_java_script_record_property_order_loop32(chunks, insertion, chunk);
                        self::create_java_script_record_property_order(names, &(insertion_2))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_rebase_error(target: &[Vec<f64>], source: &[Vec<f64>], order: &[f64]) -> self::ml_string::Text {
    if (!(self::accept_java_script_record_order_names(target)) || !(self::accept_java_script_record_property_order(source, order))) {
        self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 112, 114, 101, 97, 100, 32, 115, 111, 117, 114, 99, 101])
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s17 = self::source_ml_temporary_read_java_script_record_order_rebase_error_loop34(target, source, index);
                match source_ml_source1_s17.clone() {
                    self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Done => {
                        self::ml_string::Text::from_units(vec![])
                    }
                    self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Return(source_ml_temporary_result34) => {
                        source_ml_temporary_result34.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn rebase_java_script_record_property_order(target: &[Vec<f64>], source: &[Vec<f64>], order: &[f64]) -> Vec<f64> {
    {
        let error = self::read_java_script_record_order_rebase_error(target, source, order);
        if !error.0.is_empty() {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (error.clone()).0, message_present: true }).raise()
        } else {
            {
                let result = Vec::<f64>::new();
                {
                    let index = 0f64;
                    {
                        let result_2 = self::source_ml_temporary_rebase_java_script_record_property_order_loop35(target, source, order, result, index);
                        result_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn copy_java_script_record_property_order(target: &[Vec<f64>], source: &[Vec<f64>], order: &[f64]) -> Vec<f64> {
    if (!(self::accept_java_script_record_order_names(target)) || !(self::accept_java_script_record_property_order(source, order))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 105, 101, 108, 100, 32, 111, 114, 100, 101, 114])).0, message_present: true }).raise()
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s18 = self::source_ml_temporary_copy_java_script_record_property_order_loop36(target, source, index);
                match source_ml_source1_s18.clone() {
                    self::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop36Result::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop36Done => {
                        {
                            let result = Vec::<f64>::new();
                            {
                                let index_2 = 0f64;
                                {
                                    let result_2 = self::source_ml_temporary_copy_java_script_record_property_order_loop37(target, source, order, result, index_2);
                                    result_2
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
pub fn read_java_script_record_order_key_error(names: &[Vec<f64>], order: &[f64]) -> self::ml_string::Text {
    if self::accept_java_script_record_property_order(names, order) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 114, 100, 101, 114])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_property_keys(names: &[Vec<f64>], order: &[f64]) -> Vec<Vec<f64>> {
    {
        let error = self::read_java_script_record_order_key_error(names, order);
        if !error.0.is_empty() {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (error.clone()).0, message_present: true }).raise()
        } else {
            {
                let result = Vec::<Vec<f64>>::new();
                {
                    let index = 0f64;
                    {
                        let result_2 = self::source_ml_temporary_read_java_script_record_property_keys_loop38(names, order, result, index);
                        result_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_property_key_texts(names: &[Vec<f64>], order: &[f64]) -> Vec<self::ml_string::Text> {
    {
        let rows = self::read_java_script_record_property_keys(names, order);
        {
            let result = Vec::<self::ml_string::Text>::new();
            {
                let index = 0f64;
                {
                    let result_2 = self::source_ml_temporary_read_java_script_record_property_key_texts_loop39(&(rows), result, index);
                    result_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_property_index(names: &[Vec<f64>], key: &self::ml_string::Text) -> f64 {
    {
        let units = Vec::<f64>::new();
        {
            let index = 0f64;
            {
                let units_2 = self::source_ml_temporary_read_java_script_record_property_index_loop41(key, units, index);
                self::read_record_field_position(names, &(units_2))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_dense_property_key_texts(length: f64) -> Vec<self::ml_string::Text> {
    if ((!(self::ml_math::is_safe_integer(length)) || (length < (0f64))) || ((length == (0f64)) && (((1f64 / length)) < (0f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 100, 101, 110, 115, 101, 32, 111, 119, 110, 45, 102, 105, 101, 108, 100, 32, 108, 101, 110, 103, 116, 104])).0, message_present: true }).raise()
    } else {
        {
            let result = Vec::<self::ml_string::Text>::new();
            {
                let index = 0f64;
                {
                    let result_2 = self::source_ml_temporary_create_java_script_dense_property_key_texts_loop42(length, result, index);
                    result_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_string_own_property_values(text: &self::ml_string::Text) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let index = 0f64;
            {
                let result_2 = self::source_ml_temporary_create_java_script_string_own_property_values_loop43(text, result, index);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_value_order_loop1(mut count: f64, mut reverse: bool, mut order: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < count) {
            {
                let order_2 = { let (ml_append_right, ml_append_left) = (vec![if reverse {
                    ((count - index) - 1f64)
                } else {
                    index
                }], order); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (order, index) = (order_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            order
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRecordProjectionErrorLoop2Result {
    MlTemporaryReadRecordProjectionErrorLoop2Done,
    MlTemporaryReadRecordProjectionErrorLoop2Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_projection_error_loop2(names: &[Vec<f64>], entries: &[bool], mut index: f64) -> self::MlTemporaryReadRecordProjectionErrorLoop2Result {
    loop {
        return if (index < ((names.len() as f64))) {
            if !(match self::ml_array::optional_at(entries, self::ml_array::number_index(index)) { Some(ml_present) => ml_present, None => false }) {
                self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 104, 97, 115, 32, 97, 32, 109, 97, 108, 102, 111, 114, 109, 101, 100, 32, 102, 105, 101, 108, 100, 32, 101, 110, 116, 114, 121]))
            } else {
                if !(self::accept_record_field_name(&(self::read_required_record_name(names, index)))) {
                    self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 104, 97, 115, 32, 97, 110, 32, 105, 110, 118, 97, 108, 105, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101]))
                } else {
                    if ((index > (0f64)) && ((self::compare_record_field_names(&(self::read_required_record_name(names, (index - 1f64))), &(self::read_required_record_name(names, index)))) >= (0f64))) {
                        self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 117, 110, 105, 113, 117, 101, 32, 102, 105, 101, 108, 100, 115, 32, 105, 110, 32, 85, 84, 70, 49, 54, 32, 111, 114, 100, 101, 114]))
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
            self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRecordFieldNameLoop3Result {
    MlTemporaryAcceptRecordFieldNameLoop3Done,
    MlTemporaryAcceptRecordFieldNameLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_record_field_name_loop3(name: &[f64], mut index: f64) -> self::MlTemporaryAcceptRecordFieldNameLoop3Result {
    loop {
        return if (index < ((name.len() as f64))) {
            {
                let unit = self::ml_array::optional_at(name, self::ml_array::number_index(index));
                if ((!(match unit.clone() { Some(ml_present) => self::ml_math::is_integer(ml_present), None => false }) || {
                    let source_ml_source1_comparison_operand19 = unit.clone();
                    {
                        let source_ml_source1_comparison_operand20 = 0f64;
                        if match (source_ml_source1_comparison_operand19.clone(), None::<f64>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                            ((f64::NAN) < source_ml_source1_comparison_operand20)
                        } else {
                            ((source_ml_source1_comparison_operand19.clone().expect("a proven present value")) < source_ml_source1_comparison_operand20)
                        }
                    }
                }) || {
                    let source_ml_source1_comparison_operand21 = unit.clone();
                    {
                        let source_ml_source1_comparison_operand22 = 65535f64;
                        if match (source_ml_source1_comparison_operand21.clone(), None::<f64>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                            ((f64::NAN) > source_ml_source1_comparison_operand22)
                        } else {
                            ((source_ml_source1_comparison_operand21.clone().expect("a proven present value")) > source_ml_source1_comparison_operand22)
                        }
                    }
                }) {
                    self::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Return(false)
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
            self::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareRecordFieldNamesLoop4Result {
    MlTemporaryCompareRecordFieldNamesLoop4Done,
    MlTemporaryCompareRecordFieldNamesLoop4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_record_field_names_loop4(left: &[f64], right: &[f64], mut end: f64, mut index: f64) -> self::MlTemporaryCompareRecordFieldNamesLoop4Result {
    loop {
        return if (index < end) {
            if ((self::read_required_number_fact(left, index)) < (self::read_required_number_fact(right, index))) {
                self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Return((-1f64))
            } else {
                if ((self::read_required_number_fact(left, index)) > (self::read_required_number_fact(right, index))) {
                    self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Return(1f64)
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
            self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordFieldPositionLoop6Result {
    MlTemporaryReadRecordFieldPositionLoop6Done,
    MlTemporaryReadRecordFieldPositionLoop6Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_position_loop6(fields: &[Vec<f64>], name: &[f64], mut index: f64) -> self::MlTemporaryReadRecordFieldPositionLoop6Result {
    loop {
        return if (index < ((fields.len() as f64))) {
            {
                let comparison = self::compare_record_field_names(&(self::read_required_record_name(fields, index)), name);
                if (!((comparison < (0f64))) && !((comparison > (0f64)))) {
                    self::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Return(index)
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
            self::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordFieldPositionLoop5Result {
    MlTemporaryReadRecordFieldPositionLoop5Done,
    MlTemporaryReadRecordFieldPositionLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_position_loop5(fields: &[Vec<f64>], mut index: f64) -> self::MlTemporaryReadRecordFieldPositionLoop5Result {
    loop {
        return if (index < ((fields.len() as f64))) {
            if !(self::accept_record_field_name(&(self::read_required_record_name(fields, index)))) {
                self::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Return((-1f64))
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
            self::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_sorted_field_positions_loop10(fields: &[Vec<f64>], sorted: &[f64], mut merged: Vec<f64>, mut middle: f64, mut end: f64, mut left: f64, mut right: f64) -> Vec<f64> {
    loop {
        return if ((left < middle) || (right < end)) {
            if ((right >= end) || ((left < middle) && ((self::compare_record_field_names(&(self::read_required_record_name(fields, self::read_required_number_fact(sorted, left))), &(self::read_required_record_name(fields, self::read_required_number_fact(sorted, right))))) <= (0f64)))) {
                {
                    let merged_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_number_fact(sorted, left)], merged); self::ml_array::append(ml_append_left, ml_append_right) };
                    {
                        let left_2 = (left + 1f64);
                        {
                            (merged, left) = (merged_2, left_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let merged_3 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_number_fact(sorted, right)], merged); self::ml_array::append(ml_append_left, ml_append_right) };
                    {
                        let right_2 = (right + 1f64);
                        {
                            (merged, right) = (merged_3, right_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            merged
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_sorted_field_positions_loop9(fields: &[Vec<f64>], sorted: &[f64], mut width: f64, mut merged: Vec<f64>, mut start: f64) -> Vec<f64> {
    loop {
        return if (start < ((sorted.len() as f64))) {
            {
                let middle = if (((start + width)) < ((sorted.len() as f64))) {
                    (start + width)
                } else {
                    (sorted.len() as f64)
                };
                {
                    let end = if (((middle + width)) < ((sorted.len() as f64))) {
                        (middle + width)
                    } else {
                        (sorted.len() as f64)
                    };
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let merged_2 = self::source_ml_temporary_read_record_sorted_field_positions_loop10(fields, sorted, merged, middle, end, left, right);
                                {
                                    let start_2 = (start + (width * 2f64));
                                    {
                                        (merged, start) = (merged_2, start_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            merged
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_sorted_field_positions_loop8(fields: &[Vec<f64>], mut sorted: Vec<f64>, mut width: f64) -> Vec<f64> {
    loop {
        return if (width < ((sorted.len() as f64))) {
            {
                let merged = Vec::<f64>::new();
                {
                    let start = 0f64;
                    {
                        let merged_2 = self::source_ml_temporary_read_record_sorted_field_positions_loop9(fields, &(sorted), width, merged, start);
                        {
                            let sorted_2 = merged_2;
                            {
                                let width_2 = (width * 2f64);
                                {
                                    (sorted, width) = (sorted_2, width_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            sorted
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordSortedFieldPositionsLoop7Result {
    MlTemporaryReadRecordSortedFieldPositionsLoop7Done,
    MlTemporaryReadRecordSortedFieldPositionsLoop7Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_sorted_field_positions_loop7(fields: &[Vec<f64>], mut index: f64) -> self::MlTemporaryReadRecordSortedFieldPositionsLoop7Result {
    loop {
        return if (index < ((fields.len() as f64))) {
            if !(self::accept_record_field_name(&(self::read_required_record_name(fields, index)))) {
                self::MlTemporaryReadRecordSortedFieldPositionsLoop7Result::MlTemporaryReadRecordSortedFieldPositionsLoop7Return(vec![(-1f64)])
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
            self::MlTemporaryReadRecordSortedFieldPositionsLoop7Result::MlTemporaryReadRecordSortedFieldPositionsLoop7Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_order_loop11(fields: &[Vec<f64>], sorted: &[f64], mut selected: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((sorted.len() as f64))) {
            if ((((index + 1f64)) >= ((sorted.len() as f64))) || ((self::compare_record_field_names(&(self::read_required_record_name(fields, self::read_required_number_fact(sorted, index))), &(self::read_required_record_name(fields, self::read_required_number_fact(sorted, (index + 1f64)))))) < (0f64))) {
                {
                    let selected_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_number_fact(sorted, index)], selected); self::ml_array::append(ml_append_left, ml_append_right) };
                    {
                        let index_2 = (index + 1f64);
                        {
                            (selected, index) = (selected_2, index_2);
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
        } else {
            selected
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_insertion_order_loop14(fields: &[Vec<f64>], first: &[f64], last: &[f64], mut selected: Vec<f64>, mut index: f64, mut lower: f64, mut upper: f64) -> Vec<f64> {
    loop {
        return if (lower < upper) {
            {
                let middle = (((lower + upper) / 2f64)).floor();
                {
                    let comparison = self::compare_record_field_names(&(self::read_required_record_name(fields, index)), &(self::read_required_record_name(fields, self::read_required_number_fact(last, middle))));
                    if (comparison < (0f64)) {
                        {
                            let upper_2 = middle;
                            {
                                upper = upper_2;
                                continue;
                            }
                        }
                    } else {
                        if (comparison > (0f64)) {
                            {
                                let lower_2 = (middle + 1f64);
                                {
                                    lower = lower_2;
                                    continue;
                                }
                            }
                        } else {
                            {
                                let selected_3 = if (!(((self::read_required_number_fact(first, middle)) < index)) && !(((self::read_required_number_fact(first, middle)) > index))) {
                                    {
                                        let selected_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_number_fact(last, middle)], selected); self::ml_array::append(ml_append_left, ml_append_right) };
                                        selected_2
                                    }
                                } else {
                                    selected
                                };
                                {
                                    let lower_3 = upper;
                                    {
                                        (selected, lower) = (selected_3, lower_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            selected
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_insertion_order_loop13(fields: &[Vec<f64>], first: &[f64], last: &[f64], mut selected: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((fields.len() as f64))) {
            {
                let lower = 0f64;
                {
                    let upper = (last.len() as f64);
                    {
                        let selected_2 = self::source_ml_temporary_read_record_field_insertion_order_loop14(fields, first, last, selected, index, lower, upper);
                        {
                            let index_2 = (index + 1f64);
                            {
                                (selected, index) = (selected_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            selected
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordFieldInsertionOrderLoop12Result {
    MlTemporaryReadRecordFieldInsertionOrderLoop12Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_insertion_order_loop12(fields: &[Vec<f64>], sorted: &[f64], mut first: Vec<f64>, mut last: Vec<f64>, mut start: f64, mut index: f64) -> self::MlTemporaryReadRecordFieldInsertionOrderLoop12Result {
    loop {
        return if (index < ((sorted.len() as f64))) {
            if ((((index + 1f64)) >= ((sorted.len() as f64))) || ((self::compare_record_field_names(&(self::read_required_record_name(fields, self::read_required_number_fact(sorted, index))), &(self::read_required_record_name(fields, self::read_required_number_fact(sorted, (index + 1f64)))))) < (0f64))) {
                {
                    let first_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_number_fact(sorted, start)], first); self::ml_array::append(ml_append_left, ml_append_right) };
                    {
                        let last_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_number_fact(sorted, index)], last); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let start_2 = (index + 1f64);
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (first, last, start, index) = (first_2, last_2, start_2, index_2);
                                    continue;
                                }
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
        } else {
            self::MlTemporaryReadRecordFieldInsertionOrderLoop12Result::MlTemporaryReadRecordFieldInsertionOrderLoop12Done(first, last)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryRecordTypeFactFieldLoop15Result {
    MlTemporaryRecordTypeFactFieldLoop15Done,
    MlTemporaryRecordTypeFactFieldLoop15Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_record_type_fact_field_loop15(labels: &[self::ml_string::Text], targets: &[f64], name: &self::ml_string::Text, mut end: f64, mut edge: f64) -> self::MlTemporaryRecordTypeFactFieldLoop15Result {
    loop {
        return if (edge < end) {
            if ((self::read_required_text_fact(labels, edge)) == (*name)) {
                self::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Return(self::read_required_number_fact(targets, edge))
            } else {
                {
                    let edge_2 = (edge + 1f64);
                    {
                        edge = edge_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_record_type_fact_elements_loop16(targets: &[f64], mut values: Vec<f64>, mut end: f64, mut edge: f64) -> Vec<f64> {
    loop {
        return if (edge < end) {
            {
                let values_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_number_fact(targets, edge)], values); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let edge_2 = (edge + 1f64);
                    {
                        (values, edge) = (values_2, edge_2);
                        continue;
                    }
                }
            }
        } else {
            values
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRecordTypeOwnerPresenceLoop17Result {
    MlTemporaryReadRecordTypeOwnerPresenceLoop17Done,
    MlTemporaryReadRecordTypeOwnerPresenceLoop17Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop19(seen: &[f64], mut node: f64, mut visited: bool, mut index: f64) -> bool {
    loop {
        return if (index < ((seen.len() as f64))) {
            if ((self::read_required_number_fact(seen, index)) == node) {
                {
                    let visited_2 = true;
                    {
                        let index_2 = (index + 1f64);
                        {
                            (visited, index) = (visited_2, index_2);
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
        } else {
            visited
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop20(mut pending: Vec<f64>, children: &[f64], mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((children.len() as f64))) {
            {
                let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_number_fact(children, index)], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (pending, index) = (pending_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            pending
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop21(starts: &[f64], counts: &[f64], labels: &[self::ml_string::Text], targets: &[f64], mut pending: Vec<f64>, fields: &[f64], mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((fields.len() as f64))) {
            {
                let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, self::read_required_number_fact(fields, index), &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])))], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (pending, index) = (pending_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            pending
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordTypeOwnerPresenceLoop18Result {
    MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(f64, self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop18(texts: &[self::ml_string::Text], numbers: &[f64], starts: &[f64], counts: &[f64], labels: &[self::ml_string::Text], targets: &[f64], bindings: &[f64], mut node: f64, mut kind: self::ml_string::Text) -> self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result {
    loop {
        return if (kind == (self::ml_string::Text::from_units(vec![118, 97, 114]))) {
            {
                let id = self::record_type_fact_field(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![105, 100])));
                {
                    let variable = self::read_required_number_fact(numbers, id);
                    if ((!(self::ml_math::is_integer(variable)) || (variable < (0f64))) || (variable >= ((bindings.len() as f64)))) {
                        self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind.clone())
                    } else {
                        {
                            let bound = self::read_required_number_fact(bindings, variable);
                            {
                                let bound_kind = self::record_type_fact_kind(texts, starts, counts, labels, targets, bound);
                                if (bound_kind == (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind.clone())
                                } else {
                                    {
                                        let node_2 = bound;
                                        {
                                            let kind_2 = bound_kind.clone();
                                            {
                                                (node, kind) = (node_2, kind_2.clone());
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
            self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop17(texts: &[self::ml_string::Text], numbers: &[f64], starts: &[f64], counts: &[f64], labels: &[self::ml_string::Text], targets: &[f64], bindings: &[f64], mut owner: f64, mut pending: Vec<f64>, mut seen: Vec<f64>, mut position: f64) -> self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result {
    loop {
        return if (position < ((pending.len() as f64))) {
            {
                let node = self::read_required_number_fact(&(pending), position);
                {
                    let kind = self::record_type_fact_kind(texts, starts, counts, labels, targets, node);
                    {
                        let source_ml_source1_s23 = self::source_ml_temporary_read_record_type_owner_presence_loop18(texts, numbers, starts, counts, labels, targets, bindings, node, kind.clone());
                        match source_ml_source1_s23.clone() {
                            self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node_2, kind_2) => {
                                if (node_2 == owner) {
                                    self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Return(true)
                                } else {
                                    {
                                        let visited = false;
                                        {
                                            let index = 0f64;
                                            {
                                                let visited_2 = self::source_ml_temporary_read_record_type_owner_presence_loop19(&(seen), node_2, visited, index);
                                                if visited_2 {
                                                    {
                                                        let position_2 = (position + 1f64);
                                                        {
                                                            position = position_2;
                                                            continue;
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let seen_2 = { let (ml_append_right, ml_append_left) = (vec![node_2], seen); self::ml_array::append(ml_append_left, ml_append_right) };
                                                        if ((((kind_2 == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) || (kind_2 == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))) || (kind_2 == (self::ml_string::Text::from_units(vec![115, 101, 116])))) || (kind_2 == (self::ml_string::Text::from_units(vec![99, 101, 108, 108])))) {
                                                            {
                                                                let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])))], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let position_3 = (position + 1f64);
                                                                    {
                                                                        (pending, seen, position) = (pending_2, seen_2, position_3);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if (kind_2 == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                                                {
                                                                    let pending_3 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![107, 101, 121])))], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        let pending_4 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))], pending_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        {
                                                                            let position_4 = (position + 1f64);
                                                                            {
                                                                                (pending, seen, position) = (pending_4, seen_2, position_4);
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if (((kind_2 == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) || (kind_2 == (self::ml_string::Text::from_units(vec![117, 110, 105, 111, 110])))) || (kind_2 == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))) {
                                                                    {
                                                                        let field = if (kind_2 == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) {
                                                                            self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 115])
                                                                        } else {
                                                                            if (kind_2 == (self::ml_string::Text::from_units(vec![117, 110, 105, 111, 110]))) {
                                                                                self::ml_string::Text::from_units(vec![97, 108, 116, 101, 114, 110, 97, 116, 105, 118, 101, 115])
                                                                            } else {
                                                                                self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 115])
                                                                            }
                                                                        };
                                                                        {
                                                                            let sequence = self::record_type_fact_field(starts, counts, labels, targets, node_2, &(field));
                                                                            {
                                                                                let children = self::record_type_fact_elements(starts, counts, targets, sequence);
                                                                                {
                                                                                    let index_2 = 0f64;
                                                                                    {
                                                                                        let pending_5 = self::source_ml_temporary_read_record_type_owner_presence_loop20(pending, &(children), index_2);
                                                                                        if (kind_2 == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
                                                                                            {
                                                                                                let pending_6 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![114, 101, 116])))], pending_5); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                {
                                                                                                    let position_5 = (position + 1f64);
                                                                                                    {
                                                                                                        (pending, seen, position) = (pending_6, seen_2, position_5);
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            {
                                                                                                let position_6 = (position + 1f64);
                                                                                                {
                                                                                                    (pending, seen, position) = (pending_5, seen_2, position_6);
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
                                                                    if (kind_2 == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
                                                                        {
                                                                            let sequence_2 = self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                                                            {
                                                                                let fields = self::record_type_fact_elements(starts, counts, targets, sequence_2);
                                                                                {
                                                                                    let index_3 = 0f64;
                                                                                    {
                                                                                        let pending_7 = self::source_ml_temporary_read_record_type_owner_presence_loop21(starts, counts, labels, targets, pending, &(fields), index_3);
                                                                                        {
                                                                                            let position_7 = (position + 1f64);
                                                                                            {
                                                                                                (pending, seen, position) = (pending_7, seen_2, position_7);
                                                                                                continue;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let position_8 = (position + 1f64);
                                                                            {
                                                                                (seen, position) = (seen_2, position_8);
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
        } else {
            self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop22(key: &[f64], mut value: f64, mut at: f64) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result {
    loop {
        return if (at < ((key.len() as f64))) {
            {
                let digit = (match self::ml_array::optional_at(key, self::ml_array::number_index(at)) { Some(ml_present) => ml_present, None => f64::NAN } - 48f64);
                if ((!(self::ml_math::is_integer(digit)) || (digit < (0f64))) || (digit > (9f64))) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Return((-1f64))
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
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result {
    MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Done,
    MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_order_names_loop23(names: &[Vec<f64>], mut index: f64) -> self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result {
    loop {
        return if (index < ((names.len() as f64))) {
            {
                let name = self::read_java_script_record_order_name(names, index);
                if !(self::accept_record_field_name(&(name))) {
                    self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(false)
                } else {
                    if ((index > (0f64)) && ((self::compare_record_field_names(&(self::read_java_script_record_order_name(names, (index - 1f64))), &(name))) >= (0f64))) {
                        self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(false)
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
            self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result {
    MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Done,
    MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Result {
    MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Done,
    MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_order_permutation_loop25(order: &[f64], mut index: f64, mut value: f64, mut previous: f64) -> self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Result {
    loop {
        return if (previous < index) {
            if ((self::read_java_script_record_order_position(order, previous)) == value) {
                self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Return(false)
            } else {
                {
                    let previous_2 = (previous + 1f64);
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_order_permutation_loop24(mut count: f64, order: &[f64], mut index: f64) -> self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result {
    loop {
        return if (index < ((order.len() as f64))) {
            {
                let value = self::read_java_script_record_order_position(order, index);
                if (((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value >= count)) || ((value == (0f64)) && (((1f64 / value)) < (0f64)))) {
                    self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Return(false)
                } else {
                    {
                        let previous = 0f64;
                        {
                            let source_ml_source1_s24 = self::source_ml_temporary_accept_java_script_record_order_permutation_loop25(order, index, value, previous);
                            match source_ml_source1_s24.clone() {
                                self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Done => {
                                    {
                                        let index_2 = (index + 1f64);
                                        {
                                            index = index_2;
                                            continue;
                                        }
                                    }
                                }
                                self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Return(source_ml_temporary_result25) => {
                                    self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Return(source_ml_temporary_result25)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_record_property_order_loop27(names: &[Vec<f64>], numeric: &[f64], mut key: f64, mut position: f64) -> f64 {
    loop {
        return if ((position < ((numeric.len() as f64))) && ((self::read_java_script_literal_property_index(&(self::read_java_script_record_order_name(names, self::read_java_script_record_order_position(numeric, position))))) < key)) {
            {
                let position_2 = (position + 1f64);
                {
                    position = position_2;
                    continue;
                }
            }
        } else {
            position
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_record_property_order_loop26(names: &[Vec<f64>], mut numeric: Vec<f64>, mut field: f64) -> Vec<f64> {
    loop {
        return if (field < ((names.len() as f64))) {
            {
                let key = self::read_java_script_literal_property_index(&(self::read_java_script_record_order_name(names, field)));
                if (key < (0f64)) {
                    {
                        let field_2 = (field + 1f64);
                        {
                            field = field_2;
                            continue;
                        }
                    }
                } else {
                    {
                        let position = 0f64;
                        {
                            let position_2 = self::source_ml_temporary_create_java_script_record_property_order_loop27(names, &(numeric), key, position);
                            {
                                let numeric_2 = {
                                    let source_ml_source1_concat_value25 = self::ml_array::slice(&numeric, 0f64, position_2);
                                    {
                                        let source_ml_source1_concat_value26 = vec![field];
                                        {
                                            let source_ml_source1_concat_value27 = self::ml_array::slice(&numeric, position_2, f64::INFINITY);
                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value25), source_ml_source1_concat_value26), source_ml_source1_concat_value27)
                                        }
                                    }
                                };
                                {
                                    let field_3 = (field + 1f64);
                                    {
                                        (numeric, field) = (numeric_2, field_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            numeric
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_record_property_order_loop28(names: &[Vec<f64>], insertion: &[f64], mut result: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((insertion.len() as f64))) {
            {
                let field = self::read_java_script_record_order_position(insertion, index);
                if ((self::read_java_script_literal_property_index(&(self::read_java_script_record_order_name(names, field)))) < (0f64)) {
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value28 = result;
                            {
                                let source_ml_source1_concat_value29 = vec![field];
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value28), source_ml_source1_concat_value29)
                            }
                        };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (result, index) = (result_2, index_2);
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
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result {
    MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Done,
    MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_property_order_loop29(order: &[f64], expected: &[f64], mut index: f64) -> self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result {
    loop {
        return if (index < ((order.len() as f64))) {
            if ((self::read_java_script_record_order_position(order, index)) != (self::read_java_script_record_order_position(expected, index))) {
                self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Return(false)
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
            self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop31(names: &[Vec<f64>], mut insertion: Vec<f64>, values: &[f64], mut index: f64) -> self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result {
    loop {
        return if (index < ((values.len() as f64))) {
            {
                let field = self::read_java_script_record_order_position(values, index);
                if (((!(self::ml_math::is_integer(field)) || (field < (0f64))) || (field >= ((names.len() as f64)))) || ((field == (0f64)) && (((1f64 / field)) < (0f64)))) {
                    self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Return(self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 111, 117, 114, 99, 101, 32, 102, 105, 101, 108, 100]))
                } else {
                    if !(self::ml_array::includes(&insertion, &field, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                        {
                            let insertion_2 = {
                                let source_ml_source1_concat_value30 = insertion;
                                {
                                    let source_ml_source1_concat_value31 = vec![field];
                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value30), source_ml_source1_concat_value31)
                                }
                            };
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (insertion, index) = (insertion_2, index_2);
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
            }
        } else {
            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Done(insertion)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop30(names: &[Vec<f64>], chunks: &[Vec<f64>], mut insertion: Vec<f64>, mut chunk: f64) -> self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result {
    loop {
        return if (chunk < ((chunks.len() as f64))) {
            {
                let values = self::read_java_script_record_order_name(chunks, chunk);
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s32 = self::source_ml_temporary_read_java_script_record_order_merge_error_loop31(names, insertion, &(values), index);
                        match source_ml_source1_s32.clone() {
                            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Done(insertion_2) => {
                                {
                                    let chunk_2 = (chunk + 1f64);
                                    {
                                        (insertion, chunk) = (insertion_2, chunk_2);
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Return(source_ml_temporary_result31) => {
                                self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Return(source_ml_temporary_result31.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Done(insertion)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_java_script_record_property_order_loop33(mut insertion: Vec<f64>, values: &[f64], mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((values.len() as f64))) {
            {
                let field = self::read_java_script_record_order_position(values, index);
                if !(self::ml_array::includes(&insertion, &field, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                    {
                        let insertion_2 = {
                            let source_ml_source1_concat_value33 = insertion;
                            {
                                let source_ml_source1_concat_value34 = vec![field];
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value33), source_ml_source1_concat_value34)
                            }
                        };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (insertion, index) = (insertion_2, index_2);
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
            insertion
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_java_script_record_property_order_loop32(chunks: &[Vec<f64>], mut insertion: Vec<f64>, mut chunk: f64) -> Vec<f64> {
    loop {
        return if (chunk < ((chunks.len() as f64))) {
            {
                let values = self::read_java_script_record_order_name(chunks, chunk);
                {
                    let index = 0f64;
                    {
                        let insertion_2 = self::source_ml_temporary_merge_java_script_record_property_order_loop33(insertion, &(values), index);
                        {
                            let chunk_2 = (chunk + 1f64);
                            {
                                (insertion, chunk) = (insertion_2, chunk_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            insertion
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result {
    MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Done,
    MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_rebase_error_loop34(target: &[Vec<f64>], source: &[Vec<f64>], mut index: f64) -> self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result {
    loop {
        return if (index < ((source.len() as f64))) {
            if ((self::read_record_field_position(target, &(self::read_java_script_record_order_name(source, index)))) < (0f64)) {
                self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 112, 114, 101, 97, 100, 32, 102, 105, 101, 108, 100, 32, 105, 115, 32, 109, 105, 115, 115, 105, 110, 103, 32, 102, 114, 111, 109, 32, 105, 116, 115, 32, 116, 97, 114, 103, 101, 116]))
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
            self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_rebase_java_script_record_property_order_loop35(target: &[Vec<f64>], source: &[Vec<f64>], order: &[f64], mut result: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((order.len() as f64))) {
            {
                let name = self::read_java_script_record_order_name(source, self::read_java_script_record_order_position(order, index));
                {
                    let field = self::read_record_field_position(target, &(name));
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value35 = result;
                            {
                                let source_ml_source1_concat_value36 = vec![field];
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value35), source_ml_source1_concat_value36)
                            }
                        };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (result, index) = (result_2, index_2);
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_copy_java_script_record_property_order_loop37(target: &[Vec<f64>], source: &[Vec<f64>], order: &[f64], mut result: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((order.len() as f64))) {
            {
                let field = self::read_record_field_position(target, &(self::read_java_script_record_order_name(source, self::read_java_script_record_order_position(order, index))));
                if (field >= (0f64)) {
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value37 = result;
                            {
                                let source_ml_source1_concat_value38 = vec![field];
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value37), source_ml_source1_concat_value38)
                            }
                        };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (result, index) = (result_2, index_2);
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
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCopyJavaScriptRecordPropertyOrderLoop36Result {
    MlTemporaryCopyJavaScriptRecordPropertyOrderLoop36Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_copy_java_script_record_property_order_loop36(target: &[Vec<f64>], source: &[Vec<f64>], mut index: f64) -> self::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop36Result {
    loop {
        return if (index < ((target.len() as f64))) {
            if ((self::read_record_field_position(source, &(self::read_java_script_record_order_name(target, index)))) < (0f64)) {
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 115, 116, 32, 116, 97, 114, 103, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 115, 117, 98, 115, 101, 116])).0, message_present: true }).raise()
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
            self::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop36Result::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop36Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_property_keys_loop38(names: &[Vec<f64>], order: &[f64], mut result: Vec<Vec<f64>>, mut index: f64) -> Vec<Vec<f64>> {
    loop {
        return if (index < ((order.len() as f64))) {
            {
                let result_2 = {
                    let source_ml_source1_concat_value39 = result;
                    {
                        let source_ml_source1_concat_value40 = vec![self::read_java_script_record_order_name(names, self::read_java_script_record_order_position(order, index))];
                        self::ml_array::append(self::ml_array::append(Vec::<Vec<f64>>::new(), source_ml_source1_concat_value39), source_ml_source1_concat_value40)
                    }
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        (result, index) = (result_2, index_2);
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
pub fn source_ml_temporary_read_java_script_record_property_key_texts_loop40(units: &[f64], mut key: self::ml_string::Text, mut at: f64) -> self::ml_string::Text {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let key_2 = key.concat(&{
                    let source_ml_source1_character_argument41 = self::read_java_script_record_order_position(units, at);
                    self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source1_character_argument41]).into_iter().map(|ml_unit| ml_unit as u16).collect())
                });
                {
                    let at_2 = (at + 1f64);
                    {
                        (key, at) = (key_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            key.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_property_key_texts_loop39(rows: &[Vec<f64>], mut result: Vec<self::ml_string::Text>, mut index: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (index < ((rows.len() as f64))) {
            {
                let units = self::read_java_script_record_order_name(rows, index);
                {
                    let key = self::ml_string::Text::from_units(vec![]);
                    {
                        let at = 0f64;
                        {
                            let key_2 = self::source_ml_temporary_read_java_script_record_property_key_texts_loop40(&(units), key.clone(), at);
                            {
                                let result_2 = {
                                    let source_ml_source1_concat_value42 = result;
                                    {
                                        let source_ml_source1_concat_value43 = vec![key_2.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value42), source_ml_source1_concat_value43)
                                    }
                                };
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        (result, index) = (result_2, index_2);
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
pub fn source_ml_temporary_read_java_script_record_property_index_loop41(key: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (key.length())) {
            {
                let units_2 = {
                    let source_ml_source1_concat_value53 = units;
                    {
                        let source_ml_source1_concat_value54 = vec![{
                            let source_ml_source1_string_unit_operand44 = key.clone();
                            {
                                let source_ml_source1_string_unit_operand45 = index;
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
                                            let source_ml_source1_string_unit_operand51 = {
                                                let source_ml_source1_string_range_operand46 = text.clone();
                                                {
                                                    let source_ml_source1_string_range_operand47 = index_2;
                                                    {
                                                        let source_ml_source1_string_range_operand48 = (index_2 + 1f64);
                                                        {
                                                            let source_ml_source1_string_range_position49 = source_ml_source1_string_range_operand47;
                                                            {
                                                                let source_ml_source1_string_range_position50 = source_ml_source1_string_range_operand48;
                                                                source_ml_source1_string_range_operand46.slice(source_ml_source1_string_range_position49, source_ml_source1_string_range_position50)
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            {
                                                let source_ml_source1_string_unit_operand52 = 0f64;
                                                source_ml_source1_string_unit_operand51.code_point_at(source_ml_source1_string_unit_operand52)
                                            }
                                        }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                } }))(source_ml_source1_string_unit_operand44.clone(), source_ml_source1_string_unit_operand45)
                            }
                        }];
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value53), source_ml_source1_concat_value54)
                    }
                };
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_dense_property_key_texts_loop42(mut length: f64, mut result: Vec<self::ml_string::Text>, mut index: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (index < length) {
            {
                let result_2 = {
                    let source_ml_source1_concat_value56 = result;
                    {
                        let source_ml_source1_concat_value57 = vec![{
                            let source_ml_source1_string_argument55 = index;
                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument55))
                        }];
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value56), source_ml_source1_concat_value57)
                    }
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        (result, index) = (result_2, index_2);
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
pub fn source_ml_temporary_create_java_script_string_own_property_values_loop43(text: &self::ml_string::Text, mut result: Vec<self::ml_string::Text>, mut index: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (index < (text.length())) {
            {
                let result_2 = {
                    let source_ml_source1_concat_value65 = result;
                    {
                        let source_ml_source1_concat_value66 = vec![{
                            let source_ml_source1_string_unit_operand58 = text.clone();
                            {
                                let source_ml_source1_string_unit_operand59 = index;
                                (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if ((index_2 < (0f64)) || (index_2 >= (text_2.length()))) {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        {
                                            let source_ml_source1_string_range_operand60 = text_2.clone();
                                            {
                                                let source_ml_source1_string_range_operand61 = index_2;
                                                {
                                                    let source_ml_source1_string_range_operand62 = (index_2 + 1f64);
                                                    {
                                                        let source_ml_source1_string_range_position63 = source_ml_source1_string_range_operand61;
                                                        {
                                                            let source_ml_source1_string_range_position64 = source_ml_source1_string_range_operand62;
                                                            source_ml_source1_string_range_operand60.slice(source_ml_source1_string_range_position63, source_ml_source1_string_range_position64)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand58.clone(), source_ml_source1_string_unit_operand59)
                            }
                        }];
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value65), source_ml_source1_concat_value66)
                    }
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}
}
