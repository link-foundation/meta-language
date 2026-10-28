// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_value_error;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop90(mut numbers: Vec<f64>, mut source_ml_source1_temporary_values89: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index89: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result {
    loop {
        return if (source_ml_source1_temporary_index89 < (self::ml::Big::from_u128(source_ml_source1_temporary_values89.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values89, source_ml_source1_temporary_index89.to_index());
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2004) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        {
                            let numbers_2 = { let (ml_append_right, ml_append_left) = (vec![value_value], numbers); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index89_2 = source_ml_source1_temporary_index89.add(&self::ml::Big::from_i128(1));
                                {
                                    (numbers, source_ml_source1_temporary_index89) = (numbers_2, source_ml_source1_temporary_index89_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f2005) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f2006) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2007) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f2008, source_ml_source2_f2009) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Done(numbers)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Result {
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Done,
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_primitive_loop92(mut source_ml_source1_temporary_values91: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index91: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Result {
    loop {
        return if (source_ml_source1_temporary_index91 < (self::ml::Big::from_u128(source_ml_source1_temporary_values91.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values91, source_ml_source1_temporary_index91.to_index());
                {
                    let error = self::read_java_script_semantic_value_error(value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index91_2 = source_ml_source1_temporary_index91.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index91 = source_ml_source1_temporary_index91_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Result {
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Done,
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_code_function_loop93(mut program: self::JavaScriptCallProgram, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let entry = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value2010, _, _) => source_ml_source2_data_property_value2010.as_slice() }), self::ml_array::number_index(middle));
                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2012, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value2012.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2014, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value2014.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94 {
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Next,
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>)),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Result {
    MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Done,
    MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_ordinary_object_loop96(mut objects: Vec<self::JavaScriptOrdinaryObject>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2016, _, _, _, _) => {
                                source_ml_source2_data_property_value2016.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2018, _, _, _, _) => {
                                    source_ml_source2_data_property_value2018.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Result {
    MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Done,
    MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_ordinary_symbol_loop97(mut symbols: Vec<self::ml::Big>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source2_comparison_operand2019 = symbol.clone();
                        {
                            let source_ml_source2_comparison_operand2020 = None::<()>;
                            match (source_ml_source2_comparison_operand2019.clone(), source_ml_source2_comparison_operand2020.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                        }
                    } {
                        self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Return(true)
                        } else {
                            if ((symbol.clone().expect("a proven present value")) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result {
    MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Done,
    MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_ordinary_property_loop98(mut properties: Vec<self::JavaScriptOrdinaryProperty>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result {
    loop {
        return if (at < ((properties.len() as f64))) {
            {
                let property = self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value({
                    let source_ml_source2_nullish_field_receiver2021 = (property.clone(),);
                    if match (source_ml_source2_nullish_field_receiver2021.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present2022 = ((source_ml_source2_nullish_field_receiver2021.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present2022.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2023, _) => {
                                    let source_ml_source2_data_property_value2023 = *source_ml_source2_data_property_value2023;
                                    source_ml_source2_data_property_value2023.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                    }
                }, key.clone())) {
                    self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Return(at)
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop106(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut prototype: self::JavaScriptSemanticValue, mut depth: f64) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result {
    loop {
        return match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2024) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f2025) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f2026) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f2027) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2028) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Object(identity, source_ml_source2_f2029) => {
                {
                    let depth_2 = (depth + 1f64);
                    if ((identity == (match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2031, _, _, _, _) => {
                            source_ml_source2_data_property_value2031.clone()
                        }
                    })) || (depth_2 > (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2033, _) => source_ml_source2_data_property_value2033.as_slice() }).len() as f64)))) {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 97, 105, 110, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 121, 99, 108, 105, 99]))
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2034, _) => {
                                    source_ml_source2_data_property_value2034
                                }
                            }, identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2035, _) => source_ml_source2_data_property_value2035.as_slice() }), self::ml_array::number_index(index));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                } else {
                                    {
                                        let prototype_2 = match parent.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2036, _, _, _) => {
                                                let source_ml_source2_data_property_value2036 = *source_ml_source2_data_property_value2036;
                                                source_ml_source2_data_property_value2036.clone()
                                            }
                                        };
                                        {
                                            (prototype, depth) = (prototype_2.clone(), depth_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop105(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result {
    loop {
        return if (at < (((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2038) => source_ml_source2_data_property_value2038.as_slice() }).len() as f64))) {
            {
                let property = self::ml_array::optional_at((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2039) => source_ml_source2_data_property_value2039.as_slice() }), self::ml_array::number_index(at));
                if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_key_error(heap.clone(), match property.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2040, _) => {
                                let source_ml_source2_data_property_value2040 = *source_ml_source2_data_property_value2040;
                                source_ml_source2_data_property_value2040.clone()
                            }
                        });
                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(key_error.clone())
                        } else {
                            if ((self::find_java_script_ordinary_property(match object.clone() {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2043) => {
                                    source_ml_source2_data_property_value2043
                                }
                            }, match property.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2044, _) => {
                                    let source_ml_source2_data_property_value2044 = *source_ml_source2_data_property_value2044;
                                    source_ml_source2_data_property_value2044.clone()
                                }
                            })) != at) {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                            } else {
                                {
                                    let descriptor = match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2045) => {
                                            source_ml_source2_data_property_value2045.clone()
                                        }
                                    };
                                    if !(self::test_complete_java_script_property_descriptor(descriptor.clone())) {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 105, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115]))
                                    } else {
                                        {
                                            let source_ml_source2_s2046 = if (descriptor.clone()).9 {
                                                {
                                                    let error = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8);
                                                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Return(error.clone())
                                                    } else {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Next
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Next
                                            };
                                            match source_ml_source2_s2046.clone() {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Next => {
                                                    {
                                                        let source_ml_source2_s2047 = if (descriptor.clone()).5 {
                                                            {
                                                                let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4);
                                                                if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(error_2.clone())
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                        };
                                                        match source_ml_source2_s2047.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next => {
                                                                if (descriptor.clone()).7 {
                                                                    {
                                                                        let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6);
                                                                        if (error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(error_3.clone())
                                                                        } else {
                                                                            {
                                                                                let at_2 = (at + 1f64);
                                                                                {
                                                                                    at = at_2;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    {
                                                                        let at_3 = (at + 1f64);
                                                                        {
                                                                            at = at_3;
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(source_ml_source1_temporary_result108) => {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(source_ml_source1_temporary_result108.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Return(source_ml_source1_temporary_result108_2) => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(source_ml_source1_temporary_result108_2.clone())
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
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop104(mut heap: self::JavaScriptOrdinaryHeap, mut property_count: f64, mut source_ml_source1_temporary_values101: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index101: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result {
    loop {
        return if (source_ml_source1_temporary_index101 < (self::ml::Big::from_u128(source_ml_source1_temporary_values101.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_source1_temporary_values101, source_ml_source1_temporary_index101.to_index());
                if ({
                    let source_ml_source2_s2049 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2057, _, _, _) => {
                            let source_ml_source2_data_property_value2057 = *source_ml_source2_data_property_value2057;
                            source_ml_source2_data_property_value2057.clone()
                        }
                    };
                    match source_ml_source2_s2049.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            false
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2050) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f2051) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f2052) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f2053) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2054) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f2055, source_ml_source2_f2056) => {
                            true
                        }
                    }
                } && {
                    let source_ml_source2_s2059 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2067, _, _, _) => {
                            let source_ml_source2_data_property_value2067 = *source_ml_source2_data_property_value2067;
                            source_ml_source2_data_property_value2067.clone()
                        }
                    };
                    match source_ml_source2_s2059.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            true
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2060) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f2061) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f2062) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f2063) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2064) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f2065, source_ml_source2_f2066) => {
                            false
                        }
                    }
                }) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]))
                } else {
                    {
                        let prototype_error = self::read_java_script_ordinary_value_error(heap.clone(), match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2068, _, _, _) => {
                                let source_ml_source2_data_property_value2068 = *source_ml_source2_data_property_value2068;
                                source_ml_source2_data_property_value2068.clone()
                            }
                        });
                        if (prototype_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(prototype_error.clone())
                        } else {
                            {
                                let property_count_2 = (property_count + ((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2069) => source_ml_source2_data_property_value2069.as_slice() }).len() as f64));
                                if ((property_count_2 > (16384f64)) || ((((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2071) => source_ml_source2_data_property_value2071.as_slice() }).len() as f64)) > (4096f64))) {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source2_s2072 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop105(heap.clone(), object.clone(), at);
                                            match source_ml_source2_s2072.clone() {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Done => {
                                                    {
                                                        let prototype = match object.clone() {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2073, _, _, _) => {
                                                                let source_ml_source2_data_property_value2073 = *source_ml_source2_data_property_value2073;
                                                                source_ml_source2_data_property_value2073.clone()
                                                            }
                                                        };
                                                        {
                                                            let depth = 0f64;
                                                            {
                                                                let source_ml_source2_s2074 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop106(heap.clone(), object.clone(), prototype.clone(), depth);
                                                                match source_ml_source2_s2074.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done => {
                                                                        {
                                                                            let source_ml_source1_temporary_index101_2 = source_ml_source1_temporary_index101.add(&self::ml::Big::from_i128(1));
                                                                            {
                                                                                (property_count, source_ml_source1_temporary_index101) = (property_count_2, source_ml_source1_temporary_index101_2.clone());
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Return(source_ml_source1_temporary_result106) => {
                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(source_ml_source1_temporary_result106.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(source_ml_source1_temporary_result106_2) => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(source_ml_source1_temporary_result106_2.clone())
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
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop103(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_source1_temporary_values100: Vec<self::ml::Big>, mut source_ml_source1_temporary_index100: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result {
    loop {
        return if (source_ml_source1_temporary_index100 < (self::ml::Big::from_u128(source_ml_source1_temporary_values100.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_source1_temporary_values100, source_ml_source1_temporary_index100.to_index());
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2076, _) => {
                        source_ml_source2_data_property_value2076
                    }
                }, symbol.clone())) >= (0f64))) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = symbol.clone();
                        {
                            let source_ml_source1_temporary_index100_2 = source_ml_source1_temporary_index100.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_source1_temporary_index100) = (previous_2.clone(), source_ml_source1_temporary_index100_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop102(mut previous: self::ml::Big, mut source_ml_source1_temporary_values99: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index99: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result {
    loop {
        return if (source_ml_source1_temporary_index99 < (self::ml::Big::from_u128(source_ml_source1_temporary_values99.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_source1_temporary_values99, source_ml_source1_temporary_index99.to_index());
                if (((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2078, _, _, _, _) => {
                        source_ml_source2_data_property_value2078.clone()
                    }
                }) <= previous) || ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2080, _, _, _, _) => {
                        source_ml_source2_data_property_value2080.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2081, _, _, _, _) => {
                                source_ml_source2_data_property_value2081.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_index99_2 = source_ml_source1_temporary_index99.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_source1_temporary_index99) = (previous_2.clone(), source_ml_source1_temporary_index99_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result {
    MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done,
    MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool, self::ml::Big)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_locate_java_script_ordinary_property_loop109(mut heap: self::JavaScriptOrdinaryHeap, mut key: self::JavaScriptSemanticValue, mut current: self::ml::Big, mut depth: f64) -> self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result {
    loop {
        return if (depth < (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2083, _) => source_ml_source2_data_property_value2083.as_slice() }).len() as f64))) {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2084, _) => {
                        source_ml_source2_data_property_value2084
                    }
                }, current.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2085, _) => source_ml_source2_data_property_value2085.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                    } else {
                        {
                            let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2086) => {
                                    source_ml_source2_data_property_value2086
                                }
                            }, key.clone());
                            {
                                let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2087) => {
                                        source_ml_source2_data_property_value2087
                                    }
                                }, self::ml_array::number_index(at));
                                if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Return({
                                        let source_ml_source2_field2088 = true;
                                        {
                                            let source_ml_source2_field2089 = current.clone();
                                            {
                                                let source_ml_source2_field2091 = match property.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2090) => {
                                                        source_ml_source2_data_property_value2090.clone()
                                                    }
                                                };
                                                (source_ml_source2_field2091.clone(), source_ml_source2_field2088, source_ml_source2_field2089.clone())
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let prototype = match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2092, _, _, _) => {
                                                let source_ml_source2_data_property_value2092 = *source_ml_source2_data_property_value2092;
                                                source_ml_source2_data_property_value2092.clone()
                                            }
                                        };
                                        match prototype.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2093) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source2_f2094) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source2_f2095) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source2_f2096) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2097) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Object(current_2, source_ml_source2_f2098) => {
                                                {
                                                    let depth_2 = (depth + 1f64);
                                                    {
                                                        (current, depth) = (current_2.clone(), depth_2);
                                                        continue;
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
            self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_define_java_script_ordinary_property_loop111(mut count: f64, mut source_ml_source1_temporary_values110: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index110: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index110 < (self::ml::Big::from_u128(source_ml_source1_temporary_values110.len() as u128))) {
            {
                let entry = self::ml_array::at(&source_ml_source1_temporary_values110, source_ml_source1_temporary_index110.to_index());
                {
                    let count_2 = (count + ((match &entry { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2099) => source_ml_source2_data_property_value2099.as_slice() }).len() as f64));
                    {
                        let source_ml_source1_temporary_index110_2 = source_ml_source1_temporary_index110.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_source1_temporary_index110) = (count_2, source_ml_source1_temporary_index110_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            count
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Result {
    MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Done,
    MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Return(self::ml_string::Text),
}
