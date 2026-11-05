// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptSemanticValue;
use super::create_java_script_property_descriptor;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::plan_java_script_property_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_define_java_script_ordinary_property_loop83;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_get(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> (self::ml_string::Text, Vec<self::JavaScriptSemanticValue>, self::JavaScriptSemanticValue, self::ml_string::Text, self::JavaScriptSemanticValue, self::JavaScriptSemanticValue) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                {
                    let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                        heap_error.clone()
                    } else {
                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                            key_error.clone()
                        } else {
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                receiver_error.clone()
                            } else {
                                if ((self::find_java_script_ordinary_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value339, _) => {
                                        source_ml_source2_data_property_value339
                                    }
                                }, identity.clone())) < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                } else {
                                    self::ml_string::Text::from_units(vec![])
                                }
                            }
                        }
                    };
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        {
                            let source_ml_source2_field342 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_field343 = self::JavaScriptSemanticValue::Undefined;
                                {
                                    let source_ml_source2_field345 = Vec::<self::JavaScriptSemanticValue>::new();
                                    (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), source_ml_source2_field345, source_ml_source2_field343.clone(), error.clone(), receiver.clone(), source_ml_source2_field342.clone())
                                }
                            }
                        }
                    } else {
                        {
                            let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                            if !((property.clone()).1) {
                                {
                                    let source_ml_source2_field348 = self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source2_field349 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source2_field351 = Vec::<self::JavaScriptSemanticValue>::new();
                                            (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), source_ml_source2_field351, source_ml_source2_field349.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field348.clone())
                                        }
                                    }
                                }
                            } else {
                                if ((property.clone()).0).9 {
                                    {
                                        let source_ml_source2_field354 = ((property.clone()).0).8;
                                        {
                                            let source_ml_source2_field355 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field357 = Vec::<self::JavaScriptSemanticValue>::new();
                                                (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), source_ml_source2_field357, source_ml_source2_field355.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field354.clone())
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let getter = ((property.clone()).0).4;
                                        match getter.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                {
                                                    let source_ml_source2_field367 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field368 = self::JavaScriptSemanticValue::Undefined;
                                                        {
                                                            let source_ml_source2_field370 = Vec::<self::JavaScriptSemanticValue>::new();
                                                            (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), source_ml_source2_field370, source_ml_source2_field368.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field367.clone())
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                {
                                                    let source_ml_source2_field373 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field376 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field376, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field373.clone())
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                {
                                                    let source_ml_source2_field379 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field382 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field382, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field379.clone())
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Number(_) => {
                                                {
                                                    let source_ml_source2_field385 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field388 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field388, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field385.clone())
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                {
                                                    let source_ml_source2_field391 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field394 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field394, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field391.clone())
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Text(_) => {
                                                {
                                                    let source_ml_source2_field397 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field400 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field400, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field397.clone())
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                {
                                                    let source_ml_source2_field403 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field406 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field406, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field403.clone())
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                {
                                                    let source_ml_source2_field409 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field412 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field412, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field409.clone())
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
pub fn define_java_script_ordinary_property(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool, Vec<self::ml_string::Text>) {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _) => {
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                {
                    let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    {
                        let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                            heap_error.clone()
                        } else {
                            key_error.clone()
                        };
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            {
                                let source_ml_source2_field416 = Vec::<self::ml_string::Text>::new();
                                (error.clone(), heap.clone(), false, source_ml_source2_field416)
                            }
                        } else {
                            if ((proposed.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                                {
                                    let source_ml_source2_field420 = Vec::<self::ml_string::Text>::new();
                                    (self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]), heap.clone(), false, source_ml_source2_field420)
                                }
                            } else {
                                if ((proposed.clone()).5 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).4)) != (self::ml_string::Text::from_units(vec![])))) {
                                    {
                                        let source_ml_source2_field424 = Vec::<self::ml_string::Text>::new();
                                        (self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 103, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]), heap.clone(), false, source_ml_source2_field424)
                                    }
                                } else {
                                    if ((proposed.clone()).7 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).6)) != (self::ml_string::Text::from_units(vec![])))) {
                                        {
                                            let source_ml_source2_field428 = Vec::<self::ml_string::Text>::new();
                                            (self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 115, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]), heap.clone(), false, source_ml_source2_field428)
                                        }
                                    } else {
                                        {
                                            let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                            {
                                                let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(index));
                                                if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    {
                                                        let source_ml_source2_field434 = Vec::<self::ml_string::Text>::new();
                                                        (self::ml_string::Text::from_units(vec![68, 101, 102, 105, 110, 101, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]), heap.clone(), false, source_ml_source2_field434)
                                                    }
                                                } else {
                                                    {
                                                        let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value435) => {
                                                                source_ml_source2_data_property_value435
                                                            }
                                                        }, key.clone());
                                                        {
                                                            let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value436) => {
                                                                    source_ml_source2_data_property_value436
                                                                }
                                                            }, self::ml_array::number_index(at));
                                                            {
                                                                let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false)
                                                                } else {
                                                                    match property.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value437) => {
                                                                            source_ml_source2_data_property_value437.clone()
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let transition = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value438, _, _) => {
                                                                            source_ml_source2_data_property_value438
                                                                        }
                                                                    }, current.clone(), proposed.clone());
                                                                    if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                        {
                                                                            let source_ml_source2_field441 = (transition.clone()).2;
                                                                            {
                                                                                let source_ml_source2_field442 = Vec::<self::ml_string::Text>::new();
                                                                                (source_ml_source2_field441.clone(), heap.clone(), false, source_ml_source2_field442)
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                                                                            {
                                                                                let source_ml_source2_field446 = Vec::<self::ml_string::Text>::new();
                                                                                (self::ml_string::Text::from_units(vec![]), heap.clone(), true, source_ml_source2_field446)
                                                                            }
                                                                        } else {
                                                                            if ((at < (0f64)) && ((((match object.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value448) => {
                                                                                    source_ml_source2_data_property_value448
                                                                                }
                                                                            }).len() as f64)) >= (4096f64))) {
                                                                                {
                                                                                    let source_ml_source2_field452 = Vec::<self::ml_string::Text>::new();
                                                                                    (self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]), heap.clone(), false, source_ml_source2_field452)
                                                                                }
                                                                            } else {
                                                                                {
                                                                                    let count = 0f64;
                                                                                    {
                                                                                        let source_ml_source1_temporary_values82 = shared_constructor_field1;
                                                                                        {
                                                                                            let source_ml_source1_temporary_index82 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let count_2 = self::source_ml_source1_temporary_define_java_script_ordinary_property_loop83(count, source_ml_source1_temporary_values82, source_ml_source1_temporary_index82.clone());
                                                                                                if ((at < (0f64)) && (count_2 >= (16384f64))) {
                                                                                                    {
                                                                                                        let source_ml_source2_field457 = Vec::<self::ml_string::Text>::new();
                                                                                                        (self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]), heap.clone(), false, source_ml_source2_field457)
                                                                                                    }
                                                                                                } else {
                                                                                                    {
                                                                                                        let updated = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), (transition.clone()).0);
                                                                                                        {
                                                                                                            let properties = if (at < (0f64)) {
                                                                                                                {
                                                                                                                    let source_ml_source2_concat_value459 = match object.clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value458) => {
                                                                                                                            source_ml_source2_data_property_value458
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_concat_value460 = vec![updated.clone()];
                                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), source_ml_source2_concat_value459), source_ml_source2_concat_value460)
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                { let ml_values2 = match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value461) => {
                                                                                                                        source_ml_source2_data_property_value461
                                                                                                                    }
                                                                                                                }; let ml_callback2 = { let at = at; let updated = updated.clone(); std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if (position == at) {
                                                                                                                    updated.clone()
                                                                                                                } else {
                                                                                                                    entry.clone()
                                                                                                                } }) }; ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2, (ml_index2 as f64))).collect::<Vec<_>>() }
                                                                                                            };
                                                                                                            {
                                                                                                                let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value462, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value462.clone()
                                                                                                                    }
                                                                                                                }, Box::new(match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value463, _, _, _) => {
                                                                                                                        let source_ml_source2_data_property_value463 = *source_ml_source2_data_property_value463;
                                                                                                                        source_ml_source2_data_property_value463.clone()
                                                                                                                    }
                                                                                                                }), match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value464, _, _) => {
                                                                                                                        source_ml_source2_data_property_value464
                                                                                                                    }
                                                                                                                }, match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value465, _) => {
                                                                                                                        source_ml_source2_data_property_value465
                                                                                                                    }
                                                                                                                }, properties);
                                                                                                                {
                                                                                                                    let source_ml_source2_field466 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                                                                    {
                                                                                                                        let source_ml_source2_field469 = (transition.clone()).3;
                                                                                                                        (self::ml_string::Text::from_units(vec![]), source_ml_source2_field466.clone(), true, source_ml_source2_field469)
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
pub fn create_java_script_ordinary_set_descriptor(value: self::JavaScriptSemanticValue, creation: bool) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field563 = self::JavaScriptSemanticValue::Undefined;
        {
            let source_ml_source2_field565 = self::JavaScriptSemanticValue::Undefined;
            (creation, creation, creation, creation, source_ml_source2_field563.clone(), false, source_ml_source2_field565.clone(), false, value.clone(), true, creation, creation)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_set(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, value: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> (self::ml_string::Text, Vec<self::JavaScriptSemanticValue>, self::JavaScriptSemanticValue, self::ml_string::Text, self::JavaScriptOrdinaryHeap, self::JavaScriptSemanticValue, bool) {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _) => {
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                {
                    let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    {
                        let value_error = self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                            {
                                let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                                    heap_error.clone()
                                } else {
                                    if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                        key_error.clone()
                                    } else {
                                        if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                            value_error.clone()
                                        } else {
                                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                                receiver_error.clone()
                                            } else {
                                                if ((self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone())) < (0f64)) {
                                                    self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![])
                                                }
                                            }
                                        }
                                    }
                                };
                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                    {
                                        let source_ml_source2_field572 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source2_field574 = Vec::<self::JavaScriptSemanticValue>::new();
                                            (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), source_ml_source2_field574, source_ml_source2_field572.clone(), error.clone(), heap.clone(), receiver.clone(), false)
                                        }
                                    }
                                } else {
                                    {
                                        let located = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                        {
                                            let own = if (located.clone()).1 {
                                                (located.clone()).0
                                            } else {
                                                self::create_java_script_ordinary_set_descriptor(self::JavaScriptSemanticValue::Undefined, true)
                                            };
                                            if (own.clone()).5 {
                                                {
                                                    let setter = (own.clone()).6;
                                                    match setter.clone() {
                                                        self::JavaScriptSemanticValue::Undefined => {
                                                            {
                                                                let source_ml_source2_field586 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field588 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                    (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source2_field588, source_ml_source2_field586.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Null => {
                                                            {
                                                                let source_ml_source2_field595 = vec![value.clone()];
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field595, setter.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                            {
                                                                let source_ml_source2_field602 = vec![value.clone()];
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field602, setter.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                            {
                                                                let source_ml_source2_field609 = vec![value.clone()];
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field609, setter.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            {
                                                                let source_ml_source2_field616 = vec![value.clone()];
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field616, setter.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            {
                                                                let source_ml_source2_field623 = vec![value.clone()];
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field623, setter.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            {
                                                                let source_ml_source2_field630 = vec![value.clone()];
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field630, setter.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            {
                                                                let source_ml_source2_field637 = vec![value.clone()];
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field637, setter.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if !((own.clone()).10) {
                                                    {
                                                        let source_ml_source2_field642 = self::JavaScriptSemanticValue::Undefined;
                                                        {
                                                            let source_ml_source2_field644 = Vec::<self::JavaScriptSemanticValue>::new();
                                                            (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source2_field644, source_ml_source2_field642.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                        }
                                                    }
                                                } else {
                                                    match receiver.clone() {
                                                        self::JavaScriptSemanticValue::Undefined => {
                                                            {
                                                                let source_ml_source2_field655 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field657 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                    (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source2_field657, source_ml_source2_field655.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Null => {
                                                            {
                                                                let source_ml_source2_field662 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field664 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                    (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source2_field664, source_ml_source2_field662.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                            {
                                                                let source_ml_source2_field669 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field671 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                    (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source2_field671, source_ml_source2_field669.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                            {
                                                                let source_ml_source2_field676 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field678 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                    (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source2_field678, source_ml_source2_field676.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            {
                                                                let source_ml_source2_field683 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field685 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                    (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source2_field685, source_ml_source2_field683.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            {
                                                                let source_ml_source2_field690 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field692 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                    (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source2_field692, source_ml_source2_field690.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            {
                                                                let source_ml_source2_field697 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field699 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                    (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source2_field699, source_ml_source2_field697.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                                            {
                                                                let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity_2.clone());
                                                                {
                                                                    let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(index));
                                                                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        {
                                                                            let source_ml_source2_field706 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source2_field708 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), source_ml_source2_field708, source_ml_source2_field706.clone(), self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]), heap.clone(), receiver.clone(), false)
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value709) => {
                                                                                    source_ml_source2_data_property_value709
                                                                                }
                                                                            }, key.clone());
                                                                            {
                                                                                let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value710) => {
                                                                                        source_ml_source2_data_property_value710
                                                                                    }
                                                                                }, self::ml_array::number_index(at));
                                                                                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({
                                                                                    let source_ml_source2_nullish_field_receiver711 = (property.clone(),);
                                                                                    if match (source_ml_source2_nullish_field_receiver711.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                        {
                                                                                            let source_ml_source2_nullish_field_present712 = ((source_ml_source2_nullish_field_receiver711.clone()).0).clone().expect("a proven present value");
                                                                                            match source_ml_source2_nullish_field_present712.clone() {
                                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value713) => {
                                                                                                    source_ml_source2_data_property_value713.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                                    }
                                                                                }).9) || !(({
                                                                                    let source_ml_source2_nullish_field_receiver714 = (property.clone(),);
                                                                                    if match (source_ml_source2_nullish_field_receiver714.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                        {
                                                                                            let source_ml_source2_nullish_field_present715 = ((source_ml_source2_nullish_field_receiver714.clone()).0).clone().expect("a proven present value");
                                                                                            match source_ml_source2_nullish_field_present715.clone() {
                                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value716) => {
                                                                                                    source_ml_source2_data_property_value716.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                                    }
                                                                                }).10))) {
                                                                                    {
                                                                                        let source_ml_source2_field721 = self::JavaScriptSemanticValue::Undefined;
                                                                                        {
                                                                                            let source_ml_source2_field723 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                            (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source2_field723, source_ml_source2_field721.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        let proposed = self::create_java_script_ordinary_set_descriptor(value.clone(), match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false });
                                                                                        {
                                                                                            let transition = self::define_java_script_ordinary_property(heap.clone(), identity_2.clone(), key.clone(), proposed.clone());
                                                                                            {
                                                                                                let source_ml_source2_field724 = (transition.clone()).1;
                                                                                                {
                                                                                                    let source_ml_source2_field725 = if (((transition.clone()).0) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                        self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                                    } else {
                                                                                                        self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_field726 = (transition.clone()).0;
                                                                                                        {
                                                                                                            let source_ml_source2_field727 = (transition.clone()).2;
                                                                                                            {
                                                                                                                let source_ml_source2_field728 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                {
                                                                                                                    let source_ml_source2_field730 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                    (source_ml_source2_field725.clone(), source_ml_source2_field730, source_ml_source2_field728.clone(), source_ml_source2_field726.clone(), source_ml_source2_field724.clone(), receiver.clone(), source_ml_source2_field727)
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
pub fn read_java_script_primitive_conversion_spelling(name: &self::ml_string::Text) -> self::JavaScriptSemanticValue {
    if ((*name) == (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) {
        self::JavaScriptSemanticValue::Text(vec![100f64, 101f64, 102f64, 97f64, 117f64, 108f64, 116f64])
    } else {
        if ((*name) == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
            self::JavaScriptSemanticValue::Text(vec![110f64, 117f64, 109f64, 98f64, 101f64, 114f64])
        } else {
            if ((*name) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                self::JavaScriptSemanticValue::Text(vec![115f64, 116f64, 114f64, 105f64, 110f64, 103f64])
            } else {
                if ((*name) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102]))) {
                    self::JavaScriptSemanticValue::Text(vec![118f64, 97f64, 108f64, 117f64, 101f64, 79f64, 102f64])
                } else {
                    if ((*name) == (self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]))) {
                        self::JavaScriptSemanticValue::Text(vec![116f64, 111f64, 83f64, 116f64, 114f64, 105f64, 110f64, 103f64])
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 115, 112, 101, 108, 108, 105, 110, 103, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 110, 97, 109, 101])).0, message_present: true }).raise()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_conversion_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    if {
                        let source_ml_source2_s817 = shared_constructor_field1.clone();
                        match source_ml_source2_s817.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                false
                            }
                            _ => {
                                true
                            }
                        }
                    } {
                        self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                    } else {
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field1.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                receiver_error.clone()
                            } else {
                                if {
                                    let source_ml_source2_s828 = shared_constructor_field2.clone();
                                    match source_ml_source2_s828.clone() {
                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }
                                } {
                                    self::ml_string::Text::from_units(vec![116, 104, 101, 32, 101, 120, 111, 116, 105, 99, 32, 109, 101, 116, 104, 111, 100, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108])
                                } else {
                                    {
                                        let key_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field2.clone());
                                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                            key_error.clone()
                                        } else {
                                            if (((shared_constructor_field3 != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) && (shared_constructor_field3 != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) && (shared_constructor_field3 != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                                                self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116])
                                            } else {
                                                if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])], &shared_constructor_field4.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 109, 101, 116, 104, 111, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![])
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
pub fn continue_java_script_primitive_conversion(state: self::JavaScriptPrimitiveConversionState, phase: &self::ml_string::Text) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, _) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone(), phase.clone())))
        }
    }
}
