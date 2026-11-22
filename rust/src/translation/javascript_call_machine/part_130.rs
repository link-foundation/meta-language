// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationCall;
use super::JavaScriptArrayCreationState;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptObjectCopyCall;
use super::JavaScriptObjectEnumerationState;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_constructor_frame_operands_error;
use super::read_java_script_executable_call_function;
use super::read_java_script_selected_call_frame_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_selected_parent_special_call_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    parent: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
    frame: self::JavaScriptCallFrame,
) -> self::ml_string::Text {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            shared_constructor_field11,
            _,
            shared_constructor_field12,
            shared_constructor_field13,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            match fn_.clone() {
                self::JavaScriptCallFunction::CallFunction(
                    _,
                    _,
                    shared_constructor_field9,
                    shared_constructor_field10,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let shared_constructor_field10 = *shared_constructor_field10;
                    match parent.clone() {
                        self::JavaScriptCallFrame::CallFrame(
                            shared_constructor_field5,
                            shared_constructor_field6,
                            _,
                            shared_constructor_field7,
                            _,
                            shared_constructor_field8,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let shared_constructor_field8 = *shared_constructor_field8;
                            match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    shared_constructor_field4,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let pending = shared_constructor_field8.clone();
                                    match pending.clone() {
                                        self::JavaScriptCallPending::WaitCall(_) => {
                                            let owner =
                                                self::read_java_script_executable_call_function(
                                                    program.clone(),
                                                    heap.clone(),
                                                    shared_constructor_field5.clone(),
                                                );
                                            {
                                                let instruction = {
                                                    let source_ml_source2_optional_chain_receiver25301 =
                                                        (owner.clone(),);
                                                    match if match (source_ml_source2_optional_chain_receiver25301.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                Some({
                                                                    let source_ml_source1_temporary_optional_chain1119 = ((source_ml_source2_optional_chain_receiver25301.clone()).0).clone().expect("a proven present value");
                                                                    {
                                                                        let source_ml_source2_array_read_value25304 = match source_ml_source1_temporary_optional_chain1119.clone() {
                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value25302, _, _) => {
                                                                                source_ml_source2_data_property_value25302.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_array_read_index25305 = (shared_constructor_field6 - 1f64);
                                                                            { let ml_reference_array1333 = source_ml_source2_array_read_value25304.clone(); let ml_reference_position1333 = source_ml_source2_array_read_index25305; let ml_reference_projection1333 = self::ml_array::optional_at(&*ml_reference_array1333.borrow(), self::ml_array::number_index(ml_reference_position1333)); ml_reference_projection1333 }
                                                                        }
                                                                    }
                                                                })
                                                            } else {
                                                                None::<self::JavaScriptCallInstruction>
                                                            } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                };
                                                if {
                                                    let source_ml_source2_comparison_operand25307 = {
                                                        let source_ml_source2_optional_chain_receiver25306 =
                                                            (instruction.clone(),);
                                                        if match (source_ml_source2_optional_chain_receiver25306.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                    Some({
                                                                        let source_ml_source1_temporary_optional_chain1120 = ((source_ml_source2_optional_chain_receiver25306.clone()).0).clone().expect("a proven present value");
                                                                        match source_ml_source1_temporary_optional_chain1120.clone() {
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
                                                        source_ml_source2_comparison_operand25307
                                                            .clone(),
                                                        self::ml_string::Text::from_units(vec![
                                                            98, 111, 117, 110, 100, 70, 117, 110,
                                                            99, 116, 105, 111, 110, 67, 97, 108,
                                                            108,
                                                        ]),
                                                    ) {
                                                        (Some(ml_left), ml_right) => {
                                                            ml_left == ml_right
                                                        }
                                                        _ => false,
                                                    })
                                                } {
                                                    self::ml_string::Text::from_units(vec![])
                                                } else {
                                                    {
                                                        let bound = {
                                                            let source_ml_source2_array_read_value25312 =
                                                                shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_array_read_index25313 = self::find_java_script_ordinary_object(shared_constructor_field4.clone(), shared_constructor_field5.clone());
                                                                {
                                                                    let ml_reference_array1334 = source_ml_source2_array_read_value25312.clone();
                                                                    let ml_reference_position1334 = source_ml_source2_array_read_index25313;
                                                                    let ml_reference_projection1334 = self::ml_array::optional_at(&*ml_reference_array1334.borrow(), self::ml_array::number_index(ml_reference_position1334));
                                                                    ml_reference_projection1334
                                                                }
                                                            }
                                                        };
                                                        if ((match (bound.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match bound.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                        false
                                                                    }
                                                                    _ => {
                                                                        true
                                                                    }
                                                                }) || ((({ let ml_reference_array1335 = (((match bound.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value25324) => {
                                                                        Some((source_ml_source2_data_property_value25324.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone(); let ml_reference_projection1335 = ml_reference_array1335.borrow().len() as f64; ml_reference_projection1335 } + { let ml_reference_array1336 = shared_constructor_field7.clone(); let ml_reference_projection1336 = ml_reference_array1336.borrow().len() as f64; ml_reference_projection1336 })) > (1048576f64))) {
                                                                    self::ml_string::Text::from_units(vec![97, 32, 98, 111, 117, 110, 100, 32, 99, 97, 108, 108, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 116, 111, 114, 101, 100, 32, 97, 110, 100, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115])
                                                                } else {
                                                                    self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), (((match bound.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, source_ml_source2_data_property_value25326, _, _) => {
                                                                            let source_ml_source2_data_property_value25326 = *source_ml_source2_data_property_value25326;
                                                                            Some((source_ml_source2_data_property_value25326.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(self::JavaScriptSemanticValue,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone(), (((match bound.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, source_ml_source2_data_property_value25327, _) => {
                                                                            let source_ml_source2_data_property_value25327 = *source_ml_source2_data_property_value25327;
                                                                            Some((source_ml_source2_data_property_value25327.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(self::JavaScriptSemanticValue,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source2_concat_value25330 = (((match bound.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value25328) => {
                                                                                Some((source_ml_source2_data_property_value25328.clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone();
                                                                        {
                                                                            let source_ml_source2_concat_value25331 = shared_constructor_field7.clone();
                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), { let ml_reference_value1337 = source_ml_source2_concat_value25330.borrow().clone(); ml_reference_value1337 }), { let ml_reference_value1338 = source_ml_source2_concat_value25331.borrow().clone(); ml_reference_value1338 })
                                                                        }
                                                                    })))
                                                                }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallPending::WaitObjectSpread(
                                            _,
                                            _,
                                            state,
                                        ) => {
                                            let state = *state;
                                            match state.clone() {
                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, shared_constructor_field1, _, _, _, shared_constructor_field2, _, shared_constructor_field3, _, _, _, _) => {
                                                        let shared_constructor_field1 = *shared_constructor_field1;
                                                        let shared_constructor_field3 = *shared_constructor_field3;
                                                        {
                                                            let call = match state.clone() {
                                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25332) => {
                                                                    ((source_ml_source2_data_property_value25332.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectCopyCall>,))).0).clone()
                                                                }
                                                            };
                                                            if !(match (call.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match call.clone().expect("a proven present value") {
                                                                    self::JavaScriptObjectCopyCall::ObjectCopyCall(source_ml_source2_data_property_value25333, _, _) => {
                                                                        let source_ml_source2_data_property_value25333 = *source_ml_source2_data_property_value25333;
                                                                        source_ml_source2_data_property_value25333.clone()
                                                                    }
                                                                }, match call.clone().expect("a proven present value") {
                                                                    self::JavaScriptObjectCopyCall::ObjectCopyCall(_, source_ml_source2_data_property_value25334, _) => {
                                                                        let source_ml_source2_data_property_value25334 = *source_ml_source2_data_property_value25334;
                                                                        source_ml_source2_data_property_value25334.clone()
                                                                    }
                                                                }, match call.clone().expect("a proven present value") {
                                                                    self::JavaScriptObjectCopyCall::ObjectCopyCall(_, _, source_ml_source2_data_property_value25335) => {
                                                                        source_ml_source2_data_property_value25335.clone()
                                                                    }
                                                                })
                                                            } else {
                                                                {
                                                                    let callee = shared_constructor_field3.clone();
                                                                    {
                                                                        let receiver = if (shared_constructor_field9 == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108]))) {
                                                                            shared_constructor_field10.clone()
                                                                        } else {
                                                                            shared_constructor_field1.clone()
                                                                        };
                                                                        if (((((shared_constructor_field2 != (self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]))) || match callee.clone() {
                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                false
                                                                            }
                                                                            _ => {
                                                                                true
                                                                            }
                                                                        }) || (((((match callee.clone() {
                                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value25347, _) => {
                                                                                Some((source_ml_source2_data_property_value25347.clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(self::ml::Big,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field11)) || (({ let ml_reference_array1339 = shared_constructor_field13.clone(); let ml_reference_projection1339 = ml_reference_array1339.borrow().len() as f64; ml_reference_projection1339 }) != (0f64))) || !(self::test_java_script_semantic_same_value(({
                                                                            let source_ml_source2_array_read_value25352 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index25353 = 0f64;
                                                                                { let ml_reference_array1340 = source_ml_source2_array_read_value25352.clone(); let ml_reference_position1340 = source_ml_source2_array_read_index25353; let ml_reference_projection1340 = self::ml_array::optional_at(&*ml_reference_array1340.borrow(), self::ml_array::number_index(ml_reference_position1340)); ml_reference_projection1340 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), receiver.clone()))) {
                                                                            self::ml_string::Text::from_units(vec![97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 103, 101, 116, 116, 101, 114, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 101, 109, 112, 116, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![])
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                        }
                                        self::JavaScriptCallPending::WaitObjectEnumeration(
                                            _,
                                            _,
                                            state_2,
                                        ) => {
                                            let state_2 = *state_2;
                                            match state_2.clone() {
                                                    self::JavaScriptObjectEnumerationState::ObjectEnumeration(_, shared_constructor_field1_2, _, _, _, _, shared_constructor_field2_2, _, shared_constructor_field3_2, _, _, _, _) => {
                                                        let shared_constructor_field1_2 = *shared_constructor_field1_2;
                                                        let shared_constructor_field3_2 = *shared_constructor_field3_2;
                                                        if (shared_constructor_field2_2 != (self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]))) {
                                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114])
                                                        } else {
                                                            self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), shared_constructor_field3_2.clone(), shared_constructor_field1_2.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new())))
                                                        }
                                                    }
                                                }
                                        }
                                        self::JavaScriptCallPending::WaitArrayCreation(
                                            _,
                                            _,
                                            state_3,
                                            pending_construction,
                                            _,
                                            _,
                                        ) => {
                                            let state_3 = *state_3;
                                            {
                                                let construction = {
                                                    let source_ml_source2_optional_chain_receiver25358 = (((pending_construction.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone(),);
                                                    match if match (source_ml_source2_optional_chain_receiver25358.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                            Some({
                                                                let source_ml_source1_temporary_optional_chain1118 = ((source_ml_source2_optional_chain_receiver25358.clone()).0).clone().expect("a proven present value");
                                                                match source_ml_source1_temporary_optional_chain1118.clone() {
                                                                    self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25359, _) => {
                                                                        ((source_ml_source2_data_property_value25359.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                                                    }
                                                                }
                                                            })
                                                        } else {
                                                            None::<self::JavaScriptConstructionCall>
                                                        } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                };
                                                {
                                                    let callback = match state_3.clone() {
                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25360, _, _, _) => {
                                                                ((source_ml_source2_data_property_value25360.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationCall>,))).0).clone()
                                                            }
                                                        };
                                                    if match (construction.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    } {
                                                        if match (callback.clone(), None::<()>) {
                                                            (Some(_ml_left), Some(_ml_right)) => {
                                                                false
                                                            }
                                                            (None, None) => true,
                                                            _ => false,
                                                        } {
                                                            self::ml_string::Text::from_units(vec![
                                                                97, 110, 32, 65, 114, 114, 97, 121,
                                                                32, 99, 114, 101, 97, 116, 105,
                                                                111, 110, 32, 99, 104, 105, 108,
                                                                100, 32, 114, 101, 116, 97, 105,
                                                                110, 115, 32, 105, 116, 115, 32,
                                                                97, 99, 116, 117, 97, 108, 32, 115,
                                                                101, 108, 101, 99, 116, 101, 100,
                                                                32, 99, 97, 108, 108, 98, 97, 99,
                                                                107, 32, 111, 114, 32, 99, 111,
                                                                110, 115, 116, 114, 117, 99, 116,
                                                                111, 114, 32, 98, 111, 100, 121,
                                                            ])
                                                        } else {
                                                            self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match callback.clone().expect("a proven present value") {
                                                                    self::JavaScriptArrayCreationCall::ArrayCreationCallRecord(source_ml_source2_data_property_value25361, _, _) => {
                                                                        let source_ml_source2_data_property_value25361 = *source_ml_source2_data_property_value25361;
                                                                        source_ml_source2_data_property_value25361.clone()
                                                                    }
                                                                }, match callback.clone().expect("a proven present value") {
                                                                    self::JavaScriptArrayCreationCall::ArrayCreationCallRecord(_, source_ml_source2_data_property_value25362, _) => {
                                                                        let source_ml_source2_data_property_value25362 = *source_ml_source2_data_property_value25362;
                                                                        source_ml_source2_data_property_value25362.clone()
                                                                    }
                                                                }, match callback.clone().expect("a proven present value") {
                                                                    self::JavaScriptArrayCreationCall::ArrayCreationCallRecord(_, _, source_ml_source2_data_property_value25363) => {
                                                                        source_ml_source2_data_property_value25363.clone()
                                                                    }
                                                                })
                                                        }
                                                    } else {
                                                        {
                                                            let invalid = self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match construction.clone().expect("a proven present value") {
                                                                    self::JavaScriptConstructionCall::ConstructionCallRecord(source_ml_source2_data_property_value25364, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value25364 = *source_ml_source2_data_property_value25364;
                                                                        source_ml_source2_data_property_value25364.clone()
                                                                    }
                                                                }, match construction.clone().expect("a proven present value") {
                                                                    self::JavaScriptConstructionCall::ConstructionCallRecord(_, source_ml_source2_data_property_value25365, _, _, _) => {
                                                                        let source_ml_source2_data_property_value25365 = *source_ml_source2_data_property_value25365;
                                                                        source_ml_source2_data_property_value25365.clone()
                                                                    }
                                                                }, match construction.clone().expect("a proven present value") {
                                                                    self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, source_ml_source2_data_property_value25366, _, _) => {
                                                                        source_ml_source2_data_property_value25366.clone()
                                                                    }
                                                                });
                                                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                    invalid.clone()
                                                                } else {
                                                                    if (match construction.clone().expect("a proven present value") {
                                                                        self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, source_ml_source2_data_property_value25367, _) => {
                                                                            source_ml_source2_data_property_value25367
                                                                        }
                                                                    } && !(self::test_java_script_semantic_same_value((match frame.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25368) => {
                                                                            ((source_ml_source2_data_property_value25368.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match construction.clone().expect("a proven present value") {
                                                                        self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, _, source_ml_source2_data_property_value25369) => {
                                                                            let source_ml_source2_data_property_value25369 = *source_ml_source2_data_property_value25369;
                                                                            source_ml_source2_data_property_value25369.clone()
                                                                        }
                                                                    }))) {
                                                                        self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![])
                                                                    }
                                                                }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallPending::WaitConstruction(
                                            _,
                                            _,
                                            state_4,
                                            _,
                                        ) => {
                                            let state_4 = *state_4;
                                            {
                                                let call_2 = match state_4.clone() {
                                                        self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25370, _) => {
                                                            ((source_ml_source2_data_property_value25370.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                                        }
                                                    };
                                                if match (call_2.clone(), None::<()>) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                } {
                                                    self::ml_string::Text::from_units(vec![
                                                        97, 32, 99, 111, 110, 115, 116, 114, 117,
                                                        99, 116, 105, 111, 110, 32, 99, 104, 105,
                                                        108, 100, 32, 114, 101, 116, 97, 105, 110,
                                                        115, 32, 105, 116, 115, 32, 97, 99, 116,
                                                        117, 97, 108, 32, 115, 101, 108, 101, 99,
                                                        116, 101, 100, 32, 99, 97, 108, 108,
                                                    ])
                                                } else {
                                                    {
                                                        let invalid_2 = self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match call_2.clone().expect("a proven present value") {
                                                                self::JavaScriptConstructionCall::ConstructionCallRecord(source_ml_source2_data_property_value25371, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value25371 = *source_ml_source2_data_property_value25371;
                                                                    source_ml_source2_data_property_value25371.clone()
                                                                }
                                                            }, match call_2.clone().expect("a proven present value") {
                                                                self::JavaScriptConstructionCall::ConstructionCallRecord(_, source_ml_source2_data_property_value25372, _, _, _) => {
                                                                    let source_ml_source2_data_property_value25372 = *source_ml_source2_data_property_value25372;
                                                                    source_ml_source2_data_property_value25372.clone()
                                                                }
                                                            }, match call_2.clone().expect("a proven present value") {
                                                                self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, source_ml_source2_data_property_value25373, _, _) => {
                                                                    source_ml_source2_data_property_value25373.clone()
                                                                }
                                                            });
                                                        if (invalid_2
                                                            != (self::ml_string::Text::from_units(
                                                                vec![],
                                                            )))
                                                        {
                                                            invalid_2.clone()
                                                        } else {
                                                            if (match call_2.clone().expect("a proven present value") {
                                                                    self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, source_ml_source2_data_property_value25374, _) => {
                                                                        source_ml_source2_data_property_value25374
                                                                    }
                                                                } && !(self::test_java_script_semantic_same_value((match frame.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25375) => {
                                                                        ((source_ml_source2_data_property_value25375.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match call_2.clone().expect("a proven present value") {
                                                                    self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, _, source_ml_source2_data_property_value25376) => {
                                                                        let source_ml_source2_data_property_value25376 = *source_ml_source2_data_property_value25376;
                                                                        source_ml_source2_data_property_value25376.clone()
                                                                    }
                                                                }))) {
                                                                    self::ml_string::Text::from_units(vec![97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 111, 100, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => self::ml_string::Text::from_units(vec![]),
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
pub fn read_java_script_suspended_constructor_frame_error(
    parent: self::JavaScriptCallFrame,
    frame: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
    instruction: self::JavaScriptCallInstruction,
    destination: f64,
) -> self::ml_string::Text {
    match instruction.clone() {
        self::JavaScriptCallInstruction::ArgumentListCall(
            instruction_destination,
            _,
            callee,
            receiver,
            mode,
        ) => {
            if (mode
                == (self::ml_string::Text::from_units(vec![
                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115,
                ])))
            {
                self::read_java_script_constructor_frame_operands_error(
                    parent.clone(),
                    frame.clone(),
                    fn_.clone(),
                    destination,
                    instruction_destination,
                    callee,
                    true,
                    receiver,
                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,
                )
            } else {
                if (mode
                    == (self::ml_string::Text::from_units(vec![
                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110,
                    ])))
                {
                    self::read_java_script_constructor_frame_operands_error(
                        parent.clone(),
                        frame.clone(),
                        fn_.clone(),
                        destination,
                        instruction_destination,
                        callee,
                        false,
                        (-1f64),
                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,
                    )
                } else {
                    self::ml_string::Text::from_units(vec![
                        97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 114,
                        97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                        97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100,
                        32, 115, 111, 117, 114, 99, 101, 32, 110, 101, 119, 32, 111, 112, 101, 114,
                        97, 116, 105, 111, 110,
                    ])
                }
            }
        }
        self::JavaScriptCallInstruction::CallClassConstructor(
            instruction_destination_2,
            callee_2,
            receiver_2,
            arguments,
        ) => self::read_java_script_constructor_frame_operands_error(
            parent.clone(),
            frame.clone(),
            fn_.clone(),
            destination,
            instruction_destination_2,
            callee_2,
            true,
            receiver_2,
            Some(arguments.clone()),
        ),
        self::JavaScriptCallInstruction::ConstructCollection(
            instruction_destination_3,
            callee_3,
            arguments_2,
        ) => self::read_java_script_constructor_frame_operands_error(
            parent.clone(),
            frame.clone(),
            fn_.clone(),
            destination,
            instruction_destination_3,
            callee_3,
            false,
            (-1f64),
            Some(arguments_2.clone()),
        ),
        _ => self::ml_string::Text::from_units(vec![
            97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 114, 97, 109,
            101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
            108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 115, 111, 117, 114, 99, 101,
            32, 110, 101, 119, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110,
        ]),
    }
}
