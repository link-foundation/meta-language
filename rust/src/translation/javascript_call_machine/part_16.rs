// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptNumberCallState;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::JavaScriptTextCallState;
use super::find_java_script_call_function;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_number_call_source_operation;
use super::read_java_script_text_call_source_operation;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::resume_java_script_property_definition;
use super::resume_java_script_property_key_conversion;
use super::resume_java_script_scalar_protocol;
use super::step_java_script_scalar_protocol;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_key(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptPropertyKeyConversionTransition) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field7, shared_constructor_field8) => {
            let shared_constructor_field7 = *shared_constructor_field7;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field8, self::ml_array::number_index(((shared_constructor_field8.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, {
                                let source_ml_source2_nominal_spread1577 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1577.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                        let shared_constructor_field6 = *shared_constructor_field6;
                                        {
                                            let source_ml_source2_nominal_field1578 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field1580 = shared_constructor_field2;
                                                {
                                                    let source_ml_source2_nominal_field1582 = shared_constructor_field3;
                                                    {
                                                        let source_ml_source2_nominal_field1584 = shared_constructor_field4;
                                                        {
                                                            let source_ml_source2_nominal_field1586 = shared_constructor_field5;
                                                            {
                                                                let source_ml_source2_nominal_field1588 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1590 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state.clone()));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1578.clone(), source_ml_source2_nominal_field1580, source_ml_source2_nominal_field1582, source_ml_source2_nominal_field1584, source_ml_source2_nominal_field1586, Box::new(source_ml_source2_nominal_override1590.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })))
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(state_2, callee, receiver, arguments) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, {
                                let source_ml_source2_nominal_spread1593 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1593.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2) => {
                                        let shared_constructor_field6_2 = *shared_constructor_field6_2;
                                        {
                                            let source_ml_source2_nominal_field1594 = shared_constructor_field1_2.clone();
                                            {
                                                let source_ml_source2_nominal_field1596 = shared_constructor_field2_2;
                                                {
                                                    let source_ml_source2_nominal_field1598 = shared_constructor_field3_2;
                                                    {
                                                        let source_ml_source2_nominal_field1600 = shared_constructor_field4_2;
                                                        {
                                                            let source_ml_source2_nominal_field1602 = shared_constructor_field5_2;
                                                            {
                                                                let source_ml_source2_nominal_field1604 = shared_constructor_field6_2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1606 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state_2.clone()));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1594.clone(), source_ml_source2_nominal_field1596, source_ml_source2_nominal_field1598, source_ml_source2_nominal_field1600, source_ml_source2_nominal_field1602, Box::new(source_ml_source2_nominal_override1606.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }), callee.clone(), receiver.clone(), arguments)
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(key) => {
                            let key = *key;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, key.clone()))))
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(value) => {
                            let value = *value;
                            self::throw_java_script_call_value(machine.clone(), value.clone())
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(error_class, reason) => {
                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), reason_2.clone())
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_definition(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptPropertyDefinitionTransition) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field7, shared_constructor_field8) => {
            let shared_constructor_field7 = *shared_constructor_field7;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field8, self::ml_array::number_index(((shared_constructor_field8.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, {
                                let source_ml_source2_nominal_spread1616 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1616.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                        let shared_constructor_field6 = *shared_constructor_field6;
                                        {
                                            let source_ml_source2_nominal_field1617 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field1619 = shared_constructor_field2;
                                                {
                                                    let source_ml_source2_nominal_field1621 = shared_constructor_field3;
                                                    {
                                                        let source_ml_source2_nominal_field1623 = shared_constructor_field4;
                                                        {
                                                            let source_ml_source2_nominal_field1625 = shared_constructor_field5;
                                                            {
                                                                let source_ml_source2_nominal_field1627 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1629 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state.clone()));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1617.clone(), source_ml_source2_nominal_field1619, source_ml_source2_nominal_field1621, source_ml_source2_nominal_field1623, source_ml_source2_nominal_field1625, Box::new(source_ml_source2_nominal_override1629.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })))
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionCall(state_2, callee, receiver, arguments) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, {
                                let source_ml_source2_nominal_spread1632 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1632.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2) => {
                                        let shared_constructor_field6_2 = *shared_constructor_field6_2;
                                        {
                                            let source_ml_source2_nominal_field1633 = shared_constructor_field1_2.clone();
                                            {
                                                let source_ml_source2_nominal_field1635 = shared_constructor_field2_2;
                                                {
                                                    let source_ml_source2_nominal_field1637 = shared_constructor_field3_2;
                                                    {
                                                        let source_ml_source2_nominal_field1639 = shared_constructor_field4_2;
                                                        {
                                                            let source_ml_source2_nominal_field1641 = shared_constructor_field5_2;
                                                            {
                                                                let source_ml_source2_nominal_field1643 = shared_constructor_field6_2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1645 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state_2.clone()));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1633.clone(), source_ml_source2_nominal_field1635, source_ml_source2_nominal_field1637, source_ml_source2_nominal_field1639, source_ml_source2_nominal_field1641, Box::new(source_ml_source2_nominal_override1645.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }), callee.clone(), receiver.clone(), arguments)
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(heap, value, _) => {
                            let heap = *heap;
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), shared_constructor_field8, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(machine.clone(), value_2.clone())
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(error_class, reason) => {
                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), reason_2.clone())
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_number(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptScalarConversionTransition) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field7, shared_constructor_field8) => {
            let shared_constructor_field7 = *shared_constructor_field7;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field8, self::ml_array::number_index(((shared_constructor_field8.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptScalarConversionTransition::ScalarContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, {
                                let source_ml_source2_nominal_spread1653 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1653.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                        let shared_constructor_field6 = *shared_constructor_field6;
                                        {
                                            let source_ml_source2_nominal_field1654 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field1656 = shared_constructor_field2;
                                                {
                                                    let source_ml_source2_nominal_field1658 = shared_constructor_field3;
                                                    {
                                                        let source_ml_source2_nominal_field1660 = shared_constructor_field4;
                                                        {
                                                            let source_ml_source2_nominal_field1662 = shared_constructor_field5;
                                                            {
                                                                let source_ml_source2_nominal_field1664 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1666 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state.clone()))));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1654.clone(), source_ml_source2_nominal_field1656, source_ml_source2_nominal_field1658, source_ml_source2_nominal_field1660, source_ml_source2_nominal_field1662, Box::new(source_ml_source2_nominal_override1666.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarCall(state_2, callee, receiver, arguments) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, {
                                let source_ml_source2_nominal_spread1669 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1669.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2) => {
                                        let shared_constructor_field6_2 = *shared_constructor_field6_2;
                                        {
                                            let source_ml_source2_nominal_field1670 = shared_constructor_field1_2.clone();
                                            {
                                                let source_ml_source2_nominal_field1672 = shared_constructor_field2_2;
                                                {
                                                    let source_ml_source2_nominal_field1674 = shared_constructor_field3_2;
                                                    {
                                                        let source_ml_source2_nominal_field1676 = shared_constructor_field4_2;
                                                        {
                                                            let source_ml_source2_nominal_field1678 = shared_constructor_field5_2;
                                                            {
                                                                let source_ml_source2_nominal_field1680 = shared_constructor_field6_2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1682 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state_2.clone()))));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1670.clone(), source_ml_source2_nominal_field1672, source_ml_source2_nominal_field1674, source_ml_source2_nominal_field1676, source_ml_source2_nominal_field1678, Box::new(source_ml_source2_nominal_override1682.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }), callee.clone(), receiver.clone(), arguments)
                        }
                        self::JavaScriptScalarConversionTransition::ScalarCompleted(value) => {
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(machine.clone(), value_2.clone())
                        }
                        self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class, reason) => {
                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), reason_2.clone())
                        }
                        self::JavaScriptScalarConversionTransition::ScalarResourceLimit(value_3, reason_3) => {
                            let value_3 = *value_3;
                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, {
                                let source_ml_source2_nominal_spread1688 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1688.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_3, shared_constructor_field2_3, shared_constructor_field3_3, shared_constructor_field4_3, shared_constructor_field5_3, shared_constructor_field6_3) => {
                                        let shared_constructor_field6_3 = *shared_constructor_field6_3;
                                        {
                                            let source_ml_source2_nominal_field1689 = shared_constructor_field1_3.clone();
                                            {
                                                let source_ml_source2_nominal_field1691 = shared_constructor_field2_3;
                                                {
                                                    let source_ml_source2_nominal_field1693 = shared_constructor_field3_3;
                                                    {
                                                        let source_ml_source2_nominal_field1695 = shared_constructor_field4_3;
                                                        {
                                                            let source_ml_source2_nominal_field1697 = shared_constructor_field5_3;
                                                            {
                                                                let source_ml_source2_nominal_field1699 = shared_constructor_field6_3.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1701 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberPrimitiveValue(Box::new(value_3.clone()))));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1689.clone(), source_ml_source2_nominal_field1691, source_ml_source2_nominal_field1693, source_ml_source2_nominal_field1695, source_ml_source2_nominal_field1697, Box::new(source_ml_source2_nominal_override1701.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })), reason_3.clone())
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_call_number(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, state: self::JavaScriptNumberCallState) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                    let shared_constructor_field2 = *shared_constructor_field2;
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index(((shared_constructor_field4.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            {
                                let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1706, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1706.clone()
                                    }
                                })));
                                if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                } else {
                                    {
                                        let operation = self::read_java_script_number_call_source_operation(fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"), destination, realm.clone(), shared_constructor_field2.clone(), state.clone());
                                        if (operation == (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                        } else {
                                            match state.clone() {
                                                self::JavaScriptNumberCallState::NumberProtocolState(state_state) => {
                                                    let state_state = *state_state;
                                                    self::accept_java_script_call_number(program.clone(), machine.clone(), destination, realm.clone(), self::step_java_script_scalar_protocol(shared_constructor_field3.clone(), state_state.clone(), &(operation)))
                                                }
                                                self::JavaScriptNumberCallState::NumberPrimitiveValue(value) => {
                                                    let value = *value;
                                                    self::accept_java_script_call_number(program.clone(), machine.clone(), destination, realm.clone(), self::finish_java_script_scalar_primitive(&(operation), value.clone()))
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
pub fn resume_java_script_call_number(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, state: self::JavaScriptNumberCallState, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                    let shared_constructor_field2 = *shared_constructor_field2;
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index(((shared_constructor_field4.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            {
                                let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1715, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1715.clone()
                                    }
                                })));
                                if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                } else {
                                    {
                                        let operation = self::read_java_script_number_call_source_operation(fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"), destination, realm.clone(), shared_constructor_field2.clone(), state.clone());
                                        if (operation == (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                        } else {
                                            match state.clone() {
                                                self::JavaScriptNumberCallState::NumberProtocolState(state_state) => {
                                                    let state_state = *state_state;
                                                    self::accept_java_script_call_number(program.clone(), machine.clone(), destination, realm.clone(), self::resume_java_script_scalar_protocol(shared_constructor_field3.clone(), state_state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone())), &(operation)))
                                                }
                                                self::JavaScriptNumberCallState::NumberPrimitiveValue(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
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
pub fn accept_java_script_call_text(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptScalarConversionTransition) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field7, shared_constructor_field8) => {
            let shared_constructor_field7 = *shared_constructor_field7;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field8, self::ml_array::number_index(((shared_constructor_field8.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptScalarConversionTransition::ScalarContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, {
                                let source_ml_source2_nominal_spread1727 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1727.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                        let shared_constructor_field6 = *shared_constructor_field6;
                                        {
                                            let source_ml_source2_nominal_field1728 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field1730 = shared_constructor_field2;
                                                {
                                                    let source_ml_source2_nominal_field1732 = shared_constructor_field3;
                                                    {
                                                        let source_ml_source2_nominal_field1734 = shared_constructor_field4;
                                                        {
                                                            let source_ml_source2_nominal_field1736 = shared_constructor_field5;
                                                            {
                                                                let source_ml_source2_nominal_field1738 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1740 = self::JavaScriptCallPending::WaitText(destination, realm.clone(), Box::new(self::JavaScriptTextCallState::TextProtocolState(Box::new(state.clone()))));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1728.clone(), source_ml_source2_nominal_field1730, source_ml_source2_nominal_field1732, source_ml_source2_nominal_field1734, source_ml_source2_nominal_field1736, Box::new(source_ml_source2_nominal_override1740.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarCall(state_2, callee, receiver, arguments) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, {
                                let source_ml_source2_nominal_spread1743 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1743.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2) => {
                                        let shared_constructor_field6_2 = *shared_constructor_field6_2;
                                        {
                                            let source_ml_source2_nominal_field1744 = shared_constructor_field1_2.clone();
                                            {
                                                let source_ml_source2_nominal_field1746 = shared_constructor_field2_2;
                                                {
                                                    let source_ml_source2_nominal_field1748 = shared_constructor_field3_2;
                                                    {
                                                        let source_ml_source2_nominal_field1750 = shared_constructor_field4_2;
                                                        {
                                                            let source_ml_source2_nominal_field1752 = shared_constructor_field5_2;
                                                            {
                                                                let source_ml_source2_nominal_field1754 = shared_constructor_field6_2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1756 = self::JavaScriptCallPending::WaitText(destination, realm.clone(), Box::new(self::JavaScriptTextCallState::TextProtocolState(Box::new(state_2.clone()))));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1744.clone(), source_ml_source2_nominal_field1746, source_ml_source2_nominal_field1748, source_ml_source2_nominal_field1750, source_ml_source2_nominal_field1752, Box::new(source_ml_source2_nominal_override1756.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }), callee.clone(), receiver.clone(), arguments)
                        }
                        self::JavaScriptScalarConversionTransition::ScalarCompleted(value) => {
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(machine.clone(), value_2.clone())
                        }
                        self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class, reason) => {
                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), reason_2.clone())
                        }
                        self::JavaScriptScalarConversionTransition::ScalarResourceLimit(value_3, reason_3) => {
                            let value_3 = *value_3;
                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, {
                                let source_ml_source2_nominal_spread1762 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1762.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_3, shared_constructor_field2_3, shared_constructor_field3_3, shared_constructor_field4_3, shared_constructor_field5_3, shared_constructor_field6_3) => {
                                        let shared_constructor_field6_3 = *shared_constructor_field6_3;
                                        {
                                            let source_ml_source2_nominal_field1763 = shared_constructor_field1_3.clone();
                                            {
                                                let source_ml_source2_nominal_field1765 = shared_constructor_field2_3;
                                                {
                                                    let source_ml_source2_nominal_field1767 = shared_constructor_field3_3;
                                                    {
                                                        let source_ml_source2_nominal_field1769 = shared_constructor_field4_3;
                                                        {
                                                            let source_ml_source2_nominal_field1771 = shared_constructor_field5_3;
                                                            {
                                                                let source_ml_source2_nominal_field1773 = shared_constructor_field6_3.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1775 = self::JavaScriptCallPending::WaitText(destination, realm.clone(), Box::new(self::JavaScriptTextCallState::TextPrimitiveValue(Box::new(value_3.clone()))));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1763.clone(), source_ml_source2_nominal_field1765, source_ml_source2_nominal_field1767, source_ml_source2_nominal_field1769, source_ml_source2_nominal_field1771, Box::new(source_ml_source2_nominal_override1775.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })), reason_3.clone())
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_call_text(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, state: self::JavaScriptTextCallState) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                    let shared_constructor_field2 = *shared_constructor_field2;
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index(((shared_constructor_field4.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            {
                                let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1780, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1780.clone()
                                    }
                                })));
                                if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                } else {
                                    {
                                        let operation = self::read_java_script_text_call_source_operation(fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"), destination, realm.clone(), shared_constructor_field2.clone(), state.clone());
                                        if (operation == (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                        } else {
                                            match state.clone() {
                                                self::JavaScriptTextCallState::TextProtocolState(state_state) => {
                                                    let state_state = *state_state;
                                                    self::accept_java_script_call_text(program.clone(), machine.clone(), destination, realm.clone(), self::step_java_script_scalar_protocol(shared_constructor_field3.clone(), state_state.clone(), &(operation)))
                                                }
                                                self::JavaScriptTextCallState::TextPrimitiveValue(value) => {
                                                    let value = *value;
                                                    self::accept_java_script_call_text(program.clone(), machine.clone(), destination, realm.clone(), self::finish_java_script_scalar_primitive(&(operation), value.clone()))
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
pub fn resume_java_script_call_text(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, state: self::JavaScriptTextCallState, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                    let shared_constructor_field2 = *shared_constructor_field2;
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index(((shared_constructor_field4.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            {
                                let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1789, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1789.clone()
                                    }
                                })));
                                if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                } else {
                                    {
                                        let operation = self::read_java_script_text_call_source_operation(fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"), destination, realm.clone(), shared_constructor_field2.clone(), state.clone());
                                        if (operation == (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                        } else {
                                            match state.clone() {
                                                self::JavaScriptTextCallState::TextProtocolState(state_state) => {
                                                    let state_state = *state_state;
                                                    self::accept_java_script_call_text(program.clone(), machine.clone(), destination, realm.clone(), self::resume_java_script_scalar_protocol(shared_constructor_field3.clone(), state_state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone())), &(operation)))
                                                }
                                                self::JavaScriptTextCallState::TextPrimitiveValue(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 116, 114, 105, 110, 103, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
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
pub fn return_java_script_call_value(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frames = self::ml_array::slice(&shared_constructor_field2, 0f64, (-1f64));
                {
                    let parent = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
                    if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::JavaScriptCallMachineTransition::MachineCompleted(Box::new(shared_constructor_field1.clone()), Box::new(value.clone()))
                    } else {
                        {
                            let current = self::JavaScriptCallMachine::CallMachine(Box::new(shared_constructor_field1.clone()), frames.clone());
                            {
                                let pending = match parent.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1800) => {
                                        let source_ml_source2_data_property_value1800 = *source_ml_source2_data_property_value1800;
                                        source_ml_source2_data_property_value1800.clone()
                                    }
                                };
                                match pending.clone() {
                                    self::JavaScriptCallPending::CallIdle => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                    }
                                    self::JavaScriptCallPending::WaitCall(destination) => {
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination, value.clone()))))
                                    }
                                    self::JavaScriptCallPending::WaitGet(destination_2) => {
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_2, value.clone()))))
                                    }
                                    self::JavaScriptCallPending::WaitSet(destination_3) => {
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_3, self::JavaScriptSemanticValue::Boolean(true)))))
                                    }
                                    self::JavaScriptCallPending::WaitKey(destination_4, realm, state) => {
                                        let state = *state;
                                        self::accept_java_script_call_key(program.clone(), current.clone(), destination_4, realm.clone(), self::resume_java_script_property_key_conversion(shared_constructor_field1.clone(), state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone()))))
                                    }
                                    self::JavaScriptCallPending::WaitDefinition(destination_5, realm_2, state_2) => {
                                        let state_2 = *state_2;
                                        self::accept_java_script_call_definition(program.clone(), current.clone(), destination_5, realm_2.clone(), self::resume_java_script_property_definition(shared_constructor_field1.clone(), state_2.clone(), self::JavaScriptSemanticCompletion::NormalCompletion(Box::new(value.clone()))))
                                    }
                                    self::JavaScriptCallPending::WaitNumber(destination_6, realm_3, state_3) => {
                                        let state_3 = *state_3;
                                        self::resume_java_script_call_number(program.clone(), current.clone(), destination_6, realm_3.clone(), state_3.clone(), value.clone())
                                    }
                                    self::JavaScriptCallPending::WaitText(destination_7, realm_4, state_4) => {
                                        let state_4 = *state_4;
                                        self::resume_java_script_call_text(program.clone(), current.clone(), destination_7, realm_4.clone(), state_4.clone(), value.clone())
                                    }
                                    self::JavaScriptCallPending::WaitSourceError(_, _, _, _) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                    }
                                    self::JavaScriptCallPending::WaitError(_, _, _) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
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
