// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallProgram;
use super::JavaScriptCollectionCompletion;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptConstructionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptPrimitiveWrapperState;
use super::JavaScriptPrimitiveWrapperTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFinishJavaScriptPropertyKeyConversionLoop994Result;
use super::accept_java_script_construction_conversion;
use super::adapt_java_script_bound_construction;
use super::allocate_java_script_array;
use super::allocate_java_script_collection;
use super::allocate_java_script_weak_collection;
use super::begin_java_script_primitive_conversion;
use super::continue_java_script_construction_prototype;
use super::insert_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_construction_conversion_operation;
use super::read_java_script_construction_state_error;
use super::read_java_script_ordinary_value_error;
use super::resume_java_script_primitive_wrapper;
use super::resume_java_script_scalar_protocol;
use super::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop994;
use super::step_java_script_construction;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_construction_allocation(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptConstructionState,
    identity: self::ml::Big,
) -> self::JavaScriptConstructionTransition {
    match state.clone() {
        self::JavaScriptConstructionState::ConstructionState(
            shared_constructor_field1,
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
            shared_constructor_field12,
            shared_constructor_field13,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            if !(match (
                match state.clone() {
                    self::JavaScriptConstructionState::ConstructionState(
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
                        _,
                        _,
                        source_ml_source2_data_property_value19802,
                    ) => ((source_ml_source2_data_property_value19802
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) {
                {
                    let error = self::read_java_script_construction_state_error(
                        program.clone(),
                        heap.clone(),
                        directory.clone(),
                        state.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptConstructionTransition::ConstructionInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    } else {
                        self::adapt_java_script_bound_construction(
                            state.clone(),
                            self::resume_java_script_construction_allocation(
                                program.clone(),
                                heap.clone(),
                                directory.clone(),
                                (match state.clone() {
                                    self::JavaScriptConstructionState::ConstructionState(
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
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19803,
                                    ) => ((source_ml_source2_data_property_value19803
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptConstructionState>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value"),
                                identity.clone(),
                            ),
                        )
                    }
                }
            } else {
                {
                    let error_2 = self::read_java_script_construction_state_error(
                        program.clone(),
                        heap.clone(),
                        directory.clone(),
                        state.clone(),
                    );
                    if ((error_2 != (self::ml_string::Text::from_units(vec![])))
                        || (shared_constructor_field4
                            != (self::ml_string::Text::from_units(vec![
                                97, 108, 108, 111, 99, 97, 116, 101,
                            ]))))
                    {
                        self::JavaScriptConstructionTransition::ConstructionInvalid(
                            Box::new(heap.clone()),
                            if !error_2.0.is_empty() {
                                error_2.clone()
                            } else {
                                self::ml_string::Text::from_units(vec![
                                    99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32,
                                    105, 100, 101, 110, 116, 105, 116, 121, 32, 114, 101, 115, 112,
                                    111, 110, 100, 115, 32, 111, 110, 108, 121, 32, 116, 111, 32,
                                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108,
                                    111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 101,
                                    115, 116,
                                ])
                            },
                        )
                    } else {
                        {
                            let request = self::step_java_script_construction(
                                program.clone(),
                                heap.clone(),
                                directory.clone(),
                                state.clone(),
                            );
                            match request.clone() {
                                self::JavaScriptConstructionTransition::ConstructionResourceLimit(_, _, _) => {
                                    request.clone()
                                }
                                _ => {
                                    match request.clone() {
                                        self::JavaScriptConstructionTransition::ConstructionAllocate(_, _, _) => {
                                            if (shared_constructor_field10 == (self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]))) {
                                                {
                                                    let created = self::resume_java_script_primitive_wrapper(heap.clone(), self::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(Box::new(shared_constructor_field8.clone()), Box::new(shared_constructor_field6.clone())), identity.clone());
                                                    match created.clone() {
                                                        self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperAllocate(heap_2, _, _) => {
                                                            let heap_2 = *heap_2;
                                                            self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_2.clone()), self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                        }
                                                        self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperCompleted(heap_3, value) => {
                                                            let heap_3 = *heap_3;
                                                            let value = *value;
                                                            self::JavaScriptConstructionTransition::ConstructionCompleted(Box::new(heap_3.clone()), Box::new(value.clone()))
                                                        }
                                                        self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperInvalid(heap_4, reason) => {
                                                            let heap_4 = *heap_4;
                                                            self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_4.clone()), reason.clone())
                                                        }
                                                        self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperResourceLimit(heap_5, _, reason_2) => {
                                                            let heap_5 = *heap_5;
                                                            self::JavaScriptConstructionTransition::ConstructionResourceLimit(Box::new(heap_5.clone()), Box::new(state.clone()), reason_2.clone())
                                                        }
                                                    }
                                                }
                                            } else {
                                                if (shared_constructor_field10 == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
                                                    {
                                                        let created_2 = self::allocate_java_script_array(heap.clone(), identity.clone(), shared_constructor_field6.clone(), if (({ let ml_reference_array1029 = shared_constructor_field3.clone(); let ml_reference_projection1029 = ml_reference_array1029.borrow().len() as f64; ml_reference_projection1029 }) >= (2f64)) {
                                                            { let ml_reference_array1030 = shared_constructor_field3.clone(); let ml_reference_projection1030 = ml_reference_array1030.borrow().len() as f64; ml_reference_projection1030 }
                                                        } else {
                                                            0f64
                                                        });
                                                        match created_2.clone() {
                                                            self::JavaScriptArrayCompletion::ArrayNormal(heap_6, value_2) => {
                                                                let heap_6 = *heap_6;
                                                                let value_2 = *value_2;
                                                                if (({ let ml_reference_array1031 = shared_constructor_field3.clone(); let ml_reference_projection1031 = ml_reference_array1031.borrow().len() as f64; ml_reference_projection1031 }) == (0f64)) {
                                                                    self::JavaScriptConstructionTransition::ConstructionCompleted(Box::new(heap_6.clone()), Box::new(value_2.clone()))
                                                                } else {
                                                                    self::JavaScriptConstructionTransition::ConstructionContinue(Box::new(heap_6.clone()), Box::new({
                                                                        let source_ml_source2_nominal_field19844 = shared_constructor_field1.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field19846 = shared_constructor_field2.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field19848 = shared_constructor_field3.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field19850 = shared_constructor_field4.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field19852 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field19854 = shared_constructor_field6.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field19856 = shared_constructor_field7.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field19858 = shared_constructor_field8.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field19860 = shared_constructor_field9;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field19862 = shared_constructor_field10.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field19864 = shared_constructor_field11.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field19866 = shared_constructor_field12.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field19868 = shared_constructor_field13.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override19871 = if ((({ let ml_reference_array1032 = shared_constructor_field3.clone(); let ml_reference_projection1032 = ml_reference_array1032.borrow().len() as f64; ml_reference_projection1032 }) == (1f64)) && {
                                                                                                                                let source_ml_source2_comparison_operand19841 = {
                                                                                                                                    let source_ml_source2_optional_chain_receiver19840 = ({
                                                                                                                                        let source_ml_source2_array_read_value19838 = shared_constructor_field3.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_array_read_index19839 = 0f64;
                                                                                                                                            { let ml_reference_array1033 = source_ml_source2_array_read_value19838.clone(); let ml_reference_position1033 = source_ml_source2_array_read_index19839; let ml_reference_projection1033 = self::ml_array::optional_at(&*ml_reference_array1033.borrow(), self::ml_array::number_index(ml_reference_position1033)); ml_reference_projection1033 }
                                                                                                                                        }
                                                                                                                                    },);
                                                                                                                                    if match (source_ml_source2_optional_chain_receiver19840.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                        Some({
                                                                                                                                            let source_ml_source1_temporary_optional_chain993 = ((source_ml_source2_optional_chain_receiver19840.clone()).0).clone().expect("a proven present value");
                                                                                                                                            match source_ml_source1_temporary_optional_chain993.clone() {
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
                                                                                                                                            }
                                                                                                                                        })
                                                                                                                                    } else {
                                                                                                                                        None::<self::ml_string::Text>
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                match (source_ml_source2_comparison_operand19841.clone(), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                                                            }) {
                                                                                                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])
                                                                                                                            } else {
                                                                                                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 101, 108, 101, 109, 101, 110, 116, 115])
                                                                                                                            };
                                                                                                                            self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19844.clone()), Box::new(source_ml_source2_nominal_field19846.clone()), source_ml_source2_nominal_field19848.clone(), source_ml_source2_nominal_override19871.clone(), source_ml_source2_nominal_field19852.clone(), Box::new(source_ml_source2_nominal_field19854.clone()), Box::new(value_2.clone()), Box::new(source_ml_source2_nominal_field19858.clone()), source_ml_source2_nominal_field19860, source_ml_source2_nominal_field19862.clone(), source_ml_source2_nominal_field19864.clone(), source_ml_source2_nominal_field19866.clone(), source_ml_source2_nominal_field19868.clone())
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
                                                                    }))
                                                                }
                                                            }
                                                            self::JavaScriptArrayCompletion::ArrayPropertyComplete(heap_7, _, _) => {
                                                                let heap_7 = *heap_7;
                                                                self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_7.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                            }
                                                            self::JavaScriptArrayCompletion::ArrayNumberRequest(heap_8, _, _) => {
                                                                let heap_8 = *heap_8;
                                                                self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_8.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                            }
                                                            self::JavaScriptArrayCompletion::ArrayDefinitionContinue(heap_9, _) => {
                                                                let heap_9 = *heap_9;
                                                                self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_9.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                            }
                                                            self::JavaScriptArrayCompletion::ArrayInvalid(heap_10, reason_3) => {
                                                                let heap_10 = *heap_10;
                                                                self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_10.clone()), reason_3.clone())
                                                            }
                                                            self::JavaScriptArrayCompletion::ArrayError(heap_11, _, reason_4) => {
                                                                let heap_11 = *heap_11;
                                                                self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_11.clone()), reason_4.clone())
                                                            }
                                                            self::JavaScriptArrayCompletion::ArrayResourceLimit(heap_12, reason_5, _) => {
                                                                let heap_12 = *heap_12;
                                                                self::JavaScriptConstructionTransition::ConstructionResourceLimit(Box::new(heap_12.clone()), Box::new(state.clone()), reason_5.clone())
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if ((shared_constructor_field10 == (self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110]))) || (shared_constructor_field10 == (self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110])))) {
                                                        {
                                                            let created_3 = if (shared_constructor_field10 == (self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110]))) {
                                                                self::allocate_java_script_weak_collection(heap.clone(), identity.clone(), shared_constructor_field6.clone(), &(shared_constructor_field5))
                                                            } else {
                                                                self::allocate_java_script_collection(heap.clone(), identity.clone(), shared_constructor_field6.clone(), &(shared_constructor_field5))
                                                            };
                                                            match created_3.clone() {
                                                                self::JavaScriptCollectionCompletion::CollectionNormal(heap_13, value_3) => {
                                                                    let heap_13 = *heap_13;
                                                                    let value_3 = *value_3;
                                                                    {
                                                                        let call = self::JavaScriptConstructionCall::ConstructionCallRecord(Box::new(shared_constructor_field1.clone()), Box::new(value_3.clone()), std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1034 = shared_constructor_field3.borrow().clone(); ml_reference_value1034 }, 0f64, f64::INFINITY))), true, Box::new(shared_constructor_field2.clone()));
                                                                        self::JavaScriptConstructionTransition::ConstructionContinue(Box::new(heap_13.clone()), Box::new({
                                                                            let source_ml_source2_nominal_field19888 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field19890 = shared_constructor_field2.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field19892 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field19894 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field19896 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field19898 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field19900 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field19902 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field19904 = shared_constructor_field9;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field19906 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field19908 = shared_constructor_field11.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field19910 = shared_constructor_field12.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field19912 = shared_constructor_field13.clone();
                                                                                                                            self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19888.clone()), Box::new(source_ml_source2_nominal_field19890.clone()), source_ml_source2_nominal_field19892.clone(), self::ml_string::Text::from_units(vec![98, 111, 100, 121]), source_ml_source2_nominal_field19896.clone(), Box::new(source_ml_source2_nominal_field19898.clone()), Box::new(value_3.clone()), Box::new(source_ml_source2_nominal_field19902.clone()), source_ml_source2_nominal_field19904, source_ml_source2_nominal_field19906.clone(), source_ml_source2_nominal_field19908.clone(), Some((Some(call.clone()),)), source_ml_source2_nominal_field19912.clone())
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
                                                                        }))
                                                                    }
                                                                }
                                                                self::JavaScriptCollectionCompletion::CollectionInvalid(heap_14, reason_6) => {
                                                                    let heap_14 = *heap_14;
                                                                    self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_14.clone()), reason_6.clone())
                                                                }
                                                                self::JavaScriptCollectionCompletion::CollectionTypeError(heap_15, reason_7) => {
                                                                    let heap_15 = *heap_15;
                                                                    self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_15.clone()), reason_7.clone())
                                                                }
                                                                self::JavaScriptCollectionCompletion::CollectionResourceLimit(heap_16, reason_8) => {
                                                                    let heap_16 = *heap_16;
                                                                    self::JavaScriptConstructionTransition::ConstructionResourceLimit(Box::new(heap_16.clone()), Box::new(state.clone()), reason_8.clone())
                                                                }
                                                                self::JavaScriptCollectionCompletion::CollectionNeedIterable(heap_17, _, _) => {
                                                                    let heap_17 = *heap_17;
                                                                    self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_17.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {
                                                            let created_4 = self::insert_java_script_ordinary_object(heap.clone(), identity.clone(), shared_constructor_field6.clone());
                                                            if !((({ let ml_reference_value1035 = created_4.borrow().clone(); ml_reference_value1035 }).0).2) {
                                                                self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new((({ let ml_reference_value1036 = created_4.borrow().clone(); ml_reference_value1036 }).0).1), (({ let ml_reference_value1037 = created_4.borrow().clone(); ml_reference_value1037 }).0).0)
                                                            } else {
                                                                {
                                                                    let receiver = self::JavaScriptSemanticValue::Object(identity.clone(), false);
                                                                    if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]))) {
                                                                        self::JavaScriptConstructionTransition::ConstructionCompleted(Box::new((({ let ml_reference_value1038 = created_4.borrow().clone(); ml_reference_value1038 }).0).1), Box::new(receiver.clone()))
                                                                    } else {
                                                                        {
                                                                            let call_2 = self::JavaScriptConstructionCall::ConstructionCallRecord(Box::new(shared_constructor_field1.clone()), Box::new(receiver.clone()), std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1039 = shared_constructor_field3.borrow().clone(); ml_reference_value1039 }, 0f64, f64::INFINITY))), true, Box::new(shared_constructor_field2.clone()));
                                                                            self::JavaScriptConstructionTransition::ConstructionContinue(Box::new((({ let ml_reference_value1040 = created_4.borrow().clone(); ml_reference_value1040 }).0).1), Box::new({
                                                                                let source_ml_source2_nominal_field19924 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field19926 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field19928 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field19930 = shared_constructor_field4.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field19932 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field19934 = shared_constructor_field6.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field19936 = shared_constructor_field7.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field19938 = shared_constructor_field8.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field19940 = shared_constructor_field9;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field19942 = shared_constructor_field10.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field19944 = shared_constructor_field11.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field19946 = shared_constructor_field12.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field19948 = shared_constructor_field13.clone();
                                                                                                                                self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19924.clone()), Box::new(source_ml_source2_nominal_field19926.clone()), source_ml_source2_nominal_field19928.clone(), self::ml_string::Text::from_units(vec![98, 111, 100, 121]), source_ml_source2_nominal_field19932.clone(), Box::new(source_ml_source2_nominal_field19934.clone()), Box::new(receiver.clone()), Box::new(source_ml_source2_nominal_field19938.clone()), source_ml_source2_nominal_field19940, source_ml_source2_nominal_field19942.clone(), source_ml_source2_nominal_field19944.clone(), Some((Some(call_2.clone()),)), source_ml_source2_nominal_field19948.clone())
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
                                                                            }))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 110, 111, 116, 104, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
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
pub fn resume_java_script_construction_call(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptConstructionState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptConstructionTransition {
    match state.clone() {
        self::JavaScriptConstructionState::ConstructionState(
            _,
            _,
            _,
            shared_constructor_field1,
            _,
            _,
            shared_constructor_field2,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            if !(match (
                match state.clone() {
                    self::JavaScriptConstructionState::ConstructionState(
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
                        _,
                        _,
                        source_ml_source2_data_property_value19953,
                    ) => ((source_ml_source2_data_property_value19953
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) {
                {
                    let error = self::read_java_script_construction_state_error(
                        program.clone(),
                        heap.clone(),
                        directory.clone(),
                        state.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptConstructionTransition::ConstructionInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    } else {
                        self::adapt_java_script_bound_construction(
                            state.clone(),
                            self::resume_java_script_construction_call(
                                program.clone(),
                                heap.clone(),
                                directory.clone(),
                                (match state.clone() {
                                    self::JavaScriptConstructionState::ConstructionState(
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
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19954,
                                    ) => ((source_ml_source2_data_property_value19954
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptConstructionState>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value"),
                                completion.clone(),
                            ),
                        )
                    }
                }
            } else {
                {
                    let error_2 = {
                        let source_ml_source2_logical19957 =
                            self::read_java_script_construction_state_error(
                                program.clone(),
                                heap.clone(),
                                directory.clone(),
                                state.clone(),
                            );
                        if !source_ml_source2_logical19957.0.is_empty() {
                            source_ml_source2_logical19957.clone()
                        } else {
                            self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value19955) => {
                                    let source_ml_source2_data_property_value19955 = *source_ml_source2_data_property_value19955;
                                    source_ml_source2_data_property_value19955.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value19956) => {
                                    let source_ml_source2_data_property_value19956 = *source_ml_source2_data_property_value19956;
                                    source_ml_source2_data_property_value19956.clone()
                                }
                            })
                        }
                    };
                    if ((error_2 != (self::ml_string::Text::from_units(vec![])))
                        || match (
                            match state.clone() {
                                self::JavaScriptConstructionState::ConstructionState(
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
                                    _,
                                    source_ml_source2_data_property_value19958,
                                    _,
                                ) => ((source_ml_source2_data_property_value19958
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptConstructionCall>,)
                                    }))
                                .0)
                                    .clone(),
                            },
                            None::<()>,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        })
                    {
                        self::JavaScriptConstructionTransition::ConstructionInvalid(
                            Box::new(heap.clone()),
                            if !error_2.0.is_empty() {
                                error_2.clone()
                            } else {
                                self::ml_string::Text::from_units(vec![
                                    99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32,
                                    114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 108, 121, 32,
                                    105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                                    97, 99, 116, 117, 97, 108, 32, 99, 104, 105, 108, 100, 32, 99,
                                    97, 108, 108,
                                ])
                            },
                        )
                    } else {
                        match completion.clone() {
                            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                                value,
                            ) => {
                                let value = *value;
                                if ((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116]))) && !(match (match state.clone() {
                                    self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19962, _, _) => {
                                        ((source_ml_source2_data_property_value19962.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                    }
                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                    self::accept_java_script_construction_conversion(heap.clone(), directory.clone(), state.clone(), self::resume_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                        self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19963, _, _) => {
                                            ((source_ml_source2_data_property_value19963.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value"), completion.clone(), &(self::read_java_script_construction_conversion_operation(state.clone()))))
                                } else {
                                    if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]))) {
                                        self::continue_java_script_construction_prototype(heap.clone(), directory.clone(), state.clone(), value.clone())
                                    } else {
                                        if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![98, 111, 100, 121]))) {
                                            self::JavaScriptConstructionTransition::ConstructionCompleted(Box::new(heap.clone()), Box::new(if match value.clone() {
                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            } {
                                                value.clone()
                                            } else {
                                                shared_constructor_field2.clone()
                                            }))
                                        } else {
                                            self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 100, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 111, 114, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 112, 104, 97, 115, 101]))
                                        }
                                    }
                                }
                            }
                            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                value_2,
                            ) => {
                                let value_2 = *value_2;
                                self::JavaScriptConstructionTransition::ConstructionThrow(
                                    Box::new(heap.clone()),
                                    Box::new(value_2.clone()),
                                )
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
pub fn read_java_script_primitive_property_key_spelling(
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
        }
        self::JavaScriptSemanticValue::Null => {
            self::ml_string::Text::from_units(vec![110, 117, 108, 108])
        }
        self::JavaScriptSemanticValue::Boolean(value_value) => {
            if value_value {
                self::ml_string::Text::from_units(vec![116, 114, 117, 101])
            } else {
                self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
            }
        }
        self::JavaScriptSemanticValue::Number(value_value_2) => {
            self::ml_string::Text::from_text(&self::ml_number::js_number(value_value_2))
        }
        self::JavaScriptSemanticValue::Integer(value_value_3) => {
            self::ml_string::Text::from_text(&value_value_3.to_string())
        }
        self::JavaScriptSemanticValue::Symbol(_) => (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101,
                121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 101,
                120, 97, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121,
            ]))
            .0,
            message_present: true,
        })
        .raise(),
        self::JavaScriptSemanticValue::Object(_, _) => {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 110,
                    101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116,
                    101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105,
                    118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        }
        _ => (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 107, 101, 121,
                115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 101, 120,
                97, 99, 116, 32, 117, 110, 105, 116, 115,
            ]))
            .0,
            message_present: true,
        })
        .raise(),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_property_key_conversion(
    transition: self::JavaScriptPrimitiveConversionTransition,
) -> self::JavaScriptPropertyKeyConversionTransition {
    match transition.clone() {
        self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(state) => {
            let state = *state;
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(Box::new(
                state.clone(),
            ))
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(
            state_2,
            callee,
            receiver,
            arguments,
        ) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(
                Box::new(state_2.clone()),
                Box::new(callee.clone()),
                Box::new(receiver.clone()),
                arguments.clone(),
            )
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(value) => {
            let value = *value;
            if ((match value.clone() {
                self::JavaScriptSemanticValue::Text(_) => true,
                _ => false,
            } || match value.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => true,
                _ => false,
            }) || match value.clone() {
                self::JavaScriptSemanticValue::Symbol(_) => true,
                _ => false,
            }) {
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(
                    value.clone(),
                ))
            } else {
                match value.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
                            self::ml_string::Text::from_units(vec![
                                97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 112, 114,
                                105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114,
                                115, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111,
                                110, 116, 97, 105, 110, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116,
                            ]),
                        )
                    }
                    _ => {
                        let spelling =
                            self::read_java_script_primitive_property_key_spelling(value.clone());
                        if ((spelling.length()) > (1048576f64)) {
                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32,
                                    116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32,
                                    116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                                    85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100,
                                    103, 101, 116,
                                ]),
                            )
                        } else {
                            {
                                let units =
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                {
                                    let index = 0f64;
                                    {
                                        let source_ml_source2_s19985 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop994(&(spelling), units.clone(), index);
                                        match source_ml_source2_s19985.clone() {
                                                self::MlSource1TemporaryFinishJavaScriptPropertyKeyConversionLoop994Result::MlSource1TemporaryFinishJavaScriptPropertyKeyConversionLoop994Done => {
                                                    self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units.clone())))
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
        self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(value_2) => {
            let value_2 = *value_2;
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(Box::new(
                value_2.clone(),
            ))
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
            error_class,
            reason,
        ) => self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(
            error_class.clone(),
            reason.clone(),
        ),
        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(reason_2) => {
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_property_key_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    input: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptPropertyKeyConversionTransition {
    self::finish_java_script_property_key_conversion(self::begin_java_script_primitive_conversion(
        heap.clone(),
        input.clone(),
        exotic_key.clone(),
        &(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])),
    ))
}
