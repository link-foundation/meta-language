// Generated from complete js/src/translation/rust-type-alias-rules.js. Do not edit by hand.
use super::MlTemporaryReadRustTypeAliasLookupKeysJoin4;
use super::source_ml_temporary_read_rust_type_alias_access_prefix_lengths_loop6;
use super::source_ml_temporary_read_rust_type_alias_declaration_plan_loop2;
use super::source_ml_temporary_read_rust_type_alias_lookup_keys_loop5;

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

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        #[must_use]
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        #[must_use]
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self {
                class_name: class_name.to_owned(),
                message_units: message.encode_utf16().collect(),
                message_present: true,
            }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Number fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required text fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Boolean fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Number fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required text fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Boolean fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn group_after(kinds: &[String], values: &[String], position: f64) -> f64 {
    {
        let open = self::read_required_text_fact(values, position);
        {
            let stack = vec![if open == "{" {
                String::from("}")
            } else if open == "[" {
                String::from("]")
            } else {
                String::from(")")
            }];
            {
                let at = position + 1f64;
                {
                    let source_ml_source1_s1 =
                        self::source_ml_temporary_group_after_loop1(kinds, values, stack, at);
                    match source_ml_source1_s1 {
                        self::MlTemporaryGroupAfterLoop1Result::MlTemporaryGroupAfterLoop1Done => {
                            -1f64
                        }
                        self::MlTemporaryGroupAfterLoop1Result::MlTemporaryGroupAfterLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_declaration_plan(kinds: &[String], values: &[String]) -> Vec<String> {
    {
        let ml_library_inequality_1 = (kinds.len() as f64) != (values.len() as f64);
        if ml_library_inequality_1 {
            vec![
                String::from("error"),
                String::from("unaligned source type alias columns"),
            ]
        } else {
            {
                let result = vec![String::from("aliases")];
                {
                    let names = Vec::<String>::new();
                    {
                        let owners = Vec::<String>::new();
                        {
                            let closes = Vec::<f64>::new();
                            {
                                let position = 0f64;
                                {
                                    let source_ml_source1_s2 = self::source_ml_temporary_read_rust_type_alias_declaration_plan_loop2(kinds, values, result, names, owners, closes, position);
                                    match source_ml_source1_s2 {
                                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Done(result_2) => {
                                        result_2
                                    }
                                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(source_ml_temporary_result2) => {
                                        source_ml_temporary_result2
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
pub fn read_rust_type_alias_lookup_keys(segments: &[String], context: &[String]) -> Vec<String> {
    {
        let names = segments.to_vec();
        {
            let start = context.to_vec();
            {
                let source_ml_source1_s3 = if ((self::ml_array::optional_at(
                    &names,
                    self::ml_array::number_index(0f64),
                ))
                .unwrap_or_else(|| String::new()))
                    == "crate"
                {
                    {
                        let names_2 = self::ml_array::slice(&names, 1f64, f64::INFINITY);
                        {
                            let start_2 = Vec::<String>::new();
                            self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Next(names_2, start_2)
                        }
                    }
                } else if ((self::ml_array::optional_at(
                    &names,
                    self::ml_array::number_index(0f64),
                ))
                .unwrap_or_else(|| String::new()))
                    == "self"
                {
                    {
                        let names_3 = self::ml_array::slice(&names, 1f64, f64::INFINITY);
                        self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Next(names_3, start)
                    }
                } else if ((self::ml_array::optional_at(
                    &names,
                    self::ml_array::number_index(0f64),
                ))
                .unwrap_or_else(|| String::new()))
                    == "super"
                {
                    {
                        let count = 0f64;
                        {
                            let count_2 =
                                self::source_ml_temporary_read_rust_type_alias_lookup_keys_loop5(
                                    &(names),
                                    count,
                                );
                            if count_2 > (start.len() as f64) {
                                self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Return(Vec::<String>::new())
                            } else {
                                {
                                    let names_4 =
                                        self::ml_array::slice(&names, count_2, f64::INFINITY);
                                    {
                                        let start_3 = self::ml_array::slice(
                                            &start,
                                            0f64,
                                            ((start.len() as f64) - count_2),
                                        );
                                        self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Next(names_4, start_3)
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Next(names, start)
                };
                match source_ml_source1_s3 {
                    self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Next(names_5, start_4) => {
                        if (names_5.len() as f64) == 0f64 {
                            Vec::<String>::new()
                        } else {
                            vec![({ let (ml_append_right, ml_append_left) = (names_5, start_4); self::ml_array::append(ml_append_left, ml_append_right) }).join((String::from(".")).as_str())]
                        }
                    }
                    self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Return(source_ml_temporary_result4) => {
                        source_ml_temporary_result4
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_expansion_error(
    depth: f64,
    expansions: f64,
    recursive: bool,
) -> String {
    if recursive {
        String::from("recursive source type alias")
    } else if depth >= 128f64 {
        String::from("source type alias depth limit exceeded")
    } else if expansions >= 4096f64 {
        String::from("source type alias expansion limit exceeded")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_clone_error(current: f64, needed: f64) -> String {
    if needed > (262_144f64 - current) {
        String::from("source type alias materialization limit exceeded")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_access_prefix_lengths(count: f64, bare: bool) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let length = if bare { count } else { count - 1f64 };
            {
                self::source_ml_temporary_read_rust_type_alias_access_prefix_lengths_loop6(
                    result, length,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_type_alias_nominal_access(
    kind: &str,
    wrappers: f64,
    suffix: f64,
    role: &str,
    syntax: &str,
) -> bool {
    ((kind == "named") && (wrappers == 0f64))
        && (((suffix > 0f64) || (role == "implementation owner"))
            || ((role == "braced record") && (syntax == "named")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_namespace_disposition(alias: bool, owner: &str) -> String {
    if alias {
        String::from("alias")
    } else if ((owner == "data") || (owner == "module")) || (owner == "externalCrate") {
        String::from("block")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_value_constructor_error(
    local: bool,
    value: bool,
    alias: bool,
) -> String {
    if (!local && !value) && alias {
        String::from("a source type alias has no independent value constructor")
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryGroupAfterLoop1Result {
    MlTemporaryGroupAfterLoop1Done,
    MlTemporaryGroupAfterLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_group_after_loop1(
    kinds: &[String],
    values: &[String],
    mut stack: Vec<String>,
    mut at: f64,
) -> self::MlTemporaryGroupAfterLoop1Result {
    loop {
        return if at < (values.len() as f64) {
            if (self::read_required_text_fact(kinds, at)) != "punct" {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                {
                    let value = self::read_required_text_fact(values, at);
                    if ((value == "{") || (value == "[")) || (value == "(") {
                        {
                            let stack_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![if value == "{" {
                                        String::from("}")
                                    } else if value == "[" {
                                        String::from("]")
                                    } else {
                                        String::from(")")
                                    }],
                                    stack,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let at_3 = at + 1f64;
                                {
                                    (stack, at) = (stack_2, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if ((value == "}") || (value == "]")) || (value == ")") {
                            if value
                                == (self::read_required_text_fact(
                                    &(stack),
                                    ((stack.len() as f64) - 1f64),
                                ))
                            {
                                {
                                    let stack_3 = self::ml_array::slice(&stack, 0f64, -1f64);
                                    if (stack_3.len() as f64) == 0f64 {
                                        self::MlTemporaryGroupAfterLoop1Result::MlTemporaryGroupAfterLoop1Return(at + 1f64)
                                    } else {
                                        {
                                            let at_4 = at + 1f64;
                                            {
                                                (stack, at) = (stack_3, at_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlTemporaryGroupAfterLoop1Result::MlTemporaryGroupAfterLoop1Return(-1f64)
                            }
                        } else {
                            {
                                let at_5 = at + 1f64;
                                {
                                    at = at_5;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryGroupAfterLoop1Result::MlTemporaryGroupAfterLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result {
    MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Done(Vec<String>),
    MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(Vec<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result {
    MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Done(f64),
    MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Return(Vec<String>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_type_alias_declaration_plan_loop3(
    kinds: &[String],
    values: &[String],
    mut end: f64,
) -> self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result {
    loop {
        return if end < (values.len() as f64) {
            {
                let boundary = self::read_required_text_fact(values, end);
                if ((self::read_required_text_fact(kinds, end)) == "punct") && (boundary == ";") {
                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Done(end)
                } else if ((self::read_required_text_fact(kinds, end)) == "punct")
                    && (((boundary == "{") || (boundary == "[")) || (boundary == "("))
                {
                    {
                        let after = self::group_after(kinds, values, end);
                        if after < 0f64 {
                            self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Return(vec![String::from("error"), String::from("unclosed source type alias group")])
                        } else {
                            {
                                let end_2 = after;
                                {
                                    end = end_2;
                                    continue;
                                }
                            }
                        }
                    }
                } else if ((self::read_required_text_fact(kinds, end)) == "eof")
                    || (((self::read_required_text_fact(kinds, end)) == "punct")
                        && (boundary == "}"))
                {
                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Return(vec![String::from("error"), String::from("unclosed source type alias declaration")])
                } else {
                    {
                        let end_3 = end + 1f64;
                        {
                            end = end_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Done(end)
        };
    }
}
