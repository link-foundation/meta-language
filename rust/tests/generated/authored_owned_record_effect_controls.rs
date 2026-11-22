// Translated from JavaScript by meta-language: portable core, Rust target.
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

pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Number fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required text fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Boolean fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn read_required_number_fact(values: Vec<f64>, index: f64) -> f64 {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Number fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

pub fn read_required_text_fact(values: Vec<String>, index: f64) -> String {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required text fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

pub fn read_required_boolean_fact(values: Vec<bool>, index: f64) -> bool {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Boolean fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

pub fn read_conditional_chain_operation(role: String) -> String {
    if (role == "result") {
        String::from("readConditionalChainResult")
    } else {
        if (role == "success") {
            String::from("constructConditionalChainSuccess")
        } else {
            if (role == "failure") {
                String::from("constructConditionalChainFailure")
            } else {
                String::from("")
            }
        }
    }
}

pub fn accept_conditional_chain_operation(operation: String) -> bool {
    (((operation == (self::read_conditional_chain_operation(String::from("result")))) || (operation == (self::read_conditional_chain_operation(String::from("success"))))) || (operation == (self::read_conditional_chain_operation(String::from("failure")))))
}

pub fn read_conditional_chain_stage_limit() -> f64 {
    256f64
}

pub fn find_rust_conditional_chain_pattern(kinds: Vec<String>, values: Vec<String>, start: f64) -> f64 {
    if (((((kinds.len() as f64)) != ((values.len() as f64))) || !(self::ml_math::is_safe_integer(start))) || (start < (0f64))) {
        (-1f64)
    } else {
        {
            let closes = Vec::<String>::new();
            {
                let at = start;
                {
                    let source_ml_source1_s1 = self::source_ml_temporary_find_rust_conditional_chain_pattern_loop1(kinds, values, closes, at);
                    match source_ml_source1_s1.clone() {
                        self::MlTemporaryFindRustConditionalChainPatternLoop1Result::MlTemporaryFindRustConditionalChainPatternLoop1Done => {
                            (-1f64)
                        }
                        self::MlTemporaryFindRustConditionalChainPatternLoop1Result::MlTemporaryFindRustConditionalChainPatternLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1
                        }
                    }
                }
            }
        }
    }
}

pub fn plan_conditional_chain_layers(forms: Vec<f64>) -> Vec<f64> {
    if ((((forms.len() as f64)) < (1f64)) || (((forms.len() as f64)) > (self::read_conditional_chain_stage_limit()))) {
        Vec::<f64>::new()
    } else {
        {
            let result = Vec::<f64>::new();
            {
                let at = (forms.len() as f64);
                {
                    let source_ml_source1_s2 = self::source_ml_temporary_plan_conditional_chain_layers_loop2(forms, result, at);
                    match source_ml_source1_s2.clone() {
                        self::MlTemporaryPlanConditionalChainLayersLoop2Result::MlTemporaryPlanConditionalChainLayersLoop2Done(result_2) => {
                            result_2
                        }
                        self::MlTemporaryPlanConditionalChainLayersLoop2Result::MlTemporaryPlanConditionalChainLayersLoop2Return(source_ml_temporary_result2) => {
                            source_ml_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

pub fn plan_conditional_match_arms(guards: Vec<f64>) -> Vec<f64> {
    if ((((guards.len() as f64)) < (1f64)) || (((guards.len() as f64)) > (self::read_conditional_chain_stage_limit()))) {
        Vec::<f64>::new()
    } else {
        {
            let coverage = Vec::<f64>::new();
            {
                let layers = Vec::<f64>::new();
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s3 = self::source_ml_temporary_plan_conditional_match_arms_loop3(guards.clone(), coverage, at);
                        match source_ml_source1_s3.clone() {
                            self::MlTemporaryPlanConditionalMatchArmsLoop3Result::MlTemporaryPlanConditionalMatchArmsLoop3Done(coverage_2) => {
                                {
                                    let at_2 = (guards.len() as f64);
                                    {
                                        let layers_2 = self::source_ml_temporary_plan_conditional_match_arms_loop4(layers, at_2);
                                        {
                                            let source_ml_source1_concat_value4 = vec![(coverage_2.len() as f64)];
                                            {
                                                let source_ml_source1_concat_value5 = coverage_2;
                                                {
                                                    let source_ml_source1_concat_value6 = layers_2;
                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value4), source_ml_source1_concat_value5), source_ml_source1_concat_value6)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryPlanConditionalMatchArmsLoop3Result::MlTemporaryPlanConditionalMatchArmsLoop3Return(source_ml_temporary_result4) => {
                                source_ml_temporary_result4
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_expression_completion_form(kind: String, operation: String) -> String {
    if (kind == "rustStandardSumTry") {
        String::from("propagate")
    } else {
        if (kind == "lambda") {
            String::from("callable boundary")
        } else {
            if (kind == "if") {
                String::from("conditional")
            } else {
                if ((kind == "binary") && ((operation == "and") || (operation == "or"))) {
                    String::from("short circuit")
                } else {
                    if ((kind == "match") || (kind == "match1")) {
                        String::from("match")
                    } else {
                        if (kind == "let") {
                            String::from("binding")
                        } else {
                            if (kind == "print") {
                                String::from("print")
                            } else {
                                if ((kind == "math") && (operation == "rustOwnedRecordStateWrite")) {
                                    String::from("ordered")
                                } else {
                                    if ((kind == "math") && ((self::read_rust_expression_completion_packet_operation(operation.clone())) != "")) {
                                        String::from("packet")
                                    } else {
                                        if { let ml_value = ((self::read_rust_expression_completion_child_roles(kind.clone())).len() as f64); ml_value != 0f64 && !ml_value.is_nan() } {
                                            String::from("ordered")
                                        } else {
                                            String::from("opaque")
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

pub fn read_rust_expression_completion_child_roles(kind: String) -> Vec<String> {
    if (kind == "app") {
        vec![String::from("fn"), String::from("args")]
    } else {
        if (kind == "binary") {
            vec![String::from("left"), String::from("right")]
        } else {
            if ((kind == "unary") || (kind == "cast")) {
                vec![String::from("arg")]
            } else {
                if ((kind == "tuple") || (kind == "array")) {
                    vec![String::from("items")]
                } else {
                    if ((kind == "tupleIndex") || (kind == "field")) {
                        vec![String::from("object")]
                    } else {
                        if (kind == "math") {
                            vec![String::from("args")]
                        } else {
                            Vec::<String>::new()
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_rust_expression_completion_inert_leaf(kind: String) -> bool {
    ((((((kind == "name") || (kind == "num")) || (kind == "bool")) || (kind == "str")) || (kind == "unit")) || (kind == "functionValue"))
}

pub fn read_rust_expression_completion_short_circuit_value(operation: String) -> bool {
    (operation == "or")
}

pub fn read_rust_expression_completion_limit(kind: String) -> f64 {
    if (kind == "depth") {
        256f64
    } else {
        if (kind == "work") {
            100000f64
        } else {
            if (kind == "inspection") {
                1000000f64
            } else {
                0f64
            }
        }
    }
}

pub fn read_rust_expression_completion_packet_operation(operation: String) -> String {
    if (operation == (self::read_conditional_chain_operation(String::from("success")))) {
        String::from("success")
    } else {
        if (operation == (self::read_conditional_chain_operation(String::from("failure")))) {
            String::from("failure")
        } else {
            if (operation == (self::read_conditional_chain_operation(String::from("result")))) {
                String::from("result")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_owned_record_effect_form(kind: String, operation: String) -> String {
    if ((kind == "math") && (operation == "rustOwnedRecordStateRead")) {
        String::from("read state")
    } else {
        if ((kind == "math") && (operation == "rustOwnedRecordStateWrite")) {
            String::from("write state")
        } else {
            if (kind == "propositionValue") {
                String::from("proposition comparison")
            } else {
                self::read_rust_expression_completion_form(kind.clone(), operation.clone())
            }
        }
    }
}

pub fn read_rust_owned_record_effect_children(kind: String) -> Vec<String> {
    {
        let shared = self::read_rust_expression_completion_child_roles(kind.clone());
        if { let ml_value = (shared.len() as f64); ml_value != 0f64 && !ml_value.is_nan() } {
            shared
        } else {
            if (kind == "method") {
                vec![String::from("object"), String::from("args")]
            } else {
                if (kind == "math") {
                    vec![String::from("args")]
                } else {
                    if (kind == "index") {
                        vec![String::from("object"), String::from("index")]
                    } else {
                        if self::ml_array::includes(&vec![String::from("toString"), String::from("show"), String::from("booleanValue"), String::from("numberValue"), String::from("typeOf"), String::from("arrayIsArray")], &kind.clone(), 0f64, false, |a, b| a == b) {
                            vec![String::from("arg")]
                        } else {
                            if self::ml_array::includes(&vec![String::from("stringPoints"), String::from("copySlice"), String::from("length"), String::from("forOfIterable"), String::from("optionalField")], &kind.clone(), 0f64, false, |a, b| a == b) {
                                vec![String::from("object")]
                            } else {
                                if ((kind == "some") || (kind == "flattenOption")) {
                                    vec![String::from("value")]
                                } else {
                                    if (kind == "arrayFrom") {
                                        vec![String::from("args")]
                                    } else {
                                        if (kind == "stringTest") {
                                            vec![String::from("object"), String::from("search")]
                                        } else {
                                            if (kind == "stringMap") {
                                                vec![String::from("object")]
                                            } else {
                                                Vec::<String>::new()
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

pub fn read_rust_owned_record_effect_operation(operation: String) -> String {
    if (operation == "read") {
        String::from("rustOwnedRecordStateRead")
    } else {
        if (operation == "write") {
            String::from("rustOwnedRecordStateWrite")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_owned_record_effect_limit(kind: String) -> f64 {
    self::read_rust_expression_completion_limit(kind.clone())
}

pub fn accept_rust_owned_record_effect_scope(same_callable: bool, declared: bool) -> bool {
    (same_callable && declared)
}

pub fn accept_rust_owned_record_assertion_comparison(kind: String) -> bool {
    ((((((kind == "bool") || (kind == "fixed")) || (kind == "nat")) || (kind == "int")) || (kind == "string")) || (kind == "float"))
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindRustConditionalChainPatternLoop1Result {
    MlTemporaryFindRustConditionalChainPatternLoop1Done,
    MlTemporaryFindRustConditionalChainPatternLoop1Return(f64),
}

pub fn source_ml_temporary_find_rust_conditional_chain_pattern_loop1(mut kinds: Vec<String>, mut values: Vec<String>, mut closes: Vec<String>, mut at: f64) -> self::MlTemporaryFindRustConditionalChainPatternLoop1Result {
    loop {
        return if (at < ((kinds.len() as f64))) {
            {
                let kind = self::read_required_text_fact(kinds.clone(), at);
                {
                    let value = self::read_required_text_fact(values.clone(), at);
                    if ((!({ let ml_value = (closes.len() as f64); ml_value != 0f64 && !ml_value.is_nan() }) && (kind == "identifier")) && (value == "let")) {
                        self::MlTemporaryFindRustConditionalChainPatternLoop1Result::MlTemporaryFindRustConditionalChainPatternLoop1Return(at)
                    } else {
                        if (kind != "punct") {
                            {
                                let at_2 = (at + 1f64);
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        } else {
                            if (!({ let ml_value = (closes.len() as f64); ml_value != 0f64 && !ml_value.is_nan() }) && (value == "{")) {
                                self::MlTemporaryFindRustConditionalChainPatternLoop1Result::MlTemporaryFindRustConditionalChainPatternLoop1Return((-1f64))
                            } else {
                                if (((value == "(") || (value == "[")) || (value == "{")) {
                                    {
                                        let closes_2 = {
                                            let source_ml_source1_concat_value7 = closes;
                                            {
                                                let source_ml_source1_concat_value8 = vec![if (value == "(") {
                                                    String::from(")")
                                                } else {
                                                    if (value == "[") {
                                                        String::from("]")
                                                    } else {
                                                        String::from("}")
                                                    }
                                                }];
                                                self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_source1_concat_value7), source_ml_source1_concat_value8)
                                            }
                                        };
                                        {
                                            let at_3 = (at + 1f64);
                                            {
                                                (closes, at) = (closes_2, at_3);
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    if (((value == ")") || (value == "]")) || (value == "}")) {
                                        if (!({ let ml_value = (closes.len() as f64); ml_value != 0f64 && !ml_value.is_nan() }) || ((self::read_required_text_fact(closes.clone(), ((closes.len() as f64) - 1f64))) != value)) {
                                            self::MlTemporaryFindRustConditionalChainPatternLoop1Result::MlTemporaryFindRustConditionalChainPatternLoop1Return((-1f64))
                                        } else {
                                            {
                                                let closes_3 = self::ml_array::slice(&closes, 0f64, ((closes.len() as f64) - 1f64));
                                                {
                                                    let at_4 = (at + 1f64);
                                                    {
                                                        (closes, at) = (closes_3, at_4);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let at_5 = (at + 1f64);
                                            {
                                                at = at_5;
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
            self::MlTemporaryFindRustConditionalChainPatternLoop1Result::MlTemporaryFindRustConditionalChainPatternLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanConditionalChainLayersLoop2Result {
    MlTemporaryPlanConditionalChainLayersLoop2Done(Vec<f64>),
    MlTemporaryPlanConditionalChainLayersLoop2Return(Vec<f64>),
}

pub fn source_ml_temporary_plan_conditional_chain_layers_loop2(mut forms: Vec<f64>, mut result: Vec<f64>, mut at: f64) -> self::MlTemporaryPlanConditionalChainLayersLoop2Result {
    loop {
        return if (at > (0f64)) {
            {
                let at_2 = (at - 1f64);
                {
                    let form = self::read_required_number_fact(forms.clone(), at_2);
                    if ((form != (0f64)) && (form != (1f64))) {
                        self::MlTemporaryPlanConditionalChainLayersLoop2Result::MlTemporaryPlanConditionalChainLayersLoop2Return(Vec::<f64>::new())
                    } else {
                        {
                            let result_2 = {
                                let source_ml_source1_concat_value9 = result;
                                {
                                    let source_ml_source1_concat_value10 = vec![form, at_2];
                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value9), source_ml_source1_concat_value10)
                                }
                            };
                            {
                                (result, at) = (result_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanConditionalChainLayersLoop2Result::MlTemporaryPlanConditionalChainLayersLoop2Done(result)
        };
    }
}

pub fn source_ml_temporary_plan_conditional_match_arms_loop4(mut layers: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at > (0f64)) {
            {
                let at_2 = (at - 1f64);
                {
                    let layers_2 = {
                        let source_ml_source1_concat_value11 = layers;
                        {
                            let source_ml_source1_concat_value12 = vec![at_2];
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value11), source_ml_source1_concat_value12)
                        }
                    };
                    {
                        (layers, at) = (layers_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            layers
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanConditionalMatchArmsLoop3Result {
    MlTemporaryPlanConditionalMatchArmsLoop3Done(Vec<f64>),
    MlTemporaryPlanConditionalMatchArmsLoop3Return(Vec<f64>),
}

pub fn source_ml_temporary_plan_conditional_match_arms_loop3(mut guards: Vec<f64>, mut coverage: Vec<f64>, mut at: f64) -> self::MlTemporaryPlanConditionalMatchArmsLoop3Result {
    loop {
        return if (at < ((guards.len() as f64))) {
            {
                let guard = self::read_required_number_fact(guards.clone(), at);
                if ((guard != (0f64)) && (guard != (1f64))) {
                    self::MlTemporaryPlanConditionalMatchArmsLoop3Result::MlTemporaryPlanConditionalMatchArmsLoop3Return(Vec::<f64>::new())
                } else {
                    if (guard == (0f64)) {
                        {
                            let coverage_2 = {
                                let source_ml_source1_concat_value13 = coverage;
                                {
                                    let source_ml_source1_concat_value14 = vec![at];
                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value13), source_ml_source1_concat_value14)
                                }
                            };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (coverage, at) = (coverage_2, at_2);
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
            self::MlTemporaryPlanConditionalMatchArmsLoop3Result::MlTemporaryPlanConditionalMatchArmsLoop3Done(coverage)
        };
    }
}

fn ml_main() {
    assert!(((self::read_rust_owned_record_effect_form(String::from("math"), String::from("rustOwnedRecordStateRead"))) == "read state"), "assertion 1");
    assert!(((self::read_rust_owned_record_effect_form(String::from("math"), String::from("rustOwnedRecordStateWrite"))) == "write state"), "assertion 2");
    assert!(((self::read_rust_owned_record_effect_form(String::from("if"), String::from(""))) == "conditional"), "assertion 3");
    assert!(((self::read_rust_owned_record_effect_form(String::from("binary"), String::from("and"))) == "short circuit"), "assertion 4");
    assert!(((self::read_rust_owned_record_effect_form(String::from("match"), String::from(""))) == "match"), "assertion 5");
    assert!(((self::read_rust_owned_record_effect_children(String::from("app"))) == (vec![String::from("fn"), String::from("args")])), "assertion 6");
    assert!(((self::read_rust_owned_record_effect_children(String::from("method"))) == (vec![String::from("object"), String::from("args")])), "assertion 7");
    assert!(((self::read_rust_owned_record_effect_children(String::from("math"))) == (vec![String::from("args")])), "assertion 8");
    assert!(((self::read_rust_owned_record_effect_children(String::from("lambda"))) == (Vec::<String>::new())), "assertion 9");
    assert!(((self::read_rust_owned_record_effect_operation(String::from("read"))) == "rustOwnedRecordStateRead"), "assertion 10");
    assert!(((self::read_rust_owned_record_effect_operation(String::from("write"))) == "rustOwnedRecordStateWrite"), "assertion 11");
    assert!(((self::read_rust_owned_record_effect_operation(String::from("unknown"))) == ""), "assertion 12");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_effect_limit(String::from("depth")), 256f64), "assertion 13");
    assert!(((self::accept_rust_owned_record_effect_scope(true, true)) == (true)), "assertion 14");
    assert!(((self::accept_rust_owned_record_effect_scope(false, true)) == (false)), "assertion 15");
    assert!(((self::accept_rust_owned_record_effect_scope(true, false)) == (false)), "assertion 16");
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
fn all_literal_owned_record_effect_controls() { main(); }
