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
pub use crate::translation::frontend_rules::ml_array;

#[must_use]
pub fn solve_product_source_facts(
    bases: &[bool],
    targets: &[f64],
    sources: &[f64],
    conjunctions: &[bool],
) -> Vec<bool> {
    if (((bases.len() as f64) != (conjunctions.len() as f64))
        || ((targets.len() as f64) != (sources.len() as f64)))
    {
        Vec::<bool>::new()
    } else {
        let ml_product_condition_2 = {
            let ml_values1 = targets;
            let ml_callback1 = {
                std::rc::Rc::new(move |index: f64| -> bool {
                    ((self::ml_math::is_safe_integer(index) && (index >= (0f64)))
                        && (index < (bases.len() as f64)))
                })
            };
            ml_values1
                .iter()
                .copied()
                .enumerate()
                .all(|(ml_index1, ml_value1)| ml_callback1(ml_value1))
        };
        if ml_product_condition_2 {
            {
                let ml_product_condition_1 = {
                    let ml_values2 = sources;
                    let ml_callback2 = {
                        std::rc::Rc::new(move |index: f64| -> bool {
                            ((self::ml_math::is_safe_integer(index) && (index >= (0f64)))
                                && (index < (bases.len() as f64)))
                        })
                    };
                    ml_values2
                        .iter()
                        .copied()
                        .enumerate()
                        .all(|(ml_index2, ml_value2)| ml_callback2(ml_value2))
                };
                if ml_product_condition_1 {
                    {
                        let flags = bases.to_vec();
                        {
                            let round = 0f64;
                            {
                                self::ml_solve_product_source_facts_loop3(
                                    bases,
                                    targets,
                                    sources,
                                    conjunctions,
                                    flags,
                                    round,
                                )
                            }
                        }
                    }
                } else {
                    Vec::<bool>::new()
                }
            }
        } else {
            Vec::<bool>::new()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlClosure1Loop2Result {
    MlClosure1Loop2Done(bool, bool, bool),
}

// Checked closure lowering passes its captured state explicitly.
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn ml_closure1_loop2(
    targets: &[f64],
    previous: &[bool],
    sources: &[f64],
    mut index: f64,
    mut found: bool,
    mut any: bool,
    mut all: bool,
    mut edge: f64,
) -> self::MlClosure1Loop2Result {
    loop {
        if (edge < (targets.len() as f64)) {
            if match (
                self::ml_array::optional_at(targets, self::ml_array::number_index(edge)),
                index,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } {
                {
                    let found_2 = true;
                    if match (
                        self::ml_array::optional_at(
                            previous,
                            self::ml_array::number_index(
                                self::ml_array::optional_at(
                                    sources,
                                    self::ml_array::number_index(edge),
                                )
                                .unwrap_or(f64::NAN),
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
                    }
                    {
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
            }
            {
                {
                    let edge_4 = (edge + 1f64);
                    {
                        edge = edge_4;
                        continue;
                    }
                }
            }
        }
        return { self::MlClosure1Loop2Result::MlClosure1Loop2Done(found, any, all) };
    }
}

#[must_use]
pub fn ml_solve_product_source_facts_loop3(
    bases: &[bool],
    targets: &[f64],
    sources: &[f64],
    conjunctions: &[bool],
    mut flags: Vec<bool>,
    mut round: f64,
) -> Vec<bool> {
    loop {
        if (round < (bases.len() as f64)) {
            {
                let previous = flags;
                {
                    let flags_2 = {
                        let ml_values3 = previous.as_slice();
                        let ml_callback3 = {
                            let previous = previous.as_slice();
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
                                                            targets, previous, sources, index,
                                                            found, any, all, edge,
                                                        );
                                                        match ml_s1 {
                                                self::MlClosure1Loop2Result::MlClosure1Loop2Done(found_2, any_2, all_2) => {
                                                    if self::ml_array::optional_at(conjunctions, self::ml_array::number_index(index)).unwrap_or_default() {
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
                            .copied()
                            .enumerate()
                            .map(|(ml_index3, ml_value3)| {
                                ml_callback3(ml_value3, (ml_index3 as f64))
                            })
                            .collect::<Vec<_>>()
                    };
                    {
                        let round_2 = (round + 1f64);
                        {
                            (flags, round) = (flags_2, round_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { flags };
    }
}
