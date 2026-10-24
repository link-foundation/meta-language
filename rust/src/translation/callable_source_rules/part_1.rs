// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::MlSource1TemporaryCallableBoundLoop14Result;
use super::MlSource1TemporaryCallableFieldLoop13Result;
use super::MlSource1TemporaryCallableTypeParametersLoop5Result;
use super::MlSource1TemporaryPlanCallableTypeSyntaxJoin7;
use super::MlSource1TemporaryPlanCallableTypeSyntaxJoin9;
use super::MlSource1TemporaryPlanCallableTypeSyntaxLoop10Result;
use super::source_ml_source1_temporary_callable_bound_loop14;
use super::source_ml_source1_temporary_callable_field_loop13;
use super::source_ml_source1_temporary_callable_parameter_names_loop15;
use super::source_ml_source1_temporary_callable_type_parameter_segments_loop4;
use super::source_ml_source1_temporary_callable_type_parameters_loop5;
use super::source_ml_source1_temporary_callable_type_trim_end_loop3;
use super::source_ml_source1_temporary_callable_type_trim_start_loop2;
use super::source_ml_source1_temporary_plan_callable_type_syntax_loop8;
use super::source_ml_source1_temporary_plan_callable_type_syntax_loop10;
use super::source_ml_source1_temporary_plan_callable_type_syntax_loop11;
use super::source_ml_source1_temporary_read_type_separator_positions_loop1;

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

