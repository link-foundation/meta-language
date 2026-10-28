// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result;
use super::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result;
use super::MlTemporaryReadJavaScriptCallProgramErrorLoop42Result;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_call_registers;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_read_java_script_call_program_error_loop42;
use super::test_java_script_call_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_program_error_loop41(mut program: self::JavaScriptCallProgram, mut heap: self::JavaScriptOrdinaryHeap, mut fn_: self::JavaScriptCallFunction, mut source_ml_temporary_values36: Vec<self::JavaScriptCallInstruction>, mut source_ml_temporary_index36: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result {
    loop {
        return if (source_ml_temporary_index36 < (self::ml::Big::from_u128(source_ml_temporary_values36.len() as u128))) {
            {
                let instruction = self::ml_array::at(&source_ml_temporary_values36, source_ml_temporary_index36.to_index());
                {
                    let source_ml_temporary_values37 = self::read_java_script_call_registers(instruction.clone());
                    {
                        let source_ml_temporary_index37 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s1929 = self::source_ml_temporary_read_java_script_call_program_error_loop42(fn_.clone(), &(source_ml_temporary_values37), source_ml_temporary_index37.clone());
                            match source_ml_source1_s1929.clone() {
                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop42Result::MlTemporaryReadJavaScriptCallProgramErrorLoop42Done => {
                                    match instruction.clone() {
                                        self::JavaScriptCallInstruction::Constant(source_ml_source1_f1930, value) => {
                                            let value = *value;
                                            if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result::MlTemporaryReadJavaScriptCallProgramErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 110, 115, 116, 97, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index36_2 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index36 = source_ml_temporary_index36_2.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Copy(source_ml_source1_f1931, source_ml_source1_f1932) => {
                                            {
                                                let source_ml_temporary_index36_3 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index36 = source_ml_temporary_index36_3.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Argument(source_ml_source1_f1933, index) => {
                                            if !(self::test_java_script_call_coordinate(index, 1048576f64)) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result::MlTemporaryReadJavaScriptCallProgramErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 97, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index36_4 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index36 = source_ml_temporary_index36_4.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Jump(target) => {
                                            if !(self::test_java_script_call_coordinate(target, ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value1967) => source_ml_source1_data_property_value1967.as_slice() }).len() as f64))) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result::MlTemporaryReadJavaScriptCallProgramErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 106, 117, 109, 112, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index36_5 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index36 = source_ml_temporary_index36_5.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Branch(source_ml_source1_f1934, yes, no) => {
                                            if (!(self::test_java_script_call_coordinate(yes, ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value1968) => source_ml_source1_data_property_value1968.as_slice() }).len() as f64))) || !(self::test_java_script_call_coordinate(no, ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value1969) => source_ml_source1_data_property_value1969.as_slice() }).len() as f64)))) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result::MlTemporaryReadJavaScriptCallProgramErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 98, 114, 97, 110, 99, 104, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 98, 111, 116, 104, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index36_6 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index36 = source_ml_temporary_index36_6.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Call(source_ml_source1_f1935, source_ml_source1_f1936, source_ml_source1_f1937, source_ml_source1_f1938) => {
                                            {
                                                let source_ml_temporary_index36_7 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index36 = source_ml_temporary_index36_7.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Return(source_ml_source1_f1939) => {
                                            {
                                                let source_ml_temporary_index36_8 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index36 = source_ml_temporary_index36_8.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Throw(source_ml_source1_f1940) => {
                                            {
                                                let source_ml_temporary_index36_9 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index36 = source_ml_temporary_index36_9.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PushHandler(source_ml_source1_f1941, target_2) => {
                                            if !(self::test_java_script_call_coordinate(target_2, ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value1970) => source_ml_source1_data_property_value1970.as_slice() }).len() as f64))) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result::MlTemporaryReadJavaScriptCallProgramErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index36_10 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index36 = source_ml_temporary_index36_10.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PopHandler => {
                                            {
                                                let source_ml_temporary_index36_11 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index36 = source_ml_temporary_index36_11.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::NumberBinary(source_ml_source1_f1942, source_ml_source1_f1943, source_ml_source1_f1944, operator) => {
                                            if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 100, 100]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101]), self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]), self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result::MlTemporaryReadJavaScriptCallProgramErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index36_12 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index36 = source_ml_temporary_index36_12.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::SameValue(source_ml_source1_f1945, source_ml_source1_f1946, source_ml_source1_f1947) => {
                                            {
                                                let source_ml_temporary_index36_13 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index36 = source_ml_temporary_index36_13.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::StrictEqual(source_ml_source1_f1948, source_ml_source1_f1949, source_ml_source1_f1950) => {
                                            {
                                                let source_ml_temporary_index36_14 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index36 = source_ml_temporary_index36_14.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::RequireWrite(source_ml_source1_f1951) => {
                                            {
                                                let source_ml_temporary_index36_15 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index36 = source_ml_temporary_index36_15.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryGet(source_ml_source1_f1952, source_ml_source1_f1953, source_ml_source1_f1954, source_ml_source1_f1955) => {
                                            {
                                                let source_ml_temporary_index36_16 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index36 = source_ml_temporary_index36_16.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::OrdinarySet(source_ml_source1_f1956, source_ml_source1_f1957, source_ml_source1_f1958, source_ml_source1_f1959, source_ml_source1_f1960) => {
                                            {
                                                let source_ml_temporary_index36_17 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index36 = source_ml_temporary_index36_17.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PropertyKey(source_ml_source1_f1961, source_ml_source1_f1962, realm) => {
                                            if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value1972, _) => source_ml_source1_data_property_value1972.as_slice() }), &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result::MlTemporaryReadJavaScriptCallProgramErrorLoop41Return(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index36_18 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index36 = source_ml_temporary_index36_18.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source1_f1963, source_ml_source1_f1964, source_ml_source1_f1965, source_ml_source1_f1966, realm_2) => {
                                            if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value1974, _) => source_ml_source1_data_property_value1974.as_slice() }), &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result::MlTemporaryReadJavaScriptCallProgramErrorLoop41Return(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index36_19 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index36 = source_ml_temporary_index36_19.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop42Result::MlTemporaryReadJavaScriptCallProgramErrorLoop42Return(source_ml_temporary_result42) => {
                                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result::MlTemporaryReadJavaScriptCallProgramErrorLoop41Return(source_ml_temporary_result42.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result::MlTemporaryReadJavaScriptCallProgramErrorLoop41Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallProgramErrorLoop40Result {
    MlTemporaryReadJavaScriptCallProgramErrorLoop40Done,
    MlTemporaryReadJavaScriptCallProgramErrorLoop40Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_program_error_loop40(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_temporary_values35: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_index35: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallProgramErrorLoop40Result {
    loop {
        return if (source_ml_temporary_index35 < (self::ml::Big::from_u128(source_ml_temporary_values35.len() as u128))) {
            {
                let capture = self::ml_array::at(&source_ml_temporary_values35, source_ml_temporary_index35.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), capture.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop40Result::MlTemporaryReadJavaScriptCallProgramErrorLoop40Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_temporary_index35_2 = source_ml_temporary_index35.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index35 = source_ml_temporary_index35_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallProgramErrorLoop40Result::MlTemporaryReadJavaScriptCallProgramErrorLoop40Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_program_error_loop39(mut program: self::JavaScriptCallProgram, mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut count: f64, mut source_ml_temporary_values34: Vec<self::JavaScriptCallFunction>, mut source_ml_temporary_index34: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result {
    loop {
        return if (source_ml_temporary_index34 < (self::ml::Big::from_u128(source_ml_temporary_values34.len() as u128))) {
            {
                let fn_ = self::ml_array::at(&source_ml_temporary_values34, source_ml_temporary_index34.to_index());
                if (((match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value1976, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value1976.clone()
                    }
                }) <= previous) || ((match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value1978, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value1978.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result::MlTemporaryReadJavaScriptCallProgramErrorLoop39Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value1979, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1979.clone()
                            }
                        };
                        if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value1981, _) => source_ml_source1_data_property_value1981.as_slice() }), &match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1982, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1982.clone()
                            }
                        }, 0f64, false, |a, b| a == b)) {
                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result::MlTemporaryReadJavaScriptCallProgramErrorLoop39Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))
                        } else {
                            {
                                let callable = self::read_java_script_ordinary_value_error(heap.clone(), self::JavaScriptSemanticValue::Object(match fn_.clone() {
                                    self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value1983, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value1983.clone()
                                    }
                                }, true));
                                if (callable != (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result::MlTemporaryReadJavaScriptCallProgramErrorLoop39Return(callable.clone())
                                } else {
                                    if (((match fn_.clone() {
                                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source1_data_property_value1985, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value1985.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) && ((match fn_.clone() {
                                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source1_data_property_value1987, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value1987.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])))) {
                                        self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result::MlTemporaryReadJavaScriptCallProgramErrorLoop39Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 105, 99, 116, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 98, 105, 110, 100, 105, 110, 103]))
                                    } else {
                                        if ((self::read_java_script_ordinary_value_error(heap.clone(), match fn_.clone() {
                                            self::JavaScriptCallFunction::CallFunction(_, _, _, source_ml_source1_data_property_value1989, _, _, _, _) => {
                                                let source_ml_source1_data_property_value1989 = *source_ml_source1_data_property_value1989;
                                                source_ml_source1_data_property_value1989.clone()
                                            }
                                        })) != (self::ml_string::Text::from_units(vec![]))) {
                                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result::MlTemporaryReadJavaScriptCallProgramErrorLoop39Return(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                                        } else {
                                            if (((!(self::test_java_script_call_coordinate(match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value1990, _, _) => {
                                                    source_ml_source1_data_property_value1990
                                                }
                                            }, 4097f64)) || ((match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value1992, _, _) => {
                                                    source_ml_source1_data_property_value1992
                                                }
                                            }) == (0f64))) || !(self::test_java_script_call_coordinate(match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source1_data_property_value1993, _, _, _) => {
                                                    source_ml_source1_data_property_value1993
                                                }
                                            }, match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value1994, _, _) => {
                                                    source_ml_source1_data_property_value1994
                                                }
                                            }))) || ((((match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source1_data_property_value1998, _, _, _) => {
                                                    source_ml_source1_data_property_value1998
                                                }
                                            } + ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source1_data_property_value1999, _) => source_ml_source1_data_property_value1999.as_slice() }).len() as f64)) + 1f64)) > (match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2000, _, _) => {
                                                    source_ml_source1_data_property_value2000
                                                }
                                            }))) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result::MlTemporaryReadJavaScriptCallProgramErrorLoop39Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101]))
                                            } else {
                                                {
                                                    let source_ml_temporary_values35 = match fn_.clone() {
                                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source1_data_property_value2001, _) => {
                                                            source_ml_source1_data_property_value2001
                                                        }
                                                    };
                                                    {
                                                        let source_ml_temporary_index35 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_s2002 = self::source_ml_temporary_read_java_script_call_program_error_loop40(heap.clone(), source_ml_temporary_values35, source_ml_temporary_index35.clone());
                                                            match source_ml_source1_s2002.clone() {
                                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop40Result::MlTemporaryReadJavaScriptCallProgramErrorLoop40Done => {
                                                                    {
                                                                        let count_2 = (count + ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2003) => source_ml_source1_data_property_value2003.as_slice() }).len() as f64));
                                                                        if ((count_2 > (65536f64)) || ((((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2005) => source_ml_source1_data_property_value2005.as_slice() }).len() as f64)) == (0f64))) {
                                                                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result::MlTemporaryReadJavaScriptCallProgramErrorLoop39Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116, 32, 111, 114, 32, 104, 97, 115, 32, 110, 111, 32, 101, 110, 116, 114, 121]))
                                                                        } else {
                                                                            {
                                                                                let source_ml_temporary_values36 = match fn_.clone() {
                                                                                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2006) => {
                                                                                        source_ml_source1_data_property_value2006
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_temporary_index36 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let source_ml_source1_s2007 = self::source_ml_temporary_read_java_script_call_program_error_loop41(program.clone(), heap.clone(), fn_.clone(), source_ml_temporary_values36, source_ml_temporary_index36.clone());
                                                                                        match source_ml_source1_s2007.clone() {
                                                                                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result::MlTemporaryReadJavaScriptCallProgramErrorLoop41Done => {
                                                                                                {
                                                                                                    let source_ml_temporary_index34_2 = source_ml_temporary_index34.add(&self::ml::Big::from_i128(1));
                                                                                                    {
                                                                                                        (previous, count, source_ml_temporary_index34) = (previous_2.clone(), count_2, source_ml_temporary_index34_2.clone());
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop41Result::MlTemporaryReadJavaScriptCallProgramErrorLoop41Return(source_ml_temporary_result41) => {
                                                                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result::MlTemporaryReadJavaScriptCallProgramErrorLoop39Return(source_ml_temporary_result41.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop40Result::MlTemporaryReadJavaScriptCallProgramErrorLoop40Return(source_ml_temporary_result42) => {
                                                                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result::MlTemporaryReadJavaScriptCallProgramErrorLoop39Return(source_ml_temporary_result42.clone())
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
            self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result::MlTemporaryReadJavaScriptCallProgramErrorLoop39Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallProgramErrorLoop38Result {
    MlTemporaryReadJavaScriptCallProgramErrorLoop38Done(self::ml::Big),
    MlTemporaryReadJavaScriptCallProgramErrorLoop38Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_program_error_loop38(mut previous: self::ml::Big, mut source_ml_temporary_values33: Vec<self::ml::Big>, mut source_ml_temporary_index33: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallProgramErrorLoop38Result {
    loop {
        return if (source_ml_temporary_index33 < (self::ml::Big::from_u128(source_ml_temporary_values33.len() as u128))) {
            {
                let realm = self::ml_array::at(&source_ml_temporary_values33, source_ml_temporary_index33.to_index());
                if ((realm <= previous) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop38Result::MlTemporaryReadJavaScriptCallProgramErrorLoop38Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 97, 108, 109, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = realm.clone();
                        {
                            let source_ml_temporary_index33_2 = source_ml_temporary_index33.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_temporary_index33) = (previous_2.clone(), source_ml_temporary_index33_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallProgramErrorLoop38Result::MlTemporaryReadJavaScriptCallProgramErrorLoop38Done(previous.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_call_frame_loop43(mut fn_: self::JavaScriptCallFunction, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2009, _, _) => {
                source_ml_source1_data_property_value2009
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
pub fn source_ml_temporary_create_java_script_call_frame_loop44(mut fn_: self::JavaScriptCallFunction, mut arguments_: Vec<self::JavaScriptSemanticValue>, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source1_data_property_value2011, _, _, _) => {
                source_ml_source1_data_property_value2011
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
pub fn source_ml_temporary_create_java_script_call_frame_loop45(mut fn_: self::JavaScriptCallFunction, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source1_data_property_value2013, _) => source_ml_source1_data_property_value2013.as_slice() }).len() as f64))) {
            {
                let value = (self::ml_array::optional_at((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source1_data_property_value2014, _) => source_ml_source1_data_property_value2014.as_slice() }), self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let slot = ((match fn_.clone() {
                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source1_data_property_value2015, _, _, _) => {
                            source_ml_source1_data_property_value2015
                        }
                    } + 1f64) + index);
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
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallMachineErrorLoop49Result {
    MlTemporaryReadJavaScriptCallMachineErrorLoop49Done,
    MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallMachineErrorJoin54 {
    MlTemporaryReadJavaScriptCallMachineErrorJoin54Next,
    MlTemporaryReadJavaScriptCallMachineErrorJoin54Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallMachineErrorJoin53 {
    MlTemporaryReadJavaScriptCallMachineErrorJoin53Next,
    MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallMachineErrorLoop52Result {
    MlTemporaryReadJavaScriptCallMachineErrorLoop52Done,
    MlTemporaryReadJavaScriptCallMachineErrorLoop52Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_machine_error_loop52(mut fn_: Option<self::JavaScriptCallFunction>, mut source_ml_temporary_values48: Vec<self::JavaScriptCallHandler>, mut source_ml_temporary_index48: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallMachineErrorLoop52Result {
    loop {
        return if (source_ml_temporary_index48 < (self::ml::Big::from_u128(source_ml_temporary_values48.len() as u128))) {
            {
                let handler = self::ml_array::at(&source_ml_temporary_values48, source_ml_temporary_index48.to_index());
                if (!(self::test_java_script_call_coordinate(match handler.clone() {
                    self::JavaScriptCallHandler::CallHandler(_, source_ml_source1_data_property_value2016) => {
                        source_ml_source1_data_property_value2016
                    }
                }, {
                    let source_ml_source1_nullish_field_receiver2017 = (fn_.clone(),);
                    if match (source_ml_source1_nullish_field_receiver2017.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present2018 = ((source_ml_source1_nullish_field_receiver2017.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present2018.clone() {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2019, _, _) => {
                                    source_ml_source1_data_property_value2019
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 115, 39, 41])).0, message_present: true }).raise()
                    }
                })) || !(self::test_java_script_call_coordinate(match handler.clone() {
                    self::JavaScriptCallHandler::CallHandler(source_ml_source1_data_property_value2020, _) => {
                        source_ml_source1_data_property_value2020
                    }
                }, (({
                    let source_ml_source1_nullish_field_receiver2021 = (fn_.clone(),);
                    if match (source_ml_source1_nullish_field_receiver2021.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present2022 = ((source_ml_source1_nullish_field_receiver2021.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present2022.clone() {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2023) => {
                                    source_ml_source1_data_property_value2023
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 39, 41])).0, message_present: true }).raise()
                    }
                }).len() as f64)))) {
                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop52Result::MlTemporaryReadJavaScriptCallMachineErrorLoop52Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                } else {
                    {
                        let source_ml_temporary_index48_2 = source_ml_temporary_index48.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index48 = source_ml_temporary_index48_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallMachineErrorLoop52Result::MlTemporaryReadJavaScriptCallMachineErrorLoop52Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallMachineErrorLoop51Result {
    MlTemporaryReadJavaScriptCallMachineErrorLoop51Done,
    MlTemporaryReadJavaScriptCallMachineErrorLoop51Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_machine_error_loop51(mut machine: self::JavaScriptCallMachine, mut source_ml_temporary_values47: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_index47: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallMachineErrorLoop51Result {
    loop {
        return if (source_ml_temporary_index47 < (self::ml::Big::from_u128(source_ml_temporary_values47.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values47, source_ml_temporary_index47.to_index());
                if ((self::read_java_script_ordinary_value_error(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value2025, _) => {
                        let source_ml_source1_data_property_value2025 = *source_ml_source1_data_property_value2025;
                        source_ml_source1_data_property_value2025.clone()
                    }
                }, value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop51Result::MlTemporaryReadJavaScriptCallMachineErrorLoop51Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_temporary_index47_2 = source_ml_temporary_index47.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index47 = source_ml_temporary_index47_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallMachineErrorLoop51Result::MlTemporaryReadJavaScriptCallMachineErrorLoop51Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallMachineErrorLoop50Result {
    MlTemporaryReadJavaScriptCallMachineErrorLoop50Done,
    MlTemporaryReadJavaScriptCallMachineErrorLoop50Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_machine_error_loop50(mut machine: self::JavaScriptCallMachine, mut source_ml_temporary_values46: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_index46: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallMachineErrorLoop50Result {
    loop {
        return if (source_ml_temporary_index46 < (self::ml::Big::from_u128(source_ml_temporary_values46.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values46, source_ml_temporary_index46.to_index());
                if ((self::read_java_script_ordinary_value_error(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value2027, _) => {
                        let source_ml_source1_data_property_value2027 = *source_ml_source1_data_property_value2027;
                        source_ml_source1_data_property_value2027.clone()
                    }
                }, value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop50Result::MlTemporaryReadJavaScriptCallMachineErrorLoop50Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_temporary_index46_2 = source_ml_temporary_index46.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index46 = source_ml_temporary_index46_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallMachineErrorLoop50Result::MlTemporaryReadJavaScriptCallMachineErrorLoop50Done
        };
    }
}
