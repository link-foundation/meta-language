// Generated from complete js/src/translation/javascript-property-definition.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_temporary_set_java_script_ordinary_prototype_loop21;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn delete_java_script_ordinary_property(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
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
                        let source_ml_source1_field370 = heap.clone();
                        {
                            let source_ml_source1_field371 = false;
                            {
                                let source_ml_source1_field372 = error.clone();
                                (source_ml_source1_field372.clone(), source_ml_source1_field370.clone(), source_ml_source1_field371)
                            }
                        }
                    }
                } else {
                    {
                        let index = self::find_java_script_ordinary_object(match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value373, _) => {
                                source_ml_source1_data_property_value373
                            }
                        }, identity.clone());
                        {
                            let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value374, _) => source_ml_source1_data_property_value374.as_slice() }), self::ml_array::number_index(index));
                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                {
                                    let source_ml_source1_field375 = heap.clone();
                                    {
                                        let source_ml_source1_field376 = false;
                                        {
                                            let source_ml_source1_field377 = self::ml_string::Text::from_units(vec![68, 101, 108, 101, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                            (source_ml_source1_field377.clone(), source_ml_source1_field375.clone(), source_ml_source1_field376)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value378) => {
                                            source_ml_source1_data_property_value378
                                        }
                                    }, key.clone());
                                    {
                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value379) => {
                                                source_ml_source1_data_property_value379
                                            }
                                        }, self::ml_array::number_index(at));
                                        if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source1_field380 = heap.clone();
                                                {
                                                    let source_ml_source1_field381 = true;
                                                    {
                                                        let source_ml_source1_field382 = self::ml_string::Text::from_units(vec![]);
                                                        (source_ml_source1_field382.clone(), source_ml_source1_field380.clone(), source_ml_source1_field381)
                                                    }
                                                }
                                            }
                                        } else {
                                            if !((match property.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value383) => {
                                                    source_ml_source1_data_property_value383.clone()
                                                }
                                            }).0) {
                                                {
                                                    let source_ml_source1_field384 = heap.clone();
                                                    {
                                                        let source_ml_source1_field385 = false;
                                                        {
                                                            let source_ml_source1_field386 = self::ml_string::Text::from_units(vec![]);
                                                            (source_ml_source1_field386.clone(), source_ml_source1_field384.clone(), source_ml_source1_field385)
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let properties = { let ml_values3 = match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value387) => {
                                                            source_ml_source1_data_property_value387
                                                        }
                                                    }; let ml_callback3 = { let at = at; std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> bool { (position != at) }) }; ml_values3.iter().cloned().enumerate().filter_map(|(ml_index3, ml_value3)| if ml_callback3(ml_value3.clone(), (ml_index3 as f64)) { Some(ml_value3) } else { None }).collect::<Vec<_>>() };
                                                    {
                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value388, _, _, _, _) => {
                                                                source_ml_source1_data_property_value388.clone()
                                                            }
                                                        }, Box::new(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value389, _, _, _) => {
                                                                let source_ml_source1_data_property_value389 = *source_ml_source1_data_property_value389;
                                                                source_ml_source1_data_property_value389.clone()
                                                            }
                                                        }), match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value390, _, _) => {
                                                                source_ml_source1_data_property_value390
                                                            }
                                                        }, match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value391, _) => {
                                                                source_ml_source1_data_property_value391
                                                            }
                                                        }, properties);
                                                        {
                                                            let source_ml_source1_field392 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                            {
                                                                let source_ml_source1_field393 = true;
                                                                {
                                                                    let source_ml_source1_field394 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source1_field394.clone(), source_ml_source1_field392.clone(), source_ml_source1_field393)
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
pub fn prevent_java_script_ordinary_extensions(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source1_field395 = heap.clone();
                {
                    let source_ml_source1_field396 = false;
                    {
                        let source_ml_source1_field397 = error.clone();
                        (source_ml_source1_field397.clone(), source_ml_source1_field395.clone(), source_ml_source1_field396)
                    }
                }
            }
        } else {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value398, _) => {
                        source_ml_source1_data_property_value398
                    }
                }, identity.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value399, _) => source_ml_source1_data_property_value399.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        {
                            let source_ml_source1_field400 = heap.clone();
                            {
                                let source_ml_source1_field401 = false;
                                {
                                    let source_ml_source1_field402 = self::ml_string::Text::from_units(vec![80, 114, 101, 118, 101, 110, 116, 69, 120, 116, 101, 110, 115, 105, 111, 110, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                    (source_ml_source1_field402.clone(), source_ml_source1_field400.clone(), source_ml_source1_field401)
                                }
                            }
                        }
                    } else {
                        {
                            let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value403, _, _, _, _) => {
                                    source_ml_source1_data_property_value403.clone()
                                }
                            }, Box::new(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value404, _, _, _) => {
                                    let source_ml_source1_data_property_value404 = *source_ml_source1_data_property_value404;
                                    source_ml_source1_data_property_value404.clone()
                                }
                            }), false, match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value405, _) => {
                                    source_ml_source1_data_property_value405
                                }
                            }, match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value406) => {
                                    source_ml_source1_data_property_value406
                                }
                            });
                            {
                                let source_ml_source1_field407 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                {
                                    let source_ml_source1_field408 = true;
                                    {
                                        let source_ml_source1_field409 = self::ml_string::Text::from_units(vec![]);
                                        (source_ml_source1_field409.clone(), source_ml_source1_field407.clone(), source_ml_source1_field408)
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
pub fn set_java_script_ordinary_prototype(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, prototype: self::JavaScriptSemanticValue) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source1_field410 = heap.clone();
                {
                    let source_ml_source1_field411 = false;
                    {
                        let source_ml_source1_field412 = heap_error.clone();
                        (source_ml_source1_field412.clone(), source_ml_source1_field410.clone(), source_ml_source1_field411)
                    }
                }
            }
        } else {
            if (match prototype.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    true
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f413) => {
                    true
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f414) => {
                    true
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f415) => {
                    true
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f416) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f417) => {
                    true
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f418, source_ml_source1_f419) => {
                    true
                }
            } && match prototype.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    true
                }
                self::JavaScriptSemanticValue::Null => {
                    true
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f420) => {
                    true
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f421) => {
                    true
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f422) => {
                    true
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f423) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f424) => {
                    true
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f425, source_ml_source1_f426) => {
                    false
                }
            }) {
                {
                    let source_ml_source1_field427 = heap.clone();
                    {
                        let source_ml_source1_field428 = false;
                        {
                            let source_ml_source1_field429 = self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]);
                            (source_ml_source1_field429.clone(), source_ml_source1_field427.clone(), source_ml_source1_field428)
                        }
                    }
                }
            } else {
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        {
                            let source_ml_source1_field430 = heap.clone();
                            {
                                let source_ml_source1_field431 = false;
                                {
                                    let source_ml_source1_field432 = error.clone();
                                    (source_ml_source1_field432.clone(), source_ml_source1_field430.clone(), source_ml_source1_field431)
                                }
                            }
                        }
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value433, _) => {
                                    source_ml_source1_data_property_value433
                                }
                            }, identity.clone());
                            {
                                let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value434, _) => source_ml_source1_data_property_value434.as_slice() }), self::ml_array::number_index(index));
                                if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    {
                                        let source_ml_source1_field435 = heap.clone();
                                        {
                                            let source_ml_source1_field436 = false;
                                            {
                                                let source_ml_source1_field437 = self::ml_string::Text::from_units(vec![83, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                                (source_ml_source1_field437.clone(), source_ml_source1_field435.clone(), source_ml_source1_field436)
                                            }
                                        }
                                    }
                                } else {
                                    if self::test_java_script_semantic_same_value(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value438, _, _, _) => {
                                            let source_ml_source1_data_property_value438 = *source_ml_source1_data_property_value438;
                                            source_ml_source1_data_property_value438.clone()
                                        }
                                    }, prototype.clone()) {
                                        {
                                            let source_ml_source1_field439 = heap.clone();
                                            {
                                                let source_ml_source1_field440 = true;
                                                {
                                                    let source_ml_source1_field441 = self::ml_string::Text::from_units(vec![]);
                                                    (source_ml_source1_field441.clone(), source_ml_source1_field439.clone(), source_ml_source1_field440)
                                                }
                                            }
                                        }
                                    } else {
                                        if !(match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value442, _, _) => {
                                                source_ml_source1_data_property_value442
                                            }
                                        }) {
                                            {
                                                let source_ml_source1_field443 = heap.clone();
                                                {
                                                    let source_ml_source1_field444 = false;
                                                    {
                                                        let source_ml_source1_field445 = self::ml_string::Text::from_units(vec![]);
                                                        (source_ml_source1_field445.clone(), source_ml_source1_field443.clone(), source_ml_source1_field444)
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let next = prototype.clone();
                                                {
                                                    let depth = 0f64;
                                                    {
                                                        let source_ml_source1_s446 = self::source_ml_temporary_set_java_script_ordinary_prototype_loop21(heap.clone(), identity.clone(), next.clone(), depth);
                                                        match source_ml_source1_s446.clone() {
                                                            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done => {
                                                                {
                                                                    let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value447, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value447.clone()
                                                                        }
                                                                    }, Box::new(prototype.clone()), match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value448, _, _) => {
                                                                            source_ml_source1_data_property_value448
                                                                        }
                                                                    }, match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value449, _) => {
                                                                            source_ml_source1_data_property_value449
                                                                        }
                                                                    }, match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value450) => {
                                                                            source_ml_source1_data_property_value450
                                                                        }
                                                                    });
                                                                    {
                                                                        let source_ml_source1_field451 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                        {
                                                                            let source_ml_source1_field452 = true;
                                                                            {
                                                                                let source_ml_source1_field453 = self::ml_string::Text::from_units(vec![]);
                                                                                (source_ml_source1_field453.clone(), source_ml_source1_field451.clone(), source_ml_source1_field452)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return(source_ml_temporary_result21) => {
                                                                source_ml_temporary_result21.clone()
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
        let source_ml_source1_field454 = creation;
        {
            let source_ml_source1_field455 = creation;
            {
                let source_ml_source1_field456 = creation;
                {
                    let source_ml_source1_field457 = creation;
                    {
                        let source_ml_source1_field458 = true;
                        {
                            let source_ml_source1_field459 = value.clone();
                            {
                                let source_ml_source1_field460 = creation;
                                {
                                    let source_ml_source1_field461 = creation;
                                    {
                                        let source_ml_source1_field462 = false;
                                        {
                                            let source_ml_source1_field463 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field464 = false;
                                                {
                                                    let source_ml_source1_field465 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field455, source_ml_source1_field454, source_ml_source1_field457, source_ml_source1_field456, source_ml_source1_field463.clone(), source_ml_source1_field462, source_ml_source1_field465.clone(), source_ml_source1_field464, source_ml_source1_field459.clone(), source_ml_source1_field458, source_ml_source1_field461, source_ml_source1_field460)
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
