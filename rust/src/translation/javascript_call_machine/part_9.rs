// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64;
use super::MlSource1TemporaryIndexUtf16TextLoop70Result;
use super::MlSource1TemporaryMatchUtf16TextLoop69Result;
use super::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Result;
use super::MlSource1TemporarySplitUtf16TextLoop68Result;
use super::apply_big_integer_bitwise_operation;
use super::apply_java_script_number_bitwise_operation;
use super::divide_java_script_big_integer;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::power_java_script_big_integer;
use super::source_ml_source1_temporary_index_utf16_text_loop70;
use super::source_ml_source1_temporary_match_utf16_text_loop69;
use super::source_ml_source1_temporary_points_utf16_text_loop66;
use super::source_ml_source1_temporary_read_java_script_call_integer_width_loop62;
use super::source_ml_source1_temporary_slice_utf16_text_loop65;
use super::source_ml_source1_temporary_split_utf16_text_loop67;
use super::source_ml_source1_temporary_split_utf16_text_loop68;

pub fn render_java_script_big_integer_conversion_message(form: self::ml_string::Text, value: self::ml_string::Text) -> self::ml_string::Text {
    {
        let pieces = self::read_java_script_big_integer_conversion_message_pieces(form.clone());
        if (((pieces.len() as f64)) == (2f64)) {
            (((self::ml_array::optional_at(&pieces, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))).concat(&value)).concat(&(self::ml_array::optional_at(&pieces, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_java_script_big_integer_conversion_message_pieces(form: self::ml_string::Text) -> Vec<self::ml_string::Text> {
    if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
        vec![self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32]), self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114])]
    } else {
        if (((form == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || (form == (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])))) || (form == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))) {
            vec![self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32]), self::ml_string::Text::from_units(vec![32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116])]
        } else {
            Vec::<self::ml_string::Text>::new()
        }
    }
}

pub fn read_java_script_big_integer_builtin_operation(name: self::ml_string::Text) -> self::ml_string::Text {
    if (name == (self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116]))) {
        self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_java_script_big_integer_result_operation(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 86, 97, 108, 117, 101]))) {
        self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 99, 111, 109, 112, 97, 114, 101])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 116, 114, 105, 110, 103, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 86, 97, 108, 117, 101]))) {
            self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 45, 99, 111, 109, 112, 97, 114, 101])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 86, 97, 108, 117, 101]))) {
                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 112, 111, 119, 101, 114])
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 82, 101, 115, 117, 108, 116]))) {
                    self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114])
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 82, 101, 115, 117, 108, 116]))) {
                        self::ml_string::Text::from_units(vec![100, 105, 118, 105, 115, 105, 111, 110])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![112, 97, 114, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 116, 114, 105, 110, 103, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114]))) {
                            self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 111, 111, 108, 101, 97, 110, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 86, 97, 108, 117, 101]))) {
                                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 82, 101, 115, 117, 108, 116]))) {
                                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                                } else {
                                    self::ml_string::Text::from_units(vec![])
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_linked_java_script_big_integer_result(kind: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    ((kind == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))) && ((self::read_java_script_big_integer_result_operation(operation.clone())) != (self::ml_string::Text::from_units(vec![]))))
}

