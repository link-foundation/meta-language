// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Result;
use super::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop30;
use super::source_ml_temporary_register_java_script_ordinary_symbol_loop29;
use super::source_ml_temporary_step_java_script_descriptor_conversion_loop31;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn register_java_script_ordinary_symbol(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source1_field715 = heap.clone();
                {
                    let source_ml_source1_field716 = false;
                    {
                        let source_ml_source1_field717 = error.clone();
                        (source_ml_source1_field717.clone(), source_ml_source1_field715.clone(), source_ml_source1_field716)
                    }
                }
            }
        } else {
            if ((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value719) => source_ml_source1_data_property_value719.as_slice() }).len() as f64)) >= (16384f64)) {
                {
                    let source_ml_source1_field720 = heap.clone();
                    {
                        let source_ml_source1_field721 = false;
                        {
                            let source_ml_source1_field722 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]);
                            (source_ml_source1_field722.clone(), source_ml_source1_field720.clone(), source_ml_source1_field721)
                        }
                    }
                }
            } else {
                if ((((identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value724, _) => {
                        source_ml_source1_data_property_value724
                    }
                }, identity.clone())) >= (0f64))) || self::test_java_script_ordinary_symbol(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value725) => {
                        source_ml_source1_data_property_value725
                    }
                }, identity.clone())) {
                    {
                        let source_ml_source1_field726 = heap.clone();
                        {
                            let source_ml_source1_field727 = false;
                            {
                                let source_ml_source1_field728 = self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source1_field728.clone(), source_ml_source1_field726.clone(), source_ml_source1_field727)
                            }
                        }
                    }
                } else {
                    {
                        let at = 0f64;
                        {
                            let source_ml_temporary_values28 = match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value729) => {
                                    source_ml_source1_data_property_value729
                                }
                            };
                            {
                                let source_ml_temporary_index28 = self::ml::Big::from_i128(0);
                                {
                                    let at_2 = self::source_ml_temporary_register_java_script_ordinary_symbol_loop29(identity.clone(), at, source_ml_temporary_values28, source_ml_temporary_index28.clone());
                                    {
                                        let symbols = {
                                            let source_ml_source1_concat_value734 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value731) => source_ml_source1_data_property_value731.as_slice() }), 0f64, at_2);
                                            {
                                                let source_ml_source1_concat_value735 = vec![identity.clone()];
                                                {
                                                    let source_ml_source1_concat_value736 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value733) => source_ml_source1_data_property_value733.as_slice() }), at_2, f64::INFINITY);
                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml::Big>::new(), source_ml_source1_concat_value734), source_ml_source1_concat_value735), source_ml_source1_concat_value736)
                                                }
                                            }
                                        };
                                        {
                                            let source_ml_source1_field738 = self::JavaScriptOrdinaryHeap::OrdinaryHeap(match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value737, _) => {
                                                    source_ml_source1_data_property_value737
                                                }
                                            }, symbols);
                                            {
                                                let source_ml_source1_field739 = true;
                                                {
                                                    let source_ml_source1_field740 = self::ml_string::Text::from_units(vec![]);
                                                    (source_ml_source1_field740.clone(), source_ml_source1_field738.clone(), source_ml_source1_field739)
                                                }
                                            }
                                        }
                                    }
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
            let source_ml_source1_field741 = self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]);
            {
                let source_ml_source1_field742 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                {
                    let source_ml_source1_field743 = vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                    (source_ml_source1_field742.clone(), source_ml_source1_field741.clone(), source_ml_source1_field743)
                }
            }
        }
    } else {
        if (index == (1f64)) {
            {
                let source_ml_source1_field744 = self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]);
                {
                    let source_ml_source1_field745 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                    {
                        let source_ml_source1_field746 = vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                        (source_ml_source1_field745.clone(), source_ml_source1_field744.clone(), source_ml_source1_field746)
                    }
                }
            }
        } else {
            if (index == (2f64)) {
                {
                    let source_ml_source1_field747 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                    {
                        let source_ml_source1_field748 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                        {
                            let source_ml_source1_field749 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                            (source_ml_source1_field748.clone(), source_ml_source1_field747.clone(), source_ml_source1_field749)
                        }
                    }
                }
            } else {
                if (index == (3f64)) {
                    {
                        let source_ml_source1_field750 = self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]);
                        {
                            let source_ml_source1_field751 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                            {
                                let source_ml_source1_field752 = vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                                (source_ml_source1_field751.clone(), source_ml_source1_field750.clone(), source_ml_source1_field752)
                            }
                        }
                    }
                } else {
                    if (index == (4f64)) {
                        {
                            let source_ml_source1_field753 = self::ml_string::Text::from_units(vec![103, 101, 116]);
                            {
                                let source_ml_source1_field754 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                {
                                    let source_ml_source1_field755 = vec![103f64, 101f64, 116f64];
                                    (source_ml_source1_field754.clone(), source_ml_source1_field753.clone(), source_ml_source1_field755)
                                }
                            }
                        }
                    } else {
                        if (index == (5f64)) {
                            {
                                let source_ml_source1_field756 = self::ml_string::Text::from_units(vec![115, 101, 116]);
                                {
                                    let source_ml_source1_field757 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                    {
                                        let source_ml_source1_field758 = vec![115f64, 101f64, 116f64];
                                        (source_ml_source1_field757.clone(), source_ml_source1_field756.clone(), source_ml_source1_field758)
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source1_field759 = self::ml_string::Text::from_units(vec![]);
                                {
                                    let source_ml_source1_field760 = self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]);
                                    {
                                        let source_ml_source1_field761 = Vec::<f64>::new();
                                        (source_ml_source1_field760.clone(), source_ml_source1_field759.clone(), source_ml_source1_field761)
                                    }
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
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f762) => {
                true
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f763, source_ml_source1_f764) => {
                true
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source1_field765 = false;
        {
            let source_ml_source1_field766 = false;
            {
                let source_ml_source1_field767 = false;
                {
                    let source_ml_source1_field768 = false;
                    {
                        let source_ml_source1_field769 = false;
                        {
                            let source_ml_source1_field770 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field771 = false;
                                {
                                    let source_ml_source1_field772 = false;
                                    {
                                        let source_ml_source1_field773 = false;
                                        {
                                            let source_ml_source1_field774 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field775 = false;
                                                {
                                                    let source_ml_source1_field776 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field766, source_ml_source1_field765, source_ml_source1_field768, source_ml_source1_field767, source_ml_source1_field774.clone(), source_ml_source1_field773, source_ml_source1_field776.clone(), source_ml_source1_field775, source_ml_source1_field770.clone(), source_ml_source1_field769, source_ml_source1_field772, source_ml_source1_field771)
                                                }
                                            }
                                        }
                                    }
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
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            heap_error.clone()
        } else {
            if {
                let source_ml_source1_s778 = match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value786, _, _, _) => {
                        let source_ml_source1_data_property_value786 = *source_ml_source1_data_property_value786;
                        source_ml_source1_data_property_value786.clone()
                    }
                };
                match source_ml_source1_s778.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f779) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f780) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f781) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f782) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f783) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f784, source_ml_source1_f785) => {
                        false
                    }
                }
            } {
                self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
            } else {
                {
                    let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value787, _, _, _) => {
                            let source_ml_source1_data_property_value787 = *source_ml_source1_data_property_value787;
                            source_ml_source1_data_property_value787.clone()
                        }
                    });
                    if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                        receiver_error.clone()
                    } else {
                        if ((((!(self::ml_math::is_safe_integer(match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value788, _, _) => {
                                source_ml_source1_data_property_value788
                            }
                        })) || ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value790, _, _) => {
                                source_ml_source1_data_property_value790
                            }
                        }) < (0f64))) || ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value792, _, _) => {
                                source_ml_source1_data_property_value792
                            }
                        }) > (6f64))) || (((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value794, _, _) => {
                                source_ml_source1_data_property_value794
                            }
                        }) == (0f64)) && (((1f64 / match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value796, _, _) => {
                                source_ml_source1_data_property_value796
                            }
                        })) < (0f64)))) || (match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value797) => {
                                source_ml_source1_data_property_value797
                            }
                        } && ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value799, _, _) => {
                                source_ml_source1_data_property_value799
                            }
                        }) == (6f64)))) {
                            self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 102, 105, 101, 108, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                        } else {
                            {
                                let index = match state.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value800, _, _) => {
                                        source_ml_source1_data_property_value800
                                    }
                                };
                                {
                                    let source_ml_source1_s801 = self::source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop30(state.clone(), index);
                                    match source_ml_source1_s801.clone() {
                                        self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Done => {
                                            {
                                                let descriptor = match state.clone() {
                                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value802, _) => {
                                                        source_ml_source1_data_property_value802.clone()
                                                    }
                                                };
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
                                        self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Return(source_ml_temporary_result30) => {
                                            source_ml_temporary_result30.clone()
                                        }
                                    }
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
pub fn apply_java_script_descriptor_field(state: self::JavaScriptDescriptorConversionState, value: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    if ((((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value804, _, _) => {
            source_ml_source1_data_property_value804
        }
    }) == (4f64)) || ((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value806, _, _) => {
            source_ml_source1_data_property_value806
        }
    }) == (5f64))) && !(self::test_java_script_semantic_accessor(value.clone()))) {
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
    } else {
        {
            let before = match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value807, _) => {
                    source_ml_source1_data_property_value807.clone()
                }
            };
            {
                let descriptor = if ((match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value809, _, _) => {
                        source_ml_source1_data_property_value809
                    }
                }) == (0f64)) {
                    {
                        let source_ml_source1_spread810 = before.clone();
                        {
                            let source_ml_source1_spread_field811 = (source_ml_source1_spread810.clone()).0;
                            {
                                let source_ml_source1_spread_field812 = (source_ml_source1_spread810.clone()).1;
                                {
                                    let source_ml_source1_spread_field813 = (source_ml_source1_spread810.clone()).2;
                                    {
                                        let source_ml_source1_spread_field814 = (source_ml_source1_spread810.clone()).3;
                                        {
                                            let source_ml_source1_spread_field815 = (source_ml_source1_spread810.clone()).4;
                                            {
                                                let source_ml_source1_spread_field816 = (source_ml_source1_spread810.clone()).5;
                                                {
                                                    let source_ml_source1_spread_field817 = (source_ml_source1_spread810.clone()).6;
                                                    {
                                                        let source_ml_source1_spread_field818 = (source_ml_source1_spread810.clone()).7;
                                                        {
                                                            let source_ml_source1_spread_field819 = (source_ml_source1_spread810.clone()).8;
                                                            {
                                                                let source_ml_source1_spread_field820 = (source_ml_source1_spread810.clone()).9;
                                                                {
                                                                    let source_ml_source1_spread_field821 = (source_ml_source1_spread810.clone()).10;
                                                                    {
                                                                        let source_ml_source1_spread_field822 = (source_ml_source1_spread810.clone()).11;
                                                                        {
                                                                            let source_ml_source1_field823 = true;
                                                                            {
                                                                                let source_ml_source1_field824 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                (source_ml_source1_spread_field811, source_ml_source1_spread_field812, source_ml_source1_field824, source_ml_source1_field823, source_ml_source1_spread_field815.clone(), source_ml_source1_spread_field816, source_ml_source1_spread_field817.clone(), source_ml_source1_spread_field818, source_ml_source1_spread_field819.clone(), source_ml_source1_spread_field820, source_ml_source1_spread_field821, source_ml_source1_spread_field822)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                    if ((match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value826, _, _) => {
                            source_ml_source1_data_property_value826
                        }
                    }) == (1f64)) {
                        {
                            let source_ml_source1_spread827 = before.clone();
                            {
                                let source_ml_source1_spread_field828 = (source_ml_source1_spread827.clone()).0;
                                {
                                    let source_ml_source1_spread_field829 = (source_ml_source1_spread827.clone()).1;
                                    {
                                        let source_ml_source1_spread_field830 = (source_ml_source1_spread827.clone()).2;
                                        {
                                            let source_ml_source1_spread_field831 = (source_ml_source1_spread827.clone()).3;
                                            {
                                                let source_ml_source1_spread_field832 = (source_ml_source1_spread827.clone()).4;
                                                {
                                                    let source_ml_source1_spread_field833 = (source_ml_source1_spread827.clone()).5;
                                                    {
                                                        let source_ml_source1_spread_field834 = (source_ml_source1_spread827.clone()).6;
                                                        {
                                                            let source_ml_source1_spread_field835 = (source_ml_source1_spread827.clone()).7;
                                                            {
                                                                let source_ml_source1_spread_field836 = (source_ml_source1_spread827.clone()).8;
                                                                {
                                                                    let source_ml_source1_spread_field837 = (source_ml_source1_spread827.clone()).9;
                                                                    {
                                                                        let source_ml_source1_spread_field838 = (source_ml_source1_spread827.clone()).10;
                                                                        {
                                                                            let source_ml_source1_spread_field839 = (source_ml_source1_spread827.clone()).11;
                                                                            {
                                                                                let source_ml_source1_field840 = true;
                                                                                {
                                                                                    let source_ml_source1_field841 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                    (source_ml_source1_field841, source_ml_source1_field840, source_ml_source1_spread_field830, source_ml_source1_spread_field831, source_ml_source1_spread_field832.clone(), source_ml_source1_spread_field833, source_ml_source1_spread_field834.clone(), source_ml_source1_spread_field835, source_ml_source1_spread_field836.clone(), source_ml_source1_spread_field837, source_ml_source1_spread_field838, source_ml_source1_spread_field839)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
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
                        if ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value843, _, _) => {
                                source_ml_source1_data_property_value843
                            }
                        }) == (2f64)) {
                            {
                                let source_ml_source1_spread844 = before.clone();
                                {
                                    let source_ml_source1_spread_field845 = (source_ml_source1_spread844.clone()).0;
                                    {
                                        let source_ml_source1_spread_field846 = (source_ml_source1_spread844.clone()).1;
                                        {
                                            let source_ml_source1_spread_field847 = (source_ml_source1_spread844.clone()).2;
                                            {
                                                let source_ml_source1_spread_field848 = (source_ml_source1_spread844.clone()).3;
                                                {
                                                    let source_ml_source1_spread_field849 = (source_ml_source1_spread844.clone()).4;
                                                    {
                                                        let source_ml_source1_spread_field850 = (source_ml_source1_spread844.clone()).5;
                                                        {
                                                            let source_ml_source1_spread_field851 = (source_ml_source1_spread844.clone()).6;
                                                            {
                                                                let source_ml_source1_spread_field852 = (source_ml_source1_spread844.clone()).7;
                                                                {
                                                                    let source_ml_source1_spread_field853 = (source_ml_source1_spread844.clone()).8;
                                                                    {
                                                                        let source_ml_source1_spread_field854 = (source_ml_source1_spread844.clone()).9;
                                                                        {
                                                                            let source_ml_source1_spread_field855 = (source_ml_source1_spread844.clone()).10;
                                                                            {
                                                                                let source_ml_source1_spread_field856 = (source_ml_source1_spread844.clone()).11;
                                                                                {
                                                                                    let source_ml_source1_field857 = true;
                                                                                    {
                                                                                        let source_ml_source1_field858 = value.clone();
                                                                                        (source_ml_source1_spread_field845, source_ml_source1_spread_field846, source_ml_source1_spread_field847, source_ml_source1_spread_field848, source_ml_source1_spread_field849.clone(), source_ml_source1_spread_field850, source_ml_source1_spread_field851.clone(), source_ml_source1_spread_field852, source_ml_source1_field858.clone(), source_ml_source1_field857, source_ml_source1_spread_field855, source_ml_source1_spread_field856)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                            if ((match state.clone() {
                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value860, _, _) => {
                                    source_ml_source1_data_property_value860
                                }
                            }) == (3f64)) {
                                {
                                    let source_ml_source1_spread861 = before.clone();
                                    {
                                        let source_ml_source1_spread_field862 = (source_ml_source1_spread861.clone()).0;
                                        {
                                            let source_ml_source1_spread_field863 = (source_ml_source1_spread861.clone()).1;
                                            {
                                                let source_ml_source1_spread_field864 = (source_ml_source1_spread861.clone()).2;
                                                {
                                                    let source_ml_source1_spread_field865 = (source_ml_source1_spread861.clone()).3;
                                                    {
                                                        let source_ml_source1_spread_field866 = (source_ml_source1_spread861.clone()).4;
                                                        {
                                                            let source_ml_source1_spread_field867 = (source_ml_source1_spread861.clone()).5;
                                                            {
                                                                let source_ml_source1_spread_field868 = (source_ml_source1_spread861.clone()).6;
                                                                {
                                                                    let source_ml_source1_spread_field869 = (source_ml_source1_spread861.clone()).7;
                                                                    {
                                                                        let source_ml_source1_spread_field870 = (source_ml_source1_spread861.clone()).8;
                                                                        {
                                                                            let source_ml_source1_spread_field871 = (source_ml_source1_spread861.clone()).9;
                                                                            {
                                                                                let source_ml_source1_spread_field872 = (source_ml_source1_spread861.clone()).10;
                                                                                {
                                                                                    let source_ml_source1_spread_field873 = (source_ml_source1_spread861.clone()).11;
                                                                                    {
                                                                                        let source_ml_source1_field874 = true;
                                                                                        {
                                                                                            let source_ml_source1_field875 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                            (source_ml_source1_spread_field862, source_ml_source1_spread_field863, source_ml_source1_spread_field864, source_ml_source1_spread_field865, source_ml_source1_spread_field866.clone(), source_ml_source1_spread_field867, source_ml_source1_spread_field868.clone(), source_ml_source1_spread_field869, source_ml_source1_spread_field870.clone(), source_ml_source1_spread_field871, source_ml_source1_field875, source_ml_source1_field874)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
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
                                if ((match state.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value877, _, _) => {
                                        source_ml_source1_data_property_value877
                                    }
                                }) == (4f64)) {
                                    {
                                        let source_ml_source1_spread878 = before.clone();
                                        {
                                            let source_ml_source1_spread_field879 = (source_ml_source1_spread878.clone()).0;
                                            {
                                                let source_ml_source1_spread_field880 = (source_ml_source1_spread878.clone()).1;
                                                {
                                                    let source_ml_source1_spread_field881 = (source_ml_source1_spread878.clone()).2;
                                                    {
                                                        let source_ml_source1_spread_field882 = (source_ml_source1_spread878.clone()).3;
                                                        {
                                                            let source_ml_source1_spread_field883 = (source_ml_source1_spread878.clone()).4;
                                                            {
                                                                let source_ml_source1_spread_field884 = (source_ml_source1_spread878.clone()).5;
                                                                {
                                                                    let source_ml_source1_spread_field885 = (source_ml_source1_spread878.clone()).6;
                                                                    {
                                                                        let source_ml_source1_spread_field886 = (source_ml_source1_spread878.clone()).7;
                                                                        {
                                                                            let source_ml_source1_spread_field887 = (source_ml_source1_spread878.clone()).8;
                                                                            {
                                                                                let source_ml_source1_spread_field888 = (source_ml_source1_spread878.clone()).9;
                                                                                {
                                                                                    let source_ml_source1_spread_field889 = (source_ml_source1_spread878.clone()).10;
                                                                                    {
                                                                                        let source_ml_source1_spread_field890 = (source_ml_source1_spread878.clone()).11;
                                                                                        {
                                                                                            let source_ml_source1_field891 = true;
                                                                                            {
                                                                                                let source_ml_source1_field892 = value.clone();
                                                                                                (source_ml_source1_spread_field879, source_ml_source1_spread_field880, source_ml_source1_spread_field881, source_ml_source1_spread_field882, source_ml_source1_field892.clone(), source_ml_source1_field891, source_ml_source1_spread_field885.clone(), source_ml_source1_spread_field886, source_ml_source1_spread_field887.clone(), source_ml_source1_spread_field888, source_ml_source1_spread_field889, source_ml_source1_spread_field890)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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
                                    {
                                        let source_ml_source1_spread893 = before.clone();
                                        {
                                            let source_ml_source1_spread_field894 = (source_ml_source1_spread893.clone()).0;
                                            {
                                                let source_ml_source1_spread_field895 = (source_ml_source1_spread893.clone()).1;
                                                {
                                                    let source_ml_source1_spread_field896 = (source_ml_source1_spread893.clone()).2;
                                                    {
                                                        let source_ml_source1_spread_field897 = (source_ml_source1_spread893.clone()).3;
                                                        {
                                                            let source_ml_source1_spread_field898 = (source_ml_source1_spread893.clone()).4;
                                                            {
                                                                let source_ml_source1_spread_field899 = (source_ml_source1_spread893.clone()).5;
                                                                {
                                                                    let source_ml_source1_spread_field900 = (source_ml_source1_spread893.clone()).6;
                                                                    {
                                                                        let source_ml_source1_spread_field901 = (source_ml_source1_spread893.clone()).7;
                                                                        {
                                                                            let source_ml_source1_spread_field902 = (source_ml_source1_spread893.clone()).8;
                                                                            {
                                                                                let source_ml_source1_spread_field903 = (source_ml_source1_spread893.clone()).9;
                                                                                {
                                                                                    let source_ml_source1_spread_field904 = (source_ml_source1_spread893.clone()).10;
                                                                                    {
                                                                                        let source_ml_source1_spread_field905 = (source_ml_source1_spread893.clone()).11;
                                                                                        {
                                                                                            let source_ml_source1_field906 = true;
                                                                                            {
                                                                                                let source_ml_source1_field907 = value.clone();
                                                                                                (source_ml_source1_spread_field894, source_ml_source1_spread_field895, source_ml_source1_spread_field896, source_ml_source1_spread_field897, source_ml_source1_spread_field898.clone(), source_ml_source1_spread_field899, source_ml_source1_field907.clone(), source_ml_source1_field906, source_ml_source1_spread_field902.clone(), source_ml_source1_spread_field903, source_ml_source1_spread_field904, source_ml_source1_spread_field905)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                };
                self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value908, _, _, _) => {
                        let source_ml_source1_data_property_value908 = *source_ml_source1_data_property_value908;
                        source_ml_source1_data_property_value908.clone()
                    }
                }), (match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value909, _, _) => {
                        source_ml_source1_data_property_value909
                    }
                } + 1f64), descriptor.clone(), false)))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error = self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else {
            if match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value910) => {
                    source_ml_source1_data_property_value910
                }
            } {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 110, 101, 120, 116, 32, 115, 116, 101, 112]))
            } else {
                {
                    let current = state.clone();
                    {
                        let source_ml_source1_s911 = self::source_ml_temporary_step_java_script_descriptor_conversion_loop31(heap.clone(), current.clone());
                        match source_ml_source1_s911.clone() {
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Done(current_2) => {
                                let current_2 = *current_2;
                                if ((self::read_java_script_property_descriptor_error(match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value913, _) => {
                                        source_ml_source1_data_property_value913.clone()
                                    }
                                })) != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 98, 111, 116, 104, 32, 100, 97, 116, 97, 32, 97, 110, 100, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                } else {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(match current_2.clone() {
                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value914, _) => {
                                            source_ml_source1_data_property_value914.clone()
                                        }
                                    })
                                }
                            }
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(source_ml_temporary_result31) => {
                                let source_ml_temporary_result31 = *source_ml_temporary_result31;
                                source_ml_temporary_result31.clone()
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
pub fn begin_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, receiver: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(heap_error.clone())
        } else {
            {
                let error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
                } else {
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f915) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f916) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f917) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f918) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f919) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f920, source_ml_source1_f921) => {
                            self::step_java_script_descriptor_conversion(heap.clone(), self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), 0f64, self::create_java_script_empty_descriptor(), false))
                        }
                    }
                }
            }
        }
    }
}
