// Generated from complete js/src/translation/rust-record-rules.js. Do not edit by hand.
use super::MlSource1TemporaryPlanRustRecordFieldsLoop3Result;
use super::accept_rust_raw_identifier_name;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::render_rust_constructor_field_name;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_rust_record_fields_loop3(
    declared: &[self::ml_string::Text],
    supplied: &[self::ml_string::Text],
    mut result: Vec<f64>,
    mut index: f64,
) -> self::MlSource1TemporaryPlanRustRecordFieldsLoop3Result {
    loop {
        return if index < (declared.len() as f64) {
            {
                let position = self::ml_array::index_of(
                    supplied,
                    &(self::ml_array::optional_at(declared, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                    0f64,
                    false,
                    |a, b| a == b,
                );
                if position < 0f64 {
                    self::MlSource1TemporaryPlanRustRecordFieldsLoop3Result::MlSource1TemporaryPlanRustRecordFieldsLoop3Return(vec![-4f64, index])
                } else {
                    {
                        let result_2 = {
                            let (ml_append_right, ml_append_left) = (vec![position], result);
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
            }
        } else {
            self::MlSource1TemporaryPlanRustRecordFieldsLoop3Result::MlSource1TemporaryPlanRustRecordFieldsLoop3Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanRustRecordFieldsLoop2Result {
    MlSource1TemporaryPlanRustRecordFieldsLoop2Done,
    MlSource1TemporaryPlanRustRecordFieldsLoop2Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_rust_record_fields_loop2(
    declared: &[self::ml_string::Text],
    supplied: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlSource1TemporaryPlanRustRecordFieldsLoop2Result {
    loop {
        return if index < (supplied.len() as f64) {
            {
                let name =
                    (self::ml_array::optional_at(supplied, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let ml_library_inequality_1 =
                        (self::ml_array::index_of(supplied, &name.clone(), 0f64, false, |a, b| {
                            a == b
                        })) != index;
                    if ml_library_inequality_1 {
                        self::MlSource1TemporaryPlanRustRecordFieldsLoop2Result::MlSource1TemporaryPlanRustRecordFieldsLoop2Return(vec![-2f64, index])
                    } else {
                        if (self::ml_array::includes(
                            declared,
                            &name.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )) {
                            {
                                let index_2 = index + 1f64;
                                {
                                    index = index_2;
                                    continue;
                                }
                            }
                        }
                        {
                            self::MlSource1TemporaryPlanRustRecordFieldsLoop2Result::MlSource1TemporaryPlanRustRecordFieldsLoop2Return(vec![-3f64, index])
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanRustRecordFieldsLoop2Result::MlSource1TemporaryPlanRustRecordFieldsLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanRustRecordFieldsLoop1Result {
    MlSource1TemporaryPlanRustRecordFieldsLoop1Done,
    MlSource1TemporaryPlanRustRecordFieldsLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_rust_record_fields_loop1(
    declared: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlSource1TemporaryPlanRustRecordFieldsLoop1Result {
    loop {
        return if index < (declared.len() as f64) {
            {
                let name =
                    (self::ml_array::optional_at(declared, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let ml_library_inequality_1 =
                        (self::ml_array::index_of(declared, &name, 0f64, false, |a, b| a == b))
                            != index;
                    if ml_library_inequality_1 {
                        self::MlSource1TemporaryPlanRustRecordFieldsLoop1Result::MlSource1TemporaryPlanRustRecordFieldsLoop1Return(vec![-1f64, index])
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
            self::MlSource1TemporaryPlanRustRecordFieldsLoop1Result::MlSource1TemporaryPlanRustRecordFieldsLoop1Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_rust_record_pattern_fields_loop4(
    declared: &[self::ml_string::Text],
    supplied: &[self::ml_string::Text],
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (declared.len() as f64) {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::ml_array::index_of(
                            supplied,
                            &(self::ml_array::optional_at(
                                declared,
                                self::ml_array::number_index(index),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                            0f64,
                            false,
                            |a, b| a == b,
                        )],
                        result,
                    );
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
pub fn source_ml_source1_temporary_plan_tuple_pattern_fields_loop5(
    mut width: f64,
    mut count: f64,
    mut rest: f64,
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < width {
            if (rest < 0f64) || (index < rest) {
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
            {
                if index < ((width - count) + rest) {
                    {
                        let result_3 = {
                            let (ml_append_right, ml_append_left) = (vec![-1f64], result);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let index_3 = index + 1f64;
                            {
                                (result, index) = (result_3, index_3);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let result_4 = {
                            let (ml_append_right, ml_append_left) =
                                (vec![((index - width) + count)], result);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let index_4 = index + 1f64;
                            {
                                (result, index) = (result_4, index_4);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Result {
    MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Done(f64),
    MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_let_else_initializer_error_loop6(
    kinds: &[self::ml_string::Text],
    spellings: &[self::ml_string::Text],
    mut depth: f64,
    mut index: f64,
) -> self::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Result {
    loop {
        return if index < (spellings.len() as f64) {
            if {
                let source_ml_source2_comparison_operand7 =
                    self::ml_array::optional_at(kinds, self::ml_array::number_index(index));
                {
                    let source_ml_source2_comparison_operand8 =
                        self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]);
                    !(match (
                        source_ml_source2_comparison_operand7,
                        source_ml_source2_comparison_operand8,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            } else {
                {
                    let spelling = (self::ml_array::optional_at(
                        spellings,
                        self::ml_array::number_index(index),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if (depth == 0f64)
                        && ((spelling == (self::ml_string::Text::from_units(vec![38, 38])))
                            || (spelling == (self::ml_string::Text::from_units(vec![124, 124]))))
                    {
                        self::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Result::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Return(self::ml_string::Text::from_units(vec![97, 32, 108, 97, 122, 121, 32, 66, 111, 111, 108, 101, 97, 110, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 112, 97, 114, 101, 110, 116, 104, 101, 115, 105, 122, 101, 100]))
                    } else {
                        if ((spelling == (self::ml_string::Text::from_units(vec![40])))
                            || (spelling == (self::ml_string::Text::from_units(vec![91]))))
                            || (spelling == (self::ml_string::Text::from_units(vec![123])))
                        {
                            {
                                let depth_2 = depth + 1f64;
                                {
                                    let index_3 = index + 1f64;
                                    {
                                        (depth, index) = (depth_2, index_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if ((spelling == (self::ml_string::Text::from_units(vec![41])))
                                || (spelling == (self::ml_string::Text::from_units(vec![93]))))
                                || (spelling == (self::ml_string::Text::from_units(vec![125])))
                            {
                                if depth == 0f64 {
                                    self::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Result::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Return(self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 100, 101, 108, 105, 109, 105, 116, 101, 114, 115, 32, 97, 114, 101, 32, 117, 110, 98, 97, 108, 97, 110, 99, 101, 100]))
                                } else {
                                    {
                                        let depth_3 = depth - 1f64;
                                        if ((depth_3 == 0f64)
                                            && (spelling
                                                == (self::ml_string::Text::from_units(vec![125]))))
                                            && (index == ((spellings.len() as f64) - 1f64))
                                        {
                                            self::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Result::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Return(self::ml_string::Text::from_units(vec![97, 32, 98, 108, 111, 99, 107, 45, 101, 110, 100, 105, 110, 103, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 112, 97, 114, 101, 110, 116, 104, 101, 115, 105, 122, 101, 100]))
                                        } else {
                                            {
                                                let index_4 = index + 1f64;
                                                {
                                                    (depth, index) = (depth_3, index_4);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index_5 = index + 1f64;
                                    {
                                        index = index_5;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Result::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Done(depth)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_record_binding_names_loop7(
    reserved: &[self::ml_string::Text],
    mut count: f64,
    mut names: Vec<self::ml_string::Text>,
    mut suffix: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if (names.len() as f64) < count {
            {
                let name = (self::ml_string::Text::from_units(vec![
                    109, 108, 95, 114, 101, 99, 111, 114, 100, 95, 118, 97, 108, 117, 101,
                ]))
                .concat(&self::ml_string::Text::from_text(
                    &self::ml_number::js_number(suffix),
                ));
                {
                    let suffix_2 = suffix + 1f64;
                    if !(self::ml_array::includes(reserved, &name.clone(), 0f64, false, |a, b| {
                        a == b
                    })) {
                        {
                            let names_2 = {
                                let (ml_append_right, ml_append_left) = (vec![name.clone()], names);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                (names, suffix) = (names_2, suffix_2);
                                continue;
                            }
                        }
                    }
                    {
                        {
                            suffix = suffix_2;
                            continue;
                        }
                    }
                }
            }
        }
        return { names };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_record_constructor_name_loop8(
    reserved: &[self::ml_string::Text],
    mut candidate: self::ml_string::Text,
    mut suffix: f64,
) -> self::ml_string::Text {
    loop {
        if self::ml_array::includes(reserved, &candidate.clone(), 0f64, false, |a, b| a == b) {
            {
                let candidate_2 = (self::ml_string::Text::from_units(vec![86, 97, 108, 117, 101]))
                    .concat(&self::ml_string::Text::from_text(
                        &self::ml_number::js_number(suffix),
                    ));
                {
                    let suffix_2 = suffix + 1f64;
                    {
                        (candidate, suffix) = (candidate_2, suffix_2);
                        continue;
                    }
                }
            }
        }
        return { candidate.clone() };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Result {
    MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Done,
    MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_rust_raw_identifier_name_loop9(
    name: &self::ml_string::Text,
    mut index: f64,
) -> self::MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Result {
    loop {
        return if index < (name.length()) {
            {
                let code = {
                    let source_ml_source2_string_unit_operand9 = name.clone();
                    {
                        let source_ml_source2_string_unit_operand10 = index;
                        (std::rc::Rc::new(
                            move |text: self::ml_string::Text, position: f64| -> f64 {
                                {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                        f64::NAN
                                    } else {
                                        ({
                                    let source_ml_source2_string_unit_operand16 = {
                                        let source_ml_source2_string_range_operand11 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand12 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand13 = index_2 + 1f64;
                                                {
                                                    let source_ml_source2_string_range_position14 = source_ml_source2_string_range_operand12;
                                                    {
                                                        let source_ml_source2_string_range_position15 = source_ml_source2_string_range_operand13;
                                                        source_ml_source2_string_range_operand11.slice(source_ml_source2_string_range_position14, source_ml_source2_string_range_position15)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand17 = 0f64;
                                        source_ml_source2_string_unit_operand16.code_point_at(source_ml_source2_string_unit_operand17)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source2_string_unit_operand9,
                            source_ml_source2_string_unit_operand10,
                        )
                    }
                };
                if ((((code == 95f64) || ((65f64..=90f64).contains(&code)))
                    || ((97f64..=122f64).contains(&code)))
                    || (((index > 0f64) && (code >= 48f64)) && (code <= 57f64)))
                {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Result::MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Return(false)
                }
            }
        } else {
            self::MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Result::MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Result {
    MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Done,
    MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_rust_named_constructor_fields_loop11(
    source_ml_source1_temporary_values10: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index10: self::ml::Big,
) -> self::MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Result {
    loop {
        return if source_ml_source1_temporary_index10
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values10.len() as u128))
        {
            {
                let name = self::ml_array::at(
                    source_ml_source1_temporary_values10,
                    source_ml_source1_temporary_index10.to_index(),
                );
                if (self::accept_rust_raw_identifier_name(&(name))) {
                    {
                        let source_ml_source1_temporary_index10_2 =
                            source_ml_source1_temporary_index10.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index10 =
                                source_ml_source1_temporary_index10_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Result::MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Return(false)
                }
            }
        } else {
            self::MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Result::MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_rust_constructor_fields_loop12(
    names: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut fields: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (values.len() as f64) {
            {
                let fields_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            ((self::render_rust_constructor_field_name(
                                &((self::ml_array::optional_at(
                                    names,
                                    self::ml_array::number_index(index),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))),
                            ))
                            .concat(&self::ml_string::Text::from_units(vec![58, 32])))
                            .concat(
                                &(self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(index),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                            ),
                        ],
                        fields,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (fields, index) = (fields_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { fields };
    }
}
