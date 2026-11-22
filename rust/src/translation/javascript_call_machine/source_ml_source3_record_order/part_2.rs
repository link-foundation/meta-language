// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
pub fn create_java_script_present_record_property_order(
    names: Vec<Vec<f64>>,
    insertion: Vec<f64>,
    present: Vec<bool>,
) -> Vec<f64> {
    if ((!(self::source_ml_source3_record_order::accept_java_script_record_order_names(
        names.clone(),
    )) || ((present.len() as f64) != (names.len() as f64)))
        || !(self::source_ml_source3_record_order::accept_java_script_record_order_positions(
            (names.len() as f64),
            insertion.clone(),
        )))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114,
                101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 115, 101, 113, 117, 101,
                110, 99, 101,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s15 = self::source_ml_source3_record_order::source_ml_temporary_create_java_script_present_record_property_order_loop29(insertion.clone(), present, index);
                match source_ml_source1_s15.clone() {
                        self::source_ml_source3_record_order::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Result::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Done => {
                            self::source_ml_source3_record_order::sort_java_script_record_property_positions(names, insertion)
                        }
                    }
            }
        }
    }
}

pub fn accept_java_script_present_record_property_order(
    names: Vec<Vec<f64>>,
    order: Vec<f64>,
) -> bool {
    if (!(self::source_ml_source3_record_order::accept_java_script_record_order_names(
        names.clone(),
    )) || !(self::source_ml_source3_record_order::accept_java_script_record_order_positions(
        (names.len() as f64),
        order.clone(),
    ))) {
        false
    } else {
        {
            let expected =
                self::source_ml_source3_record_order::sort_java_script_record_property_positions(
                    names,
                    order.clone(),
                );
            {
                let index = 0f64;
                {
                    let source_ml_source1_s16 = self::source_ml_source3_record_order::source_ml_temporary_accept_java_script_present_record_property_order_loop30(order, expected, index);
                    match source_ml_source1_s16.clone() {
                            self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Done => {
                                true
                            }
                            self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Return(source_ml_temporary_result30) => {
                                source_ml_temporary_result30
                            }
                        }
                }
            }
        }
    }
}

pub fn accept_java_script_record_property_order(names: Vec<Vec<f64>>, order: Vec<f64>) -> bool {
    if (!(self::source_ml_source3_record_order::accept_java_script_record_order_names(
        names.clone(),
    )) || !(self::source_ml_source3_record_order::accept_java_script_record_order_permutation(
        (names.len() as f64),
        order.clone(),
    ))) {
        false
    } else {
        {
            let expected =
                self::source_ml_source3_record_order::create_java_script_record_property_order(
                    names,
                    order.clone(),
                );
            {
                let index = 0f64;
                {
                    let source_ml_source1_s17 = self::source_ml_source3_record_order::source_ml_temporary_accept_java_script_record_property_order_loop31(order, expected, index);
                    match source_ml_source1_s17.clone() {
                            self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Done => {
                                true
                            }
                            self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Return(source_ml_temporary_result31) => {
                                source_ml_temporary_result31
                            }
                        }
                }
            }
        }
    }
}

pub fn read_java_script_record_order_merge_error(
    names: Vec<Vec<f64>>,
    chunks: Vec<Vec<f64>>,
) -> self::ml_string::Text {
    if !(self::source_ml_source3_record_order::accept_java_script_record_order_names(names.clone()))
    {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101,
            99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101, 115,
        ])
    } else {
        {
            let insertion = Vec::<f64>::new();
            {
                let chunk = 0f64;
                {
                    let source_ml_source1_s18 = self::source_ml_source3_record_order::source_ml_temporary_read_java_script_record_order_merge_error_loop32(names.clone(), chunks, insertion, chunk);
                    match source_ml_source1_s18.clone() {
                            self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Done(insertion_2) => {
                                self::source_ml_source3_record_order::read_java_script_record_order_creation_error(names, insertion_2)
                            }
                            self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Return(source_ml_temporary_result32) => {
                                source_ml_temporary_result32.clone()
                            }
                        }
                }
            }
        }
    }
}

pub fn merge_java_script_record_property_order(
    names: Vec<Vec<f64>>,
    chunks: Vec<Vec<f64>>,
) -> Vec<f64> {
    {
        let error = self::source_ml_source3_record_order::read_java_script_record_order_merge_error(
            names.clone(),
            chunks.clone(),
        );
        if !error.0.is_empty() {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (error.clone()).0,
                message_present: true,
            })
            .raise()
        } else {
            {
                let insertion = Vec::<f64>::new();
                {
                    let chunk = 0f64;
                    {
                        let insertion_2 = self::source_ml_source3_record_order::source_ml_temporary_merge_java_script_record_property_order_loop34(chunks, insertion, chunk);
                        self::source_ml_source3_record_order::create_java_script_record_property_order(names, insertion_2)
                    }
                }
            }
        }
    }
}