pub fn read_java_script_big_integer_intrinsic_operand_kinds(operation: self::ml_string::Text) -> Vec<self::ml_string::Text> {
    {
        let form = self::read_java_script_big_integer_result_operation(operation.clone());
        if ((form == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 115, 105, 111, 110]))) || (form == (self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114])))) {
            vec![self::ml_string::Text::from_units(vec![105, 110, 116]), self::ml_string::Text::from_units(vec![105, 110, 116])]
        } else {
            if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 112, 111, 119, 101, 114]))) {
                vec![self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]), self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])]
            } else {
                if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 99, 111, 109, 112, 97, 114, 101]))) {
                    vec![self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]), self::ml_string::Text::from_units(vec![105, 110, 116])]
                } else {
                    if (form == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 45, 99, 111, 109, 112, 97, 114, 101]))) {
                        vec![self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![105, 110, 116])]
                    } else {
                        if (form == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                            vec![self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])]
                        } else {
                            if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                                vec![self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])]
                            } else {
                                if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) {
                                    vec![self::ml_string::Text::from_units(vec![98, 111, 111, 108])]
                                } else {
                                    Vec::<self::ml_string::Text>::new()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_big_integer_intrinsic_result_kind(operation: self::ml_string::Text) -> self::ml_string::Text {
    {
        let form = self::read_java_script_big_integer_result_operation(operation.clone());
        if (((form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 112, 111, 119, 101, 114]))) || (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 99, 111, 109, 112, 97, 114, 101])))) || (form == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 45, 99, 111, 109, 112, 97, 114, 101])))) {
            self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])
        } else {
            if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) {
                self::ml_string::Text::from_units(vec![105, 110, 116])
            } else {
                if (form == (self::ml_string::Text::from_units(vec![]))) {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 45, 105, 110, 116, 101, 103, 101, 114])
                }
            }
        }
    }
}

