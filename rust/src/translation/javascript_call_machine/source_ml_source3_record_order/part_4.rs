// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Result {
    MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Done,
    MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Return(bool),
}

pub fn source_ml_temporary_accept_java_script_record_order_positions_loop25(
    mut order: Vec<f64>,
    mut index: f64,
    mut value: f64,
    mut previous: f64,
) -> self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Result
{
    loop {
        return if (previous < index) {
            if ((self::source_ml_source3_record_order::read_java_script_record_order_position(
                order.clone(),
                previous,
            )) == value)
            {
                self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Return(false)
            } else {
                {
                    let previous_2 = (previous + 1f64);
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Done
        };
    }
}

pub fn source_ml_temporary_accept_java_script_record_order_positions_loop24(
    mut count: f64,
    mut order: Vec<f64>,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Result
{
    loop {
        return if (index < (order.len() as f64)) {
            {
                let value =
                    self::source_ml_source3_record_order::read_java_script_record_order_position(
                        order.clone(),
                        index,
                    );
                if (((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value >= count))
                    || ((value == (0f64)) && ((1f64 / value) < (0f64))))
                {
                    self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Return(false)
                } else {
                    {
                        let previous = 0f64;
                        {
                            let source_ml_source1_s28 = self::source_ml_source3_record_order::source_ml_temporary_accept_java_script_record_order_positions_loop25(order.clone(), index, value, previous);
                            match source_ml_source1_s28.clone() {
                                    self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Done => {
                                        {
                                            let index_2 = (index + 1f64);
                                            {
                                                index = index_2;
                                                continue;
                                            }
                                        }
                                    }
                                    self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Return(source_ml_temporary_result25) => {
                                        self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Return(source_ml_temporary_result25)
                                    }
                                }
                        }
                    }
                }
            }
        } else {
            self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Done
        };
    }
}

pub fn source_ml_temporary_sort_java_script_record_property_positions_loop27(
    mut names: Vec<Vec<f64>>,
    mut numeric: Vec<f64>,
    mut key: f64,
    mut position: f64,
) -> f64 {
    loop {
        return if ((position < (numeric.len() as f64))
            && ((self::source_ml_source3_record_order::read_java_script_literal_property_index(
                self::source_ml_source3_record_order::read_java_script_record_order_name(
                    names.clone(),
                    self::source_ml_source3_record_order::read_java_script_record_order_position(
                        numeric.clone(),
                        position,
                    ),
                ),
            )) < key))
        {
            {
                let position_2 = (position + 1f64);
                {
                    position = position_2;
                    continue;
                }
            }
        } else {
            position
        };
    }
}

pub fn source_ml_temporary_sort_java_script_record_property_positions_loop26(
    mut names: Vec<Vec<f64>>,
    mut insertion: Vec<f64>,
    mut numeric: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if (index < (insertion.len() as f64)) {
            {
                let field =
                    self::source_ml_source3_record_order::read_java_script_record_order_position(
                        insertion.clone(),
                        index,
                    );
                {
                    let key = self::source_ml_source3_record_order::read_java_script_literal_property_index(self::source_ml_source3_record_order::read_java_script_record_order_name(names.clone(), field));
                    if (key < (0f64)) {
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let position = 0f64;
                            {
                                let position_2 = self::source_ml_source3_record_order::source_ml_temporary_sort_java_script_record_property_positions_loop27(names.clone(), numeric.clone(), key, position);
                                {
                                    let numeric_2 = {
                                        let source_ml_source1_concat_value29 =
                                            self::ml_array::slice(&numeric, 0f64, position_2);
                                        {
                                            let source_ml_source1_concat_value30 = vec![field];
                                            {
                                                let source_ml_source1_concat_value31 =
                                                    self::ml_array::slice(
                                                        &numeric,
                                                        position_2,
                                                        f64::INFINITY,
                                                    );
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<f64>::new(),
                                                            source_ml_source1_concat_value29,
                                                        ),
                                                        source_ml_source1_concat_value30,
                                                    ),
                                                    source_ml_source1_concat_value31,
                                                )
                                            }
                                        }
                                    };
                                    {
                                        let index_3 = (index + 1f64);
                                        {
                                            (numeric, index) = (numeric_2, index_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            numeric
        };
    }
}

pub fn source_ml_temporary_sort_java_script_record_property_positions_loop28(
    mut names: Vec<Vec<f64>>,
    mut insertion: Vec<f64>,
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if (index < (insertion.len() as f64)) {
            {
                let field =
                    self::source_ml_source3_record_order::read_java_script_record_order_position(
                        insertion.clone(),
                        index,
                    );
                if ((self::source_ml_source3_record_order::read_java_script_literal_property_index(
                    self::source_ml_source3_record_order::read_java_script_record_order_name(
                        names.clone(),
                        field,
                    ),
                )) < (0f64))
                {
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value32 = result;
                            {
                                let source_ml_source1_concat_value33 = vec![field];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value32,
                                    ),
                                    source_ml_source1_concat_value33,
                                )
                            }
                        };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (result, index) = (result_2, index_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let index_3 = (index + 1f64);
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Result {
    MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Done,
}

pub fn source_ml_temporary_create_java_script_present_record_property_order_loop29(mut insertion: Vec<f64>, mut present: Vec<bool>, mut index: f64) -> self::source_ml_source3_record_order::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Result{
    loop {
        return if (index < (present.len() as f64)) {
            if {
                let source_ml_source1_comparison_operand34 =
                    self::ml_array::optional_at(&present, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand35 = self::ml_array::includes(
                        &insertion,
                        &index,
                        0f64,
                        false,
                        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                    );
                    !(match (
                        source_ml_source1_comparison_operand34.clone(),
                        source_ml_source1_comparison_operand35,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100,
                        32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 115, 101,
                        113, 117, 101, 110, 99, 101,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
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
            self::source_ml_source3_record_order::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Result::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result {
    MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Done,
    MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Return(bool),
}

pub fn source_ml_temporary_accept_java_script_present_record_property_order_loop30(mut order: Vec<f64>, mut expected: Vec<f64>, mut index: f64) -> self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result{
    loop {
        return if (index < (order.len() as f64)) {
            if ((self::source_ml_source3_record_order::read_java_script_record_order_position(
                order.clone(),
                index,
            ))
                != (self::source_ml_source3_record_order::read_java_script_record_order_position(
                    expected.clone(),
                    index,
                )))
            {
                self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Return(false)
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
            self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result {
    MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Done,
    MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Return(bool),
}

pub fn source_ml_temporary_accept_java_script_record_property_order_loop31(
    mut order: Vec<f64>,
    mut expected: Vec<f64>,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result
{
    loop {
        return if (index < (order.len() as f64)) {
            if ((self::source_ml_source3_record_order::read_java_script_record_order_position(
                order.clone(),
                index,
            ))
                != (self::source_ml_source3_record_order::read_java_script_record_order_position(
                    expected.clone(),
                    index,
                )))
            {
                self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Return(false)
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
            self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop33(
    mut names: Vec<Vec<f64>>,
    mut insertion: Vec<f64>,
    mut values: Vec<f64>,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result
{
    loop {
        return if (index < (values.len() as f64)) {
            {
                let field =
                    self::source_ml_source3_record_order::read_java_script_record_order_position(
                        values.clone(),
                        index,
                    );
                if (((!(self::ml_math::is_integer(field)) || (field < (0f64)))
                    || (field >= (names.len() as f64)))
                    || ((field == (0f64)) && ((1f64 / field) < (0f64))))
                {
                    self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Return(self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 111, 117, 114, 99, 101, 32, 102, 105, 101, 108, 100]))
                } else {
                    if !(self::ml_array::includes(
                        &insertion,
                        &field,
                        0f64,
                        false,
                        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                    )) {
                        {
                            let insertion_2 = {
                                let source_ml_source1_concat_value36 = insertion;
                                {
                                    let source_ml_source1_concat_value37 = vec![field];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value36,
                                        ),
                                        source_ml_source1_concat_value37,
                                    )
                                }
                            };
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (insertion, index) = (insertion_2, index_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let index_3 = (index + 1f64);
                            {
                                index = index_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Done(insertion)
        };
    }
}

pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop32(
    mut names: Vec<Vec<f64>>,
    mut chunks: Vec<Vec<f64>>,
    mut insertion: Vec<f64>,
    mut chunk: f64,
) -> self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result
{
    loop {
        return if (chunk < (chunks.len() as f64)) {
            {
                let values =
                    self::source_ml_source3_record_order::read_java_script_record_order_name(
                        chunks.clone(),
                        chunk,
                    );
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s38 = self::source_ml_source3_record_order::source_ml_temporary_read_java_script_record_order_merge_error_loop33(names.clone(), insertion, values, index);
                        match source_ml_source1_s38.clone() {
                                self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Done(insertion_2) => {
                                    {
                                        let chunk_2 = (chunk + 1f64);
                                        {
                                            (insertion, chunk) = (insertion_2, chunk_2);
                                            continue;
                                        }
                                    }
                                }
                                self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Return(source_ml_temporary_result33) => {
                                    self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Return(source_ml_temporary_result33.clone())
                                }
                            }
                    }
                }
            }
        } else {
            self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Done(insertion)
        };
    }
}

pub fn source_ml_temporary_merge_java_script_record_property_order_loop35(
    mut insertion: Vec<f64>,
    mut values: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if (index < (values.len() as f64)) {
            {
                let field =
                    self::source_ml_source3_record_order::read_java_script_record_order_position(
                        values.clone(),
                        index,
                    );
                if !(self::ml_array::includes(
                    &insertion,
                    &field,
                    0f64,
                    false,
                    |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                )) {
                    {
                        let insertion_2 = {
                            let source_ml_source1_concat_value39 = insertion;
                            {
                                let source_ml_source1_concat_value40 = vec![field];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value39,
                                    ),
                                    source_ml_source1_concat_value40,
                                )
                            }
                        };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (insertion, index) = (insertion_2, index_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let index_3 = (index + 1f64);
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            insertion
        };
    }
}

pub fn source_ml_temporary_merge_java_script_record_property_order_loop34(
    mut chunks: Vec<Vec<f64>>,
    mut insertion: Vec<f64>,
    mut chunk: f64,
) -> Vec<f64> {
    loop {
        return if (chunk < (chunks.len() as f64)) {
            {
                let values =
                    self::source_ml_source3_record_order::read_java_script_record_order_name(
                        chunks.clone(),
                        chunk,
                    );
                {
                    let index = 0f64;
                    {
                        let insertion_2 = self::source_ml_source3_record_order::source_ml_temporary_merge_java_script_record_property_order_loop35(insertion, values, index);
                        {
                            let chunk_2 = (chunk + 1f64);
                            {
                                (insertion, chunk) = (insertion_2, chunk_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            insertion
        };
    }
}

pub fn source_ml_temporary_merge_java_script_present_record_property_order_loop37(
    mut insertion: Vec<f64>,
    mut positions: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if (index < (positions.len() as f64)) {
            {
                let field =
                    self::source_ml_source3_record_order::read_java_script_record_order_position(
                        positions.clone(),
                        index,
                    );
                if !(self::ml_array::includes(
                    &insertion,
                    &field,
                    0f64,
                    false,
                    |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                )) {
                    {
                        let insertion_2 = {
                            let source_ml_source1_concat_value41 = insertion;
                            {
                                let source_ml_source1_concat_value42 = vec![field];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value41,
                                    ),
                                    source_ml_source1_concat_value42,
                                )
                            }
                        };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (insertion, index) = (insertion_2, index_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let index_3 = (index + 1f64);
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            insertion
        };
    }
}

pub fn source_ml_temporary_merge_java_script_present_record_property_order_loop36(
    mut names: Vec<Vec<f64>>,
    mut chunks: Vec<Vec<f64>>,
    mut insertion: Vec<f64>,
    mut chunk: f64,
) -> Vec<f64> {
    loop {
        return if (chunk < (chunks.len() as f64)) {
            {
                let positions =
                    self::source_ml_source3_record_order::read_java_script_record_order_name(
                        chunks.clone(),
                        chunk,
                    );
                if !(self::source_ml_source3_record_order::accept_java_script_record_order_positions((names.len() as f64), positions.clone())) {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 111, 117, 114, 99, 101, 32, 102, 105, 101, 108, 100])).0, message_present: true }).raise()
                    } else {
                        {
                            let index = 0f64;
                            {
                                let insertion_2 = self::source_ml_source3_record_order::source_ml_temporary_merge_java_script_present_record_property_order_loop37(insertion, positions, index);
                                {
                                    let chunk_2 = (chunk + 1f64);
                                    {
                                        (insertion, chunk) = (insertion_2, chunk_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
            }
        } else {
            insertion
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result {
    MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Done,
    MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_record_order_rebase_error_loop38(
    mut target: Vec<Vec<f64>>,
    mut source: Vec<Vec<f64>>,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result
{
    loop {
        return if (index < (source.len() as f64)) {
            if ((self::source_ml_source3_record_order::read_record_field_position(
                target.clone(),
                self::source_ml_source3_record_order::read_java_script_record_order_name(
                    source.clone(),
                    index,
                ),
            )) < (0f64))
            {
                self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 112, 114, 101, 97, 100, 32, 102, 105, 101, 108, 100, 32, 105, 115, 32, 109, 105, 115, 115, 105, 110, 103, 32, 102, 114, 111, 109, 32, 105, 116, 115, 32, 116, 97, 114, 103, 101, 116]))
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
            self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRebaseJavaScriptPresentRecordPropertyOrderLoop39Result {
    MlTemporaryRebaseJavaScriptPresentRecordPropertyOrderLoop39Done,
}

pub fn source_ml_temporary_rebase_java_script_present_record_property_order_loop39(mut target: Vec<Vec<f64>>, mut source: Vec<Vec<f64>>, mut index: f64) -> self::source_ml_source3_record_order::MlTemporaryRebaseJavaScriptPresentRecordPropertyOrderLoop39Result{
    loop {
        return if (index < (source.len() as f64)) {
            if ((self::source_ml_source3_record_order::read_record_field_position(
                target.clone(),
                self::source_ml_source3_record_order::read_java_script_record_order_name(
                    source.clone(),
                    index,
                ),
            )) < (0f64))
            {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114,
                        100, 32, 115, 112, 114, 101, 97, 100, 32, 102, 105, 101, 108, 100, 32, 105,
                        115, 32, 109, 105, 115, 115, 105, 110, 103, 32, 102, 114, 111, 109, 32,
                        105, 116, 115, 32, 116, 97, 114, 103, 101, 116,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
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
            self::source_ml_source3_record_order::MlTemporaryRebaseJavaScriptPresentRecordPropertyOrderLoop39Result::MlTemporaryRebaseJavaScriptPresentRecordPropertyOrderLoop39Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCopyJavaScriptRecordPropertyOrderLoop40Result {
    MlTemporaryCopyJavaScriptRecordPropertyOrderLoop40Done,
}

pub fn source_ml_temporary_copy_java_script_record_property_order_loop40(
    mut target: Vec<Vec<f64>>,
    mut source: Vec<Vec<f64>>,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop40Result
{
    loop {
        return if (index < (target.len() as f64)) {
            if ((self::source_ml_source3_record_order::read_record_field_position(
                source.clone(),
                self::source_ml_source3_record_order::read_java_script_record_order_name(
                    target.clone(),
                    index,
                ),
            )) < (0f64))
            {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 115, 116, 32, 116,
                        97, 114, 103, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 32,
                        99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32,
                        115, 117, 98, 115, 101, 116,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
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
            self::source_ml_source3_record_order::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop40Result::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop40Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCopyJavaScriptPresentRecordPropertyOrderLoop41Result {
    MlTemporaryCopyJavaScriptPresentRecordPropertyOrderLoop41Done,
}

pub fn source_ml_temporary_copy_java_script_present_record_property_order_loop41(mut target: Vec<Vec<f64>>, mut source: Vec<Vec<f64>>, mut index: f64) -> self::source_ml_source3_record_order::MlTemporaryCopyJavaScriptPresentRecordPropertyOrderLoop41Result{
    loop {
        return if (index < (target.len() as f64)) {
            if ((self::source_ml_source3_record_order::read_record_field_position(
                source.clone(),
                self::source_ml_source3_record_order::read_java_script_record_order_name(
                    target.clone(),
                    index,
                ),
            )) < (0f64))
            {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 115, 116, 32, 116,
                        97, 114, 103, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 32,
                        99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32,
                        115, 117, 98, 115, 101, 116,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
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
            self::source_ml_source3_record_order::MlTemporaryCopyJavaScriptPresentRecordPropertyOrderLoop41Result::MlTemporaryCopyJavaScriptPresentRecordPropertyOrderLoop41Done
        };
    }
}
