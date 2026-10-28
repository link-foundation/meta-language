// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin44;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop41Result;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop46Result;
use super::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop56Result;
use super::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop57Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result;
use super::MlSource1TemporaryNormalizeNumberPowerBoundLoop51Result;
use super::MlSource1TemporaryNormalizeNumberPowerBoundLoop52Result;
use super::MlSource1TemporaryRootNumberPowerBoundJoin55;
use super::convert_java_script_string_number_units;
use super::convert_string_number_rational;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_utf8_string_units;
use super::source_ml_source1_temporary_convert_java_script_number_big_integer_loop47;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop41;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop42;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop43;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop46;
use super::source_ml_source1_temporary_decompose_number_power_magnitude_loop56;
use super::source_ml_source1_temporary_decompose_number_power_magnitude_loop57;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop48;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop49;
use super::source_ml_source1_temporary_normalize_number_power_bound_loop51;
use super::source_ml_source1_temporary_normalize_number_power_bound_loop52;
use super::source_ml_source1_temporary_power_java_script_big_integer_loop50;
use super::source_ml_source1_temporary_power_java_script_number_loop58;
use super::source_ml_source1_temporary_root_number_power_integer_loop53;
use super::source_ml_source1_temporary_root_number_power_integer_loop54;

pub fn convert_java_script_utf8_string_number_units(bytes: Vec<f64>) -> f64 {
    self::convert_java_script_string_number_units(self::read_java_script_utf8_string_units(bytes))
}

pub fn read_java_script_number_builtin_operation(name: self::ml_string::Text) -> self::ml_string::Text {
    if (name == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]))) {
        self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])
    } else {
        if (name == (self::ml_string::Text::from_units(vec![105, 115, 70, 105, 110, 105, 116, 101]))) {
            self::ml_string::Text::from_units(vec![116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 70, 105, 110, 105, 116, 101])
        } else {
            if (name == (self::ml_string::Text::from_units(vec![105, 115, 78, 97, 78]))) {
                self::ml_string::Text::from_units(vec![116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 78, 97, 78])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_java_script_number_conversion_result(operation: self::ml_string::Text) -> self::ml_string::Text {
    if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])))) {
        self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])
    } else {
        if ((operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 70, 105, 110, 105, 116, 101]))) || (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 78, 97, 78])))) {
            self::ml_string::Text::from_units(vec![98, 111, 111, 108])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_java_script_number_conversion_form(kind: self::ml_string::Text) -> self::ml_string::Text {
    if (((((((kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || (kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) || (kind == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))) || (kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))) {
        kind.clone()
    } else {
        self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100])
    }
}

pub fn accept_linked_java_script_number_conversion(kind: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    ((kind == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))) && ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])))))
}

