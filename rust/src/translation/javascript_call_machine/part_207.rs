// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptObjectFromEntriesErrorRequest;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::accept_java_script_call_array_creation;
use super::accept_java_script_call_object_from_entries;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_machine_error;
use super::read_java_script_executable_call_function;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_source_exception_class;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::resume_java_script_array_creation_call;
use super::resume_java_script_array_creation_error;
use super::resume_java_script_object_from_entries_call;
use super::resume_java_script_object_from_entries_error;
use super::step_java_script_call_machine;
use super::throw_java_script_call_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_error_allocation(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    realm: self::ml::Big,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let error =
                    self::read_java_script_call_machine_error(program.clone(), machine.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field1.clone()),
                        error.clone(),
                    )
                } else {
                    {
                        let frame = {
                            let source_ml_source2_array_read_value36463 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index36464 = ({
                                    let ml_reference_array2407 = shared_constructor_field2.clone();
                                    let ml_reference_projection2407 =
                                        ml_reference_array2407.borrow().len() as f64;
                                    ml_reference_projection2407
                                } - 1f64);
                                {
                                    let ml_reference_array2408 =
                                        source_ml_source2_array_read_value36463.clone();
                                    let ml_reference_position2408 =
                                        source_ml_source2_array_read_index36464;
                                    let ml_reference_projection2408 = self::ml_array::optional_at(
                                        &*ml_reference_array2408.borrow(),
                                        self::ml_array::number_index(ml_reference_position2408),
                                    );
                                    ml_reference_projection2408
                                }
                            }
                        };
                        if match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116,
                                    105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115,
                                    32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101,
                                    32, 102, 114, 97, 109, 101,
                                ]),
                            )
                        } else {
                            {
                                let pending = match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value36466,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value36466 =
                                            *source_ml_source2_data_property_value36466;
                                        source_ml_source2_data_property_value36466.clone()
                                    }
                                };
                                match pending.clone() {
                                    self::JavaScriptCallPending::WaitArrayCreation(
                                        destination,
                                        pending_realm,
                                        state,
                                        _,
                                        _,
                                        error_2,
                                    ) => {
                                        let state = *state;
                                        {
                                            let request = self::step_java_script_call_machine(
                                                program.clone(),
                                                machine.clone(),
                                            );
                                            if ((((match request.clone() {
                                                self::JavaScriptCallMachineTransition::MachineAllocateError(_, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            } || (((((match request.clone() {
                                                self::JavaScriptCallMachineTransition::MachineAllocateError(_, source_ml_source2_data_property_value36474, _, _) => {
                                                    Some((source_ml_source2_data_property_value36474.clone(),))
                                                }
                                                _ => {
                                                    None::<(self::ml::Big,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone()) != realm)) || match value.clone() {
                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) || ((match value.clone() {
                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value36477) => {
                                                    Some((source_ml_source2_data_property_value36477,))
                                                }
                                                _ => {
                                                    None::<(bool,)>
                                                }
                                            }).clone().expect("a proven present value")).0) || ((self::read_java_script_ordinary_value_error(shared_constructor_field1.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![])))) {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 98, 101, 102, 111, 114, 101, 32, 99, 108, 101, 97, 110, 117, 112]))
                                            } else {
                                                {
                                                    let resumed = if !(match (((error_2.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                        self::resume_java_script_array_creation_call(shared_constructor_field1.clone(), state.clone(), self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(Box::new(value.clone())))
                                                    } else {
                                                        self::resume_java_script_array_creation_error(shared_constructor_field1.clone(), state.clone(), value.clone())
                                                    };
                                                    {
                                                        let discarded = ((error_2.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone();
                                                        {
                                                            let base = match pending.clone() {
                                                                self::JavaScriptCallPending::CallIdle => {
                                                                    self::JavaScriptCallPending::CallIdle
                                                                }
                                                                self::JavaScriptCallPending::WaitCall(source_ml_source2_nominal_rest_field36483) => {
                                                                    self::JavaScriptCallPending::WaitCall(source_ml_source2_nominal_rest_field36483)
                                                                }
                                                                self::JavaScriptCallPending::WaitGet(source_ml_source2_nominal_rest_field36484) => {
                                                                    self::JavaScriptCallPending::WaitGet(source_ml_source2_nominal_rest_field36484)
                                                                }
                                                                self::JavaScriptCallPending::WaitSet(source_ml_source2_nominal_rest_field36485) => {
                                                                    self::JavaScriptCallPending::WaitSet(source_ml_source2_nominal_rest_field36485)
                                                                }
                                                                self::JavaScriptCallPending::WaitHostSet(source_ml_source2_nominal_rest_field36486) => {
                                                                    self::JavaScriptCallPending::WaitHostSet(source_ml_source2_nominal_rest_field36486)
                                                                }
                                                                self::JavaScriptCallPending::WaitArgumentRest(source_ml_source2_nominal_rest_field36487, source_ml_source2_nominal_rest_field36488, source_ml_source2_nominal_rest_field36489) => {
                                                                    let source_ml_source2_nominal_rest_field36489 = *source_ml_source2_nominal_rest_field36489;
                                                                    self::JavaScriptCallPending::WaitArgumentRest(source_ml_source2_nominal_rest_field36487, source_ml_source2_nominal_rest_field36488.clone(), Box::new(source_ml_source2_nominal_rest_field36489.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitIteratorProtocol(source_ml_source2_nominal_rest_field36490, source_ml_source2_nominal_rest_field36491, source_ml_source2_nominal_rest_field36492, source_ml_source2_nominal_rest_field36493, source_ml_source2_nominal_rest_field36494, source_ml_source2_nominal_rest_field36495) => {
                                                                    let source_ml_source2_nominal_rest_field36495 = *source_ml_source2_nominal_rest_field36495;
                                                                    self::JavaScriptCallPending::WaitIteratorProtocol(source_ml_source2_nominal_rest_field36490, source_ml_source2_nominal_rest_field36491, source_ml_source2_nominal_rest_field36492, source_ml_source2_nominal_rest_field36493.clone(), source_ml_source2_nominal_rest_field36494.clone(), Box::new(source_ml_source2_nominal_rest_field36495.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArraySpread(source_ml_source2_nominal_rest_field36496, source_ml_source2_nominal_rest_field36497, source_ml_source2_nominal_rest_field36498) => {
                                                                    let source_ml_source2_nominal_rest_field36498 = *source_ml_source2_nominal_rest_field36498;
                                                                    self::JavaScriptCallPending::WaitArraySpread(source_ml_source2_nominal_rest_field36496, source_ml_source2_nominal_rest_field36497.clone(), Box::new(source_ml_source2_nominal_rest_field36498.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayIteratorNext(source_ml_source2_nominal_rest_field36499, source_ml_source2_nominal_rest_field36500, source_ml_source2_nominal_rest_field36501) => {
                                                                    let source_ml_source2_nominal_rest_field36501 = *source_ml_source2_nominal_rest_field36501;
                                                                    self::JavaScriptCallPending::WaitArrayIteratorNext(source_ml_source2_nominal_rest_field36499, source_ml_source2_nominal_rest_field36500.clone(), Box::new(source_ml_source2_nominal_rest_field36501.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitCollectionIteratorResult(source_ml_source2_nominal_rest_field36502, source_ml_source2_nominal_rest_field36503, source_ml_source2_nominal_rest_field36504) => {
                                                                    let source_ml_source2_nominal_rest_field36504 = *source_ml_source2_nominal_rest_field36504;
                                                                    self::JavaScriptCallPending::WaitCollectionIteratorResult(source_ml_source2_nominal_rest_field36502, source_ml_source2_nominal_rest_field36503.clone(), Box::new(source_ml_source2_nominal_rest_field36504.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayConcatenation(source_ml_source2_nominal_rest_field36505, source_ml_source2_nominal_rest_field36506, source_ml_source2_nominal_rest_field36507, source_ml_source2_nominal_rest_field36508) => {
                                                                    let source_ml_source2_nominal_rest_field36507 = *source_ml_source2_nominal_rest_field36507;
                                                                    self::JavaScriptCallPending::WaitArrayConcatenation(source_ml_source2_nominal_rest_field36505, source_ml_source2_nominal_rest_field36506.clone(), Box::new(source_ml_source2_nominal_rest_field36507.clone()), source_ml_source2_nominal_rest_field36508.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitArraySlice(source_ml_source2_nominal_rest_field36509, source_ml_source2_nominal_rest_field36510, source_ml_source2_nominal_rest_field36511, source_ml_source2_nominal_rest_field36512) => {
                                                                    let source_ml_source2_nominal_rest_field36511 = *source_ml_source2_nominal_rest_field36511;
                                                                    self::JavaScriptCallPending::WaitArraySlice(source_ml_source2_nominal_rest_field36509, source_ml_source2_nominal_rest_field36510.clone(), Box::new(source_ml_source2_nominal_rest_field36511.clone()), source_ml_source2_nominal_rest_field36512.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayTransform(source_ml_source2_nominal_rest_field36513, source_ml_source2_nominal_rest_field36514, source_ml_source2_nominal_rest_field36515, source_ml_source2_nominal_rest_field36516) => {
                                                                    let source_ml_source2_nominal_rest_field36515 = *source_ml_source2_nominal_rest_field36515;
                                                                    self::JavaScriptCallPending::WaitArrayTransform(source_ml_source2_nominal_rest_field36513, source_ml_source2_nominal_rest_field36514.clone(), Box::new(source_ml_source2_nominal_rest_field36515.clone()), source_ml_source2_nominal_rest_field36516.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitArraySearch(source_ml_source2_nominal_rest_field36517, source_ml_source2_nominal_rest_field36518, source_ml_source2_nominal_rest_field36519) => {
                                                                    let source_ml_source2_nominal_rest_field36519 = *source_ml_source2_nominal_rest_field36519;
                                                                    self::JavaScriptCallPending::WaitArraySearch(source_ml_source2_nominal_rest_field36517, source_ml_source2_nominal_rest_field36518.clone(), Box::new(source_ml_source2_nominal_rest_field36519.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitCollectionForEach(source_ml_source2_nominal_rest_field36520, source_ml_source2_nominal_rest_field36521, source_ml_source2_nominal_rest_field36522) => {
                                                                    let source_ml_source2_nominal_rest_field36522 = *source_ml_source2_nominal_rest_field36522;
                                                                    self::JavaScriptCallPending::WaitCollectionForEach(source_ml_source2_nominal_rest_field36520, source_ml_source2_nominal_rest_field36521.clone(), Box::new(source_ml_source2_nominal_rest_field36522.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayCallback(source_ml_source2_nominal_rest_field36523, source_ml_source2_nominal_rest_field36524, source_ml_source2_nominal_rest_field36525) => {
                                                                    let source_ml_source2_nominal_rest_field36525 = *source_ml_source2_nominal_rest_field36525;
                                                                    self::JavaScriptCallPending::WaitArrayCallback(source_ml_source2_nominal_rest_field36523, source_ml_source2_nominal_rest_field36524.clone(), Box::new(source_ml_source2_nominal_rest_field36525.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectSpread(source_ml_source2_nominal_rest_field36526, source_ml_source2_nominal_rest_field36527, source_ml_source2_nominal_rest_field36528) => {
                                                                    let source_ml_source2_nominal_rest_field36528 = *source_ml_source2_nominal_rest_field36528;
                                                                    self::JavaScriptCallPending::WaitObjectSpread(source_ml_source2_nominal_rest_field36526, source_ml_source2_nominal_rest_field36527.clone(), Box::new(source_ml_source2_nominal_rest_field36528.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectEnumeration(source_ml_source2_nominal_rest_field36529, source_ml_source2_nominal_rest_field36530, source_ml_source2_nominal_rest_field36531) => {
                                                                    let source_ml_source2_nominal_rest_field36531 = *source_ml_source2_nominal_rest_field36531;
                                                                    self::JavaScriptCallPending::WaitObjectEnumeration(source_ml_source2_nominal_rest_field36529, source_ml_source2_nominal_rest_field36530.clone(), Box::new(source_ml_source2_nominal_rest_field36531.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitPropertyDefinitions(source_ml_source2_nominal_rest_field36532, source_ml_source2_nominal_rest_field36533, source_ml_source2_nominal_rest_field36534) => {
                                                                    let source_ml_source2_nominal_rest_field36534 = *source_ml_source2_nominal_rest_field36534;
                                                                    self::JavaScriptCallPending::WaitPropertyDefinitions(source_ml_source2_nominal_rest_field36532, source_ml_source2_nominal_rest_field36533.clone(), Box::new(source_ml_source2_nominal_rest_field36534.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitPropertyReflection(source_ml_source2_nominal_rest_field36535, source_ml_source2_nominal_rest_field36536, source_ml_source2_nominal_rest_field36537) => {
                                                                    let source_ml_source2_nominal_rest_field36537 = *source_ml_source2_nominal_rest_field36537;
                                                                    self::JavaScriptCallPending::WaitPropertyReflection(source_ml_source2_nominal_rest_field36535, source_ml_source2_nominal_rest_field36536.clone(), Box::new(source_ml_source2_nominal_rest_field36537.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitInstanceOf(source_ml_source2_nominal_rest_field36538, source_ml_source2_nominal_rest_field36539, source_ml_source2_nominal_rest_field36540) => {
                                                                    let source_ml_source2_nominal_rest_field36540 = *source_ml_source2_nominal_rest_field36540;
                                                                    self::JavaScriptCallPending::WaitInstanceOf(source_ml_source2_nominal_rest_field36538, source_ml_source2_nominal_rest_field36539.clone(), Box::new(source_ml_source2_nominal_rest_field36540.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectIntegrity(source_ml_source2_nominal_rest_field36541, source_ml_source2_nominal_rest_field36542, source_ml_source2_nominal_rest_field36543) => {
                                                                    let source_ml_source2_nominal_rest_field36543 = *source_ml_source2_nominal_rest_field36543;
                                                                    self::JavaScriptCallPending::WaitObjectIntegrity(source_ml_source2_nominal_rest_field36541, source_ml_source2_nominal_rest_field36542.clone(), Box::new(source_ml_source2_nominal_rest_field36543.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectPrototype(source_ml_source2_nominal_rest_field36544, source_ml_source2_nominal_rest_field36545, source_ml_source2_nominal_rest_field36546) => {
                                                                    let source_ml_source2_nominal_rest_field36546 = *source_ml_source2_nominal_rest_field36546;
                                                                    self::JavaScriptCallPending::WaitObjectPrototype(source_ml_source2_nominal_rest_field36544, source_ml_source2_nominal_rest_field36545.clone(), Box::new(source_ml_source2_nominal_rest_field36546.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayAssignment(source_ml_source2_nominal_rest_field36547, source_ml_source2_nominal_rest_field36548, source_ml_source2_nominal_rest_field36549) => {
                                                                    let source_ml_source2_nominal_rest_field36549 = *source_ml_source2_nominal_rest_field36549;
                                                                    self::JavaScriptCallPending::WaitArrayAssignment(source_ml_source2_nominal_rest_field36547, source_ml_source2_nominal_rest_field36548.clone(), Box::new(source_ml_source2_nominal_rest_field36549.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitKey(source_ml_source2_nominal_rest_field36550, source_ml_source2_nominal_rest_field36551, source_ml_source2_nominal_rest_field36552) => {
                                                                    let source_ml_source2_nominal_rest_field36552 = *source_ml_source2_nominal_rest_field36552;
                                                                    self::JavaScriptCallPending::WaitKey(source_ml_source2_nominal_rest_field36550, source_ml_source2_nominal_rest_field36551.clone(), Box::new(source_ml_source2_nominal_rest_field36552.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitDefinition(source_ml_source2_nominal_rest_field36553, source_ml_source2_nominal_rest_field36554, source_ml_source2_nominal_rest_field36555) => {
                                                                    let source_ml_source2_nominal_rest_field36555 = *source_ml_source2_nominal_rest_field36555;
                                                                    self::JavaScriptCallPending::WaitDefinition(source_ml_source2_nominal_rest_field36553, source_ml_source2_nominal_rest_field36554.clone(), Box::new(source_ml_source2_nominal_rest_field36555.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitNumber(source_ml_source2_nominal_rest_field36556, source_ml_source2_nominal_rest_field36557, source_ml_source2_nominal_rest_field36558) => {
                                                                    let source_ml_source2_nominal_rest_field36558 = *source_ml_source2_nominal_rest_field36558;
                                                                    self::JavaScriptCallPending::WaitNumber(source_ml_source2_nominal_rest_field36556, source_ml_source2_nominal_rest_field36557.clone(), Box::new(source_ml_source2_nominal_rest_field36558.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitText(source_ml_source2_nominal_rest_field36559, source_ml_source2_nominal_rest_field36560, source_ml_source2_nominal_rest_field36561) => {
                                                                    let source_ml_source2_nominal_rest_field36561 = *source_ml_source2_nominal_rest_field36561;
                                                                    self::JavaScriptCallPending::WaitText(source_ml_source2_nominal_rest_field36559, source_ml_source2_nominal_rest_field36560.clone(), Box::new(source_ml_source2_nominal_rest_field36561.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayCreation(source_ml_source2_nominal_rest_field36562, source_ml_source2_nominal_rest_field36563, source_ml_source2_nominal_rest_field36564, source_ml_source2_nominal_rest_field36565, source_ml_source2_nominal_rest_field36566, _) => {
                                                                    let source_ml_source2_nominal_rest_field36564 = *source_ml_source2_nominal_rest_field36564;
                                                                    self::JavaScriptCallPending::WaitArrayCreation(source_ml_source2_nominal_rest_field36562, source_ml_source2_nominal_rest_field36563.clone(), Box::new(source_ml_source2_nominal_rest_field36564.clone()), source_ml_source2_nominal_rest_field36565.clone(), source_ml_source2_nominal_rest_field36566.clone(), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                                }
                                                                self::JavaScriptCallPending::WaitConstruction(source_ml_source2_nominal_rest_field36567, source_ml_source2_nominal_rest_field36568, source_ml_source2_nominal_rest_field36569, source_ml_source2_nominal_rest_field36570) => {
                                                                    let source_ml_source2_nominal_rest_field36569 = *source_ml_source2_nominal_rest_field36569;
                                                                    self::JavaScriptCallPending::WaitConstruction(source_ml_source2_nominal_rest_field36567, source_ml_source2_nominal_rest_field36568.clone(), Box::new(source_ml_source2_nominal_rest_field36569.clone()), source_ml_source2_nominal_rest_field36570.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitFunctionBind(source_ml_source2_nominal_rest_field36571, source_ml_source2_nominal_rest_field36572, source_ml_source2_nominal_rest_field36573) => {
                                                                    let source_ml_source2_nominal_rest_field36573 = *source_ml_source2_nominal_rest_field36573;
                                                                    self::JavaScriptCallPending::WaitFunctionBind(source_ml_source2_nominal_rest_field36571, source_ml_source2_nominal_rest_field36572.clone(), Box::new(source_ml_source2_nominal_rest_field36573.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitFunctionText(source_ml_source2_nominal_rest_field36574, source_ml_source2_nominal_rest_field36575, source_ml_source2_nominal_rest_field36576, source_ml_source2_nominal_rest_field36577) => {
                                                                    let source_ml_source2_nominal_rest_field36576 = *source_ml_source2_nominal_rest_field36576;
                                                                    self::JavaScriptCallPending::WaitFunctionText(source_ml_source2_nominal_rest_field36574, source_ml_source2_nominal_rest_field36575.clone(), Box::new(source_ml_source2_nominal_rest_field36576.clone()), source_ml_source2_nominal_rest_field36577.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitConstructorCheck(source_ml_source2_nominal_rest_field36578, source_ml_source2_nominal_rest_field36579, source_ml_source2_nominal_rest_field36580) => {
                                                                    let source_ml_source2_nominal_rest_field36579 = *source_ml_source2_nominal_rest_field36579;
                                                                    self::JavaScriptCallPending::WaitConstructorCheck(source_ml_source2_nominal_rest_field36578, Box::new(source_ml_source2_nominal_rest_field36579.clone()), source_ml_source2_nominal_rest_field36580)
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectFromEntries(source_ml_source2_nominal_rest_field36581, source_ml_source2_nominal_rest_field36582, source_ml_source2_nominal_rest_field36583, _) => {
                                                                    let source_ml_source2_nominal_rest_field36583 = *source_ml_source2_nominal_rest_field36583;
                                                                    self::JavaScriptCallPending::WaitObjectFromEntries(source_ml_source2_nominal_rest_field36581, source_ml_source2_nominal_rest_field36582.clone(), Box::new(source_ml_source2_nominal_rest_field36583.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                                }
                                                                self::JavaScriptCallPending::WaitPrimitiveWrapper(source_ml_source2_nominal_rest_field36584, source_ml_source2_nominal_rest_field36585) => {
                                                                    let source_ml_source2_nominal_rest_field36585 = *source_ml_source2_nominal_rest_field36585;
                                                                    self::JavaScriptCallPending::WaitPrimitiveWrapper(source_ml_source2_nominal_rest_field36584, Box::new(source_ml_source2_nominal_rest_field36585.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitSymbolAllocation(source_ml_source2_nominal_rest_field36586, source_ml_source2_nominal_rest_field36587, source_ml_source2_nominal_rest_field36588) => {
                                                                    let source_ml_source2_nominal_rest_field36587 = *source_ml_source2_nominal_rest_field36587;
                                                                    self::JavaScriptCallPending::WaitSymbolAllocation(source_ml_source2_nominal_rest_field36586, Box::new(source_ml_source2_nominal_rest_field36587.clone()), source_ml_source2_nominal_rest_field36588.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitSymbolDescription(source_ml_source2_nominal_rest_field36589, source_ml_source2_nominal_rest_field36590) => {
                                                                    let source_ml_source2_nominal_rest_field36590 = *source_ml_source2_nominal_rest_field36590;
                                                                    self::JavaScriptCallPending::WaitSymbolDescription(source_ml_source2_nominal_rest_field36589, Box::new(source_ml_source2_nominal_rest_field36590.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitFunctionNameSymbol(source_ml_source2_nominal_rest_field36591, source_ml_source2_nominal_rest_field36592, source_ml_source2_nominal_rest_field36593, source_ml_source2_nominal_rest_field36594, source_ml_source2_nominal_rest_field36595) => {
                                                                    let source_ml_source2_nominal_rest_field36592 = *source_ml_source2_nominal_rest_field36592;
                                                                    let source_ml_source2_nominal_rest_field36593 = *source_ml_source2_nominal_rest_field36593;
                                                                    self::JavaScriptCallPending::WaitFunctionNameSymbol(source_ml_source2_nominal_rest_field36591, Box::new(source_ml_source2_nominal_rest_field36592.clone()), Box::new(source_ml_source2_nominal_rest_field36593.clone()), source_ml_source2_nominal_rest_field36594.clone(), source_ml_source2_nominal_rest_field36595.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitSourceError(source_ml_source2_nominal_rest_field36596, source_ml_source2_nominal_rest_field36597, source_ml_source2_nominal_rest_field36598, source_ml_source2_nominal_rest_field36599) => {
                                                                    let source_ml_source2_nominal_rest_field36598 = *source_ml_source2_nominal_rest_field36598;
                                                                    self::JavaScriptCallPending::WaitSourceError(source_ml_source2_nominal_rest_field36596.clone(), source_ml_source2_nominal_rest_field36597.clone(), Box::new(source_ml_source2_nominal_rest_field36598.clone()), source_ml_source2_nominal_rest_field36599)
                                                                }
                                                                self::JavaScriptCallPending::WaitErrorValue(source_ml_source2_nominal_rest_field36600, source_ml_source2_nominal_rest_field36601, source_ml_source2_nominal_rest_field36602, source_ml_source2_nominal_rest_field36603, source_ml_source2_nominal_rest_field36604, source_ml_source2_nominal_rest_field36605, source_ml_source2_nominal_rest_field36606, source_ml_source2_nominal_rest_field36607, source_ml_source2_nominal_rest_field36608, source_ml_source2_nominal_rest_field36609) => {
                                                                    let source_ml_source2_nominal_rest_field36603 = *source_ml_source2_nominal_rest_field36603;
                                                                    let source_ml_source2_nominal_rest_field36604 = *source_ml_source2_nominal_rest_field36604;
                                                                    let source_ml_source2_nominal_rest_field36605 = *source_ml_source2_nominal_rest_field36605;
                                                                    let source_ml_source2_nominal_rest_field36606 = *source_ml_source2_nominal_rest_field36606;
                                                                    let source_ml_source2_nominal_rest_field36609 = *source_ml_source2_nominal_rest_field36609;
                                                                    self::JavaScriptCallPending::WaitErrorValue(source_ml_source2_nominal_rest_field36600, source_ml_source2_nominal_rest_field36601.clone(), source_ml_source2_nominal_rest_field36602.clone(), Box::new(source_ml_source2_nominal_rest_field36603.clone()), Box::new(source_ml_source2_nominal_rest_field36604.clone()), Box::new(source_ml_source2_nominal_rest_field36605.clone()), Box::new(source_ml_source2_nominal_rest_field36606.clone()), source_ml_source2_nominal_rest_field36607, source_ml_source2_nominal_rest_field36608, Box::new(source_ml_source2_nominal_rest_field36609.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitErrorIntrinsic(source_ml_source2_nominal_rest_field36610) => {
                                                                    self::JavaScriptCallPending::WaitErrorIntrinsic(source_ml_source2_nominal_rest_field36610)
                                                                }
                                                                self::JavaScriptCallPending::WaitHostCapability(source_ml_source2_nominal_rest_field36611, source_ml_source2_nominal_rest_field36612) => {
                                                                    let source_ml_source2_nominal_rest_field36612 = *source_ml_source2_nominal_rest_field36612;
                                                                    self::JavaScriptCallPending::WaitHostCapability(source_ml_source2_nominal_rest_field36611, Box::new(source_ml_source2_nominal_rest_field36612.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitError(source_ml_source2_nominal_rest_field36613, source_ml_source2_nominal_rest_field36614, source_ml_source2_nominal_rest_field36615) => {
                                                                    self::JavaScriptCallPending::WaitError(source_ml_source2_nominal_rest_field36613.clone(), source_ml_source2_nominal_rest_field36614.clone(), source_ml_source2_nominal_rest_field36615.clone())
                                                                }
                                                            };
                                                            self::accept_java_script_call_array_creation(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), base.clone())), destination, pending_realm.clone(), resumed.clone())
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallPending::WaitObjectFromEntries(
                                        destination_2,
                                        pending_realm_2,
                                        state_2,
                                        error_3,
                                    ) => {
                                        let state_2 = *state_2;
                                        {
                                            let request_2 = self::step_java_script_call_machine(
                                                program.clone(),
                                                machine.clone(),
                                            );
                                            if ((((match request_2.clone() {
                                                self::JavaScriptCallMachineTransition::MachineAllocateError(_, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            } || (((((match request_2.clone() {
                                                self::JavaScriptCallMachineTransition::MachineAllocateError(_, source_ml_source2_data_property_value36623, _, _) => {
                                                    Some((source_ml_source2_data_property_value36623.clone(),))
                                                }
                                                _ => {
                                                    None::<(self::ml::Big,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone()) != realm)) || ((self::read_java_script_ordinary_value_error(shared_constructor_field1.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![])))) || match value.clone() {
                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) || ((match value.clone() {
                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value36628) => {
                                                    Some((source_ml_source2_data_property_value36628,))
                                                }
                                                _ => {
                                                    None::<(bool,)>
                                                }
                                            }).clone().expect("a proven present value")).0) {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 98, 101, 102, 111, 114, 101, 32, 99, 108, 101, 97, 110, 117, 112]))
                                            } else {
                                                {
                                                    let resumed_2 = if !(match (((error_3.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                        self::resume_java_script_object_from_entries_call(shared_constructor_field1.clone(), state_2.clone(), self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(Box::new(value.clone())))
                                                    } else {
                                                        self::resume_java_script_object_from_entries_error(shared_constructor_field1.clone(), state_2.clone(), value.clone())
                                                    };
                                                    {
                                                        let discarded_2 = ((error_3.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone();
                                                        {
                                                            let base_2 = match pending.clone() {
                                                                self::JavaScriptCallPending::CallIdle => {
                                                                    self::JavaScriptCallPending::CallIdle
                                                                }
                                                                self::JavaScriptCallPending::WaitCall(source_ml_source2_nominal_rest_field36632) => {
                                                                    self::JavaScriptCallPending::WaitCall(source_ml_source2_nominal_rest_field36632)
                                                                }
                                                                self::JavaScriptCallPending::WaitGet(source_ml_source2_nominal_rest_field36633) => {
                                                                    self::JavaScriptCallPending::WaitGet(source_ml_source2_nominal_rest_field36633)
                                                                }
                                                                self::JavaScriptCallPending::WaitSet(source_ml_source2_nominal_rest_field36634) => {
                                                                    self::JavaScriptCallPending::WaitSet(source_ml_source2_nominal_rest_field36634)
                                                                }
                                                                self::JavaScriptCallPending::WaitHostSet(source_ml_source2_nominal_rest_field36635) => {
                                                                    self::JavaScriptCallPending::WaitHostSet(source_ml_source2_nominal_rest_field36635)
                                                                }
                                                                self::JavaScriptCallPending::WaitArgumentRest(source_ml_source2_nominal_rest_field36636, source_ml_source2_nominal_rest_field36637, source_ml_source2_nominal_rest_field36638) => {
                                                                    let source_ml_source2_nominal_rest_field36638 = *source_ml_source2_nominal_rest_field36638;
                                                                    self::JavaScriptCallPending::WaitArgumentRest(source_ml_source2_nominal_rest_field36636, source_ml_source2_nominal_rest_field36637.clone(), Box::new(source_ml_source2_nominal_rest_field36638.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitIteratorProtocol(source_ml_source2_nominal_rest_field36639, source_ml_source2_nominal_rest_field36640, source_ml_source2_nominal_rest_field36641, source_ml_source2_nominal_rest_field36642, source_ml_source2_nominal_rest_field36643, source_ml_source2_nominal_rest_field36644) => {
                                                                    let source_ml_source2_nominal_rest_field36644 = *source_ml_source2_nominal_rest_field36644;
                                                                    self::JavaScriptCallPending::WaitIteratorProtocol(source_ml_source2_nominal_rest_field36639, source_ml_source2_nominal_rest_field36640, source_ml_source2_nominal_rest_field36641, source_ml_source2_nominal_rest_field36642.clone(), source_ml_source2_nominal_rest_field36643.clone(), Box::new(source_ml_source2_nominal_rest_field36644.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArraySpread(source_ml_source2_nominal_rest_field36645, source_ml_source2_nominal_rest_field36646, source_ml_source2_nominal_rest_field36647) => {
                                                                    let source_ml_source2_nominal_rest_field36647 = *source_ml_source2_nominal_rest_field36647;
                                                                    self::JavaScriptCallPending::WaitArraySpread(source_ml_source2_nominal_rest_field36645, source_ml_source2_nominal_rest_field36646.clone(), Box::new(source_ml_source2_nominal_rest_field36647.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayIteratorNext(source_ml_source2_nominal_rest_field36648, source_ml_source2_nominal_rest_field36649, source_ml_source2_nominal_rest_field36650) => {
                                                                    let source_ml_source2_nominal_rest_field36650 = *source_ml_source2_nominal_rest_field36650;
                                                                    self::JavaScriptCallPending::WaitArrayIteratorNext(source_ml_source2_nominal_rest_field36648, source_ml_source2_nominal_rest_field36649.clone(), Box::new(source_ml_source2_nominal_rest_field36650.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitCollectionIteratorResult(source_ml_source2_nominal_rest_field36651, source_ml_source2_nominal_rest_field36652, source_ml_source2_nominal_rest_field36653) => {
                                                                    let source_ml_source2_nominal_rest_field36653 = *source_ml_source2_nominal_rest_field36653;
                                                                    self::JavaScriptCallPending::WaitCollectionIteratorResult(source_ml_source2_nominal_rest_field36651, source_ml_source2_nominal_rest_field36652.clone(), Box::new(source_ml_source2_nominal_rest_field36653.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayConcatenation(source_ml_source2_nominal_rest_field36654, source_ml_source2_nominal_rest_field36655, source_ml_source2_nominal_rest_field36656, source_ml_source2_nominal_rest_field36657) => {
                                                                    let source_ml_source2_nominal_rest_field36656 = *source_ml_source2_nominal_rest_field36656;
                                                                    self::JavaScriptCallPending::WaitArrayConcatenation(source_ml_source2_nominal_rest_field36654, source_ml_source2_nominal_rest_field36655.clone(), Box::new(source_ml_source2_nominal_rest_field36656.clone()), source_ml_source2_nominal_rest_field36657.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitArraySlice(source_ml_source2_nominal_rest_field36658, source_ml_source2_nominal_rest_field36659, source_ml_source2_nominal_rest_field36660, source_ml_source2_nominal_rest_field36661) => {
                                                                    let source_ml_source2_nominal_rest_field36660 = *source_ml_source2_nominal_rest_field36660;
                                                                    self::JavaScriptCallPending::WaitArraySlice(source_ml_source2_nominal_rest_field36658, source_ml_source2_nominal_rest_field36659.clone(), Box::new(source_ml_source2_nominal_rest_field36660.clone()), source_ml_source2_nominal_rest_field36661.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayTransform(source_ml_source2_nominal_rest_field36662, source_ml_source2_nominal_rest_field36663, source_ml_source2_nominal_rest_field36664, source_ml_source2_nominal_rest_field36665) => {
                                                                    let source_ml_source2_nominal_rest_field36664 = *source_ml_source2_nominal_rest_field36664;
                                                                    self::JavaScriptCallPending::WaitArrayTransform(source_ml_source2_nominal_rest_field36662, source_ml_source2_nominal_rest_field36663.clone(), Box::new(source_ml_source2_nominal_rest_field36664.clone()), source_ml_source2_nominal_rest_field36665.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitArraySearch(source_ml_source2_nominal_rest_field36666, source_ml_source2_nominal_rest_field36667, source_ml_source2_nominal_rest_field36668) => {
                                                                    let source_ml_source2_nominal_rest_field36668 = *source_ml_source2_nominal_rest_field36668;
                                                                    self::JavaScriptCallPending::WaitArraySearch(source_ml_source2_nominal_rest_field36666, source_ml_source2_nominal_rest_field36667.clone(), Box::new(source_ml_source2_nominal_rest_field36668.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitCollectionForEach(source_ml_source2_nominal_rest_field36669, source_ml_source2_nominal_rest_field36670, source_ml_source2_nominal_rest_field36671) => {
                                                                    let source_ml_source2_nominal_rest_field36671 = *source_ml_source2_nominal_rest_field36671;
                                                                    self::JavaScriptCallPending::WaitCollectionForEach(source_ml_source2_nominal_rest_field36669, source_ml_source2_nominal_rest_field36670.clone(), Box::new(source_ml_source2_nominal_rest_field36671.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayCallback(source_ml_source2_nominal_rest_field36672, source_ml_source2_nominal_rest_field36673, source_ml_source2_nominal_rest_field36674) => {
                                                                    let source_ml_source2_nominal_rest_field36674 = *source_ml_source2_nominal_rest_field36674;
                                                                    self::JavaScriptCallPending::WaitArrayCallback(source_ml_source2_nominal_rest_field36672, source_ml_source2_nominal_rest_field36673.clone(), Box::new(source_ml_source2_nominal_rest_field36674.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectSpread(source_ml_source2_nominal_rest_field36675, source_ml_source2_nominal_rest_field36676, source_ml_source2_nominal_rest_field36677) => {
                                                                    let source_ml_source2_nominal_rest_field36677 = *source_ml_source2_nominal_rest_field36677;
                                                                    self::JavaScriptCallPending::WaitObjectSpread(source_ml_source2_nominal_rest_field36675, source_ml_source2_nominal_rest_field36676.clone(), Box::new(source_ml_source2_nominal_rest_field36677.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectEnumeration(source_ml_source2_nominal_rest_field36678, source_ml_source2_nominal_rest_field36679, source_ml_source2_nominal_rest_field36680) => {
                                                                    let source_ml_source2_nominal_rest_field36680 = *source_ml_source2_nominal_rest_field36680;
                                                                    self::JavaScriptCallPending::WaitObjectEnumeration(source_ml_source2_nominal_rest_field36678, source_ml_source2_nominal_rest_field36679.clone(), Box::new(source_ml_source2_nominal_rest_field36680.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitPropertyDefinitions(source_ml_source2_nominal_rest_field36681, source_ml_source2_nominal_rest_field36682, source_ml_source2_nominal_rest_field36683) => {
                                                                    let source_ml_source2_nominal_rest_field36683 = *source_ml_source2_nominal_rest_field36683;
                                                                    self::JavaScriptCallPending::WaitPropertyDefinitions(source_ml_source2_nominal_rest_field36681, source_ml_source2_nominal_rest_field36682.clone(), Box::new(source_ml_source2_nominal_rest_field36683.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitPropertyReflection(source_ml_source2_nominal_rest_field36684, source_ml_source2_nominal_rest_field36685, source_ml_source2_nominal_rest_field36686) => {
                                                                    let source_ml_source2_nominal_rest_field36686 = *source_ml_source2_nominal_rest_field36686;
                                                                    self::JavaScriptCallPending::WaitPropertyReflection(source_ml_source2_nominal_rest_field36684, source_ml_source2_nominal_rest_field36685.clone(), Box::new(source_ml_source2_nominal_rest_field36686.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitInstanceOf(source_ml_source2_nominal_rest_field36687, source_ml_source2_nominal_rest_field36688, source_ml_source2_nominal_rest_field36689) => {
                                                                    let source_ml_source2_nominal_rest_field36689 = *source_ml_source2_nominal_rest_field36689;
                                                                    self::JavaScriptCallPending::WaitInstanceOf(source_ml_source2_nominal_rest_field36687, source_ml_source2_nominal_rest_field36688.clone(), Box::new(source_ml_source2_nominal_rest_field36689.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectIntegrity(source_ml_source2_nominal_rest_field36690, source_ml_source2_nominal_rest_field36691, source_ml_source2_nominal_rest_field36692) => {
                                                                    let source_ml_source2_nominal_rest_field36692 = *source_ml_source2_nominal_rest_field36692;
                                                                    self::JavaScriptCallPending::WaitObjectIntegrity(source_ml_source2_nominal_rest_field36690, source_ml_source2_nominal_rest_field36691.clone(), Box::new(source_ml_source2_nominal_rest_field36692.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectPrototype(source_ml_source2_nominal_rest_field36693, source_ml_source2_nominal_rest_field36694, source_ml_source2_nominal_rest_field36695) => {
                                                                    let source_ml_source2_nominal_rest_field36695 = *source_ml_source2_nominal_rest_field36695;
                                                                    self::JavaScriptCallPending::WaitObjectPrototype(source_ml_source2_nominal_rest_field36693, source_ml_source2_nominal_rest_field36694.clone(), Box::new(source_ml_source2_nominal_rest_field36695.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayAssignment(source_ml_source2_nominal_rest_field36696, source_ml_source2_nominal_rest_field36697, source_ml_source2_nominal_rest_field36698) => {
                                                                    let source_ml_source2_nominal_rest_field36698 = *source_ml_source2_nominal_rest_field36698;
                                                                    self::JavaScriptCallPending::WaitArrayAssignment(source_ml_source2_nominal_rest_field36696, source_ml_source2_nominal_rest_field36697.clone(), Box::new(source_ml_source2_nominal_rest_field36698.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitKey(source_ml_source2_nominal_rest_field36699, source_ml_source2_nominal_rest_field36700, source_ml_source2_nominal_rest_field36701) => {
                                                                    let source_ml_source2_nominal_rest_field36701 = *source_ml_source2_nominal_rest_field36701;
                                                                    self::JavaScriptCallPending::WaitKey(source_ml_source2_nominal_rest_field36699, source_ml_source2_nominal_rest_field36700.clone(), Box::new(source_ml_source2_nominal_rest_field36701.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitDefinition(source_ml_source2_nominal_rest_field36702, source_ml_source2_nominal_rest_field36703, source_ml_source2_nominal_rest_field36704) => {
                                                                    let source_ml_source2_nominal_rest_field36704 = *source_ml_source2_nominal_rest_field36704;
                                                                    self::JavaScriptCallPending::WaitDefinition(source_ml_source2_nominal_rest_field36702, source_ml_source2_nominal_rest_field36703.clone(), Box::new(source_ml_source2_nominal_rest_field36704.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitNumber(source_ml_source2_nominal_rest_field36705, source_ml_source2_nominal_rest_field36706, source_ml_source2_nominal_rest_field36707) => {
                                                                    let source_ml_source2_nominal_rest_field36707 = *source_ml_source2_nominal_rest_field36707;
                                                                    self::JavaScriptCallPending::WaitNumber(source_ml_source2_nominal_rest_field36705, source_ml_source2_nominal_rest_field36706.clone(), Box::new(source_ml_source2_nominal_rest_field36707.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitText(source_ml_source2_nominal_rest_field36708, source_ml_source2_nominal_rest_field36709, source_ml_source2_nominal_rest_field36710) => {
                                                                    let source_ml_source2_nominal_rest_field36710 = *source_ml_source2_nominal_rest_field36710;
                                                                    self::JavaScriptCallPending::WaitText(source_ml_source2_nominal_rest_field36708, source_ml_source2_nominal_rest_field36709.clone(), Box::new(source_ml_source2_nominal_rest_field36710.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayCreation(source_ml_source2_nominal_rest_field36711, source_ml_source2_nominal_rest_field36712, source_ml_source2_nominal_rest_field36713, source_ml_source2_nominal_rest_field36714, source_ml_source2_nominal_rest_field36715, _) => {
                                                                    let source_ml_source2_nominal_rest_field36713 = *source_ml_source2_nominal_rest_field36713;
                                                                    self::JavaScriptCallPending::WaitArrayCreation(source_ml_source2_nominal_rest_field36711, source_ml_source2_nominal_rest_field36712.clone(), Box::new(source_ml_source2_nominal_rest_field36713.clone()), source_ml_source2_nominal_rest_field36714.clone(), source_ml_source2_nominal_rest_field36715.clone(), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                                }
                                                                self::JavaScriptCallPending::WaitConstruction(source_ml_source2_nominal_rest_field36716, source_ml_source2_nominal_rest_field36717, source_ml_source2_nominal_rest_field36718, source_ml_source2_nominal_rest_field36719) => {
                                                                    let source_ml_source2_nominal_rest_field36718 = *source_ml_source2_nominal_rest_field36718;
                                                                    self::JavaScriptCallPending::WaitConstruction(source_ml_source2_nominal_rest_field36716, source_ml_source2_nominal_rest_field36717.clone(), Box::new(source_ml_source2_nominal_rest_field36718.clone()), source_ml_source2_nominal_rest_field36719.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitFunctionBind(source_ml_source2_nominal_rest_field36720, source_ml_source2_nominal_rest_field36721, source_ml_source2_nominal_rest_field36722) => {
                                                                    let source_ml_source2_nominal_rest_field36722 = *source_ml_source2_nominal_rest_field36722;
                                                                    self::JavaScriptCallPending::WaitFunctionBind(source_ml_source2_nominal_rest_field36720, source_ml_source2_nominal_rest_field36721.clone(), Box::new(source_ml_source2_nominal_rest_field36722.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitFunctionText(source_ml_source2_nominal_rest_field36723, source_ml_source2_nominal_rest_field36724, source_ml_source2_nominal_rest_field36725, source_ml_source2_nominal_rest_field36726) => {
                                                                    let source_ml_source2_nominal_rest_field36725 = *source_ml_source2_nominal_rest_field36725;
                                                                    self::JavaScriptCallPending::WaitFunctionText(source_ml_source2_nominal_rest_field36723, source_ml_source2_nominal_rest_field36724.clone(), Box::new(source_ml_source2_nominal_rest_field36725.clone()), source_ml_source2_nominal_rest_field36726.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitConstructorCheck(source_ml_source2_nominal_rest_field36727, source_ml_source2_nominal_rest_field36728, source_ml_source2_nominal_rest_field36729) => {
                                                                    let source_ml_source2_nominal_rest_field36728 = *source_ml_source2_nominal_rest_field36728;
                                                                    self::JavaScriptCallPending::WaitConstructorCheck(source_ml_source2_nominal_rest_field36727, Box::new(source_ml_source2_nominal_rest_field36728.clone()), source_ml_source2_nominal_rest_field36729)
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectFromEntries(source_ml_source2_nominal_rest_field36730, source_ml_source2_nominal_rest_field36731, source_ml_source2_nominal_rest_field36732, _) => {
                                                                    let source_ml_source2_nominal_rest_field36732 = *source_ml_source2_nominal_rest_field36732;
                                                                    self::JavaScriptCallPending::WaitObjectFromEntries(source_ml_source2_nominal_rest_field36730, source_ml_source2_nominal_rest_field36731.clone(), Box::new(source_ml_source2_nominal_rest_field36732.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                                }
                                                                self::JavaScriptCallPending::WaitPrimitiveWrapper(source_ml_source2_nominal_rest_field36733, source_ml_source2_nominal_rest_field36734) => {
                                                                    let source_ml_source2_nominal_rest_field36734 = *source_ml_source2_nominal_rest_field36734;
                                                                    self::JavaScriptCallPending::WaitPrimitiveWrapper(source_ml_source2_nominal_rest_field36733, Box::new(source_ml_source2_nominal_rest_field36734.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitSymbolAllocation(source_ml_source2_nominal_rest_field36735, source_ml_source2_nominal_rest_field36736, source_ml_source2_nominal_rest_field36737) => {
                                                                    let source_ml_source2_nominal_rest_field36736 = *source_ml_source2_nominal_rest_field36736;
                                                                    self::JavaScriptCallPending::WaitSymbolAllocation(source_ml_source2_nominal_rest_field36735, Box::new(source_ml_source2_nominal_rest_field36736.clone()), source_ml_source2_nominal_rest_field36737.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitSymbolDescription(source_ml_source2_nominal_rest_field36738, source_ml_source2_nominal_rest_field36739) => {
                                                                    let source_ml_source2_nominal_rest_field36739 = *source_ml_source2_nominal_rest_field36739;
                                                                    self::JavaScriptCallPending::WaitSymbolDescription(source_ml_source2_nominal_rest_field36738, Box::new(source_ml_source2_nominal_rest_field36739.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitFunctionNameSymbol(source_ml_source2_nominal_rest_field36740, source_ml_source2_nominal_rest_field36741, source_ml_source2_nominal_rest_field36742, source_ml_source2_nominal_rest_field36743, source_ml_source2_nominal_rest_field36744) => {
                                                                    let source_ml_source2_nominal_rest_field36741 = *source_ml_source2_nominal_rest_field36741;
                                                                    let source_ml_source2_nominal_rest_field36742 = *source_ml_source2_nominal_rest_field36742;
                                                                    self::JavaScriptCallPending::WaitFunctionNameSymbol(source_ml_source2_nominal_rest_field36740, Box::new(source_ml_source2_nominal_rest_field36741.clone()), Box::new(source_ml_source2_nominal_rest_field36742.clone()), source_ml_source2_nominal_rest_field36743.clone(), source_ml_source2_nominal_rest_field36744.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitSourceError(source_ml_source2_nominal_rest_field36745, source_ml_source2_nominal_rest_field36746, source_ml_source2_nominal_rest_field36747, source_ml_source2_nominal_rest_field36748) => {
                                                                    let source_ml_source2_nominal_rest_field36747 = *source_ml_source2_nominal_rest_field36747;
                                                                    self::JavaScriptCallPending::WaitSourceError(source_ml_source2_nominal_rest_field36745.clone(), source_ml_source2_nominal_rest_field36746.clone(), Box::new(source_ml_source2_nominal_rest_field36747.clone()), source_ml_source2_nominal_rest_field36748)
                                                                }
                                                                self::JavaScriptCallPending::WaitErrorValue(source_ml_source2_nominal_rest_field36749, source_ml_source2_nominal_rest_field36750, source_ml_source2_nominal_rest_field36751, source_ml_source2_nominal_rest_field36752, source_ml_source2_nominal_rest_field36753, source_ml_source2_nominal_rest_field36754, source_ml_source2_nominal_rest_field36755, source_ml_source2_nominal_rest_field36756, source_ml_source2_nominal_rest_field36757, source_ml_source2_nominal_rest_field36758) => {
                                                                    let source_ml_source2_nominal_rest_field36752 = *source_ml_source2_nominal_rest_field36752;
                                                                    let source_ml_source2_nominal_rest_field36753 = *source_ml_source2_nominal_rest_field36753;
                                                                    let source_ml_source2_nominal_rest_field36754 = *source_ml_source2_nominal_rest_field36754;
                                                                    let source_ml_source2_nominal_rest_field36755 = *source_ml_source2_nominal_rest_field36755;
                                                                    let source_ml_source2_nominal_rest_field36758 = *source_ml_source2_nominal_rest_field36758;
                                                                    self::JavaScriptCallPending::WaitErrorValue(source_ml_source2_nominal_rest_field36749, source_ml_source2_nominal_rest_field36750.clone(), source_ml_source2_nominal_rest_field36751.clone(), Box::new(source_ml_source2_nominal_rest_field36752.clone()), Box::new(source_ml_source2_nominal_rest_field36753.clone()), Box::new(source_ml_source2_nominal_rest_field36754.clone()), Box::new(source_ml_source2_nominal_rest_field36755.clone()), source_ml_source2_nominal_rest_field36756, source_ml_source2_nominal_rest_field36757, Box::new(source_ml_source2_nominal_rest_field36758.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitErrorIntrinsic(source_ml_source2_nominal_rest_field36759) => {
                                                                    self::JavaScriptCallPending::WaitErrorIntrinsic(source_ml_source2_nominal_rest_field36759)
                                                                }
                                                                self::JavaScriptCallPending::WaitHostCapability(source_ml_source2_nominal_rest_field36760, source_ml_source2_nominal_rest_field36761) => {
                                                                    let source_ml_source2_nominal_rest_field36761 = *source_ml_source2_nominal_rest_field36761;
                                                                    self::JavaScriptCallPending::WaitHostCapability(source_ml_source2_nominal_rest_field36760, Box::new(source_ml_source2_nominal_rest_field36761.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitError(source_ml_source2_nominal_rest_field36762, source_ml_source2_nominal_rest_field36763, source_ml_source2_nominal_rest_field36764) => {
                                                                    self::JavaScriptCallPending::WaitError(source_ml_source2_nominal_rest_field36762.clone(), source_ml_source2_nominal_rest_field36763.clone(), source_ml_source2_nominal_rest_field36764.clone())
                                                                }
                                                            };
                                                            self::accept_java_script_call_object_from_entries(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), base_2.clone())), destination_2, pending_realm_2.clone(), resumed_2.clone())
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => match pending.clone() {
                                        self::JavaScriptCallPending::WaitError(
                                            pending_realm_3,
                                            _,
                                            _,
                                        ) => {
                                            if (pending_realm_3 != realm) {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 114, 101, 97, 108, 109]))
                                            } else {
                                                {
                                                    let value_error =
                                                        self::read_java_script_ordinary_value_error(
                                                            shared_constructor_field1.clone(),
                                                            value.clone(),
                                                        );
                                                    if (value_error
                                                        != (self::ml_string::Text::from_units(
                                                            vec![],
                                                        )))
                                                    {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), value_error.clone())
                                                    } else {
                                                        match value.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, callable) => {
                                                                    if callable {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                                    } else {
                                                                        self::throw_java_script_call_value(machine.clone(), value.clone())
                                                                    }
                                                                }
                                                                _ => {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                                }
                                                            }
                                                    }
                                                }
                                            }
                                        }
                                        _ => self::JavaScriptCallMachineTransition::MachineInvalid(
                                            Box::new(shared_constructor_field1.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                69, 114, 114, 111, 114, 32, 114, 101, 115, 117,
                                                109, 112, 116, 105, 111, 110, 32, 114, 101, 113,
                                                117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97,
                                                99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105,
                                                110, 103, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                                                111, 110,
                                            ]),
                                        ),
                                    },
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
pub fn request_java_script_call_source_error(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    instruction: self::JavaScriptCallInstruction,
) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field8 = *shared_constructor_field8;
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    shared_constructor_field1,
                    shared_constructor_field2,
                ) => {
                    let shared_constructor_field1 = *shared_constructor_field1;
                    match instruction.clone() {
                        self::JavaScriptCallInstruction::Primitive(_, operation, arguments) => {
                            let error_class =
                                self::read_java_script_source_exception_class(&(operation));
                            if ((error_class == (self::ml_string::Text::from_units(vec![])))
                                || (({
                                    let ml_reference_array2409 = arguments.clone();
                                    let ml_reference_projection2409 =
                                        ml_reference_array2409.borrow().len() as f64;
                                    ml_reference_projection2409
                                }) != (2f64)))
                            {
                                self::JavaScriptCallMachineTransition::MachineInvalid(
                                    Box::new(shared_constructor_field1.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114,
                                        32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111,
                                        110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                                        115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115,
                                        115, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101,
                                        110, 116, 115,
                                    ]),
                                )
                            } else {
                                {
                                    let message = ({
                                        let source_ml_source2_array_read_value36781 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_array_read_index36782 = ({
                                                let source_ml_source2_array_read_index36780 = 0f64;
                                                {
                                                    let ml_reference_array2410 = arguments.clone();
                                                    let ml_reference_position2410 =
                                                        source_ml_source2_array_read_index36780;
                                                    let ml_reference_projection2410 =
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array2410.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position2410,
                                                            ),
                                                        );
                                                    ml_reference_projection2410
                                                }
                                            })
                                            .clone()
                                            .unwrap_or_else(|| (-1f64));
                                            {
                                                let ml_reference_array2411 =
                                                    source_ml_source2_array_read_value36781.clone();
                                                let ml_reference_position2411 =
                                                    source_ml_source2_array_read_index36782;
                                                let ml_reference_projection2411 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array2411.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position2411,
                                                        ),
                                                    );
                                                ml_reference_projection2411
                                            }
                                        }
                                    })
                                    .clone()
                                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                    {
                                        let present = ({
                                            let source_ml_source2_array_read_value36786 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_array_read_index36787 = ({
                                                    let source_ml_source2_array_read_index36785 =
                                                        1f64;
                                                    {
                                                        let ml_reference_array2412 =
                                                            arguments.clone();
                                                        let ml_reference_position2412 =
                                                            source_ml_source2_array_read_index36785;
                                                        let ml_reference_projection2412 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array2412.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position2412,
                                                                ),
                                                            );
                                                        ml_reference_projection2412
                                                    }
                                                })
                                                .clone()
                                                .unwrap_or_else(|| (-1f64));
                                                {
                                                    let ml_reference_array2413 =
                                                        source_ml_source2_array_read_value36786
                                                            .clone();
                                                    let ml_reference_position2413 =
                                                        source_ml_source2_array_read_index36787;
                                                    let ml_reference_projection2413 =
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array2413.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position2413,
                                                            ),
                                                        );
                                                    ml_reference_projection2413
                                                }
                                            }
                                        })
                                        .clone()
                                        .unwrap_or_else(|| {
                                            self::JavaScriptSemanticValue::Undefined
                                        });
                                        if (match message.clone() {
                                            self::JavaScriptSemanticValue::Text(_) => false,
                                            _ => true,
                                        } && match message.clone() {
                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                false
                                            }
                                            _ => true,
                                        }) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                                Box::new(shared_constructor_field1.clone()),
                                                self::ml_string::Text::from_units(vec![
                                                    115, 111, 117, 114, 99, 101, 32, 69, 114, 114,
                                                    111, 114, 32, 99, 111, 110, 115, 116, 114, 117,
                                                    99, 116, 105, 111, 110, 32, 114, 101, 113, 117,
                                                    105, 114, 101, 115, 32, 97, 32, 110, 111, 114,
                                                    109, 97, 108, 105, 122, 101, 100, 32, 85, 84,
                                                    70, 49, 54, 32, 109, 101, 115, 115, 97, 103,
                                                    101,
                                                ]),
                                            )
                                        } else {
                                            {
                                                let message_error =
                                                    self::read_java_script_ordinary_value_error(
                                                        shared_constructor_field1.clone(),
                                                        message.clone(),
                                                    );
                                                if (message_error
                                                    != (self::ml_string::Text::from_units(vec![])))
                                                {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), message_error.clone())
                                                } else {
                                                    match present.clone() {
                                                            self::JavaScriptSemanticValue::Boolean(value) => {
                                                                {
                                                                    let fn_ = self::read_java_script_executable_call_function(program.clone(), shared_constructor_field1.clone(), shared_constructor_field3.clone());
                                                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                                    } else {
                                                                        {
                                                                            let next = {
                                                                                let source_ml_source2_nominal_field36801 = shared_constructor_field3.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field36803 = shared_constructor_field4;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field36805 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field36807 = shared_constructor_field6.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field36809 = shared_constructor_field7.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field36811 = shared_constructor_field8.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field36813 = shared_constructor_field9.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field36815 = shared_constructor_field10.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field36817 = shared_constructor_field11.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field36819 = shared_constructor_field12.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override36821 = (shared_constructor_field4 + 1f64);
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override36822 = self::JavaScriptCallPending::WaitSourceError(match fn_.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value36799, _, _, _, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value36799.clone()
                                                                                                                                }
                                                                                                                            }, error_class.clone(), Box::new(message.clone()), value);
                                                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field36801.clone(), source_ml_source2_nominal_override36821, source_ml_source2_nominal_field36805.clone(), source_ml_source2_nominal_field36807.clone(), source_ml_source2_nominal_field36809.clone(), Box::new(source_ml_source2_nominal_override36822.clone()), source_ml_source2_nominal_field36813.clone(), source_ml_source2_nominal_field36815.clone(), source_ml_source2_nominal_field36817.clone(), source_ml_source2_nominal_field36819.clone())
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
                                                                            };
                                                                            self::JavaScriptCallMachineTransition::MachineAllocateSourceError(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), next.clone())), match fn_.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value36825, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value36825.clone()
                                                                                }
                                                                            }, error_class.clone(), Box::new(message.clone()), value)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101]))
                                                            }
                                                        }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => self::JavaScriptCallMachineTransition::MachineInvalid(
                            Box::new(shared_constructor_field1.clone()),
                            self::ml_string::Text::from_units(vec![
                                115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99,
                                111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101,
                                116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105,
                                110, 115, 116, 114, 117, 99, 116, 105, 111, 110,
                            ]),
                        ),
                    }
                }
            }
        }
    }
}
