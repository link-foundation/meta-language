// Generated from complete js/src/translation/rust-record-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_record_fields(
    declared: &[self::ml_string::Text],
    supplied: &[self::ml_string::Text],
) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let ml_s1 = self::ml_plan_rust_record_fields_loop1(declared, index);
            match ml_s1 {
                self::MlPlanRustRecordFieldsLoop1Result::MlPlanRustRecordFieldsLoop1Done => {
                    let index_2 = 0f64;
                    {
                        let ml_s2 =
                            self::ml_plan_rust_record_fields_loop2(declared, supplied, index_2);
                        match ml_s2 {
                                self::MlPlanRustRecordFieldsLoop2Result::MlPlanRustRecordFieldsLoop2Done => {
                                    {
                                        let result = Vec::<f64>::new();
                                        {
                                            let index_3 = 0f64;
                                            {
                                                let ml_s3 = self::ml_plan_rust_record_fields_loop3(declared, supplied, result, index_3);
                                                match ml_s3 {
                                                    self::MlPlanRustRecordFieldsLoop3Result::MlPlanRustRecordFieldsLoop3Done(result_2) => {
                                                        result_2
                                                    }
                                                    self::MlPlanRustRecordFieldsLoop3Result::MlPlanRustRecordFieldsLoop3Return(ml_result3) => {
                                                        ml_result3
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlPlanRustRecordFieldsLoop2Result::MlPlanRustRecordFieldsLoop2Return(ml_result3_2) => {
                                    ml_result3_2
                                }
                            }
                    }
                }
                self::MlPlanRustRecordFieldsLoop1Result::MlPlanRustRecordFieldsLoop1Return(
                    ml_result3_3,
                ) => ml_result3_3,
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_record_field_position(
    declared: &[self::ml_string::Text],
    field: &self::ml_string::Text,
) -> f64 {
    self::ml_array::index_of(declared, field, 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_record_pattern_fields(
    declared: &[self::ml_string::Text],
    supplied: &[self::ml_string::Text],
    rest: bool,
) -> Vec<f64> {
    {
        let checked = self::plan_rust_record_fields(declared, supplied);
        {
            let code = (self::ml_array::optional_at(&checked, self::ml_array::number_index(0f64)))
                .unwrap_or_else(|| 0f64);
            if (code < 0f64) && !((code == -4f64) && rest) {
                checked
            } else {
                {
                    let result = vec![0f64];
                    {
                        let index = 0f64;
                        {
                            self::ml_plan_rust_record_pattern_fields_loop4(
                                declared, supplied, result, index,
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_record_update_fields(
    declared: &[self::ml_string::Text],
    supplied: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_rust_record_pattern_fields(declared, supplied, true)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_record_update_owner(
    structure: bool,
    known: bool,
    actual: &self::ml_string::Text,
    expected: &self::ml_string::Text,
    wrappers: f64,
    diverging: bool,
) -> bool {
    structure && (diverging || ((known && ((*actual) == (*expected))) && (wrappers == 0f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_record_binding_names(
    reserved: &[self::ml_string::Text],
    count: f64,
) -> Vec<self::ml_string::Text> {
    {
        let names = Vec::<self::ml_string::Text>::new();
        {
            let suffix = 0f64;
            { self::ml_read_rust_record_binding_names_loop5(reserved, count, names, suffix) }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_record_constructor_name(
    name: &self::ml_string::Text,
    collision: bool,
    reserved: &[self::ml_string::Text],
) -> self::ml_string::Text {
    if collision {
        {
            let candidate = self::ml_string::Text::from_units(vec![86, 97, 108, 117, 101]);
            {
                let suffix = 2f64;
                { self::ml_read_rust_record_constructor_name_loop6(reserved, candidate, suffix) }
            }
        }
    } else {
        name.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_record_source_name(name: &self::ml_string::Text) -> bool {
    !(self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![98, 111, 111, 108]),
            self::ml_string::Text::from_units(vec![99, 104, 97, 114]),
            self::ml_string::Text::from_units(vec![115, 116, 114]),
            self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]),
            self::ml_string::Text::from_units(vec![86, 101, 99]),
            self::ml_string::Text::from_units(vec![66, 111, 120]),
            self::ml_string::Text::from_units(vec![102, 51, 50]),
            self::ml_string::Text::from_units(vec![102, 54, 52]),
            self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
            self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
            self::ml_string::Text::from_units(vec![117, 56]),
            self::ml_string::Text::from_units(vec![105, 56]),
            self::ml_string::Text::from_units(vec![117, 49, 54]),
            self::ml_string::Text::from_units(vec![105, 49, 54]),
            self::ml_string::Text::from_units(vec![117, 51, 50]),
            self::ml_string::Text::from_units(vec![105, 51, 50]),
            self::ml_string::Text::from_units(vec![117, 54, 52]),
            self::ml_string::Text::from_units(vec![105, 54, 52]),
            self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
            self::ml_string::Text::from_units(vec![115, 116, 100]),
            self::ml_string::Text::from_units(vec![99, 111, 114, 101]),
            self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99]),
        ],
        name,
        0f64,
        false,
        |a, b| a == b,
    ))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_keyword_name(name: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![95]),
            self::ml_string::Text::from_units(vec![97, 115]),
            self::ml_string::Text::from_units(vec![97, 115, 121, 110, 99]),
            self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116]),
            self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107]),
            self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]),
            self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101]),
            self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101]),
            self::ml_string::Text::from_units(vec![100, 121, 110]),
            self::ml_string::Text::from_units(vec![101, 108, 115, 101]),
            self::ml_string::Text::from_units(vec![101, 110, 117, 109]),
            self::ml_string::Text::from_units(vec![101, 120, 116, 101, 114, 110]),
            self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101]),
            self::ml_string::Text::from_units(vec![102, 110]),
            self::ml_string::Text::from_units(vec![102, 111, 114]),
            self::ml_string::Text::from_units(vec![105, 102]),
            self::ml_string::Text::from_units(vec![105, 109, 112, 108]),
            self::ml_string::Text::from_units(vec![105, 110]),
            self::ml_string::Text::from_units(vec![108, 101, 116]),
            self::ml_string::Text::from_units(vec![108, 111, 111, 112]),
            self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104]),
            self::ml_string::Text::from_units(vec![109, 111, 100]),
            self::ml_string::Text::from_units(vec![109, 111, 118, 101]),
            self::ml_string::Text::from_units(vec![109, 117, 116]),
            self::ml_string::Text::from_units(vec![112, 117, 98]),
            self::ml_string::Text::from_units(vec![114, 101, 102]),
            self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]),
            self::ml_string::Text::from_units(vec![115, 101, 108, 102]),
            self::ml_string::Text::from_units(vec![83, 101, 108, 102]),
            self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99]),
            self::ml_string::Text::from_units(vec![115, 116, 114, 117, 99, 116]),
            self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]),
            self::ml_string::Text::from_units(vec![116, 114, 97, 105, 116]),
            self::ml_string::Text::from_units(vec![116, 114, 117, 101]),
            self::ml_string::Text::from_units(vec![116, 121, 112, 101]),
            self::ml_string::Text::from_units(vec![117, 110, 115, 97, 102, 101]),
            self::ml_string::Text::from_units(vec![117, 115, 101]),
            self::ml_string::Text::from_units(vec![119, 104, 101, 114, 101]),
            self::ml_string::Text::from_units(vec![119, 104, 105, 108, 101]),
            self::ml_string::Text::from_units(vec![97, 98, 115, 116, 114, 97, 99, 116]),
            self::ml_string::Text::from_units(vec![98, 101, 99, 111, 109, 101]),
            self::ml_string::Text::from_units(vec![98, 111, 120]),
            self::ml_string::Text::from_units(vec![100, 111]),
            self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108]),
            self::ml_string::Text::from_units(vec![103, 101, 110]),
            self::ml_string::Text::from_units(vec![109, 97, 99, 114, 111]),
            self::ml_string::Text::from_units(vec![111, 118, 101, 114, 114, 105, 100, 101]),
            self::ml_string::Text::from_units(vec![112, 114, 105, 118]),
            self::ml_string::Text::from_units(vec![116, 114, 121]),
            self::ml_string::Text::from_units(vec![116, 121, 112, 101, 111, 102]),
            self::ml_string::Text::from_units(vec![117, 110, 115, 105, 122, 101, 100]),
            self::ml_string::Text::from_units(vec![118, 105, 114, 116, 117, 97, 108]),
            self::ml_string::Text::from_units(vec![121, 105, 101, 108, 100]),
        ],
        name,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_raw_identifier_name(name: &self::ml_string::Text) -> bool {
    if ((name.length()) == 0f64)
        || self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![95]),
                self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101]),
                self::ml_string::Text::from_units(vec![115, 101, 108, 102]),
                self::ml_string::Text::from_units(vec![83, 101, 108, 102]),
                self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]),
            ],
            name,
            0f64,
            false,
            |a, b| a == b,
        )
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let ml_s4 = self::ml_accept_rust_raw_identifier_name_loop7(name, index);
                match ml_s4 {
                    self::MlAcceptRustRawIdentifierNameLoop7Result::MlAcceptRustRawIdentifierNameLoop7Done => {
                        true
                    }
                    self::MlAcceptRustRawIdentifierNameLoop7Result::MlAcceptRustRawIdentifierNameLoop7Return(ml_result7) => {
                        ml_result7
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_keyword_token(
    name: &self::ml_string::Text,
    raw_: &self::ml_string::Text,
) -> bool {
    !(raw_.starts_with(&self::ml_string::Text::from_units(vec![114, 35])))
        || !(self::accept_rust_keyword_name(name))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_raw_identifier_error(name: &self::ml_string::Text) -> self::ml_string::Text {
    if self::accept_rust_raw_identifier_name(name) {
        self::ml_string::Text::from_units(vec![])
    } else {
        (self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 82, 117, 115, 116, 32, 114, 97, 119, 32, 105,
            100, 101, 110, 116, 105, 102, 105, 101, 114, 32, 114, 35,
        ]))
        .concat(name)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_constructor_field_name(name: &self::ml_string::Text) -> self::ml_string::Text {
    if self::accept_rust_keyword_name(name) {
        (self::ml_string::Text::from_units(vec![114, 35])).concat(name)
    } else {
        name.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_named_constructor_fields(
    language: &self::ml_string::Text,
    names: &[self::ml_string::Text],
) -> bool {
    if ((*language) != (self::ml_string::Text::from_units(vec![82, 117, 115, 116])))
        || ((names.len() as f64) == 0f64)
    {
        false
    } else {
        {
            let ml_values8 = names.to_vec();
            {
                let ml_index8 = self::ml::Big::from_i128(0);
                {
                    let ml_s5 = self::ml_accept_rust_named_constructor_fields_loop9(
                        &(ml_values8),
                        ml_index8,
                    );
                    match ml_s5 {
                        self::MlAcceptRustNamedConstructorFieldsLoop9Result::MlAcceptRustNamedConstructorFieldsLoop9Done => {
                            true
                        }
                        self::MlAcceptRustNamedConstructorFieldsLoop9Result::MlAcceptRustNamedConstructorFieldsLoop9Return(ml_result9) => {
                            ml_result9
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_constructor_fields(
    language: &self::ml_string::Text,
    head: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
) -> self::ml_string::Text {
    if (values.len() as f64) == 0f64 {
        head.clone()
    } else if (self::accept_rust_named_constructor_fields(language, names)) {
        {
            let fields = Vec::<self::ml_string::Text>::new();
            {
                let index = 0f64;
                {
                    let fields_2 = self::ml_render_rust_constructor_fields_loop10(
                        names, values, fields, index,
                    );
                    ((head.concat(&self::ml_string::Text::from_units(vec![32, 123, 32]))).concat(
                        &self::ml_string::join(
                            &fields_2,
                            &self::ml_string::Text::from_units(vec![44, 32]),
                        ),
                    ))
                    .concat(&self::ml_string::Text::from_units(vec![32, 125]))
                }
            }
        }
    } else {
        ((head.concat(&self::ml_string::Text::from_units(vec![40]))).concat(
            &self::ml_string::join(values, &self::ml_string::Text::from_units(vec![44, 32])),
        ))
        .concat(&self::ml_string::Text::from_units(vec![41]))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_data_field_encoding(named: bool) -> self::ml_string::Text {
    if named {
        self::ml_string::Text::from_units(vec![
            97, 32, 100, 97, 116, 97, 32, 116, 121, 112, 101, 32, 105, 115, 32, 97, 110, 32, 101,
            110, 117, 109, 32, 119, 104, 111, 115, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99,
            116, 111, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 110, 97, 109, 101, 100, 32,
            111, 114, 32, 112, 111, 115, 105, 116, 105, 111, 110, 97, 108, 32, 102, 105, 101, 108,
            100, 115, 59, 32, 100, 97, 116, 97, 45, 116, 121, 112, 101, 100, 32, 102, 105, 101,
            108, 100, 115, 32, 97, 114, 101, 32, 98, 111, 120, 101, 100, 44, 32, 97, 110, 100, 32,
            118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 99, 111, 109, 112, 97, 114, 101,
            100, 32, 115, 116, 114, 117, 99, 116, 117, 114, 97, 108, 108, 121,
        ])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 100, 97, 116, 97, 32, 116, 121, 112, 101, 32, 105, 115, 32, 97, 110, 32, 101,
            110, 117, 109, 32, 119, 105, 116, 104, 32, 111, 110, 101, 32, 116, 117, 112, 108, 101,
            32, 118, 97, 114, 105, 97, 110, 116, 32, 112, 101, 114, 32, 99, 111, 110, 115, 116,
            114, 117, 99, 116, 111, 114, 59, 32, 100, 97, 116, 97, 45, 116, 121, 112, 101, 100, 32,
            102, 105, 101, 108, 100, 115, 32, 97, 114, 101, 32, 98, 111, 120, 101, 100, 44, 32, 97,
            110, 100, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 99, 111, 109, 112, 97,
            114, 101, 100, 32, 115, 116, 114, 117, 99, 116, 117, 114, 97, 108, 108, 121,
        ])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustRecordFieldsLoop3Result {
    MlPlanRustRecordFieldsLoop3Done(Vec<f64>),
    MlPlanRustRecordFieldsLoop3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_record_fields_loop3(
    declared: &[self::ml_string::Text],
    supplied: &[self::ml_string::Text],
    mut result: Vec<f64>,
    mut index: f64,
) -> self::MlPlanRustRecordFieldsLoop3Result {
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
                    self::MlPlanRustRecordFieldsLoop3Result::MlPlanRustRecordFieldsLoop3Return(
                        vec![-4f64, index],
                    )
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
            self::MlPlanRustRecordFieldsLoop3Result::MlPlanRustRecordFieldsLoop3Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustRecordFieldsLoop2Result {
    MlPlanRustRecordFieldsLoop2Done,
    MlPlanRustRecordFieldsLoop2Return(Vec<f64>),
}

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
pub fn ml_read_rust_record_binding_names_loop5(
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
pub fn ml_read_rust_record_constructor_name_loop6(
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
pub enum MlAcceptRustRawIdentifierNameLoop7Result {
    MlAcceptRustRawIdentifierNameLoop7Done,
    MlAcceptRustRawIdentifierNameLoop7Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_rust_raw_identifier_name_loop7(
    name: &self::ml_string::Text,
    mut index: f64,
) -> self::MlAcceptRustRawIdentifierNameLoop7Result {
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
                    self::MlAcceptRustRawIdentifierNameLoop7Result::MlAcceptRustRawIdentifierNameLoop7Return(false)
                }
            }
        } else {
            self::MlAcceptRustRawIdentifierNameLoop7Result::MlAcceptRustRawIdentifierNameLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptRustNamedConstructorFieldsLoop9Result {
    MlAcceptRustNamedConstructorFieldsLoop9Done,
    MlAcceptRustNamedConstructorFieldsLoop9Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_rust_named_constructor_fields_loop9(
    ml_values8: &[self::ml_string::Text],
    mut ml_index8: self::ml::Big,
) -> self::MlAcceptRustNamedConstructorFieldsLoop9Result {
    loop {
        return if ml_index8 < (self::ml::Big::from_u128(ml_values8.len() as u128)) {
            {
                let name = self::ml_array::at(ml_values8, ml_index8.to_index());
                if (self::accept_rust_raw_identifier_name(&(name))) {
                    {
                        let ml_index8_2 = ml_index8.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index8 = ml_index8_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlAcceptRustNamedConstructorFieldsLoop9Result::MlAcceptRustNamedConstructorFieldsLoop9Return(false)
                }
            }
        } else {
            self::MlAcceptRustNamedConstructorFieldsLoop9Result::MlAcceptRustNamedConstructorFieldsLoop9Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_rust_constructor_fields_loop10(
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
