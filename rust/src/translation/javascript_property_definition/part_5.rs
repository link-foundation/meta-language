// Generated from complete js/src/translation/javascript-property-definition.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result;
use super::create_java_script_ordinary_set_descriptor;
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
use super::source_ml_temporary_insert_java_script_ordinary_object_loop26;
use super::source_ml_temporary_read_java_script_ordinary_own_keys_loop23;
use super::test_java_script_ordinary_symbol;

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
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value467, _) => {
                                                source_ml_source1_data_property_value467
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
                                let source_ml_source1_field468 = heap.clone();
                                {
                                    let source_ml_source1_field469 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                    {
                                        let source_ml_source1_field470 = error.clone();
                                        {
                                            let source_ml_source1_field471 = false;
                                            {
                                                let source_ml_source1_field472 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source1_field473 = receiver.clone();
                                                    {
                                                        let source_ml_source1_field474 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source1_field469.clone(), source_ml_source1_field474, source_ml_source1_field472.clone(), source_ml_source1_field470.clone(), source_ml_source1_field468.clone(), source_ml_source1_field473.clone(), source_ml_source1_field471)
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
                                                        let source_ml_source1_field482 = heap.clone();
                                                        {
                                                            let source_ml_source1_field483 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field484 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field485 = false;
                                                                    {
                                                                        let source_ml_source1_field486 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field487 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field488 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field483.clone(), source_ml_source1_field488, source_ml_source1_field486.clone(), source_ml_source1_field484.clone(), source_ml_source1_field482.clone(), source_ml_source1_field487.clone(), source_ml_source1_field485)
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
                                                        let source_ml_source1_field489 = heap.clone();
                                                        {
                                                            let source_ml_source1_field490 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field491 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field492 = false;
                                                                    {
                                                                        let source_ml_source1_field493 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field494 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field495 = vec![value.clone()];
                                                                                (source_ml_source1_field490.clone(), source_ml_source1_field495, source_ml_source1_field493.clone(), source_ml_source1_field491.clone(), source_ml_source1_field489.clone(), source_ml_source1_field494.clone(), source_ml_source1_field492)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f475) => {
                                                    {
                                                        let source_ml_source1_field496 = heap.clone();
                                                        {
                                                            let source_ml_source1_field497 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field498 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field499 = false;
                                                                    {
                                                                        let source_ml_source1_field500 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field501 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field502 = vec![value.clone()];
                                                                                (source_ml_source1_field497.clone(), source_ml_source1_field502, source_ml_source1_field500.clone(), source_ml_source1_field498.clone(), source_ml_source1_field496.clone(), source_ml_source1_field501.clone(), source_ml_source1_field499)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f476) => {
                                                    {
                                                        let source_ml_source1_field503 = heap.clone();
                                                        {
                                                            let source_ml_source1_field504 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field505 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field506 = false;
                                                                    {
                                                                        let source_ml_source1_field507 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field508 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field509 = vec![value.clone()];
                                                                                (source_ml_source1_field504.clone(), source_ml_source1_field509, source_ml_source1_field507.clone(), source_ml_source1_field505.clone(), source_ml_source1_field503.clone(), source_ml_source1_field508.clone(), source_ml_source1_field506)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f477) => {
                                                    {
                                                        let source_ml_source1_field510 = heap.clone();
                                                        {
                                                            let source_ml_source1_field511 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field512 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field513 = false;
                                                                    {
                                                                        let source_ml_source1_field514 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field515 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field516 = vec![value.clone()];
                                                                                (source_ml_source1_field511.clone(), source_ml_source1_field516, source_ml_source1_field514.clone(), source_ml_source1_field512.clone(), source_ml_source1_field510.clone(), source_ml_source1_field515.clone(), source_ml_source1_field513)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f478) => {
                                                    {
                                                        let source_ml_source1_field517 = heap.clone();
                                                        {
                                                            let source_ml_source1_field518 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field519 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field520 = false;
                                                                    {
                                                                        let source_ml_source1_field521 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field522 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field523 = vec![value.clone()];
                                                                                (source_ml_source1_field518.clone(), source_ml_source1_field523, source_ml_source1_field521.clone(), source_ml_source1_field519.clone(), source_ml_source1_field517.clone(), source_ml_source1_field522.clone(), source_ml_source1_field520)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f479) => {
                                                    {
                                                        let source_ml_source1_field524 = heap.clone();
                                                        {
                                                            let source_ml_source1_field525 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field526 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field527 = false;
                                                                    {
                                                                        let source_ml_source1_field528 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field529 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field530 = vec![value.clone()];
                                                                                (source_ml_source1_field525.clone(), source_ml_source1_field530, source_ml_source1_field528.clone(), source_ml_source1_field526.clone(), source_ml_source1_field524.clone(), source_ml_source1_field529.clone(), source_ml_source1_field527)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f480, source_ml_source1_f481) => {
                                                    {
                                                        let source_ml_source1_field531 = heap.clone();
                                                        {
                                                            let source_ml_source1_field532 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field533 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field534 = false;
                                                                    {
                                                                        let source_ml_source1_field535 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field536 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field537 = vec![value.clone()];
                                                                                (source_ml_source1_field532.clone(), source_ml_source1_field537, source_ml_source1_field535.clone(), source_ml_source1_field533.clone(), source_ml_source1_field531.clone(), source_ml_source1_field536.clone(), source_ml_source1_field534)
                                                                            }
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
                                                let source_ml_source1_field538 = heap.clone();
                                                {
                                                    let source_ml_source1_field539 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                    {
                                                        let source_ml_source1_field540 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field541 = false;
                                                            {
                                                                let source_ml_source1_field542 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_field543 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field544 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field539.clone(), source_ml_source1_field544, source_ml_source1_field542.clone(), source_ml_source1_field540.clone(), source_ml_source1_field538.clone(), source_ml_source1_field543.clone(), source_ml_source1_field541)
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
                                                        let source_ml_source1_field551 = heap.clone();
                                                        {
                                                            let source_ml_source1_field552 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field553 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field554 = false;
                                                                    {
                                                                        let source_ml_source1_field555 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field556 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field557 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field552.clone(), source_ml_source1_field557, source_ml_source1_field555.clone(), source_ml_source1_field553.clone(), source_ml_source1_field551.clone(), source_ml_source1_field556.clone(), source_ml_source1_field554)
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
                                                        let source_ml_source1_field558 = heap.clone();
                                                        {
                                                            let source_ml_source1_field559 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field560 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field561 = false;
                                                                    {
                                                                        let source_ml_source1_field562 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field563 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field564 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field559.clone(), source_ml_source1_field564, source_ml_source1_field562.clone(), source_ml_source1_field560.clone(), source_ml_source1_field558.clone(), source_ml_source1_field563.clone(), source_ml_source1_field561)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f545) => {
                                                    {
                                                        let source_ml_source1_field565 = heap.clone();
                                                        {
                                                            let source_ml_source1_field566 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field567 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field568 = false;
                                                                    {
                                                                        let source_ml_source1_field569 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field570 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field571 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field566.clone(), source_ml_source1_field571, source_ml_source1_field569.clone(), source_ml_source1_field567.clone(), source_ml_source1_field565.clone(), source_ml_source1_field570.clone(), source_ml_source1_field568)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f546) => {
                                                    {
                                                        let source_ml_source1_field572 = heap.clone();
                                                        {
                                                            let source_ml_source1_field573 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field574 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field575 = false;
                                                                    {
                                                                        let source_ml_source1_field576 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field577 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field578 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field573.clone(), source_ml_source1_field578, source_ml_source1_field576.clone(), source_ml_source1_field574.clone(), source_ml_source1_field572.clone(), source_ml_source1_field577.clone(), source_ml_source1_field575)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f547) => {
                                                    {
                                                        let source_ml_source1_field579 = heap.clone();
                                                        {
                                                            let source_ml_source1_field580 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field581 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field582 = false;
                                                                    {
                                                                        let source_ml_source1_field583 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field584 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field585 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field580.clone(), source_ml_source1_field585, source_ml_source1_field583.clone(), source_ml_source1_field581.clone(), source_ml_source1_field579.clone(), source_ml_source1_field584.clone(), source_ml_source1_field582)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f548) => {
                                                    {
                                                        let source_ml_source1_field586 = heap.clone();
                                                        {
                                                            let source_ml_source1_field587 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field588 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field589 = false;
                                                                    {
                                                                        let source_ml_source1_field590 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field591 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field592 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field587.clone(), source_ml_source1_field592, source_ml_source1_field590.clone(), source_ml_source1_field588.clone(), source_ml_source1_field586.clone(), source_ml_source1_field591.clone(), source_ml_source1_field589)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f549) => {
                                                    {
                                                        let source_ml_source1_field593 = heap.clone();
                                                        {
                                                            let source_ml_source1_field594 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field595 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field596 = false;
                                                                    {
                                                                        let source_ml_source1_field597 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field598 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field599 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field594.clone(), source_ml_source1_field599, source_ml_source1_field597.clone(), source_ml_source1_field595.clone(), source_ml_source1_field593.clone(), source_ml_source1_field598.clone(), source_ml_source1_field596)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f550) => {
                                                    {
                                                        let index = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value600, _) => {
                                                                source_ml_source1_data_property_value600
                                                            }
                                                        }, identity_2.clone());
                                                        {
                                                            let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value601, _) => source_ml_source1_data_property_value601.as_slice() }), self::ml_array::number_index(index));
                                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                {
                                                                    let source_ml_source1_field602 = heap.clone();
                                                                    {
                                                                        let source_ml_source1_field603 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                                                        {
                                                                            let source_ml_source1_field604 = self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]);
                                                                            {
                                                                                let source_ml_source1_field605 = false;
                                                                                {
                                                                                    let source_ml_source1_field606 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source1_field607 = receiver.clone();
                                                                                        {
                                                                                            let source_ml_source1_field608 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                            (source_ml_source1_field603.clone(), source_ml_source1_field608, source_ml_source1_field606.clone(), source_ml_source1_field604.clone(), source_ml_source1_field602.clone(), source_ml_source1_field607.clone(), source_ml_source1_field605)
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
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value609) => {
                                                                            source_ml_source1_data_property_value609
                                                                        }
                                                                    }, key.clone());
                                                                    {
                                                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value610) => {
                                                                                source_ml_source1_data_property_value610
                                                                            }
                                                                        }, self::ml_array::number_index(at));
                                                                        if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({
                                                                            let source_ml_source1_nullish_field_receiver611 = (property.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver611.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present612 = ((source_ml_source1_nullish_field_receiver611.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present612.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value613) => {
                                                                                            source_ml_source1_data_property_value613.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).9) || !(({
                                                                            let source_ml_source1_nullish_field_receiver614 = (property.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver614.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present615 = ((source_ml_source1_nullish_field_receiver614.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present615.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value616) => {
                                                                                            source_ml_source1_data_property_value616.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).10))) {
                                                                            {
                                                                                let source_ml_source1_field617 = heap.clone();
                                                                                {
                                                                                    let source_ml_source1_field618 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                    {
                                                                                        let source_ml_source1_field619 = self::ml_string::Text::from_units(vec![]);
                                                                                        {
                                                                                            let source_ml_source1_field620 = false;
                                                                                            {
                                                                                                let source_ml_source1_field621 = self::JavaScriptSemanticValue::Undefined;
                                                                                                {
                                                                                                    let source_ml_source1_field622 = receiver.clone();
                                                                                                    {
                                                                                                        let source_ml_source1_field623 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                        (source_ml_source1_field618.clone(), source_ml_source1_field623, source_ml_source1_field621.clone(), source_ml_source1_field619.clone(), source_ml_source1_field617.clone(), source_ml_source1_field622.clone(), source_ml_source1_field620)
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
                                                                                        let source_ml_source1_field624 = (transition.clone()).1;
                                                                                        {
                                                                                            let source_ml_source1_field625 = if (((transition.clone()).0) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_field626 = (transition.clone()).0;
                                                                                                {
                                                                                                    let source_ml_source1_field627 = (transition.clone()).2;
                                                                                                    {
                                                                                                        let source_ml_source1_field628 = self::JavaScriptSemanticValue::Undefined;
                                                                                                        {
                                                                                                            let source_ml_source1_field629 = receiver.clone();
                                                                                                            {
                                                                                                                let source_ml_source1_field630 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                (source_ml_source1_field625.clone(), source_ml_source1_field630, source_ml_source1_field628.clone(), source_ml_source1_field626.clone(), source_ml_source1_field624.clone(), source_ml_source1_field629.clone(), source_ml_source1_field627)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
pub fn read_java_script_ordinary_own_keys(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> (self::ml_string::Text, Vec<self::JavaScriptSemanticValue>) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source1_field631 = Vec::<self::JavaScriptSemanticValue>::new();
                {
                    let source_ml_source1_field632 = error.clone();
                    (source_ml_source1_field632.clone(), source_ml_source1_field631)
                }
            }
        } else {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value633, _) => {
                        source_ml_source1_data_property_value633
                    }
                }, identity.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value634, _) => source_ml_source1_data_property_value634.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        {
                            let source_ml_source1_field635 = Vec::<self::JavaScriptSemanticValue>::new();
                            {
                                let source_ml_source1_field636 = self::ml_string::Text::from_units(vec![79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 75, 101, 121, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source1_field636.clone(), source_ml_source1_field635)
                            }
                        }
                    } else {
                        {
                            let numeric = Vec::<self::JavaScriptSemanticValue>::new();
                            {
                                let positions = Vec::<f64>::new();
                                {
                                    let text = Vec::<self::JavaScriptSemanticValue>::new();
                                    {
                                        let symbols = Vec::<self::JavaScriptSemanticValue>::new();
                                        {
                                            let source_ml_temporary_values22 = match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value637) => {
                                                    source_ml_source1_data_property_value637
                                                }
                                            };
                                            {
                                                let source_ml_temporary_index22 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s638 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop23(numeric, positions, text, symbols, source_ml_temporary_values22, source_ml_temporary_index22.clone());
                                                    match source_ml_source1_s638.clone() {
                                                        self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Done(numeric_2, text_2, symbols_2) => {
                                                            {
                                                                let source_ml_source1_field642 = {
                                                                    let source_ml_source1_concat_value639 = numeric_2;
                                                                    {
                                                                        let source_ml_source1_concat_value640 = text_2;
                                                                        {
                                                                            let source_ml_source1_concat_value641 = symbols_2;
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value639), source_ml_source1_concat_value640), source_ml_source1_concat_value641)
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_field643 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source1_field643.clone(), source_ml_source1_field642)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
pub fn insert_java_script_ordinary_object(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, prototype: self::JavaScriptSemanticValue) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source1_field644 = heap.clone();
                {
                    let source_ml_source1_field645 = false;
                    {
                        let source_ml_source1_field646 = heap_error.clone();
                        (source_ml_source1_field646.clone(), source_ml_source1_field644.clone(), source_ml_source1_field645)
                    }
                }
            }
        } else {
            if ((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value648, _) => source_ml_source1_data_property_value648.as_slice() }).len() as f64)) >= (4096f64)) {
                {
                    let source_ml_source1_field649 = heap.clone();
                    {
                        let source_ml_source1_field650 = false;
                        {
                            let source_ml_source1_field651 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]);
                            (source_ml_source1_field651.clone(), source_ml_source1_field649.clone(), source_ml_source1_field650)
                        }
                    }
                }
            } else {
                if ((((identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value653, _) => {
                        source_ml_source1_data_property_value653
                    }
                }, identity.clone())) >= (0f64))) || self::test_java_script_ordinary_symbol(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value654) => {
                        source_ml_source1_data_property_value654
                    }
                }, identity.clone())) {
                    {
                        let source_ml_source1_field655 = heap.clone();
                        {
                            let source_ml_source1_field656 = false;
                            {
                                let source_ml_source1_field657 = self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source1_field657.clone(), source_ml_source1_field655.clone(), source_ml_source1_field656)
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f658) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f659) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f660) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f661) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f662) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f663, source_ml_source1_f664) => {
                            true
                        }
                    } && match prototype.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            true
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f665) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f666) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f667) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f668) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f669) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f670, source_ml_source1_f671) => {
                            false
                        }
                    }) {
                        {
                            let source_ml_source1_field672 = heap.clone();
                            {
                                let source_ml_source1_field673 = false;
                                {
                                    let source_ml_source1_field674 = self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]);
                                    (source_ml_source1_field674.clone(), source_ml_source1_field672.clone(), source_ml_source1_field673)
                                }
                            }
                        }
                    } else {
                        {
                            let error = self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                {
                                    let source_ml_source1_field675 = heap.clone();
                                    {
                                        let source_ml_source1_field676 = false;
                                        {
                                            let source_ml_source1_field677 = error.clone();
                                            (source_ml_source1_field677.clone(), source_ml_source1_field675.clone(), source_ml_source1_field676)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_temporary_values25 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value678, _) => {
                                                source_ml_source1_data_property_value678
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index25 = self::ml::Big::from_i128(0);
                                            {
                                                let at_2 = self::source_ml_temporary_insert_java_script_ordinary_object_loop26(identity.clone(), at, source_ml_temporary_values25, source_ml_temporary_index25.clone());
                                                {
                                                    let object = self::JavaScriptOrdinaryObject::OrdinaryObject(identity.clone(), Box::new(prototype.clone()), true, false, Vec::<self::JavaScriptOrdinaryProperty>::new());
                                                    {
                                                        let objects = {
                                                            let source_ml_source1_concat_value683 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value680, _) => source_ml_source1_data_property_value680.as_slice() }), 0f64, at_2);
                                                            {
                                                                let source_ml_source1_concat_value684 = vec![object.clone()];
                                                                {
                                                                    let source_ml_source1_concat_value685 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value682, _) => source_ml_source1_data_property_value682.as_slice() }), at_2, f64::INFINITY);
                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), source_ml_source1_concat_value683), source_ml_source1_concat_value684), source_ml_source1_concat_value685)
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_field687 = self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value686) => {
                                                                    source_ml_source1_data_property_value686
                                                                }
                                                            });
                                                            {
                                                                let source_ml_source1_field688 = true;
                                                                {
                                                                    let source_ml_source1_field689 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source1_field689.clone(), source_ml_source1_field687.clone(), source_ml_source1_field688)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
