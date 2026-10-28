// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
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
use super::read_java_script_semantic_value_error;

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
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value622, _) => {
                                                source_ml_source2_data_property_value622
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
                                let source_ml_source2_field623 = heap.clone();
                                {
                                    let source_ml_source2_field624 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                    {
                                        let source_ml_source2_field625 = error.clone();
                                        {
                                            let source_ml_source2_field626 = false;
                                            {
                                                let source_ml_source2_field627 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source2_field628 = receiver.clone();
                                                    {
                                                        let source_ml_source2_field629 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source2_field624.clone(), source_ml_source2_field629, source_ml_source2_field627.clone(), source_ml_source2_field625.clone(), source_ml_source2_field623.clone(), source_ml_source2_field628.clone(), source_ml_source2_field626)
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
                                                        let source_ml_source2_field637 = heap.clone();
                                                        {
                                                            let source_ml_source2_field638 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field639 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field640 = false;
                                                                    {
                                                                        let source_ml_source2_field641 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field642 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field643 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field638.clone(), source_ml_source2_field643, source_ml_source2_field641.clone(), source_ml_source2_field639.clone(), source_ml_source2_field637.clone(), source_ml_source2_field642.clone(), source_ml_source2_field640)
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
                                                        let source_ml_source2_field644 = heap.clone();
                                                        {
                                                            let source_ml_source2_field645 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field646 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field647 = false;
                                                                    {
                                                                        let source_ml_source2_field648 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field649 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field650 = vec![value.clone()];
                                                                                (source_ml_source2_field645.clone(), source_ml_source2_field650, source_ml_source2_field648.clone(), source_ml_source2_field646.clone(), source_ml_source2_field644.clone(), source_ml_source2_field649.clone(), source_ml_source2_field647)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f630) => {
                                                    {
                                                        let source_ml_source2_field651 = heap.clone();
                                                        {
                                                            let source_ml_source2_field652 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field653 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field654 = false;
                                                                    {
                                                                        let source_ml_source2_field655 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field656 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field657 = vec![value.clone()];
                                                                                (source_ml_source2_field652.clone(), source_ml_source2_field657, source_ml_source2_field655.clone(), source_ml_source2_field653.clone(), source_ml_source2_field651.clone(), source_ml_source2_field656.clone(), source_ml_source2_field654)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source2_f631) => {
                                                    {
                                                        let source_ml_source2_field658 = heap.clone();
                                                        {
                                                            let source_ml_source2_field659 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field660 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field661 = false;
                                                                    {
                                                                        let source_ml_source2_field662 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field663 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field664 = vec![value.clone()];
                                                                                (source_ml_source2_field659.clone(), source_ml_source2_field664, source_ml_source2_field662.clone(), source_ml_source2_field660.clone(), source_ml_source2_field658.clone(), source_ml_source2_field663.clone(), source_ml_source2_field661)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f632) => {
                                                    {
                                                        let source_ml_source2_field665 = heap.clone();
                                                        {
                                                            let source_ml_source2_field666 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field667 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field668 = false;
                                                                    {
                                                                        let source_ml_source2_field669 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field670 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field671 = vec![value.clone()];
                                                                                (source_ml_source2_field666.clone(), source_ml_source2_field671, source_ml_source2_field669.clone(), source_ml_source2_field667.clone(), source_ml_source2_field665.clone(), source_ml_source2_field670.clone(), source_ml_source2_field668)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source2_f633) => {
                                                    {
                                                        let source_ml_source2_field672 = heap.clone();
                                                        {
                                                            let source_ml_source2_field673 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field674 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field675 = false;
                                                                    {
                                                                        let source_ml_source2_field676 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field677 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field678 = vec![value.clone()];
                                                                                (source_ml_source2_field673.clone(), source_ml_source2_field678, source_ml_source2_field676.clone(), source_ml_source2_field674.clone(), source_ml_source2_field672.clone(), source_ml_source2_field677.clone(), source_ml_source2_field675)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f634) => {
                                                    {
                                                        let source_ml_source2_field679 = heap.clone();
                                                        {
                                                            let source_ml_source2_field680 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field681 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field682 = false;
                                                                    {
                                                                        let source_ml_source2_field683 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field684 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field685 = vec![value.clone()];
                                                                                (source_ml_source2_field680.clone(), source_ml_source2_field685, source_ml_source2_field683.clone(), source_ml_source2_field681.clone(), source_ml_source2_field679.clone(), source_ml_source2_field684.clone(), source_ml_source2_field682)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source2_f635, source_ml_source2_f636) => {
                                                    {
                                                        let source_ml_source2_field686 = heap.clone();
                                                        {
                                                            let source_ml_source2_field687 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field688 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field689 = false;
                                                                    {
                                                                        let source_ml_source2_field690 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field691 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field692 = vec![value.clone()];
                                                                                (source_ml_source2_field687.clone(), source_ml_source2_field692, source_ml_source2_field690.clone(), source_ml_source2_field688.clone(), source_ml_source2_field686.clone(), source_ml_source2_field691.clone(), source_ml_source2_field689)
                                                                            }
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
                                                let source_ml_source2_field693 = heap.clone();
                                                {
                                                    let source_ml_source2_field694 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                    {
                                                        let source_ml_source2_field695 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field696 = false;
                                                            {
                                                                let source_ml_source2_field697 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field698 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field699 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field694.clone(), source_ml_source2_field699, source_ml_source2_field697.clone(), source_ml_source2_field695.clone(), source_ml_source2_field693.clone(), source_ml_source2_field698.clone(), source_ml_source2_field696)
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
                                                        let source_ml_source2_field706 = heap.clone();
                                                        {
                                                            let source_ml_source2_field707 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field708 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field709 = false;
                                                                    {
                                                                        let source_ml_source2_field710 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field711 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field712 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field707.clone(), source_ml_source2_field712, source_ml_source2_field710.clone(), source_ml_source2_field708.clone(), source_ml_source2_field706.clone(), source_ml_source2_field711.clone(), source_ml_source2_field709)
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
                                                        let source_ml_source2_field713 = heap.clone();
                                                        {
                                                            let source_ml_source2_field714 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field715 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field716 = false;
                                                                    {
                                                                        let source_ml_source2_field717 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field718 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field719 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field714.clone(), source_ml_source2_field719, source_ml_source2_field717.clone(), source_ml_source2_field715.clone(), source_ml_source2_field713.clone(), source_ml_source2_field718.clone(), source_ml_source2_field716)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f700) => {
                                                    {
                                                        let source_ml_source2_field720 = heap.clone();
                                                        {
                                                            let source_ml_source2_field721 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field722 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field723 = false;
                                                                    {
                                                                        let source_ml_source2_field724 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field725 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field726 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field721.clone(), source_ml_source2_field726, source_ml_source2_field724.clone(), source_ml_source2_field722.clone(), source_ml_source2_field720.clone(), source_ml_source2_field725.clone(), source_ml_source2_field723)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source2_f701) => {
                                                    {
                                                        let source_ml_source2_field727 = heap.clone();
                                                        {
                                                            let source_ml_source2_field728 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field729 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field730 = false;
                                                                    {
                                                                        let source_ml_source2_field731 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field732 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field733 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field728.clone(), source_ml_source2_field733, source_ml_source2_field731.clone(), source_ml_source2_field729.clone(), source_ml_source2_field727.clone(), source_ml_source2_field732.clone(), source_ml_source2_field730)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f702) => {
                                                    {
                                                        let source_ml_source2_field734 = heap.clone();
                                                        {
                                                            let source_ml_source2_field735 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field736 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field737 = false;
                                                                    {
                                                                        let source_ml_source2_field738 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field739 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field740 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field735.clone(), source_ml_source2_field740, source_ml_source2_field738.clone(), source_ml_source2_field736.clone(), source_ml_source2_field734.clone(), source_ml_source2_field739.clone(), source_ml_source2_field737)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source2_f703) => {
                                                    {
                                                        let source_ml_source2_field741 = heap.clone();
                                                        {
                                                            let source_ml_source2_field742 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field743 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field744 = false;
                                                                    {
                                                                        let source_ml_source2_field745 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field746 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field747 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field742.clone(), source_ml_source2_field747, source_ml_source2_field745.clone(), source_ml_source2_field743.clone(), source_ml_source2_field741.clone(), source_ml_source2_field746.clone(), source_ml_source2_field744)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f704) => {
                                                    {
                                                        let source_ml_source2_field748 = heap.clone();
                                                        {
                                                            let source_ml_source2_field749 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field750 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field751 = false;
                                                                    {
                                                                        let source_ml_source2_field752 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field753 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field754 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field749.clone(), source_ml_source2_field754, source_ml_source2_field752.clone(), source_ml_source2_field750.clone(), source_ml_source2_field748.clone(), source_ml_source2_field753.clone(), source_ml_source2_field751)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f705) => {
                                                    {
                                                        let index = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value755, _) => {
                                                                source_ml_source2_data_property_value755
                                                            }
                                                        }, identity_2.clone());
                                                        {
                                                            let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value756, _) => source_ml_source2_data_property_value756.as_slice() }), self::ml_array::number_index(index));
                                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                {
                                                                    let source_ml_source2_field757 = heap.clone();
                                                                    {
                                                                        let source_ml_source2_field758 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                                                        {
                                                                            let source_ml_source2_field759 = self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]);
                                                                            {
                                                                                let source_ml_source2_field760 = false;
                                                                                {
                                                                                    let source_ml_source2_field761 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source2_field762 = receiver.clone();
                                                                                        {
                                                                                            let source_ml_source2_field763 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                            (source_ml_source2_field758.clone(), source_ml_source2_field763, source_ml_source2_field761.clone(), source_ml_source2_field759.clone(), source_ml_source2_field757.clone(), source_ml_source2_field762.clone(), source_ml_source2_field760)
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
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value764) => {
                                                                            source_ml_source2_data_property_value764
                                                                        }
                                                                    }, key.clone());
                                                                    {
                                                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value765) => {
                                                                                source_ml_source2_data_property_value765
                                                                            }
                                                                        }, self::ml_array::number_index(at));
                                                                        if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({
                                                                            let source_ml_source2_nullish_field_receiver766 = (property.clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver766.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present767 = ((source_ml_source2_nullish_field_receiver766.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source2_nullish_field_present767.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value768) => {
                                                                                            source_ml_source2_data_property_value768.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).9) || !(({
                                                                            let source_ml_source2_nullish_field_receiver769 = (property.clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver769.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present770 = ((source_ml_source2_nullish_field_receiver769.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source2_nullish_field_present770.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value771) => {
                                                                                            source_ml_source2_data_property_value771.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).10))) {
                                                                            {
                                                                                let source_ml_source2_field772 = heap.clone();
                                                                                {
                                                                                    let source_ml_source2_field773 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                    {
                                                                                        let source_ml_source2_field774 = self::ml_string::Text::from_units(vec![]);
                                                                                        {
                                                                                            let source_ml_source2_field775 = false;
                                                                                            {
                                                                                                let source_ml_source2_field776 = self::JavaScriptSemanticValue::Undefined;
                                                                                                {
                                                                                                    let source_ml_source2_field777 = receiver.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_field778 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                        (source_ml_source2_field773.clone(), source_ml_source2_field778, source_ml_source2_field776.clone(), source_ml_source2_field774.clone(), source_ml_source2_field772.clone(), source_ml_source2_field777.clone(), source_ml_source2_field775)
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
                                                                                        let source_ml_source2_field779 = (transition.clone()).1;
                                                                                        {
                                                                                            let source_ml_source2_field780 = if (((transition.clone()).0) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_field781 = (transition.clone()).0;
                                                                                                {
                                                                                                    let source_ml_source2_field782 = (transition.clone()).2;
                                                                                                    {
                                                                                                        let source_ml_source2_field783 = self::JavaScriptSemanticValue::Undefined;
                                                                                                        {
                                                                                                            let source_ml_source2_field784 = receiver.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_field785 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                (source_ml_source2_field780.clone(), source_ml_source2_field785, source_ml_source2_field783.clone(), source_ml_source2_field781.clone(), source_ml_source2_field779.clone(), source_ml_source2_field784.clone(), source_ml_source2_field782)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
pub fn read_java_script_descriptor_field(index: f64) -> (self::ml_string::Text, self::ml_string::Text, Vec<f64>) {
    if (index == (0f64)) {
        {
            let source_ml_source2_field871 = self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]);
            {
                let source_ml_source2_field872 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                {
                    let source_ml_source2_field873 = vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                    (source_ml_source2_field872.clone(), source_ml_source2_field871.clone(), source_ml_source2_field873)
                }
            }
        }
    } else {
        if (index == (1f64)) {
            {
                let source_ml_source2_field874 = self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]);
                {
                    let source_ml_source2_field875 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                    {
                        let source_ml_source2_field876 = vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                        (source_ml_source2_field875.clone(), source_ml_source2_field874.clone(), source_ml_source2_field876)
                    }
                }
            }
        } else {
            if (index == (2f64)) {
                {
                    let source_ml_source2_field877 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                    {
                        let source_ml_source2_field878 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                        {
                            let source_ml_source2_field879 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                            (source_ml_source2_field878.clone(), source_ml_source2_field877.clone(), source_ml_source2_field879)
                        }
                    }
                }
            } else {
                if (index == (3f64)) {
                    {
                        let source_ml_source2_field880 = self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]);
                        {
                            let source_ml_source2_field881 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                            {
                                let source_ml_source2_field882 = vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                                (source_ml_source2_field881.clone(), source_ml_source2_field880.clone(), source_ml_source2_field882)
                            }
                        }
                    }
                } else {
                    if (index == (4f64)) {
                        {
                            let source_ml_source2_field883 = self::ml_string::Text::from_units(vec![103, 101, 116]);
                            {
                                let source_ml_source2_field884 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                {
                                    let source_ml_source2_field885 = vec![103f64, 101f64, 116f64];
                                    (source_ml_source2_field884.clone(), source_ml_source2_field883.clone(), source_ml_source2_field885)
                                }
                            }
                        }
                    } else {
                        if (index == (5f64)) {
                            {
                                let source_ml_source2_field886 = self::ml_string::Text::from_units(vec![115, 101, 116]);
                                {
                                    let source_ml_source2_field887 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                    {
                                        let source_ml_source2_field888 = vec![115f64, 101f64, 116f64];
                                        (source_ml_source2_field887.clone(), source_ml_source2_field886.clone(), source_ml_source2_field888)
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source2_field889 = self::ml_string::Text::from_units(vec![]);
                                {
                                    let source_ml_source2_field890 = self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]);
                                    {
                                        let source_ml_source2_field891 = Vec::<f64>::new();
                                        (source_ml_source2_field890.clone(), source_ml_source2_field889.clone(), source_ml_source2_field891)
                                    }
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
pub fn convert_java_script_semantic_boolean(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![84, 111, 66, 111, 111, 108, 101, 97, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                false
            }
            self::JavaScriptSemanticValue::Null => {
                false
            }
            self::JavaScriptSemanticValue::Boolean(value_value) => {
                value_value
            }
            self::JavaScriptSemanticValue::Number(value_value_2) => {
                ((value_value_2 != (0f64)) && !(value_value_2.is_nan()))
            }
            self::JavaScriptSemanticValue::Integer(value_value_3) => {
                (value_value_3 != (self::ml::Big::from_i128(0)))
            }
            self::JavaScriptSemanticValue::Text(units) => {
                (((units.len() as f64)) != (0f64))
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f892) => {
                true
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f893, source_ml_source2_f894) => {
                true
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field895 = false;
        {
            let source_ml_source2_field896 = false;
            {
                let source_ml_source2_field897 = false;
                {
                    let source_ml_source2_field898 = false;
                    {
                        let source_ml_source2_field899 = false;
                        {
                            let source_ml_source2_field900 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_field901 = false;
                                {
                                    let source_ml_source2_field902 = false;
                                    {
                                        let source_ml_source2_field903 = false;
                                        {
                                            let source_ml_source2_field904 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field905 = false;
                                                {
                                                    let source_ml_source2_field906 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field896, source_ml_source2_field895, source_ml_source2_field898, source_ml_source2_field897, source_ml_source2_field904.clone(), source_ml_source2_field903, source_ml_source2_field906.clone(), source_ml_source2_field905, source_ml_source2_field900.clone(), source_ml_source2_field899, source_ml_source2_field902, source_ml_source2_field901)
                                                }
                                            }
                                        }
                                    }
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
pub fn test_java_script_descriptor_field_presence(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), index: f64) -> bool {
    if (index == (0f64)) {
        (descriptor.clone()).3
    } else {
        if (index == (1f64)) {
            (descriptor.clone()).1
        } else {
            if (index == (2f64)) {
                (descriptor.clone()).9
            } else {
                if (index == (3f64)) {
                    (descriptor.clone()).11
                } else {
                    if (index == (4f64)) {
                        (descriptor.clone()).5
                    } else {
                        if (index == (5f64)) {
                            (descriptor.clone()).7
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}
