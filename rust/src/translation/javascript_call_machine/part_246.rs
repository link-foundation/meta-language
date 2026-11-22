// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCallSourceUnit;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistryEntry;
use super::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Result;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_text_unit;
use super::read_java_script_semantic_value_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_function_source_directory_entries_error_loop1147(
    mut sources: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>,
    mut total: f64,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Result {
    loop {
        return if (index
            < ({
                let ml_reference_array2978 = sources.clone();
                let ml_reference_projection2978 = ml_reference_array2978.borrow().len() as f64;
                ml_reference_projection2978
            })) {
            {
                let row = {
                    let ml_reference_array2979 = sources.clone();
                    let ml_reference_position2979 = index;
                    let ml_reference_projection2979 = self::ml_array::optional_at(
                        &*ml_reference_array2979.borrow(),
                        self::ml_array::number_index(ml_reference_position2979),
                    );
                    ml_reference_projection2979
                };
                if ((((((((((match (row.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => false,
                    _ => false,
                } || ((match row.clone() {
                    Some(ml_present) => {
                        let _ = ml_present;
                        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                    }
                    None => self::ml_string::Text::from_units(vec![
                        117, 110, 100, 101, 102, 105, 110, 101, 100,
                    ]),
                }) != (self::ml_string::Text::from_units(vec![
                    111, 98, 106, 101, 99, 116,
                ]))))
                    || {
                        let source_ml_source2_nullish_field_receiver41634 = (row.clone(),);
                        if match (source_ml_source2_nullish_field_receiver41634.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source2_nullish_field_present41635 =
                                    ((source_ml_source2_nullish_field_receiver41634.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source2_nullish_field_present41635.clone() {
                                    self::JavaScriptCallSourceUnit::CallSourceUnit(_, _) => false,
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 36, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    })
                    || (({
                        let source_ml_source2_type_query_operand41645 = {
                            let source_ml_source2_nullish_field_receiver41642 = (row.clone(),);
                            if match (source_ml_source2_nullish_field_receiver41642.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present41643 =
                                        ((source_ml_source2_nullish_field_receiver41642.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source2_nullish_field_present41643.clone() {
                                        self::JavaScriptCallSourceUnit::CallSourceUnit(
                                            source_ml_source2_data_property_value41644,
                                            _,
                                        ) => source_ml_source2_data_property_value41644.clone(),
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101,
                                        110, 116, 105, 102, 105, 101, 114, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        };
                        self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
                    }) != (self::ml_string::Text::from_units(vec![
                        115, 116, 114, 105, 110, 103,
                    ]))))
                    || ((({
                        let source_ml_source2_nullish_field_receiver41649 = (row.clone(),);
                        if match (source_ml_source2_nullish_field_receiver41649.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source2_nullish_field_present41650 =
                                    ((source_ml_source2_nullish_field_receiver41649.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source2_nullish_field_present41650.clone() {
                                    self::JavaScriptCallSourceUnit::CallSourceUnit(
                                        source_ml_source2_data_property_value41651,
                                        _,
                                    ) => source_ml_source2_data_property_value41651.clone(),
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105,
                                    102, 105, 101, 114, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    })
                    .length())
                        == (0f64)))
                    || ((({
                        let source_ml_source2_nullish_field_receiver41655 = (row.clone(),);
                        if match (source_ml_source2_nullish_field_receiver41655.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source2_nullish_field_present41656 =
                                    ((source_ml_source2_nullish_field_receiver41655.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source2_nullish_field_present41656.clone() {
                                    self::JavaScriptCallSourceUnit::CallSourceUnit(
                                        source_ml_source2_data_property_value41657,
                                        _,
                                    ) => source_ml_source2_data_property_value41657.clone(),
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105,
                                    102, 105, 101, 114, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    })
                    .length())
                        > (8192f64)))
                    || {
                        let ml_values2980 = {
                            let ml_reference_value2981 =
                                (std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(
                                    &{
                                        let ml_reference_value2982 = sources.borrow().clone();
                                        ml_reference_value2982
                                    },
                                    0f64,
                                    index,
                                ))))
                                .borrow()
                                .clone();
                            ml_reference_value2981
                        };
                        let ml_callback2980 = {
                            let row = row.clone();
                            std::rc::Rc::new(move |prior: self::JavaScriptCallSourceUnit| -> bool {
                                ((match prior.clone() {
                                    self::JavaScriptCallSourceUnit::CallSourceUnit(
                                        source_ml_source2_data_property_value41662,
                                        _,
                                    ) => source_ml_source2_data_property_value41662.clone(),
                                }) == ({
                                    let source_ml_source2_nullish_field_receiver41663 =
                                        (row.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver41663.clone())
                                        .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present41664 =
                                                ((source_ml_source2_nullish_field_receiver41663
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source2_nullish_field_present41664
                                                .clone()
                                            {
                                                self::JavaScriptCallSourceUnit::CallSourceUnit(
                                                    source_ml_source2_data_property_value41665,
                                                    _,
                                                ) => source_ml_source2_data_property_value41665
                                                    .clone(),
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97,
                                                    100, 32, 112, 114, 111, 112, 101, 114, 116,
                                                    105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                                                    101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                                    97, 100, 105, 110, 103, 32, 39, 105, 100, 101,
                                                    110, 116, 105, 102, 105, 101, 114, 39, 41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                }))
                            })
                        };
                        ml_values2980
                            .iter()
                            .cloned()
                            .enumerate()
                            .any(|(ml_index2980, ml_value2980)| ml_callback2980(ml_value2980))
                    })
                    || match (
                        {
                            let source_ml_source2_nullish_field_receiver41666 = (row.clone(),);
                            if match (source_ml_source2_nullish_field_receiver41666.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present41667 =
                                        ((source_ml_source2_nullish_field_receiver41666.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source2_nullish_field_present41667.clone() {
                                        self::JavaScriptCallSourceUnit::CallSourceUnit(
                                            _,
                                            source_ml_source2_data_property_value41668,
                                        ) => {
                                            let source_ml_source2_data_property_value41668 =
                                                *source_ml_source2_data_property_value41668;
                                            source_ml_source2_data_property_value41668.clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 116, 101, 120,
                                        116, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        },
                        None::<()>,
                    ) {
                        (_ml_left, Some(_ml_right)) => false,
                        _ => false,
                    })
                    || (({
                        let source_ml_source2_type_query_operand41676 = {
                            let source_ml_source2_nullish_field_receiver41673 = (row.clone(),);
                            if match (source_ml_source2_nullish_field_receiver41673.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present41674 =
                                        ((source_ml_source2_nullish_field_receiver41673.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source2_nullish_field_present41674.clone() {
                                        self::JavaScriptCallSourceUnit::CallSourceUnit(
                                            _,
                                            source_ml_source2_data_property_value41675,
                                        ) => {
                                            let source_ml_source2_data_property_value41675 =
                                                *source_ml_source2_data_property_value41675;
                                            source_ml_source2_data_property_value41675.clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 116, 101, 120,
                                        116, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        };
                        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                    }) != (self::ml_string::Text::from_units(vec![
                        111, 98, 106, 101, 99, 116,
                    ]))))
                    || ({
                        let source_ml_source2_s41680 = {
                            let source_ml_source2_nullish_field_receiver41682 = (row.clone(),);
                            if match (source_ml_source2_nullish_field_receiver41682.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present41683 =
                                        ((source_ml_source2_nullish_field_receiver41682.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source2_nullish_field_present41683.clone() {
                                        self::JavaScriptCallSourceUnit::CallSourceUnit(
                                            _,
                                            source_ml_source2_data_property_value41684,
                                        ) => {
                                            let source_ml_source2_data_property_value41684 =
                                                *source_ml_source2_data_property_value41684;
                                            source_ml_source2_data_property_value41684.clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 116, 101, 120,
                                        116, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        };
                        match source_ml_source2_s41680.clone() {
                            self::JavaScriptSemanticValue::Text(_) => false,
                            _ => true,
                        }
                    } && {
                        let source_ml_source2_s41688 = {
                            let source_ml_source2_nullish_field_receiver41691 = (row.clone(),);
                            if match (source_ml_source2_nullish_field_receiver41691.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present41692 =
                                        ((source_ml_source2_nullish_field_receiver41691.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source2_nullish_field_present41692.clone() {
                                        self::JavaScriptCallSourceUnit::CallSourceUnit(
                                            _,
                                            source_ml_source2_data_property_value41693,
                                        ) => {
                                            let source_ml_source2_data_property_value41693 =
                                                *source_ml_source2_data_property_value41693;
                                            source_ml_source2_data_property_value41693.clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 116, 101, 120,
                                        116, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        };
                        match source_ml_source2_s41688.clone() {
                            self::JavaScriptSemanticValue::SourceText(_, _) => false,
                            _ => true,
                        }
                    }))
                    || ((self::read_java_script_semantic_value_error({
                        let source_ml_source2_nullish_field_receiver41697 = (row.clone(),);
                        if match (source_ml_source2_nullish_field_receiver41697.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source2_nullish_field_present41698 =
                                    ((source_ml_source2_nullish_field_receiver41697.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source2_nullish_field_present41698.clone() {
                                    self::JavaScriptCallSourceUnit::CallSourceUnit(
                                        _,
                                        source_ml_source2_data_property_value41699,
                                    ) => {
                                        let source_ml_source2_data_property_value41699 =
                                            *source_ml_source2_data_property_value41699;
                                        source_ml_source2_data_property_value41699.clone()
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 116, 101, 120, 116, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    })) != (self::ml_string::Text::from_units(vec![]))))
                {
                    self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Return(self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 115, 111, 117, 114, 99, 101, 32, 117, 110, 105, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 105, 100, 101, 110, 116, 105, 102, 105, 101, 114, 115, 32, 97, 110, 100, 32, 116, 101, 120, 116]))
                } else {
                    {
                        let total_2 = (total
                            + self::read_java_script_semantic_text_length({
                                let source_ml_source2_nullish_field_receiver41700 = (row.clone(),);
                                if match (source_ml_source2_nullish_field_receiver41700.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source2_nullish_field_present41701 =
                                            ((source_ml_source2_nullish_field_receiver41700
                                                .clone())
                                            .0)
                                                .clone()
                                                .expect("a proven present value");
                                        match source_ml_source2_nullish_field_present41701.clone() {
                                            self::JavaScriptCallSourceUnit::CallSourceUnit(
                                                _,
                                                source_ml_source2_data_property_value41702,
                                            ) => {
                                                let source_ml_source2_data_property_value41702 =
                                                    *source_ml_source2_data_property_value41702;
                                                source_ml_source2_data_property_value41702.clone()
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (self::ml_string::Text::from_units(vec![
                                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32,
                                            112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32,
                                            111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101,
                                            100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39,
                                            116, 101, 120, 116, 39, 41,
                                        ]))
                                        .0,
                                        message_present: true,
                                    })
                                    .raise()
                                }
                            }));
                        if (total_2 > (16777216f64)) {
                            self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Return(self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 115, 111, 117, 114, 99, 101, 32, 117, 110, 105, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101]))
                        } else {
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (total, index) = (total_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryProjectJavaScriptFunctionSourceTextLoop1150Result {
    MlSource1TemporaryProjectJavaScriptFunctionSourceTextLoop1150Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_project_java_script_function_source_text_loop1150(
    mut span: self::JavaScriptCallSourceSpan,
    mut unit: Option<self::JavaScriptCallSourceUnit>,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryProjectJavaScriptFunctionSourceTextLoop1150Result {
    loop {
        return if (index
            < (match span.clone() {
                self::JavaScriptCallSourceSpan::CallSourceSpan(
                    _,
                    _,
                    source_ml_source2_data_property_value41704,
                ) => source_ml_source2_data_property_value41704,
            })) {
            {
                let source_ml_source1_temporary_project_java_script_function_source_text_ignored1151 = {
                    let ml_method_owner2983 = units.clone();
                    let ml_method_argument2983_0 = self::read_java_script_semantic_text_unit(
                        {
                            let source_ml_source2_nullish_field_receiver41705 = (unit.clone(),);
                            if match (source_ml_source2_nullish_field_receiver41705.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present41706 =
                                        ((source_ml_source2_nullish_field_receiver41705.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source2_nullish_field_present41706.clone() {
                                        self::JavaScriptCallSourceUnit::CallSourceUnit(
                                            _,
                                            source_ml_source2_data_property_value41707,
                                        ) => {
                                            let source_ml_source2_data_property_value41707 =
                                                *source_ml_source2_data_property_value41707;
                                            source_ml_source2_data_property_value41707.clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 116, 101, 120,
                                        116, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        },
                        index,
                    );
                    let ml_inserted = vec![ml_method_argument2983_0];
                    let mut ml_method_values2983 = ml_method_owner2983.borrow_mut();
                    let ml_length = ml_method_values2983
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2983.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProjectJavaScriptFunctionSourceTextLoop1150Result::MlSource1TemporaryProjectJavaScriptFunctionSourceTextLoop1150Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Result {
    MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Done(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    ),
    MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Return(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Result {
    MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Done,
    MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Return(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Result {
    MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Done,
    MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Return(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1159(
    mut entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut end: f64,
    mut index: f64,
) -> self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Result {
    loop {
        return if (index < end) {
            {
                let entry = {
                    let ml_reference_array2984 = source.clone();
                    let ml_reference_position2984 = index;
                    let ml_reference_projection2984 = self::ml_array::optional_at(
                        &*ml_reference_array2984.borrow(),
                        self::ml_array::number_index(ml_reference_position2984),
                    );
                    ml_reference_projection2984
                };
                if match (entry.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Return(entries.clone())
                } else {
                    {
                        let source_ml_source1_temporary_order_java_script_symbol_registry_entries_ignored1160 = {
                            let ml_method_owner2985 = target.clone();
                            let ml_method_argument2985_0 =
                                entry.clone().expect("a proven present value");
                            let ml_inserted = vec![ml_method_argument2985_0];
                            let mut ml_method_values2985 = ml_method_owner2985.borrow_mut();
                            let ml_length = ml_method_values2985
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4294967295usize)
                                .expect("Invalid array length");
                            ml_method_values2985.extend(ml_inserted);
                            ml_length as f64
                        };
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
            self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Result {
    MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Done,
    MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Return(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1158(
    mut entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut middle: f64,
    mut index: f64,
) -> self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Result {
    loop {
        return if (index < middle) {
            {
                let entry = {
                    let ml_reference_array2986 = source.clone();
                    let ml_reference_position2986 = index;
                    let ml_reference_projection2986 = self::ml_array::optional_at(
                        &*ml_reference_array2986.borrow(),
                        self::ml_array::number_index(ml_reference_position2986),
                    );
                    ml_reference_projection2986
                };
                if match (entry.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Return(entries.clone())
                } else {
                    {
                        let source_ml_source1_temporary_order_java_script_symbol_registry_entries_ignored1161 = {
                            let ml_method_owner2987 = target.clone();
                            let ml_method_argument2987_0 =
                                entry.clone().expect("a proven present value");
                            let ml_inserted = vec![ml_method_argument2987_0];
                            let mut ml_method_values2987 = ml_method_owner2987.borrow_mut();
                            let ml_length = ml_method_values2987
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4294967295usize)
                                .expect("Invalid array length");
                            ml_method_values2987.extend(ml_inserted);
                            ml_length as f64
                        };
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
            self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Result {
    MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Done(f64, f64),
    MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Return(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    ),
}
