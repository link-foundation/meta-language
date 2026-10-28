// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result;
use super::define_java_script_ordinary_property;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_temporary_set_java_script_ordinary_prototype_loop54;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn set_java_script_ordinary_prototype(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, prototype: self::JavaScriptSemanticValue) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source1_field490 = heap.clone();
                {
                    let source_ml_source1_field491 = false;
                    {
                        let source_ml_source1_field492 = heap_error.clone();
                        (source_ml_source1_field492.clone(), source_ml_source1_field490.clone(), source_ml_source1_field491)
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f493) => {
                    true
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f494) => {
                    true
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f495) => {
                    true
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f496) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f497) => {
                    true
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f498, source_ml_source1_f499) => {
                    true
                }
            } && match prototype.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    true
                }
                self::JavaScriptSemanticValue::Null => {
                    true
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f500) => {
                    true
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f501) => {
                    true
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f502) => {
                    true
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f503) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f504) => {
                    true
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f505, source_ml_source1_f506) => {
                    false
                }
            }) {
                {
                    let source_ml_source1_field507 = heap.clone();
                    {
                        let source_ml_source1_field508 = false;
                        {
                            let source_ml_source1_field509 = self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]);
                            (source_ml_source1_field509.clone(), source_ml_source1_field507.clone(), source_ml_source1_field508)
                        }
                    }
                }
            } else {
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        {
                            let source_ml_source1_field510 = heap.clone();
                            {
                                let source_ml_source1_field511 = false;
                                {
                                    let source_ml_source1_field512 = error.clone();
                                    (source_ml_source1_field512.clone(), source_ml_source1_field510.clone(), source_ml_source1_field511)
                                }
                            }
                        }
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value513, _) => {
                                    source_ml_source1_data_property_value513
                                }
                            }, identity.clone());
                            {
                                let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value514, _) => source_ml_source1_data_property_value514.as_slice() }), self::ml_array::number_index(index));
                                if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    {
                                        let source_ml_source1_field515 = heap.clone();
                                        {
                                            let source_ml_source1_field516 = false;
                                            {
                                                let source_ml_source1_field517 = self::ml_string::Text::from_units(vec![83, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                                (source_ml_source1_field517.clone(), source_ml_source1_field515.clone(), source_ml_source1_field516)
                                            }
                                        }
                                    }
                                } else {
                                    if self::test_java_script_semantic_same_value(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value518, _, _, _) => {
                                            let source_ml_source1_data_property_value518 = *source_ml_source1_data_property_value518;
                                            source_ml_source1_data_property_value518.clone()
                                        }
                                    }, prototype.clone()) {
                                        {
                                            let source_ml_source1_field519 = heap.clone();
                                            {
                                                let source_ml_source1_field520 = true;
                                                {
                                                    let source_ml_source1_field521 = self::ml_string::Text::from_units(vec![]);
                                                    (source_ml_source1_field521.clone(), source_ml_source1_field519.clone(), source_ml_source1_field520)
                                                }
                                            }
                                        }
                                    } else {
                                        if !(match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value522, _, _) => {
                                                source_ml_source1_data_property_value522
                                            }
                                        }) {
                                            {
                                                let source_ml_source1_field523 = heap.clone();
                                                {
                                                    let source_ml_source1_field524 = false;
                                                    {
                                                        let source_ml_source1_field525 = self::ml_string::Text::from_units(vec![]);
                                                        (source_ml_source1_field525.clone(), source_ml_source1_field523.clone(), source_ml_source1_field524)
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let next = prototype.clone();
                                                {
                                                    let depth = 0f64;
                                                    {
                                                        let source_ml_source1_s526 = self::source_ml_temporary_set_java_script_ordinary_prototype_loop54(heap.clone(), identity.clone(), next.clone(), depth);
                                                        match source_ml_source1_s526.clone() {
                                                            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Done => {
                                                                {
                                                                    let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value527, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value527.clone()
                                                                        }
                                                                    }, Box::new(prototype.clone()), match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value528, _, _) => {
                                                                            source_ml_source1_data_property_value528
                                                                        }
                                                                    }, match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value529, _) => {
                                                                            source_ml_source1_data_property_value529
                                                                        }
                                                                    }, match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value530) => {
                                                                            source_ml_source1_data_property_value530
                                                                        }
                                                                    });
                                                                    {
                                                                        let source_ml_source1_field531 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                        {
                                                                            let source_ml_source1_field532 = true;
                                                                            {
                                                                                let source_ml_source1_field533 = self::ml_string::Text::from_units(vec![]);
                                                                                (source_ml_source1_field533.clone(), source_ml_source1_field531.clone(), source_ml_source1_field532)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Return(source_ml_temporary_result54) => {
                                                                source_ml_temporary_result54.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
        let source_ml_source1_field534 = creation;
        {
            let source_ml_source1_field535 = creation;
            {
                let source_ml_source1_field536 = creation;
                {
                    let source_ml_source1_field537 = creation;
                    {
                        let source_ml_source1_field538 = true;
                        {
                            let source_ml_source1_field539 = value.clone();
                            {
                                let source_ml_source1_field540 = creation;
                                {
                                    let source_ml_source1_field541 = creation;
                                    {
                                        let source_ml_source1_field542 = false;
                                        {
                                            let source_ml_source1_field543 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field544 = false;
                                                {
                                                    let source_ml_source1_field545 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field535, source_ml_source1_field534, source_ml_source1_field537, source_ml_source1_field536, source_ml_source1_field543.clone(), source_ml_source1_field542, source_ml_source1_field545.clone(), source_ml_source1_field544, source_ml_source1_field539.clone(), source_ml_source1_field538, source_ml_source1_field541, source_ml_source1_field540)
                                                }
                                            }
                                        }
                                    }
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
pub fn plan_java_script_ordinary_set(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, value: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> (self::ml_string::Text, Vec<self::JavaScriptSemanticValue>, self::JavaScriptSemanticValue, self::ml_string::Text, self::JavaScriptOrdinaryHeap, self::JavaScriptSemanticValue, bool) {
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
                                        if ((self::find_java_script_ordinary_object(match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value547, _) => {
                                                source_ml_source1_data_property_value547
                                            }
                                        }, identity.clone())) < (0f64)) {
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
                                let source_ml_source1_field548 = heap.clone();
                                {
                                    let source_ml_source1_field549 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                    {
                                        let source_ml_source1_field550 = error.clone();
                                        {
                                            let source_ml_source1_field551 = false;
                                            {
                                                let source_ml_source1_field552 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source1_field553 = receiver.clone();
                                                    {
                                                        let source_ml_source1_field554 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source1_field549.clone(), source_ml_source1_field554, source_ml_source1_field552.clone(), source_ml_source1_field550.clone(), source_ml_source1_field548.clone(), source_ml_source1_field553.clone(), source_ml_source1_field551)
                                                    }
                                                }
                                            }
                                        }
                                    }
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
                                                        let source_ml_source1_field562 = heap.clone();
                                                        {
                                                            let source_ml_source1_field563 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field564 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field565 = false;
                                                                    {
                                                                        let source_ml_source1_field566 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field567 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field568 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field563.clone(), source_ml_source1_field568, source_ml_source1_field566.clone(), source_ml_source1_field564.clone(), source_ml_source1_field562.clone(), source_ml_source1_field567.clone(), source_ml_source1_field565)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    {
                                                        let source_ml_source1_field569 = heap.clone();
                                                        {
                                                            let source_ml_source1_field570 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field571 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field572 = false;
                                                                    {
                                                                        let source_ml_source1_field573 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field574 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field575 = vec![value.clone()];
                                                                                (source_ml_source1_field570.clone(), source_ml_source1_field575, source_ml_source1_field573.clone(), source_ml_source1_field571.clone(), source_ml_source1_field569.clone(), source_ml_source1_field574.clone(), source_ml_source1_field572)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f555) => {
                                                    {
                                                        let source_ml_source1_field576 = heap.clone();
                                                        {
                                                            let source_ml_source1_field577 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field578 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field579 = false;
                                                                    {
                                                                        let source_ml_source1_field580 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field581 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field582 = vec![value.clone()];
                                                                                (source_ml_source1_field577.clone(), source_ml_source1_field582, source_ml_source1_field580.clone(), source_ml_source1_field578.clone(), source_ml_source1_field576.clone(), source_ml_source1_field581.clone(), source_ml_source1_field579)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f556) => {
                                                    {
                                                        let source_ml_source1_field583 = heap.clone();
                                                        {
                                                            let source_ml_source1_field584 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field585 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field586 = false;
                                                                    {
                                                                        let source_ml_source1_field587 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field588 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field589 = vec![value.clone()];
                                                                                (source_ml_source1_field584.clone(), source_ml_source1_field589, source_ml_source1_field587.clone(), source_ml_source1_field585.clone(), source_ml_source1_field583.clone(), source_ml_source1_field588.clone(), source_ml_source1_field586)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f557) => {
                                                    {
                                                        let source_ml_source1_field590 = heap.clone();
                                                        {
                                                            let source_ml_source1_field591 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field592 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field593 = false;
                                                                    {
                                                                        let source_ml_source1_field594 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field595 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field596 = vec![value.clone()];
                                                                                (source_ml_source1_field591.clone(), source_ml_source1_field596, source_ml_source1_field594.clone(), source_ml_source1_field592.clone(), source_ml_source1_field590.clone(), source_ml_source1_field595.clone(), source_ml_source1_field593)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f558) => {
                                                    {
                                                        let source_ml_source1_field597 = heap.clone();
                                                        {
                                                            let source_ml_source1_field598 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field599 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field600 = false;
                                                                    {
                                                                        let source_ml_source1_field601 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field602 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field603 = vec![value.clone()];
                                                                                (source_ml_source1_field598.clone(), source_ml_source1_field603, source_ml_source1_field601.clone(), source_ml_source1_field599.clone(), source_ml_source1_field597.clone(), source_ml_source1_field602.clone(), source_ml_source1_field600)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f559) => {
                                                    {
                                                        let source_ml_source1_field604 = heap.clone();
                                                        {
                                                            let source_ml_source1_field605 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field606 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field607 = false;
                                                                    {
                                                                        let source_ml_source1_field608 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field609 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field610 = vec![value.clone()];
                                                                                (source_ml_source1_field605.clone(), source_ml_source1_field610, source_ml_source1_field608.clone(), source_ml_source1_field606.clone(), source_ml_source1_field604.clone(), source_ml_source1_field609.clone(), source_ml_source1_field607)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f560, source_ml_source1_f561) => {
                                                    {
                                                        let source_ml_source1_field611 = heap.clone();
                                                        {
                                                            let source_ml_source1_field612 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field613 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field614 = false;
                                                                    {
                                                                        let source_ml_source1_field615 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field616 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field617 = vec![value.clone()];
                                                                                (source_ml_source1_field612.clone(), source_ml_source1_field617, source_ml_source1_field615.clone(), source_ml_source1_field613.clone(), source_ml_source1_field611.clone(), source_ml_source1_field616.clone(), source_ml_source1_field614)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if !((own.clone()).10) {
                                            {
                                                let source_ml_source1_field618 = heap.clone();
                                                {
                                                    let source_ml_source1_field619 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                    {
                                                        let source_ml_source1_field620 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field621 = false;
                                                            {
                                                                let source_ml_source1_field622 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_field623 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field624 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field619.clone(), source_ml_source1_field624, source_ml_source1_field622.clone(), source_ml_source1_field620.clone(), source_ml_source1_field618.clone(), source_ml_source1_field623.clone(), source_ml_source1_field621)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            match receiver.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    {
                                                        let source_ml_source1_field631 = heap.clone();
                                                        {
                                                            let source_ml_source1_field632 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field633 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field634 = false;
                                                                    {
                                                                        let source_ml_source1_field635 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field636 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field637 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field632.clone(), source_ml_source1_field637, source_ml_source1_field635.clone(), source_ml_source1_field633.clone(), source_ml_source1_field631.clone(), source_ml_source1_field636.clone(), source_ml_source1_field634)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    {
                                                        let source_ml_source1_field638 = heap.clone();
                                                        {
                                                            let source_ml_source1_field639 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field640 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field641 = false;
                                                                    {
                                                                        let source_ml_source1_field642 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field643 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field644 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field639.clone(), source_ml_source1_field644, source_ml_source1_field642.clone(), source_ml_source1_field640.clone(), source_ml_source1_field638.clone(), source_ml_source1_field643.clone(), source_ml_source1_field641)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f625) => {
                                                    {
                                                        let source_ml_source1_field645 = heap.clone();
                                                        {
                                                            let source_ml_source1_field646 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field647 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field648 = false;
                                                                    {
                                                                        let source_ml_source1_field649 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field650 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field651 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field646.clone(), source_ml_source1_field651, source_ml_source1_field649.clone(), source_ml_source1_field647.clone(), source_ml_source1_field645.clone(), source_ml_source1_field650.clone(), source_ml_source1_field648)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f626) => {
                                                    {
                                                        let source_ml_source1_field652 = heap.clone();
                                                        {
                                                            let source_ml_source1_field653 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field654 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field655 = false;
                                                                    {
                                                                        let source_ml_source1_field656 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field657 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field658 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field653.clone(), source_ml_source1_field658, source_ml_source1_field656.clone(), source_ml_source1_field654.clone(), source_ml_source1_field652.clone(), source_ml_source1_field657.clone(), source_ml_source1_field655)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f627) => {
                                                    {
                                                        let source_ml_source1_field659 = heap.clone();
                                                        {
                                                            let source_ml_source1_field660 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field661 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field662 = false;
                                                                    {
                                                                        let source_ml_source1_field663 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field664 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field665 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field660.clone(), source_ml_source1_field665, source_ml_source1_field663.clone(), source_ml_source1_field661.clone(), source_ml_source1_field659.clone(), source_ml_source1_field664.clone(), source_ml_source1_field662)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f628) => {
                                                    {
                                                        let source_ml_source1_field666 = heap.clone();
                                                        {
                                                            let source_ml_source1_field667 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field668 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field669 = false;
                                                                    {
                                                                        let source_ml_source1_field670 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field671 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field672 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field667.clone(), source_ml_source1_field672, source_ml_source1_field670.clone(), source_ml_source1_field668.clone(), source_ml_source1_field666.clone(), source_ml_source1_field671.clone(), source_ml_source1_field669)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f629) => {
                                                    {
                                                        let source_ml_source1_field673 = heap.clone();
                                                        {
                                                            let source_ml_source1_field674 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field675 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field676 = false;
                                                                    {
                                                                        let source_ml_source1_field677 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field678 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field679 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field674.clone(), source_ml_source1_field679, source_ml_source1_field677.clone(), source_ml_source1_field675.clone(), source_ml_source1_field673.clone(), source_ml_source1_field678.clone(), source_ml_source1_field676)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f630) => {
                                                    {
                                                        let index = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value680, _) => {
                                                                source_ml_source1_data_property_value680
                                                            }
                                                        }, identity_2.clone());
                                                        {
                                                            let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value681, _) => source_ml_source1_data_property_value681.as_slice() }), self::ml_array::number_index(index));
                                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                {
                                                                    let source_ml_source1_field682 = heap.clone();
                                                                    {
                                                                        let source_ml_source1_field683 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                                                        {
                                                                            let source_ml_source1_field684 = self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]);
                                                                            {
                                                                                let source_ml_source1_field685 = false;
                                                                                {
                                                                                    let source_ml_source1_field686 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source1_field687 = receiver.clone();
                                                                                        {
                                                                                            let source_ml_source1_field688 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                            (source_ml_source1_field683.clone(), source_ml_source1_field688, source_ml_source1_field686.clone(), source_ml_source1_field684.clone(), source_ml_source1_field682.clone(), source_ml_source1_field687.clone(), source_ml_source1_field685)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value689) => {
                                                                            source_ml_source1_data_property_value689
                                                                        }
                                                                    }, key.clone());
                                                                    {
                                                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value690) => {
                                                                                source_ml_source1_data_property_value690
                                                                            }
                                                                        }, self::ml_array::number_index(at));
                                                                        if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({
                                                                            let source_ml_source1_nullish_field_receiver691 = (property.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver691.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present692 = ((source_ml_source1_nullish_field_receiver691.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present692.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value693) => {
                                                                                            source_ml_source1_data_property_value693.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).9) || !(({
                                                                            let source_ml_source1_nullish_field_receiver694 = (property.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver694.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present695 = ((source_ml_source1_nullish_field_receiver694.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present695.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value696) => {
                                                                                            source_ml_source1_data_property_value696.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).10))) {
                                                                            {
                                                                                let source_ml_source1_field697 = heap.clone();
                                                                                {
                                                                                    let source_ml_source1_field698 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                    {
                                                                                        let source_ml_source1_field699 = self::ml_string::Text::from_units(vec![]);
                                                                                        {
                                                                                            let source_ml_source1_field700 = false;
                                                                                            {
                                                                                                let source_ml_source1_field701 = self::JavaScriptSemanticValue::Undefined;
                                                                                                {
                                                                                                    let source_ml_source1_field702 = receiver.clone();
                                                                                                    {
                                                                                                        let source_ml_source1_field703 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                        (source_ml_source1_field698.clone(), source_ml_source1_field703, source_ml_source1_field701.clone(), source_ml_source1_field699.clone(), source_ml_source1_field697.clone(), source_ml_source1_field702.clone(), source_ml_source1_field700)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            {
                                                                                let proposed = self::create_java_script_ordinary_set_descriptor(value.clone(), match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false });
                                                                                {
                                                                                    let transition = self::define_java_script_ordinary_property(heap.clone(), identity_2.clone(), key.clone(), proposed.clone());
                                                                                    {
                                                                                        let source_ml_source1_field704 = (transition.clone()).1;
                                                                                        {
                                                                                            let source_ml_source1_field705 = if (((transition.clone()).0) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_field706 = (transition.clone()).0;
                                                                                                {
                                                                                                    let source_ml_source1_field707 = (transition.clone()).2;
                                                                                                    {
                                                                                                        let source_ml_source1_field708 = self::JavaScriptSemanticValue::Undefined;
                                                                                                        {
                                                                                                            let source_ml_source1_field709 = receiver.clone();
                                                                                                            {
                                                                                                                let source_ml_source1_field710 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                (source_ml_source1_field705.clone(), source_ml_source1_field710, source_ml_source1_field708.clone(), source_ml_source1_field706.clone(), source_ml_source1_field704.clone(), source_ml_source1_field709.clone(), source_ml_source1_field707)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
