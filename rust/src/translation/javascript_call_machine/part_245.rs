// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCallSourceUnit;
use super::JavaScriptCallTemplate;
use super::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Result;
use super::find_java_script_call_code_function;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_function_source_span_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_function_source_directory_entries_error_loop1149(
    mut program: self::JavaScriptCallProgram,
    mut sources: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>,
    mut source_ml_source1_temporary_values1145: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>,
    >,
    mut source_ml_source1_temporary_index1145: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Result {
    loop {
        return if (source_ml_source1_temporary_index1145
            < ({
                let ml_reference_array2972 = source_ml_source1_temporary_values1145.clone();
                let ml_reference_projection2972 =
                    self::ml::Big::from_u128(ml_reference_array2972.borrow().len() as u128);
                ml_reference_projection2972
            })) {
            {
                let template = {
                    let ml_reference_array2973 = source_ml_source1_temporary_values1145.clone();
                    let ml_reference_position2973 = source_ml_source1_temporary_index1145.clone();
                    let ml_reference_projection2973 = self::ml_array::at(
                        &*ml_reference_array2973.borrow(),
                        ml_reference_position2973.to_index(),
                    );
                    ml_reference_projection2973
                };
                {
                    let source = match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value41579,
                        ) => ((source_ml_source2_data_property_value41579
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptCallSourceSpan>,)))
                        .0)
                            .clone(),
                    };
                    if match (source.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        {
                            let source_ml_source1_temporary_index1145_2 =
                                source_ml_source1_temporary_index1145
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1145 =
                                    source_ml_source1_temporary_index1145_2.clone();
                                continue;
                            }
                        }
                    } else {
                        {
                            let error = self::read_java_script_function_source_span_error(
                                sources.clone(),
                                source.clone().expect("a proven present value"),
                            );
                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Return(error.clone())
                            } else {
                                {
                                    let owner = {
                                        let source_ml_source2_array_read_value41582 = match program
                                            .clone()
                                        {
                                            self::JavaScriptCallProgram::CallProgram(
                                                source_ml_source2_data_property_value41580,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source2_data_property_value41580.clone(),
                                        };
                                        {
                                            let source_ml_source2_array_read_index41583 = self::find_java_script_call_code_function(program.clone(), match template.clone() {
                                                self::JavaScriptCallTemplate::CallTemplate(source_ml_source2_data_property_value41581, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value41581.clone()
                                                }
                                            });
                                            {
                                                let ml_reference_array2974 =
                                                    source_ml_source2_array_read_value41582.clone();
                                                let ml_reference_position2974 =
                                                    source_ml_source2_array_read_index41583;
                                                let ml_reference_projection2974 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array2974.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position2974,
                                                        ),
                                                    );
                                                ml_reference_projection2974
                                            }
                                        }
                                    };
                                    if (((match (
                                        {
                                            let source_ml_source2_optional_chain_receiver41584 =
                                                (owner.clone(),);
                                            match if match (source_ml_source2_optional_chain_receiver41584.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            Some({
                                                let source_ml_source1_temporary_optional_chain1146 = ((source_ml_source2_optional_chain_receiver41584.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source1_temporary_optional_chain1146.clone() {
                                                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value41585) => {
                                                        ((source_ml_source2_data_property_value41585.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSourceSpan>,))).0).clone()
                                                    }
                                                }
                                            })
                                        } else {
                                            None::<self::JavaScriptCallSourceSpan>
                                        } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                        },
                                        None::<()>,
                                    ) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } || (({
                                        let source_ml_source2_nullish_field_receiver41596 = ({
                                            let source_ml_source2_nullish_field_receiver41593 =
                                                (owner.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver41593
                                                .clone())
                                            .0
                                            {
                                                Some(ml_present) => {
                                                    let _ = ml_present.clone();
                                                    true
                                                }
                                                None => false,
                                            } {
                                                {
                                                    let source_ml_source2_nullish_field_present41594 = ((source_ml_source2_nullish_field_receiver41593.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present41594.clone() {
                                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value41595) => {
                                                            ((source_ml_source2_data_property_value41595.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSourceSpan>,))).0).clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException {
                                                    class_name: String::from("TypeError"),
                                                    message_units:
                                                        (self::ml_string::Text::from_units(vec![
                                                            67, 97, 110, 110, 111, 116, 32, 114,
                                                            101, 97, 100, 32, 112, 114, 111, 112,
                                                            101, 114, 116, 105, 101, 115, 32, 111,
                                                            102, 32, 117, 110, 100, 101, 102, 105,
                                                            110, 101, 100, 32, 40, 114, 101, 97,
                                                            100, 105, 110, 103, 32, 39, 115, 111,
                                                            117, 114, 99, 101, 39, 41,
                                                        ]))
                                                        .0,
                                                    message_present: true,
                                                })
                                                .raise()
                                            }
                                        },);
                                        if match (source_ml_source2_nullish_field_receiver41596
                                            .clone())
                                        .0
                                        {
                                            Some(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            None => false,
                                        } {
                                            {
                                                let source_ml_source2_nullish_field_present41597 = ((source_ml_source2_nullish_field_receiver41596.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source2_nullish_field_present41597.clone() {
                                                    self::JavaScriptCallSourceSpan::CallSourceSpan(source_ml_source2_data_property_value41598, _, _) => {
                                                        source_ml_source2_data_property_value41598.clone()
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException {
                                                class_name: String::from("TypeError"),
                                                message_units: (self::ml_string::Text::from_units(
                                                    vec![
                                                        67, 97, 110, 110, 111, 116, 32, 114, 101,
                                                        97, 100, 32, 112, 114, 111, 112, 101, 114,
                                                        116, 105, 101, 115, 32, 111, 102, 32, 117,
                                                        110, 100, 101, 102, 105, 110, 101, 100, 32,
                                                        40, 114, 101, 97, 100, 105, 110, 103, 32,
                                                        39, 117, 110, 105, 116, 39, 41,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        }
                                    }) != (match source
                                        .clone()
                                        .expect("a proven present value")
                                    {
                                        self::JavaScriptCallSourceSpan::CallSourceSpan(
                                            source_ml_source2_data_property_value41599,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value41599.clone(),
                                    }))) || ((match source
                                        .clone()
                                        .expect("a proven present value")
                                    {
                                        self::JavaScriptCallSourceSpan::CallSourceSpan(
                                            _,
                                            source_ml_source2_data_property_value41607,
                                            _,
                                        ) => source_ml_source2_data_property_value41607,
                                    }) < ({
                                        let source_ml_source2_nullish_field_receiver41611 = ({
                                            let source_ml_source2_nullish_field_receiver41608 =
                                                (owner.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver41608
                                                .clone())
                                            .0
                                            {
                                                Some(ml_present) => {
                                                    let _ = ml_present.clone();
                                                    true
                                                }
                                                None => false,
                                            } {
                                                {
                                                    let source_ml_source2_nullish_field_present41609 = ((source_ml_source2_nullish_field_receiver41608.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present41609.clone() {
                                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value41610) => {
                                                            ((source_ml_source2_data_property_value41610.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSourceSpan>,))).0).clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException {
                                                    class_name: String::from("TypeError"),
                                                    message_units:
                                                        (self::ml_string::Text::from_units(vec![
                                                            67, 97, 110, 110, 111, 116, 32, 114,
                                                            101, 97, 100, 32, 112, 114, 111, 112,
                                                            101, 114, 116, 105, 101, 115, 32, 111,
                                                            102, 32, 117, 110, 100, 101, 102, 105,
                                                            110, 101, 100, 32, 40, 114, 101, 97,
                                                            100, 105, 110, 103, 32, 39, 115, 111,
                                                            117, 114, 99, 101, 39, 41,
                                                        ]))
                                                        .0,
                                                    message_present: true,
                                                })
                                                .raise()
                                            }
                                        },);
                                        if match (source_ml_source2_nullish_field_receiver41611
                                            .clone())
                                        .0
                                        {
                                            Some(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            None => false,
                                        } {
                                            {
                                                let source_ml_source2_nullish_field_present41612 = ((source_ml_source2_nullish_field_receiver41611.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source2_nullish_field_present41612.clone() {
                                                    self::JavaScriptCallSourceSpan::CallSourceSpan(_, source_ml_source2_data_property_value41613, _) => {
                                                        source_ml_source2_data_property_value41613
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException {
                                                class_name: String::from("TypeError"),
                                                message_units: (self::ml_string::Text::from_units(
                                                    vec![
                                                        67, 97, 110, 110, 111, 116, 32, 114, 101,
                                                        97, 100, 32, 112, 114, 111, 112, 101, 114,
                                                        116, 105, 101, 115, 32, 111, 102, 32, 117,
                                                        110, 100, 101, 102, 105, 110, 101, 100, 32,
                                                        40, 114, 101, 97, 100, 105, 110, 103, 32,
                                                        39, 115, 116, 97, 114, 116, 39, 41,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        }
                                    }))) || ((match source
                                        .clone()
                                        .expect("a proven present value")
                                    {
                                        self::JavaScriptCallSourceSpan::CallSourceSpan(
                                            _,
                                            _,
                                            source_ml_source2_data_property_value41621,
                                        ) => source_ml_source2_data_property_value41621,
                                    }) > ({
                                        let source_ml_source2_nullish_field_receiver41625 = ({
                                            let source_ml_source2_nullish_field_receiver41622 =
                                                (owner.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver41622
                                                .clone())
                                            .0
                                            {
                                                Some(ml_present) => {
                                                    let _ = ml_present.clone();
                                                    true
                                                }
                                                None => false,
                                            } {
                                                {
                                                    let source_ml_source2_nullish_field_present41623 = ((source_ml_source2_nullish_field_receiver41622.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present41623.clone() {
                                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value41624) => {
                                                            ((source_ml_source2_data_property_value41624.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSourceSpan>,))).0).clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException {
                                                    class_name: String::from("TypeError"),
                                                    message_units:
                                                        (self::ml_string::Text::from_units(vec![
                                                            67, 97, 110, 110, 111, 116, 32, 114,
                                                            101, 97, 100, 32, 112, 114, 111, 112,
                                                            101, 114, 116, 105, 101, 115, 32, 111,
                                                            102, 32, 117, 110, 100, 101, 102, 105,
                                                            110, 101, 100, 32, 40, 114, 101, 97,
                                                            100, 105, 110, 103, 32, 39, 115, 111,
                                                            117, 114, 99, 101, 39, 41,
                                                        ]))
                                                        .0,
                                                    message_present: true,
                                                })
                                                .raise()
                                            }
                                        },);
                                        if match (source_ml_source2_nullish_field_receiver41625
                                            .clone())
                                        .0
                                        {
                                            Some(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            None => false,
                                        } {
                                            {
                                                let source_ml_source2_nullish_field_present41626 = ((source_ml_source2_nullish_field_receiver41625.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source2_nullish_field_present41626.clone() {
                                                    self::JavaScriptCallSourceSpan::CallSourceSpan(_, _, source_ml_source2_data_property_value41627) => {
                                                        source_ml_source2_data_property_value41627
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException {
                                                class_name: String::from("TypeError"),
                                                message_units: (self::ml_string::Text::from_units(
                                                    vec![
                                                        67, 97, 110, 110, 111, 116, 32, 114, 101,
                                                        97, 100, 32, 112, 114, 111, 112, 101, 114,
                                                        116, 105, 101, 115, 32, 111, 102, 32, 117,
                                                        110, 100, 101, 102, 105, 110, 101, 100, 32,
                                                        40, 114, 101, 97, 100, 105, 110, 103, 32,
                                                        39, 101, 110, 100, 39, 41,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        }
                                    }))) {
                                        self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Return(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 115, 105, 116, 101, 115, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 115, 105, 100, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                    } else {
                                        {
                                            let source_ml_source1_temporary_index1145_3 =
                                                source_ml_source1_temporary_index1145
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_source1_temporary_index1145 =
                                                    source_ml_source1_temporary_index1145_3.clone();
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Result {
    MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Done,
    MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_function_source_directory_entries_error_loop1148(
    mut sources: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>,
    mut source_ml_source1_temporary_values1144: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallFunction>>,
    >,
    mut source_ml_source1_temporary_index1144: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Result {
    loop {
        return if (source_ml_source1_temporary_index1144
            < ({
                let ml_reference_array2975 = source_ml_source1_temporary_values1144.clone();
                let ml_reference_projection2975 =
                    self::ml::Big::from_u128(ml_reference_array2975.borrow().len() as u128);
                ml_reference_projection2975
            })) {
            {
                let fn_ = {
                    let ml_reference_array2976 = source_ml_source1_temporary_values1144.clone();
                    let ml_reference_position2976 = source_ml_source1_temporary_index1144.clone();
                    let ml_reference_projection2976 = self::ml_array::at(
                        &*ml_reference_array2976.borrow(),
                        ml_reference_position2976.to_index(),
                    );
                    ml_reference_projection2976
                };
                {
                    let source = match fn_.clone() {
                        self::JavaScriptCallFunction::CallFunction(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value41630,
                        ) => ((source_ml_source2_data_property_value41630
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptCallSourceSpan>,)))
                        .0)
                            .clone(),
                    };
                    if match (source.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        {
                            let source_ml_source1_temporary_index1144_2 =
                                source_ml_source1_temporary_index1144
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1144 =
                                    source_ml_source1_temporary_index1144_2.clone();
                                continue;
                            }
                        }
                    } else {
                        {
                            let error = self::read_java_script_function_source_span_error(
                                sources.clone(),
                                source.clone().expect("a proven present value"),
                            );
                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Return(error.clone())
                            } else {
                                if !(self::ml_array::includes(
                                    &{
                                        let ml_reference_value2977 =
                                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                self::ml_string::Text::from_units(vec![
                                                    115, 116, 114, 105, 99, 116,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99,
                                                    108, 97, 115, 115,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    100, 101, 114, 105, 118, 101, 100, 32, 69, 114,
                                                    114, 111, 114,
                                                ]),
                                            ])))
                                            .borrow()
                                            .clone();
                                        ml_reference_value2977
                                    },
                                    &match fn_.clone() {
                                        self::JavaScriptCallFunction::CallFunction(
                                            _,
                                            _,
                                            source_ml_source2_data_property_value41631,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value41631.clone(),
                                    },
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                )) {
                                    self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Return(self::ml_string::Text::from_units(vec![112, 111, 114, 116, 97, 98, 108, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 98, 111, 100, 105, 101, 115, 32, 100, 111, 32, 110, 111, 116, 32, 101, 115, 116, 97, 98, 108, 105, 115, 104, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116]))
                                } else {
                                    {
                                        let source_ml_source1_temporary_index1144_3 =
                                            source_ml_source1_temporary_index1144
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index1144 =
                                                source_ml_source1_temporary_index1144_3.clone();
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
            self::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Result::MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1148Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Result {
    MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Done,
    MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1147Return(
        self::ml_string::Text,
    ),
}
