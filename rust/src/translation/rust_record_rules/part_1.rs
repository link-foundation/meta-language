// Generated from complete js/src/translation/rust-record-rules.js. Do not edit by hand.
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
pub fn plan_rust_record_fields(declared: &[String], supplied: &[String]) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let ml_s1 = self::ml_plan_rust_record_fields_loop1(declared, index);
            match ml_s1 {
                self::MlPlanRustRecordFieldsLoop1Result::MlPlanRustRecordFieldsLoop1Done => {
                    let index_2 = 0f64;
                    {
                        let ml_s2 =
                            self::ml_plan_rust_record_fields_loop2(declared, supplied, index_2);
                        match ml_s2 {
                                self::MlPlanRustRecordFieldsLoop2Result::MlPlanRustRecordFieldsLoop2Done => {
                                    {
                                        let result = Vec::<f64>::new();
                                        {
                                            let index_3 = 0f64;
                                            {
                                                let ml_s3 = self::ml_plan_rust_record_fields_loop3(declared, supplied, result, index_3);
                                                match ml_s3 {
                                                    self::MlPlanRustRecordFieldsLoop3Result::MlPlanRustRecordFieldsLoop3Done(result_2) => {
                                                        result_2
                                                    }
                                                    self::MlPlanRustRecordFieldsLoop3Result::MlPlanRustRecordFieldsLoop3Return(ml_result3) => {
                                                        ml_result3
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlPlanRustRecordFieldsLoop2Result::MlPlanRustRecordFieldsLoop2Return(ml_result3_2) => {
                                    ml_result3_2
                                }
                            }
                    }
                }
                self::MlPlanRustRecordFieldsLoop1Result::MlPlanRustRecordFieldsLoop1Return(
                    ml_result3_3,
                ) => ml_result3_3,
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_record_field_position(declared: &[String], field: &str) -> f64 {
    self::ml_array::index_of(declared, &field.to_owned(), 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_record_pattern_fields(
    declared: &[String],
    supplied: &[String],
    rest: bool,
) -> Vec<f64> {
    {
        let checked = self::plan_rust_record_fields(declared, supplied);
        {
            let code = (self::ml_array::optional_at(&checked, self::ml_array::number_index(0f64)))
                .unwrap_or_else(|| 0f64);
            if (code < 0f64) && !((code == -4f64) && rest) {
                checked
            } else {
                {
                    let result = vec![0f64];
                    {
                        let index = 0f64;
                        {
                            self::ml_plan_rust_record_pattern_fields_loop4(
                                declared, supplied, result, index,
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_record_update_fields(declared: &[String], supplied: &[String]) -> Vec<f64> {
    self::plan_rust_record_pattern_fields(declared, supplied, true)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_record_update_owner(
    structure: bool,
    known: bool,
    actual: &str,
    expected: &str,
    wrappers: f64,
    diverging: bool,
) -> bool {
    structure && (diverging || ((known && (actual == expected)) && (wrappers == 0f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_record_binding_names(reserved: &[String], count: f64) -> Vec<String> {
    {
        let names = Vec::<String>::new();
        {
            let suffix = 0f64;
            { self::ml_read_rust_record_binding_names_loop5(reserved, count, names, suffix) }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_record_source_name(name: &str) -> bool {
    !(self::ml_array::includes(
        &vec![
            String::from("bool"),
            String::from("char"),
            String::from("str"),
            String::from("String"),
            String::from("Vec"),
            String::from("Box"),
            String::from("f32"),
            String::from("f64"),
            String::from("usize"),
            String::from("isize"),
            String::from("u8"),
            String::from("i8"),
            String::from("u16"),
            String::from("i16"),
            String::from("u32"),
            String::from("i32"),
            String::from("u64"),
            String::from("i64"),
            String::from("u128"),
            String::from("i128"),
            String::from("std"),
            String::from("core"),
            String::from("alloc"),
        ],
        &name.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ))
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustRecordFieldsLoop3Result {
    MlPlanRustRecordFieldsLoop3Done(Vec<f64>),
    MlPlanRustRecordFieldsLoop3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_record_fields_loop3(
    declared: &[String],
    supplied: &[String],
    mut result: Vec<f64>,
    mut index: f64,
) -> self::MlPlanRustRecordFieldsLoop3Result {
    loop {
        return if index < (declared.len() as f64) {
            {
                let position = self::ml_array::index_of(
                    supplied,
                    &(self::ml_array::optional_at(declared, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new()),
                    0f64,
                    false,
                    |a, b| a == b,
                );
                if position < 0f64 {
                    self::MlPlanRustRecordFieldsLoop3Result::MlPlanRustRecordFieldsLoop3Return(
                        vec![-4f64, index],
                    )
                } else {
                    {
                        let result_2 = {
                            let (ml_append_right, ml_append_left) = (vec![position], result);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (result, index) = (result_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlPlanRustRecordFieldsLoop3Result::MlPlanRustRecordFieldsLoop3Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustRecordFieldsLoop2Result {
    MlPlanRustRecordFieldsLoop2Done,
    MlPlanRustRecordFieldsLoop2Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_record_fields_loop2(
    declared: &[String],
    supplied: &[String],
    mut index: f64,
) -> self::MlPlanRustRecordFieldsLoop2Result {
    loop {
        return if index < (supplied.len() as f64) {
            {
                let name =
                    (self::ml_array::optional_at(supplied, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new());
                {
                    let ml_library_inequality_1 =
                        (self::ml_array::index_of(supplied, &name.clone(), 0f64, false, |a, b| {
                            a == b
                        })) != index;
                    if ml_library_inequality_1 {
                        self::MlPlanRustRecordFieldsLoop2Result::MlPlanRustRecordFieldsLoop2Return(
                            vec![-2f64, index],
                        )
                    } else {
                        if (self::ml_array::includes(
                            declared,
                            &name.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )) {
                            {
                                let index_2 = index + 1f64;
                                {
                                    index = index_2;
                                    continue;
                                }
                            }
                        }
                        {
                            self::MlPlanRustRecordFieldsLoop2Result::MlPlanRustRecordFieldsLoop2Return(vec![-3f64, index])
                        }
                    }
                }
            }
        } else {
            self::MlPlanRustRecordFieldsLoop2Result::MlPlanRustRecordFieldsLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustRecordFieldsLoop1Result {
    MlPlanRustRecordFieldsLoop1Done,
    MlPlanRustRecordFieldsLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_record_fields_loop1(
    declared: &[String],
    mut index: f64,
) -> self::MlPlanRustRecordFieldsLoop1Result {
    loop {
        return if index < (declared.len() as f64) {
            {
                let name =
                    (self::ml_array::optional_at(declared, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new());
                {
                    let ml_library_inequality_1 =
                        (self::ml_array::index_of(declared, &name, 0f64, false, |a, b| a == b))
                            != index;
                    if ml_library_inequality_1 {
                        self::MlPlanRustRecordFieldsLoop1Result::MlPlanRustRecordFieldsLoop1Return(
                            vec![-1f64, index],
                        )
                    } else {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlPlanRustRecordFieldsLoop1Result::MlPlanRustRecordFieldsLoop1Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_record_pattern_fields_loop4(
    declared: &[String],
    supplied: &[String],
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (declared.len() as f64) {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::ml_array::index_of(
                            supplied,
                            &(self::ml_array::optional_at(
                                declared,
                                self::ml_array::number_index(index),
                            ))
                            .unwrap_or_else(|| String::new()),
                            0f64,
                            false,
                            |a, b| a == b,
                        )],
                        result,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_rust_record_binding_names_loop5(
    reserved: &[String],
    mut count: f64,
    mut names: Vec<String>,
    mut suffix: f64,
) -> Vec<String> {
    loop {
        if (names.len() as f64) < count {
            {
                let name = format!("ml_record_value{}", self::ml_number::js_number(suffix));
                {
                    let suffix_2 = suffix + 1f64;
                    if !(self::ml_array::includes(reserved, &name.clone(), 0f64, false, |a, b| {
                        a == b
                    })) {
                        {
                            let names_2 = {
                                let (ml_append_right, ml_append_left) = (vec![name.clone()], names);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                (names, suffix) = (names_2, suffix_2);
                                continue;
                            }
                        }
                    }
                    {
                        {
                            suffix = suffix_2;
                            continue;
                        }
                    }
                }
            }
        }
        return { names };
    }
}
