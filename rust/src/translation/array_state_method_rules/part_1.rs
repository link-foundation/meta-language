// Generated from complete js/src/translation/array-state-method-rules.js. Do not edit by hand.
/// JavaScript's Number operations that Rust's f64 does not share.
pub mod ml_number {
    /// ECMAScript `Number::toString`: the shortest decimal that reads back as
    /// the value, the even one on a tie, laid out as JavaScript prints it.
    #[must_use]
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
        let k = i32::try_from(digits.len()).expect("binary64 significand length fits i32");
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
    #[must_use]
    pub fn js_console(x: f64) -> String {
        if x == 0.0 && x.is_sign_negative() {
            String::from("-0")
        } else {
            js_number(x)
        }
    }

    /// `SameValue`, as `Object.is` and `assert.strictEqual` compare: NaN equals
    /// NaN, and 0 differs from -0.
    #[must_use]
    pub const fn same_value(a: f64, b: f64) -> bool {
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
    #[must_use]
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index.and_then(|index| values.get(index)).map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    const fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len()); let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
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
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values { if !out.iter().any(|previous| equal(previous, &value)) { out.push(value); } }
        out
    }
    #[must_use]
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
        #[must_use]
        pub fn from_text(text: &str) -> Self { Self(text.encode_utf16().collect()) }
        #[must_use]
        pub const fn from_units(units: Vec<u16>) -> Self { Self(units) }
        #[must_use]
        pub const fn length(&self) -> f64 { self.0.len() as f64 }
        #[must_use]
        pub fn concat(&self, other: &Self) -> Self {
            let mut units = self.0.clone(); units.extend_from_slice(&other.0); Self(units)
        }
        #[must_use]
        pub fn starts_with(&self, search: &Self) -> bool { self.0.starts_with(&search.0) }
        #[must_use]
        pub fn ends_with(&self, search: &Self) -> bool { self.0.ends_with(&search.0) }
        #[must_use]
        pub fn contains(&self, search: &Self) -> bool { self.index_of(search, 0.0) >= 0.0 }
        #[must_use]
        pub fn slice(&self, start: f64, end: f64) -> Self {
            let first = relative(start, self.0.len()); let last = relative(end, self.0.len());
            Self(self.0[first..last.max(first)].to_vec())
        }
        #[must_use]
        pub fn index_of(&self, search: &Self, position: f64) -> f64 {
            let start = absolute(position, self.0.len());
            if search.0.is_empty() { return start as f64; }
            if search.0.len() > self.0.len() { return -1.0; }
            (start..=self.0.len() - search.0.len()).find(|&index| self.0[index..].starts_with(&search.0)).map_or(-1.0, |index| index as f64)
        }
        #[must_use]
        pub fn last_index_of(&self, search: &Self, position: f64) -> f64 {
            let position = if position.is_nan() { f64::INFINITY } else { position };
            let start = absolute(position, self.0.len());
            if search.0.is_empty() { return start as f64; }
            if search.0.len() > self.0.len() { return -1.0; }
            (0..=start.min(self.0.len() - search.0.len())).rev().find(|&index| self.0[index..].starts_with(&search.0)).map_or(-1.0, |index| index as f64)
        }
        #[must_use]
        pub fn split(&self, separator: Option<&Self>, limit: f64) -> Vec<Self> {
            let maximum = uint32(limit) as usize;
            if maximum == 0 { return Vec::new(); }
            let Some(separator) = separator else { return vec![self.clone()]; };
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
        #[must_use]
        pub fn code_point_at(&self, position: f64) -> Option<f64> {
            let index = integer(position);
            if index < 0.0 || index >= self.0.len() as f64 { return None; }
            let index = index as usize; let first = self.0[index];
            if (0xd800..=0xdbff).contains(&first)
      && let Some(&second) = self.0.get(index + 1)
      && (0xdc00..=0xdfff).contains(&second) {
        return Some(f64::from((u32::from(first) - 0xd800) * 1024 + u32::from(second) - 0xdc00 + 0x10000));
      }
            Some(f64::from(first))
        }
        #[must_use]
        pub fn code_points(&self) -> Vec<Self> {
            let mut out = Vec::new(); let mut index = 0;
            while index < self.0.len() {
                let first = self.0[index];
                let width = if (0xd800..=0xdbff).contains(&first) && self.0.get(index + 1).is_some_and(|second| (0xdc00..=0xdfff).contains(second)) { 2 } else { 1 };
                out.push(Self(self.0[index..index + width].to_vec())); index += width;
            }
            out
        }
        #[must_use]
        pub fn repeat(&self, count: f64) -> Self {
            let count = integer(count);
            assert!(count >= 0.0 && count.is_finite(), "Invalid count value: {count}");
            if self.0.is_empty() { return Self(Vec::new()); }
            let size = count * self.0.len() as f64;
            assert!(size <= 536_870_888.0, "Invalid string length");
            Self(self.0.repeat(count as usize))
        }
        #[must_use]
        pub fn pad(&self, length: f64, fill: &Self, start: bool) -> Self {
            let length = integer(length).max(0.0);
            if length <= self.0.len() as f64 || fill.0.is_empty() { return self.clone(); }
            assert!(length <= 536_870_888.0, "Invalid string length");
            let count = length as usize - self.0.len();
            let padding: Vec<u16> = fill.0.iter().copied().cycle().take(count).collect();
            let pad = Self(padding); if start { pad.concat(self) } else { self.concat(&pad) }
        }
        #[must_use]
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
        #[must_use]
        pub fn trim(&self, start: bool, end: bool, rust: bool) -> Self {
            let whitespace = |unit: u16| char::from_u32(u32::from(unit)).is_some_and(|c| if rust { c.is_whitespace() } else { (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}' });
            let mut first = 0; let mut last = self.0.len();
            if start { while first < last && whitespace(self.0[first]) { first += 1; } }
            if end { while last > first && whitespace(self.0[last - 1]) { last -= 1; } }
            Self(self.0[first..last].to_vec())
        }
        #[must_use]
        pub fn map_case(&self, upper: bool) -> Self {
            let mut out = Vec::new(); let mut run = String::new();
            let flush = |run: &mut String, out: &mut Vec<u16>| { let mapped = if upper { run.to_uppercase() } else { run.to_lowercase() }; out.extend(mapped.encode_utf16()); run.clear(); };
            for scalar in char::decode_utf16(self.0.iter().copied()) {
                match scalar { Ok(c) => run.push(c), Err(error) => { flush(&mut run, &mut out); out.push(error.unpaired_surrogate()); } }
            }
            flush(&mut run, &mut out); Self(out)
        }
    }
    #[must_use]
    pub fn join(values: &[Text], separator: &Text) -> Text {
        let mut units = Vec::new();
        for (index, value) in values.iter().enumerate() { if index != 0 { units.extend_from_slice(&separator.0); } units.extend_from_slice(&value.0); }
        Text(units)
    }
    #[must_use]
    pub const fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    #[must_use]
    pub const fn absolute(value: f64, length: usize) -> usize { integer(value).max(0.0).min(length as f64) as usize }
    #[must_use]
    pub fn relative(value: f64, length: usize) -> usize { let value = integer(value); if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize } }
    fn uint32(value: f64) -> u32 { if !value.is_finite() || value == 0.0 { 0 } else { value.trunc().rem_euclid(4_294_967_296.0) as u32 } }
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_result(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if (*operation) == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101])) {
        self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 100, 32, 97, 114, 114, 97, 121])
    } else if ((*operation) == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110]))) {
            self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_array_state_method(expression: &self::ml_string::Text, operation: &self::ml_string::Text, kind: &self::ml_string::Text, reference: bool) -> bool {
    ((((*expression) == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) && ((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) && reference) && ((self::read_array_state_method_result(operation)) != (self::ml_string::Text::from_units(vec![])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_required_arguments(operation: &self::ml_string::Text) -> f64 {
    if (*operation) == (self::ml_string::Text::from_units(vec![102, 105, 108, 108])) {
        1f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_argument_role(operation: &self::ml_string::Text, position: f64) -> self::ml_string::Text {
    if (*operation) == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101])) {
        if position < 2f64 {
            self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 105, 110, 100, 101, 120])
        } else {
            self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
        }
    } else if (*operation) == (self::ml_string::Text::from_units(vec![102, 105, 108, 108])) {
            if position == 0f64 {
                self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
            } else if position == 1f64 {
                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 105, 110, 100, 101, 120])
                } else if position == 2f64 {
                        self::ml_string::Text::from_units(vec![101, 110, 100, 32, 105, 110, 100, 101, 120])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
        } else if (*operation) == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110])) {
                if position < 2f64 {
                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 105, 110, 100, 101, 120])
                } else if position == 2f64 {
                        self::ml_string::Text::from_units(vec![101, 110, 100, 32, 105, 110, 100, 101, 120])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
            } else {
                self::ml_string::Text::from_units(vec![])
            }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_argument_profile(kind: &self::ml_string::Text, element: &self::ml_string::Text, absence: &self::ml_string::Text) -> self::ml_string::Text {
    if (((*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || ((*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))) || ((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
        kind.clone()
    } else if (((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && ((((*element) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || ((*element) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))) || ((*element) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))) && ((((*absence) == (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))) || ((*absence) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))) || ((*absence) == (self::ml_string::Text::from_units(vec![98, 111, 116, 104])))) {
            (((self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 32])).concat(absence)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(element)
        } else {
            self::ml_string::Text::from_units(vec![])
        }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_suffix(initial: f64, receiver: &self::ml_string::Text, arguments_: &[self::ml_string::Text]) -> f64 {
    {
        let text = (receiver.concat(&self::ml_string::Text::from_units(vec![32]))).concat(&self::ml_string::join(arguments_, &self::ml_string::Text::from_units(vec![32])));
        {
            let suffix = initial;
            {
                self::ml_read_array_state_method_suffix_loop1(&(text), suffix)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_state_numeric_value(profile: &self::ml_string::Text, value: &self::ml_string::Text, undefined_value: &self::ml_string::Text) -> self::ml_string::Text {
    if (*profile) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])) {
        value.clone()
    } else if (*profile) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
            ((self::ml_string::Text::from_units(vec![105, 102, 32])).concat(value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 49, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 48, 102, 54, 52, 32, 125]))
        } else if (*profile) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])) {
                undefined_value.clone()
            } else {
                {
                    let parts = profile.split(Some(&self::ml_string::Text::from_units(vec![32])), 4_294_967_295f64);
                    {
                        let absence = (self::ml_array::optional_at(&parts, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let element = (self::ml_array::optional_at(&parts, self::ml_array::number_index(2f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            {
                                let present = if element == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])) {
                                    self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110, 116])
                                } else if element == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
                                        self::ml_string::Text::from_units(vec![105, 102, 32, 109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110, 116, 32, 123, 32, 49, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 48, 102, 54, 52, 32, 125])
                                    } else {
                                        self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78])
                                    };
                                if absence == (self::ml_string::Text::from_units(vec![98, 111, 116, 104])) {
                                    ((((((self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104, 32])).concat(value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 99, 114, 97, 116, 101, 58, 58, 77, 108, 79, 112, 116, 105, 111, 110, 97, 108, 58, 58, 86, 97, 108, 117, 101, 40, 109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110, 116, 41, 32, 61, 62, 32]))).concat(&present)).concat(&self::ml_string::Text::from_units(vec![44, 32, 99, 114, 97, 116, 101, 58, 58, 77, 108, 79, 112, 116, 105, 111, 110, 97, 108, 58, 58, 78, 117, 108, 108, 32, 61, 62, 32, 48, 102, 54, 52, 44, 32, 99, 114, 97, 116, 101, 58, 58, 77, 108, 79, 112, 116, 105, 111, 110, 97, 108, 58, 58, 85, 110, 100, 101, 102, 105, 110, 101, 100, 32, 61, 62, 32]))).concat(undefined_value)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                } else {
                                    ((((((self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104, 32])).concat(value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40, 109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110, 116, 41, 32, 61, 62, 32]))).concat(&present)).concat(&self::ml_string::Text::from_units(vec![44, 32, 78, 111, 110, 101, 32, 61, 62, 32]))).concat(&if absence == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])) {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_state_relative_index(value: &self::ml_string::Text, length: &self::ml_string::Text) -> self::ml_string::Text {
    (((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 95, 118, 97, 108, 117, 101, 58, 32, 102, 54, 52, 32, 61, 32])).concat(value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 32, 61, 32, 105, 102, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 95, 118, 97, 108, 117, 101, 46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 123, 32, 48, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 95, 118, 97, 108, 117, 101, 46, 116, 114, 117, 110, 99, 40, 41, 32, 125, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 98, 111, 117, 110, 100, 32, 61, 32]))).concat(length)).concat(&self::ml_string::Text::from_units(vec![32, 97, 115, 32, 102, 54, 52, 59, 32, 40, 105, 102, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 32, 60, 32, 48, 102, 54, 52, 32, 123, 32, 40, 109, 108, 95, 98, 111, 117, 110, 100, 32, 43, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 41, 46, 99, 108, 97, 109, 112, 40, 48, 102, 54, 52, 44, 32, 109, 108, 95, 98, 111, 117, 110, 100, 41, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 46, 109, 105, 110, 40, 109, 108, 95, 98, 111, 117, 110, 100, 41, 32, 125, 41, 32, 97, 115, 32, 117, 115, 105, 122, 101, 32, 125]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_identity(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if (*operation) == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101])) {
        self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115])
    } else if (*operation) == (self::ml_string::Text::from_units(vec![102, 105, 108, 108])) {
            self::ml_string::Text::from_units(vec![102, 105, 108, 108, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115])
        } else if (*operation) == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110])) {
                self::ml_string::Text::from_units(vec![99, 111, 112, 121, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115, 45, 119, 105, 116, 104, 105, 110])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![68, 101, 110, 115, 101, 32, 115, 104, 97, 114, 101, 100, 32, 97, 114, 114, 97, 121, 32, 114, 97, 110, 103, 101, 32, 109, 117, 116, 97, 116, 105, 111, 110, 115, 32, 101, 118, 97, 108, 117, 97, 116, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 101, 118, 101, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 111, 110, 99, 101, 32, 105, 110, 32, 111, 114, 100, 101, 114, 44, 32, 100, 105, 115, 116, 105, 110, 103, 117, 105, 115, 104, 32, 109, 105, 115, 115, 105, 110, 103, 32, 97, 110, 100, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 44, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 32, 78, 117, 109, 98, 101, 114, 32, 105, 110, 100, 105, 99, 101, 115, 32, 119, 105, 116, 104, 111, 117, 116, 32, 110, 97, 114, 114, 111, 119, 105, 110, 103, 32, 98, 101, 102, 111, 114, 101, 32, 98, 111, 117, 110, 100, 115, 44, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 97, 110, 100, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 101, 108, 101, 109, 101, 110, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 115, 44, 32, 97, 110, 100, 32, 99, 111, 112, 121, 32, 111, 118, 101, 114, 108, 97, 112, 112, 105, 110, 103, 32, 114, 97, 110, 103, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 100, 105, 114, 101, 99, 116, 105, 111, 110, 59, 32, 115, 112, 108, 105, 99, 101, 32, 114, 101, 116, 117, 114, 110, 115, 32, 97, 32, 102, 114, 101, 115, 104, 32, 114, 101, 109, 111, 118, 101, 100, 32, 97, 114, 114, 97, 121, 46])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_state_method_expression(target: &self::ml_string::Text, operation: &self::ml_string::Text, receiver: &self::ml_string::Text, arguments_: &[self::ml_string::Text], profiles: &[self::ml_string::Text], types: &[self::ml_string::Text], suffix: &self::ml_string::Text) -> self::ml_string::Text {
    if (self::read_array_state_method_result(operation)) == (self::ml_string::Text::from_units(vec![])) {
        self::ml_string::Text::from_units(vec![])
    } else if (*target) == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116])) {
            ((((((self::ml_string::Text::from_units(vec![40])).concat(receiver)).concat(&self::ml_string::Text::from_units(vec![41, 46]))).concat(operation)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&self::ml_string::join(arguments_, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
let ml_library_inequality_1 = (*target) != (self::ml_string::Text::from_units(vec![82, 117, 115, 116])) ;
if ml_library_inequality_1 {
                self::ml_string::Text::from_units(vec![])
            } else {
                {
                    let owner = (self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 111, 119, 110, 101, 114])).concat(suffix);
                    {
                        let output = ((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(receiver)).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                        {
                            let index = 0f64;
                            {
                                let output_2 = self::ml_render_array_state_method_expression_loop2(arguments_, types, suffix, output, index);
                                {
                                    let names = Vec::<self::ml_string::Text>::new();
                                    {
                                        let index_2 = 0f64;
                                        {
                                            let names_2 = self::ml_render_array_state_method_expression_loop3(arguments_, suffix, names, index_2);
                                            {
                                                let output_3 = output_2.concat(&((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 61, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 46, 108, 101, 110, 40, 41, 59, 32])));
                                                if (*operation) == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101])) {
                                                    {
                                                        let begin = if (arguments_.len() as f64) == 0f64 {
                                                            self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                                                        } else {
                                                            self::render_array_state_numeric_value(&((self::ml_array::optional_at(profiles, self::ml_array::number_index(0f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &((self::ml_array::optional_at(&names_2, self::ml_array::number_index(0f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &(self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78])))
                                                        };
                                                        {
                                                            let output_4 = output_3.concat(&((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 61, 32])).concat(&self::render_array_state_relative_index(&(begin), &(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104]))))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                            {
                                                                let count = if (arguments_.len() as f64) == 0f64 {
                                                                    self::ml_string::Text::from_units(vec![48, 117, 115, 105, 122, 101])
                                                                } else if (arguments_.len() as f64) == 1f64 {
                                                                        self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116])
                                                                    } else {
                                                                        ((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 108, 95, 100, 101, 108, 101, 116, 101, 95, 118, 97, 108, 117, 101, 58, 32, 102, 54, 52, 32, 61, 32])).concat(&self::render_array_state_numeric_value(&((self::ml_array::optional_at(profiles, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &((self::ml_array::optional_at(&names_2, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &(self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78]))))).concat(&self::ml_string::Text::from_units(vec![59, 32, 109, 108, 95, 100, 101, 108, 101, 116, 101, 95, 118, 97, 108, 117, 101, 46, 116, 114, 117, 110, 99, 40, 41, 46, 109, 97, 120, 40, 48, 102, 54, 52, 41, 46, 109, 105, 110, 40, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 41, 32, 97, 115, 32, 102, 54, 52, 41, 32, 97, 115, 32, 117, 115, 105, 122, 101, 32, 125]))
                                                                    };
                                                                {
                                                                    let inserted = Vec::<self::ml_string::Text>::new();
                                                                    {
                                                                        let index_3 = 2f64;
                                                                        {
                                                                            let inserted_2 = self::ml_render_array_state_method_expression_loop4(arguments_, &(names_2), inserted, index_3);
                                                                            (((((((((output_4.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 61, 32]))).concat(&count)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 115, 101, 114, 116, 101, 100, 32, 61, 32, 118, 101, 99, 33, 91]))).concat(&self::ml_string::join(&inserted_2, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![93, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 95, 32, 61, 32, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 41, 46, 99, 104, 101, 99, 107, 101, 100, 95, 97, 100, 100, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 115, 101, 114, 116, 101, 100, 46, 108, 101, 110, 40, 41, 41, 46, 102, 105, 108, 116, 101, 114, 40, 124, 108, 101, 110, 103, 116, 104, 124, 32, 42, 108, 101, 110, 103, 116, 104, 32, 60, 61, 32, 52, 50, 57, 52, 57, 54, 55, 50, 57, 53, 117, 115, 105, 122, 101, 41, 46, 101, 120, 112, 101, 99, 116, 40, 34, 73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 34, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 114, 101, 109, 111, 118, 101, 100, 58, 32, 86, 101, 99, 60, 95, 62, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 46, 115, 112, 108, 105, 99, 101, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 44, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 115, 101, 114, 116, 101, 100, 41, 46, 99, 111, 108, 108, 101, 99, 116, 40, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 114, 99, 58, 58, 82, 99, 58, 58, 110, 101, 119, 40, 115, 116, 100, 58, 58, 99, 101, 108, 108, 58, 58, 82, 101, 102, 67, 101, 108, 108, 58, 58, 110, 101, 119, 40, 109, 108, 95, 114, 101, 109, 111, 118, 101, 100, 41, 41, 32, 125]))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let first = if (*operation) == (self::ml_string::Text::from_units(vec![102, 105, 108, 108])) {
                                                            1f64
                                                        } else {
                                                            0f64
                                                        };
                                                        {
                                                            let start_value = if (arguments_.len() as f64) <= first {
                                                                self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                                                            } else {
                                                                self::render_array_state_numeric_value(&((self::ml_array::optional_at(profiles, self::ml_array::number_index(first))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &((self::ml_array::optional_at(&names_2, self::ml_array::number_index(first))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &(self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78])))
                                                            };
                                                            {
                                                                let output_5 = output_3.concat(&((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 61, 32])).concat(&self::render_array_state_relative_index(&(start_value), &(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104]))))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                                {
                                                                    let end_value = if (arguments_.len() as f64) <= 2f64 {
                                                                        self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 97, 115, 32, 102, 54, 52])
                                                                    } else {
                                                                        self::render_array_state_numeric_value(&((self::ml_array::optional_at(profiles, self::ml_array::number_index(2f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &((self::ml_array::optional_at(&names_2, self::ml_array::number_index(2f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 97, 115, 32, 102, 54, 52])))
                                                                    };
                                                                    {
                                                                        let end = ((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 32, 61, 32])).concat(&self::render_array_state_relative_index(&(end_value), &(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104]))))).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                                                                        if (*operation) == (self::ml_string::Text::from_units(vec![102, 105, 108, 108])) {
                                                                            ((((((((output_5.concat(&end)).concat(&self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 117, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![105, 102, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 60, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 32, 123, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 93, 46, 102, 105, 108, 108, 40]))).concat(&(self::ml_array::optional_at(&names_2, self::ml_array::number_index(0f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![41, 59, 32, 125, 32, 125, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                                                        } else {
                                                                            {
                                                                                let from_value = if (arguments_.len() as f64) <= 1f64 {
                                                                                    self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                                                                                } else {
                                                                                    self::render_array_state_numeric_value(&((self::ml_array::optional_at(profiles, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &((self::ml_array::optional_at(&names_2, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &(self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78])))
                                                                                };
                                                                                (((((((((((output_5.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 61, 32]))).concat(&self::render_array_state_relative_index(&(from_value), &(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104]))))).concat(&self::ml_string::Text::from_units(vec![59, 32]))).concat(&end)).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 61, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 46, 115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 115, 117, 98, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 41, 46, 109, 105, 110, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 117, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32, 105, 102, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 60, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 38, 38, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 60, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 123, 32]))).concat(&self::ml_string::Text::from_units(vec![102, 111, 114, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 32, 105, 110, 32, 40, 48, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 41, 46, 114, 101, 118, 40, 41, 32, 123, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 32, 61, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 46, 99, 108, 111, 110, 101, 40, 41, 59, 32, 125, 32]))).concat(&self::ml_string::Text::from_units(vec![125, 32, 101, 108, 115, 101, 32, 123, 32, 102, 111, 114, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 32, 105, 110, 32, 48, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 123, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 32, 61, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 46, 99, 108, 111, 110, 101, 40, 41, 59, 32, 125, 32, 125, 32, 125, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
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
pub fn ml_read_array_state_method_suffix_loop1(text: &self::ml_string::Text, mut suffix: f64) -> f64 {
    loop {
        if text.contains(&(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 111, 119, 110, 101, 114])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix)))) || text.contains(&((self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix)))).concat(&self::ml_string::Text::from_units(vec![95]))) {
            {
                let suffix_2 = suffix + 1f64;
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        }
return {
            suffix
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_array_state_method_expression_loop2(arguments_: &[self::ml_string::Text], types: &[self::ml_string::Text], suffix: &self::ml_string::Text, mut output: self::ml_string::Text, mut index: f64) -> self::ml_string::Text {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let output_2 = output.concat(&((((((((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(suffix)).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(index)))).concat(&self::ml_string::Text::from_units(vec![58, 32]))).concat(&(self::ml_array::optional_at(types, self::ml_array::number_index(index))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&(self::ml_array::optional_at(arguments_, self::ml_array::number_index(index))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                {
                    let index_2 = index + 1f64;
                    {
                        (output, index) = (output_2, index_2);
                        continue;
                    }
                }
            }
        }
return {
            output.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_array_state_method_expression_loop3(arguments_: &[self::ml_string::Text], suffix: &self::ml_string::Text, mut names: Vec<self::ml_string::Text>, mut index: f64) -> Vec<self::ml_string::Text> {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let names_2 = { let (ml_append_right, ml_append_left) = (vec![(((self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(suffix)).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(index)))], names); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = index + 1f64;
                    {
                        (names, index) = (names_2, index_2);
                        continue;
                    }
                }
            }
        }
return {
            names
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_array_state_method_expression_loop4(arguments_: &[self::ml_string::Text], names: &[self::ml_string::Text], mut inserted: Vec<self::ml_string::Text>, mut index: f64) -> Vec<self::ml_string::Text> {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let inserted_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(names, self::ml_array::number_index(index))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))], inserted); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = index + 1f64;
                    {
                        (inserted, index) = (inserted_2, index_2);
                        continue;
                    }
                }
            }
        }
return {
            inserted
        };
    }
}
