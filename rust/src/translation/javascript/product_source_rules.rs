// Generated from js/src/translation/product-source-rules.js by the checked translator.
// Styles: parity/self-translation/product-source-rules-decorators.lino.
// Do not edit by hand.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

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
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
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
            if a.is_sign_negative() { b } else { a }
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
            if a.is_sign_negative() { a } else { b }
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
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
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
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    fn integer(value: f64) -> f64 {
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
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
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
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
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

pub fn solve_product_source_facts(
    bases: Vec<bool>,
    targets: Vec<f64>,
    sources: Vec<f64>,
    conjunctions: Vec<bool>,
) -> Vec<bool> {
    if (((bases.len() as f64) != (conjunctions.len() as f64))
        || ((targets.len() as f64) != (sources.len() as f64)))
    {
        Vec::<bool>::new()
    } else {
        if !({
            let ml_values1 = targets.clone();
            let ml_callback1 = {
                let bases = bases.clone();
                std::rc::Rc::new(move |index: f64| -> bool {
                    ((self::ml_math::is_safe_integer(index) && (index >= (0f64)))
                        && (index < (bases.len() as f64)))
                })
            };
            ml_values1
                .iter()
                .cloned()
                .enumerate()
                .all(|(ml_index1, ml_value1)| ml_callback1(ml_value1))
        }) {
            Vec::<bool>::new()
        } else {
            if !({
                let ml_values2 = sources.clone();
                let ml_callback2 = {
                    let bases = bases.clone();
                    std::rc::Rc::new(move |index: f64| -> bool {
                        ((self::ml_math::is_safe_integer(index) && (index >= (0f64)))
                            && (index < (bases.len() as f64)))
                    })
                };
                ml_values2
                    .iter()
                    .cloned()
                    .enumerate()
                    .all(|(ml_index2, ml_value2)| ml_callback2(ml_value2))
            }) {
                Vec::<bool>::new()
            } else {
                {
                    let flags = bases.clone();
                    {
                        let round = 0f64;
                        {
                            let flags_2 = self::ml_solve_product_source_facts_loop3(
                                bases.clone(),
                                targets.clone(),
                                sources.clone(),
                                conjunctions.clone(),
                                flags.clone(),
                                round,
                            );
                            flags_2.clone()
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlClosure1Loop2Result {
    MlClosure1Loop2Done(bool, bool, bool),
}

// Checked closure lowering passes its captured state explicitly.
#[allow(clippy::too_many_arguments)]
pub fn ml_closure1_loop2(
    mut targets: Vec<f64>,
    mut previous: Vec<bool>,
    mut sources: Vec<f64>,
    mut index: f64,
    mut found: bool,
    mut any: bool,
    mut all: bool,
    mut edge: f64,
) -> self::MlClosure1Loop2Result {
    loop {
        return if (edge < (targets.len() as f64)) {
            if match (
                self::ml_array::optional_at(&targets, self::ml_array::number_index(edge)),
                index,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } {
                {
                    let found_2 = true;
                    if match (
                        self::ml_array::optional_at(
                            &previous,
                            self::ml_array::number_index(
                                match self::ml_array::optional_at(
                                    &sources,
                                    self::ml_array::number_index(edge),
                                ) {
                                    Some(ml_present) => ml_present,
                                    None => f64::NAN,
                                },
                            ),
                        ),
                        true,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    } {
                        {
                            let any_2 = true;
                            {
                                let edge_2 = (edge + 1f64);
                                {
                                    (found, any, edge) = (found_2, any_2, edge_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let all_2 = false;
                            {
                                let edge_3 = (edge + 1f64);
                                {
                                    (found, all, edge) = (found_2, all_2, edge_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            } else {
                {
                    let edge_4 = (edge + 1f64);
                    {
                        edge = edge_4;
                        continue;
                    }
                }
            }
        } else {
            self::MlClosure1Loop2Result::MlClosure1Loop2Done(found, any, all)
        };
    }
}

pub fn ml_solve_product_source_facts_loop3(
    mut bases: Vec<bool>,
    mut targets: Vec<f64>,
    mut sources: Vec<f64>,
    mut conjunctions: Vec<bool>,
    mut flags: Vec<bool>,
    mut round: f64,
) -> Vec<bool> {
    loop {
        return if (round < (bases.len() as f64)) {
            {
                let previous = flags.clone();
                {
                    let flags_2 = {
                        let ml_values3 = previous.clone();
                        let ml_callback3 = {
                            let targets = targets.clone();
                            let previous = previous.clone();
                            let sources = sources.clone();
                            let conjunctions = conjunctions.clone();
                            std::rc::Rc::new(move |known: bool, index: f64| -> bool {
                                if known {
                                    true
                                } else {
                                    {
                                        let found = false;
                                        {
                                            let any = false;
                                            {
                                                let all = true;
                                                {
                                                    let edge = 0f64;
                                                    {
                                                        let ml_s1 = self::ml_closure1_loop2(
                                                            targets.clone(),
                                                            previous.clone(),
                                                            sources.clone(),
                                                            index,
                                                            found,
                                                            any,
                                                            all,
                                                            edge,
                                                        );
                                                        match ml_s1.clone() {
                                                self::MlClosure1Loop2Result::MlClosure1Loop2Done(found_2, any_2, all_2) => {
                                                    if match self::ml_array::optional_at(&conjunctions, self::ml_array::number_index(index)) { Some(ml_present) => ml_present, None => false } {
                                                        (found_2 && all_2)
                                                    } else {
                                                        any_2
                                                    }
                                                }
                                            }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })
                        };
                        ml_values3
                            .iter()
                            .cloned()
                            .enumerate()
                            .map(|(ml_index3, ml_value3)| {
                                ml_callback3(ml_value3, (ml_index3 as f64))
                            })
                            .collect::<Vec<_>>()
                    };
                    {
                        let round_2 = (round + 1f64);
                        {
                            (flags, round) = (flags_2.clone(), round_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            flags.clone()
        };
    }
}
