// Generated from complete js/src/translation/unicode-scalar-rules.js. Do not edit by hand.
use super::MlNormalizeUnicodeScalarsLoop3Result;
use super::ml_normalize_unicode_scalars_loop3;
use super::ml_normalize_unicode_scalars_loop4;

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
pub fn accept_unicode_scalar(value: f64) -> bool {
    ((self::ml_math::is_safe_integer(value) && (value >= 0f64)) && (value <= 1_114_111f64))
        && !((55_296f64..=57_343f64).contains(&value))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn decode_unicode_source_scalar(first: f64, second: f64) -> f64 {
    if (((55_296f64..=56_319f64).contains(&first)) && (second >= 56_320f64))
        && (second <= 57_343f64)
    {
        ((65_536f64 + ((first - 55_296f64) * 1024f64)) + second) - 56_320f64
    } else if self::accept_unicode_scalar(first) && (first <= 65_535f64) {
        first
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unicode_source_scalar_width(scalar: f64) -> f64 {
    if scalar > 65_535f64 { 2f64 } else { 1f64 }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_unicode_property_scalar(
    ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    scalar: f64,
) -> bool {
    if (self::accept_unicode_scalar(scalar)) {
        {
            let low = 0f64;
            {
                let high = ((({ ranges.borrow().clone() }).len() as f64) / 2f64).floor();
                {
                    let ml_s1 = self::ml_accept_unicode_property_scalar_loop1(
                        ranges.clone(),
                        scalar,
                        low,
                        high,
                    );
                    match ml_s1 {
                        self::MlAcceptUnicodePropertyScalarLoop1Result::MlAcceptUnicodePropertyScalarLoop1Done => {
                            false
                        }
                        self::MlAcceptUnicodePropertyScalarLoop1Result::MlAcceptUnicodePropertyScalarLoop1Return(ml_result1) => {
                            ml_result1
                        }
                    }
                }
            }
        }
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_identifier_scalar(
    language: &str,
    first: bool,
    scalar: f64,
    starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    continuations: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> bool {
    if (self::accept_unicode_scalar(scalar)) {
        if (language != "JavaScript") && (language != "Rust") {
            false
        } else if (((scalar == 95f64) || ((language == "JavaScript") && (scalar == 36f64)))
            || (((65f64..=90f64).contains(&scalar)) || ((97f64..=122f64).contains(&scalar))))
            || ((!first && (scalar >= 48f64)) && (scalar <= 57f64))
        {
            true
        } else if scalar < 128f64 {
            false
        } else if ((language == "JavaScript") && !first)
            && ((scalar == 8204f64) || (scalar == 8205f64))
        {
            true
        } else {
            self::accept_unicode_property_scalar(
                if first {
                    starts.clone()
                } else {
                    continuations.clone()
                },
                scalar,
            )
        }
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_raw_identifier_error(name: &str, admitted: bool) -> String {
    if ((((!admitted || (name == "_")) || (name == "crate")) || (name == "self"))
        || (name == "Self"))
        || (name == "super")
    {
        format!("invalid Rust raw identifier r#{}", name.to_owned())
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_unicode_table_row(
    rows: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    stride: f64,
    key: f64,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = ((({ rows.borrow().clone() }).len() as f64) / stride).floor();
            {
                let ml_s2 =
                    self::ml_find_unicode_table_row_loop2(rows.clone(), stride, key, low, high);
                match ml_s2 {
                    self::MlFindUnicodeTableRowLoop2Result::MlFindUnicodeTableRowLoop2Done => -1f64,
                    self::MlFindUnicodeTableRowLoop2Result::MlFindUnicodeTableRowLoop2Return(
                        ml_result2,
                    ) => ml_result2,
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unicode_combining_class(
    classes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    scalar: f64,
) -> f64 {
    {
        let row = self::find_unicode_table_row(classes.clone(), 2f64, scalar);
        if row < 0f64 {
            0f64
        } else {
            (self::ml_array::optional_at(
                &{ classes.borrow().clone() },
                self::ml_array::number_index(row + 1f64),
            ))
            .unwrap_or_else(|| 0f64)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compose_unicode_scalars(
    pairs: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    first: f64,
    second: f64,
) -> f64 {
    if (((4352f64..4371f64).contains(&first)) && (second >= 4449f64)) && (second < 4470f64) {
        44_032f64 + (((((first - 4352f64) * 21f64) + second) - 4449f64) * 28f64)
    } else if ((((44_032f64..55_204f64).contains(&first))
        && (((first - 44_032f64) % 28f64) == 0f64))
        && (second > 4519f64))
        && (second < 4547f64)
    {
        (first + second) - 4519f64
    } else {
        {
            let row = self::find_unicode_table_row(
                pairs.clone(),
                2f64,
                ((first * 1_114_112f64) + second),
            );
            if row < 0f64 {
                -1f64
            } else {
                (self::ml_array::optional_at(
                    &{ pairs.borrow().clone() },
                    self::ml_array::number_index(row + 1f64),
                ))
                .unwrap_or_else(|| -1f64)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_unicode_scalars(
    input: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    classes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    decompositions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    pairs: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    limit: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        assert!(
            !((!(self::ml_math::is_safe_integer(limit)) || (limit < 1f64))
                || ((({ input.borrow().clone() }).len() as f64) > limit)),
            "{}",
            "Unicode normalization resource limit exceeded"
        );

        {
            let stack = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let index = ({ input.borrow().clone() }).len() as f64;
                {
                    let ml_s3 = self::ml_normalize_unicode_scalars_loop3(
                        input.clone(),
                        stack.clone(),
                        index,
                    );
                    match ml_s3 {
                        self::MlNormalizeUnicodeScalarsLoop3Result::MlNormalizeUnicodeScalarsLoop3Done => {
                            {
                                let ordered = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                {
                                    let steps = 0f64;
                                    {
                                        let ml_s4 = self::ml_normalize_unicode_scalars_loop4(classes.clone(), decompositions.clone(), values.clone(), limit, stack.clone(), ordered.clone(), steps);
                                        match ml_s4 {
                                            self::MlNormalizeUnicodeScalarsLoop4Result::MlNormalizeUnicodeScalarsLoop4Done => {
                                                {
                                                    let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                    {
                                                        let starter = -1f64;
                                                        {
                                                            let previous_class = 0f64;
                                                            {
                                                                let index_2 = 0f64;
                                                                {
                                                                    let ml_s5 = self::ml_normalize_unicode_scalars_loop5(classes.clone(), pairs.clone(), ordered.clone(), result.clone(), starter, previous_class, index_2);
                                                                    match ml_s5 {
                                                                        self::MlNormalizeUnicodeScalarsLoop5Result::MlNormalizeUnicodeScalarsLoop5Done => {
                                                                            result.clone()
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptUnicodePropertyScalarLoop1Result {
    MlAcceptUnicodePropertyScalarLoop1Done,
    MlAcceptUnicodePropertyScalarLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_unicode_property_scalar_loop1(
    mut ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut scalar: f64,
    mut low: f64,
    mut high: f64,
) -> self::MlAcceptUnicodePropertyScalarLoop1Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let first = (self::ml_array::optional_at(
                        &{ ranges.borrow().clone() },
                        self::ml_array::number_index(middle * 2f64),
                    ))
                    .unwrap_or_else(|| -1f64);
                    {
                        let last = (self::ml_array::optional_at(
                            &{ ranges.borrow().clone() },
                            self::ml_array::number_index((middle * 2f64) + 1f64),
                        ))
                        .unwrap_or_else(|| -1f64);
                        if scalar < first {
                            {
                                let high_2 = middle;
                                {
                                    high = high_2;
                                    continue;
                                }
                            }
                        }
                        {
                            if scalar > last {
                                {
                                    let low_2 = middle + 1f64;
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlAcceptUnicodePropertyScalarLoop1Result::MlAcceptUnicodePropertyScalarLoop1Return(true)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptUnicodePropertyScalarLoop1Result::MlAcceptUnicodePropertyScalarLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindUnicodeTableRowLoop2Result {
    MlFindUnicodeTableRowLoop2Done,
    MlFindUnicodeTableRowLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_find_unicode_table_row_loop2(
    mut rows: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut stride: f64,
    mut key: f64,
    mut low: f64,
    mut high: f64,
) -> self::MlFindUnicodeTableRowLoop2Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let value = (self::ml_array::optional_at(
                        &{ rows.borrow().clone() },
                        self::ml_array::number_index(middle * stride),
                    ))
                    .unwrap_or_else(|| -1f64);
                    if key < value {
                        {
                            let high_2 = middle;
                            {
                                high = high_2;
                                continue;
                            }
                        }
                    }
                    {
                        if key > value {
                            {
                                let low_2 = middle + 1f64;
                                {
                                    low = low_2;
                                    continue;
                                }
                            }
                        }
                        {
                            self::MlFindUnicodeTableRowLoop2Result::MlFindUnicodeTableRowLoop2Return(
                                middle * stride,
                            )
                        }
                    }
                }
            }
        } else {
            self::MlFindUnicodeTableRowLoop2Result::MlFindUnicodeTableRowLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlNormalizeUnicodeScalarsLoop5Result {
    MlNormalizeUnicodeScalarsLoop5Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_normalize_unicode_scalars_loop5(
    mut classes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut pairs: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut ordered: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut starter: f64,
    mut previous_class: f64,
    mut index: f64,
) -> self::MlNormalizeUnicodeScalarsLoop5Result {
    loop {
        if index < (({ ordered.borrow().clone() }).len() as f64) {
            {
                let scalar = (self::ml_array::optional_at(
                    &{ ordered.borrow().clone() },
                    self::ml_array::number_index(index),
                ))
                .unwrap_or_else(|| -1f64);
                {
                    let combining = self::read_unicode_combining_class(classes.clone(), scalar);
                    {
                        let composition = if starter < 0f64 {
                            -1f64
                        } else {
                            self::compose_unicode_scalars(
                                pairs.clone(),
                                (self::ml_array::optional_at(
                                    &{ result.borrow().clone() },
                                    self::ml_array::number_index(starter),
                                ))
                                .unwrap_or_else(|| -1f64),
                                scalar,
                            )
                        };
                        if (composition >= 0f64)
                            && ((previous_class == 0f64) || (previous_class < combining))
                        {
                            if (starter >= 0f64)
                                && (starter < (({ result.borrow().clone() }).len() as f64))
                            {
                                {
                                    let ml_normalize_unicode_scalars_ignored6 = {
                                        let ml_array_owner14 = result.clone();
                                        let ml_array_argument14_0: f64 = starter;
                                        let ml_array_argument14_1: f64 = 1f64;
                                        let ml_array_argument14_2: f64 = composition;
                                        let ml_array_length = ml_array_owner14.borrow().len();
                                        let ml_array_start = {
                                            let ml_relative_value: f64 = ml_array_argument14_0;
                                            let ml_relative = if ml_relative_value.is_nan() {
                                                0f64
                                            } else {
                                                ml_relative_value.trunc()
                                            };
                                            let ml_bound = ml_array_length as f64;
                                            (if ml_relative < 0f64 {
                                                (ml_bound + ml_relative).clamp(0f64, ml_bound)
                                            } else {
                                                ml_relative.min(ml_bound)
                                            }) as usize
                                        };
                                        let ml_array_count = {
                                            let ml_delete_value: f64 = ml_array_argument14_1;
                                            ml_delete_value
                                                .trunc()
                                                .max(0f64)
                                                .min((ml_array_length - ml_array_start) as f64)
                                                as usize
                                        };
                                        let ml_array_inserted = vec![ml_array_argument14_2];
                                        let _ = (ml_array_length - ml_array_count)
                                            .checked_add(ml_array_inserted.len())
                                            .filter(|length| *length <= 4_294_967_295usize)
                                            .expect("Invalid array length");
                                        let ml_removed: Vec<_> = ml_array_owner14
                                            .borrow_mut()
                                            .splice(
                                                ml_array_start..ml_array_start + ml_array_count,
                                                ml_array_inserted,
                                            )
                                            .collect();
                                        std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                                    };
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            index = index_2;
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let index_3 = index + 1f64;
                                    {
                                        index = index_3;
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let starter_3 = if combining == 0f64 {
                                    { ({ result.borrow().clone() }).len() as f64 }
                                } else {
                                    starter
                                };
                                {
                                    let ml_normalize_unicode_scalars_ignored7 = {
                                        let ml_method_owner16 = result.clone();
                                        let ml_method_argument16_0 = scalar;
                                        let ml_inserted = vec![ml_method_argument16_0];
                                        let mut ml_method_values16 = ml_method_owner16.borrow_mut();
                                        let ml_length = ml_method_values16
                                            .len()
                                            .checked_add(ml_inserted.len())
                                            .filter(|length| *length <= 4_294_967_295usize)
                                            .expect("Invalid array length");
                                        ml_method_values16.extend(ml_inserted);
                                        ml_length as f64
                                    };
                                    {
                                        let previous_class_2 = combining;
                                        {
                                            let index_4 = index + 1f64;
                                            {
                                                (starter, previous_class, index) =
                                                    (starter_3, previous_class_2, index_4);
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
        return { self::MlNormalizeUnicodeScalarsLoop5Result::MlNormalizeUnicodeScalarsLoop5Done };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlNormalizeUnicodeScalarsLoop4Result {
    MlNormalizeUnicodeScalarsLoop4Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlNormalizeUnicodeScalarsJoin8 {
    MlNormalizeUnicodeScalarsJoin8Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlNormalizeUnicodeScalarsLoop12Result {
    MlNormalizeUnicodeScalarsLoop12Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_normalize_unicode_scalars_loop12(
    mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut stack: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut offset: f64,
    mut index: f64,
) -> self::MlNormalizeUnicodeScalarsLoop12Result {
    loop {
        if index > 0f64 {
            {
                let ml_normalize_unicode_scalars_ignored13 = {
                    let ml_method_owner17 = stack.clone();
                    let ml_method_argument17_0 = (self::ml_array::optional_at(
                        &{ values.borrow().clone() },
                        self::ml_array::number_index((offset + index) - 1f64),
                    ))
                    .unwrap_or_else(|| -1f64);
                    let ml_inserted = vec![ml_method_argument17_0];
                    let mut ml_method_values17 = ml_method_owner17.borrow_mut();
                    let ml_length = ml_method_values17
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values17.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let index_2 = index - 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        }
        return {
            self::MlNormalizeUnicodeScalarsLoop12Result::MlNormalizeUnicodeScalarsLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlNormalizeUnicodeScalarsLoop15Result {
    MlNormalizeUnicodeScalarsLoop15Done(f64, f64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlNormalizeUnicodeScalarsJoin17 {
    MlNormalizeUnicodeScalarsJoin17Next,
}
