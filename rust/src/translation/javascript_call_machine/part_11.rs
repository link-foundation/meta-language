// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Result;
use super::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop56Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_insert_java_script_ordinary_object_loop59;
use super::source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop62;
use super::source_ml_temporary_read_java_script_ordinary_own_keys_loop56;
use super::source_ml_temporary_register_java_script_ordinary_symbol_loop61;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_own_keys(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> (self::ml_string::Text, Vec<self::JavaScriptSemanticValue>) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source1_field711 = Vec::<self::JavaScriptSemanticValue>::new();
                {
                    let source_ml_source1_field712 = error.clone();
                    (source_ml_source1_field712.clone(), source_ml_source1_field711)
                }
            }
        } else {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value713, _) => {
                        source_ml_source1_data_property_value713
                    }
                }, identity.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value714, _) => source_ml_source1_data_property_value714.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        {
                            let source_ml_source1_field715 = Vec::<self::JavaScriptSemanticValue>::new();
                            {
                                let source_ml_source1_field716 = self::ml_string::Text::from_units(vec![79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 75, 101, 121, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source1_field716.clone(), source_ml_source1_field715)
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
                                            let source_ml_temporary_values55 = match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value717) => {
                                                    source_ml_source1_data_property_value717
                                                }
                                            };
                                            {
                                                let source_ml_temporary_index55 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s718 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop56(numeric, positions, text, symbols, source_ml_temporary_values55, source_ml_temporary_index55.clone());
                                                    match source_ml_source1_s718.clone() {
                                                        self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop56Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop56Done(numeric_2, text_2, symbols_2) => {
                                                            {
                                                                let source_ml_source1_field722 = {
                                                                    let source_ml_source1_concat_value719 = numeric_2;
                                                                    {
                                                                        let source_ml_source1_concat_value720 = text_2;
                                                                        {
                                                                            let source_ml_source1_concat_value721 = symbols_2;
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value719), source_ml_source1_concat_value720), source_ml_source1_concat_value721)
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_field723 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source1_field723.clone(), source_ml_source1_field722)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
                let source_ml_source1_field724 = heap.clone();
                {
                    let source_ml_source1_field725 = false;
                    {
                        let source_ml_source1_field726 = heap_error.clone();
                        (source_ml_source1_field726.clone(), source_ml_source1_field724.clone(), source_ml_source1_field725)
                    }
                }
            }
        } else {
            if ((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value728, _) => source_ml_source1_data_property_value728.as_slice() }).len() as f64)) >= (4096f64)) {
                {
                    let source_ml_source1_field729 = heap.clone();
                    {
                        let source_ml_source1_field730 = false;
                        {
                            let source_ml_source1_field731 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]);
                            (source_ml_source1_field731.clone(), source_ml_source1_field729.clone(), source_ml_source1_field730)
                        }
                    }
                }
            } else {
                if ((((identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value733, _) => {
                        source_ml_source1_data_property_value733
                    }
                }, identity.clone())) >= (0f64))) || self::test_java_script_ordinary_symbol(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value734) => {
                        source_ml_source1_data_property_value734
                    }
                }, identity.clone())) {
                    {
                        let source_ml_source1_field735 = heap.clone();
                        {
                            let source_ml_source1_field736 = false;
                            {
                                let source_ml_source1_field737 = self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source1_field737.clone(), source_ml_source1_field735.clone(), source_ml_source1_field736)
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f738) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f739) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f740) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f741) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f742) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f743, source_ml_source1_f744) => {
                            true
                        }
                    } && match prototype.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            true
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f745) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f746) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f747) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f748) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f749) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f750, source_ml_source1_f751) => {
                            false
                        }
                    }) {
                        {
                            let source_ml_source1_field752 = heap.clone();
                            {
                                let source_ml_source1_field753 = false;
                                {
                                    let source_ml_source1_field754 = self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]);
                                    (source_ml_source1_field754.clone(), source_ml_source1_field752.clone(), source_ml_source1_field753)
                                }
                            }
                        }
                    } else {
                        {
                            let error = self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                {
                                    let source_ml_source1_field755 = heap.clone();
                                    {
                                        let source_ml_source1_field756 = false;
                                        {
                                            let source_ml_source1_field757 = error.clone();
                                            (source_ml_source1_field757.clone(), source_ml_source1_field755.clone(), source_ml_source1_field756)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_temporary_values58 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value758, _) => {
                                                source_ml_source1_data_property_value758
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index58 = self::ml::Big::from_i128(0);
                                            {
                                                let at_2 = self::source_ml_temporary_insert_java_script_ordinary_object_loop59(identity.clone(), at, source_ml_temporary_values58, source_ml_temporary_index58.clone());
                                                {
                                                    let object = self::JavaScriptOrdinaryObject::OrdinaryObject(identity.clone(), Box::new(prototype.clone()), true, false, Vec::<self::JavaScriptOrdinaryProperty>::new());
                                                    {
                                                        let objects = {
                                                            let source_ml_source1_concat_value763 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value760, _) => source_ml_source1_data_property_value760.as_slice() }), 0f64, at_2);
                                                            {
                                                                let source_ml_source1_concat_value764 = vec![object.clone()];
                                                                {
                                                                    let source_ml_source1_concat_value765 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value762, _) => source_ml_source1_data_property_value762.as_slice() }), at_2, f64::INFINITY);
                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), source_ml_source1_concat_value763), source_ml_source1_concat_value764), source_ml_source1_concat_value765)
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_field767 = self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value766) => {
                                                                    source_ml_source1_data_property_value766
                                                                }
                                                            });
                                                            {
                                                                let source_ml_source1_field768 = true;
                                                                {
                                                                    let source_ml_source1_field769 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source1_field769.clone(), source_ml_source1_field767.clone(), source_ml_source1_field768)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
pub fn register_java_script_ordinary_symbol(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source1_field770 = heap.clone();
                {
                    let source_ml_source1_field771 = false;
                    {
                        let source_ml_source1_field772 = error.clone();
                        (source_ml_source1_field772.clone(), source_ml_source1_field770.clone(), source_ml_source1_field771)
                    }
                }
            }
        } else {
            if ((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value774) => source_ml_source1_data_property_value774.as_slice() }).len() as f64)) >= (16384f64)) {
                {
                    let source_ml_source1_field775 = heap.clone();
                    {
                        let source_ml_source1_field776 = false;
                        {
                            let source_ml_source1_field777 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]);
                            (source_ml_source1_field777.clone(), source_ml_source1_field775.clone(), source_ml_source1_field776)
                        }
                    }
                }
            } else {
                if ((((identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value779, _) => {
                        source_ml_source1_data_property_value779
                    }
                }, identity.clone())) >= (0f64))) || self::test_java_script_ordinary_symbol(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value780) => {
                        source_ml_source1_data_property_value780
                    }
                }, identity.clone())) {
                    {
                        let source_ml_source1_field781 = heap.clone();
                        {
                            let source_ml_source1_field782 = false;
                            {
                                let source_ml_source1_field783 = self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source1_field783.clone(), source_ml_source1_field781.clone(), source_ml_source1_field782)
                            }
                        }
                    }
                } else {
                    {
                        let at = 0f64;
                        {
                            let source_ml_temporary_values60 = match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value784) => {
                                    source_ml_source1_data_property_value784
                                }
                            };
                            {
                                let source_ml_temporary_index60 = self::ml::Big::from_i128(0);
                                {
                                    let at_2 = self::source_ml_temporary_register_java_script_ordinary_symbol_loop61(identity.clone(), at, source_ml_temporary_values60, source_ml_temporary_index60.clone());
                                    {
                                        let symbols = {
                                            let source_ml_source1_concat_value789 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value786) => source_ml_source1_data_property_value786.as_slice() }), 0f64, at_2);
                                            {
                                                let source_ml_source1_concat_value790 = vec![identity.clone()];
                                                {
                                                    let source_ml_source1_concat_value791 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value788) => source_ml_source1_data_property_value788.as_slice() }), at_2, f64::INFINITY);
                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml::Big>::new(), source_ml_source1_concat_value789), source_ml_source1_concat_value790), source_ml_source1_concat_value791)
                                                }
                                            }
                                        };
                                        {
                                            let source_ml_source1_field793 = self::JavaScriptOrdinaryHeap::OrdinaryHeap(match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value792, _) => {
                                                    source_ml_source1_data_property_value792
                                                }
                                            }, symbols);
                                            {
                                                let source_ml_source1_field794 = true;
                                                {
                                                    let source_ml_source1_field795 = self::ml_string::Text::from_units(vec![]);
                                                    (source_ml_source1_field795.clone(), source_ml_source1_field793.clone(), source_ml_source1_field794)
                                                }
                                            }
                                        }
                                    }
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
            let source_ml_source1_field796 = self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]);
            {
                let source_ml_source1_field797 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                {
                    let source_ml_source1_field798 = vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                    (source_ml_source1_field797.clone(), source_ml_source1_field796.clone(), source_ml_source1_field798)
                }
            }
        }
    } else {
        if (index == (1f64)) {
            {
                let source_ml_source1_field799 = self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]);
                {
                    let source_ml_source1_field800 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                    {
                        let source_ml_source1_field801 = vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                        (source_ml_source1_field800.clone(), source_ml_source1_field799.clone(), source_ml_source1_field801)
                    }
                }
            }
        } else {
            if (index == (2f64)) {
                {
                    let source_ml_source1_field802 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                    {
                        let source_ml_source1_field803 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                        {
                            let source_ml_source1_field804 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                            (source_ml_source1_field803.clone(), source_ml_source1_field802.clone(), source_ml_source1_field804)
                        }
                    }
                }
            } else {
                if (index == (3f64)) {
                    {
                        let source_ml_source1_field805 = self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]);
                        {
                            let source_ml_source1_field806 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                            {
                                let source_ml_source1_field807 = vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                                (source_ml_source1_field806.clone(), source_ml_source1_field805.clone(), source_ml_source1_field807)
                            }
                        }
                    }
                } else {
                    if (index == (4f64)) {
                        {
                            let source_ml_source1_field808 = self::ml_string::Text::from_units(vec![103, 101, 116]);
                            {
                                let source_ml_source1_field809 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                {
                                    let source_ml_source1_field810 = vec![103f64, 101f64, 116f64];
                                    (source_ml_source1_field809.clone(), source_ml_source1_field808.clone(), source_ml_source1_field810)
                                }
                            }
                        }
                    } else {
                        if (index == (5f64)) {
                            {
                                let source_ml_source1_field811 = self::ml_string::Text::from_units(vec![115, 101, 116]);
                                {
                                    let source_ml_source1_field812 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                    {
                                        let source_ml_source1_field813 = vec![115f64, 101f64, 116f64];
                                        (source_ml_source1_field812.clone(), source_ml_source1_field811.clone(), source_ml_source1_field813)
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source1_field814 = self::ml_string::Text::from_units(vec![]);
                                {
                                    let source_ml_source1_field815 = self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]);
                                    {
                                        let source_ml_source1_field816 = Vec::<f64>::new();
                                        (source_ml_source1_field815.clone(), source_ml_source1_field814.clone(), source_ml_source1_field816)
                                    }
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
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f817) => {
                true
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f818, source_ml_source1_f819) => {
                true
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source1_field820 = false;
        {
            let source_ml_source1_field821 = false;
            {
                let source_ml_source1_field822 = false;
                {
                    let source_ml_source1_field823 = false;
                    {
                        let source_ml_source1_field824 = false;
                        {
                            let source_ml_source1_field825 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field826 = false;
                                {
                                    let source_ml_source1_field827 = false;
                                    {
                                        let source_ml_source1_field828 = false;
                                        {
                                            let source_ml_source1_field829 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field830 = false;
                                                {
                                                    let source_ml_source1_field831 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field821, source_ml_source1_field820, source_ml_source1_field823, source_ml_source1_field822, source_ml_source1_field829.clone(), source_ml_source1_field828, source_ml_source1_field831.clone(), source_ml_source1_field830, source_ml_source1_field825.clone(), source_ml_source1_field824, source_ml_source1_field827, source_ml_source1_field826)
                                                }
                                            }
                                        }
                                    }
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
                let source_ml_source1_s833 = match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value841, _, _, _) => {
                        let source_ml_source1_data_property_value841 = *source_ml_source1_data_property_value841;
                        source_ml_source1_data_property_value841.clone()
                    }
                };
                match source_ml_source1_s833.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f834) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f835) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f836) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f837) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f838) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f839, source_ml_source1_f840) => {
                        false
                    }
                }
            } {
                self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
            } else {
                {
                    let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value842, _, _, _) => {
                            let source_ml_source1_data_property_value842 = *source_ml_source1_data_property_value842;
                            source_ml_source1_data_property_value842.clone()
                        }
                    });
                    if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                        receiver_error.clone()
                    } else {
                        if ((((!(self::ml_math::is_safe_integer(match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value843, _, _) => {
                                source_ml_source1_data_property_value843
                            }
                        })) || ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value845, _, _) => {
                                source_ml_source1_data_property_value845
                            }
                        }) < (0f64))) || ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value847, _, _) => {
                                source_ml_source1_data_property_value847
                            }
                        }) > (6f64))) || (((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value849, _, _) => {
                                source_ml_source1_data_property_value849
                            }
                        }) == (0f64)) && (((1f64 / match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value851, _, _) => {
                                source_ml_source1_data_property_value851
                            }
                        })) < (0f64)))) || (match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value852) => {
                                source_ml_source1_data_property_value852
                            }
                        } && ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value854, _, _) => {
                                source_ml_source1_data_property_value854
                            }
                        }) == (6f64)))) {
                            self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 102, 105, 101, 108, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                        } else {
                            {
                                let index = match state.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value855, _, _) => {
                                        source_ml_source1_data_property_value855
                                    }
                                };
                                {
                                    let source_ml_source1_s856 = self::source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop62(state.clone(), index);
                                    match source_ml_source1_s856.clone() {
                                        self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Done => {
                                            {
                                                let descriptor = match state.clone() {
                                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value857, _) => {
                                                        source_ml_source1_data_property_value857.clone()
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
                                        self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Return(source_ml_temporary_result62) => {
                                            source_ml_temporary_result62.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
