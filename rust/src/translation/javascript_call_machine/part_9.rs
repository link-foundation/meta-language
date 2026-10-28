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
use super::source_ml_temporary_define_java_script_ordinary_property_loop53;

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
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value319, _) => {
                                        source_ml_source1_data_property_value319
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
                            let source_ml_source1_field320 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                            {
                                let source_ml_source1_field321 = error.clone();
                                {
                                    let source_ml_source1_field322 = self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source1_field323 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source1_field324 = receiver.clone();
                                            {
                                                let source_ml_source1_field325 = Vec::<self::JavaScriptSemanticValue>::new();
                                                (source_ml_source1_field320.clone(), source_ml_source1_field325, source_ml_source1_field323.clone(), source_ml_source1_field321.clone(), source_ml_source1_field324.clone(), source_ml_source1_field322.clone())
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
                                    let source_ml_source1_field326 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                    {
                                        let source_ml_source1_field327 = self::ml_string::Text::from_units(vec![]);
                                        {
                                            let source_ml_source1_field328 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field329 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source1_field330 = receiver.clone();
                                                    {
                                                        let source_ml_source1_field331 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source1_field326.clone(), source_ml_source1_field331, source_ml_source1_field329.clone(), source_ml_source1_field327.clone(), source_ml_source1_field330.clone(), source_ml_source1_field328.clone())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if ((property.clone()).0).9 {
                                    {
                                        let source_ml_source1_field332 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                        {
                                            let source_ml_source1_field333 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source1_field334 = ((property.clone()).0).8;
                                                {
                                                    let source_ml_source1_field335 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source1_field336 = receiver.clone();
                                                        {
                                                            let source_ml_source1_field337 = Vec::<self::JavaScriptSemanticValue>::new();
                                                            (source_ml_source1_field332.clone(), source_ml_source1_field337, source_ml_source1_field335.clone(), source_ml_source1_field333.clone(), source_ml_source1_field336.clone(), source_ml_source1_field334.clone())
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
                                                    let source_ml_source1_field345 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                                    {
                                                        let source_ml_source1_field346 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field347 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field348 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_field349 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field350 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field345.clone(), source_ml_source1_field350, source_ml_source1_field348.clone(), source_ml_source1_field346.clone(), source_ml_source1_field349.clone(), source_ml_source1_field347.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                {
                                                    let source_ml_source1_field351 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field352 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field353 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field354 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field355 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field356 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field351.clone(), source_ml_source1_field356, source_ml_source1_field354.clone(), source_ml_source1_field352.clone(), source_ml_source1_field355.clone(), source_ml_source1_field353.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f338) => {
                                                {
                                                    let source_ml_source1_field357 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field358 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field359 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field360 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field361 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field362 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field357.clone(), source_ml_source1_field362, source_ml_source1_field360.clone(), source_ml_source1_field358.clone(), source_ml_source1_field361.clone(), source_ml_source1_field359.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source1_f339) => {
                                                {
                                                    let source_ml_source1_field363 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field364 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field365 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field366 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field367 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field368 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field363.clone(), source_ml_source1_field368, source_ml_source1_field366.clone(), source_ml_source1_field364.clone(), source_ml_source1_field367.clone(), source_ml_source1_field365.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f340) => {
                                                {
                                                    let source_ml_source1_field369 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field370 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field371 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field372 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field373 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field374 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field369.clone(), source_ml_source1_field374, source_ml_source1_field372.clone(), source_ml_source1_field370.clone(), source_ml_source1_field373.clone(), source_ml_source1_field371.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source1_f341) => {
                                                {
                                                    let source_ml_source1_field375 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field376 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field377 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field378 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field379 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field380 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field375.clone(), source_ml_source1_field380, source_ml_source1_field378.clone(), source_ml_source1_field376.clone(), source_ml_source1_field379.clone(), source_ml_source1_field377.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f342) => {
                                                {
                                                    let source_ml_source1_field381 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field382 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field383 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field384 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field385 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field386 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field381.clone(), source_ml_source1_field386, source_ml_source1_field384.clone(), source_ml_source1_field382.clone(), source_ml_source1_field385.clone(), source_ml_source1_field383.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Object(source_ml_source1_f343, source_ml_source1_f344) => {
                                                {
                                                    let source_ml_source1_field387 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field388 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field389 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field390 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field391 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field392 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field387.clone(), source_ml_source1_field392, source_ml_source1_field390.clone(), source_ml_source1_field388.clone(), source_ml_source1_field391.clone(), source_ml_source1_field389.clone())
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
                        let source_ml_source1_field393 = heap.clone();
                        {
                            let source_ml_source1_field394 = false;
                            {
                                let source_ml_source1_field395 = error.clone();
                                {
                                    let source_ml_source1_field396 = Vec::<self::ml_string::Text>::new();
                                    (source_ml_source1_field395.clone(), source_ml_source1_field393.clone(), source_ml_source1_field394, source_ml_source1_field396)
                                }
                            }
                        }
                    }
                } else {
                    if ((proposed.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                        {
                            let source_ml_source1_field397 = heap.clone();
                            {
                                let source_ml_source1_field398 = false;
                                {
                                    let source_ml_source1_field399 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                    {
                                        let source_ml_source1_field400 = Vec::<self::ml_string::Text>::new();
                                        (source_ml_source1_field399.clone(), source_ml_source1_field397.clone(), source_ml_source1_field398, source_ml_source1_field400)
                                    }
                                }
                            }
                        }
                    } else {
                        if ((proposed.clone()).5 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).4)) != (self::ml_string::Text::from_units(vec![])))) {
                            {
                                let source_ml_source1_field401 = heap.clone();
                                {
                                    let source_ml_source1_field402 = false;
                                    {
                                        let source_ml_source1_field403 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 103, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                        {
                                            let source_ml_source1_field404 = Vec::<self::ml_string::Text>::new();
                                            (source_ml_source1_field403.clone(), source_ml_source1_field401.clone(), source_ml_source1_field402, source_ml_source1_field404)
                                        }
                                    }
                                }
                            }
                        } else {
                            if ((proposed.clone()).7 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).6)) != (self::ml_string::Text::from_units(vec![])))) {
                                {
                                    let source_ml_source1_field405 = heap.clone();
                                    {
                                        let source_ml_source1_field406 = false;
                                        {
                                            let source_ml_source1_field407 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 115, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                            {
                                                let source_ml_source1_field408 = Vec::<self::ml_string::Text>::new();
                                                (source_ml_source1_field407.clone(), source_ml_source1_field405.clone(), source_ml_source1_field406, source_ml_source1_field408)
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index = self::find_java_script_ordinary_object(match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value409, _) => {
                                            source_ml_source1_data_property_value409
                                        }
                                    }, identity.clone());
                                    {
                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value410, _) => source_ml_source1_data_property_value410.as_slice() }), self::ml_array::number_index(index));
                                        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source1_field411 = heap.clone();
                                                {
                                                    let source_ml_source1_field412 = false;
                                                    {
                                                        let source_ml_source1_field413 = self::ml_string::Text::from_units(vec![68, 101, 102, 105, 110, 101, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                                        {
                                                            let source_ml_source1_field414 = Vec::<self::ml_string::Text>::new();
                                                            (source_ml_source1_field413.clone(), source_ml_source1_field411.clone(), source_ml_source1_field412, source_ml_source1_field414)
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value415) => {
                                                        source_ml_source1_data_property_value415
                                                    }
                                                }, key.clone());
                                                {
                                                    let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value416) => {
                                                            source_ml_source1_data_property_value416
                                                        }
                                                    }, self::ml_array::number_index(at));
                                                    {
                                                        let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false)
                                                        } else {
                                                            match property.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value417) => {
                                                                    source_ml_source1_data_property_value417.clone()
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let transition = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value418, _, _) => {
                                                                    source_ml_source1_data_property_value418
                                                                }
                                                            }, current.clone(), proposed.clone());
                                                            if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                {
                                                                    let source_ml_source1_field419 = heap.clone();
                                                                    {
                                                                        let source_ml_source1_field420 = false;
                                                                        {
                                                                            let source_ml_source1_field421 = (transition.clone()).2;
                                                                            {
                                                                                let source_ml_source1_field422 = Vec::<self::ml_string::Text>::new();
                                                                                (source_ml_source1_field421.clone(), source_ml_source1_field419.clone(), source_ml_source1_field420, source_ml_source1_field422)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                                                                    {
                                                                        let source_ml_source1_field423 = heap.clone();
                                                                        {
                                                                            let source_ml_source1_field424 = true;
                                                                            {
                                                                                let source_ml_source1_field425 = self::ml_string::Text::from_units(vec![]);
                                                                                {
                                                                                    let source_ml_source1_field426 = Vec::<self::ml_string::Text>::new();
                                                                                    (source_ml_source1_field425.clone(), source_ml_source1_field423.clone(), source_ml_source1_field424, source_ml_source1_field426)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if ((at < (0f64)) && ((((match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value428) => {
                                                                            source_ml_source1_data_property_value428
                                                                        }
                                                                    }).len() as f64)) >= (4096f64))) {
                                                                        {
                                                                            let source_ml_source1_field429 = heap.clone();
                                                                            {
                                                                                let source_ml_source1_field430 = false;
                                                                                {
                                                                                    let source_ml_source1_field431 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                    {
                                                                                        let source_ml_source1_field432 = Vec::<self::ml_string::Text>::new();
                                                                                        (source_ml_source1_field431.clone(), source_ml_source1_field429.clone(), source_ml_source1_field430, source_ml_source1_field432)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let count = 0f64;
                                                                            {
                                                                                let source_ml_temporary_values52 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value433, _) => {
                                                                                        source_ml_source1_data_property_value433
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_temporary_index52 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let count_2 = self::source_ml_temporary_define_java_script_ordinary_property_loop53(count, source_ml_temporary_values52, source_ml_temporary_index52.clone());
                                                                                        if ((at < (0f64)) && (count_2 >= (16384f64))) {
                                                                                            {
                                                                                                let source_ml_source1_field434 = heap.clone();
                                                                                                {
                                                                                                    let source_ml_source1_field435 = false;
                                                                                                    {
                                                                                                        let source_ml_source1_field436 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                                        {
                                                                                                            let source_ml_source1_field437 = Vec::<self::ml_string::Text>::new();
                                                                                                            (source_ml_source1_field436.clone(), source_ml_source1_field434.clone(), source_ml_source1_field435, source_ml_source1_field437)
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
                                                                                                            let source_ml_source1_concat_value439 = match object.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value438) => {
                                                                                                                    source_ml_source1_data_property_value438
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value440 = vec![updated.clone()];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), source_ml_source1_concat_value439), source_ml_source1_concat_value440)
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        { let ml_values2 = match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value441) => {
                                                                                                                source_ml_source1_data_property_value441
                                                                                                            }
                                                                                                        }; let ml_callback2 = { let at = at; let updated = updated.clone(); std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if (position == at) {
                                                                                                            updated.clone()
                                                                                                        } else {
                                                                                                            entry.clone()
                                                                                                        } }) }; ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2, (ml_index2 as f64))).collect::<Vec<_>>() }
                                                                                                    };
                                                                                                    {
                                                                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value442, _, _, _, _) => {
                                                                                                                source_ml_source1_data_property_value442.clone()
                                                                                                            }
                                                                                                        }, Box::new(match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value443, _, _, _) => {
                                                                                                                let source_ml_source1_data_property_value443 = *source_ml_source1_data_property_value443;
                                                                                                                source_ml_source1_data_property_value443.clone()
                                                                                                            }
                                                                                                        }), match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value444, _, _) => {
                                                                                                                source_ml_source1_data_property_value444
                                                                                                            }
                                                                                                        }, match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value445, _) => {
                                                                                                                source_ml_source1_data_property_value445
                                                                                                            }
                                                                                                        }, properties);
                                                                                                        {
                                                                                                            let source_ml_source1_field446 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                                                            {
                                                                                                                let source_ml_source1_field447 = true;
                                                                                                                {
                                                                                                                    let source_ml_source1_field448 = self::ml_string::Text::from_units(vec![]);
                                                                                                                    {
                                                                                                                        let source_ml_source1_field449 = (transition.clone()).3;
                                                                                                                        (source_ml_source1_field448.clone(), source_ml_source1_field446.clone(), source_ml_source1_field447, source_ml_source1_field449)
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
                        let source_ml_source1_field450 = heap.clone();
                        {
                            let source_ml_source1_field451 = false;
                            {
                                let source_ml_source1_field452 = error.clone();
                                (source_ml_source1_field452.clone(), source_ml_source1_field450.clone(), source_ml_source1_field451)
                            }
                        }
                    }
                } else {
                    {
                        let index = self::find_java_script_ordinary_object(match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value453, _) => {
                                source_ml_source1_data_property_value453
                            }
                        }, identity.clone());
                        {
                            let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value454, _) => source_ml_source1_data_property_value454.as_slice() }), self::ml_array::number_index(index));
                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                {
                                    let source_ml_source1_field455 = heap.clone();
                                    {
                                        let source_ml_source1_field456 = false;
                                        {
                                            let source_ml_source1_field457 = self::ml_string::Text::from_units(vec![68, 101, 108, 101, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                            (source_ml_source1_field457.clone(), source_ml_source1_field455.clone(), source_ml_source1_field456)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value458) => {
                                            source_ml_source1_data_property_value458
                                        }
                                    }, key.clone());
                                    {
                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value459) => {
                                                source_ml_source1_data_property_value459
                                            }
                                        }, self::ml_array::number_index(at));
                                        if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source1_field460 = heap.clone();
                                                {
                                                    let source_ml_source1_field461 = true;
                                                    {
                                                        let source_ml_source1_field462 = self::ml_string::Text::from_units(vec![]);
                                                        (source_ml_source1_field462.clone(), source_ml_source1_field460.clone(), source_ml_source1_field461)
                                                    }
                                                }
                                            }
                                        } else {
                                            if !((match property.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value463) => {
                                                    source_ml_source1_data_property_value463.clone()
                                                }
                                            }).0) {
                                                {
                                                    let source_ml_source1_field464 = heap.clone();
                                                    {
                                                        let source_ml_source1_field465 = false;
                                                        {
                                                            let source_ml_source1_field466 = self::ml_string::Text::from_units(vec![]);
                                                            (source_ml_source1_field466.clone(), source_ml_source1_field464.clone(), source_ml_source1_field465)
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let properties = { let ml_values3 = match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value467) => {
                                                            source_ml_source1_data_property_value467
                                                        }
                                                    }; let ml_callback3 = { let at = at; std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> bool { (position != at) }) }; ml_values3.iter().cloned().enumerate().filter_map(|(ml_index3, ml_value3)| if ml_callback3(ml_value3.clone(), (ml_index3 as f64)) { Some(ml_value3) } else { None }).collect::<Vec<_>>() };
                                                    {
                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value468, _, _, _, _) => {
                                                                source_ml_source1_data_property_value468.clone()
                                                            }
                                                        }, Box::new(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value469, _, _, _) => {
                                                                let source_ml_source1_data_property_value469 = *source_ml_source1_data_property_value469;
                                                                source_ml_source1_data_property_value469.clone()
                                                            }
                                                        }), match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value470, _, _) => {
                                                                source_ml_source1_data_property_value470
                                                            }
                                                        }, match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value471, _) => {
                                                                source_ml_source1_data_property_value471
                                                            }
                                                        }, properties);
                                                        {
                                                            let source_ml_source1_field472 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                            {
                                                                let source_ml_source1_field473 = true;
                                                                {
                                                                    let source_ml_source1_field474 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source1_field474.clone(), source_ml_source1_field472.clone(), source_ml_source1_field473)
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
                let source_ml_source1_field475 = heap.clone();
                {
                    let source_ml_source1_field476 = false;
                    {
                        let source_ml_source1_field477 = error.clone();
                        (source_ml_source1_field477.clone(), source_ml_source1_field475.clone(), source_ml_source1_field476)
                    }
                }
            }
        } else {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value478, _) => {
                        source_ml_source1_data_property_value478
                    }
                }, identity.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value479, _) => source_ml_source1_data_property_value479.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        {
                            let source_ml_source1_field480 = heap.clone();
                            {
                                let source_ml_source1_field481 = false;
                                {
                                    let source_ml_source1_field482 = self::ml_string::Text::from_units(vec![80, 114, 101, 118, 101, 110, 116, 69, 120, 116, 101, 110, 115, 105, 111, 110, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                    (source_ml_source1_field482.clone(), source_ml_source1_field480.clone(), source_ml_source1_field481)
                                }
                            }
                        }
                    } else {
                        {
                            let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value483, _, _, _, _) => {
                                    source_ml_source1_data_property_value483.clone()
                                }
                            }, Box::new(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value484, _, _, _) => {
                                    let source_ml_source1_data_property_value484 = *source_ml_source1_data_property_value484;
                                    source_ml_source1_data_property_value484.clone()
                                }
                            }), false, match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value485, _) => {
                                    source_ml_source1_data_property_value485
                                }
                            }, match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value486) => {
                                    source_ml_source1_data_property_value486
                                }
                            });
                            {
                                let source_ml_source1_field487 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                {
                                    let source_ml_source1_field488 = true;
                                    {
                                        let source_ml_source1_field489 = self::ml_string::Text::from_units(vec![]);
                                        (source_ml_source1_field489.clone(), source_ml_source1_field487.clone(), source_ml_source1_field488)
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