pub fn merge_java_script_present_record_property_order(
    names: Vec<Vec<f64>>,
    chunks: Vec<Vec<f64>>,
) -> Vec<f64> {
    if !(self::source_ml_source3_record_order::accept_java_script_record_order_names(names.clone()))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114,
                101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101, 115,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let insertion = Vec::<f64>::new();
            {
                let chunk = 0f64;
                {
                    let insertion_2 = self::source_ml_source3_record_order::source_ml_temporary_merge_java_script_present_record_property_order_loop36(names.clone(), chunks, insertion, chunk);
                    self::source_ml_source3_record_order::sort_java_script_record_property_positions(
                        names,
                        insertion_2,
                    )
                }
            }
        }
    }
}

pub fn read_java_script_record_order_rebase_error(
    target: Vec<Vec<f64>>,
    source: Vec<Vec<f64>>,
    order: Vec<f64>,
) -> self::ml_string::Text {
    if (!(self::source_ml_source3_record_order::accept_java_script_record_order_names(
        target.clone(),
    )) || !(self::source_ml_source3_record_order::accept_java_script_record_property_order(
        source.clone(),
        order,
    ))) {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101,
            99, 111, 114, 100, 32, 115, 112, 114, 101, 97, 100, 32, 115, 111, 117, 114, 99, 101,
        ])
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s19 = self::source_ml_source3_record_order::source_ml_temporary_read_java_script_record_order_rebase_error_loop38(target, source, index);
                match source_ml_source1_s19.clone() {
                        self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Done => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::source_ml_source3_record_order::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Return(source_ml_temporary_result38) => {
                            source_ml_temporary_result38.clone()
                        }
                    }
            }
        }
    }
}

pub fn rebase_java_script_record_property_order(
    target: Vec<Vec<f64>>,
    source: Vec<Vec<f64>>,
    order: Vec<f64>,
) -> Vec<f64> {
    {
        let error =
            self::source_ml_source3_record_order::read_java_script_record_order_rebase_error(
                target.clone(),
                source.clone(),
                order.clone(),
            );
        if !error.0.is_empty() {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (error.clone()).0,
                message_present: true,
            })
            .raise()
        } else {
            self::source_ml_source3_record_order::project_java_script_record_property_order(
                target, source, order,
            )
        }
    }
}

pub fn rebase_java_script_present_record_property_order(
    target: Vec<Vec<f64>>,
    source: Vec<Vec<f64>>,
    order: Vec<f64>,
) -> Vec<f64> {
    if (!(self::source_ml_source3_record_order::accept_java_script_record_order_names(target.clone())) || !(self::source_ml_source3_record_order::accept_java_script_present_record_property_order(source.clone(), order.clone()))) {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 112, 114, 101, 97, 100, 32, 115, 111, 117, 114, 99, 101])).0, message_present: true }).raise()
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_s20 = self::source_ml_source3_record_order::source_ml_temporary_rebase_java_script_present_record_property_order_loop39(target.clone(), source.clone(), index);
                    match source_ml_source1_s20.clone() {
                        self::source_ml_source3_record_order::MlTemporaryRebaseJavaScriptPresentRecordPropertyOrderLoop39Result::MlTemporaryRebaseJavaScriptPresentRecordPropertyOrderLoop39Done => {
                            self::source_ml_source3_record_order::project_java_script_record_property_order(target, source, order)
                        }
                    }
                }
            }
        }
}

pub fn copy_java_script_record_property_order(
    target: Vec<Vec<f64>>,
    source: Vec<Vec<f64>>,
    order: Vec<f64>,
) -> Vec<f64> {
    if (!(self::source_ml_source3_record_order::accept_java_script_record_order_names(
        target.clone(),
    )) || !(self::source_ml_source3_record_order::accept_java_script_record_property_order(
        source.clone(),
        order.clone(),
    ))) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101,
                115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 105, 101, 108, 100, 32, 111,
                114, 100, 101, 114,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s21 = self::source_ml_source3_record_order::source_ml_temporary_copy_java_script_record_property_order_loop40(target.clone(), source.clone(), index);
                match source_ml_source1_s21.clone() {
                        self::source_ml_source3_record_order::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop40Result::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop40Done => {
                            self::source_ml_source3_record_order::project_java_script_record_property_order(target, source, order)
                        }
                    }
            }
        }
    }
}

