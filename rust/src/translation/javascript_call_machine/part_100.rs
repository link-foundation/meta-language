// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptObjectFromEntriesState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998;
use super::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999;
use super::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000;
use super::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Result;
use super::find_java_script_ordinary_object;
use super::finish_java_script_property_key_conversion;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_iterator_protocol_state_error;
use super::read_java_script_object_from_entries_entry_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_synchronous_iterator_record_error;
use super::resume_java_script_primitive_conversion;
use super::source_ml_source1_temporary_read_java_script_object_from_entries_state_error_loop997;
use super::step_java_script_primitive_conversion;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_property_key_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
) -> self::JavaScriptPropertyKeyConversionTransition {
    if ((match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
            _,
            _,
            source_ml_source2_data_property_value19987,
            _,
        ) => source_ml_source2_data_property_value19987.clone(),
    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
    {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
            self::ml_string::Text::from_units(vec![
                97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 99, 111,
                110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110,
                115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 110,
                103, 32, 104, 105, 110, 116,
            ]),
        )
    } else {
        self::finish_java_script_property_key_conversion(
            self::step_java_script_primitive_conversion(heap.clone(), state.clone()),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_property_key_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptPropertyKeyConversionTransition {
    if ((match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
            _,
            _,
            source_ml_source2_data_property_value19989,
            _,
        ) => source_ml_source2_data_property_value19989.clone(),
    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
    {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
            self::ml_string::Text::from_units(vec![
                97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 99, 111,
                110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110,
                115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 110,
                103, 32, 104, 105, 110, 116,
            ]),
        )
    } else {
        self::finish_java_script_property_key_conversion(
            self::resume_java_script_primitive_conversion(
                heap.clone(),
                state.clone(),
                completion.clone(),
            ),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_from_entries_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectFromEntriesState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error = self::read_java_script_ordinary_heap_error(heap.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        error.clone()
                    } else {
                        if (match state.clone() {
                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => false,
                        } || !(self::ml_array::includes(
                            &{
                                let ml_reference_value1041 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            97, 108, 108, 111, 99, 97, 116, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            97, 99, 113, 117, 105, 114, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![115, 116, 101, 112]),
                                        self::ml_string::Text::from_units(vec![
                                            101, 110, 116, 114, 121, 32, 107, 101, 121,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            101, 110, 116, 114, 121, 32, 107, 101, 121, 32, 103,
                                            101, 116, 116, 101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            101, 110, 116, 114, 121, 32, 118, 97, 108, 117, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            101, 110, 116, 114, 121, 32, 118, 97, 108, 117, 101,
                                            32, 103, 101, 116, 116, 101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            99, 111, 110, 118, 101, 114, 116, 32, 107, 101, 121,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            100, 101, 102, 105, 110, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            99, 108, 111, 115, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            101, 114, 114, 111, 114,
                                        ]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value1041
                            },
                            &shared_constructor_field7.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        ))) {
                            self::ml_string::Text::from_units(vec![
                                79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116,
                                114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114,
                                99, 101, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111,
                                110, 32, 112, 104, 97, 115, 101,
                            ])
                        } else {
                            {
                                let source_ml_source1_temporary_values996 =
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        shared_constructor_field2.clone(),
                                        shared_constructor_field3.clone(),
                                        shared_constructor_field4.clone(),
                                        shared_constructor_field5.clone(),
                                        shared_constructor_field6.clone(),
                                        shared_constructor_field8.clone(),
                                        shared_constructor_field9.clone(),
                                        shared_constructor_field10.clone(),
                                        shared_constructor_field11.clone(),
                                    ]));
                                {
                                    let source_ml_source1_temporary_index996 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s20017 = self::source_ml_source1_temporary_read_java_script_object_from_entries_state_error_loop997(heap.clone(), source_ml_source1_temporary_values996.clone(), source_ml_source1_temporary_index996.clone());
                                        match source_ml_source2_s20017.clone() {
                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Result::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Done => {
                                                    if ((({
                                                        let source_ml_source2_s20019 = shared_constructor_field3.clone();
                                                        match source_ml_source2_s20019.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    } || ((match shared_constructor_field3.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value20024) => {
                                                            Some((source_ml_source2_data_property_value20024,))
                                                        }
                                                        _ => {
                                                            None::<(bool,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0) || {
                                                        let source_ml_source2_s20026 = shared_constructor_field4.clone();
                                                        match source_ml_source2_s20026.clone() {
                                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) || {
                                                        let source_ml_source2_s20030 = shared_constructor_field5.clone();
                                                        match source_ml_source2_s20030.clone() {
                                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 97, 110, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108, 115])
                                                    } else {
                                                        {
                                                            let source_ml_source2_s20063 = if (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]))) {
                                                                if (({
                                                                    let source_ml_source2_s20067 = shared_constructor_field6.clone();
                                                                    match source_ml_source2_s20067.clone() {
                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }
                                                                } || !(match (match state.clone() {
                                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20069, _, _, _, _, _, _) => {
                                                                        ((source_ml_source2_data_property_value20069.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) || !(match (match state.clone() {
                                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20070, _, _, _, _, _) => {
                                                                        ((source_ml_source2_data_property_value20070.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 97, 108, 108, 111, 99, 97, 116, 101, 115, 32, 105, 116, 115, 32, 114, 101, 115, 117, 108, 116, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110]))
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998Next
                                                                }
                                                            } else {
                                                                {
                                                                    let target = if {
                                                                        let source_ml_source2_s20072 = shared_constructor_field6.clone();
                                                                        match source_ml_source2_s20072.clone() {
                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        }
                                                                    } {
                                                                        {
                                                                            let source_ml_source2_array_read_value20080 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index20081 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match shared_constructor_field6.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value20079, _) => {
                                                                                        Some((source_ml_source2_data_property_value20079.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(self::ml::Big,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone());
                                                                                { let ml_reference_array1042 = source_ml_source2_array_read_value20080.clone(); let ml_reference_position1042 = source_ml_source2_array_read_index20081; let ml_reference_projection1042 = self::ml_array::optional_at(&*ml_reference_array1042.borrow(), self::ml_array::number_index(ml_reference_position1042)); ml_reference_projection1042 }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        None::<self::JavaScriptOrdinaryObject>
                                                                    };
                                                                    if (((match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match target.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }) || ((match target.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value20091, _) => {
                                                                            Some((source_ml_source2_data_property_value20091,))
                                                                        }
                                                                        _ => {
                                                                            None::<(bool,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0) || !(self::test_java_script_semantic_same_value((((match target.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value20092, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value20092 = *source_ml_source2_data_property_value20092;
                                                                            Some((source_ml_source2_data_property_value20092.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(self::JavaScriptSemanticValue,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone(), shared_constructor_field3.clone()))) {
                                                                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 114, 101, 115, 117, 108, 116, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998Next
                                                                    }
                                                                }
                                                            };
                                                            match source_ml_source2_s20063.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998Next => {
                                                                    {
                                                                        let source_ml_source2_s20097 = if !(match (match state.clone() {
                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20098, _, _, _, _, _, _) => {
                                                                                ((source_ml_source2_data_property_value20098.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                            }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                            {
                                                                                let invalid = self::read_java_script_synchronous_iterator_record_error(heap.clone(), (match state.clone() {
                                                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20099, _, _, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value20099.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value"));
                                                                                if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999Return(invalid.clone())
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999Next
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if !(self::ml_array::includes(&{ let ml_reference_value1043 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101])]))).borrow().clone(); ml_reference_value1043 }, &shared_constructor_field7.clone(), 0f64, false, |a, b| a == b)) {
                                                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 113, 117, 105, 114, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 99, 111, 114, 100]))
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999Next
                                                                            }
                                                                        };
                                                                        match source_ml_source2_s20097.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999Next => {
                                                                                {
                                                                                    let source_ml_source2_s20147 = if !(match (match state.clone() {
                                                                                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20148, _, _, _, _, _) => {
                                                                                            ((source_ml_source2_data_property_value20148.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                        }
                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                        {
                                                                                            let invalid_2 = self::read_java_script_iterator_protocol_state_error(heap.clone(), (match state.clone() {
                                                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20149, _, _, _, _, _) => {
                                                                                                    ((source_ml_source2_data_property_value20149.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value"));
                                                                                            if (invalid_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000Return(invalid_2.clone())
                                                                                            } else {
                                                                                                if (!(self::ml_array::includes(&{ let ml_reference_value1044 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]), self::ml_string::Text::from_units(vec![115, 116, 101, 112]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])]))).borrow().clone(); ml_reference_value1044 }, &shared_constructor_field7.clone(), 0f64, false, |a, b| a == b)) || ((match (match state.clone() {
                                                                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20157, _, _, _, _, _) => {
                                                                                                        ((source_ml_source2_data_property_value20157.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().expect("a proven present value") {
                                                                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_data_property_value20158, _, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value20158.clone()
                                                                                                    }
                                                                                                }) != (if (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]))) {
                                                                                                    self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101])
                                                                                                } else {
                                                                                                    if (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![115, 116, 101, 112]))) {
                                                                                                        self::ml_string::Text::from_units(vec![115, 116, 101, 112])
                                                                                                    } else {
                                                                                                        self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])
                                                                                                    }
                                                                                                }))) {
                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 111, 110, 108, 121, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
                                                                                                } else {
                                                                                                    if ((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]))) && (!(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20165, _, _, _, _, _) => {
                                                                                                            ((source_ml_source2_data_property_value20165.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                        }
                                                                                                    }).clone().expect("a proven present value") {
                                                                                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, source_ml_source2_data_property_value20166, _, _, _, _, _, _) => {
                                                                                                            let source_ml_source2_data_property_value20166 = *source_ml_source2_data_property_value20166;
                                                                                                            source_ml_source2_data_property_value20166.clone()
                                                                                                        }
                                                                                                    }, shared_constructor_field2.clone())) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20168, _, _, _, _, _) => {
                                                                                                            ((source_ml_source2_data_property_value20168.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                        }
                                                                                                    }).clone().expect("a proven present value") {
                                                                                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, source_ml_source2_data_property_value20169, _, _, _, _, _) => {
                                                                                                            let source_ml_source2_data_property_value20169 = *source_ml_source2_data_property_value20169;
                                                                                                            source_ml_source2_data_property_value20169.clone()
                                                                                                        }
                                                                                                    }, shared_constructor_field4.clone())))) {
                                                                                                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 105, 116, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 105, 116, 101, 114, 97, 98, 108, 101, 32, 111, 114, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 83, 121, 109, 98, 111, 108]))
                                                                                                    } else {
                                                                                                        if ((shared_constructor_field7 != (self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]))) && (((match (match state.clone() {
                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20173, _, _, _, _, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value20173.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                            }
                                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(match match (match state.clone() {
                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20174, _, _, _, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value20174.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().expect("a proven present value") {
                                                                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value20175, _, _, _, _) => {
                                                                                                                let source_ml_source2_data_property_value20175 = *source_ml_source2_data_property_value20175;
                                                                                                                source_ml_source2_data_property_value20175.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value20176, _, _) => {
                                                                                                                let source_ml_source2_data_property_value20176 = *source_ml_source2_data_property_value20176;
                                                                                                                source_ml_source2_data_property_value20176.clone()
                                                                                                            }
                                                                                                        }, match (match state.clone() {
                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20177, _, _, _, _, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value20177.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().expect("a proven present value") {
                                                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value20178, _, _) => {
                                                                                                                let source_ml_source2_data_property_value20178 = *source_ml_source2_data_property_value20178;
                                                                                                                source_ml_source2_data_property_value20178.clone()
                                                                                                            }
                                                                                                        }))) || !(self::test_java_script_semantic_same_value(match match (match state.clone() {
                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20179, _, _, _, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value20179.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().expect("a proven present value") {
                                                                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value20180, _, _, _, _) => {
                                                                                                                let source_ml_source2_data_property_value20180 = *source_ml_source2_data_property_value20180;
                                                                                                                source_ml_source2_data_property_value20180.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value20181, _) => {
                                                                                                                let source_ml_source2_data_property_value20181 = *source_ml_source2_data_property_value20181;
                                                                                                                source_ml_source2_data_property_value20181.clone()
                                                                                                            }
                                                                                                        }, match (match state.clone() {
                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20182, _, _, _, _, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value20182.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().expect("a proven present value") {
                                                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value20183, _) => {
                                                                                                                let source_ml_source2_data_property_value20183 = *source_ml_source2_data_property_value20183;
                                                                                                                source_ml_source2_data_property_value20183.clone()
                                                                                                            }
                                                                                                        }))) || ((match match (match state.clone() {
                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20189, _, _, _, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value20189.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().expect("a proven present value") {
                                                                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value20190, _, _, _, _) => {
                                                                                                                let source_ml_source2_data_property_value20190 = *source_ml_source2_data_property_value20190;
                                                                                                                source_ml_source2_data_property_value20190.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value20191) => {
                                                                                                                source_ml_source2_data_property_value20191
                                                                                                            }
                                                                                                        }) != (match (match state.clone() {
                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20192, _, _, _, _, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value20192.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().expect("a proven present value") {
                                                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value20193) => {
                                                                                                                source_ml_source2_data_property_value20193
                                                                                                            }
                                                                                                        })))) {
                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 97, 99, 113, 117, 105, 114, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 110, 100, 32, 99, 97, 99, 104, 101, 100, 32, 110, 101, 120, 116, 32, 109, 101, 116, 104, 111, 100]))
                                                                                                        } else {
                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000Next
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000Next
                                                                                    };
                                                                                    match source_ml_source2_s20147.clone() {
                                                                                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000Next => {
                                                                                            if ((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]))) && !(match (match state.clone() {
                                                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20196, _, _, _, _, _, _) => {
                                                                                                    ((source_ml_source2_data_property_value20196.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                }
                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                                                self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 99, 111, 114, 100])
                                                                                            } else {
                                                                                                if ((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]))) && match (match state.clone() {
                                                                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20199, _, _, _, _, _) => {
                                                                                                        ((source_ml_source2_data_property_value20199.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                    }
                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                    self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 115, 116, 101, 100, 32, 112, 114, 111, 116, 111, 99, 111, 108])
                                                                                                } else {
                                                                                                    self::read_java_script_object_from_entries_entry_state_error(heap.clone(), state.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000Return(source_ml_source1_temporary_result1000) => {
                                                                                            source_ml_source1_temporary_result1000.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999Return(source_ml_source1_temporary_result1000_2) => {
                                                                                source_ml_source1_temporary_result1000_2.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998Return(source_ml_source1_temporary_result1000_3) => {
                                                                    source_ml_source1_temporary_result1000_3.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Result::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Return(source_ml_source1_temporary_result1000_4) => {
                                                    source_ml_source1_temporary_result1000_4.clone()
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
