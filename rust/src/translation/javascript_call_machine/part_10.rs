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
use super::source_ml_source1_temporary_define_java_script_ordinary_property_loop111;

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
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value398, _) => {
                                        source_ml_source2_data_property_value398
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
                            let source_ml_source2_field399 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                            {
                                let source_ml_source2_field400 = error.clone();
                                {
                                    let source_ml_source2_field401 = self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source2_field402 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source2_field403 = receiver.clone();
                                            {
                                                let source_ml_source2_field404 = Vec::<self::JavaScriptSemanticValue>::new();
                                                (source_ml_source2_field399.clone(), source_ml_source2_field404, source_ml_source2_field402.clone(), source_ml_source2_field400.clone(), source_ml_source2_field403.clone(), source_ml_source2_field401.clone())
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
                                    let source_ml_source2_field405 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                    {
                                        let source_ml_source2_field406 = self::ml_string::Text::from_units(vec![]);
                                        {
                                            let source_ml_source2_field407 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field408 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source2_field409 = receiver.clone();
                                                    {
                                                        let source_ml_source2_field410 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source2_field405.clone(), source_ml_source2_field410, source_ml_source2_field408.clone(), source_ml_source2_field406.clone(), source_ml_source2_field409.clone(), source_ml_source2_field407.clone())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if ((property.clone()).0).9 {
                                    {
                                        let source_ml_source2_field411 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                        {
                                            let source_ml_source2_field412 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source2_field413 = ((property.clone()).0).8;
                                                {
                                                    let source_ml_source2_field414 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field415 = receiver.clone();
                                                        {
                                                            let source_ml_source2_field416 = Vec::<self::JavaScriptSemanticValue>::new();
                                                            (source_ml_source2_field411.clone(), source_ml_source2_field416, source_ml_source2_field414.clone(), source_ml_source2_field412.clone(), source_ml_source2_field415.clone(), source_ml_source2_field413.clone())
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
                                                    let source_ml_source2_field424 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                                    {
                                                        let source_ml_source2_field425 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field426 = self::JavaScriptSemanticValue::Undefined;
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
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                {
                                                    let source_ml_source2_field430 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field431 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field432 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field433 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field434 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field435 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field430.clone(), source_ml_source2_field435, source_ml_source2_field433.clone(), source_ml_source2_field431.clone(), source_ml_source2_field434.clone(), source_ml_source2_field432.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f417) => {
                                                {
                                                    let source_ml_source2_field436 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field437 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field438 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field439 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field440 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field441 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field436.clone(), source_ml_source2_field441, source_ml_source2_field439.clone(), source_ml_source2_field437.clone(), source_ml_source2_field440.clone(), source_ml_source2_field438.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source2_f418) => {
                                                {
                                                    let source_ml_source2_field442 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field443 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field444 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field445 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field446 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field447 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field442.clone(), source_ml_source2_field447, source_ml_source2_field445.clone(), source_ml_source2_field443.clone(), source_ml_source2_field446.clone(), source_ml_source2_field444.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source2_f419) => {
                                                {
                                                    let source_ml_source2_field448 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field449 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field450 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field451 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field452 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field453 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field448.clone(), source_ml_source2_field453, source_ml_source2_field451.clone(), source_ml_source2_field449.clone(), source_ml_source2_field452.clone(), source_ml_source2_field450.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source2_f420) => {
                                                {
                                                    let source_ml_source2_field454 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field455 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field456 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field457 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field458 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field459 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field454.clone(), source_ml_source2_field459, source_ml_source2_field457.clone(), source_ml_source2_field455.clone(), source_ml_source2_field458.clone(), source_ml_source2_field456.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f421) => {
                                                {
                                                    let source_ml_source2_field460 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field461 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field462 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field463 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field464 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field465 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field460.clone(), source_ml_source2_field465, source_ml_source2_field463.clone(), source_ml_source2_field461.clone(), source_ml_source2_field464.clone(), source_ml_source2_field462.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Object(source_ml_source2_f422, source_ml_source2_f423) => {
                                                {
                                                    let source_ml_source2_field466 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field467 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field468 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field469 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field470 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field471 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field466.clone(), source_ml_source2_field471, source_ml_source2_field469.clone(), source_ml_source2_field467.clone(), source_ml_source2_field470.clone(), source_ml_source2_field468.clone())
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
                        let source_ml_source2_field472 = heap.clone();
                        {
                            let source_ml_source2_field473 = false;
                            {
                                let source_ml_source2_field474 = error.clone();
                                {
                                    let source_ml_source2_field475 = Vec::<self::ml_string::Text>::new();
                                    (source_ml_source2_field474.clone(), source_ml_source2_field472.clone(), source_ml_source2_field473, source_ml_source2_field475)
                                }
                            }
                        }
                    }
                } else {
                    if ((proposed.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                        {
                            let source_ml_source2_field476 = heap.clone();
                            {
                                let source_ml_source2_field477 = false;
                                {
                                    let source_ml_source2_field478 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                    {
                                        let source_ml_source2_field479 = Vec::<self::ml_string::Text>::new();
                                        (source_ml_source2_field478.clone(), source_ml_source2_field476.clone(), source_ml_source2_field477, source_ml_source2_field479)
                                    }
                                }
                            }
                        }
                    } else {
                        if ((proposed.clone()).5 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).4)) != (self::ml_string::Text::from_units(vec![])))) {
                            {
                                let source_ml_source2_field480 = heap.clone();
                                {
                                    let source_ml_source2_field481 = false;
                                    {
                                        let source_ml_source2_field482 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 103, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                        {
                                            let source_ml_source2_field483 = Vec::<self::ml_string::Text>::new();
                                            (source_ml_source2_field482.clone(), source_ml_source2_field480.clone(), source_ml_source2_field481, source_ml_source2_field483)
                                        }
                                    }
                                }
                            }
                        } else {
                            if ((proposed.clone()).7 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).6)) != (self::ml_string::Text::from_units(vec![])))) {
                                {
                                    let source_ml_source2_field484 = heap.clone();
                                    {
                                        let source_ml_source2_field485 = false;
                                        {
                                            let source_ml_source2_field486 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 115, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                            {
                                                let source_ml_source2_field487 = Vec::<self::ml_string::Text>::new();
                                                (source_ml_source2_field486.clone(), source_ml_source2_field484.clone(), source_ml_source2_field485, source_ml_source2_field487)
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index = self::find_java_script_ordinary_object(match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value488, _) => {
                                            source_ml_source2_data_property_value488
                                        }
                                    }, identity.clone());
                                    {
                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value489, _) => source_ml_source2_data_property_value489.as_slice() }), self::ml_array::number_index(index));
                                        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source2_field490 = heap.clone();
                                                {
                                                    let source_ml_source2_field491 = false;
                                                    {
                                                        let source_ml_source2_field492 = self::ml_string::Text::from_units(vec![68, 101, 102, 105, 110, 101, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                                        {
                                                            let source_ml_source2_field493 = Vec::<self::ml_string::Text>::new();
                                                            (source_ml_source2_field492.clone(), source_ml_source2_field490.clone(), source_ml_source2_field491, source_ml_source2_field493)
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value494) => {
                                                        source_ml_source2_data_property_value494
                                                    }
                                                }, key.clone());
                                                {
                                                    let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value495) => {
                                                            source_ml_source2_data_property_value495
                                                        }
                                                    }, self::ml_array::number_index(at));
                                                    {
                                                        let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false)
                                                        } else {
                                                            match property.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value496) => {
                                                                    source_ml_source2_data_property_value496.clone()
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let transition = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value497, _, _) => {
                                                                    source_ml_source2_data_property_value497
                                                                }
                                                            }, current.clone(), proposed.clone());
                                                            if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                {
                                                                    let source_ml_source2_field498 = heap.clone();
                                                                    {
                                                                        let source_ml_source2_field499 = false;
                                                                        {
                                                                            let source_ml_source2_field500 = (transition.clone()).2;
                                                                            {
                                                                                let source_ml_source2_field501 = Vec::<self::ml_string::Text>::new();
                                                                                (source_ml_source2_field500.clone(), source_ml_source2_field498.clone(), source_ml_source2_field499, source_ml_source2_field501)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                                                                    {
                                                                        let source_ml_source2_field502 = heap.clone();
                                                                        {
                                                                            let source_ml_source2_field503 = true;
                                                                            {
                                                                                let source_ml_source2_field504 = self::ml_string::Text::from_units(vec![]);
                                                                                {
                                                                                    let source_ml_source2_field505 = Vec::<self::ml_string::Text>::new();
                                                                                    (source_ml_source2_field504.clone(), source_ml_source2_field502.clone(), source_ml_source2_field503, source_ml_source2_field505)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if ((at < (0f64)) && ((((match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value507) => {
                                                                            source_ml_source2_data_property_value507
                                                                        }
                                                                    }).len() as f64)) >= (4096f64))) {
                                                                        {
                                                                            let source_ml_source2_field508 = heap.clone();
                                                                            {
                                                                                let source_ml_source2_field509 = false;
                                                                                {
                                                                                    let source_ml_source2_field510 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                    {
                                                                                        let source_ml_source2_field511 = Vec::<self::ml_string::Text>::new();
                                                                                        (source_ml_source2_field510.clone(), source_ml_source2_field508.clone(), source_ml_source2_field509, source_ml_source2_field511)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let count = 0f64;
                                                                            {
                                                                                let source_ml_source1_temporary_values110 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value512, _) => {
                                                                                        source_ml_source2_data_property_value512
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_temporary_index110 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let count_2 = self::source_ml_source1_temporary_define_java_script_ordinary_property_loop111(count, source_ml_source1_temporary_values110, source_ml_source1_temporary_index110.clone());
                                                                                        if ((at < (0f64)) && (count_2 >= (16384f64))) {
                                                                                            {
                                                                                                let source_ml_source2_field513 = heap.clone();
                                                                                                {
                                                                                                    let source_ml_source2_field514 = false;
                                                                                                    {
                                                                                                        let source_ml_source2_field515 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                                        {
                                                                                                            let source_ml_source2_field516 = Vec::<self::ml_string::Text>::new();
                                                                                                            (source_ml_source2_field515.clone(), source_ml_source2_field513.clone(), source_ml_source2_field514, source_ml_source2_field516)
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
                                                                                                            let source_ml_source2_concat_value518 = match object.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value517) => {
                                                                                                                    source_ml_source2_data_property_value517
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_concat_value519 = vec![updated.clone()];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), source_ml_source2_concat_value518), source_ml_source2_concat_value519)
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        { let ml_values2 = match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value520) => {
                                                                                                                source_ml_source2_data_property_value520
                                                                                                            }
                                                                                                        }; let ml_callback2 = { let at = at; let updated = updated.clone(); std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if (position == at) {
                                                                                                            updated.clone()
                                                                                                        } else {
                                                                                                            entry.clone()
                                                                                                        } }) }; ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2, (ml_index2 as f64))).collect::<Vec<_>>() }
                                                                                                    };
                                                                                                    {
                                                                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value521, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value521.clone()
                                                                                                            }
                                                                                                        }, Box::new(match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value522, _, _, _) => {
                                                                                                                let source_ml_source2_data_property_value522 = *source_ml_source2_data_property_value522;
                                                                                                                source_ml_source2_data_property_value522.clone()
                                                                                                            }
                                                                                                        }), match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value523, _, _) => {
                                                                                                                source_ml_source2_data_property_value523
                                                                                                            }
                                                                                                        }, match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value524, _) => {
                                                                                                                source_ml_source2_data_property_value524
                                                                                                            }
                                                                                                        }, properties);
                                                                                                        {
                                                                                                            let source_ml_source2_field525 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                                                            {
                                                                                                                let source_ml_source2_field526 = true;
                                                                                                                {
                                                                                                                    let source_ml_source2_field527 = self::ml_string::Text::from_units(vec![]);
                                                                                                                    {
                                                                                                                        let source_ml_source2_field528 = (transition.clone()).3;
                                                                                                                        (source_ml_source2_field527.clone(), source_ml_source2_field525.clone(), source_ml_source2_field526, source_ml_source2_field528)
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
        let source_ml_source2_field613 = creation;
        {
            let source_ml_source2_field614 = creation;
            {
                let source_ml_source2_field615 = creation;
                {
                    let source_ml_source2_field616 = creation;
                    {
                        let source_ml_source2_field617 = true;
                        {
                            let source_ml_source2_field618 = value.clone();
                            {
                                let source_ml_source2_field619 = creation;
                                {
                                    let source_ml_source2_field620 = creation;
                                    {
                                        let source_ml_source2_field621 = false;
                                        {
                                            let source_ml_source2_field622 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field623 = false;
                                                {
                                                    let source_ml_source2_field624 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field614, source_ml_source2_field613, source_ml_source2_field616, source_ml_source2_field615, source_ml_source2_field622.clone(), source_ml_source2_field621, source_ml_source2_field624.clone(), source_ml_source2_field623, source_ml_source2_field618.clone(), source_ml_source2_field617, source_ml_source2_field620, source_ml_source2_field619)
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
