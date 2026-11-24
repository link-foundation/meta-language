// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptFunctionBindCall;
use super::JavaScriptFunctionBindState;
use super::JavaScriptFunctionBindTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryRetainJavaScriptFunctionBindMetadataLoop286Result;
use super::create_java_script_property_descriptor;
use super::ml_string;
use super::read_java_script_bound_function_length;
use super::read_java_script_semantic_text_length;
use super::source_ml_source1_temporary_retain_java_script_function_bind_metadata_loop286;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn retain_java_script_function_bind_metadata(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptFunctionBindState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptFunctionBindTransition {
    match state.clone() {
        self::JavaScriptFunctionBindState::FunctionBindState(
            _,
            _,
            shared_constructor_field10,
            shared_constructor_field11,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let previous = match state.clone() {
                self::JavaScriptFunctionBindState::FunctionBindState(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value7036,
                ) => ((source_ml_source2_data_property_value7036
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptFunctionBindCall>,)))
                .0)
                    .clone(),
            };
            {
                let base = match state.clone() {
                    self::JavaScriptFunctionBindState::FunctionBindState(
                        source_ml_source2_nominal_rest_field7037,
                        source_ml_source2_nominal_rest_field7038,
                        source_ml_source2_nominal_rest_field7039,
                        source_ml_source2_nominal_rest_field7040,
                        source_ml_source2_nominal_rest_field7041,
                        source_ml_source2_nominal_rest_field7042,
                        source_ml_source2_nominal_rest_field7043,
                        source_ml_source2_nominal_rest_field7044,
                        _,
                    ) => {
                        let source_ml_source2_nominal_rest_field7037 =
                            *source_ml_source2_nominal_rest_field7037;
                        let source_ml_source2_nominal_rest_field7038 =
                            *source_ml_source2_nominal_rest_field7038;
                        let source_ml_source2_nominal_rest_field7041 =
                            *source_ml_source2_nominal_rest_field7041;
                        let source_ml_source2_nominal_rest_field7042 =
                            *source_ml_source2_nominal_rest_field7042;
                        let source_ml_source2_nominal_rest_field7044 =
                            *source_ml_source2_nominal_rest_field7044;
                        self::JavaScriptFunctionBindState::FunctionBindState(
                            Box::new(source_ml_source2_nominal_rest_field7037.clone()),
                            Box::new(source_ml_source2_nominal_rest_field7038.clone()),
                            source_ml_source2_nominal_rest_field7039.clone(),
                            source_ml_source2_nominal_rest_field7040.clone(),
                            Box::new(source_ml_source2_nominal_rest_field7041.clone()),
                            Box::new(source_ml_source2_nominal_rest_field7042.clone()),
                            source_ml_source2_nominal_rest_field7043,
                            Box::new(source_ml_source2_nominal_rest_field7044.clone()),
                            None::<(Option<self::JavaScriptFunctionBindCall>,)>,
                        )
                    }
                };
                match base.clone() {
                    self::JavaScriptFunctionBindState::FunctionBindState(
                        shared_constructor_field1,
                        shared_constructor_field2,
                        shared_constructor_field3,
                        shared_constructor_field4,
                        shared_constructor_field5,
                        shared_constructor_field6,
                        shared_constructor_field7,
                        shared_constructor_field8,
                        shared_constructor_field9,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        let shared_constructor_field2 = *shared_constructor_field2;
                        let shared_constructor_field5 = *shared_constructor_field5;
                        let shared_constructor_field6 = *shared_constructor_field6;
                        let shared_constructor_field8 = *shared_constructor_field8;
                        if ((shared_constructor_field11
                            == (self::ml_string::Text::from_units(vec![
                                108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114,
                            ])))
                            || (shared_constructor_field11
                                == (self::ml_string::Text::from_units(vec![
                                    108, 101, 110, 103, 116, 104,
                                ]))))
                        {
                            self::JavaScriptFunctionBindTransition::FunctionBindContinue(
                                Box::new(heap.clone()),
                                Box::new({
                                    let source_ml_source2_nominal_field7051 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field7053 =
                                            shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field7055 =
                                                shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field7057 =
                                                    shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field7059 =
                                                        shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field7061 =
                                                            shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field7063 =
                                                                shared_constructor_field7;
                                                            {
                                                                let source_ml_source2_nominal_field7065 =
                                                                    shared_constructor_field8
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field7067 =
                                                                        shared_constructor_field9
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_override7070 = self::read_java_script_bound_function_length(value.clone(), { let ml_reference_array552 = shared_constructor_field10.clone(); let ml_reference_projection552 = ml_reference_array552.borrow().len() as f64; ml_reference_projection552 });
                                                                        self::JavaScriptFunctionBindState::FunctionBindState(Box::new(source_ml_source2_nominal_field7051.clone()), Box::new(source_ml_source2_nominal_field7053.clone()), source_ml_source2_nominal_field7055.clone(), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101, 32, 108, 101, 110, 103, 116, 104]), Box::new(source_ml_source2_nominal_field7059.clone()), Box::new(source_ml_source2_nominal_field7061.clone()), source_ml_source2_nominal_override7070, Box::new(source_ml_source2_nominal_field7065.clone()), source_ml_source2_nominal_field7067.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }),
                            )
                        } else {
                            {
                                let size =
                                    self::read_java_script_semantic_text_length(value.clone());
                                if (size > (65530f64)) {
                                    self::JavaScriptFunctionBindTransition::FunctionBindResourceLimit(Box::new(heap.clone()), Box::new({
                                            let source_ml_source2_nominal_field7072 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field7074 = shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_field7076 = shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field7078 = shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field7080 = shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field7082 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field7084 = shared_constructor_field7;
                                                                    {
                                                                        let source_ml_source2_nominal_field7086 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field7088 = shared_constructor_field9.clone();
                                                                            self::JavaScriptFunctionBindState::FunctionBindState(Box::new(source_ml_source2_nominal_field7072.clone()), Box::new(source_ml_source2_nominal_field7074.clone()), source_ml_source2_nominal_field7076.clone(), self::ml_string::Text::from_units(vec![110, 97, 109, 101, 32, 118, 97, 108, 117, 101]), Box::new(source_ml_source2_nominal_field7080.clone()), Box::new(source_ml_source2_nominal_field7082.clone()), source_ml_source2_nominal_field7084, Box::new(value.clone()), source_ml_source2_nominal_field7088.clone())
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                } else {
                                    {
                                        let units =
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                98f64, 111f64, 117f64, 110f64, 100f64, 32f64,
                                            ]));
                                        {
                                            let index = 0f64;
                                            {
                                                let source_ml_source2_s7092 = self::source_ml_source1_temporary_retain_java_script_function_bind_metadata_loop286(value.clone(), size, units.clone(), index);
                                                match source_ml_source2_s7092.clone() {
                                                        self::MlSource1TemporaryRetainJavaScriptFunctionBindMetadataLoop286Result::MlSource1TemporaryRetainJavaScriptFunctionBindMetadataLoop286Done => {
                                                            self::JavaScriptFunctionBindTransition::FunctionBindContinue(Box::new(heap.clone()), Box::new({
                                                                let source_ml_source2_nominal_field7094 = shared_constructor_field1.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field7096 = shared_constructor_field2.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field7098 = shared_constructor_field3.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field7100 = shared_constructor_field4.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field7102 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field7104 = shared_constructor_field6.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field7106 = shared_constructor_field7;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field7108 = shared_constructor_field8.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field7110 = shared_constructor_field9.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override7113 = self::JavaScriptSemanticValue::Text(units.clone());
                                                                                                    self::JavaScriptFunctionBindState::FunctionBindState(Box::new(source_ml_source2_nominal_field7094.clone()), Box::new(source_ml_source2_nominal_field7096.clone()), source_ml_source2_nominal_field7098.clone(), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101, 32, 110, 97, 109, 101]), Box::new(source_ml_source2_nominal_field7102.clone()), Box::new(source_ml_source2_nominal_field7104.clone()), source_ml_source2_nominal_field7106, Box::new(source_ml_source2_nominal_override7113.clone()), source_ml_source2_nominal_field7110.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }))
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
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_bound_function_metadata_descriptor(
    value: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
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
> {
    {
        let descriptor = self::create_java_script_property_descriptor(
            &(self::ml_string::Text::from_units(vec![100, 97, 116, 97])),
            true,
            false,
        );
        {
            let source_ml_source1_temporary_create_java_script_bound_function_metadata_descriptor_written288 = {
                let ml_field_owner553 = descriptor.clone();
                let ml_field_value553 = value.clone();
                ml_field_owner553.borrow_mut().0.8 = ml_field_value553;
                ()
            };
            {
                let source_ml_source1_temporary_create_java_script_bound_function_metadata_descriptor_written289 = {
                    let ml_field_owner554 = descriptor.clone();
                    let ml_field_value554 = false;
                    ml_field_owner554.borrow_mut().0.10 = ml_field_value554;
                    ()
                };
                descriptor.clone()
            }
        }
    }
}
