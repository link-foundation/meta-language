// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result;
use super::create_java_script_ordinary_set_descriptor;
use super::define_java_script_ordinary_property;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_value_error;
use super::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop122;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_set(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, value: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> (self::ml_string::Text, Vec<self::JavaScriptSemanticValue>, self::JavaScriptSemanticValue, self::ml_string::Text, self::JavaScriptOrdinaryHeap, self::JavaScriptSemanticValue, bool) {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _) => {
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
                                                if ((self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone())) < (0f64)) {
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
                                        let source_ml_source2_field640 = heap.clone();
                                        {
                                            let source_ml_source2_field641 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                            {
                                                let source_ml_source2_field642 = error.clone();
                                                {
                                                    let source_ml_source2_field643 = false;
                                                    {
                                                        let source_ml_source2_field644 = self::JavaScriptSemanticValue::Undefined;
                                                        {
                                                            let source_ml_source2_field645 = receiver.clone();
                                                            {
                                                                let source_ml_source2_field646 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                (source_ml_source2_field641.clone(), source_ml_source2_field646, source_ml_source2_field644.clone(), source_ml_source2_field642.clone(), source_ml_source2_field640.clone(), source_ml_source2_field645.clone(), source_ml_source2_field643)
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
                                                                let source_ml_source2_field654 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field655 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                    {
                                                                        let source_ml_source2_field656 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field657 = false;
                                                                            {
                                                                                let source_ml_source2_field658 = self::JavaScriptSemanticValue::Undefined;
                                                                                {
                                                                                    let source_ml_source2_field659 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field660 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                        (source_ml_source2_field655.clone(), source_ml_source2_field660, source_ml_source2_field658.clone(), source_ml_source2_field656.clone(), source_ml_source2_field654.clone(), source_ml_source2_field659.clone(), source_ml_source2_field657)
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
                                                                let source_ml_source2_field661 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field662 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                    {
                                                                        let source_ml_source2_field663 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field664 = false;
                                                                            {
                                                                                let source_ml_source2_field665 = setter.clone();
                                                                                {
                                                                                    let source_ml_source2_field666 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field667 = vec![value.clone()];
                                                                                        (source_ml_source2_field662.clone(), source_ml_source2_field667, source_ml_source2_field665.clone(), source_ml_source2_field663.clone(), source_ml_source2_field661.clone(), source_ml_source2_field666.clone(), source_ml_source2_field664)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                            {
                                                                let source_ml_source2_field668 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field669 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                    {
                                                                        let source_ml_source2_field670 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field671 = false;
                                                                            {
                                                                                let source_ml_source2_field672 = setter.clone();
                                                                                {
                                                                                    let source_ml_source2_field673 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field674 = vec![value.clone()];
                                                                                        (source_ml_source2_field669.clone(), source_ml_source2_field674, source_ml_source2_field672.clone(), source_ml_source2_field670.clone(), source_ml_source2_field668.clone(), source_ml_source2_field673.clone(), source_ml_source2_field671)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                            {
                                                                let source_ml_source2_field675 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field676 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                    {
                                                                        let source_ml_source2_field677 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field678 = false;
                                                                            {
                                                                                let source_ml_source2_field679 = setter.clone();
                                                                                {
                                                                                    let source_ml_source2_field680 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field681 = vec![value.clone()];
                                                                                        (source_ml_source2_field676.clone(), source_ml_source2_field681, source_ml_source2_field679.clone(), source_ml_source2_field677.clone(), source_ml_source2_field675.clone(), source_ml_source2_field680.clone(), source_ml_source2_field678)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            {
                                                                let source_ml_source2_field682 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field683 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                    {
                                                                        let source_ml_source2_field684 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field685 = false;
                                                                            {
                                                                                let source_ml_source2_field686 = setter.clone();
                                                                                {
                                                                                    let source_ml_source2_field687 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field688 = vec![value.clone()];
                                                                                        (source_ml_source2_field683.clone(), source_ml_source2_field688, source_ml_source2_field686.clone(), source_ml_source2_field684.clone(), source_ml_source2_field682.clone(), source_ml_source2_field687.clone(), source_ml_source2_field685)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            {
                                                                let source_ml_source2_field689 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field690 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                    {
                                                                        let source_ml_source2_field691 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field692 = false;
                                                                            {
                                                                                let source_ml_source2_field693 = setter.clone();
                                                                                {
                                                                                    let source_ml_source2_field694 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field695 = vec![value.clone()];
                                                                                        (source_ml_source2_field690.clone(), source_ml_source2_field695, source_ml_source2_field693.clone(), source_ml_source2_field691.clone(), source_ml_source2_field689.clone(), source_ml_source2_field694.clone(), source_ml_source2_field692)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            {
                                                                let source_ml_source2_field696 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field697 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                    {
                                                                        let source_ml_source2_field698 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field699 = false;
                                                                            {
                                                                                let source_ml_source2_field700 = setter.clone();
                                                                                {
                                                                                    let source_ml_source2_field701 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field702 = vec![value.clone()];
                                                                                        (source_ml_source2_field697.clone(), source_ml_source2_field702, source_ml_source2_field700.clone(), source_ml_source2_field698.clone(), source_ml_source2_field696.clone(), source_ml_source2_field701.clone(), source_ml_source2_field699)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            {
                                                                let source_ml_source2_field703 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field704 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                    {
                                                                        let source_ml_source2_field705 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field706 = false;
                                                                            {
                                                                                let source_ml_source2_field707 = setter.clone();
                                                                                {
                                                                                    let source_ml_source2_field708 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field709 = vec![value.clone()];
                                                                                        (source_ml_source2_field704.clone(), source_ml_source2_field709, source_ml_source2_field707.clone(), source_ml_source2_field705.clone(), source_ml_source2_field703.clone(), source_ml_source2_field708.clone(), source_ml_source2_field706)
                                                                                    }
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
                                                } else {
                                                    match receiver.clone() {
                                                        self::JavaScriptSemanticValue::Undefined => {
                                                            {
                                                                let source_ml_source2_field723 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field724 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                    {
                                                                        let source_ml_source2_field725 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field726 = false;
                                                                            {
                                                                                let source_ml_source2_field727 = self::JavaScriptSemanticValue::Undefined;
                                                                                {
                                                                                    let source_ml_source2_field728 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field729 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                        (source_ml_source2_field724.clone(), source_ml_source2_field729, source_ml_source2_field727.clone(), source_ml_source2_field725.clone(), source_ml_source2_field723.clone(), source_ml_source2_field728.clone(), source_ml_source2_field726)
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
                                                                let source_ml_source2_field730 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field731 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                    {
                                                                        let source_ml_source2_field732 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field733 = false;
                                                                            {
                                                                                let source_ml_source2_field734 = self::JavaScriptSemanticValue::Undefined;
                                                                                {
                                                                                    let source_ml_source2_field735 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field736 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                        (source_ml_source2_field731.clone(), source_ml_source2_field736, source_ml_source2_field734.clone(), source_ml_source2_field732.clone(), source_ml_source2_field730.clone(), source_ml_source2_field735.clone(), source_ml_source2_field733)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                            {
                                                                let source_ml_source2_field737 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field738 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                    {
                                                                        let source_ml_source2_field739 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field740 = false;
                                                                            {
                                                                                let source_ml_source2_field741 = self::JavaScriptSemanticValue::Undefined;
                                                                                {
                                                                                    let source_ml_source2_field742 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field743 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                        (source_ml_source2_field738.clone(), source_ml_source2_field743, source_ml_source2_field741.clone(), source_ml_source2_field739.clone(), source_ml_source2_field737.clone(), source_ml_source2_field742.clone(), source_ml_source2_field740)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                            {
                                                                let source_ml_source2_field744 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field745 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                    {
                                                                        let source_ml_source2_field746 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field747 = false;
                                                                            {
                                                                                let source_ml_source2_field748 = self::JavaScriptSemanticValue::Undefined;
                                                                                {
                                                                                    let source_ml_source2_field749 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field750 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                        (source_ml_source2_field745.clone(), source_ml_source2_field750, source_ml_source2_field748.clone(), source_ml_source2_field746.clone(), source_ml_source2_field744.clone(), source_ml_source2_field749.clone(), source_ml_source2_field747)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            {
                                                                let source_ml_source2_field751 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field752 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                    {
                                                                        let source_ml_source2_field753 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field754 = false;
                                                                            {
                                                                                let source_ml_source2_field755 = self::JavaScriptSemanticValue::Undefined;
                                                                                {
                                                                                    let source_ml_source2_field756 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field757 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                        (source_ml_source2_field752.clone(), source_ml_source2_field757, source_ml_source2_field755.clone(), source_ml_source2_field753.clone(), source_ml_source2_field751.clone(), source_ml_source2_field756.clone(), source_ml_source2_field754)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            {
                                                                let source_ml_source2_field758 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field759 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                    {
                                                                        let source_ml_source2_field760 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field761 = false;
                                                                            {
                                                                                let source_ml_source2_field762 = self::JavaScriptSemanticValue::Undefined;
                                                                                {
                                                                                    let source_ml_source2_field763 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field764 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                        (source_ml_source2_field759.clone(), source_ml_source2_field764, source_ml_source2_field762.clone(), source_ml_source2_field760.clone(), source_ml_source2_field758.clone(), source_ml_source2_field763.clone(), source_ml_source2_field761)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            {
                                                                let source_ml_source2_field765 = heap.clone();
                                                                {
                                                                    let source_ml_source2_field766 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                    {
                                                                        let source_ml_source2_field767 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source2_field768 = false;
                                                                            {
                                                                                let source_ml_source2_field769 = self::JavaScriptSemanticValue::Undefined;
                                                                                {
                                                                                    let source_ml_source2_field770 = receiver.clone();
                                                                                    {
                                                                                        let source_ml_source2_field771 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                        (source_ml_source2_field766.clone(), source_ml_source2_field771, source_ml_source2_field769.clone(), source_ml_source2_field767.clone(), source_ml_source2_field765.clone(), source_ml_source2_field770.clone(), source_ml_source2_field768)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                                            {
                                                                let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity_2.clone());
                                                                {
                                                                    let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(index));
                                                                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        {
                                                                            let source_ml_source2_field774 = heap.clone();
                                                                            {
                                                                                let source_ml_source2_field775 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                                                                {
                                                                                    let source_ml_source2_field776 = self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]);
                                                                                    {
                                                                                        let source_ml_source2_field777 = false;
                                                                                        {
                                                                                            let source_ml_source2_field778 = self::JavaScriptSemanticValue::Undefined;
                                                                                            {
                                                                                                let source_ml_source2_field779 = receiver.clone();
                                                                                                {
                                                                                                    let source_ml_source2_field780 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                    (source_ml_source2_field775.clone(), source_ml_source2_field780, source_ml_source2_field778.clone(), source_ml_source2_field776.clone(), source_ml_source2_field774.clone(), source_ml_source2_field779.clone(), source_ml_source2_field777)
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
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value781) => {
                                                                                    source_ml_source2_data_property_value781
                                                                                }
                                                                            }, key.clone());
                                                                            {
                                                                                let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value782) => {
                                                                                        source_ml_source2_data_property_value782
                                                                                    }
                                                                                }, self::ml_array::number_index(at));
                                                                                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({
                                                                                    let source_ml_source2_nullish_field_receiver783 = (property.clone(),);
                                                                                    if match (source_ml_source2_nullish_field_receiver783.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                        {
                                                                                            let source_ml_source2_nullish_field_present784 = ((source_ml_source2_nullish_field_receiver783.clone()).0).clone().expect("a proven present value");
                                                                                            match source_ml_source2_nullish_field_present784.clone() {
                                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value785) => {
                                                                                                    source_ml_source2_data_property_value785.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                                    }
                                                                                }).9) || !(({
                                                                                    let source_ml_source2_nullish_field_receiver786 = (property.clone(),);
                                                                                    if match (source_ml_source2_nullish_field_receiver786.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                        {
                                                                                            let source_ml_source2_nullish_field_present787 = ((source_ml_source2_nullish_field_receiver786.clone()).0).clone().expect("a proven present value");
                                                                                            match source_ml_source2_nullish_field_present787.clone() {
                                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value788) => {
                                                                                                    source_ml_source2_data_property_value788.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                                    }
                                                                                }).10))) {
                                                                                    {
                                                                                        let source_ml_source2_field789 = heap.clone();
                                                                                        {
                                                                                            let source_ml_source2_field790 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                            {
                                                                                                let source_ml_source2_field791 = self::ml_string::Text::from_units(vec![]);
                                                                                                {
                                                                                                    let source_ml_source2_field792 = false;
                                                                                                    {
                                                                                                        let source_ml_source2_field793 = self::JavaScriptSemanticValue::Undefined;
                                                                                                        {
                                                                                                            let source_ml_source2_field794 = receiver.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_field795 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                (source_ml_source2_field790.clone(), source_ml_source2_field795, source_ml_source2_field793.clone(), source_ml_source2_field791.clone(), source_ml_source2_field789.clone(), source_ml_source2_field794.clone(), source_ml_source2_field792)
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
                                                                                                let source_ml_source2_field796 = (transition.clone()).1;
                                                                                                {
                                                                                                    let source_ml_source2_field797 = if (((transition.clone()).0) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                        self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                                    } else {
                                                                                                        self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_field798 = (transition.clone()).0;
                                                                                                        {
                                                                                                            let source_ml_source2_field799 = (transition.clone()).2;
                                                                                                            {
                                                                                                                let source_ml_source2_field800 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                {
                                                                                                                    let source_ml_source2_field801 = receiver.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_field802 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                        (source_ml_source2_field797.clone(), source_ml_source2_field802, source_ml_source2_field800.clone(), source_ml_source2_field798.clone(), source_ml_source2_field796.clone(), source_ml_source2_field801.clone(), source_ml_source2_field799)
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
            let source_ml_source2_field888 = self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]);
            {
                let source_ml_source2_field889 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                {
                    let source_ml_source2_field890 = vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                    (source_ml_source2_field889.clone(), source_ml_source2_field888.clone(), source_ml_source2_field890)
                }
            }
        }
    } else {
        if (index == (1f64)) {
            {
                let source_ml_source2_field891 = self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]);
                {
                    let source_ml_source2_field892 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                    {
                        let source_ml_source2_field893 = vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                        (source_ml_source2_field892.clone(), source_ml_source2_field891.clone(), source_ml_source2_field893)
                    }
                }
            }
        } else {
            if (index == (2f64)) {
                {
                    let source_ml_source2_field894 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                    {
                        let source_ml_source2_field895 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                        {
                            let source_ml_source2_field896 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                            (source_ml_source2_field895.clone(), source_ml_source2_field894.clone(), source_ml_source2_field896)
                        }
                    }
                }
            } else {
                if (index == (3f64)) {
                    {
                        let source_ml_source2_field897 = self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]);
                        {
                            let source_ml_source2_field898 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                            {
                                let source_ml_source2_field899 = vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                                (source_ml_source2_field898.clone(), source_ml_source2_field897.clone(), source_ml_source2_field899)
                            }
                        }
                    }
                } else {
                    if (index == (4f64)) {
                        {
                            let source_ml_source2_field900 = self::ml_string::Text::from_units(vec![103, 101, 116]);
                            {
                                let source_ml_source2_field901 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                {
                                    let source_ml_source2_field902 = vec![103f64, 101f64, 116f64];
                                    (source_ml_source2_field901.clone(), source_ml_source2_field900.clone(), source_ml_source2_field902)
                                }
                            }
                        }
                    } else {
                        if (index == (5f64)) {
                            {
                                let source_ml_source2_field903 = self::ml_string::Text::from_units(vec![115, 101, 116]);
                                {
                                    let source_ml_source2_field904 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                    {
                                        let source_ml_source2_field905 = vec![115f64, 101f64, 116f64];
                                        (source_ml_source2_field904.clone(), source_ml_source2_field903.clone(), source_ml_source2_field905)
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source2_field906 = self::ml_string::Text::from_units(vec![]);
                                {
                                    let source_ml_source2_field907 = self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]);
                                    {
                                        let source_ml_source2_field908 = Vec::<f64>::new();
                                        (source_ml_source2_field907.clone(), source_ml_source2_field906.clone(), source_ml_source2_field908)
                                    }
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
            self::JavaScriptSemanticValue::Symbol(_) => {
                true
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                true
            }
            _ => {
                false
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field912 = false;
        {
            let source_ml_source2_field913 = false;
            {
                let source_ml_source2_field914 = false;
                {
                    let source_ml_source2_field915 = false;
                    {
                        let source_ml_source2_field916 = false;
                        {
                            let source_ml_source2_field917 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_field918 = false;
                                {
                                    let source_ml_source2_field919 = false;
                                    {
                                        let source_ml_source2_field920 = false;
                                        {
                                            let source_ml_source2_field921 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field922 = false;
                                                {
                                                    let source_ml_source2_field923 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field913, source_ml_source2_field912, source_ml_source2_field915, source_ml_source2_field914, source_ml_source2_field921.clone(), source_ml_source2_field920, source_ml_source2_field923.clone(), source_ml_source2_field922, source_ml_source2_field917.clone(), source_ml_source2_field916, source_ml_source2_field919, source_ml_source2_field918)
                                                }
                                            }
                                        }
                                    }
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_conversion_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                    heap_error.clone()
                } else {
                    if {
                        let source_ml_source2_s925 = shared_constructor_field1.clone();
                        match source_ml_source2_s925.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                false
                            }
                            _ => {
                                true
                            }
                        }
                    } {
                        self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                    } else {
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field1.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                receiver_error.clone()
                            } else {
                                if ((((!(self::ml_math::is_safe_integer(shared_constructor_field2)) || (shared_constructor_field2 < (0f64))) || (shared_constructor_field2 > (6f64))) || ((shared_constructor_field2 == (0f64)) && (((1f64 / shared_constructor_field2)) < (0f64)))) || (shared_constructor_field4 && (shared_constructor_field2 == (6f64)))) {
                                    self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 102, 105, 101, 108, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                                } else {
                                    {
                                        let index = shared_constructor_field2;
                                        {
                                            let source_ml_source2_s948 = self::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop122(state.clone(), index);
                                            match source_ml_source2_s948.clone() {
                                                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Done => {
                                                    {
                                                        let descriptor = shared_constructor_field3.clone();
                                                        if ((descriptor.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                                                            self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 118, 97, 108, 117, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116])
                                                        } else {
                                                            if ((descriptor.clone()).5 && (((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).4)))) {
                                                                self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                            } else {
                                                                if ((descriptor.clone()).7 && (((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).6)))) {
                                                                    self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Return(source_ml_source1_temporary_result122) => {
                                                    source_ml_source1_temporary_result122.clone()
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