pub fn read_java_script_number_conversion_failure(operation: self::ml_string::Text, kind: self::ml_string::Text) -> self::ml_string::Text {
    if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) && ((kind == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116]))))) {
        self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn require_java_script_number_conversion_units(kind: self::ml_string::Text, operation: self::ml_string::Text, argument: self::ml_string::Text) -> bool {
    (self::accept_linked_java_script_number_conversion(kind.clone(), operation.clone()) && (argument == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
}

pub fn read_java_script_primitive_number_operand(language: self::ml_string::Text, kind: self::ml_string::Text, element: self::ml_string::Text) -> self::ml_string::Text {
    if (language != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (((kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) {
            self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])
        } else {
            if ((kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && (element == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_java_script_primitive_numeric_inference_kind(kind: self::ml_string::Text) -> self::ml_string::Text {
    if (((kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) {
        self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_java_script_primitive_unary_operation(symbol: self::ml_string::Text) -> self::ml_string::Text {
    if (symbol == (self::ml_string::Text::from_units(vec![43]))) {
        self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_java_script_mathematical_conversion_form(language: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if (language != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 65, 114, 103, 117, 109, 101, 110, 116, 115]))) {
            self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121])
        } else {
            if ((operation == (self::ml_string::Text::from_units(vec![109, 97, 120]))) || (operation == (self::ml_string::Text::from_units(vec![109, 105, 110])))) {
                self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 100, 105, 99])
            } else {
                if (((((((operation == (self::ml_string::Text::from_units(vec![97, 98, 115]))) || (operation == (self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114])))) || (operation == (self::ml_string::Text::from_units(vec![99, 101, 105, 108])))) || (operation == (self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99])))) || (operation == (self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100])))) || (operation == (self::ml_string::Text::from_units(vec![115, 105, 103, 110])))) || (operation == (self::ml_string::Text::from_units(vec![115, 113, 114, 116])))) {
                    self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn render_java_script_number_scalar_conversion(format: self::ml_string::Text, operation: self::ml_string::Text, form: self::ml_string::Text, value: self::ml_string::Text, utf16: bool) -> self::ml_string::Text {
    if ((!(self::accept_linked_java_script_number_conversion(self::ml_string::Text::from_units(vec![109, 97, 116, 104]), operation.clone())) || ((self::read_java_script_number_conversion_form(form.clone())) == (self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100])))) || (form == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (format == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) {
                ((self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                ((self::ml_string::Text::from_units(vec![40, 43, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
            }
        } else {
            if ((format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (format == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                if !(self::read_java_script_number_conversion_failure(operation.clone(), form.clone())).0.is_empty() {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    if (form == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                        value.clone()
                    } else {
                        if (form == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                            if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                ((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32, 95, 32, 58, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 40, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 47, 32, 48, 41, 41]))
                            } else {
                                ((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32, 95, 32, 58, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 105, 110, 32, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 110, 97, 110, 41]))
                            }
                        } else {
                            if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
                                if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                    ((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 116, 104, 101, 110, 32, 40, 49, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 41]))
                                } else {
                                    ((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 116, 104, 101, 110, 32, 49, 37, 102, 108, 111, 97, 116, 32, 101, 108, 115, 101, 32, 48, 37, 102, 108, 111, 97, 116, 41]))
                                }
                            } else {
                                if ((form == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (form == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
                                    ((self::ml_string::Text::from_units(vec![40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 73, 110, 116, 101, 103, 101, 114, 32])).concat(&if (form == (self::ml_string::Text::from_units(vec![110, 97, 116]))) {
                                        if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                            ((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![40, 90, 46, 111, 102, 95, 78, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                        }
                                    } else {
                                        value.clone()
                                    })).concat(&self::ml_string::Text::from_units(vec![41]))
                                } else {
                                    if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                        ((self::ml_string::Text::from_units(vec![40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 85, 110, 105, 116, 115, 32, 40, 83, 116, 114, 105, 110, 103, 46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32, 40, 117, 110, 105, 116, 115, 32, 58, 32, 65, 114, 114, 97, 121, 32, 70, 108, 111, 97, 116, 41, 32, 99, 32, 61, 62, 32, 105, 102, 32, 99, 46, 116, 111, 78, 97, 116, 32, 60, 32, 54, 53, 53, 51, 54, 32, 116, 104, 101, 110, 32, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 99, 46, 116, 111, 78, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 53, 50, 57, 54, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 47, 32, 49, 48, 50, 52, 41, 41, 41, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 54, 51, 50, 48, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 37, 32, 49, 48, 50, 52, 41, 41, 41, 32, 35, 91, 93, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                    } else {
                                        ((self::ml_string::Text::from_units(vec![40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 85, 116, 102, 56, 32, 40, 40, 102, 105, 120, 32, 109, 108, 95, 110, 117, 109, 101, 114, 105, 99, 95, 98, 121, 116, 101, 115, 32, 40, 116, 101, 120, 116, 32, 58, 32, 115, 116, 114, 105, 110, 103, 41, 32, 58, 32, 108, 105, 115, 116, 32, 102, 108, 111, 97, 116, 32, 58, 61, 32, 109, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 119, 105, 116, 104, 32, 69, 109, 112, 116, 121, 83, 116, 114, 105, 110, 103, 32, 61, 62, 32, 110, 105, 108, 32, 124, 32, 83, 116, 114, 105, 110, 103, 32, 99, 32, 114, 101, 115, 116, 32, 61, 62, 32, 99, 111, 110, 115, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54, 51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102, 95, 110, 97, 116, 32, 40, 65, 115, 99, 105, 105, 46, 110, 97, 116, 95, 111, 102, 95, 97, 115, 99, 105, 105, 32, 99, 41, 41, 41, 41, 32, 40, 109, 108, 95, 110, 117, 109, 101, 114, 105, 99, 95, 98, 121, 116, 101, 115, 32, 114, 101, 115, 116, 41, 32, 101, 110, 100, 41, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if (format != (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    if (form == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                        value.clone()
                    } else {
                        if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
                            ((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 49, 46, 48, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 48, 46, 48, 102, 54, 52, 32, 125, 41]))
                        } else {
                            if (form == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                                ((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 95, 32, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 102, 54, 52, 58, 58, 78, 65, 78, 32, 125]))
                            } else {
                                if ((form == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (form == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
                                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) {
                                        ((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 95, 32, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 111, 117, 114, 99, 101, 95, 101, 120, 99, 101, 112, 116, 105, 111, 110, 58, 58, 83, 111, 117, 114, 99, 101, 69, 120, 99, 101, 112, 116, 105, 111, 110, 58, 58, 102, 114, 111, 109, 95, 116, 101, 120, 116, 40, 34, 84, 121, 112, 101, 69, 114, 114, 111, 114, 34, 44, 32, 34, 67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114, 34, 41, 46, 114, 97, 105, 115, 101, 40, 41, 32, 125]))
                                    } else {
                                        ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 110, 117, 109, 98, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                    }
                                } else {
                                    {
                                        let units = if utf16 {
                                            ((self::ml_string::Text::from_units(vec![40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 46, 48, 46, 105, 116, 101, 114, 40, 41, 46, 99, 111, 112, 105, 101, 100, 40, 41]))
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 46, 101, 110, 99, 111, 100, 101, 95, 117, 116, 102, 49, 54, 40, 41]))
                                        };
                                        ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 95, 117, 110, 105, 116, 115, 40, 38])).concat(&units)).concat(&self::ml_string::Text::from_units(vec![46, 109, 97, 112, 40, 102, 54, 52, 58, 58, 102, 114, 111, 109, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 102, 54, 52, 62, 62, 40, 41, 41]))
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

pub fn read_java_script_number_conversion_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![69, 67, 77, 65, 83, 99, 114, 105, 112, 116, 32, 78, 117, 109, 98, 101, 114, 32, 97, 110, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 112, 114, 101, 115, 101, 114, 118, 101, 32, 102, 117, 108, 108, 32, 85, 84, 70, 45, 49, 54, 32, 83, 116, 114, 105, 110, 103, 78, 117, 109, 101, 114, 105, 99, 76, 105, 116, 101, 114, 97, 108, 32, 115, 121, 110, 116, 97, 120, 44, 32, 101, 120, 97, 99, 116, 32, 98, 105, 110, 97, 114, 121, 54, 52, 32, 116, 105, 101, 115, 45, 116, 111, 45, 101, 118, 101, 110, 32, 114, 111, 117, 110, 100, 105, 110, 103, 44, 32, 115, 105, 103, 110, 101, 100, 32, 122, 101, 114, 111, 44, 32, 73, 110, 102, 105, 110, 105, 116, 121, 44, 32, 78, 97, 78, 44, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 97, 98, 115, 101, 110, 99, 101, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 101, 118, 97, 108, 117, 97, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114, 59, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 111, 117, 110, 100, 115, 32, 111, 110, 99, 101, 32, 97, 110, 100, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 116, 104, 114, 111, 119, 115])
}

pub fn convert_java_script_string_big_integer_units(units: Vec<f64>) -> Vec<self::ml::Big> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s134 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop41(units.clone(), index);
            match source_ml_source2_s134.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop41Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop41Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop42(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop43(units.clone(), start_2, end);
                                    if (start_2 == end_2) {
                                        vec![self::ml::Big::from_i128(0)]
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(start_2))).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let source_ml_source2_s135 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = (start_2 + 1f64);
                                                                        self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin44::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin44Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin44::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin44Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s135.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin44::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin44Next(start_4, negative_3, signed_3) => {
                                                                if (start_4 == end_2) {
                                                                    Vec::<self::ml::Big>::new()
                                                                } else {
                                                                    {
                                                                        let radix = 10f64;
                                                                        {
                                                                            let base = self::ml::Big::from_i128(10);
                                                                            {
                                                                                let source_ml_source2_s136 = if ((!signed_3 && (((self::ml_array::optional_at(&units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
                                                                                    {
                                                                                        let marker = (self::ml_array::optional_at(&units, self::ml_array::number_index((start_4 + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                                                                                        {
                                                                                            let selected = if ((marker == (120f64)) || (marker == (88f64))) {
                                                                                                16f64
                                                                                            } else {
                                                                                                if ((marker == (98f64)) || (marker == (66f64))) {
                                                                                                    2f64
                                                                                                } else {
                                                                                                    if ((marker == (111f64)) || (marker == (79f64))) {
                                                                                                        8f64
                                                                                                    } else {
                                                                                                        0f64
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            if (selected != (0f64)) {
                                                                                                {
                                                                                                    let radix_2 = selected;
                                                                                                    {
                                                                                                        let base_2 = if (selected == (16f64)) {
                                                                                                            self::ml::Big::from_i128(16)
                                                                                                        } else {
                                                                                                            if (selected == (8f64)) {
                                                                                                                self::ml::Big::from_i128(8)
                                                                                                            } else {
                                                                                                                self::ml::Big::from_i128(2)
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let start_5 = (start_4 + 2f64);
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45Next(start_5, radix_2, base_2.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45Next(start_4, radix, base.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45Next(start_4, radix, base.clone())
                                                                                };
                                                                                match source_ml_source2_s136.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45Next(start_6, radix_3, base_3) => {
                                                                                        if (start_6 == end_2) {
                                                                                            Vec::<self::ml::Big>::new()
                                                                                        } else {
                                                                                            {
                                                                                                let digits = vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)];
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let index_2 = start_6;
                                                                                                        {
                                                                                                            let source_ml_source2_s137 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop46(units, end_2, radix_3, base_3.clone(), digits, value.clone(), index_2);
                                                                                                            match source_ml_source2_s137.clone() {
                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop46Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop46Done(value_2) => {
                                                                                                                    vec![if negative_3 {
                                                                                                                        value_2.neg()
                                                                                                                    } else {
                                                                                                                        value_2.clone()
                                                                                                                    }]
                                                                                                                }
                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop46Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop46Return(source_ml_source1_temporary_result46) => {
                                                                                                                    source_ml_source1_temporary_result46
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
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop41Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop41Return(source_ml_source1_temporary_result46_2) => {
                    source_ml_source1_temporary_result46_2
                }
            }
        }
    }
}

pub fn convert_java_script_utf8_big_integer(bytes: Vec<f64>) -> Vec<self::ml::Big> {
    self::convert_java_script_string_big_integer_units(self::read_java_script_utf8_string_units(bytes))
}

pub fn convert_java_script_number_big_integer(value: f64) -> Vec<self::ml::Big> {
    if !(self::ml_math::is_integer(value)) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let negative = (value < (0f64));
            {
                let value_2 = value.abs();
                {
                    let integer = self::ml::Big::from_i128(0);
                    {
                        let weight = self::ml::Big::from_i128(1);
                        {
                            let integer_2 = self::source_ml_source1_temporary_convert_java_script_number_big_integer_loop47(value_2, integer.clone(), weight.clone());
                            vec![if negative {
                                integer_2.neg()
                            } else {
                                integer_2.clone()
                            }]
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_boolean_big_integer(value: bool) -> self::ml::Big {
    if value {
        self::ml::Big::from_i128(1)
    } else {
        self::ml::Big::from_i128(0)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_java_script_number_big_integer(value: f64, integer: self::ml::Big) -> f64 {
    if value.is_nan() {
        2f64
    } else {
        if (value == (f64::INFINITY)) {
            1f64
        } else {
            if (value == (f64::NEG_INFINITY)) {
                (-1f64)
            } else {
                {
                    let floor = value.floor();
                    {
                        let lower = (self::ml_array::optional_at(&self::convert_java_script_number_big_integer(floor), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                        if (lower < integer) {
                            (-1f64)
                        } else {
                            if (lower > integer) {
                                1f64
                            } else {
                                if (value == floor) {
                                    0f64
                                } else {
                                    1f64
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn compare_java_script_string_big_integer_units(units: Vec<f64>, integer: self::ml::Big) -> f64 {
    {
        let result = self::convert_java_script_string_big_integer_units(units);
        if (((result.len() as f64)) != (1f64)) {
            2f64
        } else {
            {
                let value = (self::ml_array::optional_at(&result, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                if (value < integer) {
                    (-1f64)
                } else {
                    if (value > integer) {
                        1f64
                    } else {
                        0f64
                    }
                }
            }
        }
    }
}

pub fn compare_java_script_utf8_big_integer(bytes: Vec<f64>, integer: self::ml::Big) -> f64 {
    self::compare_java_script_string_big_integer_units(self::read_java_script_utf8_string_units(bytes), integer.clone())
}

pub fn divide_java_script_big_integer(left: self::ml::Big, right: self::ml::Big) -> Vec<self::ml::Big> {
    if (right == (self::ml::Big::from_i128(0))) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let remaining = if (left < (self::ml::Big::from_i128(0))) {
                left.neg()
            } else {
                left.clone()
            };
            {
                let divisor = if (right < (self::ml::Big::from_i128(0))) {
                    right.neg()
                } else {
                    right.clone()
                };
                {
                    let shifted = divisor.clone();
                    {
                        let weight = self::ml::Big::from_i128(1);
                        {
                            let quotient = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s138 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop48(remaining.clone(), shifted.clone(), weight.clone());
                                match source_ml_source2_s138.clone() {
                                    self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Done(shifted_2, weight_2) => {
                                        {
                                            let source_ml_source2_s139 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop49(remaining.clone(), shifted_2.clone(), weight_2.clone(), quotient.clone());
                                            match source_ml_source2_s139.clone() {
                                                self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Done(remaining_2, quotient_2) => {
                                                    vec![if (((left < (self::ml::Big::from_i128(0)))) != ((right < (self::ml::Big::from_i128(0))))) {
                                                        quotient_2.neg()
                                                    } else {
                                                        quotient_2.clone()
                                                    }, if (left < (self::ml::Big::from_i128(0))) {
                                                        remaining_2.neg()
                                                    } else {
                                                        remaining_2.clone()
                                                    }]
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

pub fn power_java_script_big_integer(base: self::ml::Big, exponent: self::ml::Big) -> Vec<self::ml::Big> {
    if (exponent < (self::ml::Big::from_i128(0))) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let result = self::ml::Big::from_i128(1);
            {
                let weight = base.clone();
                {
                    let remaining = exponent.clone();
                    {
                        let result_2 = self::source_ml_source1_temporary_power_java_script_big_integer_loop50(result.clone(), weight.clone(), remaining.clone());
                        vec![result_2.clone()]
                    }
                }
            }
        }
    }
}

pub fn normalize_number_power_bound(value: self::ml::Big, exponent: self::ml::Big, scale: self::ml::Big, upward: bool) -> Vec<self::ml::Big> {
    if (value <= (self::ml::Big::from_i128(0))) {
        vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(0)]
    } else {
        {
            let source_ml_source2_s140 = self::source_ml_source1_temporary_normalize_number_power_bound_loop51(value.clone(), exponent.clone(), scale.clone(), upward);
            match source_ml_source2_s140.clone() {
                self::MlSource1TemporaryNormalizeNumberPowerBoundLoop51Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop51Done(value_2, exponent_2) => {
                    {
                        let source_ml_source2_s141 = self::source_ml_source1_temporary_normalize_number_power_bound_loop52(value_2.clone(), exponent_2.clone(), scale.clone());
                        match source_ml_source2_s141.clone() {
                            self::MlSource1TemporaryNormalizeNumberPowerBoundLoop52Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop52Done(value_3, exponent_3) => {
                                vec![value_3.clone(), exponent_3.clone()]
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
pub fn divide_number_power_bound(numerator: self::ml::Big, denominator: self::ml::Big, upward: bool) -> self::ml::Big {
    {
        let result = self::divide_java_script_big_integer(numerator.clone(), denominator.clone());
        {
            let quotient = (self::ml_array::optional_at(&result, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
            {
                let remainder = (self::ml_array::optional_at(&result, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                quotient.add(&if (upward && (remainder != (self::ml::Big::from_i128(0)))) {
                    self::ml::Big::from_i128(1)
                } else {
                    self::ml::Big::from_i128(0)
                })
            }
        }
    }
}

pub fn root_number_power_integer(value: self::ml::Big) -> self::ml::Big {
    if (value <= (self::ml::Big::from_i128(0))) {
        self::ml::Big::from_i128(0)
    } else {
        {
            let root = self::ml::Big::from_i128(1);
            {
                let remaining = value.clone();
                {
                    let root_2 = self::source_ml_source1_temporary_root_number_power_integer_loop53(root.clone(), remaining.clone());
                    {
                        let root_3 = root_2.mul(&self::ml::Big::from_i128(2));
                        self::source_ml_source1_temporary_root_number_power_integer_loop54(value.clone(), root_3.clone())
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn multiply_number_power_interval(left: Vec<self::ml::Big>, right: Vec<self::ml::Big>, scale: self::ml::Big) -> Vec<self::ml::Big> {
    {
        let lower = self::normalize_number_power_bound(self::divide_number_power_bound(((self::ml_array::optional_at(&left, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).mul(&(self::ml_array::optional_at(&right, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), scale.clone(), false), ((self::ml_array::optional_at(&left, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).add(&(self::ml_array::optional_at(&right, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), scale.clone(), false);
        {
            let upper = self::normalize_number_power_bound(self::divide_number_power_bound(((self::ml_array::optional_at(&left, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).mul(&(self::ml_array::optional_at(&right, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), scale.clone(), true), ((self::ml_array::optional_at(&left, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).add(&(self::ml_array::optional_at(&right, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), scale.clone(), true);
            vec![(self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))]
        }
    }
}

pub fn root_number_power_bound(value: self::ml::Big, exponent: self::ml::Big, scale: self::ml::Big, upward: bool) -> Vec<self::ml::Big> {
    {
        let source_ml_source2_s142 = if ((self::ml::divide(&exponent, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) != (self::ml::Big::from_i128(0))) {
            {
                let value_2 = value.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent.sub(&self::ml::Big::from_i128(1));
                    self::MlSource1TemporaryRootNumberPowerBoundJoin55::MlSource1TemporaryRootNumberPowerBoundJoin55Next(value_2.clone(), exponent_2.clone())
                }
            }
        } else {
            self::MlSource1TemporaryRootNumberPowerBoundJoin55::MlSource1TemporaryRootNumberPowerBoundJoin55Next(value.clone(), exponent.clone())
        };
        match source_ml_source2_s142.clone() {
            self::MlSource1TemporaryRootNumberPowerBoundJoin55::MlSource1TemporaryRootNumberPowerBoundJoin55Next(value_3, exponent_3) => {
                {
                    let square = value_3.mul(&scale);
                    {
                        let root = self::root_number_power_integer(square.clone());
                        {
                            let root_3 = if (upward && ((root.mul(&root)) < square)) {
                                {
                                    let root_2 = root.add(&self::ml::Big::from_i128(1));
                                    root_2.clone()
                                }
                            } else {
                                root.clone()
                            };
                            self::normalize_number_power_bound(root_3.clone(), self::ml::divide(&exponent_3, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false), scale.clone(), upward)
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn root_number_power_interval(interval: Vec<self::ml::Big>, scale: self::ml::Big) -> Vec<self::ml::Big> {
    {
        let lower = self::root_number_power_bound((self::ml_array::optional_at(&interval, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&interval, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), scale.clone(), false);
        {
            let upper = self::root_number_power_bound((self::ml_array::optional_at(&interval, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&interval, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), scale.clone(), true);
            vec![(self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn invert_number_power_interval(interval: Vec<self::ml::Big>, scale: self::ml::Big) -> Vec<self::ml::Big> {
    {
        let lower = self::normalize_number_power_bound(self::divide_number_power_bound(scale.mul(&scale), (self::ml_array::optional_at(&interval, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), false), ((self::ml_array::optional_at(&interval, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).neg(), scale.clone(), false);
        {
            let upper = self::normalize_number_power_bound(self::divide_number_power_bound(scale.mul(&scale), (self::ml_array::optional_at(&interval, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), true), ((self::ml_array::optional_at(&interval, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).neg(), scale.clone(), true);
            vec![(self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))]
        }
    }
}

pub fn decompose_number_power_magnitude(value: f64) -> Vec<self::ml::Big> {
    {
        let exponent = self::ml::Big::from_i128(0);
        {
            let source_ml_source2_s143 = self::source_ml_source1_temporary_decompose_number_power_magnitude_loop56(value, exponent.clone());
            match source_ml_source2_s143.clone() {
                self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop56Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop56Done(value_2, exponent_2) => {
                    {
                        let source_ml_source2_s144 = self::source_ml_source1_temporary_decompose_number_power_magnitude_loop57(value_2, exponent_2.clone());
                        match source_ml_source2_s144.clone() {
                            self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop57Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop57Done(value_3, exponent_3) => {
                                vec![(self::ml_array::optional_at(&self::convert_java_script_number_big_integer((value_3 * 4503599627370496f64)), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), exponent_3.sub(&self::ml::Big::from_i128(52))]
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
pub fn round_number_power_bound(value: self::ml::Big, exponent: self::ml::Big, precision: self::ml::Big) -> f64 {
    if ((value == (self::ml::Big::from_i128(0))) || (exponent < (self::ml::Big::from_i128(-1076)))) {
        0f64
    } else {
        if (exponent > (self::ml::Big::from_i128(1024))) {
            f64::INFINITY
        } else {
            {
                let shift = exponent.sub(&precision);
                if (shift >= (self::ml::Big::from_i128(0))) {
                    self::convert_string_number_rational(value.mul(&(self::ml_array::optional_at(&self::power_java_script_big_integer(self::ml::Big::from_i128(2), shift.clone()), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), self::ml::Big::from_i128(1))
                } else {
                    self::convert_string_number_rational(value.clone(), (self::ml_array::optional_at(&self::power_java_script_big_integer(self::ml::Big::from_i128(2), shift.neg()), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)))
                }
            }
        }
    }
}

pub fn power_java_script_number(base: f64, exponent: f64) -> f64 {
    if exponent.is_nan() {
        f64::NAN
    } else {
        if (exponent == (0f64)) {
            1f64
        } else {
            if base.is_nan() {
                f64::NAN
            } else {
                {
                    let magnitude = base.abs();
                    if !(exponent.is_finite()) {
                        if (magnitude == (1f64)) {
                            f64::NAN
                        } else {
                            if (magnitude > (1f64)) {
                                if (exponent > (0f64)) {
                                    f64::INFINITY
                                } else {
                                    0f64
                                }
                            } else {
                                if (exponent > (0f64)) {
                                    0f64
                                } else {
                                    f64::INFINITY
                                }
                            }
                        }
                    } else {
                        {
                            let odd = (self::ml_math::is_integer(exponent) && (((exponent.abs() % 2f64)) == (1f64)));
                            {
                                let negative = (((base < (0f64)) || ((base == (0f64)) && (((1f64 / base)) < (0f64)))) && odd);
                                if (base == (0f64)) {
                                    {
                                        let value = if (exponent > (0f64)) {
                                            0f64
                                        } else {
                                            f64::INFINITY
                                        };
                                        if negative {
                                            (-value)
                                        } else {
                                            value
                                        }
                                    }
                                } else {
                                    if !(base.is_finite()) {
                                        {
                                            let value_2 = if (exponent > (0f64)) {
                                                f64::INFINITY
                                            } else {
                                                0f64
                                            };
                                            if negative {
                                                (-value_2)
                                            } else {
                                                value_2
                                            }
                                        }
                                    } else {
                                        if ((base < (0f64)) && !(self::ml_math::is_integer(exponent))) {
                                            f64::NAN
                                        } else {
                                            if (magnitude == (1f64)) {
                                                if negative {
                                                    (-1f64)
                                                } else {
                                                    1f64
                                                }
                                            } else {
                                                {
                                                    let input = self::decompose_number_power_magnitude(magnitude);
                                                    {
                                                        let power = self::decompose_number_power_magnitude(exponent.abs());
                                                        {
                                                            let precision = self::ml::Big::from_i128(192);
                                                            self::source_ml_source1_temporary_power_java_script_number_loop58(exponent, negative, input, power, precision.clone())
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

pub fn accept_java_script_exponentiation_base(prefix: self::ml_string::Text, parenthesized: bool) -> bool {
    (parenthesized || !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![43]), self::ml_string::Text::from_units(vec![45]), self::ml_string::Text::from_units(vec![33]), self::ml_string::Text::from_units(vec![126]), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 111, 102]), self::ml_string::Text::from_units(vec![118, 111, 105, 100]), self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101]), self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116])], &prefix.clone(), 0f64, false, |a, b| a == b)))
}

pub fn read_java_script_big_integer_exponent_message() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![69, 120, 112, 111, 110, 101, 110, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101])
}

pub fn render_java_script_big_integer_power(target: self::ml_string::Text, base: self::ml_string::Text, exponent: self::ml_string::Text) -> self::ml_string::Text {
    if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        ((((self::ml_string::Text::from_units(vec![109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 112, 111, 119, 101, 114, 40])).concat(&base)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&exponent)).concat(&self::ml_string::Text::from_units(vec![41]))
    } else {
        if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
            ((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 112, 111, 119, 101, 114, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40])).concat(&base)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&exponent)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if ((target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                ((((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 112, 111, 119, 101, 114, 32])).concat(&base)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&exponent)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn accept_nonzero_java_script_integer_literal(value: self::ml_string::Text) -> bool {
    (((value != (self::ml_string::Text::from_units(vec![]))) && (value != (self::ml_string::Text::from_units(vec![48])))) && (value != (self::ml_string::Text::from_units(vec![45, 48]))))
}

pub fn require_java_script_big_integer_division(language: self::ml_string::Text, operation: self::ml_string::Text, kind: self::ml_string::Text, literal: self::ml_string::Text) -> bool {
    ((((language == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) && (kind == (self::ml_string::Text::from_units(vec![105, 110, 116])))) && ((operation == (self::ml_string::Text::from_units(vec![100, 105, 118]))) || (operation == (self::ml_string::Text::from_units(vec![114, 101, 109]))))) && !(self::accept_nonzero_java_script_integer_literal(literal.clone())))
}

pub fn read_java_script_big_integer_division_message() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111])
}

pub fn render_java_script_big_integer_division(target: self::ml_string::Text, left: self::ml_string::Text, right: self::ml_string::Text) -> self::ml_string::Text {
    if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        ((((self::ml_string::Text::from_units(vec![109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 100, 105, 118, 105, 100, 101, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
    } else {
        if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
            ((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 100, 105, 118, 105, 100, 101, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if ((target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                ((((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 100, 105, 118, 105, 100, 101, 32])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_java_script_big_integer_conversion_form(kind: self::ml_string::Text) -> self::ml_string::Text {
    if ((kind == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else {
        if (kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
            self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
        } else {
            if (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
            } else {
                if (kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                } else {
                    if (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                        self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
                    } else {
                        if (kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) {
                            self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
                        } else {
                            self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100])
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_big_integer_conversion_error_class(form: self::ml_string::Text) -> self::ml_string::Text {
    if (form == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
        self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
    } else {
        if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
            self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
        } else {
            if ((form == (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))) || (form == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))) {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}
