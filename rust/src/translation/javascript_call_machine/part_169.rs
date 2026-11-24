// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptNumberCallState;
use super::JavaScriptObjectFromEntriesErrorRequest;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPropertyDefinitionCall;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptTextCallState;
use super::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334;
use super::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335;
use super::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336;
use super::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337;
use super::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_constructor_failure;
use super::read_java_script_descriptor_operands;
use super::read_java_script_executable_call_function;
use super::read_java_script_number_call_source_operation;
use super::read_java_script_number_call_state_error;
use super::read_java_script_property_definition_state_error;
use super::read_java_script_text_call_source_operation;
use super::read_java_script_text_call_state_error;
use super::test_java_script_property_descriptors_same_value;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_pending_conversion_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    frame: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
    suspended: bool,
    pending: self::JavaScriptCallPending,
) -> self::ml_string::Text {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            _,
            shared_constructor_field3,
            shared_constructor_field4,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => match program.clone() {
            self::JavaScriptCallProgram::CallProgram(
                _,
                shared_constructor_field1,
                shared_constructor_field2,
                _,
                _,
                _,
            ) => {
                let shared_constructor_field2 = *shared_constructor_field2;
                match pending.clone() {
                    self::JavaScriptCallPending::WaitKey(_, realm, _) => {
                        if !(self::ml_array::includes(
                            &{
                                let ml_reference_value1980 =
                                    shared_constructor_field1.borrow().clone();
                                ml_reference_value1980
                            },
                            &realm.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )) {
                            self::ml_string::Text::from_units(vec![
                                112, 101, 110, 100, 105, 110, 103, 32, 107, 101, 121, 32, 99, 111,
                                110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105,
                                110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108,
                                109,
                            ])
                        } else {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                    self::JavaScriptCallPending::WaitDefinition(
                        pending_destination,
                        pending_realm,
                        pending_state,
                    ) => {
                        let pending_state = *pending_state;
                        if !(self::ml_array::includes(
                            &{
                                let ml_reference_value1981 =
                                    shared_constructor_field1.borrow().clone();
                                ml_reference_value1981
                            },
                            &pending_realm.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )) {
                            self::ml_string::Text::from_units(vec![
                                112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 111, 112, 101,
                                114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111,
                                110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                                97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115,
                                105, 99, 32, 114, 101, 97, 108, 109,
                            ])
                        } else {
                            {
                                let invalid =
                                    self::read_java_script_property_definition_state_error(
                                        heap.clone(),
                                        pending_state.clone(),
                                    );
                                if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                    invalid.clone()
                                } else {
                                    {
                                        let invalid_target =
                                            self::ml_string::Text::from_units(vec![
                                                112, 114, 111, 112, 101, 114, 116, 121, 32, 100,
                                                101, 102, 105, 110, 105, 116, 105, 111, 110, 32,
                                                114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                                                115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                                108, 101, 99, 116, 101, 100, 32, 115, 111, 117,
                                                114, 99, 101, 32, 116, 97, 114, 103, 101, 116,
                                            ]);
                                        {
                                            let invalid_operation =
                                                self::ml_string::Text::from_units(vec![
                                                    112, 114, 111, 112, 101, 114, 116, 121, 32,
                                                    100, 101, 102, 105, 110, 105, 116, 105, 111,
                                                    110, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                                                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                                    32, 115, 111, 117, 114, 99, 101, 32, 114, 101,
                                                    115, 117, 108, 116, 32, 112, 111, 108, 105, 99,
                                                    121, 32, 100, 101, 115, 116, 105, 110, 97, 116,
                                                    105, 111, 110, 32, 97, 110, 100, 32, 114, 101,
                                                    97, 108, 109,
                                                ]);
                                            if !(shared_constructor_field3 > (0f64)) {
                                                invalid_target.clone()
                                            } else {
                                                {
                                                    let instruction = {
                                                        let source_ml_source2_array_read_value32036 = match fn_.clone() {
                                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value32034, _, _) => {
                                                                    source_ml_source2_data_property_value32034.clone()
                                                                }
                                                            };
                                                        {
                                                            let source_ml_source2_array_read_index32037 =
                                                                (shared_constructor_field3 - 1f64);
                                                            {
                                                                let ml_reference_array1982 = source_ml_source2_array_read_value32036.clone();
                                                                let ml_reference_position1982 = source_ml_source2_array_read_index32037;
                                                                let ml_reference_projection1982 = self::ml_array::optional_at(&*ml_reference_array1982.borrow(), self::ml_array::number_index(ml_reference_position1982));
                                                                ml_reference_projection1982
                                                            }
                                                        }
                                                    };
                                                    if match (instruction.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    } {
                                                        invalid_target.clone()
                                                    } else {
                                                        {
                                                            let operation = (match pending_state.clone() {
                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, _, source_ml_source2_data_property_value32038, _) => {
                                                                        ((source_ml_source2_data_property_value32038.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                    }
                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, _, _, source_ml_source2_data_property_value32039, _) => {
                                                                        ((source_ml_source2_data_property_value32039.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                    }
                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, source_ml_source2_data_property_value32040, _) => {
                                                                        ((source_ml_source2_data_property_value32040.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                    }
                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, source_ml_source2_data_property_value32041, _) => {
                                                                        ((source_ml_source2_data_property_value32041.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121]));
                                                            {
                                                                let source_ml_source2_s32106 = match instruction.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallInstruction::LiteralAccessor(destination, object, _, _, _, realm_2) => {
                                                                            {
                                                                                let target = {
                                                                                    let source_ml_source2_array_read_value32115 = shared_constructor_field4.clone();
                                                                                    { let ml_reference_array1983 = source_ml_source2_array_read_value32115.clone(); let ml_reference_position1983 = object; let ml_reference_projection1983 = self::ml_array::optional_at(&*ml_reference_array1983.borrow(), self::ml_array::number_index(ml_reference_position1983)); ml_reference_projection1983 }
                                                                                };
                                                                                if (match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(target.clone().expect("a proven present value"), match pending_state.clone() {
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value32117, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32117 = *source_ml_source2_data_property_value32117;
                                                                                        source_ml_source2_data_property_value32117.clone()
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value32118, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32118 = *source_ml_source2_data_property_value32118;
                                                                                        source_ml_source2_data_property_value32118.clone()
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(source_ml_source2_data_property_value32119, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32119 = *source_ml_source2_data_property_value32119;
                                                                                        source_ml_source2_data_property_value32119.clone()
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(source_ml_source2_data_property_value32120, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32120 = *source_ml_source2_data_property_value32120;
                                                                                        source_ml_source2_data_property_value32120.clone()
                                                                                    }
                                                                                }))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(invalid_target.clone())
                                                                                } else {
                                                                                    if (((operation != (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121]))) || (destination != pending_destination)) || (realm_2 != pending_realm)) {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(invalid_operation.clone())
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Next
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::JavaScriptCallInstruction::OrdinaryDataProperty(destination_2, object_2, _, _, realm_3) => {
                                                                            {
                                                                                let target_2 = {
                                                                                    let source_ml_source2_array_read_value32122 = shared_constructor_field4.clone();
                                                                                    { let ml_reference_array1984 = source_ml_source2_array_read_value32122.clone(); let ml_reference_position1984 = object_2; let ml_reference_projection1984 = self::ml_array::optional_at(&*ml_reference_array1984.borrow(), self::ml_array::number_index(ml_reference_position1984)); ml_reference_projection1984 }
                                                                                };
                                                                                if (match (target_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(target_2.clone().expect("a proven present value"), match pending_state.clone() {
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value32124, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32124 = *source_ml_source2_data_property_value32124;
                                                                                        source_ml_source2_data_property_value32124.clone()
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value32125, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32125 = *source_ml_source2_data_property_value32125;
                                                                                        source_ml_source2_data_property_value32125.clone()
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(source_ml_source2_data_property_value32126, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32126 = *source_ml_source2_data_property_value32126;
                                                                                        source_ml_source2_data_property_value32126.clone()
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(source_ml_source2_data_property_value32127, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32127 = *source_ml_source2_data_property_value32127;
                                                                                        source_ml_source2_data_property_value32127.clone()
                                                                                    }
                                                                                }))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(invalid_target.clone())
                                                                                } else {
                                                                                    if (((operation != (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121]))) || (destination_2 != pending_destination)) || (realm_3 != pending_realm)) {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(invalid_operation.clone())
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Next
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::JavaScriptCallInstruction::OrdinaryDefine(destination_3, instruction_target, instruction_key, fields, realm_4) => {
                                                                            {
                                                                                let target_3 = {
                                                                                    let source_ml_source2_array_read_value32129 = shared_constructor_field4.clone();
                                                                                    { let ml_reference_array1985 = source_ml_source2_array_read_value32129.clone(); let ml_reference_position1985 = instruction_target; let ml_reference_projection1985 = self::ml_array::optional_at(&*ml_reference_array1985.borrow(), self::ml_array::number_index(ml_reference_position1985)); ml_reference_projection1985 }
                                                                                };
                                                                                if (match (target_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(target_3.clone().expect("a proven present value"), match pending_state.clone() {
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value32131, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32131 = *source_ml_source2_data_property_value32131;
                                                                                        source_ml_source2_data_property_value32131.clone()
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value32132, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32132 = *source_ml_source2_data_property_value32132;
                                                                                        source_ml_source2_data_property_value32132.clone()
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(source_ml_source2_data_property_value32133, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32133 = *source_ml_source2_data_property_value32133;
                                                                                        source_ml_source2_data_property_value32133.clone()
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(source_ml_source2_data_property_value32134, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32134 = *source_ml_source2_data_property_value32134;
                                                                                        source_ml_source2_data_property_value32134.clone()
                                                                                    }
                                                                                }))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(invalid_target.clone())
                                                                                } else {
                                                                                    if (((operation != (self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121]))) || (destination_3 != pending_destination)) || (realm_4 != pending_realm)) {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(invalid_operation.clone())
                                                                                    } else {
                                                                                        {
                                                                                            let operands = self::read_java_script_descriptor_operands(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values1986 = { let ml_reference_value1987 = fields.borrow().clone(); ml_reference_value1987 }; let ml_callback1986 = { let frame = frame.clone(); std::rc::Rc::new(move |coordinate: f64| -> self::JavaScriptSemanticValue { ({
                                                                                                let source_ml_source2_array_read_value32136 = match frame.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value32135, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value32135.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_array_read_index32137 = coordinate;
                                                                                                    { let ml_reference_array1988 = source_ml_source2_array_read_value32136.clone(); let ml_reference_position1988 = source_ml_source2_array_read_index32137; let ml_reference_projection1988 = self::ml_array::optional_at(&*ml_reference_array1988.borrow(), self::ml_array::number_index(ml_reference_position1988)); ml_reference_projection1988 }
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values1986.iter().cloned().enumerate().map(|(ml_index1986, ml_value1986)| ml_callback1986(ml_value1986)).collect::<Vec<_>>() })));
                                                                                            {
                                                                                                let key = ({
                                                                                                    let source_ml_source2_array_read_value32139 = shared_constructor_field4.clone();
                                                                                                    { let ml_reference_array1989 = source_ml_source2_array_read_value32139.clone(); let ml_reference_position1989 = instruction_key; let ml_reference_projection1989 = self::ml_array::optional_at(&*ml_reference_array1989.borrow(), self::ml_array::number_index(ml_reference_position1989)); ml_reference_projection1989 }
                                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                                                if (((({ let ml_reference_value1990 = operands.borrow().clone(); ml_reference_value1990 }).0).1) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 114, 110, 97, 108, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 107, 101, 121, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 114, 101, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                } else {
                                                                                                    {
                                                                                                        let source_ml_source2_s32152 = match pending_state.clone() {
                                                                                                            self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, state_key, assignment, _, _) => {
                                                                                                                let state_key = *state_key;
                                                                                                                let assignment = *assignment;
                                                                                                                if (!(self::test_java_script_semantic_same_value(key.clone(), state_key.clone())) || !(self::test_java_script_property_descriptors_same_value(match match assignment.clone() {
                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(source_ml_source2_data_property_value32160, _) => {
                                                                                                                        let source_ml_source2_data_property_value32160 = *source_ml_source2_data_property_value32160;
                                                                                                                        source_ml_source2_data_property_value32160.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(source_ml_source2_data_property_value32161, _, _) => {
                                                                                                                        let source_ml_source2_data_property_value32161 = *source_ml_source2_data_property_value32161;
                                                                                                                        source_ml_source2_data_property_value32161.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(source_ml_source2_data_property_value32162, _, _) => {
                                                                                                                        let source_ml_source2_data_property_value32162 = *source_ml_source2_data_property_value32162;
                                                                                                                        source_ml_source2_data_property_value32162.clone()
                                                                                                                    }
                                                                                                                } {
                                                                                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value32163, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value32163.clone()
                                                                                                                    }
                                                                                                                }, (({ let ml_reference_value1991 = operands.borrow().clone(); ml_reference_value1991 }).0).0))) {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336Return(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 114, 110, 97, 108, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 107, 101, 121, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 114, 101, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                                } else {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336Next
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, state_key_2, descriptor, _, _, _) => {
                                                                                                                let state_key_2 = *state_key_2;
                                                                                                                if (!(self::test_java_script_semantic_same_value(key.clone(), state_key_2.clone())) || !(self::test_java_script_property_descriptors_same_value(descriptor.clone(), (({ let ml_reference_value1992 = operands.borrow().clone(); ml_reference_value1992 }).0).0))) {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336Return(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 114, 110, 97, 108, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 107, 101, 121, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 114, 101, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                                } else {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336Next
                                                                                                                }
                                                                                                            }
                                                                                                            _ => {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336Return(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 114, 110, 97, 108, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 107, 101, 121, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 114, 101, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                            }
                                                                                                        };
                                                                                                        match source_ml_source2_s32152.clone() {
                                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336Next => {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Next
                                                                                                            }
                                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336Return(source_ml_source1_temporary_result1336) => {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(source_ml_source1_temporary_result1336.clone())
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
                                                                        self::JavaScriptCallInstruction::PropertyDefinition(destination_4, instruction_target_2, _, _, realm_5, instruction_operation) => {
                                                                            {
                                                                                let target_4 = {
                                                                                    let source_ml_source2_array_read_value32165 = shared_constructor_field4.clone();
                                                                                    { let ml_reference_array1993 = source_ml_source2_array_read_value32165.clone(); let ml_reference_position1993 = instruction_target_2; let ml_reference_projection1993 = self::ml_array::optional_at(&*ml_reference_array1993.borrow(), self::ml_array::number_index(ml_reference_position1993)); ml_reference_projection1993 }
                                                                                };
                                                                                if (match (target_4.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(target_4.clone().expect("a proven present value"), match pending_state.clone() {
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value32167, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32167 = *source_ml_source2_data_property_value32167;
                                                                                        source_ml_source2_data_property_value32167.clone()
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value32168, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32168 = *source_ml_source2_data_property_value32168;
                                                                                        source_ml_source2_data_property_value32168.clone()
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(source_ml_source2_data_property_value32169, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32169 = *source_ml_source2_data_property_value32169;
                                                                                        source_ml_source2_data_property_value32169.clone()
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(source_ml_source2_data_property_value32170, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value32170 = *source_ml_source2_data_property_value32170;
                                                                                        source_ml_source2_data_property_value32170.clone()
                                                                                    }
                                                                                }))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(invalid_target.clone())
                                                                                } else {
                                                                                    if (((operation != ((((instruction_operation.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121])))) || (destination_4 != pending_destination)) || (realm_5 != pending_realm)) {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(invalid_operation.clone())
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Next
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(invalid_target.clone())
                                                                        }
                                                                    };
                                                                match source_ml_source2_s32106.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Next => {
                                                                            if (suspended && (match pending_state.clone() {
                                                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, _, _) => {
                                                                                    true
                                                                                }
                                                                                _ => {
                                                                                    false
                                                                                }
                                                                            } || ({
                                                                                let source_ml_source2_comparison_operand32180 = (((match pending_state.clone() {
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, _, source_ml_source2_data_property_value32177, _) => {
                                                                                        Some((((source_ml_source2_data_property_value32177.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone(),))
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, _, _, source_ml_source2_data_property_value32178, _) => {
                                                                                        Some((((source_ml_source2_data_property_value32178.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone(),))
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, source_ml_source2_data_property_value32179, _) => {
                                                                                        Some((((source_ml_source2_data_property_value32179.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone(),))
                                                                                    }
                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, _, _) => {
                                                                                        None::<(Option<self::ml_string::Text>,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone();
                                                                                match (source_ml_source2_comparison_operand32180.clone(), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                            } && match ((((match pending_state.clone() {
                                                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, _, _, source_ml_source2_data_property_value32182) => {
                                                                                    Some((((source_ml_source2_data_property_value32182.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone(),))
                                                                                }
                                                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, _, _, _, source_ml_source2_data_property_value32183) => {
                                                                                    Some((((source_ml_source2_data_property_value32183.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone(),))
                                                                                }
                                                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, _, source_ml_source2_data_property_value32184) => {
                                                                                    Some((((source_ml_source2_data_property_value32184.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone(),))
                                                                                }
                                                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, _, _) => {
                                                                                    None::<(Option<self::JavaScriptPropertyDefinitionCall>,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) {
                                                                                self::ml_string::Text::from_units(vec![97, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 115, 117, 115, 112, 101, 110, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114])
                                                                            } else {
                                                                                {
                                                                                    let source_ml_source2_s32208 = match pending_state.clone() {
                                                                                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, conversion, _, _) => {
                                                                                            let conversion = *conversion;
                                                                                            if !(self::test_java_script_semantic_same_value(match conversion.clone() {
                                                                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value32222, _, _) => {
                                                                                                    let source_ml_source2_data_property_value32222 = *source_ml_source2_data_property_value32222;
                                                                                                    source_ml_source2_data_property_value32222.clone()
                                                                                                }
                                                                                            }, shared_constructor_field2.clone())) {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335Return(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                                                            } else {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335Next
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, _, state_exotic_key, _, _) => {
                                                                                            {
                                                                                                let exotic_key = ((state_exotic_key.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                                                                                if (!(match (exotic_key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(self::test_java_script_semantic_same_value(exotic_key.clone().expect("a proven present value"), shared_constructor_field2.clone()))) {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335Return(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, assignment_2, _, _) => {
                                                                                            let assignment_2 = *assignment_2;
                                                                                            if !(self::test_java_script_semantic_same_value(match assignment_2.clone() {
                                                                                                self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(_, source_ml_source2_data_property_value32225) => {
                                                                                                    let source_ml_source2_data_property_value32225 = *source_ml_source2_data_property_value32225;
                                                                                                    source_ml_source2_data_property_value32225.clone()
                                                                                                }
                                                                                                self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(_, source_ml_source2_data_property_value32226, _) => {
                                                                                                    let source_ml_source2_data_property_value32226 = *source_ml_source2_data_property_value32226;
                                                                                                    source_ml_source2_data_property_value32226.clone()
                                                                                                }
                                                                                                self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(_, source_ml_source2_data_property_value32227, _) => {
                                                                                                    let source_ml_source2_data_property_value32227 = *source_ml_source2_data_property_value32227;
                                                                                                    source_ml_source2_data_property_value32227.clone()
                                                                                                }
                                                                                            }, shared_constructor_field2.clone())) {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108]))
                                                                                            } else {
                                                                                                if (suspended && match assignment_2.clone() {
                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(_, _, _) => {
                                                                                                        false
                                                                                                    }
                                                                                                    _ => {
                                                                                                        true
                                                                                                    }
                                                                                                }) {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 112, 104, 97, 115, 101]))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        _ => {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335Next
                                                                                        }
                                                                                    };
                                                                                    match source_ml_source2_s32208.clone() {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335Next => {
                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                        }
                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335Return(source_ml_source1_temporary_result1335) => {
                                                                                            source_ml_source1_temporary_result1335.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(source_ml_source1_temporary_result1335_2) => {
                                                                            source_ml_source1_temporary_result1335_2.clone()
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
                    self::JavaScriptCallPending::WaitNumber(destination_5, realm_6, state) => {
                        let state = *state;
                        if !(self::ml_array::includes(
                            &{
                                let ml_reference_value1994 =
                                    shared_constructor_field1.borrow().clone();
                                ml_reference_value1994
                            },
                            &realm_6.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )) {
                            self::ml_string::Text::from_units(vec![
                                112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114,
                                32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101,
                                116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114,
                                101, 97, 108, 109,
                            ])
                        } else {
                            {
                                let state_error = self::read_java_script_number_call_state_error(
                                    heap.clone(),
                                    state.clone(),
                                );
                                if (state_error != (self::ml_string::Text::from_units(vec![]))) {
                                    state_error.clone()
                                } else {
                                    if ((self::read_java_script_number_call_source_operation(
                                        fn_.clone(),
                                        frame.clone(),
                                        destination_5,
                                        realm_6.clone(),
                                        shared_constructor_field2.clone(),
                                        state.clone(),
                                    )) == (self::ml_string::Text::from_units(vec![])))
                                    {
                                        self::ml_string::Text::from_units(vec![
                                            112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109,
                                            98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115,
                                            105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115,
                                            32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                            115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116,
                                            114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99,
                                            101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111,
                                            108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109,
                                        ])
                                    } else {
                                        {
                                            let source_ml_source2_s32238 = if suspended {
                                                match state.clone() {
                                                        self::JavaScriptNumberCallState::NumberProtocolState(_) => {
                                                            self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337Next
                                                        }
                                                        self::JavaScriptNumberCallState::NumberPrimitiveValue(_) => {
                                                            self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337Return(self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
                                                        }
                                                    }
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337Next
                                            };
                                            match source_ml_source2_s32238.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337Next => {
                                                        self::ml_string::Text::from_units(vec![])
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337Return(source_ml_source1_temporary_result1337) => {
                                                        source_ml_source1_temporary_result1337.clone()
                                                    }
                                                }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallPending::WaitText(destination_6, realm_7, state_2) => {
                        let state_2 = *state_2;
                        if !(self::ml_array::includes(
                            &{
                                let ml_reference_value1995 =
                                    shared_constructor_field1.borrow().clone();
                                ml_reference_value1995
                            },
                            &realm_7.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )) {
                            self::ml_string::Text::from_units(vec![
                                112, 101, 110, 100, 105, 110, 103, 32, 116, 101, 120, 116, 32, 99,
                                111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97,
                                105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97,
                                108, 109,
                            ])
                        } else {
                            {
                                let state_error_2 = self::read_java_script_text_call_state_error(
                                    heap.clone(),
                                    state_2.clone(),
                                );
                                if (state_error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                    state_error_2.clone()
                                } else {
                                    if ((self::read_java_script_text_call_source_operation(
                                        fn_.clone(),
                                        frame.clone(),
                                        destination_6,
                                        realm_7.clone(),
                                        shared_constructor_field2.clone(),
                                        state_2.clone(),
                                    )) == (self::ml_string::Text::from_units(vec![])))
                                    {
                                        self::ml_string::Text::from_units(vec![
                                            112, 101, 110, 100, 105, 110, 103, 32, 116, 101, 120,
                                            116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111,
                                            117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117,
                                            99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105,
                                            118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32,
                                            97, 110, 100, 32, 114, 101, 97, 108, 109,
                                        ])
                                    } else {
                                        {
                                            let source_ml_source2_s32247 = if suspended {
                                                match state_2.clone() {
                                                        self::JavaScriptTextCallState::TextProtocolState(_) => {
                                                            self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338Next
                                                        }
                                                        self::JavaScriptTextCallState::TextPrimitiveValue(_) => {
                                                            self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338Return(self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 116, 114, 105, 110, 103, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
                                                        }
                                                    }
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338Next
                                            };
                                            match source_ml_source2_s32247.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338Next => {
                                                        self::ml_string::Text::from_units(vec![])
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338::MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338Return(source_ml_source1_temporary_result1338) => {
                                                        source_ml_source1_temporary_result1338.clone()
                                                    }
                                                }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => self::ml_string::Text::from_units(vec![]),
                }
            }
        },
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_pending_call_constructor_failure(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    callee: self::JavaScriptSemanticValue,
    retained: self::JavaScriptObjectFromEntriesErrorRequest,
) -> bool {
    match retained.clone() {
        self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
        ) => {
            if (match callee.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => false,
                _ => true,
            } || match retained.clone() {
                self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(
                    _,
                    _,
                    _,
                ) => false,
            }) {
                false
            } else {
                {
                    let owner = self::read_java_script_executable_call_function(
                        program.clone(),
                        heap.clone(),
                        (((match callee.clone() {
                            self::JavaScriptSemanticValue::Object(
                                source_ml_source2_data_property_value32255,
                                _,
                            ) => Some((source_ml_source2_data_property_value32255.clone(),)),
                            _ => None::<(self::ml::Big,)>,
                        })
                        .clone()
                        .expect("a proven present value"))
                        .0)
                            .clone(),
                    );
                    if match (owner.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        false
                    } else {
                        {
                            let actual = self::read_java_script_call_constructor_failure(
                                heap.clone(),
                                owner.clone().expect("a proven present value"),
                            );
                            if match (actual.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                false
                            } else {
                                (((shared_constructor_field1 == (match actual.clone().expect("a proven present value") {
                                    self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(source_ml_source2_data_property_value32259, _, _) => {
                                        source_ml_source2_data_property_value32259.clone()
                                    }
                                })) && (shared_constructor_field2 == (match actual.clone().expect("a proven present value") {
                                    self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(_, source_ml_source2_data_property_value32263, _) => {
                                        source_ml_source2_data_property_value32263.clone()
                                    }
                                }))) && (shared_constructor_field3 == (match actual.clone().expect("a proven present value") {
                                    self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(_, _, source_ml_source2_data_property_value32267) => {
                                        source_ml_source2_data_property_value32267.clone()
                                    }
                                })))
                            }
                        }
                    }
                }
            }
        }
    }
}
