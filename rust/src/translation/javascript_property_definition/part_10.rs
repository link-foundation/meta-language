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
use super::source_ml_temporary_read_java_script_ordinary_own_keys_loop23;

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
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value527, _) => {
                                                source_ml_source1_data_property_value527
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
                                let source_ml_source1_field528 = heap.clone();
                                {
                                    let source_ml_source1_field529 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                    {
                                        let source_ml_source1_field530 = error.clone();
                                        {
                                            let source_ml_source1_field531 = false;
                                            {
                                                let source_ml_source1_field532 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source1_field533 = receiver.clone();
                                                    {
                                                        let source_ml_source1_field534 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source1_field529.clone(), source_ml_source1_field534, source_ml_source1_field532.clone(), source_ml_source1_field530.clone(), source_ml_source1_field528.clone(), source_ml_source1_field533.clone(), source_ml_source1_field531)
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
                                                        let source_ml_source1_field542 = heap.clone();
                                                        {
                                                            let source_ml_source1_field543 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field544 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field545 = false;
                                                                    {
                                                                        let source_ml_source1_field546 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field547 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field548 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field543.clone(), source_ml_source1_field548, source_ml_source1_field546.clone(), source_ml_source1_field544.clone(), source_ml_source1_field542.clone(), source_ml_source1_field547.clone(), source_ml_source1_field545)
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
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f535) => {
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
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f536) => {
                                                    {
                                                        let source_ml_source1_field563 = heap.clone();
                                                        {
                                                            let source_ml_source1_field564 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field565 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field566 = false;
                                                                    {
                                                                        let source_ml_source1_field567 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field568 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field569 = vec![value.clone()];
                                                                                (source_ml_source1_field564.clone(), source_ml_source1_field569, source_ml_source1_field567.clone(), source_ml_source1_field565.clone(), source_ml_source1_field563.clone(), source_ml_source1_field568.clone(), source_ml_source1_field566)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f537) => {
                                                    {
                                                        let source_ml_source1_field570 = heap.clone();
                                                        {
                                                            let source_ml_source1_field571 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field572 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field573 = false;
                                                                    {
                                                                        let source_ml_source1_field574 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field575 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field576 = vec![value.clone()];
                                                                                (source_ml_source1_field571.clone(), source_ml_source1_field576, source_ml_source1_field574.clone(), source_ml_source1_field572.clone(), source_ml_source1_field570.clone(), source_ml_source1_field575.clone(), source_ml_source1_field573)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f538) => {
                                                    {
                                                        let source_ml_source1_field577 = heap.clone();
                                                        {
                                                            let source_ml_source1_field578 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field579 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field580 = false;
                                                                    {
                                                                        let source_ml_source1_field581 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field582 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field583 = vec![value.clone()];
                                                                                (source_ml_source1_field578.clone(), source_ml_source1_field583, source_ml_source1_field581.clone(), source_ml_source1_field579.clone(), source_ml_source1_field577.clone(), source_ml_source1_field582.clone(), source_ml_source1_field580)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f539) => {
                                                    {
                                                        let source_ml_source1_field584 = heap.clone();
                                                        {
                                                            let source_ml_source1_field585 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field586 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field587 = false;
                                                                    {
                                                                        let source_ml_source1_field588 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field589 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field590 = vec![value.clone()];
                                                                                (source_ml_source1_field585.clone(), source_ml_source1_field590, source_ml_source1_field588.clone(), source_ml_source1_field586.clone(), source_ml_source1_field584.clone(), source_ml_source1_field589.clone(), source_ml_source1_field587)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f540, source_ml_source1_f541) => {
                                                    {
                                                        let source_ml_source1_field591 = heap.clone();
                                                        {
                                                            let source_ml_source1_field592 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source1_field593 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field594 = false;
                                                                    {
                                                                        let source_ml_source1_field595 = setter.clone();
                                                                        {
                                                                            let source_ml_source1_field596 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field597 = vec![value.clone()];
                                                                                (source_ml_source1_field592.clone(), source_ml_source1_field597, source_ml_source1_field595.clone(), source_ml_source1_field593.clone(), source_ml_source1_field591.clone(), source_ml_source1_field596.clone(), source_ml_source1_field594)
                                                                            }
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
                                                let source_ml_source1_field598 = heap.clone();
                                                {
                                                    let source_ml_source1_field599 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                    {
                                                        let source_ml_source1_field600 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field601 = false;
                                                            {
                                                                let source_ml_source1_field602 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_field603 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field604 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field599.clone(), source_ml_source1_field604, source_ml_source1_field602.clone(), source_ml_source1_field600.clone(), source_ml_source1_field598.clone(), source_ml_source1_field603.clone(), source_ml_source1_field601)
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
                                                self::JavaScriptSemanticValue::Null => {
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
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f605) => {
                                                    {
                                                        let source_ml_source1_field625 = heap.clone();
                                                        {
                                                            let source_ml_source1_field626 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field627 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field628 = false;
                                                                    {
                                                                        let source_ml_source1_field629 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field630 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field631 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field626.clone(), source_ml_source1_field631, source_ml_source1_field629.clone(), source_ml_source1_field627.clone(), source_ml_source1_field625.clone(), source_ml_source1_field630.clone(), source_ml_source1_field628)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f606) => {
                                                    {
                                                        let source_ml_source1_field632 = heap.clone();
                                                        {
                                                            let source_ml_source1_field633 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field634 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field635 = false;
                                                                    {
                                                                        let source_ml_source1_field636 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field637 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field638 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field633.clone(), source_ml_source1_field638, source_ml_source1_field636.clone(), source_ml_source1_field634.clone(), source_ml_source1_field632.clone(), source_ml_source1_field637.clone(), source_ml_source1_field635)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f607) => {
                                                    {
                                                        let source_ml_source1_field639 = heap.clone();
                                                        {
                                                            let source_ml_source1_field640 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field641 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field642 = false;
                                                                    {
                                                                        let source_ml_source1_field643 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field644 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field645 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field640.clone(), source_ml_source1_field645, source_ml_source1_field643.clone(), source_ml_source1_field641.clone(), source_ml_source1_field639.clone(), source_ml_source1_field644.clone(), source_ml_source1_field642)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f608) => {
                                                    {
                                                        let source_ml_source1_field646 = heap.clone();
                                                        {
                                                            let source_ml_source1_field647 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field648 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field649 = false;
                                                                    {
                                                                        let source_ml_source1_field650 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field651 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field652 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field647.clone(), source_ml_source1_field652, source_ml_source1_field650.clone(), source_ml_source1_field648.clone(), source_ml_source1_field646.clone(), source_ml_source1_field651.clone(), source_ml_source1_field649)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f609) => {
                                                    {
                                                        let source_ml_source1_field653 = heap.clone();
                                                        {
                                                            let source_ml_source1_field654 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source1_field655 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field656 = false;
                                                                    {
                                                                        let source_ml_source1_field657 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field658 = receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field659 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field654.clone(), source_ml_source1_field659, source_ml_source1_field657.clone(), source_ml_source1_field655.clone(), source_ml_source1_field653.clone(), source_ml_source1_field658.clone(), source_ml_source1_field656)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f610) => {
                                                    {
                                                        let index = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value660, _) => {
                                                                source_ml_source1_data_property_value660
                                                            }
                                                        }, identity_2.clone());
                                                        {
                                                            let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value661, _) => source_ml_source1_data_property_value661.as_slice() }), self::ml_array::number_index(index));
                                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                {
                                                                    let source_ml_source1_field662 = heap.clone();
                                                                    {
                                                                        let source_ml_source1_field663 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                                                        {
                                                                            let source_ml_source1_field664 = self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]);
                                                                            {
                                                                                let source_ml_source1_field665 = false;
                                                                                {
                                                                                    let source_ml_source1_field666 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source1_field667 = receiver.clone();
                                                                                        {
                                                                                            let source_ml_source1_field668 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                            (source_ml_source1_field663.clone(), source_ml_source1_field668, source_ml_source1_field666.clone(), source_ml_source1_field664.clone(), source_ml_source1_field662.clone(), source_ml_source1_field667.clone(), source_ml_source1_field665)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    let at = self::find_java_script_ordinary_property({
                                                                        let source_ml_source1_nullish_field_receiver669 = (object.clone(),);
                                                                        if match (source_ml_source1_nullish_field_receiver669.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source1_nullish_field_present670 = ((source_ml_source1_nullish_field_receiver669.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source1_nullish_field_present670.clone() {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value671) => {
                                                                                        source_ml_source1_data_property_value671
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    }, key.clone());
                                                                    {
                                                                        let property = self::ml_array::optional_at(&{
                                                                            let source_ml_source1_nullish_field_receiver672 = (object.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver672.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present673 = ((source_ml_source1_nullish_field_receiver672.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present673.clone() {
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value674) => {
                                                                                            source_ml_source1_data_property_value674
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }, self::ml_array::number_index(at));
                                                                        if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({
                                                                            let source_ml_source1_nullish_field_receiver675 = (property.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver675.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present676 = ((source_ml_source1_nullish_field_receiver675.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present676.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value677) => {
                                                                                            source_ml_source1_data_property_value677.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).9) || !(({
                                                                            let source_ml_source1_nullish_field_receiver678 = (property.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver678.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present679 = ((source_ml_source1_nullish_field_receiver678.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present679.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value680) => {
                                                                                            source_ml_source1_data_property_value680.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).10))) {
                                                                            {
                                                                                let source_ml_source1_field681 = heap.clone();
                                                                                {
                                                                                    let source_ml_source1_field682 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                    {
                                                                                        let source_ml_source1_field683 = self::ml_string::Text::from_units(vec![]);
                                                                                        {
                                                                                            let source_ml_source1_field684 = false;
                                                                                            {
                                                                                                let source_ml_source1_field685 = self::JavaScriptSemanticValue::Undefined;
                                                                                                {
                                                                                                    let source_ml_source1_field686 = receiver.clone();
                                                                                                    {
                                                                                                        let source_ml_source1_field687 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                        (source_ml_source1_field682.clone(), source_ml_source1_field687, source_ml_source1_field685.clone(), source_ml_source1_field683.clone(), source_ml_source1_field681.clone(), source_ml_source1_field686.clone(), source_ml_source1_field684)
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
                                                                                        let source_ml_source1_field688 = (transition.clone()).1;
                                                                                        {
                                                                                            let source_ml_source1_field689 = if (((transition.clone()).0) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_field690 = (transition.clone()).0;
                                                                                                {
                                                                                                    let source_ml_source1_field691 = (transition.clone()).2;
                                                                                                    {
                                                                                                        let source_ml_source1_field692 = self::JavaScriptSemanticValue::Undefined;
                                                                                                        {
                                                                                                            let source_ml_source1_field693 = receiver.clone();
                                                                                                            {
                                                                                                                let source_ml_source1_field694 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                (source_ml_source1_field689.clone(), source_ml_source1_field694, source_ml_source1_field692.clone(), source_ml_source1_field690.clone(), source_ml_source1_field688.clone(), source_ml_source1_field693.clone(), source_ml_source1_field691)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
                let source_ml_source1_field695 = Vec::<self::JavaScriptSemanticValue>::new();
                {
                    let source_ml_source1_field696 = error.clone();
                    (source_ml_source1_field696.clone(), source_ml_source1_field695)
                }
            }
        } else {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value697, _) => {
                        source_ml_source1_data_property_value697
                    }
                }, identity.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value698, _) => source_ml_source1_data_property_value698.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        {
                            let source_ml_source1_field699 = Vec::<self::JavaScriptSemanticValue>::new();
                            {
                                let source_ml_source1_field700 = self::ml_string::Text::from_units(vec![79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 75, 101, 121, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source1_field700.clone(), source_ml_source1_field699)
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
                                            let source_ml_temporary_values22 = {
                                                let source_ml_source1_nullish_field_receiver701 = (object.clone(),);
                                                if match (source_ml_source1_nullish_field_receiver701.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source1_nullish_field_present702 = ((source_ml_source1_nullish_field_receiver701.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source1_nullish_field_present702.clone() {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value703) => {
                                                                source_ml_source1_data_property_value703
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                }
                                            };
                                            {
                                                let source_ml_temporary_index22 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s704 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop23(numeric, positions, text, symbols, source_ml_temporary_values22, source_ml_temporary_index22.clone());
                                                    match source_ml_source1_s704.clone() {
                                                        self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Done(numeric_2, text_2, symbols_2) => {
                                                            {
                                                                let source_ml_source1_field708 = {
                                                                    let source_ml_source1_concat_value705 = numeric_2;
                                                                    {
                                                                        let source_ml_source1_concat_value706 = text_2;
                                                                        {
                                                                            let source_ml_source1_concat_value707 = symbols_2;
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value705), source_ml_source1_concat_value706), source_ml_source1_concat_value707)
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_field709 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source1_field709.clone(), source_ml_source1_field708)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
