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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Number fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required text fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Boolean fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Number fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required text fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Boolean fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_record_name() -> Vec<f64> {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required record name row")).encode_utf16().collect(), message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_record_name(names: &[Vec<f64>], index: f64) -> Vec<f64> {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((names.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required record name position")).encode_utf16().collect(), message_present: true }).raise()
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
pub fn read_record_projection() -> String {
    String::from("checked ordered record values")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_projection_error(projection: &str, names: &[Vec<f64>], entries: &[bool]) -> String {
    if (projection != (self::read_record_projection())) {
        String::from("unknown checked record projection")
    } else {
        if (((names.len() as f64)) != ((entries.len() as f64))) {
            String::from("checked record projection has inconsistent entry facts")
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_s1 = self::source_ml_temporary_read_record_projection_error_loop2(names, entries, index);
                    match source_ml_source1_s1.clone() {
                        self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Done => {
                            String::from("")
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
pub fn read_record_spread_role(kind: &str) -> String {
    if (kind == "var") {
        String::from("open")
    } else {
        if (kind == "record") {
            String::from("fields")
        } else {
            String::from("refuse")
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
pub fn read_record_type_constraint_error(action: f64, name: &str) -> String {
    if ((action == (3f64)) || (action == (4f64))) {
        format!("{}{}", (format!("{}{}", (String::from("record field ")), name.to_owned())), (String::from(" is missing")))
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_method_error(kind: &str, operation: &str, parameters: f64, arguments_count: f64) -> String {
    if (kind != "function") {
        format!("{}{}", (String::from("record has no method ")), operation.to_owned())
    } else {
        if (arguments_count != parameters) {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from(".")), operation.to_owned())), (String::from("() expects ")))), (self::ml_number::js_number(parameters)))), (String::from(" arguments")))
        } else {
            String::from("")
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
pub fn record_type_fact_field(starts: &[f64], counts: &[f64], labels: &[String], targets: &[f64], owner: f64, name: &str) -> f64 {
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
pub fn record_type_fact_kind(texts: &[String], starts: &[f64], counts: &[f64], labels: &[String], targets: &[f64], owner: f64) -> String {
    {
        let kind = self::record_type_fact_field(starts, counts, labels, targets, owner, "kind");
        if (kind < (0f64)) {
            String::from("")
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
pub fn read_record_type_owner_presence(texts: &[String], numbers: &[f64], starts: &[f64], counts: &[f64], labels: &[String], targets: &[f64], bindings: &[f64], root: f64, owner: f64) -> bool {
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
pub fn read_java_script_literal_property_form(kind: &str, index: f64) -> String {
    if ((((kind == "array") || (kind == "tuple")) || (kind == "string")) && (index >= (0f64))) {
        String::from("index")
    } else {
        String::from("field")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_index_number_form(kind: &str, element: &str) -> String {
    if (kind == "float") {
        String::from("identity")
    } else {
        if ((kind == "int") || (kind == "nat")) {
            String::from("convert")
        } else {
            if ((kind == "unit") || ((kind == "option") && (element == "unit"))) {
                String::from("absent")
            } else {
                if ((kind == "option") && (((element == "float") || (element == "int")) || (element == "nat"))) {
                    String::from("optional")
                } else {
                    String::from("refuse")
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_record_order_name() -> Vec<f64> {
    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("an ordered record needs every actual own field name")).encode_utf16().collect(), message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_record_order_position() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("an ordered record needs every actual field position")).encode_utf16().collect(), message_present: true }).raise()
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
pub fn read_java_script_record_order_creation_error(names: &[Vec<f64>], insertion: &[f64]) -> String {
    if (!(self::accept_java_script_record_order_names(names)) || !(self::accept_java_script_record_order_permutation((names.len() as f64), insertion))) {
        String::from("invalid ordered record field sequence")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_record_property_order(names: &[Vec<f64>], insertion: &[f64]) -> Vec<f64> {
    {
        let error = self::read_java_script_record_order_creation_error(names, insertion);
        if !error.is_empty() {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (error.clone()).encode_utf16().collect(), message_present: true }).raise()
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
pub fn read_java_script_record_order_merge_error(names: &[Vec<f64>], chunks: &[Vec<f64>]) -> String {
    if !(self::accept_java_script_record_order_names(names)) {
        String::from("invalid ordered record field names")
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
        if !error.is_empty() {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (error.clone()).encode_utf16().collect(), message_present: true }).raise()
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
pub fn read_java_script_record_order_rebase_error(target: &[Vec<f64>], source: &[Vec<f64>], order: &[f64]) -> String {
    if (!(self::accept_java_script_record_order_names(target)) || !(self::accept_java_script_record_property_order(source, order))) {
        String::from("invalid ordered record spread source")
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s17 = self::source_ml_temporary_read_java_script_record_order_rebase_error_loop34(target, source, index);
                match source_ml_source1_s17.clone() {
                    self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Done => {
                        String::from("")
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
        if !error.is_empty() {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (error.clone()).encode_utf16().collect(), message_present: true }).raise()
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
pub fn read_java_script_record_order_key_error(names: &[Vec<f64>], order: &[f64]) -> String {
    if self::accept_java_script_record_property_order(names, order) {
        String::from("")
    } else {
        String::from("invalid ordered record property order")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_property_keys(names: &[Vec<f64>], order: &[f64]) -> Vec<Vec<f64>> {
    {
        let error = self::read_java_script_record_order_key_error(names, order);
        if !error.is_empty() {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (error.clone()).encode_utf16().collect(), message_present: true }).raise()
        } else {
            {
                let result = Vec::<Vec<f64>>::new();
                {
                    let index = 0f64;
                    {
                        let result_2 = self::source_ml_temporary_read_java_script_record_property_keys_loop36(names, order, result, index);
                        result_2
                    }
                }
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
    MlTemporaryReadRecordProjectionErrorLoop2Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_projection_error_loop2(names: &[Vec<f64>], entries: &[bool], mut index: f64) -> self::MlTemporaryReadRecordProjectionErrorLoop2Result {
    loop {
        return if (index < ((names.len() as f64))) {
            if !(match self::ml_array::optional_at(entries, self::ml_array::number_index(index)) { Some(ml_present) => ml_present, None => false }) {
                self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(String::from("checked record projection has a malformed field entry"))
            } else {
                if !(self::accept_record_field_name(&(self::read_required_record_name(names, index)))) {
                    self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(String::from("checked record projection has an invalid UTF16 field name"))
                } else {
                    if ((index > (0f64)) && ((self::compare_record_field_names(&(self::read_required_record_name(names, (index - 1f64))), &(self::read_required_record_name(names, index)))) >= (0f64))) {
                        self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(String::from("checked record projection needs unique fields in UTF16 order"))
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
                    let source_ml_source1_comparison_operand18 = unit.clone();
                    {
                        let source_ml_source1_comparison_operand19 = 0f64;
                        if match (source_ml_source1_comparison_operand18.clone(), None::<f64>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                            ((f64::NAN) < source_ml_source1_comparison_operand19)
                        } else {
                            ((source_ml_source1_comparison_operand18.clone().expect("a proven present value")) < source_ml_source1_comparison_operand19)
                        }
                    }
                }) || {
                    let source_ml_source1_comparison_operand20 = unit.clone();
                    {
                        let source_ml_source1_comparison_operand21 = 65535f64;
                        if match (source_ml_source1_comparison_operand20.clone(), None::<f64>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                            ((f64::NAN) > source_ml_source1_comparison_operand21)
                        } else {
                            ((source_ml_source1_comparison_operand20.clone().expect("a proven present value")) > source_ml_source1_comparison_operand21)
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
pub fn source_ml_temporary_record_type_fact_field_loop15(labels: &[String], targets: &[f64], name: &str, mut end: f64, mut edge: f64) -> self::MlTemporaryRecordTypeFactFieldLoop15Result {
    loop {
        return if (edge < end) {
            if ((self::read_required_text_fact(labels, edge)) == name) {
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
pub fn source_ml_temporary_read_record_type_owner_presence_loop21(starts: &[f64], counts: &[f64], labels: &[String], targets: &[f64], mut pending: Vec<f64>, fields: &[f64], mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((fields.len() as f64))) {
            {
                let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, self::read_required_number_fact(fields, index), "type")], pending); self::ml_array::append(ml_append_left, ml_append_right) };
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
    MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(f64, String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop18(texts: &[String], numbers: &[f64], starts: &[f64], counts: &[f64], labels: &[String], targets: &[f64], bindings: &[f64], mut node: f64, mut kind: String) -> self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result {
    loop {
        return if (kind == "var") {
            {
                let id = self::record_type_fact_field(starts, counts, labels, targets, node, "id");
                {
                    let variable = self::read_required_number_fact(numbers, id);
                    if ((!(self::ml_math::is_integer(variable)) || (variable < (0f64))) || (variable >= ((bindings.len() as f64)))) {
                        self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind.clone())
                    } else {
                        {
                            let bound = self::read_required_number_fact(bindings, variable);
                            {
                                let bound_kind = self::record_type_fact_kind(texts, starts, counts, labels, targets, bound);
                                if (bound_kind == "") {
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
pub fn source_ml_temporary_read_record_type_owner_presence_loop17(texts: &[String], numbers: &[f64], starts: &[f64], counts: &[f64], labels: &[String], targets: &[f64], bindings: &[f64], mut owner: f64, mut pending: Vec<f64>, mut seen: Vec<f64>, mut position: f64) -> self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result {
    loop {
        return if (position < ((pending.len() as f64))) {
            {
                let node = self::read_required_number_fact(&(pending), position);
                {
                    let kind = self::record_type_fact_kind(texts, starts, counts, labels, targets, node);
                    {
                        let source_ml_source1_s22 = self::source_ml_temporary_read_record_type_owner_presence_loop18(texts, numbers, starts, counts, labels, targets, bindings, node, kind.clone());
                        match source_ml_source1_s22.clone() {
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
                                                        if ((((kind_2 == "array") || (kind_2 == "option")) || (kind_2 == "set")) || (kind_2 == "cell")) {
                                                            {
                                                                let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, "element")], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let position_3 = (position + 1f64);
                                                                    {
                                                                        (pending, seen, position) = (pending_2, seen_2, position_3);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if (kind_2 == "map") {
                                                                {
                                                                    let pending_3 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, "key")], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        let pending_4 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, "value")], pending_3); self::ml_array::append(ml_append_left, ml_append_right) };
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
                                                                if (((kind_2 == "tuple") || (kind_2 == "union")) || (kind_2 == "function")) {
                                                                    {
                                                                        let field = if (kind_2 == "tuple") {
                                                                            String::from("elements")
                                                                        } else {
                                                                            if (kind_2 == "union") {
                                                                                String::from("alternatives")
                                                                            } else {
                                                                                String::from("params")
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
                                                                                        if (kind_2 == "function") {
                                                                                            {
                                                                                                let pending_6 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, "ret")], pending_5); self::ml_array::append(ml_append_left, ml_append_right) };
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
                                                                    if (kind_2 == "record") {
                                                                        {
                                                                            let sequence_2 = self::record_type_fact_field(starts, counts, labels, targets, node_2, "fields");
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
                            let source_ml_source1_s23 = self::source_ml_temporary_accept_java_script_record_order_permutation_loop25(order, index, value, previous);
                            match source_ml_source1_s23.clone() {
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
                                    let source_ml_source1_concat_value24 = self::ml_array::slice(&numeric, 0f64, position_2);
                                    {
                                        let source_ml_source1_concat_value25 = vec![field];
                                        {
                                            let source_ml_source1_concat_value26 = self::ml_array::slice(&numeric, position_2, f64::INFINITY);
                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value24), source_ml_source1_concat_value25), source_ml_source1_concat_value26)
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
                            let source_ml_source1_concat_value27 = result;
                            {
                                let source_ml_source1_concat_value28 = vec![field];
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value27), source_ml_source1_concat_value28)
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
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Return(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop31(names: &[Vec<f64>], mut insertion: Vec<f64>, values: &[f64], mut index: f64) -> self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result {
    loop {
        return if (index < ((values.len() as f64))) {
            {
                let field = self::read_java_script_record_order_position(values, index);
                if (((!(self::ml_math::is_integer(field)) || (field < (0f64))) || (field >= ((names.len() as f64)))) || ((field == (0f64)) && (((1f64 / field)) < (0f64)))) {
                    self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Return(String::from("invalid ordered record source field"))
                } else {
                    if !(self::ml_array::includes(&insertion, &field, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                        {
                            let insertion_2 = {
                                let source_ml_source1_concat_value29 = insertion;
                                {
                                    let source_ml_source1_concat_value30 = vec![field];
                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value29), source_ml_source1_concat_value30)
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
                        let source_ml_source1_s31 = self::source_ml_temporary_read_java_script_record_order_merge_error_loop31(names, insertion, &(values), index);
                        match source_ml_source1_s31.clone() {
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
                            let source_ml_source1_concat_value32 = insertion;
                            {
                                let source_ml_source1_concat_value33 = vec![field];
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value32), source_ml_source1_concat_value33)
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
    MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_rebase_error_loop34(target: &[Vec<f64>], source: &[Vec<f64>], mut index: f64) -> self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result {
    loop {
        return if (index < ((source.len() as f64))) {
            if ((self::read_record_field_position(target, &(self::read_java_script_record_order_name(source, index)))) < (0f64)) {
                self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Return(String::from("an ordered record spread field is missing from its target"))
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
                            let source_ml_source1_concat_value34 = result;
                            {
                                let source_ml_source1_concat_value35 = vec![field];
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value34), source_ml_source1_concat_value35)
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
pub fn source_ml_temporary_read_java_script_record_property_keys_loop36(names: &[Vec<f64>], order: &[f64], mut result: Vec<Vec<f64>>, mut index: f64) -> Vec<Vec<f64>> {
    loop {
        return if (index < ((order.len() as f64))) {
            {
                let result_2 = {
                    let source_ml_source1_concat_value36 = result;
                    {
                        let source_ml_source1_concat_value37 = vec![self::read_java_script_record_order_name(names, self::read_java_script_record_order_position(order, index))];
                        self::ml_array::append(self::ml_array::append(Vec::<Vec<f64>>::new(), source_ml_source1_concat_value36), source_ml_source1_concat_value37)
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
