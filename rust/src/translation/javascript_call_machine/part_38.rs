// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptArrayTransformCall;
use super::JavaScriptArrayTransformState;
use super::JavaScriptArrayTransformTransition;
use super::JavaScriptIndexedPropertyCreation;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptArrayTransformLoop250Result;
use super::begin_java_script_scalar_protocol;
use super::continue_java_script_array_transform_length;
use super::continue_java_script_array_transform_species;
use super::copy_java_script_array_constructor_fact;
use super::create_java_script_indexed_data_property;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_number;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_array_transform_state_error;
use super::read_java_script_indexed_property_presence;
use super::request_java_script_array_transform_allocation;
use super::request_java_script_array_transform_call;
use super::source_ml_source1_temporary_step_java_script_array_transform_loop250;
use super::step_java_script_array_species_selection;
use super::step_java_script_scalar_protocol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_array_transform_element(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayTransformState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayTransformTransition {
    match state.clone() {
        self::JavaScriptArrayTransformState::ArrayTransformState(
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
            shared_constructor_field14,
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
            shared_constructor_field18,
            shared_constructor_field19,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            self::request_java_script_array_transform_call(
                heap.clone(),
                {
                    let source_ml_source2_nominal_field5644 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_nominal_field5646 = shared_constructor_field2.clone();
                        {
                            let source_ml_source2_nominal_field5648 =
                                shared_constructor_field3.clone();
                            {
                                let source_ml_source2_nominal_field5650 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field5652 =
                                        shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_nominal_field5654 =
                                            shared_constructor_field6;
                                        {
                                            let source_ml_source2_nominal_field5656 =
                                                shared_constructor_field7;
                                            {
                                                let source_ml_source2_nominal_field5658 =
                                                    shared_constructor_field8;
                                                {
                                                    let source_ml_source2_nominal_field5660 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_nominal_field5662 =
                                                            shared_constructor_field10.clone();
                                                        {
                                                            let source_ml_source2_nominal_field5664 =
                                                                shared_constructor_field11.clone();
                                                            {
                                                                let source_ml_source2_nominal_field5666 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field5668 =
                                                                        shared_constructor_field13
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field5670 = shared_constructor_field14.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field5672 = shared_constructor_field15.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field5674 = shared_constructor_field16.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field5676 = shared_constructor_field17.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field5678 = shared_constructor_field18.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field5680 = shared_constructor_field19.clone();
                                                                                            self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5644.clone(), Box::new(source_ml_source2_nominal_field5646.clone()), Box::new(source_ml_source2_nominal_field5648.clone()), Box::new(source_ml_source2_nominal_field5650.clone()), Box::new(source_ml_source2_nominal_field5652.clone()), source_ml_source2_nominal_field5654, source_ml_source2_nominal_field5656, source_ml_source2_nominal_field5658, Box::new(source_ml_source2_nominal_field5660.clone()), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field5664.clone()), Box::new(source_ml_source2_nominal_field5666.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107]), source_ml_source2_nominal_field5670.clone(), source_ml_source2_nominal_field5672.clone(), source_ml_source2_nominal_field5674.clone(), source_ml_source2_nominal_field5676.clone(), source_ml_source2_nominal_field5678.clone(), source_ml_source2_nominal_field5680.clone())
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
                },
                shared_constructor_field3.clone(),
                shared_constructor_field4.clone(),
                std::rc::Rc::new(std::cell::RefCell::new(vec![
                    value.clone(),
                    self::JavaScriptSemanticValue::Number(shared_constructor_field7),
                    shared_constructor_field2.clone(),
                ])),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_transform(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArrayTransformState,
) -> self::JavaScriptArrayTransformTransition {
    match state.clone() {
        self::JavaScriptArrayTransformState::ArrayTransformState(
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
            shared_constructor_field14,
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
            shared_constructor_field18,
            shared_constructor_field19,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            {
                let error = self::read_java_script_array_transform_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if !(match (
                        match state.clone() {
                            self::JavaScriptArrayTransformState::ArrayTransformState(
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
                                source_ml_source2_data_property_value5688,
                            ) => ((source_ml_source2_data_property_value5688
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::JavaScriptArrayTransformTransition::ArrayTransformCall(Box::new(heap.clone()), Box::new(state.clone()), Box::new(match (match state.clone() {
                            self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value5689) => {
                                ((source_ml_source2_data_property_value5689.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(source_ml_source2_data_property_value5690, _, _) => {
                                let source_ml_source2_data_property_value5690 = *source_ml_source2_data_property_value5690;
                                source_ml_source2_data_property_value5690.clone()
                            }
                        }), Box::new(match (match state.clone() {
                            self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value5691) => {
                                ((source_ml_source2_data_property_value5691.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(_, source_ml_source2_data_property_value5692, _) => {
                                let source_ml_source2_data_property_value5692 = *source_ml_source2_data_property_value5692;
                                source_ml_source2_data_property_value5692.clone()
                            }
                        }), match (match state.clone() {
                            self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value5693) => {
                                ((source_ml_source2_data_property_value5693.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(_, _, source_ml_source2_data_property_value5694) => {
                                source_ml_source2_data_property_value5694.clone()
                            }
                        })
                    } else {
                        if (shared_constructor_field13
                            == (self::ml_string::Text::from_units(vec![
                                108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115,
                                105, 111, 110,
                            ])))
                        {
                            if match (
                                match state.clone() {
                                    self::JavaScriptArrayTransformState::ArrayTransformState(
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
                                        source_ml_source2_data_property_value5697,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value5697
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
                            } {
                                self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102,
                                        111, 114, 109, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                        101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32,
                                        97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 116, 105, 110,
                                        117, 97, 116, 105, 111, 110,
                                    ]),
                                )
                            } else {
                                self::continue_java_script_array_transform_length(heap.clone(), directory.clone(), state.clone(), self::step_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                    self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value5698, _, _, _, _, _) => {
                                        ((source_ml_source2_data_property_value5698.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value"), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                            }
                        } else {
                            if (shared_constructor_field13
                                == (self::ml_string::Text::from_units(vec![
                                    108, 101, 110, 103, 116, 104, 32, 112, 114, 105, 109, 105, 116,
                                    105, 118, 101,
                                ])))
                            {
                                self::continue_java_script_array_transform_length(
                                    heap.clone(),
                                    directory.clone(),
                                    state.clone(),
                                    self::finish_java_script_scalar_primitive(
                                        &(self::ml_string::Text::from_units(vec![
                                            99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109,
                                            105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101,
                                            114,
                                        ])),
                                        shared_constructor_field12.clone(),
                                    ),
                                )
                            } else {
                                if (shared_constructor_field13
                                    == (self::ml_string::Text::from_units(vec![
                                        115, 112, 101, 99, 105, 101, 115,
                                    ])))
                                {
                                    if match (match state.clone() {
                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value5704, _, _, _, _) => {
                                            ((source_ml_source2_data_property_value5704.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                        }
                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                    } else {
                                        self::continue_java_script_array_transform_species(heap.clone(), state.clone(), self::step_java_script_array_species_selection(heap.clone(), directory.clone(), (match state.clone() {
                                            self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value5705, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value5705.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value")))
                                    }
                                } else {
                                    if (shared_constructor_field13
                                        == (self::ml_string::Text::from_units(vec![
                                            97, 108, 108, 111, 99, 97, 116, 101,
                                        ])))
                                    {
                                        self::request_java_script_array_transform_allocation(heap.clone(), state.clone(), (match state.clone() {
                                            self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value5708, _, _, _) => {
                                                ((source_ml_source2_data_property_value5708.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 109, 97, 112]))) {
                                            shared_constructor_field6
                                        } else {
                                            0f64
                                        })
                                    } else {
                                        if (shared_constructor_field13
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 111, 110, 115, 116, 114, 117, 99, 116,
                                            ])))
                                        {
                                            {
                                                let constructor = if {
                                                    let source_ml_source2_own_property_operands5714 = (
                                                        state.clone(),
                                                        self::ml_string::Text::from_units(vec![
                                                            99, 111, 110, 115, 116, 114, 117, 99,
                                                            116, 111, 114,
                                                        ]),
                                                    );
                                                    {
                                                        let source_ml_source2_own_property_key5715 = ((source_ml_source2_own_property_operands5714.clone()).1,);
                                                        match (source_ml_source2_own_property_operands5714.clone()).0 {
                                                            self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_owned_constructor_field5716, source_ml_source2_owned_constructor_field5717, source_ml_source2_owned_constructor_field5718, source_ml_source2_owned_constructor_field5719, source_ml_source2_owned_constructor_field5720, source_ml_source2_owned_constructor_field5721) => {
                                                                ((((((((((((((((((((((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![36]))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 116, 105, 111, 110])))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107])))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![116, 104, 105, 115, 65, 114, 103, 117, 109, 101, 110, 116])))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 75, 101, 121])))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116])))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116])))) || (((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![112, 104, 97, 115, 101])))) || ((((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && match source_ml_source2_owned_constructor_field5716.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && match source_ml_source2_owned_constructor_field5717.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101]))) && match source_ml_source2_owned_constructor_field5718.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && match source_ml_source2_owned_constructor_field5719.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 70, 97, 99, 116]))) && match source_ml_source2_owned_constructor_field5720.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key5715.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) && match source_ml_source2_owned_constructor_field5721.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }))
                                                            }
                                                        }
                                                    }
                                                } {
                                                    match state.clone() {
                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value5722, _, _) => {
                                                            ((source_ml_source2_data_property_value5722.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    }
                                                } else {
                                                    None::<self::JavaScriptSemanticValue>
                                                };
                                                {
                                                    let fact = match state.clone() {
                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value5723, _) => {
                                                            ((source_ml_source2_data_property_value5723.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConstructorFact>,))).0).clone()
                                                        }
                                                    };
                                                    if (match (constructor.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    } || match (fact.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    }) {
                                                        self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 117, 115, 116, 111, 109, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                                    } else {
                                                        self::JavaScriptArrayTransformTransition::ArrayTransformConstruct(Box::new(heap.clone()), Box::new(state.clone()), Box::new(constructor.clone().expect("a proven present value")), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::Number(if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 109, 97, 112]))) {
                                                            shared_constructor_field6
                                                        } else {
                                                            0f64
                                                        })])), Box::new(self::copy_java_script_array_constructor_fact(fact.clone().expect("a proven present value"))))
                                                    }
                                                }
                                            }
                                        } else {
                                            if {
                                                let source_ml_source2_s5728 =
                                                    shared_constructor_field2.clone();
                                                match source_ml_source2_s5728.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => true,
                                                }
                                            } {
                                                self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 120, 101, 100, 32, 115, 111, 117, 114, 99, 101]))
                                            } else {
                                                if (shared_constructor_field13
                                                    == (self::ml_string::Text::from_units(vec![
                                                        108, 101, 110, 103, 116, 104,
                                                    ])))
                                                {
                                                    {
                                                        let get = self::plan_java_script_ordinary_get(heap.clone(), (((match shared_constructor_field2.clone() {
                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value5735, _) => {
                                                                Some((source_ml_source2_data_property_value5735.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml::Big,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64]))), shared_constructor_field2.clone());
                                                        if (((({
                                                            let ml_reference_value454 =
                                                                get.borrow().clone();
                                                            ml_reference_value454
                                                        })
                                                        .0)
                                                            .3)
                                                            != (self::ml_string::Text::from_units(
                                                                vec![],
                                                            )))
                                                        {
                                                            self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), (({ let ml_reference_value455 = get.borrow().clone(); ml_reference_value455 }).0).3)
                                                        } else {
                                                            if (((({ let ml_reference_value456 = get.borrow().clone(); ml_reference_value456 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                self::request_java_script_array_transform_call(heap.clone(), {
                                                                    let source_ml_source2_nominal_field5738 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field5740 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field5742 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field5744 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field5746 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field5748 = shared_constructor_field6;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field5750 = shared_constructor_field7;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field5752 = shared_constructor_field8;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field5754 = shared_constructor_field9.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field5756 = shared_constructor_field10.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field5758 = shared_constructor_field11.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field5760 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field5762 = shared_constructor_field13.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field5764 = shared_constructor_field14.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field5766 = shared_constructor_field15.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field5768 = shared_constructor_field16.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field5770 = shared_constructor_field17.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field5772 = shared_constructor_field18.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field5774 = shared_constructor_field19.clone();
                                                                                                                                            self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5738.clone(), Box::new(source_ml_source2_nominal_field5740.clone()), Box::new(source_ml_source2_nominal_field5742.clone()), Box::new(source_ml_source2_nominal_field5744.clone()), Box::new(source_ml_source2_nominal_field5746.clone()), source_ml_source2_nominal_field5748, source_ml_source2_nominal_field5750, source_ml_source2_nominal_field5752, Box::new(source_ml_source2_nominal_field5754.clone()), Box::new(source_ml_source2_nominal_field5756.clone()), Box::new(source_ml_source2_nominal_field5758.clone()), Box::new(source_ml_source2_nominal_field5760.clone()), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), source_ml_source2_nominal_field5764.clone(), source_ml_source2_nominal_field5766.clone(), source_ml_source2_nominal_field5768.clone(), source_ml_source2_nominal_field5770.clone(), source_ml_source2_nominal_field5772.clone(), source_ml_source2_nominal_field5774.clone())
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
                                                                }, (({ let ml_reference_value457 = get.borrow().clone(); ml_reference_value457 }).0).2, (({ let ml_reference_value458 = get.borrow().clone(); ml_reference_value458 }).0).4, (({ let ml_reference_value459 = get.borrow().clone(); ml_reference_value459 }).0).1)
                                                            } else {
                                                                if (((({ let ml_reference_value460 = get.borrow().clone(); ml_reference_value460 }).0).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 71, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                                } else {
                                                                    self::continue_java_script_array_transform_length(heap.clone(), directory.clone(), {
                                                                        let source_ml_source2_nominal_field5778 = shared_constructor_field1.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field5780 = shared_constructor_field2.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field5782 = shared_constructor_field3.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field5784 = shared_constructor_field4.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field5786 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field5788 = shared_constructor_field6;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field5790 = shared_constructor_field7;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field5792 = shared_constructor_field8;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field5794 = shared_constructor_field9.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field5796 = shared_constructor_field10.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field5798 = shared_constructor_field11.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field5800 = shared_constructor_field12.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field5802 = shared_constructor_field13.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field5804 = shared_constructor_field14.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field5806 = shared_constructor_field15.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field5808 = shared_constructor_field16.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field5810 = shared_constructor_field17.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field5812 = shared_constructor_field18.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field5814 = shared_constructor_field19.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_override5816 = (({ let ml_reference_value461 = get.borrow().clone(); ml_reference_value461 }).0).5;
                                                                                                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5778.clone(), Box::new(source_ml_source2_nominal_field5780.clone()), Box::new(source_ml_source2_nominal_field5782.clone()), Box::new(source_ml_source2_nominal_field5784.clone()), Box::new(source_ml_source2_nominal_field5786.clone()), source_ml_source2_nominal_field5788, source_ml_source2_nominal_field5790, source_ml_source2_nominal_field5792, Box::new(source_ml_source2_nominal_field5794.clone()), Box::new(source_ml_source2_nominal_field5796.clone()), Box::new(source_ml_source2_nominal_field5798.clone()), Box::new(source_ml_source2_nominal_override5816.clone()), source_ml_source2_nominal_field5802.clone(), source_ml_source2_nominal_field5804.clone(), source_ml_source2_nominal_field5806.clone(), source_ml_source2_nominal_field5808.clone(), source_ml_source2_nominal_field5810.clone(), source_ml_source2_nominal_field5812.clone(), source_ml_source2_nominal_field5814.clone())
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
                                                                    }, self::begin_java_script_scalar_protocol(heap.clone(), (({ let ml_reference_value462 = get.borrow().clone(); ml_reference_value462 }).0).5, shared_constructor_field5.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if (shared_constructor_field13
                                                        == (self::ml_string::Text::from_units(
                                                            vec![119, 114, 105, 116, 101],
                                                        )))
                                                    {
                                                        {
                                                            let written = self::create_java_script_indexed_data_property(heap.clone(), shared_constructor_field9.clone(), if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 109, 97, 112]))) {
                                                                shared_constructor_field7
                                                            } else {
                                                                shared_constructor_field8
                                                            }, shared_constructor_field11.clone());
                                                            match written.clone() {
                                                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyCreated(heap_2) => {
                                                                    let heap_2 = *heap_2;
                                                                    self::JavaScriptArrayTransformTransition::ArrayTransformContinue(Box::new(heap_2.clone()), Box::new({
                                                                        let source_ml_source2_nominal_field5829 = shared_constructor_field1.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field5831 = shared_constructor_field2.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field5833 = shared_constructor_field3.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field5835 = shared_constructor_field4.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field5837 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field5839 = shared_constructor_field6;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field5841 = shared_constructor_field7;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field5843 = shared_constructor_field8;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field5845 = shared_constructor_field9.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field5847 = shared_constructor_field10.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field5849 = shared_constructor_field11.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field5851 = shared_constructor_field12.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field5853 = shared_constructor_field13.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field5855 = shared_constructor_field14.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field5857 = shared_constructor_field15.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field5859 = shared_constructor_field16.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field5861 = shared_constructor_field17.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field5863 = shared_constructor_field18.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field5865 = shared_constructor_field19.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_override5868 = (shared_constructor_field7 + 1f64);
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_override5869 = (shared_constructor_field8 + 1f64);
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_override5870 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_override5871 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5829.clone(), Box::new(source_ml_source2_nominal_field5831.clone()), Box::new(source_ml_source2_nominal_field5833.clone()), Box::new(source_ml_source2_nominal_field5835.clone()), Box::new(source_ml_source2_nominal_field5837.clone()), source_ml_source2_nominal_field5839, source_ml_source2_nominal_override5868, source_ml_source2_nominal_override5869, Box::new(source_ml_source2_nominal_field5845.clone()), Box::new(source_ml_source2_nominal_override5870.clone()), Box::new(source_ml_source2_nominal_override5871.clone()), Box::new(source_ml_source2_nominal_field5851.clone()), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), source_ml_source2_nominal_field5855.clone(), source_ml_source2_nominal_field5857.clone(), source_ml_source2_nominal_field5859.clone(), source_ml_source2_nominal_field5861.clone(), source_ml_source2_nominal_field5863.clone(), source_ml_source2_nominal_field5865.clone())
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
                                                                                }
                                                                            }
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(heap_3, reason) => {
                                                                    let heap_3 = *heap_3;
                                                                    self::JavaScriptArrayTransformTransition::ArrayTransformError(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), reason.clone())
                                                                }
                                                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(heap_4, reason_2) => {
                                                                    let heap_4 = *heap_4;
                                                                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap_4.clone()), reason_2.clone())
                                                                }
                                                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyResourceLimit(heap_5, reason_3) => {
                                                                    let heap_5 = *heap_5;
                                                                    self::JavaScriptArrayTransformTransition::ArrayTransformResourceLimit(Box::new(heap_5.clone()), Box::new(state.clone()), reason_3.clone())
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if (shared_constructor_field13
                                                            != (self::ml_string::Text::from_units(
                                                                vec![114, 101, 97, 100, 121],
                                                            )))
                                                        {
                                                            self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 104, 105, 108, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                        } else {
                                                            if (shared_constructor_field7
                                                                >= shared_constructor_field6)
                                                            {
                                                                self::JavaScriptArrayTransformTransition::ArrayTransformCompleted(Box::new(heap.clone()), Box::new(shared_constructor_field9.clone()))
                                                            } else {
                                                                {
                                                                    let name = {
                                                                        let source_ml_source2_string_argument5880 = shared_constructor_field7;
                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument5880))
                                                                    };
                                                                    {
                                                                        let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                        {
                                                                            let at = 0f64;
                                                                            {
                                                                                let source_ml_source2_s5881 = self::source_ml_source1_temporary_step_java_script_array_transform_loop250(&(name), units.clone(), at);
                                                                                match source_ml_source2_s5881.clone() {
                                                                                    self::MlSource1TemporaryStepJavaScriptArrayTransformLoop250Result::MlSource1TemporaryStepJavaScriptArrayTransformLoop250Done => {
                                                                                        {
                                                                                            let key = self::JavaScriptSemanticValue::Text(units.clone());
                                                                                            {
                                                                                                let presence = self::read_java_script_indexed_property_presence(heap.clone(), (((match shared_constructor_field2.clone() {
                                                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value5883, _) => {
                                                                                                        Some((source_ml_source2_data_property_value5883.clone(),))
                                                                                                    }
                                                                                                    _ => {
                                                                                                        None::<(self::ml::Big,)>
                                                                                                    }
                                                                                                }).clone().expect("a proven present value")).0).clone(), key.clone());
                                                                                                if (((({ let ml_reference_value463 = presence.borrow().clone(); ml_reference_value463 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), (({ let ml_reference_value464 = presence.borrow().clone(); ml_reference_value464 }).0).0)
                                                                                                } else {
                                                                                                    if !((({ let ml_reference_value465 = presence.borrow().clone(); ml_reference_value465 }).0).1) {
                                                                                                        self::JavaScriptArrayTransformTransition::ArrayTransformContinue(Box::new(heap.clone()), Box::new({
                                                                                                            let source_ml_source2_nominal_field5886 = shared_constructor_field1.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field5888 = shared_constructor_field2.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field5890 = shared_constructor_field3.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field5892 = shared_constructor_field4.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field5894 = shared_constructor_field5.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field5896 = shared_constructor_field6;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field5898 = shared_constructor_field7;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field5900 = shared_constructor_field8;
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field5902 = shared_constructor_field9.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field5904 = shared_constructor_field10.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field5906 = shared_constructor_field11.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field5908 = shared_constructor_field12.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field5910 = shared_constructor_field13.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field5912 = shared_constructor_field14.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field5914 = shared_constructor_field15.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field5916 = shared_constructor_field16.clone();
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_field5918 = shared_constructor_field17.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_nominal_field5920 = shared_constructor_field18.clone();
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_nominal_field5922 = shared_constructor_field19.clone();
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source2_nominal_override5924 = (shared_constructor_field7 + 1f64);
                                                                                                                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5886.clone(), Box::new(source_ml_source2_nominal_field5888.clone()), Box::new(source_ml_source2_nominal_field5890.clone()), Box::new(source_ml_source2_nominal_field5892.clone()), Box::new(source_ml_source2_nominal_field5894.clone()), source_ml_source2_nominal_field5896, source_ml_source2_nominal_override5924, source_ml_source2_nominal_field5900, Box::new(source_ml_source2_nominal_field5902.clone()), Box::new(source_ml_source2_nominal_field5904.clone()), Box::new(source_ml_source2_nominal_field5906.clone()), Box::new(source_ml_source2_nominal_field5908.clone()), source_ml_source2_nominal_field5910.clone(), source_ml_source2_nominal_field5912.clone(), source_ml_source2_nominal_field5914.clone(), source_ml_source2_nominal_field5916.clone(), source_ml_source2_nominal_field5918.clone(), source_ml_source2_nominal_field5920.clone(), source_ml_source2_nominal_field5922.clone())
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
                                                                                                        }))
                                                                                                    } else {
                                                                                                        {
                                                                                                            let get_2 = self::plan_java_script_ordinary_get(heap.clone(), (((match shared_constructor_field2.clone() {
                                                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value5926, _) => {
                                                                                                                    Some((source_ml_source2_data_property_value5926.clone(),))
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    None::<(self::ml::Big,)>
                                                                                                                }
                                                                                                            }).clone().expect("a proven present value")).0).clone(), key.clone(), shared_constructor_field2.clone());
                                                                                                            if (((({ let ml_reference_value466 = get_2.borrow().clone(); ml_reference_value466 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), (({ let ml_reference_value467 = get_2.borrow().clone(); ml_reference_value467 }).0).3)
                                                                                                            } else {
                                                                                                                if (((({ let ml_reference_value468 = get_2.borrow().clone(); ml_reference_value468 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                                                                    self::request_java_script_array_transform_call(heap.clone(), {
                                                                                                                        let source_ml_source2_nominal_field5929 = shared_constructor_field1.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field5931 = shared_constructor_field2.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field5933 = shared_constructor_field3.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field5935 = shared_constructor_field4.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field5937 = shared_constructor_field5.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field5939 = shared_constructor_field6;
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field5941 = shared_constructor_field7;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field5943 = shared_constructor_field8;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field5945 = shared_constructor_field9.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field5947 = shared_constructor_field10.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field5949 = shared_constructor_field11.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field5951 = shared_constructor_field12.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field5953 = shared_constructor_field13.clone();
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_field5955 = shared_constructor_field14.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_nominal_field5957 = shared_constructor_field15.clone();
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_nominal_field5959 = shared_constructor_field16.clone();
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source2_nominal_field5961 = shared_constructor_field17.clone();
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source2_nominal_field5963 = shared_constructor_field18.clone();
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source2_nominal_field5965 = shared_constructor_field19.clone();
                                                                                                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5929.clone(), Box::new(source_ml_source2_nominal_field5931.clone()), Box::new(source_ml_source2_nominal_field5933.clone()), Box::new(source_ml_source2_nominal_field5935.clone()), Box::new(source_ml_source2_nominal_field5937.clone()), source_ml_source2_nominal_field5939, source_ml_source2_nominal_field5941, source_ml_source2_nominal_field5943, Box::new(source_ml_source2_nominal_field5945.clone()), Box::new(source_ml_source2_nominal_field5947.clone()), Box::new(source_ml_source2_nominal_field5949.clone()), Box::new(source_ml_source2_nominal_field5951.clone()), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), source_ml_source2_nominal_field5955.clone(), source_ml_source2_nominal_field5957.clone(), source_ml_source2_nominal_field5959.clone(), source_ml_source2_nominal_field5961.clone(), source_ml_source2_nominal_field5963.clone(), source_ml_source2_nominal_field5965.clone())
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
                                                                                                                    }, (({ let ml_reference_value469 = get_2.borrow().clone(); ml_reference_value469 }).0).2, (({ let ml_reference_value470 = get_2.borrow().clone(); ml_reference_value470 }).0).4, (({ let ml_reference_value471 = get_2.borrow().clone(); ml_reference_value471 }).0).1)
                                                                                                                } else {
                                                                                                                    if (((({ let ml_reference_value472 = get_2.borrow().clone(); ml_reference_value472 }).0).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                                                                        self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 101, 108, 101, 109, 101, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 71, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                                                                                    } else {
                                                                                                                        self::begin_java_script_array_transform_element(heap.clone(), state.clone(), (({ let ml_reference_value473 = get_2.borrow().clone(); ml_reference_value473 }).0).5)
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
