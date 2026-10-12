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
}

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
                let ml_s1 =
                    crate::global_concept_rules::ml_verify_global_concept_correspondence_loop1(
                        shapes.clone(),
                        meanings.clone(),
                        offsets.clone(),
                        edges.clone(),
                        first.clone(),
                        second.clone(),
                        at,
                    );
                match ml_s1.clone() {
                    crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop1Result::MlVerifyGlobalConceptCorrespondenceLoop1Done => {
                        true
                    }
                    crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop1Result::MlVerifyGlobalConceptCorrespondenceLoop1Return(ml_result1) => {
                        ml_result1
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlVerifyGlobalConceptCorrespondenceLoop1Result {
    MlVerifyGlobalConceptCorrespondenceLoop1Done,
    MlVerifyGlobalConceptCorrespondenceLoop1Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlVerifyGlobalConceptCorrespondenceLoop3Result {
    MlVerifyGlobalConceptCorrespondenceLoop3Done,
    MlVerifyGlobalConceptCorrespondenceLoop3Return(bool),
}

pub fn ml_verify_global_concept_correspondence_loop4(
    mut first: Vec<f64>,
    mut second: Vec<f64>,
    mut left_target: f64,
    mut right_target: f64,
    mut pair: f64,
    mut found: bool,
) -> bool {
    loop {
        return if (pair < (first.len() as f64)) {
            {
                let found_3 = if (((crate::global_concept_rules::ml_array::at(
                    &first,
                    crate::global_concept_rules::ml_array::number_index(pair),
                )) == left_target)
                    && ((crate::global_concept_rules::ml_array::at(
                        &second,
                        crate::global_concept_rules::ml_array::number_index(pair),
                    )) == right_target))
                {
                    {
                        let found_2 = true;
                        found_2
                    }
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

pub fn ml_verify_global_concept_correspondence_loop3(
    mut edges: Vec<f64>,
    mut first: Vec<f64>,
    mut second: Vec<f64>,
    mut left_start: f64,
    mut right_start: f64,
    mut count: f64,
    mut edge: f64,
) -> crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop3Result {
    loop {
        return if (edge < count) {
            {
                let left_target = crate::global_concept_rules::ml_array::at(
                    &edges,
                    crate::global_concept_rules::ml_array::number_index((left_start + edge)),
                );
                {
                    let right_target = crate::global_concept_rules::ml_array::at(
                        &edges,
                        crate::global_concept_rules::ml_array::number_index((right_start + edge)),
                    );
                    {
                        let pair = 0f64;
                        {
                            let found = false;
                            {
                                let found_2 = crate::global_concept_rules::ml_verify_global_concept_correspondence_loop4(first.clone(), second.clone(), left_target, right_target, pair, found);
                                if !found_2 {
                                    crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop3Result::MlVerifyGlobalConceptCorrespondenceLoop3Return(false)
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
            crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop3Result::MlVerifyGlobalConceptCorrespondenceLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlVerifyGlobalConceptCorrespondenceLoop2Result {
    MlVerifyGlobalConceptCorrespondenceLoop2Done,
    MlVerifyGlobalConceptCorrespondenceLoop2Return(bool),
}

pub fn ml_verify_global_concept_correspondence_loop2(
    mut first: Vec<f64>,
    mut second: Vec<f64>,
    mut at: f64,
    mut left: f64,
    mut right: f64,
    mut other: f64,
) -> crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop2Result {
    loop {
        return if (other < at) {
            if (((crate::global_concept_rules::ml_array::at(
                &first,
                crate::global_concept_rules::ml_array::number_index(other),
            )) == left)
                || ((crate::global_concept_rules::ml_array::at(
                    &second,
                    crate::global_concept_rules::ml_array::number_index(other),
                )) == right))
            {
                crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop2Result::MlVerifyGlobalConceptCorrespondenceLoop2Return(false)
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
            crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop2Result::MlVerifyGlobalConceptCorrespondenceLoop2Done
        };
    }
}

pub fn ml_verify_global_concept_correspondence_loop1(
    mut shapes: Vec<String>,
    mut meanings: Vec<String>,
    mut offsets: Vec<f64>,
    mut edges: Vec<f64>,
    mut first: Vec<f64>,
    mut second: Vec<f64>,
    mut at: f64,
) -> crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop1Result {
    loop {
        return if (at < (first.len() as f64)) {
            {
                let left = crate::global_concept_rules::ml_array::at(
                    &first,
                    crate::global_concept_rules::ml_array::number_index(at),
                );
                {
                    let right = crate::global_concept_rules::ml_array::at(
                        &second,
                        crate::global_concept_rules::ml_array::number_index(at),
                    );
                    if ((((((left < (0f64)) || (right < (0f64))) || ((left % 1f64) != (0f64)))
                        || ((right % 1f64) != (0f64)))
                        || (left >= (shapes.len() as f64)))
                        || (right >= (shapes.len() as f64)))
                    {
                        crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop1Result::MlVerifyGlobalConceptCorrespondenceLoop1Return(false)
                    } else {
                        if (((crate::global_concept_rules::ml_array::at(
                            &shapes,
                            crate::global_concept_rules::ml_array::number_index(left),
                        )) != (crate::global_concept_rules::ml_array::at(
                            &shapes,
                            crate::global_concept_rules::ml_array::number_index(right),
                        ))) || ((crate::global_concept_rules::ml_array::at(
                            &meanings,
                            crate::global_concept_rules::ml_array::number_index(left),
                        )) != (crate::global_concept_rules::ml_array::at(
                            &meanings,
                            crate::global_concept_rules::ml_array::number_index(right),
                        )))) {
                            crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop1Result::MlVerifyGlobalConceptCorrespondenceLoop1Return(false)
                        } else {
                            {
                                let other = 0f64;
                                {
                                    let ml_s2 = crate::global_concept_rules::ml_verify_global_concept_correspondence_loop2(first.clone(), second.clone(), at, left, right, other);
                                    match ml_s2.clone() {
                                        crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop2Result::MlVerifyGlobalConceptCorrespondenceLoop2Done => {
                                            {
                                                let left_start = crate::global_concept_rules::ml_array::at(&offsets, crate::global_concept_rules::ml_array::number_index(left));
                                                {
                                                    let right_start = crate::global_concept_rules::ml_array::at(&offsets, crate::global_concept_rules::ml_array::number_index(right));
                                                    {
                                                        let count = (crate::global_concept_rules::ml_array::at(&offsets, crate::global_concept_rules::ml_array::number_index((left + 1f64))) - left_start);
                                                        if (((((((((left_start < (0f64)) || (right_start < (0f64))) || (((left_start % 1f64)) != (0f64))) || (((right_start % 1f64)) != (0f64))) || (count < (0f64))) || (((count % 1f64)) != (0f64))) || (count != ((crate::global_concept_rules::ml_array::at(&offsets, crate::global_concept_rules::ml_array::number_index((right + 1f64))) - right_start)))) || (((left_start + count)) > ((edges.len() as f64)))) || (((right_start + count)) > ((edges.len() as f64)))) {
                                                            crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop1Result::MlVerifyGlobalConceptCorrespondenceLoop1Return(false)
                                                        } else {
                                                            {
                                                                let edge = 0f64;
                                                                {
                                                                    let ml_s3 = crate::global_concept_rules::ml_verify_global_concept_correspondence_loop3(edges.clone(), first.clone(), second.clone(), left_start, right_start, count, edge);
                                                                    match ml_s3.clone() {
                                                                        crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop3Result::MlVerifyGlobalConceptCorrespondenceLoop3Done => {
                                                                            {
                                                                                let at_2 = (at + 1f64);
                                                                                {
                                                                                    at = at_2;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                        crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop3Result::MlVerifyGlobalConceptCorrespondenceLoop3Return(ml_result3) => {
                                                                            crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop1Result::MlVerifyGlobalConceptCorrespondenceLoop1Return(ml_result3)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop2Result::MlVerifyGlobalConceptCorrespondenceLoop2Return(ml_result4) => {
                                            crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop1Result::MlVerifyGlobalConceptCorrespondenceLoop1Return(ml_result4)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::global_concept_rules::MlVerifyGlobalConceptCorrespondenceLoop1Result::MlVerifyGlobalConceptCorrespondenceLoop1Done
        };
    }
}

fn ml_main() {}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || ml_main())
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}
