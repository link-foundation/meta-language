// Generated from complete js/src/translation/javascript-property-definition.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_temporary_set_java_script_ordinary_prototype_loop21;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn set_java_script_ordinary_prototype(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, prototype: self::JavaScriptSemanticValue) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source1_field458 = heap.clone();
                {
                    let source_ml_source1_field459 = false;
                    {
                        let source_ml_source1_field460 = heap_error.clone();
                        (source_ml_source1_field460.clone(), source_ml_source1_field458.clone(), source_ml_source1_field459)
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f461) => {
                    true
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f462) => {
                    true
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f463) => {
                    true
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f464) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f465) => {
                    true
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f466, source_ml_source1_f467) => {
                    true
                }
            } && match prototype.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    true
                }
                self::JavaScriptSemanticValue::Null => {
                    true
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f468) => {
                    true
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f469) => {
                    true
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f470) => {
                    true
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f471) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f472) => {
                    true
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f473, source_ml_source1_f474) => {
                    false
                }
            }) {
                {
                    let source_ml_source1_field475 = heap.clone();
                    {
                        let source_ml_source1_field476 = false;
                        {
                            let source_ml_source1_field477 = self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]);
                            (source_ml_source1_field477.clone(), source_ml_source1_field475.clone(), source_ml_source1_field476)
                        }
                    }
                }
            } else {
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        {
                            let source_ml_source1_field478 = heap.clone();
                            {
                                let source_ml_source1_field479 = false;
                                {
                                    let source_ml_source1_field480 = error.clone();
                                    (source_ml_source1_field480.clone(), source_ml_source1_field478.clone(), source_ml_source1_field479)
                                }
                            }
                        }
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value481, _) => {
                                    source_ml_source1_data_property_value481
                                }
                            }, identity.clone());
                            {
                                let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value482, _) => source_ml_source1_data_property_value482.as_slice() }), self::ml_array::number_index(index));
                                if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    {
                                        let source_ml_source1_field483 = heap.clone();
                                        {
                                            let source_ml_source1_field484 = false;
                                            {
                                                let source_ml_source1_field485 = self::ml_string::Text::from_units(vec![83, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                                (source_ml_source1_field485.clone(), source_ml_source1_field483.clone(), source_ml_source1_field484)
                                            }
                                        }
                                    }
                                } else {
                                    if self::test_java_script_semantic_same_value({
                                        let source_ml_source1_nullish_field_receiver486 = (object.clone(),);
                                        if match (source_ml_source1_nullish_field_receiver486.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            {
                                                let source_ml_source1_nullish_field_present487 = ((source_ml_source1_nullish_field_receiver486.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source1_nullish_field_present487.clone() {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value488, _, _, _) => {
                                                        let source_ml_source1_data_property_value488 = *source_ml_source1_data_property_value488;
                                                        source_ml_source1_data_property_value488.clone()
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                                        }
                                    }, prototype.clone()) {
                                        {
                                            let source_ml_source1_field489 = heap.clone();
                                            {
                                                let source_ml_source1_field490 = true;
                                                {
                                                    let source_ml_source1_field491 = self::ml_string::Text::from_units(vec![]);
                                                    (source_ml_source1_field491.clone(), source_ml_source1_field489.clone(), source_ml_source1_field490)
                                                }
                                            }
                                        }
                                    } else {
                                        if !({
                                            let source_ml_source1_nullish_field_receiver492 = (object.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver492.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present493 = ((source_ml_source1_nullish_field_receiver492.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present493.clone() {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value494, _, _) => {
                                                            source_ml_source1_data_property_value494
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 120, 116, 101, 110, 115, 105, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }) {
                                            {
                                                let source_ml_source1_field495 = heap.clone();
                                                {
                                                    let source_ml_source1_field496 = false;
                                                    {
                                                        let source_ml_source1_field497 = self::ml_string::Text::from_units(vec![]);
                                                        (source_ml_source1_field497.clone(), source_ml_source1_field495.clone(), source_ml_source1_field496)
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let next = prototype.clone();
                                                {
                                                    let depth = 0f64;
                                                    {
                                                        let source_ml_source1_s498 = self::source_ml_temporary_set_java_script_ordinary_prototype_loop21(heap.clone(), identity.clone(), next.clone(), depth);
                                                        match source_ml_source1_s498.clone() {
                                                            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done => {
                                                                {
                                                                    let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject({
                                                                        let source_ml_source1_nullish_field_receiver499 = (object.clone(),);
                                                                        if match (source_ml_source1_nullish_field_receiver499.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source1_nullish_field_present500 = ((source_ml_source1_nullish_field_receiver499.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source1_nullish_field_present500.clone() {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value501, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value501.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    }, Box::new(prototype.clone()), {
                                                                        let source_ml_source1_nullish_field_receiver502 = (object.clone(),);
                                                                        if match (source_ml_source1_nullish_field_receiver502.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source1_nullish_field_present503 = ((source_ml_source1_nullish_field_receiver502.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source1_nullish_field_present503.clone() {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value504, _, _) => {
                                                                                        source_ml_source1_data_property_value504
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 120, 116, 101, 110, 115, 105, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    }, {
                                                                        let source_ml_source1_nullish_field_receiver505 = (object.clone(),);
                                                                        if match (source_ml_source1_nullish_field_receiver505.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source1_nullish_field_present506 = ((source_ml_source1_nullish_field_receiver505.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source1_nullish_field_present506.clone() {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value507, _) => {
                                                                                        source_ml_source1_data_property_value507
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    }, {
                                                                        let source_ml_source1_nullish_field_receiver508 = (object.clone(),);
                                                                        if match (source_ml_source1_nullish_field_receiver508.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source1_nullish_field_present509 = ((source_ml_source1_nullish_field_receiver508.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source1_nullish_field_present509.clone() {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value510) => {
                                                                                        source_ml_source1_data_property_value510
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    });
                                                                    {
                                                                        let source_ml_source1_field511 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                        {
                                                                            let source_ml_source1_field512 = true;
                                                                            {
                                                                                let source_ml_source1_field513 = self::ml_string::Text::from_units(vec![]);
                                                                                (source_ml_source1_field513.clone(), source_ml_source1_field511.clone(), source_ml_source1_field512)
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
        let source_ml_source1_field514 = creation;
        {
            let source_ml_source1_field515 = creation;
            {
                let source_ml_source1_field516 = creation;
                {
                    let source_ml_source1_field517 = creation;
                    {
                        let source_ml_source1_field518 = true;
                        {
                            let source_ml_source1_field519 = value.clone();
                            {
                                let source_ml_source1_field520 = creation;
                                {
                                    let source_ml_source1_field521 = creation;
                                    {
                                        let source_ml_source1_field522 = false;
                                        {
                                            let source_ml_source1_field523 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field524 = false;
                                                {
                                                    let source_ml_source1_field525 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field515, source_ml_source1_field514, source_ml_source1_field517, source_ml_source1_field516, source_ml_source1_field523.clone(), source_ml_source1_field522, source_ml_source1_field525.clone(), source_ml_source1_field524, source_ml_source1_field519.clone(), source_ml_source1_field518, source_ml_source1_field521, source_ml_source1_field520)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
