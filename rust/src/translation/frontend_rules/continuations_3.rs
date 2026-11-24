// Generated from js/src/translation/frontend-rules.js by
// js/scripts/generate-frontend-rules.mjs. Do not edit by hand.
use super::*;

pub fn source_ml_source1_temporary_read_rust_number_bounds_loop26(
    mut units: Vec<f64>,
    mut at: f64,
) -> f64 {
    loop {
        return if ((at < (units.len() as f64))
            && (({
                let source_ml_source2_comparison_operand268 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand269 = 48f64;
                    if match (source_ml_source2_comparison_operand268.clone(), None::<f64>) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    } {
                        ((f64::NAN) >= source_ml_source2_comparison_operand269)
                    } else {
                        ((source_ml_source2_comparison_operand268
                            .clone()
                            .expect("a proven present value"))
                            >= source_ml_source2_comparison_operand269)
                    }
                }
            } && {
                let source_ml_source2_comparison_operand270 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand271 = 57f64;
                    if match (source_ml_source2_comparison_operand270.clone(), None::<f64>) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    } {
                        ((f64::NAN) <= source_ml_source2_comparison_operand271)
                    } else {
                        ((source_ml_source2_comparison_operand270
                            .clone()
                            .expect("a proven present value"))
                            <= source_ml_source2_comparison_operand271)
                    }
                }
            }) || {
                let source_ml_source2_comparison_operand272 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand273 = 95f64;
                    match (
                        source_ml_source2_comparison_operand272.clone(),
                        source_ml_source2_comparison_operand273,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            })) {
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
pub enum MlSource1TemporaryReadRustNumberBoundsJoin23 {
    MlSource1TemporaryReadRustNumberBoundsJoin23Next(f64, bool),
}

pub fn source_ml_source1_temporary_read_rust_number_bounds_loop27(
    mut units: Vec<f64>,
    mut at: f64,
) -> f64 {
    loop {
        return if ((at < (units.len() as f64))
            && (({
                let source_ml_source2_comparison_operand274 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand275 = 48f64;
                    if match (source_ml_source2_comparison_operand274.clone(), None::<f64>) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    } {
                        ((f64::NAN) >= source_ml_source2_comparison_operand275)
                    } else {
                        ((source_ml_source2_comparison_operand274
                            .clone()
                            .expect("a proven present value"))
                            >= source_ml_source2_comparison_operand275)
                    }
                }
            } && {
                let source_ml_source2_comparison_operand276 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand277 = 57f64;
                    if match (source_ml_source2_comparison_operand276.clone(), None::<f64>) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    } {
                        ((f64::NAN) <= source_ml_source2_comparison_operand277)
                    } else {
                        ((source_ml_source2_comparison_operand276
                            .clone()
                            .expect("a proven present value"))
                            <= source_ml_source2_comparison_operand277)
                    }
                }
            }) || {
                let source_ml_source2_comparison_operand278 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand279 = 95f64;
                    match (
                        source_ml_source2_comparison_operand278.clone(),
                        source_ml_source2_comparison_operand279,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            })) {
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
pub enum MlSource1TemporaryAcceptPrecedenceOperandContextLoop28Result {
    MlSource1TemporaryAcceptPrecedenceOperandContextLoop28Done,
    MlSource1TemporaryAcceptPrecedenceOperandContextLoop28Return(bool),
}

pub fn source_ml_source1_temporary_accept_precedence_operand_context_loop28(
    mut owner_contexts: Vec<bool>,
    mut child_contexts: Vec<bool>,
    mut index: f64,
) -> crate::translation::frontend_rules::MlSource1TemporaryAcceptPrecedenceOperandContextLoop28Result
{
    loop {
        return if ((index < (owner_contexts.len() as f64))
            && (index < (child_contexts.len() as f64)))
        {
            if match {
                let source_ml_source2_logical280 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &owner_contexts,
                        crate::translation::frontend_rules::ml_array::number_index(index),
                    );
                if match source_ml_source2_logical280.clone() {
                    Some(ml_present) => ml_present,
                    None => false,
                } {
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &child_contexts,
                        crate::translation::frontend_rules::ml_array::number_index(index),
                    )
                } else {
                    source_ml_source2_logical280.clone()
                }
            } {
                Some(ml_present) => ml_present,
                None => false,
            } {
                crate::translation::frontend_rules::MlSource1TemporaryAcceptPrecedenceOperandContextLoop28Result::MlSource1TemporaryAcceptPrecedenceOperandContextLoop28Return(true)
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
            crate::translation::frontend_rules::MlSource1TemporaryAcceptPrecedenceOperandContextLoop28Result::MlSource1TemporaryAcceptPrecedenceOperandContextLoop28Done
        };
    }
}

