// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionsCall;
use super::JavaScriptPropertyDefinitionsState;
use super::JavaScriptPropertyDefinitionsTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040;
use super::MlSource1TemporaryStepJavaScriptPropertyDefinitionsLoop1041Result;
use super::accept_java_script_bulk_descriptor_conversion;
use super::accept_java_script_bulk_property_definition;
use super::begin_java_script_descriptor_conversion;
use super::complete_java_script_property_definition_with_symbol;
use super::construct_java_script_reflection_text;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_ordinary_own_keys;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_definitions_state_error;
use super::read_java_script_reflected_own_property;
use super::read_java_script_semantic_text_length;
use super::resume_java_script_descriptor_conversion;
use super::resume_java_script_property_definition;
use super::source_ml_source1_temporary_step_java_script_property_definitions_loop1041;
use super::step_java_script_descriptor_conversion;
use super::step_java_script_property_definition;
use super::test_java_script_reflection_ordinary_kind;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_property_definitions(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyDefinitionsState,
) -> self::JavaScriptPropertyDefinitionsTransition {
    match state.clone() {
        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
            shared_constructor_field14,
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
        ) => {
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field4,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let invalid = self::read_java_script_property_definitions_state_error(
                        heap.clone(),
                        state.clone(),
                    );
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(
                            Box::new(heap.clone()),
                            invalid.clone(),
                        )
                    } else {
                        if !(match (
                            match state.clone() {
                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(
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
                                    _,
                                    source_ml_source2_data_property_value22308,
                                ) => ((source_ml_source2_data_property_value22308
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptPropertyDefinitionsCall>,)
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
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsCall(Box::new(heap.clone()), Box::new(state.clone()), Box::new(match (match state.clone() {
                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22309) => {
                                        ((source_ml_source2_data_property_value22309.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value") {
                                    self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(source_ml_source2_data_property_value22310, _, _) => {
                                        let source_ml_source2_data_property_value22310 = *source_ml_source2_data_property_value22310;
                                        source_ml_source2_data_property_value22310.clone()
                                    }
                                }), Box::new(match (match state.clone() {
                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22311) => {
                                        ((source_ml_source2_data_property_value22311.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value") {
                                    self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(_, source_ml_source2_data_property_value22312, _) => {
                                        let source_ml_source2_data_property_value22312 = *source_ml_source2_data_property_value22312;
                                        source_ml_source2_data_property_value22312.clone()
                                    }
                                }), match (match state.clone() {
                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22313) => {
                                        ((source_ml_source2_data_property_value22313.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value") {
                                    self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(_, _, source_ml_source2_data_property_value22314) => {
                                        source_ml_source2_data_property_value22314.clone()
                                    }
                                })
                        } else {
                            if (shared_constructor_field10
                                == (self::ml_string::Text::from_units(vec![
                                    97, 108, 108, 111, 99, 97, 116, 101,
                                ])))
                            {
                                self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsAllocate(Box::new(heap.clone()), Box::new(state.clone()), Box::new(shared_constructor_field8.clone()))
                            } else {
                                if ((shared_constructor_field10 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116]))) && !(match (match state.clone() {
                                        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22320, _, _) => {
                                            ((source_ml_source2_data_property_value22320.clone().unwrap_or_else(|| (None::<self::JavaScriptDescriptorConversionState>,))).0).clone()
                                        }
                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                        self::accept_java_script_bulk_descriptor_conversion(heap.clone(), state.clone(), self::step_java_script_descriptor_conversion(heap.clone(), (match state.clone() {
                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22321, _, _) => {
                                                ((source_ml_source2_data_property_value22321.clone().unwrap_or_else(|| (None::<self::JavaScriptDescriptorConversionState>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value")))
                                    } else {
                                        if ((shared_constructor_field10 == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))) && !(match (match state.clone() {
                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22324, _) => {
                                                ((source_ml_source2_data_property_value22324.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                            self::accept_java_script_bulk_property_definition(heap.clone(), state.clone(), self::step_java_script_property_definition(heap.clone(), (match state.clone() {
                                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22325, _) => {
                                                    ((source_ml_source2_data_property_value22325.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value")))
                                        } else {
                                            if (shared_constructor_field10 == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) {
                                                if ((shared_constructor_field5 == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97, 116, 101]))) && {
                                                    let source_ml_source2_s22331 = shared_constructor_field7.clone();
                                                    match source_ml_source2_s22331.clone() {
                                                        self::JavaScriptSemanticValue::Undefined => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    }
                                                }) {
                                                    self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsCompleted(Box::new(heap.clone()), Box::new(shared_constructor_field6.clone()))
                                                } else {
                                                    if ({
                                                        let source_ml_source2_s22335 = shared_constructor_field7.clone();
                                                        match source_ml_source2_s22335.clone() {
                                                            self::JavaScriptSemanticValue::Null => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        }
                                                    } || {
                                                        let source_ml_source2_s22338 = shared_constructor_field7.clone();
                                                        match source_ml_source2_s22338.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        }
                                                    }) {
                                                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 111, 114, 32, 110, 117, 108, 108, 32, 116, 111, 32, 111, 98, 106, 101, 99, 116]))
                                                    } else {
                                                        {
                                                            let keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                            {
                                                                let source_ml_source2_s22364 = if {
                                                                    let source_ml_source2_s22366 = shared_constructor_field7.clone();
                                                                    match source_ml_source2_s22366.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            true
                                                                        }
                                                                        _ => {
                                                                            false
                                                                        }
                                                                    }
                                                                } {
                                                                    {
                                                                        let owner = {
                                                                            let source_ml_source2_array_read_value22374 = shared_constructor_field4.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index22375 = self::find_java_script_ordinary_object(shared_constructor_field4.clone(), (((match shared_constructor_field7.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value22373, _) => {
                                                                                        Some((source_ml_source2_data_property_value22373.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(self::ml::Big,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone());
                                                                                { let ml_reference_array1115 = source_ml_source2_array_read_value22374.clone(); let ml_reference_position1115 = source_ml_source2_array_read_index22375; let ml_reference_projection1115 = self::ml_array::optional_at(&*ml_reference_array1115.borrow(), self::ml_array::number_index(ml_reference_position1115)); ml_reference_projection1115 }
                                                                            }
                                                                        };
                                                                        if (match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_reflection_ordinary_kind(&(match owner.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                                                            }
                                                                        })))) {
                                                                            self::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040Return(Box::new(self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 108, 107, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 107, 101, 121, 32, 97, 110, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 112, 114, 111, 116, 111, 99, 111, 108, 115]))))
                                                                        } else {
                                                                            {
                                                                                let own = self::read_java_script_ordinary_own_keys(heap.clone(), (((match shared_constructor_field7.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value22377, _) => {
                                                                                        Some((source_ml_source2_data_property_value22377.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(self::ml::Big,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone());
                                                                                if (((({ let ml_reference_value1116 = own.borrow().clone(); ml_reference_value1116 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                    self::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040Return(Box::new(self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(Box::new(heap.clone()), (({ let ml_reference_value1117 = own.borrow().clone(); ml_reference_value1117 }).0).0)))
                                                                                } else {
                                                                                    {
                                                                                        let keys_2 = (({ let ml_reference_value1118 = own.borrow().clone(); ml_reference_value1118 }).0).1;
                                                                                        self::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040Next(keys_2.clone())
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if ({
                                                                        let source_ml_source2_s22379 = shared_constructor_field7.clone();
                                                                        match source_ml_source2_s22379.clone() {
                                                                            self::JavaScriptSemanticValue::Text(_) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        }
                                                                    } || {
                                                                        let source_ml_source2_s22383 = shared_constructor_field7.clone();
                                                                        match source_ml_source2_s22383.clone() {
                                                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        }
                                                                    }) {
                                                                        {
                                                                            let length = self::read_java_script_semantic_text_length(shared_constructor_field7.clone());
                                                                            if (length >= (4096f64)) {
                                                                                self::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040Return(Box::new(self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 98, 117, 108, 107, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 98, 117, 100, 103, 101, 116]))))
                                                                            } else {
                                                                                {
                                                                                    let at = 0f64;
                                                                                    {
                                                                                        let source_ml_source2_s22388 = self::source_ml_source1_temporary_step_java_script_property_definitions_loop1041(keys.clone(), length, at);
                                                                                        match source_ml_source2_s22388.clone() {
                                                                                            self::MlSource1TemporaryStepJavaScriptPropertyDefinitionsLoop1041Result::MlSource1TemporaryStepJavaScriptPropertyDefinitionsLoop1041Done => {
                                                                                                {
                                                                                                    let source_ml_source1_temporary_step_java_script_property_definitions_ignored1042 = { let ml_method_owner1119 = keys.clone(); let ml_method_argument1119_0 = self::construct_java_script_reflection_text(&(self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]))); let ml_inserted = vec![ml_method_argument1119_0]; let mut ml_method_values1119 = ml_method_owner1119.borrow_mut(); let ml_length = ml_method_values1119.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values1119.extend(ml_inserted); ml_length as f64 };
                                                                                                    self::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040Next(keys.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        self::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040Next(keys.clone())
                                                                    }
                                                                };
                                                                match source_ml_source2_s22364.clone() {
                                                                    self::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040Next(keys_3) => {
                                                                        if (({ let ml_reference_array1120 = keys_3.clone(); let ml_reference_projection1120 = ml_reference_array1120.borrow().len() as f64; ml_reference_projection1120 }) > (4096f64)) {
                                                                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 98, 117, 100, 103, 101, 116]))
                                                                        } else {
                                                                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsContinue(Box::new(heap.clone()), Box::new({
                                                                                let source_ml_source2_nominal_field22390 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field22392 = shared_constructor_field6.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field22394 = shared_constructor_field7.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field22396 = shared_constructor_field8.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field22398 = shared_constructor_field9.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field22400 = shared_constructor_field10.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field22402 = shared_constructor_field11.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field22404 = shared_constructor_field12;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field22406 = shared_constructor_field13.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field22408 = shared_constructor_field14;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field22410 = shared_constructor_field15.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field22412 = shared_constructor_field16.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field22414 = shared_constructor_field17.clone();
                                                                                                                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(source_ml_source2_nominal_field22390.clone(), Box::new(source_ml_source2_nominal_field22392.clone()), Box::new(source_ml_source2_nominal_field22394.clone()), Box::new(source_ml_source2_nominal_field22396.clone()), Box::new(source_ml_source2_nominal_field22398.clone()), self::ml_string::Text::from_units(vec![114, 101, 97, 100]), keys_3.clone(), source_ml_source2_nominal_field22404, source_ml_source2_nominal_field22406.clone(), source_ml_source2_nominal_field22408, source_ml_source2_nominal_field22410.clone(), source_ml_source2_nominal_field22412.clone(), source_ml_source2_nominal_field22414.clone())
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
                                                                            }))
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040::MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040Return(source_ml_source1_temporary_result1040) => {
                                                                        let source_ml_source1_temporary_result1040 = *source_ml_source1_temporary_result1040;
                                                                        source_ml_source1_temporary_result1040.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if (shared_constructor_field10 == (self::ml_string::Text::from_units(vec![114, 101, 97, 100]))) {
                                                    if (shared_constructor_field12 == ({ let ml_reference_array1121 = shared_constructor_field11.clone(); let ml_reference_projection1121 = ml_reference_array1121.borrow().len() as f64; ml_reference_projection1121 })) {
                                                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsContinue(Box::new(heap.clone()), Box::new({
                                                            let source_ml_source2_nominal_field22425 = shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field22427 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field22429 = shared_constructor_field7.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field22431 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field22433 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field22435 = shared_constructor_field10.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field22437 = shared_constructor_field11.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field22439 = shared_constructor_field12;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field22441 = shared_constructor_field13.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field22443 = shared_constructor_field14;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field22445 = shared_constructor_field15.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field22447 = shared_constructor_field16.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field22449 = shared_constructor_field17.clone();
                                                                                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(source_ml_source2_nominal_field22425.clone(), Box::new(source_ml_source2_nominal_field22427.clone()), Box::new(source_ml_source2_nominal_field22429.clone()), Box::new(source_ml_source2_nominal_field22431.clone()), Box::new(source_ml_source2_nominal_field22433.clone()), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]), source_ml_source2_nominal_field22437.clone(), source_ml_source2_nominal_field22439, source_ml_source2_nominal_field22441.clone(), source_ml_source2_nominal_field22443, source_ml_source2_nominal_field22445.clone(), source_ml_source2_nominal_field22447.clone(), source_ml_source2_nominal_field22449.clone())
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
                                                        }))
                                                    } else {
                                                        {
                                                            let key = {
                                                                let source_ml_source2_array_read_value22454 = shared_constructor_field11.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index22455 = shared_constructor_field12;
                                                                    { let ml_reference_array1122 = source_ml_source2_array_read_value22454.clone(); let ml_reference_position1122 = source_ml_source2_array_read_index22455; let ml_reference_projection1122 = self::ml_array::optional_at(&*ml_reference_array1122.borrow(), self::ml_array::number_index(ml_reference_position1122)); ml_reference_projection1122 }
                                                                }
                                                            };
                                                            if match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 107, 101, 121]))
                                                            } else {
                                                                {
                                                                    let property = self::read_java_script_reflected_own_property(heap.clone(), shared_constructor_field7.clone(), key.clone().expect("a proven present value"));
                                                                    if (match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !((({ let ml_reference_value1123 = (match property.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value22457) => {
                                                                            source_ml_source2_data_property_value22457.clone()
                                                                        }
                                                                    }).borrow().clone(); ml_reference_value1123 }).0).2)) {
                                                                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsContinue(Box::new(heap.clone()), Box::new({
                                                                            let source_ml_source2_nominal_field22460 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field22462 = shared_constructor_field6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field22464 = shared_constructor_field7.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field22466 = shared_constructor_field8.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field22468 = shared_constructor_field9.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field22470 = shared_constructor_field10.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field22472 = shared_constructor_field11.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field22474 = shared_constructor_field12;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field22476 = shared_constructor_field13.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field22478 = shared_constructor_field14;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field22480 = shared_constructor_field15.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field22482 = shared_constructor_field16.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field22484 = shared_constructor_field17.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override22486 = (shared_constructor_field12 + 1f64);
                                                                                                                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(source_ml_source2_nominal_field22460.clone(), Box::new(source_ml_source2_nominal_field22462.clone()), Box::new(source_ml_source2_nominal_field22464.clone()), Box::new(source_ml_source2_nominal_field22466.clone()), Box::new(source_ml_source2_nominal_field22468.clone()), source_ml_source2_nominal_field22470.clone(), source_ml_source2_nominal_field22472.clone(), source_ml_source2_nominal_override22486, source_ml_source2_nominal_field22476.clone(), source_ml_source2_nominal_field22478, source_ml_source2_nominal_field22480.clone(), source_ml_source2_nominal_field22482.clone(), source_ml_source2_nominal_field22484.clone())
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
                                                                        }))
                                                                    } else {
                                                                        if {
                                                                            let source_ml_source2_s22488 = shared_constructor_field7.clone();
                                                                            match source_ml_source2_s22488.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }
                                                                        } {
                                                                            self::accept_java_script_bulk_descriptor_conversion(heap.clone(), state.clone(), self::begin_java_script_descriptor_conversion(heap.clone(), (({ let ml_reference_value1124 = (match property.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value22492) => {
                                                                                    source_ml_source2_data_property_value22492.clone()
                                                                                }
                                                                            }).borrow().clone(); ml_reference_value1124 }).0).8))
                                                                        } else {
                                                                            {
                                                                                let get = self::plan_java_script_ordinary_get(heap.clone(), (((match shared_constructor_field7.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value22494, _) => {
                                                                                        Some((source_ml_source2_data_property_value22494.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(self::ml::Big,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone(), key.clone().expect("a proven present value"), shared_constructor_field7.clone());
                                                                                if (((({ let ml_reference_value1125 = get.borrow().clone(); ml_reference_value1125 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                    self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(Box::new(heap.clone()), (({ let ml_reference_value1126 = get.borrow().clone(); ml_reference_value1126 }).0).3)
                                                                                } else {
                                                                                    if (((({ let ml_reference_value1127 = get.borrow().clone(); ml_reference_value1127 }).0).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                                        self::accept_java_script_bulk_descriptor_conversion(heap.clone(), state.clone(), self::begin_java_script_descriptor_conversion(heap.clone(), (({ let ml_reference_value1128 = get.borrow().clone(); ml_reference_value1128 }).0).5))
                                                                                    } else {
                                                                                        if (((({ let ml_reference_value1129 = get.borrow().clone(); ml_reference_value1129 }).0).0) != (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 118, 97, 108, 117, 101]))
                                                                                        } else {
                                                                                            {
                                                                                                let call = self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(Box::new((({ let ml_reference_value1130 = get.borrow().clone(); ml_reference_value1130 }).0).2), Box::new((({ let ml_reference_value1131 = get.borrow().clone(); ml_reference_value1131 }).0).4), (({ let ml_reference_value1132 = get.borrow().clone(); ml_reference_value1132 }).0).1);
                                                                                                match call.clone() {
                                                                                                    self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                                                        let shared_constructor_field1 = *shared_constructor_field1;
                                                                                                        let shared_constructor_field2 = *shared_constructor_field2;
                                                                                                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsCall(Box::new(heap.clone()), Box::new({
                                                                                                            let source_ml_source2_nominal_field22497 = shared_constructor_field5.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field22499 = shared_constructor_field6.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field22501 = shared_constructor_field7.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field22503 = shared_constructor_field8.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field22505 = shared_constructor_field9.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field22507 = shared_constructor_field10.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field22509 = shared_constructor_field11.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field22511 = shared_constructor_field12;
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field22513 = shared_constructor_field13.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field22515 = shared_constructor_field14;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field22517 = shared_constructor_field15.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field22519 = shared_constructor_field16.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field22521 = shared_constructor_field17.clone();
                                                                                                                                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(source_ml_source2_nominal_field22497.clone(), Box::new(source_ml_source2_nominal_field22499.clone()), Box::new(source_ml_source2_nominal_field22501.clone()), Box::new(source_ml_source2_nominal_field22503.clone()), Box::new(source_ml_source2_nominal_field22505.clone()), self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]), source_ml_source2_nominal_field22509.clone(), source_ml_source2_nominal_field22511, source_ml_source2_nominal_field22513.clone(), source_ml_source2_nominal_field22515, source_ml_source2_nominal_field22517.clone(), source_ml_source2_nominal_field22519.clone(), Some((Some(call.clone()),)))
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
                                                                                                        }), Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone())
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
                                                } else {
                                                    if (shared_constructor_field10 == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]))) {
                                                        if (shared_constructor_field14 == ({ let ml_reference_array1133 = shared_constructor_field13.clone(); let ml_reference_projection1133 = ml_reference_array1133.borrow().len() as f64; ml_reference_projection1133 })) {
                                                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsCompleted(Box::new(heap.clone()), Box::new(shared_constructor_field6.clone()))
                                                        } else {
                                                            {
                                                                let property_2 = {
                                                                    let source_ml_source2_array_read_value22537 = shared_constructor_field13.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index22538 = shared_constructor_field14;
                                                                        { let ml_reference_array1134 = source_ml_source2_array_read_value22537.clone(); let ml_reference_position1134 = source_ml_source2_array_read_index22538; let ml_reference_projection1134 = self::ml_array::optional_at(&*ml_reference_array1134.borrow(), self::ml_array::number_index(ml_reference_position1134)); ml_reference_projection1134 }
                                                                    }
                                                                };
                                                                if match (property_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 112, 114, 101, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114]))
                                                                } else {
                                                                    self::accept_java_script_bulk_property_definition(heap.clone(), state.clone(), self::complete_java_script_property_definition_with_symbol(heap.clone(), shared_constructor_field6.clone(), match property_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value22540, _) => {
                                                                            let source_ml_source2_data_property_value22540 = *source_ml_source2_data_property_value22540;
                                                                            source_ml_source2_data_property_value22540.clone()
                                                                        }
                                                                    }, match property_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value22541) => {
                                                                            source_ml_source2_data_property_value22541.clone()
                                                                        }
                                                                    }, Some(shared_constructor_field9.clone()), None::<self::ml_string::Text>))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 112, 104, 97, 115, 101]))
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
pub fn resume_java_script_property_definitions_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyDefinitionsState,
    completion: self::JavaScriptSemanticCompletion,
) -> self::JavaScriptPropertyDefinitionsTransition {
    match state.clone() {
        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(
            _,
            _,
            _,
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
            let error = {
                let source_ml_source2_logical22545 =
                    self::read_java_script_property_definitions_state_error(
                        heap.clone(),
                        state.clone(),
                    );
                if !source_ml_source2_logical22545.0.is_empty() {
                    source_ml_source2_logical22545.clone()
                } else {
                    self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        match completion.clone() {
                            self::JavaScriptSemanticCompletion::NormalCompletion(
                                source_ml_source2_data_property_value22543,
                            ) => {
                                let source_ml_source2_data_property_value22543 =
                                    *source_ml_source2_data_property_value22543;
                                source_ml_source2_data_property_value22543.clone()
                            }
                            self::JavaScriptSemanticCompletion::ThrowCompletion(
                                source_ml_source2_data_property_value22544,
                            ) => {
                                let source_ml_source2_data_property_value22544 =
                                    *source_ml_source2_data_property_value22544;
                                source_ml_source2_data_property_value22544.clone()
                            }
                        },
                    )
                }
            };
            if ((error != (self::ml_string::Text::from_units(vec![])))
                || match (
                    match state.clone() {
                        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(
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
                            _,
                            source_ml_source2_data_property_value22546,
                        ) => ((source_ml_source2_data_property_value22546
                            .clone()
                            .unwrap_or_else(
                                || (None::<self::JavaScriptPropertyDefinitionsCall>,),
                            ))
                        .0)
                            .clone(),
                    },
                    None::<()>,
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                })
            {
                self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(
                    Box::new(heap.clone()),
                    if !error.0.is_empty() {
                        error.clone()
                    } else {
                        self::ml_string::Text::from_units(vec![
                            98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111,
                            110, 115, 32, 114, 101, 115, 117, 109, 101, 32, 111, 110, 108, 121, 32,
                            116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                            108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100,
                        ])
                    },
                )
            } else {
                match completion.clone() {
                    self::JavaScriptSemanticCompletion::NormalCompletion(value) => {
                        let value = *value;
                        if (shared_constructor_field1
                            == (self::ml_string::Text::from_units(vec![
                                103, 101, 116, 116, 101, 114,
                            ])))
                        {
                            self::accept_java_script_bulk_descriptor_conversion(
                                heap.clone(),
                                state.clone(),
                                self::begin_java_script_descriptor_conversion(
                                    heap.clone(),
                                    value.clone(),
                                ),
                            )
                        } else {
                            if ((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116]))) && !(match (match state.clone() {
                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22552, _, _) => {
                                        ((source_ml_source2_data_property_value22552.clone().unwrap_or_else(|| (None::<self::JavaScriptDescriptorConversionState>,))).0).clone()
                                    }
                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                    self::accept_java_script_bulk_descriptor_conversion(heap.clone(), state.clone(), self::resume_java_script_descriptor_conversion(heap.clone(), (match state.clone() {
                                        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22553, _, _) => {
                                            ((source_ml_source2_data_property_value22553.clone().unwrap_or_else(|| (None::<self::JavaScriptDescriptorConversionState>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value"), completion.clone()))
                                } else {
                                    if ((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))) && !(match (match state.clone() {
                                        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22556, _) => {
                                            ((source_ml_source2_data_property_value22556.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                        }
                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                        self::accept_java_script_bulk_property_definition(heap.clone(), state.clone(), self::resume_java_script_property_definition(heap.clone(), (match state.clone() {
                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22557, _) => {
                                                ((source_ml_source2_data_property_value22557.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value"), completion.clone()))
                                    } else {
                                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 112, 104, 97, 115, 101]))
                                    }
                                }
                        }
                    }
                    self::JavaScriptSemanticCompletion::ThrowCompletion(value_2) => {
                        let value_2 = *value_2;
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsThrow(
                            Box::new(heap.clone()),
                            Box::new(value_2.clone()),
                        )
                    }
                }
            }
        }
    }
}
