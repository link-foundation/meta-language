// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
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
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::resume_java_script_array_creation_call;
use super::resume_java_script_object_from_entries_call;
use super::step_java_script_call_machine;
use super::test_java_script_error_intrinsic_mode;
use super::throw_java_script_array_creation;
use super::throw_java_script_call_value;
use super::throw_java_script_object_from_entries;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_error_allocation_abrupt(
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
                            let source_ml_source2_array_read_value37084 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index37085 = ({
                                    let ml_reference_array2442 = shared_constructor_field2.clone();
                                    let ml_reference_projection2442 =
                                        ml_reference_array2442.borrow().len() as f64;
                                    ml_reference_projection2442
                                } - 1f64);
                                {
                                    let ml_reference_array2443 =
                                        source_ml_source2_array_read_value37084.clone();
                                    let ml_reference_position2443 =
                                        source_ml_source2_array_read_index37085;
                                    let ml_reference_projection2443 = self::ml_array::optional_at(
                                        &*ml_reference_array2443.borrow(),
                                        self::ml_array::number_index(ml_reference_position2443),
                                    );
                                    ml_reference_projection2443
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
                                    97, 110, 32, 97, 98, 114, 117, 112, 116, 32, 97, 108, 108, 111,
                                    99, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32,
                                    110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                    117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100,
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
                                        source_ml_source2_data_property_value37087,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value37087 =
                                            *source_ml_source2_data_property_value37087;
                                        source_ml_source2_data_property_value37087.clone()
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
                                            if ((match request.clone() {
                                                self::JavaScriptCallMachineTransition::MachineAllocateError(_, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            } || (((((match request.clone() {
                                                self::JavaScriptCallMachineTransition::MachineAllocateError(_, source_ml_source2_data_property_value37095, _, _) => {
                                                    Some((source_ml_source2_data_property_value37095.clone(),))
                                                }
                                                _ => {
                                                    None::<(self::ml::Big,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone()) != realm)) || ((self::read_java_script_ordinary_value_error(shared_constructor_field1.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![])))) {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 97, 98, 114, 117, 112, 116, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 113, 117, 101, 115, 116]))
                                            } else {
                                                {
                                                    let resumed = if !(match (((error_2.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                        self::resume_java_script_array_creation_call(shared_constructor_field1.clone(), state.clone(), self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(Box::new(value.clone())))
                                                    } else {
                                                        self::throw_java_script_array_creation(shared_constructor_field1.clone(), state.clone(), value.clone())
                                                    };
                                                    {
                                                        let discarded = ((error_2.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone();
                                                        {
                                                            let base = match pending.clone() {
                                                                self::JavaScriptCallPending::CallIdle => {
                                                                    self::JavaScriptCallPending::CallIdle
                                                                }
                                                                self::JavaScriptCallPending::WaitCall(source_ml_source2_nominal_rest_field37101) => {
                                                                    self::JavaScriptCallPending::WaitCall(source_ml_source2_nominal_rest_field37101)
                                                                }
                                                                self::JavaScriptCallPending::WaitGet(source_ml_source2_nominal_rest_field37102) => {
                                                                    self::JavaScriptCallPending::WaitGet(source_ml_source2_nominal_rest_field37102)
                                                                }
                                                                self::JavaScriptCallPending::WaitSet(source_ml_source2_nominal_rest_field37103) => {
                                                                    self::JavaScriptCallPending::WaitSet(source_ml_source2_nominal_rest_field37103)
                                                                }
                                                                self::JavaScriptCallPending::WaitHostSet(source_ml_source2_nominal_rest_field37104) => {
                                                                    self::JavaScriptCallPending::WaitHostSet(source_ml_source2_nominal_rest_field37104)
                                                                }
                                                                self::JavaScriptCallPending::WaitArgumentRest(source_ml_source2_nominal_rest_field37105, source_ml_source2_nominal_rest_field37106, source_ml_source2_nominal_rest_field37107) => {
                                                                    let source_ml_source2_nominal_rest_field37107 = *source_ml_source2_nominal_rest_field37107;
                                                                    self::JavaScriptCallPending::WaitArgumentRest(source_ml_source2_nominal_rest_field37105, source_ml_source2_nominal_rest_field37106.clone(), Box::new(source_ml_source2_nominal_rest_field37107.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitIteratorProtocol(source_ml_source2_nominal_rest_field37108, source_ml_source2_nominal_rest_field37109, source_ml_source2_nominal_rest_field37110, source_ml_source2_nominal_rest_field37111, source_ml_source2_nominal_rest_field37112, source_ml_source2_nominal_rest_field37113) => {
                                                                    let source_ml_source2_nominal_rest_field37113 = *source_ml_source2_nominal_rest_field37113;
                                                                    self::JavaScriptCallPending::WaitIteratorProtocol(source_ml_source2_nominal_rest_field37108, source_ml_source2_nominal_rest_field37109, source_ml_source2_nominal_rest_field37110, source_ml_source2_nominal_rest_field37111.clone(), source_ml_source2_nominal_rest_field37112.clone(), Box::new(source_ml_source2_nominal_rest_field37113.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArraySpread(source_ml_source2_nominal_rest_field37114, source_ml_source2_nominal_rest_field37115, source_ml_source2_nominal_rest_field37116) => {
                                                                    let source_ml_source2_nominal_rest_field37116 = *source_ml_source2_nominal_rest_field37116;
                                                                    self::JavaScriptCallPending::WaitArraySpread(source_ml_source2_nominal_rest_field37114, source_ml_source2_nominal_rest_field37115.clone(), Box::new(source_ml_source2_nominal_rest_field37116.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayIteratorNext(source_ml_source2_nominal_rest_field37117, source_ml_source2_nominal_rest_field37118, source_ml_source2_nominal_rest_field37119) => {
                                                                    let source_ml_source2_nominal_rest_field37119 = *source_ml_source2_nominal_rest_field37119;
                                                                    self::JavaScriptCallPending::WaitArrayIteratorNext(source_ml_source2_nominal_rest_field37117, source_ml_source2_nominal_rest_field37118.clone(), Box::new(source_ml_source2_nominal_rest_field37119.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitCollectionIteratorResult(source_ml_source2_nominal_rest_field37120, source_ml_source2_nominal_rest_field37121, source_ml_source2_nominal_rest_field37122) => {
                                                                    let source_ml_source2_nominal_rest_field37122 = *source_ml_source2_nominal_rest_field37122;
                                                                    self::JavaScriptCallPending::WaitCollectionIteratorResult(source_ml_source2_nominal_rest_field37120, source_ml_source2_nominal_rest_field37121.clone(), Box::new(source_ml_source2_nominal_rest_field37122.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayConcatenation(source_ml_source2_nominal_rest_field37123, source_ml_source2_nominal_rest_field37124, source_ml_source2_nominal_rest_field37125, source_ml_source2_nominal_rest_field37126) => {
                                                                    let source_ml_source2_nominal_rest_field37125 = *source_ml_source2_nominal_rest_field37125;
                                                                    self::JavaScriptCallPending::WaitArrayConcatenation(source_ml_source2_nominal_rest_field37123, source_ml_source2_nominal_rest_field37124.clone(), Box::new(source_ml_source2_nominal_rest_field37125.clone()), source_ml_source2_nominal_rest_field37126.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitArraySlice(source_ml_source2_nominal_rest_field37127, source_ml_source2_nominal_rest_field37128, source_ml_source2_nominal_rest_field37129, source_ml_source2_nominal_rest_field37130) => {
                                                                    let source_ml_source2_nominal_rest_field37129 = *source_ml_source2_nominal_rest_field37129;
                                                                    self::JavaScriptCallPending::WaitArraySlice(source_ml_source2_nominal_rest_field37127, source_ml_source2_nominal_rest_field37128.clone(), Box::new(source_ml_source2_nominal_rest_field37129.clone()), source_ml_source2_nominal_rest_field37130.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayTransform(source_ml_source2_nominal_rest_field37131, source_ml_source2_nominal_rest_field37132, source_ml_source2_nominal_rest_field37133, source_ml_source2_nominal_rest_field37134) => {
                                                                    let source_ml_source2_nominal_rest_field37133 = *source_ml_source2_nominal_rest_field37133;
                                                                    self::JavaScriptCallPending::WaitArrayTransform(source_ml_source2_nominal_rest_field37131, source_ml_source2_nominal_rest_field37132.clone(), Box::new(source_ml_source2_nominal_rest_field37133.clone()), source_ml_source2_nominal_rest_field37134.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitArraySearch(source_ml_source2_nominal_rest_field37135, source_ml_source2_nominal_rest_field37136, source_ml_source2_nominal_rest_field37137) => {
                                                                    let source_ml_source2_nominal_rest_field37137 = *source_ml_source2_nominal_rest_field37137;
                                                                    self::JavaScriptCallPending::WaitArraySearch(source_ml_source2_nominal_rest_field37135, source_ml_source2_nominal_rest_field37136.clone(), Box::new(source_ml_source2_nominal_rest_field37137.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitCollectionForEach(source_ml_source2_nominal_rest_field37138, source_ml_source2_nominal_rest_field37139, source_ml_source2_nominal_rest_field37140) => {
                                                                    let source_ml_source2_nominal_rest_field37140 = *source_ml_source2_nominal_rest_field37140;
                                                                    self::JavaScriptCallPending::WaitCollectionForEach(source_ml_source2_nominal_rest_field37138, source_ml_source2_nominal_rest_field37139.clone(), Box::new(source_ml_source2_nominal_rest_field37140.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayCallback(source_ml_source2_nominal_rest_field37141, source_ml_source2_nominal_rest_field37142, source_ml_source2_nominal_rest_field37143) => {
                                                                    let source_ml_source2_nominal_rest_field37143 = *source_ml_source2_nominal_rest_field37143;
                                                                    self::JavaScriptCallPending::WaitArrayCallback(source_ml_source2_nominal_rest_field37141, source_ml_source2_nominal_rest_field37142.clone(), Box::new(source_ml_source2_nominal_rest_field37143.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectSpread(source_ml_source2_nominal_rest_field37144, source_ml_source2_nominal_rest_field37145, source_ml_source2_nominal_rest_field37146) => {
                                                                    let source_ml_source2_nominal_rest_field37146 = *source_ml_source2_nominal_rest_field37146;
                                                                    self::JavaScriptCallPending::WaitObjectSpread(source_ml_source2_nominal_rest_field37144, source_ml_source2_nominal_rest_field37145.clone(), Box::new(source_ml_source2_nominal_rest_field37146.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectEnumeration(source_ml_source2_nominal_rest_field37147, source_ml_source2_nominal_rest_field37148, source_ml_source2_nominal_rest_field37149) => {
                                                                    let source_ml_source2_nominal_rest_field37149 = *source_ml_source2_nominal_rest_field37149;
                                                                    self::JavaScriptCallPending::WaitObjectEnumeration(source_ml_source2_nominal_rest_field37147, source_ml_source2_nominal_rest_field37148.clone(), Box::new(source_ml_source2_nominal_rest_field37149.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitPropertyDefinitions(source_ml_source2_nominal_rest_field37150, source_ml_source2_nominal_rest_field37151, source_ml_source2_nominal_rest_field37152) => {
                                                                    let source_ml_source2_nominal_rest_field37152 = *source_ml_source2_nominal_rest_field37152;
                                                                    self::JavaScriptCallPending::WaitPropertyDefinitions(source_ml_source2_nominal_rest_field37150, source_ml_source2_nominal_rest_field37151.clone(), Box::new(source_ml_source2_nominal_rest_field37152.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitPropertyReflection(source_ml_source2_nominal_rest_field37153, source_ml_source2_nominal_rest_field37154, source_ml_source2_nominal_rest_field37155) => {
                                                                    let source_ml_source2_nominal_rest_field37155 = *source_ml_source2_nominal_rest_field37155;
                                                                    self::JavaScriptCallPending::WaitPropertyReflection(source_ml_source2_nominal_rest_field37153, source_ml_source2_nominal_rest_field37154.clone(), Box::new(source_ml_source2_nominal_rest_field37155.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitInstanceOf(source_ml_source2_nominal_rest_field37156, source_ml_source2_nominal_rest_field37157, source_ml_source2_nominal_rest_field37158) => {
                                                                    let source_ml_source2_nominal_rest_field37158 = *source_ml_source2_nominal_rest_field37158;
                                                                    self::JavaScriptCallPending::WaitInstanceOf(source_ml_source2_nominal_rest_field37156, source_ml_source2_nominal_rest_field37157.clone(), Box::new(source_ml_source2_nominal_rest_field37158.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectIntegrity(source_ml_source2_nominal_rest_field37159, source_ml_source2_nominal_rest_field37160, source_ml_source2_nominal_rest_field37161) => {
                                                                    let source_ml_source2_nominal_rest_field37161 = *source_ml_source2_nominal_rest_field37161;
                                                                    self::JavaScriptCallPending::WaitObjectIntegrity(source_ml_source2_nominal_rest_field37159, source_ml_source2_nominal_rest_field37160.clone(), Box::new(source_ml_source2_nominal_rest_field37161.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectPrototype(source_ml_source2_nominal_rest_field37162, source_ml_source2_nominal_rest_field37163, source_ml_source2_nominal_rest_field37164) => {
                                                                    let source_ml_source2_nominal_rest_field37164 = *source_ml_source2_nominal_rest_field37164;
                                                                    self::JavaScriptCallPending::WaitObjectPrototype(source_ml_source2_nominal_rest_field37162, source_ml_source2_nominal_rest_field37163.clone(), Box::new(source_ml_source2_nominal_rest_field37164.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayAssignment(source_ml_source2_nominal_rest_field37165, source_ml_source2_nominal_rest_field37166, source_ml_source2_nominal_rest_field37167) => {
                                                                    let source_ml_source2_nominal_rest_field37167 = *source_ml_source2_nominal_rest_field37167;
                                                                    self::JavaScriptCallPending::WaitArrayAssignment(source_ml_source2_nominal_rest_field37165, source_ml_source2_nominal_rest_field37166.clone(), Box::new(source_ml_source2_nominal_rest_field37167.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitKey(source_ml_source2_nominal_rest_field37168, source_ml_source2_nominal_rest_field37169, source_ml_source2_nominal_rest_field37170) => {
                                                                    let source_ml_source2_nominal_rest_field37170 = *source_ml_source2_nominal_rest_field37170;
                                                                    self::JavaScriptCallPending::WaitKey(source_ml_source2_nominal_rest_field37168, source_ml_source2_nominal_rest_field37169.clone(), Box::new(source_ml_source2_nominal_rest_field37170.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitDefinition(source_ml_source2_nominal_rest_field37171, source_ml_source2_nominal_rest_field37172, source_ml_source2_nominal_rest_field37173) => {
                                                                    let source_ml_source2_nominal_rest_field37173 = *source_ml_source2_nominal_rest_field37173;
                                                                    self::JavaScriptCallPending::WaitDefinition(source_ml_source2_nominal_rest_field37171, source_ml_source2_nominal_rest_field37172.clone(), Box::new(source_ml_source2_nominal_rest_field37173.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitNumber(source_ml_source2_nominal_rest_field37174, source_ml_source2_nominal_rest_field37175, source_ml_source2_nominal_rest_field37176) => {
                                                                    let source_ml_source2_nominal_rest_field37176 = *source_ml_source2_nominal_rest_field37176;
                                                                    self::JavaScriptCallPending::WaitNumber(source_ml_source2_nominal_rest_field37174, source_ml_source2_nominal_rest_field37175.clone(), Box::new(source_ml_source2_nominal_rest_field37176.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitText(source_ml_source2_nominal_rest_field37177, source_ml_source2_nominal_rest_field37178, source_ml_source2_nominal_rest_field37179) => {
                                                                    let source_ml_source2_nominal_rest_field37179 = *source_ml_source2_nominal_rest_field37179;
                                                                    self::JavaScriptCallPending::WaitText(source_ml_source2_nominal_rest_field37177, source_ml_source2_nominal_rest_field37178.clone(), Box::new(source_ml_source2_nominal_rest_field37179.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayCreation(source_ml_source2_nominal_rest_field37180, source_ml_source2_nominal_rest_field37181, source_ml_source2_nominal_rest_field37182, source_ml_source2_nominal_rest_field37183, source_ml_source2_nominal_rest_field37184, _) => {
                                                                    let source_ml_source2_nominal_rest_field37182 = *source_ml_source2_nominal_rest_field37182;
                                                                    self::JavaScriptCallPending::WaitArrayCreation(source_ml_source2_nominal_rest_field37180, source_ml_source2_nominal_rest_field37181.clone(), Box::new(source_ml_source2_nominal_rest_field37182.clone()), source_ml_source2_nominal_rest_field37183.clone(), source_ml_source2_nominal_rest_field37184.clone(), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                                }
                                                                self::JavaScriptCallPending::WaitConstruction(source_ml_source2_nominal_rest_field37185, source_ml_source2_nominal_rest_field37186, source_ml_source2_nominal_rest_field37187, source_ml_source2_nominal_rest_field37188) => {
                                                                    let source_ml_source2_nominal_rest_field37187 = *source_ml_source2_nominal_rest_field37187;
                                                                    self::JavaScriptCallPending::WaitConstruction(source_ml_source2_nominal_rest_field37185, source_ml_source2_nominal_rest_field37186.clone(), Box::new(source_ml_source2_nominal_rest_field37187.clone()), source_ml_source2_nominal_rest_field37188.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitFunctionBind(source_ml_source2_nominal_rest_field37189, source_ml_source2_nominal_rest_field37190, source_ml_source2_nominal_rest_field37191) => {
                                                                    let source_ml_source2_nominal_rest_field37191 = *source_ml_source2_nominal_rest_field37191;
                                                                    self::JavaScriptCallPending::WaitFunctionBind(source_ml_source2_nominal_rest_field37189, source_ml_source2_nominal_rest_field37190.clone(), Box::new(source_ml_source2_nominal_rest_field37191.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitFunctionText(source_ml_source2_nominal_rest_field37192, source_ml_source2_nominal_rest_field37193, source_ml_source2_nominal_rest_field37194, source_ml_source2_nominal_rest_field37195) => {
                                                                    let source_ml_source2_nominal_rest_field37194 = *source_ml_source2_nominal_rest_field37194;
                                                                    self::JavaScriptCallPending::WaitFunctionText(source_ml_source2_nominal_rest_field37192, source_ml_source2_nominal_rest_field37193.clone(), Box::new(source_ml_source2_nominal_rest_field37194.clone()), source_ml_source2_nominal_rest_field37195.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitConstructorCheck(source_ml_source2_nominal_rest_field37196, source_ml_source2_nominal_rest_field37197, source_ml_source2_nominal_rest_field37198) => {
                                                                    let source_ml_source2_nominal_rest_field37197 = *source_ml_source2_nominal_rest_field37197;
                                                                    self::JavaScriptCallPending::WaitConstructorCheck(source_ml_source2_nominal_rest_field37196, Box::new(source_ml_source2_nominal_rest_field37197.clone()), source_ml_source2_nominal_rest_field37198)
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectFromEntries(source_ml_source2_nominal_rest_field37199, source_ml_source2_nominal_rest_field37200, source_ml_source2_nominal_rest_field37201, _) => {
                                                                    let source_ml_source2_nominal_rest_field37201 = *source_ml_source2_nominal_rest_field37201;
                                                                    self::JavaScriptCallPending::WaitObjectFromEntries(source_ml_source2_nominal_rest_field37199, source_ml_source2_nominal_rest_field37200.clone(), Box::new(source_ml_source2_nominal_rest_field37201.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                                }
                                                                self::JavaScriptCallPending::WaitPrimitiveWrapper(source_ml_source2_nominal_rest_field37202, source_ml_source2_nominal_rest_field37203) => {
                                                                    let source_ml_source2_nominal_rest_field37203 = *source_ml_source2_nominal_rest_field37203;
                                                                    self::JavaScriptCallPending::WaitPrimitiveWrapper(source_ml_source2_nominal_rest_field37202, Box::new(source_ml_source2_nominal_rest_field37203.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitSymbolAllocation(source_ml_source2_nominal_rest_field37204, source_ml_source2_nominal_rest_field37205, source_ml_source2_nominal_rest_field37206) => {
                                                                    let source_ml_source2_nominal_rest_field37205 = *source_ml_source2_nominal_rest_field37205;
                                                                    self::JavaScriptCallPending::WaitSymbolAllocation(source_ml_source2_nominal_rest_field37204, Box::new(source_ml_source2_nominal_rest_field37205.clone()), source_ml_source2_nominal_rest_field37206.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitSymbolDescription(source_ml_source2_nominal_rest_field37207, source_ml_source2_nominal_rest_field37208) => {
                                                                    let source_ml_source2_nominal_rest_field37208 = *source_ml_source2_nominal_rest_field37208;
                                                                    self::JavaScriptCallPending::WaitSymbolDescription(source_ml_source2_nominal_rest_field37207, Box::new(source_ml_source2_nominal_rest_field37208.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitFunctionNameSymbol(source_ml_source2_nominal_rest_field37209, source_ml_source2_nominal_rest_field37210, source_ml_source2_nominal_rest_field37211, source_ml_source2_nominal_rest_field37212, source_ml_source2_nominal_rest_field37213) => {
                                                                    let source_ml_source2_nominal_rest_field37210 = *source_ml_source2_nominal_rest_field37210;
                                                                    let source_ml_source2_nominal_rest_field37211 = *source_ml_source2_nominal_rest_field37211;
                                                                    self::JavaScriptCallPending::WaitFunctionNameSymbol(source_ml_source2_nominal_rest_field37209, Box::new(source_ml_source2_nominal_rest_field37210.clone()), Box::new(source_ml_source2_nominal_rest_field37211.clone()), source_ml_source2_nominal_rest_field37212.clone(), source_ml_source2_nominal_rest_field37213.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitSourceError(source_ml_source2_nominal_rest_field37214, source_ml_source2_nominal_rest_field37215, source_ml_source2_nominal_rest_field37216, source_ml_source2_nominal_rest_field37217) => {
                                                                    let source_ml_source2_nominal_rest_field37216 = *source_ml_source2_nominal_rest_field37216;
                                                                    self::JavaScriptCallPending::WaitSourceError(source_ml_source2_nominal_rest_field37214.clone(), source_ml_source2_nominal_rest_field37215.clone(), Box::new(source_ml_source2_nominal_rest_field37216.clone()), source_ml_source2_nominal_rest_field37217)
                                                                }
                                                                self::JavaScriptCallPending::WaitErrorValue(source_ml_source2_nominal_rest_field37218, source_ml_source2_nominal_rest_field37219, source_ml_source2_nominal_rest_field37220, source_ml_source2_nominal_rest_field37221, source_ml_source2_nominal_rest_field37222, source_ml_source2_nominal_rest_field37223, source_ml_source2_nominal_rest_field37224, source_ml_source2_nominal_rest_field37225, source_ml_source2_nominal_rest_field37226, source_ml_source2_nominal_rest_field37227) => {
                                                                    let source_ml_source2_nominal_rest_field37221 = *source_ml_source2_nominal_rest_field37221;
                                                                    let source_ml_source2_nominal_rest_field37222 = *source_ml_source2_nominal_rest_field37222;
                                                                    let source_ml_source2_nominal_rest_field37223 = *source_ml_source2_nominal_rest_field37223;
                                                                    let source_ml_source2_nominal_rest_field37224 = *source_ml_source2_nominal_rest_field37224;
                                                                    let source_ml_source2_nominal_rest_field37227 = *source_ml_source2_nominal_rest_field37227;
                                                                    self::JavaScriptCallPending::WaitErrorValue(source_ml_source2_nominal_rest_field37218, source_ml_source2_nominal_rest_field37219.clone(), source_ml_source2_nominal_rest_field37220.clone(), Box::new(source_ml_source2_nominal_rest_field37221.clone()), Box::new(source_ml_source2_nominal_rest_field37222.clone()), Box::new(source_ml_source2_nominal_rest_field37223.clone()), Box::new(source_ml_source2_nominal_rest_field37224.clone()), source_ml_source2_nominal_rest_field37225, source_ml_source2_nominal_rest_field37226, Box::new(source_ml_source2_nominal_rest_field37227.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitErrorIntrinsic(source_ml_source2_nominal_rest_field37228) => {
                                                                    self::JavaScriptCallPending::WaitErrorIntrinsic(source_ml_source2_nominal_rest_field37228)
                                                                }
                                                                self::JavaScriptCallPending::WaitHostCapability(source_ml_source2_nominal_rest_field37229, source_ml_source2_nominal_rest_field37230) => {
                                                                    let source_ml_source2_nominal_rest_field37230 = *source_ml_source2_nominal_rest_field37230;
                                                                    self::JavaScriptCallPending::WaitHostCapability(source_ml_source2_nominal_rest_field37229, Box::new(source_ml_source2_nominal_rest_field37230.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitError(source_ml_source2_nominal_rest_field37231, source_ml_source2_nominal_rest_field37232, source_ml_source2_nominal_rest_field37233) => {
                                                                    self::JavaScriptCallPending::WaitError(source_ml_source2_nominal_rest_field37231.clone(), source_ml_source2_nominal_rest_field37232.clone(), source_ml_source2_nominal_rest_field37233.clone())
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
                                            if ((match request_2.clone() {
                                                self::JavaScriptCallMachineTransition::MachineAllocateError(_, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            } || (((((match request_2.clone() {
                                                self::JavaScriptCallMachineTransition::MachineAllocateError(_, source_ml_source2_data_property_value37241, _, _) => {
                                                    Some((source_ml_source2_data_property_value37241.clone(),))
                                                }
                                                _ => {
                                                    None::<(self::ml::Big,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone()) != realm)) || ((self::read_java_script_ordinary_value_error(shared_constructor_field1.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![])))) {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 97, 98, 114, 117, 112, 116, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 113, 117, 101, 115, 116, 32, 97, 110, 100, 32, 116, 104, 114, 111, 119, 110, 32, 118, 97, 108, 117, 101]))
                                            } else {
                                                {
                                                    let resumed_2 = if !(match (((error_3.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                        self::resume_java_script_object_from_entries_call(shared_constructor_field1.clone(), state_2.clone(), self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(Box::new(value.clone())))
                                                    } else {
                                                        self::throw_java_script_object_from_entries(shared_constructor_field1.clone(), state_2.clone(), value.clone())
                                                    };
                                                    {
                                                        let discarded_2 = ((error_3.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone();
                                                        {
                                                            let base_2 = match pending.clone() {
                                                                self::JavaScriptCallPending::CallIdle => {
                                                                    self::JavaScriptCallPending::CallIdle
                                                                }
                                                                self::JavaScriptCallPending::WaitCall(source_ml_source2_nominal_rest_field37247) => {
                                                                    self::JavaScriptCallPending::WaitCall(source_ml_source2_nominal_rest_field37247)
                                                                }
                                                                self::JavaScriptCallPending::WaitGet(source_ml_source2_nominal_rest_field37248) => {
                                                                    self::JavaScriptCallPending::WaitGet(source_ml_source2_nominal_rest_field37248)
                                                                }
                                                                self::JavaScriptCallPending::WaitSet(source_ml_source2_nominal_rest_field37249) => {
                                                                    self::JavaScriptCallPending::WaitSet(source_ml_source2_nominal_rest_field37249)
                                                                }
                                                                self::JavaScriptCallPending::WaitHostSet(source_ml_source2_nominal_rest_field37250) => {
                                                                    self::JavaScriptCallPending::WaitHostSet(source_ml_source2_nominal_rest_field37250)
                                                                }
                                                                self::JavaScriptCallPending::WaitArgumentRest(source_ml_source2_nominal_rest_field37251, source_ml_source2_nominal_rest_field37252, source_ml_source2_nominal_rest_field37253) => {
                                                                    let source_ml_source2_nominal_rest_field37253 = *source_ml_source2_nominal_rest_field37253;
                                                                    self::JavaScriptCallPending::WaitArgumentRest(source_ml_source2_nominal_rest_field37251, source_ml_source2_nominal_rest_field37252.clone(), Box::new(source_ml_source2_nominal_rest_field37253.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitIteratorProtocol(source_ml_source2_nominal_rest_field37254, source_ml_source2_nominal_rest_field37255, source_ml_source2_nominal_rest_field37256, source_ml_source2_nominal_rest_field37257, source_ml_source2_nominal_rest_field37258, source_ml_source2_nominal_rest_field37259) => {
                                                                    let source_ml_source2_nominal_rest_field37259 = *source_ml_source2_nominal_rest_field37259;
                                                                    self::JavaScriptCallPending::WaitIteratorProtocol(source_ml_source2_nominal_rest_field37254, source_ml_source2_nominal_rest_field37255, source_ml_source2_nominal_rest_field37256, source_ml_source2_nominal_rest_field37257.clone(), source_ml_source2_nominal_rest_field37258.clone(), Box::new(source_ml_source2_nominal_rest_field37259.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArraySpread(source_ml_source2_nominal_rest_field37260, source_ml_source2_nominal_rest_field37261, source_ml_source2_nominal_rest_field37262) => {
                                                                    let source_ml_source2_nominal_rest_field37262 = *source_ml_source2_nominal_rest_field37262;
                                                                    self::JavaScriptCallPending::WaitArraySpread(source_ml_source2_nominal_rest_field37260, source_ml_source2_nominal_rest_field37261.clone(), Box::new(source_ml_source2_nominal_rest_field37262.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayIteratorNext(source_ml_source2_nominal_rest_field37263, source_ml_source2_nominal_rest_field37264, source_ml_source2_nominal_rest_field37265) => {
                                                                    let source_ml_source2_nominal_rest_field37265 = *source_ml_source2_nominal_rest_field37265;
                                                                    self::JavaScriptCallPending::WaitArrayIteratorNext(source_ml_source2_nominal_rest_field37263, source_ml_source2_nominal_rest_field37264.clone(), Box::new(source_ml_source2_nominal_rest_field37265.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitCollectionIteratorResult(source_ml_source2_nominal_rest_field37266, source_ml_source2_nominal_rest_field37267, source_ml_source2_nominal_rest_field37268) => {
                                                                    let source_ml_source2_nominal_rest_field37268 = *source_ml_source2_nominal_rest_field37268;
                                                                    self::JavaScriptCallPending::WaitCollectionIteratorResult(source_ml_source2_nominal_rest_field37266, source_ml_source2_nominal_rest_field37267.clone(), Box::new(source_ml_source2_nominal_rest_field37268.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayConcatenation(source_ml_source2_nominal_rest_field37269, source_ml_source2_nominal_rest_field37270, source_ml_source2_nominal_rest_field37271, source_ml_source2_nominal_rest_field37272) => {
                                                                    let source_ml_source2_nominal_rest_field37271 = *source_ml_source2_nominal_rest_field37271;
                                                                    self::JavaScriptCallPending::WaitArrayConcatenation(source_ml_source2_nominal_rest_field37269, source_ml_source2_nominal_rest_field37270.clone(), Box::new(source_ml_source2_nominal_rest_field37271.clone()), source_ml_source2_nominal_rest_field37272.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitArraySlice(source_ml_source2_nominal_rest_field37273, source_ml_source2_nominal_rest_field37274, source_ml_source2_nominal_rest_field37275, source_ml_source2_nominal_rest_field37276) => {
                                                                    let source_ml_source2_nominal_rest_field37275 = *source_ml_source2_nominal_rest_field37275;
                                                                    self::JavaScriptCallPending::WaitArraySlice(source_ml_source2_nominal_rest_field37273, source_ml_source2_nominal_rest_field37274.clone(), Box::new(source_ml_source2_nominal_rest_field37275.clone()), source_ml_source2_nominal_rest_field37276.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayTransform(source_ml_source2_nominal_rest_field37277, source_ml_source2_nominal_rest_field37278, source_ml_source2_nominal_rest_field37279, source_ml_source2_nominal_rest_field37280) => {
                                                                    let source_ml_source2_nominal_rest_field37279 = *source_ml_source2_nominal_rest_field37279;
                                                                    self::JavaScriptCallPending::WaitArrayTransform(source_ml_source2_nominal_rest_field37277, source_ml_source2_nominal_rest_field37278.clone(), Box::new(source_ml_source2_nominal_rest_field37279.clone()), source_ml_source2_nominal_rest_field37280.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitArraySearch(source_ml_source2_nominal_rest_field37281, source_ml_source2_nominal_rest_field37282, source_ml_source2_nominal_rest_field37283) => {
                                                                    let source_ml_source2_nominal_rest_field37283 = *source_ml_source2_nominal_rest_field37283;
                                                                    self::JavaScriptCallPending::WaitArraySearch(source_ml_source2_nominal_rest_field37281, source_ml_source2_nominal_rest_field37282.clone(), Box::new(source_ml_source2_nominal_rest_field37283.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitCollectionForEach(source_ml_source2_nominal_rest_field37284, source_ml_source2_nominal_rest_field37285, source_ml_source2_nominal_rest_field37286) => {
                                                                    let source_ml_source2_nominal_rest_field37286 = *source_ml_source2_nominal_rest_field37286;
                                                                    self::JavaScriptCallPending::WaitCollectionForEach(source_ml_source2_nominal_rest_field37284, source_ml_source2_nominal_rest_field37285.clone(), Box::new(source_ml_source2_nominal_rest_field37286.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayCallback(source_ml_source2_nominal_rest_field37287, source_ml_source2_nominal_rest_field37288, source_ml_source2_nominal_rest_field37289) => {
                                                                    let source_ml_source2_nominal_rest_field37289 = *source_ml_source2_nominal_rest_field37289;
                                                                    self::JavaScriptCallPending::WaitArrayCallback(source_ml_source2_nominal_rest_field37287, source_ml_source2_nominal_rest_field37288.clone(), Box::new(source_ml_source2_nominal_rest_field37289.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectSpread(source_ml_source2_nominal_rest_field37290, source_ml_source2_nominal_rest_field37291, source_ml_source2_nominal_rest_field37292) => {
                                                                    let source_ml_source2_nominal_rest_field37292 = *source_ml_source2_nominal_rest_field37292;
                                                                    self::JavaScriptCallPending::WaitObjectSpread(source_ml_source2_nominal_rest_field37290, source_ml_source2_nominal_rest_field37291.clone(), Box::new(source_ml_source2_nominal_rest_field37292.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectEnumeration(source_ml_source2_nominal_rest_field37293, source_ml_source2_nominal_rest_field37294, source_ml_source2_nominal_rest_field37295) => {
                                                                    let source_ml_source2_nominal_rest_field37295 = *source_ml_source2_nominal_rest_field37295;
                                                                    self::JavaScriptCallPending::WaitObjectEnumeration(source_ml_source2_nominal_rest_field37293, source_ml_source2_nominal_rest_field37294.clone(), Box::new(source_ml_source2_nominal_rest_field37295.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitPropertyDefinitions(source_ml_source2_nominal_rest_field37296, source_ml_source2_nominal_rest_field37297, source_ml_source2_nominal_rest_field37298) => {
                                                                    let source_ml_source2_nominal_rest_field37298 = *source_ml_source2_nominal_rest_field37298;
                                                                    self::JavaScriptCallPending::WaitPropertyDefinitions(source_ml_source2_nominal_rest_field37296, source_ml_source2_nominal_rest_field37297.clone(), Box::new(source_ml_source2_nominal_rest_field37298.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitPropertyReflection(source_ml_source2_nominal_rest_field37299, source_ml_source2_nominal_rest_field37300, source_ml_source2_nominal_rest_field37301) => {
                                                                    let source_ml_source2_nominal_rest_field37301 = *source_ml_source2_nominal_rest_field37301;
                                                                    self::JavaScriptCallPending::WaitPropertyReflection(source_ml_source2_nominal_rest_field37299, source_ml_source2_nominal_rest_field37300.clone(), Box::new(source_ml_source2_nominal_rest_field37301.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitInstanceOf(source_ml_source2_nominal_rest_field37302, source_ml_source2_nominal_rest_field37303, source_ml_source2_nominal_rest_field37304) => {
                                                                    let source_ml_source2_nominal_rest_field37304 = *source_ml_source2_nominal_rest_field37304;
                                                                    self::JavaScriptCallPending::WaitInstanceOf(source_ml_source2_nominal_rest_field37302, source_ml_source2_nominal_rest_field37303.clone(), Box::new(source_ml_source2_nominal_rest_field37304.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectIntegrity(source_ml_source2_nominal_rest_field37305, source_ml_source2_nominal_rest_field37306, source_ml_source2_nominal_rest_field37307) => {
                                                                    let source_ml_source2_nominal_rest_field37307 = *source_ml_source2_nominal_rest_field37307;
                                                                    self::JavaScriptCallPending::WaitObjectIntegrity(source_ml_source2_nominal_rest_field37305, source_ml_source2_nominal_rest_field37306.clone(), Box::new(source_ml_source2_nominal_rest_field37307.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectPrototype(source_ml_source2_nominal_rest_field37308, source_ml_source2_nominal_rest_field37309, source_ml_source2_nominal_rest_field37310) => {
                                                                    let source_ml_source2_nominal_rest_field37310 = *source_ml_source2_nominal_rest_field37310;
                                                                    self::JavaScriptCallPending::WaitObjectPrototype(source_ml_source2_nominal_rest_field37308, source_ml_source2_nominal_rest_field37309.clone(), Box::new(source_ml_source2_nominal_rest_field37310.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayAssignment(source_ml_source2_nominal_rest_field37311, source_ml_source2_nominal_rest_field37312, source_ml_source2_nominal_rest_field37313) => {
                                                                    let source_ml_source2_nominal_rest_field37313 = *source_ml_source2_nominal_rest_field37313;
                                                                    self::JavaScriptCallPending::WaitArrayAssignment(source_ml_source2_nominal_rest_field37311, source_ml_source2_nominal_rest_field37312.clone(), Box::new(source_ml_source2_nominal_rest_field37313.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitKey(source_ml_source2_nominal_rest_field37314, source_ml_source2_nominal_rest_field37315, source_ml_source2_nominal_rest_field37316) => {
                                                                    let source_ml_source2_nominal_rest_field37316 = *source_ml_source2_nominal_rest_field37316;
                                                                    self::JavaScriptCallPending::WaitKey(source_ml_source2_nominal_rest_field37314, source_ml_source2_nominal_rest_field37315.clone(), Box::new(source_ml_source2_nominal_rest_field37316.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitDefinition(source_ml_source2_nominal_rest_field37317, source_ml_source2_nominal_rest_field37318, source_ml_source2_nominal_rest_field37319) => {
                                                                    let source_ml_source2_nominal_rest_field37319 = *source_ml_source2_nominal_rest_field37319;
                                                                    self::JavaScriptCallPending::WaitDefinition(source_ml_source2_nominal_rest_field37317, source_ml_source2_nominal_rest_field37318.clone(), Box::new(source_ml_source2_nominal_rest_field37319.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitNumber(source_ml_source2_nominal_rest_field37320, source_ml_source2_nominal_rest_field37321, source_ml_source2_nominal_rest_field37322) => {
                                                                    let source_ml_source2_nominal_rest_field37322 = *source_ml_source2_nominal_rest_field37322;
                                                                    self::JavaScriptCallPending::WaitNumber(source_ml_source2_nominal_rest_field37320, source_ml_source2_nominal_rest_field37321.clone(), Box::new(source_ml_source2_nominal_rest_field37322.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitText(source_ml_source2_nominal_rest_field37323, source_ml_source2_nominal_rest_field37324, source_ml_source2_nominal_rest_field37325) => {
                                                                    let source_ml_source2_nominal_rest_field37325 = *source_ml_source2_nominal_rest_field37325;
                                                                    self::JavaScriptCallPending::WaitText(source_ml_source2_nominal_rest_field37323, source_ml_source2_nominal_rest_field37324.clone(), Box::new(source_ml_source2_nominal_rest_field37325.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitArrayCreation(source_ml_source2_nominal_rest_field37326, source_ml_source2_nominal_rest_field37327, source_ml_source2_nominal_rest_field37328, source_ml_source2_nominal_rest_field37329, source_ml_source2_nominal_rest_field37330, _) => {
                                                                    let source_ml_source2_nominal_rest_field37328 = *source_ml_source2_nominal_rest_field37328;
                                                                    self::JavaScriptCallPending::WaitArrayCreation(source_ml_source2_nominal_rest_field37326, source_ml_source2_nominal_rest_field37327.clone(), Box::new(source_ml_source2_nominal_rest_field37328.clone()), source_ml_source2_nominal_rest_field37329.clone(), source_ml_source2_nominal_rest_field37330.clone(), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                                }
                                                                self::JavaScriptCallPending::WaitConstruction(source_ml_source2_nominal_rest_field37331, source_ml_source2_nominal_rest_field37332, source_ml_source2_nominal_rest_field37333, source_ml_source2_nominal_rest_field37334) => {
                                                                    let source_ml_source2_nominal_rest_field37333 = *source_ml_source2_nominal_rest_field37333;
                                                                    self::JavaScriptCallPending::WaitConstruction(source_ml_source2_nominal_rest_field37331, source_ml_source2_nominal_rest_field37332.clone(), Box::new(source_ml_source2_nominal_rest_field37333.clone()), source_ml_source2_nominal_rest_field37334.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitFunctionBind(source_ml_source2_nominal_rest_field37335, source_ml_source2_nominal_rest_field37336, source_ml_source2_nominal_rest_field37337) => {
                                                                    let source_ml_source2_nominal_rest_field37337 = *source_ml_source2_nominal_rest_field37337;
                                                                    self::JavaScriptCallPending::WaitFunctionBind(source_ml_source2_nominal_rest_field37335, source_ml_source2_nominal_rest_field37336.clone(), Box::new(source_ml_source2_nominal_rest_field37337.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitFunctionText(source_ml_source2_nominal_rest_field37338, source_ml_source2_nominal_rest_field37339, source_ml_source2_nominal_rest_field37340, source_ml_source2_nominal_rest_field37341) => {
                                                                    let source_ml_source2_nominal_rest_field37340 = *source_ml_source2_nominal_rest_field37340;
                                                                    self::JavaScriptCallPending::WaitFunctionText(source_ml_source2_nominal_rest_field37338, source_ml_source2_nominal_rest_field37339.clone(), Box::new(source_ml_source2_nominal_rest_field37340.clone()), source_ml_source2_nominal_rest_field37341.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitConstructorCheck(source_ml_source2_nominal_rest_field37342, source_ml_source2_nominal_rest_field37343, source_ml_source2_nominal_rest_field37344) => {
                                                                    let source_ml_source2_nominal_rest_field37343 = *source_ml_source2_nominal_rest_field37343;
                                                                    self::JavaScriptCallPending::WaitConstructorCheck(source_ml_source2_nominal_rest_field37342, Box::new(source_ml_source2_nominal_rest_field37343.clone()), source_ml_source2_nominal_rest_field37344)
                                                                }
                                                                self::JavaScriptCallPending::WaitObjectFromEntries(source_ml_source2_nominal_rest_field37345, source_ml_source2_nominal_rest_field37346, source_ml_source2_nominal_rest_field37347, _) => {
                                                                    let source_ml_source2_nominal_rest_field37347 = *source_ml_source2_nominal_rest_field37347;
                                                                    self::JavaScriptCallPending::WaitObjectFromEntries(source_ml_source2_nominal_rest_field37345, source_ml_source2_nominal_rest_field37346.clone(), Box::new(source_ml_source2_nominal_rest_field37347.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                                }
                                                                self::JavaScriptCallPending::WaitPrimitiveWrapper(source_ml_source2_nominal_rest_field37348, source_ml_source2_nominal_rest_field37349) => {
                                                                    let source_ml_source2_nominal_rest_field37349 = *source_ml_source2_nominal_rest_field37349;
                                                                    self::JavaScriptCallPending::WaitPrimitiveWrapper(source_ml_source2_nominal_rest_field37348, Box::new(source_ml_source2_nominal_rest_field37349.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitSymbolAllocation(source_ml_source2_nominal_rest_field37350, source_ml_source2_nominal_rest_field37351, source_ml_source2_nominal_rest_field37352) => {
                                                                    let source_ml_source2_nominal_rest_field37351 = *source_ml_source2_nominal_rest_field37351;
                                                                    self::JavaScriptCallPending::WaitSymbolAllocation(source_ml_source2_nominal_rest_field37350, Box::new(source_ml_source2_nominal_rest_field37351.clone()), source_ml_source2_nominal_rest_field37352.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitSymbolDescription(source_ml_source2_nominal_rest_field37353, source_ml_source2_nominal_rest_field37354) => {
                                                                    let source_ml_source2_nominal_rest_field37354 = *source_ml_source2_nominal_rest_field37354;
                                                                    self::JavaScriptCallPending::WaitSymbolDescription(source_ml_source2_nominal_rest_field37353, Box::new(source_ml_source2_nominal_rest_field37354.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitFunctionNameSymbol(source_ml_source2_nominal_rest_field37355, source_ml_source2_nominal_rest_field37356, source_ml_source2_nominal_rest_field37357, source_ml_source2_nominal_rest_field37358, source_ml_source2_nominal_rest_field37359) => {
                                                                    let source_ml_source2_nominal_rest_field37356 = *source_ml_source2_nominal_rest_field37356;
                                                                    let source_ml_source2_nominal_rest_field37357 = *source_ml_source2_nominal_rest_field37357;
                                                                    self::JavaScriptCallPending::WaitFunctionNameSymbol(source_ml_source2_nominal_rest_field37355, Box::new(source_ml_source2_nominal_rest_field37356.clone()), Box::new(source_ml_source2_nominal_rest_field37357.clone()), source_ml_source2_nominal_rest_field37358.clone(), source_ml_source2_nominal_rest_field37359.clone())
                                                                }
                                                                self::JavaScriptCallPending::WaitSourceError(source_ml_source2_nominal_rest_field37360, source_ml_source2_nominal_rest_field37361, source_ml_source2_nominal_rest_field37362, source_ml_source2_nominal_rest_field37363) => {
                                                                    let source_ml_source2_nominal_rest_field37362 = *source_ml_source2_nominal_rest_field37362;
                                                                    self::JavaScriptCallPending::WaitSourceError(source_ml_source2_nominal_rest_field37360.clone(), source_ml_source2_nominal_rest_field37361.clone(), Box::new(source_ml_source2_nominal_rest_field37362.clone()), source_ml_source2_nominal_rest_field37363)
                                                                }
                                                                self::JavaScriptCallPending::WaitErrorValue(source_ml_source2_nominal_rest_field37364, source_ml_source2_nominal_rest_field37365, source_ml_source2_nominal_rest_field37366, source_ml_source2_nominal_rest_field37367, source_ml_source2_nominal_rest_field37368, source_ml_source2_nominal_rest_field37369, source_ml_source2_nominal_rest_field37370, source_ml_source2_nominal_rest_field37371, source_ml_source2_nominal_rest_field37372, source_ml_source2_nominal_rest_field37373) => {
                                                                    let source_ml_source2_nominal_rest_field37367 = *source_ml_source2_nominal_rest_field37367;
                                                                    let source_ml_source2_nominal_rest_field37368 = *source_ml_source2_nominal_rest_field37368;
                                                                    let source_ml_source2_nominal_rest_field37369 = *source_ml_source2_nominal_rest_field37369;
                                                                    let source_ml_source2_nominal_rest_field37370 = *source_ml_source2_nominal_rest_field37370;
                                                                    let source_ml_source2_nominal_rest_field37373 = *source_ml_source2_nominal_rest_field37373;
                                                                    self::JavaScriptCallPending::WaitErrorValue(source_ml_source2_nominal_rest_field37364, source_ml_source2_nominal_rest_field37365.clone(), source_ml_source2_nominal_rest_field37366.clone(), Box::new(source_ml_source2_nominal_rest_field37367.clone()), Box::new(source_ml_source2_nominal_rest_field37368.clone()), Box::new(source_ml_source2_nominal_rest_field37369.clone()), Box::new(source_ml_source2_nominal_rest_field37370.clone()), source_ml_source2_nominal_rest_field37371, source_ml_source2_nominal_rest_field37372, Box::new(source_ml_source2_nominal_rest_field37373.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitErrorIntrinsic(source_ml_source2_nominal_rest_field37374) => {
                                                                    self::JavaScriptCallPending::WaitErrorIntrinsic(source_ml_source2_nominal_rest_field37374)
                                                                }
                                                                self::JavaScriptCallPending::WaitHostCapability(source_ml_source2_nominal_rest_field37375, source_ml_source2_nominal_rest_field37376) => {
                                                                    let source_ml_source2_nominal_rest_field37376 = *source_ml_source2_nominal_rest_field37376;
                                                                    self::JavaScriptCallPending::WaitHostCapability(source_ml_source2_nominal_rest_field37375, Box::new(source_ml_source2_nominal_rest_field37376.clone()))
                                                                }
                                                                self::JavaScriptCallPending::WaitError(source_ml_source2_nominal_rest_field37377, source_ml_source2_nominal_rest_field37378, source_ml_source2_nominal_rest_field37379) => {
                                                                    self::JavaScriptCallPending::WaitError(source_ml_source2_nominal_rest_field37377.clone(), source_ml_source2_nominal_rest_field37378.clone(), source_ml_source2_nominal_rest_field37379.clone())
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
                                        self::JavaScriptCallPending::WaitSourceError(
                                            pending_realm_3,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            if (pending_realm_3 != realm) {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 97, 98, 114, 117, 112, 116, 32, 97, 108, 108, 111, 99, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 114, 101, 97, 108, 109]))
                                            } else {
                                                {
                                                    let invalid =
                                                        self::read_java_script_ordinary_value_error(
                                                            shared_constructor_field1.clone(),
                                                            value.clone(),
                                                        );
                                                    if (invalid
                                                        != (self::ml_string::Text::from_units(
                                                            vec![],
                                                        )))
                                                    {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), invalid.clone())
                                                    } else {
                                                        self::throw_java_script_call_value(
                                                            machine.clone(),
                                                            value.clone(),
                                                        )
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallPending::WaitErrorValue(
                                            _,
                                            pending_realm_4,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            if (pending_realm_4 != realm) {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 97, 98, 114, 117, 112, 116, 32, 97, 108, 108, 111, 99, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 114, 101, 97, 108, 109]))
                                            } else {
                                                {
                                                    let invalid_2 =
                                                        self::read_java_script_ordinary_value_error(
                                                            shared_constructor_field1.clone(),
                                                            value.clone(),
                                                        );
                                                    if (invalid_2
                                                        != (self::ml_string::Text::from_units(
                                                            vec![],
                                                        )))
                                                    {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), invalid_2.clone())
                                                    } else {
                                                        self::throw_java_script_call_value(
                                                            machine.clone(),
                                                            value.clone(),
                                                        )
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallPending::WaitError(
                                            pending_realm_5,
                                            _,
                                            _,
                                        ) => {
                                            if (pending_realm_5 != realm) {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 97, 98, 114, 117, 112, 116, 32, 97, 108, 108, 111, 99, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 114, 101, 97, 108, 109]))
                                            } else {
                                                {
                                                    let invalid_3 =
                                                        self::read_java_script_ordinary_value_error(
                                                            shared_constructor_field1.clone(),
                                                            value.clone(),
                                                        );
                                                    if (invalid_3
                                                        != (self::ml_string::Text::from_units(
                                                            vec![],
                                                        )))
                                                    {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), invalid_3.clone())
                                                    } else {
                                                        self::throw_java_script_call_value(
                                                            machine.clone(),
                                                            value.clone(),
                                                        )
                                                    }
                                                }
                                            }
                                        }
                                        _ => self::JavaScriptCallMachineTransition::MachineInvalid(
                                            Box::new(shared_constructor_field1.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                97, 110, 32, 97, 98, 114, 117, 112, 116, 32, 97,
                                                108, 108, 111, 99, 97, 116, 111, 114, 32, 114, 101,
                                                115, 117, 108, 116, 32, 110, 101, 101, 100, 115,
                                                32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                                32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111,
                                                99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117,
                                                101, 115, 116,
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
pub fn resume_java_script_error_intrinsic(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    realm: self::ml::Big,
    identity: self::ml::Big,
    value: self::JavaScriptSemanticValue,
    abrupt: bool,
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
                            let source_ml_source2_array_read_value37409 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index37410 = ({
                                    let ml_reference_array2444 = shared_constructor_field2.clone();
                                    let ml_reference_projection2444 =
                                        ml_reference_array2444.borrow().len() as f64;
                                    ml_reference_projection2444
                                } - 1f64);
                                {
                                    let ml_reference_array2445 =
                                        source_ml_source2_array_read_value37409.clone();
                                    let ml_reference_position2445 =
                                        source_ml_source2_array_read_index37410;
                                    let ml_reference_projection2445 = self::ml_array::optional_at(
                                        &*ml_reference_array2445.borrow(),
                                        self::ml_array::number_index(ml_reference_position2445),
                                    );
                                    ml_reference_projection2445
                                }
                            }
                        };
                        if ((match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } || {
                            let source_ml_source2_s37412 =
                                match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value37414,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value37414 =
                                            *source_ml_source2_data_property_value37414;
                                        source_ml_source2_data_property_value37414.clone()
                                    }
                                };
                            match source_ml_source2_s37412.clone() {
                                self::JavaScriptCallPending::WaitErrorIntrinsic(_) => false,
                                _ => true,
                            }
                        }) || ((match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(
                                source_ml_source2_data_property_value37416,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value37416.clone(),
                        }) != identity))
                        {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 69, 114, 114, 111, 114, 32, 105, 110, 116, 114,
                                    105, 110, 115, 105, 99, 32, 114, 101, 115, 117, 108, 116, 32,
                                    114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                                    99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103,
                                    32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 105, 100, 101, 110,
                                    116, 105, 116, 121,
                                ]),
                            )
                        } else {
                            {
                                let fn_ = self::read_java_script_executable_call_function(
                                    program.clone(),
                                    shared_constructor_field1.clone(),
                                    identity.clone(),
                                );
                                if ((match (fn_.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } || ((match fn_.clone().expect("a proven present value") {
                                    self::JavaScriptCallFunction::CallFunction(
                                        _,
                                        source_ml_source2_data_property_value37420,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value37420.clone(),
                                }) != realm))
                                    || !(self::test_java_script_error_intrinsic_mode(
                                        &(match fn_.clone().expect("a proven present value") {
                                            self::JavaScriptCallFunction::CallFunction(
                                                _,
                                                _,
                                                source_ml_source2_data_property_value37421,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source2_data_property_value37421.clone(),
                                        }),
                                    )))
                                {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field1.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            97, 110, 32, 69, 114, 114, 111, 114, 32, 105, 110, 116,
                                            114, 105, 110, 115, 105, 99, 32, 114, 101, 115, 117,
                                            108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                                            105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101,
                                            120, 116, 101, 114, 110, 97, 108, 32, 114, 101, 97,
                                            108, 109, 32, 97, 117, 116, 104, 111, 114, 105, 116,
                                            121,
                                        ]),
                                    )
                                } else {
                                    {
                                        let invalid = self::read_java_script_ordinary_value_error(
                                            shared_constructor_field1.clone(),
                                            value.clone(),
                                        );
                                        if (invalid != (self::ml_string::Text::from_units(vec![])))
                                        {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                                Box::new(shared_constructor_field1.clone()),
                                                invalid.clone(),
                                            )
                                        } else {
                                            if abrupt {
                                                self::throw_java_script_call_value(
                                                    machine.clone(),
                                                    value.clone(),
                                                )
                                            } else {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), ((match match frame.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value37427, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value37427 = *source_ml_source2_data_property_value37427;
                                                        source_ml_source2_data_property_value37427.clone()
                                                    }
                                                } {
                                                    self::JavaScriptCallPending::WaitErrorIntrinsic(source_ml_source2_data_property_value37428) => {
                                                        Some((source_ml_source2_data_property_value37428,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0, value.clone()))))
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