pub fn copy_java_script_present_record_property_order(
    target: Vec<Vec<f64>>,
    source: Vec<Vec<f64>>,
    order: Vec<f64>,
) -> Vec<f64> {
    if (!(self::source_ml_source3_record_order::accept_java_script_record_order_names(target.clone())) || !(self::source_ml_source3_record_order::accept_java_script_present_record_property_order(source.clone(), order.clone()))) {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 105, 101, 108, 100, 32, 111, 114, 100, 101, 114])).0, message_present: true }).raise()
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_s22 = self::source_ml_source3_record_order::source_ml_temporary_copy_java_script_present_record_property_order_loop41(target.clone(), source.clone(), index);
                    match source_ml_source1_s22.clone() {
                        self::source_ml_source3_record_order::MlTemporaryCopyJavaScriptPresentRecordPropertyOrderLoop41Result::MlTemporaryCopyJavaScriptPresentRecordPropertyOrderLoop41Done => {
                            self::source_ml_source3_record_order::project_java_script_record_property_order(target, source, order)
                        }
                    }
                }
            }
        }
}

pub fn project_java_script_record_property_order(
    target: Vec<Vec<f64>>,
    source: Vec<Vec<f64>>,
    order: Vec<f64>,
) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let index = 0f64;
            {
                let result_2 = self::source_ml_source3_record_order::source_ml_temporary_project_java_script_record_property_order_loop42(target, source, order, result, index);
                result_2
            }
        }
    }
}

pub fn read_java_script_record_order_key_error(
    names: Vec<Vec<f64>>,
    order: Vec<f64>,
) -> self::ml_string::Text {
    if self::source_ml_source3_record_order::accept_java_script_record_property_order(names, order)
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101,
            99, 111, 114, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 114, 100, 101,
            114,
        ])
    }
}

pub fn read_java_script_record_property_keys(
    names: Vec<Vec<f64>>,
    order: Vec<f64>,
) -> Vec<Vec<f64>> {
    {
        let error = self::source_ml_source3_record_order::read_java_script_record_order_key_error(
            names.clone(),
            order.clone(),
        );
        if !error.0.is_empty() {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (error.clone()).0,
                message_present: true,
            })
            .raise()
        } else {
            {
                let result = Vec::<Vec<f64>>::new();
                {
                    let index = 0f64;
                    {
                        let result_2 = self::source_ml_source3_record_order::source_ml_temporary_read_java_script_record_property_keys_loop43(names, order, result, index);
                        result_2
                    }
                }
            }
        }
    }
}

pub fn read_java_script_record_property_key_texts(
    names: Vec<Vec<f64>>,
    order: Vec<f64>,
) -> Vec<self::ml_string::Text> {
    {
        let rows = self::source_ml_source3_record_order::read_java_script_record_property_keys(
            names, order,
        );
        self::source_ml_source3_record_order::read_java_script_record_property_name_texts(rows)
    }
}

pub fn read_java_script_present_record_property_key_texts(
    names: Vec<Vec<f64>>,
    order: Vec<f64>,
) -> Vec<self::ml_string::Text> {
    if !(self::source_ml_source3_record_order::accept_java_script_present_record_property_order(
        names.clone(),
        order.clone(),
    )) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114,
                101, 99, 111, 114, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 114,
                100, 101, 114,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let rows = Vec::<Vec<f64>>::new();
            {
                let index = 0f64;
                {
                    let rows_2 = self::source_ml_source3_record_order::source_ml_temporary_read_java_script_present_record_property_key_texts_loop44(names, order, rows, index);
                    self::source_ml_source3_record_order::read_java_script_record_property_name_texts(rows_2)
                }
            }
        }
    }
}

pub fn read_java_script_record_property_name_texts(
    rows: Vec<Vec<f64>>,
) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let index = 0f64;
            {
                let result_2 = self::source_ml_source3_record_order::source_ml_temporary_read_java_script_record_property_name_texts_loop45(rows, result, index);
                result_2
            }
        }
    }
}

pub fn read_java_script_record_property_index(
    names: Vec<Vec<f64>>,
    key: self::ml_string::Text,
) -> f64 {
    {
        let units = Vec::<f64>::new();
        {
            let index = 0f64;
            {
                let units_2 = self::source_ml_source3_record_order::source_ml_temporary_read_java_script_record_property_index_loop47(key.clone(), units, index);
                self::source_ml_source3_record_order::read_record_field_position(names, units_2)
            }
        }
    }
}

