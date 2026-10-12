// Generated from js/src/translation/frontend-rules.js by
// js/scripts/generate-frontend-rules.mjs. Do not edit by hand.
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDocumentationParameterRangeJoin18 {
    MlFindDocumentationParameterRangeJoin18Next(f64, bool),
}

pub fn ml_read_rust_number_bounds_loop22(units: &[f64], mut at: f64) -> f64 {
    loop {
        return if ((at < (units.len() as f64))
            && ((((match crate::translation::frontend_rules::ml_array::optional_at(
                &units,
                crate::translation::frontend_rules::ml_array::number_index(at),
            ) {
                Some(ml_present) => ml_present,
                None => f64::NAN,
            }) >= (48f64))
                && ((match crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                ) {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                }) <= (57f64)))
                || match (
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    ),
                    95f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }))
        {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

pub fn ml_read_rust_number_bounds_loop25(units: &[f64], mut at: f64) -> f64 {
    loop {
        return if ((at < (units.len() as f64))
            && ((((match crate::translation::frontend_rules::ml_array::optional_at(
                &units,
                crate::translation::frontend_rules::ml_array::number_index(at),
            ) {
                Some(ml_present) => ml_present,
                None => f64::NAN,
            }) >= (97f64))
                && ((match crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                ) {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                }) <= (122f64)))
                || (((match crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                ) {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                }) >= (48f64))
                    && ((match crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    ) {
                        Some(ml_present) => ml_present,
                        None => f64::NAN,
                    }) <= (57f64)))))
        {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRustNumberBoundsJoin24 {
    MlReadRustNumberBoundsJoin24Next(f64, bool),
}

pub fn ml_read_rust_number_bounds_loop26(units: &[f64], mut at: f64) -> f64 {
    loop {
        return if ((at < (units.len() as f64))
            && ((((match crate::translation::frontend_rules::ml_array::optional_at(
                &units,
                crate::translation::frontend_rules::ml_array::number_index(at),
            ) {
                Some(ml_present) => ml_present,
                None => f64::NAN,
            }) >= (48f64))
                && ((match crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                ) {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                }) <= (57f64)))
                || match (
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    ),
                    95f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }))
        {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRustNumberBoundsJoin23 {
    MlReadRustNumberBoundsJoin23Next(f64, bool),
}

pub fn ml_read_rust_number_bounds_loop27(units: &[f64], mut at: f64) -> f64 {
    loop {
        return if ((at < (units.len() as f64))
            && ((((match crate::translation::frontend_rules::ml_array::optional_at(
                &units,
                crate::translation::frontend_rules::ml_array::number_index(at),
            ) {
                Some(ml_present) => ml_present,
                None => f64::NAN,
            }) >= (48f64))
                && ((match crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                ) {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                }) <= (57f64)))
                || match (
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    ),
                    95f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }))
        {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
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