pub fn source_ml_source1_temporary_read_lexical_winner_index_loop29(
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
                    if ((best < (0f64)) || {
                        let source_ml_source2_comparison_operand281 =
                            crate::translation::frontend_rules::ml_array::optional_at(
                                &ends,
                                crate::translation::frontend_rules::ml_array::number_index(index),
                            );
                        {
                            let source_ml_source2_comparison_operand282 =
                                crate::translation::frontend_rules::ml_array::optional_at(
                                    &ends,
                                    crate::translation::frontend_rules::ml_array::number_index(
                                        best,
                                    ),
                                );
                            if match (source_ml_source2_comparison_operand281.clone(), None::<f64>)
                            {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                if match (
                                    source_ml_source2_comparison_operand282.clone(),
                                    None::<f64>,
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    ((f64::NAN) > (f64::NAN))
                                } else {
                                    ((f64::NAN)
                                        > (source_ml_source2_comparison_operand282
                                            .clone()
                                            .expect("a proven present value")))
                                }
                            } else {
                                if match (
                                    source_ml_source2_comparison_operand282.clone(),
                                    None::<f64>,
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    ((source_ml_source2_comparison_operand281
                                        .clone()
                                        .expect("a proven present value"))
                                        > (f64::NAN))
                                } else {
                                    ((source_ml_source2_comparison_operand281
                                        .clone()
                                        .expect("a proven present value"))
                                        > (source_ml_source2_comparison_operand282
                                            .clone()
                                            .expect("a proven present value")))
                                }
                            }
                        }
                    }) {
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop30Result {
    MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop30Done,
    MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop30Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop31Result {
    MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop31Done,
    MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop31Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Result {
    MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Done(bool),
    MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_temporal_binding_read_loop32(mut bounds: Vec<f64>, mut names: Vec<String>, mut ends: Vec<f64>, mut read_names: Vec<String>, mut read_starts: Vec<f64>, mut read: f64, mut scope: Option<f64>, mut bound: bool, mut captured: bool, mut module_read: bool, mut at: Option<f64>) -> crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Result{
    loop {
        return if {
            let source_ml_source2_comparison_operand283 = at.clone();
            {
                let source_ml_source2_comparison_operand284 =
                    (match crate::translation::frontend_rules::ml_array::optional_at(
                        &bounds,
                        crate::translation::frontend_rules::ml_array::number_index(
                            (match scope.clone() {
                                Some(ml_present) => ml_present,
                                None => f64::NAN,
                            } * 3f64),
                        ),
                    ) {
                        Some(ml_present) => ml_present,
                        None => f64::NAN,
                    } + match crate::translation::frontend_rules::ml_array::optional_at(
                        &bounds,
                        crate::translation::frontend_rules::ml_array::number_index(
                            ((match scope.clone() {
                                Some(ml_present) => ml_present,
                                None => f64::NAN,
                            } * 3f64)
                                + 1f64),
                        ),
                    ) {
                        Some(ml_present) => ml_present,
                        None => f64::NAN,
                    });
                if match (source_ml_source2_comparison_operand283.clone(), None::<f64>) {
                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                    (None, None) => true,
                    _ => false,
                } {
                    ((f64::NAN) < source_ml_source2_comparison_operand284)
                } else {
                    ((source_ml_source2_comparison_operand283
                        .clone()
                        .expect("a proven present value"))
                        < source_ml_source2_comparison_operand284)
                }
            }
        } {
            if {
                let source_ml_source2_comparison_operand285 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &names,
                        crate::translation::frontend_rules::ml_array::number_index(
                            match at.clone() {
                                Some(ml_present) => ml_present,
                                None => f64::NAN,
                            },
                        ),
                    );
                {
                    let source_ml_source2_comparison_operand286 =
                        crate::translation::frontend_rules::ml_array::optional_at(
                            &read_names,
                            crate::translation::frontend_rules::ml_array::number_index(read),
                        );
                    match (
                        source_ml_source2_comparison_operand285.clone(),
                        source_ml_source2_comparison_operand286.clone(),
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            } {
                if ({
                    let source_ml_source2_comparison_operand287 =
                        crate::translation::frontend_rules::ml_array::optional_at(
                            &ends,
                            crate::translation::frontend_rules::ml_array::number_index(
                                match at.clone() {
                                    Some(ml_present) => ml_present,
                                    None => f64::NAN,
                                },
                            ),
                        );
                    {
                        let source_ml_source2_comparison_operand288 = 0f64;
                        if match (source_ml_source2_comparison_operand287.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) >= source_ml_source2_comparison_operand288)
                        } else {
                            ((source_ml_source2_comparison_operand287
                                .clone()
                                .expect("a proven present value"))
                                >= source_ml_source2_comparison_operand288)
                        }
                    }
                } && module_read)
                {
                    crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Result::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Return(vec![read, 2f64])
                } else {
                    if ({
                        let source_ml_source2_comparison_operand289 =
                            crate::translation::frontend_rules::ml_array::optional_at(
                                &ends,
                                crate::translation::frontend_rules::ml_array::number_index(
                                    match at.clone() {
                                        Some(ml_present) => ml_present,
                                        None => f64::NAN,
                                    },
                                ),
                            );
                        {
                            let source_ml_source2_comparison_operand290 = 0f64;
                            if match (source_ml_source2_comparison_operand289.clone(), None::<f64>)
                            {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                ((f64::NAN) >= source_ml_source2_comparison_operand290)
                            } else {
                                ((source_ml_source2_comparison_operand289
                                    .clone()
                                    .expect("a proven present value"))
                                    >= source_ml_source2_comparison_operand290)
                            }
                        }
                    } && {
                        let source_ml_source2_comparison_operand291 =
                            crate::translation::frontend_rules::ml_array::optional_at(
                                &read_starts,
                                crate::translation::frontend_rules::ml_array::number_index(read),
                            );
                        {
                            let source_ml_source2_comparison_operand292 =
                                crate::translation::frontend_rules::ml_array::optional_at(
                                    &ends,
                                    crate::translation::frontend_rules::ml_array::number_index(
                                        match at.clone() {
                                            Some(ml_present) => ml_present,
                                            None => f64::NAN,
                                        },
                                    ),
                                );
                            if match (source_ml_source2_comparison_operand291.clone(), None::<f64>)
                            {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                if match (
                                    source_ml_source2_comparison_operand292.clone(),
                                    None::<f64>,
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    ((f64::NAN) < (f64::NAN))
                                } else {
                                    ((f64::NAN)
                                        < (source_ml_source2_comparison_operand292
                                            .clone()
                                            .expect("a proven present value")))
                                }
                            } else {
                                if match (
                                    source_ml_source2_comparison_operand292.clone(),
                                    None::<f64>,
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    ((source_ml_source2_comparison_operand291
                                        .clone()
                                        .expect("a proven present value"))
                                        < (f64::NAN))
                                } else {
                                    ((source_ml_source2_comparison_operand291
                                        .clone()
                                        .expect("a proven present value"))
                                        < (source_ml_source2_comparison_operand292
                                            .clone()
                                            .expect("a proven present value")))
                                }
                            }
                        }
                    }) {
                        crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Result::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Return(vec![read, if captured {
                            1f64
                        } else {
                            0f64
                        }])
                    } else {
                        {
                            let bound_2 = true;
                            {
                                let at_2 = (match at.clone() {
                                    Some(ml_present) => ml_present,
                                    None => f64::NAN,
                                } + 1f64);
                                {
                                    (bound, at) = (bound_2, Some(at_2));
                                    continue;
                                }
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = (match at.clone() {
                        Some(ml_present) => ml_present,
                        None => f64::NAN,
                    } + 1f64);
                    {
                        at = Some(at_3);
                        continue;
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Result::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Done(bound)
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_temporal_binding_read_loop31(mut parents: Vec<f64>, mut bounds: Vec<f64>, mut names: Vec<String>, mut ends: Vec<f64>, mut read_names: Vec<String>, mut read_starts: Vec<f64>, mut read: f64, mut scope: Option<f64>, mut bound: bool, mut captured: bool, mut module_read: bool) -> crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop31Result{
    loop {
        return if ({
            let source_ml_source2_comparison_operand293 = scope.clone();
            {
                let source_ml_source2_comparison_operand294 = 0f64;
                if match (source_ml_source2_comparison_operand293.clone(), None::<f64>) {
                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                    (None, None) => true,
                    _ => false,
                } {
                    ((f64::NAN) >= source_ml_source2_comparison_operand294)
                } else {
                    ((source_ml_source2_comparison_operand293
                        .clone()
                        .expect("a proven present value"))
                        >= source_ml_source2_comparison_operand294)
                }
            }
        } && !bound)
        {
            {
                let at = crate::translation::frontend_rules::ml_array::optional_at(
                    &bounds,
                    crate::translation::frontend_rules::ml_array::number_index(
                        (match scope.clone() {
                            Some(ml_present) => ml_present,
                            None => f64::NAN,
                        } * 3f64),
                    ),
                );
                {
                    let source_ml_source2_s295 = crate::translation::frontend_rules::source_ml_source1_temporary_read_java_script_temporal_binding_read_loop32(bounds.clone(), names.clone(), ends.clone(), read_names.clone(), read_starts.clone(), read, scope.clone(), bound, captured, module_read, at.clone());
                    match source_ml_source2_s295.clone() {
                        crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Result::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Done(bound_2) => {
                            {
                                let captured_3 = if (!bound_2 && {
                                    let source_ml_source2_comparison_operand296 = crate::translation::frontend_rules::ml_array::optional_at(&bounds, crate::translation::frontend_rules::ml_array::number_index(((match scope.clone() { Some(ml_present) => ml_present, None => f64::NAN } * 3f64) + 2f64)));
                                    {
                                        let source_ml_source2_comparison_operand297 = 1f64;
                                        match (source_ml_source2_comparison_operand296.clone(), source_ml_source2_comparison_operand297) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                    }
                                }) {
                                    {
                                        let captured_2 = true;
                                        captured_2
                                    }
                                } else {
                                    captured
                                };
                                {
                                    let module_read_3 = if (!bound_2 && {
                                        let source_ml_source2_comparison_operand298 = crate::translation::frontend_rules::ml_array::optional_at(&bounds, crate::translation::frontend_rules::ml_array::number_index(((match scope.clone() { Some(ml_present) => ml_present, None => f64::NAN } * 3f64) + 2f64)));
                                        {
                                            let source_ml_source2_comparison_operand299 = 2f64;
                                            match (source_ml_source2_comparison_operand298.clone(), source_ml_source2_comparison_operand299) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        }
                                    }) {
                                        {
                                            let module_read_2 = true;
                                            module_read_2
                                        }
                                    } else {
                                        module_read
                                    };
                                    {
                                        let scope_2 = crate::translation::frontend_rules::ml_array::optional_at(&parents, crate::translation::frontend_rules::ml_array::number_index(match scope.clone() { Some(ml_present) => ml_present, None => f64::NAN }));
                                        {
                                            (scope, bound, captured, module_read) = (scope_2.clone(), bound_2, captured_3, module_read_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Result::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop32Return(source_ml_source1_temporary_result32) => {
                            crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop31Result::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop31Return(source_ml_source1_temporary_result32.clone())
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop31Result::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop31Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_temporal_binding_read_loop30(mut parents: Vec<f64>, mut bounds: Vec<f64>, mut names: Vec<String>, mut ends: Vec<f64>, mut read_scopes: Vec<f64>, mut read_names: Vec<String>, mut read_starts: Vec<f64>, mut read: f64) -> crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop30Result{
    loop {
        return if (read < (read_names.len() as f64)) {
            {
                let scope = crate::translation::frontend_rules::ml_array::optional_at(
                    &read_scopes,
                    crate::translation::frontend_rules::ml_array::number_index(read),
                );
                {
                    let bound = false;
                    {
                        let captured = false;
                        {
                            let module_read = false;
                            {
                                let source_ml_source2_s300 = crate::translation::frontend_rules::source_ml_source1_temporary_read_java_script_temporal_binding_read_loop31(parents.clone(), bounds.clone(), names.clone(), ends.clone(), read_names.clone(), read_starts.clone(), read, scope.clone(), bound, captured, module_read);
                                match source_ml_source2_s300.clone() {
                                    crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop31Result::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop31Done => {
                                        {
                                            let read_2 = (read + 1f64);
                                            {
                                                read = read_2;
                                                continue;
                                            }
                                        }
                                    }
                                    crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop31Result::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop31Return(source_ml_source1_temporary_result31) => {
                                        crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop30Result::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop30Return(source_ml_source1_temporary_result31.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop30Result::MlSource1TemporaryReadJavaScriptTemporalBindingReadLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptLexicalDeclarationStepJoin33 {
    MlSource1TemporaryReadJavaScriptLexicalDeclarationStepJoin33Next,
    MlSource1TemporaryReadJavaScriptLexicalDeclarationStepJoin33Return(f64),
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
