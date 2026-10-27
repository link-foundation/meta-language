// Generated from complete js/src/translation/rust-byte-ascii-rules.js. Do not edit by hand.
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
            let other = if digits == down {
                increment(&down)
            } else {
                Some(down)
            };
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
        if x < 0.0 { format!("-{body}") } else { body }
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
        let digits = if digits.is_empty() {
            String::from("0")
        } else {
            digits
        };
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
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
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
    const fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> f64 {
        if values.is_empty() {
            return -1.0;
        }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) {
                return -1.0;
            }
            let start = if from < 0.0 {
                (values.len() as f64 + from) as usize
            } else {
                from.min((values.len() - 1) as f64) as usize
            };
            (0..=start)
                .rev()
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 {
                (values.len() as f64 + from).max(0.0) as usize
            } else {
                from.min(values.len() as f64) as usize
            };
            (start..values.len())
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        }
    }
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) {
                out[index].1 = value;
            } else {
                out.push((key, value));
            }
        }
        out
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_byte_ascii(method: &str) -> Vec<String> {
    {
        let ranges = if method == "is_ascii" {
            vec![0f64, 127f64]
        } else if method == "is_ascii_alphabetic" {
            vec![65f64, 90f64, 97f64, 122f64]
        } else if method == "is_ascii_alphanumeric" {
            vec![48f64, 57f64, 65f64, 90f64, 97f64, 122f64]
        } else if method == "is_ascii_control" {
            vec![0f64, 31f64, 127f64, 127f64]
        } else if method == "is_ascii_digit" {
            vec![48f64, 57f64]
        } else if method == "is_ascii_graphic" {
            vec![33f64, 126f64]
        } else if method == "is_ascii_hexdigit" {
            vec![48f64, 57f64, 65f64, 70f64, 97f64, 102f64]
        } else if method == "is_ascii_lowercase" {
            vec![97f64, 122f64]
        } else if method == "is_ascii_punctuation" {
            vec![33f64, 47f64, 58f64, 64f64, 91f64, 96f64, 123f64, 126f64]
        } else if method == "is_ascii_uppercase" {
            vec![65f64, 90f64]
        } else if method == "is_ascii_whitespace" {
            vec![9f64, 10f64, 12f64, 13f64, 32f64, 32f64]
        } else {
            Vec::<f64>::new()
        };
        {
            let terms = Vec::<String>::new();
            {
                let at = 0f64;
                { self::source_ml_temporary_plan_rust_byte_ascii_loop1(&(ranges), terms, at) }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_byte_ascii_arity(method: &str, ordinary: bool) -> f64 {
    if ordinary && (((self::plan_rust_byte_ascii(method)).len() as f64) > 0f64) {
        0f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_byte_ascii_receiver_error(
    kind: &str,
    bits: &str,
    signed: bool,
    owner_kind: &str,
    owner: &str,
    children: f64,
    proven: bool,
    immutable: bool,
) -> String {
    if (((((((kind == "fixed") && (bits == "8")) && !signed) && (owner_kind == "primitive"))
        && (owner == "u8"))
        && (children == 0f64))
        && proven)
        && immutable
    {
        String::new()
    } else {
        String::from(
            "a byte predicate needs an actual standard u8 receiver with immutable source layers",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_byte_ascii_loop1(
    ranges: &[f64],
    mut terms: Vec<String>,
    mut at: f64,
) -> Vec<String> {
    loop {
        if at < (ranges.len() as f64) {
            {
                let terms_2 = {
                    let source_ml_source1_concat_value3 = terms;
                    {
                        let source_ml_source1_concat_value4 = vec![
                            String::from("operand"),
                            {
                                let source_ml_source1_string_argument1 =
                                    (self::ml_array::optional_at(
                                        ranges,
                                        self::ml_array::number_index(at),
                                    ))
                                    .unwrap_or_else(|| 0f64);
                                self::ml_number::js_number(source_ml_source1_string_argument1)
                            },
                            String::from("ge"),
                            String::from("operand"),
                            {
                                let source_ml_source1_string_argument2 =
                                    (self::ml_array::optional_at(
                                        ranges,
                                        self::ml_array::number_index(at + 1f64),
                                    ))
                                    .unwrap_or_else(|| 0f64);
                                self::ml_number::js_number(source_ml_source1_string_argument2)
                            },
                            String::from("le"),
                            String::from("and"),
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<String>::new(),
                                source_ml_source1_concat_value3,
                            ),
                            source_ml_source1_concat_value4,
                        )
                    }
                };
                if at > 0f64 {
                    {
                        let terms_3 = {
                            let source_ml_source1_concat_value5 = terms_2;
                            {
                                let source_ml_source1_concat_value6 = vec![String::from("or")];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source1_concat_value5,
                                    ),
                                    source_ml_source1_concat_value6,
                                )
                            }
                        };
                        {
                            let at_2 = at + 2f64;
                            {
                                (terms, at) = (terms_3, at_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let at_3 = at + 2f64;
                        {
                            (terms, at) = (terms_2, at_3);
                            continue;
                        }
                    }
                }
            }
        }
        return { terms };
    }
}
