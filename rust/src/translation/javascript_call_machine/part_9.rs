// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop24Result;
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
use super::source_ml_temporary_insert_java_script_ordinary_object_loop27;
use super::source_ml_temporary_read_java_script_ordinary_own_keys_loop24;
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
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value492, _) => {
                                                source_ml_source1_data_property_value492
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
                                let source_ml_source1_field493 = heap.clone();
                                {
                                    let source_ml_source1_field494 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                    {
                                        let source_ml_source1_field495 = error.clone();
                                        {
                                            let source_ml_source1_field496 = false;
                                            {
                                                let source_ml_source1_field497 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source1_field498 = receiver.clone();
                                                    {
                                                        let source_ml_source1_field499 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source1_field494.clone(), source_ml_source1_field499, source_ml_source1_field497.clone(), source_ml_source1_field495.clone(), source_ml_source1_field493.clone(), source_ml_source1_field498.clone(), source_ml_source1_field496)
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
                                                        let source_ml_source1_field507 = heap.clone();
                                                        {
                                                            let source_ml_source1_field508 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field509 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field510 = false;
                                                                    {
                                                                        let source_ml_source1_field511 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field512 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field513 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field508.clone(), source_ml_source1_field513, source_ml_source1_field511.clone(), source_ml_source1_field509.clone(), source_ml_source1_field507.clone(), source_ml_source1_field512.clone(), source_ml_source1_field510)
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
                                                        let source_ml_source1_field514 = heap.clone();
                                                        {
                                                            let source_ml_source1_field515 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field516 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field517 = false;
                                                                    {
                                                                        let source_ml_source1_field518 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field519 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field520 = vec![value.clone()];
                                                                                (source_ml_source1_field515.clone(), source_ml_source1_field520, source_ml_source1_field518.clone(), source_ml_source1_field516.clone(), source_ml_source1_field514.clone(), source_ml_source1_field519.clone(), source_ml_source1_field517)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f500) => {
                                                    {
                                                        let source_ml_source1_field521 = heap.clone();
                                                        {
                                                            let source_ml_source1_field522 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field523 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field524 = false;
                                                                    {
                                                                        let source_ml_source1_field525 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field526 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field527 = vec![value.clone()];
                                                                                (source_ml_source1_field522.clone(), source_ml_source1_field527, source_ml_source1_field525.clone(), source_ml_source1_field523.clone(), source_ml_source1_field521.clone(), source_ml_source1_field526.clone(), source_ml_source1_field524)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f501) => {
                                                    {
                                                        let source_ml_source1_field528 = heap.clone();
                                                        {
                                                            let source_ml_source1_field529 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field530 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field531 = false;
                                                                    {
                                                                        let source_ml_source1_field532 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field533 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field534 = vec![value.clone()];
                                                                                (source_ml_source1_field529.clone(), source_ml_source1_field534, source_ml_source1_field532.clone(), source_ml_source1_field530.clone(), source_ml_source1_field528.clone(), source_ml_source1_field533.clone(), source_ml_source1_field531)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f502) => {
                                                    {
                                                        let source_ml_source1_field535 = heap.clone();
                                                        {
                                                            let source_ml_source1_field536 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field537 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field538 = false;
                                                                    {
                                                                        let source_ml_source1_field539 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field540 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field541 = vec![value.clone()];
                                                                                (source_ml_source1_field536.clone(), source_ml_source1_field541, source_ml_source1_field539.clone(), source_ml_source1_field537.clone(), source_ml_source1_field535.clone(), source_ml_source1_field540.clone(), source_ml_source1_field538)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f503) => {
                                                    {
                                                        let source_ml_source1_field542 = heap.clone();
                                                        {
                                                            let source_ml_source1_field543 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field544 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field545 = false;
                                                                    {
                                                                        let source_ml_source1_field546 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field547 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field548 = vec![value.clone()];
                                                                                (source_ml_source1_field543.clone(), source_ml_source1_field548, source_ml_source1_field546.clone(), source_ml_source1_field544.clone(), source_ml_source1_field542.clone(), source_ml_source1_field547.clone(), source_ml_source1_field545)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f504) => {
                                                    {
                                                        let source_ml_source1_field549 = heap.clone();
                                                        {
                                                            let source_ml_source1_field550 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field551 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field552 = false;
                                                                    {
                                                                        let source_ml_source1_field553 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field554 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field555 = vec![value.clone()];
                                                                                (source_ml_source1_field550.clone(), source_ml_source1_field555, source_ml_source1_field553.clone(), source_ml_source1_field551.clone(), source_ml_source1_field549.clone(), source_ml_source1_field554.clone(), source_ml_source1_field552)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f505, source_ml_source1_f506) => {
                                                    {
                                                        let source_ml_source1_field556 = heap.clone();
                                                        {
                                                            let source_ml_source1_field557 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field558 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field559 = false;
                                                                    {
                                                                        let source_ml_source1_field560 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field561 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field562 = vec![value.clone()];
                                                                                (source_ml_source1_field557.clone(), source_ml_source1_field562, source_ml_source1_field560.clone(), source_ml_source1_field558.clone(), source_ml_source1_field556.clone(), source_ml_source1_field561.clone(), source_ml_source1_field559)
                                                                            }
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
                                                let source_ml_source1_field563 = heap.clone();
                                                {
                                                    let source_ml_source1_field564 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                    {
                                                        let source_ml_source1_field565 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field566 = false;
                                                            {
                                                                let source_ml_source1_field567 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_field568 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field569 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field564.clone(), source_ml_source1_field569, source_ml_source1_field567.clone(), source_ml_source1_field565.clone(), source_ml_source1_field563.clone(), source_ml_source1_field568.clone(), source_ml_source1_field566)
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
                                                        let source_ml_source1_field576 = heap.clone();
                                                        {
                                                            let source_ml_source1_field577 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field578 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field579 = false;
                                                                    {
                                                                        let source_ml_source1_field580 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field581 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field582 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field577.clone(), source_ml_source1_field582, source_ml_source1_field580.clone(), source_ml_source1_field578.clone(), source_ml_source1_field576.clone(), source_ml_source1_field581.clone(), source_ml_source1_field579)
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
                                                        let source_ml_source1_field583 = heap.clone();
                                                        {
                                                            let source_ml_source1_field584 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field585 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field586 = false;
                                                                    {
                                                                        let source_ml_source1_field587 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field588 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field589 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field584.clone(), source_ml_source1_field589, source_ml_source1_field587.clone(), source_ml_source1_field585.clone(), source_ml_source1_field583.clone(), source_ml_source1_field588.clone(), source_ml_source1_field586)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f570) => {
                                                    {
                                                        let source_ml_source1_field590 = heap.clone();
                                                        {
                                                            let source_ml_source1_field591 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field592 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field593 = false;
                                                                    {
                                                                        let source_ml_source1_field594 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field595 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field596 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field591.clone(), source_ml_source1_field596, source_ml_source1_field594.clone(), source_ml_source1_field592.clone(), source_ml_source1_field590.clone(), source_ml_source1_field595.clone(), source_ml_source1_field593)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f571) => {
                                                    {
                                                        let source_ml_source1_field597 = heap.clone();
                                                        {
                                                            let source_ml_source1_field598 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field599 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field600 = false;
                                                                    {
                                                                        let source_ml_source1_field601 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field602 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field603 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field598.clone(), source_ml_source1_field603, source_ml_source1_field601.clone(), source_ml_source1_field599.clone(), source_ml_source1_field597.clone(), source_ml_source1_field602.clone(), source_ml_source1_field600)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f572) => {
                                                    {
                                                        let source_ml_source1_field604 = heap.clone();
                                                        {
                                                            let source_ml_source1_field605 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field606 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field607 = false;
                                                                    {
                                                                        let source_ml_source1_field608 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field609 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field610 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field605.clone(), source_ml_source1_field610, source_ml_source1_field608.clone(), source_ml_source1_field606.clone(), source_ml_source1_field604.clone(), source_ml_source1_field609.clone(), source_ml_source1_field607)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f573) => {
                                                    {
                                                        let source_ml_source1_field611 = heap.clone();
                                                        {
                                                            let source_ml_source1_field612 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field613 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field614 = false;
                                                                    {
                                                                        let source_ml_source1_field615 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field616 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field617 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field612.clone(), source_ml_source1_field617, source_ml_source1_field615.clone(), source_ml_source1_field613.clone(), source_ml_source1_field611.clone(), source_ml_source1_field616.clone(), source_ml_source1_field614)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f574) => {
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
                                                }
                                                self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f575) => {
                                                    {
                                                        let index = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value625, _) => {
                                                                source_ml_source1_data_property_value625
                                                            }
                                                        }, identity_2.clone());
                                                        {
                                                            let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value626, _) => source_ml_source1_data_property_value626.as_slice() }), self::ml_array::number_index(index));
                                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                {
                                                                    let source_ml_source1_field627 = heap.clone();
                                                                    {
                                                                        let source_ml_source1_field628 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                                                        {
                                                                            let source_ml_source1_field629 = self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]);
                                                                            {
                                                                                let source_ml_source1_field630 = false;
                                                                                {
                                                                                    let source_ml_source1_field631 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source1_field632 = receiver.clone();
                                                                                        {
                                                                                            let source_ml_source1_field633 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                            (source_ml_source1_field628.clone(), source_ml_source1_field633, source_ml_source1_field631.clone(), source_ml_source1_field629.clone(), source_ml_source1_field627.clone(), source_ml_source1_field632.clone(), source_ml_source1_field630)
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
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value634) => {
                                                                            source_ml_source1_data_property_value634
                                                                        }
                                                                    }, key.clone());
                                                                    {
                                                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value635) => {
                                                                                source_ml_source1_data_property_value635
                                                                            }
                                                                        }, self::ml_array::number_index(at));
                                                                        if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({
                                                                            let source_ml_source1_nullish_field_receiver636 = (property.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver636.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present637 = ((source_ml_source1_nullish_field_receiver636.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present637.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value638) => {
                                                                                            source_ml_source1_data_property_value638.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).9) || !(({
                                                                            let source_ml_source1_nullish_field_receiver639 = (property.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver639.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present640 = ((source_ml_source1_nullish_field_receiver639.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present640.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value641) => {
                                                                                            source_ml_source1_data_property_value641.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).10))) {
                                                                            {
                                                                                let source_ml_source1_field642 = heap.clone();
                                                                                {
                                                                                    let source_ml_source1_field643 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                    {
                                                                                        let source_ml_source1_field644 = self::ml_string::Text::from_units(vec![]);
                                                                                        {
                                                                                            let source_ml_source1_field645 = false;
                                                                                            {
                                                                                                let source_ml_source1_field646 = self::JavaScriptSemanticValue::Undefined;
                                                                                                {
                                                                                                    let source_ml_source1_field647 = receiver.clone();
                                                                                                    {
                                                                                                        let source_ml_source1_field648 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                        (source_ml_source1_field643.clone(), source_ml_source1_field648, source_ml_source1_field646.clone(), source_ml_source1_field644.clone(), source_ml_source1_field642.clone(), source_ml_source1_field647.clone(), source_ml_source1_field645)
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
                                                                                        let source_ml_source1_field649 = (transition.clone()).1;
                                                                                        {
                                                                                            let source_ml_source1_field650 = if (((transition.clone()).0) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_field651 = (transition.clone()).0;
                                                                                                {
                                                                                                    let source_ml_source1_field652 = (transition.clone()).2;
                                                                                                    {
                                                                                                        let source_ml_source1_field653 = self::JavaScriptSemanticValue::Undefined;
                                                                                                        {
                                                                                                            let source_ml_source1_field654 = receiver.clone();
                                                                                                            {
                                                                                                                let source_ml_source1_field655 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                (source_ml_source1_field650.clone(), source_ml_source1_field655, source_ml_source1_field653.clone(), source_ml_source1_field651.clone(), source_ml_source1_field649.clone(), source_ml_source1_field654.clone(), source_ml_source1_field652)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
                let source_ml_source1_field656 = Vec::<self::JavaScriptSemanticValue>::new();
                {
                    let source_ml_source1_field657 = error.clone();
                    (source_ml_source1_field657.clone(), source_ml_source1_field656)
                }
            }
        } else {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value658, _) => {
                        source_ml_source1_data_property_value658
                    }
                }, identity.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value659, _) => source_ml_source1_data_property_value659.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        {
                            let source_ml_source1_field660 = Vec::<self::JavaScriptSemanticValue>::new();
                            {
                                let source_ml_source1_field661 = self::ml_string::Text::from_units(vec![79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 75, 101, 121, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source1_field661.clone(), source_ml_source1_field660)
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
                                            let source_ml_temporary_values23 = match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value662) => {
                                                    source_ml_source1_data_property_value662
                                                }
                                            };
                                            {
                                                let source_ml_temporary_index23 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s663 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop24(numeric, positions, text, symbols, source_ml_temporary_values23, source_ml_temporary_index23.clone());
                                                    match source_ml_source1_s663.clone() {
                                                        self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop24Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop24Done(numeric_2, text_2, symbols_2) => {
                                                            {
                                                                let source_ml_source1_field667 = {
                                                                    let source_ml_source1_concat_value664 = numeric_2;
                                                                    {
                                                                        let source_ml_source1_concat_value665 = text_2;
                                                                        {
                                                                            let source_ml_source1_concat_value666 = symbols_2;
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value664), source_ml_source1_concat_value665), source_ml_source1_concat_value666)
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_field668 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source1_field668.clone(), source_ml_source1_field667)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
                let source_ml_source1_field669 = heap.clone();
                {
                    let source_ml_source1_field670 = false;
                    {
                        let source_ml_source1_field671 = heap_error.clone();
                        (source_ml_source1_field671.clone(), source_ml_source1_field669.clone(), source_ml_source1_field670)
                    }
                }
            }
        } else {
            if ((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value673, _) => source_ml_source1_data_property_value673.as_slice() }).len() as f64)) >= (4096f64)) {
                {
                    let source_ml_source1_field674 = heap.clone();
                    {
                        let source_ml_source1_field675 = false;
                        {
                            let source_ml_source1_field676 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]);
                            (source_ml_source1_field676.clone(), source_ml_source1_field674.clone(), source_ml_source1_field675)
                        }
                    }
                }
            } else {
                if ((((identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value678, _) => {
                        source_ml_source1_data_property_value678
                    }
                }, identity.clone())) >= (0f64))) || self::test_java_script_ordinary_symbol(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value679) => {
                        source_ml_source1_data_property_value679
                    }
                }, identity.clone())) {
                    {
                        let source_ml_source1_field680 = heap.clone();
                        {
                            let source_ml_source1_field681 = false;
                            {
                                let source_ml_source1_field682 = self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source1_field682.clone(), source_ml_source1_field680.clone(), source_ml_source1_field681)
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f683) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f684) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f685) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f686) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f687) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f688, source_ml_source1_f689) => {
                            true
                        }
                    } && match prototype.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            true
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f690) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f691) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f692) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f693) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f694) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f695, source_ml_source1_f696) => {
                            false
                        }
                    }) {
                        {
                            let source_ml_source1_field697 = heap.clone();
                            {
                                let source_ml_source1_field698 = false;
                                {
                                    let source_ml_source1_field699 = self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]);
                                    (source_ml_source1_field699.clone(), source_ml_source1_field697.clone(), source_ml_source1_field698)
                                }
                            }
                        }
                    } else {
                        {
                            let error = self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                {
                                    let source_ml_source1_field700 = heap.clone();
                                    {
                                        let source_ml_source1_field701 = false;
                                        {
                                            let source_ml_source1_field702 = error.clone();
                                            (source_ml_source1_field702.clone(), source_ml_source1_field700.clone(), source_ml_source1_field701)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_temporary_values26 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value703, _) => {
                                                source_ml_source1_data_property_value703
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index26 = self::ml::Big::from_i128(0);
                                            {
                                                let at_2 = self::source_ml_temporary_insert_java_script_ordinary_object_loop27(identity.clone(), at, source_ml_temporary_values26, source_ml_temporary_index26.clone());
                                                {
                                                    let object = self::JavaScriptOrdinaryObject::OrdinaryObject(identity.clone(), Box::new(prototype.clone()), true, false, Vec::<self::JavaScriptOrdinaryProperty>::new());
                                                    {
                                                        let objects = {
                                                            let source_ml_source1_concat_value708 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value705, _) => source_ml_source1_data_property_value705.as_slice() }), 0f64, at_2);
                                                            {
                                                                let source_ml_source1_concat_value709 = vec![object.clone()];
                                                                {
                                                                    let source_ml_source1_concat_value710 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value707, _) => source_ml_source1_data_property_value707.as_slice() }), at_2, f64::INFINITY);
                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), source_ml_source1_concat_value708), source_ml_source1_concat_value709), source_ml_source1_concat_value710)
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_field712 = self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value711) => {
                                                                    source_ml_source1_data_property_value711
                                                                }
                                                            });
                                                            {
                                                                let source_ml_source1_field713 = true;
                                                                {
                                                                    let source_ml_source1_field714 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source1_field714.clone(), source_ml_source1_field712.clone(), source_ml_source1_field713)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
