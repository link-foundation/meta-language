// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptFunctionBindCall;
use super::JavaScriptFunctionBindState;
use super::JavaScriptFunctionBindTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::create_java_script_bound_function_metadata_descriptor;
use super::define_java_script_ordinary_property;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_bound_function_metadata_key;
use super::read_java_script_function_bind_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_semantic_text_length;
use super::retain_java_script_function_bind_metadata;
use super::source_ml_source1_temporary_resume_java_script_function_bind_allocation_loop291;
use super::test_java_script_reflection_property_budget;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_function_bind(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptFunctionBindState,
) -> self::JavaScriptFunctionBindTransition {
    match state.clone() {
        self::JavaScriptFunctionBindState::FunctionBindState(
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
        ) => {
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field12 = *shared_constructor_field12;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field4,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error = self::read_java_script_function_bind_state_error(
                        heap.clone(),
                        state.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptFunctionBindTransition::FunctionBindInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    } else {
                        if !(match (
                            match state.clone() {
                                self::JavaScriptFunctionBindState::FunctionBindState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value7114,
                                ) => ((source_ml_source2_data_property_value7114
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptFunctionBindCall>,)
                                    }))
                                .0)
                                    .clone(),
                            },
                            None::<()>,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }) {
                            self::JavaScriptFunctionBindTransition::FunctionBindCall(
                                Box::new(heap.clone()),
                                Box::new(state.clone()),
                                Box::new(
                                    match (match state.clone() {
                                        self::JavaScriptFunctionBindState::FunctionBindState(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value7115,
                                        ) => ((source_ml_source2_data_property_value7115
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptFunctionBindCall>,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .expect("a proven present value")
                                    {
                                        self::JavaScriptFunctionBindCall::FunctionBindCall(
                                            source_ml_source2_data_property_value7116,
                                            _,
                                            _,
                                        ) => {
                                            let source_ml_source2_data_property_value7116 =
                                                *source_ml_source2_data_property_value7116;
                                            source_ml_source2_data_property_value7116.clone()
                                        }
                                    },
                                ),
                                Box::new(
                                    match (match state.clone() {
                                        self::JavaScriptFunctionBindState::FunctionBindState(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value7117,
                                        ) => ((source_ml_source2_data_property_value7117
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptFunctionBindCall>,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .expect("a proven present value")
                                    {
                                        self::JavaScriptFunctionBindCall::FunctionBindCall(
                                            _,
                                            source_ml_source2_data_property_value7118,
                                            _,
                                        ) => {
                                            let source_ml_source2_data_property_value7118 =
                                                *source_ml_source2_data_property_value7118;
                                            source_ml_source2_data_property_value7118.clone()
                                        }
                                    },
                                ),
                                match (match state.clone() {
                                    self::JavaScriptFunctionBindState::FunctionBindState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value7119,
                                    ) => ((source_ml_source2_data_property_value7119
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptFunctionBindCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptFunctionBindCall::FunctionBindCall(
                                        _,
                                        _,
                                        source_ml_source2_data_property_value7120,
                                    ) => source_ml_source2_data_property_value7120.clone(),
                                },
                            )
                        } else {
                            if (shared_constructor_field8
                                == (self::ml_string::Text::from_units(vec![
                                    110, 97, 109, 101, 32, 118, 97, 108, 117, 101,
                                ])))
                            {
                                self::retain_java_script_function_bind_metadata(
                                    heap.clone(),
                                    state.clone(),
                                    shared_constructor_field12.clone(),
                                )
                            } else {
                                if {
                                    let source_ml_source2_s7125 = shared_constructor_field5.clone();
                                    match source_ml_source2_s7125.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => false,
                                        _ => true,
                                    }
                                } {
                                    self::JavaScriptFunctionBindTransition::FunctionBindInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 116,
                                            97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                            117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101,
                                            32, 116, 97, 114, 103, 101, 116,
                                        ]),
                                    )
                                } else {
                                    if (shared_constructor_field8
                                        == (self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 116, 111, 116, 121, 112, 101,
                                        ])))
                                    {
                                        {
                                            let target = {
                                                let source_ml_source2_array_read_value7135 =
                                                    shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_array_read_index7136 = self::find_java_script_ordinary_object(shared_constructor_field4.clone(), (((match shared_constructor_field5.clone() {
                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value7134, _) => {
                                                                Some((source_ml_source2_data_property_value7134.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml::Big,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone());
                                                    {
                                                        let ml_reference_array555 =
                                                            source_ml_source2_array_read_value7135
                                                                .clone();
                                                        let ml_reference_position555 =
                                                            source_ml_source2_array_read_index7136;
                                                        let ml_reference_projection555 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array555.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position555,
                                                                ),
                                                            );
                                                        ml_reference_projection555
                                                    }
                                                }
                                            };
                                            if match (target.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::JavaScriptFunctionBindTransition::FunctionBindInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 115, 101, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                            } else {
                                                self::JavaScriptFunctionBindTransition::FunctionBindContinue(Box::new(heap.clone()), Box::new({
                                                        let source_ml_source2_nominal_field7151 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field7153 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field7155 = shared_constructor_field7.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field7157 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field7159 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field7161 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field7163 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field7165 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field7167 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override7170 = match target.clone().expect("a proven present value") {
                                                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value7137, _, _, _, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7137 = *source_ml_source2_data_property_value7137;
                                                                                                    source_ml_source2_data_property_value7137.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value7138, _, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7138 = *source_ml_source2_data_property_value7138;
                                                                                                    source_ml_source2_data_property_value7138.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value7139, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7139 = *source_ml_source2_data_property_value7139;
                                                                                                    source_ml_source2_data_property_value7139.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value7140, _, _, _, _, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7140 = *source_ml_source2_data_property_value7140;
                                                                                                    source_ml_source2_data_property_value7140.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value7141, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7141 = *source_ml_source2_data_property_value7141;
                                                                                                    source_ml_source2_data_property_value7141.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value7142, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7142 = *source_ml_source2_data_property_value7142;
                                                                                                    source_ml_source2_data_property_value7142.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value7143, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7143 = *source_ml_source2_data_property_value7143;
                                                                                                    source_ml_source2_data_property_value7143.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value7144, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7144 = *source_ml_source2_data_property_value7144;
                                                                                                    source_ml_source2_data_property_value7144.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value7145, _, _, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7145 = *source_ml_source2_data_property_value7145;
                                                                                                    source_ml_source2_data_property_value7145.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value7146, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7146 = *source_ml_source2_data_property_value7146;
                                                                                                    source_ml_source2_data_property_value7146.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source2_data_property_value7147, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7147 = *source_ml_source2_data_property_value7147;
                                                                                                    source_ml_source2_data_property_value7147.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source2_data_property_value7148, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7148 = *source_ml_source2_data_property_value7148;
                                                                                                    source_ml_source2_data_property_value7148.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source2_data_property_value7149, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value7149 = *source_ml_source2_data_property_value7149;
                                                                                                    source_ml_source2_data_property_value7149.clone()
                                                                                                }
                                                                                            };
                                                                                            self::JavaScriptFunctionBindState::FunctionBindState(Box::new(source_ml_source2_nominal_field7151.clone()), Box::new(source_ml_source2_nominal_field7153.clone()), source_ml_source2_nominal_field7155.clone(), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), Box::new(source_ml_source2_nominal_override7170.clone()), Box::new(source_ml_source2_nominal_field7161.clone()), source_ml_source2_nominal_field7163, Box::new(source_ml_source2_nominal_field7165.clone()), source_ml_source2_nominal_field7167.clone())
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
                                    } else {
                                        if (shared_constructor_field8
                                            == (self::ml_string::Text::from_units(vec![
                                                97, 108, 108, 111, 99, 97, 116, 101,
                                            ])))
                                        {
                                            if (({
                                                let ml_reference_array556 =
                                                    shared_constructor_field4.clone();
                                                let ml_reference_projection556 =
                                                    ml_reference_array556.borrow().len() as f64;
                                                ml_reference_projection556
                                            }) >= (4096f64))
                                            {
                                                self::JavaScriptFunctionBindTransition::FunctionBindResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 98, 117, 100, 103, 101, 116]))
                                            } else {
                                                self::JavaScriptFunctionBindTransition::FunctionBindAllocate(Box::new(heap.clone()), Box::new(state.clone()), Box::new(shared_constructor_field9.clone()))
                                            }
                                        } else {
                                            if (shared_constructor_field8
                                                == (self::ml_string::Text::from_units(vec![
                                                    108, 101, 110, 103, 116, 104, 32, 111, 119, 110,
                                                ])))
                                            {
                                                {
                                                    let target_2 = {
                                                        let source_ml_source2_array_read_value7182 =
                                                            shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_array_read_index7183 = self::find_java_script_ordinary_object(shared_constructor_field4.clone(), (((match shared_constructor_field5.clone() {
                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value7181, _) => {
                                                                        Some((source_ml_source2_data_property_value7181.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::ml::Big,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone());
                                                            {
                                                                let ml_reference_array557 = source_ml_source2_array_read_value7182.clone();
                                                                let ml_reference_position557 = source_ml_source2_array_read_index7183;
                                                                let ml_reference_projection557 = self::ml_array::optional_at(&*ml_reference_array557.borrow(), self::ml_array::number_index(ml_reference_position557));
                                                                ml_reference_projection557
                                                            }
                                                        }
                                                    };
                                                    if match (target_2.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    } {
                                                        self::JavaScriptFunctionBindTransition::FunctionBindInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 111, 119, 110, 45, 112, 114, 111, 112, 101, 114, 116, 121, 32, 105, 110, 118, 101, 110, 116, 111, 114, 121]))
                                                    } else {
                                                        self::JavaScriptFunctionBindTransition::FunctionBindContinue(Box::new(heap.clone()), Box::new({
                                                                let source_ml_source2_nominal_field7211 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field7213 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field7215 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field7217 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field7219 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field7221 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field7223 = shared_constructor_field11;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field7225 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field7227 = shared_constructor_field13.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override7229 = if ((self::find_java_script_ordinary_property(match target_2.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value7197, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value7197.clone()
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value7198, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value7198.clone()
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value7199) => {
                                                                                                            source_ml_source2_data_property_value7199.clone()
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value7200, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value7200.clone()
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value7201, _) => {
                                                                                                            source_ml_source2_data_property_value7201.clone()
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value7202, _) => {
                                                                                                            source_ml_source2_data_property_value7202.clone()
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value7203, _) => {
                                                                                                            source_ml_source2_data_property_value7203.clone()
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value7204, _) => {
                                                                                                            source_ml_source2_data_property_value7204.clone()
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value7205) => {
                                                                                                            source_ml_source2_data_property_value7205.clone()
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value7206, _, _) => {
                                                                                                            source_ml_source2_data_property_value7206.clone()
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value7207, _, _) => {
                                                                                                            source_ml_source2_data_property_value7207.clone()
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value7208, _, _) => {
                                                                                                            source_ml_source2_data_property_value7208.clone()
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value7209, _, _) => {
                                                                                                            source_ml_source2_data_property_value7209.clone()
                                                                                                        }
                                                                                                    }, self::read_java_script_bound_function_metadata_key(true))) < (0f64)) {
                                                                                                        self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101, 32, 108, 101, 110, 103, 116, 104])
                                                                                                    } else {
                                                                                                        self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])
                                                                                                    };
                                                                                                    self::JavaScriptFunctionBindState::FunctionBindState(Box::new(source_ml_source2_nominal_field7211.clone()), Box::new(source_ml_source2_nominal_field7213.clone()), source_ml_source2_nominal_field7215.clone(), source_ml_source2_nominal_override7229.clone(), Box::new(source_ml_source2_nominal_field7219.clone()), Box::new(source_ml_source2_nominal_field7221.clone()), source_ml_source2_nominal_field7223, Box::new(source_ml_source2_nominal_field7225.clone()), source_ml_source2_nominal_field7227.clone())
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
                                            } else {
                                                if ((shared_constructor_field8
                                                    == (self::ml_string::Text::from_units(vec![
                                                        108, 101, 110, 103, 116, 104,
                                                    ])))
                                                    || (shared_constructor_field8
                                                        == (self::ml_string::Text::from_units(
                                                            vec![110, 97, 109, 101],
                                                        ))))
                                                {
                                                    {
                                                        let get = self::plan_java_script_ordinary_get(heap.clone(), (((match shared_constructor_field5.clone() {
                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value7235, _) => {
                                                                    Some((source_ml_source2_data_property_value7235.clone(),))
                                                                }
                                                                _ => {
                                                                    None::<(self::ml::Big,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0).clone(), self::read_java_script_bound_function_metadata_key((shared_constructor_field8 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))), shared_constructor_field5.clone());
                                                        if (((({
                                                            let ml_reference_value558 =
                                                                get.borrow().clone();
                                                            ml_reference_value558
                                                        })
                                                        .0)
                                                            .3)
                                                            != (self::ml_string::Text::from_units(
                                                                vec![],
                                                            )))
                                                        {
                                                            self::JavaScriptFunctionBindTransition::FunctionBindInvalid(Box::new(heap.clone()), (({ let ml_reference_value559 = get.borrow().clone(); ml_reference_value559 }).0).3)
                                                        } else {
                                                            if (((({ let ml_reference_value560 = get.borrow().clone(); ml_reference_value560 }).0).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                    self::retain_java_script_function_bind_metadata(heap.clone(), state.clone(), (({ let ml_reference_value561 = get.borrow().clone(); ml_reference_value561 }).0).5)
                                                                } else {
                                                                    if (((({ let ml_reference_value562 = get.borrow().clone(); ml_reference_value562 }).0).0) != (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                        self::JavaScriptFunctionBindTransition::FunctionBindInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 103, 101, 116, 116, 101, 114]))
                                                                    } else {
                                                                        {
                                                                            let call = self::JavaScriptFunctionBindCall::FunctionBindCall(Box::new((({ let ml_reference_value563 = get.borrow().clone(); ml_reference_value563 }).0).2), Box::new((({ let ml_reference_value564 = get.borrow().clone(); ml_reference_value564 }).0).4), (({ let ml_reference_value565 = get.borrow().clone(); ml_reference_value565 }).0).1);
                                                                            match call.clone() {
                                                                                self::JavaScriptFunctionBindCall::FunctionBindCall(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                                    let shared_constructor_field1 = *shared_constructor_field1;
                                                                                    let shared_constructor_field2 = *shared_constructor_field2;
                                                                                    self::JavaScriptFunctionBindTransition::FunctionBindCall(Box::new(heap.clone()), Box::new({
                                                                                        let source_ml_source2_nominal_field7242 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field7244 = shared_constructor_field6.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field7246 = shared_constructor_field7.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field7248 = shared_constructor_field8.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field7250 = shared_constructor_field9.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field7252 = shared_constructor_field10.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field7254 = shared_constructor_field11;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field7256 = shared_constructor_field12.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field7258 = shared_constructor_field13.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override7260 = if (shared_constructor_field8 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]))) {
                                                                                                                                self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114])
                                                                                                                            } else {
                                                                                                                                self::ml_string::Text::from_units(vec![110, 97, 109, 101, 32, 103, 101, 116, 116, 101, 114])
                                                                                                                            };
                                                                                                                            self::JavaScriptFunctionBindState::FunctionBindState(Box::new(source_ml_source2_nominal_field7242.clone()), Box::new(source_ml_source2_nominal_field7244.clone()), source_ml_source2_nominal_field7246.clone(), source_ml_source2_nominal_override7260.clone(), Box::new(source_ml_source2_nominal_field7250.clone()), Box::new(source_ml_source2_nominal_field7252.clone()), source_ml_source2_nominal_field7254, Box::new(source_ml_source2_nominal_field7256.clone()), Some((Some(call.clone()),)))
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }), Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone())
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                        }
                                                    }
                                                } else {
                                                    if {
                                                        let source_ml_source2_s7266 =
                                                            shared_constructor_field10.clone();
                                                        match source_ml_source2_s7266.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }
                                                    } {
                                                        self::JavaScriptFunctionBindTransition::FunctionBindInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110]))
                                                    } else {
                                                        if ((shared_constructor_field8 == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101, 32, 110, 97, 109, 101]))) && ((self::read_java_script_semantic_text_length(shared_constructor_field12.clone())) > (65536f64))) {
                                                                self::JavaScriptFunctionBindTransition::FunctionBindResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                            } else {
                                                                if !(self::test_java_script_reflection_property_budget(heap.clone(), shared_constructor_field10.clone(), self::read_java_script_bound_function_metadata_key((shared_constructor_field8 == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101, 32, 108, 101, 110, 103, 116, 104])))))) {
                                                                    self::JavaScriptFunctionBindTransition::FunctionBindResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                                } else {
                                                                    {
                                                                        let length = (shared_constructor_field8 == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101, 32, 108, 101, 110, 103, 116, 104])));
                                                                        {
                                                                            let descriptor = self::create_java_script_bound_function_metadata_descriptor(if length {
                                                                                self::JavaScriptSemanticValue::Number(shared_constructor_field11)
                                                                            } else {
                                                                                shared_constructor_field12.clone()
                                                                            });
                                                                            {
                                                                                let defined = self::define_java_script_ordinary_property(heap.clone(), (((match shared_constructor_field10.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value7282, _) => {
                                                                                        Some((source_ml_source2_data_property_value7282.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(self::ml::Big,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone(), self::read_java_script_bound_function_metadata_key(length), descriptor.clone());
                                                                                if ((((({ let ml_reference_value566 = defined.borrow().clone(); ml_reference_value566 }).0).0) != (self::ml_string::Text::from_units(vec![]))) || !((({ let ml_reference_value567 = defined.borrow().clone(); ml_reference_value567 }).0).3)) {
                                                                                    self::JavaScriptFunctionBindTransition::FunctionBindInvalid(Box::new(heap.clone()), {
                                                                                        let source_ml_source2_logical7283 = (({ let ml_reference_value568 = defined.borrow().clone(); ml_reference_value568 }).0).0;
                                                                                        if !source_ml_source2_logical7283.0.is_empty() {
                                                                                            source_ml_source2_logical7283.clone()
                                                                                        } else {
                                                                                            self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104, 32, 98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 115, 117, 99, 99, 101, 101, 100, 115])
                                                                                        }
                                                                                    })
                                                                                } else {
                                                                                    if length {
                                                                                        self::JavaScriptFunctionBindTransition::FunctionBindContinue(Box::new((({ let ml_reference_value569 = defined.borrow().clone(); ml_reference_value569 }).0).1), Box::new({
                                                                                            let source_ml_source2_nominal_field7285 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field7287 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field7289 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field7291 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field7293 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field7295 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field7297 = shared_constructor_field11;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field7299 = shared_constructor_field12.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field7301 = shared_constructor_field13.clone();
                                                                                                                            self::JavaScriptFunctionBindState::FunctionBindState(Box::new(source_ml_source2_nominal_field7285.clone()), Box::new(source_ml_source2_nominal_field7287.clone()), source_ml_source2_nominal_field7289.clone(), self::ml_string::Text::from_units(vec![110, 97, 109, 101]), Box::new(source_ml_source2_nominal_field7293.clone()), Box::new(source_ml_source2_nominal_field7295.clone()), source_ml_source2_nominal_field7297, Box::new(source_ml_source2_nominal_field7299.clone()), source_ml_source2_nominal_field7301.clone())
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }))
                                                                                    } else {
                                                                                        self::JavaScriptFunctionBindTransition::FunctionBindCompleted(Box::new((({ let ml_reference_value570 = defined.borrow().clone(); ml_reference_value570 }).0).1), Box::new(shared_constructor_field10.clone()))
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
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_bound_function_object(
    state: self::JavaScriptFunctionBindState,
    identity: self::ml::Big,
) -> self::JavaScriptOrdinaryObject {
    match state.clone() {
        self::JavaScriptFunctionBindState::FunctionBindState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            shared_constructor_field4,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field4 = *shared_constructor_field4;
            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                identity.clone(),
                Box::new(shared_constructor_field4.clone()),
                true,
                true,
                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                    self::JavaScriptOrdinaryProperty,
                >::new())),
                Box::new(shared_constructor_field1.clone()),
                Box::new(shared_constructor_field2.clone()),
                std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(
                    &{
                        let ml_reference_value571 = shared_constructor_field3.borrow().clone();
                        ml_reference_value571
                    },
                    0f64,
                    f64::INFINITY,
                ))),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_function_bind_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptFunctionBindState,
    identity: self::ml::Big,
) -> self::JavaScriptFunctionBindTransition {
    match state.clone() {
        self::JavaScriptFunctionBindState::FunctionBindState(
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
            shared_constructor_field14,
            shared_constructor_field15,
        ) => {
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field14 = *shared_constructor_field14;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    shared_constructor_field2,
                    shared_constructor_field3,
                    shared_constructor_field4,
                    shared_constructor_field5,
                    shared_constructor_field6,
                ) => {
                    let error = self::read_java_script_function_bind_state_error(
                        heap.clone(),
                        state.clone(),
                    );
                    if ((error != (self::ml_string::Text::from_units(vec![])))
                        || (shared_constructor_field10
                            != (self::ml_string::Text::from_units(vec![
                                97, 108, 108, 111, 99, 97, 116, 101,
                            ]))))
                    {
                        self::JavaScriptFunctionBindTransition::FunctionBindInvalid(
                            Box::new(heap.clone()),
                            if !error.0.is_empty() {
                                error.clone()
                            } else {
                                self::ml_string::Text::from_units(vec![
                                    98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111,
                                    110, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 114, 101,
                                    115, 117, 109, 101, 115, 32, 111, 110, 108, 121, 32, 105, 116,
                                    115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99,
                                    97, 116, 105, 111, 110,
                                ])
                            },
                        )
                    } else {
                        {
                            let request =
                                self::step_java_script_function_bind(heap.clone(), state.clone());
                            match request.clone() {
                                self::JavaScriptFunctionBindTransition::FunctionBindAllocate(
                                    _,
                                    _,
                                    _,
                                ) => {
                                    if ((((identity <= (self::ml::Big::from_i128(0)))
                                        || (identity
                                            > (self::ml::Big::from_i128(18446744073709551615))))
                                        || ((self::find_java_script_ordinary_object(
                                            shared_constructor_field1.clone(),
                                            identity.clone(),
                                        )) >= (0f64)))
                                        || self::ml_array::includes(
                                            &{
                                                let ml_reference_value572 =
                                                    shared_constructor_field2.borrow().clone();
                                                ml_reference_value572
                                            },
                                            &identity.clone(),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        ))
                                    {
                                        self::JavaScriptFunctionBindTransition::FunctionBindInvalid(
                                            Box::new(heap.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116,
                                                105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116,
                                                105, 111, 110, 32, 114, 101, 113, 117, 105, 114,
                                                101, 115, 32, 97, 32, 102, 114, 101, 115, 104, 32,
                                                101, 120, 116, 101, 114, 110, 97, 108, 32, 103,
                                                108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116,
                                                105, 116, 121,
                                            ]),
                                        )
                                    } else {
                                        {
                                            let object =
                                                self::create_java_script_bound_function_object(
                                                    state.clone(),
                                                    identity.clone(),
                                                );
                                            {
                                                let at = 0f64;
                                                {
                                                    let at_2 = self::source_ml_source1_temporary_resume_java_script_function_bind_allocation_loop291(heap.clone(), identity.clone(), at);
                                                    {
                                                        let next = {
                                                            let source_ml_source2_nominal_field7327 =
                                                                shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field7329 =
                                                                    shared_constructor_field2
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field7331 =
                                                                        shared_constructor_field3
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field7333 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field7335 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field7337 = shared_constructor_field6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_override7339 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                            let source_ml_source2_concat_value7323 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value573 = shared_constructor_field1.borrow().clone(); ml_reference_value573 }, 0f64, at_2)));
                                                                                            {
                                                                                                let source_ml_source2_concat_value7324 = std::rc::Rc::new(std::cell::RefCell::new(vec![object.clone()]));
                                                                                                {
                                                                                                    let source_ml_source2_concat_value7325 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value574 = shared_constructor_field1.borrow().clone(); ml_reference_value574 }, at_2, f64::INFINITY)));
                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { let ml_reference_value575 = source_ml_source2_concat_value7323.borrow().clone(); ml_reference_value575 }), { let ml_reference_value576 = source_ml_source2_concat_value7324.borrow().clone(); ml_reference_value576 }), { let ml_reference_value577 = source_ml_source2_concat_value7325.borrow().clone(); ml_reference_value577 })
                                                                                                }
                                                                                            }
                                                                                        }));
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_nominal_override7339.clone(), source_ml_source2_nominal_field7329.clone(), source_ml_source2_nominal_field7331.clone(), source_ml_source2_nominal_field7333.clone(), source_ml_source2_nominal_field7335.clone(), source_ml_source2_nominal_field7337.clone())
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let invalid = self::read_java_script_ordinary_heap_error(next.clone());
                                                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::JavaScriptFunctionBindTransition::FunctionBindInvalid(Box::new(heap.clone()), invalid.clone())
                                                                } else {
                                                                    self::JavaScriptFunctionBindTransition::FunctionBindContinue(Box::new(next.clone()), Box::new({
                                                                        let source_ml_source2_nominal_field7341 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field7343 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field7345 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field7347 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field7349 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field7351 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field7353 = shared_constructor_field13;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field7355 = shared_constructor_field14.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field7357 = shared_constructor_field15.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override7360 = self::JavaScriptSemanticValue::Object(identity.clone(), true);
                                                                                                            self::JavaScriptFunctionBindState::FunctionBindState(Box::new(source_ml_source2_nominal_field7341.clone()), Box::new(source_ml_source2_nominal_field7343.clone()), source_ml_source2_nominal_field7345.clone(), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 111, 119, 110]), Box::new(source_ml_source2_nominal_field7349.clone()), Box::new(source_ml_source2_nominal_override7360.clone()), source_ml_source2_nominal_field7353, Box::new(source_ml_source2_nominal_field7355.clone()), source_ml_source2_nominal_field7357.clone())
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
                                _ => request.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}
