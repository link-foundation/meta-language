// Generated from js/src/translation/frontend-rules.js by
// js/scripts/generate-frontend-rules.mjs. Do not edit by hand.
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRustNumberBoundsJoin23 {
    MlReadRustNumberBoundsJoin23Next(f64, bool),
}

pub fn ml_read_rust_number_bounds_loop27(units: &[f64], mut at: f64) -> f64 {
    loop {
        return if ((at < (units.len() as f64))
            && (((crate::translation::frontend_rules::ml_array::optional_at(
                units,
                crate::translation::frontend_rules::ml_array::number_index(at),
            )
            .unwrap_or(f64::NAN)
                >= (48f64))
                && (crate::translation::frontend_rules::ml_array::optional_at(
                    units,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                )
                .unwrap_or(f64::NAN)
                    <= (57f64)))
                || match (
                    crate::translation::frontend_rules::ml_array::optional_at(
                        units,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptPrecedenceOperandContextLoop28Result {
    MlAcceptPrecedenceOperandContextLoop28Done,
    MlAcceptPrecedenceOperandContextLoop28Return(bool),
}

pub fn ml_accept_precedence_operand_context_loop28(
    mut owner_contexts: Vec<bool>,
    mut child_contexts: Vec<bool>,
    mut index: f64,
) -> crate::translation::frontend_rules::MlAcceptPrecedenceOperandContextLoop28Result {
    loop {
        return if ((index < (owner_contexts.len() as f64))
            && (index < (child_contexts.len() as f64)))
        {
            if ({
                let ml_logical22 = crate::translation::frontend_rules::ml_array::optional_at(
                    &owner_contexts,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if ml_logical22.unwrap_or_default() {
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &child_contexts,
                        crate::translation::frontend_rules::ml_array::number_index(index),
                    )
                } else {
                    ml_logical22
                }
            })
            .unwrap_or_default()
            {
                crate::translation::frontend_rules::MlAcceptPrecedenceOperandContextLoop28Result::MlAcceptPrecedenceOperandContextLoop28Return(true)
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
            crate::translation::frontend_rules::MlAcceptPrecedenceOperandContextLoop28Result::MlAcceptPrecedenceOperandContextLoop28Done
        };
    }
}

pub fn ml_read_lexical_winner_index_loop29(
    mut ends: Vec<f64>,
    mut eligible: Vec<bool>,
    mut best: f64,
    mut index: f64,
) -> f64 {
    loop {
        return if ((index < (ends.len() as f64)) && (index < (eligible.len() as f64))) {
            {
                let best_3 = if match crate::translation::frontend_rules::ml_array::optional_at(
                    &eligible,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                ) {
                    Some(ml_present) => ml_present,
                    None => false,
                } {
                    if ((best < (0f64))
                        || ((match crate::translation::frontend_rules::ml_array::optional_at(
                            &ends,
                            crate::translation::frontend_rules::ml_array::number_index(index),
                        ) {
                            Some(ml_present) => ml_present,
                            None => f64::NAN,
                        }) > (match crate::translation::frontend_rules::ml_array::optional_at(
                            &ends,
                            crate::translation::frontend_rules::ml_array::number_index(best),
                        ) {
                            Some(ml_present) => ml_present,
                            None => f64::NAN,
                        })))
                    {
                        {
                            let best_2 = index;
                            best_2
                        }
                    } else {
                        best
                    }
                } else {
                    best
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        (best, index) = (best_3, index_2);
                        continue;
                    }
                }
            }
        } else {
            best
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
