// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Result;
use super::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop100Result;
use super::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop111Result;
use super::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result;
use super::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Result;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_value_error;
use super::source_ml_source1_temporary_find_java_script_ordinary_object_loop98;
use super::source_ml_source1_temporary_find_java_script_ordinary_property_loop100;
use super::source_ml_source1_temporary_locate_java_script_ordinary_property_loop111;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop104;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop105;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop106;
use super::source_ml_source1_temporary_test_java_script_ordinary_symbol_loop99;
use super::test_java_script_semantic_accessor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_strict_equal(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_java_script_semantic_value_error(right.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 111, 112, 101, 114, 97, 110, 100, 115])).0, message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Null => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Number(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Number(value) => {
                        (left_value == value)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_kind(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> self::ml_string::Text {
    {
        let data = ((descriptor.clone()).9 || (descriptor.clone()).11);
        {
            let accessor = ((descriptor.clone()).5 || (descriptor.clone()).7);
            if (data && accessor) {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else {
                if data {
                    self::ml_string::Text::from_units(vec![100, 97, 116, 97])
                } else {
                    if accessor {
                        self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
                    } else {
                        self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_error(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> self::ml_string::Text {
    if ((self::read_java_script_property_descriptor_kind(descriptor.clone())) == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))) {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        if ((descriptor.clone()).9 && ((self::read_java_script_semantic_value_error((descriptor.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
            self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108, 117, 101])
        } else {
            if ((descriptor.clone()).5 && (((self::read_java_script_semantic_value_error((descriptor.clone()).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).4)))) {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                if ((descriptor.clone()).7 && (((self::read_java_script_semantic_value_error((descriptor.clone()).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).6)))) {
                    self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_complete_java_script_property_descriptor(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> bool {
    if ((!((descriptor.clone()).1) || !((descriptor.clone()).3)) || ((self::read_java_script_property_descriptor_error(descriptor.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if (kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97]))) {
                ((descriptor.clone()).9 && (descriptor.clone()).11)
            } else {
                (((kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) && (descriptor.clone()).5) && (descriptor.clone()).7)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_property_descriptor(kind: &self::ml_string::Text, configurable: bool, enumerable: bool) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field279 = true;
        {
            let source_ml_source2_field280 = configurable;
            {
                let source_ml_source2_field281 = true;
                {
                    let source_ml_source2_field282 = enumerable;
                    {
                        let source_ml_source2_field283 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                        {
                            let source_ml_source2_field284 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_field285 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                {
                                    let source_ml_source2_field286 = false;
                                    {
                                        let source_ml_source2_field287 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                        {
                                            let source_ml_source2_field288 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field289 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                                {
                                                    let source_ml_source2_field290 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field280, source_ml_source2_field279, source_ml_source2_field282, source_ml_source2_field281, source_ml_source2_field288.clone(), source_ml_source2_field287, source_ml_source2_field290.clone(), source_ml_source2_field289, source_ml_source2_field284.clone(), source_ml_source2_field283, source_ml_source2_field286, source_ml_source2_field285)
                                                }
                                            }
                                        }
                                    }
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
pub fn apply_java_script_property_descriptor_fields(base: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field291 = true;
        {
            let source_ml_source2_field292 = if (proposed.clone()).1 {
                (proposed.clone()).0
            } else {
                (base.clone()).0
            };
            {
                let source_ml_source2_field293 = true;
                {
                    let source_ml_source2_field294 = if (proposed.clone()).3 {
                        (proposed.clone()).2
                    } else {
                        (base.clone()).2
                    };
                    {
                        let source_ml_source2_field295 = (base.clone()).9;
                        {
                            let source_ml_source2_field296 = if ((base.clone()).9 && (proposed.clone()).9) {
                                (proposed.clone()).8
                            } else {
                                (base.clone()).8
                            };
                            {
                                let source_ml_source2_field297 = (base.clone()).11;
                                {
                                    let source_ml_source2_field298 = if ((base.clone()).11 && (proposed.clone()).11) {
                                        (proposed.clone()).10
                                    } else {
                                        (base.clone()).10
                                    };
                                    {
                                        let source_ml_source2_field299 = (base.clone()).5;
                                        {
                                            let source_ml_source2_field300 = if ((base.clone()).5 && (proposed.clone()).5) {
                                                (proposed.clone()).4
                                            } else {
                                                (base.clone()).4
                                            };
                                            {
                                                let source_ml_source2_field301 = (base.clone()).7;
                                                {
                                                    let source_ml_source2_field302 = if ((base.clone()).7 && (proposed.clone()).7) {
                                                        (proposed.clone()).6
                                                    } else {
                                                        (base.clone()).6
                                                    };
                                                    (source_ml_source2_field292, source_ml_source2_field291, source_ml_source2_field294, source_ml_source2_field293, source_ml_source2_field300.clone(), source_ml_source2_field299, source_ml_source2_field302.clone(), source_ml_source2_field301, source_ml_source2_field296.clone(), source_ml_source2_field295, source_ml_source2_field298, source_ml_source2_field297)
                                                }
                                            }
                                        }
                                    }
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
pub fn read_java_script_property_descriptor_writes(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> Vec<self::ml_string::Text> {
    {
        let writes = Vec::<self::ml_string::Text>::new();
        {
            let writes_3 = if (descriptor.clone()).1 {
                {
                    let writes_2 = {
                        let source_ml_source2_concat_value303 = writes;
                        {
                            let source_ml_source2_concat_value304 = vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value303), source_ml_source2_concat_value304)
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
                            let source_ml_source2_concat_value305 = writes_3;
                            {
                                let source_ml_source2_concat_value306 = vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])];
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value305), source_ml_source2_concat_value306)
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
                                let source_ml_source2_concat_value307 = writes_5;
                                {
                                    let source_ml_source2_concat_value308 = vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])];
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value307), source_ml_source2_concat_value308)
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
                                    let source_ml_source2_concat_value309 = writes_7;
                                    {
                                        let source_ml_source2_concat_value310 = vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value309), source_ml_source2_concat_value310)
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
                                        let source_ml_source2_concat_value311 = writes_9;
                                        {
                                            let source_ml_source2_concat_value312 = vec![self::ml_string::Text::from_units(vec![103, 101, 116])];
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value311), source_ml_source2_concat_value312)
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
                                            let source_ml_source2_concat_value313 = writes_11;
                                            {
                                                let source_ml_source2_concat_value314 = vec![self::ml_string::Text::from_units(vec![115, 101, 116])];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value313), source_ml_source2_concat_value314)
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
        let source_ml_source2_field315 = disposition.clone();
        {
            let source_ml_source2_field316 = error.clone();
            {
                let source_ml_source2_field317 = descriptor.clone();
                {
                    let source_ml_source2_field318 = writes;
                    (source_ml_source2_field317.clone(), source_ml_source2_field315.clone(), source_ml_source2_field316.clone(), source_ml_source2_field318)
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
                                    let source_ml_source2_s319 = if !((current.clone()).0) {
                                        if ((proposed.clone()).1 && (proposed.clone()).0) {
                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else {
                                            if ((proposed.clone()).3 && (((proposed.clone()).2) != ((current.clone()).2))) {
                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if ((proposed.clone()).5 && !(self::test_java_script_semantic_same_value((proposed.clone()).4, (current.clone()).4))) {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                        } else {
                                                            if ((proposed.clone()).7 && !(self::test_java_script_semantic_same_value((proposed.clone()).6, (current.clone()).6))) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Next
                                                            }
                                                        }
                                                    } else {
                                                        if !((current.clone()).10) {
                                                            if ((proposed.clone()).11 && (proposed.clone()).10) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                if (proposed.clone()).9 {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value((proposed.clone()).8, (current.clone()).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                                } else {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Next
                                    };
                                    match source_ml_source2_s319.clone() {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Next => {
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
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Return(source_ml_source1_temporary_result96) => {
                                            source_ml_source1_temporary_result96.clone()
                                        }
                                    }
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
pub fn find_java_script_ordinary_object(objects: Vec<self::JavaScriptOrdinaryObject>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = (objects.len() as f64);
            {
                let source_ml_source2_s323 = self::source_ml_source1_temporary_find_java_script_ordinary_object_loop98(objects, identity.clone(), low, high);
                match source_ml_source2_s323.clone() {
                    self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Return(source_ml_source1_temporary_result98) => {
                        source_ml_source1_temporary_result98
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
                let source_ml_source2_s324 = self::source_ml_source1_temporary_test_java_script_ordinary_symbol_loop99(symbols, identity.clone(), low, high);
                match source_ml_source2_s324.clone() {
                    self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Done => {
                        false
                    }
                    self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Return(source_ml_source1_temporary_result99) => {
                        source_ml_source1_temporary_result99
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_value_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2) => {
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
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::JavaScriptSemanticValue::Number(_) => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::JavaScriptSemanticValue::Symbol(identity) => {
                            if self::test_java_script_ordinary_symbol(shared_constructor_field2, identity.clone()) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f329, source_ml_source2_f330) => {
                            {
                                let identity_2 = source_ml_source2_f329.clone();
                                {
                                    let callable = source_ml_source2_f330;
                                    {
                                        let at = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity_2.clone());
                                        if (at < (0f64)) {
                                            self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                        } else {
                                            {
                                                let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(at));
                                                if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (({
                                                    let source_ml_source2_nullish_field_receiver372 = (object.clone(),);
                                                    if match (source_ml_source2_nullish_field_receiver372.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        {
                                                            let source_ml_source2_nullish_field_present373 = ((source_ml_source2_nullish_field_receiver372.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_source2_nullish_field_present373.clone() {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value374, _) => {
                                                                    source_ml_source2_data_property_value374
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_key_error(heap: self::JavaScriptOrdinaryHeap, key: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    if (match key.clone() {
        self::JavaScriptSemanticValue::Text(_) => {
            false
        }
        _ => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Symbol(_) => {
            false
        }
        _ => {
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
            let source_ml_source2_s389 = self::source_ml_source1_temporary_find_java_script_ordinary_property_loop100(properties, key.clone(), at);
            match source_ml_source2_s389.clone() {
                self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop100Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop100Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop100Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop100Return(source_ml_source1_temporary_result100) => {
                    source_ml_source1_temporary_result100
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_heap_error(heap: self::JavaScriptOrdinaryHeap) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2) => {
            if ((((shared_constructor_field1.len() as f64)) > (4096f64)) || (((shared_constructor_field2.len() as f64)) > (16384f64))) {
                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let previous = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_temporary_values101 = shared_constructor_field1.clone();
                        {
                            let source_ml_source1_temporary_index101 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s395 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop104(previous.clone(), source_ml_source1_temporary_values101, source_ml_source1_temporary_index101.clone());
                                match source_ml_source2_s395.clone() {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Done(previous_2) => {
                                        {
                                            let previous_3 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_temporary_values102 = shared_constructor_field2;
                                                {
                                                    let source_ml_source1_temporary_index102 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s397 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop105(heap.clone(), previous_3.clone(), source_ml_source1_temporary_values102, source_ml_source1_temporary_index102.clone());
                                                        match source_ml_source2_s397.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Done => {
                                                                {
                                                                    let property_count = 0f64;
                                                                    {
                                                                        let source_ml_source1_temporary_values103 = shared_constructor_field1;
                                                                        {
                                                                            let source_ml_source1_temporary_index103 = self::ml::Big::from_i128(0);
                                                                            {
                                                                                let source_ml_source2_s399 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop106(heap.clone(), property_count, source_ml_source1_temporary_values103, source_ml_source1_temporary_index103.clone());
                                                                                match source_ml_source2_s399.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done => {
                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Return(source_ml_source1_temporary_result106) => {
                                                                                        source_ml_source1_temporary_result106.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(source_ml_source1_temporary_result110) => {
                                                                source_ml_source1_temporary_result110.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(source_ml_source1_temporary_result110_2) => {
                                        source_ml_source1_temporary_result110_2.clone()
                                    }
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
pub fn replace_java_script_ordinary_object(heap: self::JavaScriptOrdinaryHeap, replacement: self::JavaScriptOrdinaryObject) -> self::JavaScriptOrdinaryHeap {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2) => {
            {
                let objects = { let ml_values1 = shared_constructor_field1; let ml_callback1 = { let replacement = replacement.clone(); std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptOrdinaryObject { if ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value403, _, _, _, _) => {
                        source_ml_source2_data_property_value403.clone()
                    }
                }) == (match replacement.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value404, _, _, _, _) => {
                        source_ml_source2_data_property_value404.clone()
                    }
                })) {
                    replacement.clone()
                } else {
                    object.clone()
                } }) }; ml_values1.iter().cloned().enumerate().map(|(ml_index1, ml_value1)| ml_callback1(ml_value1)).collect::<Vec<_>>() };
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, shared_constructor_field2)
            }
        }
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
                let source_ml_source2_s406 = self::source_ml_source1_temporary_locate_java_script_ordinary_property_loop111(heap.clone(), key.clone(), current.clone(), depth);
                match source_ml_source2_s406.clone() {
                    self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop111Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop111Done => {
                        {
                            let source_ml_source2_field407 = false;
                            {
                                let source_ml_source2_field408 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_field409 = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false);
                                    (source_ml_source2_field409.clone(), source_ml_source2_field407, source_ml_source2_field408.clone())
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop111Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop111Return(source_ml_source1_temporary_result111) => {
                        source_ml_source1_temporary_result111.clone()
                    }
                }
            }
        }
    }
}
