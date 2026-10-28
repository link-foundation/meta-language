// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::create_java_script_property_descriptor;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_property_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_define_java_script_ordinary_property_loop113;

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
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value411, _) => {
                                        source_ml_source2_data_property_value411
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
                            let source_ml_source2_field412 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                            {
                                let source_ml_source2_field413 = error.clone();
                                {
                                    let source_ml_source2_field414 = self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source2_field415 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source2_field416 = receiver.clone();
                                            {
                                                let source_ml_source2_field417 = Vec::<self::JavaScriptSemanticValue>::new();
                                                (source_ml_source2_field412.clone(), source_ml_source2_field417, source_ml_source2_field415.clone(), source_ml_source2_field413.clone(), source_ml_source2_field416.clone(), source_ml_source2_field414.clone())
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                            if !((property.clone()).1) {
                                {
                                    let source_ml_source2_field418 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                    {
                                        let source_ml_source2_field419 = self::ml_string::Text::from_units(vec![]);
                                        {
                                            let source_ml_source2_field420 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field421 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source2_field422 = receiver.clone();
                                                    {
                                                        let source_ml_source2_field423 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source2_field418.clone(), source_ml_source2_field423, source_ml_source2_field421.clone(), source_ml_source2_field419.clone(), source_ml_source2_field422.clone(), source_ml_source2_field420.clone())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if ((property.clone()).0).9 {
                                    {
                                        let source_ml_source2_field424 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                        {
                                            let source_ml_source2_field425 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source2_field426 = ((property.clone()).0).8;
                                                {
                                                    let source_ml_source2_field427 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field428 = receiver.clone();
                                                        {
                                                            let source_ml_source2_field429 = Vec::<self::JavaScriptSemanticValue>::new();
                                                            (source_ml_source2_field424.clone(), source_ml_source2_field429, source_ml_source2_field427.clone(), source_ml_source2_field425.clone(), source_ml_source2_field428.clone(), source_ml_source2_field426.clone())
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let getter = ((property.clone()).0).4;
                                        match getter.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                {
                                                    let source_ml_source2_field437 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                                    {
                                                        let source_ml_source2_field438 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field439 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field440 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field441 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field442 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field437.clone(), source_ml_source2_field442, source_ml_source2_field440.clone(), source_ml_source2_field438.clone(), source_ml_source2_field441.clone(), source_ml_source2_field439.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                {
                                                    let source_ml_source2_field443 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field444 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field445 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field446 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field447 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field448 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field443.clone(), source_ml_source2_field448, source_ml_source2_field446.clone(), source_ml_source2_field444.clone(), source_ml_source2_field447.clone(), source_ml_source2_field445.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                {
                                                    let source_ml_source2_field449 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field450 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field451 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field452 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field453 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field454 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field449.clone(), source_ml_source2_field454, source_ml_source2_field452.clone(), source_ml_source2_field450.clone(), source_ml_source2_field453.clone(), source_ml_source2_field451.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Number(_) => {
                                                {
                                                    let source_ml_source2_field455 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field456 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field457 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field458 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field459 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field460 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field455.clone(), source_ml_source2_field460, source_ml_source2_field458.clone(), source_ml_source2_field456.clone(), source_ml_source2_field459.clone(), source_ml_source2_field457.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                {
                                                    let source_ml_source2_field461 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field462 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field463 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field464 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field465 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field466 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field461.clone(), source_ml_source2_field466, source_ml_source2_field464.clone(), source_ml_source2_field462.clone(), source_ml_source2_field465.clone(), source_ml_source2_field463.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Text(_) => {
                                                {
                                                    let source_ml_source2_field467 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field468 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field469 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field470 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field471 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field472 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field467.clone(), source_ml_source2_field472, source_ml_source2_field470.clone(), source_ml_source2_field468.clone(), source_ml_source2_field471.clone(), source_ml_source2_field469.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                {
                                                    let source_ml_source2_field473 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field474 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field475 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field476 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field477 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field478 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field473.clone(), source_ml_source2_field478, source_ml_source2_field476.clone(), source_ml_source2_field474.clone(), source_ml_source2_field477.clone(), source_ml_source2_field475.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                {
                                                    let source_ml_source2_field479 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field480 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field481 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field482 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field483 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field484 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field479.clone(), source_ml_source2_field484, source_ml_source2_field482.clone(), source_ml_source2_field480.clone(), source_ml_source2_field483.clone(), source_ml_source2_field481.clone())
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
                                let source_ml_source2_field485 = heap.clone();
                                {
                                    let source_ml_source2_field486 = false;
                                    {
                                        let source_ml_source2_field487 = error.clone();
                                        {
                                            let source_ml_source2_field488 = Vec::<self::ml_string::Text>::new();
                                            (source_ml_source2_field487.clone(), source_ml_source2_field485.clone(), source_ml_source2_field486, source_ml_source2_field488)
                                        }
                                    }
                                }
                            }
                        } else {
                            if ((proposed.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                                {
                                    let source_ml_source2_field489 = heap.clone();
                                    {
                                        let source_ml_source2_field490 = false;
                                        {
                                            let source_ml_source2_field491 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                            {
                                                let source_ml_source2_field492 = Vec::<self::ml_string::Text>::new();
                                                (source_ml_source2_field491.clone(), source_ml_source2_field489.clone(), source_ml_source2_field490, source_ml_source2_field492)
                                            }
                                        }
                                    }
                                }
                            } else {
                                if ((proposed.clone()).5 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).4)) != (self::ml_string::Text::from_units(vec![])))) {
                                    {
                                        let source_ml_source2_field493 = heap.clone();
                                        {
                                            let source_ml_source2_field494 = false;
                                            {
                                                let source_ml_source2_field495 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 103, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                                {
                                                    let source_ml_source2_field496 = Vec::<self::ml_string::Text>::new();
                                                    (source_ml_source2_field495.clone(), source_ml_source2_field493.clone(), source_ml_source2_field494, source_ml_source2_field496)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if ((proposed.clone()).7 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).6)) != (self::ml_string::Text::from_units(vec![])))) {
                                        {
                                            let source_ml_source2_field497 = heap.clone();
                                            {
                                                let source_ml_source2_field498 = false;
                                                {
                                                    let source_ml_source2_field499 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 115, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                                    {
                                                        let source_ml_source2_field500 = Vec::<self::ml_string::Text>::new();
                                                        (source_ml_source2_field499.clone(), source_ml_source2_field497.clone(), source_ml_source2_field498, source_ml_source2_field500)
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                            {
                                                let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(index));
                                                if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    {
                                                        let source_ml_source2_field503 = heap.clone();
                                                        {
                                                            let source_ml_source2_field504 = false;
                                                            {
                                                                let source_ml_source2_field505 = self::ml_string::Text::from_units(vec![68, 101, 102, 105, 110, 101, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                                                {
                                                                    let source_ml_source2_field506 = Vec::<self::ml_string::Text>::new();
                                                                    (source_ml_source2_field505.clone(), source_ml_source2_field503.clone(), source_ml_source2_field504, source_ml_source2_field506)
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value507) => {
                                                                source_ml_source2_data_property_value507
                                                            }
                                                        }, key.clone());
                                                        {
                                                            let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value508) => {
                                                                    source_ml_source2_data_property_value508
                                                                }
                                                            }, self::ml_array::number_index(at));
                                                            {
                                                                let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false)
                                                                } else {
                                                                    match property.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value509) => {
                                                                            source_ml_source2_data_property_value509.clone()
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let transition = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value510, _, _) => {
                                                                            source_ml_source2_data_property_value510
                                                                        }
                                                                    }, current.clone(), proposed.clone());
                                                                    if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                        {
                                                                            let source_ml_source2_field511 = heap.clone();
                                                                            {
                                                                                let source_ml_source2_field512 = false;
                                                                                {
                                                                                    let source_ml_source2_field513 = (transition.clone()).2;
                                                                                    {
                                                                                        let source_ml_source2_field514 = Vec::<self::ml_string::Text>::new();
                                                                                        (source_ml_source2_field513.clone(), source_ml_source2_field511.clone(), source_ml_source2_field512, source_ml_source2_field514)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                                                                            {
                                                                                let source_ml_source2_field515 = heap.clone();
                                                                                {
                                                                                    let source_ml_source2_field516 = true;
                                                                                    {
                                                                                        let source_ml_source2_field517 = self::ml_string::Text::from_units(vec![]);
                                                                                        {
                                                                                            let source_ml_source2_field518 = Vec::<self::ml_string::Text>::new();
                                                                                            (source_ml_source2_field517.clone(), source_ml_source2_field515.clone(), source_ml_source2_field516, source_ml_source2_field518)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if ((at < (0f64)) && ((((match object.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value520) => {
                                                                                    source_ml_source2_data_property_value520
                                                                                }
                                                                            }).len() as f64)) >= (4096f64))) {
                                                                                {
                                                                                    let source_ml_source2_field521 = heap.clone();
                                                                                    {
                                                                                        let source_ml_source2_field522 = false;
                                                                                        {
                                                                                            let source_ml_source2_field523 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                            {
                                                                                                let source_ml_source2_field524 = Vec::<self::ml_string::Text>::new();
                                                                                                (source_ml_source2_field523.clone(), source_ml_source2_field521.clone(), source_ml_source2_field522, source_ml_source2_field524)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                {
                                                                                    let count = 0f64;
                                                                                    {
                                                                                        let source_ml_source1_temporary_values112 = shared_constructor_field1;
                                                                                        {
                                                                                            let source_ml_source1_temporary_index112 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let count_2 = self::source_ml_source1_temporary_define_java_script_ordinary_property_loop113(count, source_ml_source1_temporary_values112, source_ml_source1_temporary_index112.clone());
                                                                                                if ((at < (0f64)) && (count_2 >= (16384f64))) {
                                                                                                    {
                                                                                                        let source_ml_source2_field526 = heap.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_field527 = false;
                                                                                                            {
                                                                                                                let source_ml_source2_field528 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                                                {
                                                                                                                    let source_ml_source2_field529 = Vec::<self::ml_string::Text>::new();
                                                                                                                    (source_ml_source2_field528.clone(), source_ml_source2_field526.clone(), source_ml_source2_field527, source_ml_source2_field529)
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    {
                                                                                                        let updated = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), (transition.clone()).0);
                                                                                                        {
                                                                                                            let properties = if (at < (0f64)) {
                                                                                                                {
                                                                                                                    let source_ml_source2_concat_value531 = match object.clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value530) => {
                                                                                                                            source_ml_source2_data_property_value530
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_concat_value532 = vec![updated.clone()];
                                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), source_ml_source2_concat_value531), source_ml_source2_concat_value532)
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                { let ml_values2 = match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value533) => {
                                                                                                                        source_ml_source2_data_property_value533
                                                                                                                    }
                                                                                                                }; let ml_callback2 = { let at = at; let updated = updated.clone(); std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if (position == at) {
                                                                                                                    updated.clone()
                                                                                                                } else {
                                                                                                                    entry.clone()
                                                                                                                } }) }; ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2, (ml_index2 as f64))).collect::<Vec<_>>() }
                                                                                                            };
                                                                                                            {
                                                                                                                let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value534, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value534.clone()
                                                                                                                    }
                                                                                                                }, Box::new(match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value535, _, _, _) => {
                                                                                                                        let source_ml_source2_data_property_value535 = *source_ml_source2_data_property_value535;
                                                                                                                        source_ml_source2_data_property_value535.clone()
                                                                                                                    }
                                                                                                                }), match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value536, _, _) => {
                                                                                                                        source_ml_source2_data_property_value536
                                                                                                                    }
                                                                                                                }, match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value537, _) => {
                                                                                                                        source_ml_source2_data_property_value537
                                                                                                                    }
                                                                                                                }, properties);
                                                                                                                {
                                                                                                                    let source_ml_source2_field538 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                                                                    {
                                                                                                                        let source_ml_source2_field539 = true;
                                                                                                                        {
                                                                                                                            let source_ml_source2_field540 = self::ml_string::Text::from_units(vec![]);
                                                                                                                            {
                                                                                                                                let source_ml_source2_field541 = (transition.clone()).3;
                                                                                                                                (source_ml_source2_field540.clone(), source_ml_source2_field538.clone(), source_ml_source2_field539, source_ml_source2_field541)
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_ordinary_set_descriptor(value: self::JavaScriptSemanticValue, creation: bool) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field626 = creation;
        {
            let source_ml_source2_field627 = creation;
            {
                let source_ml_source2_field628 = creation;
                {
                    let source_ml_source2_field629 = creation;
                    {
                        let source_ml_source2_field630 = true;
                        {
                            let source_ml_source2_field631 = value.clone();
                            {
                                let source_ml_source2_field632 = creation;
                                {
                                    let source_ml_source2_field633 = creation;
                                    {
                                        let source_ml_source2_field634 = false;
                                        {
                                            let source_ml_source2_field635 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field636 = false;
                                                {
                                                    let source_ml_source2_field637 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field627, source_ml_source2_field626, source_ml_source2_field629, source_ml_source2_field628, source_ml_source2_field635.clone(), source_ml_source2_field634, source_ml_source2_field637.clone(), source_ml_source2_field636, source_ml_source2_field631.clone(), source_ml_source2_field630, source_ml_source2_field633, source_ml_source2_field632)
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
