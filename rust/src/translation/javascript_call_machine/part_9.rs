// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Result;
use super::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result;
use super::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result;
use super::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result;
use super::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Result;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_property_descriptor_kind;
use super::read_java_script_semantic_value_error;
use super::source_ml_source1_temporary_find_java_script_ordinary_object_loop96;
use super::source_ml_source1_temporary_find_java_script_ordinary_property_loop98;
use super::source_ml_source1_temporary_locate_java_script_ordinary_property_loop109;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop102;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop103;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop104;
use super::source_ml_source1_temporary_test_java_script_ordinary_symbol_loop97;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_property_descriptor(kind: &self::ml_string::Text, configurable: bool, enumerable: bool) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field266 = true;
        {
            let source_ml_source2_field267 = configurable;
            {
                let source_ml_source2_field268 = true;
                {
                    let source_ml_source2_field269 = enumerable;
                    {
                        let source_ml_source2_field270 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                        {
                            let source_ml_source2_field271 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_field272 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                {
                                    let source_ml_source2_field273 = false;
                                    {
                                        let source_ml_source2_field274 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                        {
                                            let source_ml_source2_field275 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field276 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                                {
                                                    let source_ml_source2_field277 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field267, source_ml_source2_field266, source_ml_source2_field269, source_ml_source2_field268, source_ml_source2_field275.clone(), source_ml_source2_field274, source_ml_source2_field277.clone(), source_ml_source2_field276, source_ml_source2_field271.clone(), source_ml_source2_field270, source_ml_source2_field273, source_ml_source2_field272)
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
        let source_ml_source2_field278 = true;
        {
            let source_ml_source2_field279 = if (proposed.clone()).1 {
                (proposed.clone()).0
            } else {
                (base.clone()).0
            };
            {
                let source_ml_source2_field280 = true;
                {
                    let source_ml_source2_field281 = if (proposed.clone()).3 {
                        (proposed.clone()).2
                    } else {
                        (base.clone()).2
                    };
                    {
                        let source_ml_source2_field282 = (base.clone()).9;
                        {
                            let source_ml_source2_field283 = if ((base.clone()).9 && (proposed.clone()).9) {
                                (proposed.clone()).8
                            } else {
                                (base.clone()).8
                            };
                            {
                                let source_ml_source2_field284 = (base.clone()).11;
                                {
                                    let source_ml_source2_field285 = if ((base.clone()).11 && (proposed.clone()).11) {
                                        (proposed.clone()).10
                                    } else {
                                        (base.clone()).10
                                    };
                                    {
                                        let source_ml_source2_field286 = (base.clone()).5;
                                        {
                                            let source_ml_source2_field287 = if ((base.clone()).5 && (proposed.clone()).5) {
                                                (proposed.clone()).4
                                            } else {
                                                (base.clone()).4
                                            };
                                            {
                                                let source_ml_source2_field288 = (base.clone()).7;
                                                {
                                                    let source_ml_source2_field289 = if ((base.clone()).7 && (proposed.clone()).7) {
                                                        (proposed.clone()).6
                                                    } else {
                                                        (base.clone()).6
                                                    };
                                                    (source_ml_source2_field279, source_ml_source2_field278, source_ml_source2_field281, source_ml_source2_field280, source_ml_source2_field287.clone(), source_ml_source2_field286, source_ml_source2_field289.clone(), source_ml_source2_field288, source_ml_source2_field283.clone(), source_ml_source2_field282, source_ml_source2_field285, source_ml_source2_field284)
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
                        let source_ml_source2_concat_value290 = writes;
                        {
                            let source_ml_source2_concat_value291 = vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value290), source_ml_source2_concat_value291)
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
                            let source_ml_source2_concat_value292 = writes_3;
                            {
                                let source_ml_source2_concat_value293 = vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])];
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value292), source_ml_source2_concat_value293)
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
                                let source_ml_source2_concat_value294 = writes_5;
                                {
                                    let source_ml_source2_concat_value295 = vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])];
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value294), source_ml_source2_concat_value295)
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
                                    let source_ml_source2_concat_value296 = writes_7;
                                    {
                                        let source_ml_source2_concat_value297 = vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value296), source_ml_source2_concat_value297)
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
                                        let source_ml_source2_concat_value298 = writes_9;
                                        {
                                            let source_ml_source2_concat_value299 = vec![self::ml_string::Text::from_units(vec![103, 101, 116])];
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value298), source_ml_source2_concat_value299)
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
                                            let source_ml_source2_concat_value300 = writes_11;
                                            {
                                                let source_ml_source2_concat_value301 = vec![self::ml_string::Text::from_units(vec![115, 101, 116])];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value300), source_ml_source2_concat_value301)
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
        let source_ml_source2_field302 = disposition.clone();
        {
            let source_ml_source2_field303 = error.clone();
            {
                let source_ml_source2_field304 = descriptor.clone();
                {
                    let source_ml_source2_field305 = writes;
                    (source_ml_source2_field304.clone(), source_ml_source2_field302.clone(), source_ml_source2_field303.clone(), source_ml_source2_field305)
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
                                    let source_ml_source2_s306 = if !((current.clone()).0) {
                                        if ((proposed.clone()).1 && (proposed.clone()).0) {
                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else {
                                            if ((proposed.clone()).3 && (((proposed.clone()).2) != ((current.clone()).2))) {
                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if ((proposed.clone()).5 && !(self::test_java_script_semantic_same_value((proposed.clone()).4, (current.clone()).4))) {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                        } else {
                                                            if ((proposed.clone()).7 && !(self::test_java_script_semantic_same_value((proposed.clone()).6, (current.clone()).6))) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Next
                                                            }
                                                        }
                                                    } else {
                                                        if !((current.clone()).10) {
                                                            if ((proposed.clone()).11 && (proposed.clone()).10) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                if (proposed.clone()).9 {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value((proposed.clone()).8, (current.clone()).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                                } else {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Next
                                    };
                                    match source_ml_source2_s306.clone() {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Next => {
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
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(source_ml_source1_temporary_result94) => {
                                            source_ml_source1_temporary_result94.clone()
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
                let source_ml_source2_s310 = self::source_ml_source1_temporary_find_java_script_ordinary_object_loop96(objects, identity.clone(), low, high);
                match source_ml_source2_s310.clone() {
                    self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Return(source_ml_source1_temporary_result96) => {
                        source_ml_source1_temporary_result96
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
                let source_ml_source2_s311 = self::source_ml_source1_temporary_test_java_script_ordinary_symbol_loop97(symbols, identity.clone(), low, high);
                match source_ml_source2_s311.clone() {
                    self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Done => {
                        false
                    }
                    self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Return(source_ml_source1_temporary_result97) => {
                        source_ml_source1_temporary_result97
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f312) => {
                    {
                        let source_ml_source2_f328 = source_ml_source2_f312;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f313) => {
                    {
                        let source_ml_source2_f334 = source_ml_source2_f313;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f314) => {
                    {
                        let source_ml_source2_f340 = source_ml_source2_f314.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f315) => {
                    {
                        let source_ml_source2_f346 = source_ml_source2_f315;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    if self::test_java_script_ordinary_symbol(match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value348) => {
                            source_ml_source2_data_property_value348
                        }
                    }, identity.clone()) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f316, source_ml_source2_f317) => {
                    {
                        let identity_2 = source_ml_source2_f316.clone();
                        {
                            let callable = source_ml_source2_f317;
                            {
                                let at = self::find_java_script_ordinary_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value354, _) => {
                                        source_ml_source2_data_property_value354
                                    }
                                }, identity_2.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value355, _) => source_ml_source2_data_property_value355.as_slice() }), self::ml_array::number_index(at));
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (({
                                            let source_ml_source2_nullish_field_receiver359 = (object.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver359.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present360 = ((source_ml_source2_nullish_field_receiver359.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present360.clone() {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value361, _) => {
                                                            source_ml_source2_data_property_value361
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f362) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f363) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f364) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f365) => {
            false
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f366) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f367, source_ml_source2_f368) => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f369) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f370) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f371) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f372) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f373) => {
            false
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f374, source_ml_source2_f375) => {
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
            let source_ml_source2_s376 = self::source_ml_source1_temporary_find_java_script_ordinary_property_loop98(properties, key.clone(), at);
            match source_ml_source2_s376.clone() {
                self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Return(source_ml_source1_temporary_result98) => {
                    source_ml_source1_temporary_result98
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_heap_error(heap: self::JavaScriptOrdinaryHeap) -> self::ml_string::Text {
    if (((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value378, _) => source_ml_source2_data_property_value378.as_slice() }).len() as f64)) > (4096f64)) || ((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value380) => source_ml_source2_data_property_value380.as_slice() }).len() as f64)) > (16384f64))) {
        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
    } else {
        {
            let previous = self::ml::Big::from_i128(0);
            {
                let source_ml_source1_temporary_values99 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value381, _) => {
                        source_ml_source2_data_property_value381
                    }
                };
                {
                    let source_ml_source1_temporary_index99 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s382 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop102(previous.clone(), source_ml_source1_temporary_values99, source_ml_source1_temporary_index99.clone());
                        match source_ml_source2_s382.clone() {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done(previous_2) => {
                                {
                                    let previous_3 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_temporary_values100 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value383) => {
                                                source_ml_source2_data_property_value383
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_index100 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s384 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop103(heap.clone(), previous_3.clone(), source_ml_source1_temporary_values100, source_ml_source1_temporary_index100.clone());
                                                match source_ml_source2_s384.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done => {
                                                        {
                                                            let property_count = 0f64;
                                                            {
                                                                let source_ml_source1_temporary_values101 = match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value385, _) => {
                                                                        source_ml_source2_data_property_value385
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_temporary_index101 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source2_s386 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop104(heap.clone(), property_count, source_ml_source1_temporary_values101, source_ml_source1_temporary_index101.clone());
                                                                        match source_ml_source2_s386.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Done => {
                                                                                self::ml_string::Text::from_units(vec![])
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(source_ml_source1_temporary_result104) => {
                                                                                source_ml_source1_temporary_result104.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Return(source_ml_source1_temporary_result108) => {
                                                        source_ml_source1_temporary_result108.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(source_ml_source1_temporary_result108_2) => {
                                source_ml_source1_temporary_result108_2.clone()
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
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value387, _) => {
                source_ml_source2_data_property_value387
            }
        }; let ml_callback1 = { let replacement = replacement.clone(); std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptOrdinaryObject { if ((match object.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value390, _, _, _, _) => {
                source_ml_source2_data_property_value390.clone()
            }
        }) == (match replacement.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value391, _, _, _, _) => {
                source_ml_source2_data_property_value391.clone()
            }
        })) {
            replacement.clone()
        } else {
            object.clone()
        } }) }; ml_values1.iter().cloned().enumerate().map(|(ml_index1, ml_value1)| ml_callback1(ml_value1)).collect::<Vec<_>>() };
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value392) => {
                source_ml_source2_data_property_value392
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
                let source_ml_source2_s393 = self::source_ml_source1_temporary_locate_java_script_ordinary_property_loop109(heap.clone(), key.clone(), current.clone(), depth);
                match source_ml_source2_s393.clone() {
                    self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done => {
                        {
                            let source_ml_source2_field394 = false;
                            {
                                let source_ml_source2_field395 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_field396 = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false);
                                    (source_ml_source2_field396.clone(), source_ml_source2_field394, source_ml_source2_field395.clone())
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Return(source_ml_source1_temporary_result109) => {
                        source_ml_source1_temporary_result109.clone()
                    }
                }
            }
        }
    }
}
