// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::find_java_script_heap_directory_object;
use super::find_java_script_heap_directory_property;
use super::ml_array;
use super::ml_string;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_class_receiver_error(
    heap: self::JavaScriptOrdinaryHeap,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            match callee.clone() {
                self::JavaScriptSemanticValue::Object(callee_identity, callee_callable) => {
                    if !callee_callable {
                        self::ml_string::Text::from_units(vec![
                            111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32,
                            99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101,
                            101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                            99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 110, 100, 32, 110, 111,
                            110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 108, 108, 111, 99, 97,
                            116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114,
                        ])
                    } else {
                        match receiver.clone() {
                            self::JavaScriptSemanticValue::Object(identity, callable) => {
                                if callable {
                                    self::ml_string::Text::from_units(vec![
                                        111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97,
                                        115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116,
                                        105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116,
                                        115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108,
                                        97, 98, 108, 101, 32, 97, 110, 100, 32, 110, 111, 110, 99,
                                        97, 108, 108, 97, 98, 108, 101, 32, 97, 108, 108, 111, 99,
                                        97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101,
                                        114,
                                    ])
                                } else {
                                    {
                                        let constructor = {
                                            let source_ml_source2_array_read_value25147 =
                                                shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_array_read_index25148 =
                                                    self::find_java_script_heap_directory_object(
                                                        shared_constructor_field1.clone(),
                                                        callee_identity.clone(),
                                                    );
                                                {
                                                    let ml_reference_array1325 =
                                                        source_ml_source2_array_read_value25147
                                                            .clone();
                                                    let ml_reference_position1325 =
                                                        source_ml_source2_array_read_index25148;
                                                    let ml_reference_projection1325 =
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array1325.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position1325,
                                                            ),
                                                        );
                                                    ml_reference_projection1325
                                                }
                                            }
                                        };
                                        {
                                            let instance = {
                                                let source_ml_source2_array_read_value25151 =
                                                    shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_array_read_index25152 = self::find_java_script_heap_directory_object(shared_constructor_field1.clone(), identity.clone());
                                                    {
                                                        let ml_reference_array1326 =
                                                            source_ml_source2_array_read_value25151
                                                                .clone();
                                                        let ml_reference_position1326 =
                                                            source_ml_source2_array_read_index25152;
                                                        let ml_reference_projection1326 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array1326.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position1326,
                                                                ),
                                                            );
                                                        ml_reference_projection1326
                                                    }
                                                }
                                            };
                                            if match (constructor.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::ml_string::Text::from_units(vec![
                                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99,
                                                    108, 97, 115, 115, 32, 99, 111, 110, 115, 116,
                                                    114, 117, 99, 116, 105, 111, 110, 32, 99, 97,
                                                    110, 110, 111, 116, 32, 115, 117, 98, 115, 116,
                                                    105, 116, 117, 116, 101, 32, 111, 112, 97, 113,
                                                    117, 101, 32, 99, 111, 110, 115, 116, 114, 117,
                                                    99, 116, 111, 114, 32, 111, 114, 32, 114, 101,
                                                    99, 101, 105, 118, 101, 114, 32, 107, 105, 110,
                                                    100, 115,
                                                ])
                                            } else {
                                                match constructor.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, properties) => {
                                                        if match (instance.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 111, 112, 97, 113, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 107, 105, 110, 100, 115])
                                                        } else {
                                                            match instance.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, prototype, _, _, _, _, _, _, _) => {
                                                                    let prototype = *prototype;
                                                                    {
                                                                        let field = {
                                                                            let source_ml_source2_array_read_index25170 = self::find_java_script_heap_directory_property(properties.clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![112f64, 114f64, 111f64, 116f64, 111f64, 116f64, 121f64, 112f64, 101f64]))));
                                                                            { let ml_reference_array1327 = properties.clone(); let ml_reference_position1327 = source_ml_source2_array_read_index25170; let ml_reference_projection1327 = self::ml_array::optional_at(&*ml_reference_array1327.borrow(), self::ml_array::number_index(ml_reference_position1327)); ml_reference_projection1327 }
                                                                        };
                                                                        if (((((!(match (field.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (({ let ml_reference_value1328 = (match field.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value25171) => {
                                                                                source_ml_source2_data_property_value25171.clone()
                                                                            }
                                                                        }).borrow().clone(); ml_reference_value1328 }).0).9) && !((({ let ml_reference_value1329 = (match field.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value25172) => {
                                                                                source_ml_source2_data_property_value25172.clone()
                                                                            }
                                                                        }).borrow().clone(); ml_reference_value1329 }).0).5)) && !((({ let ml_reference_value1330 = (match field.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value25173) => {
                                                                                source_ml_source2_data_property_value25173.clone()
                                                                            }
                                                                        }).borrow().clone(); ml_reference_value1330 }).0).7)) && {
                                                                            let source_ml_source2_s25175 = (({ let ml_reference_value1331 = (match field.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value25178) => {
                                                                                    source_ml_source2_data_property_value25178.clone()
                                                                                }
                                                                            }).borrow().clone(); ml_reference_value1331 }).0).8;
                                                                            match source_ml_source2_s25175.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    true
                                                                                }
                                                                                _ => {
                                                                                    false
                                                                                }
                                                                            }
                                                                        }) && self::test_java_script_semantic_same_value((({ let ml_reference_value1332 = (match field.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value25179) => {
                                                                                source_ml_source2_data_property_value25179.clone()
                                                                            }
                                                                        }).borrow().clone(); ml_reference_value1332 }).0).8, prototype.clone())) {
                                                                            self::ml_string::Text::from_units(vec![])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 99, 108, 97, 115, 115, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 119, 110, 32, 100, 97, 116, 97, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 111, 112, 97, 113, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 107, 105, 110, 100, 115])
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 111, 112, 97, 113, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 107, 105, 110, 100, 115])
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => self::ml_string::Text::from_units(vec![
                                111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115,
                                32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32,
                                110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 110, 100,
                                32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 108,
                                108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118,
                                101, 114,
                            ]),
                        }
                    }
                }
                _ => self::ml_string::Text::from_units(vec![
                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111,
                    110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115,
                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98,
                    108, 101, 32, 97, 110, 100, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108,
                    101, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105,
                    118, 101, 114,
                ]),
            }
        }
    }
}
