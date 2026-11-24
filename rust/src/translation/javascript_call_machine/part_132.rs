// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCallSourceUnit;
use super::JavaScriptCallTemplate;
use super::JavaScriptFunctionTextResult;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::MlOptional;
use super::MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Result;
use super::MlSource1TemporaryInitializeJavaScriptSymbolRegistryDirectoryLoop1165Result;
use super::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Result;
use super::find_java_script_call_code_function;
use super::find_java_script_call_template;
use super::find_java_script_heap_directory_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::project_java_script_function_source_text;
use super::read_java_script_function_source_directory_error;
use super::read_java_script_heap_directory_value_error;
use super::source_ml_source1_temporary_find_java_script_symbol_registration_position_loop1168;
use super::source_ml_source1_temporary_find_java_script_symbol_registry_key_loop1167;
use super::source_ml_source1_temporary_initialize_java_script_symbol_registry_directory_loop1165;
use super::source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1155;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_function_text(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    input: self::JavaScriptSemanticValue,
) -> self::JavaScriptFunctionTextResult {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = {
                let source_ml_source2_logical25457 =
                    self::read_java_script_heap_directory_value_error(heap.clone(), input.clone());
                if !source_ml_source2_logical25457.0.is_empty() {
                    source_ml_source2_logical25457.clone()
                } else {
                    self::read_java_script_function_source_directory_error(program.clone())
                }
            };
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptFunctionTextResult::FunctionTextInvalid(error.clone())
            } else {
                if (match input.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                } || !(((match input.clone() {
                    self::JavaScriptSemanticValue::Object(
                        _,
                        source_ml_source2_data_property_value25460,
                    ) => Some((source_ml_source2_data_property_value25460,)),
                    _ => None::<(bool,)>,
                })
                .clone()
                .expect("a proven present value"))
                .0))
                {
                    self::JavaScriptFunctionTextResult::FunctionTextError(
                        self::ml_string::Text::from_units(vec![
                            84, 121, 112, 101, 69, 114, 114, 111, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            70, 117, 110, 99, 116, 105, 111, 110, 46, 112, 114, 111, 116, 111, 116,
                            121, 112, 101, 46, 116, 111, 83, 116, 114, 105, 110, 103, 32, 114, 101,
                            113, 117, 105, 114, 101, 115, 32, 97, 32, 99, 97, 108, 108, 97, 98,
                            108, 101, 32, 114, 101, 99, 101, 105, 118, 101, 114,
                        ]),
                    )
                } else {
                    {
                        let object = {
                            let source_ml_source2_array_read_value25464 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_array_read_index25465 =
                                    self::find_java_script_heap_directory_object(
                                        shared_constructor_field1.clone(),
                                        (((match input.clone() {
                                            self::JavaScriptSemanticValue::Object(
                                                source_ml_source2_data_property_value25463,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value25463
                                                    .clone(),))
                                            }
                                            _ => None::<(self::ml::Big,)>,
                                        })
                                        .clone()
                                        .expect("a proven present value"))
                                        .0)
                                            .clone(),
                                    );
                                {
                                    let ml_reference_array1352 =
                                        source_ml_source2_array_read_value25464.clone();
                                    let ml_reference_position1352 =
                                        source_ml_source2_array_read_index25465;
                                    let ml_reference_projection1352 = self::ml_array::optional_at(
                                        &*ml_reference_array1352.borrow(),
                                        self::ml_array::number_index(ml_reference_position1352),
                                    );
                                    ml_reference_projection1352
                                }
                            }
                        };
                        if {
                            let source_ml_source2_comparison_operand25467 = {
                                let source_ml_source2_optional_chain_receiver25466 =
                                    (object.clone(),);
                                if match (source_ml_source2_optional_chain_receiver25466.clone()).0
                                {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    Some({
                                        let source_ml_source1_temporary_optional_chain1152 =
                                            ((source_ml_source2_optional_chain_receiver25466
                                                .clone())
                                            .0)
                                                .clone()
                                                .expect("a proven present value");
                                        match source_ml_source1_temporary_optional_chain1152.clone() {
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
                            match (
                                source_ml_source2_comparison_operand25467.clone(),
                                self::ml_string::Text::from_units(vec![
                                    98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110,
                                    79, 98, 106, 101, 99, 116,
                                ]),
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            }
                        } {
                            self::JavaScriptFunctionTextResult::FunctionTextNeedAuthority(
                                self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100]),
                            )
                        } else {
                            if (!(match (object.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(
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
                                ) => true,
                                _ => false,
                            }) {
                                {
                                    let template = {
                                        let source_ml_source2_array_read_value25482 = (match program.clone() {
                                                self::JavaScriptCallProgram::CallProgram(_, _, _, source_ml_source2_data_property_value25479, _, _) => {
                                                    ((source_ml_source2_data_property_value25479.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,))).0).clone()
                                                }
                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallTemplate>::new())));
                                        {
                                            let source_ml_source2_array_read_index25483 = self::find_java_script_call_template(program.clone(), (((match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, source_ml_source2_data_property_value25480, _, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value25480.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml::Big,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone(), ((match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, source_ml_source2_data_property_value25481, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value25481,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0);
                                            {
                                                let ml_reference_array1353 =
                                                    source_ml_source2_array_read_value25482.clone();
                                                let ml_reference_position1353 =
                                                    source_ml_source2_array_read_index25483;
                                                let ml_reference_projection1353 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1353.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1353,
                                                        ),
                                                    );
                                                ml_reference_projection1353
                                            }
                                        }
                                    };
                                    {
                                        let source = {
                                            let source_ml_source2_optional_chain_receiver25484 =
                                                (template.clone(),);
                                            match if match (source_ml_source2_optional_chain_receiver25484.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    Some({
                                                        let source_ml_source1_temporary_optional_chain1153 = ((source_ml_source2_optional_chain_receiver25484.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source1_temporary_optional_chain1153.clone() {
                                                            self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25485) => {
                                                                ((source_ml_source2_data_property_value25485.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSourceSpan>,))).0).clone()
                                                            }
                                                        }
                                                    })
                                                } else {
                                                    None::<self::JavaScriptCallSourceSpan>
                                                } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                        };
                                        if match (source.clone(), None::<()>) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        } {
                                            self::JavaScriptFunctionTextResult::FunctionTextNeedAuthority(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101]))
                                        } else {
                                            self::JavaScriptFunctionTextResult::FunctionTextValue(Box::new(self::project_java_script_function_source_text((match program.clone() {
                                                    self::JavaScriptCallProgram::CallProgram(_, _, _, _, _, source_ml_source2_data_property_value25486) => {
                                                        ((source_ml_source2_data_property_value25486.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,))).0).clone()
                                                    }
                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceUnit>::new()))), source.clone().expect("a proven present value"))))
                                        }
                                    }
                                }
                            } else {
                                {
                                    let fn_ = {
                                        let source_ml_source2_array_read_value25489 = match program
                                            .clone()
                                        {
                                            self::JavaScriptCallProgram::CallProgram(
                                                source_ml_source2_data_property_value25487,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source2_data_property_value25487.clone(),
                                        };
                                        {
                                            let source_ml_source2_array_read_index25490 = self::find_java_script_call_code_function(program.clone(), (((match input.clone() {
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value25488, _) => {
                                                        Some((source_ml_source2_data_property_value25488.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml::Big,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone());
                                            {
                                                let ml_reference_array1354 =
                                                    source_ml_source2_array_read_value25489.clone();
                                                let ml_reference_position1354 =
                                                    source_ml_source2_array_read_index25490;
                                                let ml_reference_projection1354 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1354.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1354,
                                                        ),
                                                    );
                                                ml_reference_projection1354
                                            }
                                        }
                                    };
                                    {
                                        let source_2 = {
                                            let source_ml_source2_optional_chain_receiver25491 =
                                                (fn_.clone(),);
                                            match if match (source_ml_source2_optional_chain_receiver25491.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    Some({
                                                        let source_ml_source1_temporary_optional_chain1154 = ((source_ml_source2_optional_chain_receiver25491.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source1_temporary_optional_chain1154.clone() {
                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25492) => {
                                                                ((source_ml_source2_data_property_value25492.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSourceSpan>,))).0).clone()
                                                            }
                                                        }
                                                    })
                                                } else {
                                                    None::<self::JavaScriptCallSourceSpan>
                                                } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                        };
                                        if !(match (source_2.clone(), None::<()>) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        }) {
                                            self::JavaScriptFunctionTextResult::FunctionTextValue(Box::new(self::project_java_script_function_source_text((match program.clone() {
                                                    self::JavaScriptCallProgram::CallProgram(_, _, _, _, _, source_ml_source2_data_property_value25493) => {
                                                        ((source_ml_source2_data_property_value25493.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,))).0).clone()
                                                    }
                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceUnit>::new()))), source_2.clone().expect("a proven present value"))))
                                        } else {
                                            self::JavaScriptFunctionTextResult::FunctionTextNeedAuthority(if (!(match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::ml_array::includes(&{ let ml_reference_value1355 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114])]))).borrow().clone(); ml_reference_value1355 }, &match fn_.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value25494, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value25494.clone()
                                                    }
                                                }, 0f64, false, |a, b| a == b)) {
                                                    self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![101, 120, 116, 101, 114, 110, 97, 108])
                                                })
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
pub fn read_java_script_symbol_registry_model() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32,
        83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 49,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn order_java_script_symbol_registry_entries(
    entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>> {
    {
        let source = entries.clone();
        {
            let width = 1f64;
            {
                let source_ml_source2_s25495 = self::source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1155(entries.clone(), source.clone(), width);
                match source_ml_source2_s25495.clone() {
                    self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Done(source_2) => {
                        source_2.clone()
                    }
                    self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Return(source_ml_source1_temporary_result1155) => {
                        source_ml_source1_temporary_result1155.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn initialize_java_script_symbol_registry_directory(
    heap: self::JavaScriptOrdinaryHeap,
) -> self::JavaScriptOrdinaryHeap {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            if {
                let source_ml_source2_comparison_operand25497 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value25496,
                        _,
                    ) => ((source_ml_source2_data_property_value25496
                        .clone()
                        .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                    .0)
                        .clone(),
                };
                {
                    let source_ml_source2_comparison_operand25498 = None::<()>;
                    !(match (
                        source_ml_source2_comparison_operand25497.clone(),
                        source_ml_source2_comparison_operand25498.clone(),
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } {
                heap.clone()
            } else {
                {
                    let entries = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                        self::JavaScriptSymbolRegistryEntry,
                    >::new(
                    )));
                    {
                        let source_ml_source1_temporary_values1164 = (match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value25499,
                                _,
                                _,
                            ) => ((source_ml_source2_data_property_value25499
                                .clone()
                                .unwrap_or_else(|| {
                                    (None::<
                                        std::rc::Rc<
                                            std::cell::RefCell<
                                                Vec<self::JavaScriptSymbolRegistrationFact>,
                                            >,
                                        >,
                                    >,)
                                }))
                            .0)
                                .clone(),
                        })
                        .clone()
                        .unwrap_or_else(|| {
                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                self::JavaScriptSymbolRegistrationFact,
                            >::new(
                            )))
                        });
                        {
                            let source_ml_source1_temporary_index1164 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s25500 = self::source_ml_source1_temporary_initialize_java_script_symbol_registry_directory_loop1165(entries.clone(), source_ml_source1_temporary_values1164.clone(), source_ml_source1_temporary_index1164.clone());
                                match source_ml_source2_s25500.clone() {
                                    self::MlSource1TemporaryInitializeJavaScriptSymbolRegistryDirectoryLoop1165Result::MlSource1TemporaryInitializeJavaScriptSymbolRegistryDirectoryLoop1165Done => {
                                        {
                                            let source_ml_source2_nominal_field25502 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field25504 = shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_field25506 = shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field25508 = shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field25510 = shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field25512 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_override25514 = self::read_java_script_symbol_registry_model();
                                                                    {
                                                                        let source_ml_source2_nominal_override25515 = self::order_java_script_symbol_registry_entries(entries.clone());
                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_nominal_field25502.clone(), source_ml_source2_nominal_field25504.clone(), source_ml_source2_nominal_field25506.clone(), source_ml_source2_nominal_field25508.clone(), Some((Some(source_ml_source2_nominal_override25514.clone()),)), Some((Some(source_ml_source2_nominal_override25515.clone()),)))
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
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_symbol_registry_key(
    heap: self::JavaScriptOrdinaryHeap,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    {
        let entries = (match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value25516,
            ) => ((source_ml_source2_data_property_value25516
                .clone()
                .unwrap_or_else(|| {
                    (None::<
                        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
                    >,)
                }))
            .0)
                .clone(),
        })
        .clone()
        .unwrap_or_else(|| {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                self::JavaScriptSymbolRegistryEntry,
            >::new()))
        });
        {
            let low = 0f64;
            {
                let high = {
                    let ml_reference_array1356 = entries.clone();
                    let ml_reference_projection1356 = ml_reference_array1356.borrow().len() as f64;
                    ml_reference_projection1356
                };
                {
                    let source_ml_source2_s25517 = self::source_ml_source1_temporary_find_java_script_symbol_registry_key_loop1167(key.clone(), entries.clone(), low, high);
                    match source_ml_source2_s25517.clone() {
                        self::MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Result::MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Done(low_2) => {
                            ((-low_2) - 1f64)
                        }
                        self::MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Result::MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Return(source_ml_source1_temporary_result1167) => {
                            source_ml_source1_temporary_result1167
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_symbol_registration_position(
    facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>,
    identity: self::ml::Big,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array1357 = facts.clone();
                let ml_reference_projection1357 = ml_reference_array1357.borrow().len() as f64;
                ml_reference_projection1357
            };
            {
                let low_2 = self::source_ml_source1_temporary_find_java_script_symbol_registration_position_loop1168(facts.clone(), identity.clone(), low, high);
                low_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn retain_java_script_symbol_registration(
    heap: self::JavaScriptOrdinaryHeap,
    fact: self::JavaScriptSymbolRegistrationFact,
) -> self::JavaScriptOrdinaryHeap {
    match fact.clone() {
        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
            shared_constructor_field7,
            _,
            _,
        ) => {
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    shared_constructor_field2,
                    shared_constructor_field3,
                    shared_constructor_field4,
                    shared_constructor_field5,
                    shared_constructor_field6,
                ) => {
                    let prior_facts = (match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value25518,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value25518
                            .clone()
                            .unwrap_or_else(|| {
                                (None::<
                                    std::rc::Rc<
                                        std::cell::RefCell<
                                            Vec<self::JavaScriptSymbolRegistrationFact>,
                                        >,
                                    >,
                                >,)
                            }))
                        .0)
                            .clone(),
                    })
                    .clone()
                    .unwrap_or_else(|| {
                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                            self::JavaScriptSymbolRegistrationFact,
                        >::new()))
                    });
                    {
                        let position = self::find_java_script_symbol_registration_position(
                            prior_facts.clone(),
                            shared_constructor_field7.clone(),
                        );
                        {
                            let after = if {
                                let source_ml_source2_comparison_operand25525 = {
                                    let source_ml_source2_optional_chain_receiver25522 = ({
                                        let ml_reference_array1358 = prior_facts.clone();
                                        let ml_reference_position1358 = position;
                                        let ml_reference_projection1358 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array1358.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position1358,
                                                ),
                                            );
                                        ml_reference_projection1358
                                    },);
                                    if match (source_ml_source2_optional_chain_receiver25522
                                        .clone())
                                    .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        Some({
                                            let source_ml_source1_temporary_optional_chain1169 =
                                                ((source_ml_source2_optional_chain_receiver25522
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source1_temporary_optional_chain1169.clone() {
                                                    self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source2_data_property_value25523, _, _) => {
                                                        source_ml_source2_data_property_value25523.clone()
                                                    }
                                                }
                                        })
                                    } else {
                                        None::<self::ml::Big>
                                    }
                                };
                                {
                                    let source_ml_source2_comparison_operand25526 =
                                        shared_constructor_field7.clone();
                                    match (
                                        source_ml_source2_comparison_operand25525.clone(),
                                        source_ml_source2_comparison_operand25526.clone(),
                                    ) {
                                        (Some(ml_left), ml_right) => ml_left == ml_right,
                                        _ => false,
                                    }
                                }
                            } {
                                (position + 1f64)
                            } else {
                                position
                            };
                            {
                                let source_ml_source2_nominal_field25531 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_nominal_field25533 =
                                        shared_constructor_field2.clone();
                                    {
                                        let source_ml_source2_nominal_field25535 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_nominal_field25537 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field25539 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field25541 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_override25544 =
                                                            std::rc::Rc::new(
                                                                std::cell::RefCell::new({
                                                                    let source_ml_source2_concat_value25527 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1359 = prior_facts.borrow().clone(); ml_reference_value1359 }, 0f64, position)));
                                                                    {
                                                                        let source_ml_source2_concat_value25528 = std::rc::Rc::new(std::cell::RefCell::new(vec![fact.clone()]));
                                                                        {
                                                                            let source_ml_source2_concat_value25529 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1360 = prior_facts.borrow().clone(); ml_reference_value1360 }, after, f64::INFINITY)));
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSymbolRegistrationFact>::new(), { let ml_reference_value1361 = source_ml_source2_concat_value25527.borrow().clone(); ml_reference_value1361 }), { let ml_reference_value1362 = source_ml_source2_concat_value25528.borrow().clone(); ml_reference_value1362 }), { let ml_reference_value1363 = source_ml_source2_concat_value25529.borrow().clone(); ml_reference_value1363 })
                                                                        }
                                                                    }
                                                                }),
                                                            );
                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_nominal_field25531.clone(), source_ml_source2_nominal_field25533.clone(), Some((Some(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 49])),)), Some((Some(source_ml_source2_nominal_override25544.clone()),)), source_ml_source2_nominal_field25539.clone(), source_ml_source2_nominal_field25541.clone())
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_symbol_registration_fact(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> self::MlOptional<self::JavaScriptSymbolRegistrationFact> {
    {
        let facts = (match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                _,
                _,
                _,
                source_ml_source2_data_property_value25545,
                _,
                _,
            ) => ((source_ml_source2_data_property_value25545
                .clone()
                .unwrap_or_else(|| {
                    (None::<
                        std::rc::Rc<
                            std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
                        >,
                    >,)
                }))
            .0)
                .clone(),
        })
        .clone()
        .unwrap_or_else(|| {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                self::JavaScriptSymbolRegistrationFact,
            >::new()))
        });
        {
            let fact = {
                let source_ml_source2_array_read_index25547 =
                    self::find_java_script_symbol_registration_position(
                        facts.clone(),
                        identity.clone(),
                    );
                {
                    let ml_reference_array1364 = facts.clone();
                    let ml_reference_position1364 = source_ml_source2_array_read_index25547;
                    let ml_reference_projection1364 = self::ml_array::optional_at(
                        &*ml_reference_array1364.borrow(),
                        self::ml_array::number_index(ml_reference_position1364),
                    );
                    ml_reference_projection1364
                }
            };
            if {
                let source_ml_source2_comparison_operand25550 = {
                    let source_ml_source2_optional_chain_receiver25548 = (fact.clone(),);
                    if match (source_ml_source2_optional_chain_receiver25548.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        Some({
                            let source_ml_source1_temporary_optional_chain1170 =
                                ((source_ml_source2_optional_chain_receiver25548.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            match source_ml_source1_temporary_optional_chain1170.clone() {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    source_ml_source2_data_property_value25549,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value25549.clone(),
                            }
                        })
                    } else {
                        None::<self::ml::Big>
                    }
                };
                match (
                    source_ml_source2_comparison_operand25550.clone(),
                    identity.clone(),
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            } {
                if match fact.clone() {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    self::MlOptional::Value(fact.clone().expect("a proven present value"))
                } else {
                    self::MlOptional::Undefined
                }
            } else {
                self::MlOptional::Undefined
            }
        }
    }
}
