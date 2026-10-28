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
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value394, _) => {
                                        source_ml_source2_data_property_value394
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
                            let source_ml_source2_field395 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                            {
                                let source_ml_source2_field396 = error.clone();
                                {
                                    let source_ml_source2_field397 = self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source2_field398 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source2_field399 = receiver.clone();
                                            {
                                                let source_ml_source2_field400 = Vec::<self::JavaScriptSemanticValue>::new();
                                                (source_ml_source2_field395.clone(), source_ml_source2_field400, source_ml_source2_field398.clone(), source_ml_source2_field396.clone(), source_ml_source2_field399.clone(), source_ml_source2_field397.clone())
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
                                    let source_ml_source2_field401 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                    {
                                        let source_ml_source2_field402 = self::ml_string::Text::from_units(vec![]);
                                        {
                                            let source_ml_source2_field403 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field404 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source2_field405 = receiver.clone();
                                                    {
                                                        let source_ml_source2_field406 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source2_field401.clone(), source_ml_source2_field406, source_ml_source2_field404.clone(), source_ml_source2_field402.clone(), source_ml_source2_field405.clone(), source_ml_source2_field403.clone())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if ((property.clone()).0).9 {
                                    {
                                        let source_ml_source2_field407 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                        {
                                            let source_ml_source2_field408 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source2_field409 = ((property.clone()).0).8;
                                                {
                                                    let source_ml_source2_field410 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field411 = receiver.clone();
                                                        {
                                                            let source_ml_source2_field412 = Vec::<self::JavaScriptSemanticValue>::new();
                                                            (source_ml_source2_field407.clone(), source_ml_source2_field412, source_ml_source2_field410.clone(), source_ml_source2_field408.clone(), source_ml_source2_field411.clone(), source_ml_source2_field409.clone())
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
                                                    let source_ml_source2_field420 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                                    {
                                                        let source_ml_source2_field421 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field422 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field423 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field424 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field425 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field420.clone(), source_ml_source2_field425, source_ml_source2_field423.clone(), source_ml_source2_field421.clone(), source_ml_source2_field424.clone(), source_ml_source2_field422.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                {
                                                    let source_ml_source2_field426 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field427 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field428 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field429 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field430 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field431 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field426.clone(), source_ml_source2_field431, source_ml_source2_field429.clone(), source_ml_source2_field427.clone(), source_ml_source2_field430.clone(), source_ml_source2_field428.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f413) => {
                                                {
                                                    let source_ml_source2_field432 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field433 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field434 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field435 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field436 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field437 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field432.clone(), source_ml_source2_field437, source_ml_source2_field435.clone(), source_ml_source2_field433.clone(), source_ml_source2_field436.clone(), source_ml_source2_field434.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source2_f414) => {
                                                {
                                                    let source_ml_source2_field438 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field439 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field440 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field441 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field442 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field443 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field438.clone(), source_ml_source2_field443, source_ml_source2_field441.clone(), source_ml_source2_field439.clone(), source_ml_source2_field442.clone(), source_ml_source2_field440.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source2_f415) => {
                                                {
                                                    let source_ml_source2_field444 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field445 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field446 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field447 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field448 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field449 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field444.clone(), source_ml_source2_field449, source_ml_source2_field447.clone(), source_ml_source2_field445.clone(), source_ml_source2_field448.clone(), source_ml_source2_field446.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source2_f416) => {
                                                {
                                                    let source_ml_source2_field450 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field451 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field452 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field453 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field454 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field455 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field450.clone(), source_ml_source2_field455, source_ml_source2_field453.clone(), source_ml_source2_field451.clone(), source_ml_source2_field454.clone(), source_ml_source2_field452.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f417) => {
                                                {
                                                    let source_ml_source2_field456 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field457 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field458 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field459 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field460 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field461 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field456.clone(), source_ml_source2_field461, source_ml_source2_field459.clone(), source_ml_source2_field457.clone(), source_ml_source2_field460.clone(), source_ml_source2_field458.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Object(source_ml_source2_f418, source_ml_source2_f419) => {
                                                {
                                                    let source_ml_source2_field462 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field463 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field464 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field465 = getter.clone();
                                                                {
                                                                    let source_ml_source2_field466 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field467 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field462.clone(), source_ml_source2_field467, source_ml_source2_field465.clone(), source_ml_source2_field463.clone(), source_ml_source2_field466.clone(), source_ml_source2_field464.clone())
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
                        let source_ml_source2_field468 = heap.clone();
                        {
                            let source_ml_source2_field469 = false;
                            {
                                let source_ml_source2_field470 = error.clone();
                                {
                                    let source_ml_source2_field471 = Vec::<self::ml_string::Text>::new();
                                    (source_ml_source2_field470.clone(), source_ml_source2_field468.clone(), source_ml_source2_field469, source_ml_source2_field471)
                                }
                            }
                        }
                    }
                } else {
                    if ((proposed.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                        {
                            let source_ml_source2_field472 = heap.clone();
                            {
                                let source_ml_source2_field473 = false;
                                {
                                    let source_ml_source2_field474 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                    {
                                        let source_ml_source2_field475 = Vec::<self::ml_string::Text>::new();
                                        (source_ml_source2_field474.clone(), source_ml_source2_field472.clone(), source_ml_source2_field473, source_ml_source2_field475)
                                    }
                                }
                            }
                        }
                    } else {
                        if ((proposed.clone()).5 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).4)) != (self::ml_string::Text::from_units(vec![])))) {
                            {
                                let source_ml_source2_field476 = heap.clone();
                                {
                                    let source_ml_source2_field477 = false;
                                    {
                                        let source_ml_source2_field478 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 103, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                        {
                                            let source_ml_source2_field479 = Vec::<self::ml_string::Text>::new();
                                            (source_ml_source2_field478.clone(), source_ml_source2_field476.clone(), source_ml_source2_field477, source_ml_source2_field479)
                                        }
                                    }
                                }
                            }
                        } else {
                            if ((proposed.clone()).7 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).6)) != (self::ml_string::Text::from_units(vec![])))) {
                                {
                                    let source_ml_source2_field480 = heap.clone();
                                    {
                                        let source_ml_source2_field481 = false;
                                        {
                                            let source_ml_source2_field482 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 115, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                            {
                                                let source_ml_source2_field483 = Vec::<self::ml_string::Text>::new();
                                                (source_ml_source2_field482.clone(), source_ml_source2_field480.clone(), source_ml_source2_field481, source_ml_source2_field483)
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index = self::find_java_script_ordinary_object(match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value484, _) => {
                                            source_ml_source2_data_property_value484
                                        }
                                    }, identity.clone());
                                    {
                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value485, _) => source_ml_source2_data_property_value485.as_slice() }), self::ml_array::number_index(index));
                                        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source2_field486 = heap.clone();
                                                {
                                                    let source_ml_source2_field487 = false;
                                                    {
                                                        let source_ml_source2_field488 = self::ml_string::Text::from_units(vec![68, 101, 102, 105, 110, 101, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                                        {
                                                            let source_ml_source2_field489 = Vec::<self::ml_string::Text>::new();
                                                            (source_ml_source2_field488.clone(), source_ml_source2_field486.clone(), source_ml_source2_field487, source_ml_source2_field489)
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value490) => {
                                                        source_ml_source2_data_property_value490
                                                    }
                                                }, key.clone());
                                                {
                                                    let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value491) => {
                                                            source_ml_source2_data_property_value491
                                                        }
                                                    }, self::ml_array::number_index(at));
                                                    {
                                                        let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false)
                                                        } else {
                                                            match property.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value492) => {
                                                                    source_ml_source2_data_property_value492.clone()
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let transition = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value493, _, _) => {
                                                                    source_ml_source2_data_property_value493
                                                                }
                                                            }, current.clone(), proposed.clone());
                                                            if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                {
                                                                    let source_ml_source2_field494 = heap.clone();
                                                                    {
                                                                        let source_ml_source2_field495 = false;
                                                                        {
                                                                            let source_ml_source2_field496 = (transition.clone()).2;
                                                                            {
                                                                                let source_ml_source2_field497 = Vec::<self::ml_string::Text>::new();
                                                                                (source_ml_source2_field496.clone(), source_ml_source2_field494.clone(), source_ml_source2_field495, source_ml_source2_field497)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                                                                    {
                                                                        let source_ml_source2_field498 = heap.clone();
                                                                        {
                                                                            let source_ml_source2_field499 = true;
                                                                            {
                                                                                let source_ml_source2_field500 = self::ml_string::Text::from_units(vec![]);
                                                                                {
                                                                                    let source_ml_source2_field501 = Vec::<self::ml_string::Text>::new();
                                                                                    (source_ml_source2_field500.clone(), source_ml_source2_field498.clone(), source_ml_source2_field499, source_ml_source2_field501)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if ((at < (0f64)) && ((((match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value503) => {
                                                                            source_ml_source2_data_property_value503
                                                                        }
                                                                    }).len() as f64)) >= (4096f64))) {
                                                                        {
                                                                            let source_ml_source2_field504 = heap.clone();
                                                                            {
                                                                                let source_ml_source2_field505 = false;
                                                                                {
                                                                                    let source_ml_source2_field506 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                    {
                                                                                        let source_ml_source2_field507 = Vec::<self::ml_string::Text>::new();
                                                                                        (source_ml_source2_field506.clone(), source_ml_source2_field504.clone(), source_ml_source2_field505, source_ml_source2_field507)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let count = 0f64;
                                                                            {
                                                                                let source_ml_source1_temporary_values110 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value508, _) => {
                                                                                        source_ml_source2_data_property_value508
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_temporary_index110 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let count_2 = self::source_ml_source1_temporary_define_java_script_ordinary_property_loop111(count, source_ml_source1_temporary_values110, source_ml_source1_temporary_index110.clone());
                                                                                        if ((at < (0f64)) && (count_2 >= (16384f64))) {
                                                                                            {
                                                                                                let source_ml_source2_field509 = heap.clone();
                                                                                                {
                                                                                                    let source_ml_source2_field510 = false;
                                                                                                    {
                                                                                                        let source_ml_source2_field511 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                                        {
                                                                                                            let source_ml_source2_field512 = Vec::<self::ml_string::Text>::new();
                                                                                                            (source_ml_source2_field511.clone(), source_ml_source2_field509.clone(), source_ml_source2_field510, source_ml_source2_field512)
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
                                                                                                            let source_ml_source2_concat_value514 = match object.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value513) => {
                                                                                                                    source_ml_source2_data_property_value513
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_concat_value515 = vec![updated.clone()];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), source_ml_source2_concat_value514), source_ml_source2_concat_value515)
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        { let ml_values2 = match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value516) => {
                                                                                                                source_ml_source2_data_property_value516
                                                                                                            }
                                                                                                        }; let ml_callback2 = { let at = at; let updated = updated.clone(); std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if (position == at) {
                                                                                                            updated.clone()
                                                                                                        } else {
                                                                                                            entry.clone()
                                                                                                        } }) }; ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2, (ml_index2 as f64))).collect::<Vec<_>>() }
                                                                                                    };
                                                                                                    {
                                                                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value517, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value517.clone()
                                                                                                            }
                                                                                                        }, Box::new(match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value518, _, _, _) => {
                                                                                                                let source_ml_source2_data_property_value518 = *source_ml_source2_data_property_value518;
                                                                                                                source_ml_source2_data_property_value518.clone()
                                                                                                            }
                                                                                                        }), match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value519, _, _) => {
                                                                                                                source_ml_source2_data_property_value519
                                                                                                            }
                                                                                                        }, match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value520, _) => {
                                                                                                                source_ml_source2_data_property_value520
                                                                                                            }
                                                                                                        }, properties);
                                                                                                        {
                                                                                                            let source_ml_source2_field521 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                                                            {
                                                                                                                let source_ml_source2_field522 = true;
                                                                                                                {
                                                                                                                    let source_ml_source2_field523 = self::ml_string::Text::from_units(vec![]);
                                                                                                                    {
                                                                                                                        let source_ml_source2_field524 = (transition.clone()).3;
                                                                                                                        (source_ml_source2_field523.clone(), source_ml_source2_field521.clone(), source_ml_source2_field522, source_ml_source2_field524)
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
                        let source_ml_source2_field525 = heap.clone();
                        {
                            let source_ml_source2_field526 = false;
                            {
                                let source_ml_source2_field527 = error.clone();
                                (source_ml_source2_field527.clone(), source_ml_source2_field525.clone(), source_ml_source2_field526)
                            }
                        }
                    }
                } else {
                    {
                        let index = self::find_java_script_ordinary_object(match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value528, _) => {
                                source_ml_source2_data_property_value528
                            }
                        }, identity.clone());
                        {
                            let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value529, _) => source_ml_source2_data_property_value529.as_slice() }), self::ml_array::number_index(index));
                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                {
                                    let source_ml_source2_field530 = heap.clone();
                                    {
                                        let source_ml_source2_field531 = false;
                                        {
                                            let source_ml_source2_field532 = self::ml_string::Text::from_units(vec![68, 101, 108, 101, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                            (source_ml_source2_field532.clone(), source_ml_source2_field530.clone(), source_ml_source2_field531)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value533) => {
                                            source_ml_source2_data_property_value533
                                        }
                                    }, key.clone());
                                    {
                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value534) => {
                                                source_ml_source2_data_property_value534
                                            }
                                        }, self::ml_array::number_index(at));
                                        if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source2_field535 = heap.clone();
                                                {
                                                    let source_ml_source2_field536 = true;
                                                    {
                                                        let source_ml_source2_field537 = self::ml_string::Text::from_units(vec![]);
                                                        (source_ml_source2_field537.clone(), source_ml_source2_field535.clone(), source_ml_source2_field536)
                                                    }
                                                }
                                            }
                                        } else {
                                            if !((match property.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value538) => {
                                                    source_ml_source2_data_property_value538.clone()
                                                }
                                            }).0) {
                                                {
                                                    let source_ml_source2_field539 = heap.clone();
                                                    {
                                                        let source_ml_source2_field540 = false;
                                                        {
                                                            let source_ml_source2_field541 = self::ml_string::Text::from_units(vec![]);
                                                            (source_ml_source2_field541.clone(), source_ml_source2_field539.clone(), source_ml_source2_field540)
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let properties = { let ml_values3 = match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value542) => {
                                                            source_ml_source2_data_property_value542
                                                        }
                                                    }; let ml_callback3 = { let at = at; std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> bool { (position != at) }) }; ml_values3.iter().cloned().enumerate().filter_map(|(ml_index3, ml_value3)| if ml_callback3(ml_value3.clone(), (ml_index3 as f64)) { Some(ml_value3) } else { None }).collect::<Vec<_>>() };
                                                    {
                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value543, _, _, _, _) => {
                                                                source_ml_source2_data_property_value543.clone()
                                                            }
                                                        }, Box::new(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value544, _, _, _) => {
                                                                let source_ml_source2_data_property_value544 = *source_ml_source2_data_property_value544;
                                                                source_ml_source2_data_property_value544.clone()
                                                            }
                                                        }), match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value545, _, _) => {
                                                                source_ml_source2_data_property_value545
                                                            }
                                                        }, match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value546, _) => {
                                                                source_ml_source2_data_property_value546
                                                            }
                                                        }, properties);
                                                        {
                                                            let source_ml_source2_field547 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                            {
                                                                let source_ml_source2_field548 = true;
                                                                {
                                                                    let source_ml_source2_field549 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source2_field549.clone(), source_ml_source2_field547.clone(), source_ml_source2_field548)
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
                let source_ml_source2_field550 = heap.clone();
                {
                    let source_ml_source2_field551 = false;
                    {
                        let source_ml_source2_field552 = error.clone();
                        (source_ml_source2_field552.clone(), source_ml_source2_field550.clone(), source_ml_source2_field551)
                    }
                }
            }
        } else {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value553, _) => {
                        source_ml_source2_data_property_value553
                    }
                }, identity.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value554, _) => source_ml_source2_data_property_value554.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        {
                            let source_ml_source2_field555 = heap.clone();
                            {
                                let source_ml_source2_field556 = false;
                                {
                                    let source_ml_source2_field557 = self::ml_string::Text::from_units(vec![80, 114, 101, 118, 101, 110, 116, 69, 120, 116, 101, 110, 115, 105, 111, 110, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                    (source_ml_source2_field557.clone(), source_ml_source2_field555.clone(), source_ml_source2_field556)
                                }
                            }
                        }
                    } else {
                        {
                            let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value558, _, _, _, _) => {
                                    source_ml_source2_data_property_value558.clone()
                                }
                            }, Box::new(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value559, _, _, _) => {
                                    let source_ml_source2_data_property_value559 = *source_ml_source2_data_property_value559;
                                    source_ml_source2_data_property_value559.clone()
                                }
                            }), false, match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value560, _) => {
                                    source_ml_source2_data_property_value560
                                }
                            }, match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value561) => {
                                    source_ml_source2_data_property_value561
                                }
                            });
                            {
                                let source_ml_source2_field562 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                {
                                    let source_ml_source2_field563 = true;
                                    {
                                        let source_ml_source2_field564 = self::ml_string::Text::from_units(vec![]);
                                        (source_ml_source2_field564.clone(), source_ml_source2_field562.clone(), source_ml_source2_field563)
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
