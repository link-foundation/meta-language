// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlAcceptRecordFieldNameLoop3Result;
use super::MlCompareRecordFieldNamesLoop4Result;
use super::MlReadRecordFieldInsertionOrderLoop12Result;
use super::MlReadRecordFieldPositionLoop5Result;
use super::MlReadRecordFieldPositionLoop6Result;
use super::MlReadRecordProjectionErrorLoop2Result;
use super::MlReadRecordSortedFieldPositionsLoop7Result;
use super::MlReadRecordTypeOwnerPresenceLoop17Result;
use super::MlRecordTypeFactFieldLoop15Result;
use super::ml_accept_record_field_name_loop3;
use super::ml_compare_record_field_names_loop4;
use super::ml_read_record_field_insertion_order_loop12;
use super::ml_read_record_field_insertion_order_loop13;
use super::ml_read_record_field_order_loop11;
use super::ml_read_record_field_position_loop5;
use super::ml_read_record_field_position_loop6;
use super::ml_read_record_projection_error_loop2;
use super::ml_read_record_sorted_field_positions_loop7;
use super::ml_read_record_sorted_field_positions_loop8;
use super::ml_read_record_type_owner_presence_loop17;
use super::ml_record_type_fact_elements_loop16;
use super::ml_record_type_fact_field_loop15;

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
pub fn refuse_required_record_name() -> Vec<f64> {
    panic!("{}", "missing required record name row")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_record_name(names: &[Vec<f64>], index: f64) -> Vec<f64> {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (names.len() as f64))),
            "{}",
            "invalid required record name position"
        );

        (self::ml_array::optional_at(names, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_record_name)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_value_order(count: f64, reverse: bool) -> Vec<f64> {
    if !(self::ml_math::is_safe_integer(count)) || (count < 0f64) {
        vec![-1f64]
    } else {
        {
            let order = Vec::<f64>::new();
            {
                let index = 0f64;
                { self::ml_read_record_value_order_loop1(count, reverse, order, index) }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_projection() -> String {
    String::from("checked ordered record values")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_projection_error(
    projection: &str,
    names: &[Vec<f64>],
    entries: &[bool],
) -> String {
    {
        let ml_library_inequality_2 = projection != (self::read_record_projection());
        if ml_library_inequality_2 {
            String::from("unknown checked record projection")
        } else {
            let ml_library_inequality_1 = (names.len() as f64) != (entries.len() as f64);
            if ml_library_inequality_1 {
                String::from("checked record projection has inconsistent entry facts")
            } else {
                {
                    let index = 0f64;
                    {
                        let ml_s1 =
                            self::ml_read_record_projection_error_loop2(names, entries, index);
                        match ml_s1 {
                        self::MlReadRecordProjectionErrorLoop2Result::MlReadRecordProjectionErrorLoop2Done => {
                            String::new()
                        }
                        self::MlReadRecordProjectionErrorLoop2Result::MlReadRecordProjectionErrorLoop2Return(ml_result2) => {
                            ml_result2
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
pub fn accept_record_field_name(name: &[f64]) -> bool {
    {
        let index = 0f64;
        {
            let ml_s2 = self::ml_accept_record_field_name_loop3(name, index);
            match ml_s2 {
                self::MlAcceptRecordFieldNameLoop3Result::MlAcceptRecordFieldNameLoop3Done => true,
                self::MlAcceptRecordFieldNameLoop3Result::MlAcceptRecordFieldNameLoop3Return(
                    ml_result3,
                ) => ml_result3,
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_record_field_names(left: &[f64], right: &[f64]) -> f64 {
    {
        let end = if (left.len() as f64) < (right.len() as f64) {
            left.len() as f64
        } else {
            right.len() as f64
        };
        {
            let index = 0f64;
            {
                let ml_s3 = self::ml_compare_record_field_names_loop4(left, right, end, index);
                match ml_s3 {
                    self::MlCompareRecordFieldNamesLoop4Result::MlCompareRecordFieldNamesLoop4Done => {
                        if (left.len() as f64) < (right.len() as f64) {
                            -1f64
                        } else if (left.len() as f64) > (right.len() as f64) {
                                1f64
                            } else {
                                0f64
                            }
                    }
                    self::MlCompareRecordFieldNamesLoop4Result::MlCompareRecordFieldNamesLoop4Return(ml_result4) => {
                        ml_result4
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_position(fields: &[Vec<f64>], name: &[f64]) -> f64 {
    if (self::accept_record_field_name(name)) {
        {
            let index = 0f64;
            {
                let ml_s4 = self::ml_read_record_field_position_loop5(fields, index);
                match ml_s4 {
                    self::MlReadRecordFieldPositionLoop5Result::MlReadRecordFieldPositionLoop5Done => {
                        {
                            let index_2 = 0f64;
                            {
                                let ml_s5 = self::ml_read_record_field_position_loop6(fields, name, index_2);
                                match ml_s5 {
                                    self::MlReadRecordFieldPositionLoop6Result::MlReadRecordFieldPositionLoop6Done => {
                                        -1f64
                                    }
                                    self::MlReadRecordFieldPositionLoop6Result::MlReadRecordFieldPositionLoop6Return(ml_result6) => {
                                        ml_result6
                                    }
                                }
                            }
                        }
                    }
                    self::MlReadRecordFieldPositionLoop5Result::MlReadRecordFieldPositionLoop5Return(ml_result6_2) => {
                        ml_result6_2
                    }
                }
            }
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_sorted_field_positions(fields: &[Vec<f64>]) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let ml_s6 = self::ml_read_record_sorted_field_positions_loop7(fields, index);
            match ml_s6 {
                self::MlReadRecordSortedFieldPositionsLoop7Result::MlReadRecordSortedFieldPositionsLoop7Done => {
                    {
                        let sorted = self::read_record_value_order((fields.len() as f64), false);
                        {
                            let width = 1f64;
                            {
                                self::ml_read_record_sorted_field_positions_loop8(fields, sorted, width)
                            }
                        }
                    }
                }
                self::MlReadRecordSortedFieldPositionsLoop7Result::MlReadRecordSortedFieldPositionsLoop7Return(ml_result10) => {
                    ml_result10
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
        if ((sorted.len() as f64) > 0f64)
            && ((self::read_required_number_fact(&(sorted), 0f64)) < 0f64)
        {
            vec![-1f64]
        } else {
            {
                let selected = Vec::<f64>::new();
                {
                    let index = 0f64;
                    { self::ml_read_record_field_order_loop11(fields, &(sorted), selected, index) }
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
        if ((sorted.len() as f64) > 0f64)
            && ((self::read_required_number_fact(&(sorted), 0f64)) < 0f64)
        {
            vec![-1f64]
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
                                let ml_s7 = self::ml_read_record_field_insertion_order_loop12(
                                    fields,
                                    &(sorted),
                                    first,
                                    last,
                                    start,
                                    index,
                                );
                                match ml_s7 {
                                    self::MlReadRecordFieldInsertionOrderLoop12Result::MlReadRecordFieldInsertionOrderLoop12Done(first_2, last_2) => {
                                        {
                                            let selected = Vec::<f64>::new();
                                            {
                                                let index_2 = 0f64;
                                                {
                                                    self::ml_read_record_field_insertion_order_loop13(fields, &(first_2), &(last_2), selected, index_2)
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
pub fn read_record_spread_role(kind: &str) -> String {
    if kind == "var" {
        String::from("open")
    } else if kind == "record" {
        String::from("fields")
    } else {
        String::from("refuse")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_type_constraint_step(
    left: &[Vec<f64>],
    right: &[Vec<f64>],
    left_open: bool,
    right_open: bool,
    phase: f64,
    index: f64,
) -> Vec<f64> {
    if phase < 1f64 {
        if index >= (left.len() as f64) {
            vec![5f64, -1f64, -1f64, 1f64, 0f64]
        } else {
            {
                let matching = self::read_record_field_position(
                    right,
                    &(self::read_required_record_name(left, index)),
                );
                if matching >= 0f64 {
                    vec![0f64, index, matching, 0f64, (index + 1f64)]
                } else {
                    vec![
                        if right_open { 1f64 } else { 3f64 },
                        index,
                        -1f64,
                        0f64,
                        (index + 1f64),
                    ]
                }
            }
        }
    } else if index >= (right.len() as f64) {
        vec![6f64, -1f64, -1f64, 1f64, index]
    } else {
        {
            let matching_2 = self::read_record_field_position(
                left,
                &(self::read_required_record_name(right, index)),
            );
            if matching_2 >= 0f64 {
                vec![5f64, matching_2, index, 1f64, (index + 1f64)]
            } else {
                vec![
                    if left_open { 2f64 } else { 4f64 },
                    -1f64,
                    index,
                    1f64,
                    (index + 1f64),
                ]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_type_constraint_error(action: f64, name: &str) -> String {
    if (action == 3f64) || (action == 4f64) {
        format!("record field {} is missing", name.to_owned())
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_method_error(
    kind: &str,
    operation: &str,
    parameters: f64,
    arguments_count: f64,
) -> String {
    if kind == "function" {
        if arguments_count == parameters {
            String::new()
        } else {
            format!(
                ".{}() expects {} arguments",
                operation.to_owned(),
                self::ml_number::js_number(parameters)
            )
        }
    } else {
        format!("record has no method {}", operation.to_owned())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_inference_step(fields: &[Vec<f64>], name: &[f64], open: bool) -> Vec<f64> {
    {
        let index = self::read_record_field_position(fields, name);
        if index >= 0f64 {
            vec![0f64, index]
        } else {
            vec![if open { 1f64 } else { 2f64 }, -1f64]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn record_type_fact_field(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    owner: f64,
    name: &str,
) -> f64 {
    if owner < 0f64 {
        -1f64
    } else {
        {
            let start = self::read_required_number_fact(starts, owner);
            {
                let end = start + self::read_required_number_fact(counts, owner);
                {
                    let edge = start;
                    {
                        let ml_s8 = self::ml_record_type_fact_field_loop15(
                            labels, targets, name, end, edge,
                        );
                        match ml_s8 {
                            self::MlRecordTypeFactFieldLoop15Result::MlRecordTypeFactFieldLoop15Done => {
                                -1f64
                            }
                            self::MlRecordTypeFactFieldLoop15Result::MlRecordTypeFactFieldLoop15Return(ml_result15) => {
                                ml_result15
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
pub fn record_type_fact_kind(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    owner: f64,
) -> String {
    {
        let kind = self::record_type_fact_field(starts, counts, labels, targets, owner, "kind");
        if kind < 0f64 {
            String::new()
        } else {
            self::read_required_text_fact(texts, kind)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn record_type_fact_elements(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    owner: f64,
) -> Vec<f64> {
    {
        let values = Vec::<f64>::new();
        if owner < 0f64 {
            values
        } else {
            {
                let start = self::read_required_number_fact(starts, owner);
                {
                    let end = start + self::read_required_number_fact(counts, owner);
                    {
                        let edge = start;
                        { self::ml_record_type_fact_elements_loop16(targets, values, end, edge) }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_record_type_owner_presence(
    texts: &[String],
    numbers: &[f64],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    bindings: &[f64],
    root: f64,
    owner: f64,
) -> bool {
    {
        let pending = vec![root];
        {
            let seen = Vec::<f64>::new();
            {
                let position = 0f64;
                {
                    let ml_s9 = self::ml_read_record_type_owner_presence_loop17(
                        texts, numbers, starts, counts, labels, targets, bindings, owner, pending,
                        seen, position,
                    );
                    match ml_s9 {
                        self::MlReadRecordTypeOwnerPresenceLoop17Result::MlReadRecordTypeOwnerPresenceLoop17Done => {
                            false
                        }
                        self::MlReadRecordTypeOwnerPresenceLoop17Result::MlReadRecordTypeOwnerPresenceLoop17Return(ml_result17) => {
                            ml_result17
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_value_order_loop1(
    mut count: f64,
    mut reverse: bool,
    mut order: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let order_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![if reverse {
                            (count - index) - 1f64
                        } else {
                            index
                        }],
                        order,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (order, index) = (order_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { order };
    }
}
