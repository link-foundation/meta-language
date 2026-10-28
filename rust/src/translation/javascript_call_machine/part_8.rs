// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptOrdinaryObjectLoop38Result;
use super::MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Result;
use super::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result;
use super::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36;
use super::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Result;
use super::MlTemporaryTestJavaScriptOrdinarySymbolLoop39Result;
use super::apply_java_script_property_descriptor_fields;
use super::create_java_script_property_descriptor;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_property_descriptor_kind;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_find_java_script_ordinary_object_loop38;
use super::source_ml_temporary_find_java_script_ordinary_property_loop40;
use super::source_ml_temporary_locate_java_script_ordinary_property_loop51;
use super::source_ml_temporary_read_java_script_literal_property_index_loop37;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop44;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop45;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop46;
use super::source_ml_temporary_test_java_script_ordinary_symbol_loop39;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_writes(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> Vec<self::ml_string::Text> {
    {
        let writes = Vec::<self::ml_string::Text>::new();
        {
            let writes_3 = if (descriptor.clone()).1 {
                {
                    let writes_2 = {
                        let source_ml_source1_concat_value211 = writes;
                        {
                            let source_ml_source1_concat_value212 = vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value211), source_ml_source1_concat_value212)
                        }
                    };
                    writes_2
                }
            } else {
                writes
            };
            {
                let writes_5 = if (descriptor.clone()).3 {
                    {
                        let writes_4 = {
                            let source_ml_source1_concat_value213 = writes_3;
                            {
                                let source_ml_source1_concat_value214 = vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])];
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value213), source_ml_source1_concat_value214)
                            }
                        };
                        writes_4
                    }
                } else {
                    writes_3
                };
                {
                    let writes_7 = if (descriptor.clone()).9 {
                        {
                            let writes_6 = {
                                let source_ml_source1_concat_value215 = writes_5;
                                {
                                    let source_ml_source1_concat_value216 = vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])];
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value215), source_ml_source1_concat_value216)
                                }
                            };
                            writes_6
                        }
                    } else {
                        writes_5
                    };
                    {
                        let writes_9 = if (descriptor.clone()).11 {
                            {
                                let writes_8 = {
                                    let source_ml_source1_concat_value217 = writes_7;
                                    {
                                        let source_ml_source1_concat_value218 = vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value217), source_ml_source1_concat_value218)
                                    }
                                };
                                writes_8
                            }
                        } else {
                            writes_7
                        };
                        {
                            let writes_11 = if (descriptor.clone()).5 {
                                {
                                    let writes_10 = {
                                        let source_ml_source1_concat_value219 = writes_9;
                                        {
                                            let source_ml_source1_concat_value220 = vec![self::ml_string::Text::from_units(vec![103, 101, 116])];
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value219), source_ml_source1_concat_value220)
                                        }
                                    };
                                    writes_10
                                }
                            } else {
                                writes_9
                            };
                            {
                                let writes_13 = if (descriptor.clone()).7 {
                                    {
                                        let writes_12 = {
                                            let source_ml_source1_concat_value221 = writes_11;
                                            {
                                                let source_ml_source1_concat_value222 = vec![self::ml_string::Text::from_units(vec![115, 101, 116])];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value221), source_ml_source1_concat_value222)
                                            }
                                        };
                                        writes_12
                                    }
                                } else {
                                    writes_11
                                };
                                writes_13
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
pub fn create_java_script_property_transition(disposition: &self::ml_string::Text, error: &self::ml_string::Text, descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), writes: Vec<self::ml_string::Text>) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>) {
    {
        let source_ml_source1_field223 = disposition.clone();
        {
            let source_ml_source1_field224 = error.clone();
            {
                let source_ml_source1_field225 = descriptor.clone();
                {
                    let source_ml_source1_field226 = writes;
                    (source_ml_source1_field225.clone(), source_ml_source1_field223.clone(), source_ml_source1_field224.clone(), source_ml_source1_field226)
                }
            }
        }
    }
}

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
                                    let source_ml_source1_s227 = if !((current.clone()).0) {
                                        if ((proposed.clone()).1 && (proposed.clone()).0) {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else {
                                            if ((proposed.clone()).3 && (((proposed.clone()).2) != ((current.clone()).2))) {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if ((proposed.clone()).5 && !(self::test_java_script_semantic_same_value((proposed.clone()).4, (current.clone()).4))) {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                        } else {
                                                            if ((proposed.clone()).7 && !(self::test_java_script_semantic_same_value((proposed.clone()).6, (current.clone()).6))) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Next
                                                            }
                                                        }
                                                    } else {
                                                        if !((current.clone()).10) {
                                                            if ((proposed.clone()).11 && (proposed.clone()).10) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                if (proposed.clone()).9 {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value((proposed.clone()).8, (current.clone()).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                                } else {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Next
                                    };
                                    match source_ml_source1_s227.clone() {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Next => {
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
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36::MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Return(source_ml_temporary_result36) => {
                                            source_ml_temporary_result36.clone()
                                        }
                                    }
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
            let source_ml_source1_comparison_operand228 = self::ml_array::optional_at(key, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand229 = 48f64;
                match (source_ml_source1_comparison_operand228.clone(), source_ml_source1_comparison_operand229) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) {
            (-1f64)
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s230 = self::source_ml_temporary_read_java_script_literal_property_index_loop37(key, value, at);
                        match source_ml_source1_s230.clone() {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Done(value_2) => {
                                if (value_2 <= (4294967294f64)) {
                                    value_2
                                } else {
                                    (-1f64)
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Return(source_ml_temporary_result37) => {
                                source_ml_temporary_result37
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
                let source_ml_source1_s231 = self::source_ml_temporary_find_java_script_ordinary_object_loop38(objects, identity.clone(), low, high);
                match source_ml_source1_s231.clone() {
                    self::MlTemporaryFindJavaScriptOrdinaryObjectLoop38Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop38Done => {
                        (-1f64)
                    }
                    self::MlTemporaryFindJavaScriptOrdinaryObjectLoop38Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop38Return(source_ml_temporary_result38) => {
                        source_ml_temporary_result38
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
                let source_ml_source1_s232 = self::source_ml_temporary_test_java_script_ordinary_symbol_loop39(symbols, identity.clone(), low, high);
                match source_ml_source1_s232.clone() {
                    self::MlTemporaryTestJavaScriptOrdinarySymbolLoop39Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop39Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptOrdinarySymbolLoop39Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop39Return(source_ml_temporary_result39) => {
                        source_ml_temporary_result39
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f233) => {
                    {
                        let source_ml_source1_f249 = source_ml_source1_f233;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f234) => {
                    {
                        let source_ml_source1_f255 = source_ml_source1_f234;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f235) => {
                    {
                        let source_ml_source1_f261 = source_ml_source1_f235.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f236) => {
                    {
                        let source_ml_source1_f267 = source_ml_source1_f236;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    if self::test_java_script_ordinary_symbol(match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value269) => {
                            source_ml_source1_data_property_value269
                        }
                    }, identity.clone()) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f237, source_ml_source1_f238) => {
                    {
                        let identity_2 = source_ml_source1_f237.clone();
                        {
                            let callable = source_ml_source1_f238;
                            {
                                let at = self::find_java_script_ordinary_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value275, _) => {
                                        source_ml_source1_data_property_value275
                                    }
                                }, identity_2.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value276, _) => source_ml_source1_data_property_value276.as_slice() }), self::ml_array::number_index(at));
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (({
                                            let source_ml_source1_nullish_field_receiver280 = (object.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver280.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present281 = ((source_ml_source1_nullish_field_receiver280.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present281.clone() {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value282, _) => {
                                                            source_ml_source1_data_property_value282
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f283) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f284) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f285) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f286) => {
            false
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f287) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f288, source_ml_source1_f289) => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f290) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f291) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f292) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f293) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f294) => {
            false
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f295, source_ml_source1_f296) => {
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
            let source_ml_source1_s297 = self::source_ml_temporary_find_java_script_ordinary_property_loop40(properties, key.clone(), at);
            match source_ml_source1_s297.clone() {
                self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Done => {
                    (-1f64)
                }
                self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Return(source_ml_temporary_result40) => {
                    source_ml_temporary_result40
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_heap_error(heap: self::JavaScriptOrdinaryHeap) -> self::ml_string::Text {
    if (((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value299, _) => source_ml_source1_data_property_value299.as_slice() }).len() as f64)) > (4096f64)) || ((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value301) => source_ml_source1_data_property_value301.as_slice() }).len() as f64)) > (16384f64))) {
        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
    } else {
        {
            let previous = self::ml::Big::from_i128(0);
            {
                let source_ml_temporary_values41 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value302, _) => {
                        source_ml_source1_data_property_value302
                    }
                };
                {
                    let source_ml_temporary_index41 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s303 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop44(previous.clone(), source_ml_temporary_values41, source_ml_temporary_index41.clone());
                        match source_ml_source1_s303.clone() {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Done(previous_2) => {
                                {
                                    let previous_3 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_temporary_values42 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value304) => {
                                                source_ml_source1_data_property_value304
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index42 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_s305 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop45(heap.clone(), previous_3.clone(), source_ml_temporary_values42, source_ml_temporary_index42.clone());
                                                match source_ml_source1_s305.clone() {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Done => {
                                                        {
                                                            let property_count = 0f64;
                                                            {
                                                                let source_ml_temporary_values43 = match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value306, _) => {
                                                                        source_ml_source1_data_property_value306
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_temporary_index43 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source1_s307 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop46(heap.clone(), property_count, source_ml_temporary_values43, source_ml_temporary_index43.clone());
                                                                        match source_ml_source1_s307.clone() {
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Done => {
                                                                                self::ml_string::Text::from_units(vec![])
                                                                            }
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Return(source_ml_temporary_result46) => {
                                                                                source_ml_temporary_result46.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Return(source_ml_temporary_result50) => {
                                                        source_ml_temporary_result50.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Return(source_ml_temporary_result50_2) => {
                                source_ml_temporary_result50_2.clone()
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
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value308, _) => {
                source_ml_source1_data_property_value308
            }
        }; let ml_callback1 = { let replacement = replacement.clone(); std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptOrdinaryObject { if ((match object.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value311, _, _, _, _) => {
                source_ml_source1_data_property_value311.clone()
            }
        }) == (match replacement.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value312, _, _, _, _) => {
                source_ml_source1_data_property_value312.clone()
            }
        })) {
            replacement.clone()
        } else {
            object.clone()
        } }) }; ml_values1.iter().cloned().enumerate().map(|(ml_index1, ml_value1)| ml_callback1(ml_value1)).collect::<Vec<_>>() };
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value313) => {
                source_ml_source1_data_property_value313
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
                let source_ml_source1_s314 = self::source_ml_temporary_locate_java_script_ordinary_property_loop51(heap.clone(), key.clone(), current.clone(), depth);
                match source_ml_source1_s314.clone() {
                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Done => {
                        {
                            let source_ml_source1_field315 = false;
                            {
                                let source_ml_source1_field316 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_field317 = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false);
                                    (source_ml_source1_field317.clone(), source_ml_source1_field315, source_ml_source1_field316.clone())
                                }
                            }
                        }
                    }
                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Return(source_ml_temporary_result51) => {
                        source_ml_temporary_result51.clone()
                    }
                }
            }
        }
    }
}
