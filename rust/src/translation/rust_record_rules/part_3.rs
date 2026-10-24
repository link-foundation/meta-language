// Generated from complete js/src/translation/rust-record-rules.js. Do not edit by hand.
use super::MlPlanRustRecordFieldsLoop2Result;
use super::accept_rust_raw_identifier_name;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::render_rust_constructor_field_name;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_record_fields_loop2(
    declared: &[self::ml_string::Text],
    supplied: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlPlanRustRecordFieldsLoop2Result {
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
                        self::MlPlanRustRecordFieldsLoop2Result::MlPlanRustRecordFieldsLoop2Return(
                            vec![-2f64, index],
                        )
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
                            self::MlPlanRustRecordFieldsLoop2Result::MlPlanRustRecordFieldsLoop2Return(vec![-3f64, index])
                        }
                    }
                }
            }
        } else {
            self::MlPlanRustRecordFieldsLoop2Result::MlPlanRustRecordFieldsLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustRecordFieldsLoop1Result {
    MlPlanRustRecordFieldsLoop1Done,
    MlPlanRustRecordFieldsLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_record_fields_loop1(
    declared: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlPlanRustRecordFieldsLoop1Result {
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
                        self::MlPlanRustRecordFieldsLoop1Result::MlPlanRustRecordFieldsLoop1Return(
                            vec![-1f64, index],
                        )
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
            self::MlPlanRustRecordFieldsLoop1Result::MlPlanRustRecordFieldsLoop1Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_record_pattern_fields_loop4(
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
pub fn ml_plan_tuple_pattern_fields_loop5(
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_rust_record_binding_names_loop6(
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
pub fn ml_read_rust_record_constructor_name_loop7(
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
pub enum MlAcceptRustRawIdentifierNameLoop8Result {
    MlAcceptRustRawIdentifierNameLoop8Done,
    MlAcceptRustRawIdentifierNameLoop8Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_rust_raw_identifier_name_loop8(
    name: &self::ml_string::Text,
    mut index: f64,
) -> self::MlAcceptRustRawIdentifierNameLoop8Result {
    loop {
        return if index < (name.length()) {
            {
                let code =
                    (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 {
                        {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                f64::NAN
                            } else {
                                ((text.slice(index_2, (index_2 + 1f64))).code_point_at(0f64))
                                    .clone()
                                    .unwrap_or_else(|| f64::NAN)
                            }
                        }
                    }))(name.clone(), index);
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
                    self::MlAcceptRustRawIdentifierNameLoop8Result::MlAcceptRustRawIdentifierNameLoop8Return(false)
                }
            }
        } else {
            self::MlAcceptRustRawIdentifierNameLoop8Result::MlAcceptRustRawIdentifierNameLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptRustNamedConstructorFieldsLoop10Result {
    MlAcceptRustNamedConstructorFieldsLoop10Done,
    MlAcceptRustNamedConstructorFieldsLoop10Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_rust_named_constructor_fields_loop10(
    ml_values9: &[self::ml_string::Text],
    mut ml_index9: self::ml::Big,
) -> self::MlAcceptRustNamedConstructorFieldsLoop10Result {
    loop {
        return if ml_index9 < (self::ml::Big::from_u128(ml_values9.len() as u128)) {
            {
                let name = self::ml_array::at(ml_values9, ml_index9.to_index());
                if (self::accept_rust_raw_identifier_name(&(name))) {
                    {
                        let ml_index9_2 = ml_index9.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index9 = ml_index9_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlAcceptRustNamedConstructorFieldsLoop10Result::MlAcceptRustNamedConstructorFieldsLoop10Return(false)
                }
            }
        } else {
            self::MlAcceptRustNamedConstructorFieldsLoop10Result::MlAcceptRustNamedConstructorFieldsLoop10Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_rust_constructor_fields_loop11(
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
