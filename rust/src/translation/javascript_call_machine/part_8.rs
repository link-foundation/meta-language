// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result;
use super::create_java_script_property_descriptor;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_property_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_temporary_define_java_script_ordinary_property_loop21;
use super::source_ml_temporary_set_java_script_ordinary_prototype_loop22;
use super::test_java_script_semantic_same_value;

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
                        let source_ml_source1_field338 = heap.clone();
                        {
                            let source_ml_source1_field339 = false;
                            {
                                let source_ml_source1_field340 = error.clone();
                                {
                                    let source_ml_source1_field341 = Vec::<self::ml_string::Text>::new();
                                    (source_ml_source1_field340.clone(), source_ml_source1_field338.clone(), source_ml_source1_field339, source_ml_source1_field341)
                                }
                            }
                        }
                    }
                } else {
                    if ((proposed.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                        {
                            let source_ml_source1_field342 = heap.clone();
                            {
                                let source_ml_source1_field343 = false;
                                {
                                    let source_ml_source1_field344 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                    {
                                        let source_ml_source1_field345 = Vec::<self::ml_string::Text>::new();
                                        (source_ml_source1_field344.clone(), source_ml_source1_field342.clone(), source_ml_source1_field343, source_ml_source1_field345)
                                    }
                                }
                            }
                        }
                    } else {
                        if ((proposed.clone()).5 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).4)) != (self::ml_string::Text::from_units(vec![])))) {
                            {
                                let source_ml_source1_field346 = heap.clone();
                                {
                                    let source_ml_source1_field347 = false;
                                    {
                                        let source_ml_source1_field348 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 103, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                        {
                                            let source_ml_source1_field349 = Vec::<self::ml_string::Text>::new();
                                            (source_ml_source1_field348.clone(), source_ml_source1_field346.clone(), source_ml_source1_field347, source_ml_source1_field349)
                                        }
                                    }
                                }
                            }
                        } else {
                            if ((proposed.clone()).7 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).6)) != (self::ml_string::Text::from_units(vec![])))) {
                                {
                                    let source_ml_source1_field350 = heap.clone();
                                    {
                                        let source_ml_source1_field351 = false;
                                        {
                                            let source_ml_source1_field352 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 115, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                            {
                                                let source_ml_source1_field353 = Vec::<self::ml_string::Text>::new();
                                                (source_ml_source1_field352.clone(), source_ml_source1_field350.clone(), source_ml_source1_field351, source_ml_source1_field353)
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index = self::find_java_script_ordinary_object(match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value354, _) => {
                                            source_ml_source1_data_property_value354
                                        }
                                    }, identity.clone());
                                    {
                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value355, _) => source_ml_source1_data_property_value355.as_slice() }), self::ml_array::number_index(index));
                                        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source1_field356 = heap.clone();
                                                {
                                                    let source_ml_source1_field357 = false;
                                                    {
                                                        let source_ml_source1_field358 = self::ml_string::Text::from_units(vec![68, 101, 102, 105, 110, 101, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                                        {
                                                            let source_ml_source1_field359 = Vec::<self::ml_string::Text>::new();
                                                            (source_ml_source1_field358.clone(), source_ml_source1_field356.clone(), source_ml_source1_field357, source_ml_source1_field359)
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value360) => {
                                                        source_ml_source1_data_property_value360
                                                    }
                                                }, key.clone());
                                                {
                                                    let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value361) => {
                                                            source_ml_source1_data_property_value361
                                                        }
                                                    }, self::ml_array::number_index(at));
                                                    {
                                                        let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false)
                                                        } else {
                                                            match property.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value362) => {
                                                                    source_ml_source1_data_property_value362.clone()
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let transition = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value363, _, _) => {
                                                                    source_ml_source1_data_property_value363
                                                                }
                                                            }, current.clone(), proposed.clone());
                                                            if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                {
                                                                    let source_ml_source1_field364 = heap.clone();
                                                                    {
                                                                        let source_ml_source1_field365 = false;
                                                                        {
                                                                            let source_ml_source1_field366 = (transition.clone()).2;
                                                                            {
                                                                                let source_ml_source1_field367 = Vec::<self::ml_string::Text>::new();
                                                                                (source_ml_source1_field366.clone(), source_ml_source1_field364.clone(), source_ml_source1_field365, source_ml_source1_field367)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                                                                    {
                                                                        let source_ml_source1_field368 = heap.clone();
                                                                        {
                                                                            let source_ml_source1_field369 = true;
                                                                            {
                                                                                let source_ml_source1_field370 = self::ml_string::Text::from_units(vec![]);
                                                                                {
                                                                                    let source_ml_source1_field371 = Vec::<self::ml_string::Text>::new();
                                                                                    (source_ml_source1_field370.clone(), source_ml_source1_field368.clone(), source_ml_source1_field369, source_ml_source1_field371)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if ((at < (0f64)) && ((((match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value373) => {
                                                                            source_ml_source1_data_property_value373
                                                                        }
                                                                    }).len() as f64)) >= (4096f64))) {
                                                                        {
                                                                            let source_ml_source1_field374 = heap.clone();
                                                                            {
                                                                                let source_ml_source1_field375 = false;
                                                                                {
                                                                                    let source_ml_source1_field376 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                    {
                                                                                        let source_ml_source1_field377 = Vec::<self::ml_string::Text>::new();
                                                                                        (source_ml_source1_field376.clone(), source_ml_source1_field374.clone(), source_ml_source1_field375, source_ml_source1_field377)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let count = 0f64;
                                                                            {
                                                                                let source_ml_temporary_values20 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value378, _) => {
                                                                                        source_ml_source1_data_property_value378
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_temporary_index20 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let count_2 = self::source_ml_temporary_define_java_script_ordinary_property_loop21(count, source_ml_temporary_values20, source_ml_temporary_index20.clone());
                                                                                        if ((at < (0f64)) && (count_2 >= (16384f64))) {
                                                                                            {
                                                                                                let source_ml_source1_field379 = heap.clone();
                                                                                                {
                                                                                                    let source_ml_source1_field380 = false;
                                                                                                    {
                                                                                                        let source_ml_source1_field381 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                                        {
                                                                                                            let source_ml_source1_field382 = Vec::<self::ml_string::Text>::new();
                                                                                                            (source_ml_source1_field381.clone(), source_ml_source1_field379.clone(), source_ml_source1_field380, source_ml_source1_field382)
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
                                                                                                            let source_ml_source1_concat_value384 = match object.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value383) => {
                                                                                                                    source_ml_source1_data_property_value383
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value385 = vec![updated.clone()];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), source_ml_source1_concat_value384), source_ml_source1_concat_value385)
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        { let ml_values2 = match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value386) => {
                                                                                                                source_ml_source1_data_property_value386
                                                                                                            }
                                                                                                        }; let ml_callback2 = { let at = at; let updated = updated.clone(); std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if (position == at) {
                                                                                                            updated.clone()
                                                                                                        } else {
                                                                                                            entry.clone()
                                                                                                        } }) }; ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2, (ml_index2 as f64))).collect::<Vec<_>>() }
                                                                                                    };
                                                                                                    {
                                                                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value387, _, _, _, _) => {
                                                                                                                source_ml_source1_data_property_value387.clone()
                                                                                                            }
                                                                                                        }, Box::new(match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value388, _, _, _) => {
                                                                                                                let source_ml_source1_data_property_value388 = *source_ml_source1_data_property_value388;
                                                                                                                source_ml_source1_data_property_value388.clone()
                                                                                                            }
                                                                                                        }), match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value389, _, _) => {
                                                                                                                source_ml_source1_data_property_value389
                                                                                                            }
                                                                                                        }, match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value390, _) => {
                                                                                                                source_ml_source1_data_property_value390
                                                                                                            }
                                                                                                        }, properties);
                                                                                                        {
                                                                                                            let source_ml_source1_field391 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                                                            {
                                                                                                                let source_ml_source1_field392 = true;
                                                                                                                {
                                                                                                                    let source_ml_source1_field393 = self::ml_string::Text::from_units(vec![]);
                                                                                                                    {
                                                                                                                        let source_ml_source1_field394 = (transition.clone()).3;
                                                                                                                        (source_ml_source1_field393.clone(), source_ml_source1_field391.clone(), source_ml_source1_field392, source_ml_source1_field394)
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
                                            let source_ml_source1_field402 = self::ml_string::Text::from_units(vec![68, 101, 108, 101, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                            (source_ml_source1_field402.clone(), source_ml_source1_field400.clone(), source_ml_source1_field401)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value403) => {
                                            source_ml_source1_data_property_value403
                                        }
                                    }, key.clone());
                                    {
                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value404) => {
                                                source_ml_source1_data_property_value404
                                            }
                                        }, self::ml_array::number_index(at));
                                        if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source1_field405 = heap.clone();
                                                {
                                                    let source_ml_source1_field406 = true;
                                                    {
                                                        let source_ml_source1_field407 = self::ml_string::Text::from_units(vec![]);
                                                        (source_ml_source1_field407.clone(), source_ml_source1_field405.clone(), source_ml_source1_field406)
                                                    }
                                                }
                                            }
                                        } else {
                                            if !((match property.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value408) => {
                                                    source_ml_source1_data_property_value408.clone()
                                                }
                                            }).0) {
                                                {
                                                    let source_ml_source1_field409 = heap.clone();
                                                    {
                                                        let source_ml_source1_field410 = false;
                                                        {
                                                            let source_ml_source1_field411 = self::ml_string::Text::from_units(vec![]);
                                                            (source_ml_source1_field411.clone(), source_ml_source1_field409.clone(), source_ml_source1_field410)
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let properties = { let ml_values3 = match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value412) => {
                                                            source_ml_source1_data_property_value412
                                                        }
                                                    }; let ml_callback3 = { let at = at; std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> bool { (position != at) }) }; ml_values3.iter().cloned().enumerate().filter_map(|(ml_index3, ml_value3)| if ml_callback3(ml_value3.clone(), (ml_index3 as f64)) { Some(ml_value3) } else { None }).collect::<Vec<_>>() };
                                                    {
                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value413, _, _, _, _) => {
                                                                source_ml_source1_data_property_value413.clone()
                                                            }
                                                        }, Box::new(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value414, _, _, _) => {
                                                                let source_ml_source1_data_property_value414 = *source_ml_source1_data_property_value414;
                                                                source_ml_source1_data_property_value414.clone()
                                                            }
                                                        }), match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value415, _, _) => {
                                                                source_ml_source1_data_property_value415
                                                            }
                                                        }, match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value416, _) => {
                                                                source_ml_source1_data_property_value416
                                                            }
                                                        }, properties);
                                                        {
                                                            let source_ml_source1_field417 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                            {
                                                                let source_ml_source1_field418 = true;
                                                                {
                                                                    let source_ml_source1_field419 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source1_field419.clone(), source_ml_source1_field417.clone(), source_ml_source1_field418)
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
                let source_ml_source1_field420 = heap.clone();
                {
                    let source_ml_source1_field421 = false;
                    {
                        let source_ml_source1_field422 = error.clone();
                        (source_ml_source1_field422.clone(), source_ml_source1_field420.clone(), source_ml_source1_field421)
                    }
                }
            }
        } else {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value423, _) => {
                        source_ml_source1_data_property_value423
                    }
                }, identity.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value424, _) => source_ml_source1_data_property_value424.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        {
                            let source_ml_source1_field425 = heap.clone();
                            {
                                let source_ml_source1_field426 = false;
                                {
                                    let source_ml_source1_field427 = self::ml_string::Text::from_units(vec![80, 114, 101, 118, 101, 110, 116, 69, 120, 116, 101, 110, 115, 105, 111, 110, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                    (source_ml_source1_field427.clone(), source_ml_source1_field425.clone(), source_ml_source1_field426)
                                }
                            }
                        }
                    } else {
                        {
                            let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value428, _, _, _, _) => {
                                    source_ml_source1_data_property_value428.clone()
                                }
                            }, Box::new(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value429, _, _, _) => {
                                    let source_ml_source1_data_property_value429 = *source_ml_source1_data_property_value429;
                                    source_ml_source1_data_property_value429.clone()
                                }
                            }), false, match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value430, _) => {
                                    source_ml_source1_data_property_value430
                                }
                            }, match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value431) => {
                                    source_ml_source1_data_property_value431
                                }
                            });
                            {
                                let source_ml_source1_field432 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                {
                                    let source_ml_source1_field433 = true;
                                    {
                                        let source_ml_source1_field434 = self::ml_string::Text::from_units(vec![]);
                                        (source_ml_source1_field434.clone(), source_ml_source1_field432.clone(), source_ml_source1_field433)
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
                let source_ml_source1_field435 = heap.clone();
                {
                    let source_ml_source1_field436 = false;
                    {
                        let source_ml_source1_field437 = heap_error.clone();
                        (source_ml_source1_field437.clone(), source_ml_source1_field435.clone(), source_ml_source1_field436)
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f438) => {
                    true
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f439) => {
                    true
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f440) => {
                    true
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f441) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f442) => {
                    true
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f443, source_ml_source1_f444) => {
                    true
                }
            } && match prototype.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    true
                }
                self::JavaScriptSemanticValue::Null => {
                    true
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f445) => {
                    true
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f446) => {
                    true
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f447) => {
                    true
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f448) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f449) => {
                    true
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f450, source_ml_source1_f451) => {
                    false
                }
            }) {
                {
                    let source_ml_source1_field452 = heap.clone();
                    {
                        let source_ml_source1_field453 = false;
                        {
                            let source_ml_source1_field454 = self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]);
                            (source_ml_source1_field454.clone(), source_ml_source1_field452.clone(), source_ml_source1_field453)
                        }
                    }
                }
            } else {
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        {
                            let source_ml_source1_field455 = heap.clone();
                            {
                                let source_ml_source1_field456 = false;
                                {
                                    let source_ml_source1_field457 = error.clone();
                                    (source_ml_source1_field457.clone(), source_ml_source1_field455.clone(), source_ml_source1_field456)
                                }
                            }
                        }
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value458, _) => {
                                    source_ml_source1_data_property_value458
                                }
                            }, identity.clone());
                            {
                                let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value459, _) => source_ml_source1_data_property_value459.as_slice() }), self::ml_array::number_index(index));
                                if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    {
                                        let source_ml_source1_field460 = heap.clone();
                                        {
                                            let source_ml_source1_field461 = false;
                                            {
                                                let source_ml_source1_field462 = self::ml_string::Text::from_units(vec![83, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                                (source_ml_source1_field462.clone(), source_ml_source1_field460.clone(), source_ml_source1_field461)
                                            }
                                        }
                                    }
                                } else {
                                    if self::test_java_script_semantic_same_value(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value463, _, _, _) => {
                                            let source_ml_source1_data_property_value463 = *source_ml_source1_data_property_value463;
                                            source_ml_source1_data_property_value463.clone()
                                        }
                                    }, prototype.clone()) {
                                        {
                                            let source_ml_source1_field464 = heap.clone();
                                            {
                                                let source_ml_source1_field465 = true;
                                                {
                                                    let source_ml_source1_field466 = self::ml_string::Text::from_units(vec![]);
                                                    (source_ml_source1_field466.clone(), source_ml_source1_field464.clone(), source_ml_source1_field465)
                                                }
                                            }
                                        }
                                    } else {
                                        if !(match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value467, _, _) => {
                                                source_ml_source1_data_property_value467
                                            }
                                        }) {
                                            {
                                                let source_ml_source1_field468 = heap.clone();
                                                {
                                                    let source_ml_source1_field469 = false;
                                                    {
                                                        let source_ml_source1_field470 = self::ml_string::Text::from_units(vec![]);
                                                        (source_ml_source1_field470.clone(), source_ml_source1_field468.clone(), source_ml_source1_field469)
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let next = prototype.clone();
                                                {
                                                    let depth = 0f64;
                                                    {
                                                        let source_ml_source1_s471 = self::source_ml_temporary_set_java_script_ordinary_prototype_loop22(heap.clone(), identity.clone(), next.clone(), depth);
                                                        match source_ml_source1_s471.clone() {
                                                            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Done => {
                                                                {
                                                                    let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value472, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value472.clone()
                                                                        }
                                                                    }, Box::new(prototype.clone()), match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value473, _, _) => {
                                                                            source_ml_source1_data_property_value473
                                                                        }
                                                                    }, match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value474, _) => {
                                                                            source_ml_source1_data_property_value474
                                                                        }
                                                                    }, match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value475) => {
                                                                            source_ml_source1_data_property_value475
                                                                        }
                                                                    });
                                                                    {
                                                                        let source_ml_source1_field476 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                        {
                                                                            let source_ml_source1_field477 = true;
                                                                            {
                                                                                let source_ml_source1_field478 = self::ml_string::Text::from_units(vec![]);
                                                                                (source_ml_source1_field478.clone(), source_ml_source1_field476.clone(), source_ml_source1_field477)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Return(source_ml_temporary_result22) => {
                                                                source_ml_temporary_result22.clone()
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
        let source_ml_source1_field479 = creation;
        {
            let source_ml_source1_field480 = creation;
            {
                let source_ml_source1_field481 = creation;
                {
                    let source_ml_source1_field482 = creation;
                    {
                        let source_ml_source1_field483 = true;
                        {
                            let source_ml_source1_field484 = value.clone();
                            {
                                let source_ml_source1_field485 = creation;
                                {
                                    let source_ml_source1_field486 = creation;
                                    {
                                        let source_ml_source1_field487 = false;
                                        {
                                            let source_ml_source1_field488 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field489 = false;
                                                {
                                                    let source_ml_source1_field490 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field480, source_ml_source1_field479, source_ml_source1_field482, source_ml_source1_field481, source_ml_source1_field488.clone(), source_ml_source1_field487, source_ml_source1_field490.clone(), source_ml_source1_field489, source_ml_source1_field484.clone(), source_ml_source1_field483, source_ml_source1_field486, source_ml_source1_field485)
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
