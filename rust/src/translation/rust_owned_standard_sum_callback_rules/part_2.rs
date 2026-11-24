// Generated from complete js/src/translation/rust-owned-standard-sum-callback-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12;
use super::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14;
use super::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15;
use super::ml;
use super::ml_array;
use super::source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop16;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_standard_sum_callback_move_capture_error(
    moved: bool,
    names: &[String],
    roles: &[String],
    identities: &[String],
    layers: &[f64],
    borrowed: &[bool],
    assigned: f64,
) -> String {
    {
        let ml_library_inequality_1 = assigned != 0f64;
        if ml_library_inequality_1 {
            String::from(
                "mutable callback captures need their actual environment state interpretation",
            )
        } else if (((((names.len() as f64) > 1024f64)
            || ((roles.len() as f64) != (names.len() as f64)))
            || ((identities.len() as f64) != (names.len() as f64)))
            || ((layers.len() as f64) != (names.len() as f64)))
            || ((borrowed.len() as f64) != (names.len() as f64))
        {
            String::from(
                "the complete copied capture environment needs every actual source binding",
            )
        } else if ((names.len() as f64) > 0f64) && !moved {
            String::from("captured owned-domain callbacks need an explicit move source closure")
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source1_s4 = self::source_ml_temporary_read_rust_owned_standard_sum_callback_move_capture_error_loop5(names, roles, identities, layers, borrowed, at);
                    match source_ml_source1_s4 {
                            self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Done => {
                                String::new()
                            }
                            self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Return(source_ml_temporary_result5) => {
                                source_ml_temporary_result5
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_owned_standard_sum_callback_move_closure_copy(
    parameters: &[String],
    result: &str,
    moved: bool,
    names: &[String],
    roles: &[String],
    identities: &[String],
    layers: &[f64],
    borrowed: &[bool],
    assigned: f64,
) -> bool {
    if ((parameters.len() as f64) > 1024f64) || ((result != "copy") && (result != "owned")) {
        false
    } else {
        {
            let source_ml_temporary_values7 = parameters.to_vec();
            {
                let source_ml_temporary_index7 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s5 = self::source_ml_temporary_accept_rust_owned_standard_sum_callback_move_closure_copy_loop8(&(source_ml_temporary_values7), source_ml_temporary_index7);
                    match source_ml_source1_s5 {
                        self::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Result::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Done => {
                            (self::read_rust_owned_standard_sum_callback_move_capture_error(moved, names, roles, identities, layers, borrowed, assigned)).is_empty()
                        }
                        self::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Result::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Return(source_ml_temporary_result8) => {
                            source_ml_temporary_result8
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_standard_sum_callback_immediate_capture_error(
    captures: f64,
    kind: &str,
    identity: bool,
    moved: bool,
    names: f64,
) -> String {
    if (captures == 0f64)
        || (((((captures > 0f64) && (kind == "lambda")) && identity) && moved)
            && ((names).partial_cmp(&(captures)) == Some(std::cmp::Ordering::Equal)))
    {
        String::new()
    } else {
        String::from(
            "a copied callback environment needs its actual explicit move Lambda source and span identity",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_standard_sum_callback_closure_copy(
    parameters: &[String],
    result: &str,
    captures: f64,
    assigned: f64,
) -> bool {
    if (((captures != 0f64) || (assigned != 0f64)) || ((parameters.len() as f64) > 1024f64))
        || ((result != "copy") && (result != "owned"))
    {
        false
    } else {
        {
            let source_ml_temporary_values9 = parameters.to_vec();
            {
                let source_ml_temporary_index9 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s6 = self::source_ml_temporary_accept_rust_owned_standard_sum_callback_closure_copy_loop10(&(source_ml_temporary_values9), source_ml_temporary_index9);
                    match source_ml_source1_s6 {
                        self::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Result::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Done => {
                            true
                        }
                        self::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Result::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Return(source_ml_temporary_result10) => {
                            source_ml_temporary_result10
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_standard_sum_callback_source_error(
    kind: &str,
    parameters: f64,
    expected: f64,
    wrappers: f64,
    callable_copy: bool,
    result: &str,
) -> String {
    if ((((kind == "function")
        && ((parameters).partial_cmp(&(expected)) == Some(std::cmp::Ordering::Equal)))
        && (wrappers == 0f64))
        && callable_copy)
        && ((result == "copy") || (result == "owned"))
    {
        String::new()
    } else {
        String::from(
            "the callback needs its actual Copy callable and complete movable parameter/result contracts",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_standard_sum_callback_selection_token_limit() -> f64 {
    4096f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_owned_standard_sum_callback_selection(
    values: &[String],
    kinds: &[String],
) -> Vec<f64> {
    if ((((values.len() as f64) != (kinds.len() as f64)) || ((values.len() as f64) == 0f64))
        || ((values.len() as f64) > 4096f64))
        || {
            let source_ml_source1_comparison_operand7 =
                self::ml_array::optional_at(values, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand8 = String::from("(");
                !(match (
                    source_ml_source1_comparison_operand7,
                    source_ml_source1_comparison_operand8,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })
            }
        }
    {
        Vec::<f64>::new()
    } else {
        {
            let at = 1f64;
            {
                let layers = 0f64;
                {
                    let at_2 = self::source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop11(values, at, layers);
                    {
                        let moved = {
                            let source_ml_source1_comparison_operand9 = self::ml_array::optional_at(
                                values,
                                self::ml_array::number_index(at_2),
                            );
                            {
                                let source_ml_source1_comparison_operand10 = String::from("move");
                                match (
                                    source_ml_source1_comparison_operand9,
                                    source_ml_source1_comparison_operand10,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        };
                        {
                            let at_4 = if moved { { at_2 + 1f64 } } else { at_2 };
                            {
                                let code = 1f64;
                                {
                                    let source_ml_source1_s26 = if {
                                        let source_ml_source1_comparison_operand27 =
                                            self::ml_array::optional_at(
                                                values,
                                                self::ml_array::number_index(at_4),
                                            );
                                        {
                                            let source_ml_source1_comparison_operand28 =
                                                String::from("||");
                                            match (
                                                source_ml_source1_comparison_operand27,
                                                source_ml_source1_comparison_operand28,
                                            ) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            }
                                        }
                                    } || {
                                        let source_ml_source1_comparison_operand29 =
                                            self::ml_array::optional_at(
                                                values,
                                                self::ml_array::number_index(at_4),
                                            );
                                        {
                                            let source_ml_source1_comparison_operand30 =
                                                String::from("|");
                                            match (
                                                source_ml_source1_comparison_operand29,
                                                source_ml_source1_comparison_operand30,
                                            ) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            }
                                        }
                                    } {
                                        {
                                            let source_ml_source1_s35 = if {
                                                let source_ml_source1_comparison_operand36 =
                                                    self::ml_array::optional_at(
                                                        values,
                                                        self::ml_array::number_index(at_4),
                                                    );
                                                {
                                                    let source_ml_source1_comparison_operand37 =
                                                        String::from("||");
                                                    match (
                                                        source_ml_source1_comparison_operand36,
                                                        source_ml_source1_comparison_operand37,
                                                    ) {
                                                        (Some(ml_left), ml_right) => {
                                                            ml_left == ml_right
                                                        }
                                                        _ => false,
                                                    }
                                                }
                                            } {
                                                {
                                                    let at_5 = at_4 + 1f64;
                                                    self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Next(at_5)
                                                }
                                            } else {
                                                {
                                                    let at_6 = at_4 + 1f64;
                                                    {
                                                        let budget = 0f64;
                                                        {
                                                            let at_7 = self::source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop16(values, at_6, budget);
                                                            if {
                                                                let source_ml_source1_comparison_operand38 = self::ml_array::optional_at(values, self::ml_array::number_index(at_7));
                                                                {
                                                                    let source_ml_source1_comparison_operand39 =
                                                                        String::from("|");
                                                                    !(match (source_ml_source1_comparison_operand38, source_ml_source1_comparison_operand39) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                }
                                                            } {
                                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Return(Vec::<f64>::new())
                                                            } else {
                                                                {
                                                                    let at_8 = at_7 + 1f64;
                                                                    self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Next(at_8)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            match source_ml_source1_s35 {
                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Next(at_9) => {
                                                    if {
                                                        let source_ml_source1_comparison_operand40 = self::ml_array::optional_at(values, self::ml_array::number_index(at_9));
                                                        {
                                                            let source_ml_source1_comparison_operand41 = String::from("->");
                                                            !(match (source_ml_source1_comparison_operand40, source_ml_source1_comparison_operand41) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                        }
                                                    } {
                                                        self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Return(Vec::<f64>::new())
                                                    } else {
                                                        {
                                                            let at_10 = at_9 + 1f64;
                                                            {
                                                                let code_2 = 3f64;
                                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Next(at_10, code_2)
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Return(source_ml_temporary_result15) => {
                                                    self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Return(source_ml_temporary_result15)
                                                }
                                            }
                                        }
                                    } else if moved {
                                        self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Return(Vec::<f64>::new())
                                    } else {
                                        self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Next(at_4, code)
                                    };
                                    match source_ml_source1_s26 {
                                        self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Next(at_11, code_3) => {
                                            {
                                                let start = at_11;
                                                if {
                                                    let source_ml_source1_comparison_operand42 = self::ml_array::optional_at(kinds, self::ml_array::number_index(at_11));
                                                    {
                                                        let source_ml_source1_comparison_operand43 = String::from("identifier");
                                                        !(match (source_ml_source1_comparison_operand42, source_ml_source1_comparison_operand43) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                    }
                                                } && !(self::ml_array::includes(&vec![String::from("crate"), String::from("self"), String::from("super")], &(self::ml_array::optional_at(values, self::ml_array::number_index(at_11))).unwrap_or_else(|| String::new()), 0f64, false, |a, b| a == b)) {
                                                    Vec::<f64>::new()
                                                } else {
                                                    {
                                                        let at_12 = at_11 + 1f64;
                                                        {
                                                            let parts = 1f64;
                                                            {
                                                                let at_13 = self::source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop13(values, kinds, at_12, parts);
                                                                if code_3 == 3f64 {
                                                                    vec![3f64, start, at_13]
                                                                } else {
                                                                    {
                                                                        let source_ml_source1_s50 = if {
                                                                            let source_ml_source1_comparison_operand51 = self::ml_array::optional_at(values, self::ml_array::number_index(at_13));
                                                                            {
                                                                                let source_ml_source1_comparison_operand52 = String::from("(");
                                                                                match (source_ml_source1_comparison_operand51, source_ml_source1_comparison_operand52) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                            }
                                                                        } {
                                                                            {
                                                                                let code_4 = 2f64;
                                                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Next(code_4)
                                                                            }
                                                                        } else {
                                                                            if {
                                                                                let source_ml_source1_comparison_operand53 = self::ml_array::optional_at(values, self::ml_array::number_index(at_13));
                                                                                {
                                                                                    let source_ml_source1_comparison_operand54 = String::from(")");
                                                                                    !(match (source_ml_source1_comparison_operand53, source_ml_source1_comparison_operand54) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                }
                                                                            } && {
                                                                                let source_ml_source1_comparison_operand55 = self::ml_array::optional_at(values, self::ml_array::number_index(at_13));
                                                                                {
                                                                                    let source_ml_source1_comparison_operand56 = String::from(",");
                                                                                    !(match (source_ml_source1_comparison_operand55, source_ml_source1_comparison_operand56) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                }
                                                                            } {
                                                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Return(Vec::<f64>::new())
                                                                            } else {
                                                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Next(code_3)
                                                                            }
                                                                        };
                                                                        match source_ml_source1_s50 {
                                                                            self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Next(code_5) => {
                                                                                vec![code_5, start, at_13]
                                                                            }
                                                                            self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Return(source_ml_temporary_result14) => {
                                                                                source_ml_temporary_result14
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Return(source_ml_temporary_result14_2) => {
                                            source_ml_temporary_result14_2
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_standard_sum_callback_written_selection(
    owner: &str,
    path: &str,
    alias: bool,
    shadow: bool,
) -> bool {
    ((!alias && !shadow) && (owner == "library"))
        && ((path == "std::string::String") || (path == "std::vec::Vec"))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2 {
    MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Next,
    MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Return(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1 {
    MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Next,
    MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Return(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Result {
    MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Done,
    MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Result {
    MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Done,
    MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_standard_sum_callback_closure_header_error_loop4(
    names: &[String],
    mut at: f64,
    mut before: f64,
) -> self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Result {
    loop {
        return if before < at {
            if {
                let source_ml_source1_comparison_operand57 =
                    self::ml_array::optional_at(names, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand58 =
                        self::ml_array::optional_at(names, self::ml_array::number_index(before));
                    match (
                        source_ml_source1_comparison_operand57,
                        source_ml_source1_comparison_operand58,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            } {
                self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Return(String::from("callback parameters need distinct source bindings"))
            } else {
                {
                    let before_2 = before + 1f64;
                    {
                        before = before_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_standard_sum_callback_closure_header_error_loop3(
    names: &[String],
    roles: &[String],
    mut at: f64,
) -> self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Result {
    loop {
        return if at < (roles.len() as f64) {
            if {
                let source_ml_source1_comparison_operand59 =
                    self::ml_array::optional_at(roles, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand60 = String::from("copy");
                    !(match (
                        source_ml_source1_comparison_operand59,
                        source_ml_source1_comparison_operand60,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } && {
                let source_ml_source1_comparison_operand61 =
                    self::ml_array::optional_at(roles, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand62 = String::from("owned");
                    !(match (
                        source_ml_source1_comparison_operand61,
                        source_ml_source1_comparison_operand62,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Return(String::from("the callback parameter needs its actual direct Copy or supported owned source domain"))
            } else {
                {
                    let before = 0f64;
                    {
                        let source_ml_source1_s63 = self::source_ml_temporary_read_rust_owned_standard_sum_callback_closure_header_error_loop4(names, at, before);
                        match source_ml_source1_s63 {
                            self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Done => {
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Return(source_ml_temporary_result4) => {
                                self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Return(source_ml_temporary_result4.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result {
    MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Done,
    MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Result {
    MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Done,
    MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_standard_sum_callback_move_capture_error_loop6(
    names: &[String],
    mut at: f64,
    mut before: f64,
) -> self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Result {
    loop {
        return if before < at {
            if {
                let source_ml_source1_comparison_operand64 =
                    self::ml_array::optional_at(names, self::ml_array::number_index(before));
                {
                    let source_ml_source1_comparison_operand65 =
                        self::ml_array::optional_at(names, self::ml_array::number_index(at));
                    match (
                        source_ml_source1_comparison_operand64,
                        source_ml_source1_comparison_operand65,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            } {
                self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Return(String::from("copied callback captures need distinct actual lexical bindings"))
            } else {
                {
                    let before_2 = before + 1f64;
                    {
                        before = before_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_standard_sum_callback_move_capture_error_loop5(
    names: &[String],
    roles: &[String],
    identities: &[String],
    layers: &[f64],
    borrowed: &[bool],
    mut at: f64,
) -> self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result {
    loop {
        return if at < (names.len() as f64) {
            if ((({
                let source_ml_source1_comparison_operand66 =
                    self::ml_array::optional_at(names, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand67 = String::new();
                    match (
                        source_ml_source1_comparison_operand66,
                        source_ml_source1_comparison_operand67,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } || {
                let source_ml_source1_comparison_operand68 =
                    self::ml_array::optional_at(roles, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand69 = String::from("copy");
                    !(match (
                        source_ml_source1_comparison_operand68,
                        source_ml_source1_comparison_operand69,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            }) || {
                let source_ml_source1_comparison_operand70 =
                    self::ml_array::optional_at(identities, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand71 = String::new();
                    match (
                        source_ml_source1_comparison_operand70,
                        source_ml_source1_comparison_operand71,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            }) || {
                let source_ml_source1_comparison_operand72 =
                    self::ml_array::optional_at(layers, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand73 = 0f64;
                    !(match (
                        source_ml_source1_comparison_operand72,
                        source_ml_source1_comparison_operand73,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            }) || {
                let source_ml_source1_comparison_operand74 =
                    self::ml_array::optional_at(borrowed, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand75 = false;
                    !(match (
                        source_ml_source1_comparison_operand74,
                        source_ml_source1_comparison_operand75,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Return(String::from("a moved callback capture needs its actual complete direct Copy value without borrowed-place or loan provenance"))
            } else {
                {
                    let before = 0f64;
                    {
                        let source_ml_source1_s76 = self::source_ml_temporary_read_rust_owned_standard_sum_callback_move_capture_error_loop6(names, at, before);
                        match source_ml_source1_s76 {
                            self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Done => {
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Return(source_ml_temporary_result6) => {
                                self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Return(source_ml_temporary_result6.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Result {
    MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Done,
    MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_rust_owned_standard_sum_callback_move_closure_copy_loop8(
    source_ml_temporary_values7: &[String],
    mut source_ml_temporary_index7: self::ml::Big,
) -> self::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Result {
    loop {
        return if source_ml_temporary_index7
            < (self::ml::Big::from_u128(source_ml_temporary_values7.len() as u128))
        {
            {
                let role = self::ml_array::at(
                    source_ml_temporary_values7,
                    source_ml_temporary_index7.to_index(),
                );
                if (role != "copy") && (role != "owned") {
                    self::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Result::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Return(false)
                } else {
                    {
                        let source_ml_temporary_index7_2 =
                            source_ml_temporary_index7.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index7 = source_ml_temporary_index7_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Result::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Result {
    MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Done,
    MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_rust_owned_standard_sum_callback_closure_copy_loop10(
    source_ml_temporary_values9: &[String],
    mut source_ml_temporary_index9: self::ml::Big,
) -> self::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Result {
    loop {
        return if source_ml_temporary_index9
            < (self::ml::Big::from_u128(source_ml_temporary_values9.len() as u128))
        {
            {
                let role = self::ml_array::at(
                    source_ml_temporary_values9,
                    source_ml_temporary_index9.to_index(),
                );
                if (role != "copy") && (role != "owned") {
                    self::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Result::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Return(false)
                } else {
                    {
                        let source_ml_temporary_index9_2 =
                            source_ml_temporary_index9.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index9 = source_ml_temporary_index9_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Result::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop11(
    values: &[String],
    mut at: f64,
    mut layers: f64,
) -> f64 {
    loop {
        return if {
            let source_ml_source1_comparison_operand77 =
                self::ml_array::optional_at(values, self::ml_array::number_index(at));
            {
                let source_ml_source1_comparison_operand78 = String::from("(");
                match (
                    source_ml_source1_comparison_operand77,
                    source_ml_source1_comparison_operand78,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } && (layers < 256f64)
        {
            {
                let at_2 = at + 1f64;
                {
                    let layers_2 = layers + 1f64;
                    {
                        (at, layers) = (at_2, layers_2);
                        continue;
                    }
                }
            }
        } else {
            at
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop13(
    values: &[String],
    kinds: &[String],
    mut at: f64,
    mut parts: f64,
) -> f64 {
    loop {
        if ({
            let source_ml_source1_comparison_operand79 =
                self::ml_array::optional_at(values, self::ml_array::number_index(at));
            {
                let source_ml_source1_comparison_operand80 = String::from("::");
                match (
                    source_ml_source1_comparison_operand79,
                    source_ml_source1_comparison_operand80,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } && {
            let source_ml_source1_comparison_operand81 =
                self::ml_array::optional_at(kinds, self::ml_array::number_index(at + 1f64));
            {
                let source_ml_source1_comparison_operand82 = String::from("identifier");
                match (
                    source_ml_source1_comparison_operand81,
                    source_ml_source1_comparison_operand82,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }) && (parts < 256f64)
        {
            {
                let at_2 = at + 2f64;
                {
                    let parts_2 = parts + 1f64;
                    {
                        (at, parts) = (at_2, parts_2);
                        continue;
                    }
                }
            }
        }
        return { at };
    }
}
