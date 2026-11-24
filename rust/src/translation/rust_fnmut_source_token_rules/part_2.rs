// Generated from complete js/src/translation/rust-fnmut-source-token-rules.js. Do not edit by hand.
use super::MlTemporaryDelimitedPositionsLoop2Result;
use super::callback_positions;
use super::closing_position;
use super::is_identifier;
use super::is_punctuation;
use super::ml_array;
use super::read_required_number_fact;
use super::token_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_delimited_positions_loop2(
    kinds: &[String],
    values: &[String],
    mut end: f64,
    mut ranges: Vec<f64>,
    mut begin: f64,
    mut angles: f64,
    mut at: f64,
) -> self::MlTemporaryDelimitedPositionsLoop2Result {
    loop {
        return if at < end {
            if (self::token_fact(kinds, at)) != "punct" {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                {
                    let value = self::token_fact(values, at);
                    if ((value == "(") || (value == "[")) || (value == "{") {
                        {
                            let close = self::closing_position(
                                kinds,
                                values,
                                at,
                                &(value),
                                &(if value == "(" {
                                    String::from(")")
                                } else if value == "[" {
                                    String::from("]")
                                } else {
                                    String::from("}")
                                }),
                            );
                            if (close < 0f64) || (close >= end) {
                                self::MlTemporaryDelimitedPositionsLoop2Result::MlTemporaryDelimitedPositionsLoop2Return(vec![-1f64, at])
                            } else {
                                {
                                    let at_3 = close;
                                    {
                                        let at_4 = at_3 + 1f64;
                                        {
                                            at = at_4;
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let angles_3 = if value == "<" {
                                { angles + 1f64 }
                            } else {
                                angles
                            };
                            {
                                let angles_5 = if value == ">" {
                                    { angles_3 - 1f64 }
                                } else {
                                    angles_3
                                };
                                {
                                    let angles_7 = if value == ">>" {
                                        { angles_5 - 2f64 }
                                    } else {
                                        angles_5
                                    };
                                    if (((angles_7 < 0f64) || (value == ")")) || (value == "]"))
                                        || (value == "}")
                                    {
                                        self::MlTemporaryDelimitedPositionsLoop2Result::MlTemporaryDelimitedPositionsLoop2Return(vec![-1f64, at])
                                    } else if (value == ",") && (angles_7 == 0f64) {
                                        if (begin).partial_cmp(&(at))
                                            == Some(std::cmp::Ordering::Equal)
                                        {
                                            self::MlTemporaryDelimitedPositionsLoop2Result::MlTemporaryDelimitedPositionsLoop2Return(vec![-1f64, at])
                                        } else {
                                            {
                                                let ranges_2 = {
                                                    let (ml_append_right, ml_append_left) =
                                                        (vec![begin, at], ranges);
                                                    self::ml_array::append(
                                                        ml_append_left,
                                                        ml_append_right,
                                                    )
                                                };
                                                {
                                                    let begin_2 = at + 1f64;
                                                    {
                                                        let at_5 = at + 1f64;
                                                        {
                                                            (ranges, begin, angles, at) =
                                                                (ranges_2, begin_2, angles_7, at_5);
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let at_6 = at + 1f64;
                                            {
                                                (angles, at) = (angles_7, at_6);
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
            self::MlTemporaryDelimitedPositionsLoop2Result::MlTemporaryDelimitedPositionsLoop2Done(
                ranges, begin, angles,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFnMutSourceTokenPlanLoop6Result {
    MlTemporaryReadRustFnMutSourceTokenPlanLoop6Done,
    MlTemporaryReadRustFnMutSourceTokenPlanLoop6Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_fn_mut_source_token_plan_loop6(
    kinds: &[String],
    values: &[String],
    mut header_end: f64,
    mut at: f64,
) -> self::MlTemporaryReadRustFnMutSourceTokenPlanLoop6Result {
    loop {
        return if at < header_end {
            if self::is_identifier(kinds, values, at, "where") {
                self::MlTemporaryReadRustFnMutSourceTokenPlanLoop6Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop6Return(vec![-1f64, 6f64, at])
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFnMutSourceTokenPlanLoop6Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFnMutSourceTokenPlanLoop5Result {
    MlTemporaryReadRustFnMutSourceTokenPlanLoop5Done(Vec<f64>, f64, f64),
    MlTemporaryReadRustFnMutSourceTokenPlanLoop5Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_fn_mut_source_token_plan_loop5(
    kinds: &[String],
    values: &[String],
    formals: &[f64],
    mut callback: Vec<f64>,
    mut callback_index: f64,
    mut callbacks: f64,
    mut at: f64,
) -> self::MlTemporaryReadRustFnMutSourceTokenPlanLoop5Result {
    loop {
        return if at < (formals.len() as f64) {
            {
                let binder = self::read_required_number_fact(formals, at);
                {
                    let positions = self::callback_positions(
                        kinds,
                        values,
                        binder,
                        self::read_required_number_fact(formals, (at + 1f64)),
                    );
                    if (positions.len() as f64) == 0f64 {
                        {
                            let at_2 = at + 2f64;
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                    {
                        if (self::read_required_number_fact(&(positions), 0f64)) < 0f64 {
                            self::MlTemporaryReadRustFnMutSourceTokenPlanLoop5Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop5Return(vec![-1f64, 5f64, self::read_required_number_fact(&(positions), 1f64)])
                        } else {
                            {
                                let callback_2 = positions;
                                {
                                    let callback_index_2 = at / 2f64;
                                    {
                                        let callbacks_2 = callbacks + 1f64;
                                        {
                                            let at_3 = at + 2f64;
                                            {
                                                (callback, callback_index, callbacks, at) = (
                                                    callback_2,
                                                    callback_index_2,
                                                    callbacks_2,
                                                    at_3,
                                                );
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
            self::MlTemporaryReadRustFnMutSourceTokenPlanLoop5Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop5Done(callback, callback_index, callbacks)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFnMutSourceTokenPlanJoin4 {
    MlTemporaryReadRustFnMutSourceTokenPlanJoin4Next(Vec<f64>, f64),
    MlTemporaryReadRustFnMutSourceTokenPlanJoin4Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFnMutSourceTokenPlanLoop7Result {
    MlTemporaryReadRustFnMutSourceTokenPlanLoop7Done(Vec<f64>),
    MlTemporaryReadRustFnMutSourceTokenPlanLoop7Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_fn_mut_source_token_plan_loop7(
    kinds: &[String],
    values: &[String],
    mut generics: Vec<f64>,
    mut generic_names: Vec<String>,
    ranges: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadRustFnMutSourceTokenPlanLoop7Result {
    loop {
        return if at < (ranges.len() as f64) {
            {
                let binder = self::read_required_number_fact(ranges, at);
                {
                    let name = self::token_fact(values, binder);
                    if (((self::read_required_number_fact(ranges, (at + 1f64))) != (binder + 1f64))
                        || ((self::token_fact(kinds, binder)) != "identifier"))
                        || self::ml_array::includes(
                            &generic_names,
                            &name.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )
                    {
                        self::MlTemporaryReadRustFnMutSourceTokenPlanLoop7Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop7Return(vec![-1f64, 3f64, binder])
                    } else {
                        {
                            let generics_2 = {
                                let (ml_append_right, ml_append_left) = (vec![binder], generics);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let generic_names_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![name.clone()], generic_names);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let at_2 = at + 2f64;
                                    {
                                        (generics, generic_names, at) =
                                            (generics_2, generic_names_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFnMutSourceTokenPlanLoop7Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop7Done(generics)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFnMutSourceTokenPlanLoop3Result {
    MlTemporaryReadRustFnMutSourceTokenPlanLoop3Done(f64, bool),
    MlTemporaryReadRustFnMutSourceTokenPlanLoop3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_fn_mut_source_token_plan_loop3(
    kinds: &[String],
    values: &[String],
    mut start: f64,
    mut header_end: f64,
    mut selected: bool,
) -> self::MlTemporaryReadRustFnMutSourceTokenPlanLoop3Result {
    loop {
        return if (((header_end < (kinds.len() as f64))
            && !(self::is_punctuation(kinds, values, header_end, "{")))
            && !(self::is_punctuation(kinds, values, header_end, ";")))
            && ((self::token_fact(kinds, header_end)) != "eof")
        {
            if (header_end - start) >= 1_000_000f64 {
                self::MlTemporaryReadRustFnMutSourceTokenPlanLoop3Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop3Return(vec![-1f64, 2f64, start])
            } else {
                {
                    let selected_3 = if self::is_identifier(kinds, values, header_end, "FnMut") {
                        { true }
                    } else {
                        selected
                    };
                    {
                        let header_end_2 = header_end + 1f64;
                        {
                            (header_end, selected) = (header_end_2, selected_3);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFnMutSourceTokenPlanLoop3Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop3Done(header_end, selected)
        };
    }
}
