// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptObjectIntegrityTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195;
use super::MlSource1TemporaryBeginJavaScriptObjectObservationLoop1196Result;
use super::find_java_script_ordinary_object;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::observe_java_script_boxed_primitive;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_begin_java_script_object_observation_loop1196;
use super::test_java_script_object_observation_operation;
use super::test_java_script_ordinary_integrity_kind;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_object_observation(
    heap: self::JavaScriptOrdinaryHeap,
    operation: &self::ml_string::Text,
    input: self::JavaScriptSemanticValue,
    other: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectIntegrityTransition {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = {
                let source_ml_source2_logical26464 = {
                    let source_ml_source2_logical26463 =
                        self::read_java_script_ordinary_heap_error(heap.clone());
                    if !source_ml_source2_logical26463.0.is_empty() {
                        source_ml_source2_logical26463.clone()
                    } else {
                        self::read_java_script_ordinary_value_error(heap.clone(), input.clone())
                    }
                };
                if !source_ml_source2_logical26464.0.is_empty() {
                    source_ml_source2_logical26464.clone()
                } else {
                    self::read_java_script_ordinary_value_error(heap.clone(), other.clone())
                }
            };
            if ((error != (self::ml_string::Text::from_units(vec![])))
                || !(self::test_java_script_object_observation_operation(operation)))
            {
                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(
                    Box::new(heap.clone()),
                    if !error.0.is_empty() {
                        error.clone()
                    } else {
                        self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 32, 111, 98, 115, 101, 114, 118, 97, 116,
                            105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116,
                            104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103,
                            105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116,
                            105, 111, 110,
                        ])
                    },
                )
            } else {
                if ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        79, 98, 106, 101, 99, 116, 46, 98, 117, 105, 108, 116, 105, 110, 84, 97,
                        103,
                    ])))
                {
                    match input.clone() {
                        self::JavaScriptSemanticValue::Object(identity, _) => {
                            let object = {
                                let source_ml_source2_array_read_value26469 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_array_read_index26470 =
                                        self::find_java_script_ordinary_object(
                                            shared_constructor_field1.clone(),
                                            identity.clone(),
                                        );
                                    {
                                        let ml_reference_array1431 =
                                            source_ml_source2_array_read_value26469.clone();
                                        let ml_reference_position1431 =
                                            source_ml_source2_array_read_index26470;
                                        let ml_reference_projection1431 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array1431.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position1431,
                                                ),
                                            );
                                        ml_reference_projection1431
                                    }
                                }
                            };
                            if match (object.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 32,
                                        105, 110, 115, 112, 101, 99, 116, 105, 111, 110, 32, 114,
                                        101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108,
                                        32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108,
                                        111, 116, 115,
                                    ]),
                                )
                            } else {
                                {
                                    let tag = self::ml_string::Text::from_units(vec![
                                        79, 98, 106, 101, 99, 116,
                                    ]);
                                    {
                                        let source_ml_source2_s26601 = if (match object
                                            .clone()
                                            .expect("a proven present value")
                                        {
                                            self::JavaScriptOrdinaryObject::ArrayObject(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => true,
                                            _ => false,
                                        } || match object
                                            .clone()
                                            .expect("a proven present value")
                                        {
                                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => true,
                                            _ => false,
                                        }) {
                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))
                                        } else {
                                            if ((match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value26614, _, _, _, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value26614,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value26615, _, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value26615,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value26616, _, _, _, _, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value26616,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value26617, _, _) => {
                                                            Some((source_ml_source2_data_property_value26617,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value26618, _, _) => {
                                                            Some((source_ml_source2_data_property_value26618,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value26619, _, _) => {
                                                            Some((source_ml_source2_data_property_value26619,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value26620, _, _) => {
                                                            Some((source_ml_source2_data_property_value26620,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value26621, _) => {
                                                            Some((source_ml_source2_data_property_value26621,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value26622, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value26622,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value26623, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value26623,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value26624, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value26624,))
                                                        }
                                                        _ => {
                                                            None::<(bool,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0 {
                                                        self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110]))
                                                    } else {
                                                        match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]))
                                                            }
                                                            _ => {
                                                                match object.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                        if !(self::ml_array::includes(&{ let ml_reference_value1432 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value1432 }, &self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), 0f64, false, |a, b| a == b)) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                        } else {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                        }
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                        if !(self::ml_array::includes(&{ let ml_reference_value1433 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value1433 }, &self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), 0f64, false, |a, b| a == b)) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                        } else {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                        }
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                        if !(self::ml_array::includes(&{ let ml_reference_value1434 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value1434 }, &self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116]), 0f64, false, |a, b| a == b)) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                        } else {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                        }
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                        if !(self::ml_array::includes(&{ let ml_reference_value1435 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value1435 }, &self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), 0f64, false, |a, b| a == b)) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                        } else {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                        }
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                        if !(self::ml_array::includes(&{ let ml_reference_value1436 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value1436 }, &self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), 0f64, false, |a, b| a == b)) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                        } else {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                        }
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                        if !(self::ml_array::includes(&{ let ml_reference_value1437 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value1437 }, &self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), 0f64, false, |a, b| a == b)) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                        } else {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                        }
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                        if !(self::ml_array::includes(&{ let ml_reference_value1438 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value1438 }, &self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), 0f64, false, |a, b| a == b)) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                        } else {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                        }
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                        if !(self::ml_array::includes(&{ let ml_reference_value1439 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value1439 }, &self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), 0f64, false, |a, b| a == b)) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                        } else {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                        }
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, regular_expression, string_data, primitive_data, _, _) => {
                                                                        if !(match (((string_data.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))
                                                                        } else {
                                                                            if {
                                                                                let source_ml_source2_comparison_operand26725 = {
                                                                                    let source_ml_source2_optional_chain_receiver26724 = (((primitive_data.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone(),);
                                                                                    if match (source_ml_source2_optional_chain_receiver26724.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                        Some({
                                                                                            let source_ml_source1_temporary_optional_chain1192 = ((source_ml_source2_optional_chain_receiver26724.clone()).0).clone().expect("a proven present value");
                                                                                            match source_ml_source1_temporary_optional_chain1192.clone() {
                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                    self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                                    self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                                                                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Number(_) => {
                                                                                                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                                                                    self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Text(_) => {
                                                                                                    self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                                                    self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                                                                    self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                    self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                                                                                                }
                                                                                            }
                                                                                        })
                                                                                    } else {
                                                                                        None::<self::ml_string::Text>
                                                                                    }
                                                                                };
                                                                                match (source_ml_source2_comparison_operand26725.clone(), self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                            } {
                                                                                self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110]))
                                                                            } else {
                                                                                if {
                                                                                    let source_ml_source2_comparison_operand26728 = {
                                                                                        let source_ml_source2_optional_chain_receiver26727 = (((primitive_data.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone(),);
                                                                                        if match (source_ml_source2_optional_chain_receiver26727.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                            Some({
                                                                                                let source_ml_source1_temporary_optional_chain1193 = ((source_ml_source2_optional_chain_receiver26727.clone()).0).clone().expect("a proven present value");
                                                                                                match source_ml_source1_temporary_optional_chain1193.clone() {
                                                                                                    self::JavaScriptSemanticValue::Undefined => {
                                                                                                        self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
                                                                                                    }
                                                                                                    self::JavaScriptSemanticValue::Null => {
                                                                                                        self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                                                                                                    }
                                                                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                                                                        self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                                                                                                    }
                                                                                                    self::JavaScriptSemanticValue::Number(_) => {
                                                                                                        self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                                                                                                    }
                                                                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                                                                        self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
                                                                                                    }
                                                                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                                                                        self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                                                                                                    }
                                                                                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])
                                                                                                    }
                                                                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                                                                        self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                                                                                                    }
                                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                }
                                                                                            })
                                                                                        } else {
                                                                                            None::<self::ml_string::Text>
                                                                                        }
                                                                                    };
                                                                                    match (source_ml_source2_comparison_operand26728.clone(), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                } {
                                                                                    self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]))
                                                                                } else {
                                                                                    if {
                                                                                        let source_ml_source2_comparison_operand26730 = ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                                                        match (source_ml_source2_comparison_operand26730.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                    } {
                                                                                        self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(self::ml_string::Text::from_units(vec![82, 101, 103, 69, 120, 112]))
                                                                                    } else {
                                                                                        self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                        if !(self::ml_array::includes(&{ let ml_reference_value1440 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value1440 }, &self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116]), 0f64, false, |a, b| a == b)) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                        } else {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                        }
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                        if !(self::ml_array::includes(&{ let ml_reference_value1441 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value1441 }, &self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), 0f64, false, |a, b| a == b)) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                        } else {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                        }
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                        if !(self::ml_array::includes(&{ let ml_reference_value1442 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value1442 }, &self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116]), 0f64, false, |a, b| a == b)) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                        } else {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                        }
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                        if !(self::ml_array::includes(&{ let ml_reference_value1443 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value1443 }, &self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116]), 0f64, false, |a, b| a == b)) {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                        } else {
                                                                            self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag.clone())
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                        };
                                        match source_ml_source2_s26601.clone() {
                                                    self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(tag_2) => {
                                                        {
                                                            let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                            {
                                                                let index = 0f64;
                                                                {
                                                                    let source_ml_source2_s26732 = self::source_ml_source1_temporary_begin_java_script_object_observation_loop1196(&(tag_2), units.clone(), index);
                                                                    match source_ml_source2_s26732.clone() {
                                                                        self::MlSource1TemporaryBeginJavaScriptObjectObservationLoop1196Result::MlSource1TemporaryBeginJavaScriptObjectObservationLoop1196Done => {
                                                                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(Box::new(heap.clone()), Box::new(self::JavaScriptSemanticValue::Text(units.clone())))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195::MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(source_ml_source1_temporary_result1197) => {
                                                        let source_ml_source1_temporary_result1197 = *source_ml_source1_temporary_result1197;
                                                        source_ml_source1_temporary_result1197.clone()
                                                    }
                                                }
                                    }
                                }
                            }
                        }
                        _ => self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                98, 117, 105, 108, 116, 105, 110, 32, 116, 97, 103, 32, 105, 110,
                                115, 112, 101, 99, 116, 105, 111, 110, 32, 102, 111, 108, 108, 111,
                                119, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 79, 98, 106,
                                101, 99, 116,
                            ]),
                        ),
                    }
                } else {
                    if ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 46, 114, 101, 103, 117, 108, 97, 114, 69,
                            120, 112, 114, 101, 115, 115, 105, 111, 110, 83, 108, 111, 116,
                        ])))
                    {
                        {
                            let object_2 = match input.clone() {
                                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                    let source_ml_source2_array_read_value26736 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_array_read_index26737 =
                                            self::find_java_script_ordinary_object(
                                                shared_constructor_field1.clone(),
                                                identity_2.clone(),
                                            );
                                        {
                                            let ml_reference_array1444 =
                                                source_ml_source2_array_read_value26736.clone();
                                            let ml_reference_position1444 =
                                                source_ml_source2_array_read_index26737;
                                            let ml_reference_projection1444 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array1444.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position1444,
                                                    ),
                                                );
                                            ml_reference_projection1444
                                        }
                                    }
                                }
                                _ => None::<self::JavaScriptOrdinaryObject>,
                            };
                            if (!(match (object_2.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }) && !(self::test_java_script_ordinary_integrity_kind(
                                &(match object_2.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
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
                                    ) => self::ml_string::Text::from_units(vec![
                                        99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111,
                                        110, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![
                                            97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
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
                                    ) => self::ml_string::Text::from_units(vec![
                                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116,
                                        101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![
                                            109, 97, 112, 79, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![
                                            115, 101, 116, 79, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101,
                                        99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106,
                                        101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106,
                                        101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102,
                                        101, 114, 79, 98, 106, 101, 99, 116,
                                    ]),
                                }),
                            ))) {
                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        111, 112, 97, 113, 117, 101, 32, 73, 115, 82, 101, 103, 69,
                                        120, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                                        105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110,
                                        116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32,
                                        112, 114, 111, 116, 111, 99, 111, 108,
                                    ]),
                                )
                            } else {
                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(
                                    Box::new(heap.clone()),
                                    Box::new(self::JavaScriptSemanticValue::Boolean(
                                        ({
                                            let source_ml_source2_comparison_operand26739 = {
                                                let source_ml_source2_optional_chain_receiver26738 =
                                                    (object_2.clone(),);
                                                if match (source_ml_source2_optional_chain_receiver26738.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                Some({
                                                    let source_ml_source1_temporary_optional_chain1194 = ((source_ml_source2_optional_chain_receiver26738.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_temporary_optional_chain1194.clone() {
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
                                                    }
                                                })
                                            } else {
                                                None::<self::ml_string::Text>
                                            }
                                            };
                                            match (
                                                source_ml_source2_comparison_operand26739.clone(),
                                                self::ml_string::Text::from_units(vec![
                                                    111, 114, 100, 105, 110, 97, 114, 121, 79, 98,
                                                    106, 101, 99, 116,
                                                ]),
                                            ) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            }
                                        } && {
                                            let source_ml_source2_comparison_operand26744 = {
                                                let source_ml_source2_nullish_field_receiver26741 =
                                                    (object_2.clone(),);
                                                if match (source_ml_source2_nullish_field_receiver26741.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present26742 = ((source_ml_source2_nullish_field_receiver26741.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present26742.clone() {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value26743, _, _, _, _) => {
                                                            ((source_ml_source2_data_property_value26743.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                                        }
                                                        _ => {
                                                            None::<bool>
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 117, 108, 97, 114, 69, 120, 112, 114, 101, 115, 115, 105, 111, 110, 39, 41])).0, message_present: true }).raise()
                                            }
                                            };
                                            match (
                                                source_ml_source2_comparison_operand26744.clone(),
                                                true,
                                            ) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            }
                                        }),
                                    )),
                                )
                            }
                        }
                    } else {
                        self::observe_java_script_boxed_primitive(
                            heap.clone(),
                            operation,
                            input.clone(),
                            other.clone(),
                        )
                    }
                }
            }
        }
    }
}
