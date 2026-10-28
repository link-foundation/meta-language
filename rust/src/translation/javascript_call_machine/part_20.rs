// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_call_primitive_arity_error;
use super::read_java_script_call_registers;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop132;
use super::test_java_script_call_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop131(mut program: self::JavaScriptCallProgram, mut heap: self::JavaScriptOrdinaryHeap, mut fn_: self::JavaScriptCallFunction, mut source_ml_source1_temporary_values126: Vec<self::JavaScriptCallInstruction>, mut source_ml_source1_temporary_index126: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result {
    loop {
        return match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, shared_constructor_field2) => {
                match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(_, shared_constructor_field1, _) => {
                        if (source_ml_source1_temporary_index126 < (self::ml::Big::from_u128(source_ml_source1_temporary_values126.len() as u128))) {
                            {
                                let instruction = self::ml_array::at(&source_ml_source1_temporary_values126, source_ml_source1_temporary_index126.to_index());
                                {
                                    let source_ml_source1_temporary_values127 = self::read_java_script_call_registers(instruction.clone());
                                    {
                                        let source_ml_source1_temporary_index127 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s2314 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop132(fn_.clone(), &(source_ml_source1_temporary_values127), source_ml_source1_temporary_index127.clone());
                                            match source_ml_source2_s2314.clone() {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Done => {
                                                    match instruction.clone() {
                                                        self::JavaScriptCallInstruction::Constant(_, value) => {
                                                            let value = *value;
                                                            if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 110, 115, 116, 97, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index126_2 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_2.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Copy(_, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index126_3 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_3.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Argument(_, index) => {
                                                            if !(self::test_java_script_call_coordinate(index, 1048576f64)) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 97, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index126_4 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_4.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Jump(target) => {
                                                            if !(self::test_java_script_call_coordinate(target, (shared_constructor_field2.len() as f64))) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 106, 117, 109, 112, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index126_5 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_5.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Branch(_, yes, no) => {
                                                            if (!(self::test_java_script_call_coordinate(yes, (shared_constructor_field2.len() as f64))) || !(self::test_java_script_call_coordinate(no, (shared_constructor_field2.len() as f64)))) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 98, 114, 97, 110, 99, 104, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 98, 111, 116, 104, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index126_6 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_6.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index126_7 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_7.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Return(_) => {
                                                            {
                                                                let source_ml_source1_temporary_index126_8 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_8.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Throw(_) => {
                                                            {
                                                                let source_ml_source1_temporary_index126_9 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_9.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::PushHandler(_, target_2) => {
                                                            if !(self::test_java_script_call_coordinate(target_2, (shared_constructor_field2.len() as f64))) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index126_10 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_10.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::PopHandler => {
                                                            {
                                                                let source_ml_source1_temporary_index126_11 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_11.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::NumberBinary(_, _, _, operator) => {
                                                            if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 100, 100]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101]), self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]), self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index126_12 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_12.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index126_13 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_13.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index126_14 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_14.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                            {
                                                                let source_ml_source1_temporary_index126_15 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_15.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Primitive(_, operation, arguments) => {
                                                            {
                                                                let error = self::read_java_script_call_primitive_arity_error(&(operation), (arguments.len() as f64));
                                                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(error.clone())
                                                                } else {
                                                                    {
                                                                        let source_ml_source1_temporary_index126_16 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                        {
                                                                            source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_16.clone();
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index126_17 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_17.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index126_18 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_18.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::PropertyKey(_, _, realm) => {
                                                            if !(self::ml_array::includes(&shared_constructor_field1, &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index126_19 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_19.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, realm_2) => {
                                                            if !(self::ml_array::includes(&shared_constructor_field1, &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index126_20 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_20.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Return(source_ml_source1_temporary_result132) => {
                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(source_ml_source1_temporary_result132.clone())
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Done
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop130(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values125: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index125: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result {
    loop {
        return if (source_ml_source1_temporary_index125 < (self::ml::Big::from_u128(source_ml_source1_temporary_values125.len() as u128))) {
            {
                let capture = self::ml_array::at(&source_ml_source1_temporary_values125, source_ml_source1_temporary_index125.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), capture.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index125_2 = source_ml_source1_temporary_index125.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index125 = source_ml_source1_temporary_index125_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop129(mut program: self::JavaScriptCallProgram, mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut count: f64, mut source_ml_source1_temporary_values124: Vec<self::JavaScriptCallFunction>, mut source_ml_source1_temporary_index124: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result {
    loop {
        return if (source_ml_source1_temporary_index124 < (self::ml::Big::from_u128(source_ml_source1_temporary_values124.len() as u128))) {
            {
                let fn_ = self::ml_array::at(&source_ml_source1_temporary_values124, source_ml_source1_temporary_index124.to_index());
                if (((match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2362, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value2362.clone()
                    }
                }) <= previous) || ((match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2364, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value2364.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2365, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value2365.clone()
                            }
                        };
                        if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value2367, _) => source_ml_source2_data_property_value2367.as_slice() }), &match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value2368, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value2368.clone()
                            }
                        }, 0f64, false, |a, b| a == b)) {
                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))
                        } else {
                            {
                                let callable = self::read_java_script_ordinary_value_error(heap.clone(), self::JavaScriptSemanticValue::Object(match fn_.clone() {
                                    self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2369, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value2369.clone()
                                    }
                                }, true));
                                if (callable != (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(callable.clone())
                                } else {
                                    if (((match fn_.clone() {
                                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value2371, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value2371.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) && ((match fn_.clone() {
                                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value2373, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value2373.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])))) {
                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 105, 99, 116, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 98, 105, 110, 100, 105, 110, 103]))
                                    } else {
                                        if ((self::read_java_script_ordinary_value_error(heap.clone(), match fn_.clone() {
                                            self::JavaScriptCallFunction::CallFunction(_, _, _, source_ml_source2_data_property_value2375, _, _, _, _) => {
                                                let source_ml_source2_data_property_value2375 = *source_ml_source2_data_property_value2375;
                                                source_ml_source2_data_property_value2375.clone()
                                            }
                                        })) != (self::ml_string::Text::from_units(vec![]))) {
                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                                        } else {
                                            if (((!(self::test_java_script_call_coordinate(match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2376, _, _) => {
                                                    source_ml_source2_data_property_value2376
                                                }
                                            }, 4097f64)) || ((match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2378, _, _) => {
                                                    source_ml_source2_data_property_value2378
                                                }
                                            }) == (0f64))) || !(self::test_java_script_call_coordinate(match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source2_data_property_value2379, _, _, _) => {
                                                    source_ml_source2_data_property_value2379
                                                }
                                            }, match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2380, _, _) => {
                                                    source_ml_source2_data_property_value2380
                                                }
                                            }))) || ((((match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source2_data_property_value2384, _, _, _) => {
                                                    source_ml_source2_data_property_value2384
                                                }
                                            } + ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source2_data_property_value2385, _) => source_ml_source2_data_property_value2385.as_slice() }).len() as f64)) + 1f64)) > (match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2386, _, _) => {
                                                    source_ml_source2_data_property_value2386
                                                }
                                            }))) {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_values125 = match fn_.clone() {
                                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source2_data_property_value2387, _) => {
                                                            source_ml_source2_data_property_value2387
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_temporary_index125 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s2388 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop130(heap.clone(), source_ml_source1_temporary_values125, source_ml_source1_temporary_index125.clone());
                                                            match source_ml_source2_s2388.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Done => {
                                                                    {
                                                                        let count_2 = (count + ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2389) => source_ml_source2_data_property_value2389.as_slice() }).len() as f64));
                                                                        if ((count_2 > (65536f64)) || ((((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2391) => source_ml_source2_data_property_value2391.as_slice() }).len() as f64)) == (0f64))) {
                                                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116, 32, 111, 114, 32, 104, 97, 115, 32, 110, 111, 32, 101, 110, 116, 114, 121]))
                                                                        } else {
                                                                            {
                                                                                let source_ml_source1_temporary_values126 = match fn_.clone() {
                                                                                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2392) => {
                                                                                        source_ml_source2_data_property_value2392
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_temporary_index126 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let source_ml_source2_s2393 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop131(program.clone(), heap.clone(), fn_.clone(), source_ml_source1_temporary_values126, source_ml_source1_temporary_index126.clone());
                                                                                        match source_ml_source2_s2393.clone() {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Done => {
                                                                                                {
                                                                                                    let source_ml_source1_temporary_index124_2 = source_ml_source1_temporary_index124.add(&self::ml::Big::from_i128(1));
                                                                                                    {
                                                                                                        (previous, count, source_ml_source1_temporary_index124) = (previous_2.clone(), count_2, source_ml_source1_temporary_index124_2.clone());
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(source_ml_source1_temporary_result131) => {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(source_ml_source1_temporary_result131.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Return(source_ml_source1_temporary_result132) => {
                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(source_ml_source1_temporary_result132.clone())
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
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop128(mut previous: self::ml::Big, mut source_ml_source1_temporary_values123: Vec<self::ml::Big>, mut source_ml_source1_temporary_index123: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result {
    loop {
        return if (source_ml_source1_temporary_index123 < (self::ml::Big::from_u128(source_ml_source1_temporary_values123.len() as u128))) {
            {
                let realm = self::ml_array::at(&source_ml_source1_temporary_values123, source_ml_source1_temporary_index123.to_index());
                if ((realm <= previous) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 97, 108, 109, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = realm.clone();
                        {
                            let source_ml_source1_temporary_index123_2 = source_ml_source1_temporary_index123.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_source1_temporary_index123) = (previous_2.clone(), source_ml_source1_temporary_index123_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Done(previous.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop133(mut fn_: self::JavaScriptCallFunction, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2395, _, _) => {
                source_ml_source2_data_property_value2395
            }
        })) {
            {
                let registers_2 = { let (ml_append_right, ml_append_left) = (vec![self::JavaScriptSemanticValue::Undefined], registers); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (registers, index) = (registers_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            registers
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop134(mut fn_: self::JavaScriptCallFunction, mut arguments_: Vec<self::JavaScriptSemanticValue>, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source2_data_property_value2397, _, _, _) => {
                source_ml_source2_data_property_value2397
            }
        })) {
            {
                let value = (self::ml_array::optional_at(&arguments_, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let slot = (index + 1f64);
                    {
                        let registers_2 = { let ml_values8 = registers; let ml_callback8 = { let slot = slot; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, position: f64| -> self::JavaScriptSemanticValue { if (position == slot) {
                            value.clone()
                        } else {
                            entry.clone()
                        } }) }; ml_values8.iter().cloned().enumerate().map(|(ml_index8, ml_value8)| ml_callback8(ml_value8, (ml_index8 as f64))).collect::<Vec<_>>() };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (registers, index) = (registers_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            registers
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop135(mut fn_: self::JavaScriptCallFunction, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, shared_constructor_field1, _, shared_constructor_field2, _) => {
                if (index < ((shared_constructor_field2.len() as f64))) {
                    {
                        let value = (self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        {
                            let slot = ((shared_constructor_field1 + 1f64) + index);
                            {
                                let registers_2 = { let ml_values9 = registers; let ml_callback9 = { let slot = slot; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, position: f64| -> self::JavaScriptSemanticValue { if (position == slot) {
                                    value.clone()
                                } else {
                                    entry.clone()
                                } }) }; ml_values9.iter().cloned().enumerate().map(|(ml_index9, ml_value9)| ml_callback9(ml_value9, (ml_index9 as f64))).collect::<Vec<_>>() };
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        (registers, index) = (registers_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    registers
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136 {
    MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136Next,
    MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result {
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Done,
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145 {
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Next,
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146 {
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next,
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144 {
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next,
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result {
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Done,
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop143(mut fn_: Option<self::JavaScriptCallFunction>, mut source_ml_source1_temporary_values139: Vec<self::JavaScriptCallHandler>, mut source_ml_source1_temporary_index139: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result {
    loop {
        return if (source_ml_source1_temporary_index139 < (self::ml::Big::from_u128(source_ml_source1_temporary_values139.len() as u128))) {
            {
                let handler = self::ml_array::at(&source_ml_source1_temporary_values139, source_ml_source1_temporary_index139.to_index());
                if (!(self::test_java_script_call_coordinate(match handler.clone() {
                    self::JavaScriptCallHandler::CallHandler(_, source_ml_source2_data_property_value2402) => {
                        source_ml_source2_data_property_value2402
                    }
                }, {
                    let source_ml_source2_nullish_field_receiver2403 = (fn_.clone(),);
                    if match (source_ml_source2_nullish_field_receiver2403.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present2404 = ((source_ml_source2_nullish_field_receiver2403.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present2404.clone() {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2405, _, _) => {
                                    source_ml_source2_data_property_value2405
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 115, 39, 41])).0, message_present: true }).raise()
                    }
                })) || !(self::test_java_script_call_coordinate(match handler.clone() {
                    self::JavaScriptCallHandler::CallHandler(source_ml_source2_data_property_value2406, _) => {
                        source_ml_source2_data_property_value2406
                    }
                }, (({
                    let source_ml_source2_nullish_field_receiver2407 = (fn_.clone(),);
                    if match (source_ml_source2_nullish_field_receiver2407.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present2408 = ((source_ml_source2_nullish_field_receiver2407.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present2408.clone() {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2409) => {
                                    source_ml_source2_data_property_value2409
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 39, 41])).0, message_present: true }).raise()
                    }
                }).len() as f64)))) {
                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index139_2 = source_ml_source1_temporary_index139.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index139 = source_ml_source1_temporary_index139_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result {
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Done,
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop142(mut machine: self::JavaScriptCallMachine, mut source_ml_source1_temporary_values138: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index138: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result {
    loop {
        return if (source_ml_source1_temporary_index138 < (self::ml::Big::from_u128(source_ml_source1_temporary_values138.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values138, source_ml_source1_temporary_index138.to_index());
                if ((self::read_java_script_ordinary_value_error(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value2411, _) => {
                        let source_ml_source2_data_property_value2411 = *source_ml_source2_data_property_value2411;
                        source_ml_source2_data_property_value2411.clone()
                    }
                }, value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index138_2 = source_ml_source1_temporary_index138.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index138 = source_ml_source1_temporary_index138_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Result {
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Done,
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop141(mut machine: self::JavaScriptCallMachine, mut source_ml_source1_temporary_values137: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index137: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Result {
    loop {
        return if (source_ml_source1_temporary_index137 < (self::ml::Big::from_u128(source_ml_source1_temporary_values137.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values137, source_ml_source1_temporary_index137.to_index());
                if ((self::read_java_script_ordinary_value_error(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value2413, _) => {
                        let source_ml_source2_data_property_value2413 = *source_ml_source2_data_property_value2413;
                        source_ml_source2_data_property_value2413.clone()
                    }
                }, value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index137_2 = source_ml_source1_temporary_index137.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index137 = source_ml_source1_temporary_index137_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Done
        };
    }
}