pub fn render_java_script_big_integer_comparison(target: self::ml_string::Text, form: self::ml_string::Text, left: self::ml_string::Text, right: self::ml_string::Text, utf16: bool) -> self::ml_string::Text {
    if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 99, 111, 109, 112, 97, 114, 101]))) {
        if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            ((((self::ml_string::Text::from_units(vec![109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 109, 112, 97, 114, 101, 78, 117, 109, 98, 101, 114, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                ((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 109, 112, 97, 114, 101, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 110, 117, 109, 98, 101, 114, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                if ((target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                    ((((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 109, 112, 97, 114, 101, 78, 117, 109, 98, 101, 114, 32])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    } else {
        if (form != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 45, 99, 111, 109, 112, 97, 114, 101]))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
                ((((self::ml_string::Text::from_units(vec![40, 40, 109, 108, 95, 116, 101, 120, 116, 44, 109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 41, 61, 62, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 109, 112, 97, 114, 101, 85, 110, 105, 116, 115, 40, 65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 40, 123, 108, 101, 110, 103, 116, 104, 58, 109, 108, 95, 116, 101, 120, 116, 46, 108, 101, 110, 103, 116, 104, 125, 44, 40, 95, 44, 105, 110, 100, 101, 120, 41, 61, 62, 109, 108, 95, 116, 101, 120, 116, 46, 99, 104, 97, 114, 67, 111, 100, 101, 65, 116, 40, 105, 110, 100, 101, 120, 41, 41, 44, 109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 41, 41, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                    ((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 109, 112, 97, 114, 101, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 115, 116, 114, 105, 110, 103, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 117, 110, 105, 116, 115, 40, 38])).concat(&if utf16 {
                        ((self::ml_string::Text::from_units(vec![40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 46, 48, 46, 105, 116, 101, 114, 40, 41, 46, 99, 111, 112, 105, 101, 100, 40, 41]))
                    } else {
                        ((self::ml_string::Text::from_units(vec![40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 46, 101, 110, 99, 111, 100, 101, 95, 117, 116, 102, 49, 54, 40, 41]))
                    })).concat(&self::ml_string::Text::from_units(vec![46, 109, 97, 112, 40, 102, 54, 52, 58, 58, 102, 114, 111, 109, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 102, 54, 52, 62, 62, 40, 41, 44]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
                } else {
                    if (target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                        ((((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 109, 112, 97, 114, 101, 85, 110, 105, 116, 115, 32, 40, 83, 116, 114, 105, 110, 103, 46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32, 40, 117, 110, 105, 116, 115, 32, 58, 32, 65, 114, 114, 97, 121, 32, 70, 108, 111, 97, 116, 41, 32, 99, 32, 61, 62, 32, 105, 102, 32, 99, 46, 116, 111, 78, 97, 116, 32, 60, 32, 54, 53, 53, 51, 54, 32, 116, 104, 101, 110, 32, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 99, 46, 116, 111, 78, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 53, 50, 57, 54, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 47, 32, 49, 48, 50, 52, 41, 41, 41, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 54, 51, 50, 48, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 37, 32, 49, 48, 50, 52, 41, 41, 41, 32, 35, 91, 93, 32])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 32]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
                    } else {
                        if (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113]))) {
                            ((((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 109, 112, 97, 114, 101, 85, 116, 102, 56, 32, 40, 40, 102, 105, 120, 32, 109, 108, 95, 99, 111, 109, 112, 97, 114, 101, 95, 98, 121, 116, 101, 115, 32, 40, 116, 101, 120, 116, 32, 58, 32, 115, 116, 114, 105, 110, 103, 41, 32, 58, 32, 108, 105, 115, 116, 32, 102, 108, 111, 97, 116, 32, 58, 61, 32, 109, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 119, 105, 116, 104, 32, 69, 109, 112, 116, 121, 83, 116, 114, 105, 110, 103, 32, 61, 62, 32, 110, 105, 108, 32, 124, 32, 83, 116, 114, 105, 110, 103, 32, 99, 32, 114, 101, 115, 116, 32, 61, 62, 32, 99, 111, 110, 115, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54, 51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102, 95, 110, 97, 116, 32, 40, 65, 115, 99, 105, 105, 46, 110, 97, 116, 95, 111, 102, 95, 97, 115, 99, 105, 105, 32, 99, 41, 41, 41, 41, 32, 40, 109, 108, 95, 99, 111, 109, 112, 97, 114, 101, 95, 98, 121, 116, 101, 115, 32, 114, 101, 115, 116, 41, 32, 101, 110, 100, 41, 32])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 32]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
                        } else {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

pub fn render_java_script_number_power(target: self::ml_string::Text, base: self::ml_string::Text, exponent: self::ml_string::Text) -> self::ml_string::Text {
    if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        ((((self::ml_string::Text::from_units(vec![109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 112, 111, 119, 101, 114, 78, 117, 109, 98, 101, 114, 40])).concat(&base)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&exponent)).concat(&self::ml_string::Text::from_units(vec![41]))
    } else {
        if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
            ((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 112, 111, 119, 101, 114, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 110, 117, 109, 98, 101, 114, 40])).concat(&base)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&exponent)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if ((target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                ((((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 112, 111, 119, 101, 114, 78, 117, 109, 98, 101, 114, 32])).concat(&base)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&exponent)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn render_java_script_big_integer_result_expression(target: self::ml_string::Text, form: self::ml_string::Text, value: self::ml_string::Text, utf16: bool) -> self::ml_string::Text {
    if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) {
        if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            ((self::ml_string::Text::from_units(vec![109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 66, 111, 111, 108, 101, 97, 110, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 111, 111, 108, 101, 97, 110, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                if ((target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                    ((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 66, 111, 111, 108, 101, 97, 110, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    } else {
        if ((form != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) && (form != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
                if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                    ((self::ml_string::Text::from_units(vec![109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 78, 117, 109, 98, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                } else {
                    ((self::ml_string::Text::from_units(vec![40, 40, 109, 108, 95, 116, 101, 120, 116, 41, 61, 62, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 85, 110, 105, 116, 115, 40, 65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 40, 123, 108, 101, 110, 103, 116, 104, 58, 109, 108, 95, 116, 101, 120, 116, 46, 108, 101, 110, 103, 116, 104, 125, 44, 32, 40, 95, 44, 105, 110, 100, 101, 120, 41, 61, 62, 109, 108, 95, 116, 101, 120, 116, 46, 99, 104, 97, 114, 67, 111, 100, 101, 65, 116, 40, 105, 110, 100, 101, 120, 41, 41, 41, 41, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                }
            } else {
                if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                    if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                        ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 110, 117, 109, 98, 101, 114, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                    } else {
                        {
                            let units = if utf16 {
                                ((self::ml_string::Text::from_units(vec![40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 46, 48, 46, 105, 116, 101, 114, 40, 41, 46, 99, 111, 112, 105, 101, 100, 40, 41]))
                            } else {
                                ((self::ml_string::Text::from_units(vec![40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 46, 101, 110, 99, 111, 100, 101, 95, 117, 116, 102, 49, 54, 40, 41]))
                            };
                            ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 115, 116, 114, 105, 110, 103, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 117, 110, 105, 116, 115, 40, 38])).concat(&units)).concat(&self::ml_string::Text::from_units(vec![46, 109, 97, 112, 40, 102, 54, 52, 58, 58, 102, 114, 111, 109, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 102, 54, 52, 62, 62, 40, 41, 41]))
                        }
                    }
                } else {
                    if (target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                        if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                            ((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 78, 117, 109, 98, 101, 114, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                        } else {
                            ((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 85, 110, 105, 116, 115, 32, 40, 83, 116, 114, 105, 110, 103, 46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32, 40, 117, 110, 105, 116, 115, 32, 58, 32, 65, 114, 114, 97, 121, 32, 70, 108, 111, 97, 116, 41, 32, 99, 32, 61, 62, 32, 105, 102, 32, 99, 46, 116, 111, 78, 97, 116, 32, 60, 32, 54, 53, 53, 51, 54, 32, 116, 104, 101, 110, 32, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 99, 46, 116, 111, 78, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 53, 50, 57, 54, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 47, 32, 49, 48, 50, 52, 41, 41, 41, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 54, 51, 50, 48, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 37, 32, 49, 48, 50, 52, 41, 41, 41, 32, 35, 91, 93, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                        }
                    } else {
                        if (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113]))) {
                            if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                                ((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 78, 117, 109, 98, 101, 114, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                            } else {
                                ((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 85, 116, 102, 56, 32, 40, 40, 102, 105, 120, 32, 109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 95, 98, 121, 116, 101, 115, 32, 40, 116, 101, 120, 116, 32, 58, 32, 115, 116, 114, 105, 110, 103, 41, 32, 58, 32, 108, 105, 115, 116, 32, 102, 108, 111, 97, 116, 32, 58, 61, 32, 109, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 119, 105, 116, 104, 32, 69, 109, 112, 116, 121, 83, 116, 114, 105, 110, 103, 32, 61, 62, 32, 110, 105, 108, 32, 124, 32, 83, 116, 114, 105, 110, 103, 32, 99, 32, 114, 101, 115, 116, 32, 61, 62, 32, 99, 111, 110, 115, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54, 51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102, 95, 110, 97, 116, 32, 40, 65, 115, 99, 105, 105, 46, 110, 97, 116, 95, 111, 102, 95, 97, 115, 99, 105, 105, 32, 99, 41, 41, 41, 41, 32, 40, 109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 95, 98, 121, 116, 101, 115, 32, 114, 101, 115, 116, 41, 32, 101, 110, 100, 41, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                            }
                        } else {
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
pub fn read_java_script_call_integer_arity(operation: &self::ml_string::Text) -> f64 {
    if ((((*operation) == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 105, 110, 116, 101, 103, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115])))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
        1f64
    } else {
        if self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116]), self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100])], operation, 0f64, false, |a, b| a == b) {
            2f64
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_width(value: self::ml::Big) -> f64 {
    {
        let rest = if (value < (self::ml::Big::from_i128(0))) {
            value.neg()
        } else {
            value.clone()
        };
        {
            let width = 0f64;
            {
                let source_ml_source2_s145 = self::source_ml_source1_temporary_read_java_script_call_integer_width_loop62(rest.clone(), width);
                match source_ml_source2_s145.clone() {
                    self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Done(width_2) => {
                        width_2
                    }
                    self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Return(source_ml_source1_temporary_result62) => {
                        source_ml_source1_temporary_result62
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_bit_code(operation: &self::ml_string::Text) -> f64 {
    if (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
        0f64
    } else {
        if (((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
            1f64
        } else {
            if (((*operation) == (self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
                2f64
            } else {
                if (((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116])))) {
                    3f64
                } else {
                    if (((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116])))) {
                        4f64
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100]))) {
                            5f64
                        } else {
                            if (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
                                6f64
                            } else {
                                (-1f64)
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
pub fn execute_java_script_call_number_bits(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let left = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let right = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Number(0f64));
            match left.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
                self::JavaScriptSemanticValue::Null => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f146) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
                self::JavaScriptSemanticValue::Number(left_value) => {
                    match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f152) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Number(value) => {
                            {
                                let code = self::read_java_script_call_integer_bit_code(operation);
                                if (code < (0f64)) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                } else {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::apply_java_script_number_bitwise_operation(code, left_value, value))))
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f153) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f154) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f155) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f156, source_ml_source2_f157) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f147) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f148) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f149) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f150, source_ml_source2_f151) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_integer_primitive(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    if operation.contains(&self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])) {
        self::execute_java_script_call_number_bits(operation, values)
    } else {
        {
            let first = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            {
                let second = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0)));
                match first.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f158) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f159) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Integer(first_value) => {
                        match second.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f164) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source2_f165) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Integer(value) => {
                                {
                                    let left = first_value.clone();
                                    {
                                        let right = value.clone();
                                        if ((((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114])))) && (right == (self::ml::Big::from_i128(0)))) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111]))
                                        } else {
                                            if (((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) && (right < (self::ml::Big::from_i128(0)))) {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![69, 120, 112, 111, 110, 101, 110, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101]))
                                            } else {
                                                {
                                                    let left_width = self::read_java_script_call_integer_width(left.clone());
                                                    {
                                                        let right_width = self::read_java_script_call_integer_width(right.clone());
                                                        if ((left_width < (0f64)) || (right_width < (0f64))) {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 105, 110, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                        } else {
                                                            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (left < right) {
                                                                    (-1f64)
                                                                } else {
                                                                    if (left > right) {
                                                                        1f64
                                                                    } else {
                                                                        0f64
                                                                    }
                                                                })))
                                                            } else {
                                                                if (((*operation) == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) && (((left_width + right_width)) > (4096f64))) {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                } else {
                                                                    if ((((((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) && (left != (self::ml::Big::from_i128(0)))) && (left != (self::ml::Big::from_i128(1)))) && (left != (self::ml::Big::from_i128(-1)))) && ((right.mul(&{
                                                                        let source_ml_source2_integer_operand172 = left_width;
                                                                        {
                                                                            let source_ml_source2_integer_result173 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(source_ml_source2_integer_operand172);
                                                                            if (((source_ml_source2_integer_result173.len() as f64)) == (1f64)) {
                                                                                (self::ml_array::optional_at(&source_ml_source2_integer_result173, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_integer_operand172)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                            }
                                                                        }
                                                                    })) > (self::ml::Big::from_i128(4096)))) {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 112, 111, 110, 101, 110, 116, 105, 97, 116, 105, 111, 110, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                    } else {
                                                                        {
                                                                            let source_ml_source2_s178 = if (((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116])))) {
                                                                                {
                                                                                    let growing = if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) {
                                                                                        (right >= (self::ml::Big::from_i128(0)))
                                                                                    } else {
                                                                                        (right < (self::ml::Big::from_i128(0)))
                                                                                    };
                                                                                    {
                                                                                        let count = if (right < (self::ml::Big::from_i128(0))) {
                                                                                            right.neg()
                                                                                        } else {
                                                                                            right.clone()
                                                                                        };
                                                                                        if (((left != (self::ml::Big::from_i128(0))) && growing) && (count > ({
                                                                                            let source_ml_source2_integer_operand181 = (4096f64 - left_width);
                                                                                            {
                                                                                                let source_ml_source2_integer_result182 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(source_ml_source2_integer_operand181);
                                                                                                if (((source_ml_source2_integer_result182.len() as f64)) == (1f64)) {
                                                                                                    (self::ml_array::optional_at(&source_ml_source2_integer_result182, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                                                } else {
                                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_integer_operand181)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                                                }
                                                                                            }
                                                                                        }))) {
                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 115, 104, 105, 102, 116, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))))
                                                                                        } else {
                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Next
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Next
                                                                            };
                                                                            match source_ml_source2_s178.clone() {
                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Next => {
                                                                                    {
                                                                                        let code = self::read_java_script_call_integer_bit_code(operation);
                                                                                        {
                                                                                            let result = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source2_s183 = if ((*operation) == (self::ml_string::Text::from_units(vec![97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                    {
                                                                                                        let result_2 = left.add(&right);
                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_2.clone())
                                                                                                    }
                                                                                                } else {
                                                                                                    if ((*operation) == (self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                        {
                                                                                                            let result_3 = left.sub(&right);
                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_3.clone())
                                                                                                        }
                                                                                                    } else {
                                                                                                        if ((*operation) == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                            {
                                                                                                                let result_4 = left.mul(&right);
                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_4.clone())
                                                                                                            }
                                                                                                        } else {
                                                                                                            if (((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114])))) {
                                                                                                                {
                                                                                                                    let result_5 = (self::ml_array::optional_at(&self::divide_java_script_big_integer(left.clone(), right.clone()), self::ml_array::number_index(if ((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                                        0f64
                                                                                                                    } else {
                                                                                                                        1f64
                                                                                                                    }))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_5.clone())
                                                                                                                }
                                                                                                            } else {
                                                                                                                if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) {
                                                                                                                    {
                                                                                                                        let result_6 = (self::ml_array::optional_at(&self::power_java_script_big_integer(left.clone(), right.clone()), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_6.clone())
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    if ((*operation) == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 105, 110, 116, 101, 103, 101, 114]))) {
                                                                                                                        {
                                                                                                                            let result_7 = left.neg();
                                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_7.clone())
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        if (code >= (0f64)) {
                                                                                                                            {
                                                                                                                                let result_8 = self::apply_big_integer_bitwise_operation(code, left.clone(), right.clone());
                                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_8.clone())
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))))
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                match source_ml_source2_s183.clone() {
                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_9) => {
                                                                                                        if ((self::read_java_script_call_integer_width(result_9.clone())) < (0f64)) {
                                                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                                                        } else {
                                                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(result_9.clone())))
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Return(source_ml_source1_temporary_result64) => {
                                                                                                        let source_ml_source1_temporary_result64 = *source_ml_source1_temporary_result64;
                                                                                                        source_ml_source1_temporary_result64.clone()
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Return(source_ml_source1_temporary_result64_2) => {
                                                                                    let source_ml_source1_temporary_result64_2 = *source_ml_source1_temporary_result64_2;
                                                                                    source_ml_source1_temporary_result64_2.clone()
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
                            self::JavaScriptSemanticValue::Text(source_ml_source2_f166) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f167) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Object(source_ml_source2_f168, source_ml_source2_f169) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f160) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f161) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f162, source_ml_source2_f163) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function2(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integral = if (value < (0f64)) {
                (-(((-value)).floor()))
            } else {
                value.floor()
            };
            {
                let remainder = (integral % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_integer(value: f64) -> f64 {
    if value.is_nan() {
        0f64
    } else {
        value.trunc()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_slice_position(value: f64, length: f64) -> f64 {
    {
        let integer = self::read_utf16_text_integer(value);
        if (integer < (0f64)) {
            self::ml_math::max((length + integer), 0f64)
        } else {
            self::ml_math::min(integer, length)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn slice_utf16_text(text: &[f64], start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::read_utf16_text_slice_position(start, (text.len() as f64));
        {
            let last = self::read_utf16_text_slice_position(end, (text.len() as f64));
            {
                let result = Vec::<f64>::new();
                {
                    let at = first;
                    {
                        let result_2 = self::source_ml_source1_temporary_slice_utf16_text_loop65(text, last, result, at);
                        result_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn substring_utf16_text(text: &[f64], start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(start), 0f64), (text.len() as f64));
        {
            let last = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(end), 0f64), (text.len() as f64));
            self::slice_utf16_text(text, self::ml_math::min(first, last), self::ml_math::max(first, last))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn character_utf16_text(text: &[f64], position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn relative_character_utf16_text(text: &[f64], position: f64) -> Vec<Vec<f64>> {
    {
        let integer = self::read_utf16_text_integer(position);
        {
            let at = if (integer < (0f64)) {
                ((text.len() as f64) + integer)
            } else {
                integer
            };
            if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
                Vec::<Vec<f64>>::new()
            } else {
                vec![vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn code_point_utf16_text(text: &[f64], position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            {
                let first = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                    vec![if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        (((65536f64 + ((first - 55296f64) * 1024f64)) + second) - 56320f64)
                    } else {
                        first
                    }]
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn points_utf16_text(text: &[f64]) -> Vec<Vec<f64>> {
    {
        let result = Vec::<Vec<f64>>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_points_utf16_text_loop66(text, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn split_utf16_text(text: &[f64], separator: &[f64], limit: f64, missing: bool) -> Vec<Vec<f64>> {
    {
        let count = self::dependency_function2(limit);
        {
            let result = Vec::<Vec<f64>>::new();
            if (count == (0f64)) {
                result
            } else {
                if missing {
                    vec![self::slice_utf16_text(text, 0f64, (text.len() as f64))]
                } else {
                    if (((separator.len() as f64)) == (0f64)) {
                        {
                            let at = 0f64;
                            {
                                let result_2 = self::source_ml_source1_temporary_split_utf16_text_loop67(text, count, result, at);
                                result_2
                            }
                        }
                    } else {
                        {
                            let start = 0f64;
                            {
                                let source_ml_source2_s184 = self::source_ml_source1_temporary_split_utf16_text_loop68(text, separator, count, result, start);
                                match source_ml_source2_s184.clone() {
                                    self::MlSource1TemporarySplitUtf16TextLoop68Result::MlSource1TemporarySplitUtf16TextLoop68Done(result_3, start_2) => {
                                        {
                                            let result_4 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text, start_2, (text.len() as f64))], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                            result_4
                                        }
                                    }
                                    self::MlSource1TemporarySplitUtf16TextLoop68Result::MlSource1TemporarySplitUtf16TextLoop68Return(source_ml_source1_temporary_result68) => {
                                        source_ml_source1_temporary_result68
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
pub fn match_utf16_text(text: &[f64], search: &[f64], start: f64) -> bool {
    if ((start < (0f64)) || (((start + (search.len() as f64))) > ((text.len() as f64)))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source2_s185 = self::source_ml_source1_temporary_match_utf16_text_loop69(text, search, start, at);
                match source_ml_source2_s185.clone() {
                    self::MlSource1TemporaryMatchUtf16TextLoop69Result::MlSource1TemporaryMatchUtf16TextLoop69Done => {
                        true
                    }
                    self::MlSource1TemporaryMatchUtf16TextLoop69Result::MlSource1TemporaryMatchUtf16TextLoop69Return(source_ml_source1_temporary_result69) => {
                        source_ml_source1_temporary_result69
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn index_utf16_text(text: &[f64], search: &[f64], position: f64) -> f64 {
    {
        let start = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(position), 0f64), (text.len() as f64));
        {
            let at = start;
            {
                let source_ml_source2_s186 = self::source_ml_source1_temporary_index_utf16_text_loop70(text, search, at);
                match source_ml_source2_s186.clone() {
                    self::MlSource1TemporaryIndexUtf16TextLoop70Result::MlSource1TemporaryIndexUtf16TextLoop70Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryIndexUtf16TextLoop70Result::MlSource1TemporaryIndexUtf16TextLoop70Return(source_ml_source1_temporary_result70) => {
                        source_ml_source1_temporary_result70
                    }
                }
            }
        }
    }
}
