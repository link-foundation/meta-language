// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_call_machine_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;
use super::throw_java_script_call_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_error_allocation(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, realm: self::ml::Big, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let error = self::read_java_script_call_machine_error(program.clone(), machine.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), error.clone())
                } else {
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(((shared_constructor_field2.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            if {
                                let source_ml_source2_s2281 = match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2300) => {
                                        let source_ml_source2_data_property_value2300 = *source_ml_source2_data_property_value2300;
                                        source_ml_source2_data_property_value2300.clone()
                                    }
                                };
                                match source_ml_source2_s2281.clone() {
                                    self::JavaScriptCallPending::WaitError(_, _, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                }
                            } {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                            } else {
                                if {
                                    let source_ml_source2_comparison_operand2308 = match match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2302) => {
                                            let source_ml_source2_data_property_value2302 = *source_ml_source2_data_property_value2302;
                                            source_ml_source2_data_property_value2302.clone()
                                        }
                                    } {
                                        self::JavaScriptCallPending::CallIdle => {
                                            None::<self::ml::Big>
                                        }
                                        self::JavaScriptCallPending::WaitCall(_) => {
                                            None::<self::ml::Big>
                                        }
                                        self::JavaScriptCallPending::WaitGet(_) => {
                                            None::<self::ml::Big>
                                        }
                                        self::JavaScriptCallPending::WaitSet(_) => {
                                            None::<self::ml::Big>
                                        }
                                        self::JavaScriptCallPending::WaitKey(_, source_ml_source2_data_property_value2303, _) => {
                                            Some(source_ml_source2_data_property_value2303.clone())
                                        }
                                        self::JavaScriptCallPending::WaitDefinition(_, source_ml_source2_data_property_value2304, _) => {
                                            Some(source_ml_source2_data_property_value2304.clone())
                                        }
                                        self::JavaScriptCallPending::WaitNumber(_, source_ml_source2_data_property_value2305, _) => {
                                            Some(source_ml_source2_data_property_value2305.clone())
                                        }
                                        self::JavaScriptCallPending::WaitText(_, source_ml_source2_data_property_value2306, _) => {
                                            Some(source_ml_source2_data_property_value2306.clone())
                                        }
                                        self::JavaScriptCallPending::WaitError(source_ml_source2_data_property_value2307, _, _) => {
                                            Some(source_ml_source2_data_property_value2307.clone())
                                        }
                                    };
                                    !(match (source_ml_source2_comparison_operand2308.clone(), realm.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 114, 101, 97, 108, 109]))
                                } else {
                                    {
                                        let value_error = self::read_java_script_ordinary_value_error(shared_constructor_field1.clone(), value.clone());
                                        if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), value_error.clone())
                                        } else {
                                            match value.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Number(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Text(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Object(_, callable) => {
                                                    if callable {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                    } else {
                                                        self::throw_java_script_call_value(machine.clone(), value.clone())
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_semantic_same_value_loop2(left_units: &[f64], right_units: &[f64], mut at: f64) -> self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if (at < ((left_units.len() as f64))) {
            if (((self::ml_array::optional_at(left_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(right_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)))) {
                self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3 {
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Next,
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>)),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Result {
    MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Done,
    MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_ordinary_object_loop5(mut objects: Vec<self::JavaScriptOrdinaryObject>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2328, _, _, _, _) => {
                                source_ml_source2_data_property_value2328.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2330, _, _, _, _) => {
                                    source_ml_source2_data_property_value2330.clone()
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
                                    high = middle;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Result {
    MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Done,
    MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_ordinary_symbol_loop6(mut symbols: Vec<self::ml::Big>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source2_comparison_operand2332 = None::<()>;
                        match (symbol.clone(), source_ml_source2_comparison_operand2332.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                    } {
                        self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Return(true)
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
                                    high = middle;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Result {
    MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Done,
    MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_ordinary_property_loop7(mut properties: Vec<self::JavaScriptOrdinaryProperty>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Result {
    loop {
        return if (at < ((properties.len() as f64))) {
            {
                let property = self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value({
                    let source_ml_source2_nullish_field_receiver2333 = (property.clone(),);
                    if match (source_ml_source2_nullish_field_receiver2333.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present2334 = ((source_ml_source2_nullish_field_receiver2333.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present2334.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2335, _) => {
                                    let source_ml_source2_data_property_value2335 = *source_ml_source2_data_property_value2335;
                                    source_ml_source2_data_property_value2335.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                    }
                }, key.clone())) {
                    self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Return(at)
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
            self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop15(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut prototype: self::JavaScriptSemanticValue, mut depth: f64) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _) => {
                match prototype.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
                    }
                    self::JavaScriptSemanticValue::Object(identity, _) => {
                        {
                            let depth_2 = (depth + 1f64);
                            if ((identity == (match object.clone() {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2343, _, _, _, _) => {
                                    source_ml_source2_data_property_value2343.clone()
                                }
                            })) || (depth_2 > ((shared_constructor_field1.len() as f64)))) {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 97, 105, 110, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 121, 99, 108, 105, 99]))
                            } else {
                                {
                                    let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                    {
                                        let parent = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(index));
                                        if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                        } else {
                                            {
                                                let prototype_2 = match parent.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2348, _, _, _) => {
                                                        let source_ml_source2_data_property_value2348 = *source_ml_source2_data_property_value2348;
                                                        source_ml_source2_data_property_value2348.clone()
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
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop14(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result {
    loop {
        return match object.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, shared_constructor_field1) => {
                if (at < ((shared_constructor_field1.len() as f64))) {
                    {
                        let property = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(at));
                        if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_key_error(heap.clone(), match property.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2352, _) => {
                                        let source_ml_source2_data_property_value2352 = *source_ml_source2_data_property_value2352;
                                        source_ml_source2_data_property_value2352.clone()
                                    }
                                });
                                if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(key_error.clone())
                                } else {
                                    if ((self::find_java_script_ordinary_property(shared_constructor_field1, match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2356, _) => {
                                            let source_ml_source2_data_property_value2356 = *source_ml_source2_data_property_value2356;
                                            source_ml_source2_data_property_value2356.clone()
                                        }
                                    })) != at) {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                                    } else {
                                        {
                                            let descriptor = match property.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2357) => {
                                                    source_ml_source2_data_property_value2357.clone()
                                                }
                                            };
                                            if !(self::test_complete_java_script_property_descriptor(descriptor.clone())) {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 105, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115]))
                                            } else {
                                                {
                                                    let source_ml_source2_s2358 = if (descriptor.clone()).9 {
                                                        {
                                                            let error = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8);
                                                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(error.clone())
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next
                                                    };
                                                    match source_ml_source2_s2358.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next => {
                                                            {
                                                                let source_ml_source2_s2359 = if (descriptor.clone()).5 {
                                                                    {
                                                                        let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4);
                                                                        if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(error_2.clone())
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next
                                                                };
                                                                match source_ml_source2_s2359.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next => {
                                                                        if (descriptor.clone()).7 {
                                                                            {
                                                                                let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6);
                                                                                if (error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(error_3.clone())
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
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(source_ml_source1_temporary_result17) => {
                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_source1_temporary_result17.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(source_ml_source1_temporary_result17_2) => {
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_source1_temporary_result17_2.clone())
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
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop13(mut heap: self::JavaScriptOrdinaryHeap, mut property_count: f64, mut source_ml_source1_temporary_values10: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index10: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result {
    loop {
        return if (source_ml_source1_temporary_index10 < (self::ml::Big::from_u128(source_ml_source1_temporary_values10.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_source1_temporary_values10, source_ml_source1_temporary_index10.to_index());
                if ({
                    let source_ml_source2_s2361 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2369, _, _, _) => {
                            let source_ml_source2_data_property_value2369 = *source_ml_source2_data_property_value2369;
                            source_ml_source2_data_property_value2369.clone()
                        }
                    };
                    match source_ml_source2_s2361.clone() {
                        self::JavaScriptSemanticValue::Null => {
                            false
                        }
                        _ => {
                            true
                        }
                    }
                } && {
                    let source_ml_source2_s2371 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2379, _, _, _) => {
                            let source_ml_source2_data_property_value2379 = *source_ml_source2_data_property_value2379;
                            source_ml_source2_data_property_value2379.clone()
                        }
                    };
                    match source_ml_source2_s2371.clone() {
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            false
                        }
                        _ => {
                            true
                        }
                    }
                }) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]))
                } else {
                    {
                        let prototype_error = self::read_java_script_ordinary_value_error(heap.clone(), match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2380, _, _, _) => {
                                let source_ml_source2_data_property_value2380 = *source_ml_source2_data_property_value2380;
                                source_ml_source2_data_property_value2380.clone()
                            }
                        });
                        if (prototype_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(prototype_error.clone())
                        } else {
                            {
                                let property_count_2 = (property_count + ((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2381) => source_ml_source2_data_property_value2381.as_slice() }).len() as f64));
                                if ((property_count_2 > (16384f64)) || ((((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2383) => source_ml_source2_data_property_value2383.as_slice() }).len() as f64)) > (4096f64))) {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source2_s2384 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop14(heap.clone(), object.clone(), at);
                                            match source_ml_source2_s2384.clone() {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done => {
                                                    {
                                                        let prototype = match object.clone() {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2385, _, _, _) => {
                                                                let source_ml_source2_data_property_value2385 = *source_ml_source2_data_property_value2385;
                                                                source_ml_source2_data_property_value2385.clone()
                                                            }
                                                        };
                                                        {
                                                            let depth = 0f64;
                                                            {
                                                                let source_ml_source2_s2386 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop15(heap.clone(), object.clone(), prototype.clone(), depth);
                                                                match source_ml_source2_s2386.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done => {
                                                                        {
                                                                            let source_ml_source1_temporary_index10_2 = source_ml_source1_temporary_index10.add(&self::ml::Big::from_i128(1));
                                                                            {
                                                                                (property_count, source_ml_source1_temporary_index10) = (property_count_2, source_ml_source1_temporary_index10_2.clone());
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(source_ml_source1_temporary_result15) => {
                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_source1_temporary_result15.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_source1_temporary_result15_2) => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_source1_temporary_result15_2.clone())
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
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop12(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_source1_temporary_values9: Vec<self::ml::Big>, mut source_ml_source1_temporary_index9: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result {
    loop {
        return if (source_ml_source1_temporary_index9 < (self::ml::Big::from_u128(source_ml_source1_temporary_values9.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_source1_temporary_values9, source_ml_source1_temporary_index9.to_index());
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2388, _) => {
                        source_ml_source2_data_property_value2388
                    }
                }, symbol.clone())) >= (0f64))) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index9_2 = source_ml_source1_temporary_index9.add(&self::ml::Big::from_i128(1));
                        {
                            (previous, source_ml_source1_temporary_index9) = (symbol.clone(), source_ml_source1_temporary_index9_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop11(mut previous: self::ml::Big, mut source_ml_source1_temporary_values8: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index8: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result {
    loop {
        return if (source_ml_source1_temporary_index8 < (self::ml::Big::from_u128(source_ml_source1_temporary_values8.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_source1_temporary_values8, source_ml_source1_temporary_index8.to_index());
                if (((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2390, _, _, _, _) => {
                        source_ml_source2_data_property_value2390.clone()
                    }
                }) <= previous) || ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2392, _, _, _, _) => {
                        source_ml_source2_data_property_value2392.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2393, _, _, _, _) => {
                                source_ml_source2_data_property_value2393.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_index8_2 = source_ml_source1_temporary_index8.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_source1_temporary_index8) = (previous_2.clone(), source_ml_source1_temporary_index8_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result {
    MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done,
    MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool, self::ml::Big)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_locate_java_script_ordinary_property_loop18(mut heap: self::JavaScriptOrdinaryHeap, mut key: self::JavaScriptSemanticValue, mut current: self::ml::Big, mut depth: f64) -> self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _) => {
                if (depth < ((shared_constructor_field1.len() as f64))) {
                    {
                        let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), current.clone());
                        {
                            let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(index));
                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                            } else {
                                {
                                    let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2398) => {
                                            source_ml_source2_data_property_value2398
                                        }
                                    }, key.clone());
                                    {
                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2399) => {
                                                source_ml_source2_data_property_value2399
                                            }
                                        }, self::ml_array::number_index(at));
                                        if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                            self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Return({
                                                let source_ml_source2_field2400 = true;
                                                {
                                                    let source_ml_source2_field2403 = match property.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2402) => {
                                                            source_ml_source2_data_property_value2402.clone()
                                                        }
                                                    };
                                                    (source_ml_source2_field2403.clone(), source_ml_source2_field2400, current.clone())
                                                }
                                            })
                                        } else {
                                            {
                                                let prototype = match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2404, _, _, _) => {
                                                        let source_ml_source2_data_property_value2404 = *source_ml_source2_data_property_value2404;
                                                        source_ml_source2_data_property_value2404.clone()
                                                    }
                                                };
                                                match prototype.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Number(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Object(current_2, _) => {
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
                    self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_define_java_script_ordinary_property_loop20(mut count: f64, mut source_ml_source1_temporary_values19: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index19: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index19 < (self::ml::Big::from_u128(source_ml_source1_temporary_values19.len() as u128))) {
            {
                let entry = self::ml_array::at(&source_ml_source1_temporary_values19, source_ml_source1_temporary_index19.to_index());
                {
                    let count_2 = (count + ((match &entry { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2411) => source_ml_source2_data_property_value2411.as_slice() }).len() as f64));
                    {
                        let source_ml_source1_temporary_index19_2 = source_ml_source1_temporary_index19.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_source1_temporary_index19) = (count_2, source_ml_source1_temporary_index19_2.clone());
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
