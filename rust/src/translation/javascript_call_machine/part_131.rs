// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCallSourceUnit;
use super::JavaScriptCallTemplate;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryProjectJavaScriptFunctionSourceTextLoop1150Result;
use super::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121;
use super::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Result;
use super::MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Result;
use super::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Result;
use super::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Result;
use super::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Result;
use super::construct_java_script_source_text;
use super::create_java_script_empty_descriptor;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_semantic_text_length;
use super::source_ml_source1_temporary_project_java_script_function_source_text_loop1150;
use super::source_ml_source1_temporary_read_java_script_constructor_frame_operands_error_loop1122;
use super::source_ml_source1_temporary_read_java_script_descriptor_operands_loop1123;
use super::source_ml_source1_temporary_read_java_script_function_source_directory_entries_error_loop1147;
use super::source_ml_source1_temporary_read_java_script_function_source_directory_entries_error_loop1148;
use super::source_ml_source1_temporary_read_java_script_function_source_directory_entries_error_loop1149;
use super::source_ml_source3_record_order;
use super::test_java_script_ordinary_class_constructor_mode;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_constructor_frame_operands_error(
    parent: self::JavaScriptCallFrame,
    frame: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
    destination: f64,
    actual_destination: f64,
    callee_register: f64,
    ordinary: bool,
    receiver_register: f64,
    argument_registers: Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,
) -> self::ml_string::Text {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            shared_constructor_field2,
            _,
            shared_constructor_field3,
            shared_constructor_field4,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => match parent.clone() {
            self::JavaScriptCallFrame::CallFrame(
                _,
                _,
                shared_constructor_field1,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (actual_destination != destination) {
                    self::ml_string::Text::from_units(vec![
                        97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 114,
                        97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                        97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100,
                        32, 115, 111, 117, 114, 99, 101, 32, 110, 101, 119, 32, 111, 112, 101, 114,
                        97, 116, 105, 111, 110,
                    ])
                } else {
                    if ((self::test_java_script_ordinary_class_constructor_mode(
                        &(match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(
                                _,
                                _,
                                source_ml_source2_data_property_value25379,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value25379.clone(),
                        }),
                    )) != ordinary)
                    {
                        self::ml_string::Text::from_units(vec![
                            97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102,
                            114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                            115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 111,
                            114, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 111, 108, 108,
                            101, 99, 116, 105, 111, 110, 32, 111, 112, 101, 114, 97, 116, 105, 111,
                            110,
                        ])
                    } else {
                        {
                            let callee = ({
                                let source_ml_source2_array_read_value25381 =
                                    shared_constructor_field1.clone();
                                {
                                    let ml_reference_array1341 =
                                        source_ml_source2_array_read_value25381.clone();
                                    let ml_reference_position1341 = callee_register;
                                    let ml_reference_projection1341 = self::ml_array::optional_at(
                                        &*ml_reference_array1341.borrow(),
                                        self::ml_array::number_index(ml_reference_position1341),
                                    );
                                    ml_reference_projection1341
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                            {
                                let receiver = if ordinary {
                                    ({
                                        let source_ml_source2_array_read_value25384 =
                                            shared_constructor_field1.clone();
                                        {
                                            let ml_reference_array1342 =
                                                source_ml_source2_array_read_value25384.clone();
                                            let ml_reference_position1342 = receiver_register;
                                            let ml_reference_projection1342 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array1342.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position1342,
                                                    ),
                                                );
                                            ml_reference_projection1342
                                        }
                                    })
                                    .clone()
                                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)
                                } else {
                                    self::JavaScriptSemanticValue::Undefined
                                };
                                if ((match callee.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => false,
                                    _ => true,
                                } || (((((match callee.clone() {
                                    self::JavaScriptSemanticValue::Object(
                                        source_ml_source2_data_property_value25390,
                                        _,
                                    ) => {
                                        Some((source_ml_source2_data_property_value25390.clone(),))
                                    }
                                    _ => None::<(self::ml::Big,)>,
                                })
                                .clone()
                                .expect("a proven present value"))
                                .0)
                                    .clone())
                                    != shared_constructor_field2))
                                    || !(self::test_java_script_semantic_same_value(
                                        receiver.clone(),
                                        ({
                                            let source_ml_source2_array_read_value25393 =
                                                shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_array_read_index25394 = 0f64;
                                                {
                                                    let ml_reference_array1343 =
                                                        source_ml_source2_array_read_value25393
                                                            .clone();
                                                    let ml_reference_position1343 =
                                                        source_ml_source2_array_read_index25394;
                                                    let ml_reference_projection1343 =
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array1343.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position1343,
                                                            ),
                                                        );
                                                    ml_reference_projection1343
                                                }
                                            }
                                        })
                                        .clone()
                                        .unwrap_or_else(
                                            || self::JavaScriptSemanticValue::Undefined,
                                        ),
                                    )))
                                {
                                    self::ml_string::Text::from_units(vec![
                                        97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 99,
                                        108, 97, 115, 115, 32, 102, 114, 97, 109, 101, 32, 114,
                                        101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99,
                                        116, 117, 97, 108, 32, 110, 101, 119, 32, 99, 97, 108, 108,
                                        101, 101, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105,
                                        118, 101, 114,
                                    ])
                                } else {
                                    {
                                        let source_ml_source2_s25398 = if !(match (
                                            argument_registers.clone(),
                                            None::<()>,
                                        ) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        }) {
                                            if (({
                                                let ml_reference_array1344 = argument_registers
                                                    .clone()
                                                    .expect("a proven present value");
                                                let ml_reference_projection1344 =
                                                    ml_reference_array1344.borrow().len() as f64;
                                                ml_reference_projection1344
                                            }) != ({
                                                let ml_reference_array1345 =
                                                    shared_constructor_field4.clone();
                                                let ml_reference_projection1345 =
                                                    ml_reference_array1345.borrow().len() as f64;
                                                ml_reference_projection1345
                                            })) {
                                                self::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 108, 105, 115, 116]))
                                            } else {
                                                {
                                                    let at = 0f64;
                                                    {
                                                        let source_ml_source2_s25401 = self::source_ml_source1_temporary_read_java_script_constructor_frame_operands_error_loop1122(parent.clone(), frame.clone(), Some(argument_registers.clone().expect("a proven present value")), at);
                                                        match source_ml_source2_s25401.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Result::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Done => {
                                                                    self::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121Next
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Result::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Return(source_ml_source1_temporary_result1122) => {
                                                                    self::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121Return(source_ml_source1_temporary_result1122.clone())
                                                                }
                                                            }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121Next
                                        };
                                        match source_ml_source2_s25398.clone() {
                                                self::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121Next => {
                                                    self::ml_string::Text::from_units(vec![])
                                                }
                                                self::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121Return(source_ml_source1_temporary_result1121) => {
                                                    source_ml_source1_temporary_result1121.clone()
                                                }
                                            }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_operands(
    operands: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
            std::rc::Rc<
                std::cell::RefCell<(
                    (
                        bool,
                        bool,
                        bool,
                        bool,
                        self::JavaScriptSemanticValue,
                        bool,
                        self::JavaScriptSemanticValue,
                        bool,
                        self::JavaScriptSemanticValue,
                        bool,
                        bool,
                        bool,
                    ),
                    Vec<f64>,
                )>,
            >,
            self::ml_string::Text,
        ),
        Vec<f64>,
    )>,
> {
    {
        let descriptor = self::create_java_script_empty_descriptor();
        if (({
            let ml_reference_array1346 = operands.clone();
            let ml_reference_projection1346 = ml_reference_array1346.borrow().len() as f64;
            ml_reference_projection1346
        }) != (12f64))
        {
            std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source3_record_field451 = descriptor.clone();
                {
                    let source_ml_source3_record_field452 =
                        self::ml_string::Text::from_units(vec![
                            97, 110, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 112, 114, 111,
                            112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116,
                            111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 115, 105, 120, 32,
                            97, 99, 116, 117, 97, 108, 32, 112, 114, 101, 115, 101, 110, 99, 101,
                            32, 97, 110, 100, 32, 112, 97, 121, 108, 111, 97, 100, 32, 112, 97,
                            105, 114, 115,
                        ]);
                    ((source_ml_source3_record_field451.clone(), source_ml_source3_record_field452.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![100f64, 101f64, 115f64, 99f64, 114f64, 105f64, 112f64, 116f64, 111f64, 114f64], vec![101f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                }
            }))
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source2_s25404 = self::source_ml_source1_temporary_read_java_script_descriptor_operands_loop1123(operands.clone(), descriptor.clone(), index);
                    match source_ml_source2_s25404.clone() {
                        self::MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Result::MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Done => {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field25406 = self::read_java_script_property_descriptor_error(descriptor.clone());
                                {
                                    let source_ml_source3_record_field453 = descriptor.clone();
                                    {
                                        let source_ml_source3_record_field454 = source_ml_source2_field25406.clone();
                                        ((source_ml_source3_record_field453.clone(), source_ml_source3_record_field454.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![100f64, 101f64, 115f64, 99f64, 114f64, 105f64, 112f64, 116f64, 111f64, 114f64], vec![101f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                                    }
                                }
                            }))
                        }
                        self::MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Result::MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Return(source_ml_source1_temporary_result1123) => {
                            source_ml_source1_temporary_result1123.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_function_source_span_error(
    sources: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>,
    span: self::JavaScriptCallSourceSpan,
) -> self::ml_string::Text {
    match span.clone() {
        self::JavaScriptCallSourceSpan::CallSourceSpan(
            _,
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            if match (span.clone(), None::<()>) {
                (_ml_left, Some(_ml_right)) => false,
                _ => false,
            } {
                self::ml_string::Text::from_units(vec![
                    102, 117, 110, 99, 116, 105, 111, 110, 32, 115, 111, 117, 114, 99, 101, 32,
                    115, 112, 97, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117,
                    97, 108, 32, 110, 111, 110, 101, 109, 112, 116, 121, 32, 85, 84, 70, 49, 54,
                    32, 105, 110, 116, 101, 114, 118, 97, 108, 115, 32, 105, 110, 32, 97, 32, 115,
                    117, 112, 112, 108, 105, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 117,
                    110, 105, 116,
                ])
            } else {
                {
                    let unit = {
                        let ml_values1347 = {
                            let ml_reference_value1348 = sources.borrow().clone();
                            ml_reference_value1348
                        };
                        let ml_callback1347 = {
                            let span = span.clone();
                            std::rc::Rc::new(move |row: self::JavaScriptCallSourceUnit| -> bool {
                                ((match row.clone() {
                                    self::JavaScriptCallSourceUnit::CallSourceUnit(
                                        source_ml_source2_data_property_value25409,
                                        _,
                                    ) => source_ml_source2_data_property_value25409.clone(),
                                }) == (match span.clone() {
                                    self::JavaScriptCallSourceSpan::CallSourceSpan(
                                        source_ml_source2_data_property_value25410,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value25410.clone(),
                                }))
                            })
                        };
                        ml_values1347.iter().cloned().enumerate().find_map(
                            |(ml_index1347, ml_value1347)| {
                                if ml_callback1347(ml_value1347.clone()) {
                                    Some(ml_value1347)
                                } else {
                                    None
                                }
                            },
                        )
                    };
                    if (((((((match span.clone() {
                        self::JavaScriptCallSourceSpan::CallSourceSpan(_, _, _) => false,
                    } || match (unit.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) || !(self::ml_math::is_safe_integer(
                        shared_constructor_field1,
                    ))) || {
                        let source_ml_source2_same_value_operand25416 = shared_constructor_field1;
                        {
                            let source_ml_source2_same_value_operand25417 = (-0f64);
                            (((source_ml_source2_same_value_operand25416
                                == source_ml_source2_same_value_operand25417)
                                && ((source_ml_source2_same_value_operand25416 != (0f64))
                                    || ((1f64 / source_ml_source2_same_value_operand25416)
                                        == (1f64 / source_ml_source2_same_value_operand25417))))
                                || ((source_ml_source2_same_value_operand25416
                                    != source_ml_source2_same_value_operand25416)
                                    && (source_ml_source2_same_value_operand25417
                                        != source_ml_source2_same_value_operand25417)))
                        }
                    }) || (shared_constructor_field1 < (0f64)))
                        || !(self::ml_math::is_safe_integer(shared_constructor_field2)))
                        || (shared_constructor_field2 <= shared_constructor_field1))
                        || (shared_constructor_field2
                            > (self::read_java_script_semantic_text_length(
                                match unit.clone().expect("a proven present value") {
                                    self::JavaScriptCallSourceUnit::CallSourceUnit(
                                        _,
                                        source_ml_source2_data_property_value25428,
                                    ) => {
                                        let source_ml_source2_data_property_value25428 =
                                            *source_ml_source2_data_property_value25428;
                                        source_ml_source2_data_property_value25428.clone()
                                    }
                                },
                            ))))
                    {
                        self::ml_string::Text::from_units(vec![
                            102, 117, 110, 99, 116, 105, 111, 110, 32, 115, 111, 117, 114, 99, 101,
                            32, 115, 112, 97, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97,
                            99, 116, 117, 97, 108, 32, 110, 111, 110, 101, 109, 112, 116, 121, 32,
                            85, 84, 70, 49, 54, 32, 105, 110, 116, 101, 114, 118, 97, 108, 115, 32,
                            105, 110, 32, 97, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 115,
                            111, 117, 114, 99, 101, 32, 117, 110, 105, 116,
                        ])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_function_source_directory_error(
    program: self::JavaScriptCallProgram,
) -> self::ml_string::Text {
    {
        let empty_sources = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
            self::JavaScriptCallSourceUnit,
        >::new()));
        {
            let retained_sources =
                match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value25429,
                    ) => ((source_ml_source2_data_property_value25429
                        .clone()
                        .unwrap_or_else(|| {
                            (None::<
                                std::rc::Rc<
                                    std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>,
                                >,
                            >,)
                        }))
                    .0)
                        .clone(),
                };
            if match (retained_sources.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                self::read_java_script_function_source_directory_entries_error(
                    program.clone(),
                    empty_sources.clone(),
                )
            } else {
                self::read_java_script_function_source_directory_entries_error(
                    program.clone(),
                    retained_sources.clone().expect("a proven present value"),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_function_source_directory_entries_error(
    program: self::JavaScriptCallProgram,
    sources: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>,
) -> self::ml_string::Text {
    if (({
        let ml_reference_array1349 = sources.clone();
        let ml_reference_projection1349 = ml_reference_array1349.borrow().len() as f64;
        ml_reference_projection1349
    }) > (4096f64))
    {
        self::ml_string::Text::from_units(vec![
            102, 117, 110, 99, 116, 105, 111, 110, 32, 115, 111, 117, 114, 99, 101, 32, 117, 110,
            105, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115,
            104, 97, 114, 101, 100, 32, 115, 116, 97, 116, 105, 99, 32, 100, 105, 114, 101, 99,
            116, 111, 114, 121, 32, 98, 117, 100, 103, 101, 116,
        ])
    } else {
        {
            let total = 0f64;
            {
                let index = 0f64;
                {
                    let source_ml_source2_s25432 = self::source_ml_source1_temporary_read_java_script_function_source_directory_entries_error_loop1147(sources.clone(), total, index);
                    match source_ml_source2_s25432.clone() {
                        self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Done => {
                            {
                                let source_ml_source1_temporary_values1144 = match program.clone() {
                                    self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value25433, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value25433.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_index1144 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s25434 = self::source_ml_source1_temporary_read_java_script_function_source_directory_entries_error_loop1148(sources.clone(), source_ml_source1_temporary_values1144.clone(), source_ml_source1_temporary_index1144.clone());
                                        match source_ml_source2_s25434.clone() {
                                            self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Done => {
                                                {
                                                    let source_ml_source1_temporary_values1145 = (match program.clone() {
                                                        self::JavaScriptCallProgram::CallProgram(_, _, _, source_ml_source2_data_property_value25435, _, _) => {
                                                            ((source_ml_source2_data_property_value25435.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,))).0).clone()
                                                        }
                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallTemplate>::new())));
                                                    {
                                                        let source_ml_source1_temporary_index1145 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s25436 = self::source_ml_source1_temporary_read_java_script_function_source_directory_entries_error_loop1149(program.clone(), sources.clone(), source_ml_source1_temporary_values1145.clone(), source_ml_source1_temporary_index1145.clone());
                                                            match source_ml_source2_s25436.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Done => {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Return(source_ml_source1_temporary_result1149) => {
                                                                    source_ml_source1_temporary_result1149.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Return(source_ml_source1_temporary_result1149_2) => {
                                                source_ml_source1_temporary_result1149_2.clone()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Return(source_ml_source1_temporary_result1149_3) => {
                            source_ml_source1_temporary_result1149_3.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn project_java_script_function_source_text(
    sources: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>,
    span: self::JavaScriptCallSourceSpan,
) -> self::JavaScriptSemanticValue {
    match span.clone() {
        self::JavaScriptCallSourceSpan::CallSourceSpan(
            _,
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let unit = {
                let ml_values1350 = {
                    let ml_reference_value1351 = sources.borrow().clone();
                    ml_reference_value1351
                };
                let ml_callback1350 = {
                    let span = span.clone();
                    std::rc::Rc::new(move |row: self::JavaScriptCallSourceUnit| -> bool {
                        ((match row.clone() {
                            self::JavaScriptCallSourceUnit::CallSourceUnit(
                                source_ml_source2_data_property_value25439,
                                _,
                            ) => source_ml_source2_data_property_value25439.clone(),
                        }) == (match span.clone() {
                            self::JavaScriptCallSourceSpan::CallSourceSpan(
                                source_ml_source2_data_property_value25440,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value25440.clone(),
                        }))
                    })
                };
                ml_values1350.iter().cloned().enumerate().find_map(
                    |(ml_index1350, ml_value1350)| {
                        if ml_callback1350(ml_value1350.clone()) {
                            Some(ml_value1350)
                        } else {
                            None
                        }
                    },
                )
            };
            if match (unit.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 106, 101, 99, 116, 105,
                        111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115,
                        32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 116, 97, 105, 110, 101, 100,
                        32, 115, 111, 117, 114, 99, 101, 32, 117, 110, 105, 116,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
            } else {
                if {
                    let source_ml_source2_s25442 =
                        match unit.clone().expect("a proven present value") {
                            self::JavaScriptCallSourceUnit::CallSourceUnit(
                                _,
                                source_ml_source2_data_property_value25445,
                            ) => {
                                let source_ml_source2_data_property_value25445 =
                                    *source_ml_source2_data_property_value25445;
                                source_ml_source2_data_property_value25445.clone()
                            }
                        };
                    match source_ml_source2_s25442.clone() {
                        self::JavaScriptSemanticValue::SourceText(_, _) => true,
                        _ => false,
                    }
                } {
                    self::construct_java_script_source_text(
                        &({
                            let source_ml_source2_string_range_operand25450 = (((match match unit
                                .clone()
                                .expect("a proven present value")
                            {
                                self::JavaScriptCallSourceUnit::CallSourceUnit(
                                    _,
                                    source_ml_source2_data_property_value25446,
                                ) => {
                                    let source_ml_source2_data_property_value25446 =
                                        *source_ml_source2_data_property_value25446;
                                    source_ml_source2_data_property_value25446.clone()
                                }
                            } {
                                self::JavaScriptSemanticValue::SourceText(
                                    _,
                                    source_ml_source2_data_property_value25447,
                                ) => Some((source_ml_source2_data_property_value25447.clone(),)),
                                _ => None::<(self::ml_string::Text,)>,
                            })
                            .clone()
                            .expect("a proven present value"))
                            .0)
                                .clone();
                            {
                                let source_ml_source2_string_range_operand25451 =
                                    shared_constructor_field1;
                                {
                                    let source_ml_source2_string_range_operand25452 =
                                        shared_constructor_field2;
                                    {
                                        let source_ml_source2_string_range_position25453 =
                                            source_ml_source2_string_range_operand25451;
                                        {
                                            let source_ml_source2_string_range_position25454 =
                                                source_ml_source2_string_range_operand25452;
                                            source_ml_source2_string_range_operand25450.slice(
                                                source_ml_source2_string_range_position25453,
                                                source_ml_source2_string_range_position25454,
                                            )
                                        }
                                    }
                                }
                            }
                        }),
                    )
                } else {
                    {
                        let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let index = shared_constructor_field1;
                            {
                                let source_ml_source2_s25456 = self::source_ml_source1_temporary_project_java_script_function_source_text_loop1150(span.clone(), Some(unit.clone().expect("a proven present value")), units.clone(), index);
                                match source_ml_source2_s25456.clone() {
                                        self::MlSource1TemporaryProjectJavaScriptFunctionSourceTextLoop1150Result::MlSource1TemporaryProjectJavaScriptFunctionSourceTextLoop1150Done => {
                                            self::JavaScriptSemanticValue::Text(units.clone())
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
