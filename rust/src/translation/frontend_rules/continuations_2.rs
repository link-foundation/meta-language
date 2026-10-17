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
                let ml_logical23 = crate::translation::frontend_rules::ml_array::optional_at(
                    &owner_contexts,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if ml_logical23.unwrap_or_default() {
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &child_contexts,
                        crate::translation::frontend_rules::ml_array::number_index(index),
                    )
                } else {
                    ml_logical23
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
    ends: &[f64],
    eligible: &[bool],
    mut best: f64,
    mut index: f64,
) -> f64 {
    loop {
        if ((index < (ends.len() as f64)) && (index < (eligible.len() as f64))) {
            {
                let best_3 = if (crate::translation::frontend_rules::ml_array::optional_at(
                    eligible,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                ))
                .unwrap_or_default()
                {
                    if ((best < (0f64))
                        || (((crate::translation::frontend_rules::ml_array::optional_at(
                            ends,
                            crate::translation::frontend_rules::ml_array::number_index(index),
                        ))
                        .unwrap_or(f64::NAN))
                            > ((crate::translation::frontend_rules::ml_array::optional_at(
                                ends,
                                crate::translation::frontend_rules::ml_array::number_index(best),
                            ))
                            .unwrap_or(f64::NAN))))
                    {
                        { index }
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
        }
        return { best };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadJavaScriptTemporalBindingReadLoop30Result {
    MlReadJavaScriptTemporalBindingReadLoop30Done,
    MlReadJavaScriptTemporalBindingReadLoop30Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadJavaScriptTemporalBindingReadLoop31Result {
    MlReadJavaScriptTemporalBindingReadLoop31Done,
    MlReadJavaScriptTemporalBindingReadLoop31Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadJavaScriptTemporalBindingReadLoop32Result {
    MlReadJavaScriptTemporalBindingReadLoop32Done(bool),
    MlReadJavaScriptTemporalBindingReadLoop32Return(Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_java_script_temporal_binding_read_loop32(
    bounds: &[f64],
    names: &[String],
    ends: &[f64],
    read_names: &[String],
    read_starts: &[f64],
    mut read: f64,
    mut scope: Option<f64>,
    mut bound: bool,
    mut captured: bool,
    mut module_read: bool,
    mut at: Option<f64>,
) -> crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop32Result {
    loop {
        return if ((at).unwrap_or(f64::NAN))
            < ((crate::translation::frontend_rules::ml_array::optional_at(
                bounds,
                crate::translation::frontend_rules::ml_array::number_index(
                    ((scope).unwrap_or(f64::NAN) * 3f64),
                ),
            ))
            .unwrap_or(f64::NAN)
                + (crate::translation::frontend_rules::ml_array::optional_at(
                    bounds,
                    crate::translation::frontend_rules::ml_array::number_index(
                        (((scope).unwrap_or(f64::NAN) * 3f64) + 1f64),
                    ),
                ))
                .unwrap_or(f64::NAN))
        {
            if match (
                crate::translation::frontend_rules::ml_array::optional_at(
                    names,
                    crate::translation::frontend_rules::ml_array::number_index(
                        (at).unwrap_or(f64::NAN),
                    ),
                ),
                crate::translation::frontend_rules::ml_array::optional_at(
                    read_names,
                    crate::translation::frontend_rules::ml_array::number_index(read),
                ),
            ) {
                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                (None, None) => true,
                _ => false,
            } {
                if (((crate::translation::frontend_rules::ml_array::optional_at(
                    ends,
                    crate::translation::frontend_rules::ml_array::number_index(
                        (at).unwrap_or(f64::NAN),
                    ),
                ))
                .unwrap_or(f64::NAN))
                    >= 0f64)
                    && module_read
                {
                    crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop32Result::MlReadJavaScriptTemporalBindingReadLoop32Return(vec![read, 2f64])
                } else if (((crate::translation::frontend_rules::ml_array::optional_at(
                    ends,
                    crate::translation::frontend_rules::ml_array::number_index(
                        (at).unwrap_or(f64::NAN),
                    ),
                ))
                .unwrap_or(f64::NAN))
                    >= 0f64)
                    && (((crate::translation::frontend_rules::ml_array::optional_at(
                        read_starts,
                        crate::translation::frontend_rules::ml_array::number_index(read),
                    ))
                    .unwrap_or(f64::NAN))
                        < ((crate::translation::frontend_rules::ml_array::optional_at(
                            ends,
                            crate::translation::frontend_rules::ml_array::number_index(
                                (at).unwrap_or(f64::NAN),
                            ),
                        ))
                        .unwrap_or(f64::NAN)))
                {
                    crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop32Result::MlReadJavaScriptTemporalBindingReadLoop32Return(vec![read, if captured {
                            1f64
                        } else {
                            0f64
                        }])
                } else {
                    {
                        let bound_2 = true;
                        {
                            let at_2 = (at).unwrap_or(f64::NAN) + 1f64;
                            {
                                (bound, at) = (bound_2, Some(at_2));
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at).unwrap_or(f64::NAN) + 1f64;
                    {
                        at = Some(at_3);
                        continue;
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop32Result::MlReadJavaScriptTemporalBindingReadLoop32Done(bound)
        };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_java_script_temporal_binding_read_loop31(
    parents: &[f64],
    bounds: &[f64],
    names: &[String],
    ends: &[f64],
    read_names: &[String],
    read_starts: &[f64],
    mut read: f64,
    mut scope: Option<f64>,
    mut bound: bool,
    mut captured: bool,
    mut module_read: bool,
) -> crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop31Result {
    loop {
        return if (((scope).unwrap_or(f64::NAN)) >= 0f64) && !bound {
            {
                let at = crate::translation::frontend_rules::ml_array::optional_at(
                    bounds,
                    crate::translation::frontend_rules::ml_array::number_index(
                        ((scope).unwrap_or(f64::NAN) * 3f64),
                    ),
                );
                {
                    let ml_s24 = crate::translation::frontend_rules::ml_read_java_script_temporal_binding_read_loop32(bounds, names, ends, read_names, read_starts, read, scope, bound, captured, module_read, at);
                    match ml_s24 {
                        crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop32Result::MlReadJavaScriptTemporalBindingReadLoop32Done(bound_2) => {
                            {
                                let captured_3 = if !bound_2 && match (crate::translation::frontend_rules::ml_array::optional_at(bounds, crate::translation::frontend_rules::ml_array::number_index(((scope).unwrap_or(f64::NAN) * 3f64) + 2f64)), 1f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } {
                                    {
                                        true
                                    }
                                } else {
                                    captured
                                };
                                {
                                    let module_read_3 = if !bound_2 && match (crate::translation::frontend_rules::ml_array::optional_at(bounds, crate::translation::frontend_rules::ml_array::number_index(((scope).unwrap_or(f64::NAN) * 3f64) + 2f64)), 2f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } {
                                        {
                                            true
                                        }
                                    } else {
                                        module_read
                                    };
                                    {
                                        let scope_2 = crate::translation::frontend_rules::ml_array::optional_at(parents, crate::translation::frontend_rules::ml_array::number_index((scope).unwrap_or(f64::NAN)));
                                        {
                                            (scope, bound, captured, module_read) = (scope_2, bound_2, captured_3, module_read_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop32Result::MlReadJavaScriptTemporalBindingReadLoop32Return(ml_result32) => {
                            crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop31Result::MlReadJavaScriptTemporalBindingReadLoop31Return(ml_result32)
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop31Result::MlReadJavaScriptTemporalBindingReadLoop31Done
        };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_java_script_temporal_binding_read_loop30(
    parents: &[f64],
    bounds: &[f64],
    names: &[String],
    ends: &[f64],
    read_scopes: &[f64],
    read_names: &[String],
    read_starts: &[f64],
    mut read: f64,
) -> crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop30Result {
    loop {
        return if read < (read_names.len() as f64) {
            {
                let scope = crate::translation::frontend_rules::ml_array::optional_at(
                    read_scopes,
                    crate::translation::frontend_rules::ml_array::number_index(read),
                );
                {
                    let bound = false;
                    {
                        let captured = false;
                        {
                            let module_read = false;
                            {
                                let ml_s25 = crate::translation::frontend_rules::ml_read_java_script_temporal_binding_read_loop31(parents, bounds, names, ends, read_names, read_starts, read, scope, bound, captured, module_read);
                                match ml_s25 {
                                    crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop31Result::MlReadJavaScriptTemporalBindingReadLoop31Done => {
                                        {
                                            let read_2 = read + 1f64;
                                            {
                                                read = read_2;
                                                continue;
                                            }
                                        }
                                    }
                                    crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop31Result::MlReadJavaScriptTemporalBindingReadLoop31Return(ml_result31) => {
                                        crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop30Result::MlReadJavaScriptTemporalBindingReadLoop30Return(ml_result31)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlReadJavaScriptTemporalBindingReadLoop30Result::MlReadJavaScriptTemporalBindingReadLoop30Done
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
