// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::JavaScriptSymbolRegistryTransition;
use super::MlOptional;
use super::MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Result;
use super::MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177;
use super::find_java_script_symbol_registry_key;
use super::initialize_java_script_symbol_registry_directory;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::read_java_script_symbol_registration_fact;
use super::read_java_script_symbol_registry_model;
use super::register_java_script_ordinary_symbol;
use super::retain_java_script_symbol_registration;
use super::source_ml_source1_temporary_execute_java_script_symbol_registry_operation_loop1173;
use super::test_java_script_semantic_same_value;
use super::test_java_script_standard_collection_for_each_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_symbol_registry_operation(
    heap: self::JavaScriptOrdinaryHeap,
    operation: &self::ml_string::Text,
    input: self::JavaScriptSemanticValue,
) -> self::JavaScriptSymbolRegistryTransition {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            {
                let invalid =
                    self::read_java_script_ordinary_value_error(heap.clone(), input.clone());
                if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(
                        Box::new(heap.clone()),
                        invalid.clone(),
                    )
                } else {
                    if ((*operation)
                        == (self::ml_string::Text::from_units(vec![107, 101, 121, 70, 111, 114])))
                    {
                        match input.clone() {
                            self::JavaScriptSemanticValue::Symbol(identity) => {
                                let fact = self::read_java_script_symbol_registration_fact(
                                    heap.clone(),
                                    identity.clone(),
                                );
                                if match (fact.clone(), None::<()>) {
                                    (self::MlOptional::Value(_ml_left), Some(_ml_right)) => false,
                                    (self::MlOptional::Null, None) => false,
                                    (self::MlOptional::Undefined, None) => true,
                                    _ => false,
                                } {
                                    self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            83, 121, 109, 98, 111, 108, 46, 107, 101, 121, 70, 111,
                                            114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                                            97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115,
                                            116, 114, 97, 116, 105, 111, 110, 32, 97, 117, 116,
                                            104, 111, 114, 105, 116, 121, 32, 102, 111, 114, 32,
                                            116, 104, 105, 115, 32, 83, 121, 109, 98, 111, 108,
                                        ]),
                                    )
                                } else {
                                    if !({
                                        let source_ml_source2_nullish_field_receiver25552 =
                                            (fact.clone(),);
                                        if match (source_ml_source2_nullish_field_receiver25552
                                            .clone())
                                        .0
                                        {
                                            self::MlOptional::Value(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            self::MlOptional::Null => false,
                                            self::MlOptional::Undefined => false,
                                        } {
                                            {
                                                let source_ml_source2_nullish_field_present25553 = match (source_ml_source2_nullish_field_receiver25552.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                                match source_ml_source2_nullish_field_present25553.clone() {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source2_data_property_value25554, _) => {
                                                            source_ml_source2_data_property_value25554
                                                        }
                                                    }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (if ((match (source_ml_source2_nullish_field_receiver25552.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]) }, self::MlOptional::Null => self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]), self::MlOptional::Undefined => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) {
                                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 39, 41])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 39, 41])
                                                }).0, message_present: true }).raise()
                                        }
                                    }) {
                                        self::JavaScriptSymbolRegistryTransition::SymbolRegistryValue(Box::new(heap.clone()), Box::new(self::JavaScriptSemanticValue::Undefined))
                                    } else {
                                        {
                                            let key = {
                                                let source_ml_source2_nullish_field_receiver25555 =
                                                    (fact.clone(),);
                                                if match (source_ml_source2_nullish_field_receiver25555.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                                                        {
                                                            let source_ml_source2_nullish_field_present25556 = match (source_ml_source2_nullish_field_receiver25555.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                                            match source_ml_source2_nullish_field_present25556.clone() {
                                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source2_data_property_value25557) => {
                                                                    ((source_ml_source2_data_property_value25557.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (if ((match (source_ml_source2_nullish_field_receiver25555.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]) }, self::MlOptional::Null => self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]), self::MlOptional::Undefined => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) {
                                                            self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])
                                                        }).0, message_present: true }).raise()
                                                    }
                                            };
                                            if match (key.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 70, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115]))
                                            } else {
                                                self::JavaScriptSymbolRegistryTransition::SymbolRegistryValue(Box::new(heap.clone()), Box::new(key.clone().expect("a proven present value")))
                                            }
                                        }
                                    }
                                }
                            }
                            _ => self::JavaScriptSymbolRegistryTransition::SymbolRegistryError(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    83, 121, 109, 98, 111, 108, 46, 107, 101, 121, 70, 111, 114,
                                    32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32,
                                    97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116,
                                    105, 118, 101, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105,
                                    116, 104, 111, 117, 116, 32, 99, 111, 101, 114, 99, 105, 111,
                                    110,
                                ]),
                            ),
                        }
                    } else {
                        if (((*operation)
                            != (self::ml_string::Text::from_units(vec![102, 111, 114])))
                            || (match input.clone() {
                                self::JavaScriptSemanticValue::Text(_) => false,
                                _ => true,
                            } && match input.clone() {
                                self::JavaScriptSemanticValue::SourceText(_, _) => false,
                                _ => true,
                            }))
                        {
                            self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    83, 121, 109, 98, 111, 108, 46, 102, 111, 114, 32, 114, 101,
                                    116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                    117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100,
                                    32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 116, 101, 120,
                                    116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                                ]),
                            )
                        } else {
                            if ((self::read_java_script_semantic_text_length(input.clone()))
                                > (1048576f64))
                            {
                                self::JavaScriptSymbolRegistryTransition::SymbolRegistryResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 112, 114, 111, 102, 105, 108, 101]))
                            } else {
                                {
                                    let heap_2 =
                                        self::initialize_java_script_symbol_registry_directory(
                                            heap.clone(),
                                        );
                                    match heap_2.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            shared_constructor_field1,
                                            shared_constructor_field2,
                                            shared_constructor_field3,
                                            shared_constructor_field4,
                                            shared_constructor_field5,
                                            shared_constructor_field6,
                                        ) => {
                                            let at = self::find_java_script_symbol_registry_key(
                                                heap_2.clone(),
                                                input.clone(),
                                            );
                                            {
                                                let entry = {
                                                    let source_ml_source2_optional_chain_receiver25562 = (match heap_2.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source2_data_property_value25561) => {
                                                                ((source_ml_source2_data_property_value25561.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                            }
                                                        },);
                                                    match if match (source_ml_source2_optional_chain_receiver25562.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                            Some({
                                                                let source_ml_source1_temporary_optional_chain1171 = ((source_ml_source2_optional_chain_receiver25562.clone()).0).clone().expect("a proven present value");
                                                                { let ml_reference_array1365 = source_ml_source1_temporary_optional_chain1171.clone(); let ml_reference_position1365 = at; let ml_reference_projection1365 = self::ml_array::optional_at(&*ml_reference_array1365.borrow(), self::ml_array::number_index(ml_reference_position1365)); ml_reference_projection1365 }
                                                            })
                                                        } else {
                                                            None::<self::JavaScriptSymbolRegistryEntry>
                                                        } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                };
                                                if !(match (entry.clone(), None::<()>) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                }) {
                                                    self::JavaScriptSymbolRegistryTransition::SymbolRegistryValue(Box::new(heap_2.clone()), Box::new(self::JavaScriptSemanticValue::Symbol(match entry.clone().expect("a proven present value") {
                                                            self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source2_data_property_value25565, _) => {
                                                                source_ml_source2_data_property_value25565.clone()
                                                            }
                                                        })))
                                                } else {
                                                    {
                                                        let source_ml_source1_temporary_values1172 =
                                                            shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source1_temporary_index1172 =
                                                                self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s25567 = self::source_ml_source1_temporary_execute_java_script_symbol_registry_operation_loop1173(heap_2.clone(), source_ml_source1_temporary_values1172.clone(), source_ml_source1_temporary_index1172.clone());
                                                                match source_ml_source2_s25567.clone() {
                                                                        self::MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Result::MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Done => {
                                                                            if ({
                                                                                let source_ml_source2_comparison_operand25569 = match heap_2.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source2_data_property_value25568, _) => {
                                                                                        ((source_ml_source2_data_property_value25568.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_comparison_operand25570 = None::<()>;
                                                                                    !(match (source_ml_source2_comparison_operand25569.clone(), source_ml_source2_comparison_operand25570.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                }
                                                                            } && (((match heap_2.clone() {
                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source2_data_property_value25572, _) => {
                                                                                    ((source_ml_source2_data_property_value25572.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value")) != (self::read_java_script_symbol_registry_model()))) {
                                                                                self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(Box::new(heap_2.clone()), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 97, 103, 101, 110, 116, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 109, 111, 100, 101, 108]))
                                                                            } else {
                                                                                if (({ let ml_reference_array1366 = shared_constructor_field2.clone(); let ml_reference_projection1366 = ml_reference_array1366.borrow().len() as f64; ml_reference_projection1366 }) >= (16384f64)) {
                                                                                    self::JavaScriptSymbolRegistryTransition::SymbolRegistryResourceLimit(Box::new(heap_2.clone()), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 49, 54, 51, 56, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                                                } else {
                                                                                    self::JavaScriptSymbolRegistryTransition::SymbolRegistryAllocate(Box::new(if {
                                                                                        let source_ml_source2_comparison_operand25576 = match heap_2.clone() {
                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source2_data_property_value25575, _) => {
                                                                                                ((source_ml_source2_data_property_value25575.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_comparison_operand25577 = None::<()>;
                                                                                            match (source_ml_source2_comparison_operand25576.clone(), source_ml_source2_comparison_operand25577.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                        }
                                                                                    } {
                                                                                        {
                                                                                            let source_ml_source2_nominal_field25579 = shared_constructor_field1.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field25581 = shared_constructor_field2.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field25583 = shared_constructor_field3.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field25585 = shared_constructor_field4.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field25587 = shared_constructor_field5.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field25589 = shared_constructor_field6.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override25591 = self::read_java_script_symbol_registry_model();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override25592 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistryEntry>::new()));
                                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_nominal_field25579.clone(), source_ml_source2_nominal_field25581.clone(), source_ml_source2_nominal_field25583.clone(), source_ml_source2_nominal_field25585.clone(), Some((Some(source_ml_source2_nominal_override25591.clone()),)), Some((Some(source_ml_source2_nominal_override25592.clone()),)))
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        heap_2.clone()
                                                                                    }), Box::new(input.clone()))
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Result::MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Return(source_ml_source1_temporary_result1173) => {
                                                                            let source_ml_source1_temporary_result1173 = *source_ml_source1_temporary_result1173;
                                                                            source_ml_source1_temporary_result1173.clone()
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
pub fn import_java_script_symbol_registry_witness(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
) -> self::JavaScriptSymbolRegistryTransition {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            if ((match key.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                _ => true,
            } && match key.clone() {
                self::JavaScriptSemanticValue::Text(_) => false,
                _ => true,
            }) && match key.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => false,
                _ => true,
            }) {
                self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        97, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114,
                        97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115, 32, 114,
                        101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                        108, 32, 111, 112, 116, 105, 111, 110, 97, 108, 32, 112, 114, 105, 109,
                        105, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 107, 101, 121,
                    ]),
                )
            } else {
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), key.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(
                            Box::new(heap.clone()),
                            invalid.clone(),
                        )
                    } else {
                        {
                            let heap_2 = self::initialize_java_script_symbol_registry_directory(
                                heap.clone(),
                            );
                            {
                                let registered = match key.clone() {
                                    self::JavaScriptSemanticValue::Undefined => false,
                                    _ => true,
                                };
                                {
                                    let prior = self::read_java_script_symbol_registration_fact(
                                        heap_2.clone(),
                                        identity.clone(),
                                    );
                                    {
                                        let prior_key = {
                                            let source_ml_source2_optional_chain_receiver25596 =
                                                (prior.clone(),);
                                            match if match (source_ml_source2_optional_chain_receiver25596.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                                                Some({
                                                    let source_ml_source1_temporary_optional_chain1174 = match (source_ml_source2_optional_chain_receiver25596.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                                    match source_ml_source1_temporary_optional_chain1174.clone() {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source2_data_property_value25597) => {
                                                            ((source_ml_source2_data_property_value25597.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    }
                                                })
                                            } else {
                                                None::<self::JavaScriptSemanticValue>
                                            } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                        };
                                        if (!(match (prior.clone(), None::<()>) {
                                            (
                                                self::MlOptional::Value(_ml_left),
                                                Some(_ml_right),
                                            ) => false,
                                            (self::MlOptional::Null, None) => false,
                                            (self::MlOptional::Undefined, None) => true,
                                            _ => false,
                                        }) && ((({
                                            let source_ml_source2_nullish_field_receiver25601 =
                                                (prior.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver25601
                                                .clone())
                                            .0
                                            {
                                                self::MlOptional::Value(ml_present) => {
                                                    let _ = ml_present.clone();
                                                    true
                                                }
                                                self::MlOptional::Null => false,
                                                self::MlOptional::Undefined => false,
                                            } {
                                                {
                                                    let source_ml_source2_nullish_field_present25602 = match (source_ml_source2_nullish_field_receiver25601.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                                    match source_ml_source2_nullish_field_present25602.clone() {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source2_data_property_value25603, _) => {
                                                            source_ml_source2_data_property_value25603
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (if ((match (source_ml_source2_nullish_field_receiver25601.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]) }, self::MlOptional::Null => self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]), self::MlOptional::Undefined => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) {
                                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 39, 41])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 39, 41])
                                                }).0, message_present: true }).raise()
                                            }
                                        }) != registered)
                                            || (!(match (prior_key.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            }) && (!registered
                                                || !(self::test_java_script_semantic_same_value(
                                                    prior_key
                                                        .clone()
                                                        .expect("a proven present value"),
                                                    key.clone(),
                                                ))))))
                                        {
                                            self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(Box::new(heap_2.clone()), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 112, 114, 101, 115, 101, 114, 118, 101, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 111, 114, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 101, 120, 97, 99, 116, 32, 107, 101, 121]))
                                        } else {
                                            {
                                                let at = if registered {
                                                    self::find_java_script_symbol_registry_key(
                                                        heap_2.clone(),
                                                        key.clone(),
                                                    )
                                                } else {
                                                    (-1f64)
                                                };
                                                {
                                                    let entry = {
                                                        let source_ml_source2_optional_chain_receiver25605 = (match heap_2.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source2_data_property_value25604) => {
                                                                ((source_ml_source2_data_property_value25604.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                            }
                                                        },);
                                                        match if match (source_ml_source2_optional_chain_receiver25605.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                            Some({
                                                                let source_ml_source1_temporary_optional_chain1175 = ((source_ml_source2_optional_chain_receiver25605.clone()).0).clone().expect("a proven present value");
                                                                { let ml_reference_array1367 = source_ml_source1_temporary_optional_chain1175.clone(); let ml_reference_position1367 = at; let ml_reference_projection1367 = self::ml_array::optional_at(&*ml_reference_array1367.borrow(), self::ml_array::number_index(ml_reference_position1367)); ml_reference_projection1367 }
                                                            })
                                                        } else {
                                                            None::<self::JavaScriptSymbolRegistryEntry>
                                                        } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                    };
                                                    if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match entry.clone().expect("a proven present value") {
                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source2_data_property_value25609, _) => {
                                                            source_ml_source2_data_property_value25609.clone()
                                                        }
                                                    }) != identity)) {
                                                        self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(Box::new(heap_2.clone()), self::ml_string::Text::from_units(vec![111, 110, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 104, 97, 115, 32, 111, 110, 101, 32, 99, 97, 110, 111, 110, 105, 99, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                                    } else {
                                                        if ((!(match (prior.clone(), None::<()>) { (self::MlOptional::Value(_ml_left), Some(_ml_right)) => false, (self::MlOptional::Null, None) => false, (self::MlOptional::Undefined, None) => true, _ => false }) && (({
                                                            let source_ml_source2_nullish_field_receiver25613 = (prior.clone(),);
                                                            if match (source_ml_source2_nullish_field_receiver25613.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                                                                {
                                                                    let source_ml_source2_nullish_field_present25614 = match (source_ml_source2_nullish_field_receiver25613.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                                                    match source_ml_source2_nullish_field_present25614.clone() {
                                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source2_data_property_value25615, _) => {
                                                                            source_ml_source2_data_property_value25615
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (if ((match (source_ml_source2_nullish_field_receiver25613.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]) }, self::MlOptional::Null => self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]), self::MlOptional::Undefined => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) {
                                                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 39, 41])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 39, 41])
                                                                }).0, message_present: true }).raise()
                                                            }
                                                        }) == registered)) && (!registered || ((!(match (prior_key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value(prior_key.clone().expect("a proven present value"), key.clone())) && {
                                                            let source_ml_source2_comparison_operand25618 = {
                                                                let source_ml_source2_optional_chain_receiver25616 = (entry.clone(),);
                                                                if match (source_ml_source2_optional_chain_receiver25616.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                    Some({
                                                                        let source_ml_source1_temporary_optional_chain1176 = ((source_ml_source2_optional_chain_receiver25616.clone()).0).clone().expect("a proven present value");
                                                                        match source_ml_source1_temporary_optional_chain1176.clone() {
                                                                            self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source2_data_property_value25617, _) => {
                                                                                source_ml_source2_data_property_value25617.clone()
                                                                            }
                                                                        }
                                                                    })
                                                                } else {
                                                                    None::<self::ml::Big>
                                                                }
                                                            };
                                                            match (source_ml_source2_comparison_operand25618.clone(), identity.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                        }))) {
                                                            self::JavaScriptSymbolRegistryTransition::SymbolRegistryValue(Box::new(heap_2.clone()), Box::new(self::JavaScriptSemanticValue::Symbol(identity.clone())))
                                                        } else {
                                                            {
                                                                let next = heap_2.clone();
                                                                {
                                                                    let source_ml_source2_s25622 = if !(self::ml_array::includes(&{ let ml_reference_value1368 = (match heap_2.clone() {
                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value25624, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value25624.clone()
                                                                        }
                                                                    }).borrow().clone(); ml_reference_value1368 }, &identity.clone(), 0f64, false, |a, b| a == b)) {
                                                                        {
                                                                            let added = self::register_java_script_ordinary_symbol(heap_2.clone(), identity.clone());
                                                                            if !((({ let ml_reference_value1369 = added.borrow().clone(); ml_reference_value1369 }).0).2) {
                                                                                self::MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177::MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177Return(Box::new(self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(Box::new(heap_2.clone()), (({ let ml_reference_value1370 = added.borrow().clone(); ml_reference_value1370 }).0).0)))
                                                                            } else {
                                                                                {
                                                                                    let next_2 = (({ let ml_reference_value1371 = added.borrow().clone(); ml_reference_value1371 }).0).1;
                                                                                    self::MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177::MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177Next(Box::new(next_2.clone()))
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        self::MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177::MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177Next(Box::new(next.clone()))
                                                                    };
                                                                    match source_ml_source2_s25622.clone() {
                                                                        self::MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177::MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177Next(next_3) => {
                                                                            let next_3 = *next_3;
                                                                            {
                                                                                let fact = if registered {
                                                                                    {
                                                                                        let source_ml_source2_constructor_field25627 = Some(key.clone());
                                                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(identity.clone(), registered, Some((source_ml_source2_constructor_field25627.clone(),)))
                                                                                    }
                                                                                } else {
                                                                                    self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(identity.clone(), registered, None::<(Option<self::JavaScriptSemanticValue>,)>)
                                                                                };
                                                                                {
                                                                                    let classified = self::retain_java_script_symbol_registration(next_3.clone(), fact.clone());
                                                                                    match classified.clone() {
                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                                                                            {
                                                                                                let entries = (match next_3.clone() {
                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source2_data_property_value25630) => {
                                                                                                        ((source_ml_source2_data_property_value25630.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistryEntry>::new())));
                                                                                                {
                                                                                                    let insert = ((-at) - 1f64);
                                                                                                    {
                                                                                                        let registry = if (registered && match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                let source_ml_source2_concat_value25631 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1372 = entries.borrow().clone(); ml_reference_value1372 }, 0f64, insert)));
                                                                                                                {
                                                                                                                    let source_ml_source2_concat_value25632 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(identity.clone(), Box::new(key.clone()))]));
                                                                                                                    {
                                                                                                                        let source_ml_source2_concat_value25633 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1373 = entries.borrow().clone(); ml_reference_value1373 }, insert, f64::INFINITY)));
                                                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSymbolRegistryEntry>::new(), { let ml_reference_value1374 = source_ml_source2_concat_value25631.borrow().clone(); ml_reference_value1374 }), { let ml_reference_value1375 = source_ml_source2_concat_value25632.borrow().clone(); ml_reference_value1375 }), { let ml_reference_value1376 = source_ml_source2_concat_value25633.borrow().clone(); ml_reference_value1376 })
                                                                                                                    }
                                                                                                                }
                                                                                                            }))
                                                                                                        } else {
                                                                                                            entries.clone()
                                                                                                        };
                                                                                                        {
                                                                                                            let updated = {
                                                                                                                let source_ml_source2_nominal_field25635 = shared_constructor_field1.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field25637 = shared_constructor_field2.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field25639 = shared_constructor_field3.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field25641 = shared_constructor_field4.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field25643 = shared_constructor_field5.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field25645 = shared_constructor_field6.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override25647 = self::read_java_script_symbol_registry_model();
                                                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_nominal_field25635.clone(), source_ml_source2_nominal_field25637.clone(), source_ml_source2_nominal_field25639.clone(), source_ml_source2_nominal_field25641.clone(), Some((Some(source_ml_source2_nominal_override25647.clone()),)), Some((Some(registry.clone()),)))
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let failure = self::read_java_script_ordinary_heap_error(updated.clone());
                                                                                                                if (failure == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                    self::JavaScriptSymbolRegistryTransition::SymbolRegistryValue(Box::new(updated.clone()), Box::new(self::JavaScriptSemanticValue::Symbol(identity.clone())))
                                                                                                                } else {
                                                                                                                    self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(Box::new(heap_2.clone()), failure.clone())
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
                                                                        self::MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177::MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177Return(source_ml_source1_temporary_result1177) => {
                                                                            let source_ml_source1_temporary_result1177 = *source_ml_source1_temporary_result1177;
                                                                            source_ml_source1_temporary_result1177.clone()
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_registered_symbol(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
) -> self::JavaScriptSymbolRegistryTransition {
    {
        let request = self::execute_java_script_symbol_registry_operation(
            heap.clone(),
            &(self::ml_string::Text::from_units(vec![102, 111, 114])),
            key.clone(),
        );
        match request.clone() {
            self::JavaScriptSymbolRegistryTransition::SymbolRegistryAllocate(heap_2, _) => {
                let heap_2 = *heap_2;
                self::import_java_script_symbol_registry_witness(
                    heap_2.clone(),
                    identity.clone(),
                    key.clone(),
                )
            }
            _ => request.clone(),
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_collection_for_each_operation(operation: &self::ml_string::Text) -> bool {
    self::test_java_script_standard_collection_for_each_operation(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_for_each_kind(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            77, 97, 112, 46, 102, 111, 114, 69, 97, 99, 104,
        ])))
    {
        self::ml_string::Text::from_units(vec![77, 97, 112])
    } else {
        if ((*operation)
            == (self::ml_string::Text::from_units(vec![
                83, 101, 116, 46, 102, 111, 114, 69, 97, 99, 104,
            ])))
        {
            self::ml_string::Text::from_units(vec![83, 101, 116])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}
