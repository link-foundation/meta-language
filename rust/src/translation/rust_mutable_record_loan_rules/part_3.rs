// Generated from complete js/src/translation/rust-mutable-record-loan-rules.js. Do not edit by hand.
use super::accept_rust_owned_record_loop_count;
use super::accept_rust_owned_record_loop_exit;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::ml_string_number;
use super::read_rust_owned_record_field_identity;
use super::test_rust_owned_record_loop_continuing_exit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_field_root(
    identity: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (identity.starts_with(&self::ml_string::Text::from_units(vec![
        114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32,
    ]))) {
        {
            let boundary = {
                let source_ml_source2_string_search_operand8 = identity.clone();
                {
                    let source_ml_source2_string_search_operand9 =
                        self::ml_string::Text::from_units(vec![32]);
                    {
                        let source_ml_source2_string_search_text10 =
                            source_ml_source2_string_search_operand9;
                        {
                            let source_ml_source2_string_search_position11 = f64::NAN;
                            source_ml_source2_string_search_operand8.last_index_of(
                                &source_ml_source2_string_search_text10,
                                source_ml_source2_string_search_position11,
                            )
                        }
                    }
                }
            };
            {
                let root = {
                    let source_ml_source2_string_range_operand12 = identity.clone();
                    {
                        let source_ml_source2_string_range_operand13 = 13f64;
                        {
                            let source_ml_source2_string_range_operand14 = boundary;
                            {
                                let source_ml_source2_string_range_position15 =
                                    source_ml_source2_string_range_operand13;
                                {
                                    let source_ml_source2_string_range_position16 =
                                        source_ml_source2_string_range_operand14;
                                    source_ml_source2_string_range_operand12.slice(
                                        source_ml_source2_string_range_position15,
                                        source_ml_source2_string_range_position16,
                                    )
                                }
                            }
                        }
                    }
                };
                {
                    let index = self::ml_string_number::convert_java_script_string_number_units(
                        &({
                            let source_ml_source2_string_range_operand17 = identity.clone();
                            {
                                let source_ml_source2_string_range_operand18 = boundary + 1f64;
                                {
                                    let source_ml_source2_string_range_position19 =
                                        source_ml_source2_string_range_operand18;
                                    {
                                        let source_ml_source2_string_range_position20 =
                                            f64::INFINITY;
                                        source_ml_source2_string_range_operand17.slice(
                                            source_ml_source2_string_range_position19,
                                            source_ml_source2_string_range_position20,
                                        )
                                    }
                                }
                            }
                        })
                        .0
                        .iter()
                        .copied()
                        .map(f64::from)
                        .collect::<Vec<f64>>(),
                    );
                    if (self::read_rust_owned_record_field_identity(&(root), index)) == (*identity)
                    {
                        root.clone()
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_absent_field_state(
    root: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*root) == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))
    {
        self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101])
    } else {
        self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_ultimate_root(
    identity: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let root = self::read_rust_owned_record_field_root(identity);
        if root == (self::ml_string::Text::from_units(vec![])) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let depth = 0f64;
                {
                    let source_ml_source2_s21 = self::source_ml_source1_temporary_read_rust_owned_record_ultimate_root_loop3(root, depth);
                    match source_ml_source2_s21 {
                        self::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Result::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Done => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Result::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Return(source_ml_source1_temporary_result3) => {
                            source_ml_source1_temporary_result3
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_owned_record_field_descendant(
    root: &self::ml_string::Text,
    identity: &self::ml_string::Text,
) -> bool {
    {
        let parent = self::read_rust_owned_record_field_root(identity);
        {
            let depth = 0f64;
            {
                let source_ml_source2_s22 =
                    self::source_ml_source1_temporary_test_rust_owned_record_field_descendant_loop4(
                        root, parent, depth,
                    );
                match source_ml_source2_s22 {
                    self::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Result::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Done => {
                        false
                    }
                    self::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Result::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Return(source_ml_source1_temporary_result4) => {
                        source_ml_source1_temporary_result4
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_whole_error(
    root: &self::ml_string::Text,
    identities: &[self::ml_string::Text],
    states: &[self::ml_string::Text],
) -> self::ml_string::Text {
    {
        let ml_library_inequality_1 = (identities.len() as f64) != (states.len() as f64);
        if ml_library_inequality_1 {
            self::ml_string::Text::from_units(vec![
                114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 97, 118, 97, 105,
                108, 97, 98, 105, 108, 105, 116, 121, 32, 110, 101, 101, 100, 115, 32, 101, 118,
                101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32,
                115, 116, 97, 116, 101,
            ])
        } else if (*root) == (self::ml_string::Text::from_units(vec![])) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_temporary_values5 = identities.to_vec();
                    {
                        let source_ml_source1_temporary_index5 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s23 = self::source_ml_source1_temporary_read_rust_owned_record_whole_error_loop6(root, states, index, &(source_ml_source1_temporary_values5), source_ml_source1_temporary_index5);
                            match source_ml_source2_s23 {
                                self::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Result::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Done => {
                                    self::ml_string::Text::from_units(vec![])
                                }
                                self::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Result::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Return(source_ml_source1_temporary_result6) => {
                                    source_ml_source1_temporary_result6
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
pub fn read_rust_owned_record_field_error(
    root: &self::ml_string::Text,
    field: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*root)
        == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101])))
        && ((*field)
            == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101])))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 113,
            117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 118,
            97, 105, 108, 97, 98, 108, 101, 32, 114, 111, 111, 116, 32, 97, 110, 100, 32, 117, 110,
            109, 111, 118, 101, 100, 32, 102, 105, 101, 108, 100,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_update_error(
    profile: &self::ml_string::Text,
    owner: bool,
    layers: f64,
    fields: f64,
    positions: f64,
) -> self::ml_string::Text {
    if ((((((((*profile)
        == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100])))
        || ((*profile) == (self::ml_string::Text::from_units(vec![99, 111, 112, 121]))))
        && owner)
        && (layers == 0f64))
        && self::ml_math::is_integer(fields))
        && (fields >= 0f64))
        && (fields <= 1024f64))
        && ((positions).partial_cmp(&(fields)) == Some(std::cmp::Ordering::Equal))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112,
            100, 97, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116,
            117, 97, 108, 32, 100, 101, 115, 116, 114, 117, 99, 116, 111, 114, 45, 102, 114, 101,
            101, 32, 115, 116, 114, 117, 99, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108,
            101, 116, 101, 32, 102, 105, 101, 108, 100, 32, 108, 97, 121, 111, 117, 116,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_mutable_record_value_loans(
    reference: bool,
    place: bool,
    copy: bool,
    contained: f64,
) -> bool {
    (reference || (place && !copy)) || (contained > 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_rust_mutable_record_loan_root(
    root: &self::ml_string::Text,
    lexical: &self::ml_string::Text,
    loans: &[self::ml_string::Text],
) -> bool {
    if (*root) == (self::ml_string::Text::from_units(vec![])) {
        false
    } else {
        {
            let source_ml_source1_temporary_values7 = loans.to_vec();
            {
                let source_ml_source1_temporary_index7 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s24 = self::source_ml_source1_temporary_select_rust_mutable_record_loan_root_loop8(root, lexical, &(source_ml_source1_temporary_values7), source_ml_source1_temporary_index7);
                    match source_ml_source2_s24 {
                        self::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Result::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Done => {
                            false
                        }
                        self::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Result::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Return(source_ml_source1_temporary_result8) => {
                            source_ml_source1_temporary_result8
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_mutable_record_loan_identity(ordinal: f64) -> self::ml_string::Text {
    if self::accept_rust_owned_record_loop_count(ordinal) {
        (self::ml_string::Text::from_units(vec![
            109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 108, 111, 97,
            110, 32,
        ]))
        .concat(&{
            let source_ml_source2_string_argument25 = ordinal;
            self::ml_string::Text::from_text(&self::ml_number::js_number(
                source_ml_source2_string_argument25,
            ))
        })
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_mutable_record_loan_budget_error(
    nodes: f64,
    text: f64,
    added_nodes: f64,
    added_text: f64,
) -> self::ml_string::Text {
    if ((((!(self::accept_rust_owned_record_loop_count(nodes))
        || !(self::accept_rust_owned_record_loop_count(text)))
        || !(self::accept_rust_owned_record_loop_count(added_nodes)))
        || !(self::accept_rust_owned_record_loop_count(added_text)))
        || (nodes > (100_000f64 - added_nodes)))
        || (text > (16_777_216f64 - added_text))
    {
        self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32,
            114, 101, 99, 111, 114, 100, 32, 118, 97, 108, 117, 101, 32, 108, 111, 97, 110, 115,
            32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 111, 117, 114,
            99, 101, 32, 98, 117, 100, 103, 101, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_mutable_record_loan_escape(
    carried: bool,
    root_survives: bool,
    live: bool,
) -> bool {
    (carried && root_survives) && live
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_mutable_record_loan_move_error(
    owned: bool,
    overlapping: bool,
) -> self::ml_string::Text {
    if owned && overlapping {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 111, 119, 110, 101, 100, 32, 109, 111, 118, 101, 32, 111, 118, 101, 114,
            108, 97, 112, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 114, 118,
            105, 118, 105, 110, 103, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111,
            114, 100, 32, 118, 97, 108, 117, 101, 32, 108, 111, 97, 110,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_rust_owned_record_loop_iterations_loop1(
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < 255f64 {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (vec![index], result);
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result {
    MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Done,
    MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_owned_record_loop_progress_error_loop2(
    exits: &[self::ml_string::Text],
    counts: &[f64],
    budget_error: &self::ml_string::Text,
    mut index: f64,
) -> self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result {
    loop {
        return if index < (exits.len() as f64) {
            {
                let exit = self::ml_array::optional_at(exits, self::ml_array::number_index(index));
                {
                    let count =
                        self::ml_array::optional_at(counts, self::ml_array::number_index(index));
                    if {
                        let source_ml_source2_comparison_operand26 = exit.clone();
                        {
                            let source_ml_source2_comparison_operand27 = None::<()>;
                            match (
                                source_ml_source2_comparison_operand26,
                                source_ml_source2_comparison_operand27,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }
                        }
                    } {
                        self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(budget_error.clone())
                    } else if {
                        let source_ml_source2_comparison_operand28 = count;
                        {
                            let source_ml_source2_comparison_operand29 = None::<()>;
                            match (
                                source_ml_source2_comparison_operand28,
                                source_ml_source2_comparison_operand29,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }
                        }
                    } {
                        self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(budget_error.clone())
                    } else if (!(self::accept_rust_owned_record_loop_exit(
                        &(exit.clone().expect("a proven present value")),
                    )) || !(self::accept_rust_owned_record_loop_count(
                        count.expect("a proven present value"),
                    ))) || ((count.expect("a proven present value")) > 256f64)
                    {
                        self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(budget_error.clone())
                    } else if self::test_rust_owned_record_loop_continuing_exit(
                        &(exit.clone().expect("a proven present value")),
                    ) && ((count.expect("a proven present value")) != 1f64)
                    {
                        self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 111, 110, 116, 105, 110, 117, 105, 110, 103, 32, 114, 101, 99, 111, 114, 100, 32, 108, 111, 111, 112, 32, 112, 97, 116, 104, 32, 109, 117, 115, 116, 32, 100, 101, 99, 114, 101, 109, 101, 110, 116, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 117, 110, 116, 101, 114, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 99, 101]))
                    } else {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Result {
    MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Done,
    MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_owned_record_ultimate_root_loop3(
    mut root: self::ml_string::Text,
    mut depth: f64,
) -> self::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Result {
    loop {
        return if depth < 1024f64 {
            {
                let parent = self::read_rust_owned_record_field_root(&(root));
                if parent == (self::ml_string::Text::from_units(vec![])) {
                    self::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Result::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Return(root.clone())
                } else {
                    {
                        let root_2 = parent;
                        {
                            let depth_2 = depth + 1f64;
                            {
                                (root, depth) = (root_2, depth_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Result::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Result {
    MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Done,
    MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_rust_owned_record_field_descendant_loop4(
    root: &self::ml_string::Text,
    mut parent: self::ml_string::Text,
    mut depth: f64,
) -> self::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Result {
    loop {
        return if (depth < 1024f64) && (parent != (self::ml_string::Text::from_units(vec![]))) {
            if parent == (*root) {
                self::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Result::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Return(true)
            } else {
                {
                    let parent_2 = self::read_rust_owned_record_field_root(&(parent));
                    {
                        let depth_2 = depth + 1f64;
                        {
                            (parent, depth) = (parent_2, depth_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Result::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Result {
    MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Done,
    MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_owned_record_whole_error_loop6(
    root: &self::ml_string::Text,
    states: &[self::ml_string::Text],
    mut index: f64,
    source_ml_source1_temporary_values5: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index5: self::ml::Big,
) -> self::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Result {
    loop {
        return if source_ml_source1_temporary_index5
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values5.len() as u128))
        {
            {
                let identity = self::ml_array::at(
                    source_ml_source1_temporary_values5,
                    source_ml_source1_temporary_index5.to_index(),
                );
                if self::test_rust_owned_record_field_descendant(root, &(identity)) && {
                    let source_ml_source2_comparison_operand30 =
                        self::ml_array::optional_at(states, self::ml_array::number_index(index));
                    {
                        let source_ml_source2_comparison_operand31 =
                            self::ml_string::Text::from_units(vec![
                                97, 118, 97, 105, 108, 97, 98, 108, 101,
                            ]);
                        !(match (
                            source_ml_source2_comparison_operand30,
                            source_ml_source2_comparison_operand31,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                } {
                    self::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Result::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Return(self::ml_string::Text::from_units(vec![97, 32, 112, 97, 114, 116, 105, 97, 108, 108, 121, 32, 109, 111, 118, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 117, 115, 101, 100, 32, 97, 115, 32, 97, 32, 119, 104, 111, 108, 101, 32, 118, 97, 108, 117, 101]))
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            let source_ml_source1_temporary_index5_2 =
                                source_ml_source1_temporary_index5
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (index, source_ml_source1_temporary_index5) =
                                    (index_2, source_ml_source1_temporary_index5_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Result::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Result {
    MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Done,
    MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_select_rust_mutable_record_loan_root_loop8(
    root: &self::ml_string::Text,
    lexical: &self::ml_string::Text,
    source_ml_source1_temporary_values7: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index7: self::ml::Big,
) -> self::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Result {
    loop {
        return if source_ml_source1_temporary_index7
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values7.len() as u128))
        {
            {
                let loan = self::ml_array::at(
                    source_ml_source1_temporary_values7,
                    source_ml_source1_temporary_index7.to_index(),
                );
                if ((loan == (*root))
                    || (((*lexical) != (self::ml_string::Text::from_units(vec![])))
                        && (loan == (*lexical))))
                    || self::test_rust_owned_record_field_descendant(root, &(loan))
                {
                    self::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Result::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Return(true)
                } else {
                    {
                        let source_ml_source1_temporary_index7_2 =
                            source_ml_source1_temporary_index7.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index7 =
                                source_ml_source1_temporary_index7_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Result::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Done
        };
    }
}
