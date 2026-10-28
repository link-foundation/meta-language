// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptOrdinaryObjectLoop6Result;
use super::MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Result;
use super::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result;
use super::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4;
use super::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result;
use super::MlTemporaryTestJavaScriptOrdinarySymbolLoop7Result;
use super::apply_java_script_property_descriptor_fields;
use super::create_java_script_property_descriptor;
use super::create_java_script_property_transition;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_property_descriptor_kind;
use super::read_java_script_property_descriptor_writes;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_find_java_script_ordinary_object_loop6;
use super::source_ml_temporary_find_java_script_ordinary_property_loop8;
use super::source_ml_temporary_locate_java_script_ordinary_property_loop19;
use super::source_ml_temporary_read_java_script_literal_property_index_loop5;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop12;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop13;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop14;
use super::source_ml_temporary_test_java_script_ordinary_symbol_loop7;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_property_definition(exists: bool, extensible: bool, current: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>) {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(error), current.clone(), Vec::<self::ml_string::Text>::new())
        } else {
            if (exists && !(self::test_complete_java_script_property_descriptor(current.clone()))) {
                self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 67, 117, 114, 114, 101, 110, 116, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114])), current.clone(), Vec::<self::ml_string::Text>::new())
            } else {
                {
                    let kind = self::read_java_script_property_descriptor_kind(proposed.clone());
                    if !exists {
                        if !extensible {
                            self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new())
                        } else {
                            {
                                let base = self::create_java_script_property_descriptor(&(kind), false, false);
                                {
                                    let descriptor = self::apply_java_script_property_descriptor_fields(base.clone(), proposed.clone());
                                    self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![99, 114, 101, 97, 116, 101])), &(self::ml_string::Text::from_units(vec![])), descriptor.clone(), self::read_java_script_property_descriptor_writes(descriptor.clone()))
                                }
                            }
                        }
                    } else {
                        if (((((!((proposed.clone()).1) && !((proposed.clone()).3)) && !((proposed.clone()).9)) && !((proposed.clone()).11)) && !((proposed.clone()).5)) && !((proposed.clone()).7)) {
                            self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![107, 101, 101, 112])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new())
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(current.clone());
                                {
                                    let source_ml_source1_s172 = if !((current.clone()).0) {
                                        if ((proposed.clone()).1 && (proposed.clone()).0) {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else {
                                            if ((proposed.clone()).3 && (((proposed.clone()).2) != ((current.clone()).2))) {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if ((proposed.clone()).5 && !(self::test_java_script_semantic_same_value((proposed.clone()).4, (current.clone()).4))) {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                        } else {
                                                            if ((proposed.clone()).7 && !(self::test_java_script_semantic_same_value((proposed.clone()).6, (current.clone()).6))) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Next
                                                            }
                                                        }
                                                    } else {
                                                        if !((current.clone()).10) {
                                                            if ((proposed.clone()).11 && (proposed.clone()).10) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                if (proposed.clone()).9 {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value((proposed.clone()).8, (current.clone()).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                                } else {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Next
                                    };
                                    match source_ml_source1_s172.clone() {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Next => {
                                            {
                                                let base_2 = if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::create_java_script_property_descriptor(&(kind), (current.clone()).0, (current.clone()).2)
                                                } else {
                                                    current.clone()
                                                };
                                                {
                                                    let descriptor_2 = self::apply_java_script_property_descriptor_fields(base_2.clone(), proposed.clone());
                                                    {
                                                        let conversion = ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original));
                                                        self::create_java_script_property_transition(&(if conversion {
                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![117, 112, 100, 97, 116, 101])
                                                        }), &(self::ml_string::Text::from_units(vec![])), descriptor_2.clone(), self::read_java_script_property_descriptor_writes(if conversion {
                                                            descriptor_2.clone()
                                                        } else {
                                                            proposed.clone()
                                                        }))
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4::MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Return(source_ml_temporary_result4) => {
                                            source_ml_temporary_result4.clone()
                                        }
                                    }
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
pub fn read_java_script_literal_property_index(key: &[f64]) -> f64 {
    if ((((key.len() as f64)) == (0f64)) || (((key.len() as f64)) > (10f64))) {
        (-1f64)
    } else {
        if ((((key.len() as f64)) > (1f64)) && {
            let source_ml_source1_comparison_operand173 = self::ml_array::optional_at(key, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand174 = 48f64;
                match (source_ml_source1_comparison_operand173.clone(), source_ml_source1_comparison_operand174) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) {
            (-1f64)
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s175 = self::source_ml_temporary_read_java_script_literal_property_index_loop5(key, value, at);
                        match source_ml_source1_s175.clone() {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Done(value_2) => {
                                if (value_2 <= (4294967294f64)) {
                                    value_2
                                } else {
                                    (-1f64)
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Return(source_ml_temporary_result5) => {
                                source_ml_temporary_result5
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
pub fn read_java_script_literal_property_form(kind: &self::ml_string::Text, index: f64) -> self::ml_string::Text {
    if (((((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) || ((*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) || ((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && (index >= (0f64))) {
        self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
    } else {
        self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_index_number_form(kind: &self::ml_string::Text, element: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else {
        if (((*kind) == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || ((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
        } else {
            if (((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) || (((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && ((*element) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))) {
                self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
            } else {
                if (((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && ((((*element) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || ((*element) == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || ((*element) == (self::ml_string::Text::from_units(vec![110, 97, 116]))))) {
                    self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
                } else {
                    self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_object(objects: Vec<self::JavaScriptOrdinaryObject>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = (objects.len() as f64);
            {
                let source_ml_source1_s176 = self::source_ml_temporary_find_java_script_ordinary_object_loop6(objects, identity.clone(), low, high);
                match source_ml_source1_s176.clone() {
                    self::MlTemporaryFindJavaScriptOrdinaryObjectLoop6Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop6Done => {
                        (-1f64)
                    }
                    self::MlTemporaryFindJavaScriptOrdinaryObjectLoop6Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop6Return(source_ml_temporary_result6) => {
                        source_ml_temporary_result6
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_ordinary_symbol(symbols: Vec<self::ml::Big>, identity: self::ml::Big) -> bool {
    {
        let low = 0f64;
        {
            let high = (symbols.len() as f64);
            {
                let source_ml_source1_s177 = self::source_ml_temporary_test_java_script_ordinary_symbol_loop7(symbols, identity.clone(), low, high);
                match source_ml_source1_s177.clone() {
                    self::MlTemporaryTestJavaScriptOrdinarySymbolLoop7Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop7Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptOrdinarySymbolLoop7Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop7Return(source_ml_temporary_result7) => {
                        source_ml_temporary_result7
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_value_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    {
        let error = self::read_java_script_semantic_value_error(value.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptSemanticValue::Null => {
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f178) => {
                    {
                        let source_ml_source1_f194 = source_ml_source1_f178;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f179) => {
                    {
                        let source_ml_source1_f200 = source_ml_source1_f179;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f180) => {
                    {
                        let source_ml_source1_f206 = source_ml_source1_f180.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f181) => {
                    {
                        let source_ml_source1_f212 = source_ml_source1_f181;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    if self::test_java_script_ordinary_symbol(match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value214) => {
                            source_ml_source1_data_property_value214
                        }
                    }, identity.clone()) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f182, source_ml_source1_f183) => {
                    {
                        let identity_2 = source_ml_source1_f182.clone();
                        {
                            let callable = source_ml_source1_f183;
                            {
                                let at = self::find_java_script_ordinary_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value220, _) => {
                                        source_ml_source1_data_property_value220
                                    }
                                }, identity_2.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value221, _) => source_ml_source1_data_property_value221.as_slice() }), self::ml_array::number_index(at));
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (({
                                            let source_ml_source1_nullish_field_receiver225 = (object.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver225.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present226 = ((source_ml_source1_nullish_field_receiver225.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present226.clone() {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value227, _) => {
                                                            source_ml_source1_data_property_value227
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }) == callable)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                        }
                                    }
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
pub fn read_java_script_ordinary_key_error(heap: self::JavaScriptOrdinaryHeap, key: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    if (match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f228) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f229) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f230) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f231) => {
            false
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f232) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f233, source_ml_source1_f234) => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f235) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f236) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f237) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f238) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f239) => {
            false
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f240, source_ml_source1_f241) => {
            true
        }
    }) {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 101, 120, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 104, 111, 117, 116, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 99, 111, 101, 114, 99, 105, 111, 110])
    } else {
        self::read_java_script_ordinary_value_error(heap.clone(), key.clone())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_property(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source1_s242 = self::source_ml_temporary_find_java_script_ordinary_property_loop8(properties, key.clone(), at);
            match source_ml_source1_s242.clone() {
                self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Done => {
                    (-1f64)
                }
                self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Return(source_ml_temporary_result8) => {
                    source_ml_temporary_result8
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_heap_error(heap: self::JavaScriptOrdinaryHeap) -> self::ml_string::Text {
    if (((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value244, _) => source_ml_source1_data_property_value244.as_slice() }).len() as f64)) > (4096f64)) || ((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value246) => source_ml_source1_data_property_value246.as_slice() }).len() as f64)) > (16384f64))) {
        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
    } else {
        {
            let previous = self::ml::Big::from_i128(0);
            {
                let source_ml_temporary_values9 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value247, _) => {
                        source_ml_source1_data_property_value247
                    }
                };
                {
                    let source_ml_temporary_index9 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s248 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop12(previous.clone(), source_ml_temporary_values9, source_ml_temporary_index9.clone());
                        match source_ml_source1_s248.clone() {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done(previous_2) => {
                                {
                                    let previous_3 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_temporary_values10 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value249) => {
                                                source_ml_source1_data_property_value249
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index10 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_s250 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop13(heap.clone(), previous_3.clone(), source_ml_temporary_values10, source_ml_temporary_index10.clone());
                                                match source_ml_source1_s250.clone() {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done => {
                                                        {
                                                            let property_count = 0f64;
                                                            {
                                                                let source_ml_temporary_values11 = match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value251, _) => {
                                                                        source_ml_source1_data_property_value251
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_temporary_index11 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source1_s252 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop14(heap.clone(), property_count, source_ml_temporary_values11, source_ml_temporary_index11.clone());
                                                                        match source_ml_source1_s252.clone() {
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done => {
                                                                                self::ml_string::Text::from_units(vec![])
                                                                            }
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_temporary_result14) => {
                                                                                source_ml_temporary_result14.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_temporary_result18) => {
                                                        source_ml_temporary_result18.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(source_ml_temporary_result18_2) => {
                                source_ml_temporary_result18_2.clone()
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
pub fn replace_java_script_ordinary_object(heap: self::JavaScriptOrdinaryHeap, replacement: self::JavaScriptOrdinaryObject) -> self::JavaScriptOrdinaryHeap {
    {
        let objects = { let ml_values1 = match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value253, _) => {
                source_ml_source1_data_property_value253
            }
        }; let ml_callback1 = { let replacement = replacement.clone(); std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptOrdinaryObject { if ((match object.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value256, _, _, _, _) => {
                source_ml_source1_data_property_value256.clone()
            }
        }) == (match replacement.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value257, _, _, _, _) => {
                source_ml_source1_data_property_value257.clone()
            }
        })) {
            replacement.clone()
        } else {
            object.clone()
        } }) }; ml_values1.iter().cloned().enumerate().map(|(ml_index1, ml_value1)| ml_callback1(ml_value1)).collect::<Vec<_>>() };
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value258) => {
                source_ml_source1_data_property_value258
            }
        })
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn locate_java_script_ordinary_property(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool, self::ml::Big) {
    {
        let current = identity.clone();
        {
            let depth = 0f64;
            {
                let source_ml_source1_s259 = self::source_ml_temporary_locate_java_script_ordinary_property_loop19(heap.clone(), key.clone(), current.clone(), depth);
                match source_ml_source1_s259.clone() {
                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Done => {
                        {
                            let source_ml_source1_field260 = false;
                            {
                                let source_ml_source1_field261 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_field262 = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false);
                                    (source_ml_source1_field262.clone(), source_ml_source1_field260, source_ml_source1_field261.clone())
                                }
                            }
                        }
                    }
                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Return(source_ml_temporary_result19) => {
                        source_ml_temporary_result19.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_get(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> (self::ml_string::Text, Vec<self::JavaScriptSemanticValue>, self::JavaScriptSemanticValue, self::ml_string::Text, self::JavaScriptSemanticValue, self::JavaScriptSemanticValue) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                {
                    let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                        heap_error.clone()
                    } else {
                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                            key_error.clone()
                        } else {
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                receiver_error.clone()
                            } else {
                                if ((self::find_java_script_ordinary_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value264, _) => {
                                        source_ml_source1_data_property_value264
                                    }
                                }, identity.clone())) < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                } else {
                                    self::ml_string::Text::from_units(vec![])
                                }
                            }
                        }
                    };
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        {
                            let source_ml_source1_field265 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                            {
                                let source_ml_source1_field266 = error.clone();
                                {
                                    let source_ml_source1_field267 = self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source1_field268 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source1_field269 = receiver.clone();
                                            {
                                                let source_ml_source1_field270 = Vec::<self::JavaScriptSemanticValue>::new();
                                                (source_ml_source1_field265.clone(), source_ml_source1_field270, source_ml_source1_field268.clone(), source_ml_source1_field266.clone(), source_ml_source1_field269.clone(), source_ml_source1_field267.clone())
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                            if !((property.clone()).1) {
                                {
                                    let source_ml_source1_field271 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                    {
                                        let source_ml_source1_field272 = self::ml_string::Text::from_units(vec![]);
                                        {
                                            let source_ml_source1_field273 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field274 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source1_field275 = receiver.clone();
                                                    {
                                                        let source_ml_source1_field276 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source1_field271.clone(), source_ml_source1_field276, source_ml_source1_field274.clone(), source_ml_source1_field272.clone(), source_ml_source1_field275.clone(), source_ml_source1_field273.clone())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if ((property.clone()).0).9 {
                                    {
                                        let source_ml_source1_field277 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                        {
                                            let source_ml_source1_field278 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source1_field279 = ((property.clone()).0).8;
                                                {
                                                    let source_ml_source1_field280 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source1_field281 = receiver.clone();
                                                        {
                                                            let source_ml_source1_field282 = Vec::<self::JavaScriptSemanticValue>::new();
                                                            (source_ml_source1_field277.clone(), source_ml_source1_field282, source_ml_source1_field280.clone(), source_ml_source1_field278.clone(), source_ml_source1_field281.clone(), source_ml_source1_field279.clone())
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let getter = ((property.clone()).0).4;
                                        match getter.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                {
                                                    let source_ml_source1_field290 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                                    {
                                                        let source_ml_source1_field291 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field292 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field293 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_field294 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field295 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field290.clone(), source_ml_source1_field295, source_ml_source1_field293.clone(), source_ml_source1_field291.clone(), source_ml_source1_field294.clone(), source_ml_source1_field292.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                {
                                                    let source_ml_source1_field296 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field297 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field298 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field299 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field300 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field301 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field296.clone(), source_ml_source1_field301, source_ml_source1_field299.clone(), source_ml_source1_field297.clone(), source_ml_source1_field300.clone(), source_ml_source1_field298.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f283) => {
                                                {
                                                    let source_ml_source1_field302 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field303 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field304 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field305 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field306 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field307 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field302.clone(), source_ml_source1_field307, source_ml_source1_field305.clone(), source_ml_source1_field303.clone(), source_ml_source1_field306.clone(), source_ml_source1_field304.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source1_f284) => {
                                                {
                                                    let source_ml_source1_field308 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field309 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field310 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field311 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field312 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field313 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field308.clone(), source_ml_source1_field313, source_ml_source1_field311.clone(), source_ml_source1_field309.clone(), source_ml_source1_field312.clone(), source_ml_source1_field310.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f285) => {
                                                {
                                                    let source_ml_source1_field314 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field315 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field316 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field317 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field318 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field319 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field314.clone(), source_ml_source1_field319, source_ml_source1_field317.clone(), source_ml_source1_field315.clone(), source_ml_source1_field318.clone(), source_ml_source1_field316.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source1_f286) => {
                                                {
                                                    let source_ml_source1_field320 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field321 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field322 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field323 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field324 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field325 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field320.clone(), source_ml_source1_field325, source_ml_source1_field323.clone(), source_ml_source1_field321.clone(), source_ml_source1_field324.clone(), source_ml_source1_field322.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f287) => {
                                                {
                                                    let source_ml_source1_field326 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field327 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field328 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field329 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field330 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field331 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field326.clone(), source_ml_source1_field331, source_ml_source1_field329.clone(), source_ml_source1_field327.clone(), source_ml_source1_field330.clone(), source_ml_source1_field328.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Object(source_ml_source1_f288, source_ml_source1_f289) => {
                                                {
                                                    let source_ml_source1_field332 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field333 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field334 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field335 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field336 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field337 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field332.clone(), source_ml_source1_field337, source_ml_source1_field335.clone(), source_ml_source1_field333.clone(), source_ml_source1_field336.clone(), source_ml_source1_field334.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
