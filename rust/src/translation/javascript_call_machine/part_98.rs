// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptConstructionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveWrapperTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987;
use super::MlSource1TemporaryStepJavaScriptConstructionAllocationLoop991Result;
use super::accept_java_script_construction_conversion;
use super::begin_java_script_primitive_wrapper;
use super::continue_java_script_construction_prototype;
use super::create_java_script_ordinary_set_descriptor;
use super::create_java_script_property_descriptor;
use super::define_java_script_ordinary_property;
use super::dependency_function5;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_construction_conversion_operation;
use super::read_java_script_construction_fact;
use super::request_java_script_construction_implementation;
use super::select_java_script_construction_root;
use super::source_ml_source1_temporary_step_java_script_construction_allocation_loop988;
use super::source_ml_source1_temporary_step_java_script_construction_allocation_loop991;
use super::step_java_script_scalar_protocol;
use super::test_java_script_reflection_property_budget;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_construction_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptConstructionState,
) -> self::JavaScriptConstructionTransition {
    match state.clone() {
        self::JavaScriptConstructionState::ConstructionState(
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
            shared_constructor_field14,
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
        ) => {
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            if (shared_constructor_field8
                == (self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 32, 114, 111, 111, 116,
                ])))
            {
                self::select_java_script_construction_root(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                    true,
                )
            } else {
                if (shared_constructor_field8
                    == (self::ml_string::Text::from_units(vec![
                        112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 102, 97, 108, 108, 98, 97,
                        99, 107,
                    ])))
                {
                    self::select_java_script_construction_root(
                        heap.clone(),
                        directory.clone(),
                        state.clone(),
                        false,
                    )
                } else {
                    if (shared_constructor_field8
                        == (self::ml_string::Text::from_units(vec![
                            99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 112, 114, 105, 109,
                            105, 116, 105, 118, 101,
                        ])))
                    {
                        self::accept_java_script_construction_conversion(
                            heap.clone(),
                            directory.clone(),
                            state.clone(),
                            self::finish_java_script_scalar_primitive(
                                &(self::read_java_script_construction_conversion_operation(
                                    state.clone(),
                                )),
                                shared_constructor_field12.clone(),
                            ),
                        )
                    } else {
                        if ((shared_constructor_field8
                            == (self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 116,
                            ])))
                            && !(match (
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
                                        source_ml_source2_data_property_value19647,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value19647
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptPrimitiveConversionState>,)
                                        }))
                                    .0)
                                        .clone(),
                                },
                                None::<()>,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }))
                        {
                            self::accept_java_script_construction_conversion(
                                heap.clone(),
                                directory.clone(),
                                state.clone(),
                                self::step_java_script_scalar_protocol(
                                    heap.clone(),
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
                                            source_ml_source2_data_property_value19648,
                                            _,
                                            _,
                                        ) => ((source_ml_source2_data_property_value19648
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptPrimitiveConversionState>,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .expect("a proven present value"),
                                    &(self::read_java_script_construction_conversion_operation(
                                        state.clone(),
                                    )),
                                ),
                            )
                        } else {
                            if (shared_constructor_field8
                                == (self::ml_string::Text::from_units(vec![
                                    112, 114, 111, 116, 111, 116, 121, 112, 101,
                                ])))
                            {
                                if {
                                    let source_ml_source2_s19652 =
                                        shared_constructor_field6.clone();
                                    match source_ml_source2_s19652.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => false,
                                        _ => true,
                                    }
                                } {
                                    self::JavaScriptConstructionTransition::ConstructionInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111,
                                            110, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101,
                                            32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115,
                                            32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                            110, 101, 119, 84, 97, 114, 103, 101, 116,
                                        ]),
                                    )
                                } else {
                                    {
                                        let get = self::plan_java_script_ordinary_get(
                                            heap.clone(),
                                            (((match shared_constructor_field6.clone() {
                                                self::JavaScriptSemanticValue::Object(
                                                    source_ml_source2_data_property_value19657,
                                                    _,
                                                ) => Some((
                                                    source_ml_source2_data_property_value19657
                                                        .clone(),
                                                )),
                                                _ => None::<(self::ml::Big,)>,
                                            })
                                            .clone()
                                            .expect("a proven present value"))
                                            .0)
                                                .clone(),
                                            self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                                std::cell::RefCell::new(vec![
                                                    112f64, 114f64, 111f64, 116f64, 111f64, 116f64,
                                                    121f64, 112f64, 101f64,
                                                ]),
                                            )),
                                            shared_constructor_field6.clone(),
                                        );
                                        if (((({
                                            let ml_reference_value1005 = get.borrow().clone();
                                            ml_reference_value1005
                                        })
                                        .0)
                                            .3)
                                            != (self::ml_string::Text::from_units(vec![])))
                                        {
                                            self::request_java_script_construction_implementation(
                                                heap.clone(),
                                                state.clone(),
                                                &(self::ml_string::Text::from_units(vec![
                                                    110, 101, 119, 84, 97, 114, 103, 101, 116, 32,
                                                    112, 114, 111, 116, 111, 116, 121, 112, 101,
                                                    32, 71, 101, 116,
                                                ])),
                                                &((({
                                                    let ml_reference_value1006 =
                                                        get.borrow().clone();
                                                    ml_reference_value1006
                                                })
                                                .0)
                                                    .3),
                                            )
                                        } else {
                                            if (((({
                                                let ml_reference_value1007 = get.borrow().clone();
                                                ml_reference_value1007
                                            })
                                            .0)
                                                .0)
                                                == (self::ml_string::Text::from_units(vec![
                                                    118, 97, 108, 117, 101,
                                                ])))
                                            {
                                                self::continue_java_script_construction_prototype(
                                                    heap.clone(),
                                                    directory.clone(),
                                                    state.clone(),
                                                    (({
                                                        let ml_reference_value1008 =
                                                            get.borrow().clone();
                                                        ml_reference_value1008
                                                    })
                                                    .0)
                                                        .5,
                                                )
                                            } else {
                                                if (((({
                                                    let ml_reference_value1009 =
                                                        get.borrow().clone();
                                                    ml_reference_value1009
                                                })
                                                .0)
                                                    .0)
                                                    != (self::ml_string::Text::from_units(vec![
                                                        99, 97, 108, 108,
                                                    ])))
                                                {
                                                    self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 103, 101, 116, 116, 101, 114]))
                                                } else {
                                                    {
                                                        let call = self::JavaScriptConstructionCall::ConstructionCallRecord(Box::new((({ let ml_reference_value1010 = get.borrow().clone(); ml_reference_value1010 }).0).2), Box::new((({ let ml_reference_value1011 = get.borrow().clone(); ml_reference_value1011 }).0).4), (({ let ml_reference_value1012 = get.borrow().clone(); ml_reference_value1012 }).0).1, false, Box::new(self::JavaScriptSemanticValue::Undefined));
                                                        match call.clone() {
                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, _, shared_constructor_field4) => {
                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                                let shared_constructor_field4 = *shared_constructor_field4;
                                                                self::JavaScriptConstructionTransition::ConstructionCall(Box::new(heap.clone()), Box::new({
                                                                    let source_ml_source2_nominal_field19660 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field19662 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field19664 = shared_constructor_field7.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field19666 = shared_constructor_field8.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field19668 = shared_constructor_field9.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field19670 = shared_constructor_field10.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field19672 = shared_constructor_field11.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field19674 = shared_constructor_field12.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field19676 = shared_constructor_field13;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field19678 = shared_constructor_field14.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field19680 = shared_constructor_field15.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field19682 = shared_constructor_field16.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field19684 = shared_constructor_field17.clone();
                                                                                                                    self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19660.clone()), Box::new(source_ml_source2_nominal_field19662.clone()), source_ml_source2_nominal_field19664.clone(), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]), source_ml_source2_nominal_field19668.clone(), Box::new(source_ml_source2_nominal_field19670.clone()), Box::new(source_ml_source2_nominal_field19672.clone()), Box::new(source_ml_source2_nominal_field19674.clone()), source_ml_source2_nominal_field19676, source_ml_source2_nominal_field19678.clone(), source_ml_source2_nominal_field19680.clone(), Some((Some(call.clone()),)), source_ml_source2_nominal_field19684.clone())
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
                                                                }), Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone(), false, Box::new(shared_constructor_field4.clone()))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if (shared_constructor_field8
                                    == (self::ml_string::Text::from_units(vec![
                                        97, 108, 108, 111, 99, 97, 116, 101,
                                    ])))
                                {
                                    {
                                        let source_ml_source2_s19714 =
                                            if (shared_constructor_field14
                                                == (self::ml_string::Text::from_units(vec![
                                                    119, 114, 97, 112, 112, 101, 114,
                                                ])))
                                            {
                                                {
                                                    let request =
                                                        self::begin_java_script_primitive_wrapper(
                                                            heap.clone(),
                                                            shared_constructor_field12.clone(),
                                                            shared_constructor_field10.clone(),
                                                        );
                                                    match request.clone() {
                                                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperResourceLimit(heap_2, _, reason) => {
                                                        let heap_2 = *heap_2;
                                                        self::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987Return(Box::new(self::JavaScriptConstructionTransition::ConstructionResourceLimit(Box::new(heap_2.clone()), Box::new(state.clone()), reason.clone())))
                                                    }
                                                    _ => {
                                                        match request.clone() {
                                                            self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperAllocate(_, _, _) => {
                                                                self::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987Next
                                                            }
                                                            self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperCompleted(heap_3, _) => {
                                                                let heap_3 = *heap_3;
                                                                self::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987Return(Box::new(self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 119, 114, 97, 112, 112, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))))
                                                            }
                                                            self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperInvalid(heap_4, source_ml_source2_f19724) => {
                                                                let heap_4 = *heap_4;
                                                                self::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987Return(Box::new(self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_4.clone()), source_ml_source2_f19724.clone())))
                                                            }
                                                            self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperResourceLimit(heap_5, _, _) => {
                                                                let heap_5 = *heap_5;
                                                                self::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987Return(Box::new(self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap_5.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 119, 114, 97, 112, 112, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))))
                                                            }
                                                        }
                                                    }
                                                }
                                                }
                                            } else {
                                                {
                                                    let properties = 0f64;
                                                    {
                                                        let source_ml_source1_temporary_values986 = match heap.clone() {
                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value19730, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value19730.clone()
                                                        }
                                                    };
                                                        {
                                                            let source_ml_source1_temporary_index986 =
                                                                self::ml::Big::from_i128(0);
                                                            {
                                                                let properties_2 = self::source_ml_source1_temporary_step_java_script_construction_allocation_loop988(properties, source_ml_source1_temporary_values986.clone(), source_ml_source1_temporary_index986.clone());
                                                                if ((({ let ml_reference_array1013 = match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value19732, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value19732.clone()
                                                                }
                                                            }; let ml_reference_projection1013 = ml_reference_array1013.borrow().len() as f64; ml_reference_projection1013 }) >= (4096f64)) || ((shared_constructor_field14 == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) && (properties_2 >= (16384f64)))) {
                                                                self::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987Return(Box::new(self::JavaScriptConstructionTransition::ConstructionResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116, 32, 97, 110, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 98, 117, 100, 103, 101, 116]))))
                                                            } else {
                                                                self::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987Next
                                                            }
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                        match source_ml_source2_s19714.clone() {
                                            self::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987Next => {
                                                self::JavaScriptConstructionTransition::ConstructionAllocate(Box::new(heap.clone()), Box::new(state.clone()), Box::new(shared_constructor_field10.clone()))
                                            }
                                            self::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987::MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987Return(source_ml_source1_temporary_result987) => {
                                                let source_ml_source1_temporary_result987 = *source_ml_source1_temporary_result987;
                                                source_ml_source1_temporary_result987.clone()
                                            }
                                        }
                                    }
                                } else {
                                    if (shared_constructor_field8
                                        == (self::ml_string::Text::from_units(vec![
                                            97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104,
                                        ])))
                                    {
                                        {
                                            let length = ({
                                                let source_ml_source2_array_read_value19739 =
                                                    shared_constructor_field7.clone();
                                                {
                                                    let source_ml_source2_array_read_index19740 =
                                                        0f64;
                                                    {
                                                        let ml_reference_array1014 =
                                                            source_ml_source2_array_read_value19739
                                                                .clone();
                                                        let ml_reference_position1014 =
                                                            source_ml_source2_array_read_index19740;
                                                        let ml_reference_projection1014 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array1014.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position1014,
                                                                ),
                                                            );
                                                        ml_reference_projection1014
                                                    }
                                                }
                                            })
                                            .clone()
                                            .unwrap_or_else(|| {
                                                self::JavaScriptSemanticValue::Undefined
                                            });
                                            {
                                                let receiver = shared_constructor_field11.clone();
                                                match length.clone() {
                                                    self::JavaScriptSemanticValue::Number(value) => {
                                                        match receiver.clone() {
                                                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                                                {
                                                                    let unsigned = self::dependency_function5(value);
                                                                    if (unsigned != value) {
                                                                        {
                                                                            let fact = self::read_java_script_construction_fact(directory.clone(), shared_constructor_field5.clone(), Some(heap.clone()));
                                                                            if match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 97, 108, 109]))
                                                                            } else {
                                                                                {
                                                                                    let source_ml_source2_constructor_field19748 = Some(match fact.clone().expect("a proven present value") {
                                                                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value19747, _, _) => {
                                                                                            source_ml_source2_data_property_value19747.clone()
                                                                                        }
                                                                                    });
                                                                                    self::JavaScriptConstructionTransition::ConstructionError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]), Some((source_ml_source2_constructor_field19748.clone(),)))
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let descriptor = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false);
                                                                            {
                                                                                let source_ml_source1_temporary_step_java_script_construction_allocation_written989 = { let ml_field_owner1015 = descriptor.clone(); let ml_field_value1015 = self::JavaScriptSemanticValue::Number(unsigned); ml_field_owner1015.borrow_mut().0.8 = ml_field_value1015; () };
                                                                                {
                                                                                    let source_ml_source1_temporary_step_java_script_construction_allocation_written990 = { let ml_field_owner1016 = descriptor.clone(); let ml_field_value1016 = true; ml_field_owner1016.borrow_mut().0.10 = ml_field_value1016; () };
                                                                                    {
                                                                                        let defined = self::define_java_script_ordinary_property(heap.clone(), identity.clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64]))), descriptor.clone());
                                                                                        if ((({ let ml_reference_value1017 = defined.borrow().clone(); ml_reference_value1017 }).0).3 && (((({ let ml_reference_value1018 = defined.borrow().clone(); ml_reference_value1018 }).0).0) == (self::ml_string::Text::from_units(vec![])))) {
                                                                                            self::JavaScriptConstructionTransition::ConstructionCompleted(Box::new((({ let ml_reference_value1019 = defined.borrow().clone(); ml_reference_value1019 }).0).1), Box::new(receiver.clone()))
                                                                                        } else {
                                                                                            self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new((({ let ml_reference_value1020 = defined.borrow().clone(); ml_reference_value1020 }).0).1), {
                                                                                                let source_ml_source2_logical19749 = (({ let ml_reference_value1021 = defined.borrow().clone(); ml_reference_value1021 }).0).0;
                                                                                                if !source_ml_source2_logical19749.0.is_empty() {
                                                                                                    source_ml_source2_logical19749.clone()
                                                                                                } else {
                                                                                                    self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104, 32, 65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 109, 117, 115, 116, 32, 115, 117, 99, 99, 101, 101, 100])
                                                                                                }
                                                                                            })
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 105, 110, 103, 108, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 105, 110, 112, 117, 116, 32, 97, 110, 100, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 105, 110, 103, 108, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 105, 110, 112, 117, 116, 32, 97, 110, 100, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if (shared_constructor_field8
                                            == (self::ml_string::Text::from_units(vec![
                                                97, 114, 114, 97, 121, 32, 101, 108, 101, 109, 101,
                                                110, 116, 115,
                                            ])))
                                        {
                                            if (shared_constructor_field13
                                                == ({
                                                    let ml_reference_array1022 =
                                                        shared_constructor_field7.clone();
                                                    let ml_reference_projection1022 =
                                                        ml_reference_array1022.borrow().len()
                                                            as f64;
                                                    ml_reference_projection1022
                                                }))
                                            {
                                                self::JavaScriptConstructionTransition::ConstructionCompleted(Box::new(heap.clone()), Box::new(shared_constructor_field11.clone()))
                                            } else {
                                                if {
                                                    let source_ml_source2_s19758 =
                                                        shared_constructor_field11.clone();
                                                    match source_ml_source2_s19758.clone() {
                                                        self::JavaScriptSemanticValue::Object(
                                                            _,
                                                            _,
                                                        ) => false,
                                                        _ => true,
                                                    }
                                                } {
                                                    self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 101, 108, 101, 109, 101, 110, 116, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                } else {
                                                    {
                                                        let spelling = {
                                                            let source_ml_source2_string_argument19763 =
                                                                shared_constructor_field13;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument19763))
                                                        };
                                                        {
                                                            let units = std::rc::Rc::new(
                                                                std::cell::RefCell::new(
                                                                    Vec::<f64>::new(),
                                                                ),
                                                            );
                                                            {
                                                                let index = 0f64;
                                                                {
                                                                    let source_ml_source2_s19764 = self::source_ml_source1_temporary_step_java_script_construction_allocation_loop991(&(spelling), units.clone(), index);
                                                                    match source_ml_source2_s19764.clone() {
                                                                        self::MlSource1TemporaryStepJavaScriptConstructionAllocationLoop991Result::MlSource1TemporaryStepJavaScriptConstructionAllocationLoop991Done => {
                                                                            {
                                                                                let key = self::JavaScriptSemanticValue::Text(units.clone());
                                                                                if !(self::test_java_script_reflection_property_budget(heap.clone(), shared_constructor_field11.clone(), key.clone())) {
                                                                                    self::JavaScriptConstructionTransition::ConstructionResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                                                } else {
                                                                                    {
                                                                                        let defined_2 = self::define_java_script_ordinary_property(heap.clone(), (((match shared_constructor_field11.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value19767, _) => {
                                                                                                Some((source_ml_source2_data_property_value19767.clone(),))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(self::ml::Big,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0).clone(), key.clone(), self::create_java_script_ordinary_set_descriptor(({
                                                                                            let source_ml_source2_array_read_value19770 = shared_constructor_field7.clone();
                                                                                            {
                                                                                                let source_ml_source2_array_read_index19771 = shared_constructor_field13;
                                                                                                { let ml_reference_array1023 = source_ml_source2_array_read_value19770.clone(); let ml_reference_position1023 = source_ml_source2_array_read_index19771; let ml_reference_projection1023 = self::ml_array::optional_at(&*ml_reference_array1023.borrow(), self::ml_array::number_index(ml_reference_position1023)); ml_reference_projection1023 }
                                                                                            }
                                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), true));
                                                                                        if ((({ let ml_reference_value1024 = defined_2.borrow().clone(); ml_reference_value1024 }).0).3 && (((({ let ml_reference_value1025 = defined_2.borrow().clone(); ml_reference_value1025 }).0).0) == (self::ml_string::Text::from_units(vec![])))) {
                                                                                            self::JavaScriptConstructionTransition::ConstructionContinue(Box::new((({ let ml_reference_value1026 = defined_2.borrow().clone(); ml_reference_value1026 }).0).1), Box::new({
                                                                                                let source_ml_source2_nominal_field19774 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field19776 = shared_constructor_field6.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field19778 = shared_constructor_field7.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field19780 = shared_constructor_field8.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field19782 = shared_constructor_field9.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field19784 = shared_constructor_field10.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field19786 = shared_constructor_field11.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field19788 = shared_constructor_field12.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field19790 = shared_constructor_field13;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field19792 = shared_constructor_field14.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field19794 = shared_constructor_field15.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field19796 = shared_constructor_field16.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field19798 = shared_constructor_field17.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_override19800 = (shared_constructor_field13 + 1f64);
                                                                                                                                                    self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19774.clone()), Box::new(source_ml_source2_nominal_field19776.clone()), source_ml_source2_nominal_field19778.clone(), source_ml_source2_nominal_field19780.clone(), source_ml_source2_nominal_field19782.clone(), Box::new(source_ml_source2_nominal_field19784.clone()), Box::new(source_ml_source2_nominal_field19786.clone()), Box::new(source_ml_source2_nominal_field19788.clone()), source_ml_source2_nominal_override19800, source_ml_source2_nominal_field19792.clone(), source_ml_source2_nominal_field19794.clone(), source_ml_source2_nominal_field19796.clone(), source_ml_source2_nominal_field19798.clone())
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
                                                                                        } else {
                                                                                            self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new((({ let ml_reference_value1027 = defined_2.borrow().clone(); ml_reference_value1027 }).0).1), {
                                                                                                let source_ml_source2_logical19801 = (({ let ml_reference_value1028 = defined_2.borrow().clone(); ml_reference_value1028 }).0).0;
                                                                                                if !source_ml_source2_logical19801.0.is_empty() {
                                                                                                    source_ml_source2_logical19801.clone()
                                                                                                } else {
                                                                                                    self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104, 32, 65, 114, 114, 97, 121, 32, 101, 108, 101, 109, 101, 110, 116, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 109, 117, 115, 116, 32, 115, 117, 99, 99, 101, 101, 100])
                                                                                                }
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
                                            }
                                        } else {
                                            self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 111, 114, 32, 115, 101, 116, 117, 112, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
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
