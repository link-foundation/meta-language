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
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value626, _) => {
                                                source_ml_source2_data_property_value626
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
                                let source_ml_source2_field627 = heap.clone();
                                {
                                    let source_ml_source2_field628 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                    {
                                        let source_ml_source2_field629 = error.clone();
                                        {
                                            let source_ml_source2_field630 = false;
                                            {
                                                let source_ml_source2_field631 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source2_field632 = receiver.clone();
                                                    {
                                                        let source_ml_source2_field633 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source2_field628.clone(), source_ml_source2_field633, source_ml_source2_field631.clone(), source_ml_source2_field629.clone(), source_ml_source2_field627.clone(), source_ml_source2_field632.clone(), source_ml_source2_field630)
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
                                                        let source_ml_source2_field641 = heap.clone();
                                                        {
                                                            let source_ml_source2_field642 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field643 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field644 = false;
                                                                    {
                                                                        let source_ml_source2_field645 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field646 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field647 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field642.clone(), source_ml_source2_field647, source_ml_source2_field645.clone(), source_ml_source2_field643.clone(), source_ml_source2_field641.clone(), source_ml_source2_field646.clone(), source_ml_source2_field644)
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
                                                        let source_ml_source2_field648 = heap.clone();
                                                        {
                                                            let source_ml_source2_field649 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field650 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field651 = false;
                                                                    {
                                                                        let source_ml_source2_field652 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field653 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field654 = vec![value.clone()];
                                                                                (source_ml_source2_field649.clone(), source_ml_source2_field654, source_ml_source2_field652.clone(), source_ml_source2_field650.clone(), source_ml_source2_field648.clone(), source_ml_source2_field653.clone(), source_ml_source2_field651)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f634) => {
                                                    {
                                                        let source_ml_source2_field655 = heap.clone();
                                                        {
                                                            let source_ml_source2_field656 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field657 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field658 = false;
                                                                    {
                                                                        let source_ml_source2_field659 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field660 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field661 = vec![value.clone()];
                                                                                (source_ml_source2_field656.clone(), source_ml_source2_field661, source_ml_source2_field659.clone(), source_ml_source2_field657.clone(), source_ml_source2_field655.clone(), source_ml_source2_field660.clone(), source_ml_source2_field658)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source2_f635) => {
                                                    {
                                                        let source_ml_source2_field662 = heap.clone();
                                                        {
                                                            let source_ml_source2_field663 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field664 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field665 = false;
                                                                    {
                                                                        let source_ml_source2_field666 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field667 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field668 = vec![value.clone()];
                                                                                (source_ml_source2_field663.clone(), source_ml_source2_field668, source_ml_source2_field666.clone(), source_ml_source2_field664.clone(), source_ml_source2_field662.clone(), source_ml_source2_field667.clone(), source_ml_source2_field665)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f636) => {
                                                    {
                                                        let source_ml_source2_field669 = heap.clone();
                                                        {
                                                            let source_ml_source2_field670 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field671 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field672 = false;
                                                                    {
                                                                        let source_ml_source2_field673 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field674 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field675 = vec![value.clone()];
                                                                                (source_ml_source2_field670.clone(), source_ml_source2_field675, source_ml_source2_field673.clone(), source_ml_source2_field671.clone(), source_ml_source2_field669.clone(), source_ml_source2_field674.clone(), source_ml_source2_field672)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source2_f637) => {
                                                    {
                                                        let source_ml_source2_field676 = heap.clone();
                                                        {
                                                            let source_ml_source2_field677 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field678 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field679 = false;
                                                                    {
                                                                        let source_ml_source2_field680 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field681 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field682 = vec![value.clone()];
                                                                                (source_ml_source2_field677.clone(), source_ml_source2_field682, source_ml_source2_field680.clone(), source_ml_source2_field678.clone(), source_ml_source2_field676.clone(), source_ml_source2_field681.clone(), source_ml_source2_field679)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f638) => {
                                                    {
                                                        let source_ml_source2_field683 = heap.clone();
                                                        {
                                                            let source_ml_source2_field684 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field685 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field686 = false;
                                                                    {
                                                                        let source_ml_source2_field687 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field688 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field689 = vec![value.clone()];
                                                                                (source_ml_source2_field684.clone(), source_ml_source2_field689, source_ml_source2_field687.clone(), source_ml_source2_field685.clone(), source_ml_source2_field683.clone(), source_ml_source2_field688.clone(), source_ml_source2_field686)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source2_f639, source_ml_source2_f640) => {
                                                    {
                                                        let source_ml_source2_field690 = heap.clone();
                                                        {
                                                            let source_ml_source2_field691 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field692 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field693 = false;
                                                                    {
                                                                        let source_ml_source2_field694 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field695 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field696 = vec![value.clone()];
                                                                                (source_ml_source2_field691.clone(), source_ml_source2_field696, source_ml_source2_field694.clone(), source_ml_source2_field692.clone(), source_ml_source2_field690.clone(), source_ml_source2_field695.clone(), source_ml_source2_field693)
                                                                            }
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
                                                let source_ml_source2_field697 = heap.clone();
                                                {
                                                    let source_ml_source2_field698 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                    {
                                                        let source_ml_source2_field699 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field700 = false;
                                                            {
                                                                let source_ml_source2_field701 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field702 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field703 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field698.clone(), source_ml_source2_field703, source_ml_source2_field701.clone(), source_ml_source2_field699.clone(), source_ml_source2_field697.clone(), source_ml_source2_field702.clone(), source_ml_source2_field700)
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
                                                        let source_ml_source2_field710 = heap.clone();
                                                        {
                                                            let source_ml_source2_field711 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field712 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field713 = false;
                                                                    {
                                                                        let source_ml_source2_field714 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field715 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field716 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field711.clone(), source_ml_source2_field716, source_ml_source2_field714.clone(), source_ml_source2_field712.clone(), source_ml_source2_field710.clone(), source_ml_source2_field715.clone(), source_ml_source2_field713)
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
                                                        let source_ml_source2_field717 = heap.clone();
                                                        {
                                                            let source_ml_source2_field718 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field719 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field720 = false;
                                                                    {
                                                                        let source_ml_source2_field721 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field722 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field723 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field718.clone(), source_ml_source2_field723, source_ml_source2_field721.clone(), source_ml_source2_field719.clone(), source_ml_source2_field717.clone(), source_ml_source2_field722.clone(), source_ml_source2_field720)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f704) => {
                                                    {
                                                        let source_ml_source2_field724 = heap.clone();
                                                        {
                                                            let source_ml_source2_field725 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field726 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field727 = false;
                                                                    {
                                                                        let source_ml_source2_field728 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field729 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field730 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field725.clone(), source_ml_source2_field730, source_ml_source2_field728.clone(), source_ml_source2_field726.clone(), source_ml_source2_field724.clone(), source_ml_source2_field729.clone(), source_ml_source2_field727)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source2_f705) => {
                                                    {
                                                        let source_ml_source2_field731 = heap.clone();
                                                        {
                                                            let source_ml_source2_field732 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field733 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field734 = false;
                                                                    {
                                                                        let source_ml_source2_field735 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field736 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field737 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field732.clone(), source_ml_source2_field737, source_ml_source2_field735.clone(), source_ml_source2_field733.clone(), source_ml_source2_field731.clone(), source_ml_source2_field736.clone(), source_ml_source2_field734)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f706) => {
                                                    {
                                                        let source_ml_source2_field738 = heap.clone();
                                                        {
                                                            let source_ml_source2_field739 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field740 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field741 = false;
                                                                    {
                                                                        let source_ml_source2_field742 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field743 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field744 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field739.clone(), source_ml_source2_field744, source_ml_source2_field742.clone(), source_ml_source2_field740.clone(), source_ml_source2_field738.clone(), source_ml_source2_field743.clone(), source_ml_source2_field741)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source2_f707) => {
                                                    {
                                                        let source_ml_source2_field745 = heap.clone();
                                                        {
                                                            let source_ml_source2_field746 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field747 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field748 = false;
                                                                    {
                                                                        let source_ml_source2_field749 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field750 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field751 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field746.clone(), source_ml_source2_field751, source_ml_source2_field749.clone(), source_ml_source2_field747.clone(), source_ml_source2_field745.clone(), source_ml_source2_field750.clone(), source_ml_source2_field748)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f708) => {
                                                    {
                                                        let source_ml_source2_field752 = heap.clone();
                                                        {
                                                            let source_ml_source2_field753 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field754 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field755 = false;
                                                                    {
                                                                        let source_ml_source2_field756 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field757 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field758 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field753.clone(), source_ml_source2_field758, source_ml_source2_field756.clone(), source_ml_source2_field754.clone(), source_ml_source2_field752.clone(), source_ml_source2_field757.clone(), source_ml_source2_field755)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f709) => {
                                                    {
                                                        let index = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value759, _) => {
                                                                source_ml_source2_data_property_value759
                                                            }
                                                        }, identity_2.clone());
                                                        {
                                                            let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value760, _) => source_ml_source2_data_property_value760.as_slice() }), self::ml_array::number_index(index));
                                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                {
                                                                    let source_ml_source2_field761 = heap.clone();
                                                                    {
                                                                        let source_ml_source2_field762 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                                                        {
                                                                            let source_ml_source2_field763 = self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]);
                                                                            {
                                                                                let source_ml_source2_field764 = false;
                                                                                {
                                                                                    let source_ml_source2_field765 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source2_field766 = receiver.clone();
                                                                                        {
                                                                                            let source_ml_source2_field767 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                            (source_ml_source2_field762.clone(), source_ml_source2_field767, source_ml_source2_field765.clone(), source_ml_source2_field763.clone(), source_ml_source2_field761.clone(), source_ml_source2_field766.clone(), source_ml_source2_field764)
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
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value768) => {
                                                                            source_ml_source2_data_property_value768
                                                                        }
                                                                    }, key.clone());
                                                                    {
                                                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value769) => {
                                                                                source_ml_source2_data_property_value769
                                                                            }
                                                                        }, self::ml_array::number_index(at));
                                                                        if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({
                                                                            let source_ml_source2_nullish_field_receiver770 = (property.clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver770.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present771 = ((source_ml_source2_nullish_field_receiver770.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source2_nullish_field_present771.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value772) => {
                                                                                            source_ml_source2_data_property_value772.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).9) || !(({
                                                                            let source_ml_source2_nullish_field_receiver773 = (property.clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver773.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present774 = ((source_ml_source2_nullish_field_receiver773.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source2_nullish_field_present774.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value775) => {
                                                                                            source_ml_source2_data_property_value775.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).10))) {
                                                                            {
                                                                                let source_ml_source2_field776 = heap.clone();
                                                                                {
                                                                                    let source_ml_source2_field777 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                    {
                                                                                        let source_ml_source2_field778 = self::ml_string::Text::from_units(vec![]);
                                                                                        {
                                                                                            let source_ml_source2_field779 = false;
                                                                                            {
                                                                                                let source_ml_source2_field780 = self::JavaScriptSemanticValue::Undefined;
                                                                                                {
                                                                                                    let source_ml_source2_field781 = receiver.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_field782 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                        (source_ml_source2_field777.clone(), source_ml_source2_field782, source_ml_source2_field780.clone(), source_ml_source2_field778.clone(), source_ml_source2_field776.clone(), source_ml_source2_field781.clone(), source_ml_source2_field779)
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
                                                                                        let source_ml_source2_field783 = (transition.clone()).1;
                                                                                        {
                                                                                            let source_ml_source2_field784 = if (((transition.clone()).0) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_field785 = (transition.clone()).0;
                                                                                                {
                                                                                                    let source_ml_source2_field786 = (transition.clone()).2;
                                                                                                    {
                                                                                                        let source_ml_source2_field787 = self::JavaScriptSemanticValue::Undefined;
                                                                                                        {
                                                                                                            let source_ml_source2_field788 = receiver.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_field789 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                (source_ml_source2_field784.clone(), source_ml_source2_field789, source_ml_source2_field787.clone(), source_ml_source2_field785.clone(), source_ml_source2_field783.clone(), source_ml_source2_field788.clone(), source_ml_source2_field786)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
            let source_ml_source2_field875 = self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]);
            {
                let source_ml_source2_field876 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                {
                    let source_ml_source2_field877 = vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                    (source_ml_source2_field876.clone(), source_ml_source2_field875.clone(), source_ml_source2_field877)
                }
            }
        }
    } else {
        if (index == (1f64)) {
            {
                let source_ml_source2_field878 = self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]);
                {
                    let source_ml_source2_field879 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                    {
                        let source_ml_source2_field880 = vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                        (source_ml_source2_field879.clone(), source_ml_source2_field878.clone(), source_ml_source2_field880)
                    }
                }
            }
        } else {
            if (index == (2f64)) {
                {
                    let source_ml_source2_field881 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                    {
                        let source_ml_source2_field882 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                        {
                            let source_ml_source2_field883 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                            (source_ml_source2_field882.clone(), source_ml_source2_field881.clone(), source_ml_source2_field883)
                        }
                    }
                }
            } else {
                if (index == (3f64)) {
                    {
                        let source_ml_source2_field884 = self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]);
                        {
                            let source_ml_source2_field885 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                            {
                                let source_ml_source2_field886 = vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                                (source_ml_source2_field885.clone(), source_ml_source2_field884.clone(), source_ml_source2_field886)
                            }
                        }
                    }
                } else {
                    if (index == (4f64)) {
                        {
                            let source_ml_source2_field887 = self::ml_string::Text::from_units(vec![103, 101, 116]);
                            {
                                let source_ml_source2_field888 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                {
                                    let source_ml_source2_field889 = vec![103f64, 101f64, 116f64];
                                    (source_ml_source2_field888.clone(), source_ml_source2_field887.clone(), source_ml_source2_field889)
                                }
                            }
                        }
                    } else {
                        if (index == (5f64)) {
                            {
                                let source_ml_source2_field890 = self::ml_string::Text::from_units(vec![115, 101, 116]);
                                {
                                    let source_ml_source2_field891 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                    {
                                        let source_ml_source2_field892 = vec![115f64, 101f64, 116f64];
                                        (source_ml_source2_field891.clone(), source_ml_source2_field890.clone(), source_ml_source2_field892)
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source2_field893 = self::ml_string::Text::from_units(vec![]);
                                {
                                    let source_ml_source2_field894 = self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]);
                                    {
                                        let source_ml_source2_field895 = Vec::<f64>::new();
                                        (source_ml_source2_field894.clone(), source_ml_source2_field893.clone(), source_ml_source2_field895)
                                    }
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
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f896) => {
                true
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f897, source_ml_source2_f898) => {
                true
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field899 = false;
        {
            let source_ml_source2_field900 = false;
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
                                    let source_ml_source2_field906 = false;
                                    {
                                        let source_ml_source2_field907 = false;
                                        {
                                            let source_ml_source2_field908 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field909 = false;
                                                {
                                                    let source_ml_source2_field910 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field900, source_ml_source2_field899, source_ml_source2_field902, source_ml_source2_field901, source_ml_source2_field908.clone(), source_ml_source2_field907, source_ml_source2_field910.clone(), source_ml_source2_field909, source_ml_source2_field904.clone(), source_ml_source2_field903, source_ml_source2_field906, source_ml_source2_field905)
                                                }
                                            }
                                        }
                                    }
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
