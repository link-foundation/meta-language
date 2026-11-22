// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptConstructionState;
use super::JavaScriptSemanticValue;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_construction_arguments_error(
    frame: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
    instruction: self::JavaScriptCallInstruction,
    state: self::JavaScriptConstructionState,
) -> self::ml_string::Text {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            _,
            shared_constructor_field1,
            _,
            shared_constructor_field2,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            match instruction.clone() {
                self::JavaScriptCallInstruction::CollectionAllocate(
                    destination,
                    prototype,
                    kind,
                ) => {
                    let actual = {
                        let source_ml_source2_array_read_value32351 = match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value32349,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value32349.clone(),
                        };
                        {
                            let source_ml_source2_array_read_index32352 = shared_constructor_field1;
                            {
                                let ml_reference_array1999 =
                                    source_ml_source2_array_read_value32351.clone();
                                let ml_reference_position1999 =
                                    source_ml_source2_array_read_index32352;
                                let ml_reference_projection1999 = self::ml_array::optional_at(
                                    &*ml_reference_array1999.borrow(),
                                    self::ml_array::number_index(ml_reference_position1999),
                                );
                                ml_reference_projection1999
                            }
                        }
                    };
                    if ((((({
                        let source_ml_source2_comparison_operand32354 = {
                            let source_ml_source2_optional_chain_receiver32353 = (actual.clone(),);
                            if match (source_ml_source2_optional_chain_receiver32353.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                Some({
                                    let source_ml_source1_temporary_optional_chain1340 =
                                        ((source_ml_source2_optional_chain_receiver32353.clone())
                                            .0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source1_temporary_optional_chain1340.clone() {
                                            self::JavaScriptCallInstruction::Constant(_, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 97, 110, 116])
                                            }
                                            self::JavaScriptCallInstruction::Copy(_, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 112, 121])
                                            }
                                            self::JavaScriptCallInstruction::Argument(_, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116])
                                            }
                                            self::JavaScriptCallInstruction::ArgumentCount(_) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 67, 111, 117, 110, 116])
                                            }
                                            self::JavaScriptCallInstruction::ArgumentAt(_, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 65, 116])
                                            }
                                            self::JavaScriptCallInstruction::FunctionBind(_) => {
                                                self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 66, 105, 110, 100])
                                            }
                                            self::JavaScriptCallInstruction::BoundFunctionCall(_) => {
                                                self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 67, 97, 108, 108])
                                            }
                                            self::JavaScriptCallInstruction::FunctionToString(_) => {
                                                self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 84, 111, 83, 116, 114, 105, 110, 103])
                                            }
                                            self::JavaScriptCallInstruction::PushFinally(_) => {
                                                self::ml_string::Text::from_units(vec![112, 117, 115, 104, 70, 105, 110, 97, 108, 108, 121])
                                            }
                                            self::JavaScriptCallInstruction::FinishFinally => {
                                                self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104, 70, 105, 110, 97, 108, 108, 121])
                                            }
                                            self::JavaScriptCallInstruction::CompleteJump(_, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101, 74, 117, 109, 112])
                                            }
                                            self::JavaScriptCallInstruction::Jump(_) => {
                                                self::ml_string::Text::from_units(vec![106, 117, 109, 112])
                                            }
                                            self::JavaScriptCallInstruction::Branch(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![98, 114, 97, 110, 99, 104])
                                            }
                                            self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![99, 97, 108, 108])
                                            }
                                            self::JavaScriptCallInstruction::ArgumentListOpen(_) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 76, 105, 115, 116, 79, 112, 101, 110])
                                            }
                                            self::JavaScriptCallInstruction::ArgumentListAppend(_, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 76, 105, 115, 116, 65, 112, 112, 101, 110, 100])
                                            }
                                            self::JavaScriptCallInstruction::ArgumentListCall(_, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 76, 105, 115, 116, 67, 97, 108, 108])
                                            }
                                            self::JavaScriptCallInstruction::Construct(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])
                                            }
                                            self::JavaScriptCallInstruction::ConstructorCheck(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 67, 104, 101, 99, 107])
                                            }
                                            self::JavaScriptCallInstruction::ReadNewTarget(_) => {
                                                self::ml_string::Text::from_units(vec![114, 101, 97, 100, 78, 101, 119, 84, 97, 114, 103, 101, 116])
                                            }
                                            self::JavaScriptCallInstruction::ArrayJoinEnter(_, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 74, 111, 105, 110, 69, 110, 116, 101, 114])
                                            }
                                            self::JavaScriptCallInstruction::ArrayJoinLeave(_) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 74, 111, 105, 110, 76, 101, 97, 118, 101])
                                            }
                                            self::JavaScriptCallInstruction::ConstructErrorClass(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 69, 114, 114, 111, 114, 67, 108, 97, 115, 115])
                                            }
                                            self::JavaScriptCallInstruction::CallClassConstructor(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![99, 97, 108, 108, 67, 108, 97, 115, 115, 67, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])
                                            }
                                            self::JavaScriptCallInstruction::ConstructCollection(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110])
                                            }
                                            self::JavaScriptCallInstruction::CollectionAllocate(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 65, 108, 108, 111, 99, 97, 116, 101])
                                            }
                                            self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 65, 108, 108, 111, 99, 97, 116, 101])
                                            }
                                            self::JavaScriptCallInstruction::ArgumentRest(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 82, 101, 115, 116])
                                            }
                                            self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 80, 114, 111, 116, 111, 99, 111, 108])
                                            }
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 82, 101, 115, 117, 108, 116])
                                            }
                                            self::JavaScriptCallInstruction::CollectionOperation(_, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 79, 112, 101, 114, 97, 116, 105, 111, 110])
                                            }
                                            self::JavaScriptCallInstruction::PrimitiveMethod(_, _) => {
                                                self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 77, 101, 116, 104, 111, 100])
                                            }
                                            self::JavaScriptCallInstruction::CollectionForEach(_, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 70, 111, 114, 69, 97, 99, 104])
                                            }
                                            self::JavaScriptCallInstruction::ArrayConcatenation(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 67, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110])
                                            }
                                            self::JavaScriptCallInstruction::ArraySlice(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 83, 108, 105, 99, 101])
                                            }
                                            self::JavaScriptCallInstruction::ArrayCreation(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 67, 114, 101, 97, 116, 105, 111, 110])
                                            }
                                            self::JavaScriptCallInstruction::ArrayCallback(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 67, 97, 108, 108, 98, 97, 99, 107])
                                            }
                                            self::JavaScriptCallInstruction::ArraySearch(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 83, 101, 97, 114, 99, 104])
                                            }
                                            self::JavaScriptCallInstruction::ArrayTransform(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 84, 114, 97, 110, 115, 102, 111, 114, 109])
                                            }
                                            self::JavaScriptCallInstruction::ObjectEnumeration(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 69, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110])
                                            }
                                            self::JavaScriptCallInstruction::PropertyDefinitions(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 68, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115])
                                            }
                                            self::JavaScriptCallInstruction::PropertyReflection(_, _, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 82, 101, 102, 108, 101, 99, 116, 105, 111, 110])
                                            }
                                            self::JavaScriptCallInstruction::InstanceOf(_, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 79, 102])
                                            }
                                            self::JavaScriptCallInstruction::ObjectObservation(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 79, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110])
                                            }
                                            self::JavaScriptCallInstruction::CollectionCopy(_, _, _, _, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 67, 111, 112, 121])
                                            }
                                            self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 69, 114, 114, 111, 114, 86, 97, 108, 117, 101])
                                            }
                                            self::JavaScriptCallInstruction::ReadDerivedReceiver(_) => {
                                                self::ml_string::Text::from_units(vec![114, 101, 97, 100, 68, 101, 114, 105, 118, 101, 100, 82, 101, 99, 101, 105, 118, 101, 114])
                                            }
                                            self::JavaScriptCallInstruction::InvokeErrorIntrinsic(_) => {
                                                self::ml_string::Text::from_units(vec![105, 110, 118, 111, 107, 101, 69, 114, 114, 111, 114, 73, 110, 116, 114, 105, 110, 115, 105, 99])
                                            }
                                            self::JavaScriptCallInstruction::InvokeHostCapability(_) => {
                                                self::ml_string::Text::from_units(vec![105, 110, 118, 111, 107, 101, 72, 111, 115, 116, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121])
                                            }
                                            self::JavaScriptCallInstruction::Return(_) => {
                                                self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110])
                                            }
                                            self::JavaScriptCallInstruction::Throw(_) => {
                                                self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119])
                                            }
                                            self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                                self::ml_string::Text::from_units(vec![112, 117, 115, 104, 72, 97, 110, 100, 108, 101, 114])
                                            }
                                            self::JavaScriptCallInstruction::PopHandler => {
                                                self::ml_string::Text::from_units(vec![112, 111, 112, 72, 97, 110, 100, 108, 101, 114])
                                            }
                                            self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 66, 105, 110, 97, 114, 121])
                                            }
                                            self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![115, 97, 109, 101, 86, 97, 108, 117, 101])
                                            }
                                            self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108])
                                            }
                                            self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 87, 114, 105, 116, 101])
                                            }
                                            self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101])
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryAllocate(_, _) => {
                                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 65, 108, 108, 111, 99, 97, 116, 101])
                                            }
                                            self::JavaScriptCallInstruction::LiteralPrototype(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108, 80, 114, 111, 116, 111, 116, 121, 112, 101])
                                            }
                                            self::JavaScriptCallInstruction::LiteralAccessor(_, _, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108, 65, 99, 99, 101, 115, 115, 111, 114])
                                            }
                                            self::JavaScriptCallInstruction::ObjectFromEntries(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 70, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115])
                                            }
                                            self::JavaScriptCallInstruction::PrimitiveWrapper(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 87, 114, 97, 112, 112, 101, 114])
                                            }
                                            self::JavaScriptCallInstruction::SymbolAllocate(_, _) => {
                                                self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108, 65, 108, 108, 111, 99, 97, 116, 101])
                                            }
                                            self::JavaScriptCallInstruction::SymbolDescription(_, _) => {
                                                self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108, 68, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110])
                                            }
                                            self::JavaScriptCallInstruction::SymbolRegistry(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114, 121])
                                            }
                                            self::JavaScriptCallInstruction::FunctionName(_, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 78, 97, 109, 101])
                                            }
                                            self::JavaScriptCallInstruction::ArrayAllocate(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 65, 108, 108, 111, 99, 97, 116, 101])
                                            }
                                            self::JavaScriptCallInstruction::ClosureAllocate(_, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 65, 108, 108, 111, 99, 97, 116, 101])
                                            }
                                            self::JavaScriptCallInstruction::ArraySpread(_, _, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 83, 112, 114, 101, 97, 100])
                                            }
                                            self::JavaScriptCallInstruction::ObjectSpread(_, _, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 83, 112, 114, 101, 97, 100])
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryDataProperty(_, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121])
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 71, 101, 116])
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryDelete(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 68, 101, 108, 101, 116, 101])
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryHas(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 72, 97, 115])
                                            }
                                            self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 83, 101, 116])
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryDefine(_, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 68, 101, 102, 105, 110, 101])
                                            }
                                            self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                                self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 75, 101, 121])
                                            }
                                            self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _, _) => {
                                                self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 68, 101, 102, 105, 110, 105, 116, 105, 111, 110])
                                            }
                                        }
                                })
                            } else {
                                None::<self::ml_string::Text>
                            }
                        };
                        !(match (
                            source_ml_source2_comparison_operand32354.clone(),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 65, 108, 108, 111,
                                99, 97, 116, 101,
                            ]),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    } || {
                        let source_ml_source2_comparison_operand32362 = {
                            let source_ml_source2_nullish_field_receiver32356 = (actual.clone(),);
                            if match (source_ml_source2_nullish_field_receiver32356.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present32357 =
                                        ((source_ml_source2_nullish_field_receiver32356.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source2_nullish_field_present32357.clone() {
                                            self::JavaScriptCallInstruction::CollectionAllocate(_, _, source_ml_source2_data_property_value32358) => {
                                                Some(source_ml_source2_data_property_value32358.clone())
                                            }
                                            self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, _, source_ml_source2_data_property_value32359, _, _) => {
                                                Some(source_ml_source2_data_property_value32359.clone())
                                            }
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(_, source_ml_source2_data_property_value32360, _, _) => {
                                                Some(source_ml_source2_data_property_value32360.clone())
                                            }
                                            self::JavaScriptCallInstruction::LiteralAccessor(_, _, _, _, source_ml_source2_data_property_value32361, _) => {
                                                Some(source_ml_source2_data_property_value32361.clone())
                                            }
                                            _ => {
                                                None::<self::ml_string::Text>
                                            }
                                        }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 105, 110,
                                        100, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        };
                        !(match (
                            source_ml_source2_comparison_operand32362.clone(),
                            kind.clone(),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }) || {
                        let source_ml_source2_comparison_operand32433 = {
                            let source_ml_source2_nullish_field_receiver32364 = (actual.clone(),);
                            if match (source_ml_source2_nullish_field_receiver32364.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present32365 =
                                        ((source_ml_source2_nullish_field_receiver32364.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source2_nullish_field_present32365.clone() {
                                            self::JavaScriptCallInstruction::Constant(source_ml_source2_data_property_value32366, _) => {
                                                Some(source_ml_source2_data_property_value32366)
                                            }
                                            self::JavaScriptCallInstruction::Copy(source_ml_source2_data_property_value32367, _) => {
                                                Some(source_ml_source2_data_property_value32367)
                                            }
                                            self::JavaScriptCallInstruction::Argument(source_ml_source2_data_property_value32368, _) => {
                                                Some(source_ml_source2_data_property_value32368)
                                            }
                                            self::JavaScriptCallInstruction::ArgumentCount(source_ml_source2_data_property_value32369) => {
                                                Some(source_ml_source2_data_property_value32369)
                                            }
                                            self::JavaScriptCallInstruction::ArgumentAt(source_ml_source2_data_property_value32370, _) => {
                                                Some(source_ml_source2_data_property_value32370)
                                            }
                                            self::JavaScriptCallInstruction::FunctionBind(source_ml_source2_data_property_value32371) => {
                                                Some(source_ml_source2_data_property_value32371)
                                            }
                                            self::JavaScriptCallInstruction::BoundFunctionCall(source_ml_source2_data_property_value32372) => {
                                                Some(source_ml_source2_data_property_value32372)
                                            }
                                            self::JavaScriptCallInstruction::FunctionToString(source_ml_source2_data_property_value32373) => {
                                                Some(source_ml_source2_data_property_value32373)
                                            }
                                            self::JavaScriptCallInstruction::Call(source_ml_source2_data_property_value32374, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32374)
                                            }
                                            self::JavaScriptCallInstruction::ArgumentListCall(source_ml_source2_data_property_value32375, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32375)
                                            }
                                            self::JavaScriptCallInstruction::Construct(source_ml_source2_data_property_value32376, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32376)
                                            }
                                            self::JavaScriptCallInstruction::ConstructorCheck(source_ml_source2_data_property_value32377, _, _) => {
                                                Some(source_ml_source2_data_property_value32377)
                                            }
                                            self::JavaScriptCallInstruction::ReadNewTarget(source_ml_source2_data_property_value32378) => {
                                                Some(source_ml_source2_data_property_value32378)
                                            }
                                            self::JavaScriptCallInstruction::ArrayJoinEnter(source_ml_source2_data_property_value32379, _) => {
                                                Some(source_ml_source2_data_property_value32379)
                                            }
                                            self::JavaScriptCallInstruction::ConstructErrorClass(source_ml_source2_data_property_value32380, _, _) => {
                                                Some(source_ml_source2_data_property_value32380)
                                            }
                                            self::JavaScriptCallInstruction::CallClassConstructor(source_ml_source2_data_property_value32381, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32381)
                                            }
                                            self::JavaScriptCallInstruction::ConstructCollection(source_ml_source2_data_property_value32382, _, _) => {
                                                Some(source_ml_source2_data_property_value32382)
                                            }
                                            self::JavaScriptCallInstruction::CollectionAllocate(source_ml_source2_data_property_value32383, _, _) => {
                                                Some(source_ml_source2_data_property_value32383)
                                            }
                                            self::JavaScriptCallInstruction::CollectionIteratorAllocate(source_ml_source2_data_property_value32384, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32384)
                                            }
                                            self::JavaScriptCallInstruction::ArgumentRest(source_ml_source2_data_property_value32385, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32385)
                                            }
                                            self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_data_property_value32386, _, _, _, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32386)
                                            }
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(source_ml_source2_data_property_value32387, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32387)
                                            }
                                            self::JavaScriptCallInstruction::CollectionOperation(source_ml_source2_data_property_value32388, _) => {
                                                Some(source_ml_source2_data_property_value32388)
                                            }
                                            self::JavaScriptCallInstruction::PrimitiveMethod(source_ml_source2_data_property_value32389, _) => {
                                                Some(source_ml_source2_data_property_value32389)
                                            }
                                            self::JavaScriptCallInstruction::CollectionForEach(source_ml_source2_data_property_value32390, _) => {
                                                Some(source_ml_source2_data_property_value32390)
                                            }
                                            self::JavaScriptCallInstruction::ArrayConcatenation(source_ml_source2_data_property_value32391, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32391)
                                            }
                                            self::JavaScriptCallInstruction::ArraySlice(source_ml_source2_data_property_value32392, _, _) => {
                                                Some(source_ml_source2_data_property_value32392)
                                            }
                                            self::JavaScriptCallInstruction::ArrayCreation(source_ml_source2_data_property_value32393, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32393)
                                            }
                                            self::JavaScriptCallInstruction::ArrayCallback(source_ml_source2_data_property_value32394, _, _) => {
                                                Some(source_ml_source2_data_property_value32394)
                                            }
                                            self::JavaScriptCallInstruction::ArraySearch(source_ml_source2_data_property_value32395, _, _) => {
                                                Some(source_ml_source2_data_property_value32395)
                                            }
                                            self::JavaScriptCallInstruction::ArrayTransform(source_ml_source2_data_property_value32396, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32396)
                                            }
                                            self::JavaScriptCallInstruction::ObjectEnumeration(source_ml_source2_data_property_value32397, _, _) => {
                                                Some(source_ml_source2_data_property_value32397)
                                            }
                                            self::JavaScriptCallInstruction::PropertyDefinitions(source_ml_source2_data_property_value32398, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32398)
                                            }
                                            self::JavaScriptCallInstruction::PropertyReflection(source_ml_source2_data_property_value32399, _, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32399)
                                            }
                                            self::JavaScriptCallInstruction::InstanceOf(source_ml_source2_data_property_value32400, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32400)
                                            }
                                            self::JavaScriptCallInstruction::ObjectObservation(source_ml_source2_data_property_value32401, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32401)
                                            }
                                            self::JavaScriptCallInstruction::CollectionCopy(source_ml_source2_data_property_value32402, _, _, _, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32402)
                                            }
                                            self::JavaScriptCallInstruction::ConstructErrorValue(source_ml_source2_data_property_value32403, _, _, _, _, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32403)
                                            }
                                            self::JavaScriptCallInstruction::ReadDerivedReceiver(source_ml_source2_data_property_value32404) => {
                                                Some(source_ml_source2_data_property_value32404)
                                            }
                                            self::JavaScriptCallInstruction::InvokeErrorIntrinsic(source_ml_source2_data_property_value32405) => {
                                                Some(source_ml_source2_data_property_value32405)
                                            }
                                            self::JavaScriptCallInstruction::InvokeHostCapability(source_ml_source2_data_property_value32406) => {
                                                Some(source_ml_source2_data_property_value32406)
                                            }
                                            self::JavaScriptCallInstruction::PushHandler(source_ml_source2_data_property_value32407, _) => {
                                                Some(source_ml_source2_data_property_value32407)
                                            }
                                            self::JavaScriptCallInstruction::NumberBinary(source_ml_source2_data_property_value32408, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32408)
                                            }
                                            self::JavaScriptCallInstruction::SameValue(source_ml_source2_data_property_value32409, _, _) => {
                                                Some(source_ml_source2_data_property_value32409)
                                            }
                                            self::JavaScriptCallInstruction::StrictEqual(source_ml_source2_data_property_value32410, _, _) => {
                                                Some(source_ml_source2_data_property_value32410)
                                            }
                                            self::JavaScriptCallInstruction::Primitive(source_ml_source2_data_property_value32411, _, _) => {
                                                Some(source_ml_source2_data_property_value32411)
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryAllocate(source_ml_source2_data_property_value32412, _) => {
                                                Some(source_ml_source2_data_property_value32412)
                                            }
                                            self::JavaScriptCallInstruction::LiteralPrototype(source_ml_source2_data_property_value32413, _, _) => {
                                                Some(source_ml_source2_data_property_value32413)
                                            }
                                            self::JavaScriptCallInstruction::LiteralAccessor(source_ml_source2_data_property_value32414, _, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32414)
                                            }
                                            self::JavaScriptCallInstruction::ObjectFromEntries(source_ml_source2_data_property_value32415, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32415)
                                            }
                                            self::JavaScriptCallInstruction::PrimitiveWrapper(source_ml_source2_data_property_value32416, _, _) => {
                                                Some(source_ml_source2_data_property_value32416)
                                            }
                                            self::JavaScriptCallInstruction::SymbolAllocate(source_ml_source2_data_property_value32417, _) => {
                                                Some(source_ml_source2_data_property_value32417)
                                            }
                                            self::JavaScriptCallInstruction::SymbolDescription(source_ml_source2_data_property_value32418, _) => {
                                                Some(source_ml_source2_data_property_value32418)
                                            }
                                            self::JavaScriptCallInstruction::SymbolRegistry(source_ml_source2_data_property_value32419, _, _) => {
                                                Some(source_ml_source2_data_property_value32419)
                                            }
                                            self::JavaScriptCallInstruction::FunctionName(source_ml_source2_data_property_value32420, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32420)
                                            }
                                            self::JavaScriptCallInstruction::ArrayAllocate(source_ml_source2_data_property_value32421, _, _) => {
                                                Some(source_ml_source2_data_property_value32421)
                                            }
                                            self::JavaScriptCallInstruction::ClosureAllocate(source_ml_source2_data_property_value32422, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32422)
                                            }
                                            self::JavaScriptCallInstruction::ArraySpread(source_ml_source2_data_property_value32423, _, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32423)
                                            }
                                            self::JavaScriptCallInstruction::ObjectSpread(source_ml_source2_data_property_value32424, _, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32424)
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryDataProperty(source_ml_source2_data_property_value32425, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32425)
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryGet(source_ml_source2_data_property_value32426, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32426)
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryDelete(source_ml_source2_data_property_value32427, _, _) => {
                                                Some(source_ml_source2_data_property_value32427)
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryHas(source_ml_source2_data_property_value32428, _, _) => {
                                                Some(source_ml_source2_data_property_value32428)
                                            }
                                            self::JavaScriptCallInstruction::OrdinarySet(source_ml_source2_data_property_value32429, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32429)
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryDefine(source_ml_source2_data_property_value32430, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32430)
                                            }
                                            self::JavaScriptCallInstruction::PropertyKey(source_ml_source2_data_property_value32431, _, _) => {
                                                Some(source_ml_source2_data_property_value32431)
                                            }
                                            self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source2_data_property_value32432, _, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32432)
                                            }
                                            _ => {
                                                None::<f64>
                                            }
                                        }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115,
                                        116, 105, 110, 97, 116, 105, 111, 110, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        };
                        !(match (
                            source_ml_source2_comparison_operand32433.clone(),
                            destination,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }) || {
                        let source_ml_source2_comparison_operand32447 = {
                            let source_ml_source2_nullish_field_receiver32435 = (actual.clone(),);
                            if match (source_ml_source2_nullish_field_receiver32435.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present32436 =
                                        ((source_ml_source2_nullish_field_receiver32435.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source2_nullish_field_present32436.clone() {
                                            self::JavaScriptCallInstruction::CollectionAllocate(_, source_ml_source2_data_property_value32437, _) => {
                                                Some(source_ml_source2_data_property_value32437)
                                            }
                                            self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, source_ml_source2_data_property_value32438, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32438)
                                            }
                                            self::JavaScriptCallInstruction::ArgumentRest(_, source_ml_source2_data_property_value32439, _, _) => {
                                                Some(source_ml_source2_data_property_value32439)
                                            }
                                            self::JavaScriptCallInstruction::ArrayCreation(_, _, source_ml_source2_data_property_value32440, _) => {
                                                Some(source_ml_source2_data_property_value32440)
                                            }
                                            self::JavaScriptCallInstruction::ObjectEnumeration(_, _, source_ml_source2_data_property_value32441) => {
                                                Some(source_ml_source2_data_property_value32441)
                                            }
                                            self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, source_ml_source2_data_property_value32442, _, _, _, _, _) => {
                                                Some(source_ml_source2_data_property_value32442)
                                            }
                                            self::JavaScriptCallInstruction::OrdinaryAllocate(_, source_ml_source2_data_property_value32443) => {
                                                Some(source_ml_source2_data_property_value32443)
                                            }
                                            self::JavaScriptCallInstruction::ObjectFromEntries(_, _, source_ml_source2_data_property_value32444, _) => {
                                                Some(source_ml_source2_data_property_value32444)
                                            }
                                            self::JavaScriptCallInstruction::PrimitiveWrapper(_, _, source_ml_source2_data_property_value32445) => {
                                                Some(source_ml_source2_data_property_value32445)
                                            }
                                            self::JavaScriptCallInstruction::ArrayAllocate(_, source_ml_source2_data_property_value32446, _) => {
                                                Some(source_ml_source2_data_property_value32446)
                                            }
                                            _ => {
                                                None::<f64>
                                            }
                                        }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111,
                                        116, 111, 116, 121, 112, 101, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        };
                        !(match (source_ml_source2_comparison_operand32447.clone(), prototype) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }) || (({
                        let ml_reference_array2000 = shared_constructor_field2.clone();
                        let ml_reference_projection2000 =
                            ml_reference_array2000.borrow().len() as f64;
                        ml_reference_projection2000
                    }) != ({
                        let ml_reference_array2001 = match state.clone() {
                            self::JavaScriptConstructionState::ConstructionState(
                                _,
                                _,
                                source_ml_source2_data_property_value32452,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value32452.clone(),
                        };
                        let ml_reference_projection2001 =
                            ml_reference_array2001.borrow().len() as f64;
                        ml_reference_projection2001
                    }))) || {
                        let ml_values2002 = {
                            let ml_reference_value2003 = shared_constructor_field2.borrow().clone();
                            ml_reference_value2003
                        };
                        let ml_callback2002 = {
                            let state = state.clone();
                            std::rc::Rc::new(
                                move |value: self::JavaScriptSemanticValue, index: f64| -> bool {
                                    !(self::test_java_script_semantic_same_value(value.clone(), ({
                            let source_ml_source2_array_read_value32455 = match state.clone() {
                                self::JavaScriptConstructionState::ConstructionState(_, _, source_ml_source2_data_property_value32454, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value32454.clone()
                                }
                            };
                            {
                                let source_ml_source2_array_read_index32456 = index;
                                { let ml_reference_array2004 = source_ml_source2_array_read_value32455.clone(); let ml_reference_position2004 = source_ml_source2_array_read_index32456; let ml_reference_projection2004 = self::ml_array::optional_at(&*ml_reference_array2004.borrow(), self::ml_array::number_index(ml_reference_position2004)); ml_reference_projection2004 }
                            }
                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                },
                            )
                        };
                        ml_values2002.iter().cloned().enumerate().any(
                            |(ml_index2002, ml_value2002)| {
                                ml_callback2002(ml_value2002, (ml_index2002 as f64))
                            },
                        )
                    }) {
                        self::ml_string::Text::from_units(vec![
                            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 105, 116,
                            105, 97, 108, 105, 122, 101, 114, 32, 114, 101, 117, 115, 101, 32, 114,
                            101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                            97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 97, 108, 108,
                            111, 99, 97, 116, 105, 111, 110, 32, 105, 110, 115, 116, 114, 117, 99,
                            116, 105, 111, 110, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101,
                            116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 97, 114, 103, 117,
                            109, 101, 110, 116, 115,
                        ])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                _ => self::ml_string::Text::from_units(vec![
                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 105, 116, 105,
                    97, 108, 105, 122, 101, 114, 32, 114, 101, 117, 115, 101, 32, 114, 101, 116,
                    97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                    101, 108, 101, 99, 116, 101, 100, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111,
                    110, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110,
                    100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114,
                    101, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115,
                ]),
            }
        }
    }
}
