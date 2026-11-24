// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayCreationCall;
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorSelectedCall;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveWrapperState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Result;
use super::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413;
use super::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Result;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_array_creation_state_error;
use super::read_java_script_ordinary_heap_error;
use super::source_ml_source1_temporary_begin_java_script_array_creation_loop416;
use super::source_ml_source1_temporary_read_java_script_array_creation_callback_error_loop414;
use super::test_java_script_array_creation_operation;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_creation_callback_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArrayCreationState::ArrayCreationState(
            _,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            _,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            _,
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
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
        ) => {
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
            {
                let source_ml_source2_s14547 = if !(match (
                    match state.clone() {
                        self::JavaScriptArrayCreationState::ArrayCreationState(
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
                            _,
                            _,
                            source_ml_source2_data_property_value14548,
                            _,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value14548
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptArrayCreationCall>,)))
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
                        let call = (match state.clone() {
                            self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                _,
                                _,
                                source_ml_source2_data_property_value14549,
                                _,
                                _,
                                _,
                            ) => ((source_ml_source2_data_property_value14549
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArrayCreationCall>,)))
                            .0)
                                .clone(),
                        })
                        .clone()
                        .expect("a proven present value");
                        match call.clone() {
                            self::JavaScriptArrayCreationCall::ArrayCreationCallRecord(
                                shared_constructor_field1,
                                shared_constructor_field2,
                                shared_constructor_field3,
                            ) => {
                                let shared_constructor_field1 = *shared_constructor_field1;
                                let shared_constructor_field2 = *shared_constructor_field2;
                                if ((((match call.clone() {
                                    self::JavaScriptArrayCreationCall::ArrayCreationCallRecord(
                                        _,
                                        _,
                                        _,
                                    ) => false,
                                } || {
                                    let source_ml_source2_s14554 =
                                        shared_constructor_field1.clone();
                                    match source_ml_source2_s14554.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => false,
                                        _ => true,
                                    }
                                }) || !(((match shared_constructor_field1.clone() {
                                    self::JavaScriptSemanticValue::Object(
                                        _,
                                        source_ml_source2_data_property_value14559,
                                    ) => Some((source_ml_source2_data_property_value14559,)),
                                    _ => None::<(bool,)>,
                                })
                                .clone()
                                .expect("a proven present value"))
                                .0)) || (({
                                    let ml_reference_array833 = shared_constructor_field3.clone();
                                    let ml_reference_projection833 =
                                        ml_reference_array833.borrow().len() as f64;
                                    ml_reference_projection833
                                }) > (2f64)))
                                    || !(self::ml_array::includes(
                                        &{
                                            let ml_reference_value834 =
                                                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        109, 101, 116, 104, 111, 100, 32, 103, 101,
                                                        116, 116, 101, 114,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        108, 101, 110, 103, 116, 104, 32, 103, 101,
                                                        116, 116, 101, 114,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        108, 101, 110, 103, 116, 104, 32, 99, 111,
                                                        110, 118, 101, 114, 115, 105, 111, 110,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        105, 116, 101, 114, 97, 116, 111, 114, 32,
                                                        97, 99, 113, 117, 105, 115, 105, 116, 105,
                                                        111, 110,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        105, 116, 101, 114, 97, 116, 101,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        101, 108, 101, 109, 101, 110, 116, 32, 103,
                                                        101, 116, 116, 101, 114,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        109, 97, 112,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        108, 101, 110, 103, 116, 104, 32, 115, 101,
                                                        116, 116, 101, 114,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        97, 114, 114, 97, 121, 32, 108, 101, 110,
                                                        103, 116, 104,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        99, 108, 111, 115, 101,
                                                    ]),
                                                ])))
                                                .borrow()
                                                .clone();
                                            ml_reference_value834
                                        },
                                        &shared_constructor_field8.clone(),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    )))
                                {
                                    self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 97, 108, 108, 98, 97, 99, 107]))
                                } else {
                                    {
                                        let source_ml_source1_temporary_values409 =
                                            std::rc::Rc::new(std::cell::RefCell::new(
                                                self::ml_array::append(
                                                    vec![
                                                        shared_constructor_field1.clone(),
                                                        shared_constructor_field2.clone(),
                                                    ],
                                                    {
                                                        let ml_reference_value835 =
                                                            shared_constructor_field3
                                                                .borrow()
                                                                .clone();
                                                        ml_reference_value835
                                                    },
                                                ),
                                            ));
                                        {
                                            let source_ml_source1_temporary_index409 =
                                                self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s14566 = self::source_ml_source1_temporary_read_java_script_array_creation_callback_error_loop414(heap.clone(), source_ml_source1_temporary_values409.clone(), source_ml_source1_temporary_index409.clone());
                                                match source_ml_source2_s14566.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Result::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Done => {
                                                        if (((shared_constructor_field8 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && (match (match state.clone() {
                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14569, _, _, _, _, _, _, _) => {
                                                                ((source_ml_source2_data_property_value14569.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                            }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (!((match (match state.clone() {
                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14570, _, _, _, _, _, _, _) => {
                                                                ((source_ml_source2_data_property_value14570.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                            }
                                                        }).clone().expect("a proven present value") {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source2_data_property_value14571) => {
                                                                source_ml_source2_data_property_value14571.clone()
                                                            }
                                                        }).ends_with(&self::ml_string::Text::from_units(vec![71, 101, 116, 116, 101, 114]))) && !((match (match state.clone() {
                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14572, _, _, _, _, _, _, _) => {
                                                                ((source_ml_source2_data_property_value14572.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                            }
                                                        }).clone().expect("a proven present value") {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source2_data_property_value14573) => {
                                                                source_ml_source2_data_property_value14573.clone()
                                                            }
                                                        }).ends_with(&self::ml_string::Text::from_units(vec![67, 97, 108, 108])))))) || ((shared_constructor_field8 == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]))) && {
                                                            let source_ml_source2_comparison_operand14578 = {
                                                                let source_ml_source2_optional_chain_receiver14577 = (match state.clone() {
                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14576, _, _, _, _) => {
                                                                        ((source_ml_source2_data_property_value14576.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                    }
                                                                },);
                                                                if match (source_ml_source2_optional_chain_receiver14577.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                    Some({
                                                                        let source_ml_source1_temporary_optional_chain410 = ((source_ml_source2_optional_chain_receiver14577.clone()).0).clone().expect("a proven present value");
                                                                        match source_ml_source1_temporary_optional_chain410.clone() {
                                                                            self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(_, _) => {
                                                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 65, 115, 115, 105, 103, 110, 109, 101, 110, 116, 68, 101, 102, 105, 110, 105, 116, 105, 111, 110])
                                                                            }
                                                                            self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(_, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 65, 115, 115, 105, 103, 110, 109, 101, 110, 116, 78, 117, 109, 98, 101, 114])
                                                                            }
                                                                            self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(_, _, _) => {
                                                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 65, 115, 115, 105, 103, 110, 109, 101, 110, 116, 80, 114, 105, 109, 105, 116, 105, 118, 101])
                                                                            }
                                                                        }
                                                                    })
                                                                } else {
                                                                    None::<self::ml_string::Text>
                                                                }
                                                            };
                                                            !(match (source_ml_source2_comparison_operand14578.clone(), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 65, 115, 115, 105, 103, 110, 109, 101, 110, 116, 78, 117, 109, 98, 101, 114])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                        })) {
                                                            self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 110, 101, 115, 116, 101, 100, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 115, 116, 97, 116, 101]))
                                                        } else {
                                                            if ((shared_constructor_field8 == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && ((((!(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), shared_constructor_field5.clone())) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), shared_constructor_field6.clone()))) || (({ let ml_reference_array836 = shared_constructor_field3.clone(); let ml_reference_projection836 = ml_reference_array836.borrow().len() as f64; ml_reference_projection836 }) != (2f64))) || !(self::test_java_script_semantic_same_value(({
                                                                let source_ml_source2_array_read_value14589 = shared_constructor_field3.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index14590 = 0f64;
                                                                    { let ml_reference_array837 = source_ml_source2_array_read_value14589.clone(); let ml_reference_position837 = source_ml_source2_array_read_index14590; let ml_reference_projection837 = self::ml_array::optional_at(&*ml_reference_array837.borrow(), self::ml_array::number_index(ml_reference_position837)); ml_reference_projection837 }
                                                                }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field13.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                                let source_ml_source2_array_read_value14593 = shared_constructor_field3.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index14594 = 1f64;
                                                                    { let ml_reference_array838 = source_ml_source2_array_read_value14593.clone(); let ml_reference_position838 = source_ml_source2_array_read_index14594; let ml_reference_projection838 = self::ml_array::optional_at(&*ml_reference_array838.borrow(), self::ml_array::number_index(ml_reference_position838)); ml_reference_projection838 }
                                                                }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), self::JavaScriptSemanticValue::Number(shared_constructor_field9))))) {
                                                                self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 109, 97, 112, 112, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 105, 110, 100, 101, 120, 32, 116, 104, 105, 115, 65, 114, 103, 117, 109, 101, 110, 116, 32, 97, 110, 100, 32, 99, 97, 108, 108, 98, 97, 99, 107]))
                                                            } else {
                                                                if ((((shared_constructor_field8 == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116, 101, 114]))) && (!(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), shared_constructor_field4.clone())) || (({ let ml_reference_array839 = shared_constructor_field3.clone(); let ml_reference_projection839 = ml_reference_array839.borrow().len() as f64; ml_reference_projection839 }) != (0f64)))) || (self::ml_array::includes(&{ let ml_reference_value840 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value840 }, &shared_constructor_field8.clone(), 0f64, false, |a, b| a == b) && (!(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), shared_constructor_field11.clone())) || (({ let ml_reference_array841 = shared_constructor_field3.clone(); let ml_reference_projection841 = ml_reference_array841.borrow().len() as f64; ml_reference_projection841 }) != (0f64))))) || ((shared_constructor_field8 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 115, 101, 116, 116, 101, 114]))) && ((!(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), shared_constructor_field12.clone())) || (({ let ml_reference_array842 = shared_constructor_field3.clone(); let ml_reference_projection842 = ml_reference_array842.borrow().len() as f64; ml_reference_projection842 }) != (1f64))) || !(self::test_java_script_semantic_same_value(({
                                                                    let source_ml_source2_array_read_value14614 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index14615 = 0f64;
                                                                        { let ml_reference_array843 = source_ml_source2_array_read_value14614.clone(); let ml_reference_position843 = source_ml_source2_array_read_index14615; let ml_reference_projection843 = self::ml_array::optional_at(&*ml_reference_array843.borrow(), self::ml_array::number_index(ml_reference_position843)); ml_reference_projection843 }
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), self::JavaScriptSemanticValue::Number(if shared_constructor_field7 {
                                                                    shared_constructor_field9
                                                                } else {
                                                                    shared_constructor_field10
                                                                })))))) {
                                                                    self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                } else {
                                                                    if !(match ({
                                                                        let source_ml_source2_optional_chain_receiver14620 = (match state.clone() {
                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14619, _, _, _, _, _, _, _, _) => {
                                                                                ((source_ml_source2_data_property_value14619.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                            }
                                                                        },);
                                                                        match if match (source_ml_source2_optional_chain_receiver14620.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            Some({
                                                                                let source_ml_source1_temporary_optional_chain411 = ((source_ml_source2_optional_chain_receiver14620.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source1_temporary_optional_chain411.clone() {
                                                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14621) => {
                                                                                        ((source_ml_source2_data_property_value14621.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                    }
                                                                                }
                                                                            })
                                                                        } else {
                                                                            None::<self::JavaScriptIteratorSelectedCall>
                                                                        } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                        {
                                                                            let nested = {
                                                                                let source_ml_source2_nullish_field_receiver14623 = (match state.clone() {
                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14622, _, _, _, _, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value14622.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                    }
                                                                                },);
                                                                                if match (source_ml_source2_nullish_field_receiver14623.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    {
                                                                                        let source_ml_source2_nullish_field_present14624 = ((source_ml_source2_nullish_field_receiver14623.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source2_nullish_field_present14624.clone() {
                                                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14625) => {
                                                                                                ((source_ml_source2_data_property_value14625.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 39, 41])).0, message_present: true }).raise()
                                                                                }
                                                                            };
                                                                            if ((!(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), {
                                                                                let source_ml_source2_nullish_field_receiver14627 = (nested.clone(),);
                                                                                if match (source_ml_source2_nullish_field_receiver14627.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    {
                                                                                        let source_ml_source2_nullish_field_present14628 = ((source_ml_source2_nullish_field_receiver14627.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source2_nullish_field_present14628.clone() {
                                                                                            self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(source_ml_source2_data_property_value14629, _, _) => {
                                                                                                let source_ml_source2_data_property_value14629 = *source_ml_source2_data_property_value14629;
                                                                                                source_ml_source2_data_property_value14629.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 101, 101, 39, 41])).0, message_present: true }).raise()
                                                                                }
                                                                            })) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), {
                                                                                let source_ml_source2_nullish_field_receiver14631 = (nested.clone(),);
                                                                                if match (source_ml_source2_nullish_field_receiver14631.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    {
                                                                                        let source_ml_source2_nullish_field_present14632 = ((source_ml_source2_nullish_field_receiver14631.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source2_nullish_field_present14632.clone() {
                                                                                            self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(_, source_ml_source2_data_property_value14633, _) => {
                                                                                                let source_ml_source2_data_property_value14633 = *source_ml_source2_data_property_value14633;
                                                                                                source_ml_source2_data_property_value14633.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 99, 101, 105, 118, 101, 114, 39, 41])).0, message_present: true }).raise()
                                                                                }
                                                                            }))) || (({ let ml_reference_array844 = shared_constructor_field3.clone(); let ml_reference_projection844 = ml_reference_array844.borrow().len() as f64; ml_reference_projection844 }) != ({ let ml_reference_array845 = {
                                                                                let source_ml_source2_nullish_field_receiver14639 = (nested.clone(),);
                                                                                if match (source_ml_source2_nullish_field_receiver14639.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    {
                                                                                        let source_ml_source2_nullish_field_present14640 = ((source_ml_source2_nullish_field_receiver14639.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source2_nullish_field_present14640.clone() {
                                                                                            self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(_, _, source_ml_source2_data_property_value14641) => {
                                                                                                source_ml_source2_data_property_value14641.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 97, 114, 103, 117, 109, 101, 110, 116, 115, 39, 41])).0, message_present: true }).raise()
                                                                                }
                                                                            }; let ml_reference_projection845 = ml_reference_array845.borrow().len() as f64; ml_reference_projection845 }))) {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107]))
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Next
                                                                            }
                                                                        }
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Next
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Result::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Return(source_ml_source1_temporary_result414) => {
                                                        self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Return(source_ml_source1_temporary_result414.clone())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if (self::ml_array::includes(
                        &{
                            let ml_reference_value846 =
                                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    self::ml_string::Text::from_units(vec![
                                        109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116, 101,
                                        114,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101,
                                        114,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116,
                                        101, 114,
                                    ]),
                                    self::ml_string::Text::from_units(vec![109, 97, 112]),
                                    self::ml_string::Text::from_units(vec![
                                        108, 101, 110, 103, 116, 104, 32, 115, 101, 116, 116, 101,
                                        114,
                                    ]),
                                ])))
                                .borrow()
                                .clone();
                            ml_reference_value846
                        },
                        &shared_constructor_field8.clone(),
                        0f64,
                        false,
                        |a, b| a == b,
                    ) || !(match (
                        {
                            let source_ml_source2_optional_chain_receiver14644 =
                                (match state.clone() {
                                    self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value14643,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value14643
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptIteratorProtocolState>,)
                                        }))
                                    .0)
                                        .clone(),
                                },);
                            match if match (source_ml_source2_optional_chain_receiver14644.clone())
                                .0
                            {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                Some({
                                    let source_ml_source1_temporary_optional_chain412 =
                                        ((source_ml_source2_optional_chain_receiver14644.clone())
                                            .0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source1_temporary_optional_chain412.clone() {
                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value14645,
                                        ) => ((source_ml_source2_data_property_value14645
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptIteratorSelectedCall>,)
                                            }))
                                        .0)
                                            .clone(),
                                    }
                                })
                            } else {
                                None::<self::JavaScriptIteratorSelectedCall>
                            } {
                                Some(ml_nested) => match ml_nested {
                                    Some(ml_present) => Some(ml_present),
                                    None => None,
                                },
                                None => None,
                            }
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    })) {
                        self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 108, 111, 115, 101, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 98, 97, 99, 107]))
                    } else {
                        self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Next
                    }
                };
                match source_ml_source2_s14547.clone() {
                    self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Next => {
                        self::ml_string::Text::from_units(vec![])
                    }
                    self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Return(source_ml_source1_temporary_result413) => {
                        source_ml_source1_temporary_result413.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_array_creation(
    heap: self::JavaScriptOrdinaryHeap,
    operation: &self::ml_string::Text,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    array_prototype: self::JavaScriptSemanticValue,
    iterator_key: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayCreationTransition {
    {
        let invalid = self::read_java_script_ordinary_heap_error(heap.clone());
        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                Box::new(heap.clone()),
                invalid.clone(),
            )
        } else {
            if (({
                let ml_reference_array847 = arguments_.clone();
                let ml_reference_projection847 = ml_reference_array847.borrow().len() as f64;
                ml_reference_projection847
            }) > (1048576f64))
            {
                self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 97,
                        114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115, 32, 114,
                        101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 115, 101, 108,
                        101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 118, 97, 108,
                        117, 101, 32, 109, 111, 100, 101, 108, 32, 112, 114, 111, 102, 105, 108,
                        101,
                    ]),
                )
            } else {
                {
                    let source_ml_source1_temporary_values415 =
                        std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(
                            vec![
                                receiver.clone(),
                                array_prototype.clone(),
                                iterator_key.clone(),
                                exotic_key.clone(),
                            ],
                            {
                                let ml_reference_value848 = arguments_.borrow().clone();
                                ml_reference_value848
                            },
                        )));
                    {
                        let source_ml_source1_temporary_index415 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s14646 = self::source_ml_source1_temporary_begin_java_script_array_creation_loop416(heap.clone(), source_ml_source1_temporary_values415.clone(), source_ml_source1_temporary_index415.clone());
                            match source_ml_source2_s14646.clone() {
                                self::MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Result::MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Done => {
                                    if !(self::test_java_script_array_creation_operation(operation)) {
                                        self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let input = if ((*operation) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109]))) {
                                                ({
                                                    let source_ml_source2_array_read_index14648 = 0f64;
                                                    { let ml_reference_array849 = arguments_.clone(); let ml_reference_position849 = source_ml_source2_array_read_index14648; let ml_reference_projection849 = self::ml_array::optional_at(&*ml_reference_array849.borrow(), self::ml_array::number_index(ml_reference_position849)); ml_reference_projection849 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)
                                            } else {
                                                self::JavaScriptSemanticValue::Undefined
                                            };
                                            {
                                                let callback = if ((*operation) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109]))) {
                                                    ({
                                                        let source_ml_source2_array_read_index14650 = 1f64;
                                                        { let ml_reference_array850 = arguments_.clone(); let ml_reference_position850 = source_ml_source2_array_read_index14650; let ml_reference_projection850 = self::ml_array::optional_at(&*ml_reference_array850.borrow(), self::ml_array::number_index(ml_reference_position850)); ml_reference_projection850 }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)
                                                } else {
                                                    self::JavaScriptSemanticValue::Undefined
                                                };
                                                {
                                                    let this_argument = if ((*operation) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109]))) {
                                                        ({
                                                            let source_ml_source2_array_read_index14652 = 2f64;
                                                            { let ml_reference_array851 = arguments_.clone(); let ml_reference_position851 = source_ml_source2_array_read_index14652; let ml_reference_projection851 = self::ml_array::optional_at(&*ml_reference_array851.borrow(), self::ml_array::number_index(ml_reference_position851)); ml_reference_projection851 }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)
                                                    } else {
                                                        self::JavaScriptSemanticValue::Undefined
                                                    };
                                                    {
                                                        let mapping = match callback.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        };
                                                        if (mapping && (match callback.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        } || !(((match callback.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value14655) => {
                                                                Some((source_ml_source2_data_property_value14655,))
                                                            }
                                                            _ => {
                                                                None::<(bool,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0))) {
                                                            self::JavaScriptArrayCreationTransition::ArrayCreationError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 109, 97, 112, 112, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101]))
                                                        } else {
                                                            if (((*operation) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109]))) && (match input.clone() {
                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            } || match input.clone() {
                                                                self::JavaScriptSemanticValue::Null => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            })) {
                                                                self::JavaScriptArrayCreationTransition::ArrayCreationError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 111, 98, 106, 101, 99, 116, 45, 99, 111, 101, 114, 99, 105, 98, 108, 101, 32, 105, 110, 112, 117, 116]))
                                                            } else {
                                                                {
                                                                    let state = {
                                                                        let source_ml_source2_constructor_field14667 = if ((*operation) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 111, 102]))) {
                                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field14668 = 0f64;
                                                                            {
                                                                                let source_ml_source2_constructor_field14669 = if ((*operation) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 111, 102]))) {
                                                                                    { let ml_reference_array852 = arguments_.clone(); let ml_reference_projection852 = ml_reference_array852.borrow().len() as f64; ml_reference_projection852 }
                                                                                } else {
                                                                                    0f64
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_constructor_field14670 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source2_constructor_field14671 = self::JavaScriptSemanticValue::Undefined;
                                                                                        {
                                                                                            let source_ml_source2_constructor_field14672 = self::JavaScriptSemanticValue::Undefined;
                                                                                            {
                                                                                                let source_ml_source2_constructor_field14673 = self::JavaScriptSemanticValue::Undefined;
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field14674 = self::JavaScriptSemanticValue::Undefined;
                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(operation.clone(), Box::new(receiver.clone()), arguments_.clone(), Box::new(array_prototype.clone()), Box::new(iterator_key.clone()), Box::new(exotic_key.clone()), Box::new(input.clone()), Box::new(callback.clone()), Box::new(this_argument.clone()), mapping, false, source_ml_source2_constructor_field14667.clone(), source_ml_source2_constructor_field14668, source_ml_source2_constructor_field14669, Box::new(source_ml_source2_constructor_field14670.clone()), Box::new(source_ml_source2_constructor_field14671.clone()), Box::new(source_ml_source2_constructor_field14672.clone()), Box::new(source_ml_source2_constructor_field14673.clone()), Box::new(source_ml_source2_constructor_field14674.clone()), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSynchronousIteratorRecord>,)>, None::<(Option<self::JavaScriptIteratorProtocolState>,)>, None::<(Option<self::JavaScriptPrimitiveConversionState>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptPrimitiveWrapperState>,)>, None::<(Option<self::JavaScriptArrayAssignmentState>,)>, None::<(Option<self::JavaScriptArrayCreationCall>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<self::ml_string::Text>,)>)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    {
                                                                        let error = self::read_java_script_array_creation_state_error(heap.clone(), state.clone());
                                                                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap.clone()), error.clone())
                                                                        } else {
                                                                            self::JavaScriptArrayCreationTransition::ArrayCreationContinue(Box::new(heap.clone()), Box::new(state.clone()))
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
                                self::MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Result::MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Return(source_ml_source1_temporary_result416) => {
                                    let source_ml_source1_temporary_result416 = *source_ml_source1_temporary_result416;
                                    source_ml_source1_temporary_result416.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
