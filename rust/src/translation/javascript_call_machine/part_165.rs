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
use super::read_java_script_error_construction_operands_error;
use super::read_java_script_error_prior_directory_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_source_exception_class;
use super::test_java_script_error_intrinsic_mode;
use super::test_java_script_host_capability_mode;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_pending_source_intrinsic_error(
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
            shared_constructor_field3,
            shared_constructor_field4,
            _,
            _,
            _,
            _,
            shared_constructor_field5,
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
                        self::JavaScriptCallPending::WaitSourceError(
                            realm,
                            error_class,
                            pending_message,
                            message_present,
                        ) => {
                            let pending_message = *pending_message;
                            if (!(self::ml_array::includes(
                                &{
                                    let ml_reference_value1906 = (match program.clone() {
                                        self::JavaScriptCallProgram::CallProgram(
                                            _,
                                            source_ml_source2_data_property_value31332,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value31332.clone(),
                                    })
                                    .borrow()
                                    .clone();
                                    ml_reference_value1906
                                },
                                &realm.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            )) || (shared_constructor_field3 != realm))
                            {
                                self::ml_string::Text::from_units(vec![
                                    112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99,
                                    101, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97,
                                    116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111,
                                    117, 114, 99, 101, 32, 114, 101, 97, 108, 109,
                                ])
                            } else {
                                if ((match pending_message.clone() {
                                    self::JavaScriptSemanticValue::Text(_) => false,
                                    _ => true,
                                } && match pending_message.clone() {
                                    self::JavaScriptSemanticValue::SourceText(_, _) => false,
                                    _ => true,
                                }) || ((self::read_java_script_ordinary_value_error(
                                    heap.clone(),
                                    pending_message.clone(),
                                )) != (self::ml_string::Text::from_units(vec![]))))
                                {
                                    self::ml_string::Text::from_units(vec![
                                        112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114,
                                        99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97,
                                        105, 110, 115, 32, 101, 120, 97, 99, 116, 32, 85, 84, 70,
                                        49, 54, 32, 109, 101, 115, 115, 97, 103, 101, 32, 97, 110,
                                        100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 112, 114,
                                        101, 115, 101, 110, 99, 101,
                                    ])
                                } else {
                                    if !(shared_constructor_field1 > (0f64)) {
                                        self::ml_string::Text::from_units(vec![
                                            112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117,
                                            114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101,
                                            116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99,
                                            116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32,
                                            105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110,
                                            32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110,
                                            116, 115,
                                        ])
                                    } else {
                                        {
                                            let instruction = {
                                                let source_ml_source2_array_read_value31342 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_array_read_index31343 =
                                                        (shared_constructor_field1 - 1f64);
                                                    {
                                                        let ml_reference_array1907 =
                                                            source_ml_source2_array_read_value31342
                                                                .clone();
                                                        let ml_reference_position1907 =
                                                            source_ml_source2_array_read_index31343;
                                                        let ml_reference_projection1907 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array1907.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position1907,
                                                                ),
                                                            );
                                                        ml_reference_projection1907
                                                    }
                                                }
                                            };
                                            if match (instruction.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::ml_string::Text::from_units(vec![
                                                    112, 101, 110, 100, 105, 110, 103, 32, 115,
                                                    111, 117, 114, 99, 101, 32, 69, 114, 114, 111,
                                                    114, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                                                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                                    32, 115, 111, 117, 114, 99, 101, 32, 105, 110,
                                                    115, 116, 114, 117, 99, 116, 105, 111, 110, 32,
                                                    97, 110, 100, 32, 97, 114, 103, 117, 109, 101,
                                                    110, 116, 115,
                                                ])
                                            } else {
                                                match instruction.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::Primitive(_, operation, arguments) => {
                                                        if ((((self::read_java_script_source_exception_class(&(operation))) != error_class) || (error_class == (self::ml_string::Text::from_units(vec![])))) || (({ let ml_reference_array1908 = arguments.clone(); let ml_reference_projection1908 = ml_reference_array1908.borrow().len() as f64; ml_reference_projection1908 }) != (2f64))) {
                                                            self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115])
                                                        } else {
                                                            {
                                                                let message = ({
                                                                    let source_ml_source2_array_read_value31348 = shared_constructor_field2.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index31349 = ({
                                                                            let source_ml_source2_array_read_index31347 = 0f64;
                                                                            { let ml_reference_array1909 = arguments.clone(); let ml_reference_position1909 = source_ml_source2_array_read_index31347; let ml_reference_projection1909 = self::ml_array::optional_at(&*ml_reference_array1909.borrow(), self::ml_array::number_index(ml_reference_position1909)); ml_reference_projection1909 }
                                                                        }).clone().unwrap_or_else(|| (-1f64));
                                                                        { let ml_reference_array1910 = source_ml_source2_array_read_value31348.clone(); let ml_reference_position1910 = source_ml_source2_array_read_index31349; let ml_reference_projection1910 = self::ml_array::optional_at(&*ml_reference_array1910.borrow(), self::ml_array::number_index(ml_reference_position1910)); ml_reference_projection1910 }
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                {
                                                                    let present = ({
                                                                        let source_ml_source2_array_read_value31353 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index31354 = ({
                                                                                let source_ml_source2_array_read_index31352 = 1f64;
                                                                                { let ml_reference_array1911 = arguments.clone(); let ml_reference_position1911 = source_ml_source2_array_read_index31352; let ml_reference_projection1911 = self::ml_array::optional_at(&*ml_reference_array1911.borrow(), self::ml_array::number_index(ml_reference_position1911)); ml_reference_projection1911 }
                                                                            }).clone().unwrap_or_else(|| (-1f64));
                                                                            { let ml_reference_array1912 = source_ml_source2_array_read_value31353.clone(); let ml_reference_position1912 = source_ml_source2_array_read_index31354; let ml_reference_projection1912 = self::ml_array::optional_at(&*ml_reference_array1912.borrow(), self::ml_array::number_index(ml_reference_position1912)); ml_reference_projection1912 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                    if !(self::test_java_script_semantic_same_value(message.clone(), pending_message.clone())) {
                                                                        self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115])
                                                                    } else {
                                                                        match present.clone() {
                                                                            self::JavaScriptSemanticValue::Boolean(value) => {
                                                                                if (value != message_present) {
                                                                                    self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115])
                                                                                } else {
                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                }
                                                                            }
                                                                            _ => {
                                                                                self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115])
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115])
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallPending::WaitErrorValue(
                            destination,
                            realm_2,
                            error_class_2,
                            pending_constructor,
                            pending_new_target,
                            pending_prototype,
                            pending_message_2,
                            message_present_2,
                            bind_receiver,
                            before_directory,
                        ) => {
                            let pending_constructor = *pending_constructor;
                            let pending_new_target = *pending_new_target;
                            let pending_prototype = *pending_prototype;
                            let pending_message_2 = *pending_message_2;
                            let before_directory = *before_directory;
                            if !(shared_constructor_field1 > (0f64)) {
                                self::ml_string::Text::from_units(vec![
                                    69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32, 114,
                                    101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116,
                                    97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                                    32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110,
                                ])
                            } else {
                                {
                                    let instruction_2 = {
                                        let source_ml_source2_array_read_value31359 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_array_read_index31360 =
                                                (shared_constructor_field1 - 1f64);
                                            {
                                                let ml_reference_array1913 =
                                                    source_ml_source2_array_read_value31359.clone();
                                                let ml_reference_position1913 =
                                                    source_ml_source2_array_read_index31360;
                                                let ml_reference_projection1913 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1913.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1913,
                                                        ),
                                                    );
                                                ml_reference_projection1913
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
                                        self::JavaScriptCallInstruction::ConstructErrorValue(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => false,
                                        _ => true,
                                    }) {
                                        self::ml_string::Text::from_units(vec![
                                            69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32,
                                            114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32,
                                            114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                                            32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115,
                                            116, 114, 117, 99, 116, 111, 114, 32, 105, 110, 115,
                                            116, 114, 117, 99, 116, 105, 111, 110,
                                        ])
                                    } else {
                                        if (((((((match instruction_2.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::ConstructErrorValue(source_ml_source2_data_property_value31371, _, _, _, _, _, _, _, _) => {
                                                Some((source_ml_source2_data_property_value31371,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0) != destination) || (((((match instruction_2.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, source_ml_source2_data_property_value31373, _, _) => {
                                                Some((source_ml_source2_data_property_value31373.clone(),))
                                            }
                                            _ => {
                                                None::<(self::ml::Big,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone()) != realm_2)) || (((((match instruction_2.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, _, source_ml_source2_data_property_value31375, _) => {
                                                Some((source_ml_source2_data_property_value31375.clone(),))
                                            }
                                            _ => {
                                                None::<(self::ml_string::Text,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone()) != error_class_2)) || ((((match instruction_2.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value31377) => {
                                                Some((source_ml_source2_data_property_value31377,))
                                            }
                                            _ => {
                                                None::<(bool,)>
                                            }
                                        }).clone().expect("a proven present value")).0) != bind_receiver)) {
                                            self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 44, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 111, 108, 101])
                                        } else {
                                            {
                                                let constructor = ({
                                                    let source_ml_source2_array_read_value31380 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source2_array_read_index31381 = ((match instruction_2.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ConstructErrorValue(_, source_ml_source2_data_property_value31379, _, _, _, _, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value31379,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0;
                                                        { let ml_reference_array1914 = source_ml_source2_array_read_value31380.clone(); let ml_reference_position1914 = source_ml_source2_array_read_index31381; let ml_reference_projection1914 = self::ml_array::optional_at(&*ml_reference_array1914.borrow(), self::ml_array::number_index(ml_reference_position1914)); ml_reference_projection1914 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                {
                                                    let new_target = ({
                                                        let source_ml_source2_array_read_value31384 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31385 = ((match instruction_2.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::ConstructErrorValue(_, _, source_ml_source2_data_property_value31383, _, _, _, _, _, _) => {
                                                                    Some((source_ml_source2_data_property_value31383,))
                                                                }
                                                                _ => {
                                                                    None::<(f64,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0;
                                                            { let ml_reference_array1915 = source_ml_source2_array_read_value31384.clone(); let ml_reference_position1915 = source_ml_source2_array_read_index31385; let ml_reference_projection1915 = self::ml_array::optional_at(&*ml_reference_array1915.borrow(), self::ml_array::number_index(ml_reference_position1915)); ml_reference_projection1915 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    {
                                                        let prototype = ({
                                                            let source_ml_source2_array_read_value31388 = shared_constructor_field2.clone();
                                                            {
                                                                let source_ml_source2_array_read_index31389 = ((match instruction_2.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, source_ml_source2_data_property_value31387, _, _, _, _, _) => {
                                                                        Some((source_ml_source2_data_property_value31387,))
                                                                    }
                                                                    _ => {
                                                                        None::<(f64,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0;
                                                                { let ml_reference_array1916 = source_ml_source2_array_read_value31388.clone(); let ml_reference_position1916 = source_ml_source2_array_read_index31389; let ml_reference_projection1916 = self::ml_array::optional_at(&*ml_reference_array1916.borrow(), self::ml_array::number_index(ml_reference_position1916)); ml_reference_projection1916 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                        {
                                                            let message_2 = ({
                                                                let source_ml_source2_array_read_value31392 = shared_constructor_field2.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index31393 = ((match instruction_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, source_ml_source2_data_property_value31391, _, _, _, _) => {
                                                                            Some((source_ml_source2_data_property_value31391,))
                                                                        }
                                                                        _ => {
                                                                            None::<(f64,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0;
                                                                    { let ml_reference_array1917 = source_ml_source2_array_read_value31392.clone(); let ml_reference_position1917 = source_ml_source2_array_read_index31393; let ml_reference_projection1917 = self::ml_array::optional_at(&*ml_reference_array1917.borrow(), self::ml_array::number_index(ml_reference_position1917)); ml_reference_projection1917 }
                                                                }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                            {
                                                                let present_2 = ({
                                                                    let source_ml_source2_array_read_value31396 = shared_constructor_field2.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index31397 = ((match instruction_2.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, source_ml_source2_data_property_value31395, _, _, _) => {
                                                                                Some((source_ml_source2_data_property_value31395,))
                                                                            }
                                                                            _ => {
                                                                                None::<(f64,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0;
                                                                        { let ml_reference_array1918 = source_ml_source2_array_read_value31396.clone(); let ml_reference_position1918 = source_ml_source2_array_read_index31397; let ml_reference_projection1918 = self::ml_array::optional_at(&*ml_reference_array1918.borrow(), self::ml_array::number_index(ml_reference_position1918)); ml_reference_projection1918 }
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                {
                                                                    let error = self::read_java_script_error_construction_operands_error(heap.clone(), constructor.clone(), new_target.clone(), prototype.clone(), message_2.clone(), present_2.clone());
                                                                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                        error.clone()
                                                                    } else {
                                                                        if (((!(self::test_java_script_semantic_same_value(constructor.clone(), pending_constructor.clone())) || !(self::test_java_script_semantic_same_value(new_target.clone(), pending_new_target.clone()))) || !(self::test_java_script_semantic_same_value(prototype.clone(), pending_prototype.clone()))) || !(self::test_java_script_semantic_same_value(message_2.clone(), pending_message_2.clone()))) {
                                                                            self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115])
                                                                        } else {
                                                                            if (match present_2.clone() {
                                                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } || ((((match present_2.clone() {
                                                                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_data_property_value31400) => {
                                                                                    Some((source_ml_source2_data_property_value31400,))
                                                                                }
                                                                                _ => {
                                                                                    None::<(bool,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0) != message_present_2)) {
                                                                                self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101])
                                                                            } else {
                                                                                {
                                                                                    let prior = self::read_java_script_error_prior_directory_error(heap.clone(), before_directory.clone());
                                                                                    if (prior != (self::ml_string::Text::from_units(vec![]))) {
                                                                                        prior.clone()
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
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallPending::WaitErrorIntrinsic(destination_2) => {
                            if !(shared_constructor_field1 > (0f64)) {
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 69,
                                    114, 114, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32,
                                    114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                                    99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101,
                                    114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110,
                                    115, 116, 114, 117, 99, 116, 105, 111, 110,
                                ])
                            } else {
                                {
                                    let instruction_3 = {
                                        let source_ml_source2_array_read_value31405 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_array_read_index31406 =
                                                (shared_constructor_field1 - 1f64);
                                            {
                                                let ml_reference_array1919 =
                                                    source_ml_source2_array_read_value31405.clone();
                                                let ml_reference_position1919 =
                                                    source_ml_source2_array_read_index31406;
                                                let ml_reference_projection1919 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1919.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1919,
                                                        ),
                                                    );
                                                ml_reference_projection1919
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
                                        self::JavaScriptCallInstruction::InvokeErrorIntrinsic(
                                            _,
                                        ) => false,
                                        _ => true,
                                    }) {
                                        self::ml_string::Text::from_units(vec![
                                            97, 110, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32,
                                            69, 114, 114, 111, 114, 32, 99, 97, 108, 108, 98, 97,
                                            99, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101,
                                            103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111,
                                            117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117,
                                            99, 116, 105, 111, 110,
                                        ])
                                    } else {
                                        if (!(self::test_java_script_error_intrinsic_mode(&(shared_constructor_field4))) || ((((match instruction_3.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::InvokeErrorIntrinsic(source_ml_source2_data_property_value31410) => {
                                                Some((source_ml_source2_data_property_value31410,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0) != destination_2)) {
                                            self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 69, 114, 114, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110])
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallPending::WaitHostCapability(
                            destination_3,
                            before_directory_2,
                        ) => {
                            let before_directory_2 = *before_directory_2;
                            if !(shared_constructor_field1 > (0f64)) {
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 99, 97,
                                    112, 97, 98, 105, 108, 105, 116, 121, 32, 114, 101, 116, 97,
                                    105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 115,
                                    111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99,
                                    116, 105, 111, 110,
                                ])
                            } else {
                                {
                                    let instruction_4 = {
                                        let source_ml_source2_array_read_value31415 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_array_read_index31416 =
                                                (shared_constructor_field1 - 1f64);
                                            {
                                                let ml_reference_array1920 =
                                                    source_ml_source2_array_read_value31415.clone();
                                                let ml_reference_position1920 =
                                                    source_ml_source2_array_read_index31416;
                                                let ml_reference_projection1920 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1920.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1920,
                                                        ),
                                                    );
                                                ml_reference_projection1920
                                            }
                                        }
                                    };
                                    if (match (instruction_4.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } || match instruction_4
                                        .clone()
                                        .expect("a proven present value")
                                    {
                                        self::JavaScriptCallInstruction::InvokeHostCapability(
                                            _,
                                        ) => false,
                                        _ => true,
                                    }) {
                                        self::ml_string::Text::from_units(vec![
                                            97, 110, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32,
                                            99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 32, 114,
                                            101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                                            99, 116, 117, 97, 108, 32, 113, 117, 97, 108, 105, 102,
                                            105, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32,
                                            105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110,
                                        ])
                                    } else {
                                        if (!(self::test_java_script_host_capability_mode(&(shared_constructor_field4))) || ((((match instruction_4.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::InvokeHostCapability(source_ml_source2_data_property_value31420) => {
                                                Some((source_ml_source2_data_property_value31420,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0) != destination_3)) {
                                            self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110])
                                        } else {
                                            {
                                                let prior_2 = self::read_java_script_error_prior_directory_error(heap.clone(), before_directory_2.clone());
                                                if (prior_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                    prior_2.clone()
                                                } else {
                                                    self::ml_string::Text::from_units(vec![])
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
            }
        }
    }
}
