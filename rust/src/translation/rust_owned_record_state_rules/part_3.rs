// Generated from complete js/src/translation/rust-owned-record-state-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_rust_owned_record_field_identity;
use super::read_rust_owned_record_field_root;
use super::read_rust_owned_record_place_depth_limit;
use super::test_rust_owned_record_field_descendant;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_state_write_error(
    mutable: bool,
    callable: bool,
    scope: bool,
    root: &self::ml_string::Text,
    field: &self::ml_string::Text,
    type_: bool,
    loan: bool,
) -> self::ml_string::Text {
    if mutable {
        if callable {
            if scope {
                if ((*root)
                    != (self::ml_string::Text::from_units(vec![
                        97, 118, 97, 105, 108, 97, 98, 108, 101,
                    ])))
                    || ((((*field)
                        != (self::ml_string::Text::from_units(vec![
                            97, 118, 97, 105, 108, 97, 98, 108, 101,
                        ])))
                        && ((*field)
                            != (self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100]))))
                        && ((*field)
                            != (self::ml_string::Text::from_units(vec![
                                112, 111, 115, 115, 105, 98, 108, 121, 32, 109, 111, 118, 101, 100,
                            ]))))
                {
                    self::ml_string::Text::from_units(vec![
                        102, 105, 101, 108, 100, 32, 114, 101, 105, 110, 105, 116, 105, 97, 108,
                        105, 122, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105,
                        116, 115, 32, 115, 116, 105, 108, 108, 32, 105, 110, 105, 116, 105, 97,
                        108, 105, 122, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 114, 111,
                        111, 116, 32, 97, 110, 100, 32, 101, 120, 97, 99, 116, 32, 97, 118, 97,
                        105, 108, 97, 98, 108, 101, 32, 111, 114, 32, 109, 111, 118, 101, 100, 32,
                        102, 105, 101, 108, 100,
                    ])
                } else if type_ {
                    if loan {
                        self::ml_string::Text::from_units(vec![
                            102, 105, 101, 108, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101,
                            110, 116, 32, 111, 118, 101, 114, 108, 97, 112, 115, 32, 97, 110, 32,
                            97, 99, 116, 117, 97, 108, 32, 115, 117, 114, 118, 105, 118, 105, 110,
                            103, 32, 119, 104, 111, 108, 101, 32, 111, 114, 32, 102, 105, 101, 108,
                            100, 32, 108, 111, 97, 110,
                        ])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                } else {
                    self::ml_string::Text::from_units(vec![
                        102, 105, 101, 108, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110,
                        116, 32, 109, 117, 115, 116, 32, 112, 114, 101, 115, 101, 114, 118, 101,
                        32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112,
                        108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112,
                        101, 32, 97, 110, 100, 32, 111, 119, 110, 101, 114,
                    ])
                }
            } else {
                self::ml_string::Text::from_units(vec![
                    116, 104, 105, 115, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32,
                    110, 101, 101, 100, 115, 32, 97, 110, 32, 101, 120, 112, 108, 105, 99, 105,
                    116, 32, 114, 101, 99, 111, 114, 100, 32, 115, 116, 97, 116, 101, 32, 114, 101,
                    115, 117, 108, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111,
                    110, 32, 98, 101, 102, 111, 114, 101, 32, 97, 110, 32, 111, 117, 116, 101, 114,
                    32, 112, 108, 97, 99, 101, 32, 99, 97, 110, 32, 98, 101, 32, 99, 104, 97, 110,
                    103, 101, 100,
                ])
            }
        } else {
            self::ml_string::Text::from_units(vec![
                97, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 109, 117, 116, 97, 98, 108, 101,
                32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115,
                32, 115, 101, 112, 97, 114, 97, 116, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101,
                32, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 32, 115, 116, 97, 116,
                101, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110,
            ])
        }
    } else {
        self::ml_string::Text::from_units(vec![
            114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 97, 115, 115, 105, 103,
            110, 109, 101, 110, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99,
            116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 111, 111, 116, 32, 98,
            105, 110, 100, 105, 110, 103,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_owned_record_state_loan_overlap(
    root: &self::ml_string::Text,
    target: &self::ml_string::Text,
    loans: &[self::ml_string::Text],
    lexical: bool,
) -> bool {
    {
        let precise = false;
        {
            let source_ml_temporary_values6 = loans.to_vec();
            {
                let source_ml_temporary_index6 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s23 =
                        self::source_ml_temporary_test_rust_owned_record_state_loan_overlap_loop7(
                            root,
                            target,
                            precise,
                            &(source_ml_temporary_values6),
                            source_ml_temporary_index6,
                        );
                    match source_ml_source1_s23 {
                        self::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Result::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Done(precise_2) => {
                            if precise_2 {
                                false
                            } else {
                                lexical || self::ml_array::includes(loans, root, 0f64, false, |a, b| a == b)
                            }
                        }
                        self::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Result::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Return(source_ml_temporary_result7) => {
                            source_ml_temporary_result7
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_owned_record_state_packet(roots: f64) -> Vec<f64> {
    if (!(self::ml_math::is_integer(roots)) || (roots < 0f64)) || (roots > 256f64) {
        Vec::<f64>::new()
    } else {
        {
            let result = vec![0f64];
            {
                let index = 0f64;
                {
                    self::source_ml_temporary_plan_rust_owned_record_state_packet_loop8(
                        roots, result, index,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_owned_record_state_fields(fields: f64, selected: f64) -> Vec<f64> {
    if ((((!(self::ml_math::is_integer(fields)) || (fields < 1f64)) || (fields > 1024f64))
        || !(self::ml_math::is_integer(selected)))
        || (selected < 0f64))
        || (selected >= fields)
    {
        Vec::<f64>::new()
    } else {
        {
            let result = Vec::<f64>::new();
            {
                let index = 0f64;
                {
                    self::source_ml_temporary_plan_rust_owned_record_state_fields_loop9(
                        fields, selected, result, index,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_state_field_identity(
    root: &self::ml_string::Text,
    fields: &[f64],
) -> self::ml_string::Text {
    {
        let result = root.clone();
        {
            let source_ml_temporary_values10 = fields.to_vec();
            {
                let source_ml_temporary_index10 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s24 = self::source_ml_temporary_read_rust_owned_record_state_field_identity_loop11(result, &(source_ml_temporary_values10), source_ml_temporary_index10);
                    match source_ml_source1_s24 {
                        self::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Result::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Done(result_2) => {
                            result_2
                        }
                        self::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Result::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Return(source_ml_temporary_result11) => {
                            source_ml_temporary_result11
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result {
    MlTemporaryReadRustOwnedRecordUltimateRootLoop1Done,
    MlTemporaryReadRustOwnedRecordUltimateRootLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_record_ultimate_root_loop1(
    mut root: self::ml_string::Text,
    mut depth: f64,
) -> self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result {
    loop {
        return if depth < 1024f64 {
            {
                let parent = self::read_rust_owned_record_field_root(&(root));
                if parent == (self::ml_string::Text::from_units(vec![])) {
                    self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Return(root.clone())
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
            self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result {
    MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Done,
    MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_rust_owned_record_field_descendant_loop2(
    root: &self::ml_string::Text,
    mut parent: self::ml_string::Text,
    mut depth: f64,
) -> self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result {
    loop {
        return if (depth < 1024f64) && (parent != (self::ml_string::Text::from_units(vec![]))) {
            if parent == (*root) {
                self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Return(true)
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
            self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result {
    MlTemporaryReadRustOwnedRecordWholeErrorLoop4Done,
    MlTemporaryReadRustOwnedRecordWholeErrorLoop4Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_record_whole_error_loop4(
    root: &self::ml_string::Text,
    states: &[self::ml_string::Text],
    mut index: f64,
    source_ml_temporary_values3: &[self::ml_string::Text],
    mut source_ml_temporary_index3: self::ml::Big,
) -> self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result {
    loop {
        return if source_ml_temporary_index3
            < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))
        {
            {
                let identity = self::ml_array::at(
                    source_ml_temporary_values3,
                    source_ml_temporary_index3.to_index(),
                );
                if self::test_rust_owned_record_field_descendant(root, &(identity)) && {
                    let source_ml_source1_comparison_operand25 =
                        self::ml_array::optional_at(states, self::ml_array::number_index(index));
                    {
                        let source_ml_source1_comparison_operand26 =
                            self::ml_string::Text::from_units(vec![
                                97, 118, 97, 105, 108, 97, 98, 108, 101,
                            ]);
                        !(match (
                            source_ml_source1_comparison_operand25,
                            source_ml_source1_comparison_operand26,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                } {
                    self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Return(self::ml_string::Text::from_units(vec![97, 32, 112, 97, 114, 116, 105, 97, 108, 108, 121, 32, 109, 111, 118, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 117, 115, 101, 100, 32, 97, 115, 32, 97, 32, 119, 104, 111, 108, 101, 32, 118, 97, 108, 117, 101]))
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            let source_ml_temporary_index3_2 =
                                source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                            {
                                (index, source_ml_temporary_index3) =
                                    (index_2, source_ml_temporary_index3_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result {
    MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Done,
    MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_record_place_token_count_loop5(
    forms: &[self::ml_string::Text],
    mut depth: f64,
    mut atom: bool,
    mut at: f64,
) -> self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result {
    loop {
        return if at < (forms.len() as f64) {
            {
                let form = self::ml_array::optional_at(forms, self::ml_array::number_index(at));
                if atom {
                    if {
                        let source_ml_source1_comparison_operand27 = form.clone();
                        {
                            let source_ml_source1_comparison_operand28 =
                                self::ml_string::Text::from_units(vec![40]);
                            match (
                                source_ml_source1_comparison_operand27,
                                source_ml_source1_comparison_operand28,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            }
                        }
                    } {
                        {
                            let depth_2 = depth + 1f64;
                            if depth_2 >= (self::read_rust_owned_record_place_depth_limit()) {
                                self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return(-1f64)
                            } else {
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (depth, at) = (depth_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else if {
                        let source_ml_source1_comparison_operand29 = form.clone();
                        {
                            let source_ml_source1_comparison_operand30 =
                                self::ml_string::Text::from_units(vec![110, 97, 109, 101]);
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
                            let atom_2 = false;
                            {
                                let at_3 = at + 1f64;
                                {
                                    (atom, at) = (atom_2, at_3);
                                    continue;
                                }
                            }
                        }
                    } else {
                        self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return(-1f64)
                    }
                } else if {
                    let source_ml_source1_comparison_operand31 = form.clone();
                    {
                        let source_ml_source1_comparison_operand32 =
                            self::ml_string::Text::from_units(vec![46]);
                        match (
                            source_ml_source1_comparison_operand31,
                            source_ml_source1_comparison_operand32,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                } {
                    {
                        let at_4 = at + 1f64;
                        if {
                            let source_ml_source1_comparison_operand33 =
                                self::ml_array::optional_at(
                                    forms,
                                    self::ml_array::number_index(at_4),
                                );
                            {
                                let source_ml_source1_comparison_operand34 =
                                    self::ml_string::Text::from_units(vec![110, 97, 109, 101]);
                                !(match (
                                    source_ml_source1_comparison_operand33,
                                    source_ml_source1_comparison_operand34,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                })
                            }
                        } && {
                            let source_ml_source1_comparison_operand35 =
                                self::ml_array::optional_at(
                                    forms,
                                    self::ml_array::number_index(at_4),
                                );
                            {
                                let source_ml_source1_comparison_operand36 =
                                    self::ml_string::Text::from_units(vec![
                                        110, 117, 109, 98, 101, 114,
                                    ]);
                                !(match (
                                    source_ml_source1_comparison_operand35,
                                    source_ml_source1_comparison_operand36,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                })
                            }
                        } {
                            self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return(-1f64)
                        } else {
                            {
                                let at_5 = at_4 + 1f64;
                                {
                                    at = at_5;
                                    continue;
                                }
                            }
                        }
                    }
                } else if {
                    let source_ml_source1_comparison_operand37 = form.clone();
                    {
                        let source_ml_source1_comparison_operand38 =
                            self::ml_string::Text::from_units(vec![41]);
                        match (
                            source_ml_source1_comparison_operand37,
                            source_ml_source1_comparison_operand38,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                } {
                    if depth == 0f64 {
                        self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return(-1f64)
                    } else {
                        {
                            let depth_3 = depth - 1f64;
                            {
                                let at_6 = at + 1f64;
                                {
                                    (depth, at) = (depth_3, at_6);
                                    continue;
                                }
                            }
                        }
                    }
                } else if {
                    let source_ml_source1_comparison_operand39 = form.clone();
                    {
                        let source_ml_source1_comparison_operand40 =
                            self::ml_string::Text::from_units(vec![125]);
                        match (
                            source_ml_source1_comparison_operand39,
                            source_ml_source1_comparison_operand40,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                } {
                    self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return(if depth == 0f64 {
                                    at
                                } else {
                                    -1f64
                                })
                } else {
                    self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return(-1f64)
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Result {
    MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Done(bool),
    MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_rust_owned_record_state_loan_overlap_loop7(
    root: &self::ml_string::Text,
    target: &self::ml_string::Text,
    mut precise: bool,
    source_ml_temporary_values6: &[self::ml_string::Text],
    mut source_ml_temporary_index6: self::ml::Big,
) -> self::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Result {
    loop {
        return if source_ml_temporary_index6
            < (self::ml::Big::from_u128(source_ml_temporary_values6.len() as u128))
        {
            {
                let loan = self::ml_array::at(
                    source_ml_temporary_values6,
                    source_ml_temporary_index6.to_index(),
                );
                if self::test_rust_owned_record_field_descendant(root, &(loan)) {
                    {
                        let precise_2 = true;
                        if ((loan == (*target))
                            || self::test_rust_owned_record_field_descendant(target, &(loan)))
                            || self::test_rust_owned_record_field_descendant(&(loan), target)
                        {
                            self::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Result::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Return(true)
                        } else {
                            {
                                let source_ml_temporary_index6_2 =
                                    source_ml_temporary_index6.add(&self::ml::Big::from_i128(1));
                                {
                                    (precise, source_ml_temporary_index6) =
                                        (precise_2, source_ml_temporary_index6_2);
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index6_3 =
                            source_ml_temporary_index6.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index6 = source_ml_temporary_index6_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Result::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Done(precise)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_record_state_packet_loop8(
    mut roots: f64,
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < roots {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (vec![(index + 1f64)], result);
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_record_state_fields_loop9(
    mut fields: f64,
    mut selected: f64,
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < fields {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) =
                        (vec![if index == selected { -1f64 } else { index }], result);
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
pub enum MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Result {
    MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Done(self::ml_string::Text),
    MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_record_state_field_identity_loop11(
    mut result: self::ml_string::Text,
    source_ml_temporary_values10: &[f64],
    mut source_ml_temporary_index10: self::ml::Big,
) -> self::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Result {
    loop {
        return if source_ml_temporary_index10
            < (self::ml::Big::from_u128(source_ml_temporary_values10.len() as u128))
        {
            {
                let index = self::ml_array::at(
                    source_ml_temporary_values10,
                    source_ml_temporary_index10.to_index(),
                );
                {
                    let result_2 = self::read_rust_owned_record_field_identity(&(result), index);
                    if result_2 == (self::ml_string::Text::from_units(vec![])) {
                        self::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Result::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Return(self::ml_string::Text::from_units(vec![]))
                    } else {
                        {
                            let source_ml_temporary_index10_2 =
                                source_ml_temporary_index10.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_temporary_index10) =
                                    (result_2, source_ml_temporary_index10_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Result::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Done(result.clone())
        };
    }
}
