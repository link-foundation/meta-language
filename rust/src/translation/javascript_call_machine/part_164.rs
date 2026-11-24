// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_pending_source_intrinsic_error;
use super::read_java_script_ordinary_value_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_pending_intrinsic_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    frame: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
    suspended: bool,
    pending: self::JavaScriptCallPending,
) -> self::ml_string::Text {
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field3,
            _,
            _,
        ) => {
            match frame.clone() {
                self::JavaScriptCallFrame::CallFrame(
                    _,
                    shared_constructor_field1,
                    shared_constructor_field2,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    match pending.clone() {
                        self::JavaScriptCallPending::WaitSymbolAllocation(
                            pending_destination,
                            description,
                            pending_registry_key,
                        ) => {
                            let description = *description;
                            if !(shared_constructor_field1 > (0f64)) {
                                self::ml_string::Text::from_units(vec![
                                    83, 121, 109, 98, 111, 108, 32, 97, 108, 108, 111, 99, 97, 116,
                                    105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                    116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118,
                                    101, 114, 116, 101, 100, 32, 111, 112, 116, 105, 111, 110, 97,
                                    108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110,
                                    32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100,
                                    32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110,
                                ])
                            } else {
                                {
                                    let instruction = {
                                        let source_ml_source2_array_read_value31276 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_array_read_index31277 =
                                                (shared_constructor_field1 - 1f64);
                                            {
                                                let ml_reference_array1895 =
                                                    source_ml_source2_array_read_value31276.clone();
                                                let ml_reference_position1895 =
                                                    source_ml_source2_array_read_index31277;
                                                let ml_reference_projection1895 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1895.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1895,
                                                        ),
                                                    );
                                                ml_reference_projection1895
                                            }
                                        }
                                    };
                                    if (((match (instruction.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } || !(self::ml_array::includes(
                                        &{
                                            let ml_reference_value1896 =
                                                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        117, 110, 100, 101, 102, 105, 110, 101, 100,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        116, 101, 120, 116,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        115, 111, 117, 114, 99, 101, 84, 101, 120,
                                                        116,
                                                    ]),
                                                ])))
                                                .borrow()
                                                .clone();
                                            ml_reference_value1896
                                        },
                                        &match description.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::ml_string::Text::from_units(vec![
                                                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                                                ])
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::ml_string::Text::from_units(vec![
                                                    110, 117, 108, 108,
                                                ])
                                            }
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                self::ml_string::Text::from_units(vec![
                                                    98, 111, 111, 108, 101, 97, 110,
                                                ])
                                            }
                                            self::JavaScriptSemanticValue::Number(_) => {
                                                self::ml_string::Text::from_units(vec![
                                                    110, 117, 109, 98, 101, 114,
                                                ])
                                            }
                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                self::ml_string::Text::from_units(vec![
                                                    105, 110, 116, 101, 103, 101, 114,
                                                ])
                                            }
                                            self::JavaScriptSemanticValue::Text(_) => {
                                                self::ml_string::Text::from_units(vec![
                                                    116, 101, 120, 116,
                                                ])
                                            }
                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                self::ml_string::Text::from_units(vec![
                                                    115, 111, 117, 114, 99, 101, 84, 101, 120, 116,
                                                ])
                                            }
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                self::ml_string::Text::from_units(vec![
                                                    115, 121, 109, 98, 111, 108,
                                                ])
                                            }
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                self::ml_string::Text::from_units(vec![
                                                    111, 98, 106, 101, 99, 116,
                                                ])
                                            }
                                        },
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    ))) || ((self::read_java_script_ordinary_value_error(
                                        heap.clone(),
                                        description.clone(),
                                    )) != (self::ml_string::Text::from_units(
                                        vec![],
                                    )))) || suspended)
                                    {
                                        self::ml_string::Text::from_units(vec![
                                            83, 121, 109, 98, 111, 108, 32, 97, 108, 108, 111, 99,
                                            97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105,
                                            110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                            108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100,
                                            32, 111, 112, 116, 105, 111, 110, 97, 108, 32, 100,
                                            101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 32,
                                            97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101,
                                            100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105,
                                            111, 110,
                                        ])
                                    } else {
                                        match instruction.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::SymbolAllocate(destination, input) => {
                                                if ((!(match (((pending_registry_key.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || (destination != pending_destination)) || !(self::test_java_script_semantic_same_value(({
                                                    let source_ml_source2_array_read_value31279 = shared_constructor_field2.clone();
                                                    { let ml_reference_array1897 = source_ml_source2_array_read_value31279.clone(); let ml_reference_position1897 = input; let ml_reference_projection1897 = self::ml_array::optional_at(&*ml_reference_array1897.borrow(), self::ml_array::number_index(ml_reference_position1897)); ml_reference_projection1897 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), description.clone()))) {
                                                    self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 111, 112, 116, 105, 111, 110, 97, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![])
                                                }
                                            }
                                            self::JavaScriptCallInstruction::SymbolRegistry(destination_2, input_2, operation) => {
                                                {
                                                    let registry_key = ((pending_registry_key.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                                    if ((((operation != (self::ml_string::Text::from_units(vec![102, 111, 114]))) || match (registry_key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::ml_array::includes(&{ let ml_reference_value1898 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![116, 101, 120, 116]), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])]))).borrow().clone(); ml_reference_value1898 }, &match description.clone() {
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
                                                    }, 0f64, false, |a, b| a == b))) || !(self::test_java_script_semantic_same_value(registry_key.clone().expect("a proven present value"), description.clone()))) {
                                                        self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 111, 112, 116, 105, 111, 110, 97, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110])
                                                    } else {
                                                        if ((destination_2 != pending_destination) || !(self::test_java_script_semantic_same_value(({
                                                            let source_ml_source2_array_read_value31282 = shared_constructor_field2.clone();
                                                            { let ml_reference_array1899 = source_ml_source2_array_read_value31282.clone(); let ml_reference_position1899 = input_2; let ml_reference_projection1899 = self::ml_array::optional_at(&*ml_reference_array1899.borrow(), self::ml_array::number_index(ml_reference_position1899)); ml_reference_projection1899 }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), description.clone()))) {
                                                            self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 111, 112, 116, 105, 111, 110, 97, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 111, 112, 116, 105, 111, 110, 97, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110])
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallPending::WaitSymbolDescription(
                            destination_3,
                            symbol,
                        ) => {
                            let symbol = *symbol;
                            if !(shared_constructor_field1 > (0f64)) {
                                self::ml_string::Text::from_units(vec![
                                    83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114, 105,
                                    112, 116, 105, 111, 110, 32, 109, 101, 116, 97, 100, 97, 116,
                                    97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                                    32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116,
                                    101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 105, 110, 112,
                                    117, 116, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117,
                                    99, 116, 105, 111, 110,
                                ])
                            } else {
                                {
                                    let instruction_2 = {
                                        let source_ml_source2_array_read_value31288 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_array_read_index31289 =
                                                (shared_constructor_field1 - 1f64);
                                            {
                                                let ml_reference_array1900 =
                                                    source_ml_source2_array_read_value31288.clone();
                                                let ml_reference_position1900 =
                                                    source_ml_source2_array_read_index31289;
                                                let ml_reference_projection1900 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1900.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1900,
                                                        ),
                                                    );
                                                ml_reference_projection1900
                                            }
                                        }
                                    };
                                    if (match (instruction_2.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } || match instruction_2
                                        .clone()
                                        .expect("a proven present value")
                                    {
                                        self::JavaScriptCallInstruction::SymbolDescription(
                                            _,
                                            _,
                                        ) => false,
                                        _ => true,
                                    }) {
                                        self::ml_string::Text::from_units(vec![
                                            83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114,
                                            105, 112, 116, 105, 111, 110, 32, 109, 101, 116, 97,
                                            100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115,
                                            32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                            115, 101, 108, 101, 99, 116, 101, 100, 32, 83, 121,
                                            109, 98, 111, 108, 32, 105, 110, 112, 117, 116, 32, 97,
                                            110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116,
                                            105, 111, 110,
                                        ])
                                    } else {
                                        if ((((((((match instruction_2.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::SymbolDescription(source_ml_source2_data_property_value31293, _) => {
                                                Some((source_ml_source2_data_property_value31293,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0) != destination_3) || match symbol.clone() {
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) || ((self::read_java_script_ordinary_value_error(heap.clone(), symbol.clone())) != (self::ml_string::Text::from_units(vec![])))) || !(self::test_java_script_semantic_same_value(({
                                            let source_ml_source2_array_read_value31297 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_array_read_index31298 = ((match instruction_2.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::SymbolDescription(_, source_ml_source2_data_property_value31296) => {
                                                        Some((source_ml_source2_data_property_value31296,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0;
                                                { let ml_reference_array1901 = source_ml_source2_array_read_value31297.clone(); let ml_reference_position1901 = source_ml_source2_array_read_index31298; let ml_reference_projection1901 = self::ml_array::optional_at(&*ml_reference_array1901.borrow(), self::ml_array::number_index(ml_reference_position1901)); ml_reference_projection1901 }
                                            }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), symbol.clone()))) || suspended) {
                                            self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 105, 110, 112, 117, 116, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110])
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallPending::WaitFunctionNameSymbol(
                            destination_4,
                            object,
                            key,
                            prefix,
                            realm,
                        ) => {
                            let object = *object;
                            let key = *key;
                            if !(shared_constructor_field1 > (0f64)) {
                                self::ml_string::Text::from_units(vec![
                                    83, 121, 109, 98, 111, 108, 32, 110, 97, 109, 101, 32, 109,
                                    101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105,
                                    110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                    115, 101, 108, 101, 99, 116, 101, 100, 32, 102, 117, 110, 99,
                                    116, 105, 111, 110, 32, 112, 114, 111, 112, 101, 114, 116, 121,
                                    32, 107, 101, 121, 32, 112, 114, 101, 102, 105, 120, 32, 97,
                                    110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111,
                                    110,
                                ])
                            } else {
                                {
                                    let instruction_3 = {
                                        let source_ml_source2_array_read_value31303 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_array_read_index31304 =
                                                (shared_constructor_field1 - 1f64);
                                            {
                                                let ml_reference_array1902 =
                                                    source_ml_source2_array_read_value31303.clone();
                                                let ml_reference_position1902 =
                                                    source_ml_source2_array_read_index31304;
                                                let ml_reference_projection1902 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1902.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1902,
                                                        ),
                                                    );
                                                ml_reference_projection1902
                                            }
                                        }
                                    };
                                    if (match (instruction_3.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } || match instruction_3
                                        .clone()
                                        .expect("a proven present value")
                                    {
                                        self::JavaScriptCallInstruction::FunctionName(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => false,
                                        _ => true,
                                    }) {
                                        self::ml_string::Text::from_units(vec![
                                            83, 121, 109, 98, 111, 108, 32, 110, 97, 109, 101, 32,
                                            109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116,
                                            97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                            117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101,
                                            100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32,
                                            112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101,
                                            121, 32, 112, 114, 101, 102, 105, 120, 32, 97, 110,
                                            100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105,
                                            111, 110,
                                        ])
                                    } else {
                                        if (((((((((((match instruction_3.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::FunctionName(source_ml_source2_data_property_value31311, _, _, _, _) => {
                                                Some((source_ml_source2_data_property_value31311,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0) != destination_4) || (((((match instruction_3.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::FunctionName(_, _, _, _, source_ml_source2_data_property_value31313) => {
                                                Some((source_ml_source2_data_property_value31313.clone(),))
                                            }
                                            _ => {
                                                None::<(self::ml::Big,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone()) != realm)) || (((((match instruction_3.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::FunctionName(_, _, _, source_ml_source2_data_property_value31315, _) => {
                                                Some((source_ml_source2_data_property_value31315.clone(),))
                                            }
                                            _ => {
                                                None::<(self::ml_string::Text,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone()) != prefix)) || match key.clone() {
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) || match object.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) || !(match match object.clone() {
                                            self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value31319) => {
                                                Some(source_ml_source2_data_property_value31319)
                                            }
                                            _ => {
                                                None::<bool>
                                            }
                                        } { Some(ml_present) => ml_present, None => false })) || !(self::test_java_script_semantic_same_value(({
                                            let source_ml_source2_array_read_value31322 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_array_read_index31323 = ((match instruction_3.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::FunctionName(_, source_ml_source2_data_property_value31321, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value31321,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0;
                                                { let ml_reference_array1903 = source_ml_source2_array_read_value31322.clone(); let ml_reference_position1903 = source_ml_source2_array_read_index31323; let ml_reference_projection1903 = self::ml_array::optional_at(&*ml_reference_array1903.borrow(), self::ml_array::number_index(ml_reference_position1903)); ml_reference_projection1903 }
                                            }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), object.clone()))) || !(self::test_java_script_semantic_same_value(({
                                            let source_ml_source2_array_read_value31326 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_array_read_index31327 = ((match instruction_3.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::FunctionName(_, _, source_ml_source2_data_property_value31325, _, _) => {
                                                        Some((source_ml_source2_data_property_value31325,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0;
                                                { let ml_reference_array1904 = source_ml_source2_array_read_value31326.clone(); let ml_reference_position1904 = source_ml_source2_array_read_index31327; let ml_reference_projection1904 = self::ml_array::optional_at(&*ml_reference_array1904.borrow(), self::ml_array::number_index(ml_reference_position1904)); ml_reference_projection1904 }
                                            }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), key.clone()))) {
                                            self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 110, 97, 109, 101, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 112, 114, 101, 102, 105, 120, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110])
                                        } else {
                                            {
                                                let invalid = {
                                                    let source_ml_source2_logical31328 = self::read_java_script_ordinary_value_error(heap.clone(), key.clone());
                                                    if !source_ml_source2_logical31328.0.is_empty() {
                                                        source_ml_source2_logical31328.clone()
                                                    } else {
                                                        self::read_java_script_ordinary_value_error(heap.clone(), object.clone())
                                                    }
                                                };
                                                if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                    invalid.clone()
                                                } else {
                                                    self::ml_string::Text::from_units(vec![])
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallPending::WaitError(realm_2, _, _) => {
                            if !(self::ml_array::includes(
                                &{
                                    let ml_reference_value1905 = (match program.clone() {
                                        self::JavaScriptCallProgram::CallProgram(
                                            _,
                                            source_ml_source2_data_property_value31330,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value31330.clone(),
                                    })
                                    .borrow()
                                    .clone();
                                    ml_reference_value1905
                                },
                                &realm_2.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            )) {
                                self::ml_string::Text::from_units(vec![
                                    112, 101, 110, 100, 105, 110, 103, 32, 69, 114, 114, 111, 114,
                                    32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114,
                                    101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99,
                                    116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105,
                                    99, 32, 114, 101, 97, 108, 109,
                                ])
                            } else {
                                self::ml_string::Text::from_units(vec![])
                            }
                        }
                        _ => self::read_java_script_call_pending_source_intrinsic_error(
                            program.clone(),
                            heap.clone(),
                            frame.clone(),
                            fn_.clone(),
                            suspended,
                            pending.clone(),
                        ),
                    }
                }
            }
        }
    }
}
