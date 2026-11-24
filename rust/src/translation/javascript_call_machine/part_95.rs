// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptConstructionTransition;
use super::JavaScriptConstructorRealm;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::ml_string;
use super::read_java_script_construction_fact;
use super::read_java_script_construction_prototype_root;
use super::read_java_script_constructor_authority_target;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn clear_java_script_construction_child(
    state: self::JavaScriptConstructionState,
) -> self::JavaScriptConstructionState {
    {
        let call = match state.clone() {
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
                source_ml_source2_data_property_value19120,
                _,
            ) => ((source_ml_source2_data_property_value19120
                .clone()
                .unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,)))
            .0)
                .clone(),
        };
        {
            let conversion = match state.clone() {
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
                    source_ml_source2_data_property_value19121,
                    _,
                    _,
                ) => ((source_ml_source2_data_property_value19121
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,)))
                .0)
                    .clone(),
            };
            {
                let base = match state.clone() {
                    self::JavaScriptConstructionState::ConstructionState(
                        source_ml_source2_nominal_rest_field19122,
                        source_ml_source2_nominal_rest_field19123,
                        source_ml_source2_nominal_rest_field19124,
                        source_ml_source2_nominal_rest_field19125,
                        source_ml_source2_nominal_rest_field19126,
                        source_ml_source2_nominal_rest_field19127,
                        source_ml_source2_nominal_rest_field19128,
                        source_ml_source2_nominal_rest_field19129,
                        source_ml_source2_nominal_rest_field19130,
                        source_ml_source2_nominal_rest_field19131,
                        _,
                        _,
                        source_ml_source2_nominal_rest_field19132,
                    ) => {
                        let source_ml_source2_nominal_rest_field19122 =
                            *source_ml_source2_nominal_rest_field19122;
                        let source_ml_source2_nominal_rest_field19123 =
                            *source_ml_source2_nominal_rest_field19123;
                        let source_ml_source2_nominal_rest_field19127 =
                            *source_ml_source2_nominal_rest_field19127;
                        let source_ml_source2_nominal_rest_field19128 =
                            *source_ml_source2_nominal_rest_field19128;
                        let source_ml_source2_nominal_rest_field19129 =
                            *source_ml_source2_nominal_rest_field19129;
                        self::JavaScriptConstructionState::ConstructionState(
                            Box::new(source_ml_source2_nominal_rest_field19122.clone()),
                            Box::new(source_ml_source2_nominal_rest_field19123.clone()),
                            source_ml_source2_nominal_rest_field19124.clone(),
                            source_ml_source2_nominal_rest_field19125.clone(),
                            source_ml_source2_nominal_rest_field19126.clone(),
                            Box::new(source_ml_source2_nominal_rest_field19127.clone()),
                            Box::new(source_ml_source2_nominal_rest_field19128.clone()),
                            Box::new(source_ml_source2_nominal_rest_field19129.clone()),
                            source_ml_source2_nominal_rest_field19130,
                            source_ml_source2_nominal_rest_field19131.clone(),
                            None::<(Option<self::JavaScriptPrimitiveConversionState>,)>,
                            None::<(Option<self::JavaScriptConstructionCall>,)>,
                            source_ml_source2_nominal_rest_field19132.clone(),
                        )
                    }
                };
                base.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_construction_implementation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptConstructionState,
    operation: &self::ml_string::Text,
    reason: &self::ml_string::Text,
) -> self::JavaScriptConstructionTransition {
    match state.clone() {
        self::JavaScriptConstructionState::ConstructionState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
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
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            self::JavaScriptConstructionTransition::ConstructionNeedImplementation(
                Box::new(heap.clone()),
                Box::new(state.clone()),
                Box::new(shared_constructor_field1.clone()),
                Box::new(shared_constructor_field2.clone()),
                shared_constructor_field3.clone(),
                operation.clone(),
                reason.clone(),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_construction_conversion_operation(
    state: self::JavaScriptConstructionState,
) -> self::ml_string::Text {
    if ((match state.clone() {
        self::JavaScriptConstructionState::ConstructionState(
            _,
            _,
            _,
            _,
            source_ml_source2_data_property_value19137,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => source_ml_source2_data_property_value19137.clone(),
    }) == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])))
    {
        self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
            116, 101, 120, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110,
            117, 109, 98, 101, 114,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_construction_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptConstructionState,
    transition: self::JavaScriptScalarConversionTransition,
) -> self::JavaScriptConstructionTransition {
    {
        let base = self::clear_java_script_construction_child(state.clone());
        match base.clone() {
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
                match transition.clone() {
                    self::JavaScriptScalarConversionTransition::ScalarContinuation(state_2) => {
                        let state_2 = *state_2;
                        self::JavaScriptConstructionTransition::ConstructionContinue(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field19139 =
                                    shared_constructor_field5.clone();
                                {
                                    let source_ml_source2_nominal_field19141 =
                                        shared_constructor_field6.clone();
                                    {
                                        let source_ml_source2_nominal_field19143 =
                                            shared_constructor_field7.clone();
                                        {
                                            let source_ml_source2_nominal_field19145 =
                                                shared_constructor_field8.clone();
                                            {
                                                let source_ml_source2_nominal_field19147 =
                                                    shared_constructor_field9.clone();
                                                {
                                                    let source_ml_source2_nominal_field19149 =
                                                        shared_constructor_field10.clone();
                                                    {
                                                        let source_ml_source2_nominal_field19151 =
                                                            shared_constructor_field11.clone();
                                                        {
                                                            let source_ml_source2_nominal_field19153 =
                                                                shared_constructor_field12.clone();
                                                            {
                                                                let source_ml_source2_nominal_field19155 =
                                                                    shared_constructor_field13;
                                                                {
                                                                    let source_ml_source2_nominal_field19157 =
                                                                        shared_constructor_field14
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field19159 = shared_constructor_field15.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field19161 = shared_constructor_field16.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field19163 = shared_constructor_field17.clone();
                                                                                self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19139.clone()), Box::new(source_ml_source2_nominal_field19141.clone()), source_ml_source2_nominal_field19143.clone(), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116]), source_ml_source2_nominal_field19147.clone(), Box::new(source_ml_source2_nominal_field19149.clone()), Box::new(source_ml_source2_nominal_field19151.clone()), Box::new(source_ml_source2_nominal_field19153.clone()), source_ml_source2_nominal_field19155, source_ml_source2_nominal_field19157.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field19161.clone(), source_ml_source2_nominal_field19163.clone())
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
                            }),
                        )
                    }
                    self::JavaScriptScalarConversionTransition::ScalarCall(
                        state_3,
                        callee,
                        receiver,
                        arguments,
                    ) => {
                        let state_3 = *state_3;
                        let callee = *callee;
                        let receiver = *receiver;
                        {
                            let call = self::JavaScriptConstructionCall::ConstructionCallRecord(
                                Box::new(callee.clone()),
                                Box::new(receiver.clone()),
                                arguments.clone(),
                                false,
                                Box::new(self::JavaScriptSemanticValue::Undefined),
                            );
                            match call.clone() {
                                self::JavaScriptConstructionCall::ConstructionCallRecord(
                                    shared_constructor_field1,
                                    shared_constructor_field2,
                                    shared_constructor_field3,
                                    _,
                                    shared_constructor_field4,
                                ) => {
                                    let shared_constructor_field1 = *shared_constructor_field1;
                                    let shared_constructor_field2 = *shared_constructor_field2;
                                    let shared_constructor_field4 = *shared_constructor_field4;
                                    self::JavaScriptConstructionTransition::ConstructionCall(
                                        Box::new(heap.clone()),
                                        Box::new({
                                            let source_ml_source2_nominal_field19168 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field19170 =
                                                    shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field19172 =
                                                        shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_nominal_field19174 =
                                                            shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field19176 =
                                                                shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field19178 =
                                                                    shared_constructor_field10
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field19180 =
                                                                        shared_constructor_field11
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field19182 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field19184 = shared_constructor_field13;
                                                                            {
                                                                                let source_ml_source2_nominal_field19186 = shared_constructor_field14.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field19188 = shared_constructor_field15.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field19190 = shared_constructor_field16.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field19192 = shared_constructor_field17.clone();
                                                                                            self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19168.clone()), Box::new(source_ml_source2_nominal_field19170.clone()), source_ml_source2_nominal_field19172.clone(), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116]), source_ml_source2_nominal_field19176.clone(), Box::new(source_ml_source2_nominal_field19178.clone()), Box::new(source_ml_source2_nominal_field19180.clone()), Box::new(source_ml_source2_nominal_field19182.clone()), source_ml_source2_nominal_field19184, source_ml_source2_nominal_field19186.clone(), Some((Some(state_3.clone()),)), Some((Some(call.clone()),)), source_ml_source2_nominal_field19192.clone())
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
                                        }),
                                        Box::new(shared_constructor_field1.clone()),
                                        Box::new(shared_constructor_field2.clone()),
                                        shared_constructor_field3.clone(),
                                        false,
                                        Box::new(shared_constructor_field4.clone()),
                                    )
                                }
                            }
                        }
                    }
                    self::JavaScriptScalarConversionTransition::ScalarCompleted(value) => {
                        let value = *value;
                        self::JavaScriptConstructionTransition::ConstructionContinue(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field19202 =
                                    shared_constructor_field5.clone();
                                {
                                    let source_ml_source2_nominal_field19204 =
                                        shared_constructor_field6.clone();
                                    {
                                        let source_ml_source2_nominal_field19206 =
                                            shared_constructor_field7.clone();
                                        {
                                            let source_ml_source2_nominal_field19208 =
                                                shared_constructor_field8.clone();
                                            {
                                                let source_ml_source2_nominal_field19210 =
                                                    shared_constructor_field9.clone();
                                                {
                                                    let source_ml_source2_nominal_field19212 =
                                                        shared_constructor_field10.clone();
                                                    {
                                                        let source_ml_source2_nominal_field19214 =
                                                            shared_constructor_field11.clone();
                                                        {
                                                            let source_ml_source2_nominal_field19216 =
                                                                shared_constructor_field12.clone();
                                                            {
                                                                let source_ml_source2_nominal_field19218 =
                                                                    shared_constructor_field13;
                                                                {
                                                                    let source_ml_source2_nominal_field19220 =
                                                                        shared_constructor_field14
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field19222 = shared_constructor_field15.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field19224 = shared_constructor_field16.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field19226 = shared_constructor_field17.clone();
                                                                                self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19202.clone()), Box::new(source_ml_source2_nominal_field19204.clone()), source_ml_source2_nominal_field19206.clone(), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101]), source_ml_source2_nominal_field19210.clone(), Box::new(source_ml_source2_nominal_field19212.clone()), Box::new(source_ml_source2_nominal_field19214.clone()), Box::new(value.clone()), source_ml_source2_nominal_field19218, self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]), source_ml_source2_nominal_field19222.clone(), source_ml_source2_nominal_field19224.clone(), source_ml_source2_nominal_field19226.clone())
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
                            }),
                        )
                    }
                    self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                        let value_2 = *value_2;
                        self::JavaScriptConstructionTransition::ConstructionThrow(
                            Box::new(heap.clone()),
                            Box::new(value_2.clone()),
                        )
                    }
                    self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(
                        error_class,
                        reason,
                    ) => {
                        let fact = self::read_java_script_construction_fact(
                            directory.clone(),
                            match state.clone() {
                                self::JavaScriptConstructionState::ConstructionState(
                                    source_ml_source2_data_property_value19231,
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
                                    let source_ml_source2_data_property_value19231 =
                                        *source_ml_source2_data_property_value19231;
                                    source_ml_source2_data_property_value19231.clone()
                                }
                            },
                            Some(heap.clone()),
                        );
                        if match (fact.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptConstructionTransition::ConstructionInvalid(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99,
                                    111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116,
                                    97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114,
                                    101, 97, 108, 109, 32, 102, 97, 99, 116,
                                ]),
                            )
                        } else {
                            {
                                let source_ml_source2_constructor_field19236 = Some(match fact.clone().expect("a proven present value") {
                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value19235, _, _) => {
                                            source_ml_source2_data_property_value19235.clone()
                                        }
                                    });
                                self::JavaScriptConstructionTransition::ConstructionError(
                                    Box::new(heap.clone()),
                                    error_class.clone(),
                                    reason.clone(),
                                    Some((source_ml_source2_constructor_field19236.clone(),)),
                                )
                            }
                        }
                    }
                    self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                        self::JavaScriptConstructionTransition::ConstructionInvalid(
                            Box::new(heap.clone()),
                            reason_2.clone(),
                        )
                    }
                    self::JavaScriptScalarConversionTransition::ScalarResourceLimit(
                        value_3,
                        reason_3,
                    ) => {
                        let value_3 = *value_3;
                        self::JavaScriptConstructionTransition::ConstructionResourceLimit(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field19238 =
                                    shared_constructor_field5.clone();
                                {
                                    let source_ml_source2_nominal_field19240 =
                                        shared_constructor_field6.clone();
                                    {
                                        let source_ml_source2_nominal_field19242 =
                                            shared_constructor_field7.clone();
                                        {
                                            let source_ml_source2_nominal_field19244 =
                                                shared_constructor_field8.clone();
                                            {
                                                let source_ml_source2_nominal_field19246 =
                                                    shared_constructor_field9.clone();
                                                {
                                                    let source_ml_source2_nominal_field19248 =
                                                        shared_constructor_field10.clone();
                                                    {
                                                        let source_ml_source2_nominal_field19250 =
                                                            shared_constructor_field11.clone();
                                                        {
                                                            let source_ml_source2_nominal_field19252 =
                                                                shared_constructor_field12.clone();
                                                            {
                                                                let source_ml_source2_nominal_field19254 =
                                                                    shared_constructor_field13;
                                                                {
                                                                    let source_ml_source2_nominal_field19256 =
                                                                        shared_constructor_field14
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field19258 = shared_constructor_field15.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field19260 = shared_constructor_field16.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field19262 = shared_constructor_field17.clone();
                                                                                self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19238.clone()), Box::new(source_ml_source2_nominal_field19240.clone()), source_ml_source2_nominal_field19242.clone(), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101]), source_ml_source2_nominal_field19246.clone(), Box::new(source_ml_source2_nominal_field19248.clone()), Box::new(source_ml_source2_nominal_field19250.clone()), Box::new(value_3.clone()), source_ml_source2_nominal_field19254, source_ml_source2_nominal_field19256.clone(), source_ml_source2_nominal_field19258.clone(), source_ml_source2_nominal_field19260.clone(), source_ml_source2_nominal_field19262.clone())
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
                            }),
                            reason_3.clone(),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_java_script_construction_root(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptConstructionState,
    object_self: bool,
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
            {
                let candidate = if object_self {
                    shared_constructor_field1.clone()
                } else {
                    shared_constructor_field2.clone()
                };
                {
                    let fact = self::read_java_script_construction_fact(
                        directory.clone(),
                        candidate.clone(),
                        Some(heap.clone()),
                    );
                    {
                        let root = if match (fact.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            None::<self::JavaScriptConstructorRealm>
                        } else {
                            {
                                let ml_values988 = {
                                    let ml_reference_value989 = ((match directory.clone() {
                                self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(_, _, _, _, _, source_ml_source2_data_property_value19268) => {
                                    ((source_ml_source2_data_property_value19268.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptConstructorRealm>>>>,))).0).clone()
                                }
                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptConstructorRealm>::new())))).borrow().clone();
                                    ml_reference_value989
                                };
                                let ml_callback988 = {
                                    let fact = fact.clone();
                                    std::rc::Rc::new(
                                        move |row: self::JavaScriptConstructorRealm| -> bool {
                                            ((match row.clone() {
                                self::JavaScriptConstructorRealm::ConstructorRealm(source_ml_source2_data_property_value19271, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value19271.clone()
                                }
                            }) == (match fact.clone().expect("a proven present value") {
                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value19272, _, _) => {
                                    source_ml_source2_data_property_value19272.clone()
                                }
                            }))
                                        },
                                    )
                                };
                                ml_values988.iter().cloned().enumerate().find_map(
                                    |(ml_index988, ml_value988)| {
                                        if ml_callback988(ml_value988.clone()) {
                                            Some(ml_value988)
                                        } else {
                                            None
                                        }
                                    },
                                )
                            }
                        };
                        {
                            let authority = (self::read_java_script_constructor_authority_target(
                                heap.clone(),
                                candidate.clone(),
                            ))
                            .clone()
                            .unwrap_or_else(|| candidate.clone());
                            if match (root.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::JavaScriptConstructionTransition::ConstructionNeedAuthority(
                                    Box::new(heap.clone()),
                                    Box::new(state.clone()),
                                    Box::new(authority.clone()),
                                )
                            } else {
                                {
                                    let prototype =
                                        self::read_java_script_construction_prototype_root(
                                            root.clone().expect("a proven present value"),
                                            state.clone(),
                                            object_self,
                                        );
                                    if match (prototype.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        self::JavaScriptConstructionTransition::ConstructionNeedAuthority(Box::new(heap.clone()), Box::new(state.clone()), Box::new(authority.clone()))
                                    } else {
                                        self::JavaScriptConstructionTransition::ConstructionContinue(
                                            Box::new(heap.clone()),
                                            Box::new({
                                                let source_ml_source2_nominal_field19274 =
                                                    shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_nominal_field19276 =
                                                        shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source2_nominal_field19278 =
                                                            shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field19280 =
                                                                shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_nominal_field19282 =
                                                                    shared_constructor_field5
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field19284 =
                                                                        shared_constructor_field6
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field19286 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field19288 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field19290 = shared_constructor_field9;
                                                                                {
                                                                                    let source_ml_source2_nominal_field19292 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field19294 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field19296 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field19298 = shared_constructor_field13.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override19301 = prototype.clone().expect("a proven present value");
                                                                                                    self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19274.clone()), Box::new(source_ml_source2_nominal_field19276.clone()), source_ml_source2_nominal_field19278.clone(), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), source_ml_source2_nominal_field19282.clone(), Box::new(source_ml_source2_nominal_override19301.clone()), Box::new(source_ml_source2_nominal_field19286.clone()), Box::new(source_ml_source2_nominal_field19288.clone()), source_ml_source2_nominal_field19290, source_ml_source2_nominal_field19292.clone(), source_ml_source2_nominal_field19294.clone(), source_ml_source2_nominal_field19296.clone(), source_ml_source2_nominal_field19298.clone())
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
                                            }),
                                        )
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
