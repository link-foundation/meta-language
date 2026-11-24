// Generated from js/src/global-concept-rules.js by
// js/scripts/generate-global-concept-rules.mjs. Do not edit by hand.
// Translated from JavaScript by meta-language: portable core, Rust target.
// JavaScript Number equality and separate arithmetic rounding are preserved.
// Array index casts are guarded by the generated runtime.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons,
    clippy::float_cmp,
    clippy::suboptimal_flops,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
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

#[must_use]
pub fn verify_global_concept_correspondence(
    shapes: Vec<String>,
    meanings: Vec<String>,
    offsets: Vec<f64>,
    edges: Vec<f64>,
    first: Vec<f64>,
    second: Vec<f64>,
) -> bool {
    if (((((first.len() as f64) == (0f64)) || ((first.len() as f64) != (second.len() as f64)))
        || ((shapes.len() as f64) != (meanings.len() as f64)))
        || ((offsets.len() as f64) != ((shapes.len() as f64) + 1f64)))
    {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source1_s1 = crate::global_concept_rules::source_ml_temporary_verify_global_concept_correspondence_loop1(shapes.clone(), meanings.clone(), offsets.clone(), edges.clone(), first.clone(), second.clone(), at);
                match source_ml_source1_s1.clone() {
                    crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Done => {
                        true
                    }
                    crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Return(source_ml_temporary_result1) => {
                        source_ml_temporary_result1
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Result {
    MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Done,
    MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryVerifyGlobalConceptCorrespondenceLoop3Result {
    MlTemporaryVerifyGlobalConceptCorrespondenceLoop3Done,
    MlTemporaryVerifyGlobalConceptCorrespondenceLoop3Return(bool),
}

pub fn source_ml_temporary_verify_global_concept_correspondence_loop4(
    mut first: Vec<f64>,
    mut second: Vec<f64>,
    mut left_target: Option<f64>,
    mut right_target: Option<f64>,
    mut pair: f64,
    mut found: bool,
) -> bool {
    loop {
        return if (pair < (first.len() as f64)) {
            {
                let found_3 = if ({
                    let source_ml_source1_comparison_operand2 =
                        crate::global_concept_rules::ml_array::optional_at(
                            &first,
                            crate::global_concept_rules::ml_array::number_index(pair),
                        );
                    {
                        let source_ml_source1_comparison_operand3 = left_target.clone();
                        match (
                            source_ml_source1_comparison_operand2.clone(),
                            source_ml_source1_comparison_operand3.clone(),
                        ) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        }
                    }
                } && {
                    let source_ml_source1_comparison_operand4 =
                        crate::global_concept_rules::ml_array::optional_at(
                            &second,
                            crate::global_concept_rules::ml_array::number_index(pair),
                        );
                    {
                        let source_ml_source1_comparison_operand5 = right_target.clone();
                        match (
                            source_ml_source1_comparison_operand4.clone(),
                            source_ml_source1_comparison_operand5.clone(),
                        ) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        }
                    }
                }) {
                    { true }
                } else {
                    found
                };
                {
                    let pair_2 = (pair + 1f64);
                    {
                        (pair, found) = (pair_2, found_3);
                        continue;
                    }
                }
            }
        } else {
            found
        };
    }
}

pub fn source_ml_temporary_verify_global_concept_correspondence_loop3(
    mut edges: Vec<f64>,
    mut first: Vec<f64>,
    mut second: Vec<f64>,
    mut left_start: Option<f64>,
    mut right_start: Option<f64>,
    mut count: f64,
    mut edge: f64,
) -> crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop3Result {
    loop {
        return if (edge < count) {
            {
                let left_target = crate::global_concept_rules::ml_array::optional_at(
                    &edges,
                    crate::global_concept_rules::ml_array::number_index(
                        (left_start.unwrap_or(f64::NAN) + edge),
                    ),
                );
                {
                    let right_target = crate::global_concept_rules::ml_array::optional_at(
                        &edges,
                        crate::global_concept_rules::ml_array::number_index(
                            (right_start.unwrap_or(f64::NAN) + edge),
                        ),
                    );
                    {
                        let pair = 0f64;
                        {
                            let found = false;
                            {
                                let found_2 = crate::global_concept_rules::source_ml_temporary_verify_global_concept_correspondence_loop4(first.clone(), second.clone(), left_target.clone(), right_target.clone(), pair, found);
                                if !found_2 {
                                    crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop3Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop3Return(false)
                                } else {
                                    {
                                        let edge_2 = (edge + 1f64);
                                        {
                                            edge = edge_2;
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
            crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop3Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryVerifyGlobalConceptCorrespondenceLoop2Result {
    MlTemporaryVerifyGlobalConceptCorrespondenceLoop2Done,
    MlTemporaryVerifyGlobalConceptCorrespondenceLoop2Return(bool),
}

pub fn source_ml_temporary_verify_global_concept_correspondence_loop2(
    mut first: Vec<f64>,
    mut second: Vec<f64>,
    mut at: f64,
    mut left: Option<f64>,
    mut right: Option<f64>,
    mut other: f64,
) -> crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop2Result {
    loop {
        return if (other < at) {
            if ({
                let source_ml_source1_comparison_operand6 =
                    crate::global_concept_rules::ml_array::optional_at(
                        &first,
                        crate::global_concept_rules::ml_array::number_index(other),
                    );
                {
                    let source_ml_source1_comparison_operand7 = left.clone();
                    match (
                        source_ml_source1_comparison_operand6.clone(),
                        source_ml_source1_comparison_operand7.clone(),
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            } || {
                let source_ml_source1_comparison_operand8 =
                    crate::global_concept_rules::ml_array::optional_at(
                        &second,
                        crate::global_concept_rules::ml_array::number_index(other),
                    );
                {
                    let source_ml_source1_comparison_operand9 = right.clone();
                    match (
                        source_ml_source1_comparison_operand8.clone(),
                        source_ml_source1_comparison_operand9.clone(),
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            }) {
                crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop2Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop2Return(false)
            } else {
                {
                    let other_2 = (other + 1f64);
                    {
                        other = other_2;
                        continue;
                    }
                }
            }
        } else {
            crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop2Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop2Done
        };
    }
}

pub fn source_ml_temporary_verify_global_concept_correspondence_loop1(
    mut shapes: Vec<String>,
    mut meanings: Vec<String>,
    mut offsets: Vec<f64>,
    mut edges: Vec<f64>,
    mut first: Vec<f64>,
    mut second: Vec<f64>,
    mut at: f64,
) -> crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Result {
    loop {
        return if (at < (first.len() as f64)) {
            {
                let left = crate::global_concept_rules::ml_array::optional_at(
                    &first,
                    crate::global_concept_rules::ml_array::number_index(at),
                );
                {
                    let right = crate::global_concept_rules::ml_array::optional_at(
                        &second,
                        crate::global_concept_rules::ml_array::number_index(at),
                    );
                    if ((((({
                        let source_ml_source1_comparison_operand10 = left.clone();
                        {
                            let source_ml_source1_comparison_operand11 = 0f64;
                            if match (source_ml_source1_comparison_operand10.clone(), None::<f64>) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                ((f64::NAN) < source_ml_source1_comparison_operand11)
                            } else {
                                ((source_ml_source1_comparison_operand10
                                    .clone()
                                    .expect("a proven present value"))
                                    < source_ml_source1_comparison_operand11)
                            }
                        }
                    } || {
                        let source_ml_source1_comparison_operand12 = right.clone();
                        {
                            let source_ml_source1_comparison_operand13 = 0f64;
                            if match (source_ml_source1_comparison_operand12.clone(), None::<f64>) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                ((f64::NAN) < source_ml_source1_comparison_operand13)
                            } else {
                                ((source_ml_source1_comparison_operand12
                                    .clone()
                                    .expect("a proven present value"))
                                    < source_ml_source1_comparison_operand13)
                            }
                        }
                    }) || ((match left.clone() {
                        Some(ml_present) => ml_present,
                        None => f64::NAN,
                    } % 1f64)
                        != (0f64)))
                        || ((right.unwrap_or(f64::NAN) % 1f64) != (0f64)))
                        || {
                            let source_ml_source1_comparison_operand14 = left.clone();
                            {
                                let source_ml_source1_comparison_operand15 = (shapes.len() as f64);
                                if match (
                                    source_ml_source1_comparison_operand14.clone(),
                                    None::<f64>,
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    ((f64::NAN) >= source_ml_source1_comparison_operand15)
                                } else {
                                    ((source_ml_source1_comparison_operand14
                                        .clone()
                                        .expect("a proven present value"))
                                        >= source_ml_source1_comparison_operand15)
                                }
                            }
                        })
                        || {
                            let source_ml_source1_comparison_operand16 = right.clone();
                            {
                                let source_ml_source1_comparison_operand17 = (shapes.len() as f64);
                                if match (
                                    source_ml_source1_comparison_operand16.clone(),
                                    None::<f64>,
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    ((f64::NAN) >= source_ml_source1_comparison_operand17)
                                } else {
                                    ((source_ml_source1_comparison_operand16
                                        .clone()
                                        .expect("a proven present value"))
                                        >= source_ml_source1_comparison_operand17)
                                }
                            }
                        })
                    {
                        crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Return(false)
                    } else {
                        if ({
                            let source_ml_source1_comparison_operand18 =
                                crate::global_concept_rules::ml_array::optional_at(
                                    &shapes,
                                    crate::global_concept_rules::ml_array::number_index(match left
                                        .clone()
                                    {
                                        Some(ml_present) => ml_present,
                                        None => f64::NAN,
                                    }),
                                );
                            {
                                let source_ml_source1_comparison_operand19 =
                                    crate::global_concept_rules::ml_array::optional_at(
                                        &shapes,
                                        crate::global_concept_rules::ml_array::number_index(
                                            match right.clone() {
                                                Some(ml_present) => ml_present,
                                                None => f64::NAN,
                                            },
                                        ),
                                    );
                                !(match (
                                    source_ml_source1_comparison_operand18.clone(),
                                    source_ml_source1_comparison_operand19.clone(),
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                })
                            }
                        } || {
                            let source_ml_source1_comparison_operand20 =
                                crate::global_concept_rules::ml_array::optional_at(
                                    &meanings,
                                    crate::global_concept_rules::ml_array::number_index(match left
                                        .clone()
                                    {
                                        Some(ml_present) => ml_present,
                                        None => f64::NAN,
                                    }),
                                );
                            {
                                let source_ml_source1_comparison_operand21 =
                                    crate::global_concept_rules::ml_array::optional_at(
                                        &meanings,
                                        crate::global_concept_rules::ml_array::number_index(
                                            match right.clone() {
                                                Some(ml_present) => ml_present,
                                                None => f64::NAN,
                                            },
                                        ),
                                    );
                                !(match (
                                    source_ml_source1_comparison_operand20.clone(),
                                    source_ml_source1_comparison_operand21.clone(),
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                })
                            }
                        }) {
                            crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Return(false)
                        } else {
                            {
                                let other = 0f64;
                                {
                                    let source_ml_source1_s22 = crate::global_concept_rules::source_ml_temporary_verify_global_concept_correspondence_loop2(first.clone(), second.clone(), at, left.clone(), right.clone(), other);
                                    match source_ml_source1_s22.clone() {
                                        crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop2Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop2Done => {
                                            {
                                                let left_start = crate::global_concept_rules::ml_array::optional_at(&offsets, crate::global_concept_rules::ml_array::number_index(left.unwrap_or(f64::NAN)));
                                                {
                                                    let right_start = crate::global_concept_rules::ml_array::optional_at(&offsets, crate::global_concept_rules::ml_array::number_index(right.unwrap_or(f64::NAN)));
                                                    {
                                                        let count = (crate::global_concept_rules::ml_array::optional_at(&offsets, crate::global_concept_rules::ml_array::number_index(left.unwrap_or(f64::NAN) + 1f64)).unwrap_or(f64::NAN) - left_start.unwrap_or(f64::NAN));
                                                        if (((((((({
                                                            let source_ml_source1_comparison_operand23 = left_start.clone();
                                                            {
                                                                let source_ml_source1_comparison_operand24 = 0f64;
                                                                if match (source_ml_source1_comparison_operand23.clone(), None::<f64>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                                                                    ((f64::NAN) < source_ml_source1_comparison_operand24)
                                                                } else {
                                                                    ((source_ml_source1_comparison_operand23.clone().expect("a proven present value")) < source_ml_source1_comparison_operand24)
                                                                }
                                                            }
                                                        } || {
                                                            let source_ml_source1_comparison_operand25 = right_start.clone();
                                                            {
                                                                let source_ml_source1_comparison_operand26 = 0f64;
                                                                if match (source_ml_source1_comparison_operand25.clone(), None::<f64>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                                                                    ((f64::NAN) < source_ml_source1_comparison_operand26)
                                                                } else {
                                                                    ((source_ml_source1_comparison_operand25.clone().expect("a proven present value")) < source_ml_source1_comparison_operand26)
                                                                }
                                                            }
                                                        }) || (((match left_start.clone() { Some(ml_present) => ml_present, None => f64::NAN } % 1f64)) != (0f64))) || (((match right_start.clone() { Some(ml_present) => ml_present, None => f64::NAN } % 1f64)) != (0f64))) || (count < (0f64))) || (((count % 1f64)) != (0f64))) || (count != ((match crate::global_concept_rules::ml_array::optional_at(&offsets, crate::global_concept_rules::ml_array::number_index((match right.clone() { Some(ml_present) => ml_present, None => f64::NAN } + 1f64))) { Some(ml_present) => ml_present, None => f64::NAN } - match right_start.clone() { Some(ml_present) => ml_present, None => f64::NAN })))) || (((match left_start.clone() { Some(ml_present) => ml_present, None => f64::NAN } + count)) > ((edges.len() as f64)))) || (((match right_start.clone() { Some(ml_present) => ml_present, None => f64::NAN } + count)) > ((edges.len() as f64)))) {
                                                            crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Return(false)
                                                        } else {
                                                            {
                                                                let edge = 0f64;
                                                                {
                                                                    let source_ml_source1_s27 = crate::global_concept_rules::source_ml_temporary_verify_global_concept_correspondence_loop3(edges.clone(), first.clone(), second.clone(), left_start.clone(), right_start.clone(), count, edge);
                                                                    match source_ml_source1_s27.clone() {
                                                                        crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop3Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop3Done => {
                                                                            {
                                                                                let at_2 = (at + 1f64);
                                                                                {
                                                                                    at = at_2;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                        crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop3Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop3Return(source_ml_temporary_result3) => {
                                                                            crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Return(source_ml_temporary_result3)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop2Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop2Return(source_ml_temporary_result4) => {
                                            crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Return(source_ml_temporary_result4)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::global_concept_rules::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Result::MlTemporaryVerifyGlobalConceptCorrespondenceLoop1Done
        };
    }
}

const fn ml_main() {}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(ml_main)
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}