/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    #[must_use]
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if (-0.5..0.0).contains(&x) {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where `f64::signum` does not.
    #[must_use]
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
    #[must_use]
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() { b } else { a }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    #[must_use]
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() { a } else { b }
        } else {
            f64::NAN
        }
    }

    #[must_use]
    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    #[must_use]
    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    #[must_use]
    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    #[must_use]
    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
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
pub fn refuse_required_number_fact() -> f64 {
    panic!("{}", "missing required Number fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    panic!("{}", "missing required text fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    panic!("{}", "missing required Boolean fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required Number fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required text fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required Boolean fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_type_separator_positions(units: &[f64], separator: f64) -> Vec<f64> {
    {
        let depth = 0f64;
        {
            let positions = Vec::<f64>::new();
            {
                let at = 0f64;
                {
                    self::source_ml_source1_temporary_read_type_separator_positions_loop1(
                        units, separator, depth, positions, at,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_union_type_field(field: &str) -> bool {
    (field == "kind") || (field == "alternatives")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_union_type_count(count: f64) -> bool {
    self::ml_math::is_safe_integer(count) && (count >= 2f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_union_target_refusal(target: &str) -> String {
    if target == "Rust" {
        String::from("no Rust type for union")
    } else if target == "Lean" {
        String::from("no Lean type for union")
    } else if target == "Rocq" {
        String::from("no Rocq type for union")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_statement_role(kind: &str) -> String {
    if (kind == "const") || (kind == "let") {
        String::from("declare")
    } else if kind == "assign" {
        String::from("assign")
    } else if kind == "assignReference" {
        String::from("reference")
    } else if ((kind == "return") || (kind == "expr")) || (kind == "print") {
        String::from("read")
    } else if kind == "block" {
        String::from("block")
    } else if kind == "if" {
        String::from("branch")
    } else if kind == "switch" {
        String::from("switch")
    } else if (kind == "while") || (kind == "doWhile") {
        String::from("loop")
    } else if kind == "for" {
        String::from("for")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_callable_type_whitespace(unit: f64) -> bool {
    (((((((((((9f64..=13f64).contains(&unit)) || (unit == 32f64)) || (unit == 160f64))
        || (unit == 5760f64))
        || ((8192f64..=8202f64).contains(&unit)))
        || (unit == 8232f64))
        || (unit == 8233f64))
        || (unit == 8239f64))
        || (unit == 8287f64))
        || (unit == 12_288f64))
        || (unit == 65_279f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_type_trim_start(units: &[f64], start: f64, end: f64) -> f64 {
    {
        let at = start;
        { self::source_ml_source1_temporary_callable_type_trim_start_loop2(units, end, at) }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_type_trim_end(units: &[f64], start: f64, end: f64) -> f64 {
    {
        let at = end;
        { self::source_ml_source1_temporary_callable_type_trim_end_loop3(units, start, at) }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_type_parameter_segments(
    units: &[f64],
    start: f64,
    end: f64,
    named: bool,
) -> Vec<f64> {
    {
        let part = self::ml_array::slice(units, start, end);
        {
            let colons = if named {
                self::read_type_separator_positions(&(part), 58f64)
            } else {
                Vec::<f64>::new()
            };
            {
                let ml_product_condition_1 = {
                    let ml_value = colons.len() as f64;
                    ml_value != 0f64 && !ml_value.is_nan()
                };
                if ml_product_condition_1 {
                    {
                        let from = self::read_required_number_fact(&(colons), 0f64) + 1f64;
                        {
                            let at = 1f64;
                            {
                                let segments = Vec::<f64>::new();
                                {
                                    let segments_2 = self::source_ml_source1_temporary_callable_type_parameter_segments_loop4(start, &(part), &(colons), from, at, segments);
                                    {
                                        let source_ml_source2_concat_value1 =
                                            vec![(colons.len() as f64)];
                                        {
                                            let source_ml_source2_concat_value2 = segments_2;
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source2_concat_value1,
                                                ),
                                                source_ml_source2_concat_value2,
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    vec![1f64, start, end]
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_type_parameters(units: &[f64], start: f64, end: f64, named: bool) -> Vec<f64> {
    {
        let part = self::ml_array::slice(units, start, end);
        {
            let commas = self::read_type_separator_positions(&(part), 44f64);
            {
                let from = 0f64;
                {
                    let at = 0f64;
                    {
                        let count = 0f64;
                        {
                            let plan = Vec::<f64>::new();
                            {
                                let source_ml_source2_s3 = self::source_ml_source1_temporary_callable_type_parameters_loop5(units, start, named, &(part), &(commas), from, at, count, plan);
                                match source_ml_source2_s3 {
                                    self::MlSource1TemporaryCallableTypeParametersLoop5Result::MlSource1TemporaryCallableTypeParametersLoop5Done(count_2, plan_2) => {
                                        {
                                            let source_ml_source2_concat_value4 = vec![count_2];
                                            {
                                                let source_ml_source2_concat_value5 = plan_2;
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value4), source_ml_source2_concat_value5)
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
pub fn plan_callable_type_syntax(units: &[f64]) -> Vec<f64> {
    {
        let ml_product_condition_1 = {
            let ml_value = units.len() as f64;
            ml_value != 0f64 && !ml_value.is_nan()
        };
        if ml_product_condition_1 {
            {
                let source_ml_source2_s8 = if (self::read_required_number_fact(units, 0f64))
                    == 40f64
                {
                    {
                        let depth = 0f64;
                        {
                            let close = -1f64;
                            {
                                let at = 0f64;
                                {
                                    let close_2 = self::source_ml_source1_temporary_plan_callable_type_syntax_loop11(units, depth, close, at);
                                    {
                                        let arrow = self::callable_type_trim_start(
                                            units,
                                            (close_2 + 1f64),
                                            (units.len() as f64),
                                        );
                                        if (((close_2 >= 0f64)
                                            && ((arrow + 1f64) < (units.len() as f64)))
                                            && ((self::read_required_number_fact(units, arrow))
                                                == 61f64))
                                            && ((self::read_required_number_fact(
                                                units,
                                                (arrow + 1f64),
                                            )) == 62f64)
                                        {
                                            {
                                                let start = self::callable_type_trim_start(
                                                    units,
                                                    (arrow + 2f64),
                                                    (units.len() as f64),
                                                );
                                                {
                                                    let end = self::callable_type_trim_end(
                                                        units,
                                                        start,
                                                        (units.len() as f64),
                                                    );
                                                    self::MlSource1TemporaryPlanCallableTypeSyntaxJoin7::MlSource1TemporaryPlanCallableTypeSyntaxJoin7Return(if start == end {
                                                    vec![-1f64]
                                                } else {
                                                    {
                                                        let source_ml_source2_concat_value9 = vec![1f64, start, end];
                                                        {
                                                            let source_ml_source2_concat_value10 = self::callable_type_parameters(units, 1f64, close_2, true);
                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value9), source_ml_source2_concat_value10)
                                                        }
                                                    }
                                                })
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryPlanCallableTypeSyntaxJoin7::MlSource1TemporaryPlanCallableTypeSyntaxJoin7Next
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryPlanCallableTypeSyntaxJoin7::MlSource1TemporaryPlanCallableTypeSyntaxJoin7Next
                };
                match source_ml_source2_s8 {
                self::MlSource1TemporaryPlanCallableTypeSyntaxJoin7::MlSource1TemporaryPlanCallableTypeSyntaxJoin7Next => {
                    {
                        let prefix = vec![102f64, 117f64, 110f64, 99f64, 116f64, 105f64, 111f64, 110f64];
                        {
                            let at_2 = 0f64;
                            {
                                let at_3 = self::source_ml_source1_temporary_plan_callable_type_syntax_loop8(units, &(prefix), at_2);
                                {
                                    let source_ml_source2_s12 = if at_3 == (prefix.len() as f64) {
                                        {
                                            let open = self::callable_type_trim_start(units, at_3, (units.len() as f64));
                                            if (open < (units.len() as f64)) && ((self::read_required_number_fact(units, open)) == 40f64) {
                                                {
                                                    let close_3 = units.len() as f64;
                                                    {
                                                        let source_ml_source2_s13 = self::source_ml_source1_temporary_plan_callable_type_syntax_loop10(units, open, close_3);
                                                        match source_ml_source2_s13 {
                                                            self::MlSource1TemporaryPlanCallableTypeSyntaxLoop10Result::MlSource1TemporaryPlanCallableTypeSyntaxLoop10Done => {
                                                                self::MlSource1TemporaryPlanCallableTypeSyntaxJoin9::MlSource1TemporaryPlanCallableTypeSyntaxJoin9Next
                                                            }
                                                            self::MlSource1TemporaryPlanCallableTypeSyntaxLoop10Result::MlSource1TemporaryPlanCallableTypeSyntaxLoop10Return(source_ml_source1_temporary_result10) => {
                                                                self::MlSource1TemporaryPlanCallableTypeSyntaxJoin9::MlSource1TemporaryPlanCallableTypeSyntaxJoin9Return(source_ml_source1_temporary_result10)
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryPlanCallableTypeSyntaxJoin9::MlSource1TemporaryPlanCallableTypeSyntaxJoin9Next
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanCallableTypeSyntaxJoin9::MlSource1TemporaryPlanCallableTypeSyntaxJoin9Next
                                    };
                                    match source_ml_source2_s12 {
                                        self::MlSource1TemporaryPlanCallableTypeSyntaxJoin9::MlSource1TemporaryPlanCallableTypeSyntaxJoin9Next => {
                                            if ((self::read_required_number_fact(units, 0f64)) == 40f64) && ((self::read_required_number_fact(units, ((units.len() as f64) - 1f64))) == 41f64) {
                                                vec![0f64, 1f64, ((units.len() as f64) - 1f64)]
                                            } else {
                                                Vec::<f64>::new()
                                            }
                                        }
                                        self::MlSource1TemporaryPlanCallableTypeSyntaxJoin9::MlSource1TemporaryPlanCallableTypeSyntaxJoin9Return(source_ml_source1_temporary_result9) => {
                                            source_ml_source1_temporary_result9
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryPlanCallableTypeSyntaxJoin7::MlSource1TemporaryPlanCallableTypeSyntaxJoin7Return(source_ml_source1_temporary_result10_2) => {
                    source_ml_source1_temporary_result10_2
                }
            }
            }
        } else {
            Vec::<f64>::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_linked_application_error(
    kind: &str,
    parameters: f64,
    arguments_count: f64,
) -> String {
    if kind == "function" {
        if (parameters).partial_cmp(&(arguments_count)) == Some(std::cmp::Ordering::Equal) {
            String::new()
        } else {
            String::from("a linked application has the wrong number of arguments")
        }
    } else {
        String::from("a linked application does not target a function")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_field(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    field: &str,
) -> f64 {
    if (node < 0f64) || (node >= (starts.len() as f64)) {
        -1f64
    } else {
        {
            let at = self::read_required_number_fact(starts, node);
            {
                let source_ml_source2_s14 = self::source_ml_source1_temporary_callable_field_loop13(
                    starts, counts, labels, targets, node, field, at,
                );
                match source_ml_source2_s14 {
                    self::MlSource1TemporaryCallableFieldLoop13Result::MlSource1TemporaryCallableFieldLoop13Done => {
                        -1f64
                    }
                    self::MlSource1TemporaryCallableFieldLoop13Result::MlSource1TemporaryCallableFieldLoop13Return(source_ml_source1_temporary_result13) => {
                        source_ml_source1_temporary_result13
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_text(texts: &[String], node: f64) -> String {
    if (node < 0f64) || (node >= (texts.len() as f64)) {
        String::new()
    } else {
        self::read_required_text_fact(texts, node)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_truthy(forms: &[String], texts: &[String], node: f64) -> bool {
    if (node < 0f64) || (node >= (forms.len() as f64)) {
        false
    } else if ((self::read_required_text_fact(forms, node)) == "object")
        || ((self::read_required_text_fact(forms, node)) == "array")
    {
        true
    } else if (self::read_required_text_fact(forms, node)) == "boolean" {
        (self::read_required_text_fact(texts, node)) == "true"
    } else if (self::read_required_text_fact(forms, node)) == "number" {
        (((self::read_required_text_fact(texts, node)) != "0")
            && ((self::read_required_text_fact(texts, node)) != "-0"))
            && ((self::read_required_text_fact(texts, node)) != "NaN")
    } else {
        ((self::read_required_text_fact(forms, node)) == "string")
            && (!(self::read_required_text_fact(texts, node)).is_empty())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_bound(names: &[String], name: &str) -> bool {
    {
        let at = names.len() as f64;
        {
            let source_ml_source2_s15 =
                self::source_ml_source1_temporary_callable_bound_loop14(names, name, at);
            match source_ml_source2_s15 {
                self::MlSource1TemporaryCallableBoundLoop14Result::MlSource1TemporaryCallableBoundLoop14Done => {
                    false
                }
                self::MlSource1TemporaryCallableBoundLoop14Result::MlSource1TemporaryCallableBoundLoop14Return(source_ml_source1_temporary_result14) => {
                    source_ml_source1_temporary_result14
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_element(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    node: f64,
    index: f64,
) -> f64 {
    if ((node < 0f64) || (index < 0f64))
        || (index >= (self::read_required_number_fact(counts, node)))
    {
        -1f64
    } else {
        self::read_required_number_fact(
            targets,
            (self::read_required_number_fact(starts, node) + index),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_tag(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    field: &str,
) -> String {
    self::callable_text(
        texts,
        self::callable_field(starts, counts, labels, targets, node, field),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_parameter_names(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    parameters: f64,
) -> Vec<String> {
    {
        let names = Vec::<String>::new();
        if parameters < 0f64 {
            names
        } else {
            {
                let at = 0f64;
                {
                    self::source_ml_source1_temporary_callable_parameter_names_loop15(
                        texts, starts, counts, labels, targets, parameters, names, at,
                    )
                }
            }
        }
    }
}
