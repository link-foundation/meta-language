// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptFunctionNameResult;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistryTransition;
use super::MlSource1TemporaryExecuteJavaScriptCallObjectInstructionJoin1317;
use super::accept_java_script_call_array_callback;
use super::accept_java_script_call_array_concatenation;
use super::accept_java_script_call_array_creation;
use super::accept_java_script_call_array_search;
use super::accept_java_script_call_array_slice;
use super::accept_java_script_call_array_spread;
use super::accept_java_script_call_array_transform;
use super::accept_java_script_call_collection_for_each;
use super::accept_java_script_call_construction;
use super::accept_java_script_call_constructor_check;
use super::accept_java_script_call_definition;
use super::accept_java_script_call_instance_of;
use super::accept_java_script_call_object_enumeration;
use super::accept_java_script_call_object_from_entries;
use super::accept_java_script_call_object_integrity;
use super::accept_java_script_call_object_spread;
use super::accept_java_script_call_primitive_wrapper;
use super::accept_java_script_call_property_definitions;
use super::accept_java_script_call_property_reflection;
use super::advance_java_script_call_frame;
use super::begin_java_script_array_callback;
use super::begin_java_script_array_concatenation;
use super::begin_java_script_array_creation;
use super::begin_java_script_array_search;
use super::begin_java_script_array_slice;
use super::begin_java_script_array_spread;
use super::begin_java_script_array_transform;
use super::begin_java_script_collection_for_each;
use super::begin_java_script_construction;
use super::begin_java_script_instance_of;
use super::begin_java_script_object_assignment;
use super::begin_java_script_object_enumeration;
use super::begin_java_script_object_from_entries;
use super::begin_java_script_object_observation;
use super::begin_java_script_object_spread;
use super::begin_java_script_primitive_wrapper;
use super::begin_java_script_property_definitions;
use super::begin_java_script_property_reflection;
use super::complete_java_script_property_definition_with_symbol;
use super::construct_java_script_property_function_name;
use super::create_java_script_ordinary_set_descriptor;
use super::create_java_script_property_descriptor;
use super::execute_java_script_symbol_registry_operation;
use super::find_java_script_ordinary_object;
use super::finish_java_script_call_function_name;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_executable_call_function;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::set_java_script_ordinary_prototype;
use super::test_java_script_semantic_same_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_observation_instruction(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    instruction: self::JavaScriptCallInstruction,
    realm: self::ml::Big,
) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            _,
            _,
            shared_constructor_field4,
            shared_constructor_field5,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(
                shared_constructor_field2,
                shared_constructor_field3,
            ) => {
                let shared_constructor_field2 = *shared_constructor_field2;
                match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(
                        _,
                        _,
                        shared_constructor_field1,
                        _,
                        _,
                        _,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        {
                            let next = self::advance_java_script_call_frame(frame.clone());
                            {
                                let current = self::replace_java_script_call_frame(
                                    shared_constructor_field2.clone(),
                                    shared_constructor_field3.clone(),
                                    next.clone(),
                                );
                                {
                                    let registers = shared_constructor_field4.clone();
                                    match instruction.clone() {
                                            self::JavaScriptCallInstruction::CollectionForEach(destination, operation) => {
                                                self::accept_java_script_call_collection_for_each(program.clone(), current.clone(), destination, realm.clone(), self::begin_java_script_collection_for_each(shared_constructor_field2.clone(), &(operation), ({
                                                    let source_ml_source2_array_read_index30210 = 0f64;
                                                    { let ml_reference_array1734 = registers.clone(); let ml_reference_position1734 = source_ml_source2_array_read_index30210; let ml_reference_projection1734 = self::ml_array::optional_at(&*ml_reference_array1734.borrow(), self::ml_array::number_index(ml_reference_position1734)); ml_reference_projection1734 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                    let source_ml_source2_array_read_value30212 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30213 = 0f64;
                                                        { let ml_reference_array1735 = source_ml_source2_array_read_value30212.clone(); let ml_reference_position1735 = source_ml_source2_array_read_index30213; let ml_reference_projection1735 = self::ml_array::optional_at(&*ml_reference_array1735.borrow(), self::ml_array::number_index(ml_reference_position1735)); ml_reference_projection1735 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                    let source_ml_source2_array_read_value30215 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30216 = 1f64;
                                                        { let ml_reference_array1736 = source_ml_source2_array_read_value30215.clone(); let ml_reference_position1736 = source_ml_source2_array_read_index30216; let ml_reference_projection1736 = self::ml_array::optional_at(&*ml_reference_array1736.borrow(), self::ml_array::number_index(ml_reference_position1736)); ml_reference_projection1736 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                            }
                                            self::JavaScriptCallInstruction::ArrayConcatenation(destination_2, input, spreadable_key, species) => {
                                                let species = *species;
                                                self::accept_java_script_call_array_concatenation(program.clone(), current.clone(), destination_2, realm.clone(), self::begin_java_script_array_concatenation(shared_constructor_field2.clone(), species.clone(), ({
                                                    let source_ml_source2_array_read_index30219 = (((input.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()).clone().unwrap_or_else(|| 0f64);
                                                    { let ml_reference_array1737 = registers.clone(); let ml_reference_position1737 = source_ml_source2_array_read_index30219; let ml_reference_projection1737 = self::ml_array::optional_at(&*ml_reference_array1737.borrow(), self::ml_array::number_index(ml_reference_position1737)); ml_reference_projection1737 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field5.clone(), ({ let ml_reference_array1738 = registers.clone(); let ml_reference_position1738 = spreadable_key; let ml_reference_projection1738 = self::ml_array::optional_at(&*ml_reference_array1738.borrow(), self::ml_array::number_index(ml_reference_position1738)); ml_reference_projection1738 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field1.clone()))
                                            }
                                            self::JavaScriptCallInstruction::ArraySlice(destination_3, input_2, species_2) => {
                                                let species_2 = *species_2;
                                                self::accept_java_script_call_array_slice(program.clone(), current.clone(), destination_3, realm.clone(), self::begin_java_script_array_slice(shared_constructor_field2.clone(), species_2.clone(), ({
                                                    let source_ml_source2_array_read_index30226 = (((input_2.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()).clone().unwrap_or_else(|| 0f64);
                                                    { let ml_reference_array1739 = registers.clone(); let ml_reference_position1739 = source_ml_source2_array_read_index30226; let ml_reference_projection1739 = self::ml_array::optional_at(&*ml_reference_array1739.borrow(), self::ml_array::number_index(ml_reference_position1739)); ml_reference_projection1739 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                    let source_ml_source2_array_read_value30228 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30229 = 0f64;
                                                        { let ml_reference_array1740 = source_ml_source2_array_read_value30228.clone(); let ml_reference_position1740 = source_ml_source2_array_read_index30229; let ml_reference_projection1740 = self::ml_array::optional_at(&*ml_reference_array1740.borrow(), self::ml_array::number_index(ml_reference_position1740)); ml_reference_projection1740 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                    let source_ml_source2_array_read_value30231 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30232 = 1f64;
                                                        { let ml_reference_array1741 = source_ml_source2_array_read_value30231.clone(); let ml_reference_position1741 = source_ml_source2_array_read_index30232; let ml_reference_projection1741 = self::ml_array::optional_at(&*ml_reference_array1741.borrow(), self::ml_array::number_index(ml_reference_position1741)); ml_reference_projection1741 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (({ let ml_reference_array1742 = shared_constructor_field5.clone(); let ml_reference_projection1742 = ml_reference_array1742.borrow().len() as f64; ml_reference_projection1742 }) > (1f64)), shared_constructor_field1.clone()))
                                            }
                                            self::JavaScriptCallInstruction::ArrayCallback(destination_4, operation_2, input_3) => {
                                                self::accept_java_script_call_array_callback(program.clone(), current.clone(), destination_4, realm.clone(), self::begin_java_script_array_callback(shared_constructor_field2.clone(), &(operation_2), ({
                                                    let source_ml_source2_array_read_index30238 = (((input_3.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()).clone().unwrap_or_else(|| 0f64);
                                                    { let ml_reference_array1743 = registers.clone(); let ml_reference_position1743 = source_ml_source2_array_read_index30238; let ml_reference_projection1743 = self::ml_array::optional_at(&*ml_reference_array1743.borrow(), self::ml_array::number_index(ml_reference_position1743)); ml_reference_projection1743 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                    let source_ml_source2_array_read_value30240 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30241 = 0f64;
                                                        { let ml_reference_array1744 = source_ml_source2_array_read_value30240.clone(); let ml_reference_position1744 = source_ml_source2_array_read_index30241; let ml_reference_projection1744 = self::ml_array::optional_at(&*ml_reference_array1744.borrow(), self::ml_array::number_index(ml_reference_position1744)); ml_reference_projection1744 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                    let source_ml_source2_array_read_value30243 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30244 = 1f64;
                                                        { let ml_reference_array1745 = source_ml_source2_array_read_value30243.clone(); let ml_reference_position1745 = source_ml_source2_array_read_index30244; let ml_reference_projection1745 = self::ml_array::optional_at(&*ml_reference_array1745.borrow(), self::ml_array::number_index(ml_reference_position1745)); ml_reference_projection1745 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field1.clone(), (({ let ml_reference_array1746 = shared_constructor_field5.clone(); let ml_reference_projection1746 = ml_reference_array1746.borrow().len() as f64; ml_reference_projection1746 }) >= (2f64))))
                                            }
                                            self::JavaScriptCallInstruction::ArraySearch(destination_5, operation_3, input_4) => {
                                                {
                                                    let at = (operation_3 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 97, 116])));
                                                    self::accept_java_script_call_array_search(program.clone(), current.clone(), destination_5, realm.clone(), self::begin_java_script_array_search(shared_constructor_field2.clone(), &(operation_3), ({
                                                        let source_ml_source2_array_read_index30250 = (((input_4.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()).clone().unwrap_or_else(|| 0f64);
                                                        { let ml_reference_array1747 = registers.clone(); let ml_reference_position1747 = source_ml_source2_array_read_index30250; let ml_reference_projection1747 = self::ml_array::optional_at(&*ml_reference_array1747.borrow(), self::ml_array::number_index(ml_reference_position1747)); ml_reference_projection1747 }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), if at {
                                                        self::JavaScriptSemanticValue::Undefined
                                                    } else {
                                                        ({
                                                            let source_ml_source2_array_read_value30252 = shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_array_read_index30253 = 0f64;
                                                                { let ml_reference_array1748 = source_ml_source2_array_read_value30252.clone(); let ml_reference_position1748 = source_ml_source2_array_read_index30253; let ml_reference_projection1748 = self::ml_array::optional_at(&*ml_reference_array1748.borrow(), self::ml_array::number_index(ml_reference_position1748)); ml_reference_projection1748 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)
                                                    }, ({
                                                        let source_ml_source2_array_read_value30255 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_array_read_index30256 = if at {
                                                                0f64
                                                            } else {
                                                                1f64
                                                            };
                                                            { let ml_reference_array1749 = source_ml_source2_array_read_value30255.clone(); let ml_reference_position1749 = source_ml_source2_array_read_index30256; let ml_reference_projection1749 = self::ml_array::optional_at(&*ml_reference_array1749.borrow(), self::ml_array::number_index(ml_reference_position1749)); ml_reference_projection1749 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (({ let ml_reference_array1750 = shared_constructor_field5.clone(); let ml_reference_projection1750 = ml_reference_array1750.borrow().len() as f64; ml_reference_projection1750 }) >= (if at {
                                                        1f64
                                                    } else {
                                                        2f64
                                                    })), shared_constructor_field1.clone()))
                                                }
                                            }
                                            self::JavaScriptCallInstruction::ArrayTransform(destination_6, operation_4, species_3, input_5) => {
                                                let species_3 = *species_3;
                                                self::accept_java_script_call_array_transform(program.clone(), current.clone(), destination_6, realm.clone(), self::begin_java_script_array_transform(shared_constructor_field2.clone(), species_3.clone(), &(operation_4), ({
                                                    let source_ml_source2_array_read_index30262 = (((input_5.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()).clone().unwrap_or_else(|| 0f64);
                                                    { let ml_reference_array1751 = registers.clone(); let ml_reference_position1751 = source_ml_source2_array_read_index30262; let ml_reference_projection1751 = self::ml_array::optional_at(&*ml_reference_array1751.borrow(), self::ml_array::number_index(ml_reference_position1751)); ml_reference_projection1751 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                    let source_ml_source2_array_read_value30264 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30265 = 0f64;
                                                        { let ml_reference_array1752 = source_ml_source2_array_read_value30264.clone(); let ml_reference_position1752 = source_ml_source2_array_read_index30265; let ml_reference_projection1752 = self::ml_array::optional_at(&*ml_reference_array1752.borrow(), self::ml_array::number_index(ml_reference_position1752)); ml_reference_projection1752 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                    let source_ml_source2_array_read_value30267 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30268 = 1f64;
                                                        { let ml_reference_array1753 = source_ml_source2_array_read_value30267.clone(); let ml_reference_position1753 = source_ml_source2_array_read_index30268; let ml_reference_projection1753 = self::ml_array::optional_at(&*ml_reference_array1753.borrow(), self::ml_array::number_index(ml_reference_position1753)); ml_reference_projection1753 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field1.clone()))
                                            }
                                            self::JavaScriptCallInstruction::ObjectEnumeration(destination_7, operation_5, prototype) => {
                                                self::accept_java_script_call_object_enumeration(program.clone(), current.clone(), destination_7, realm.clone(), self::begin_java_script_object_enumeration(shared_constructor_field2.clone(), &(operation_5), ({
                                                    let source_ml_source2_array_read_value30272 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30273 = 0f64;
                                                        { let ml_reference_array1754 = source_ml_source2_array_read_value30272.clone(); let ml_reference_position1754 = source_ml_source2_array_read_index30273; let ml_reference_projection1754 = self::ml_array::optional_at(&*ml_reference_array1754.borrow(), self::ml_array::number_index(ml_reference_position1754)); ml_reference_projection1754 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1755 = registers.clone(); let ml_reference_position1755 = prototype; let ml_reference_projection1755 = self::ml_array::optional_at(&*ml_reference_array1755.borrow(), self::ml_array::number_index(ml_reference_position1755)); ml_reference_projection1755 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                            }
                                            self::JavaScriptCallInstruction::PropertyDefinitions(destination_8, operation_6, target, input_6) => {
                                                self::accept_java_script_call_property_definitions(program.clone(), current.clone(), destination_8, realm.clone(), self::begin_java_script_property_definitions(shared_constructor_field2.clone(), &(operation_6), ({ let ml_reference_array1756 = registers.clone(); let ml_reference_position1756 = target; let ml_reference_projection1756 = self::ml_array::optional_at(&*ml_reference_array1756.borrow(), self::ml_array::number_index(ml_reference_position1756)); ml_reference_projection1756 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1757 = registers.clone(); let ml_reference_position1757 = input_6; let ml_reference_projection1757 = self::ml_array::optional_at(&*ml_reference_array1757.borrow(), self::ml_array::number_index(ml_reference_position1757)); ml_reference_projection1757 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field1.clone()))
                                            }
                                            self::JavaScriptCallInstruction::PropertyReflection(destination_9, operation_7, input_7, key, object_prototype, array_prototype) => {
                                                self::accept_java_script_call_property_reflection(current.clone(), destination_9, realm.clone(), self::begin_java_script_property_reflection(shared_constructor_field2.clone(), &(operation_7), ({ let ml_reference_array1758 = registers.clone(); let ml_reference_position1758 = input_7; let ml_reference_projection1758 = self::ml_array::optional_at(&*ml_reference_array1758.borrow(), self::ml_array::number_index(ml_reference_position1758)); ml_reference_projection1758 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1759 = registers.clone(); let ml_reference_position1759 = key; let ml_reference_projection1759 = self::ml_array::optional_at(&*ml_reference_array1759.borrow(), self::ml_array::number_index(ml_reference_position1759)); ml_reference_projection1759 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1760 = registers.clone(); let ml_reference_position1760 = object_prototype; let ml_reference_projection1760 = self::ml_array::optional_at(&*ml_reference_array1760.borrow(), self::ml_array::number_index(ml_reference_position1760)); ml_reference_projection1760 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1761 = registers.clone(); let ml_reference_position1761 = array_prototype; let ml_reference_projection1761 = self::ml_array::optional_at(&*ml_reference_array1761.borrow(), self::ml_array::number_index(ml_reference_position1761)); ml_reference_projection1761 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                            }
                                            self::JavaScriptCallInstruction::InstanceOf(destination_10, left, right, key_2, operation_8) => {
                                                self::accept_java_script_call_instance_of(program.clone(), current.clone(), destination_10, realm.clone(), self::begin_java_script_instance_of(shared_constructor_field2.clone(), &(operation_8), ({ let ml_reference_array1762 = registers.clone(); let ml_reference_position1762 = left; let ml_reference_projection1762 = self::ml_array::optional_at(&*ml_reference_array1762.borrow(), self::ml_array::number_index(ml_reference_position1762)); ml_reference_projection1762 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1763 = registers.clone(); let ml_reference_position1763 = right; let ml_reference_projection1763 = self::ml_array::optional_at(&*ml_reference_array1763.borrow(), self::ml_array::number_index(ml_reference_position1763)); ml_reference_projection1763 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1764 = registers.clone(); let ml_reference_position1764 = key_2; let ml_reference_projection1764 = self::ml_array::optional_at(&*ml_reference_array1764.borrow(), self::ml_array::number_index(ml_reference_position1764)); ml_reference_projection1764 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                            }
                                            self::JavaScriptCallInstruction::ObjectObservation(destination_11, operation_9, input_8, other) => {
                                                self::accept_java_script_call_object_integrity(current.clone(), destination_11, realm.clone(), self::begin_java_script_object_observation(shared_constructor_field2.clone(), &(operation_9), ({ let ml_reference_array1765 = registers.clone(); let ml_reference_position1765 = input_8; let ml_reference_projection1765 = self::ml_array::optional_at(&*ml_reference_array1765.borrow(), self::ml_array::number_index(ml_reference_position1765)); ml_reference_projection1765 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1766 = registers.clone(); let ml_reference_position1766 = other; let ml_reference_projection1766 = self::ml_array::optional_at(&*ml_reference_array1766.borrow(), self::ml_array::number_index(ml_reference_position1766)); ml_reference_projection1766 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                            }
                                            self::JavaScriptCallInstruction::ArraySpread(destination_12, target_2, input_9, instruction_index, iterator_key, realm_2) => {
                                                {
                                                    let index = ({ let ml_reference_array1767 = registers.clone(); let ml_reference_position1767 = instruction_index; let ml_reference_projection1767 = self::ml_array::optional_at(&*ml_reference_array1767.borrow(), self::ml_array::number_index(ml_reference_position1767)); ml_reference_projection1767 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    match index.clone() {
                                                        self::JavaScriptSemanticValue::Number(value) => {
                                                            self::accept_java_script_call_array_spread(program.clone(), current.clone(), destination_12, realm_2.clone(), self::begin_java_script_array_spread(shared_constructor_field2.clone(), ({ let ml_reference_array1768 = registers.clone(); let ml_reference_position1768 = target_2; let ml_reference_projection1768 = self::ml_array::optional_at(&*ml_reference_array1768.borrow(), self::ml_array::number_index(ml_reference_position1768)); ml_reference_projection1768 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1769 = registers.clone(); let ml_reference_position1769 = input_9; let ml_reference_projection1769 = self::ml_array::optional_at(&*ml_reference_array1769.borrow(), self::ml_array::number_index(ml_reference_position1769)); ml_reference_projection1769 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), value, ({ let ml_reference_array1770 = registers.clone(); let ml_reference_position1770 = iterator_key; let ml_reference_projection1770 = self::ml_array::optional_at(&*ml_reference_array1770.borrow(), self::ml_array::number_index(ml_reference_position1770)); ml_reference_projection1770 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                                        }
                                                        _ => {
                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100, 32, 105, 110, 100, 101, 120, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101]))
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptCallInstruction::ObjectSpread(destination_13, target_3, input_10, realm_3, excluded, operation_10) => {
                                                self::accept_java_script_call_object_spread(program.clone(), current.clone(), destination_13, realm_3.clone(), if {
                                                    let source_ml_source2_comparison_operand30313 = ((operation_10.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                                    match (source_ml_source2_comparison_operand30313.clone(), self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                } {
                                                    self::begin_java_script_object_assignment(shared_constructor_field2.clone(), ({ let ml_reference_array1771 = registers.clone(); let ml_reference_position1771 = target_3; let ml_reference_projection1771 = self::ml_array::optional_at(&*ml_reference_array1771.borrow(), self::ml_array::number_index(ml_reference_position1771)); ml_reference_projection1771 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1772 = registers.clone(); let ml_reference_position1772 = input_10; let ml_reference_projection1772 = self::ml_array::optional_at(&*ml_reference_array1772.borrow(), self::ml_array::number_index(ml_reference_position1772)); ml_reference_projection1772 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field1.clone())
                                                } else {
                                                    self::begin_java_script_object_spread(shared_constructor_field2.clone(), ({ let ml_reference_array1773 = registers.clone(); let ml_reference_position1773 = target_3; let ml_reference_projection1773 = self::ml_array::optional_at(&*ml_reference_array1773.borrow(), self::ml_array::number_index(ml_reference_position1773)); ml_reference_projection1773 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1774 = registers.clone(); let ml_reference_position1774 = input_10; let ml_reference_projection1774 = self::ml_array::optional_at(&*ml_reference_array1774.borrow(), self::ml_array::number_index(ml_reference_position1774)); ml_reference_projection1774 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values1775 = { let ml_reference_value1776 = ((((excluded.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value1776 }; let ml_callback1775 = { let registers = registers.clone(); std::rc::Rc::new(move |index_2: f64| -> self::JavaScriptSemanticValue { ({
                                                        let source_ml_source2_array_read_value30326 = registers.clone();
                                                        {
                                                            let source_ml_source2_array_read_index30327 = index_2;
                                                            { let ml_reference_array1777 = source_ml_source2_array_read_value30326.clone(); let ml_reference_position1777 = source_ml_source2_array_read_index30327; let ml_reference_projection1777 = self::ml_array::optional_at(&*ml_reference_array1777.borrow(), self::ml_array::number_index(ml_reference_position1777)); ml_reference_projection1777 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values1775.iter().cloned().enumerate().map(|(ml_index1775, ml_value1775)| ml_callback1775(ml_value1775)).collect::<Vec<_>>() })))
                                                })
                                            }
                                            _ => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 102, 97, 109, 105, 108, 121]))
                                            }
                                        }
                                }
                            }
                        }
                    }
                }
            }
        },
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_object_instruction(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    instruction: self::JavaScriptCallInstruction,
    realm: self::ml::Big,
) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            _,
            _,
            shared_constructor_field15,
            shared_constructor_field16,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(
                shared_constructor_field13,
                shared_constructor_field14,
            ) => {
                let shared_constructor_field13 = *shared_constructor_field13;
                match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(
                        _,
                        _,
                        shared_constructor_field12,
                        _,
                        _,
                        _,
                    ) => {
                        let shared_constructor_field12 = *shared_constructor_field12;
                        {
                            let next = self::advance_java_script_call_frame(frame.clone());
                            match next.clone() {
                                self::JavaScriptCallFrame::CallFrame(
                                    shared_constructor_field2,
                                    shared_constructor_field3,
                                    shared_constructor_field4,
                                    shared_constructor_field5,
                                    shared_constructor_field6,
                                    shared_constructor_field7,
                                    shared_constructor_field8,
                                    shared_constructor_field9,
                                    shared_constructor_field10,
                                    shared_constructor_field11,
                                ) => {
                                    let shared_constructor_field7 = *shared_constructor_field7;
                                    {
                                        let current = self::replace_java_script_call_frame(
                                            shared_constructor_field13.clone(),
                                            shared_constructor_field14.clone(),
                                            next.clone(),
                                        );
                                        match current.clone() {
                                            self::JavaScriptCallMachine::CallMachine(
                                                _,
                                                shared_constructor_field1,
                                            ) => {
                                                let registers = shared_constructor_field15.clone();
                                                match instruction.clone() {
                                                            self::JavaScriptCallInstruction::Construct(destination, callee, new_target, arguments) => {
                                                                {
                                                                    let directory = match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value30332, _) => {
                                                                            ((source_ml_source2_data_property_value30332.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                        }
                                                                    };
                                                                    if match (directory.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field13.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                                    } else {
                                                                        self::accept_java_script_call_construction(program.clone(), current.clone(), destination, realm.clone(), self::begin_java_script_construction(program.clone(), shared_constructor_field13.clone(), directory.clone().expect("a proven present value"), ({ let ml_reference_array1778 = registers.clone(); let ml_reference_position1778 = callee; let ml_reference_projection1778 = self::ml_array::optional_at(&*ml_reference_array1778.borrow(), self::ml_array::number_index(ml_reference_position1778)); ml_reference_projection1778 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1779 = registers.clone(); let ml_reference_position1779 = new_target; let ml_reference_projection1779 = self::ml_array::optional_at(&*ml_reference_array1779.borrow(), self::ml_array::number_index(ml_reference_position1779)); ml_reference_projection1779 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values1780 = { let ml_reference_value1781 = arguments.borrow().clone(); ml_reference_value1781 }; let ml_callback1780 = { let registers = registers.clone(); std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { ({
                                                                            let source_ml_source2_array_read_value30339 = registers.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index30340 = index;
                                                                                { let ml_reference_array1782 = source_ml_source2_array_read_value30339.clone(); let ml_reference_position1782 = source_ml_source2_array_read_index30340; let ml_reference_projection1782 = self::ml_array::optional_at(&*ml_reference_array1782.borrow(), self::ml_array::number_index(ml_reference_position1782)); ml_reference_projection1782 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values1780.iter().cloned().enumerate().map(|(ml_index1780, ml_value1780)| ml_callback1780(ml_value1780)).collect::<Vec<_>>() }))), None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>)
                                                                    }
                                                                }
                                                            }
                                                            self::JavaScriptCallInstruction::ConstructorCheck(destination_2, input, required) => {
                                                                self::accept_java_script_call_constructor_check(program.clone(), current.clone(), destination_2, ({ let ml_reference_array1783 = registers.clone(); let ml_reference_position1783 = input; let ml_reference_projection1783 = self::ml_array::optional_at(&*ml_reference_array1783.borrow(), self::ml_array::number_index(ml_reference_position1783)); ml_reference_projection1783 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), required)
                                                            }
                                                            self::JavaScriptCallInstruction::ReadNewTarget(destination_3) => {
                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field13.clone(), shared_constructor_field14.clone(), self::write_java_script_call_register(next.clone(), destination_3, (match frame.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value30345) => {
                                                                        ((source_ml_source2_data_property_value30345.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                            }
                                                            self::JavaScriptCallInstruction::ArrayJoinEnter(destination_4, instruction_receiver) => {
                                                                {
                                                                    let receiver = ({ let ml_reference_array1784 = registers.clone(); let ml_reference_position1784 = instruction_receiver; let ml_reference_projection1784 = self::ml_array::optional_at(&*ml_reference_array1784.borrow(), self::ml_array::number_index(ml_reference_position1784)); ml_reference_projection1784 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                    if (match receiver.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    } || !(match (match frame.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value30350, _) => {
                                                                            ((source_ml_source2_data_property_value30350.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                        }
                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field13.clone()), self::ml_string::Text::from_units(vec![106, 111, 105, 110, 32, 101, 110, 116, 114, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 97, 110, 100, 32, 102, 114, 101, 115, 104, 32, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110]))
                                                                    } else {
                                                                        {
                                                                            let recursive = { let ml_values1785 = { let ml_reference_value1786 = shared_constructor_field14.borrow().clone(); ml_reference_value1786 }; let ml_callback1785 = { let program = program.clone(); let machine = machine.clone(); let realm = realm.clone(); let receiver = receiver.clone(); std::rc::Rc::new(move |parent: self::JavaScriptCallFrame| -> bool { ((!(match (match parent.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value30353, _) => {
                                                                                    ((source_ml_source2_data_property_value30353.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                                                                                let source_ml_source2_comparison_operand30358 = {
                                                                                    let source_ml_source2_optional_chain_receiver30356 = (self::read_java_script_executable_call_function(program.clone(), match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value30354, _) => {
                                                                                            let source_ml_source2_data_property_value30354 = *source_ml_source2_data_property_value30354;
                                                                                            source_ml_source2_data_property_value30354.clone()
                                                                                        }
                                                                                    }, match parent.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value30355, _, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value30355.clone()
                                                                                        }
                                                                                    }),);
                                                                                    if match (source_ml_source2_optional_chain_receiver30356.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                        Some({
                                                                                            let source_ml_source1_temporary_optional_chain1314 = ((source_ml_source2_optional_chain_receiver30356.clone()).0).clone().expect("a proven present value");
                                                                                            match source_ml_source1_temporary_optional_chain1314.clone() {
                                                                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value30357, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value30357.clone()
                                                                                                }
                                                                                            }
                                                                                        })
                                                                                    } else {
                                                                                        None::<self::ml::Big>
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_comparison_operand30359 = realm.clone();
                                                                                    match (source_ml_source2_comparison_operand30358.clone(), source_ml_source2_comparison_operand30359.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                }
                                                                            }) && self::test_java_script_semantic_same_value((match parent.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value30360, _) => {
                                                                                    ((source_ml_source2_data_property_value30360.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value"), receiver.clone())) }) }; ml_values1785.iter().cloned().enumerate().any(|(ml_index1785, ml_value1785)| ml_callback1785(ml_value1785)) };
                                                                            {
                                                                                let joined_frame = if recursive {
                                                                                    next.clone()
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source2_nominal_field30362 = shared_constructor_field2.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field30364 = shared_constructor_field3;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field30366 = shared_constructor_field4.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field30368 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field30370 = shared_constructor_field6.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field30372 = shared_constructor_field7.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field30374 = shared_constructor_field8.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field30376 = shared_constructor_field9.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field30378 = shared_constructor_field10.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field30380 = shared_constructor_field11.clone();
                                                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field30362.clone(), source_ml_source2_nominal_field30364, source_ml_source2_nominal_field30366.clone(), source_ml_source2_nominal_field30368.clone(), source_ml_source2_nominal_field30370.clone(), Box::new(source_ml_source2_nominal_field30372.clone()), source_ml_source2_nominal_field30374.clone(), source_ml_source2_nominal_field30376.clone(), Some((Some(receiver.clone()),)), source_ml_source2_nominal_field30380.clone())
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
                                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field13.clone(), shared_constructor_field1.clone(), self::write_java_script_call_register(joined_frame.clone(), destination_4, self::JavaScriptSemanticValue::Boolean(!recursive)))))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::JavaScriptCallInstruction::ArrayJoinLeave(instruction_receiver_2) => {
                                                                {
                                                                    let receiver_2 = ({ let ml_reference_array1787 = registers.clone(); let ml_reference_position1787 = instruction_receiver_2; let ml_reference_projection1787 = self::ml_array::optional_at(&*ml_reference_array1787.borrow(), self::ml_array::number_index(ml_reference_position1787)); ml_reference_projection1787 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                    if (match (match frame.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value30387, _) => {
                                                                            ((source_ml_source2_data_property_value30387.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                        }
                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value((match frame.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value30388, _) => {
                                                                            ((source_ml_source2_data_property_value30388.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value"), receiver_2.clone()))) {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field13.clone()), self::ml_string::Text::from_units(vec![106, 111, 105, 110, 32, 108, 101, 97, 118, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 108, 121, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 103, 117, 97, 114, 100]))
                                                                    } else {
                                                                        {
                                                                            let released = match next.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value30390, _) => {
                                                                                    ((source_ml_source2_data_property_value30390.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let released_frame = match next.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_rest_field30391, source_ml_source2_nominal_rest_field30392, source_ml_source2_nominal_rest_field30393, source_ml_source2_nominal_rest_field30394, source_ml_source2_nominal_rest_field30395, source_ml_source2_nominal_rest_field30396, source_ml_source2_nominal_rest_field30397, source_ml_source2_nominal_rest_field30398, _, source_ml_source2_nominal_rest_field30399) => {
                                                                                        let source_ml_source2_nominal_rest_field30396 = *source_ml_source2_nominal_rest_field30396;
                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_rest_field30391.clone(), source_ml_source2_nominal_rest_field30392, source_ml_source2_nominal_rest_field30393.clone(), source_ml_source2_nominal_rest_field30394.clone(), source_ml_source2_nominal_rest_field30395.clone(), Box::new(source_ml_source2_nominal_rest_field30396.clone()), source_ml_source2_nominal_rest_field30397.clone(), source_ml_source2_nominal_rest_field30398.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_nominal_rest_field30399.clone())
                                                                                    }
                                                                                };
                                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field13.clone(), shared_constructor_field1.clone(), released_frame.clone())))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::JavaScriptCallInstruction::ArrayCreation(destination_5, operation, prototype, iterator_key) => {
                                                                self::accept_java_script_call_array_creation(program.clone(), current.clone(), destination_5, realm.clone(), self::begin_java_script_array_creation(shared_constructor_field13.clone(), &(operation), ({
                                                                    let source_ml_source2_array_read_index30404 = 0f64;
                                                                    { let ml_reference_array1788 = registers.clone(); let ml_reference_position1788 = source_ml_source2_array_read_index30404; let ml_reference_projection1788 = self::ml_array::optional_at(&*ml_reference_array1788.borrow(), self::ml_array::number_index(ml_reference_position1788)); ml_reference_projection1788 }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field16.clone(), ({ let ml_reference_array1789 = registers.clone(); let ml_reference_position1789 = prototype; let ml_reference_projection1789 = self::ml_array::optional_at(&*ml_reference_array1789.borrow(), self::ml_array::number_index(ml_reference_position1789)); ml_reference_projection1789 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1790 = registers.clone(); let ml_reference_position1790 = iterator_key; let ml_reference_projection1790 = self::ml_array::optional_at(&*ml_reference_array1790.borrow(), self::ml_array::number_index(ml_reference_position1790)); ml_reference_projection1790 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field12.clone()))
                                                            }
                                                            self::JavaScriptCallInstruction::LiteralPrototype(destination_6, instruction_object, instruction_value) => {
                                                                {
                                                                    let object = ({ let ml_reference_array1791 = registers.clone(); let ml_reference_position1791 = instruction_object; let ml_reference_projection1791 = self::ml_array::optional_at(&*ml_reference_array1791.borrow(), self::ml_array::number_index(ml_reference_position1791)); ml_reference_projection1791 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                    {
                                                                        let value = ({ let ml_reference_array1792 = registers.clone(); let ml_reference_position1792 = instruction_value; let ml_reference_projection1792 = self::ml_array::optional_at(&*ml_reference_array1792.borrow(), self::ml_array::number_index(ml_reference_position1792)); ml_reference_projection1792 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                        match object.clone() {
                                                                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                                                                if (match value.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                } && match value.clone() {
                                                                                    self::JavaScriptSemanticValue::Null => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }) {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field13.clone(), shared_constructor_field14.clone(), self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Undefined))))
                                                                                } else {
                                                                                    {
                                                                                        let installed = self::set_java_script_ordinary_prototype(shared_constructor_field13.clone(), identity.clone(), value.clone());
                                                                                        if (((({ let ml_reference_value1793 = installed.borrow().clone(); ml_reference_value1793 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new((({ let ml_reference_value1794 = installed.borrow().clone(); ml_reference_value1794 }).0).1), (({ let ml_reference_value1795 = installed.borrow().clone(); ml_reference_value1795 }).0).0)
                                                                                        } else {
                                                                                            if !((({ let ml_reference_value1796 = installed.borrow().clone(); ml_reference_value1796 }).0).2) {
                                                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new((({ let ml_reference_value1797 = installed.borrow().clone(); ml_reference_value1797 }).0).1), self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 108, 105, 116, 101, 114, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 97, 100, 109, 105, 116, 32, 105, 116, 115, 32, 97, 99, 121, 99, 108, 105, 99, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                                                            } else {
                                                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame((({ let ml_reference_value1798 = installed.borrow().clone(); ml_reference_value1798 }).0).1, shared_constructor_field14.clone(), self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Undefined))))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            _ => {
                                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field13.clone()), self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 105, 110, 115, 116, 97, 108, 108, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 79, 98, 106, 101, 99, 116]))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::JavaScriptCallInstruction::LiteralAccessor(destination_7, object_2, key, instruction_value_2, kind, realm_2) => {
                                                                {
                                                                    let descriptor = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])), true, true);
                                                                    {
                                                                        let value_2 = ({ let ml_reference_array1799 = registers.clone(); let ml_reference_position1799 = instruction_value_2; let ml_reference_projection1799 = self::ml_array::optional_at(&*ml_reference_array1799.borrow(), self::ml_array::number_index(ml_reference_position1799)); ml_reference_projection1799 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                        {
                                                                            let source_ml_source2_s30425 = if (kind == (self::ml_string::Text::from_units(vec![103, 101, 116]))) {
                                                                                {
                                                                                    let source_ml_source1_temporary_execute_java_script_call_object_instruction_written1318 = { let ml_field_owner1800 = descriptor.clone(); let ml_field_value1800 = value_2.clone(); ml_field_owner1800.borrow_mut().0.4 = ml_field_value1800; () };
                                                                                    {
                                                                                        let source_ml_source1_temporary_execute_java_script_call_object_instruction_written1319 = { let ml_field_owner1801 = descriptor.clone(); let ml_field_value1801 = false; ml_field_owner1801.borrow_mut().0.7 = ml_field_value1801; () };
                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallObjectInstructionJoin1317::MlSource1TemporaryExecuteJavaScriptCallObjectInstructionJoin1317Next
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                {
                                                                                    let source_ml_source1_temporary_execute_java_script_call_object_instruction_written1320 = { let ml_field_owner1802 = descriptor.clone(); let ml_field_value1802 = value_2.clone(); ml_field_owner1802.borrow_mut().0.6 = ml_field_value1802; () };
                                                                                    {
                                                                                        let source_ml_source1_temporary_execute_java_script_call_object_instruction_written1321 = { let ml_field_owner1803 = descriptor.clone(); let ml_field_value1803 = false; ml_field_owner1803.borrow_mut().0.5 = ml_field_value1803; () };
                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallObjectInstructionJoin1317::MlSource1TemporaryExecuteJavaScriptCallObjectInstructionJoin1317Next
                                                                                    }
                                                                                }
                                                                            };
                                                                            match source_ml_source2_s30425.clone() {
                                                                                self::MlSource1TemporaryExecuteJavaScriptCallObjectInstructionJoin1317::MlSource1TemporaryExecuteJavaScriptCallObjectInstructionJoin1317Next => {
                                                                                    self::accept_java_script_call_definition(program.clone(), current.clone(), destination_7, realm_2.clone(), self::complete_java_script_property_definition_with_symbol(shared_constructor_field13.clone(), ({ let ml_reference_array1804 = registers.clone(); let ml_reference_position1804 = object_2; let ml_reference_projection1804 = self::ml_array::optional_at(&*ml_reference_array1804.borrow(), self::ml_array::number_index(ml_reference_position1804)); ml_reference_projection1804 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1805 = registers.clone(); let ml_reference_position1805 = key; let ml_reference_projection1805 = self::ml_array::optional_at(&*ml_reference_array1805.borrow(), self::ml_array::number_index(ml_reference_position1805)); ml_reference_projection1805 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), descriptor.clone(), Some(shared_constructor_field12.clone()), None::<self::ml_string::Text>))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::JavaScriptCallInstruction::ObjectFromEntries(destination_8, input_2, prototype_2, iterator_key_2) => {
                                                                self::accept_java_script_call_object_from_entries(program.clone(), current.clone(), destination_8, realm.clone(), self::begin_java_script_object_from_entries(shared_constructor_field13.clone(), ({ let ml_reference_array1806 = registers.clone(); let ml_reference_position1806 = input_2; let ml_reference_projection1806 = self::ml_array::optional_at(&*ml_reference_array1806.borrow(), self::ml_array::number_index(ml_reference_position1806)); ml_reference_projection1806 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1807 = registers.clone(); let ml_reference_position1807 = prototype_2; let ml_reference_projection1807 = self::ml_array::optional_at(&*ml_reference_array1807.borrow(), self::ml_array::number_index(ml_reference_position1807)); ml_reference_projection1807 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1808 = registers.clone(); let ml_reference_position1808 = iterator_key_2; let ml_reference_projection1808 = self::ml_array::optional_at(&*ml_reference_array1808.borrow(), self::ml_array::number_index(ml_reference_position1808)); ml_reference_projection1808 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field12.clone()))
                                                            }
                                                            self::JavaScriptCallInstruction::PrimitiveWrapper(destination_9, input_3, prototype_3) => {
                                                                self::accept_java_script_call_primitive_wrapper(current.clone(), destination_9, self::begin_java_script_primitive_wrapper(shared_constructor_field13.clone(), ({ let ml_reference_array1809 = registers.clone(); let ml_reference_position1809 = input_3; let ml_reference_projection1809 = self::ml_array::optional_at(&*ml_reference_array1809.borrow(), self::ml_array::number_index(ml_reference_position1809)); ml_reference_projection1809 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1810 = registers.clone(); let ml_reference_position1810 = prototype_3; let ml_reference_projection1810 = self::ml_array::optional_at(&*ml_reference_array1810.borrow(), self::ml_array::number_index(ml_reference_position1810)); ml_reference_projection1810 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                                            }
                                                            self::JavaScriptCallInstruction::SymbolAllocate(destination_10, input_4) => {
                                                                {
                                                                    let description = ({ let ml_reference_array1811 = registers.clone(); let ml_reference_position1811 = input_4; let ml_reference_projection1811 = self::ml_array::optional_at(&*ml_reference_array1811.borrow(), self::ml_array::number_index(ml_reference_position1811)); ml_reference_projection1811 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                    if !(self::ml_array::includes(&{ let ml_reference_value1812 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]), self::ml_string::Text::from_units(vec![116, 101, 120, 116]), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])]))).borrow().clone(); ml_reference_value1812 }, &match description.clone() {
                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                            self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
                                                                        }
                                                                        self::JavaScriptSemanticValue::Null => {
                                                                            self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                                                                        }
                                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                                            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                                                                        }
                                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                                            self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                                                                        }
                                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                                            self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
                                                                        }
                                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                                            self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                                                                        }
                                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                            self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])
                                                                        }
                                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                                            self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                                                                        }
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                                                                        }
                                                                    }, 0f64, false, |a, b| a == b)) {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field13.clone()), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 111, 112, 116, 105, 111, 110, 97, 108, 32, 116, 101, 120, 116, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field13.clone(), shared_constructor_field14.clone(), {
                                                                            let source_ml_source2_nominal_field30453 = shared_constructor_field2.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field30455 = shared_constructor_field3;
                                                                                {
                                                                                    let source_ml_source2_nominal_field30457 = shared_constructor_field4.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field30459 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field30461 = shared_constructor_field6.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field30463 = shared_constructor_field7.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field30465 = shared_constructor_field8.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field30467 = shared_constructor_field9.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field30469 = shared_constructor_field10.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field30471 = shared_constructor_field11.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override30473 = self::JavaScriptCallPending::WaitSymbolAllocation(destination_10, Box::new(description.clone()), None::<(Option<self::JavaScriptSemanticValue>,)>);
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field30453.clone(), source_ml_source2_nominal_field30455, source_ml_source2_nominal_field30457.clone(), source_ml_source2_nominal_field30459.clone(), source_ml_source2_nominal_field30461.clone(), Box::new(source_ml_source2_nominal_override30473.clone()), source_ml_source2_nominal_field30465.clone(), source_ml_source2_nominal_field30467.clone(), source_ml_source2_nominal_field30469.clone(), source_ml_source2_nominal_field30471.clone())
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
                                                                        })))
                                                                    }
                                                                }
                                                            }
                                                            self::JavaScriptCallInstruction::SymbolDescription(destination_11, input_5) => {
                                                                {
                                                                    let symbol = ({ let ml_reference_array1813 = registers.clone(); let ml_reference_position1813 = input_5; let ml_reference_projection1813 = self::ml_array::optional_at(&*ml_reference_array1813.borrow(), self::ml_array::number_index(ml_reference_position1813)); ml_reference_projection1813 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                    match symbol.clone() {
                                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field13.clone(), shared_constructor_field14.clone(), {
                                                                                let source_ml_source2_nominal_field30480 = shared_constructor_field2.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field30482 = shared_constructor_field3;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field30484 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field30486 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field30488 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field30490 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field30492 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field30494 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field30496 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field30498 = shared_constructor_field11.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override30500 = self::JavaScriptCallPending::WaitSymbolDescription(destination_11, Box::new(symbol.clone()));
                                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field30480.clone(), source_ml_source2_nominal_field30482, source_ml_source2_nominal_field30484.clone(), source_ml_source2_nominal_field30486.clone(), source_ml_source2_nominal_field30488.clone(), Box::new(source_ml_source2_nominal_override30500.clone()), source_ml_source2_nominal_field30492.clone(), source_ml_source2_nominal_field30494.clone(), source_ml_source2_nominal_field30496.clone(), source_ml_source2_nominal_field30498.clone())
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
                                                                            })))
                                                                        }
                                                                        _ => {
                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field13.clone()), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 83, 121, 109, 98, 111, 108, 32, 105, 110, 112, 117, 116]))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::JavaScriptCallInstruction::SymbolRegistry(destination_12, input_6, operation_2) => {
                                                                {
                                                                    let result = self::execute_java_script_symbol_registry_operation(shared_constructor_field13.clone(), &(operation_2), ({ let ml_reference_array1814 = registers.clone(); let ml_reference_position1814 = input_6; let ml_reference_projection1814 = self::ml_array::optional_at(&*ml_reference_array1814.borrow(), self::ml_array::number_index(ml_reference_position1814)); ml_reference_projection1814 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                                    match result.clone() {
                                                                        self::JavaScriptSymbolRegistryTransition::SymbolRegistryValue(heap, value_3) => {
                                                                            let heap = *heap;
                                                                            let value_3 = *value_3;
                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), shared_constructor_field14.clone(), self::write_java_script_call_register(next.clone(), destination_12, value_3.clone()))))
                                                                        }
                                                                        self::JavaScriptSymbolRegistryTransition::SymbolRegistryError(_, error_class, reason) => {
                                                                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                                                                        }
                                                                        self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(_, reason_2) => {
                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field13.clone()), reason_2.clone())
                                                                        }
                                                                        self::JavaScriptSymbolRegistryTransition::SymbolRegistryResourceLimit(_, reason_3) => {
                                                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_3.clone())
                                                                        }
                                                                        self::JavaScriptSymbolRegistryTransition::SymbolRegistryAllocate(heap_2, key_2) => {
                                                                            let heap_2 = *heap_2;
                                                                            let key_2 = *key_2;
                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field14.clone(), {
                                                                                let source_ml_source2_nominal_field30515 = shared_constructor_field2.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field30517 = shared_constructor_field3;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field30519 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field30521 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field30523 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field30525 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field30527 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field30529 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field30531 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field30533 = shared_constructor_field11.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override30535 = {
                                                                                                                            let source_ml_source2_constructor_field30513 = Some(key_2.clone());
                                                                                                                            self::JavaScriptCallPending::WaitSymbolAllocation(destination_12, Box::new(key_2.clone()), Some((source_ml_source2_constructor_field30513.clone(),)))
                                                                                                                        };
                                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field30515.clone(), source_ml_source2_nominal_field30517, source_ml_source2_nominal_field30519.clone(), source_ml_source2_nominal_field30521.clone(), source_ml_source2_nominal_field30523.clone(), Box::new(source_ml_source2_nominal_override30535.clone()), source_ml_source2_nominal_field30527.clone(), source_ml_source2_nominal_field30529.clone(), source_ml_source2_nominal_field30531.clone(), source_ml_source2_nominal_field30533.clone())
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
                                                                            })))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::JavaScriptCallInstruction::FunctionName(destination_13, instruction_object_2, instruction_key, prefix, realm_3) => {
                                                                {
                                                                    let object_3 = ({ let ml_reference_array1815 = registers.clone(); let ml_reference_position1815 = instruction_object_2; let ml_reference_projection1815 = self::ml_array::optional_at(&*ml_reference_array1815.borrow(), self::ml_array::number_index(ml_reference_position1815)); ml_reference_projection1815 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                    {
                                                                        let key_3 = ({ let ml_reference_array1816 = registers.clone(); let ml_reference_position1816 = instruction_key; let ml_reference_projection1816 = self::ml_array::optional_at(&*ml_reference_array1816.borrow(), self::ml_array::number_index(ml_reference_position1816)); ml_reference_projection1816 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                        {
                                                                            let named = self::construct_java_script_property_function_name(key_3.clone(), &(prefix));
                                                                            {
                                                                                let target = match object_3.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                                                                        {
                                                                                            let source_ml_source2_array_read_value30545 = match shared_constructor_field13.clone() {
                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value30542, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value30542.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_array_read_index30546 = self::find_java_script_ordinary_object(match shared_constructor_field13.clone() {
                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value30544, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value30544.clone()
                                                                                                    }
                                                                                                }, identity_2.clone());
                                                                                                { let ml_reference_array1817 = source_ml_source2_array_read_value30545.clone(); let ml_reference_position1817 = source_ml_source2_array_read_index30546; let ml_reference_projection1817 = self::ml_array::optional_at(&*ml_reference_array1817.borrow(), self::ml_array::number_index(ml_reference_position1817)); ml_reference_projection1817 }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    _ => {
                                                                                        None::<self::JavaScriptOrdinaryObject>
                                                                                    }
                                                                                };
                                                                                if ((match object_3.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                } || !(((match object_3.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value30549) => {
                                                                                        Some((source_ml_source2_data_property_value30549,))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(bool,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0)) || {
                                                                                    let source_ml_source2_comparison_operand30551 = {
                                                                                        let source_ml_source2_optional_chain_receiver30550 = (target.clone(),);
                                                                                        if match (source_ml_source2_optional_chain_receiver30550.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                            Some({
                                                                                                let source_ml_source1_temporary_optional_chain1316 = ((source_ml_source2_optional_chain_receiver30550.clone()).0).clone().expect("a proven present value");
                                                                                                match source_ml_source1_temporary_optional_chain1316.clone() {
                                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                                                        self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                                                                                    }
                                                                                                }
                                                                                            })
                                                                                        } else {
                                                                                            None::<self::ml_string::Text>
                                                                                        }
                                                                                    };
                                                                                    !(match (source_ml_source2_comparison_operand30551.clone(), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                }) {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field13.clone()), self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 117, 110, 116, 105, 109, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101]))
                                                                                } else {
                                                                                    match named.clone() {
                                                                                        self::JavaScriptFunctionNameResult::FunctionNameValue(value_4) => {
                                                                                            let value_4 = *value_4;
                                                                                            self::finish_java_script_call_function_name(program.clone(), current.clone(), destination_13, object_3.clone(), realm_3.clone(), value_4.clone())
                                                                                        }
                                                                                        self::JavaScriptFunctionNameResult::FunctionNameInvalid(reason_4) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field13.clone()), reason_4.clone())
                                                                                        }
                                                                                        self::JavaScriptFunctionNameResult::FunctionNameResourceLimit(reason_5) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_5.clone())
                                                                                        }
                                                                                        self::JavaScriptFunctionNameResult::FunctionNameSymbolDescription(_, _) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field13.clone(), shared_constructor_field14.clone(), {
                                                                                                let source_ml_source2_nominal_field30560 = shared_constructor_field2.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field30562 = shared_constructor_field3;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field30564 = shared_constructor_field4.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field30566 = shared_constructor_field5.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field30568 = shared_constructor_field6.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field30570 = shared_constructor_field7.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field30572 = shared_constructor_field8.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field30574 = shared_constructor_field9.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field30576 = shared_constructor_field10.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field30578 = shared_constructor_field11.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override30580 = self::JavaScriptCallPending::WaitFunctionNameSymbol(destination_13, Box::new(object_3.clone()), Box::new(key_3.clone()), prefix.clone(), realm_3.clone());
                                                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field30560.clone(), source_ml_source2_nominal_field30562, source_ml_source2_nominal_field30564.clone(), source_ml_source2_nominal_field30566.clone(), source_ml_source2_nominal_field30568.clone(), Box::new(source_ml_source2_nominal_override30580.clone()), source_ml_source2_nominal_field30572.clone(), source_ml_source2_nominal_field30574.clone(), source_ml_source2_nominal_field30576.clone(), source_ml_source2_nominal_field30578.clone())
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
                                                                                            })))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::JavaScriptCallInstruction::OrdinaryDataProperty(destination_14, object_4, key_4, value_5, realm_4) => {
                                                                self::accept_java_script_call_definition(program.clone(), current.clone(), destination_14, realm_4.clone(), self::complete_java_script_property_definition_with_symbol(shared_constructor_field13.clone(), ({ let ml_reference_array1818 = registers.clone(); let ml_reference_position1818 = object_4; let ml_reference_projection1818 = self::ml_array::optional_at(&*ml_reference_array1818.borrow(), self::ml_array::number_index(ml_reference_position1818)); ml_reference_projection1818 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1819 = registers.clone(); let ml_reference_position1819 = key_4; let ml_reference_projection1819 = self::ml_array::optional_at(&*ml_reference_array1819.borrow(), self::ml_array::number_index(ml_reference_position1819)); ml_reference_projection1819 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), self::create_java_script_ordinary_set_descriptor(({ let ml_reference_array1820 = registers.clone(); let ml_reference_position1820 = value_5; let ml_reference_projection1820 = self::ml_array::optional_at(&*ml_reference_array1820.borrow(), self::ml_array::number_index(ml_reference_position1820)); ml_reference_projection1820 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), true), Some(shared_constructor_field12.clone()), None::<self::ml_string::Text>))
                                                            }
                                                            _ => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field13.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 102, 97, 109, 105, 108, 121]))
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
        },
    }
}
