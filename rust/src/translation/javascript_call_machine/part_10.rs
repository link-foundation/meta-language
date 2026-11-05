// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Result;
use super::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Result;
use super::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result;
use super::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Result;
use super::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Result;
use super::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Result;
use super::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Result;
use super::convert_java_script_boolean_big_integer;
use super::convert_java_script_number_big_integer;
use super::convert_java_script_string_big_integer_units;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_call_integer_text;
use super::read_java_script_call_integer_width;
use super::read_java_script_integer_protocol_operation_error;
use super::refuse_java_script_call_integer_conversion;
use super::source_ml_source1_temporary_find_java_script_ordinary_object_loop68;
use super::source_ml_source1_temporary_find_java_script_ordinary_property_loop70;
use super::source_ml_source1_temporary_locate_java_script_ordinary_property_loop81;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop74;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop75;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop76;
use super::source_ml_source1_temporary_read_java_script_semantic_value_error_loop64;
use super::source_ml_source1_temporary_test_java_script_ordinary_symbol_loop69;
use super::source_ml_source1_temporary_test_java_script_semantic_same_value_loop65;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_integer_conversion(operation: &self::ml_string::Text, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    {
        let error = self::read_java_script_integer_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::refuse_java_script_call_integer_conversion(&(self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])), &(self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])))
                }
                self::JavaScriptSemanticValue::Null => {
                    self::refuse_java_script_call_integer_conversion(&(self::ml_string::Text::from_units(vec![110, 117, 108, 108])), &(self::ml_string::Text::from_units(vec![110, 117, 108, 108])))
                }
                self::JavaScriptSemanticValue::Boolean(value_value) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(self::convert_java_script_boolean_big_integer(value_value))))
                }
                self::JavaScriptSemanticValue::Number(value_value_2) => {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 101, 103, 101, 114]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 78, 117, 109, 98, 101, 114, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116]))
                    } else {
                        {
                            let converted = self::convert_java_script_number_big_integer(value_value_2);
                            if (((converted.len() as f64)) != (1f64)) {
                                self::refuse_java_script_call_integer_conversion(&(self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])), &(self::ml_string::Text::from_text(&self::ml_number::js_number(value_value_2))))
                            } else {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer((self::ml_array::optional_at(&converted, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)))))
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
                }
                self::JavaScriptSemanticValue::Text(units) => {
                    if (((units.len() as f64)) > (4096f64)) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                    } else {
                        {
                            let converted_2 = self::convert_java_script_string_big_integer_units(units.clone());
                            if (((converted_2.len() as f64)) != (1f64)) {
                                self::refuse_java_script_call_integer_conversion(&(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])), &(self::read_java_script_call_integer_text(&(units))))
                            } else {
                                {
                                    let integer = (self::ml_array::optional_at(&converted_2, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                    if ((self::read_java_script_call_integer_width(integer.clone())) < (0f64)) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                    } else {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(integer.clone())))
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116]))
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_value_error(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Text(units) => {
            if (((units.len() as f64)) > (1048576f64)) {
                self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s105 = self::source_ml_source1_temporary_read_java_script_semantic_value_error_loop64(&(units), at);
                        match source_ml_source2_s105.clone() {
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Return(source_ml_source1_temporary_result64) => {
                                source_ml_source1_temporary_result64.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if ((identity > (self::ml::Big::from_i128(0))) && (identity <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, _) => {
            if ((identity_2 > (self::ml::Big::from_i128(0))) && (identity_2 <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
        _ => {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_same_value(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_java_script_semantic_value_error(right.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![83, 97, 109, 101, 86, 97, 108, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 119, 111, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 117, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 109, 111, 100, 101, 108])).0, message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Null => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Boolean(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Boolean(value) => {
                        (left_value == value)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Number(left_value_2) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(left_value_3) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(left_units) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Text(units) => {
                        if (((left_units.len() as f64)) != ((units.len() as f64))) {
                            false
                        } else {
                            {
                                let at = 0f64;
                                {
                                    let source_ml_source2_s145 = self::source_ml_source1_temporary_test_java_script_semantic_same_value_loop65(&(left_units), &(units), at);
                                    match source_ml_source2_s145.clone() {
                                        self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Done => {
                                            true
                                        }
                                        self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Return(source_ml_source1_temporary_result65) => {
                                            source_ml_source1_temporary_result65
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(left_identity) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, _) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Object(identity_2, _) => {
                        (left_identity_2 == identity_2)
                    }
                    _ => {
                        false
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_accessor(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
        false
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                true
            }
            self::JavaScriptSemanticValue::Object(_, source_ml_source2_f164) => {
                source_ml_source2_f164
            }
            _ => {
                false
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
        let source_ml_source2_field211 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
        {
            let source_ml_source2_field212 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source2_field213 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                {
                    let source_ml_source2_field215 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                    {
                        let source_ml_source2_field216 = self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source2_field217 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                            {
                                let source_ml_source2_field218 = self::JavaScriptSemanticValue::Undefined;
                                (configurable, true, enumerable, true, source_ml_source2_field216.clone(), source_ml_source2_field215, source_ml_source2_field218.clone(), source_ml_source2_field217, source_ml_source2_field212.clone(), source_ml_source2_field211, false, source_ml_source2_field213)
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
        let source_ml_source2_field220 = if (proposed.clone()).1 {
            (proposed.clone()).0
        } else {
            (base.clone()).0
        };
        {
            let source_ml_source2_field222 = if (proposed.clone()).3 {
                (proposed.clone()).2
            } else {
                (base.clone()).2
            };
            {
                let source_ml_source2_field223 = (base.clone()).9;
                {
                    let source_ml_source2_field224 = if ((base.clone()).9 && (proposed.clone()).9) {
                        (proposed.clone()).8
                    } else {
                        (base.clone()).8
                    };
                    {
                        let source_ml_source2_field225 = (base.clone()).11;
                        {
                            let source_ml_source2_field226 = if ((base.clone()).11 && (proposed.clone()).11) {
                                (proposed.clone()).10
                            } else {
                                (base.clone()).10
                            };
                            {
                                let source_ml_source2_field227 = (base.clone()).5;
                                {
                                    let source_ml_source2_field228 = if ((base.clone()).5 && (proposed.clone()).5) {
                                        (proposed.clone()).4
                                    } else {
                                        (base.clone()).4
                                    };
                                    {
                                        let source_ml_source2_field229 = (base.clone()).7;
                                        {
                                            let source_ml_source2_field230 = if ((base.clone()).7 && (proposed.clone()).7) {
                                                (proposed.clone()).6
                                            } else {
                                                (base.clone()).6
                                            };
                                            (source_ml_source2_field220, true, source_ml_source2_field222, true, source_ml_source2_field228.clone(), source_ml_source2_field227, source_ml_source2_field230.clone(), source_ml_source2_field229, source_ml_source2_field224.clone(), source_ml_source2_field223, source_ml_source2_field226, source_ml_source2_field225)
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
                        let source_ml_source2_concat_value232 = vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])];
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), writes), source_ml_source2_concat_value232)
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
                            let source_ml_source2_concat_value234 = vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), writes_3), source_ml_source2_concat_value234)
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
                                let source_ml_source2_concat_value236 = vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])];
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), writes_5), source_ml_source2_concat_value236)
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
                                    let source_ml_source2_concat_value238 = vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])];
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), writes_7), source_ml_source2_concat_value238)
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
                                        let source_ml_source2_concat_value240 = vec![self::ml_string::Text::from_units(vec![103, 101, 116])];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), writes_9), source_ml_source2_concat_value240)
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
                                            let source_ml_source2_concat_value242 = vec![self::ml_string::Text::from_units(vec![115, 101, 116])];
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), writes_11), source_ml_source2_concat_value242)
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
    (descriptor.clone(), disposition.clone(), error.clone(), writes)
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
                                    let source_ml_source2_s247 = if !((current.clone()).0) {
                                        if ((proposed.clone()).1 && (proposed.clone()).0) {
                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else {
                                            if ((proposed.clone()).3 && (((proposed.clone()).2) != ((current.clone()).2))) {
                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if ((proposed.clone()).5 && !(self::test_java_script_semantic_same_value((proposed.clone()).4, (current.clone()).4))) {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                        } else {
                                                            if ((proposed.clone()).7 && !(self::test_java_script_semantic_same_value((proposed.clone()).6, (current.clone()).6))) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Next
                                                            }
                                                        }
                                                    } else {
                                                        if !((current.clone()).10) {
                                                            if ((proposed.clone()).11 && (proposed.clone()).10) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                if (proposed.clone()).9 {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value((proposed.clone()).8, (current.clone()).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                                } else {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Next
                                    };
                                    match source_ml_source2_s247.clone() {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Next => {
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
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Return(source_ml_source1_temporary_result66) => {
                                            source_ml_source1_temporary_result66.clone()
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
                let source_ml_source2_s251 = self::source_ml_source1_temporary_find_java_script_ordinary_object_loop68(objects, identity.clone(), low, high);
                match source_ml_source2_s251.clone() {
                    self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Return(source_ml_source1_temporary_result68) => {
                        source_ml_source1_temporary_result68
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
                let source_ml_source2_s252 = self::source_ml_source1_temporary_test_java_script_ordinary_symbol_loop69(symbols, identity.clone(), low, high);
                match source_ml_source2_s252.clone() {
                    self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Done => {
                        false
                    }
                    self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Return(source_ml_source1_temporary_result69) => {
                        source_ml_source1_temporary_result69
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
                        self::JavaScriptSemanticValue::Symbol(identity) => {
                            if self::test_java_script_ordinary_symbol(shared_constructor_field2, identity.clone()) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f257, source_ml_source2_f258) => {
                            {
                                let at = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), source_ml_source2_f257.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(at));
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (({
                                            let source_ml_source2_nullish_field_receiver300 = (object.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver300.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present301 = ((source_ml_source2_nullish_field_receiver300.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present301.clone() {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value302, _) => {
                                                            source_ml_source2_data_property_value302
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }) == source_ml_source2_f258)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            self::ml_string::Text::from_units(vec![])
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
            let source_ml_source2_s317 = self::source_ml_source1_temporary_find_java_script_ordinary_property_loop70(properties, key.clone(), at);
            match source_ml_source2_s317.clone() {
                self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Return(source_ml_source1_temporary_result70) => {
                    source_ml_source1_temporary_result70
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
                        let source_ml_source1_temporary_values71 = shared_constructor_field1.clone();
                        {
                            let source_ml_source1_temporary_index71 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s323 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop74(previous.clone(), source_ml_source1_temporary_values71, source_ml_source1_temporary_index71.clone());
                                match source_ml_source2_s323.clone() {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Done(previous_2) => {
                                        {
                                            let previous_3 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_temporary_values72 = shared_constructor_field2;
                                                {
                                                    let source_ml_source1_temporary_index72 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s325 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop75(heap.clone(), previous_3.clone(), source_ml_source1_temporary_values72, source_ml_source1_temporary_index72.clone());
                                                        match source_ml_source2_s325.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Done => {
                                                                {
                                                                    let property_count = 0f64;
                                                                    {
                                                                        let source_ml_source1_temporary_values73 = shared_constructor_field1;
                                                                        {
                                                                            let source_ml_source1_temporary_index73 = self::ml::Big::from_i128(0);
                                                                            {
                                                                                let source_ml_source2_s327 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop76(heap.clone(), property_count, source_ml_source1_temporary_values73, source_ml_source1_temporary_index73.clone());
                                                                                match source_ml_source2_s327.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Done => {
                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Return(source_ml_source1_temporary_result76) => {
                                                                                        source_ml_source1_temporary_result76.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Return(source_ml_source1_temporary_result80) => {
                                                                source_ml_source1_temporary_result80.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Return(source_ml_source1_temporary_result80_2) => {
                                        source_ml_source1_temporary_result80_2.clone()
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
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value331, _, _, _, _) => {
                        source_ml_source2_data_property_value331.clone()
                    }
                }) == (match replacement.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value332, _, _, _, _) => {
                        source_ml_source2_data_property_value332.clone()
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
        let depth = 0f64;
        {
            let source_ml_source2_s334 = self::source_ml_source1_temporary_locate_java_script_ordinary_property_loop81(heap.clone(), key.clone(), identity.clone(), depth);
            match source_ml_source2_s334.clone() {
                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Done => {
                    {
                        let source_ml_source2_field336 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_field337 = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false);
                            (source_ml_source2_field337.clone(), false, source_ml_source2_field336.clone())
                        }
                    }
                }
                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Return(source_ml_source1_temporary_result81) => {
                    source_ml_source1_temporary_result81.clone()
                }
            }
        }
    }
}
