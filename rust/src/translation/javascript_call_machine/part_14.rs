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
                                let source_ml_source2_nominal_spread1566 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1566.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                        let shared_constructor_field6 = *shared_constructor_field6;
                                        {
                                            let source_ml_source2_nominal_field1567 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field1569 = shared_constructor_field2;
                                                {
                                                    let source_ml_source2_nominal_field1571 = shared_constructor_field3;
                                                    {
                                                        let source_ml_source2_nominal_field1573 = shared_constructor_field4;
                                                        {
                                                            let source_ml_source2_nominal_field1575 = shared_constructor_field5;
                                                            {
                                                                let source_ml_source2_nominal_field1577 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1579 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state.clone()));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1567.clone(), source_ml_source2_nominal_field1569, source_ml_source2_nominal_field1571, source_ml_source2_nominal_field1573, source_ml_source2_nominal_field1575, Box::new(source_ml_source2_nominal_override1579.clone()))
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
                                let source_ml_source2_nominal_spread1582 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1582.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2) => {
                                        let shared_constructor_field6_2 = *shared_constructor_field6_2;
                                        {
                                            let source_ml_source2_nominal_field1583 = shared_constructor_field1_2.clone();
                                            {
                                                let source_ml_source2_nominal_field1585 = shared_constructor_field2_2;
                                                {
                                                    let source_ml_source2_nominal_field1587 = shared_constructor_field3_2;
                                                    {
                                                        let source_ml_source2_nominal_field1589 = shared_constructor_field4_2;
                                                        {
                                                            let source_ml_source2_nominal_field1591 = shared_constructor_field5_2;
                                                            {
                                                                let source_ml_source2_nominal_field1593 = shared_constructor_field6_2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1595 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state_2.clone()));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1583.clone(), source_ml_source2_nominal_field1585, source_ml_source2_nominal_field1587, source_ml_source2_nominal_field1589, source_ml_source2_nominal_field1591, Box::new(source_ml_source2_nominal_override1595.clone()))
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
                                let source_ml_source2_nominal_spread1605 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1605.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                        let shared_constructor_field6 = *shared_constructor_field6;
                                        {
                                            let source_ml_source2_nominal_field1606 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field1608 = shared_constructor_field2;
                                                {
                                                    let source_ml_source2_nominal_field1610 = shared_constructor_field3;
                                                    {
                                                        let source_ml_source2_nominal_field1612 = shared_constructor_field4;
                                                        {
                                                            let source_ml_source2_nominal_field1614 = shared_constructor_field5;
                                                            {
                                                                let source_ml_source2_nominal_field1616 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1618 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state.clone()));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1606.clone(), source_ml_source2_nominal_field1608, source_ml_source2_nominal_field1610, source_ml_source2_nominal_field1612, source_ml_source2_nominal_field1614, Box::new(source_ml_source2_nominal_override1618.clone()))
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
                                let source_ml_source2_nominal_spread1621 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1621.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2) => {
                                        let shared_constructor_field6_2 = *shared_constructor_field6_2;
                                        {
                                            let source_ml_source2_nominal_field1622 = shared_constructor_field1_2.clone();
                                            {
                                                let source_ml_source2_nominal_field1624 = shared_constructor_field2_2;
                                                {
                                                    let source_ml_source2_nominal_field1626 = shared_constructor_field3_2;
                                                    {
                                                        let source_ml_source2_nominal_field1628 = shared_constructor_field4_2;
                                                        {
                                                            let source_ml_source2_nominal_field1630 = shared_constructor_field5_2;
                                                            {
                                                                let source_ml_source2_nominal_field1632 = shared_constructor_field6_2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1634 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state_2.clone()));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1622.clone(), source_ml_source2_nominal_field1624, source_ml_source2_nominal_field1626, source_ml_source2_nominal_field1628, source_ml_source2_nominal_field1630, Box::new(source_ml_source2_nominal_override1634.clone()))
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
                                let source_ml_source2_nominal_spread1642 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1642.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                        let shared_constructor_field6 = *shared_constructor_field6;
                                        {
                                            let source_ml_source2_nominal_field1643 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field1645 = shared_constructor_field2;
                                                {
                                                    let source_ml_source2_nominal_field1647 = shared_constructor_field3;
                                                    {
                                                        let source_ml_source2_nominal_field1649 = shared_constructor_field4;
                                                        {
                                                            let source_ml_source2_nominal_field1651 = shared_constructor_field5;
                                                            {
                                                                let source_ml_source2_nominal_field1653 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1655 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state.clone()))));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1643.clone(), source_ml_source2_nominal_field1645, source_ml_source2_nominal_field1647, source_ml_source2_nominal_field1649, source_ml_source2_nominal_field1651, Box::new(source_ml_source2_nominal_override1655.clone()))
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
                                let source_ml_source2_nominal_spread1658 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1658.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2) => {
                                        let shared_constructor_field6_2 = *shared_constructor_field6_2;
                                        {
                                            let source_ml_source2_nominal_field1659 = shared_constructor_field1_2.clone();
                                            {
                                                let source_ml_source2_nominal_field1661 = shared_constructor_field2_2;
                                                {
                                                    let source_ml_source2_nominal_field1663 = shared_constructor_field3_2;
                                                    {
                                                        let source_ml_source2_nominal_field1665 = shared_constructor_field4_2;
                                                        {
                                                            let source_ml_source2_nominal_field1667 = shared_constructor_field5_2;
                                                            {
                                                                let source_ml_source2_nominal_field1669 = shared_constructor_field6_2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1671 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state_2.clone()))));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1659.clone(), source_ml_source2_nominal_field1661, source_ml_source2_nominal_field1663, source_ml_source2_nominal_field1665, source_ml_source2_nominal_field1667, Box::new(source_ml_source2_nominal_override1671.clone()))
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
                                let source_ml_source2_nominal_spread1677 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1677.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_3, shared_constructor_field2_3, shared_constructor_field3_3, shared_constructor_field4_3, shared_constructor_field5_3, shared_constructor_field6_3) => {
                                        let shared_constructor_field6_3 = *shared_constructor_field6_3;
                                        {
                                            let source_ml_source2_nominal_field1678 = shared_constructor_field1_3.clone();
                                            {
                                                let source_ml_source2_nominal_field1680 = shared_constructor_field2_3;
                                                {
                                                    let source_ml_source2_nominal_field1682 = shared_constructor_field3_3;
                                                    {
                                                        let source_ml_source2_nominal_field1684 = shared_constructor_field4_3;
                                                        {
                                                            let source_ml_source2_nominal_field1686 = shared_constructor_field5_3;
                                                            {
                                                                let source_ml_source2_nominal_field1688 = shared_constructor_field6_3.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1690 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberPrimitiveValue(Box::new(value_3.clone()))));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1678.clone(), source_ml_source2_nominal_field1680, source_ml_source2_nominal_field1682, source_ml_source2_nominal_field1684, source_ml_source2_nominal_field1686, Box::new(source_ml_source2_nominal_override1690.clone()))
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
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1695, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1695.clone()
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
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1704, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1704.clone()
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
                                let source_ml_source2_nominal_spread1716 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1716.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                        let shared_constructor_field6 = *shared_constructor_field6;
                                        {
                                            let source_ml_source2_nominal_field1717 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field1719 = shared_constructor_field2;
                                                {
                                                    let source_ml_source2_nominal_field1721 = shared_constructor_field3;
                                                    {
                                                        let source_ml_source2_nominal_field1723 = shared_constructor_field4;
                                                        {
                                                            let source_ml_source2_nominal_field1725 = shared_constructor_field5;
                                                            {
                                                                let source_ml_source2_nominal_field1727 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1729 = self::JavaScriptCallPending::WaitText(destination, realm.clone(), Box::new(self::JavaScriptTextCallState::TextProtocolState(Box::new(state.clone()))));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1717.clone(), source_ml_source2_nominal_field1719, source_ml_source2_nominal_field1721, source_ml_source2_nominal_field1723, source_ml_source2_nominal_field1725, Box::new(source_ml_source2_nominal_override1729.clone()))
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
                                let source_ml_source2_nominal_spread1732 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1732.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2) => {
                                        let shared_constructor_field6_2 = *shared_constructor_field6_2;
                                        {
                                            let source_ml_source2_nominal_field1733 = shared_constructor_field1_2.clone();
                                            {
                                                let source_ml_source2_nominal_field1735 = shared_constructor_field2_2;
                                                {
                                                    let source_ml_source2_nominal_field1737 = shared_constructor_field3_2;
                                                    {
                                                        let source_ml_source2_nominal_field1739 = shared_constructor_field4_2;
                                                        {
                                                            let source_ml_source2_nominal_field1741 = shared_constructor_field5_2;
                                                            {
                                                                let source_ml_source2_nominal_field1743 = shared_constructor_field6_2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1745 = self::JavaScriptCallPending::WaitText(destination, realm.clone(), Box::new(self::JavaScriptTextCallState::TextProtocolState(Box::new(state_2.clone()))));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1733.clone(), source_ml_source2_nominal_field1735, source_ml_source2_nominal_field1737, source_ml_source2_nominal_field1739, source_ml_source2_nominal_field1741, Box::new(source_ml_source2_nominal_override1745.clone()))
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
                                let source_ml_source2_nominal_spread1751 = frame.clone().expect("a proven present value");
                                match source_ml_source2_nominal_spread1751.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_3, shared_constructor_field2_3, shared_constructor_field3_3, shared_constructor_field4_3, shared_constructor_field5_3, shared_constructor_field6_3) => {
                                        let shared_constructor_field6_3 = *shared_constructor_field6_3;
                                        {
                                            let source_ml_source2_nominal_field1752 = shared_constructor_field1_3.clone();
                                            {
                                                let source_ml_source2_nominal_field1754 = shared_constructor_field2_3;
                                                {
                                                    let source_ml_source2_nominal_field1756 = shared_constructor_field3_3;
                                                    {
                                                        let source_ml_source2_nominal_field1758 = shared_constructor_field4_3;
                                                        {
                                                            let source_ml_source2_nominal_field1760 = shared_constructor_field5_3;
                                                            {
                                                                let source_ml_source2_nominal_field1762 = shared_constructor_field6_3.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override1764 = self::JavaScriptCallPending::WaitText(destination, realm.clone(), Box::new(self::JavaScriptTextCallState::TextPrimitiveValue(Box::new(value_3.clone()))));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1752.clone(), source_ml_source2_nominal_field1754, source_ml_source2_nominal_field1756, source_ml_source2_nominal_field1758, source_ml_source2_nominal_field1760, Box::new(source_ml_source2_nominal_override1764.clone()))
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
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1769, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1769.clone()
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
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1778, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1778.clone()
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
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1789) => {
                                        let source_ml_source2_data_property_value1789 = *source_ml_source2_data_property_value1789;
                                        source_ml_source2_data_property_value1789.clone()
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
