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
pub use crate::global_concept_rules::ml_array;

/// Execute the checked shared global concept identity decision.
#[must_use]
pub fn contains_global_concept_identity(identities: &[String], candidate: String) -> bool {
    {
        let index = 0f64;
        {
            let ml_s1 =
                crate::global_concept_identity_rules::ml_contains_global_concept_identity_loop1(
                    identities, candidate, index,
                );
            match ml_s1 {
                crate::global_concept_identity_rules::MlContainsGlobalConceptIdentityLoop1Result::MlContainsGlobalConceptIdentityLoop1Done => {
                    false
                }
                crate::global_concept_identity_rules::MlContainsGlobalConceptIdentityLoop1Result::MlContainsGlobalConceptIdentityLoop1Return(ml_result1) => {
                    ml_result1
                }
            }
        }
    }
}

/// Execute the checked shared global concept identity decision.
#[must_use]
pub fn accept_global_concept_history(
    identities: &[String],
    aliases: &[String],
    unique_identities: f64,
    unique_aliases: f64,
) -> bool {
    ((((identities.len() as f64) == unique_identities)
        && ((aliases.len() as f64) == unique_aliases))
        && !(crate::global_concept_identity_rules::contains_global_concept_identity(
            identities,
            String::new(),
        )))
        && !(crate::global_concept_identity_rules::contains_global_concept_identity(
            aliases,
            String::new(),
        ))
}

/// Execute the checked shared global concept identity decision.
#[must_use]
pub fn select_global_concept_identity(
    preferred: String,
    fallback: String,
    reserved: &[String],
    assigned: &[String],
) -> String {
    if preferred.is_empty() {
        {
            let candidate = fallback;
            {
                crate::global_concept_identity_rules::ml_select_global_concept_identity_loop2(
                    reserved, assigned, candidate,
                )
            }
        }
    } else if crate::global_concept_identity_rules::contains_global_concept_identity(
        assigned,
        preferred.clone(),
    ) {
        String::new()
    } else {
        preferred
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlContainsGlobalConceptIdentityLoop1Result {
    MlContainsGlobalConceptIdentityLoop1Done,
    MlContainsGlobalConceptIdentityLoop1Return(bool),
}

/// Execute the checked shared global concept identity decision.
#[must_use]
pub fn ml_contains_global_concept_identity_loop1(
    identities: &[String],
    mut candidate: String,
    mut index: f64,
) -> crate::global_concept_identity_rules::MlContainsGlobalConceptIdentityLoop1Result {
    loop {
        return if index < (identities.len() as f64) {
            if match (
                crate::global_concept_identity_rules::ml_array::optional_at(
                    identities,
                    crate::global_concept_identity_rules::ml_array::number_index(index),
                ),
                candidate.clone(),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } {
                crate::global_concept_identity_rules::MlContainsGlobalConceptIdentityLoop1Result::MlContainsGlobalConceptIdentityLoop1Return(true)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            crate::global_concept_identity_rules::MlContainsGlobalConceptIdentityLoop1Result::MlContainsGlobalConceptIdentityLoop1Done
        };
    }
}

/// Execute the checked shared global concept identity decision.
#[must_use]
pub fn ml_select_global_concept_identity_loop2(
    reserved: &[String],
    assigned: &[String],
    mut candidate: String,
) -> String {
    loop {
        if crate::global_concept_identity_rules::contains_global_concept_identity(
            reserved,
            candidate.clone(),
        ) || crate::global_concept_identity_rules::contains_global_concept_identity(
            assigned,
            candidate.clone(),
        ) {
            {
                let candidate_2 = format!("{}{}", candidate, (String::from(".new")));
                {
                    candidate = candidate_2;
                    continue;
                }
            }
        }
        return candidate;
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
