// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptCollectionForEachCall;
use super::JavaScriptCollectionForEachState;
use super::JavaScriptCollectionForEachTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Result;
use super::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Result;
use super::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_collection_for_each_kind;
use super::read_java_script_collection_iterator_receiver_error;
use super::read_java_script_ordinary_heap_error;
use super::source_ml_source1_temporary_begin_java_script_collection_for_each_loop1182;
use super::source_ml_source1_temporary_read_java_script_collection_for_each_state_error_loop1179;
use super::source_ml_source1_temporary_read_java_script_collection_for_each_state_error_loop1180;
use super::test_java_script_collection_for_each_operation;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_for_each_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptCollectionForEachState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptCollectionForEachState::CollectionForEachState(
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            _,
        ) => {
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field11 = *shared_constructor_field11;
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
                    let error = self::read_java_script_ordinary_heap_error(heap.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        error.clone()
                    } else {
                        if (match state.clone() {
                            self::JavaScriptCollectionForEachState::CollectionForEachState(
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
                        } || !(self::test_java_script_collection_for_each_operation(
                            &(shared_constructor_field5),
                        ))) {
                            self::ml_string::Text::from_units(vec![
                                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114,
                                69, 97, 99, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105,
                                115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105,
                                111, 110,
                            ])
                        } else {
                            {
                                let kind = self::read_java_script_collection_for_each_kind(
                                    &(shared_constructor_field5),
                                );
                                {
                                    let receiver =
                                        self::read_java_script_collection_iterator_receiver_error(
                                            heap.clone(),
                                            shared_constructor_field6.clone(),
                                            &(kind),
                                        );
                                    if (((({
                                        let ml_reference_value1377 = receiver.borrow().clone();
                                        ml_reference_value1377
                                    })
                                    .0)
                                        .0)
                                        != (self::ml_string::Text::from_units(vec![])))
                                    {
                                        (({
                                            let ml_reference_value1378 = receiver.borrow().clone();
                                            ml_reference_value1378
                                        })
                                        .0)
                                            .0
                                    } else {
                                        {
                                            let source_ml_source1_temporary_values1178 =
                                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    shared_constructor_field7.clone(),
                                                    shared_constructor_field8.clone(),
                                                    shared_constructor_field11.clone(),
                                                    shared_constructor_field12.clone(),
                                                ]));
                                            {
                                                let source_ml_source1_temporary_index1178 =
                                                    self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s25666 = self::source_ml_source1_temporary_read_java_script_collection_for_each_state_error_loop1179(heap.clone(), source_ml_source1_temporary_values1178.clone(), source_ml_source1_temporary_index1178.clone());
                                                    match source_ml_source2_s25666.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Result::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Done => {
                                                                if ({
                                                                    let source_ml_source2_s25668 = shared_constructor_field7.clone();
                                                                    match source_ml_source2_s25668.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }
                                                                } || !(((match shared_constructor_field7.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value25673) => {
                                                                        Some((source_ml_source2_data_property_value25673,))
                                                                    }
                                                                    _ => {
                                                                        None::<(bool,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0)) {
                                                                    self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114, 69, 97, 99, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 99, 97, 108, 108, 98, 97, 99, 107])
                                                                } else {
                                                                    {
                                                                        let collection = if {
                                                                            let source_ml_source2_s25675 = shared_constructor_field6.clone();
                                                                            match source_ml_source2_s25675.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    true
                                                                                }
                                                                                _ => {
                                                                                    false
                                                                                }
                                                                            }
                                                                        } {
                                                                            {
                                                                                let source_ml_source2_array_read_value25683 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index25684 = self::find_java_script_ordinary_object(shared_constructor_field4.clone(), (((match shared_constructor_field6.clone() {
                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value25682, _) => {
                                                                                            Some((source_ml_source2_data_property_value25682.clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(self::ml::Big,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone());
                                                                                    { let ml_reference_array1379 = source_ml_source2_array_read_value25683.clone(); let ml_reference_position1379 = source_ml_source2_array_read_index25684; let ml_reference_projection1379 = self::ml_array::optional_at(&*ml_reference_array1379.borrow(), self::ml_array::number_index(ml_reference_position1379)); ml_reference_projection1379 }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            None::<self::JavaScriptOrdinaryObject>
                                                                        };
                                                                        if (match (collection.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (match collection.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                                false
                                                                            }
                                                                            _ => {
                                                                                true
                                                                            }
                                                                        } && match collection.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                                false
                                                                            }
                                                                            _ => {
                                                                                true
                                                                            }
                                                                        })) {
                                                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114, 69, 97, 99, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 111, 98, 106, 101, 99, 116])
                                                                        } else {
                                                                            if (((!(self::ml_math::is_safe_integer(shared_constructor_field9)) || (shared_constructor_field9 < (0f64))) || {
                                                                                let source_ml_source2_same_value_operand25701 = shared_constructor_field9;
                                                                                {
                                                                                    let source_ml_source2_same_value_operand25702 = (-0f64);
                                                                                    (((source_ml_source2_same_value_operand25701 == source_ml_source2_same_value_operand25702) && ((source_ml_source2_same_value_operand25701 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand25701)) == ((1f64 / source_ml_source2_same_value_operand25702))))) || ((source_ml_source2_same_value_operand25701 != source_ml_source2_same_value_operand25701) && (source_ml_source2_same_value_operand25702 != source_ml_source2_same_value_operand25702)))
                                                                                }
                                                                            }) || (shared_constructor_field9 > ({ let ml_reference_array1380 = (((match collection.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, source_ml_source2_data_property_value25707) => {
                                                                                    Some((source_ml_source2_data_property_value25707.clone(),))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, source_ml_source2_data_property_value25708) => {
                                                                                    Some((source_ml_source2_data_property_value25708.clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone(); let ml_reference_projection1380 = ml_reference_array1380.borrow().len() as f64; ml_reference_projection1380 }))) {
                                                                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114, 69, 97, 99, 104, 32, 99, 117, 114, 115, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 45, 104, 105, 115, 116, 111, 114, 121, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                                                                            } else {
                                                                                if !(self::ml_array::includes(&{ let ml_reference_value1381 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107])]))).borrow().clone(); ml_reference_value1381 }, &shared_constructor_field10.clone(), 0f64, false, |a, b| a == b)) {
                                                                                    self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114, 69, 97, 99, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 112, 104, 97, 115, 101])
                                                                                } else {
                                                                                    if (shared_constructor_field10 == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]))) {
                                                                                        if ((!(match (match state.clone() {
                                                                                            self::JavaScriptCollectionForEachState::CollectionForEachState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value25712) => {
                                                                                                ((source_ml_source2_data_property_value25712.clone().unwrap_or_else(|| (None::<self::JavaScriptCollectionForEachCall>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || {
                                                                                            let source_ml_source2_s25714 = shared_constructor_field11.clone();
                                                                                            match source_ml_source2_s25714.clone() {
                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                    false
                                                                                                }
                                                                                                _ => {
                                                                                                    true
                                                                                                }
                                                                                            }
                                                                                        }) || {
                                                                                            let source_ml_source2_s25717 = shared_constructor_field12.clone();
                                                                                            match source_ml_source2_s25717.clone() {
                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                    false
                                                                                                }
                                                                                                _ => {
                                                                                                    true
                                                                                                }
                                                                                            }
                                                                                        }) {
                                                                                            self::ml_string::Text::from_units(vec![97, 32, 114, 101, 97, 100, 121, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114, 69, 97, 99, 104, 32, 104, 97, 115, 32, 110, 111, 32, 115, 116, 97, 108, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 32, 111, 114, 32, 104, 101, 108, 100, 32, 101, 110, 116, 114, 121])
                                                                                        } else {
                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                        }
                                                                                    } else {
                                                                                        if ((shared_constructor_field9 == (0f64)) || match (match state.clone() {
                                                                                            self::JavaScriptCollectionForEachState::CollectionForEachState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value25721) => {
                                                                                                ((source_ml_source2_data_property_value25721.clone().unwrap_or_else(|| (None::<self::JavaScriptCollectionForEachCall>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                            self::ml_string::Text::from_units(vec![97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114, 69, 97, 99, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 108, 114, 101, 97, 100, 121, 45, 97, 100, 118, 97, 110, 99, 101, 100, 32, 111, 114, 100, 105, 110, 97, 108, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108])
                                                                                        } else {
                                                                                            {
                                                                                                let call = (match state.clone() {
                                                                                                    self::JavaScriptCollectionForEachState::CollectionForEachState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value25722) => {
                                                                                                        ((source_ml_source2_data_property_value25722.clone().unwrap_or_else(|| (None::<self::JavaScriptCollectionForEachCall>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().expect("a proven present value");
                                                                                                match call.clone() {
                                                                                                    self::JavaScriptCollectionForEachCall::CollectionForEachSelectedCall(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                                                        let shared_constructor_field1 = *shared_constructor_field1;
                                                                                                        let shared_constructor_field2 = *shared_constructor_field2;
                                                                                                        {
                                                                                                            let arguments_ = shared_constructor_field3.clone();
                                                                                                            if (((match call.clone() {
                                                                                                                self::JavaScriptCollectionForEachCall::CollectionForEachSelectedCall(_, _, _) => {
                                                                                                                    false
                                                                                                                }
                                                                                                            } || (({ let ml_reference_array1382 = arguments_.clone(); let ml_reference_projection1382 = ml_reference_array1382.borrow().len() as f64; ml_reference_projection1382 }) != (3f64))) || !(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), shared_constructor_field7.clone()))) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), shared_constructor_field8.clone()))) {
                                                                                                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114, 69, 97, 99, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 98, 97, 99, 107, 44, 32, 116, 104, 105, 115, 65, 114, 103, 117, 109, 101, 110, 116, 32, 97, 110, 100, 32, 116, 104, 114, 101, 101, 32, 111, 112, 101, 114, 97, 110, 100, 115])
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let expected = std::rc::Rc::new(std::cell::RefCell::new(vec![shared_constructor_field12.clone(), if (kind == (self::ml_string::Text::from_units(vec![83, 101, 116]))) {
                                                                                                                        shared_constructor_field12.clone()
                                                                                                                    } else {
                                                                                                                        shared_constructor_field11.clone()
                                                                                                                    }, shared_constructor_field6.clone()]));
                                                                                                                    {
                                                                                                                        let at = 0f64;
                                                                                                                        {
                                                                                                                            let source_ml_source2_s25737 = self::source_ml_source1_temporary_read_java_script_collection_for_each_state_error_loop1180(arguments_.clone(), expected.clone(), at);
                                                                                                                            match source_ml_source2_s25737.clone() {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Result::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Done => {
                                                                                                                                    if ((kind == (self::ml_string::Text::from_units(vec![83, 101, 116]))) && !(self::test_java_script_semantic_same_value(shared_constructor_field11.clone(), shared_constructor_field12.clone()))) {
                                                                                                                                        self::ml_string::Text::from_units(vec![83, 101, 116, 32, 102, 111, 114, 69, 97, 99, 104, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 105, 110, 32, 98, 111, 116, 104, 32, 107, 101, 121, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115])
                                                                                                                                    } else {
                                                                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Result::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Return(source_ml_source1_temporary_result1180) => {
                                                                                                                                    source_ml_source1_temporary_result1180.clone()
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
                                                            self::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Result::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Return(source_ml_source1_temporary_result1180_2) => {
                                                                source_ml_source1_temporary_result1180_2.clone()
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
pub fn begin_java_script_collection_for_each(
    heap: self::JavaScriptOrdinaryHeap,
    operation: &self::ml_string::Text,
    receiver: self::JavaScriptSemanticValue,
    callback: self::JavaScriptSemanticValue,
    this_argument: self::JavaScriptSemanticValue,
) -> self::JavaScriptCollectionForEachTransition {
    if !(self::test_java_script_collection_for_each_operation(operation)) {
        self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(
            Box::new(heap.clone()),
            self::ml_string::Text::from_units(vec![
                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114, 69, 97, 99, 104,
                32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 111, 112, 101, 114, 97,
                116, 105, 111, 110,
            ]),
        )
    } else {
        {
            let kind = self::read_java_script_collection_for_each_kind(operation);
            {
                let checked = self::read_java_script_collection_iterator_receiver_error(
                    heap.clone(),
                    receiver.clone(),
                    &(kind),
                );
                if (((({
                    let ml_reference_value1383 = checked.borrow().clone();
                    ml_reference_value1383
                })
                .0)
                    .0)
                    != (self::ml_string::Text::from_units(vec![])))
                {
                    if (({
                        let ml_reference_value1384 = checked.borrow().clone();
                        ml_reference_value1384
                    })
                    .0)
                        .1
                    {
                        self::JavaScriptCollectionForEachTransition::CollectionForEachError(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                            ]),
                            operation.concat(&self::ml_string::Text::from_units(vec![
                                32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32,
                                97, 99, 116, 117, 97, 108, 32, 99, 111, 108, 108, 101, 99, 116,
                                105, 111, 110, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115,
                                108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114,
                            ])),
                        )
                    } else {
                        self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(
                            Box::new(heap.clone()),
                            (({
                                let ml_reference_value1385 = checked.borrow().clone();
                                ml_reference_value1385
                            })
                            .0)
                                .0,
                        )
                    }
                } else {
                    {
                        let source_ml_source1_temporary_values1181 =
                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                callback.clone(),
                                this_argument.clone(),
                            ]));
                        {
                            let source_ml_source1_temporary_index1181 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s25740 = self::source_ml_source1_temporary_begin_java_script_collection_for_each_loop1182(heap.clone(), source_ml_source1_temporary_values1181.clone(), source_ml_source1_temporary_index1181.clone());
                                match source_ml_source2_s25740.clone() {
                                    self::MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Result::MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Done => {
                                        if (match callback.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        } || !(((match callback.clone() {
                                            self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value25743) => {
                                                Some((source_ml_source2_data_property_value25743,))
                                            }
                                            _ => {
                                                None::<(bool,)>
                                            }
                                        }).clone().expect("a proven present value")).0)) {
                                            self::JavaScriptCollectionForEachTransition::CollectionForEachError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114, 69, 97, 99, 104, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101]))
                                        } else {
                                            self::JavaScriptCollectionForEachTransition::CollectionForEachContinue(Box::new(heap.clone()), Box::new({
                                                let source_ml_source2_constructor_field25748 = 0f64;
                                                {
                                                    let source_ml_source2_constructor_field25750 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_constructor_field25751 = self::JavaScriptSemanticValue::Undefined;
                                                        self::JavaScriptCollectionForEachState::CollectionForEachState(operation.clone(), Box::new(receiver.clone()), Box::new(callback.clone()), Box::new(this_argument.clone()), source_ml_source2_constructor_field25748, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), Box::new(source_ml_source2_constructor_field25750.clone()), Box::new(source_ml_source2_constructor_field25751.clone()), None::<(Option<self::JavaScriptCollectionForEachCall>,)>)
                                                    }
                                                }
                                            }))
                                        }
                                    }
                                    self::MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Result::MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Return(source_ml_source1_temporary_result1182) => {
                                        let source_ml_source1_temporary_result1182 = *source_ml_source1_temporary_result1182;
                                        source_ml_source1_temporary_result1182.clone()
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
pub fn step_java_script_collection_for_each(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptCollectionForEachState,
) -> self::JavaScriptCollectionForEachTransition {
    match state.clone() {
        self::JavaScriptCollectionForEachState::CollectionForEachState(
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field3,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error = self::read_java_script_collection_for_each_state_error(
                        heap.clone(),
                        state.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    } else {
                        if !(match (
                            match state.clone() {
                                self::JavaScriptCollectionForEachState::CollectionForEachState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value25752,
                                ) => ((source_ml_source2_data_property_value25752
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptCollectionForEachCall>,)
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
                            self::JavaScriptCollectionForEachTransition::CollectionForEachCall(Box::new(heap.clone()), Box::new(state.clone()), Box::new(match (match state.clone() {
                                    self::JavaScriptCollectionForEachState::CollectionForEachState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value25753) => {
                                        ((source_ml_source2_data_property_value25753.clone().unwrap_or_else(|| (None::<self::JavaScriptCollectionForEachCall>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value") {
                                    self::JavaScriptCollectionForEachCall::CollectionForEachSelectedCall(source_ml_source2_data_property_value25754, _, _) => {
                                        let source_ml_source2_data_property_value25754 = *source_ml_source2_data_property_value25754;
                                        source_ml_source2_data_property_value25754.clone()
                                    }
                                }), Box::new(match (match state.clone() {
                                    self::JavaScriptCollectionForEachState::CollectionForEachState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value25755) => {
                                        ((source_ml_source2_data_property_value25755.clone().unwrap_or_else(|| (None::<self::JavaScriptCollectionForEachCall>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value") {
                                    self::JavaScriptCollectionForEachCall::CollectionForEachSelectedCall(_, source_ml_source2_data_property_value25756, _) => {
                                        let source_ml_source2_data_property_value25756 = *source_ml_source2_data_property_value25756;
                                        source_ml_source2_data_property_value25756.clone()
                                    }
                                }), match (match state.clone() {
                                    self::JavaScriptCollectionForEachState::CollectionForEachState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value25757) => {
                                        ((source_ml_source2_data_property_value25757.clone().unwrap_or_else(|| (None::<self::JavaScriptCollectionForEachCall>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value") {
                                    self::JavaScriptCollectionForEachCall::CollectionForEachSelectedCall(_, _, source_ml_source2_data_property_value25758) => {
                                        source_ml_source2_data_property_value25758.clone()
                                    }
                                })
                        } else {
                            {
                                let collection = if {
                                    let source_ml_source2_s25760 =
                                        shared_constructor_field5.clone();
                                    match source_ml_source2_s25760.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => true,
                                        _ => false,
                                    }
                                } {
                                    {
                                        let source_ml_source2_array_read_value25768 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_array_read_index25769 = self::find_java_script_ordinary_object(shared_constructor_field3.clone(), (((match shared_constructor_field5.clone() {
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value25767, _) => {
                                                        Some((source_ml_source2_data_property_value25767.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml::Big,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone());
                                            {
                                                let ml_reference_array1386 =
                                                    source_ml_source2_array_read_value25768.clone();
                                                let ml_reference_position1386 =
                                                    source_ml_source2_array_read_index25769;
                                                let ml_reference_projection1386 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1386.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1386,
                                                        ),
                                                    );
                                                ml_reference_projection1386
                                            }
                                        }
                                    }
                                } else {
                                    None::<self::JavaScriptOrdinaryObject>
                                };
                                if (match (collection.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } || (match collection.clone().expect("a proven present value")
                                {
                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                        false
                                    }
                                    _ => true,
                                } && match collection
                                    .clone()
                                    .expect("a proven present value")
                                {
                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                        false
                                    }
                                    _ => true,
                                })) {
                                    self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114, 69, 97, 99, 104, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 116, 97, 105, 110, 101, 100, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 115, 111, 117, 114, 99, 101]))
                                } else {
                                    if (shared_constructor_field8
                                        == ({
                                            let ml_reference_array1387 =
                                                (((match collection
                                                    .clone()
                                                    .expect("a proven present value")
                                                {
                                                    self::JavaScriptOrdinaryObject::MapObject(
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        source_ml_source2_data_property_value25786,
                                                    ) => Some((
                                                        source_ml_source2_data_property_value25786
                                                            .clone(),
                                                    )),
                                                    self::JavaScriptOrdinaryObject::SetObject(
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        source_ml_source2_data_property_value25787,
                                                    ) => Some((
                                                        source_ml_source2_data_property_value25787
                                                            .clone(),
                                                    )),
                                                    _ => None::<(
                                                        std::rc::Rc<
                                                            std::cell::RefCell<
                                                                Vec<
                                                                    self::JavaScriptCollectionEntry,
                                                                >,
                                                            >,
                                                        >,
                                                    )>,
                                                })
                                                .clone()
                                                .expect("a proven present value"))
                                                .0)
                                                    .clone();
                                            let ml_reference_projection1387 =
                                                ml_reference_array1387.borrow().len() as f64;
                                            ml_reference_projection1387
                                        }))
                                    {
                                        self::JavaScriptCollectionForEachTransition::CollectionForEachCompleted(Box::new(heap.clone()), Box::new(self::JavaScriptSemanticValue::Undefined))
                                    } else {
                                        {
                                            let entry = {
                                                let source_ml_source2_array_read_value25791 = (((match collection.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, source_ml_source2_data_property_value25788) => {
                                                            Some((source_ml_source2_data_property_value25788.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, source_ml_source2_data_property_value25789) => {
                                                            Some((source_ml_source2_data_property_value25789.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone();
                                                {
                                                    let source_ml_source2_array_read_index25792 =
                                                        shared_constructor_field8;
                                                    {
                                                        let ml_reference_array1388 =
                                                            source_ml_source2_array_read_value25791
                                                                .clone();
                                                        let ml_reference_position1388 =
                                                            source_ml_source2_array_read_index25792;
                                                        let ml_reference_projection1388 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array1388.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position1388,
                                                                ),
                                                            );
                                                        ml_reference_projection1388
                                                    }
                                                }
                                            };
                                            if match (entry.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114, 69, 97, 99, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 110, 115, 101, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                                            } else {
                                                {
                                                    let cursor = (shared_constructor_field8 + 1f64);
                                                    if !(match entry.clone().expect("a proven present value") {
                                                            self::JavaScriptCollectionEntry::CollectionEntry(_, _, source_ml_source2_data_property_value25794) => {
                                                                source_ml_source2_data_property_value25794
                                                            }
                                                        }) {
                                                            self::JavaScriptCollectionForEachTransition::CollectionForEachContinue(Box::new(heap.clone()), Box::new({
                                                                let source_ml_source2_nominal_field25796 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field25798 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field25800 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field25802 = shared_constructor_field7.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field25804 = shared_constructor_field8;
                                                                                {
                                                                                    let source_ml_source2_nominal_field25806 = shared_constructor_field9.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field25808 = shared_constructor_field10.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field25810 = shared_constructor_field11.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field25812 = shared_constructor_field12.clone();
                                                                                                self::JavaScriptCollectionForEachState::CollectionForEachState(source_ml_source2_nominal_field25796.clone(), Box::new(source_ml_source2_nominal_field25798.clone()), Box::new(source_ml_source2_nominal_field25800.clone()), Box::new(source_ml_source2_nominal_field25802.clone()), cursor, source_ml_source2_nominal_field25806.clone(), Box::new(source_ml_source2_nominal_field25808.clone()), Box::new(source_ml_source2_nominal_field25810.clone()), source_ml_source2_nominal_field25812.clone())
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
                                                                let key = match entry.clone().expect("a proven present value") {
                                                                    self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value25815, _, _) => {
                                                                        let source_ml_source2_data_property_value25815 = *source_ml_source2_data_property_value25815;
                                                                        source_ml_source2_data_property_value25815.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let value = if match collection.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                            true
                                                                        }
                                                                        _ => {
                                                                            false
                                                                        }
                                                                    } {
                                                                        match entry.clone().expect("a proven present value") {
                                                                            self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value25822, _, _) => {
                                                                                let source_ml_source2_data_property_value25822 = *source_ml_source2_data_property_value25822;
                                                                                source_ml_source2_data_property_value25822.clone()
                                                                            }
                                                                        }
                                                                    } else {
                                                                        match entry.clone().expect("a proven present value") {
                                                                            self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source2_data_property_value25823, _) => {
                                                                                let source_ml_source2_data_property_value25823 = *source_ml_source2_data_property_value25823;
                                                                                source_ml_source2_data_property_value25823.clone()
                                                                            }
                                                                        }
                                                                    };
                                                                    {
                                                                        let arguments_ = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone(), if match collection.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        } {
                                                                            value.clone()
                                                                        } else {
                                                                            key.clone()
                                                                        }, shared_constructor_field5.clone()]));
                                                                        {
                                                                            let call = self::JavaScriptCollectionForEachCall::CollectionForEachSelectedCall(Box::new(shared_constructor_field6.clone()), Box::new(shared_constructor_field7.clone()), arguments_.clone());
                                                                            match call.clone() {
                                                                                self::JavaScriptCollectionForEachCall::CollectionForEachSelectedCall(shared_constructor_field1, shared_constructor_field2, _) => {
                                                                                    let shared_constructor_field1 = *shared_constructor_field1;
                                                                                    let shared_constructor_field2 = *shared_constructor_field2;
                                                                                    self::JavaScriptCollectionForEachTransition::CollectionForEachCall(Box::new(heap.clone()), Box::new({
                                                                                        let source_ml_source2_nominal_field25834 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field25836 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field25838 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field25840 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field25842 = shared_constructor_field8;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field25844 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field25846 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field25848 = shared_constructor_field11.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field25850 = shared_constructor_field12.clone();
                                                                                                                        self::JavaScriptCollectionForEachState::CollectionForEachState(source_ml_source2_nominal_field25834.clone(), Box::new(source_ml_source2_nominal_field25836.clone()), Box::new(source_ml_source2_nominal_field25838.clone()), Box::new(source_ml_source2_nominal_field25840.clone()), cursor, self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107]), Box::new(key.clone()), Box::new(value.clone()), Some((Some(call.clone()),)))
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }), Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), arguments_.clone())
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