pub fn create_java_script_dense_property_key_texts(length: f64) -> Vec<self::ml_string::Text> {
    if ((!(self::ml_math::is_safe_integer(length)) || (length < (0f64)))
        || ((length == (0f64)) && ((1f64 / length) < (0f64))))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 100, 101, 110, 115, 101, 32, 111, 119, 110,
                45, 102, 105, 101, 108, 100, 32, 108, 101, 110, 103, 116, 104,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let result = Vec::<self::ml_string::Text>::new();
            {
                let index = 0f64;
                {
                    let result_2 = self::source_ml_source3_record_order::source_ml_temporary_create_java_script_dense_property_key_texts_loop48(length, result, index);
                    result_2
                }
            }
        }
    }
}

pub fn create_java_script_string_own_property_values(
    text: self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let index = 0f64;
            {
                let result_2 = self::source_ml_source3_record_order::source_ml_temporary_create_java_script_string_own_property_values_loop49(text.clone(), result, index);
                result_2
            }
        }
    }
}

pub fn source_ml_temporary_read_record_value_order_loop1(
    mut count: f64,
    mut reverse: bool,
    mut order: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if (index < count) {
            {
                let order_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![if reverse {
                            ((count - index) - 1f64)
                        } else {
                            index
                        }],
                        order,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        (order, index) = (order_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            order
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRecordProjectionErrorLoop2Result {
    MlTemporaryReadRecordProjectionErrorLoop2Done,
    MlTemporaryReadRecordProjectionErrorLoop2Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_record_projection_error_loop2(
    mut names: Vec<Vec<f64>>,
    mut entries: Vec<bool>,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryReadRecordProjectionErrorLoop2Result {
    loop {
        return if (index < (names.len() as f64)) {
            if !(match self::ml_array::optional_at(&entries, self::ml_array::number_index(index)) {
                Some(ml_present) => ml_present,
                None => false,
            }) {
                self::source_ml_source3_record_order::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 104, 97, 115, 32, 97, 32, 109, 97, 108, 102, 111, 114, 109, 101, 100, 32, 102, 105, 101, 108, 100, 32, 101, 110, 116, 114, 121]))
            } else {
                if !(self::source_ml_source3_record_order::accept_record_field_name(
                    self::source_ml_source3_record_order::read_required_record_name(
                        names.clone(),
                        index,
                    ),
                )) {
                    self::source_ml_source3_record_order::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 104, 97, 115, 32, 97, 110, 32, 105, 110, 118, 97, 108, 105, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101]))
                } else {
                    if ((index > (0f64))
                        && ((self::source_ml_source3_record_order::compare_record_field_names(
                            self::source_ml_source3_record_order::read_required_record_name(
                                names.clone(),
                                (index - 1f64),
                            ),
                            self::source_ml_source3_record_order::read_required_record_name(
                                names.clone(),
                                index,
                            ),
                        )) >= (0f64)))
                    {
                        self::source_ml_source3_record_order::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 117, 110, 105, 113, 117, 101, 32, 102, 105, 101, 108, 100, 115, 32, 105, 110, 32, 85, 84, 70, 49, 54, 32, 111, 114, 100, 101, 114]))
                    } else {
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::source_ml_source3_record_order::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRecordFieldNameLoop3Result {
    MlTemporaryAcceptRecordFieldNameLoop3Done,
    MlTemporaryAcceptRecordFieldNameLoop3Return(bool),
}

pub fn source_ml_temporary_accept_record_field_name_loop3(
    mut name: Vec<f64>,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryAcceptRecordFieldNameLoop3Result {
    loop {
        return if (index < (name.len() as f64)) {
            {
                let unit = self::ml_array::optional_at(&name, self::ml_array::number_index(index));
                if ((!(match unit.clone() {
                    Some(ml_present) => self::ml_math::is_integer(ml_present),
                    None => false,
                }) || {
                    let source_ml_source1_comparison_operand23 = unit.clone();
                    {
                        let source_ml_source1_comparison_operand24 = 0f64;
                        if match (source_ml_source1_comparison_operand23.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) < source_ml_source1_comparison_operand24)
                        } else {
                            ((source_ml_source1_comparison_operand23
                                .clone()
                                .expect("a proven present value"))
                                < source_ml_source1_comparison_operand24)
                        }
                    }
                }) || {
                    let source_ml_source1_comparison_operand25 = unit.clone();
                    {
                        let source_ml_source1_comparison_operand26 = 65535f64;
                        if match (source_ml_source1_comparison_operand25.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) > source_ml_source1_comparison_operand26)
                        } else {
                            ((source_ml_source1_comparison_operand25
                                .clone()
                                .expect("a proven present value"))
                                > source_ml_source1_comparison_operand26)
                        }
                    }
                }) {
                    self::source_ml_source3_record_order::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Return(false)
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::source_ml_source3_record_order::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareRecordFieldNamesLoop4Result {
    MlTemporaryCompareRecordFieldNamesLoop4Done,
    MlTemporaryCompareRecordFieldNamesLoop4Return(f64),
}

pub fn source_ml_temporary_compare_record_field_names_loop4(
    mut left: Vec<f64>,
    mut right: Vec<f64>,
    mut end: f64,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryCompareRecordFieldNamesLoop4Result {
    loop {
        return if (index < end) {
            if ((self::source_ml_source3_record_order::read_required_number_fact(
                left.clone(),
                index,
            )) < (self::source_ml_source3_record_order::read_required_number_fact(
                right.clone(),
                index,
            ))) {
                self::source_ml_source3_record_order::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Return((-1f64))
            } else {
                if ((self::source_ml_source3_record_order::read_required_number_fact(
                    left.clone(),
                    index,
                )) > (self::source_ml_source3_record_order::read_required_number_fact(
                    right.clone(),
                    index,
                ))) {
                    self::source_ml_source3_record_order::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Return(1f64)
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::source_ml_source3_record_order::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordFieldPositionLoop6Result {
    MlTemporaryReadRecordFieldPositionLoop6Done,
    MlTemporaryReadRecordFieldPositionLoop6Return(f64),
}

pub fn source_ml_temporary_read_record_field_position_loop6(
    mut fields: Vec<Vec<f64>>,
    mut name: Vec<f64>,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryReadRecordFieldPositionLoop6Result {
    loop {
        return if (index < (fields.len() as f64)) {
            {
                let comparison = self::source_ml_source3_record_order::compare_record_field_names(
                    self::source_ml_source3_record_order::read_required_record_name(
                        fields.clone(),
                        index,
                    ),
                    name.clone(),
                );
                if (!(comparison < (0f64)) && !(comparison > (0f64))) {
                    self::source_ml_source3_record_order::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Return(index)
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::source_ml_source3_record_order::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordFieldPositionLoop5Result {
    MlTemporaryReadRecordFieldPositionLoop5Done,
    MlTemporaryReadRecordFieldPositionLoop5Return(f64),
}

pub fn source_ml_temporary_read_record_field_position_loop5(
    mut fields: Vec<Vec<f64>>,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryReadRecordFieldPositionLoop5Result {
    loop {
        return if (index < (fields.len() as f64)) {
            if !(self::source_ml_source3_record_order::accept_record_field_name(
                self::source_ml_source3_record_order::read_required_record_name(
                    fields.clone(),
                    index,
                ),
            )) {
                self::source_ml_source3_record_order::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Return((-1f64))
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
            self::source_ml_source3_record_order::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Done
        };
    }
}

pub fn source_ml_temporary_read_record_sorted_field_positions_loop10(
    mut fields: Vec<Vec<f64>>,
    mut sorted: Vec<f64>,
    mut merged: Vec<f64>,
    mut middle: f64,
    mut end: f64,
    mut left: f64,
    mut right: f64,
) -> Vec<f64> {
    loop {
        return if ((left < middle) || (right < end)) {
            if ((right >= end)
                || ((left < middle)
                    && ((self::source_ml_source3_record_order::compare_record_field_names(
                        self::source_ml_source3_record_order::read_required_record_name(
                            fields.clone(),
                            self::source_ml_source3_record_order::read_required_number_fact(
                                sorted.clone(),
                                left,
                            ),
                        ),
                        self::source_ml_source3_record_order::read_required_record_name(
                            fields.clone(),
                            self::source_ml_source3_record_order::read_required_number_fact(
                                sorted.clone(),
                                right,
                            ),
                        ),
                    )) <= (0f64))))
            {
                {
                    let merged_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![
                                self::source_ml_source3_record_order::read_required_number_fact(
                                    sorted.clone(),
                                    left,
                                ),
                            ],
                            merged,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let left_2 = (left + 1f64);
                        {
                            (merged, left) = (merged_2, left_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let merged_3 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![
                                self::source_ml_source3_record_order::read_required_number_fact(
                                    sorted.clone(),
                                    right,
                                ),
                            ],
                            merged,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let right_2 = (right + 1f64);
                        {
                            (merged, right) = (merged_3, right_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            merged
        };
    }
}
